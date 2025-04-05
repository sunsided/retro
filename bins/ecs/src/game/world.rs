use crate::game::archetype::*;
use crate::game::component::*;
use crate::game::system::*;
use crate::game::system::render_system::RenderSystemData;

include!("../../generated/world_gen.rs");

#[derive(Debug, Clone, Default)]
pub struct RenderState;
