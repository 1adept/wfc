use std::collections::HashSet;

pub struct Pattern<T> {
    pub(crate) values: Vec<T>,
    pub(crate) connections: Vec<HashSet<usize>>,
}

impl<T> Pattern<T> {
    pub fn new() -> Self {
        Self {
            values: Vec::new(),
            connections: Vec::new(),
        }
    }

    // pub(crate) fn get(&self, id: usize) -> &T {
    //     &self.values[id]
    // }

    /// Adds a value to this `Pattern` and returns an id for referencing
    pub fn add(&mut self, value: T) -> usize {
        let id = self.values.len();
        self.values.push(value);
        self.connections.push(HashSet::new());
        self.connections[id].insert(id);
        id
    }

    pub fn connect(&mut self, left: &usize, right: &usize) {
        self.connections[*left].insert(*right);
        self.connections[*right].insert(*left);
    }
}
