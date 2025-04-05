/// The ID of a world.
#[derive(Debug, Copy, Clone, Ord, PartialOrd, Eq, PartialEq)]
pub struct WorldId(core::num::NonZeroU64);

#[allow(dead_code)]
impl WorldId {
    /// Returns a new, unique world ID.
    ///
    /// Uniqueness is guaranteed by using a monotonically increasing `AtomicU64` counter
    /// for generating IDs, starting from 1.
    ///
    /// # Implementation
    /// This function uses a thread-safe counter with sequential consistency ordering
    /// to ensure unique IDs even under concurrent access.
    pub fn new() -> Self {
        static WORLD_IDS: core::sync::atomic::AtomicU64 = core::sync::atomic::AtomicU64::new(1);
        let id = WORLD_IDS.fetch_add(1, core::sync::atomic::Ordering::SeqCst);
        WorldId(core::num::NonZeroU64::new(id).expect("ID was zero"))
    }

    /// Returns this ID as a [`NonZeroU64`](core::num::NonZeroU64) value.
    pub const fn as_nonzero_u64(&self) -> core::num::NonZeroU64 {
        self.0
    }

    /// Returns this ID as a `u64` value.
    pub const fn as_u64(&self) -> u64 {
        self.0.get()
    }
}

impl core::hash::Hash for WorldId {
    fn hash<H: core::hash::Hasher>(&self, state: &mut H) {
        self.0.hash(state);
    }
}

impl From<WorldId> for core::num::NonZeroU64 {
    fn from(value: WorldId) -> core::num::NonZeroU64 {
        value.as_nonzero_u64()
    }
}

impl From<WorldId> for u64 {
    fn from(value: WorldId) -> u64 {
        value.as_u64()
    }
}

/// A command in the [`World`].
#[allow(dead_code)]
pub enum WorldCommand {
    /// Spawn an entity.
    ///
    /// # Note:
    /// Due to the reusability of systems, not all commands are valid in all worlds.
    SpawnEntity(ArchetypeEntityData),
    /// Despawn an entity.
    DespawnEntity(EntityId),
    // TODO: Allow for user-specified commands.
}

/// Sender of [`WorldCommand`] instances.
#[allow(dead_code)]
pub trait WorldCommandSender {
    type Error: core::error::Error;

    /// Sends a command. May block until the send is complete.
    fn send(&self, command: WorldCommand) -> Result<(), Self::Error>;
}

/// Sender of [`WorldCommand`] instances.
#[allow(dead_code)]
pub trait WorldCommandReceiver {
    type Error: core::error::Error;

    /// Attempts to receive a message from the channel without blocking.
    /// Returns [`Ok(None)`] if no command was queued.
    fn recv(&self) -> Result<Option<WorldCommand>, Self::Error>;
}

/// A sender and receiver of [`WorldCommand`] instances.
#[allow(dead_code)]
pub trait WorldCommandQueue: WorldCommandSender + WorldCommandReceiver { }

impl<T> WorldCommandQueue for T
where
    T: WorldCommandSender + WorldCommandReceiver
{ }

/// A frame context.
#[derive(Debug, Clone)]
#[allow(dead_code)]
pub struct FrameContext {
    /// The world ID.
    pub world_id: WorldId,
    /// The frame number.
    pub frame_number: u64,
    /// The delta time since the last frame.
    pub delta_time_secs: f32,
    /// The fixed time for fixed-time systems. Defaults to 60 Hz (~16.66 ms).
    pub fixed_time_secs: f32,
    /// The start time of the current frame.
    pub current_frame_start: std::time::Instant,
    /// The start time of the last frame.
    pub last_frame_start: std::time::Instant,
}

#[allow(dead_code)]
impl FrameContext {
    /// Constructs a new frame context.
    fn new(world_id: WorldId) -> Self {
        Self {
            world_id,
            frame_number: 0,
            delta_time_secs: 0.0,
            fixed_time_secs: 1.0 / 60.0,
            current_frame_start: std::time::Instant::now(),
            last_frame_start: std::time::Instant::now(),
        }
    }
}

