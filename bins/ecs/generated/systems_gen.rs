/// The ID of a [`System`].
#[derive(Debug, Copy, Clone, Ord, PartialOrd, Eq, PartialEq)]
pub struct SystemId(core::num::NonZeroU64);

impl SystemId {
    /// Returns this ID as a [`NonZeroU64`](core::num::NonZeroU64) value.
    pub const fn as_nonzero_u64(&self) -> core::num::NonZeroU64 {
        self.0
    }

    /// Returns this ID as a `u64` value.
    pub const fn as_u64(&self) -> u64 {
        self.0.get()
    }
}

impl core::hash::Hash for SystemId {
    fn hash<H: core::hash::Hasher>(&self, state: &mut H) {
        self.0.hash(state);
    }
}

impl From<SystemId> for core::num::NonZeroU64 {
    fn from(value: SystemId) -> core::num::NonZeroU64 {
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
    ///
    /// This phase is executed exactly once per frame.
    Startup = 0,
    /// The `FixedUpdate` system phase.
    ///
    /// This phase runs using a **fixed-time loop** of approximately 60.0 Hz (i.e., every 0.0167 seconds) and may iterate zero or multiple times per frame.
    FixedUpdate = 1,
    /// The `Update` system phase.
    ///
    /// This phase is executed exactly once per frame.
    Update = 2,
    /// The `Render` system phase.
    ///
    /// This phase is executed exactly once per frame.
    Render = 3,
}

#[allow(dead_code)]
impl SystemPhase {
    /// The number of system phases.
    pub const COUNT: usize = 4;

    /// The number of seconds the [`FixedUpdate`](SystemPhase::FixedUpdate) fixed-time step should run.
    pub const FIXED_UPDATE_SECS: f32 = 0.01666666753590107;

    /// The frequency of the [`FixedUpdate`](SystemPhase::FixedUpdate) fixed-time step.
    pub const FIXED_UPDATE_HZ: f32 = 60.0;

    /// Returns the number of seconds between the fixed time steps if this phase
    /// is configured to run in fixed time, otherwise [`None`].
    pub const fn fixed_secs(&self) -> Option<f32> {
        match self {
            Self::Startup => None,
            Self::FixedUpdate => Some(Self::FIXED_UPDATE_SECS),
            Self::Update => None,
            Self::Render => None,
        }
    }

    /// Returns frequency this phase should run if this phase
    /// is configured to run in fixed time, otherwise [`None`].
    pub const fn fixed_hz(&self) -> Option<f32> {
        match self {
            Self::Startup => None,
            Self::FixedUpdate => Some(Self::FIXED_UPDATE_HZ),
            Self::Update => None,
            Self::Render => None,
        }
    }
}

/// Trait for constructing system instances.
pub trait CreateSystems {

    /// Create an instance of type [`PhysicsSystem`].
    fn create_physics(&self) -> PhysicsSystem;

