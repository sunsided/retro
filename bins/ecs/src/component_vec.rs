use std::any::Any;

pub struct ComponentVec {
    pub data: Vec<Box<dyn Any + Send + Sync>>,
}

unsafe impl Send for ComponentVec {}

impl ComponentVec {
    pub fn new() -> Self {
        Self { data: Vec::new() }
    }

    pub fn push<T: 'static + Send + Sync>(&mut self, component: T) {
        self.data.push(Box::new(component));
    }

    pub fn get<T: 'static>(&self, index: usize) -> Option<&T> {
        let borrowed = self.data.get(index)?;
        borrowed.downcast_ref::<T>()
    }

    pub fn get_mut<T: 'static>(&mut self, index: usize) -> Option<&mut T> {
        let borrowed = self.data.get_mut(index)?;
        borrowed.downcast_mut::<T>()
    }
}
