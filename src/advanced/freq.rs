use crate::AdvancedModule;

pub struct Freq<T> {
    value: T,
    limit: u8,
    freq: u8,
    uses: u8,
}

impl<T> Freq<T> {
    pub fn new(value: T) -> Self {
        Self {
            value,
            limit: u8::MAX,
            freq: 1,
            uses: 0,
        }
    }

    pub fn limit(self, limit: u8) -> Self {
        Self { limit, ..self }
    }
    pub fn frequency(self, freq: u8) -> Self {
        Self { freq, ..self }
    }
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
