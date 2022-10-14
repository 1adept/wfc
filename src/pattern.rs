use std::collections::HashSet;

use grid::Grid;

use crate::Module;

#[derive(Clone, Copy, PartialEq, Eq)]
pub struct ModuleId {
    pub(crate) id: usize,
}

impl ModuleId {
    pub(crate) fn new(id: usize) -> Self {
        ModuleId { id }
    }
}

pub struct Pattern<T>
where
    T: Clone + PartialEq,
{
    pub(crate) values: Vec<Module<T>>,
    pub(crate) connections: Vec<HashSet<usize>>,
}

impl<T> Pattern<T>
where
    T: Clone + PartialEq,
{
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
    pub fn add(&mut self, value: Module<T>) -> ModuleId {
        let id = self.values.len();
        self.values.push(value);
        self.connections.push(HashSet::new());
        self.connections[id].insert(id);
        ModuleId { id }
    }

    pub fn connect(&mut self, left: &ModuleId, right: &ModuleId) {
        self.connections[left.id].insert(right.id);
        self.connections[right.id].insert(left.id);
    }

    pub fn connect_each(&mut self, from: &ModuleId, to_modules: &[ModuleId]) {
        to_modules.iter().for_each(|m| self.connect(from, m));
    }

    pub fn connect_all(&mut self, modules: &[ModuleId]) {
        modules
            .iter()
            .for_each(|m| modules.iter().for_each(|m2| self.connect(m, m2)));
    }

    pub fn connect_unidirectional(&mut self, from: &ModuleId, to: &ModuleId) {
        self.connect(from, to);
    }

    pub fn connect_all_unidirectional(&mut self, from: &ModuleId, to_other_modules: &[ModuleId]) {
        to_other_modules
            .iter()
            .for_each(|to_mod| self.connect_unidirectional(from, to_mod));
    }
}

impl<T> Default for Pattern<T>
where
    T: Clone + PartialEq,
{
    fn default() -> Self {
        Self {
            values: Vec::new(),
            connections: Vec::new(),
        }
    }
}

impl<T> From<&[&[T]]> for Pattern<T>
where
    T: Clone + PartialEq,
{
    fn from(data: &[&[T]]) -> Self {
        todo!()
    }
}

impl<T> From<Grid<T>> for Pattern<T>
where
    T: Clone + PartialEq,
{
    fn from(_: Grid<T>) -> Self {
        todo!()
    }
}
