use std::ops::{Deref, DerefMut};
use std::rc::Rc;

pub struct Font<'a>(Rc<rusttype::Font<'a>>);

impl<'a> Font<'a> {
    pub fn new(font: &'a [u8]) -> Self {
        Font(Rc::new(rusttype::Font::from_bytes(font).unwrap()))
    }
}

impl<'a> Clone for Font<'a> {
    fn clone(&self) -> Self {
        Font(Rc::clone(&self))
    }
}

impl<'a> Deref for Font<'a> {
    type Target = Rc<rusttype::Font<'a>>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<'a> DerefMut for Font<'a> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
