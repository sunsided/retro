use std::any::Any;
use std::cell::{Ref, RefCell, RefMut};

pub struct ComponentVec {
    pub data: Vec<RefCell<Box<dyn Any>>>,
}

impl ComponentVec {
    pub fn new() -> Self {
        Self { data: Vec::new() }
    }

    pub fn push<T: 'static>(&mut self, component: T) {
        self.data.push(RefCell::new(Box::new(component)));
    }

    pub fn get<T: 'static>(&self, index: usize) -> Option<Ref<T>> {
        let borrowed = self.data.get(index)?.borrow(); // Ref<Box<dyn Any>>
        Ref::filter_map(borrowed, |b| b.downcast_ref::<T>()).ok()
    }

    pub fn get_mut<T: 'static>(&self, index: usize) -> Option<RefMut<T>> {
        let borrowed = self.data.get(index)?.borrow_mut(); // RefMut<Box<dyn Any>>
        RefMut::filter_map(borrowed, |b| b.downcast_mut::<T>()).ok()
    }
}
