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

impl CubeArchetype {
    /// Frontloads entities satisfying `should_frontload`.
    /// Returns the number of entities frontloaded.
    pub fn frontload_entities<F>(&mut self, entity_lookup: &mut EntityLocationMap<EntityId, EntityArchetypeRef>, should_frontload: F) -> usize
    where
        F: Fn(&CubeEntityData) -> bool,
    {
        let mut left = 0;
        let mut right = self.entities.len().wrapping_sub(1);
        let mut swaps_made = Vec::new();

        loop {
            // Move left pointer forward until an invalid entry is found
            while left <= right && should_frontload(&self.entity_data_at(left)) {
                left += 1;
            }

            // Move right pointer backward until a valid entry is found
            while left <= right && !should_frontload(&self.entity_data_at(right)) {
                right = right.wrapping_sub(1);
            }

            if left >= right {
                break;
            }

            // Swap entities and record swapped indices
            self.swap_entities(left, right);
            swaps_made.push((left, self.entities[left]));
            swaps_made.push((right, self.entities[right]));

            left += 1;
            right = right.wrapping_sub(1);
        }

        // Only update indices for swapped entities
        for (index, entity_id) in swaps_made {
            let reference = EntityArchetypeRef {
                archetype: self.id(),
                index
            };
            entity_lookup.insert(entity_id, reference);
        }

        // At this point, `left` indicates how many entities are valid (frontloaded)
        left
    }

    #[inline(always)]
    pub fn entity_data_at(&self, idx: usize) -> CubeEntityData {
        CubeEntityData {
            position: self.positions[idx].clone(),
            velocity: self.velocities[idx].clone(),
            transform: self.transforms[idx].clone(),
            model: self.models[idx].clone(),
        }
    }

    #[inline(always)]
    pub fn swap_entities(&mut self, idx_a: usize, idx_b: usize) {
        self.entities.swap(idx_a, idx_b);
        self.positions.swap(idx_a, idx_b);
        self.velocities.swap(idx_a, idx_b);
        self.transforms.swap(idx_a, idx_b);
        self.models.swap(idx_a, idx_b);
    }
}
