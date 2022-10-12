use crate::Wave;

pub struct FrequencyWave<T>
where
    T: Clone + PartialEq,
{
    modules: Vec<T>,
    dict: Vec<Vec<usize>>,
}

impl<T> Wave<T> for FrequencyWave<T>
where
    T: Clone + PartialEq,
{
    fn new(modules: crate::Pattern<T>) -> Self {
        todo!()
    }

    fn modules(&self) -> &[T] {
        todo!()
    }

    fn connections_of(&self, id: usize) -> &[usize] {
        todo!()
    }

    fn solve_from(&self, ids: &[usize]) -> usize {
        todo!()
    }
}
