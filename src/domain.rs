use std::fmt::Display;

/// Domain tracks the indices of modules that are possible for a location in a WaveFunctionCollapse
pub struct Domain {
    pub(crate) possible_modules: Vec<usize>,
}

impl Domain {
    pub(crate) fn new(possible_modules: Vec<usize>) -> Self {
        Self { possible_modules }
    }

    pub(crate) fn solve(&mut self, solved_id: &usize) {
        self.possible_modules.retain(|id| id == solved_id);
    }

    pub(crate) fn entropy(&self) -> usize {
        self.possible_modules.len()
    }

    pub(crate) fn is_solved(&self) -> bool {
        self.entropy() == 1
    }

    pub(crate) fn get_solution(&self) -> Option<usize> {
        if self.is_solved() {
            Some(self.possible_modules[0])
        } else {
            None
        }
    }

    pub(crate) fn retain(&mut self, valid: &[usize]) -> bool {
        let count_before = self.possible_modules.len();
        self.possible_modules
            .retain(|mod_id| valid.contains(mod_id));
        count_before != self.possible_modules.len()
    }
}

/// Displays number of possible modules for this location
impl Display for Domain {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self.possible_modules.len())
    }
}