/// Marker trait for worlds.
#[allow(dead_code)]
pub trait World {
    /// The ID of this world.
    const ID: WorldId;
}

/// Marker trait for world states.
pub trait WorldState { }


/// A world holding all archetypes.
#[derive(Debug, Clone)]
pub struct MainWorld<E, Q> {
    /// The archetypes.
    archetypes: MainWorldArchetypes,
    /// The systems.
    systems: MainWorldSystems,
    /// The entity locator.
    entity_locations: std::collections::HashMap<EntityId, EntityArchetypeRef>,
    /// The frame context.
    context: FrameContext,
    /// The fixed-time accumulators.
    fixed_accumulators: FixedAccumulators,
    /// The system phase events
    events: Option<E>,
    /// User-specific data.
    pub states: MainWorldStates,
    command_queue: Q
}

impl<E, Q> World for MainWorld<E, Q> {
    const ID: WorldId = WorldId(core::num::NonZeroU64::new(1).expect("Invalid ID on ECS construction time"));
}

/// The user states used in the world.
#[derive(Debug, Clone, Default)]
pub struct MainWorldStates {
    /// The render state
    ///
    /// The state is used in the following systems:
    /// - [`RenderSystem`]
    pub render: RenderState,
}

impl WorldState for MainWorldStates {
}

/// The archetypes used in the world.
#[derive(Debug, Clone, Default)]
struct MainWorldArchetypes {
    pub particle: ParticleArchetype,
    pub player: PlayerArchetype,
    pub foreground_object: ForegroundObjectArchetype,
    pub background_object: BackgroundObjectArchetype,
}

/// The systems.
#[derive(Debug, Clone)]
struct MainWorldSystems {
    pub physics: PhysicsSystem,
    pub render: RenderSystem,
}

/// Spawns an entity into the world.
impl<E, Q> Spawn<ParticleEntityData> for MainWorld<E, Q> {
    /// Spawn a new entity into the world.
    #[inline]
    fn spawn(&mut self, data: ParticleEntityData) -> EntityId {
        self.spawn_particle(data)
    }
}

/// Spawns an entity into the world.
impl<E, Q> Spawn<PlayerEntityData> for MainWorld<E, Q> {
    /// Spawn a new entity into the world.
    #[inline]
    fn spawn(&mut self, data: PlayerEntityData) -> EntityId {
        self.spawn_player(data)
    }
}

/// Spawns an entity into the world.
impl<E, Q> Spawn<ForegroundObjectEntityData> for MainWorld<E, Q> {
    /// Spawn a new entity into the world.
    #[inline]
    fn spawn(&mut self, data: ForegroundObjectEntityData) -> EntityId {
        self.spawn_foreground_object(data)
    }
}

/// Spawns an entity into the world.
impl<E, Q> Spawn<BackgroundObjectEntityData> for MainWorld<E, Q> {
    /// Spawn a new entity into the world.
    #[inline]
    fn spawn(&mut self, data: BackgroundObjectEntityData) -> EntityId {
        self.spawn_background_object(data)
    }
}

/// Helper trait to prevent accidental abuse of the Archetype's spawning function.
pub trait WorldEntityRegistry {
    /// Registers an entity with the world.
    fn register(&mut self, id: EntityId, archetype: EntityArchetypeRef) -> EntityId;
}

#[allow(dead_code)]
impl<E, Q>MainWorld<E, Q> {
    /// Creates a new [`MainWorld`].
    pub fn new<S>(
        create_systems: &S,
        render: RenderState,
        phase_events: Option<E>,
        command_queue: Q) -> Self
    where
        S: CreateSystems,
        E: SystemPhaseEvents<MainWorldStates>,
        Q: WorldCommandQueue
    {
        let context = FrameContext::new(Self::ID);
        Self {
            archetypes: Default::default(),
            systems: MainWorldSystems {
                physics: create_systems.create_physics(),
                render: create_systems.create_render(),
            },
            entity_locations: Default::default(),
            context,
            states: MainWorldStates {
                render,
            },
            fixed_accumulators: Default::default(),
            events: phase_events,
            command_queue
        }
    }

