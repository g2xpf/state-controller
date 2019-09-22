use std::default::Default;
use std::ops::{Deref, DerefMut};

#[derive(Debug)]
pub struct PandoraBox<T>(Option<T>);

impl<T> PandoraBox<T> {
    pub fn empty() -> Self {
        PandoraBox(None)
    }

    pub(crate) fn new(t: T) -> Self {
        PandoraBox(Some(t))
    }
}

impl<T> Deref for PandoraBox<T> {
    type Target = T;
    fn deref(&self) -> &Self::Target {
        self.0
            .as_ref()
            .unwrap_or_else(|| panic!("PandoraBox is empty"))
    }
}

impl<T> DerefMut for PandoraBox<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        self.0
            .as_mut()
            .unwrap_or_else(|| panic!("PandoraBox is empty"))
    }
}

impl<T> Default for PandoraBox<T> {
    fn default() -> Self {
        PandoraBox::empty()
    }
}
