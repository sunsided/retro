use crate::archetype::*;
use crate::component::*;
use crate::system::*;

include!("../generated/world_gen.rs");

#[derive(Debug, Clone, Default)]
pub struct UserState;
