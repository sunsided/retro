use crate::WgpuState;
use parking_lot::Mutex;
use std::fmt::{Debug, Formatter};
use std::sync::Arc;

#[derive(Clone, Default)]
pub struct RenderSystemData {
    state: Option<Arc<Mutex<WgpuState>>>,
}

impl RenderSystemData {
    pub fn set_state(&mut self, state: Option<Arc<Mutex<WgpuState>>>) {
        self.state = state;
    }
}

impl Debug for RenderSystemData {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        if self.state.is_some() {
            write!(f, "RenderSystemData")
        } else {
            write!(f, "RenderSystemData (uninitialized)")
        }
    }
}
