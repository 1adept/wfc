use std::{
    collections::{HashSet},
};



use crate::{Module};

#[derive(Clone, Copy, PartialEq, Eq, Hash)]
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
    pub(crate) connections: Vec<HashSet<ModuleId>>,
}

impl<T> Pattern<T>
where
    T: Clone + PartialEq,
{
    #[must_use]
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
        let mod_id = ModuleId::new(id);
        self.connections[id].insert(mod_id);
        mod_id
    }

    pub fn connect(&mut self, left: &ModuleId, right: &ModuleId) {
        self.connections[left.id].insert(*right);
        self.connections[right.id].insert(*left);
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