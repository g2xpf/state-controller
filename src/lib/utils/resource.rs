use crate::error::ResourceUnloadedError;
use std::cell::{BorrowError, BorrowMutError, Ref, RefCell, RefMut};
use std::fmt;
use std::rc::Rc;

pub struct Resource<T>(Option<Rc<RefCell<T>>>);

impl<T> Resource<T> {
    pub fn empty() -> Self {
        Resource(None)
    }

    pub fn set(&mut self, t: T) {
        self.0 = Some(Rc::new(RefCell::new(t)));
    }

    pub fn try_borrow(&self) -> Option<Result<Ref<T>, BorrowError>> {
        self.0.as_ref().map(|r| r.try_borrow())
    }

    pub fn try_borrow_mut(&mut self) -> Option<Result<RefMut<T>, BorrowMutError>> {
        self.0.as_mut().map(|r| r.try_borrow_mut())
    }

    pub fn borrow(&self) -> Ref<T> {
        self.try_borrow()
            .unwrap_or_else(|| panic!("Resource uninitialized!"))
            .unwrap_or_else(|err| panic!("{:?}", err))
    }

    pub fn borrow_mut(&mut self) -> RefMut<T> {
        self.try_borrow_mut()
            .unwrap_or_else(|| panic!("Resource uninitialized!"))
            .unwrap_or_else(|err| panic!("{:?}", err))
    }

    pub fn try_clone(&self) -> Result<Resource<T>, ResourceUnloadedError<T>> {
        self.0
            .as_ref()
            .map(|rc| Resource(Some(Rc::clone(rc))))
            .ok_or(ResourceUnloadedError::new())
    }
}

impl<T> fmt::Debug for Resource<T>
where
    T: fmt::Debug,
{
    fn fmt(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        if let Some(ref resource) = self.0 {
            formatter.write_fmt(format_args!("Resource {:?}", resource))
        } else {
            write!(formatter, "Resource uninitialized")
        }
    }
}
