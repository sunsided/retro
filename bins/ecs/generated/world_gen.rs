/// A frame context.
#[derive(Debug, Clone)]
#[allow(dead_code)]
pub struct FrameContext {
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
impl Default for FrameContext {
    fn default() -> Self {
        Self {
            frame_number: 0,
            delta_time_secs: 0.0,
            fixed_time_secs: 1.0 / 60.0,
            current_frame_start: std::time::Instant::now(),
            last_frame_start: std::time::Instant::now(),
        }
    }
}

/// A world holding all archetypes.
#[derive(Debug, Clone)]
pub struct World<E = ()> {
    pub particle: ParticleArchetype,
    pub player: PlayerArchetype,
    pub foreground_object: ForegroundObjectArchetype,
    pub background_object: BackgroundObjectArchetype,
    pub physics_system: PhysicsSystem,
    pub render_system: RenderSystem,

    /// The entity locator.
    entity_locations: std::collections::HashMap<EntityId, EntityArchetypeRef>,

    /// The frame context.
    context: FrameContext,

    /// User-specific data.
    pub state: UserState,

    /// The fixed-time accumulators.
    fixed_accumulators: FixedAccumulators,

    /// The system phase events
    events: Option<E>
}

/// Time accumulators for fixed-time phases.
#[derive(Debug, Clone, Default)]
struct FixedAccumulators {
    /// The time accumulator for the [`FixedUpdate`](SystemPhase::FixedUpdate) fixed-time step.
    fixed_update: f32,
}

#[allow(dead_code)]
impl<E> World<E> {
    /// Creates a new [`World`].
    pub fn new<S>(create_systems: &S, state: UserState, phase_events: Option<E>) -> Self
    where
        S: CreateSystems,
        E: SystemPhaseEvents
    {
        let context = FrameContext::default();
        let physics_system = create_systems.create_physics();
        let render_system = create_systems.create_render();
        Self {
            particle: ParticleArchetype::default(),
            player: PlayerArchetype::default(),
            foreground_object: ForegroundObjectArchetype::default(),
            background_object: BackgroundObjectArchetype::default(),
            physics_system,
            render_system,
            entity_locations: Default::default(),
            context,
            state,
            fixed_accumulators: Default::default(),
            events: phase_events
        }
    }

    /// Spawn a new `Particle` entity into the world.
    pub fn spawn_particle_with(
        &mut self,
        position: PositionComponent,
        velocity: VelocityComponent,
    ) -> EntityId {
        let (entity_id, archetype_ref) = self.particle
            .spawn_with(
                position,
                velocity,
            );

        self.entity_locations.insert(entity_id, archetype_ref);
        entity_id
    }

    /// Spawn a new `Player` entity into the world.
    pub fn spawn_player_with(
        &mut self,
        position: PositionComponent,
        velocity: VelocityComponent,
        health: HealthComponent,
    ) -> EntityId {
        let (entity_id, archetype_ref) = self.player
            .spawn_with(
                position,
                velocity,
                health,
            );

        self.entity_locations.insert(entity_id, archetype_ref);
        entity_id
    }

    /// Spawn a new `ForegroundObject` entity into the world.
    pub fn spawn_foreground_object_with(
        &mut self,
        position: PositionComponent,
        collider: ColliderComponent,
    ) -> EntityId {
        let (entity_id, archetype_ref) = self.foreground_object
            .spawn_with(
                position,
                collider,
            );

        self.entity_locations.insert(entity_id, archetype_ref);
        entity_id
    }

    /// Spawn a new `BackgroundObject` entity into the world.
    pub fn spawn_background_object_with(
        &mut self,
        position: PositionComponent,
    ) -> EntityId {
        let (entity_id, archetype_ref) = self.background_object
            .spawn_with(
                position,
            );

        self.entity_locations.insert(entity_id, archetype_ref);
        entity_id
    }

