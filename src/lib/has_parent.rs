use crate::{state::State, types::Shifter};
use std::cell::{Ref, RefMut};

pub trait HasParent<P>: State
where
    P: State,
{
    fn parent_ref<'a>(&self, shifter: &'a Shifter) -> Ref<'a, P> {
        let parent = shifter
            .get::<P>()
            .unwrap_or_else(|| panic!("Tried to call the unregistered parent"))
            .borrow();
        Ref::map(parent, |s| s.downcast_ref::<P>().unwrap())
    }

    fn parent_mut<'a>(&self, shifter: &'a mut Shifter) -> RefMut<'a, P> {
        let parent = shifter
            .get::<P>()
            .unwrap_or_else(|| panic!("Tried to call the unregistered parent"))
            .borrow_mut();
        RefMut::map(parent, |s| s.downcast_mut::<P>().unwrap())
    }
}
