use grid::{Grid, GridPos};
use rand::Rng;

use crate::{Domain, Module, ModuleId, Pattern};

pub struct Wave<T>
where
    T: Clone + PartialEq,
{
    modules: Vec<Module<T>>,
    dict: Vec<Vec<ModuleId>>,
}

impl<T> Wave<T>
where
    T: Clone + PartialEq,
{
    #[must_use]
    pub fn new(pattern: crate::Pattern<T>) -> Self {
        let Pattern {
            values: modules,
            connections: dict,
        } = pattern;
        let dict = dict
            .into_iter()
            .map(|c| c.into_iter().collect::<Vec<_>>())
            .collect();
        Self { modules, dict }
    }

    fn connections_of(&self, id: ModuleId) -> &[ModuleId] {
        &self.dict[id.id][..]
    }

    fn solve_from(&mut self, domain: &Domain) -> ModuleId {
        let possible_ids = self.solveable_modules(domain);
        assert!(!possible_ids.is_empty(), "Possible Modules cannot be empty");
        let solution = self.falls_into(&possible_ids);
        // self._fullfills(possible_ids)

        self.modules[solution.id].use_module();
        solution
    }

    /// Calculates the sum of all ratings, then a random number is is generated in that range
    /// The ratings are summed up (rating = `sum_of_previous`) so theres no 'gaps' and with the random number the first rating is picked that is >= `random_number`
    fn falls_into(&self, possible_ids: &[ModuleId]) -> ModuleId {
        let mut ratings: Vec<_> = possible_ids
            .iter()
            .map(|id| (self.modules[id.id].rate() as usize, id))
            .collect();
        let sum_ratings: usize = ratings.iter().map(|(rating, _)| rating).sum();

        let mut rand = rand::thread_rng();
        // let random = rand.gen_range(0..sum_ratings);
        let random = rand.gen::<usize>() % sum_ratings;

        let mut last_summed_rating = 0;
        for mut rating in &mut ratings.iter_mut() {
            last_summed_rating += rating.0;
            rating.0 = last_summed_rating;
        }

        let falls_into = ratings
            .into_iter()
            .skip_while(|(rating, _id)| *rating <= random)
            .take(1)
            .map(|(_, id)| id)
            .collect::<Vec<_>>()[0];
        *falls_into
    }

    /// Gets MAX(rating) and generates a random number in that range
    /// Then all ratings below that randomly generated number are collected and one of the remaining IDs is picked
    fn _fullfills(&self, possible_ids: &[ModuleId]) -> ModuleId {
        let ratings: Vec<_> = possible_ids
            .iter()
            .map(|id| (self.modules[id.id].rate() as usize, id))
            .collect();
        let max_rating = ratings.iter().map(|(rating, _)| rating).max().unwrap();

        let mut rand = rand::thread_rng();
        //A random number to determine which ratings are valid (all >= random_rating)
        let random = rand.gen_range(0..*max_rating);
        let fulfills_rating: Vec<_> = ratings
            .into_iter()
            .filter(|(rating, _)| *rating >= random)
            .map(|(_rating, id)| id)
            .collect();
        let random = rand.gen_range(0..fulfills_rating.len());
        *fulfills_rating[random]
    }

    #[must_use]
    pub fn validate(&self, result: &Grid<T>) -> bool {
        let validate_with = |item, other: Option<&T>| {
            other.map_or(true, |m| self.validate_values_connected(item, m)) //If other is none, then `item` is valid because it CANNOT be invalid
        };

        result.iter().enumerate().all(|(index, item)| {
            let position = GridPos::new(index);
            validate_with(item, result.get_at_offset(&position, 0, 1))
                && validate_with(item, result.get_at_offset(&position, 1, 0))
        })
    }

    /// Collapse wave
    ///
    /// # Panics
    /// panics when a grid-cell is encountered, that is already solved
    pub fn collapse(&mut self, width: u8, height: u8) -> Grid<T> {
        let mut domains = self.create_domain_grid(width, height);

        while !Wave::<T>::completely_collapsed(&domains) {
            let next_collapse_pos = Wave::<T>::find_next_pos_collapse(&domains);
            let dom = domains.get_mut(&next_collapse_pos).unwrap();

            assert!(!dom.is_solved(), "Tried solving a domain that is already solved. Therefore collapse failed somewhere!");

            //Solve from own possible Modules
            dom.solve(self.solve_from(dom));

            self.propagate(next_collapse_pos, &mut domains);
        }

        self.build_solution_grid(&domains)
    }

    fn propagate(&self, position: GridPos, domains: &mut Grid<Domain>) {
        let domain = domains.get_mut(&position).unwrap();

        let module_id = &domain.get_solution();

        if module_id.is_none() {
            return; //Cannot propagate unsolved domain
        }

        let module_id = module_id.unwrap();
        let possible_neighbor_modules = self.connections_of(module_id);
        if possible_neighbor_modules.len() == self.modules.len() {
            return; //ALL modules possible. No point in continueing
        }

        for neighbor_pos in domains.get_neighbors_flat(&position) {
            let neighbord_domain = domains.get_mut(&neighbor_pos).unwrap();
            // retain those modules, which are valid for the source #domain
            if neighbord_domain.retain(possible_neighbor_modules) {
                //Count changed -> propagate change to neighbors
                self.propagate(neighbor_pos, domains);
            }
        }
    }

    fn create_domain_grid(&self, width: u8, height: u8) -> Grid<Domain> {
        let size: u64 = u64::from(width) * u64::from(height);
        let num_modules = self.modules.len();
        let domains = (0..size)
            .into_iter()
            .map(|_| (0..num_modules).map(ModuleId::new).collect())
            .map(Domain::new)
            .collect::<Vec<Domain>>();
        Grid::new(width as usize, domains)
    }

    fn build_solution_grid(&self, solved_domains: &Grid<Domain>) -> Grid<T> {
        let width = solved_domains.width();
        let data = solved_domains
            .iter()
            .map(|dom| {
                self.get_solution_value(
                    dom.get_solution()
                        .expect("Trying to build solution of unsolved WFC!"),
                )
            })
            .collect();
        Grid::new(width, data)
    }

    fn completely_collapsed(domains: &Grid<Domain>) -> bool {
        domains.iter().all(Domain::is_solved)
    }

    fn find_next_pos_collapse(domains: &Grid<Domain>) -> GridPos {
        let mut valid_domains: Vec<(usize, &Domain)> = domains
            .iter()
            .enumerate()
            .filter(|(_grid_index, dom)| !dom.is_solved())
            .collect();
        valid_domains.sort_by_key(|(_, dom)| dom.entropy());
        let min_entropy = valid_domains[0].1.entropy();
        let min_positions: Vec<(usize, &Domain)> = valid_domains
            .into_iter()
            .take_while(|(_, dom)| min_entropy == dom.entropy())
            .collect();

        let random_min_pos = min_positions[rand::thread_rng().gen_range(0..min_positions.len())].0;
        GridPos::new(random_min_pos)
    }

    fn get_solution_value(&self, id: ModuleId) -> T {
        self.modules[id.id].value().clone()
    }

    fn validate_values_connected(&self, value: &T, other: &T) -> bool {
        let value1 = self.find_id(value);
        let value2 = self.find_id(other);

        if let (Some(value1), Some(value2)) = (value1, value2) {
            self.connections_of(value1).contains(&value2)
        } else {
            false
        }
    }

    fn find_id(&self, value: &T) -> Option<ModuleId> {
        self.modules
            .iter()
            .position(|inner| inner.value() == value)
            .map(ModuleId::new)
    }

    fn solveable_modules(&self, domain: &Domain) -> Vec<ModuleId> {
        domain
            .possible_modules
            .iter()
            .filter(|id| self.modules[id.id].is_useable())
            .copied()
            .collect::<Vec<_>>()
    }
}
