//! This module serves as the entry point for system components and phases used in the engine.
//!
//! It includes various submodules that are responsible for different aspects of the engine's functionality:
//! - [`components`]: Defines the reusable components of the engine.
//! - [`phases`]: Contains logic for different engine execution phases (e.g., fixed updates, rendering).
//!
//! Additionally, this module pulls in generated code related to components, systems, world, and archetypes,
//! ensuring that the underlying engine structure is dynamically extensible.
//!
//! The main structure exported by this module is the `SystemFactory`, which implements
//! the [`CreateSystems`] trait to create and manage the systems used during the engine's execution.

#![allow(unused_imports)]

pub mod components;
pub mod phases;
mod main_world;
mod command_queue;
mod phase_events;
pub mod input_state;

pub use main_world::InitializeMainWorld;
pub use command_queue::CommandQueue;
pub use phase_events::PhaseEvents;
use crate::engine::components::*;
use crate::engine::phases::fixed_update::physics::PhysicsSystemData;
use crate::engine::phases::update::{color_cycle::BackgroundColorCycleSystemData, camera::CameraSystemData};
use crate::engine::phases::render::{
    WgpuRenderState,
    render::RenderSystemData,
    background::{BackgroundColorData, RenderBackgroundSystemData},
    vga_blit::VgaBlitSystemData,
    camera_render::CameraRenderSystemData,
    dynamic_light::DynamicLightSystemData,
};
pub use crate::engine::input_state::InputState;
use crate::engine::camera_fps_controller::CameraFpsControllerData;

/// The hash map used for entity lookups.
type EntityLocationMap<K, V> = fxhash::FxHashMap<K, V>;

include!("../generated/components_gen.rs");
include!("../generated/world_gen.rs");
include!("../generated/systems_gen.rs");
include!("../generated/archetypes_gen.rs");

/// The factory of the different systems.
///
/// Implements [`CreateSystems`].
pub struct SystemFactory;

impl<E, Q> MainWorld<E, Q> {
    pub fn test_frontload(&mut self) -> usize {
        self.archetypes.cube.frontload_entities(&mut self.entity_locations, |entity| {
            entity.position.x > 7.0 &&
                entity.position.z > 9.0
        })
    }
}

impl FrontloadEntitiesScan for CubeArchetype {
}

impl GetEntityAt for CubeArchetype {
    type EntityRef<'archetype> = CubeEntityRef<'archetype>;

    fn get_at<'archetype>(&'archetype self, index: usize) -> Option<Self::EntityRef<'archetype>> {
        self.get_at(index)
    }
}

pub trait GetEntityAt {
    type EntityRef<'archetype>
    where
        Self: 'archetype;

    fn get_at<'archetype>(&'archetype self, index: usize) -> Option<Self::EntityRef<'archetype>>;
}

pub trait FrontloadEntitiesScan:
    Archetype
    + FrontloadEntities
    + GetEntityAt
    + core::ops::Index<usize, Output = EntityId>
{

    /// Frontloads entities satisfying `should_frontload`.
    /// Returns the number of entities frontloaded.
    fn frontload_entities<F>(&mut self, entity_lookup: &mut EntityLocationMap<EntityId, EntityArchetypeRef>, should_frontload: F) -> usize
    where
        for<'archetype> F: Fn(&<Self as GetEntityAt>::EntityRef<'archetype>) -> bool,
    {
        let mut left = 0;
        let mut right = self.len().wrapping_sub(1);
        let mut swaps_made = Vec::new();

        loop {
            // Move left pointer forward until an invalid entry is found
            while left <= right {
                let reference = self.get_at(left).unwrap();
                if should_frontload(&reference) {
                    left += 1;
                } else {
                    break;
                }
            }

            // Move right pointer backward until a valid entry is found
            while left <= right {
                let reference = self.get_at(right).unwrap();
                if !should_frontload(&reference) {
                    right = right.wrapping_sub(1);
                } else {
                    break;
                }
            }

            if left >= right {
                break;
            }

            // Swap entities and record swapped indices
            self.swap_entities(left, right);
            swaps_made.push((left, self[left]));
            swaps_made.push((right, self[right]));

            left += 1;
            right = right.wrapping_sub(1);
        }

        // Only update indices for swapped entities
        for (index, entity_id) in swaps_made {
            let reference = EntityArchetypeRef {
                archetype: Self::ID,
                index
            };
            entity_lookup.insert(entity_id, reference);
        }

        // At this point, `left` indicates how many entities are valid (frontloaded)
        left
    }
}
