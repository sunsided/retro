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

    /// Sends a command. May block until sending is complete.
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

    /// Resets the frame context, e.g. after the application came back to foreground.
    fn reset(&mut self) {
        self.current_frame_start = std::time::Instant::now();
        self.last_frame_start = std::time::Instant::now();
    }
}

/// Marker trait for worlds.
#[allow(dead_code)]
pub trait World {
    /// The ID of this world.
    const ID: WorldId;

    /// The ID of this world.
    #[inline]
    #[allow(dead_code)]
    fn id(&self) -> WorldId {
        Self::ID
    }
}


/// A world  all archetypes.
#[derive(Debug)]
pub struct MainWorld<E, Q> {
    /// The archetypes.
    archetypes: MainWorldArchetypes,
    /// The systems.
    pub systems: MainWorldSystems,
    /// The entity locator.
    //
    // Hashmap type not provided by design. Provide your own implementation such as fxhash::FxHashMap via type alias.
    // Example:
    //      type EntityLocationMap<K, V> = fxhash::FxHashMap<K, V>;
    //
    entity_locations: EntityLocationMap<EntityId, EntityArchetypeRef>,
    /// The frame context.
    context: FrameContext,
    /// The fixed-time accumulators.
    fixed_accumulators: FixedAccumulators,
    /// The phase delta times.
    delta_timers: DeltaTimers,
    /// The system phase events
    events: E,
    /// User-specific data.
    pub states: MainWorldStates,
    command_queue: Q
}

impl<E, Q> World for MainWorld<E, Q> {
    const ID: WorldId = WorldId(core::num::NonZeroU64::new(1).expect("Invalid ID on ECS construction time"));
}

/// The user states used in the world.
#[derive(Debug, Default)]
pub struct MainWorldStates {
    /// The WGPU render state; will be initialized in the Render phase hooks
    ///
    /// The state is used in the following systems:
    /// - [`CameraSystem`]
    /// - [`CameraRenderSystem`]
    /// - [`FrameGlobalsSystem`]
    /// - [`RenderBackgroundSystem`]
    /// - [`RenderSkySystem`]
    /// - [`RenderSystem`]
    /// - [`TileRenderSystem`]
    /// - [`DynamicLightSystem`]
    /// - [`VgaBlitSystem`]
    pub wgpu_render: WgpuRenderState,
    /// The device input obtained from the window system.
    ///
    /// The state is used in the following systems:
    /// - [`CameraSystem`]
    pub input: InputState,
    /// The uniforms of the active camera.
    ///
    /// The state is used in the following systems:
    /// - [`CameraRenderSystem`]
    pub active_camera: ActiveCameraState,
}

/// The archetypes used in the world.
#[derive(Debug, Clone, Default)]
struct MainWorldArchetypes {
    pub environment: EnvironmentArchetype,
    pub camera: CameraArchetype,
    pub dynamic_light: DynamicLightArchetype,
    pub tile: TileArchetype,
    pub cube: CubeArchetype,
    pub player: PlayerArchetype,
    pub foreground_object: ForegroundObjectArchetype,
    pub background_object: BackgroundObjectArchetype,
    pub scene_background: SceneBackgroundArchetype,
}

/// The systems.
#[derive(Debug)]
pub struct MainWorldSystems {
    time_of_day: TimeOfDaySystem,
    sun: SunSystem,
    fog: FogSystem,
    stars: StarsSystem,
    frame_globals: FrameGlobalsSystem,
    render_sky: RenderSkySystem,
    camera: CameraSystem,
    camera_render: CameraRenderSystem,
    dynamic_light: DynamicLightSystem,
    tile_render: TileRenderSystem,
    physics: PhysicsSystem,
    render: RenderSystem,
    background_color_cycle: BackgroundColorCycleSystem,
    render_background: RenderBackgroundSystem,
    vga_blit: VgaBlitSystem,
}

impl MainWorldSystems {

    /// Gives immutable access to the [`TimeOfDaySystemData`] of the [`TimeOfDaySystem`].
    #[inline]
    pub fn as_time_of_day_ref(&self) -> &TimeOfDaySystemData {
        use core::ops::Deref;
        self.time_of_day.deref()
    }

    /// Gives mutable access to the [`TimeOfDaySystemData`] of the [`TimeOfDaySystem`].
    #[inline]
    pub fn as_time_of_day_mut(&mut self) -> &mut TimeOfDaySystemData {
        use core::ops::DerefMut;
        self.time_of_day.deref_mut()
    }

    /// Gives immutable access to the [`SunSystemData`] of the [`SunSystem`].
    #[inline]
    pub fn as_sun_ref(&self) -> &SunSystemData {
        use core::ops::Deref;
        self.sun.deref()
    }

    /// Gives mutable access to the [`SunSystemData`] of the [`SunSystem`].
    #[inline]
    pub fn as_sun_mut(&mut self) -> &mut SunSystemData {
        use core::ops::DerefMut;
        self.sun.deref_mut()
    }

    /// Gives immutable access to the [`FogSystemData`] of the [`FogSystem`].
    #[inline]
    pub fn as_fog_ref(&self) -> &FogSystemData {
        use core::ops::Deref;
        self.fog.deref()
    }

    /// Gives mutable access to the [`FogSystemData`] of the [`FogSystem`].
    #[inline]
    pub fn as_fog_mut(&mut self) -> &mut FogSystemData {
        use core::ops::DerefMut;
        self.fog.deref_mut()
    }

    /// Gives immutable access to the [`StarsSystemData`] of the [`StarsSystem`].
    #[inline]
    pub fn as_stars_ref(&self) -> &StarsSystemData {
        use core::ops::Deref;
        self.stars.deref()
    }

    /// Gives mutable access to the [`StarsSystemData`] of the [`StarsSystem`].
    #[inline]
    pub fn as_stars_mut(&mut self) -> &mut StarsSystemData {
        use core::ops::DerefMut;
        self.stars.deref_mut()
    }

    /// Gives immutable access to the [`FrameGlobalsSystemData`] of the [`FrameGlobalsSystem`].
    #[inline]
    pub fn as_frame_globals_ref(&self) -> &FrameGlobalsSystemData {
        use core::ops::Deref;
        self.frame_globals.deref()
    }

    /// Gives mutable access to the [`FrameGlobalsSystemData`] of the [`FrameGlobalsSystem`].
    #[inline]
    pub fn as_frame_globals_mut(&mut self) -> &mut FrameGlobalsSystemData {
        use core::ops::DerefMut;
        self.frame_globals.deref_mut()
    }

    /// Gives immutable access to the [`RenderSkySystemData`] of the [`RenderSkySystem`].
    #[inline]
    pub fn as_render_sky_ref(&self) -> &RenderSkySystemData {
        use core::ops::Deref;
        self.render_sky.deref()
    }

    /// Gives mutable access to the [`RenderSkySystemData`] of the [`RenderSkySystem`].
    #[inline]
    pub fn as_render_sky_mut(&mut self) -> &mut RenderSkySystemData {
        use core::ops::DerefMut;
        self.render_sky.deref_mut()
    }

    /// Gives immutable access to the [`CameraSystemData`] of the [`CameraSystem`].
    #[inline]
    pub fn as_camera_ref(&self) -> &CameraSystemData {
        use core::ops::Deref;
        self.camera.deref()
    }

    /// Gives mutable access to the [`CameraSystemData`] of the [`CameraSystem`].
    #[inline]
    pub fn as_camera_mut(&mut self) -> &mut CameraSystemData {
        use core::ops::DerefMut;
        self.camera.deref_mut()
    }

    /// Gives immutable access to the [`CameraRenderSystemData`] of the [`CameraRenderSystem`].
    #[inline]
    pub fn as_camera_render_ref(&self) -> &CameraRenderSystemData {
        use core::ops::Deref;
        self.camera_render.deref()
    }

    /// Gives mutable access to the [`CameraRenderSystemData`] of the [`CameraRenderSystem`].
    #[inline]
    pub fn as_camera_render_mut(&mut self) -> &mut CameraRenderSystemData {
        use core::ops::DerefMut;
        self.camera_render.deref_mut()
    }

    /// Gives immutable access to the [`DynamicLightSystemData`] of the [`DynamicLightSystem`].
    #[inline]
    pub fn as_dynamic_light_ref(&self) -> &DynamicLightSystemData {
        use core::ops::Deref;
        self.dynamic_light.deref()
    }

    /// Gives mutable access to the [`DynamicLightSystemData`] of the [`DynamicLightSystem`].
    #[inline]
    pub fn as_dynamic_light_mut(&mut self) -> &mut DynamicLightSystemData {
        use core::ops::DerefMut;
        self.dynamic_light.deref_mut()
    }

    /// Gives immutable access to the [`TileRenderSystemData`] of the [`TileRenderSystem`].
    #[inline]
    pub fn as_tile_render_ref(&self) -> &TileRenderSystemData {
        use core::ops::Deref;
        self.tile_render.deref()
    }

    /// Gives mutable access to the [`TileRenderSystemData`] of the [`TileRenderSystem`].
    #[inline]
    pub fn as_tile_render_mut(&mut self) -> &mut TileRenderSystemData {
        use core::ops::DerefMut;
        self.tile_render.deref_mut()
    }

    /// Gives immutable access to the [`PhysicsSystemData`] of the [`PhysicsSystem`].
    #[inline]
    pub fn as_physics_ref(&self) -> &PhysicsSystemData {
        use core::ops::Deref;
        self.physics.deref()
    }

    /// Gives mutable access to the [`PhysicsSystemData`] of the [`PhysicsSystem`].
    #[inline]
    pub fn as_physics_mut(&mut self) -> &mut PhysicsSystemData {
        use core::ops::DerefMut;
        self.physics.deref_mut()
    }

    /// Gives immutable access to the [`RenderSystemData`] of the [`RenderSystem`].
    #[inline]
    pub fn as_render_ref(&self) -> &RenderSystemData {
        use core::ops::Deref;
        self.render.deref()
    }

    /// Gives mutable access to the [`RenderSystemData`] of the [`RenderSystem`].
    #[inline]
    pub fn as_render_mut(&mut self) -> &mut RenderSystemData {
        use core::ops::DerefMut;
        self.render.deref_mut()
    }

    /// Gives immutable access to the [`BackgroundColorCycleSystemData`] of the [`BackgroundColorCycleSystem`].
    #[inline]
    pub fn as_background_color_cycle_ref(&self) -> &BackgroundColorCycleSystemData {
        use core::ops::Deref;
        self.background_color_cycle.deref()
    }

    /// Gives mutable access to the [`BackgroundColorCycleSystemData`] of the [`BackgroundColorCycleSystem`].
    #[inline]
    pub fn as_background_color_cycle_mut(&mut self) -> &mut BackgroundColorCycleSystemData {
        use core::ops::DerefMut;
        self.background_color_cycle.deref_mut()
    }

    /// Gives immutable access to the [`RenderBackgroundSystemData`] of the [`RenderBackgroundSystem`].
    #[inline]
    pub fn as_render_background_ref(&self) -> &RenderBackgroundSystemData {
        use core::ops::Deref;
        self.render_background.deref()
    }

    /// Gives mutable access to the [`RenderBackgroundSystemData`] of the [`RenderBackgroundSystem`].
    #[inline]
    pub fn as_render_background_mut(&mut self) -> &mut RenderBackgroundSystemData {
        use core::ops::DerefMut;
        self.render_background.deref_mut()
    }

    /// Gives immutable access to the [`VgaBlitSystemData`] of the [`VgaBlitSystem`].
    #[inline]
    pub fn as_vga_blit_ref(&self) -> &VgaBlitSystemData {
        use core::ops::Deref;
        self.vga_blit.deref()
    }

