use crate::game::archetype::*;
use crate::game::component::*;
use crate::game::system::render_system::RenderSystemData;
use crate::game::system::*;

include!("../../generated/world_gen.rs");

#[derive(Debug, Clone, Default)]
pub struct RenderState;
