pub enum Module<T>
where
    T: Clone + PartialEq,
{
    Basic { value: T },
    Advanced { module: Box<dyn AdvancedModule<T>> },
}

impl<T> Module<T>
where
    T: Clone + PartialEq,
{
    pub(crate) fn value(&self) -> &T {
        match self {
            Module::Basic { value } => value,
            Module::Advanced { module } => module.value(),
        }
    }

    pub(crate) fn is_useable(&self) -> bool {
        match self {
            Module::Basic { value: _ } => true,
            Module::Advanced { module } => module.is_useable(),
        }
    }

    pub(crate) fn use_module(&mut self) {
        match self {
            Module::Basic { value: _ } => (),
            Module::Advanced { module } => module.use_module(),
        }
    }

    pub(crate) fn rate(&self) -> u8 {
        match self {
            Module::Basic { value: _ } => 1,
            Module::Advanced { module } => module.rate(),
        }
    }
}

impl<T> PartialEq for Module<T>
where
    T: Clone + PartialEq,
{
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (Self::Basic { value: l_value }, Self::Basic { value: r_value }) => l_value == r_value,
            (Self::Advanced { module: l_module }, Self::Advanced { module: r_module }) => {
                l_module.value() == r_module.value() && matches!(l_module, _r_module)
            }
            _ => false,
        }
    }
}

pub trait AdvancedModule<T>
where
    T: Clone + PartialEq,
{
    fn value(&self) -> &T;
    fn is_useable(&self) -> bool;
    fn use_module(&mut self);
    /// Returns the rate this `Module` appears over other `Module`s
    fn rate(&self) -> u8;
}