    /// Gives mutable access to the [`VgaBlitSystemData`] of the [`VgaBlitSystem`].
    #[inline]
    pub fn as_vga_blit_mut(&mut self) -> &mut VgaBlitSystemData {
        use core::ops::DerefMut;
        self.vga_blit.deref_mut()
    }
}

impl AsRef<TimeOfDaySystemData> for MainWorldSystems {
    #[inline]
    fn as_ref(&self) -> &TimeOfDaySystemData {
        self.as_time_of_day_ref()
    }
}

impl AsMut<TimeOfDaySystemData> for MainWorldSystems {
    #[inline]
    fn as_mut(&mut self) -> &mut TimeOfDaySystemData {
        self.as_time_of_day_mut()
    }
}

impl AsRef<SunSystemData> for MainWorldSystems {
    #[inline]
    fn as_ref(&self) -> &SunSystemData {
        self.as_sun_ref()
    }
}

impl AsMut<SunSystemData> for MainWorldSystems {
    #[inline]
    fn as_mut(&mut self) -> &mut SunSystemData {
        self.as_sun_mut()
    }
}

impl AsRef<FogSystemData> for MainWorldSystems {
    #[inline]
    fn as_ref(&self) -> &FogSystemData {
        self.as_fog_ref()
    }
}

impl AsMut<FogSystemData> for MainWorldSystems {
    #[inline]
    fn as_mut(&mut self) -> &mut FogSystemData {
        self.as_fog_mut()
    }
}

impl AsRef<StarsSystemData> for MainWorldSystems {
    #[inline]
    fn as_ref(&self) -> &StarsSystemData {
        self.as_stars_ref()
    }
}

impl AsMut<StarsSystemData> for MainWorldSystems {
    #[inline]
    fn as_mut(&mut self) -> &mut StarsSystemData {
        self.as_stars_mut()
    }
}

impl AsRef<FrameGlobalsSystemData> for MainWorldSystems {
    #[inline]
    fn as_ref(&self) -> &FrameGlobalsSystemData {
        self.as_frame_globals_ref()
    }
}

impl AsMut<FrameGlobalsSystemData> for MainWorldSystems {
    #[inline]
    fn as_mut(&mut self) -> &mut FrameGlobalsSystemData {
        self.as_frame_globals_mut()
    }
}

impl AsRef<RenderSkySystemData> for MainWorldSystems {
    #[inline]
    fn as_ref(&self) -> &RenderSkySystemData {
        self.as_render_sky_ref()
    }
}

impl AsMut<RenderSkySystemData> for MainWorldSystems {
    #[inline]
    fn as_mut(&mut self) -> &mut RenderSkySystemData {
        self.as_render_sky_mut()
    }
}

impl AsRef<CameraSystemData> for MainWorldSystems {
    #[inline]
    fn as_ref(&self) -> &CameraSystemData {
        self.as_camera_ref()
    }
}

impl AsMut<CameraSystemData> for MainWorldSystems {
    #[inline]
    fn as_mut(&mut self) -> &mut CameraSystemData {
        self.as_camera_mut()
    }
}

impl AsRef<CameraRenderSystemData> for MainWorldSystems {
    #[inline]
    fn as_ref(&self) -> &CameraRenderSystemData {
        self.as_camera_render_ref()
    }
}

impl AsMut<CameraRenderSystemData> for MainWorldSystems {
    #[inline]
    fn as_mut(&mut self) -> &mut CameraRenderSystemData {
        self.as_camera_render_mut()
    }
}

impl AsRef<DynamicLightSystemData> for MainWorldSystems {
    #[inline]
    fn as_ref(&self) -> &DynamicLightSystemData {
        self.as_dynamic_light_ref()
    }
}

impl AsMut<DynamicLightSystemData> for MainWorldSystems {
    #[inline]
    fn as_mut(&mut self) -> &mut DynamicLightSystemData {
        self.as_dynamic_light_mut()
    }
}

impl AsRef<TileRenderSystemData> for MainWorldSystems {
    #[inline]
    fn as_ref(&self) -> &TileRenderSystemData {
        self.as_tile_render_ref()
    }
}

impl AsMut<TileRenderSystemData> for MainWorldSystems {
    #[inline]
    fn as_mut(&mut self) -> &mut TileRenderSystemData {
        self.as_tile_render_mut()
    }
}

impl AsRef<PhysicsSystemData> for MainWorldSystems {
    #[inline]
    fn as_ref(&self) -> &PhysicsSystemData {
        self.as_physics_ref()
    }
}

impl AsMut<PhysicsSystemData> for MainWorldSystems {
    #[inline]
    fn as_mut(&mut self) -> &mut PhysicsSystemData {
        self.as_physics_mut()
    }
}

impl AsRef<RenderSystemData> for MainWorldSystems {
    #[inline]
    fn as_ref(&self) -> &RenderSystemData {
        self.as_render_ref()
    }
}

impl AsMut<RenderSystemData> for MainWorldSystems {
    #[inline]
    fn as_mut(&mut self) -> &mut RenderSystemData {
        self.as_render_mut()
    }
}

impl AsRef<BackgroundColorCycleSystemData> for MainWorldSystems {
    #[inline]
    fn as_ref(&self) -> &BackgroundColorCycleSystemData {
        self.as_background_color_cycle_ref()
    }
}

impl AsMut<BackgroundColorCycleSystemData> for MainWorldSystems {
    #[inline]
    fn as_mut(&mut self) -> &mut BackgroundColorCycleSystemData {
        self.as_background_color_cycle_mut()
    }
}

impl AsRef<RenderBackgroundSystemData> for MainWorldSystems {
    #[inline]
    fn as_ref(&self) -> &RenderBackgroundSystemData {
        self.as_render_background_ref()
    }
}

impl AsMut<RenderBackgroundSystemData> for MainWorldSystems {
    #[inline]
    fn as_mut(&mut self) -> &mut RenderBackgroundSystemData {
        self.as_render_background_mut()
    }
}

impl AsRef<VgaBlitSystemData> for MainWorldSystems {
    #[inline]
    fn as_ref(&self) -> &VgaBlitSystemData {
        self.as_vga_blit_ref()
    }
}

impl AsMut<VgaBlitSystemData> for MainWorldSystems {
    #[inline]
    fn as_mut(&mut self) -> &mut VgaBlitSystemData {
        self.as_vga_blit_mut()
    }
}

/// Spawns an entity into the world.
impl<E, Q> Spawn<EnvironmentEntityData> for MainWorld<E, Q> {
    /// Spawn a new entity into the world.
    #[inline]
    fn spawn(&mut self, data: EnvironmentEntityData) -> EntityId {
        self.spawn_environment(data)
    }
}

/// Spawns an entity into the world.
impl<E, Q> Spawn<CameraEntityData> for MainWorld<E, Q> {
    /// Spawn a new entity into the world.
    #[inline]
    fn spawn(&mut self, data: CameraEntityData) -> EntityId {
        self.spawn_camera(data)
    }
}

/// Spawns an entity into the world.
impl<E, Q> Spawn<DynamicLightEntityData> for MainWorld<E, Q> {
    /// Spawn a new entity into the world.
    #[inline]
    fn spawn(&mut self, data: DynamicLightEntityData) -> EntityId {
        self.spawn_dynamic_light(data)
    }
}

/// Spawns an entity into the world.
impl<E, Q> Spawn<TileEntityData> for MainWorld<E, Q> {
    /// Spawn a new entity into the world.
    #[inline]
    fn spawn(&mut self, data: TileEntityData) -> EntityId {
        self.spawn_tile(data)
    }
}

/// Spawns an entity into the world.
impl<E, Q> Spawn<CubeEntityData> for MainWorld<E, Q> {
    /// Spawn a new entity into the world.
    #[inline]
    fn spawn(&mut self, data: CubeEntityData) -> EntityId {
        self.spawn_cube(data)
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

/// Spawns an entity into the world.
impl<E, Q> Spawn<SceneBackgroundEntityData> for MainWorld<E, Q> {
    /// Spawn a new entity into the world.
    #[inline]
    fn spawn(&mut self, data: SceneBackgroundEntityData) -> EntityId {
        self.spawn_scene_background(data)
    }
}

/// Helper trait to prevent accidental abuse of the Archetype's spawning function.
pub trait WorldEntityRegistry {
    /// Registers an entity with the world.
    fn register(&mut self, id: EntityId, archetype: EntityArchetypeRef) -> EntityId;
}

#[allow(dead_code)]
impl<Q> MainWorld<NoOpPhaseEvents, Q> {
    /// Creates a new [`MainWorld`].
    pub fn new<S>(
        create_systems: &S,
        wgpu_render: WgpuRenderState,
        input: InputState,
        active_camera: ActiveCameraState,
        command_queue: Q) -> Self
    where
        S: CreateSystems,
        Q: WorldCommandQueue
    {
        Self::new_with_events(
            create_systems,
            wgpu_render,
            input,
            active_camera,
            command_queue,
            NoOpPhaseEvents
        )
    }
}

#[allow(dead_code)]
impl<E, Q> MainWorld<E, Q> {
    /// Creates a new [`MainWorld`].
    pub fn new_with_events<S>(
        create_systems: &S,
        wgpu_render: WgpuRenderState,
        input: InputState,
        active_camera: ActiveCameraState,
        command_queue: Q,
        phase_events: E) -> Self
    where
        S: CreateSystem<TimeOfDaySystem>
         + CreateSystem<SunSystem>
         + CreateSystem<FogSystem>
         + CreateSystem<StarsSystem>
         + CreateSystem<FrameGlobalsSystem>
         + CreateSystem<RenderSkySystem>
         + CreateSystem<CameraSystem>
         + CreateSystem<CameraRenderSystem>
         + CreateSystem<DynamicLightSystem>
         + CreateSystem<TileRenderSystem>
         + CreateSystem<PhysicsSystem>
         + CreateSystem<RenderSystem>
         + CreateSystem<BackgroundColorCycleSystem>
         + CreateSystem<RenderBackgroundSystem>
         + CreateSystem<VgaBlitSystem>,
        E: SystemPhaseEvents,
        Q: WorldCommandQueue
    {
        let context = FrameContext::new(Self::ID);
        Self {
            archetypes: Default::default(),
            systems: MainWorldSystems {
                time_of_day: CreateSystem::<TimeOfDaySystem>::create(create_systems),
                sun: CreateSystem::<SunSystem>::create(create_systems),
                fog: CreateSystem::<FogSystem>::create(create_systems),
                stars: CreateSystem::<StarsSystem>::create(create_systems),
                frame_globals: CreateSystem::<FrameGlobalsSystem>::create(create_systems),
                render_sky: CreateSystem::<RenderSkySystem>::create(create_systems),
                camera: CreateSystem::<CameraSystem>::create(create_systems),
                camera_render: CreateSystem::<CameraRenderSystem>::create(create_systems),
                dynamic_light: CreateSystem::<DynamicLightSystem>::create(create_systems),
                tile_render: CreateSystem::<TileRenderSystem>::create(create_systems),
                physics: CreateSystem::<PhysicsSystem>::create(create_systems),
                render: CreateSystem::<RenderSystem>::create(create_systems),
                background_color_cycle: CreateSystem::<BackgroundColorCycleSystem>::create(create_systems),
                render_background: CreateSystem::<RenderBackgroundSystem>::create(create_systems),
                vga_blit: CreateSystem::<VgaBlitSystem>::create(create_systems),
            },
            entity_locations: Default::default(),
            context,
            states: MainWorldStates {
                wgpu_render,
                input,
                active_camera,
            },
            fixed_accumulators: Default::default(),
            delta_timers: DeltaTimers::default(),
            events: phase_events,
            command_queue
        }
    }

    /// Resets the frame context, e.g. after the application comes back from background.
    pub fn reset_frame_context(&mut self) {
        self.context.reset();
    }

    /// Returns the overall number of entities in this world.
    pub fn len(&self) -> usize {
        self.entity_locations.len()
    }

    /// Indicates whether there are no entities in this world.
    pub fn is_empty(&self) -> bool {
        self.entity_locations.is_empty()
    }

    /// De-spawns an entity given by its [`EntityId`]. Returns an error if the entity was unknown in this world.
    pub fn despawn_by_id(&mut self, id: EntityId) -> Result<(), DespawnError> {
        self.handle_despawn_command(id)
    }

    /// Spawn a new `Environment` entity into the world given its [`EnvironmentEntityData`].
    #[inline]
    pub fn spawn_environment(
        &mut self,
        environment: EnvironmentEntityData
    ) -> EntityId {
        self.spawn_environment_with(
            environment.time_of_day,
            environment.sun,
            environment.skydome,
            environment.ambient_light,
            environment.stars,
            environment.fog,
        )
    }

    /// Spawn a new `Environment` entity into the world.
    pub fn spawn_environment_with(
        &mut self,
        time_of_day: TimeOfDayComponent,
        sun: SunComponent,
        skydome: SkydomeComponent,
        ambient_light: AmbientLightComponent,
        stars: StarsComponent,
        fog: FogComponent,
    ) -> EntityId {
        struct Registry<'a>(&'a mut EntityLocationMap<EntityId, EntityArchetypeRef>);

        impl WorldEntityRegistry for Registry<'_> {
            #[inline(always)]
            fn register(&mut self, id: EntityId, archetype: EntityArchetypeRef) -> EntityId {
                self.0.insert(id, archetype);
                id
            }
        }

        self.archetypes.environment
            .spawn_with(
                time_of_day,
                sun,
                skydome,
                ambient_light,
                stars,
                fog,
                Registry(&mut self.entity_locations)
            )
    }