    /// Callback function for an archetype to spawn an entity.
    #[doc(hidden)]
    #[inline]
    pub fn register_entity_from_spawn(&mut self, entity_id: EntityId, archetype_ref: EntityArchetypeRef) {
        self.entity_locations.insert(entity_id, archetype_ref);
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
        E: SystemPhaseEvents
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
        E: SystemPhaseEvents
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
        E: SystemPhaseEvents
    {
        if let Some(events) = &mut self.events {
            events.on_begin_phase(SystemPhase::Startup, &self.context, &mut self.state);
        }

        self.apply_system_phase_startup_without_events();

        if let Some(events) = &mut self.events {
            events.on_end_phase(SystemPhase::Startup, &self.context, &mut self.state);
        }
    }

    /// Execute system phase `Startup` without calling events. See [`apply_system_phase_startup`](Self::apply_system_phase_startup).
    #[inline]
    fn apply_system_phase_startup_without_events(&mut self)
    where
        E: SystemPhaseEvents
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
        E: SystemPhaseEvents
    {
        if let Some(events) = &mut self.events {
            events.on_begin_phase(SystemPhase::Startup, &self.context, &mut self.state);
        }
        // no systems in this phase

        if let Some(events) = &mut self.events {
            events.on_end_phase(SystemPhase::Startup, &self.context, &mut self.state);
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
        E: SystemPhaseEvents
    {
        if let Some(events) = &mut self.events {
            events.on_begin_phase(SystemPhase::FixedUpdate, &self.context, &mut self.state);
        }

        self.apply_system_phase_fixed_update_without_events();

        if let Some(events) = &mut self.events {
            events.on_end_phase(SystemPhase::FixedUpdate, &self.context, &mut self.state);
        }
    }

    /// Execute system phase `FixedUpdate` without calling events. See [`apply_system_phase_fixed_update`](Self::apply_system_phase_fixed_update).
    #[inline]
    fn apply_system_phase_fixed_update_without_events(&mut self)
    where
        E: SystemPhaseEvents
    {
        // System group 1 of 1
        {
            // Apply PhysicsSystem to ParticleArchetype
            self.physics_system.apply_many(
                &self.context,
                &self.particle.entities,
                &self.particle.velocities,
                &mut self.particle.positions,
            );
            // Apply PhysicsSystem to PlayerArchetype
            self.physics_system.apply_many(
                &self.context,
                &self.player.entities,
                &self.player.velocities,
                &mut self.player.positions,
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
        E: SystemPhaseEvents
    {
        if let Some(events) = &mut self.events {
            events.on_begin_phase(SystemPhase::FixedUpdate, &self.context, &mut self.state);
        }
        // Only one system in this phase; skip spawning thread.
        self.apply_system_phase_fixed_update_without_events();

        if let Some(events) = &mut self.events {
            events.on_end_phase(SystemPhase::FixedUpdate, &self.context, &mut self.state);
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
        E: SystemPhaseEvents
    {
        if let Some(events) = &mut self.events {
            events.on_begin_phase(SystemPhase::Update, &self.context, &mut self.state);
        }

        self.apply_system_phase_update_without_events();

        if let Some(events) = &mut self.events {
            events.on_end_phase(SystemPhase::Update, &self.context, &mut self.state);
        }
    }

    /// Execute system phase `Update` without calling events. See [`apply_system_phase_update`](Self::apply_system_phase_update).
    #[inline]
    fn apply_system_phase_update_without_events(&mut self)
    where
        E: SystemPhaseEvents
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
        E: SystemPhaseEvents
    {
        if let Some(events) = &mut self.events {
            events.on_begin_phase(SystemPhase::Update, &self.context, &mut self.state);
        }
        // no systems in this phase

        if let Some(events) = &mut self.events {
            events.on_end_phase(SystemPhase::Update, &self.context, &mut self.state);
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
        E: SystemPhaseEvents
    {
        if let Some(events) = &mut self.events {
            events.on_begin_phase(SystemPhase::Render, &self.context, &mut self.state);
        }

        self.apply_system_phase_render_without_events();

        if let Some(events) = &mut self.events {
            events.on_end_phase(SystemPhase::Render, &self.context, &mut self.state);
        }
    }

    /// Execute system phase `Render` without calling events. See [`apply_system_phase_render`](Self::apply_system_phase_render).
    #[inline]
    fn apply_system_phase_render_without_events(&mut self)
    where
        E: SystemPhaseEvents
    {
        // System group 1 of 1
        {
            // Apply RenderSystem to ParticleArchetype
            self.render_system.apply_many(
                &self.context,
                &self.state,
                &self.particle.positions,
            );
            // Apply RenderSystem to PlayerArchetype
            self.render_system.apply_many(
                &self.context,
                &self.state,
                &self.player.positions,
            );
            // Apply RenderSystem to ForegroundObjectArchetype
            self.render_system.apply_many(
                &self.context,
                &self.state,
                &self.foreground_object.positions,
            );
            // Apply RenderSystem to BackgroundObjectArchetype
            self.render_system.apply_many(
                &self.context,
                &self.state,
                &self.background_object.positions,
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
        E: SystemPhaseEvents
    {
        if let Some(events) = &mut self.events {
            events.on_begin_phase(SystemPhase::Render, &self.context, &mut self.state);
        }
        // Only one system in this phase; skip spawning thread.
        self.apply_system_phase_render_without_events();

        if let Some(events) = &mut self.events {
            events.on_end_phase(SystemPhase::Render, &self.context, &mut self.state);
        }
    }
}