    /// Spawn a new `Particle` entity into the world given its [`ParticleEntityData`].
    #[inline]
    pub fn spawn_particle(
        &mut self,
        particle: ParticleEntityData
    ) -> EntityId {
        self.spawn_particle_with(
            particle.position,
            particle.velocity,
        )
    }

    /// Spawn a new `Particle` entity into the world.
    pub fn spawn_particle_with(
        &mut self,
        position: PositionComponent,
        velocity: VelocityComponent,
    ) -> EntityId {
        struct Registry<'a>(&'a mut std::collections::HashMap<EntityId, EntityArchetypeRef>);

        impl WorldEntityRegistry for Registry<'_> {
            #[inline(always)]
            fn register(&mut self, id: EntityId, archetype: EntityArchetypeRef) -> EntityId {
                self.0.insert(id, archetype);
                id
            }
        }

        self.archetypes.particle
            .spawn_with(
                position,
                velocity,
                Registry(&mut self.entity_locations)
            )
    }

    /// Spawn a new `Player` entity into the world given its [`PlayerEntityData`].
    #[inline]
    pub fn spawn_player(
        &mut self,
        player: PlayerEntityData
    ) -> EntityId {
        self.spawn_player_with(
            player.position,
            player.velocity,
            player.health,
        )
    }

    /// Spawn a new `Player` entity into the world.
    pub fn spawn_player_with(
        &mut self,
        position: PositionComponent,
        velocity: VelocityComponent,
        health: HealthComponent,
    ) -> EntityId {
        struct Registry<'a>(&'a mut std::collections::HashMap<EntityId, EntityArchetypeRef>);

        impl WorldEntityRegistry for Registry<'_> {
            #[inline(always)]
            fn register(&mut self, id: EntityId, archetype: EntityArchetypeRef) -> EntityId {
                self.0.insert(id, archetype);
                id
            }
        }

        self.archetypes.player
            .spawn_with(
                position,
                velocity,
                health,
                Registry(&mut self.entity_locations)
            )
    }

    /// Spawn a new `ForegroundObject` entity into the world given its [`ForegroundObjectEntityData`].
    #[inline]
    pub fn spawn_foreground_object(
        &mut self,
        foreground_object: ForegroundObjectEntityData
    ) -> EntityId {
        self.spawn_foreground_object_with(
            foreground_object.position,
            foreground_object.collider,
        )
    }

    /// Spawn a new `ForegroundObject` entity into the world.
    pub fn spawn_foreground_object_with(
        &mut self,
        position: PositionComponent,
        collider: ColliderComponent,
    ) -> EntityId {
        struct Registry<'a>(&'a mut std::collections::HashMap<EntityId, EntityArchetypeRef>);

        impl WorldEntityRegistry for Registry<'_> {
            #[inline(always)]
            fn register(&mut self, id: EntityId, archetype: EntityArchetypeRef) -> EntityId {
                self.0.insert(id, archetype);
                id
            }
        }

        self.archetypes.foreground_object
            .spawn_with(
                position,
                collider,
                Registry(&mut self.entity_locations)
            )
    }

    /// Spawn a new `BackgroundObject` entity into the world given its [`BackgroundObjectEntityData`].
    #[inline]
    pub fn spawn_background_object(
        &mut self,
        background_object: BackgroundObjectEntityData
    ) -> EntityId {
        self.spawn_background_object_with(
            background_object.position,
        )
    }

    /// Spawn a new `BackgroundObject` entity into the world.
    pub fn spawn_background_object_with(
        &mut self,
        position: PositionComponent,
    ) -> EntityId {
        struct Registry<'a>(&'a mut std::collections::HashMap<EntityId, EntityArchetypeRef>);

        impl WorldEntityRegistry for Registry<'_> {
            #[inline(always)]
            fn register(&mut self, id: EntityId, archetype: EntityArchetypeRef) -> EntityId {
                self.0.insert(id, archetype);
                id
            }
        }

        self.archetypes.background_object
            .spawn_with(
                position,
                Registry(&mut self.entity_locations)
            )
    }

    /// Runs a per-frame update of the frame context at the start of a frame.
    fn on_begin_frame(&mut self) {
        self.context.current_frame_start = std::time::Instant::now();
        if self.context.frame_number == 0 {
            self.context.delta_time_secs = 0.0;
        } else {
            self.context.delta_time_secs = (self.context.current_frame_start - self.context.last_frame_start).as_secs_f32();
        }
        self.context.frame_number = self.context.frame_number.wrapping_add(1);
    }

    /// Runs a per-frame update of the frame context at the end of a frame.
    fn on_end_frame(&mut self) {
        self.context.last_frame_start = self.context.current_frame_start;
    }

    /// Apply all system phases sequentially.
    ///
    /// This method executes all system phases in sequence.
    /// The phases are processed in the following order:
    ///
    /// - Startup
    /// - FixedUpdate
    /// - Update
    /// - Render
    /// 
    /// # Type Parameters
    /// - `E`: The type implementing [`SystemPhaseEvents`] for phase lifecycle events
    pub fn apply_system_phases(&mut self)
    where
        E: SystemPhaseEvents<MainWorldStates>,
        Q: WorldCommandSender
    {
        self.on_begin_frame();
        self.apply_system_phase_startup();

        // Apply fixed-time loop for phase FixedUpdate
        self.context.fixed_time_secs = SystemPhase::FIXED_UPDATE_SECS;
        self.fixed_accumulators.fixed_update += self.context.delta_time_secs;
        while self.fixed_accumulators.fixed_update >= SystemPhase::FIXED_UPDATE_SECS {
            self.apply_system_phase_fixed_update();
            self.fixed_accumulators.fixed_update -= SystemPhase::FIXED_UPDATE_SECS;
        }

        self.context.fixed_time_secs = 0.0;
        self.apply_system_phase_update();
        self.apply_system_phase_render();
        self.on_end_frame();
    }

    /// Apply all system phases with thread parallelism.
    ///
    /// This method executes all system phases in sequence, but with parallel execution
    /// of systems within each phase. The phases are processed in the following order:
    ///
    /// - Startup
    /// - FixedUpdate
    /// - Update
    /// - Render
    /// 
    /// # Type Parameters
    /// - `E`: The type implementing [`SystemPhaseEvents`] for phase lifecycle events
    ///
    /// # Parallelism
    /// - Each phase is processed sequentially
    /// - Within each phase, systems are executed in parallel where possible
    /// - Thread parallelism is implemented using the Rayon library
    pub fn par_apply_system_phases(&mut self)
    where
        E: SystemPhaseEvents<MainWorldStates>,
        Q: WorldCommandSender
    {
        self.on_begin_frame();
        self.par_apply_system_phase_startup();

        // Apply fixed-time loop for phase FixedUpdate
        self.context.fixed_time_secs = SystemPhase::FIXED_UPDATE_SECS;
        self.fixed_accumulators.fixed_update += self.context.delta_time_secs;
        while self.fixed_accumulators.fixed_update >= SystemPhase::FIXED_UPDATE_SECS {
            self.par_apply_system_phase_fixed_update();
            self.fixed_accumulators.fixed_update -= SystemPhase::FIXED_UPDATE_SECS;
        }

        self.context.fixed_time_secs = 0.0;
        self.par_apply_system_phase_update();
        self.par_apply_system_phase_render();
        self.on_end_frame();
    }

    /// Execute system phase `Startup`
    ///
    /// This method executes all systems scheduled for the `Startup` phase in sequential order.
    /// Systems are organized into groups that must be executed in sequence, while systems within each
    /// group can potentially be parallelized (see `par_apply_system_phase_startup`).
    ///
    /// # Execution Flow
    /// 1. Triggers the phase start event if event handling is enabled
    /// 2. Sequentially processes each system group
    /// 3. For each system in a group:
    ///    - Applies the system to all matching archetypes
    ///    - Provides read-only access to input components
    ///    - Provides mutable access to output components
    /// 4. Triggers the phase end event if event handling is enabled
    ///
    /// # Type Parameters
    /// - `E`: The type implementing `SystemPhaseEvents` for phase lifecycle events
    fn apply_system_phase_startup(&mut self)
    where
        E: SystemPhaseEvents<MainWorldStates>,
        Q: WorldCommandSender
    {
        if let Some(events) = &mut self.events {
            events.on_begin_phase(SystemPhase::Startup, &self.context, &mut self.states);
        }

        self.apply_system_phase_startup_without_events();

        if let Some(events) = &mut self.events {
            events.on_end_phase(SystemPhase::Startup, &self.context, &mut self.states);
        }
    }

    /// Execute system phase `Startup` without calling events. See [`apply_system_phase_startup`](Self::apply_system_phase_startup).
    #[inline]
    fn apply_system_phase_startup_without_events(&mut self)
    where
        E: SystemPhaseEvents<MainWorldStates>,
        Q: WorldCommandSender
    {
        // no systems in this phase
    }

    /// Applies the specified system phase with parallel execution of systems.
    ///
    /// This method uses Rayon's parallel scoping to execute systems concurrently within each system
    /// group. Systems within a group are executed in parallel while groups themselves are processed
    /// sequentially.
    ///
    /// # Type Parameters
    /// - `E`: The type implementing `SystemPhaseEvents` for phase lifecycle events
    ///
    /// # Notes
    /// - Systems in each group are spawned as parallel tasks
    /// - Each system operates on its affected archetypes concurrently
    /// - Phase events are triggered at the start and end of execution
    fn par_apply_system_phase_startup(&mut self)
    where
        E: SystemPhaseEvents<MainWorldStates>,
        Q: WorldCommandSender
    {
        if let Some(events) = &mut self.events {
            events.on_begin_phase(SystemPhase::Startup, &self.context, &mut self.states);
        }
        // no systems in this phase

        if let Some(events) = &mut self.events {
            events.on_end_phase(SystemPhase::Startup, &self.context, &mut self.states);
        }
    }

    /// Execute system phase `FixedUpdate`
    ///
    /// This method executes all systems scheduled for the `FixedUpdate` phase in sequential order.
    /// Systems are organized into groups that must be executed in sequence, while systems within each
    /// group can potentially be parallelized (see `par_apply_system_phase_fixed_update`).
    ///
    /// # Execution Flow
    /// 1. Triggers the phase start event if event handling is enabled
    /// 2. Sequentially processes each system group
    /// 3. For each system in a group:
    ///    - Applies the system to all matching archetypes
    ///    - Provides read-only access to input components
    ///    - Provides mutable access to output components
    /// 4. Triggers the phase end event if event handling is enabled
    ///
    /// # Type Parameters
    /// - `E`: The type implementing `SystemPhaseEvents` for phase lifecycle events
    fn apply_system_phase_fixed_update(&mut self)
    where
        E: SystemPhaseEvents<MainWorldStates>,
        Q: WorldCommandSender
    {
        if let Some(events) = &mut self.events {
            events.on_begin_phase(SystemPhase::FixedUpdate, &self.context, &mut self.states);
        }

        self.apply_system_phase_fixed_update_without_events();

        if let Some(events) = &mut self.events {
            events.on_end_phase(SystemPhase::FixedUpdate, &self.context, &mut self.states);
        }
    }

    /// Execute system phase `FixedUpdate` without calling events. See [`apply_system_phase_fixed_update`](Self::apply_system_phase_fixed_update).
    #[inline]
    fn apply_system_phase_fixed_update_without_events(&mut self)
    where
        E: SystemPhaseEvents<MainWorldStates>,
        Q: WorldCommandSender
    {
        // TODO: Test if PhysicsSystem is part of the MainWorld.
        // System group 1 of 1
        {
            // Apply PhysicsSystem to ParticleArchetype
            self.systems.physics.apply_many(
                &self.context,
                &self.archetypes.particle.entities,
                &self.archetypes.particle.velocities,
                &mut self.archetypes.particle.positions,
                &self.command_queue
            );
            // Apply PhysicsSystem to PlayerArchetype
            self.systems.physics.apply_many(
                &self.context,
                &self.archetypes.player.entities,
                &self.archetypes.player.velocities,
                &mut self.archetypes.player.positions,
                &self.command_queue
            );
        }
    }

    /// Applies the specified system phase with parallel execution of systems.
    ///
    /// This method uses Rayon's parallel scoping to execute systems concurrently within each system
    /// group. Systems within a group are executed in parallel while groups themselves are processed
    /// sequentially.
    ///
    /// # Type Parameters
    /// - `E`: The type implementing `SystemPhaseEvents` for phase lifecycle events
    ///
    /// # Notes
    /// - Systems in each group are spawned as parallel tasks
    /// - Each system operates on its affected archetypes concurrently
    /// - Phase events are triggered at the start and end of execution
    fn par_apply_system_phase_fixed_update(&mut self)
    where
        E: SystemPhaseEvents<MainWorldStates>,
        Q: WorldCommandSender
    {
        if let Some(events) = &mut self.events {
            events.on_begin_phase(SystemPhase::FixedUpdate, &self.context, &mut self.states);
        }
        // Only one system in this phase; skip spawning thread.
        self.apply_system_phase_fixed_update_without_events();

        if let Some(events) = &mut self.events {
            events.on_end_phase(SystemPhase::FixedUpdate, &self.context, &mut self.states);
        }
    }

    /// Execute system phase `Update`
    ///
    /// This method executes all systems scheduled for the `Update` phase in sequential order.
    /// Systems are organized into groups that must be executed in sequence, while systems within each
    /// group can potentially be parallelized (see `par_apply_system_phase_update`).
    ///
    /// # Execution Flow
    /// 1. Triggers the phase start event if event handling is enabled
    /// 2. Sequentially processes each system group
    /// 3. For each system in a group:
    ///    - Applies the system to all matching archetypes
    ///    - Provides read-only access to input components
    ///    - Provides mutable access to output components
    /// 4. Triggers the phase end event if event handling is enabled
    ///
    /// # Type Parameters
    /// - `E`: The type implementing `SystemPhaseEvents` for phase lifecycle events
    fn apply_system_phase_update(&mut self)
    where
        E: SystemPhaseEvents<MainWorldStates>,
        Q: WorldCommandSender
    {
        if let Some(events) = &mut self.events {
            events.on_begin_phase(SystemPhase::Update, &self.context, &mut self.states);
        }

        self.apply_system_phase_update_without_events();

        if let Some(events) = &mut self.events {
            events.on_end_phase(SystemPhase::Update, &self.context, &mut self.states);
        }
    }

    /// Execute system phase `Update` without calling events. See [`apply_system_phase_update`](Self::apply_system_phase_update).
    #[inline]
    fn apply_system_phase_update_without_events(&mut self)
    where
        E: SystemPhaseEvents<MainWorldStates>,
        Q: WorldCommandSender
    {
        // no systems in this phase
    }

    /// Applies the specified system phase with parallel execution of systems.
    ///
    /// This method uses Rayon's parallel scoping to execute systems concurrently within each system
    /// group. Systems within a group are executed in parallel while groups themselves are processed
    /// sequentially.
    ///
    /// # Type Parameters
    /// - `E`: The type implementing `SystemPhaseEvents` for phase lifecycle events
    ///
    /// # Notes
    /// - Systems in each group are spawned as parallel tasks
    /// - Each system operates on its affected archetypes concurrently
    /// - Phase events are triggered at the start and end of execution
    fn par_apply_system_phase_update(&mut self)
    where
        E: SystemPhaseEvents<MainWorldStates>,
        Q: WorldCommandSender
    {
        if let Some(events) = &mut self.events {
            events.on_begin_phase(SystemPhase::Update, &self.context, &mut self.states);
        }
        // no systems in this phase

        if let Some(events) = &mut self.events {
            events.on_end_phase(SystemPhase::Update, &self.context, &mut self.states);
        }
    }

    /// Execute system phase `Render`
    ///
    /// This method executes all systems scheduled for the `Render` phase in sequential order.
    /// Systems are organized into groups that must be executed in sequence, while systems within each
    /// group can potentially be parallelized (see `par_apply_system_phase_render`).
    ///
    /// # Execution Flow
    /// 1. Triggers the phase start event if event handling is enabled
    /// 2. Sequentially processes each system group
    /// 3. For each system in a group:
    ///    - Applies the system to all matching archetypes
    ///    - Provides read-only access to input components
    ///    - Provides mutable access to output components
    /// 4. Triggers the phase end event if event handling is enabled
    ///
    /// # Type Parameters
    /// - `E`: The type implementing `SystemPhaseEvents` for phase lifecycle events
    fn apply_system_phase_render(&mut self)
    where
        E: SystemPhaseEvents<MainWorldStates>,
        Q: WorldCommandSender
    {
        if let Some(events) = &mut self.events {
            events.on_begin_phase(SystemPhase::Render, &self.context, &mut self.states);
        }

        self.apply_system_phase_render_without_events();

        if let Some(events) = &mut self.events {
            events.on_end_phase(SystemPhase::Render, &self.context, &mut self.states);
        }
    }

    /// Execute system phase `Render` without calling events. See [`apply_system_phase_render`](Self::apply_system_phase_render).
    #[inline]
    fn apply_system_phase_render_without_events(&mut self)
    where
        E: SystemPhaseEvents<MainWorldStates>,
        Q: WorldCommandSender
    {
        // TODO: Test if RenderSystem is part of the MainWorld.
        // System group 1 of 1
        {
            // Apply RenderSystem to ParticleArchetype
            self.systems.render.apply_many(
                &self.context,
                &self.states.render,
                &self.archetypes.particle.positions,
            );
            // Apply RenderSystem to PlayerArchetype
            self.systems.render.apply_many(
                &self.context,
                &self.states.render,
                &self.archetypes.player.positions,
            );
            // Apply RenderSystem to ForegroundObjectArchetype
            self.systems.render.apply_many(
                &self.context,
                &self.states.render,
                &self.archetypes.foreground_object.positions,
            );
            // Apply RenderSystem to BackgroundObjectArchetype
            self.systems.render.apply_many(
                &self.context,
                &self.states.render,
                &self.archetypes.background_object.positions,
            );
        }
    }

    /// Applies the specified system phase with parallel execution of systems.
    ///
    /// This method uses Rayon's parallel scoping to execute systems concurrently within each system
    /// group. Systems within a group are executed in parallel while groups themselves are processed
    /// sequentially.
    ///
    /// # Type Parameters
    /// - `E`: The type implementing `SystemPhaseEvents` for phase lifecycle events
    ///
    /// # Notes
    /// - Systems in each group are spawned as parallel tasks
    /// - Each system operates on its affected archetypes concurrently
    /// - Phase events are triggered at the start and end of execution
    fn par_apply_system_phase_render(&mut self)
    where
        E: SystemPhaseEvents<MainWorldStates>,
        Q: WorldCommandSender
    {
        if let Some(events) = &mut self.events {
            events.on_begin_phase(SystemPhase::Render, &self.context, &mut self.states);
        }
        // Only one system in this phase; skip spawning thread.
        self.apply_system_phase_render_without_events();

        if let Some(events) = &mut self.events {
            events.on_end_phase(SystemPhase::Render, &self.context, &mut self.states);
        }
    }

    /// Handles all queued commands.
    fn handle_commands(&mut self)
    where
        Q: WorldCommandReceiver
    {
        loop {
            match self.command_queue.recv() {
                Ok(Some(cmd)) => {
                    match cmd {
                        WorldCommand::SpawnEntity(data) => self.handle_spawn_command(data),
                        WorldCommand::DespawnEntity(id) => self.handle_despawn_command(id).expect("Failed to despawn"),
                    }
                }
                Ok(None) => break,
                Err(error) => {
                    // TODO: Make logging configurable.
                    tracing::error!(?error, "Failed to dequeue world command");
                    break;
                }
            }
        }
    }

    fn handle_spawn_command(&mut self, data: ArchetypeEntityData) {
        match data {
            // Accept only archetypes known to the world
            ArchetypeEntityData::Particle(data) => {
                let id = self.spawn_particle(data);
                tracing::trace!(%id, "Spawned Particle entity");
            },
            ArchetypeEntityData::Player(data) => {
                let id = self.spawn_player(data);
                tracing::trace!(%id, "Spawned Player entity");
            },
            ArchetypeEntityData::ForegroundObject(data) => {
                let id = self.spawn_foreground_object(data);
                tracing::trace!(%id, "Spawned ForegroundObject entity");
            },
            ArchetypeEntityData::BackgroundObject(data) => {
                let id = self.spawn_background_object(data);
                tracing::trace!(%id, "Spawned BackgroundObject entity");
            },
            // TODO: Make it a non-panic on release builds?
            #[allow(unreachable_patterns)]
            unhandled => panic!("Cannot spawn entity for {id} which is not known to this world.", id = unhandled.archetype_id())
        }
    }

    fn handle_despawn_command(&mut self, id: EntityId) -> Result<(), DespawnError> {
         if let Some(loc) = self.entity_locations.remove(&id) {
            match loc.archetype {
                ParticleArchetype::ID => {
                    self.archetypes
                        .particle
                        .drop_at_index(loc.index)
                        .map_err(|index| DespawnError::InvalidIndexInArchetype(index, ParticleArchetype::ID))?;
                }
                PlayerArchetype::ID => {
                    self.archetypes
                        .player
                        .drop_at_index(loc.index)
                        .map_err(|index| DespawnError::InvalidIndexInArchetype(index, PlayerArchetype::ID))?;
                }
                ForegroundObjectArchetype::ID => {
                    self.archetypes
                        .foreground_object
                        .drop_at_index(loc.index)
                        .map_err(|index| DespawnError::InvalidIndexInArchetype(index, ForegroundObjectArchetype::ID))?;
                }
                BackgroundObjectArchetype::ID => {
                    self.archetypes
                        .background_object
                        .drop_at_index(loc.index)
                        .map_err(|index| DespawnError::InvalidIndexInArchetype(index, BackgroundObjectArchetype::ID))?;
                }
                // TODO: Make it a non-panic on release builds?
                #[allow(unreachable_patterns)]
                archetype_id => unreachable!("Cannot remove entity {id} from archetype {archetype_id} which is not known to this world.")
            }
         } else {
            tracing::error!(%id, "Attempted to remove unknown entity");
         }
         Ok(())
    }
}

/// Time accumulators for fixed-time phases.
#[derive(Debug, Clone, Default)]
struct FixedAccumulators {
    /// The time accumulator for the [`FixedUpdate`](SystemPhase::FixedUpdate) fixed-time step.
    fixed_update: f32,
}

#[derive(Debug)]
pub enum DespawnError {
    InvalidIndexInArchetype(usize, ArchetypeId)
}

impl core::fmt::Display for DespawnError {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> Result<(), core::fmt::Error> {
        match self {
            Self::InvalidIndexInArchetype(index, aid) => {
                write!(f, "Failed to despawn entity at invalid index {index} in archetype {aid}")
            }
        }
    }
}

impl core::error::Error for DespawnError { }