    /// Spawn a new `Camera` entity into the world given its [`CameraEntityData`].
    #[inline]
    pub fn spawn_camera(
        &mut self,
        camera: CameraEntityData
    ) -> EntityId {
        self.spawn_camera_with(
            camera.camera,
            camera.camera_fps_controller,
        )
    }

    /// Spawn a new `Camera` entity into the world.
    pub fn spawn_camera_with(
        &mut self,
        camera: CameraComponent,
        camera_fps_controller: CameraFpsControllerComponent,
    ) -> EntityId {
        struct Registry<'a>(&'a mut EntityLocationMap<EntityId, EntityArchetypeRef>);

        impl WorldEntityRegistry for Registry<'_> {
            #[inline(always)]
            fn register(&mut self, id: EntityId, archetype: EntityArchetypeRef) -> EntityId {
                self.0.insert(id, archetype);
                id
            }
        }

        self.archetypes.camera
            .spawn_with(
                camera,
                camera_fps_controller,
                Registry(&mut self.entity_locations)
            )
    }

    /// Spawn a new `DynamicLight` entity into the world given its [`DynamicLightEntityData`].
    #[inline]
    pub fn spawn_dynamic_light(
        &mut self,
        dynamic_light: DynamicLightEntityData
    ) -> EntityId {
        self.spawn_dynamic_light_with(
            dynamic_light.dynamic_light,
            dynamic_light.color,
            dynamic_light.position,
        )
    }

    /// Spawn a new `DynamicLight` entity into the world.
    pub fn spawn_dynamic_light_with(
        &mut self,
        dynamic_light: DynamicLightComponent,
        color: ColorComponent,
        position: PositionComponent,
    ) -> EntityId {
        struct Registry<'a>(&'a mut EntityLocationMap<EntityId, EntityArchetypeRef>);

        impl WorldEntityRegistry for Registry<'_> {
            #[inline(always)]
            fn register(&mut self, id: EntityId, archetype: EntityArchetypeRef) -> EntityId {
                self.0.insert(id, archetype);
                id
            }
        }

        self.archetypes.dynamic_light
            .spawn_with(
                dynamic_light,
                color,
                position,
                Registry(&mut self.entity_locations)
            )
    }

    /// Spawn a new `Tile` entity into the world given its [`TileEntityData`].
    #[inline]
    pub fn spawn_tile(
        &mut self,
        tile: TileEntityData
    ) -> EntityId {
        self.spawn_tile_with(
            tile.tile,
            tile.position,
        )
    }

    /// Spawn a new `Tile` entity into the world.
    pub fn spawn_tile_with(
        &mut self,
        tile: TileComponent,
        position: PositionComponent,
    ) -> EntityId {
        struct Registry<'a>(&'a mut EntityLocationMap<EntityId, EntityArchetypeRef>);

        impl WorldEntityRegistry for Registry<'_> {
            #[inline(always)]
            fn register(&mut self, id: EntityId, archetype: EntityArchetypeRef) -> EntityId {
                self.0.insert(id, archetype);
                id
            }
        }

        self.archetypes.tile
            .spawn_with(
                tile,
                position,
                Registry(&mut self.entity_locations)
            )
    }

    /// Spawn a new `Cube` entity into the world given its [`CubeEntityData`].
    #[inline]
    pub fn spawn_cube(
        &mut self,
        cube: CubeEntityData
    ) -> EntityId {
        self.spawn_cube_with(
            cube.position,
            cube.velocity,
            cube.transform,
            cube.model,
        )
    }

    /// Spawn a new `Cube` entity into the world.
    pub fn spawn_cube_with(
        &mut self,
        position: PositionComponent,
        velocity: VelocityComponent,
        transform: TransformComponent,
        model: ModelComponent,
    ) -> EntityId {
        struct Registry<'a>(&'a mut EntityLocationMap<EntityId, EntityArchetypeRef>);

        impl WorldEntityRegistry for Registry<'_> {
            #[inline(always)]
            fn register(&mut self, id: EntityId, archetype: EntityArchetypeRef) -> EntityId {
                self.0.insert(id, archetype);
                id
            }
        }

        self.archetypes.cube
            .spawn_with(
                position,
                velocity,
                transform,
                model,
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
        struct Registry<'a>(&'a mut EntityLocationMap<EntityId, EntityArchetypeRef>);

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
        struct Registry<'a>(&'a mut EntityLocationMap<EntityId, EntityArchetypeRef>);

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
        struct Registry<'a>(&'a mut EntityLocationMap<EntityId, EntityArchetypeRef>);

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

    /// Spawn a new `SceneBackground` entity into the world given its [`SceneBackgroundEntityData`].
    #[inline]
    pub fn spawn_scene_background(
        &mut self,
        scene_background: SceneBackgroundEntityData
    ) -> EntityId {
        self.spawn_scene_background_with(
            scene_background.background_color,
        )
    }

    /// Spawn a new `SceneBackground` entity into the world.
    pub fn spawn_scene_background_with(
        &mut self,
        background_color: BackgroundColorComponent,
    ) -> EntityId {
        struct Registry<'a>(&'a mut EntityLocationMap<EntityId, EntityArchetypeRef>);

        impl WorldEntityRegistry for Registry<'_> {
            #[inline(always)]
            fn register(&mut self, id: EntityId, archetype: EntityArchetypeRef) -> EntityId {
                self.0.insert(id, archetype);
                id
            }
        }

        self.archetypes.scene_background
            .spawn_with(
                background_color,
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

    /// Runs a per-frame update of the frame context at the start of a frame.
    fn on_begin_startup_phase(&mut self) {
        let now = std::time::Instant::now();
        let previous = self.delta_timers.previous_startup_start;
        self.delta_timers.current_startup_start = now;
        self.context.current_frame_start = now;
        self.context.last_frame_start = previous;
        if self.context.frame_number == 0 {
            self.context.delta_time_secs = 0.0;
        } else {
            self.context.delta_time_secs = (now - previous).as_secs_f32();
        }
    }

    /// Runs a per-frame update of the frame context at the end of a frame.
    fn on_end_startup_phase(&mut self) {
        self.delta_timers.previous_startup_start = self.delta_timers.current_startup_start;
    }

    /// Runs a per-frame update of the frame context at the start of a frame.
    fn on_begin_fixed_update_phase(&mut self) {
        let now = std::time::Instant::now();
        let previous = self.delta_timers.previous_fixed_update_start;
        self.delta_timers.current_fixed_update_start = now;
        self.context.current_frame_start = now;
        self.context.last_frame_start = previous;
        if self.context.frame_number == 0 {
            self.context.delta_time_secs = 0.0;
        } else {
            self.context.delta_time_secs = (now - previous).as_secs_f32();
        }
    }

    /// Runs a per-frame update of the frame context at the end of a frame.
    fn on_end_fixed_update_phase(&mut self) {
        self.delta_timers.previous_fixed_update_start = self.delta_timers.current_fixed_update_start;
    }

    /// Runs a per-frame update of the frame context at the start of a frame.
    fn on_begin_update_phase(&mut self) {
        let now = std::time::Instant::now();
        let previous = self.delta_timers.previous_update_start;
        self.delta_timers.current_update_start = now;
        self.context.current_frame_start = now;
        self.context.last_frame_start = previous;
        if self.context.frame_number == 0 {
            self.context.delta_time_secs = 0.0;
        } else {
            self.context.delta_time_secs = (now - previous).as_secs_f32();
        }
    }

    /// Runs a per-frame update of the frame context at the end of a frame.
    fn on_end_update_phase(&mut self) {
        self.delta_timers.previous_update_start = self.delta_timers.current_update_start;
    }

    /// Runs a per-frame update of the frame context at the start of a frame.
    fn on_begin_render_phase(&mut self) {
        let now = std::time::Instant::now();
        let previous = self.delta_timers.previous_render_start;
        self.delta_timers.current_render_start = now;
        self.context.current_frame_start = now;
        self.context.last_frame_start = previous;
        if self.context.frame_number == 0 {
            self.context.delta_time_secs = 0.0;
        } else {
            self.context.delta_time_secs = (now - previous).as_secs_f32();
        }
    }

    /// Runs a per-frame update of the frame context at the end of a frame.
    fn on_end_render_phase(&mut self) {
        self.delta_timers.previous_render_start = self.delta_timers.current_render_start;
    }

    /// Apply all system phases sequentially.
    ///
    /// This method executes all system phases in sequence.
    /// The phases are processed in the following order:
    ///
    /// - `Startup`
    /// - `FixedUpdate`
    /// - `Update`
    /// - ~~`Render`~~ (skipped here, must be executed manually)
    /// 
    /// # Type Parameters
    /// - `E`: The type implementing [`SystemPhaseEvents`] for phase lifecycle events
    pub fn apply_system_phases(&mut self)
    where
        E: SystemPhaseEvents,
        Q: WorldCommandSender + WorldCommandReceiver
    {
        self.on_begin_frame();
        self.handle_commands();
        // The Startup phase always runs.
        self.apply_system_phase_startup();

        // Apply fixed-time loop for phase FixedUpdate
        self.context.fixed_time_secs = SystemPhase::FIXED_UPDATE_SECS;
        self.fixed_accumulators.fixed_update += self.context.delta_time_secs;
        while self.fixed_accumulators.fixed_update >= SystemPhase::FIXED_UPDATE_SECS {
            self.apply_system_phase_fixed_update();
            self.fixed_accumulators.fixed_update -= SystemPhase::FIXED_UPDATE_SECS;
        }
        self.context.fixed_time_secs = 0.0;
        // The Update phase always runs.
        self.apply_system_phase_update();

        // NOTE: The Render phase is marked manual and will be skipped here.
        //       self.par_apply_system_phase_render();
        self.on_end_frame();
        self.handle_commands();
    }

    /// Apply all system phases with thread parallelism.
    ///
    /// This method executes all system phases in sequence, but with parallel execution
    /// of systems within each phase. The phases are processed in the following order:
    ///
    /// - `Startup`
    /// - `FixedUpdate`
    /// - `Update`
    /// - ~~`Render`~~ (skipped here, must be executed manually)
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
        E: SystemPhaseEvents,
        Q: WorldCommandSender + WorldCommandReceiver
    {
        self.on_begin_frame();
        self.handle_commands();
        // The Startup phase always runs.
        self.apply_system_phase_startup();

        // Apply fixed-time loop for phase FixedUpdate
        self.context.fixed_time_secs = SystemPhase::FIXED_UPDATE_SECS;
        self.fixed_accumulators.fixed_update += self.context.delta_time_secs;
        while self.fixed_accumulators.fixed_update >= SystemPhase::FIXED_UPDATE_SECS {
            self.apply_system_phase_fixed_update();
            self.fixed_accumulators.fixed_update -= SystemPhase::FIXED_UPDATE_SECS;
        }
        self.context.fixed_time_secs = 0.0;
        // The Update phase always runs.
        self.apply_system_phase_update();

        // NOTE: The Render phase is marked manual and will be skipped here.
        //       self.par_apply_system_phase_render();
        self.on_end_frame();
        self.handle_commands();
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
        E: SystemPhaseEvents,
        Q: WorldCommandSender + WorldCommandReceiver
    {
        self.on_begin_startup_phase();
        let result = SystemStartupPhaseEvents::on_begin_phase(
            &mut self.events,
            &self.context,
        )
        .inspect_err(|error| tracing::error!(%error, "Failed to handle phase begin events"))
        .ok();

        self.apply_system_phase_startup_without_events();

        SystemStartupPhaseEvents::on_end_phase(
            &mut self.events,
            result,
            &self.context,
        )
        .inspect_err(|error| tracing::error!(%error, "Failed to handle phase end events"))
        .ok();

        // Process all commands.
        self.handle_commands();
        self.on_end_startup_phase();
    }

    /// Execute system phase `Startup` without calling events. See [`apply_system_phase_startup`](Self::apply_system_phase_startup).
    #[inline]
    fn apply_system_phase_startup_without_events(&mut self)
    where
        E: SystemPhaseEvents,
        Q: WorldCommandSender + WorldCommandReceiver
    {
        // no systems in this phase

        // Process all commands.
        self.handle_commands();
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
        E: SystemPhaseEvents,
        Q: WorldCommandSender + WorldCommandReceiver
    {
        self.on_begin_startup_phase();
        let result = SystemStartupPhaseEvents::on_begin_phase(
            &mut self.events,
            &self.context,
        )
        .inspect_err(|error| tracing::error!(%error, "Failed to handle phase begin events"))
        .ok();
        // no systems in this phase

        SystemStartupPhaseEvents::on_end_phase(
            &mut self.events,
            result,
            &self.context,
        )
        .inspect_err(|error| tracing::error!(%error, "Failed to handle phase end events"))
        .ok();

        // Process all commands.
        self.handle_commands();
        self.on_end_startup_phase();
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
        E: SystemPhaseEvents,
        Q: WorldCommandSender + WorldCommandReceiver
    {
        self.on_begin_fixed_update_phase();
        let result = SystemFixedUpdatePhaseEvents::on_begin_phase(
            &mut self.events,
            &self.context,
        )
        .inspect_err(|error| tracing::error!(%error, "Failed to handle phase begin events"))
        .ok();

        self.apply_system_phase_fixed_update_without_events();

        SystemFixedUpdatePhaseEvents::on_end_phase(
            &mut self.events,
            result,
            &self.context,
        )
        .inspect_err(|error| tracing::error!(%error, "Failed to handle phase end events"))
        .ok();

        // Process all commands.
        self.handle_commands();
        self.on_end_fixed_update_phase();
    }

    /// Execute system phase `FixedUpdate` without calling events. See [`apply_system_phase_fixed_update`](Self::apply_system_phase_fixed_update).
    #[inline]
    fn apply_system_phase_fixed_update_without_events(&mut self)
    where
        E: SystemPhaseEvents,
        Q: WorldCommandSender + WorldCommandReceiver
    {
        // System group 1 of 2
        let is_ready = self.systems.time_of_day.is_ready(
               &self.context,
           );
        if is_ready && self.systems.time_of_day.on_begin_phase(
                    &self.context,
                )
                .inspect_err(|error| tracing::error!(?error, "TimeOfDaySystem::on_begin_phase returned an error"))
                .is_ok()
        {
            // Apply TimeOfDaySystem to EnvironmentArchetype
            self.systems.time_of_day.apply_many(
                &self.context,
                &mut self.archetypes.environment.time_of_daies,
            );

            self.systems.time_of_day.on_end_phase(
                &self.context,
            )
            .inspect_err(|error| tracing::error!(?error, "TimeOfDaySystem::on_end_phase returned an error"))
            .ok();
        }
        // System group 1 of 2
        let is_ready = self.systems.physics.is_ready(
               &self.context,
           );
        if is_ready && self.systems.physics.on_begin_phase(
                    &self.context,
                )
                .inspect_err(|error| tracing::error!(?error, "PhysicsSystem::on_begin_phase returned an error"))
                .is_ok()
        {
            // Apply PhysicsSystem to CubeArchetype
            self.systems.physics.apply_many(
                &self.context,
                &self.archetypes.cube.velocities,
                &mut self.archetypes.cube.positions,
            );
            // Apply PhysicsSystem to PlayerArchetype
            self.systems.physics.apply_many(
                &self.context,
                &self.archetypes.player.velocities,
                &mut self.archetypes.player.positions,
            );

            self.systems.physics.on_end_phase(
                &self.context,
            )
            .inspect_err(|error| tracing::error!(?error, "PhysicsSystem::on_end_phase returned an error"))
            .ok();
        }
        // System group 2 of 2
        let is_ready = self.systems.sun.is_ready(
           );
        if is_ready && self.systems.sun.on_begin_phase(
                )
                .inspect_err(|error| tracing::error!(?error, "SunSystem::on_begin_phase returned an error"))
                .is_ok()
        {
            // Apply SunSystem to EnvironmentArchetype
            self.systems.sun.apply_many(
                &self.archetypes.environment.time_of_daies,
                &mut self.archetypes.environment.suns,
                &mut self.archetypes.environment.skydomes,
                &mut self.archetypes.environment.ambient_lights,
            );

            self.systems.sun.on_end_phase(
            )
            .inspect_err(|error| tracing::error!(?error, "SunSystem::on_end_phase returned an error"))
            .ok();
        }
        // System group 2 of 2
        let is_ready = self.systems.fog.is_ready(
           );
        if is_ready && self.systems.fog.on_begin_phase(
                )
                .inspect_err(|error| tracing::error!(?error, "FogSystem::on_begin_phase returned an error"))
                .is_ok()
        {
            // Apply FogSystem to EnvironmentArchetype
            self.systems.fog.apply_many(
                &self.archetypes.environment.time_of_daies,
                &mut self.archetypes.environment.fogs,
            );

            self.systems.fog.on_end_phase(
            )
            .inspect_err(|error| tracing::error!(?error, "FogSystem::on_end_phase returned an error"))
            .ok();
        }
        // System group 2 of 2
        let is_ready = self.systems.stars.is_ready(
           );
        if is_ready && self.systems.stars.on_begin_phase(
                )
                .inspect_err(|error| tracing::error!(?error, "StarsSystem::on_begin_phase returned an error"))
                .is_ok()
        {
            // Apply StarsSystem to EnvironmentArchetype
            self.systems.stars.apply_many(
                &self.archetypes.environment.time_of_daies,
                &mut self.archetypes.environment.stars,
            );

            self.systems.stars.on_end_phase(
            )
            .inspect_err(|error| tracing::error!(?error, "StarsSystem::on_end_phase returned an error"))
            .ok();
        }

        // Process all commands.
        self.handle_commands();
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
        E: SystemPhaseEvents,
        Q: WorldCommandSender + WorldCommandReceiver
    {
        self.on_begin_fixed_update_phase();
        let result = SystemFixedUpdatePhaseEvents::on_begin_phase(
            &mut self.events,
            &self.context,
        )
        .inspect_err(|error| tracing::error!(%error, "Failed to handle phase begin events"))
        .ok();

        // System group 1
        let is_time_of_day_ready = self.systems.time_of_day.is_ready(
               &self.context,
           );
        let is_time_of_day_ready = is_time_of_day_ready &
            self.systems.time_of_day.on_begin_phase(
                &self.context,
            )
            .inspect_err(|error| tracing::error!(%error, "TimeOfDaySystem::on_begin_phase returned an error"))
            .is_ok();
        let is_physics_ready = self.systems.physics.is_ready(
               &self.context,
           );
        let is_physics_ready = is_physics_ready &
            self.systems.physics.on_begin_phase(
                &self.context,
            )
            .inspect_err(|error| tracing::error!(%error, "PhysicsSystem::on_begin_phase returned an error"))
            .is_ok();

        {
            rayon::scope(|s| {
                // TODO: Instead of parallelizing systems (and then have them access archetypes), parallelize archetypes and apply all systems. This should improve data parallelism because it keeps caches hot.
                if is_time_of_day_ready {
                    s.spawn(|_| {
                        // Apply TimeOfDaySystem to EnvironmentArchetype
                        self.systems.time_of_day.apply_many(
                            &self.context,
                            &mut self.archetypes.environment.time_of_daies,
                        );
                    });
                }
                if is_physics_ready {
                    s.spawn(|_| {
                        // Apply PhysicsSystem to CubeArchetype
                        self.systems.physics.apply_many(
                            &self.context,
                            &self.archetypes.cube.velocities,
                            &mut self.archetypes.cube.positions,
                        );
                        // Apply PhysicsSystem to PlayerArchetype
                        self.systems.physics.apply_many(
                            &self.context,
                            &self.archetypes.player.velocities,
                            &mut self.archetypes.player.positions,
                        );
                    });
                }
            });
        }

        self.systems.time_of_day.on_end_phase(
            &self.context,
        )
        .inspect_err(|error| tracing::error!(%error, "TimeOfDaySystem::on_end_phase returned an error"))
        .ok();

        self.systems.physics.on_end_phase(
            &self.context,
        )
        .inspect_err(|error| tracing::error!(%error, "PhysicsSystem::on_end_phase returned an error"))
        .ok();

        // System group 2
        let is_sun_ready = self.systems.sun.is_ready(
           );
        let is_sun_ready = is_sun_ready &
            self.systems.sun.on_begin_phase(
            )
            .inspect_err(|error| tracing::error!(%error, "SunSystem::on_begin_phase returned an error"))
            .is_ok();
        let is_fog_ready = self.systems.fog.is_ready(
           );
        let is_fog_ready = is_fog_ready &
            self.systems.fog.on_begin_phase(
            )
            .inspect_err(|error| tracing::error!(%error, "FogSystem::on_begin_phase returned an error"))
            .is_ok();
        let is_stars_ready = self.systems.stars.is_ready(
           );
        let is_stars_ready = is_stars_ready &
            self.systems.stars.on_begin_phase(
            )
            .inspect_err(|error| tracing::error!(%error, "StarsSystem::on_begin_phase returned an error"))
            .is_ok();

        {
            rayon::scope(|s| {
                // TODO: Instead of parallelizing systems (and then have them access archetypes), parallelize archetypes and apply all systems. This should improve data parallelism because it keeps caches hot.
                if is_sun_ready {
                    s.spawn(|_| {
                        // Apply SunSystem to EnvironmentArchetype
                        self.systems.sun.apply_many(
                            &self.archetypes.environment.time_of_daies,
                            &mut self.archetypes.environment.suns,
                            &mut self.archetypes.environment.skydomes,
                            &mut self.archetypes.environment.ambient_lights,
                        );
                    });
                }
                if is_fog_ready {
                    s.spawn(|_| {
                        // Apply FogSystem to EnvironmentArchetype
                        self.systems.fog.apply_many(
                            &self.archetypes.environment.time_of_daies,
                            &mut self.archetypes.environment.fogs,
                        );
                    });
                }
                if is_stars_ready {
                    s.spawn(|_| {
                        // Apply StarsSystem to EnvironmentArchetype
                        self.systems.stars.apply_many(
                            &self.archetypes.environment.time_of_daies,
                            &mut self.archetypes.environment.stars,
                        );
                    });
                }
            });
        }

        self.systems.sun.on_end_phase(
        )
        .inspect_err(|error| tracing::error!(%error, "SunSystem::on_end_phase returned an error"))
        .ok();

        self.systems.fog.on_end_phase(
        )
        .inspect_err(|error| tracing::error!(%error, "FogSystem::on_end_phase returned an error"))
        .ok();

        self.systems.stars.on_end_phase(
        )
        .inspect_err(|error| tracing::error!(%error, "StarsSystem::on_end_phase returned an error"))
        .ok();

        SystemFixedUpdatePhaseEvents::on_end_phase(
            &mut self.events,
            result,
            &self.context,
        )
        .inspect_err(|error| tracing::error!(%error, "Failed to handle phase end events"))
        .ok();

        // Process all commands.
        self.handle_commands();
        self.on_end_fixed_update_phase();
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
        E: SystemPhaseEvents,
        Q: WorldCommandSender + WorldCommandReceiver
    {
        self.on_begin_update_phase();
        let result = SystemUpdatePhaseEvents::on_begin_phase(
            &mut self.events,
            &self.context,
        )
        .inspect_err(|error| tracing::error!(%error, "Failed to handle phase begin events"))
        .ok();

        self.apply_system_phase_update_without_events();

        SystemUpdatePhaseEvents::on_end_phase(
            &mut self.events,
            result,
            &self.context,
        )
        .inspect_err(|error| tracing::error!(%error, "Failed to handle phase end events"))
        .ok();

        // Process all commands.
        self.handle_commands();
        self.on_end_update_phase();
    }

    /// Execute system phase `Update` without calling events. See [`apply_system_phase_update`](Self::apply_system_phase_update).
    #[inline]
    fn apply_system_phase_update_without_events(&mut self)
    where
        E: SystemPhaseEvents,
        Q: WorldCommandSender + WorldCommandReceiver
    {
        // System group 1 of 1
        let is_ready = self.systems.camera.is_ready(
               &self.states.input,
               &self.states.wgpu_render,
           );
        if is_ready && self.systems.camera.on_begin_phase(
                    &self.states.input,
                    &self.states.wgpu_render,
                )
                .inspect_err(|error| tracing::error!(?error, "CameraSystem::on_begin_phase returned an error"))
                .is_ok()
        {
            // Apply CameraSystem to CameraArchetype
            self.systems.camera.apply_many(
                &self.states.input,
                &self.states.wgpu_render,
                &mut self.archetypes.camera.cameras,
                &mut self.archetypes.camera.camera_fps_controllers,
            );

            self.systems.camera.on_end_phase(
                &self.states.input,
                &self.states.wgpu_render,
            )
            .inspect_err(|error| tracing::error!(?error, "CameraSystem::on_end_phase returned an error"))
            .ok();
        }
        // System group 1 of 1
        let is_ready = self.systems.background_color_cycle.is_ready(
               &self.context,
           );
        if is_ready && self.systems.background_color_cycle.on_begin_phase(
                    &self.context,
                )
                .inspect_err(|error| tracing::error!(?error, "BackgroundColorCycleSystem::on_begin_phase returned an error"))
                .is_ok()
        {
            // Apply BackgroundColorCycleSystem to SceneBackgroundArchetype
            self.systems.background_color_cycle.apply_many(
                &self.context,
                &mut self.archetypes.scene_background.background_colors,
            );

            self.systems.background_color_cycle.on_end_phase(
                &self.context,
            )
            .inspect_err(|error| tracing::error!(?error, "BackgroundColorCycleSystem::on_end_phase returned an error"))
            .ok();
        }

        // Process all commands.
        self.handle_commands();
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
        E: SystemPhaseEvents,
        Q: WorldCommandSender + WorldCommandReceiver
    {
        self.on_begin_update_phase();
        let result = SystemUpdatePhaseEvents::on_begin_phase(
            &mut self.events,
            &self.context,
        )
        .inspect_err(|error| tracing::error!(%error, "Failed to handle phase begin events"))
        .ok();
        // Only one system in this phase; skip spawning thread.
        self.apply_system_phase_update_without_events();

        SystemUpdatePhaseEvents::on_end_phase(
            &mut self.events,
            result,
            &self.context,
        )
        .inspect_err(|error| tracing::error!(%error, "Failed to handle phase end events"))
        .ok();

        // Process all commands.
        self.handle_commands();
        self.on_end_update_phase();
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
    pub fn apply_system_phase_render(&mut self)
    where
        E: SystemPhaseEvents,
        Q: WorldCommandSender + WorldCommandReceiver
    {
        self.on_begin_render_phase();
        let result = SystemRenderPhaseEvents::on_begin_phase(
            &mut self.events,
            &self.context,
            &mut self.states.wgpu_render,
        )
        .inspect_err(|error| tracing::error!(%error, "Failed to handle phase begin events"))
        .ok();

        self.apply_system_phase_render_without_events();

        SystemRenderPhaseEvents::on_end_phase(
            &mut self.events,
            result,
            &self.context,
            &mut self.states.wgpu_render,
        )
        .inspect_err(|error| tracing::error!(%error, "Failed to handle phase end events"))
        .ok();

        // Process all commands.
        self.handle_commands();
        self.on_end_render_phase();
    }

    /// Execute system phase `Render` without calling events. See [`apply_system_phase_render`](Self::apply_system_phase_render).
    #[inline]
    fn apply_system_phase_render_without_events(&mut self)
    where
        E: SystemPhaseEvents,
        Q: WorldCommandSender + WorldCommandReceiver
    {
        // System group 1 of 8
        let is_ready = self.systems.frame_globals.is_ready(
               &mut self.states.wgpu_render,
           );
        if is_ready && self.systems.frame_globals.on_begin_phase(
                    &mut self.states.wgpu_render,
                )
                .inspect_err(|error| tracing::error!(?error, "FrameGlobalsSystem::on_begin_phase returned an error"))
                .is_ok()
        {
            // Apply FrameGlobalsSystem to EnvironmentArchetype
            self.systems.frame_globals.apply_many(
                &mut self.states.wgpu_render,
                &self.archetypes.environment.time_of_daies,
                &self.archetypes.environment.suns,
                &self.archetypes.environment.fogs,
                &self.archetypes.environment.ambient_lights,
                &self.archetypes.environment.stars,
                &self.archetypes.environment.skydomes,
            );

            self.systems.frame_globals.on_end_phase(
                &mut self.states.wgpu_render,
            )
            .inspect_err(|error| tracing::error!(?error, "FrameGlobalsSystem::on_end_phase returned an error"))
            .ok();
        }
        // System group 2 of 8
        let is_ready = self.systems.render_background.is_ready(
               &mut self.states.wgpu_render,
           );
        if is_ready && self.systems.render_background.on_begin_phase(
                    &mut self.states.wgpu_render,
                )
                .inspect_err(|error| tracing::error!(?error, "RenderBackgroundSystem::on_begin_phase returned an error"))
                .is_ok()
        {
            // Apply RenderBackgroundSystem to SceneBackgroundArchetype
            self.systems.render_background.apply_many(
                &mut self.states.wgpu_render,
                &self.archetypes.scene_background.background_colors,
            );

            self.systems.render_background.on_end_phase(
                &mut self.states.wgpu_render,
            )
            .inspect_err(|error| tracing::error!(?error, "RenderBackgroundSystem::on_end_phase returned an error"))
            .ok();
        }
        // System group 3 of 8
        let is_ready = self.systems.render_sky.is_ready(
               &mut self.states.wgpu_render,
           );
        if is_ready && self.systems.render_sky.on_begin_phase(
                    &mut self.states.wgpu_render,
                )
                .inspect_err(|error| tracing::error!(?error, "RenderSkySystem::on_begin_phase returned an error"))
                .is_ok()
        {
            // Apply RenderSkySystem to EnvironmentArchetype
            self.systems.render_sky.apply_many(
                &mut self.states.wgpu_render,
                &self.archetypes.environment.skydomes,
            );

            self.systems.render_sky.on_end_phase(
                &mut self.states.wgpu_render,
            )
            .inspect_err(|error| tracing::error!(?error, "RenderSkySystem::on_end_phase returned an error"))
            .ok();
        }
        // System group 4 of 8
        let is_ready = self.systems.render.is_ready(
               &mut self.states.wgpu_render,
           );
        if is_ready && self.systems.render.on_begin_phase(
                    &mut self.states.wgpu_render,
                )
                .inspect_err(|error| tracing::error!(?error, "RenderSystem::on_begin_phase returned an error"))
                .is_ok()
        {
            // Apply RenderSystem to CubeArchetype
            self.systems.render.apply_many(
                &mut self.states.wgpu_render,
                &self.archetypes.cube.positions,
                &self.archetypes.cube.transforms,
                &self.archetypes.cube.models,
            );

            self.systems.render.on_end_phase(
                &mut self.states.wgpu_render,
            )
            .inspect_err(|error| tracing::error!(?error, "RenderSystem::on_end_phase returned an error"))
            .ok();
        }
        // System group 5 of 8
        let is_ready = self.systems.dynamic_light.is_ready(
               &self.context,
               &mut self.states.wgpu_render,
           );
        if is_ready && self.systems.dynamic_light.on_begin_phase(
                    &self.context,
                    &mut self.states.wgpu_render,
                )
                .inspect_err(|error| tracing::error!(?error, "DynamicLightSystem::on_begin_phase returned an error"))
                .is_ok()
        {
            // Apply DynamicLightSystem to DynamicLightArchetype
            self.systems.dynamic_light.apply_many(
                &self.context,
                &mut self.states.wgpu_render,
                &self.archetypes.dynamic_light.colors,
                &mut self.archetypes.dynamic_light.dynamic_lights,
            );

            self.systems.dynamic_light.on_end_phase(
                &self.context,
                &mut self.states.wgpu_render,
            )
            .inspect_err(|error| tracing::error!(?error, "DynamicLightSystem::on_end_phase returned an error"))
            .ok();
        }
        // System group 6 of 8
        let is_ready = self.systems.tile_render.is_ready(
               &mut self.states.wgpu_render,
           );
        if is_ready && self.systems.tile_render.on_begin_phase(
                    &mut self.states.wgpu_render,
                )
                .inspect_err(|error| tracing::error!(?error, "TileRenderSystem::on_begin_phase returned an error"))
                .is_ok()
        {
            // Apply TileRenderSystem to TileArchetype
            self.systems.tile_render.apply_many(
                &mut self.states.wgpu_render,
                &self.archetypes.tile.tiles,
                &self.archetypes.tile.positions,
            );

            self.systems.tile_render.on_end_phase(
                &mut self.states.wgpu_render,
            )
            .inspect_err(|error| tracing::error!(?error, "TileRenderSystem::on_end_phase returned an error"))
            .ok();
        }
        // System group 7 of 8
        let is_ready = self.systems.vga_blit.is_ready(
               &mut self.states.wgpu_render,
           );
        if is_ready && self.systems.vga_blit.on_begin_phase(
                    &mut self.states.wgpu_render,
                )
                .inspect_err(|error| tracing::error!(?error, "VgaBlitSystem::on_begin_phase returned an error"))
                .is_ok()
        {
            // Apply VgaBlitSystem to SceneBackgroundArchetype
            self.systems.vga_blit.apply_many(
                &mut self.states.wgpu_render,
                &self.archetypes.scene_background.background_colors,
            );

            self.systems.vga_blit.on_end_phase(
                &mut self.states.wgpu_render,
            )
            .inspect_err(|error| tracing::error!(?error, "VgaBlitSystem::on_end_phase returned an error"))
            .ok();
        }
        // System group 8 of 8
        let is_ready = self.systems.camera_render.is_ready(
               &self.states.wgpu_render,
               &mut self.states.active_camera,
           );
        if is_ready && self.systems.camera_render.on_begin_phase(
                    &self.states.wgpu_render,
                    &mut self.states.active_camera,
                )
                .inspect_err(|error| tracing::error!(?error, "CameraRenderSystem::on_begin_phase returned an error"))
                .is_ok()
        {
            // Apply CameraRenderSystem to CameraArchetype
            self.systems.camera_render.apply_many(
                &self.states.wgpu_render,
                &mut self.states.active_camera,
                &self.archetypes.camera.cameras,
            );

            self.systems.camera_render.on_end_phase(
                &self.states.wgpu_render,
                &mut self.states.active_camera,
            )
            .inspect_err(|error| tracing::error!(?error, "CameraRenderSystem::on_end_phase returned an error"))
            .ok();
        }

        // Process all commands.
        self.handle_commands();
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
    pub fn par_apply_system_phase_render(&mut self)
    where
        E: SystemPhaseEvents,
        Q: WorldCommandSender + WorldCommandReceiver
    {
        self.on_begin_render_phase();
        let result = SystemRenderPhaseEvents::on_begin_phase(
            &mut self.events,
            &self.context,
            &mut self.states.wgpu_render,
        )
        .inspect_err(|error| tracing::error!(%error, "Failed to handle phase begin events"))
        .ok();

        // System group 1
        let is_frame_globals_ready = self.systems.frame_globals.is_ready(
               &mut self.states.wgpu_render,
           );
        let is_frame_globals_ready = is_frame_globals_ready &
            self.systems.frame_globals.on_begin_phase(
                &mut self.states.wgpu_render,
            )
            .inspect_err(|error| tracing::error!(%error, "FrameGlobalsSystem::on_begin_phase returned an error"))
            .is_ok();

        {
            rayon::scope(|s| {
                // TODO: Instead of parallelizing systems (and then have them access archetypes), parallelize archetypes and apply all systems. This should improve data parallelism because it keeps caches hot.
                if is_frame_globals_ready {
                    s.spawn(|_| {
                        // Apply FrameGlobalsSystem to EnvironmentArchetype
                        self.systems.frame_globals.apply_many(
                            &mut self.states.wgpu_render,
                            &self.archetypes.environment.time_of_daies,
                            &self.archetypes.environment.suns,
                            &self.archetypes.environment.fogs,
                            &self.archetypes.environment.ambient_lights,
                            &self.archetypes.environment.stars,
                            &self.archetypes.environment.skydomes,
                        );
                    });
                }
            });
        }

        self.systems.frame_globals.on_end_phase(
            &mut self.states.wgpu_render,
        )
        .inspect_err(|error| tracing::error!(%error, "FrameGlobalsSystem::on_end_phase returned an error"))
        .ok();

        // System group 2
        let is_render_background_ready = self.systems.render_background.is_ready(
               &mut self.states.wgpu_render,
           );
        let is_render_background_ready = is_render_background_ready &
            self.systems.render_background.on_begin_phase(
                &mut self.states.wgpu_render,
            )
            .inspect_err(|error| tracing::error!(%error, "RenderBackgroundSystem::on_begin_phase returned an error"))
            .is_ok();

        {
            rayon::scope(|s| {
                // TODO: Instead of parallelizing systems (and then have them access archetypes), parallelize archetypes and apply all systems. This should improve data parallelism because it keeps caches hot.
                if is_render_background_ready {
                    s.spawn(|_| {
                        // Apply RenderBackgroundSystem to SceneBackgroundArchetype
                        self.systems.render_background.apply_many(
                            &mut self.states.wgpu_render,
                            &self.archetypes.scene_background.background_colors,
                        );
                    });
                }
            });
        }

        self.systems.render_background.on_end_phase(
            &mut self.states.wgpu_render,
        )
        .inspect_err(|error| tracing::error!(%error, "RenderBackgroundSystem::on_end_phase returned an error"))
        .ok();

        // System group 3
        let is_render_sky_ready = self.systems.render_sky.is_ready(
               &mut self.states.wgpu_render,
           );
        let is_render_sky_ready = is_render_sky_ready &
            self.systems.render_sky.on_begin_phase(
                &mut self.states.wgpu_render,
            )
            .inspect_err(|error| tracing::error!(%error, "RenderSkySystem::on_begin_phase returned an error"))
            .is_ok();

        {
            rayon::scope(|s| {
                // TODO: Instead of parallelizing systems (and then have them access archetypes), parallelize archetypes and apply all systems. This should improve data parallelism because it keeps caches hot.
                if is_render_sky_ready {
                    s.spawn(|_| {
                        // Apply RenderSkySystem to EnvironmentArchetype
                        self.systems.render_sky.apply_many(
                            &mut self.states.wgpu_render,
                            &self.archetypes.environment.skydomes,
                        );
                    });
                }
            });
        }

        self.systems.render_sky.on_end_phase(
            &mut self.states.wgpu_render,
        )
        .inspect_err(|error| tracing::error!(%error, "RenderSkySystem::on_end_phase returned an error"))
        .ok();

        // System group 4
        let is_render_ready = self.systems.render.is_ready(
               &mut self.states.wgpu_render,
           );
        let is_render_ready = is_render_ready &
            self.systems.render.on_begin_phase(
                &mut self.states.wgpu_render,
            )
            .inspect_err(|error| tracing::error!(%error, "RenderSystem::on_begin_phase returned an error"))
            .is_ok();

        {
            rayon::scope(|s| {
                // TODO: Instead of parallelizing systems (and then have them access archetypes), parallelize archetypes and apply all systems. This should improve data parallelism because it keeps caches hot.
                if is_render_ready {
                    s.spawn(|_| {
                        // Apply RenderSystem to CubeArchetype
                        self.systems.render.apply_many(
                            &mut self.states.wgpu_render,
                            &self.archetypes.cube.positions,
                            &self.archetypes.cube.transforms,
                            &self.archetypes.cube.models,
                        );
                    });
                }
            });
        }

        self.systems.render.on_end_phase(
            &mut self.states.wgpu_render,
        )
        .inspect_err(|error| tracing::error!(%error, "RenderSystem::on_end_phase returned an error"))
        .ok();

        // System group 5
        let is_dynamic_light_ready = self.systems.dynamic_light.is_ready(
               &self.context,
               &mut self.states.wgpu_render,
           );
        let is_dynamic_light_ready = is_dynamic_light_ready &
            self.systems.dynamic_light.on_begin_phase(
                &self.context,
                &mut self.states.wgpu_render,
            )
            .inspect_err(|error| tracing::error!(%error, "DynamicLightSystem::on_begin_phase returned an error"))
            .is_ok();

        {
            rayon::scope(|s| {
                // TODO: Instead of parallelizing systems (and then have them access archetypes), parallelize archetypes and apply all systems. This should improve data parallelism because it keeps caches hot.
                if is_dynamic_light_ready {
                    s.spawn(|_| {
                        // Apply DynamicLightSystem to DynamicLightArchetype
                        self.systems.dynamic_light.apply_many(
                            &self.context,
                            &mut self.states.wgpu_render,
                            &self.archetypes.dynamic_light.colors,
                            &mut self.archetypes.dynamic_light.dynamic_lights,
                        );
                    });
                }
            });
        }

        self.systems.dynamic_light.on_end_phase(
            &self.context,
            &mut self.states.wgpu_render,
        )
        .inspect_err(|error| tracing::error!(%error, "DynamicLightSystem::on_end_phase returned an error"))
        .ok();

        // System group 6
        let is_tile_render_ready = self.systems.tile_render.is_ready(
               &mut self.states.wgpu_render,
           );
        let is_tile_render_ready = is_tile_render_ready &
            self.systems.tile_render.on_begin_phase(
                &mut self.states.wgpu_render,
            )
            .inspect_err(|error| tracing::error!(%error, "TileRenderSystem::on_begin_phase returned an error"))
            .is_ok();

        {
            rayon::scope(|s| {
                // TODO: Instead of parallelizing systems (and then have them access archetypes), parallelize archetypes and apply all systems. This should improve data parallelism because it keeps caches hot.
                if is_tile_render_ready {
                    s.spawn(|_| {
                        // Apply TileRenderSystem to TileArchetype
                        self.systems.tile_render.apply_many(
                            &mut self.states.wgpu_render,
                            &self.archetypes.tile.tiles,
                            &self.archetypes.tile.positions,
                        );
                    });
                }
            });
        }

        self.systems.tile_render.on_end_phase(
            &mut self.states.wgpu_render,
        )
        .inspect_err(|error| tracing::error!(%error, "TileRenderSystem::on_end_phase returned an error"))
        .ok();

        // System group 7
        let is_vga_blit_ready = self.systems.vga_blit.is_ready(
               &mut self.states.wgpu_render,
           );
        let is_vga_blit_ready = is_vga_blit_ready &
            self.systems.vga_blit.on_begin_phase(
                &mut self.states.wgpu_render,
            )
            .inspect_err(|error| tracing::error!(%error, "VgaBlitSystem::on_begin_phase returned an error"))
            .is_ok();

        {
            rayon::scope(|s| {
                // TODO: Instead of parallelizing systems (and then have them access archetypes), parallelize archetypes and apply all systems. This should improve data parallelism because it keeps caches hot.
                if is_vga_blit_ready {
                    s.spawn(|_| {
                        // Apply VgaBlitSystem to SceneBackgroundArchetype
                        self.systems.vga_blit.apply_many(
                            &mut self.states.wgpu_render,
                            &self.archetypes.scene_background.background_colors,
                        );
                    });
                }
            });
        }

        self.systems.vga_blit.on_end_phase(
            &mut self.states.wgpu_render,
        )
        .inspect_err(|error| tracing::error!(%error, "VgaBlitSystem::on_end_phase returned an error"))
        .ok();

        // System group 8
        let is_camera_render_ready = self.systems.camera_render.is_ready(
               &self.states.wgpu_render,
               &mut self.states.active_camera,
           );
        let is_camera_render_ready = is_camera_render_ready &
            self.systems.camera_render.on_begin_phase(
                &self.states.wgpu_render,
                &mut self.states.active_camera,
            )
            .inspect_err(|error| tracing::error!(%error, "CameraRenderSystem::on_begin_phase returned an error"))
            .is_ok();

        {
            rayon::scope(|s| {
                // TODO: Instead of parallelizing systems (and then have them access archetypes), parallelize archetypes and apply all systems. This should improve data parallelism because it keeps caches hot.
                if is_camera_render_ready {
                    s.spawn(|_| {
                        // Apply CameraRenderSystem to CameraArchetype
                        self.systems.camera_render.apply_many(
                            &self.states.wgpu_render,
                            &mut self.states.active_camera,
                            &self.archetypes.camera.cameras,
                        );
                    });
                }
            });
        }

        self.systems.camera_render.on_end_phase(
            &self.states.wgpu_render,
            &mut self.states.active_camera,
        )
        .inspect_err(|error| tracing::error!(%error, "CameraRenderSystem::on_end_phase returned an error"))
        .ok();

        SystemRenderPhaseEvents::on_end_phase(
            &mut self.events,
            result,
            &self.context,
            &mut self.states.wgpu_render,
        )
        .inspect_err(|error| tracing::error!(%error, "Failed to handle phase end events"))
        .ok();

        // Process all commands.
        self.handle_commands();
        self.on_end_render_phase();
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
            ArchetypeEntityData::Environment(data) => {
                let id = self.spawn_environment(data);
                tracing::trace!(%id, "Spawned Environment entity");
            },
            ArchetypeEntityData::Camera(data) => {
                let id = self.spawn_camera(data);
                tracing::trace!(%id, "Spawned Camera entity");
            },
            ArchetypeEntityData::DynamicLight(data) => {
                let id = self.spawn_dynamic_light(data);
                tracing::trace!(%id, "Spawned DynamicLight entity");
            },
            ArchetypeEntityData::Tile(data) => {
                let id = self.spawn_tile(data);
                tracing::trace!(%id, "Spawned Tile entity");
            },
            ArchetypeEntityData::Cube(data) => {
                let id = self.spawn_cube(data);
                tracing::trace!(%id, "Spawned Cube entity");
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
            ArchetypeEntityData::SceneBackground(data) => {
                let id = self.spawn_scene_background(data);
                tracing::trace!(%id, "Spawned SceneBackground entity");
            },
            // TODO: Make it a non-panic on release builds?
            #[allow(unreachable_patterns)]
            unhandled => panic!("Cannot spawn entity for {id} which is not known to this world.", id = unhandled.archetype_id())
        }
    }

    fn handle_despawn_command(&mut self, id: EntityId) -> Result<(), DespawnError> {
         if let Some(loc) = self.entity_locations.remove(&id) {
            let result = match loc.archetype {
                EnvironmentArchetype::ID => {
                    self.archetypes
                        .environment
                        .drop_at_index(loc.index)
                        .map_err(|index| DespawnError::InvalidIndexInArchetype(index, EnvironmentArchetype::ID))?
                }
                CameraArchetype::ID => {
                    self.archetypes
                        .camera
                        .drop_at_index(loc.index)
                        .map_err(|index| DespawnError::InvalidIndexInArchetype(index, CameraArchetype::ID))?
                }
                DynamicLightArchetype::ID => {
                    self.archetypes
                        .dynamic_light
                        .drop_at_index(loc.index)
                        .map_err(|index| DespawnError::InvalidIndexInArchetype(index, DynamicLightArchetype::ID))?
                }
                TileArchetype::ID => {
                    self.archetypes
                        .tile
                        .drop_at_index(loc.index)
                        .map_err(|index| DespawnError::InvalidIndexInArchetype(index, TileArchetype::ID))?
                }
                CubeArchetype::ID => {
                    self.archetypes
                        .cube
                        .drop_at_index(loc.index)
                        .map_err(|index| DespawnError::InvalidIndexInArchetype(index, CubeArchetype::ID))?
                }
                PlayerArchetype::ID => {
                    self.archetypes
                        .player
                        .drop_at_index(loc.index)
                        .map_err(|index| DespawnError::InvalidIndexInArchetype(index, PlayerArchetype::ID))?
                }
                ForegroundObjectArchetype::ID => {
                    self.archetypes
                        .foreground_object
                        .drop_at_index(loc.index)
                        .map_err(|index| DespawnError::InvalidIndexInArchetype(index, ForegroundObjectArchetype::ID))?
                }
                BackgroundObjectArchetype::ID => {
                    self.archetypes
                        .background_object
                        .drop_at_index(loc.index)
                        .map_err(|index| DespawnError::InvalidIndexInArchetype(index, BackgroundObjectArchetype::ID))?
                }
                SceneBackgroundArchetype::ID => {
                    self.archetypes
                        .scene_background
                        .drop_at_index(loc.index)
                        .map_err(|index| DespawnError::InvalidIndexInArchetype(index, SceneBackgroundArchetype::ID))?
                }
                // TODO: Make it a non-panic on release builds?
                #[allow(unreachable_patterns)]
                archetype_id => unreachable!("Cannot remove entity {id} from archetype {archetype_id} which is not known to this world.")
            };

            if let Some(moved_entity) = result {
                self.entity_locations.insert(moved_entity, EntityArchetypeRef {
                    archetype: loc.archetype,
                    index: loc.index
                });
            }
         } else {
            return Err(DespawnError::EntityNotFound(id));
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
/// Delta time trackers for all phases.
#[derive(Debug, Clone)]
struct DeltaTimers {
    /// The current start time for the [`Startup`](StartupPhase) phase.
    current_startup_start: std::time::Instant,
    /// The previous start time for the [`Startup`](StartupPhase) phase.
    previous_startup_start: std::time::Instant,
    /// The current start time for the [`FixedUpdate`](FixedUpdatePhase) phase.
    current_fixed_update_start: std::time::Instant,
    /// The previous start time for the [`FixedUpdate`](FixedUpdatePhase) phase.
    previous_fixed_update_start: std::time::Instant,
    /// The current start time for the [`Update`](UpdatePhase) phase.
    current_update_start: std::time::Instant,
    /// The previous start time for the [`Update`](UpdatePhase) phase.
    previous_update_start: std::time::Instant,
    /// The current start time for the [`Render`](RenderPhase) phase.
    current_render_start: std::time::Instant,
    /// The previous start time for the [`Render`](RenderPhase) phase.
    previous_render_start: std::time::Instant,
}

impl Default for DeltaTimers {
    fn default() -> Self {
        let now = std::time::Instant::now();
        Self {
            current_startup_start: now,
            previous_startup_start: now,
            current_fixed_update_start: now,
            previous_fixed_update_start: now,
            current_update_start: now,
            previous_update_start: now,
            current_render_start: now,
            previous_render_start: now,
        }
    }
}

#[derive(Debug)]
pub enum DespawnError {
    EntityNotFound(EntityId),
    InvalidIndexInArchetype(usize, ArchetypeId)
}

impl core::fmt::Display for DespawnError {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> Result<(), core::fmt::Error> {
        match self {
            Self::EntityNotFound(id) => {
                write!(f, "Failed to despawn unknown entity {id}")
            }
            Self::InvalidIndexInArchetype(index, aid) => {
                write!(f, "Failed to despawn entity at invalid index {index} in archetype {aid}")
            }
        }
    }
}
impl core::error::Error for DespawnError { }

impl<E, Q> MainWorld<E, Q> {
    /// Frontloads entities provided by their archetype IDs and indexes (e.g., from quadtree results,
    /// followed by entity location lookups). Performs best when the indices are presorted in ascending order.
    ///
    /// Returns the count of frontloaded entities.
    #[allow(dead_code)]
    pub fn frontload(
        &mut self,
        archetype: ArchetypeId,
        indices_to_frontload: &[usize],
        previous_frontload_pivot: Option<usize>
    ) -> usize {
        match archetype {
            ArchetypeId::Environment => {
                self.frontload_environments_by_indices_sorted(
                    indices_to_frontload,
                    previous_frontload_pivot
                )
            },
            ArchetypeId::Camera => {
                self.frontload_cameras_by_indices_sorted(
                    indices_to_frontload,
                    previous_frontload_pivot
                )
            },
            ArchetypeId::DynamicLight => {
                self.frontload_dynamic_lights_by_indices_sorted(
                    indices_to_frontload,
                    previous_frontload_pivot
                )
            },
            ArchetypeId::Tile => {
                self.frontload_tiles_by_indices_sorted(
                    indices_to_frontload,
                    previous_frontload_pivot
                )
            },
            ArchetypeId::Cube => {
                self.frontload_cubes_by_indices_sorted(
                    indices_to_frontload,
                    previous_frontload_pivot
                )
            },
            ArchetypeId::Player => {
                self.frontload_players_by_indices_sorted(
                    indices_to_frontload,
                    previous_frontload_pivot
                )
            },
            ArchetypeId::ForegroundObject => {
                self.frontload_foreground_objects_by_indices_sorted(
                    indices_to_frontload,
                    previous_frontload_pivot
                )
            },
            ArchetypeId::BackgroundObject => {
                self.frontload_background_objects_by_indices_sorted(
                    indices_to_frontload,
                    previous_frontload_pivot
                )
            },
            ArchetypeId::SceneBackground => {
                self.frontload_scene_backgrounds_by_indices_sorted(
                    indices_to_frontload,
                    previous_frontload_pivot
                )
            },
            #[allow(unreachable_patterns)]
            _ => {
                0
            }
        }
    }

    /// Frontloads entities provided by their IDs (e.g., from quadtree results).
    /// Returns the count of frontloaded entities.
    #[inline(always)]
    #[allow(dead_code)]
    pub fn frontload_environments(
        &mut self,
        entities_to_frontload: &[EntityId],
        previous_frontload_pivot: Option<usize>
    ) -> usize {
        self.archetypes
            .environment
            .frontload(
                &mut self.entity_locations,
                entities_to_frontload,
                previous_frontload_pivot
            )
    }

    /// Frontloads entities provided by their IDs (e.g., from quadtree results).
    ///
    /// This function operates directly on indices and assumes that the index list is sorted
    /// for coherent cache access. It works with unsorted lists but may have degraded performance.
    ///
    /// Returns the count of frontloaded entities.
    #[inline(always)]
    #[allow(dead_code)]
    pub fn frontload_environments_by_indices_sorted<I>(
        &mut self,
        indices_to_frontload: I,
        previous_frontload_pivot: Option<usize>
    ) -> usize
    where
        I: AsRef<[usize]>
    {
        self.archetypes
            .environment
            .frontload_by_indices_sorted(
                &mut self.entity_locations,
                indices_to_frontload,
                previous_frontload_pivot
            )
    }

    /// Frontloads entities satisfying `should_frontload`.
    /// Returns the number of entities frontloaded.
    #[inline(always)]
    #[allow(dead_code)]
    pub fn frontload_environments_by_scan<F>(
        &mut self,
        should_frontload: F,
    ) -> usize
    where
        for<'archetype> F: FnMut(&EnvironmentEntityRef<'archetype>) -> bool,
    {
        self.archetypes
            .environment
            .frontload_scan(
                &mut self.entity_locations,
                should_frontload
            )
    }

    /// Frontloads entities provided by their IDs (e.g., from quadtree results).
    /// Returns the count of frontloaded entities.
    #[inline(always)]
    #[allow(dead_code)]
    pub fn frontload_cameras(
        &mut self,
        entities_to_frontload: &[EntityId],
        previous_frontload_pivot: Option<usize>
    ) -> usize {
        self.archetypes
            .camera
            .frontload(
                &mut self.entity_locations,
                entities_to_frontload,
                previous_frontload_pivot
            )
    }

    /// Frontloads entities provided by their IDs (e.g., from quadtree results).
    ///
    /// This function operates directly on indices and assumes that the index list is sorted
    /// for coherent cache access. It works with unsorted lists but may have degraded performance.
    ///
    /// Returns the count of frontloaded entities.
    #[inline(always)]
    #[allow(dead_code)]
    pub fn frontload_cameras_by_indices_sorted<I>(
        &mut self,
        indices_to_frontload: I,
        previous_frontload_pivot: Option<usize>
    ) -> usize
    where
        I: AsRef<[usize]>
    {
        self.archetypes
            .camera
            .frontload_by_indices_sorted(
                &mut self.entity_locations,
                indices_to_frontload,
                previous_frontload_pivot
            )
    }

    /// Frontloads entities satisfying `should_frontload`.
    /// Returns the number of entities frontloaded.
    #[inline(always)]
    #[allow(dead_code)]
    pub fn frontload_cameras_by_scan<F>(
        &mut self,
        should_frontload: F,
    ) -> usize
    where
        for<'archetype> F: FnMut(&CameraEntityRef<'archetype>) -> bool,
    {
        self.archetypes
            .camera
            .frontload_scan(
                &mut self.entity_locations,
                should_frontload
            )
    }

    /// Frontloads entities provided by their IDs (e.g., from quadtree results).
    /// Returns the count of frontloaded entities.
    #[inline(always)]
    #[allow(dead_code)]
    pub fn frontload_dynamic_lights(
        &mut self,
        entities_to_frontload: &[EntityId],
        previous_frontload_pivot: Option<usize>
    ) -> usize {
        self.archetypes
            .dynamic_light
            .frontload(
                &mut self.entity_locations,
                entities_to_frontload,
                previous_frontload_pivot
            )
    }

    /// Frontloads entities provided by their IDs (e.g., from quadtree results).
    ///
    /// This function operates directly on indices and assumes that the index list is sorted
    /// for coherent cache access. It works with unsorted lists but may have degraded performance.
    ///
    /// Returns the count of frontloaded entities.
    #[inline(always)]
    #[allow(dead_code)]
    pub fn frontload_dynamic_lights_by_indices_sorted<I>(
        &mut self,
        indices_to_frontload: I,
        previous_frontload_pivot: Option<usize>
    ) -> usize
    where
        I: AsRef<[usize]>
    {
        self.archetypes
            .dynamic_light
            .frontload_by_indices_sorted(
                &mut self.entity_locations,
                indices_to_frontload,
                previous_frontload_pivot
            )
    }

    /// Frontloads entities satisfying `should_frontload`.
    /// Returns the number of entities frontloaded.
    #[inline(always)]
    #[allow(dead_code)]
    pub fn frontload_dynamic_lights_by_scan<F>(
        &mut self,
        should_frontload: F,
    ) -> usize
    where
        for<'archetype> F: FnMut(&DynamicLightEntityRef<'archetype>) -> bool,
    {
        self.archetypes
            .dynamic_light
            .frontload_scan(
                &mut self.entity_locations,
                should_frontload
            )
    }

    /// Frontloads entities provided by their IDs (e.g., from quadtree results).
    /// Returns the count of frontloaded entities.
    #[inline(always)]
    #[allow(dead_code)]
    pub fn frontload_tiles(
        &mut self,
        entities_to_frontload: &[EntityId],
        previous_frontload_pivot: Option<usize>
    ) -> usize {
        self.archetypes
            .tile
            .frontload(
                &mut self.entity_locations,
                entities_to_frontload,
                previous_frontload_pivot
            )
    }

    /// Frontloads entities provided by their IDs (e.g., from quadtree results).
    ///
    /// This function operates directly on indices and assumes that the index list is sorted
    /// for coherent cache access. It works with unsorted lists but may have degraded performance.
    ///
    /// Returns the count of frontloaded entities.
    #[inline(always)]
    #[allow(dead_code)]
    pub fn frontload_tiles_by_indices_sorted<I>(
        &mut self,
        indices_to_frontload: I,
        previous_frontload_pivot: Option<usize>
    ) -> usize
    where
        I: AsRef<[usize]>
    {
        self.archetypes
            .tile
            .frontload_by_indices_sorted(
                &mut self.entity_locations,
                indices_to_frontload,
                previous_frontload_pivot
            )
    }

    /// Frontloads entities satisfying `should_frontload`.
    /// Returns the number of entities frontloaded.
    #[inline(always)]
    #[allow(dead_code)]
    pub fn frontload_tiles_by_scan<F>(
        &mut self,
        should_frontload: F,
    ) -> usize
    where
        for<'archetype> F: FnMut(&TileEntityRef<'archetype>) -> bool,
    {
        self.archetypes
            .tile
            .frontload_scan(
                &mut self.entity_locations,
                should_frontload
            )
    }

    /// Frontloads entities provided by their IDs (e.g., from quadtree results).
    /// Returns the count of frontloaded entities.
    #[inline(always)]
    #[allow(dead_code)]
    pub fn frontload_cubes(
        &mut self,
        entities_to_frontload: &[EntityId],
        previous_frontload_pivot: Option<usize>
    ) -> usize {
        self.archetypes
            .cube
            .frontload(
                &mut self.entity_locations,
                entities_to_frontload,
                previous_frontload_pivot
            )
    }

    /// Frontloads entities provided by their IDs (e.g., from quadtree results).
    ///
    /// This function operates directly on indices and assumes that the index list is sorted
    /// for coherent cache access. It works with unsorted lists but may have degraded performance.
    ///
    /// Returns the count of frontloaded entities.
    #[inline(always)]
    #[allow(dead_code)]
    pub fn frontload_cubes_by_indices_sorted<I>(
        &mut self,
        indices_to_frontload: I,
        previous_frontload_pivot: Option<usize>
    ) -> usize
    where
        I: AsRef<[usize]>
    {
        self.archetypes
            .cube
            .frontload_by_indices_sorted(
                &mut self.entity_locations,
                indices_to_frontload,
                previous_frontload_pivot
            )
    }

    /// Frontloads entities satisfying `should_frontload`.
    /// Returns the number of entities frontloaded.
    #[inline(always)]
    #[allow(dead_code)]
    pub fn frontload_cubes_by_scan<F>(
        &mut self,
        should_frontload: F,
    ) -> usize
    where
        for<'archetype> F: FnMut(&CubeEntityRef<'archetype>) -> bool,
    {
        self.archetypes
            .cube
            .frontload_scan(
                &mut self.entity_locations,
                should_frontload
            )
    }

    /// Frontloads entities provided by their IDs (e.g., from quadtree results).
    /// Returns the count of frontloaded entities.
    #[inline(always)]
    #[allow(dead_code)]
    pub fn frontload_players(
        &mut self,
        entities_to_frontload: &[EntityId],
        previous_frontload_pivot: Option<usize>
    ) -> usize {
        self.archetypes
            .player
            .frontload(
                &mut self.entity_locations,
                entities_to_frontload,
                previous_frontload_pivot
            )
    }

    /// Frontloads entities provided by their IDs (e.g., from quadtree results).
    ///
    /// This function operates directly on indices and assumes that the index list is sorted
    /// for coherent cache access. It works with unsorted lists but may have degraded performance.
    ///
    /// Returns the count of frontloaded entities.
    #[inline(always)]
    #[allow(dead_code)]
    pub fn frontload_players_by_indices_sorted<I>(
        &mut self,
        indices_to_frontload: I,
        previous_frontload_pivot: Option<usize>
    ) -> usize
    where
        I: AsRef<[usize]>
    {
        self.archetypes
            .player
            .frontload_by_indices_sorted(
                &mut self.entity_locations,
                indices_to_frontload,
                previous_frontload_pivot
            )
    }

    /// Frontloads entities satisfying `should_frontload`.
    /// Returns the number of entities frontloaded.
    #[inline(always)]
    #[allow(dead_code)]
    pub fn frontload_players_by_scan<F>(
        &mut self,
        should_frontload: F,
    ) -> usize
    where
        for<'archetype> F: FnMut(&PlayerEntityRef<'archetype>) -> bool,
    {
        self.archetypes
            .player
            .frontload_scan(
                &mut self.entity_locations,
                should_frontload
            )
    }

    /// Frontloads entities provided by their IDs (e.g., from quadtree results).
    /// Returns the count of frontloaded entities.
    #[inline(always)]
    #[allow(dead_code)]
    pub fn frontload_foreground_objects(
        &mut self,
        entities_to_frontload: &[EntityId],
        previous_frontload_pivot: Option<usize>
    ) -> usize {
        self.archetypes
            .foreground_object
            .frontload(
                &mut self.entity_locations,
                entities_to_frontload,
                previous_frontload_pivot
            )
    }

    /// Frontloads entities provided by their IDs (e.g., from quadtree results).
    ///
    /// This function operates directly on indices and assumes that the index list is sorted
    /// for coherent cache access. It works with unsorted lists but may have degraded performance.
    ///
    /// Returns the count of frontloaded entities.
    #[inline(always)]
    #[allow(dead_code)]
    pub fn frontload_foreground_objects_by_indices_sorted<I>(
        &mut self,
        indices_to_frontload: I,
        previous_frontload_pivot: Option<usize>
    ) -> usize
    where
        I: AsRef<[usize]>
    {
        self.archetypes
            .foreground_object
            .frontload_by_indices_sorted(
                &mut self.entity_locations,
                indices_to_frontload,
                previous_frontload_pivot
            )
    }

    /// Frontloads entities satisfying `should_frontload`.
    /// Returns the number of entities frontloaded.
    #[inline(always)]
    #[allow(dead_code)]
    pub fn frontload_foreground_objects_by_scan<F>(
        &mut self,
        should_frontload: F,
    ) -> usize
    where
        for<'archetype> F: FnMut(&ForegroundObjectEntityRef<'archetype>) -> bool,
    {
        self.archetypes
            .foreground_object
            .frontload_scan(
                &mut self.entity_locations,
                should_frontload
            )
    }

    /// Frontloads entities provided by their IDs (e.g., from quadtree results).
    /// Returns the count of frontloaded entities.
    #[inline(always)]
    #[allow(dead_code)]
    pub fn frontload_background_objects(
        &mut self,
        entities_to_frontload: &[EntityId],
        previous_frontload_pivot: Option<usize>
    ) -> usize {
        self.archetypes
            .background_object
            .frontload(
                &mut self.entity_locations,
                entities_to_frontload,
                previous_frontload_pivot
            )
    }

    /// Frontloads entities provided by their IDs (e.g., from quadtree results).
    ///
    /// This function operates directly on indices and assumes that the index list is sorted
    /// for coherent cache access. It works with unsorted lists but may have degraded performance.
    ///
    /// Returns the count of frontloaded entities.
    #[inline(always)]
    #[allow(dead_code)]
    pub fn frontload_background_objects_by_indices_sorted<I>(
        &mut self,
        indices_to_frontload: I,
        previous_frontload_pivot: Option<usize>
    ) -> usize
    where
        I: AsRef<[usize]>
    {
        self.archetypes
            .background_object
            .frontload_by_indices_sorted(
                &mut self.entity_locations,
                indices_to_frontload,
                previous_frontload_pivot
            )
    }

    /// Frontloads entities satisfying `should_frontload`.
    /// Returns the number of entities frontloaded.
    #[inline(always)]
    #[allow(dead_code)]
    pub fn frontload_background_objects_by_scan<F>(
        &mut self,
        should_frontload: F,
    ) -> usize
    where
        for<'archetype> F: FnMut(&BackgroundObjectEntityRef<'archetype>) -> bool,
    {
        self.archetypes
            .background_object
            .frontload_scan(
                &mut self.entity_locations,
                should_frontload
            )
    }

    /// Frontloads entities provided by their IDs (e.g., from quadtree results).
    /// Returns the count of frontloaded entities.
    #[inline(always)]
    #[allow(dead_code)]
    pub fn frontload_scene_backgrounds(
        &mut self,
        entities_to_frontload: &[EntityId],
        previous_frontload_pivot: Option<usize>
    ) -> usize {
        self.archetypes
            .scene_background
            .frontload(
                &mut self.entity_locations,
                entities_to_frontload,
                previous_frontload_pivot
            )
    }

    /// Frontloads entities provided by their IDs (e.g., from quadtree results).
    ///
    /// This function operates directly on indices and assumes that the index list is sorted
    /// for coherent cache access. It works with unsorted lists but may have degraded performance.
    ///
    /// Returns the count of frontloaded entities.
    #[inline(always)]
    #[allow(dead_code)]
    pub fn frontload_scene_backgrounds_by_indices_sorted<I>(
        &mut self,
        indices_to_frontload: I,
        previous_frontload_pivot: Option<usize>
    ) -> usize
    where
        I: AsRef<[usize]>
    {
        self.archetypes
            .scene_background
            .frontload_by_indices_sorted(
                &mut self.entity_locations,
                indices_to_frontload,
                previous_frontload_pivot
            )
    }

    /// Frontloads entities satisfying `should_frontload`.
    /// Returns the number of entities frontloaded.
    #[inline(always)]
    #[allow(dead_code)]
    pub fn frontload_scene_backgrounds_by_scan<F>(
        &mut self,
        should_frontload: F,
    ) -> usize
    where
        for<'archetype> F: FnMut(&SceneBackgroundEntityRef<'archetype>) -> bool,
    {
        self.archetypes
            .scene_background
            .frontload_scan(
                &mut self.entity_locations,
                should_frontload
            )
    }
}