use std::cell::{Ref, RefCell, RefMut};
use std::sync::{
    mpsc::{channel, Receiver, Sender},
    Arc, Mutex,
};
use std::thread;

pub struct LazyCell<T> {
    resource: RefCell<Option<T>>,
    loading: Arc<Mutex<bool>>,
    loaded: RefCell<bool>,
    txrx: (Sender<T>, Receiver<T>),
}

impl<T> LazyCell<T>
where
    T: Send + 'static,
{
    pub fn load<F>(f: F) -> Self
    where
        F: Fn() -> T,
        F: Send + 'static,
    {
        let loading = Arc::new(Mutex::new(false));
        let txrx = channel();

        let (loading_dup, tx) = (Arc::clone(&loading), txrx.0.clone());
        thread::spawn(move || {
            let resource = f();
            let mut loading_dup = loading_dup.lock().unwrap();
            *loading_dup = true;
            tx.send(resource).unwrap();
        });

        LazyCell {
            resource: RefCell::new(None),
            loading,
            loaded: RefCell::new(false),
            txrx,
        }
    }

    pub fn try_borrow(&self) -> Option<Ref<T>> {
        if *self.loaded.borrow() {
            println!("return faster");
            return Some(Ref::map(self.resource.borrow(), |r| r.as_ref().unwrap()));
        }

        if *self.loading.lock().unwrap() {
            *self.loaded.borrow_mut() = true;
            let resource = self.txrx.1.recv().unwrap();
            *self.resource.borrow_mut() = Some(resource);
            Some(Ref::map(self.resource.borrow(), |r| r.as_ref().unwrap()))
        } else {
            None
        }
    }

    pub fn try_borrow_mut(&mut self) -> Option<RefMut<T>> {
        if *self.loaded.borrow() {
            println!("return faster");
            return Some(RefMut::map(self.resource.borrow_mut(), |r| {
                r.as_mut().unwrap()
            }));
        }

        if *self.loading.lock().unwrap() {
            *self.loaded.borrow_mut() = true;
            let resource = self.txrx.1.recv().unwrap();
            let mut borrow_mut = self.resource.borrow_mut();
            *borrow_mut = Some(resource);
            Some(RefMut::map(borrow_mut, |r| r.as_mut().unwrap()))
        } else {
            None
        }
    }
}
