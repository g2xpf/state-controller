use std::any;
use std::error::Error;
use std::fmt;
use std::marker::PhantomData;

pub struct ResourceUnloadedError<T>(PhantomData<T>);

impl<T> ResourceUnloadedError<T> {
    pub fn new() -> Self {
        ResourceUnloadedError(PhantomData)
    }
}

impl<T> fmt::Display for ResourceUnloadedError<T> {
    fn fmt(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        write!(
            formatter,
            "Resource \"{}\" is not loaded: you must call \"poll\" before extraction.",
            any::type_name::<T>()
        )
    }
}

impl<T> fmt::Debug for ResourceUnloadedError<T> {
    fn fmt(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        write!(
            formatter,
            "Resource \"{}\" is not loaded: you must call \"poll\" before extraction.",
            any::type_name::<T>()
        )
    }
}

impl<T> Error for ResourceUnloadedError<T> {}
