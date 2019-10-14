use super::resource::Resource;
use crate::error::ResourceUnloadedError;

use std::cell::RefCell;
use std::sync::{
    mpsc::{channel, Receiver, Sender},
    Arc, Mutex,
};
use std::thread;

pub struct LazyCell<T> {
    pub resource: Resource<T>,
    loading: Arc<Mutex<bool>>,
    loaded: RefCell<bool>,
    txrx: (Sender<T>, Receiver<T>),
}

impl<T> LazyCell<T>
where
    T: Send + 'static,
{
    pub fn new() -> Self {
        LazyCell {
            resource: Resource::empty(),
            loading: Arc::new(Mutex::new(false)),
            loaded: RefCell::new(false),
            txrx: channel(),
        }
    }

    pub fn load<F>(&mut self, loader: F)
    where
        F: Fn() -> T,
        F: Send + 'static,
    {
        let (loading_dup, tx) = (Arc::clone(&self.loading), self.txrx.0.clone());
        thread::spawn(move || {
            let resource = loader();
            let mut loading_dup = loading_dup.lock().unwrap();
            *loading_dup = true;
            tx.send(resource).unwrap();
        });
    }

    pub fn poll(&mut self) -> bool {
        if *self.loaded.borrow() {
            return true;
        }

        if *self.loading.lock().unwrap() {
            *self.loaded.borrow_mut() = true;
            let resource = self.txrx.1.recv().unwrap();
            self.resource.set(resource);
            true
        } else {
            false
        }
    }

    pub fn write(&self, target: &mut Resource<T>) {
        if let Err(err) = self.try_write(target) {
            panic!("{:?}", err);
        }
    }

    pub fn try_write(&self, target: &mut Resource<T>) -> Result<(), ResourceUnloadedError<T>> {
        *target = self.resource.try_clone()?;
        Ok(())
    }

    pub fn clone_resource(&self) -> Resource<T> {
        self.try_clone_resource().unwrap_or_else(|err| {
            panic!("{:?}", err);
        })
    }

    pub fn try_clone_resource(&self) -> Result<Resource<T>, ResourceUnloadedError<T>> {
        self.resource.try_clone()
    }
}
