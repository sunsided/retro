use crate::system::*;
use crate::archetype::*;

include!(concat!(env!("OUT_DIR"), "/world.gen.rs"));

#[derive(Debug, Clone)]
pub struct WorldData;
