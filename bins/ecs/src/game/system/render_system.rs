use std::fmt::{Debug, Formatter};
use std::sync::Arc;
use parking_lot::Mutex;
use crate::WgpuState;

#[derive(Clone, Default)]
pub struct RenderSystemData {
    pub state: Option<Arc<Mutex<WgpuState>>>
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
