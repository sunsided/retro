use crate::archetype::*;
use crate::system::*;
use crate::component::*;

include!("../generated/world_gen.rs");

#[derive(Debug, Clone, Default)]
pub struct UserState;
