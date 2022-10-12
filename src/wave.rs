use crate::{domain::Domain, pattern::Pattern};
use grid::*;
use rand::Rng;

pub trait Wave<T>
where
    T: Clone + PartialEq,
{
    fn new(modules: Pattern<T>) -> Self;
    fn modules(&self) -> &[T];
    fn connections_of(&self, id: usize) -> &[usize];

    /// `ids` is not guaranteed to contain all IDs of the `Pattern`
    fn solve_from(&self, ids: &[usize]) -> usize;

    // Already implemented

    fn validate(&self, result: &Grid<T>) -> bool {
        let validate_with = |item, other: Option<&T>| {
            other
                .map(|m| self.validate_values_connected(item, m))
                .unwrap_or(true) //If other is none, then `item` is valid because it CANNOT be invalid
        };

        result.iter().enumerate().all(|(index, item)| {
            let position = GridPos::new(index);
            validate_with(item, result.get_at_offset(&position, 0, 1))
                && validate_with(item, result.get_at_offset(&position, 1, 0))
        })
    }

    fn collapse(&self, width: u8, height: u8) -> Grid<T> {
        let mut domains = self.create_domain_grid(width, height);

        while !self.completely_collapsed(&domains) {
            let next_collapse_pos = self.find_next_pos_collapse(&domains);
            let dom = domains.get_mut(&next_collapse_pos).unwrap();

            if dom.is_solved() {
                panic!("ALREADY SOLVED !!!");
            }
            //Solve from own possible Modules
            dom.solve(&self.solve_from(&dom.possible_modules));

            self.propagate(&next_collapse_pos, &mut domains);
        }

        self.build_solution_grid(&domains)
    }
    fn propagate(&self, position: &GridPos, domains: &mut Grid<Domain>) {
        let domain = domains.get_mut(position).unwrap();

        let module_id = &domain.get_solution();

        if module_id.is_none() {
            return; //Cannot propagate unsolved domain
        }

        let module_id = module_id.unwrap();
        let possible_neighbor_modules = self.connections_of(module_id);
        if possible_neighbor_modules.len() == self.modules().len() {
            return; //ALL modules possible. No point in continueing
        }

        for neighbor_pos in domains.get_neighbors_flat(position) {
            let neighbord_domain = domains.get_mut(&neighbor_pos).unwrap();
            // retain those modules, which are valid for the source #domain
            if neighbord_domain.retain(possible_neighbor_modules) {
                //Count changed -> propagate change to neighbors
                self.propagate(&neighbor_pos, domains);
            }
        }
    }
    fn create_domain_grid(&self, width: u8, height: u8) -> Grid<Domain> {
        let size = width * height;
        let num_modules = self.modules().len();
        let domains = (0..size)
            .into_iter()
            .map(|_| (0..num_modules).collect())
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
                    &dom.get_solution()
                        .expect("Trying to build solution of unsolved WFC!"),
                )
            })
            .collect();
        Grid::new(width, data)
    }

    fn completely_collapsed(&self, domains: &Grid<Domain>) -> bool {
        domains.iter().all(Domain::is_solved)
    }
    fn find_next_pos_collapse(&self, domains: &Grid<Domain>) -> GridPos {
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

    // --- Utils

    fn get_solution_value(&self, id: &usize) -> T {
        self.modules()[*id].clone()
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
    fn find_id(&self, value: &T) -> Option<usize> {
        self.modules().iter().position(|inner| inner == value)
    }
}
