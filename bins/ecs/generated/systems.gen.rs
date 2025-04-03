/// The ID of a [`System`].
#[derive(Debug, Copy, Clone, Ord, PartialOrd, Eq, PartialEq)]
pub struct SystemId(std::num::NonZeroU64);

impl SystemId {
    /// Returns this ID as a [`NonZeroU64`](std::num::NonZeroU64) value.
    pub const fn as_nonzero_u64(&self) -> std::num::NonZeroU64 {
        self.0
    }

    /// Returns this ID as a `u64` value.
    pub const fn as_u64(&self) -> u64 {
            self.0.get()
    }
}

impl std::hash::Hash for SystemId {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.0.hash(state);
    }
}

impl From<SystemId> for std::num::NonZeroU64 {
    fn from(value: SystemId) -> std::num::NonZeroU64 {
        value.as_nonzero_u64()
    }
}

impl From<SystemId> for u64 {
    fn from(value: SystemId) -> u64 {
        value.as_u64()
    }
}

/// Marker trait for systems.
pub trait System: 'static + Send + Sync {
    /// The ID of this system.
    const ID: SystemId;
}

/// The system phases.
#[derive(Debug, Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash)]
#[allow(dead_code)]
pub enum SystemPhase {
    /// The `Startup` system phase.
    Startup = 0,
    /// The `FixedUpdate` system phase.
    FixedUpdate = 1,
    /// The `Update` system phase.
    Update = 2,
    /// The `Render` system phase.
    Render = 3,
}

#[allow(dead_code)]
impl SystemPhase {
    /// The number of system phases.
    pub const COUNT: usize = 4;
}

/// Trait for constructing system instances.
pub trait CreateSystems {

    /// Create an instance of type [`PhysicsSystem`].
    fn create_physics(&self) -> PhysicsSystem;

    /// Create an instance of type [`RenderSystem`].
    fn create_render(&self) -> RenderSystem;
}

/// Trait for constructing system instances.
pub trait SystemPhaseEvents {
    /// Marks the beginning of a phase.
    #[inline(always)]
    #[allow(unused_variables)]
    fn on_begin_phase(&mut self, phase: SystemPhase) {
        // empty placeholder implementation
    }

    /// Marks the end of a phase.
    #[inline(always)]
    #[allow(unused_variables)]
    fn on_end_phase(&mut self, phase: SystemPhase) {
        // empty placeholder implementation
    }
}

/// An alias type for a no-op implementation of [`SystemPhaseEvents`].
#[allow(dead_code)]
pub type NoOpPhaseEvents = ();

/// A constant for passing no system type events.
#[allow(dead_code)]
const NO_PHASE_EVENTS: Option<NoOpPhaseEvents> = Option::<NoOpPhaseEvents>::None;

#[automatically_derived]
impl SystemPhaseEvents for NoOpPhaseEvents {
    #[inline(always)]
    fn on_begin_phase(&mut self, _phase: SystemPhase) {
        // empty placeholder implementation
    }

    #[inline(always)]
    fn on_end_phase(&mut self, _phase: SystemPhase) {
        // empty placeholder implementation
    }
}

/// A system operating on multiple [`Component`]s.
///
/// The business logic is implemented via the [`ApplyPhysicsSystem`] trait.
/// See also [`PhysicsData`] for the systems-specific data.
///
/// ### Reads
/// 
/// - [`VelocityComponent`]
///
/// ### Mutates
/// 
/// - [`PositionComponent`]
#[derive(Debug, Clone)]
pub struct PhysicsSystem(PhysicsSystemData);

/// Implements the business logic for the [`PhysicsSystem`] system.
#[allow(dead_code)]
pub trait ApplyPhysicsSystem: System {
    /// Applies the system's business logic to the given entity components.
    ///
    /// ### Reads
    /// 
    /// - `velocity`: A reference to the input component of type [`VelocityComponent`].
    ///
    /// ### Mutates
    /// 
    /// - `position`: A mutable reference to the component of type [`PositionComponent`].
    fn apply_single(
        &self,
        context: &FrameContext,
        entity: EntityId,
        velocity: &VelocityComponent,
        position: &mut PositionComponent,
    );

    /// Applies the system's business logic to the given entity components.
    ///
    /// ### Reads
    /// 
    /// - `velocity`: A slice of the input components of type [`VelocityComponent`].
    ///
    /// ### Mutates
    /// 
    /// - `position`: A mutable slice of the components of type [`PositionComponent`].
    fn apply_many(
        &self,
        context: &FrameContext,
        entities: &[EntityId],
        velocities: &[VelocityComponent],
        positions: &mut [PositionComponent],
    ) {
        let iter = entities.iter().zip(velocities.iter().zip(positions.iter_mut()));
        for (entity, (velocity, position)) in iter {
            self.apply_single(
                &context,
                *entity,
                velocity,
                position,
            );
        }
    }
}

#[allow(dead_code)]
impl PhysicsSystem {
    /// Apply the system to the given entity components
    #[inline]
    pub fn apply_single(
        &self,
        context: &FrameContext,
        entity: EntityId,
        velocity: &VelocityComponent,
        position: &mut PositionComponent,
    ) {
        ApplyPhysicsSystem::apply_single(
            self,
            context,
            entity,
            velocity,
            position,
        );
    }

