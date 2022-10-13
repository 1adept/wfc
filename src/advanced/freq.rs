use crate::AdvancedModule;

pub struct Freq<T> {
    value: T,
    limit: u8,
    freq: u8,
    uses: u8,
}

impl<T> AdvancedModule<T> for Freq<T>
where
    T: Clone + PartialEq,
{
    fn value(&self) -> &T {
        &self.value
    }

    fn is_useable(&self) -> bool {
        self.uses < self.limit
    }

    fn use_module(&mut self) {
        self.uses += 1;
    }

    fn rate(&self) -> u8 {
        self.freq
    }
}
