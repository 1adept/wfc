use crate::{pattern::Pattern, wave::Wave};

pub struct BasicWave<T>
where
    T: Clone + PartialEq,
{
    modules: Vec<T>,
    dict: Vec<Vec<usize>>,
}

impl<T> Wave<T> for BasicWave<T>
where
    T: Clone + PartialEq,
{
    fn new(pattern: Pattern<T>) -> Self {
        let Pattern {
            values: modules,
            connections: dict,
        } = pattern;
        let dict = dict
            .into_iter()
            .map(|c| c.into_iter().collect::<Vec<usize>>())
            .collect();
        Self { modules, dict }
    }

    fn modules(&self) -> &[T] {
        &self.modules
    }

    fn connections_of(&self, id: usize) -> &[usize] {
        &self.dict[id][..]
    }

    fn solve_from(&self, ids: &[usize]) -> usize {
        use rand::Rng;

        let random = rand::thread_rng().gen_range(0..ids.len());
        let random = ids[random];
        random
    }
}
