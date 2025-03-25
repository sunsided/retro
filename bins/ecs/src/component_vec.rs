use std::any::Any;

pub struct ComponentVec {
    pub data: Vec<Box<dyn Any>>,
}

impl ComponentVec {
    pub fn new() -> Self {
        Self { data: Vec::new() }
    }

    pub fn push<T: 'static>(&mut self, component: T) {
        self.data.push(Box::new(component));
    }

    pub fn get<T: 'static>(&self, index: usize) -> Option<&T> {
        self.data.get(index)?.downcast_ref()
    }

    pub fn get_mut<T: 'static>(&mut self, index: usize) -> Option<&mut T> {
        self.data.get_mut(index)?.downcast_mut()
    }
}