    /// Create an instance of type [`RenderSystem`].
    fn create_render(&self) -> RenderSystem;
}

/// Trait for constructing system instances.
pub trait SystemPhaseEvents<S>
where
    S: WorldState
{
    /// Marks the beginning of a phase.
    #[inline(always)]
    #[allow(unused_variables)]
    fn on_begin_phase(&mut self, phase: SystemPhase, context: &FrameContext, state: &mut S) {
        // empty placeholder implementation
    }

    /// Marks the end of a phase.
    #[inline(always)]
    #[allow(unused_variables)]
    fn on_end_phase(&mut self, phase: SystemPhase, context: &FrameContext, state: &mut S) {
        // empty placeholder implementation
    }
}

/// A no-op event struct.
pub struct NoOpPhaseEvents<S> {
    _phantom: core::marker::PhantomData<S>
}

#[automatically_derived]
impl<S> SystemPhaseEvents<S> for NoOpPhaseEvents<S>
where
    S: WorldState
{
    #[inline(always)]
    fn on_begin_phase(&mut self, _phase: SystemPhase, _context: &FrameContext, _state: &mut S) {
        // empty placeholder implementation
    }

    #[inline(always)]
    fn on_end_phase(&mut self, _phase: SystemPhase, _context: &FrameContext, _state: &mut S) {
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
        commands: &impl WorldCommandSender
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
        commands: &impl WorldCommandSender
    ) {
        let count = velocities.len();
        debug_assert_eq!(entities.len(), count);
        let zipped_iter = entities.iter().zip(velocities.iter().zip(positions.iter_mut()));
        for (entity, (velocity, position)) in zipped_iter {
            self.apply_single(
                &context,
                *entity,
                velocity,
                position,
                commands
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
        commands: &impl WorldCommandSender
    ) {
        ApplyPhysicsSystem::apply_single(
            self,
            context,
            entity,
            velocity,
            position,
            commands
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
        commands: &impl WorldCommandSender
    ) {
        ApplyPhysicsSystem::apply_many(
            self,
            context,
            entities,
            velocities,
            positions,
            commands
        );
    }

    /// Applies this system to the [`ParticleArchetype`].
    #[inline]
    #[deprecated]
    fn apply_to_particle(
        &self,
        archetype: &mut ParticleArchetype,
        context: &FrameContext,
        commands: &impl WorldCommandSender
    ) {
        self.apply_many(
            &context,
            &archetype.entities,
            &archetype.velocities,
            &mut archetype.positions,
            commands
        );
    }

    /// Applies this system to the [`PlayerArchetype`].
    #[inline]
    #[deprecated]
    fn apply_to_player(
        &self,
        archetype: &mut PlayerArchetype,
        context: &FrameContext,
        commands: &impl WorldCommandSender
    ) {
        self.apply_many(
            &context,
            &archetype.entities,
            &archetype.velocities,
            &mut archetype.positions,
            commands
        );
    }
}

#[automatically_derived]
impl System for PhysicsSystem {
    const ID: SystemId = SystemId(core::num::NonZeroU64::new(1).expect("A zero system ID was provided"));
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
impl core::ops::Deref for PhysicsSystem {
    type Target = PhysicsSystemData;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

#[automatically_derived]
impl core::ops::DerefMut for PhysicsSystem {
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
        context: &FrameContext,
        render: &RenderState,
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
        context: &FrameContext,
        render: &RenderState,
        positions: &[PositionComponent],
    ) {
        let zipped_iter = positions;
        for position in zipped_iter {
            self.apply_single(
                &context,
                render,
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
        context: &FrameContext,
        render: &RenderState,
        position: &PositionComponent,
    ) {
        ApplyRenderSystem::apply_single(
            self,
            context,
            render,
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
        context: &FrameContext,
        render: &RenderState,
        positions: &[PositionComponent],
    ) {
        ApplyRenderSystem::apply_many(
            self,
            context,
            render,
            positions,
        );
    }

    /// Applies this system to the [`ParticleArchetype`].
    #[inline]
    #[deprecated]
    fn apply_to_particle(
        &self,
        archetype: &mut ParticleArchetype,
        context: &FrameContext,
        render: &RenderState,
    ) {
        self.apply_many(
            &context,
            render,
            &archetype.positions,
        );
    }

    /// Applies this system to the [`PlayerArchetype`].
    #[inline]
    #[deprecated]
    fn apply_to_player(
        &self,
        archetype: &mut PlayerArchetype,
        context: &FrameContext,
        render: &RenderState,
    ) {
        self.apply_many(
            &context,
            render,
            &archetype.positions,
        );
    }

    /// Applies this system to the [`ForegroundObjectArchetype`].
    #[inline]
    #[deprecated]
    fn apply_to_foreground_object(
        &self,
        archetype: &mut ForegroundObjectArchetype,
        context: &FrameContext,
        render: &RenderState,
    ) {
        self.apply_many(
            &context,
            render,
            &archetype.positions,
        );
    }

    /// Applies this system to the [`BackgroundObjectArchetype`].
    #[inline]
    #[deprecated]
    fn apply_to_background_object(
        &self,
        archetype: &mut BackgroundObjectArchetype,
        context: &FrameContext,
        render: &RenderState,
    ) {
        self.apply_many(
            &context,
            render,
            &archetype.positions,
        );
    }
}

#[automatically_derived]
impl System for RenderSystem {
    const ID: SystemId = SystemId(core::num::NonZeroU64::new(2).expect("A zero system ID was provided"));
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
impl core::ops::Deref for RenderSystem {
    type Target = RenderSystemData;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

#[automatically_derived]
impl core::ops::DerefMut for RenderSystem {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}