    /// Applies the system's business logic to the given entity components.
    ///
    /// ### Reads
    /// 
    /// - `velocity`: A slice of the input components of type [`VelocityComponent`].
    ///
    /// ### Mutates
    /// 
    /// - `position`: A mutable slice of the components of type [`PositionComponent`].
    #[inline]
    fn apply_many(
        &self,
        context: &FrameContext,
        entities: &[EntityId],
        velocities: &[VelocityComponent],
        positions: &mut [PositionComponent],
    ) {
        ApplyPhysicsSystem::apply_many(
            self,
            context,
            entities,
            velocities,
            positions,
        );
    }

    /// Applies this system to the [`ParticleArchetype`].
    #[inline]
    pub fn apply_to_particle(
        &self,
        archetype: &mut ParticleArchetype,
        context: &FrameContext,
    ) {
        self.apply_many(
            &context,
            &archetype.entities,
            &archetype.velocities,
            &mut archetype.positions,
        );
    }

    /// Applies this system to the [`PlayerArchetype`].
    #[inline]
    pub fn apply_to_player(
        &self,
        archetype: &mut PlayerArchetype,
        context: &FrameContext,
    ) {
        self.apply_many(
            &context,
            &archetype.entities,
            &archetype.velocities,
            &mut archetype.positions,
        );
    }
}

#[automatically_derived]
impl System for PhysicsSystem {
    const ID: SystemId = SystemId(std::num::NonZeroU64::new(1).expect("A zero system ID was provided"));
}

impl PartialEq<SystemId> for PhysicsSystem {
    fn eq(&self, other: &SystemId) -> bool {
        Self::ID.eq(other)
    }
}

#[automatically_derived]
impl From<PhysicsSystemData> for PhysicsSystem {
    fn from(data: PhysicsSystemData) -> Self {
        Self(data)
    }
}

#[automatically_derived]
impl std::ops::Deref for PhysicsSystem {
    type Target = PhysicsSystemData;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

#[automatically_derived]
impl std::ops::DerefMut for PhysicsSystem {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

/// A system operating on multiple [`Component`]s.
///
/// The business logic is implemented via the [`ApplyRenderSystem`] trait.
/// See also [`RenderData`] for the systems-specific data.
///
/// ### Reads
/// 
/// - [`PositionComponent`]
///
/// ### Mutates
/// 
#[derive(Debug, Clone)]
pub struct RenderSystem(RenderSystemData);

/// Implements the business logic for the [`RenderSystem`] system.
#[allow(dead_code)]
pub trait ApplyRenderSystem: System {
    /// Applies the system's business logic to the given entity components.
    ///
    /// ### Reads
    /// 
    /// - `position`: A reference to the input component of type [`PositionComponent`].
    ///
    /// ### Mutates
    /// 
    fn apply_single(
        &self,
        position: &PositionComponent,
    );

    /// Applies the system's business logic to the given entity components.
    ///
    /// ### Reads
    /// 
    /// - `position`: A slice of the input components of type [`PositionComponent`].
    ///
    /// ### Mutates
    /// 
    fn apply_many(
        &self,
        positions: &[PositionComponent],
    ) {
        let iter = positions;
        for position in iter {
            self.apply_single(
                position,
            );
        }
    }
}

#[allow(dead_code)]
impl RenderSystem {
    /// Apply the system to the given entity components
    #[inline]
    pub fn apply_single(
        &self,
        position: &PositionComponent,
    ) {
        ApplyRenderSystem::apply_single(
            self,
            position,
        );
    }

    /// Applies the system's business logic to the given entity components.
    ///
    /// ### Reads
    /// 
    /// - `position`: A slice of the input components of type [`PositionComponent`].
    ///
    /// ### Mutates
    /// 
    #[inline]
    fn apply_many(
        &self,
        positions: &[PositionComponent],
    ) {
        ApplyRenderSystem::apply_many(
            self,
            positions,
        );
    }

    /// Applies this system to the [`ParticleArchetype`].
    #[inline]
    pub fn apply_to_particle(
        &self,
        archetype: &mut ParticleArchetype,
    ) {
        self.apply_many(
            &archetype.positions,
        );
    }

    /// Applies this system to the [`PlayerArchetype`].
    #[inline]
    pub fn apply_to_player(
        &self,
        archetype: &mut PlayerArchetype,
    ) {
        self.apply_many(
            &archetype.positions,
        );
    }

    /// Applies this system to the [`ForegroundObjectArchetype`].
    #[inline]
    pub fn apply_to_foreground_object(
        &self,
        archetype: &mut ForegroundObjectArchetype,
    ) {
        self.apply_many(
            &archetype.positions,
        );
    }

    /// Applies this system to the [`BackgroundObjectArchetype`].
    #[inline]
    pub fn apply_to_background_object(
        &self,
        archetype: &mut BackgroundObjectArchetype,
    ) {
        self.apply_many(
            &archetype.positions,
        );
    }
}

#[automatically_derived]
impl System for RenderSystem {
    const ID: SystemId = SystemId(std::num::NonZeroU64::new(2).expect("A zero system ID was provided"));
}

impl PartialEq<SystemId> for RenderSystem {
    fn eq(&self, other: &SystemId) -> bool {
        Self::ID.eq(other)
    }
}

#[automatically_derived]
impl From<RenderSystemData> for RenderSystem {
    fn from(data: RenderSystemData) -> Self {
        Self(data)
    }
}

#[automatically_derived]
impl std::ops::Deref for RenderSystem {
    type Target = RenderSystemData;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

#[automatically_derived]
impl std::ops::DerefMut for RenderSystem {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}