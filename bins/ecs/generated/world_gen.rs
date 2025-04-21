/// A command in the [`World`].
#[allow(dead_code)]
pub enum WorldCommand<UserCommand> {
    /// Spawn an entity.
    ///
    /// # Note:
    /// Due to the reusability of systems, not all commands are valid in all worlds.
    SpawnEntity(ArchetypeEntityData),
    /// Despawn an entity.
    DespawnEntity(::sillyecs::EntityId),
    /// A user-specific command.
    User(UserCommand)
}

pub trait WorldUserCommand {
    type UserCommand: core::marker::Send + core::fmt::Debug;
}

/// Sender of [`WorldCommand`] instances.
#[allow(dead_code)]
pub trait WorldCommandSender: WorldUserCommand {
    type Error: core::error::Error;

    /// Sends a command. May block until sending is complete.
    fn send(&self, command: WorldCommand<Self::UserCommand>) -> Result<(), Self::Error>;
}

/// Sender of [`WorldCommand`] instances.
#[allow(dead_code)]
pub trait WorldCommandReceiver: WorldUserCommand {
    type Error: core::error::Error;

    /// Attempts to receive a message from the channel without blocking.
    /// Returns [`Ok(None)`] if no command was queued.
    fn recv(&self) -> Result<Option<WorldCommand<Self::UserCommand>>, Self::Error>;
}

/// Handler of a [`WorldCommand`]'s user command.
#[allow(dead_code)]
pub trait WorldUserCommandHandler: WorldUserCommand {
    /// Handles a user command.
    fn handle_user_command(&mut self, command: Self::UserCommand);
}

/// A sender and receiver of [`WorldCommand`] instances.
#[allow(dead_code)]
pub trait WorldCommandQueue: WorldCommandSender + WorldCommandReceiver { }

impl<T, U> WorldCommandQueue for T
where
    T: WorldCommandSender<UserCommand = U> + WorldCommandReceiver<UserCommand = U>
{ }


/// A world  all archetypes.
#[derive(Debug)]
pub struct MainWorld<E, Q> {
    /// The archetypes.
    archetypes: MainWorldArchetypes,
    /// The systems.
    pub systems: MainWorldSystems,
    /// The frame context.
    context: ::sillyecs::FrameContext,
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

impl<E, Q> ::sillyecs::World for MainWorld<E, Q> {
    const ID: ::sillyecs::WorldId = ::sillyecs::WorldId::new_from(core::num::NonZeroU64::new(1).expect("Invalid ID on ECS construction time"));
}

/// The user states used in the world.
///
/// It encapsulates ownership of and access to the following state types:
/// 
/// - [WgpuRenderState](WgpuRenderState)
/// - [ResourceManagerState](ResourceManagerState)
/// - [InputState](InputState)
/// - [ActiveCameraState](ActiveCameraState)
#[derive(Debug, Default)]
pub struct MainWorldStates {
    /// The WGPU render state; will be initialized in the Render phase hooks
    ///
    /// The state is used in the following systems:
    /// - [`WgpuInitTextureSystem`]
    /// - [`WgpuInitUniformBindGroupSystem`]
    /// - [`WgpuInitShaderSystem`]
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
    /// The WGPU resource manager
    ///
    /// The state is used in the following systems:
    /// - [`WgpuInitTextureSystem`]
    /// - [`WgpuInitUniformBindGroupSystem`]
    /// - [`WgpuInitShaderSystem`]
    pub resource_manager: ResourceManagerState,
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

#[allow(dead_code)]
impl MainWorldStates {
    pub const fn new(
        wgpu_render: WgpuRenderState,
        resource_manager: ResourceManagerState,
        input: InputState,
        active_camera: ActiveCameraState,
    ) -> Self {
        Self {
            wgpu_render,
            resource_manager,
            input,
            active_camera,
        }
    }
}

/// The archetypes used in the world.
#[derive(Debug, Clone, Default)]
struct MainWorldArchetypes {
    /// The entity locator.
    //
    // Hashmap type not provided by design. Provide your own implementation such as fxhash::FxHashMap via type alias.
    // Example:
    //      type EntityLocationMap<K, V> = fxhash::FxHashMap<K, V>;
    //
    entity_locations: EntityLocationMap<::sillyecs::EntityId, EntityArchetypeRef>,

    pub collection: MainWorldArchetypeCollection
}

/// The archetypes used in the world.
#[derive(Debug, Clone, Default)]
struct MainWorldArchetypeCollection {
    pub wgpu_uniform_bind_group: WgpuUniformBindGroupArchetype,
    pub wgpu_shader: WgpuShaderArchetype,
    pub wgpu_texture: WgpuTextureArchetype,
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
    wgpu_init_uniform_bind_group: WgpuInitUniformBindGroupSystem,
    wgpu_init_shader: WgpuInitShaderSystem,
    wgpu_init_texture: WgpuInitTextureSystem,
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

    /// Gives immutable access to the [`WgpuInitUniformBindGroupSystemData`] of the [`WgpuInitUniformBindGroupSystem`].
    #[inline]
    pub fn as_wgpu_init_uniform_bind_group_ref(&self) -> &WgpuInitUniformBindGroupSystemData {
        use core::ops::Deref;
        self.wgpu_init_uniform_bind_group.deref()
    }

    /// Gives mutable access to the [`WgpuInitUniformBindGroupSystemData`] of the [`WgpuInitUniformBindGroupSystem`].
    #[inline]
    pub fn as_wgpu_init_uniform_bind_group_mut(&mut self) -> &mut WgpuInitUniformBindGroupSystemData {
        use core::ops::DerefMut;
        self.wgpu_init_uniform_bind_group.deref_mut()
    }

    /// Gives immutable access to the [`WgpuInitShaderSystemData`] of the [`WgpuInitShaderSystem`].
    #[inline]
    pub fn as_wgpu_init_shader_ref(&self) -> &WgpuInitShaderSystemData {
        use core::ops::Deref;
        self.wgpu_init_shader.deref()
    }

    /// Gives mutable access to the [`WgpuInitShaderSystemData`] of the [`WgpuInitShaderSystem`].
    #[inline]
    pub fn as_wgpu_init_shader_mut(&mut self) -> &mut WgpuInitShaderSystemData {
        use core::ops::DerefMut;
        self.wgpu_init_shader.deref_mut()
    }

    /// Gives immutable access to the [`WgpuInitTextureSystemData`] of the [`WgpuInitTextureSystem`].
    #[inline]
    pub fn as_wgpu_init_texture_ref(&self) -> &WgpuInitTextureSystemData {
        use core::ops::Deref;
        self.wgpu_init_texture.deref()
    }

    /// Gives mutable access to the [`WgpuInitTextureSystemData`] of the [`WgpuInitTextureSystem`].
    #[inline]
    pub fn as_wgpu_init_texture_mut(&mut self) -> &mut WgpuInitTextureSystemData {
        use core::ops::DerefMut;
        self.wgpu_init_texture.deref_mut()
    }

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

impl AsRef<WgpuInitUniformBindGroupSystemData> for MainWorldSystems {
    #[inline]
    fn as_ref(&self) -> &WgpuInitUniformBindGroupSystemData {
        self.as_wgpu_init_uniform_bind_group_ref()
    }
}

impl AsMut<WgpuInitUniformBindGroupSystemData> for MainWorldSystems {
    #[inline]
    fn as_mut(&mut self) -> &mut WgpuInitUniformBindGroupSystemData {
        self.as_wgpu_init_uniform_bind_group_mut()
    }
}

impl AsRef<WgpuInitShaderSystemData> for MainWorldSystems {
    #[inline]
    fn as_ref(&self) -> &WgpuInitShaderSystemData {
        self.as_wgpu_init_shader_ref()
    }
}

impl AsMut<WgpuInitShaderSystemData> for MainWorldSystems {
    #[inline]
    fn as_mut(&mut self) -> &mut WgpuInitShaderSystemData {
        self.as_wgpu_init_shader_mut()
    }
}

impl AsRef<WgpuInitTextureSystemData> for MainWorldSystems {
    #[inline]
    fn as_ref(&self) -> &WgpuInitTextureSystemData {
        self.as_wgpu_init_texture_ref()
    }
}

impl AsMut<WgpuInitTextureSystemData> for MainWorldSystems {
    #[inline]
    fn as_mut(&mut self) -> &mut WgpuInitTextureSystemData {
        self.as_wgpu_init_texture_mut()
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
impl<E, Q> Spawn<WgpuUniformBindGroupEntityData> for MainWorld<E, Q> {
    /// Spawn a new entity into the world.
    #[inline]
    fn spawn(&mut self, data: WgpuUniformBindGroupEntityData) -> ::sillyecs::EntityId {
        self.spawn_wgpu_uniform_bind_group(data)
    }
}

/// Spawns an entity into the world.
impl<E, Q> Spawn<WgpuUniformBindGroupEntityComponents> for MainWorld<E, Q> {
    /// Spawn a new entity into the world.
    #[inline]
    fn spawn(&mut self, data: WgpuUniformBindGroupEntityComponents) -> ::sillyecs::EntityId {
        self.spawn_wgpu_uniform_bind_group(data)
    }
}

/// Spawns an entity into the world.
impl<E, Q> Spawn<WgpuShaderEntityData> for MainWorld<E, Q> {
    /// Spawn a new entity into the world.
    #[inline]
    fn spawn(&mut self, data: WgpuShaderEntityData) -> ::sillyecs::EntityId {
        self.spawn_wgpu_shader(data)
    }
}

/// Spawns an entity into the world.
impl<E, Q> Spawn<WgpuShaderEntityComponents> for MainWorld<E, Q> {
    /// Spawn a new entity into the world.
    #[inline]
    fn spawn(&mut self, data: WgpuShaderEntityComponents) -> ::sillyecs::EntityId {
        self.spawn_wgpu_shader(data)
    }
}

/// Spawns an entity into the world.
impl<E, Q> Spawn<WgpuTextureEntityData> for MainWorld<E, Q> {
    /// Spawn a new entity into the world.
    #[inline]
    fn spawn(&mut self, data: WgpuTextureEntityData) -> ::sillyecs::EntityId {
        self.spawn_wgpu_texture(data)
    }
}

/// Spawns an entity into the world.
impl<E, Q> Spawn<WgpuTextureEntityComponents> for MainWorld<E, Q> {
    /// Spawn a new entity into the world.
    #[inline]
    fn spawn(&mut self, data: WgpuTextureEntityComponents) -> ::sillyecs::EntityId {
        self.spawn_wgpu_texture(data)
    }
}

/// Spawns an entity into the world.
impl<E, Q> Spawn<EnvironmentEntityData> for MainWorld<E, Q> {
    /// Spawn a new entity into the world.
    #[inline]
    fn spawn(&mut self, data: EnvironmentEntityData) -> ::sillyecs::EntityId {
        self.spawn_environment(data)
    }
}

/// Spawns an entity into the world.
impl<E, Q> Spawn<EnvironmentEntityComponents> for MainWorld<E, Q> {
    /// Spawn a new entity into the world.
    #[inline]
    fn spawn(&mut self, data: EnvironmentEntityComponents) -> ::sillyecs::EntityId {
        self.spawn_environment(data)
    }
}

/// Spawns an entity into the world.
impl<E, Q> Spawn<CameraEntityData> for MainWorld<E, Q> {
    /// Spawn a new entity into the world.
    #[inline]
    fn spawn(&mut self, data: CameraEntityData) -> ::sillyecs::EntityId {
        self.spawn_camera(data)
    }
}

/// Spawns an entity into the world.
impl<E, Q> Spawn<CameraEntityComponents> for MainWorld<E, Q> {
    /// Spawn a new entity into the world.
    #[inline]
    fn spawn(&mut self, data: CameraEntityComponents) -> ::sillyecs::EntityId {
        self.spawn_camera(data)
    }
}

/// Spawns an entity into the world.
impl<E, Q> Spawn<DynamicLightEntityData> for MainWorld<E, Q> {
    /// Spawn a new entity into the world.
    #[inline]
    fn spawn(&mut self, data: DynamicLightEntityData) -> ::sillyecs::EntityId {
        self.spawn_dynamic_light(data)
    }
}

/// Spawns an entity into the world.
impl<E, Q> Spawn<DynamicLightEntityComponents> for MainWorld<E, Q> {
    /// Spawn a new entity into the world.
    #[inline]
    fn spawn(&mut self, data: DynamicLightEntityComponents) -> ::sillyecs::EntityId {
        self.spawn_dynamic_light(data)
    }
}

/// Spawns an entity into the world.
impl<E, Q> Spawn<TileEntityData> for MainWorld<E, Q> {
    /// Spawn a new entity into the world.
    #[inline]
    fn spawn(&mut self, data: TileEntityData) -> ::sillyecs::EntityId {
        self.spawn_tile(data)
    }
}

/// Spawns an entity into the world.
impl<E, Q> Spawn<TileEntityComponents> for MainWorld<E, Q> {
    /// Spawn a new entity into the world.
    #[inline]
    fn spawn(&mut self, data: TileEntityComponents) -> ::sillyecs::EntityId {
        self.spawn_tile(data)
    }
}

/// Spawns an entity into the world.
impl<E, Q> Spawn<CubeEntityData> for MainWorld<E, Q> {
    /// Spawn a new entity into the world.
    #[inline]
    fn spawn(&mut self, data: CubeEntityData) -> ::sillyecs::EntityId {
        self.spawn_cube(data)
    }
}

/// Spawns an entity into the world.
impl<E, Q> Spawn<CubeEntityComponents> for MainWorld<E, Q> {
    /// Spawn a new entity into the world.
    #[inline]
    fn spawn(&mut self, data: CubeEntityComponents) -> ::sillyecs::EntityId {
        self.spawn_cube(data)
    }
}

/// Spawns an entity into the world.
impl<E, Q> Spawn<PlayerEntityData> for MainWorld<E, Q> {
    /// Spawn a new entity into the world.
    #[inline]
    fn spawn(&mut self, data: PlayerEntityData) -> ::sillyecs::EntityId {
        self.spawn_player(data)
    }
}

/// Spawns an entity into the world.
impl<E, Q> Spawn<PlayerEntityComponents> for MainWorld<E, Q> {
    /// Spawn a new entity into the world.
    #[inline]
    fn spawn(&mut self, data: PlayerEntityComponents) -> ::sillyecs::EntityId {
        self.spawn_player(data)
    }
}

/// Spawns an entity into the world.
impl<E, Q> Spawn<ForegroundObjectEntityData> for MainWorld<E, Q> {
    /// Spawn a new entity into the world.
    #[inline]
    fn spawn(&mut self, data: ForegroundObjectEntityData) -> ::sillyecs::EntityId {
        self.spawn_foreground_object(data)
    }
}

/// Spawns an entity into the world.
impl<E, Q> Spawn<ForegroundObjectEntityComponents> for MainWorld<E, Q> {
    /// Spawn a new entity into the world.
    #[inline]
    fn spawn(&mut self, data: ForegroundObjectEntityComponents) -> ::sillyecs::EntityId {
        self.spawn_foreground_object(data)
    }
}

/// Spawns an entity into the world.
impl<E, Q> Spawn<BackgroundObjectEntityData> for MainWorld<E, Q> {
    /// Spawn a new entity into the world.
    #[inline]
    fn spawn(&mut self, data: BackgroundObjectEntityData) -> ::sillyecs::EntityId {
        self.spawn_background_object(data)
    }
}

/// Spawns an entity into the world.
impl<E, Q> Spawn<BackgroundObjectEntityComponents> for MainWorld<E, Q> {
    /// Spawn a new entity into the world.
    #[inline]
    fn spawn(&mut self, data: BackgroundObjectEntityComponents) -> ::sillyecs::EntityId {
        self.spawn_background_object(data)
    }
}

/// Spawns an entity into the world.
impl<E, Q> Spawn<SceneBackgroundEntityData> for MainWorld<E, Q> {
    /// Spawn a new entity into the world.
    #[inline]
    fn spawn(&mut self, data: SceneBackgroundEntityData) -> ::sillyecs::EntityId {
        self.spawn_scene_background(data)
    }
}

/// Spawns an entity into the world.
impl<E, Q> Spawn<SceneBackgroundEntityComponents> for MainWorld<E, Q> {
    /// Spawn a new entity into the world.
    #[inline]
    fn spawn(&mut self, data: SceneBackgroundEntityComponents) -> ::sillyecs::EntityId {
        self.spawn_scene_background(data)
    }
}

/// Helper trait to prevent accidental abuse of the Archetype's spawning function.
pub trait WorldEntityRegistry {
    /// Registers an entity with the world.
    fn register(&mut self, id: ::sillyecs::EntityId, archetype: EntityArchetypeRef) -> ::sillyecs::EntityId;
}

#[allow(dead_code)]
impl<Q> MainWorld<NoOpPhaseEvents, Q> {
    /// Creates a new [`MainWorld`].
    pub fn new<S>(
        create_systems: &S,
        states: MainWorldStates,
        command_queue: Q) -> Self
    where
        S: CreateSystems,
        Q: WorldCommandQueue
    {
        Self::new_with_events(
            create_systems,
            states,
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
        states: MainWorldStates,
        command_queue: Q,
        phase_events: E) -> Self
    where
        S: CreateSystem<WgpuInitUniformBindGroupSystem>
         + CreateSystem<WgpuInitShaderSystem>
         + CreateSystem<WgpuInitTextureSystem>
         + CreateSystem<TimeOfDaySystem>
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
        let context = ::sillyecs::FrameContext::new(<Self as ::sillyecs::World>::ID);
        Self {
            archetypes: Default::default(),
            systems: MainWorldSystems {
                wgpu_init_uniform_bind_group: CreateSystem::<WgpuInitUniformBindGroupSystem>::create(create_systems),
                wgpu_init_shader: CreateSystem::<WgpuInitShaderSystem>::create(create_systems),
                wgpu_init_texture: CreateSystem::<WgpuInitTextureSystem>::create(create_systems),
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
            context,
            states,
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
        self.archetypes.entity_locations.len()
    }

    /// Indicates whether there are no entities in this world.
    pub fn is_empty(&self) -> bool {
        self.archetypes.entity_locations.is_empty()
    }

    /// De-spawns an entity given by its [`::sillyecs::EntityId`]. Returns an error if the entity was unknown in this world.
    pub fn despawn_by_id(&mut self, id: ::sillyecs::EntityId) -> Result<(), DespawnError> {
        self.handle_despawn_command(id)
    }

    /// Spawn a new `WgpuUniformBindGroup` entity into the world given its [`WgpuUniformBindGroupEntityData`].
    #[inline]
    pub fn spawn_wgpu_uniform_bind_group<Entity>(
        &mut self,
        wgpu_uniform_bind_group: Entity
    ) -> ::sillyecs::EntityId
    where
        Entity: Into<WgpuUniformBindGroupEntityComponents>
    {
        let wgpu_uniform_bind_group = wgpu_uniform_bind_group.into();
        self.spawn_wgpu_uniform_bind_group_with(
            wgpu_uniform_bind_group.uniform_type,
            wgpu_uniform_bind_group.wgpu_bind_group_layout,
            wgpu_uniform_bind_group.wgpu_bind_group,
            wgpu_uniform_bind_group.wgpu_buffer,
        )
    }

    /// Spawn a new `WgpuUniformBindGroup` entity into the world.
    pub fn spawn_wgpu_uniform_bind_group_with(
        &mut self,
        uniform_type: UniformTypeComponent,
        wgpu_bind_group_layout: WgpuBindGroupLayoutComponent,
        wgpu_bind_group: WgpuBindGroupComponent,
        wgpu_buffer: WgpuBufferComponent,
    ) -> ::sillyecs::EntityId {
        struct Registry<'a>(&'a mut EntityLocationMap<::sillyecs::EntityId, EntityArchetypeRef>);

        impl WorldEntityRegistry for Registry<'_> {
            #[inline(always)]
            fn register(&mut self, id: ::sillyecs::EntityId, archetype: EntityArchetypeRef) -> ::sillyecs::EntityId {
                self.0.insert(id, archetype);
                id
            }
        }

        let registry = Registry(&mut self.archetypes.entity_locations);
        self.archetypes
            .collection
            .wgpu_uniform_bind_group
            .spawn_with(
                uniform_type,
                wgpu_bind_group_layout,
                wgpu_bind_group,
                wgpu_buffer,
                registry
            )
    }

    /// Spawn a new `WgpuShader` entity into the world given its [`WgpuShaderEntityData`].
    #[inline]
    pub fn spawn_wgpu_shader<Entity>(
        &mut self,
        wgpu_shader: Entity
    ) -> ::sillyecs::EntityId
    where
        Entity: Into<WgpuShaderEntityComponents>
    {
        let wgpu_shader = wgpu_shader.into();
        self.spawn_wgpu_shader_with(
            wgpu_shader.wgpu_shader,
        )
    }

    /// Spawn a new `WgpuShader` entity into the world.
    pub fn spawn_wgpu_shader_with(
        &mut self,
        wgpu_shader: WgpuShaderComponent,
    ) -> ::sillyecs::EntityId {
        struct Registry<'a>(&'a mut EntityLocationMap<::sillyecs::EntityId, EntityArchetypeRef>);

        impl WorldEntityRegistry for Registry<'_> {
            #[inline(always)]
            fn register(&mut self, id: ::sillyecs::EntityId, archetype: EntityArchetypeRef) -> ::sillyecs::EntityId {
                self.0.insert(id, archetype);
                id
            }
        }

        let registry = Registry(&mut self.archetypes.entity_locations);
        self.archetypes
            .collection
            .wgpu_shader
            .spawn_with(
                wgpu_shader,
                registry
            )
    }

    /// Spawn a new `WgpuTexture` entity into the world given its [`WgpuTextureEntityData`].
    #[inline]
    pub fn spawn_wgpu_texture<Entity>(
        &mut self,
        wgpu_texture: Entity
    ) -> ::sillyecs::EntityId
    where
        Entity: Into<WgpuTextureEntityComponents>
    {
        let wgpu_texture = wgpu_texture.into();
        self.spawn_wgpu_texture_with(
            wgpu_texture.texture_source,
            wgpu_texture.wgpu_texture,
        )
    }

    /// Spawn a new `WgpuTexture` entity into the world.
    pub fn spawn_wgpu_texture_with(
        &mut self,
        texture_source: TextureSourceComponent,
        wgpu_texture: WgpuTextureComponent,
    ) -> ::sillyecs::EntityId {
        struct Registry<'a>(&'a mut EntityLocationMap<::sillyecs::EntityId, EntityArchetypeRef>);

        impl WorldEntityRegistry for Registry<'_> {
            #[inline(always)]
            fn register(&mut self, id: ::sillyecs::EntityId, archetype: EntityArchetypeRef) -> ::sillyecs::EntityId {
                self.0.insert(id, archetype);
                id
            }
        }

        let registry = Registry(&mut self.archetypes.entity_locations);
        self.archetypes
            .collection
            .wgpu_texture
            .spawn_with(
                texture_source,
                wgpu_texture,
                registry
            )
    }

    /// Spawn a new `Environment` entity into the world given its [`EnvironmentEntityData`].
    #[inline]
    pub fn spawn_environment<Entity>(
        &mut self,
        environment: Entity
    ) -> ::sillyecs::EntityId
    where
        Entity: Into<EnvironmentEntityComponents>
    {
        let environment = environment.into();
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
    ) -> ::sillyecs::EntityId {
        struct Registry<'a>(&'a mut EntityLocationMap<::sillyecs::EntityId, EntityArchetypeRef>);

        impl WorldEntityRegistry for Registry<'_> {
            #[inline(always)]
            fn register(&mut self, id: ::sillyecs::EntityId, archetype: EntityArchetypeRef) -> ::sillyecs::EntityId {
                self.0.insert(id, archetype);
                id
            }
        }

        let registry = Registry(&mut self.archetypes.entity_locations);
        self.archetypes
            .collection
            .environment
            .spawn_with(
                time_of_day,
                sun,
                skydome,
                ambient_light,
                stars,
                fog,
                registry
            )
    }

    /// Spawn a new `Camera` entity into the world given its [`CameraEntityData`].
    #[inline]
    pub fn spawn_camera<Entity>(
        &mut self,
        camera: Entity
    ) -> ::sillyecs::EntityId
    where
        Entity: Into<CameraEntityComponents>
    {
        let camera = camera.into();
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
    ) -> ::sillyecs::EntityId {
        struct Registry<'a>(&'a mut EntityLocationMap<::sillyecs::EntityId, EntityArchetypeRef>);

        impl WorldEntityRegistry for Registry<'_> {
            #[inline(always)]
            fn register(&mut self, id: ::sillyecs::EntityId, archetype: EntityArchetypeRef) -> ::sillyecs::EntityId {
                self.0.insert(id, archetype);
                id
            }
        }

        let registry = Registry(&mut self.archetypes.entity_locations);
        self.archetypes
            .collection
            .camera
            .spawn_with(
                camera,
                camera_fps_controller,
                registry
            )
    }

    /// Spawn a new `DynamicLight` entity into the world given its [`DynamicLightEntityData`].
    #[inline]
    pub fn spawn_dynamic_light<Entity>(
        &mut self,
        dynamic_light: Entity
    ) -> ::sillyecs::EntityId
    where
        Entity: Into<DynamicLightEntityComponents>
    {
        let dynamic_light = dynamic_light.into();
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
    ) -> ::sillyecs::EntityId {
        struct Registry<'a>(&'a mut EntityLocationMap<::sillyecs::EntityId, EntityArchetypeRef>);

        impl WorldEntityRegistry for Registry<'_> {
            #[inline(always)]
            fn register(&mut self, id: ::sillyecs::EntityId, archetype: EntityArchetypeRef) -> ::sillyecs::EntityId {
                self.0.insert(id, archetype);
                id
            }
        }

        let registry = Registry(&mut self.archetypes.entity_locations);
        self.archetypes
            .collection
            .dynamic_light
            .spawn_with(
                dynamic_light,
                color,
                position,
                registry
            )
    }

    /// Spawn a new `Tile` entity into the world given its [`TileEntityData`].
    #[inline]
    pub fn spawn_tile<Entity>(
        &mut self,
        tile: Entity
    ) -> ::sillyecs::EntityId
    where
        Entity: Into<TileEntityComponents>
    {
        let tile = tile.into();
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
    ) -> ::sillyecs::EntityId {
        struct Registry<'a>(&'a mut EntityLocationMap<::sillyecs::EntityId, EntityArchetypeRef>);

        impl WorldEntityRegistry for Registry<'_> {
            #[inline(always)]
            fn register(&mut self, id: ::sillyecs::EntityId, archetype: EntityArchetypeRef) -> ::sillyecs::EntityId {
                self.0.insert(id, archetype);
                id
            }
        }

        let registry = Registry(&mut self.archetypes.entity_locations);
        self.archetypes
            .collection
            .tile
            .spawn_with(
                tile,
                position,
                registry
            )
    }

    /// Spawn a new `Cube` entity into the world given its [`CubeEntityData`].
    #[inline]
    pub fn spawn_cube<Entity>(
        &mut self,
        cube: Entity
    ) -> ::sillyecs::EntityId
    where
        Entity: Into<CubeEntityComponents>
    {
        let cube = cube.into();
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
    ) -> ::sillyecs::EntityId {
        struct Registry<'a>(&'a mut EntityLocationMap<::sillyecs::EntityId, EntityArchetypeRef>);

        impl WorldEntityRegistry for Registry<'_> {
            #[inline(always)]
            fn register(&mut self, id: ::sillyecs::EntityId, archetype: EntityArchetypeRef) -> ::sillyecs::EntityId {
                self.0.insert(id, archetype);
                id
            }
        }

        let registry = Registry(&mut self.archetypes.entity_locations);
        self.archetypes
            .collection
            .cube
            .spawn_with(
                position,
                velocity,
                transform,
                model,
                registry
            )
    }

    /// Spawn a new `Player` entity into the world given its [`PlayerEntityData`].
    #[inline]
    pub fn spawn_player<Entity>(
        &mut self,
        player: Entity
    ) -> ::sillyecs::EntityId
    where
        Entity: Into<PlayerEntityComponents>
    {
        let player = player.into();
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
    ) -> ::sillyecs::EntityId {
        struct Registry<'a>(&'a mut EntityLocationMap<::sillyecs::EntityId, EntityArchetypeRef>);

        impl WorldEntityRegistry for Registry<'_> {
            #[inline(always)]
            fn register(&mut self, id: ::sillyecs::EntityId, archetype: EntityArchetypeRef) -> ::sillyecs::EntityId {
                self.0.insert(id, archetype);
                id
            }
        }

        let registry = Registry(&mut self.archetypes.entity_locations);
        self.archetypes
            .collection
            .player
            .spawn_with(
                position,
                velocity,
                health,
                registry
            )
    }

    /// Spawn a new `ForegroundObject` entity into the world given its [`ForegroundObjectEntityData`].
    #[inline]
    pub fn spawn_foreground_object<Entity>(
        &mut self,
        foreground_object: Entity
    ) -> ::sillyecs::EntityId
    where
        Entity: Into<ForegroundObjectEntityComponents>
    {
        let foreground_object = foreground_object.into();
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
    ) -> ::sillyecs::EntityId {
        struct Registry<'a>(&'a mut EntityLocationMap<::sillyecs::EntityId, EntityArchetypeRef>);

        impl WorldEntityRegistry for Registry<'_> {
            #[inline(always)]
            fn register(&mut self, id: ::sillyecs::EntityId, archetype: EntityArchetypeRef) -> ::sillyecs::EntityId {
                self.0.insert(id, archetype);
                id
            }
        }

        let registry = Registry(&mut self.archetypes.entity_locations);
        self.archetypes
            .collection
            .foreground_object
            .spawn_with(
                position,
                collider,
                registry
            )
    }

    /// Spawn a new `BackgroundObject` entity into the world given its [`BackgroundObjectEntityData`].
    #[inline]
    pub fn spawn_background_object<Entity>(
        &mut self,
        background_object: Entity
    ) -> ::sillyecs::EntityId
    where
        Entity: Into<BackgroundObjectEntityComponents>
    {
        let background_object = background_object.into();
        self.spawn_background_object_with(
            background_object.position,
        )
    }

    /// Spawn a new `BackgroundObject` entity into the world.
    pub fn spawn_background_object_with(
        &mut self,
        position: PositionComponent,
    ) -> ::sillyecs::EntityId {
        struct Registry<'a>(&'a mut EntityLocationMap<::sillyecs::EntityId, EntityArchetypeRef>);

        impl WorldEntityRegistry for Registry<'_> {
            #[inline(always)]
            fn register(&mut self, id: ::sillyecs::EntityId, archetype: EntityArchetypeRef) -> ::sillyecs::EntityId {
                self.0.insert(id, archetype);
                id
            }
        }

        let registry = Registry(&mut self.archetypes.entity_locations);
        self.archetypes
            .collection
            .background_object
            .spawn_with(
                position,
                registry
            )
    }

    /// Spawn a new `SceneBackground` entity into the world given its [`SceneBackgroundEntityData`].
    #[inline]
    pub fn spawn_scene_background<Entity>(
        &mut self,
        scene_background: Entity
    ) -> ::sillyecs::EntityId
    where
        Entity: Into<SceneBackgroundEntityComponents>
    {
        let scene_background = scene_background.into();
        self.spawn_scene_background_with(
            scene_background.background_color,
        )
    }

    /// Spawn a new `SceneBackground` entity into the world.
    pub fn spawn_scene_background_with(
        &mut self,
        background_color: BackgroundColorComponent,
    ) -> ::sillyecs::EntityId {
        struct Registry<'a>(&'a mut EntityLocationMap<::sillyecs::EntityId, EntityArchetypeRef>);

        impl WorldEntityRegistry for Registry<'_> {
            #[inline(always)]
            fn register(&mut self, id: ::sillyecs::EntityId, archetype: EntityArchetypeRef) -> ::sillyecs::EntityId {
                self.0.insert(id, archetype);
                id
            }
        }

        let registry = Registry(&mut self.archetypes.entity_locations);
        self.archetypes
            .collection
            .scene_background
            .spawn_with(
                background_color,
                registry
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
    fn on_begin_wgpu_reinit_phase(&mut self) {
        let now = std::time::Instant::now();
        let previous = self.delta_timers.previous_wgpu_reinit_start;
        self.delta_timers.current_wgpu_reinit_start = now;
        self.context.current_frame_start = now;
        self.context.last_frame_start = previous;
        if self.context.frame_number == 0 {
            self.context.delta_time_secs = 0.0;
        } else {
            self.context.delta_time_secs = (now - previous).as_secs_f32();
        }
    }

    /// Runs a per-frame update of the frame context at the end of a frame.
    fn on_end_wgpu_reinit_phase(&mut self) {
        self.delta_timers.previous_wgpu_reinit_start = self.delta_timers.current_wgpu_reinit_start;
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
    /// - ~~`Startup`~~ (skipped here, must be executed manually)
    /// - ~~`WgpuReinit`~~ (skipped here, must be executed manually)
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

        // NOTE: The Startup phase is marked manual and will be skipped here.
        //       self.par_apply_system_phase_startup();

        // NOTE: The WgpuReinit phase is marked manual and will be skipped here.
        //       self.par_apply_system_phase_wgpu_reinit();

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
    /// - ~~`Startup`~~ (skipped here, must be executed manually)
    /// - ~~`WgpuReinit`~~ (skipped here, must be executed manually)
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

        // NOTE: The Startup phase is marked manual and will be skipped here.
        //       self.par_apply_system_phase_startup();

        // NOTE: The WgpuReinit phase is marked manual and will be skipped here.
        //       self.par_apply_system_phase_wgpu_reinit();

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
    pub fn apply_system_phase_startup(&mut self)
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
    pub fn par_apply_system_phase_startup(&mut self)
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

    /// Execute system phase `WgpuReinit`
    ///
    /// This method executes all systems scheduled for the `WgpuReinit` phase in sequential order.
    /// Systems are organized into groups that must be executed in sequence, while systems within each
    /// group can potentially be parallelized (see `par_apply_system_phase_wgpu_reinit`).
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
    pub fn apply_system_phase_wgpu_reinit(&mut self)
    where
        E: SystemPhaseEvents,
        Q: WorldCommandSender + WorldCommandReceiver
    {
        self.on_begin_wgpu_reinit_phase();
        let result = SystemWgpuReinitPhaseEvents::on_begin_phase(
            &mut self.events,
            &self.context,
        )
        .inspect_err(|error| tracing::error!(%error, "Failed to handle phase begin events"))
        .ok();

        self.apply_system_phase_wgpu_reinit_without_events();

        SystemWgpuReinitPhaseEvents::on_end_phase(
            &mut self.events,
            result,
            &self.context,
        )
        .inspect_err(|error| tracing::error!(%error, "Failed to handle phase end events"))
        .ok();

        // Process all commands.
        self.handle_commands();
        self.on_end_wgpu_reinit_phase();
    }

    /// Execute system phase `WgpuReinit` without calling events. See [`apply_system_phase_wgpu_reinit`](Self::apply_system_phase_wgpu_reinit).
    #[inline]
    fn apply_system_phase_wgpu_reinit_without_events(&mut self)
    where
        E: SystemPhaseEvents,
        Q: WorldCommandSender + WorldCommandReceiver
    {
        // System group 1 of 3
        let is_ready = self.systems.wgpu_init_shader.is_ready(
                       &self.states.wgpu_render,
                       
           );
        if is_ready && self.systems.wgpu_init_shader.on_begin_phase(
                        
                        
            )
            .inspect_err(|error| tracing::error!(?error, "WgpuInitShaderSystem::on_begin_phase returned an error"))
            .is_ok()
        {
            // Preflight
            {
                // System has no preflight step
            }

            // Systems
            {
                let entities: [&[::sillyecs::EntityId]; 1] = [
                    &self.archetypes.collection.wgpu_shader.entities,
                ];
                let wgpu_shader_outputs: [&mut [WgpuShaderComponent]; 1] = [
                    &mut self.archetypes.collection.wgpu_shader.wgpu_shaders,
                ];

                // Apply WgpuInitShaderSystem to all archetypes
                self.systems.wgpu_init_shader.apply_all(
                            &self.states.wgpu_render,
                            
                    entities,
                    wgpu_shader_outputs,
                );
            }

            // Postflight
            {
                // Postflight of WgpuInitShaderSystem for WgpuShaderArchetype
                self.systems.wgpu_init_shader.postflight(
                            
                            &mut self.states.resource_manager,
                    &self.archetypes.collection.wgpu_shader.entities,
                    &self.archetypes.collection.wgpu_shader.wgpu_shaders,
                );
            }

            self.systems.wgpu_init_shader.on_end_phase(
                        
                        &mut self.states.resource_manager,
            )
            .inspect_err(|error| tracing::error!(?error, "WgpuInitShaderSystem::on_end_phase returned an error"))
            .ok();
        }
        // System group 2 of 3
        let is_ready = self.systems.wgpu_init_uniform_bind_group.is_ready(
                       &self.states.wgpu_render,
                       
           );
        if is_ready && self.systems.wgpu_init_uniform_bind_group.on_begin_phase(
                        
                        
            )
            .inspect_err(|error| tracing::error!(?error, "WgpuInitUniformBindGroupSystem::on_begin_phase returned an error"))
            .is_ok()
        {
            // Preflight
            {
                // System has no preflight step
            }

            // Systems
            {
                let entities: [&[::sillyecs::EntityId]; 1] = [
                    &self.archetypes.collection.wgpu_uniform_bind_group.entities,
                ];
                let uniform_type_inputs: [&[UniformTypeComponent]; 1] = [
                    &self.archetypes.collection.wgpu_uniform_bind_group.uniform_types,
                ];
                let wgpu_bind_group_layout_outputs: [&mut [WgpuBindGroupLayoutComponent]; 1] = [
                    &mut self.archetypes.collection.wgpu_uniform_bind_group.wgpu_bind_group_layouts,
                ];
                let wgpu_bind_group_outputs: [&mut [WgpuBindGroupComponent]; 1] = [
                    &mut self.archetypes.collection.wgpu_uniform_bind_group.wgpu_bind_groups,
                ];
                let wgpu_buffer_outputs: [&mut [WgpuBufferComponent]; 1] = [
                    &mut self.archetypes.collection.wgpu_uniform_bind_group.wgpu_buffers,
                ];

                // Apply WgpuInitUniformBindGroupSystem to all archetypes
                self.systems.wgpu_init_uniform_bind_group.apply_all(
                            &self.states.wgpu_render,
                            
                    entities,
                    uniform_type_inputs,
                    wgpu_bind_group_layout_outputs,
                    wgpu_bind_group_outputs,
                    wgpu_buffer_outputs,
                );
            }

            // Postflight
            {
                // Postflight of WgpuInitUniformBindGroupSystem for WgpuUniformBindGroupArchetype
                self.systems.wgpu_init_uniform_bind_group.postflight(
                            
                            &mut self.states.resource_manager,
                    &self.archetypes.collection.wgpu_uniform_bind_group.entities,
                    &self.archetypes.collection.wgpu_uniform_bind_group.uniform_types,
                    &self.archetypes.collection.wgpu_uniform_bind_group.wgpu_bind_group_layouts,
                    &self.archetypes.collection.wgpu_uniform_bind_group.wgpu_bind_groups,
                    &self.archetypes.collection.wgpu_uniform_bind_group.wgpu_buffers,
                );
            }

            self.systems.wgpu_init_uniform_bind_group.on_end_phase(
                        
                        &mut self.states.resource_manager,
            )
            .inspect_err(|error| tracing::error!(?error, "WgpuInitUniformBindGroupSystem::on_end_phase returned an error"))
            .ok();
        }
        // System group 3 of 3
        let is_ready = self.systems.wgpu_init_texture.is_ready(
                       &self.states.wgpu_render,
                       
           );
        if is_ready && self.systems.wgpu_init_texture.on_begin_phase(
                        
                        
            )
            .inspect_err(|error| tracing::error!(?error, "WgpuInitTextureSystem::on_begin_phase returned an error"))
            .is_ok()
        {
            // Preflight
            {
                // System has no preflight step
            }

            // Systems
            {
                let entities: [&[::sillyecs::EntityId]; 1] = [
                    &self.archetypes.collection.wgpu_texture.entities,
                ];
                let texture_source_inputs: [&[TextureSourceComponent]; 1] = [
                    &self.archetypes.collection.wgpu_texture.texture_sources,
                ];
                let wgpu_texture_outputs: [&mut [WgpuTextureComponent]; 1] = [
                    &mut self.archetypes.collection.wgpu_texture.wgpu_textures,
                ];

                // Apply WgpuInitTextureSystem to all archetypes
                self.systems.wgpu_init_texture.apply_all(
                            &self.states.wgpu_render,
                            
                    entities,
                    texture_source_inputs,
                    wgpu_texture_outputs,
                );
            }

            // Postflight
            {
                // Postflight of WgpuInitTextureSystem for WgpuTextureArchetype
                self.systems.wgpu_init_texture.postflight(
                            
                            &mut self.states.resource_manager,
                    &self.archetypes.collection.wgpu_texture.entities,
                    &self.archetypes.collection.wgpu_texture.texture_sources,
                    &self.archetypes.collection.wgpu_texture.wgpu_textures,
                );
            }

            self.systems.wgpu_init_texture.on_end_phase(
                        
                        &mut self.states.resource_manager,
            )
            .inspect_err(|error| tracing::error!(?error, "WgpuInitTextureSystem::on_end_phase returned an error"))
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
    pub fn par_apply_system_phase_wgpu_reinit(&mut self)
    where
        E: SystemPhaseEvents,
        Q: WorldCommandSender + WorldCommandReceiver
    {
        self.on_begin_wgpu_reinit_phase();
        let result = SystemWgpuReinitPhaseEvents::on_begin_phase(
            &mut self.events,
            &self.context,
        )
        .inspect_err(|error| tracing::error!(%error, "Failed to handle phase begin events"))
        .ok();

        // System group 1
        let is_wgpu_init_shader_ready = self.systems.wgpu_init_shader.is_ready(
                       &self.states.wgpu_render,
                       
           );
        let is_wgpu_init_shader_ready = is_wgpu_init_shader_ready &
            self.systems.wgpu_init_shader.on_begin_phase(
                        
                        
            )
            .inspect_err(|error| tracing::error!(%error, "WgpuInitShaderSystem::on_begin_phase returned an error"))
            .is_ok();

        {
            // TODO: Skip spawning thread here when there is only one system, or when all run in sequence
            rayon::scope(|s| {
                // TODO: Instead of parallelizing systems (and then have them access archetypes), parallelize archetypes and apply all systems. This should improve data parallelism because it keeps caches hot.
                if is_wgpu_init_shader_ready {
                    s.spawn(|_| {
                        // Preflight
                        {
                            // System has no preflight step
                        }

                        // Systems
                        {
                            let entities: [&[::sillyecs::EntityId]; 1] = [
                                &self.archetypes.collection.wgpu_shader.entities,
                            ];
                            let wgpu_shader_outputs: [&mut [WgpuShaderComponent]; 1] = [
                                &mut self.archetypes.collection.wgpu_shader.wgpu_shaders,
                            ];

                            // Apply WgpuInitShaderSystem to all archetypes
                            self.systems.wgpu_init_shader.apply_all(
                                        &self.states.wgpu_render,
                                        
                                entities,
                                wgpu_shader_outputs,
                            );
                        }

                        // Postflight
                        {
                            // Postflight of WgpuInitShaderSystem for WgpuShaderArchetype
                            self.systems.wgpu_init_shader.postflight(
                                        
                                        &mut self.states.resource_manager,
                                &self.archetypes.collection.wgpu_shader.entities,
                                &self.archetypes.collection.wgpu_shader.wgpu_shaders,
                            );
                        }
                    });
                }
            });
        }

        self.systems.wgpu_init_shader.on_end_phase(
                    
                    &mut self.states.resource_manager,
        )
        .inspect_err(|error| tracing::error!(%error, "WgpuInitShaderSystem::on_end_phase returned an error"))
        .ok();

        // System group 2
        let is_wgpu_init_uniform_bind_group_ready = self.systems.wgpu_init_uniform_bind_group.is_ready(
                       &self.states.wgpu_render,
                       
           );
        let is_wgpu_init_uniform_bind_group_ready = is_wgpu_init_uniform_bind_group_ready &
            self.systems.wgpu_init_uniform_bind_group.on_begin_phase(
                        
                        
            )
            .inspect_err(|error| tracing::error!(%error, "WgpuInitUniformBindGroupSystem::on_begin_phase returned an error"))
            .is_ok();

        {
            // TODO: Skip spawning thread here when there is only one system, or when all run in sequence
            rayon::scope(|s| {
                // TODO: Instead of parallelizing systems (and then have them access archetypes), parallelize archetypes and apply all systems. This should improve data parallelism because it keeps caches hot.
                if is_wgpu_init_uniform_bind_group_ready {
                    s.spawn(|_| {
                        // Preflight
                        {
                            // System has no preflight step
                        }

                        // Systems
                        {
                            let entities: [&[::sillyecs::EntityId]; 1] = [
                                &self.archetypes.collection.wgpu_uniform_bind_group.entities,
                            ];
                            let uniform_type_inputs: [&[UniformTypeComponent]; 1] = [
                                &self.archetypes.collection.wgpu_uniform_bind_group.uniform_types,
                            ];
                            let wgpu_bind_group_layout_outputs: [&mut [WgpuBindGroupLayoutComponent]; 1] = [
                                &mut self.archetypes.collection.wgpu_uniform_bind_group.wgpu_bind_group_layouts,
                            ];
                            let wgpu_bind_group_outputs: [&mut [WgpuBindGroupComponent]; 1] = [
                                &mut self.archetypes.collection.wgpu_uniform_bind_group.wgpu_bind_groups,
                            ];
                            let wgpu_buffer_outputs: [&mut [WgpuBufferComponent]; 1] = [
                                &mut self.archetypes.collection.wgpu_uniform_bind_group.wgpu_buffers,
                            ];

                            // Apply WgpuInitUniformBindGroupSystem to all archetypes
                            self.systems.wgpu_init_uniform_bind_group.apply_all(
                                        &self.states.wgpu_render,
                                        
                                entities,
                                uniform_type_inputs,
                                wgpu_bind_group_layout_outputs,
                                wgpu_bind_group_outputs,
                                wgpu_buffer_outputs,
                            );
                        }

                        // Postflight
                        {
                            // Postflight of WgpuInitUniformBindGroupSystem for WgpuUniformBindGroupArchetype
                            self.systems.wgpu_init_uniform_bind_group.postflight(
                                        
                                        &mut self.states.resource_manager,
                                &self.archetypes.collection.wgpu_uniform_bind_group.entities,
                                &self.archetypes.collection.wgpu_uniform_bind_group.uniform_types,
                                &self.archetypes.collection.wgpu_uniform_bind_group.wgpu_bind_group_layouts,
                                &self.archetypes.collection.wgpu_uniform_bind_group.wgpu_bind_groups,
                                &self.archetypes.collection.wgpu_uniform_bind_group.wgpu_buffers,
                            );
                        }
                    });
                }
            });
        }

        self.systems.wgpu_init_uniform_bind_group.on_end_phase(
                    
                    &mut self.states.resource_manager,
        )
        .inspect_err(|error| tracing::error!(%error, "WgpuInitUniformBindGroupSystem::on_end_phase returned an error"))
        .ok();

        // System group 3
        let is_wgpu_init_texture_ready = self.systems.wgpu_init_texture.is_ready(
                       &self.states.wgpu_render,
                       
           );
        let is_wgpu_init_texture_ready = is_wgpu_init_texture_ready &
            self.systems.wgpu_init_texture.on_begin_phase(
                        
                        
            )
            .inspect_err(|error| tracing::error!(%error, "WgpuInitTextureSystem::on_begin_phase returned an error"))
            .is_ok();

        {
            // TODO: Skip spawning thread here when there is only one system, or when all run in sequence
            rayon::scope(|s| {
                // TODO: Instead of parallelizing systems (and then have them access archetypes), parallelize archetypes and apply all systems. This should improve data parallelism because it keeps caches hot.
                if is_wgpu_init_texture_ready {
                    s.spawn(|_| {
                        // Preflight
                        {
                            // System has no preflight step
                        }

                        // Systems
                        {
                            let entities: [&[::sillyecs::EntityId]; 1] = [
                                &self.archetypes.collection.wgpu_texture.entities,
                            ];
                            let texture_source_inputs: [&[TextureSourceComponent]; 1] = [
                                &self.archetypes.collection.wgpu_texture.texture_sources,
                            ];
                            let wgpu_texture_outputs: [&mut [WgpuTextureComponent]; 1] = [
                                &mut self.archetypes.collection.wgpu_texture.wgpu_textures,
                            ];

                            // Apply WgpuInitTextureSystem to all archetypes
                            self.systems.wgpu_init_texture.apply_all(
                                        &self.states.wgpu_render,
                                        
                                entities,
                                texture_source_inputs,
                                wgpu_texture_outputs,
                            );
                        }

                        // Postflight
                        {
                            // Postflight of WgpuInitTextureSystem for WgpuTextureArchetype
                            self.systems.wgpu_init_texture.postflight(
                                        
                                        &mut self.states.resource_manager,
                                &self.archetypes.collection.wgpu_texture.entities,
                                &self.archetypes.collection.wgpu_texture.texture_sources,
                                &self.archetypes.collection.wgpu_texture.wgpu_textures,
                            );
                        }
                    });
                }
            });
        }

        self.systems.wgpu_init_texture.on_end_phase(
                    
                    &mut self.states.resource_manager,
        )
        .inspect_err(|error| tracing::error!(%error, "WgpuInitTextureSystem::on_end_phase returned an error"))
        .ok();

        SystemWgpuReinitPhaseEvents::on_end_phase(
            &mut self.events,
            result,
            &self.context,
        )
        .inspect_err(|error| tracing::error!(%error, "Failed to handle phase end events"))
        .ok();

        // Process all commands.
        self.handle_commands();
        self.on_end_wgpu_reinit_phase();
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
            // Preflight
            {
                // System has no preflight step
            }

            // Systems
            {
                let time_of_day_outputs: [&mut [TimeOfDayComponent]; 1] = [
                    &mut self.archetypes.collection.environment.time_of_days,
                ];

                // Apply TimeOfDaySystem to all archetypes
                self.systems.time_of_day.apply_all(
                    &self.context,
                    time_of_day_outputs,
                );
            }

            // Postflight
            {
                // System has no preflight step
            }

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
            // Preflight
            {
                // System has no preflight step
            }

            // Systems
            {
                let velocity_inputs: [&[VelocityComponent]; 2] = [
                    &self.archetypes.collection.cube.velocities,
                    &self.archetypes.collection.player.velocities,
                ];
                let position_outputs: [&mut [PositionComponent]; 2] = [
                    &mut self.archetypes.collection.cube.positions,
                    &mut self.archetypes.collection.player.positions,
                ];

                // Apply PhysicsSystem to all archetypes
                self.systems.physics.apply_all(
                    &self.context,
                    velocity_inputs,
                    position_outputs,
                );
            }

            // Postflight
            {
                // System has no preflight step
            }

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
            // Preflight
            {
                // System has no preflight step
            }

            // Systems
            {
                let time_of_day_inputs: [&[TimeOfDayComponent]; 1] = [
                    &self.archetypes.collection.environment.time_of_days,
                ];
                let sun_outputs: [&mut [SunComponent]; 1] = [
                    &mut self.archetypes.collection.environment.suns,
                ];
                let skydome_outputs: [&mut [SkydomeComponent]; 1] = [
                    &mut self.archetypes.collection.environment.skydomes,
                ];
                let ambient_light_outputs: [&mut [AmbientLightComponent]; 1] = [
                    &mut self.archetypes.collection.environment.ambient_lights,
                ];

                // Apply SunSystem to all archetypes
                self.systems.sun.apply_all(
                    time_of_day_inputs,
                    sun_outputs,
                    skydome_outputs,
                    ambient_light_outputs,
                );
            }

            // Postflight
            {
                // System has no preflight step
            }

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
            // Preflight
            {
                // System has no preflight step
            }

            // Systems
            {
                let time_of_day_inputs: [&[TimeOfDayComponent]; 1] = [
                    &self.archetypes.collection.environment.time_of_days,
                ];
                let fog_outputs: [&mut [FogComponent]; 1] = [
                    &mut self.archetypes.collection.environment.fogs,
                ];

                // Apply FogSystem to all archetypes
                self.systems.fog.apply_all(
                    time_of_day_inputs,
                    fog_outputs,
                );
            }

            // Postflight
            {
                // System has no preflight step
            }

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
            // Preflight
            {
                // System has no preflight step
            }

            // Systems
            {
                let time_of_day_inputs: [&[TimeOfDayComponent]; 1] = [
                    &self.archetypes.collection.environment.time_of_days,
                ];
                let stars_outputs: [&mut [StarsComponent]; 1] = [
                    &mut self.archetypes.collection.environment.stars,
                ];

                // Apply StarsSystem to all archetypes
                self.systems.stars.apply_all(
                    time_of_day_inputs,
                    stars_outputs,
                );
            }

            // Postflight
            {
                // System has no preflight step
            }

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
            // TODO: Skip spawning thread here when there is only one system, or when all run in sequence
            rayon::scope(|s| {
                // TODO: Instead of parallelizing systems (and then have them access archetypes), parallelize archetypes and apply all systems. This should improve data parallelism because it keeps caches hot.
                if is_time_of_day_ready {
                    s.spawn(|_| {
                        // Preflight
                        {
                            // System has no preflight step
                        }

                        // Systems
                        {
                            let time_of_day_outputs: [&mut [TimeOfDayComponent]; 1] = [
                                &mut self.archetypes.collection.environment.time_of_days,
                            ];

                            // Apply TimeOfDaySystem to all archetypes
                            self.systems.time_of_day.apply_all(
                                &self.context,
                                time_of_day_outputs,
                            );
                        }

                        // Postflight
                        {
                            // System has no preflight step
                        }
                    });
                }
                if is_physics_ready {
                    s.spawn(|_| {
                        // Preflight
                        {
                            // System has no preflight step
                        }

                        // Systems
                        {
                            let velocity_inputs: [&[VelocityComponent]; 2] = [
                                &self.archetypes.collection.cube.velocities,
                                &self.archetypes.collection.player.velocities,
                            ];
                            let position_outputs: [&mut [PositionComponent]; 2] = [
                                &mut self.archetypes.collection.cube.positions,
                                &mut self.archetypes.collection.player.positions,
                            ];

                            // Apply PhysicsSystem to all archetypes
                            self.systems.physics.apply_all(
                                &self.context,
                                velocity_inputs,
                                position_outputs,
                            );
                        }

                        // Postflight
                        {
                            // System has no preflight step
                        }
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
            // TODO: Skip spawning thread here when there is only one system, or when all run in sequence
            rayon::scope(|s| {
                // TODO: Instead of parallelizing systems (and then have them access archetypes), parallelize archetypes and apply all systems. This should improve data parallelism because it keeps caches hot.
                if is_sun_ready {
                    s.spawn(|_| {
                        // Preflight
                        {
                            // System has no preflight step
                        }

                        // Systems
                        {
                            let time_of_day_inputs: [&[TimeOfDayComponent]; 1] = [
                                &self.archetypes.collection.environment.time_of_days,
                            ];
                            let sun_outputs: [&mut [SunComponent]; 1] = [
                                &mut self.archetypes.collection.environment.suns,
                            ];
                            let skydome_outputs: [&mut [SkydomeComponent]; 1] = [
                                &mut self.archetypes.collection.environment.skydomes,
                            ];
                            let ambient_light_outputs: [&mut [AmbientLightComponent]; 1] = [
                                &mut self.archetypes.collection.environment.ambient_lights,
                            ];

                            // Apply SunSystem to all archetypes
                            self.systems.sun.apply_all(
                                time_of_day_inputs,
                                sun_outputs,
                                skydome_outputs,
                                ambient_light_outputs,
                            );
                        }

                        // Postflight
                        {
                            // System has no preflight step
                        }
                    });
                }
                if is_fog_ready {
                    s.spawn(|_| {
                        // Preflight
                        {
                            // System has no preflight step
                        }

                        // Systems
                        {
                            let time_of_day_inputs: [&[TimeOfDayComponent]; 1] = [
                                &self.archetypes.collection.environment.time_of_days,
                            ];
                            let fog_outputs: [&mut [FogComponent]; 1] = [
                                &mut self.archetypes.collection.environment.fogs,
                            ];

                            // Apply FogSystem to all archetypes
                            self.systems.fog.apply_all(
                                time_of_day_inputs,
                                fog_outputs,
                            );
                        }

                        // Postflight
                        {
                            // System has no preflight step
                        }
                    });
                }
                if is_stars_ready {
                    s.spawn(|_| {
                        // Preflight
                        {
                            // System has no preflight step
                        }

                        // Systems
                        {
                            let time_of_day_inputs: [&[TimeOfDayComponent]; 1] = [
                                &self.archetypes.collection.environment.time_of_days,
                            ];
                            let stars_outputs: [&mut [StarsComponent]; 1] = [
                                &mut self.archetypes.collection.environment.stars,
                            ];

                            // Apply StarsSystem to all archetypes
                            self.systems.stars.apply_all(
                                time_of_day_inputs,
                                stars_outputs,
                            );
                        }

                        // Postflight
                        {
                            // System has no preflight step
                        }
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
            // Preflight
            {
                // System has no preflight step
            }

            // Systems
            {
                let camera_outputs: [&mut [CameraComponent]; 1] = [
                    &mut self.archetypes.collection.camera.cameras,
                ];
                let camera_fps_controller_outputs: [&mut [CameraFpsControllerComponent]; 1] = [
                    &mut self.archetypes.collection.camera.camera_fps_controllers,
                ];

                // Apply CameraSystem to all archetypes
                self.systems.camera.apply_all(
                            &self.states.input,
                            &self.states.wgpu_render,
                    camera_outputs,
                    camera_fps_controller_outputs,
                );
            }

            // Postflight
            {
                // System has no preflight step
            }

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
            // Preflight
            {
                // System has no preflight step
            }

            // Systems
            {
                let background_color_outputs: [&mut [BackgroundColorComponent]; 1] = [
                    &mut self.archetypes.collection.scene_background.background_colors,
                ];

                // Apply BackgroundColorCycleSystem to all archetypes
                self.systems.background_color_cycle.apply_all(
                    &self.context,
                    background_color_outputs,
                );
            }

            // Postflight
            {
                // System has no preflight step
            }

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
        let is_ready = self.systems.render_background.is_ready(
                       &self.states.wgpu_render,
           );
        if is_ready && self.systems.render_background.on_begin_phase(
                        &self.states.wgpu_render,
            )
            .inspect_err(|error| tracing::error!(?error, "RenderBackgroundSystem::on_begin_phase returned an error"))
            .is_ok()
        {
            // Preflight
            {
                // System has no preflight step
            }

            // Systems
            {
                let background_color_inputs: [&[BackgroundColorComponent]; 1] = [
                    &self.archetypes.collection.scene_background.background_colors,
                ];

                // Apply RenderBackgroundSystem to all archetypes
                self.systems.render_background.apply_all(
                            &mut self.states.wgpu_render,
                    background_color_inputs,
                );
            }

            // Postflight
            {
                // System has no preflight step
            }

            self.systems.render_background.on_end_phase(
                        &self.states.wgpu_render,
            )
            .inspect_err(|error| tracing::error!(?error, "RenderBackgroundSystem::on_end_phase returned an error"))
            .ok();
        }
        // System group 2 of 8
        let is_ready = self.systems.render_sky.is_ready(
                       &self.states.wgpu_render,
           );
        if is_ready && self.systems.render_sky.on_begin_phase(
                        &self.states.wgpu_render,
            )
            .inspect_err(|error| tracing::error!(?error, "RenderSkySystem::on_begin_phase returned an error"))
            .is_ok()
        {
            // Preflight
            {
                // System has no preflight step
            }

            // Systems
            {
                let skydome_inputs: [&[SkydomeComponent]; 1] = [
                    &self.archetypes.collection.environment.skydomes,
                ];

                // Apply RenderSkySystem to all archetypes
                self.systems.render_sky.apply_all(
                            &mut self.states.wgpu_render,
                    skydome_inputs,
                );
            }

            // Postflight
            {
                // System has no preflight step
            }

            self.systems.render_sky.on_end_phase(
                        &self.states.wgpu_render,
            )
            .inspect_err(|error| tracing::error!(?error, "RenderSkySystem::on_end_phase returned an error"))
            .ok();
        }
        // System group 3 of 8
        let is_ready = self.systems.dynamic_light.is_ready(
               &self.context,
                       &self.states.wgpu_render,
           );
        if is_ready && self.systems.dynamic_light.on_begin_phase(
                &self.context,
                        &self.states.wgpu_render,
            )
            .inspect_err(|error| tracing::error!(?error, "DynamicLightSystem::on_begin_phase returned an error"))
            .is_ok()
        {
            // Preflight
            {
                // System has no preflight step
            }

            // Systems
            {
                let color_inputs: [&[ColorComponent]; 1] = [
                    &self.archetypes.collection.dynamic_light.colors,
                ];
                let dynamic_light_outputs: [&mut [DynamicLightComponent]; 1] = [
                    &mut self.archetypes.collection.dynamic_light.dynamic_lights,
                ];

                // Apply DynamicLightSystem to all archetypes
                self.systems.dynamic_light.apply_all(
                    &self.context,
                            &mut self.states.wgpu_render,
                    color_inputs,
                    dynamic_light_outputs,
                );
            }

            // Postflight
            {
                // System has no preflight step
            }

            self.systems.dynamic_light.on_end_phase(
                &self.context,
                        &self.states.wgpu_render,
            )
            .inspect_err(|error| tracing::error!(?error, "DynamicLightSystem::on_end_phase returned an error"))
            .ok();
        }
        // System group 4 of 8
        let is_ready = self.systems.tile_render.is_ready(
                       &self.states.wgpu_render,
           );
        if is_ready && self.systems.tile_render.on_begin_phase(
                        &self.states.wgpu_render,
            )
            .inspect_err(|error| tracing::error!(?error, "TileRenderSystem::on_begin_phase returned an error"))
            .is_ok()
        {
            // Preflight
            {
                // System has no preflight step
            }

            // Systems
            {
                let tile_inputs: [&[TileComponent]; 1] = [
                    &self.archetypes.collection.tile.tiles,
                ];
                let position_inputs: [&[PositionComponent]; 1] = [
                    &self.archetypes.collection.tile.positions,
                ];

                // Apply TileRenderSystem to all archetypes
                self.systems.tile_render.apply_all(
                            &mut self.states.wgpu_render,
                    tile_inputs,
                    position_inputs,
                );
            }

            // Postflight
            {
                // System has no preflight step
            }

            self.systems.tile_render.on_end_phase(
                        &self.states.wgpu_render,
            )
            .inspect_err(|error| tracing::error!(?error, "TileRenderSystem::on_end_phase returned an error"))
            .ok();
        }
        // System group 5 of 8
        let is_ready = self.systems.render.is_ready(
                       &self.states.wgpu_render,
           );
        if is_ready && self.systems.render.on_begin_phase(
                        &self.states.wgpu_render,
            )
            .inspect_err(|error| tracing::error!(?error, "RenderSystem::on_begin_phase returned an error"))
            .is_ok()
        {
            // Preflight
            {
                // System has no preflight step
            }

            // Systems
            {
                let position_inputs: [&[PositionComponent]; 1] = [
                    &self.archetypes.collection.cube.positions,
                ];
                let transform_inputs: [&[TransformComponent]; 1] = [
                    &self.archetypes.collection.cube.transforms,
                ];
                let model_inputs: [&[ModelComponent]; 1] = [
                    &self.archetypes.collection.cube.models,
                ];

                // Apply RenderSystem to all archetypes
                self.systems.render.apply_all(
                            &mut self.states.wgpu_render,
                    position_inputs,
                    transform_inputs,
                    model_inputs,
                );
            }

            // Postflight
            {
                // System has no preflight step
            }

            self.systems.render.on_end_phase(
                        &self.states.wgpu_render,
            )
            .inspect_err(|error| tracing::error!(?error, "RenderSystem::on_end_phase returned an error"))
            .ok();
        }
        // System group 6 of 8
        let is_ready = self.systems.vga_blit.is_ready(
                       &self.states.wgpu_render,
           );
        if is_ready && self.systems.vga_blit.on_begin_phase(
                        &self.states.wgpu_render,
            )
            .inspect_err(|error| tracing::error!(?error, "VgaBlitSystem::on_begin_phase returned an error"))
            .is_ok()
        {
            // Preflight
            {
                // System has no preflight step
            }

            // Systems
            {
                let background_color_inputs: [&[BackgroundColorComponent]; 1] = [
                    &self.archetypes.collection.scene_background.background_colors,
                ];

                // Apply VgaBlitSystem to all archetypes
                self.systems.vga_blit.apply_all(
                            &mut self.states.wgpu_render,
                    background_color_inputs,
                );
            }

            // Postflight
            {
                // System has no preflight step
            }

            self.systems.vga_blit.on_end_phase(
                        &self.states.wgpu_render,
            )
            .inspect_err(|error| tracing::error!(?error, "VgaBlitSystem::on_end_phase returned an error"))
            .ok();
        }
        // System group 7 of 8
        let is_ready = self.systems.frame_globals.is_ready(
                       &self.states.wgpu_render,
           );
        if is_ready && self.systems.frame_globals.on_begin_phase(
                        &self.states.wgpu_render,
            )
            .inspect_err(|error| tracing::error!(?error, "FrameGlobalsSystem::on_begin_phase returned an error"))
            .is_ok()
        {
            // Preflight
            {
                // System has no preflight step
            }

            // Systems
            {
                let time_of_day_inputs: [&[TimeOfDayComponent]; 1] = [
                    &self.archetypes.collection.environment.time_of_days,
                ];
                let sun_inputs: [&[SunComponent]; 1] = [
                    &self.archetypes.collection.environment.suns,
                ];
                let fog_inputs: [&[FogComponent]; 1] = [
                    &self.archetypes.collection.environment.fogs,
                ];
                let ambient_light_inputs: [&[AmbientLightComponent]; 1] = [
                    &self.archetypes.collection.environment.ambient_lights,
                ];
                let stars_inputs: [&[StarsComponent]; 1] = [
                    &self.archetypes.collection.environment.stars,
                ];
                let skydome_inputs: [&[SkydomeComponent]; 1] = [
                    &self.archetypes.collection.environment.skydomes,
                ];

                // Apply FrameGlobalsSystem to all archetypes
                self.systems.frame_globals.apply_all(
                            &mut self.states.wgpu_render,
                    time_of_day_inputs,
                    sun_inputs,
                    fog_inputs,
                    ambient_light_inputs,
                    stars_inputs,
                    skydome_inputs,
                );
            }

            // Postflight
            {
                // System has no preflight step
            }

            self.systems.frame_globals.on_end_phase(
                        &self.states.wgpu_render,
            )
            .inspect_err(|error| tracing::error!(?error, "FrameGlobalsSystem::on_end_phase returned an error"))
            .ok();
        }
        // System group 8 of 8
        let is_ready = self.systems.camera_render.is_ready(
                       &self.states.wgpu_render,
                       &self.states.active_camera,
           );
        if is_ready && self.systems.camera_render.on_begin_phase(
                        &self.states.wgpu_render,
                        &self.states.active_camera,
            )
            .inspect_err(|error| tracing::error!(?error, "CameraRenderSystem::on_begin_phase returned an error"))
            .is_ok()
        {
            // Preflight
            {
                // System has no preflight step
            }

            // Systems
            {
                let camera_inputs: [&[CameraComponent]; 1] = [
                    &self.archetypes.collection.camera.cameras,
                ];

                // Apply CameraRenderSystem to all archetypes
                self.systems.camera_render.apply_all(
                            &self.states.wgpu_render,
                            &mut self.states.active_camera,
                    camera_inputs,
                );
            }

            // Postflight
            {
                // System has no preflight step
            }

            self.systems.camera_render.on_end_phase(
                        &self.states.wgpu_render,
                        &self.states.active_camera,
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
        let is_render_background_ready = self.systems.render_background.is_ready(
                       &self.states.wgpu_render,
           );
        let is_render_background_ready = is_render_background_ready &
            self.systems.render_background.on_begin_phase(
                        &self.states.wgpu_render,
            )
            .inspect_err(|error| tracing::error!(%error, "RenderBackgroundSystem::on_begin_phase returned an error"))
            .is_ok();

        {
            // TODO: Skip spawning thread here when there is only one system, or when all run in sequence
            rayon::scope(|s| {
                // TODO: Instead of parallelizing systems (and then have them access archetypes), parallelize archetypes and apply all systems. This should improve data parallelism because it keeps caches hot.
                if is_render_background_ready {
                    s.spawn(|_| {
                        // Preflight
                        {
                            // System has no preflight step
                        }

                        // Systems
                        {
                            let background_color_inputs: [&[BackgroundColorComponent]; 1] = [
                                &self.archetypes.collection.scene_background.background_colors,
                            ];

                            // Apply RenderBackgroundSystem to all archetypes
                            self.systems.render_background.apply_all(
                                        &mut self.states.wgpu_render,
                                background_color_inputs,
                            );
                        }

                        // Postflight
                        {
                            // System has no preflight step
                        }
                    });
                }
            });
        }

        self.systems.render_background.on_end_phase(
                    &self.states.wgpu_render,
        )
        .inspect_err(|error| tracing::error!(%error, "RenderBackgroundSystem::on_end_phase returned an error"))
        .ok();

        // System group 2
        let is_render_sky_ready = self.systems.render_sky.is_ready(
                       &self.states.wgpu_render,
           );
        let is_render_sky_ready = is_render_sky_ready &
            self.systems.render_sky.on_begin_phase(
                        &self.states.wgpu_render,
            )
            .inspect_err(|error| tracing::error!(%error, "RenderSkySystem::on_begin_phase returned an error"))
            .is_ok();

        {
            // TODO: Skip spawning thread here when there is only one system, or when all run in sequence
            rayon::scope(|s| {
                // TODO: Instead of parallelizing systems (and then have them access archetypes), parallelize archetypes and apply all systems. This should improve data parallelism because it keeps caches hot.
                if is_render_sky_ready {
                    s.spawn(|_| {
                        // Preflight
                        {
                            // System has no preflight step
                        }

                        // Systems
                        {
                            let skydome_inputs: [&[SkydomeComponent]; 1] = [
                                &self.archetypes.collection.environment.skydomes,
                            ];

                            // Apply RenderSkySystem to all archetypes
                            self.systems.render_sky.apply_all(
                                        &mut self.states.wgpu_render,
                                skydome_inputs,
                            );
                        }

                        // Postflight
                        {
                            // System has no preflight step
                        }
                    });
                }
            });
        }

        self.systems.render_sky.on_end_phase(
                    &self.states.wgpu_render,
        )
        .inspect_err(|error| tracing::error!(%error, "RenderSkySystem::on_end_phase returned an error"))
        .ok();

        // System group 3
        let is_dynamic_light_ready = self.systems.dynamic_light.is_ready(
               &self.context,
                       &self.states.wgpu_render,
           );
        let is_dynamic_light_ready = is_dynamic_light_ready &
            self.systems.dynamic_light.on_begin_phase(
                &self.context,
                        &self.states.wgpu_render,
            )
            .inspect_err(|error| tracing::error!(%error, "DynamicLightSystem::on_begin_phase returned an error"))
            .is_ok();

        {
            // TODO: Skip spawning thread here when there is only one system, or when all run in sequence
            rayon::scope(|s| {
                // TODO: Instead of parallelizing systems (and then have them access archetypes), parallelize archetypes and apply all systems. This should improve data parallelism because it keeps caches hot.
                if is_dynamic_light_ready {
                    s.spawn(|_| {
                        // Preflight
                        {
                            // System has no preflight step
                        }

                        // Systems
                        {
                            let color_inputs: [&[ColorComponent]; 1] = [
                                &self.archetypes.collection.dynamic_light.colors,
                            ];
                            let dynamic_light_outputs: [&mut [DynamicLightComponent]; 1] = [
                                &mut self.archetypes.collection.dynamic_light.dynamic_lights,
                            ];

                            // Apply DynamicLightSystem to all archetypes
                            self.systems.dynamic_light.apply_all(
                                &self.context,
                                        &mut self.states.wgpu_render,
                                color_inputs,
                                dynamic_light_outputs,
                            );
                        }

                        // Postflight
                        {
                            // System has no preflight step
                        }
                    });
                }
            });
        }

        self.systems.dynamic_light.on_end_phase(
            &self.context,
                    &self.states.wgpu_render,
        )
        .inspect_err(|error| tracing::error!(%error, "DynamicLightSystem::on_end_phase returned an error"))
        .ok();

        // System group 4
        let is_tile_render_ready = self.systems.tile_render.is_ready(
                       &self.states.wgpu_render,
           );
        let is_tile_render_ready = is_tile_render_ready &
            self.systems.tile_render.on_begin_phase(
                        &self.states.wgpu_render,
            )
            .inspect_err(|error| tracing::error!(%error, "TileRenderSystem::on_begin_phase returned an error"))
            .is_ok();

        {
            // TODO: Skip spawning thread here when there is only one system, or when all run in sequence
            rayon::scope(|s| {
                // TODO: Instead of parallelizing systems (and then have them access archetypes), parallelize archetypes and apply all systems. This should improve data parallelism because it keeps caches hot.
                if is_tile_render_ready {
                    s.spawn(|_| {
                        // Preflight
                        {
                            // System has no preflight step
                        }

                        // Systems
                        {
                            let tile_inputs: [&[TileComponent]; 1] = [
                                &self.archetypes.collection.tile.tiles,
                            ];
                            let position_inputs: [&[PositionComponent]; 1] = [
                                &self.archetypes.collection.tile.positions,
                            ];

                            // Apply TileRenderSystem to all archetypes
                            self.systems.tile_render.apply_all(
                                        &mut self.states.wgpu_render,
                                tile_inputs,
                                position_inputs,
                            );
                        }

                        // Postflight
                        {
                            // System has no preflight step
                        }
                    });
                }
            });
        }

        self.systems.tile_render.on_end_phase(
                    &self.states.wgpu_render,
        )
        .inspect_err(|error| tracing::error!(%error, "TileRenderSystem::on_end_phase returned an error"))
        .ok();

        // System group 5
        let is_render_ready = self.systems.render.is_ready(
                       &self.states.wgpu_render,
           );
        let is_render_ready = is_render_ready &
            self.systems.render.on_begin_phase(
                        &self.states.wgpu_render,
            )
            .inspect_err(|error| tracing::error!(%error, "RenderSystem::on_begin_phase returned an error"))
            .is_ok();

        {
            // TODO: Skip spawning thread here when there is only one system, or when all run in sequence
            rayon::scope(|s| {
                // TODO: Instead of parallelizing systems (and then have them access archetypes), parallelize archetypes and apply all systems. This should improve data parallelism because it keeps caches hot.
                if is_render_ready {
                    s.spawn(|_| {
                        // Preflight
                        {
                            // System has no preflight step
                        }

                        // Systems
                        {
                            let position_inputs: [&[PositionComponent]; 1] = [
                                &self.archetypes.collection.cube.positions,
                            ];
                            let transform_inputs: [&[TransformComponent]; 1] = [
                                &self.archetypes.collection.cube.transforms,
                            ];
                            let model_inputs: [&[ModelComponent]; 1] = [
                                &self.archetypes.collection.cube.models,
                            ];

                            // Apply RenderSystem to all archetypes
                            self.systems.render.apply_all(
                                        &mut self.states.wgpu_render,
                                position_inputs,
                                transform_inputs,
                                model_inputs,
                            );
                        }

                        // Postflight
                        {
                            // System has no preflight step
                        }
                    });
                }
            });
        }

        self.systems.render.on_end_phase(
                    &self.states.wgpu_render,
        )
        .inspect_err(|error| tracing::error!(%error, "RenderSystem::on_end_phase returned an error"))
        .ok();

        // System group 6
        let is_vga_blit_ready = self.systems.vga_blit.is_ready(
                       &self.states.wgpu_render,
           );
        let is_vga_blit_ready = is_vga_blit_ready &
            self.systems.vga_blit.on_begin_phase(
                        &self.states.wgpu_render,
            )
            .inspect_err(|error| tracing::error!(%error, "VgaBlitSystem::on_begin_phase returned an error"))
            .is_ok();

        {
            // TODO: Skip spawning thread here when there is only one system, or when all run in sequence
            rayon::scope(|s| {
                // TODO: Instead of parallelizing systems (and then have them access archetypes), parallelize archetypes and apply all systems. This should improve data parallelism because it keeps caches hot.
                if is_vga_blit_ready {
                    s.spawn(|_| {
                        // Preflight
                        {
                            // System has no preflight step
                        }

                        // Systems
                        {
                            let background_color_inputs: [&[BackgroundColorComponent]; 1] = [
                                &self.archetypes.collection.scene_background.background_colors,
                            ];

                            // Apply VgaBlitSystem to all archetypes
                            self.systems.vga_blit.apply_all(
                                        &mut self.states.wgpu_render,
                                background_color_inputs,
                            );
                        }

                        // Postflight
                        {
                            // System has no preflight step
                        }
                    });
                }
            });
        }

        self.systems.vga_blit.on_end_phase(
                    &self.states.wgpu_render,
        )
        .inspect_err(|error| tracing::error!(%error, "VgaBlitSystem::on_end_phase returned an error"))
        .ok();

        // System group 7
        let is_frame_globals_ready = self.systems.frame_globals.is_ready(
                       &self.states.wgpu_render,
           );
        let is_frame_globals_ready = is_frame_globals_ready &
            self.systems.frame_globals.on_begin_phase(
                        &self.states.wgpu_render,
            )
            .inspect_err(|error| tracing::error!(%error, "FrameGlobalsSystem::on_begin_phase returned an error"))
            .is_ok();

        {
            // TODO: Skip spawning thread here when there is only one system, or when all run in sequence
            rayon::scope(|s| {
                // TODO: Instead of parallelizing systems (and then have them access archetypes), parallelize archetypes and apply all systems. This should improve data parallelism because it keeps caches hot.
                if is_frame_globals_ready {
                    s.spawn(|_| {
                        // Preflight
                        {
                            // System has no preflight step
                        }

                        // Systems
                        {
                            let time_of_day_inputs: [&[TimeOfDayComponent]; 1] = [
                                &self.archetypes.collection.environment.time_of_days,
                            ];
                            let sun_inputs: [&[SunComponent]; 1] = [
                                &self.archetypes.collection.environment.suns,
                            ];
                            let fog_inputs: [&[FogComponent]; 1] = [
                                &self.archetypes.collection.environment.fogs,
                            ];
                            let ambient_light_inputs: [&[AmbientLightComponent]; 1] = [
                                &self.archetypes.collection.environment.ambient_lights,
                            ];
                            let stars_inputs: [&[StarsComponent]; 1] = [
                                &self.archetypes.collection.environment.stars,
                            ];
                            let skydome_inputs: [&[SkydomeComponent]; 1] = [
                                &self.archetypes.collection.environment.skydomes,
                            ];

                            // Apply FrameGlobalsSystem to all archetypes
                            self.systems.frame_globals.apply_all(
                                        &mut self.states.wgpu_render,
                                time_of_day_inputs,
                                sun_inputs,
                                fog_inputs,
                                ambient_light_inputs,
                                stars_inputs,
                                skydome_inputs,
                            );
                        }

                        // Postflight
                        {
                            // System has no preflight step
                        }
                    });
                }
            });
        }

        self.systems.frame_globals.on_end_phase(
                    &self.states.wgpu_render,
        )
        .inspect_err(|error| tracing::error!(%error, "FrameGlobalsSystem::on_end_phase returned an error"))
        .ok();

        // System group 8
        let is_camera_render_ready = self.systems.camera_render.is_ready(
                       &self.states.wgpu_render,
                       &self.states.active_camera,
           );
        let is_camera_render_ready = is_camera_render_ready &
            self.systems.camera_render.on_begin_phase(
                        &self.states.wgpu_render,
                        &self.states.active_camera,
            )
            .inspect_err(|error| tracing::error!(%error, "CameraRenderSystem::on_begin_phase returned an error"))
            .is_ok();

        {
            // TODO: Skip spawning thread here when there is only one system, or when all run in sequence
            rayon::scope(|s| {
                // TODO: Instead of parallelizing systems (and then have them access archetypes), parallelize archetypes and apply all systems. This should improve data parallelism because it keeps caches hot.
                if is_camera_render_ready {
                    s.spawn(|_| {
                        // Preflight
                        {
                            // System has no preflight step
                        }

                        // Systems
                        {
                            let camera_inputs: [&[CameraComponent]; 1] = [
                                &self.archetypes.collection.camera.cameras,
                            ];

                            // Apply CameraRenderSystem to all archetypes
                            self.systems.camera_render.apply_all(
                                        &self.states.wgpu_render,
                                        &mut self.states.active_camera,
                                camera_inputs,
                            );
                        }

                        // Postflight
                        {
                            // System has no preflight step
                        }
                    });
                }
            });
        }

        self.systems.camera_render.on_end_phase(
                    &self.states.wgpu_render,
                    &self.states.active_camera,
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

    /// Sends a command.
    ///
    /// Note that commands are picked up before and after simulation phases (not between system runs).
    /// As a result, issuing commands from the outside world will not have any effect until the next
    /// ECS phase run is executed.
    #[inline]
    pub fn command(&mut self, command: WorldCommand<Q::UserCommand>) -> Result<(), Q::Error>
    where
        Q: WorldCommandSender
    {
        self.command_queue.send(command)
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
                        WorldCommand::User(cmd) => self.handle_user_command(cmd),
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

    #[inline]
    fn handle_user_command(&mut self, data: <Q as WorldUserCommand>::UserCommand)
    where
        Q: WorldCommandReceiver,
        Self: WorldUserCommandHandler<UserCommand = <Q as WorldUserCommand>::UserCommand>
    {
        WorldUserCommandHandler::handle_user_command(self, data);
    }

    fn handle_spawn_command(&mut self, data: ArchetypeEntityData) {
        match data {
            // Accept only archetypes known to the world
            ArchetypeEntityData::WgpuUniformBindGroup(data) => {
                let id = self.spawn_wgpu_uniform_bind_group(data);
                tracing::trace!(%id, "Spawned WgpuUniformBindGroup entity");
            },
            ArchetypeEntityData::WgpuShader(data) => {
                let id = self.spawn_wgpu_shader(data);
                tracing::trace!(%id, "Spawned WgpuShader entity");
            },
            ArchetypeEntityData::WgpuTexture(data) => {
                let id = self.spawn_wgpu_texture(data);
                tracing::trace!(%id, "Spawned WgpuTexture entity");
            },
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

    fn handle_despawn_command(&mut self, id: ::sillyecs::EntityId) -> Result<(), DespawnError> {
         if let Some(loc) = self.archetypes.entity_locations.remove(&id) {
            let result = match loc.archetype {
                WgpuUniformBindGroupArchetype::ID => {
                    self.archetypes
                        .collection
                        .wgpu_uniform_bind_group
                        .drop_at_index(loc.index)
                        .map_err(|index| DespawnError::InvalidIndexInArchetype(index, WgpuUniformBindGroupArchetype::ID))?
                }
                WgpuShaderArchetype::ID => {
                    self.archetypes
                        .collection
                        .wgpu_shader
                        .drop_at_index(loc.index)
                        .map_err(|index| DespawnError::InvalidIndexInArchetype(index, WgpuShaderArchetype::ID))?
                }
                WgpuTextureArchetype::ID => {
                    self.archetypes
                        .collection
                        .wgpu_texture
                        .drop_at_index(loc.index)
                        .map_err(|index| DespawnError::InvalidIndexInArchetype(index, WgpuTextureArchetype::ID))?
                }
                EnvironmentArchetype::ID => {
                    self.archetypes
                        .collection
                        .environment
                        .drop_at_index(loc.index)
                        .map_err(|index| DespawnError::InvalidIndexInArchetype(index, EnvironmentArchetype::ID))?
                }
                CameraArchetype::ID => {
                    self.archetypes
                        .collection
                        .camera
                        .drop_at_index(loc.index)
                        .map_err(|index| DespawnError::InvalidIndexInArchetype(index, CameraArchetype::ID))?
                }
                DynamicLightArchetype::ID => {
                    self.archetypes
                        .collection
                        .dynamic_light
                        .drop_at_index(loc.index)
                        .map_err(|index| DespawnError::InvalidIndexInArchetype(index, DynamicLightArchetype::ID))?
                }
                TileArchetype::ID => {
                    self.archetypes
                        .collection
                        .tile
                        .drop_at_index(loc.index)
                        .map_err(|index| DespawnError::InvalidIndexInArchetype(index, TileArchetype::ID))?
                }
                CubeArchetype::ID => {
                    self.archetypes
                        .collection
                        .cube
                        .drop_at_index(loc.index)
                        .map_err(|index| DespawnError::InvalidIndexInArchetype(index, CubeArchetype::ID))?
                }
                PlayerArchetype::ID => {
                    self.archetypes
                        .collection
                        .player
                        .drop_at_index(loc.index)
                        .map_err(|index| DespawnError::InvalidIndexInArchetype(index, PlayerArchetype::ID))?
                }
                ForegroundObjectArchetype::ID => {
                    self.archetypes
                        .collection
                        .foreground_object
                        .drop_at_index(loc.index)
                        .map_err(|index| DespawnError::InvalidIndexInArchetype(index, ForegroundObjectArchetype::ID))?
                }
                BackgroundObjectArchetype::ID => {
                    self.archetypes
                        .collection
                        .background_object
                        .drop_at_index(loc.index)
                        .map_err(|index| DespawnError::InvalidIndexInArchetype(index, BackgroundObjectArchetype::ID))?
                }
                SceneBackgroundArchetype::ID => {
                    self.archetypes
                        .collection
                        .scene_background
                        .drop_at_index(loc.index)
                        .map_err(|index| DespawnError::InvalidIndexInArchetype(index, SceneBackgroundArchetype::ID))?
                }
                // TODO: Make it a non-panic on release builds?
                #[allow(unreachable_patterns)]
                archetype_id => unreachable!("Cannot remove entity {id} from archetype {archetype_id} which is not known to this world.")
            };

            if let Some(moved_entity) = result {
                self.archetypes.entity_locations.insert(moved_entity, EntityArchetypeRef {
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
    /// The current start time for the [`WgpuReinit`](WgpuReinitPhase) phase.
    current_wgpu_reinit_start: std::time::Instant,
    /// The previous start time for the [`WgpuReinit`](WgpuReinitPhase) phase.
    previous_wgpu_reinit_start: std::time::Instant,
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
            current_wgpu_reinit_start: now,
            previous_wgpu_reinit_start: now,
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
    EntityNotFound(::sillyecs::EntityId),
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
            ArchetypeId::WgpuUniformBindGroup => {
                self.frontload_wgpu_uniform_bind_groups_by_indices_sorted(
                    indices_to_frontload,
                    previous_frontload_pivot
                )
            },
            ArchetypeId::WgpuShader => {
                self.frontload_wgpu_shaders_by_indices_sorted(
                    indices_to_frontload,
                    previous_frontload_pivot
                )
            },
            ArchetypeId::WgpuTexture => {
                self.frontload_wgpu_textures_by_indices_sorted(
                    indices_to_frontload,
                    previous_frontload_pivot
                )
            },
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
    pub fn frontload_wgpu_uniform_bind_groups(
        &mut self,
        entities_to_frontload: &[::sillyecs::EntityId],
        previous_frontload_pivot: Option<usize>
    ) -> usize {
        self.archetypes
            .collection
            .wgpu_uniform_bind_group
            .frontload(
                &mut self.archetypes.entity_locations,
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
    pub fn frontload_wgpu_uniform_bind_groups_by_indices_sorted<I>(
        &mut self,
        indices_to_frontload: I,
        previous_frontload_pivot: Option<usize>
    ) -> usize
    where
        I: AsRef<[usize]>
    {
        self.archetypes
            .collection
            .wgpu_uniform_bind_group
            .frontload_by_indices_sorted(
                &mut self.archetypes.entity_locations,
                indices_to_frontload,
                previous_frontload_pivot
            )
    }

    /// Frontloads entities satisfying `should_frontload`.
    /// Returns the number of entities frontloaded.
    #[inline(always)]
    #[allow(dead_code)]
    pub fn frontload_wgpu_uniform_bind_groups_by_scan<F>(
        &mut self,
        should_frontload: F,
    ) -> usize
    where
        for<'archetype> F: FnMut(&WgpuUniformBindGroupEntityRef<'archetype>) -> bool,
    {
        self.archetypes
            .collection
            .wgpu_uniform_bind_group
            .frontload_scan(
                &mut self.archetypes.entity_locations,
                should_frontload
            )
    }

    /// Frontloads entities provided by their IDs (e.g., from quadtree results).
    /// Returns the count of frontloaded entities.
    #[inline(always)]
    #[allow(dead_code)]
    pub fn frontload_wgpu_shaders(
        &mut self,
        entities_to_frontload: &[::sillyecs::EntityId],
        previous_frontload_pivot: Option<usize>
    ) -> usize {
        self.archetypes
            .collection
            .wgpu_shader
            .frontload(
                &mut self.archetypes.entity_locations,
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
    pub fn frontload_wgpu_shaders_by_indices_sorted<I>(
        &mut self,
        indices_to_frontload: I,
        previous_frontload_pivot: Option<usize>
    ) -> usize
    where
        I: AsRef<[usize]>
    {
        self.archetypes
            .collection
            .wgpu_shader
            .frontload_by_indices_sorted(
                &mut self.archetypes.entity_locations,
                indices_to_frontload,
                previous_frontload_pivot
            )
    }

    /// Frontloads entities satisfying `should_frontload`.
    /// Returns the number of entities frontloaded.
    #[inline(always)]
    #[allow(dead_code)]
    pub fn frontload_wgpu_shaders_by_scan<F>(
        &mut self,
        should_frontload: F,
    ) -> usize
    where
        for<'archetype> F: FnMut(&WgpuShaderEntityRef<'archetype>) -> bool,
    {
        self.archetypes
            .collection
            .wgpu_shader
            .frontload_scan(
                &mut self.archetypes.entity_locations,
                should_frontload
            )
    }

    /// Frontloads entities provided by their IDs (e.g., from quadtree results).
    /// Returns the count of frontloaded entities.
    #[inline(always)]
    #[allow(dead_code)]
    pub fn frontload_wgpu_textures(
        &mut self,
        entities_to_frontload: &[::sillyecs::EntityId],
        previous_frontload_pivot: Option<usize>
    ) -> usize {
        self.archetypes
            .collection
            .wgpu_texture
            .frontload(
                &mut self.archetypes.entity_locations,
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
    pub fn frontload_wgpu_textures_by_indices_sorted<I>(
        &mut self,
        indices_to_frontload: I,
        previous_frontload_pivot: Option<usize>
    ) -> usize
    where
        I: AsRef<[usize]>
    {
        self.archetypes
            .collection
            .wgpu_texture
            .frontload_by_indices_sorted(
                &mut self.archetypes.entity_locations,
                indices_to_frontload,
                previous_frontload_pivot
            )
    }

    /// Frontloads entities satisfying `should_frontload`.
    /// Returns the number of entities frontloaded.
    #[inline(always)]
    #[allow(dead_code)]
    pub fn frontload_wgpu_textures_by_scan<F>(
        &mut self,
        should_frontload: F,
    ) -> usize
    where
        for<'archetype> F: FnMut(&WgpuTextureEntityRef<'archetype>) -> bool,
    {
        self.archetypes
            .collection
            .wgpu_texture
            .frontload_scan(
                &mut self.archetypes.entity_locations,
                should_frontload
            )
    }

    /// Frontloads entities provided by their IDs (e.g., from quadtree results).
    /// Returns the count of frontloaded entities.
    #[inline(always)]
    #[allow(dead_code)]
    pub fn frontload_environments(
        &mut self,
        entities_to_frontload: &[::sillyecs::EntityId],
        previous_frontload_pivot: Option<usize>
    ) -> usize {
        self.archetypes
            .collection
            .environment
            .frontload(
                &mut self.archetypes.entity_locations,
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
            .collection
            .environment
            .frontload_by_indices_sorted(
                &mut self.archetypes.entity_locations,
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
            .collection
            .environment
            .frontload_scan(
                &mut self.archetypes.entity_locations,
                should_frontload
            )
    }

    /// Frontloads entities provided by their IDs (e.g., from quadtree results).
    /// Returns the count of frontloaded entities.
    #[inline(always)]
    #[allow(dead_code)]
    pub fn frontload_cameras(
        &mut self,
        entities_to_frontload: &[::sillyecs::EntityId],
        previous_frontload_pivot: Option<usize>
    ) -> usize {
        self.archetypes
            .collection
            .camera
            .frontload(
                &mut self.archetypes.entity_locations,
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
            .collection
            .camera
            .frontload_by_indices_sorted(
                &mut self.archetypes.entity_locations,
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
            .collection
            .camera
            .frontload_scan(
                &mut self.archetypes.entity_locations,
                should_frontload
            )
    }

    /// Frontloads entities provided by their IDs (e.g., from quadtree results).
    /// Returns the count of frontloaded entities.
    #[inline(always)]
    #[allow(dead_code)]
    pub fn frontload_dynamic_lights(
        &mut self,
        entities_to_frontload: &[::sillyecs::EntityId],
        previous_frontload_pivot: Option<usize>
    ) -> usize {
        self.archetypes
            .collection
            .dynamic_light
            .frontload(
                &mut self.archetypes.entity_locations,
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
            .collection
            .dynamic_light
            .frontload_by_indices_sorted(
                &mut self.archetypes.entity_locations,
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
            .collection
            .dynamic_light
            .frontload_scan(
                &mut self.archetypes.entity_locations,
                should_frontload
            )
    }

    /// Frontloads entities provided by their IDs (e.g., from quadtree results).
    /// Returns the count of frontloaded entities.
    #[inline(always)]
    #[allow(dead_code)]
    pub fn frontload_tiles(
        &mut self,
        entities_to_frontload: &[::sillyecs::EntityId],
        previous_frontload_pivot: Option<usize>
    ) -> usize {
        self.archetypes
            .collection
            .tile
            .frontload(
                &mut self.archetypes.entity_locations,
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
            .collection
            .tile
            .frontload_by_indices_sorted(
                &mut self.archetypes.entity_locations,
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
            .collection
            .tile
            .frontload_scan(
                &mut self.archetypes.entity_locations,
                should_frontload
            )
    }

    /// Frontloads entities provided by their IDs (e.g., from quadtree results).
    /// Returns the count of frontloaded entities.
    #[inline(always)]
    #[allow(dead_code)]
    pub fn frontload_cubes(
        &mut self,
        entities_to_frontload: &[::sillyecs::EntityId],
        previous_frontload_pivot: Option<usize>
    ) -> usize {
        self.archetypes
            .collection
            .cube
            .frontload(
                &mut self.archetypes.entity_locations,
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
            .collection
            .cube
            .frontload_by_indices_sorted(
                &mut self.archetypes.entity_locations,
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
            .collection
            .cube
            .frontload_scan(
                &mut self.archetypes.entity_locations,
                should_frontload
            )
    }

    /// Frontloads entities provided by their IDs (e.g., from quadtree results).
    /// Returns the count of frontloaded entities.
    #[inline(always)]
    #[allow(dead_code)]
    pub fn frontload_players(
        &mut self,
        entities_to_frontload: &[::sillyecs::EntityId],
        previous_frontload_pivot: Option<usize>
    ) -> usize {
        self.archetypes
            .collection
            .player
            .frontload(
                &mut self.archetypes.entity_locations,
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
            .collection
            .player
            .frontload_by_indices_sorted(
                &mut self.archetypes.entity_locations,
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
            .collection
            .player
            .frontload_scan(
                &mut self.archetypes.entity_locations,
                should_frontload
            )
    }

    /// Frontloads entities provided by their IDs (e.g., from quadtree results).
    /// Returns the count of frontloaded entities.
    #[inline(always)]
    #[allow(dead_code)]
    pub fn frontload_foreground_objects(
        &mut self,
        entities_to_frontload: &[::sillyecs::EntityId],
        previous_frontload_pivot: Option<usize>
    ) -> usize {
        self.archetypes
            .collection
            .foreground_object
            .frontload(
                &mut self.archetypes.entity_locations,
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
            .collection
            .foreground_object
            .frontload_by_indices_sorted(
                &mut self.archetypes.entity_locations,
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
            .collection
            .foreground_object
            .frontload_scan(
                &mut self.archetypes.entity_locations,
                should_frontload
            )
    }

    /// Frontloads entities provided by their IDs (e.g., from quadtree results).
    /// Returns the count of frontloaded entities.
    #[inline(always)]
    #[allow(dead_code)]
    pub fn frontload_background_objects(
        &mut self,
        entities_to_frontload: &[::sillyecs::EntityId],
        previous_frontload_pivot: Option<usize>
    ) -> usize {
        self.archetypes
            .collection
            .background_object
            .frontload(
                &mut self.archetypes.entity_locations,
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
            .collection
            .background_object
            .frontload_by_indices_sorted(
                &mut self.archetypes.entity_locations,
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
            .collection
            .background_object
            .frontload_scan(
                &mut self.archetypes.entity_locations,
                should_frontload
            )
    }

    /// Frontloads entities provided by their IDs (e.g., from quadtree results).
    /// Returns the count of frontloaded entities.
    #[inline(always)]
    #[allow(dead_code)]
    pub fn frontload_scene_backgrounds(
        &mut self,
        entities_to_frontload: &[::sillyecs::EntityId],
        previous_frontload_pivot: Option<usize>
    ) -> usize {
        self.archetypes
            .collection
            .scene_background
            .frontload(
                &mut self.archetypes.entity_locations,
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
            .collection
            .scene_background
            .frontload_by_indices_sorted(
                &mut self.archetypes.entity_locations,
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
            .collection
            .scene_background
            .frontload_scan(
                &mut self.archetypes.entity_locations,
                should_frontload
            )
    }
}

pub trait ComponentAccess {

    /// Gets the [`TimeOfDay`](TimeOfDayComponent) component of the specified entity.
    #[allow(dead_code, unused)]
    #[inline]
    fn get_time_of_day_component(&self, entity_id: ::sillyecs::EntityId) -> Option<&TimeOfDayComponent> {
        None
    }

    /// Gets the [`Sun`](SunComponent) component of the specified entity.
    #[allow(dead_code, unused)]
    #[inline]
    fn get_sun_component(&self, entity_id: ::sillyecs::EntityId) -> Option<&SunComponent> {
        None
    }

    /// Gets the [`Skydome`](SkydomeComponent) component of the specified entity.
    #[allow(dead_code, unused)]
    #[inline]
    fn get_skydome_component(&self, entity_id: ::sillyecs::EntityId) -> Option<&SkydomeComponent> {
        None
    }

    /// Gets the [`Fog`](FogComponent) component of the specified entity.
    #[allow(dead_code, unused)]
    #[inline]
    fn get_fog_component(&self, entity_id: ::sillyecs::EntityId) -> Option<&FogComponent> {
        None
    }

    /// Gets the [`AmbientLight`](AmbientLightComponent) component of the specified entity.
    #[allow(dead_code, unused)]
    #[inline]
    fn get_ambient_light_component(&self, entity_id: ::sillyecs::EntityId) -> Option<&AmbientLightComponent> {
        None
    }

    /// Gets the [`Stars`](StarsComponent) component of the specified entity.
    #[allow(dead_code, unused)]
    #[inline]
    fn get_stars_component(&self, entity_id: ::sillyecs::EntityId) -> Option<&StarsComponent> {
        None
    }

    /// Gets the [`Camera`](CameraComponent) component of the specified entity.
    #[allow(dead_code, unused)]
    #[inline]
    fn get_camera_component(&self, entity_id: ::sillyecs::EntityId) -> Option<&CameraComponent> {
        None
    }

    /// Gets the [`CameraFpsController`](CameraFpsControllerComponent) component of the specified entity.
    #[allow(dead_code, unused)]
    #[inline]
    fn get_camera_fps_controller_component(&self, entity_id: ::sillyecs::EntityId) -> Option<&CameraFpsControllerComponent> {
        None
    }

    /// Gets the [`Position`](PositionComponent) component of the specified entity.
    #[allow(dead_code, unused)]
    #[inline]
    fn get_position_component(&self, entity_id: ::sillyecs::EntityId) -> Option<&PositionComponent> {
        None
    }

    /// Gets the [`Transform`](TransformComponent) component of the specified entity.
    #[allow(dead_code, unused)]
    #[inline]
    fn get_transform_component(&self, entity_id: ::sillyecs::EntityId) -> Option<&TransformComponent> {
        None
    }

    /// Gets the [`Velocity`](VelocityComponent) component of the specified entity.
    #[allow(dead_code, unused)]
    #[inline]
    fn get_velocity_component(&self, entity_id: ::sillyecs::EntityId) -> Option<&VelocityComponent> {
        None
    }

    /// Gets the [`Tile`](TileComponent) component of the specified entity.
    #[allow(dead_code, unused)]
    #[inline]
    fn get_tile_component(&self, entity_id: ::sillyecs::EntityId) -> Option<&TileComponent> {
        None
    }

    /// Gets the [`Health`](HealthComponent) component of the specified entity.
    #[allow(dead_code, unused)]
    #[inline]
    fn get_health_component(&self, entity_id: ::sillyecs::EntityId) -> Option<&HealthComponent> {
        None
    }

    /// Gets the [`Collider`](ColliderComponent) component of the specified entity.
    #[allow(dead_code, unused)]
    #[inline]
    fn get_collider_component(&self, entity_id: ::sillyecs::EntityId) -> Option<&ColliderComponent> {
        None
    }

    /// Gets the [`Model`](ModelComponent) component of the specified entity.
    #[allow(dead_code, unused)]
    #[inline]
    fn get_model_component(&self, entity_id: ::sillyecs::EntityId) -> Option<&ModelComponent> {
        None
    }

    /// Gets the [`Color`](ColorComponent) component of the specified entity.
    #[allow(dead_code, unused)]
    #[inline]
    fn get_color_component(&self, entity_id: ::sillyecs::EntityId) -> Option<&ColorComponent> {
        None
    }

    /// Gets the [`BackgroundColor`](BackgroundColorComponent) component of the specified entity.
    #[allow(dead_code, unused)]
    #[inline]
    fn get_background_color_component(&self, entity_id: ::sillyecs::EntityId) -> Option<&BackgroundColorComponent> {
        None
    }

    /// Gets the [`DynamicLight`](DynamicLightComponent) component of the specified entity.
    #[allow(dead_code, unused)]
    #[inline]
    fn get_dynamic_light_component(&self, entity_id: ::sillyecs::EntityId) -> Option<&DynamicLightComponent> {
        None
    }

    /// Gets the [`WgpuShader`](WgpuShaderComponent) component of the specified entity.
    #[allow(dead_code, unused)]
    #[inline]
    fn get_wgpu_shader_component(&self, entity_id: ::sillyecs::EntityId) -> Option<&WgpuShaderComponent> {
        None
    }

    /// Gets the [`TextureSource`](TextureSourceComponent) component of the specified entity.
    #[allow(dead_code, unused)]
    #[inline]
    fn get_texture_source_component(&self, entity_id: ::sillyecs::EntityId) -> Option<&TextureSourceComponent> {
        None
    }

    /// Gets the [`WgpuTexture`](WgpuTextureComponent) component of the specified entity.
    #[allow(dead_code, unused)]
    #[inline]
    fn get_wgpu_texture_component(&self, entity_id: ::sillyecs::EntityId) -> Option<&WgpuTextureComponent> {
        None
    }

    /// Gets the [`UniformType`](UniformTypeComponent) component of the specified entity.
    #[allow(dead_code, unused)]
    #[inline]
    fn get_uniform_type_component(&self, entity_id: ::sillyecs::EntityId) -> Option<&UniformTypeComponent> {
        None
    }

    /// Gets the [`WgpuBindGroupLayout`](WgpuBindGroupLayoutComponent) component of the specified entity.
    #[allow(dead_code, unused)]
    #[inline]
    fn get_wgpu_bind_group_layout_component(&self, entity_id: ::sillyecs::EntityId) -> Option<&WgpuBindGroupLayoutComponent> {
        None
    }

    /// Gets the [`WgpuBindGroup`](WgpuBindGroupComponent) component of the specified entity.
    #[allow(dead_code, unused)]
    #[inline]
    fn get_wgpu_bind_group_component(&self, entity_id: ::sillyecs::EntityId) -> Option<&WgpuBindGroupComponent> {
        None
    }

    /// Gets the [`WgpuBuffer`](WgpuBufferComponent) component of the specified entity.
    #[allow(dead_code, unused)]
    #[inline]
    fn get_wgpu_buffer_component(&self, entity_id: ::sillyecs::EntityId) -> Option<&WgpuBufferComponent> {
        None
    }
}

pub trait ComponentAccessMut: ComponentAccess {

    /// Mutably gets the [`TimeOfDay`](TimeOfDayComponent) component of the specified entity.
    #[allow(dead_code, unused)]
    #[inline]
    fn get_time_of_day_component_mut(&mut self, entity_id: ::sillyecs::EntityId) -> Option<&mut TimeOfDayComponent> {
        None
    }

    /// Mutably gets the [`Sun`](SunComponent) component of the specified entity.
    #[allow(dead_code, unused)]
    #[inline]
    fn get_sun_component_mut(&mut self, entity_id: ::sillyecs::EntityId) -> Option<&mut SunComponent> {
        None
    }

    /// Mutably gets the [`Skydome`](SkydomeComponent) component of the specified entity.
    #[allow(dead_code, unused)]
    #[inline]
    fn get_skydome_component_mut(&mut self, entity_id: ::sillyecs::EntityId) -> Option<&mut SkydomeComponent> {
        None
    }

    /// Mutably gets the [`Fog`](FogComponent) component of the specified entity.
    #[allow(dead_code, unused)]
    #[inline]
    fn get_fog_component_mut(&mut self, entity_id: ::sillyecs::EntityId) -> Option<&mut FogComponent> {
        None
    }

    /// Mutably gets the [`AmbientLight`](AmbientLightComponent) component of the specified entity.
    #[allow(dead_code, unused)]
    #[inline]
    fn get_ambient_light_component_mut(&mut self, entity_id: ::sillyecs::EntityId) -> Option<&mut AmbientLightComponent> {
        None
    }

    /// Mutably gets the [`Stars`](StarsComponent) component of the specified entity.
    #[allow(dead_code, unused)]
    #[inline]
    fn get_stars_component_mut(&mut self, entity_id: ::sillyecs::EntityId) -> Option<&mut StarsComponent> {
        None
    }

    /// Mutably gets the [`Camera`](CameraComponent) component of the specified entity.
    #[allow(dead_code, unused)]
    #[inline]
    fn get_camera_component_mut(&mut self, entity_id: ::sillyecs::EntityId) -> Option<&mut CameraComponent> {
        None
    }

    /// Mutably gets the [`CameraFpsController`](CameraFpsControllerComponent) component of the specified entity.
    #[allow(dead_code, unused)]
    #[inline]
    fn get_camera_fps_controller_component_mut(&mut self, entity_id: ::sillyecs::EntityId) -> Option<&mut CameraFpsControllerComponent> {
        None
    }

    /// Mutably gets the [`Position`](PositionComponent) component of the specified entity.
    #[allow(dead_code, unused)]
    #[inline]
    fn get_position_component_mut(&mut self, entity_id: ::sillyecs::EntityId) -> Option<&mut PositionComponent> {
        None
    }

    /// Mutably gets the [`Transform`](TransformComponent) component of the specified entity.
    #[allow(dead_code, unused)]
    #[inline]
    fn get_transform_component_mut(&mut self, entity_id: ::sillyecs::EntityId) -> Option<&mut TransformComponent> {
        None
    }

    /// Mutably gets the [`Velocity`](VelocityComponent) component of the specified entity.
    #[allow(dead_code, unused)]
    #[inline]
    fn get_velocity_component_mut(&mut self, entity_id: ::sillyecs::EntityId) -> Option<&mut VelocityComponent> {
        None
    }

    /// Mutably gets the [`Tile`](TileComponent) component of the specified entity.
    #[allow(dead_code, unused)]
    #[inline]
    fn get_tile_component_mut(&mut self, entity_id: ::sillyecs::EntityId) -> Option<&mut TileComponent> {
        None
    }

    /// Mutably gets the [`Health`](HealthComponent) component of the specified entity.
    #[allow(dead_code, unused)]
    #[inline]
    fn get_health_component_mut(&mut self, entity_id: ::sillyecs::EntityId) -> Option<&mut HealthComponent> {
        None
    }

    /// Mutably gets the [`Collider`](ColliderComponent) component of the specified entity.
    #[allow(dead_code, unused)]
    #[inline]
    fn get_collider_component_mut(&mut self, entity_id: ::sillyecs::EntityId) -> Option<&mut ColliderComponent> {
        None
    }

    /// Mutably gets the [`Model`](ModelComponent) component of the specified entity.
    #[allow(dead_code, unused)]
    #[inline]
    fn get_model_component_mut(&mut self, entity_id: ::sillyecs::EntityId) -> Option<&mut ModelComponent> {
        None
    }

    /// Mutably gets the [`Color`](ColorComponent) component of the specified entity.
    #[allow(dead_code, unused)]
    #[inline]
    fn get_color_component_mut(&mut self, entity_id: ::sillyecs::EntityId) -> Option<&mut ColorComponent> {
        None
    }

    /// Mutably gets the [`BackgroundColor`](BackgroundColorComponent) component of the specified entity.
    #[allow(dead_code, unused)]
    #[inline]
    fn get_background_color_component_mut(&mut self, entity_id: ::sillyecs::EntityId) -> Option<&mut BackgroundColorComponent> {
        None
    }

    /// Mutably gets the [`DynamicLight`](DynamicLightComponent) component of the specified entity.
    #[allow(dead_code, unused)]
    #[inline]
    fn get_dynamic_light_component_mut(&mut self, entity_id: ::sillyecs::EntityId) -> Option<&mut DynamicLightComponent> {
        None
    }

    /// Mutably gets the [`WgpuShader`](WgpuShaderComponent) component of the specified entity.
    #[allow(dead_code, unused)]
    #[inline]
    fn get_wgpu_shader_component_mut(&mut self, entity_id: ::sillyecs::EntityId) -> Option<&mut WgpuShaderComponent> {
        None
    }

    /// Mutably gets the [`TextureSource`](TextureSourceComponent) component of the specified entity.
    #[allow(dead_code, unused)]
    #[inline]
    fn get_texture_source_component_mut(&mut self, entity_id: ::sillyecs::EntityId) -> Option<&mut TextureSourceComponent> {
        None
    }

    /// Mutably gets the [`WgpuTexture`](WgpuTextureComponent) component of the specified entity.
    #[allow(dead_code, unused)]
    #[inline]
    fn get_wgpu_texture_component_mut(&mut self, entity_id: ::sillyecs::EntityId) -> Option<&mut WgpuTextureComponent> {
        None
    }

    /// Mutably gets the [`UniformType`](UniformTypeComponent) component of the specified entity.
    #[allow(dead_code, unused)]
    #[inline]
    fn get_uniform_type_component_mut(&mut self, entity_id: ::sillyecs::EntityId) -> Option<&mut UniformTypeComponent> {
        None
    }

    /// Mutably gets the [`WgpuBindGroupLayout`](WgpuBindGroupLayoutComponent) component of the specified entity.
    #[allow(dead_code, unused)]
    #[inline]
    fn get_wgpu_bind_group_layout_component_mut(&mut self, entity_id: ::sillyecs::EntityId) -> Option<&mut WgpuBindGroupLayoutComponent> {
        None
    }

    /// Mutably gets the [`WgpuBindGroup`](WgpuBindGroupComponent) component of the specified entity.
    #[allow(dead_code, unused)]
    #[inline]
    fn get_wgpu_bind_group_component_mut(&mut self, entity_id: ::sillyecs::EntityId) -> Option<&mut WgpuBindGroupComponent> {
        None
    }

    /// Mutably gets the [`WgpuBuffer`](WgpuBufferComponent) component of the specified entity.
    #[allow(dead_code, unused)]
    #[inline]
    fn get_wgpu_buffer_component_mut(&mut self, entity_id: ::sillyecs::EntityId) -> Option<&mut WgpuBufferComponent> {
        None
    }
}

pub trait EntityAccess {

    /// Accesses the `WgpuUniformBindGroup` entity with the given ID if it exists in this archetype.
    #[allow(dead_code, unused)]
    #[inline]
    fn get_wgpu_uniform_bind_group_entity(
        &self,
        entity_id: ::sillyecs::EntityId
    ) -> Option<WgpuUniformBindGroupEntityRef>
    {
        None
    }

    /// Accesses the `WgpuShader` entity with the given ID if it exists in this archetype.
    #[allow(dead_code, unused)]
    #[inline]
    fn get_wgpu_shader_entity(
        &self,
        entity_id: ::sillyecs::EntityId
    ) -> Option<WgpuShaderEntityRef>
    {
        None
    }

    /// Accesses the `WgpuTexture` entity with the given ID if it exists in this archetype.
    #[allow(dead_code, unused)]
    #[inline]
    fn get_wgpu_texture_entity(
        &self,
        entity_id: ::sillyecs::EntityId
    ) -> Option<WgpuTextureEntityRef>
    {
        None
    }

    /// Accesses the `Environment` entity with the given ID if it exists in this archetype.
    #[allow(dead_code, unused)]
    #[inline]
    fn get_environment_entity(
        &self,
        entity_id: ::sillyecs::EntityId
    ) -> Option<EnvironmentEntityRef>
    {
        None
    }

    /// Accesses the `Camera` entity with the given ID if it exists in this archetype.
    #[allow(dead_code, unused)]
    #[inline]
    fn get_camera_entity(
        &self,
        entity_id: ::sillyecs::EntityId
    ) -> Option<CameraEntityRef>
    {
        None
    }

    /// Accesses the `DynamicLight` entity with the given ID if it exists in this archetype.
    #[allow(dead_code, unused)]
    #[inline]
    fn get_dynamic_light_entity(
        &self,
        entity_id: ::sillyecs::EntityId
    ) -> Option<DynamicLightEntityRef>
    {
        None
    }

    /// Accesses the `Tile` entity with the given ID if it exists in this archetype.
    #[allow(dead_code, unused)]
    #[inline]
    fn get_tile_entity(
        &self,
        entity_id: ::sillyecs::EntityId
    ) -> Option<TileEntityRef>
    {
        None
    }

    /// Accesses the `Cube` entity with the given ID if it exists in this archetype.
    #[allow(dead_code, unused)]
    #[inline]
    fn get_cube_entity(
        &self,
        entity_id: ::sillyecs::EntityId
    ) -> Option<CubeEntityRef>
    {
        None
    }

    /// Accesses the `Player` entity with the given ID if it exists in this archetype.
    #[allow(dead_code, unused)]
    #[inline]
    fn get_player_entity(
        &self,
        entity_id: ::sillyecs::EntityId
    ) -> Option<PlayerEntityRef>
    {
        None
    }

    /// Accesses the `ForegroundObject` entity with the given ID if it exists in this archetype.
    #[allow(dead_code, unused)]
    #[inline]
    fn get_foreground_object_entity(
        &self,
        entity_id: ::sillyecs::EntityId
    ) -> Option<ForegroundObjectEntityRef>
    {
        None
    }

    /// Accesses the `BackgroundObject` entity with the given ID if it exists in this archetype.
    #[allow(dead_code, unused)]
    #[inline]
    fn get_background_object_entity(
        &self,
        entity_id: ::sillyecs::EntityId
    ) -> Option<BackgroundObjectEntityRef>
    {
        None
    }

    /// Accesses the `SceneBackground` entity with the given ID if it exists in this archetype.
    #[allow(dead_code, unused)]
    #[inline]
    fn get_scene_background_entity(
        &self,
        entity_id: ::sillyecs::EntityId
    ) -> Option<SceneBackgroundEntityRef>
    {
        None
    }
}

pub trait EntityAccessMut: EntityAccess {

    /// Mutably accesses the `WgpuUniformBindGroup` entity with the given ID if it exists in this archetype.
    #[allow(dead_code, unused)]
    #[inline]
    fn get_wgpu_uniform_bind_group_entity_mut(
        &mut self,
        entity_id: ::sillyecs::EntityId
    ) -> Option<WgpuUniformBindGroupEntityMut>
    {
        None
    }

    /// Mutably accesses the `WgpuShader` entity with the given ID if it exists in this archetype.
    #[allow(dead_code, unused)]
    #[inline]
    fn get_wgpu_shader_entity_mut(
        &mut self,
        entity_id: ::sillyecs::EntityId
    ) -> Option<WgpuShaderEntityMut>
    {
        None
    }

    /// Mutably accesses the `WgpuTexture` entity with the given ID if it exists in this archetype.
    #[allow(dead_code, unused)]
    #[inline]
    fn get_wgpu_texture_entity_mut(
        &mut self,
        entity_id: ::sillyecs::EntityId
    ) -> Option<WgpuTextureEntityMut>
    {
        None
    }

    /// Mutably accesses the `Environment` entity with the given ID if it exists in this archetype.
    #[allow(dead_code, unused)]
    #[inline]
    fn get_environment_entity_mut(
        &mut self,
        entity_id: ::sillyecs::EntityId
    ) -> Option<EnvironmentEntityMut>
    {
        None
    }

    /// Mutably accesses the `Camera` entity with the given ID if it exists in this archetype.
    #[allow(dead_code, unused)]
    #[inline]
    fn get_camera_entity_mut(
        &mut self,
        entity_id: ::sillyecs::EntityId
    ) -> Option<CameraEntityMut>
    {
        None
    }

    /// Mutably accesses the `DynamicLight` entity with the given ID if it exists in this archetype.
    #[allow(dead_code, unused)]
    #[inline]
    fn get_dynamic_light_entity_mut(
        &mut self,
        entity_id: ::sillyecs::EntityId
    ) -> Option<DynamicLightEntityMut>
    {
        None
    }

    /// Mutably accesses the `Tile` entity with the given ID if it exists in this archetype.
    #[allow(dead_code, unused)]
    #[inline]
    fn get_tile_entity_mut(
        &mut self,
        entity_id: ::sillyecs::EntityId
    ) -> Option<TileEntityMut>
    {
        None
    }

    /// Mutably accesses the `Cube` entity with the given ID if it exists in this archetype.
    #[allow(dead_code, unused)]
    #[inline]
    fn get_cube_entity_mut(
        &mut self,
        entity_id: ::sillyecs::EntityId
    ) -> Option<CubeEntityMut>
    {
        None
    }

    /// Mutably accesses the `Player` entity with the given ID if it exists in this archetype.
    #[allow(dead_code, unused)]
    #[inline]
    fn get_player_entity_mut(
        &mut self,
        entity_id: ::sillyecs::EntityId
    ) -> Option<PlayerEntityMut>
    {
        None
    }

    /// Mutably accesses the `ForegroundObject` entity with the given ID if it exists in this archetype.
    #[allow(dead_code, unused)]
    #[inline]
    fn get_foreground_object_entity_mut(
        &mut self,
        entity_id: ::sillyecs::EntityId
    ) -> Option<ForegroundObjectEntityMut>
    {
        None
    }

    /// Mutably accesses the `BackgroundObject` entity with the given ID if it exists in this archetype.
    #[allow(dead_code, unused)]
    #[inline]
    fn get_background_object_entity_mut(
        &mut self,
        entity_id: ::sillyecs::EntityId
    ) -> Option<BackgroundObjectEntityMut>
    {
        None
    }

    /// Mutably accesses the `SceneBackground` entity with the given ID if it exists in this archetype.
    #[allow(dead_code, unused)]
    #[inline]
    fn get_scene_background_entity_mut(
        &mut self,
        entity_id: ::sillyecs::EntityId
    ) -> Option<SceneBackgroundEntityMut>
    {
        None
    }
}

//noinspection RsSortImplTraitMembers
impl<E, Q> EntityAccess for MainWorld<E, Q> {

    /// Accesses the `WgpuUniformBindGroup` entity with the given ID if it exists in this archetype.
    #[allow(dead_code, unused)]
    #[inline]
    fn get_wgpu_uniform_bind_group_entity(
        &self,
        entity_id: ::sillyecs::EntityId
    ) -> Option<WgpuUniformBindGroupEntityRef>
    {
        EntityAccess::get_wgpu_uniform_bind_group_entity(&self.archetypes, entity_id)
    }

    /// Accesses the `WgpuShader` entity with the given ID if it exists in this archetype.
    #[allow(dead_code, unused)]
    #[inline]
    fn get_wgpu_shader_entity(
        &self,
        entity_id: ::sillyecs::EntityId
    ) -> Option<WgpuShaderEntityRef>
    {
        EntityAccess::get_wgpu_shader_entity(&self.archetypes, entity_id)
    }

    /// Accesses the `WgpuTexture` entity with the given ID if it exists in this archetype.
    #[allow(dead_code, unused)]
    #[inline]
    fn get_wgpu_texture_entity(
        &self,
        entity_id: ::sillyecs::EntityId
    ) -> Option<WgpuTextureEntityRef>
    {
        EntityAccess::get_wgpu_texture_entity(&self.archetypes, entity_id)
    }

    /// Accesses the `Environment` entity with the given ID if it exists in this archetype.
    #[allow(dead_code, unused)]
    #[inline]
    fn get_environment_entity(
        &self,
        entity_id: ::sillyecs::EntityId
    ) -> Option<EnvironmentEntityRef>
    {
        EntityAccess::get_environment_entity(&self.archetypes, entity_id)
    }

    /// Accesses the `Camera` entity with the given ID if it exists in this archetype.
    #[allow(dead_code, unused)]
    #[inline]
    fn get_camera_entity(
        &self,
        entity_id: ::sillyecs::EntityId
    ) -> Option<CameraEntityRef>
    {
        EntityAccess::get_camera_entity(&self.archetypes, entity_id)
    }

    /// Accesses the `DynamicLight` entity with the given ID if it exists in this archetype.
    #[allow(dead_code, unused)]
    #[inline]
    fn get_dynamic_light_entity(
        &self,
        entity_id: ::sillyecs::EntityId
    ) -> Option<DynamicLightEntityRef>
    {
        EntityAccess::get_dynamic_light_entity(&self.archetypes, entity_id)
    }

    /// Accesses the `Tile` entity with the given ID if it exists in this archetype.
    #[allow(dead_code, unused)]
    #[inline]
    fn get_tile_entity(
        &self,
        entity_id: ::sillyecs::EntityId
    ) -> Option<TileEntityRef>
    {
        EntityAccess::get_tile_entity(&self.archetypes, entity_id)
    }

    /// Accesses the `Cube` entity with the given ID if it exists in this archetype.
    #[allow(dead_code, unused)]
    #[inline]
    fn get_cube_entity(
        &self,
        entity_id: ::sillyecs::EntityId
    ) -> Option<CubeEntityRef>
    {
        EntityAccess::get_cube_entity(&self.archetypes, entity_id)
    }

    /// Accesses the `Player` entity with the given ID if it exists in this archetype.
    #[allow(dead_code, unused)]
    #[inline]
    fn get_player_entity(
        &self,
        entity_id: ::sillyecs::EntityId
    ) -> Option<PlayerEntityRef>
    {
        EntityAccess::get_player_entity(&self.archetypes, entity_id)
    }

    /// Accesses the `ForegroundObject` entity with the given ID if it exists in this archetype.
    #[allow(dead_code, unused)]
    #[inline]
    fn get_foreground_object_entity(
        &self,
        entity_id: ::sillyecs::EntityId
    ) -> Option<ForegroundObjectEntityRef>
    {
        EntityAccess::get_foreground_object_entity(&self.archetypes, entity_id)
    }

    /// Accesses the `BackgroundObject` entity with the given ID if it exists in this archetype.
    #[allow(dead_code, unused)]
    #[inline]
    fn get_background_object_entity(
        &self,
        entity_id: ::sillyecs::EntityId
    ) -> Option<BackgroundObjectEntityRef>
    {
        EntityAccess::get_background_object_entity(&self.archetypes, entity_id)
    }

    /// Accesses the `SceneBackground` entity with the given ID if it exists in this archetype.
    #[allow(dead_code, unused)]
    #[inline]
    fn get_scene_background_entity(
        &self,
        entity_id: ::sillyecs::EntityId
    ) -> Option<SceneBackgroundEntityRef>
    {
        EntityAccess::get_scene_background_entity(&self.archetypes, entity_id)
    }
}

//noinspection RsSortImplTraitMembers
impl<E, Q> EntityAccessMut for MainWorld<E, Q> {

    /// Mutably accesses the `WgpuUniformBindGroup` entity with the given ID if it exists in this archetype.
    #[allow(dead_code, unused)]
    #[inline]
    fn get_wgpu_uniform_bind_group_entity_mut(
        &mut self,
        entity_id: ::sillyecs::EntityId
    ) -> Option<WgpuUniformBindGroupEntityMut>
    {
        EntityAccessMut::get_wgpu_uniform_bind_group_entity_mut(&mut self.archetypes, entity_id)
    }

    /// Mutably accesses the `WgpuShader` entity with the given ID if it exists in this archetype.
    #[allow(dead_code, unused)]
    #[inline]
    fn get_wgpu_shader_entity_mut(
        &mut self,
        entity_id: ::sillyecs::EntityId
    ) -> Option<WgpuShaderEntityMut>
    {
        EntityAccessMut::get_wgpu_shader_entity_mut(&mut self.archetypes, entity_id)
    }

    /// Mutably accesses the `WgpuTexture` entity with the given ID if it exists in this archetype.
    #[allow(dead_code, unused)]
    #[inline]
    fn get_wgpu_texture_entity_mut(
        &mut self,
        entity_id: ::sillyecs::EntityId
    ) -> Option<WgpuTextureEntityMut>
    {
        EntityAccessMut::get_wgpu_texture_entity_mut(&mut self.archetypes, entity_id)
    }

    /// Mutably accesses the `Environment` entity with the given ID if it exists in this archetype.
    #[allow(dead_code, unused)]
    #[inline]
    fn get_environment_entity_mut(
        &mut self,
        entity_id: ::sillyecs::EntityId
    ) -> Option<EnvironmentEntityMut>
    {
        EntityAccessMut::get_environment_entity_mut(&mut self.archetypes, entity_id)
    }

    /// Mutably accesses the `Camera` entity with the given ID if it exists in this archetype.
    #[allow(dead_code, unused)]
    #[inline]
    fn get_camera_entity_mut(
        &mut self,
        entity_id: ::sillyecs::EntityId
    ) -> Option<CameraEntityMut>
    {
        EntityAccessMut::get_camera_entity_mut(&mut self.archetypes, entity_id)
    }

    /// Mutably accesses the `DynamicLight` entity with the given ID if it exists in this archetype.
    #[allow(dead_code, unused)]
    #[inline]
    fn get_dynamic_light_entity_mut(
        &mut self,
        entity_id: ::sillyecs::EntityId
    ) -> Option<DynamicLightEntityMut>
    {
        EntityAccessMut::get_dynamic_light_entity_mut(&mut self.archetypes, entity_id)
    }

    /// Mutably accesses the `Tile` entity with the given ID if it exists in this archetype.
    #[allow(dead_code, unused)]
    #[inline]
    fn get_tile_entity_mut(
        &mut self,
        entity_id: ::sillyecs::EntityId
    ) -> Option<TileEntityMut>
    {
        EntityAccessMut::get_tile_entity_mut(&mut self.archetypes, entity_id)
    }

    /// Mutably accesses the `Cube` entity with the given ID if it exists in this archetype.
    #[allow(dead_code, unused)]
    #[inline]
    fn get_cube_entity_mut(
        &mut self,
        entity_id: ::sillyecs::EntityId
    ) -> Option<CubeEntityMut>
    {
        EntityAccessMut::get_cube_entity_mut(&mut self.archetypes, entity_id)
    }

    /// Mutably accesses the `Player` entity with the given ID if it exists in this archetype.
    #[allow(dead_code, unused)]
    #[inline]
    fn get_player_entity_mut(
        &mut self,
        entity_id: ::sillyecs::EntityId
    ) -> Option<PlayerEntityMut>
    {
        EntityAccessMut::get_player_entity_mut(&mut self.archetypes, entity_id)
    }

    /// Mutably accesses the `ForegroundObject` entity with the given ID if it exists in this archetype.
    #[allow(dead_code, unused)]
    #[inline]
    fn get_foreground_object_entity_mut(
        &mut self,
        entity_id: ::sillyecs::EntityId
    ) -> Option<ForegroundObjectEntityMut>
    {
        EntityAccessMut::get_foreground_object_entity_mut(&mut self.archetypes, entity_id)
    }

    /// Mutably accesses the `BackgroundObject` entity with the given ID if it exists in this archetype.
    #[allow(dead_code, unused)]
    #[inline]
    fn get_background_object_entity_mut(
        &mut self,
        entity_id: ::sillyecs::EntityId
    ) -> Option<BackgroundObjectEntityMut>
    {
        EntityAccessMut::get_background_object_entity_mut(&mut self.archetypes, entity_id)
    }

    /// Mutably accesses the `SceneBackground` entity with the given ID if it exists in this archetype.
    #[allow(dead_code, unused)]
    #[inline]
    fn get_scene_background_entity_mut(
        &mut self,
        entity_id: ::sillyecs::EntityId
    ) -> Option<SceneBackgroundEntityMut>
    {
        EntityAccessMut::get_scene_background_entity_mut(&mut self.archetypes, entity_id)
    }
}

//noinspection RsSortImplTraitMembers
impl<E, Q> ComponentAccess for MainWorld<E, Q> {

    /// Gets the [`AmbientLight`](AmbientLightComponent) component of the specified entity.
    #[allow(dead_code, unused)]
    #[inline]
    fn get_ambient_light_component(&self, entity_id: ::sillyecs::EntityId) -> Option<&AmbientLightComponent> {
        ComponentAccess::get_ambient_light_component(&self.archetypes, entity_id)
    }

    /// Gets the [`BackgroundColor`](BackgroundColorComponent) component of the specified entity.
    #[allow(dead_code, unused)]
    #[inline]
    fn get_background_color_component(&self, entity_id: ::sillyecs::EntityId) -> Option<&BackgroundColorComponent> {
        ComponentAccess::get_background_color_component(&self.archetypes, entity_id)
    }

    /// Gets the [`Camera`](CameraComponent) component of the specified entity.
    #[allow(dead_code, unused)]
    #[inline]
    fn get_camera_component(&self, entity_id: ::sillyecs::EntityId) -> Option<&CameraComponent> {
        ComponentAccess::get_camera_component(&self.archetypes, entity_id)
    }

    /// Gets the [`CameraFpsController`](CameraFpsControllerComponent) component of the specified entity.
    #[allow(dead_code, unused)]
    #[inline]
    fn get_camera_fps_controller_component(&self, entity_id: ::sillyecs::EntityId) -> Option<&CameraFpsControllerComponent> {
        ComponentAccess::get_camera_fps_controller_component(&self.archetypes, entity_id)
    }

    /// Gets the [`Collider`](ColliderComponent) component of the specified entity.
    #[allow(dead_code, unused)]
    #[inline]
    fn get_collider_component(&self, entity_id: ::sillyecs::EntityId) -> Option<&ColliderComponent> {
        ComponentAccess::get_collider_component(&self.archetypes, entity_id)
    }

    /// Gets the [`Color`](ColorComponent) component of the specified entity.
    #[allow(dead_code, unused)]
    #[inline]
    fn get_color_component(&self, entity_id: ::sillyecs::EntityId) -> Option<&ColorComponent> {
        ComponentAccess::get_color_component(&self.archetypes, entity_id)
    }

    /// Gets the [`DynamicLight`](DynamicLightComponent) component of the specified entity.
    #[allow(dead_code, unused)]
    #[inline]
    fn get_dynamic_light_component(&self, entity_id: ::sillyecs::EntityId) -> Option<&DynamicLightComponent> {
        ComponentAccess::get_dynamic_light_component(&self.archetypes, entity_id)
    }

    /// Gets the [`Fog`](FogComponent) component of the specified entity.
    #[allow(dead_code, unused)]
    #[inline]
    fn get_fog_component(&self, entity_id: ::sillyecs::EntityId) -> Option<&FogComponent> {
        ComponentAccess::get_fog_component(&self.archetypes, entity_id)
    }

    /// Gets the [`Health`](HealthComponent) component of the specified entity.
    #[allow(dead_code, unused)]
    #[inline]
    fn get_health_component(&self, entity_id: ::sillyecs::EntityId) -> Option<&HealthComponent> {
        ComponentAccess::get_health_component(&self.archetypes, entity_id)
    }

    /// Gets the [`Model`](ModelComponent) component of the specified entity.
    #[allow(dead_code, unused)]
    #[inline]
    fn get_model_component(&self, entity_id: ::sillyecs::EntityId) -> Option<&ModelComponent> {
        ComponentAccess::get_model_component(&self.archetypes, entity_id)
    }

    /// Gets the [`Position`](PositionComponent) component of the specified entity.
    #[allow(dead_code, unused)]
    #[inline]
    fn get_position_component(&self, entity_id: ::sillyecs::EntityId) -> Option<&PositionComponent> {
        ComponentAccess::get_position_component(&self.archetypes, entity_id)
    }

    /// Gets the [`Skydome`](SkydomeComponent) component of the specified entity.
    #[allow(dead_code, unused)]
    #[inline]
    fn get_skydome_component(&self, entity_id: ::sillyecs::EntityId) -> Option<&SkydomeComponent> {
        ComponentAccess::get_skydome_component(&self.archetypes, entity_id)
    }

    /// Gets the [`Stars`](StarsComponent) component of the specified entity.
    #[allow(dead_code, unused)]
    #[inline]
    fn get_stars_component(&self, entity_id: ::sillyecs::EntityId) -> Option<&StarsComponent> {
        ComponentAccess::get_stars_component(&self.archetypes, entity_id)
    }

    /// Gets the [`Sun`](SunComponent) component of the specified entity.
    #[allow(dead_code, unused)]
    #[inline]
    fn get_sun_component(&self, entity_id: ::sillyecs::EntityId) -> Option<&SunComponent> {
        ComponentAccess::get_sun_component(&self.archetypes, entity_id)
    }

    /// Gets the [`TextureSource`](TextureSourceComponent) component of the specified entity.
    #[allow(dead_code, unused)]
    #[inline]
    fn get_texture_source_component(&self, entity_id: ::sillyecs::EntityId) -> Option<&TextureSourceComponent> {
        ComponentAccess::get_texture_source_component(&self.archetypes, entity_id)
    }

    /// Gets the [`Tile`](TileComponent) component of the specified entity.
    #[allow(dead_code, unused)]
    #[inline]
    fn get_tile_component(&self, entity_id: ::sillyecs::EntityId) -> Option<&TileComponent> {
        ComponentAccess::get_tile_component(&self.archetypes, entity_id)
    }

    /// Gets the [`TimeOfDay`](TimeOfDayComponent) component of the specified entity.
    #[allow(dead_code, unused)]
    #[inline]
    fn get_time_of_day_component(&self, entity_id: ::sillyecs::EntityId) -> Option<&TimeOfDayComponent> {
        ComponentAccess::get_time_of_day_component(&self.archetypes, entity_id)
    }

    /// Gets the [`Transform`](TransformComponent) component of the specified entity.
    #[allow(dead_code, unused)]
    #[inline]
    fn get_transform_component(&self, entity_id: ::sillyecs::EntityId) -> Option<&TransformComponent> {
        ComponentAccess::get_transform_component(&self.archetypes, entity_id)
    }

    /// Gets the [`UniformType`](UniformTypeComponent) component of the specified entity.
    #[allow(dead_code, unused)]
    #[inline]
    fn get_uniform_type_component(&self, entity_id: ::sillyecs::EntityId) -> Option<&UniformTypeComponent> {
        ComponentAccess::get_uniform_type_component(&self.archetypes, entity_id)
    }

    /// Gets the [`Velocity`](VelocityComponent) component of the specified entity.
    #[allow(dead_code, unused)]
    #[inline]
    fn get_velocity_component(&self, entity_id: ::sillyecs::EntityId) -> Option<&VelocityComponent> {
        ComponentAccess::get_velocity_component(&self.archetypes, entity_id)
    }

    /// Gets the [`WgpuBindGroup`](WgpuBindGroupComponent) component of the specified entity.
    #[allow(dead_code, unused)]
    #[inline]
    fn get_wgpu_bind_group_component(&self, entity_id: ::sillyecs::EntityId) -> Option<&WgpuBindGroupComponent> {
        ComponentAccess::get_wgpu_bind_group_component(&self.archetypes, entity_id)
    }

    /// Gets the [`WgpuBindGroupLayout`](WgpuBindGroupLayoutComponent) component of the specified entity.
    #[allow(dead_code, unused)]
    #[inline]
    fn get_wgpu_bind_group_layout_component(&self, entity_id: ::sillyecs::EntityId) -> Option<&WgpuBindGroupLayoutComponent> {
        ComponentAccess::get_wgpu_bind_group_layout_component(&self.archetypes, entity_id)
    }

    /// Gets the [`WgpuBuffer`](WgpuBufferComponent) component of the specified entity.
    #[allow(dead_code, unused)]
    #[inline]
    fn get_wgpu_buffer_component(&self, entity_id: ::sillyecs::EntityId) -> Option<&WgpuBufferComponent> {
        ComponentAccess::get_wgpu_buffer_component(&self.archetypes, entity_id)
    }

    /// Gets the [`WgpuShader`](WgpuShaderComponent) component of the specified entity.
    #[allow(dead_code, unused)]
    #[inline]
    fn get_wgpu_shader_component(&self, entity_id: ::sillyecs::EntityId) -> Option<&WgpuShaderComponent> {
        ComponentAccess::get_wgpu_shader_component(&self.archetypes, entity_id)
    }

    /// Gets the [`WgpuTexture`](WgpuTextureComponent) component of the specified entity.
    #[allow(dead_code, unused)]
    #[inline]
    fn get_wgpu_texture_component(&self, entity_id: ::sillyecs::EntityId) -> Option<&WgpuTextureComponent> {
        ComponentAccess::get_wgpu_texture_component(&self.archetypes, entity_id)
    }
}

//noinspection RsSortImplTraitMembers
impl<E, Q> ComponentAccessMut for MainWorld<E, Q> {

    /// Mutably gets the [`AmbientLight`](AmbientLightComponent) component of the specified entity.
    #[allow(dead_code, unused)]
    #[inline]
    fn get_ambient_light_component_mut(&mut self, entity_id: ::sillyecs::EntityId) -> Option<&mut AmbientLightComponent> {
        ComponentAccessMut::get_ambient_light_component_mut(&mut self.archetypes, entity_id)
    }

    /// Mutably gets the [`BackgroundColor`](BackgroundColorComponent) component of the specified entity.
    #[allow(dead_code, unused)]
    #[inline]
    fn get_background_color_component_mut(&mut self, entity_id: ::sillyecs::EntityId) -> Option<&mut BackgroundColorComponent> {
        ComponentAccessMut::get_background_color_component_mut(&mut self.archetypes, entity_id)
    }

    /// Mutably gets the [`Camera`](CameraComponent) component of the specified entity.
    #[allow(dead_code, unused)]
    #[inline]
    fn get_camera_component_mut(&mut self, entity_id: ::sillyecs::EntityId) -> Option<&mut CameraComponent> {
        ComponentAccessMut::get_camera_component_mut(&mut self.archetypes, entity_id)
    }

    /// Mutably gets the [`CameraFpsController`](CameraFpsControllerComponent) component of the specified entity.
    #[allow(dead_code, unused)]
    #[inline]
    fn get_camera_fps_controller_component_mut(&mut self, entity_id: ::sillyecs::EntityId) -> Option<&mut CameraFpsControllerComponent> {
        ComponentAccessMut::get_camera_fps_controller_component_mut(&mut self.archetypes, entity_id)
    }

    /// Mutably gets the [`Collider`](ColliderComponent) component of the specified entity.
    #[allow(dead_code, unused)]
    #[inline]
    fn get_collider_component_mut(&mut self, entity_id: ::sillyecs::EntityId) -> Option<&mut ColliderComponent> {
        ComponentAccessMut::get_collider_component_mut(&mut self.archetypes, entity_id)
    }

    /// Mutably gets the [`Color`](ColorComponent) component of the specified entity.
    #[allow(dead_code, unused)]
    #[inline]
    fn get_color_component_mut(&mut self, entity_id: ::sillyecs::EntityId) -> Option<&mut ColorComponent> {
        ComponentAccessMut::get_color_component_mut(&mut self.archetypes, entity_id)
    }

    /// Mutably gets the [`DynamicLight`](DynamicLightComponent) component of the specified entity.
    #[allow(dead_code, unused)]
    #[inline]
    fn get_dynamic_light_component_mut(&mut self, entity_id: ::sillyecs::EntityId) -> Option<&mut DynamicLightComponent> {
        ComponentAccessMut::get_dynamic_light_component_mut(&mut self.archetypes, entity_id)
    }

    /// Mutably gets the [`Fog`](FogComponent) component of the specified entity.
    #[allow(dead_code, unused)]
    #[inline]
    fn get_fog_component_mut(&mut self, entity_id: ::sillyecs::EntityId) -> Option<&mut FogComponent> {
        ComponentAccessMut::get_fog_component_mut(&mut self.archetypes, entity_id)
    }

    /// Mutably gets the [`Health`](HealthComponent) component of the specified entity.
    #[allow(dead_code, unused)]
    #[inline]
    fn get_health_component_mut(&mut self, entity_id: ::sillyecs::EntityId) -> Option<&mut HealthComponent> {
        ComponentAccessMut::get_health_component_mut(&mut self.archetypes, entity_id)
    }

    /// Mutably gets the [`Model`](ModelComponent) component of the specified entity.
    #[allow(dead_code, unused)]
    #[inline]
    fn get_model_component_mut(&mut self, entity_id: ::sillyecs::EntityId) -> Option<&mut ModelComponent> {
        ComponentAccessMut::get_model_component_mut(&mut self.archetypes, entity_id)
    }

    /// Mutably gets the [`Position`](PositionComponent) component of the specified entity.
    #[allow(dead_code, unused)]
    #[inline]
    fn get_position_component_mut(&mut self, entity_id: ::sillyecs::EntityId) -> Option<&mut PositionComponent> {
        ComponentAccessMut::get_position_component_mut(&mut self.archetypes, entity_id)
    }

    /// Mutably gets the [`Skydome`](SkydomeComponent) component of the specified entity.
    #[allow(dead_code, unused)]
    #[inline]
    fn get_skydome_component_mut(&mut self, entity_id: ::sillyecs::EntityId) -> Option<&mut SkydomeComponent> {
        ComponentAccessMut::get_skydome_component_mut(&mut self.archetypes, entity_id)
    }

    /// Mutably gets the [`Stars`](StarsComponent) component of the specified entity.
    #[allow(dead_code, unused)]
    #[inline]
    fn get_stars_component_mut(&mut self, entity_id: ::sillyecs::EntityId) -> Option<&mut StarsComponent> {
        ComponentAccessMut::get_stars_component_mut(&mut self.archetypes, entity_id)
    }

    /// Mutably gets the [`Sun`](SunComponent) component of the specified entity.
    #[allow(dead_code, unused)]
    #[inline]
    fn get_sun_component_mut(&mut self, entity_id: ::sillyecs::EntityId) -> Option<&mut SunComponent> {
        ComponentAccessMut::get_sun_component_mut(&mut self.archetypes, entity_id)
    }

    /// Mutably gets the [`TextureSource`](TextureSourceComponent) component of the specified entity.
    #[allow(dead_code, unused)]
    #[inline]
    fn get_texture_source_component_mut(&mut self, entity_id: ::sillyecs::EntityId) -> Option<&mut TextureSourceComponent> {
        ComponentAccessMut::get_texture_source_component_mut(&mut self.archetypes, entity_id)
    }

    /// Mutably gets the [`Tile`](TileComponent) component of the specified entity.
    #[allow(dead_code, unused)]
    #[inline]
    fn get_tile_component_mut(&mut self, entity_id: ::sillyecs::EntityId) -> Option<&mut TileComponent> {
        ComponentAccessMut::get_tile_component_mut(&mut self.archetypes, entity_id)
    }

    /// Mutably gets the [`TimeOfDay`](TimeOfDayComponent) component of the specified entity.
    #[allow(dead_code, unused)]
    #[inline]
    fn get_time_of_day_component_mut(&mut self, entity_id: ::sillyecs::EntityId) -> Option<&mut TimeOfDayComponent> {
        ComponentAccessMut::get_time_of_day_component_mut(&mut self.archetypes, entity_id)
    }

    /// Mutably gets the [`Transform`](TransformComponent) component of the specified entity.
    #[allow(dead_code, unused)]
    #[inline]
    fn get_transform_component_mut(&mut self, entity_id: ::sillyecs::EntityId) -> Option<&mut TransformComponent> {
        ComponentAccessMut::get_transform_component_mut(&mut self.archetypes, entity_id)
    }

    /// Mutably gets the [`UniformType`](UniformTypeComponent) component of the specified entity.
    #[allow(dead_code, unused)]
    #[inline]
    fn get_uniform_type_component_mut(&mut self, entity_id: ::sillyecs::EntityId) -> Option<&mut UniformTypeComponent> {
        ComponentAccessMut::get_uniform_type_component_mut(&mut self.archetypes, entity_id)
    }

    /// Mutably gets the [`Velocity`](VelocityComponent) component of the specified entity.
    #[allow(dead_code, unused)]
    #[inline]
    fn get_velocity_component_mut(&mut self, entity_id: ::sillyecs::EntityId) -> Option<&mut VelocityComponent> {
        ComponentAccessMut::get_velocity_component_mut(&mut self.archetypes, entity_id)
    }

    /// Mutably gets the [`WgpuBindGroup`](WgpuBindGroupComponent) component of the specified entity.
    #[allow(dead_code, unused)]
    #[inline]
    fn get_wgpu_bind_group_component_mut(&mut self, entity_id: ::sillyecs::EntityId) -> Option<&mut WgpuBindGroupComponent> {
        ComponentAccessMut::get_wgpu_bind_group_component_mut(&mut self.archetypes, entity_id)
    }

    /// Mutably gets the [`WgpuBindGroupLayout`](WgpuBindGroupLayoutComponent) component of the specified entity.
    #[allow(dead_code, unused)]
    #[inline]
    fn get_wgpu_bind_group_layout_component_mut(&mut self, entity_id: ::sillyecs::EntityId) -> Option<&mut WgpuBindGroupLayoutComponent> {
        ComponentAccessMut::get_wgpu_bind_group_layout_component_mut(&mut self.archetypes, entity_id)
    }

    /// Mutably gets the [`WgpuBuffer`](WgpuBufferComponent) component of the specified entity.
    #[allow(dead_code, unused)]
    #[inline]
    fn get_wgpu_buffer_component_mut(&mut self, entity_id: ::sillyecs::EntityId) -> Option<&mut WgpuBufferComponent> {
        ComponentAccessMut::get_wgpu_buffer_component_mut(&mut self.archetypes, entity_id)
    }

    /// Mutably gets the [`WgpuShader`](WgpuShaderComponent) component of the specified entity.
    #[allow(dead_code, unused)]
    #[inline]
    fn get_wgpu_shader_component_mut(&mut self, entity_id: ::sillyecs::EntityId) -> Option<&mut WgpuShaderComponent> {
        ComponentAccessMut::get_wgpu_shader_component_mut(&mut self.archetypes, entity_id)
    }

    /// Mutably gets the [`WgpuTexture`](WgpuTextureComponent) component of the specified entity.
    #[allow(dead_code, unused)]
    #[inline]
    fn get_wgpu_texture_component_mut(&mut self, entity_id: ::sillyecs::EntityId) -> Option<&mut WgpuTextureComponent> {
        ComponentAccessMut::get_wgpu_texture_component_mut(&mut self.archetypes, entity_id)
    }
}

//noinspection RsSortImplTraitMembers
impl EntityAccess for MainWorldArchetypes {

    /// Accesses the `WgpuUniformBindGroup` entity with the given ID if it exists in this archetype.
    #[allow(dead_code, unused)]
    fn get_wgpu_uniform_bind_group_entity(
        &self,
        entity_id: ::sillyecs::EntityId
    ) -> Option<WgpuUniformBindGroupEntityRef>
    {
        let ear = self.entity_locations.get(&entity_id)?.clone();
        if ear.archetype != WgpuUniformBindGroupArchetype::ID {
            return None;
        }
        self.collection
            .wgpu_uniform_bind_group
            .get_entity_at(ear.index)
    }

    /// Accesses the `WgpuShader` entity with the given ID if it exists in this archetype.
    #[allow(dead_code, unused)]
    fn get_wgpu_shader_entity(
        &self,
        entity_id: ::sillyecs::EntityId
    ) -> Option<WgpuShaderEntityRef>
    {
        let ear = self.entity_locations.get(&entity_id)?.clone();
        if ear.archetype != WgpuShaderArchetype::ID {
            return None;
        }
        self.collection
            .wgpu_shader
            .get_entity_at(ear.index)
    }

    /// Accesses the `WgpuTexture` entity with the given ID if it exists in this archetype.
    #[allow(dead_code, unused)]
    fn get_wgpu_texture_entity(
        &self,
        entity_id: ::sillyecs::EntityId
    ) -> Option<WgpuTextureEntityRef>
    {
        let ear = self.entity_locations.get(&entity_id)?.clone();
        if ear.archetype != WgpuTextureArchetype::ID {
            return None;
        }
        self.collection
            .wgpu_texture
            .get_entity_at(ear.index)
    }

    /// Accesses the `Environment` entity with the given ID if it exists in this archetype.
    #[allow(dead_code, unused)]
    fn get_environment_entity(
        &self,
        entity_id: ::sillyecs::EntityId
    ) -> Option<EnvironmentEntityRef>
    {
        let ear = self.entity_locations.get(&entity_id)?.clone();
        if ear.archetype != EnvironmentArchetype::ID {
            return None;
        }
        self.collection
            .environment
            .get_entity_at(ear.index)
    }

    /// Accesses the `Camera` entity with the given ID if it exists in this archetype.
    #[allow(dead_code, unused)]
    fn get_camera_entity(
        &self,
        entity_id: ::sillyecs::EntityId
    ) -> Option<CameraEntityRef>
    {
        let ear = self.entity_locations.get(&entity_id)?.clone();
        if ear.archetype != CameraArchetype::ID {
            return None;
        }
        self.collection
            .camera
            .get_entity_at(ear.index)
    }

    /// Accesses the `DynamicLight` entity with the given ID if it exists in this archetype.
    #[allow(dead_code, unused)]
    fn get_dynamic_light_entity(
        &self,
        entity_id: ::sillyecs::EntityId
    ) -> Option<DynamicLightEntityRef>
    {
        let ear = self.entity_locations.get(&entity_id)?.clone();
        if ear.archetype != DynamicLightArchetype::ID {
            return None;
        }
        self.collection
            .dynamic_light
            .get_entity_at(ear.index)
    }

    /// Accesses the `Tile` entity with the given ID if it exists in this archetype.
    #[allow(dead_code, unused)]
    fn get_tile_entity(
        &self,
        entity_id: ::sillyecs::EntityId
    ) -> Option<TileEntityRef>
    {
        let ear = self.entity_locations.get(&entity_id)?.clone();
        if ear.archetype != TileArchetype::ID {
            return None;
        }
        self.collection
            .tile
            .get_entity_at(ear.index)
    }

    /// Accesses the `Cube` entity with the given ID if it exists in this archetype.
    #[allow(dead_code, unused)]
    fn get_cube_entity(
        &self,
        entity_id: ::sillyecs::EntityId
    ) -> Option<CubeEntityRef>
    {
        let ear = self.entity_locations.get(&entity_id)?.clone();
        if ear.archetype != CubeArchetype::ID {
            return None;
        }
        self.collection
            .cube
            .get_entity_at(ear.index)
    }

    /// Accesses the `Player` entity with the given ID if it exists in this archetype.
    #[allow(dead_code, unused)]
    fn get_player_entity(
        &self,
        entity_id: ::sillyecs::EntityId
    ) -> Option<PlayerEntityRef>
    {
        let ear = self.entity_locations.get(&entity_id)?.clone();
        if ear.archetype != PlayerArchetype::ID {
            return None;
        }
        self.collection
            .player
            .get_entity_at(ear.index)
    }

    /// Accesses the `ForegroundObject` entity with the given ID if it exists in this archetype.
    #[allow(dead_code, unused)]
    fn get_foreground_object_entity(
        &self,
        entity_id: ::sillyecs::EntityId
    ) -> Option<ForegroundObjectEntityRef>
    {
        let ear = self.entity_locations.get(&entity_id)?.clone();
        if ear.archetype != ForegroundObjectArchetype::ID {
            return None;
        }
        self.collection
            .foreground_object
            .get_entity_at(ear.index)
    }

    /// Accesses the `BackgroundObject` entity with the given ID if it exists in this archetype.
    #[allow(dead_code, unused)]
    fn get_background_object_entity(
        &self,
        entity_id: ::sillyecs::EntityId
    ) -> Option<BackgroundObjectEntityRef>
    {
        let ear = self.entity_locations.get(&entity_id)?.clone();
        if ear.archetype != BackgroundObjectArchetype::ID {
            return None;
        }
        self.collection
            .background_object
            .get_entity_at(ear.index)
    }

    /// Accesses the `SceneBackground` entity with the given ID if it exists in this archetype.
    #[allow(dead_code, unused)]
    fn get_scene_background_entity(
        &self,
        entity_id: ::sillyecs::EntityId
    ) -> Option<SceneBackgroundEntityRef>
    {
        let ear = self.entity_locations.get(&entity_id)?.clone();
        if ear.archetype != SceneBackgroundArchetype::ID {
            return None;
        }
        self.collection
            .scene_background
            .get_entity_at(ear.index)
    }
}

//noinspection RsSortImplTraitMembers
impl EntityAccessMut for MainWorldArchetypes {

    /// Mutably accesses the `WgpuUniformBindGroup` entity with the given ID if it exists in this archetype.
    #[allow(dead_code, unused)]
    fn get_wgpu_uniform_bind_group_entity_mut(
        &mut self,
        entity_id: ::sillyecs::EntityId
    ) -> Option<WgpuUniformBindGroupEntityMut>
    {
        let ear = self.entity_locations.get(&entity_id)?.clone();
        if ear.archetype != WgpuUniformBindGroupArchetype::ID {
            return None;
        }
        self.collection
            .wgpu_uniform_bind_group
            .get_entity_at_mut(ear.index)
    }

    /// Mutably accesses the `WgpuShader` entity with the given ID if it exists in this archetype.
    #[allow(dead_code, unused)]
    fn get_wgpu_shader_entity_mut(
        &mut self,
        entity_id: ::sillyecs::EntityId
    ) -> Option<WgpuShaderEntityMut>
    {
        let ear = self.entity_locations.get(&entity_id)?.clone();
        if ear.archetype != WgpuShaderArchetype::ID {
            return None;
        }
        self.collection
            .wgpu_shader
            .get_entity_at_mut(ear.index)
    }

    /// Mutably accesses the `WgpuTexture` entity with the given ID if it exists in this archetype.
    #[allow(dead_code, unused)]
    fn get_wgpu_texture_entity_mut(
        &mut self,
        entity_id: ::sillyecs::EntityId
    ) -> Option<WgpuTextureEntityMut>
    {
        let ear = self.entity_locations.get(&entity_id)?.clone();
        if ear.archetype != WgpuTextureArchetype::ID {
            return None;
        }
        self.collection
            .wgpu_texture
            .get_entity_at_mut(ear.index)
    }

    /// Mutably accesses the `Environment` entity with the given ID if it exists in this archetype.
    #[allow(dead_code, unused)]
    fn get_environment_entity_mut(
        &mut self,
        entity_id: ::sillyecs::EntityId
    ) -> Option<EnvironmentEntityMut>
    {
        let ear = self.entity_locations.get(&entity_id)?.clone();
        if ear.archetype != EnvironmentArchetype::ID {
            return None;
        }
        self.collection
            .environment
            .get_entity_at_mut(ear.index)
    }

    /// Mutably accesses the `Camera` entity with the given ID if it exists in this archetype.
    #[allow(dead_code, unused)]
    fn get_camera_entity_mut(
        &mut self,
        entity_id: ::sillyecs::EntityId
    ) -> Option<CameraEntityMut>
    {
        let ear = self.entity_locations.get(&entity_id)?.clone();
        if ear.archetype != CameraArchetype::ID {
            return None;
        }
        self.collection
            .camera
            .get_entity_at_mut(ear.index)
    }

    /// Mutably accesses the `DynamicLight` entity with the given ID if it exists in this archetype.
    #[allow(dead_code, unused)]
    fn get_dynamic_light_entity_mut(
        &mut self,
        entity_id: ::sillyecs::EntityId
    ) -> Option<DynamicLightEntityMut>
    {
        let ear = self.entity_locations.get(&entity_id)?.clone();
        if ear.archetype != DynamicLightArchetype::ID {
            return None;
        }
        self.collection
            .dynamic_light
            .get_entity_at_mut(ear.index)
    }

    /// Mutably accesses the `Tile` entity with the given ID if it exists in this archetype.
    #[allow(dead_code, unused)]
    fn get_tile_entity_mut(
        &mut self,
        entity_id: ::sillyecs::EntityId
    ) -> Option<TileEntityMut>
    {
        let ear = self.entity_locations.get(&entity_id)?.clone();
        if ear.archetype != TileArchetype::ID {
            return None;
        }
        self.collection
            .tile
            .get_entity_at_mut(ear.index)
    }

    /// Mutably accesses the `Cube` entity with the given ID if it exists in this archetype.
    #[allow(dead_code, unused)]
    fn get_cube_entity_mut(
        &mut self,
        entity_id: ::sillyecs::EntityId
    ) -> Option<CubeEntityMut>
    {
        let ear = self.entity_locations.get(&entity_id)?.clone();
        if ear.archetype != CubeArchetype::ID {
            return None;
        }
        self.collection
            .cube
            .get_entity_at_mut(ear.index)
    }

    /// Mutably accesses the `Player` entity with the given ID if it exists in this archetype.
    #[allow(dead_code, unused)]
    fn get_player_entity_mut(
        &mut self,
        entity_id: ::sillyecs::EntityId
    ) -> Option<PlayerEntityMut>
    {
        let ear = self.entity_locations.get(&entity_id)?.clone();
        if ear.archetype != PlayerArchetype::ID {
            return None;
        }
        self.collection
            .player
            .get_entity_at_mut(ear.index)
    }

    /// Mutably accesses the `ForegroundObject` entity with the given ID if it exists in this archetype.
    #[allow(dead_code, unused)]
    fn get_foreground_object_entity_mut(
        &mut self,
        entity_id: ::sillyecs::EntityId
    ) -> Option<ForegroundObjectEntityMut>
    {
        let ear = self.entity_locations.get(&entity_id)?.clone();
        if ear.archetype != ForegroundObjectArchetype::ID {
            return None;
        }
        self.collection
            .foreground_object
            .get_entity_at_mut(ear.index)
    }

    /// Mutably accesses the `BackgroundObject` entity with the given ID if it exists in this archetype.
    #[allow(dead_code, unused)]
    fn get_background_object_entity_mut(
        &mut self,
        entity_id: ::sillyecs::EntityId
    ) -> Option<BackgroundObjectEntityMut>
    {
        let ear = self.entity_locations.get(&entity_id)?.clone();
        if ear.archetype != BackgroundObjectArchetype::ID {
            return None;
        }
        self.collection
            .background_object
            .get_entity_at_mut(ear.index)
    }

    /// Mutably accesses the `SceneBackground` entity with the given ID if it exists in this archetype.
    #[allow(dead_code, unused)]
    fn get_scene_background_entity_mut(
        &mut self,
        entity_id: ::sillyecs::EntityId
    ) -> Option<SceneBackgroundEntityMut>
    {
        let ear = self.entity_locations.get(&entity_id)?.clone();
        if ear.archetype != SceneBackgroundArchetype::ID {
            return None;
        }
        self.collection
            .scene_background
            .get_entity_at_mut(ear.index)
    }
}

//noinspection RsSortImplTraitMembers
impl ComponentAccess for MainWorldArchetypes {

    /// Gets the `AmbientLight` component of the specified entity.
    #[allow(dead_code)]
    fn get_ambient_light_component(&self, entity_id: ::sillyecs::EntityId) -> Option<&AmbientLightComponent> {
        let ear = self.entity_locations.get(&entity_id)?.clone();
        match ear.archetype {
            EnvironmentArchetype::ID => self.collection.environment.get_ambient_light_component_at(ear.index),
            #[allow(dead_code)]
            _ => None
        }
    }

    /// Gets the `BackgroundColor` component of the specified entity.
    #[allow(dead_code)]
    fn get_background_color_component(&self, entity_id: ::sillyecs::EntityId) -> Option<&BackgroundColorComponent> {
        let ear = self.entity_locations.get(&entity_id)?.clone();
        match ear.archetype {
            SceneBackgroundArchetype::ID => self.collection.scene_background.get_background_color_component_at(ear.index),
            #[allow(dead_code)]
            _ => None
        }
    }

    /// Gets the `Camera` component of the specified entity.
    #[allow(dead_code)]
    fn get_camera_component(&self, entity_id: ::sillyecs::EntityId) -> Option<&CameraComponent> {
        let ear = self.entity_locations.get(&entity_id)?.clone();
        match ear.archetype {
            CameraArchetype::ID => self.collection.camera.get_camera_component_at(ear.index),
            #[allow(dead_code)]
            _ => None
        }
    }

    /// Gets the `CameraFpsController` component of the specified entity.
    #[allow(dead_code)]
    fn get_camera_fps_controller_component(&self, entity_id: ::sillyecs::EntityId) -> Option<&CameraFpsControllerComponent> {
        let ear = self.entity_locations.get(&entity_id)?.clone();
        match ear.archetype {
            CameraArchetype::ID => self.collection.camera.get_camera_fps_controller_component_at(ear.index),
            #[allow(dead_code)]
            _ => None
        }
    }

    /// Gets the `Collider` component of the specified entity.
    #[allow(dead_code)]
    fn get_collider_component(&self, entity_id: ::sillyecs::EntityId) -> Option<&ColliderComponent> {
        let ear = self.entity_locations.get(&entity_id)?.clone();
        match ear.archetype {
            ForegroundObjectArchetype::ID => self.collection.foreground_object.get_collider_component_at(ear.index),
            #[allow(dead_code)]
            _ => None
        }
    }

    /// Gets the `Color` component of the specified entity.
    #[allow(dead_code)]
    fn get_color_component(&self, entity_id: ::sillyecs::EntityId) -> Option<&ColorComponent> {
        let ear = self.entity_locations.get(&entity_id)?.clone();
        match ear.archetype {
            DynamicLightArchetype::ID => self.collection.dynamic_light.get_color_component_at(ear.index),
            #[allow(dead_code)]
            _ => None
        }
    }

    /// Gets the `DynamicLight` component of the specified entity.
    #[allow(dead_code)]
    fn get_dynamic_light_component(&self, entity_id: ::sillyecs::EntityId) -> Option<&DynamicLightComponent> {
        let ear = self.entity_locations.get(&entity_id)?.clone();
        match ear.archetype {
            DynamicLightArchetype::ID => self.collection.dynamic_light.get_dynamic_light_component_at(ear.index),
            #[allow(dead_code)]
            _ => None
        }
    }

    /// Gets the `Fog` component of the specified entity.
    #[allow(dead_code)]
    fn get_fog_component(&self, entity_id: ::sillyecs::EntityId) -> Option<&FogComponent> {
        let ear = self.entity_locations.get(&entity_id)?.clone();
        match ear.archetype {
            EnvironmentArchetype::ID => self.collection.environment.get_fog_component_at(ear.index),
            #[allow(dead_code)]
            _ => None
        }
    }

    /// Gets the `Health` component of the specified entity.
    #[allow(dead_code)]
    fn get_health_component(&self, entity_id: ::sillyecs::EntityId) -> Option<&HealthComponent> {
        let ear = self.entity_locations.get(&entity_id)?.clone();
        match ear.archetype {
            PlayerArchetype::ID => self.collection.player.get_health_component_at(ear.index),
            #[allow(dead_code)]
            _ => None
        }
    }

    /// Gets the `Model` component of the specified entity.
    #[allow(dead_code)]
    fn get_model_component(&self, entity_id: ::sillyecs::EntityId) -> Option<&ModelComponent> {
        let ear = self.entity_locations.get(&entity_id)?.clone();
        match ear.archetype {
            CubeArchetype::ID => self.collection.cube.get_model_component_at(ear.index),
            #[allow(dead_code)]
            _ => None
        }
    }

    /// Gets the `Position` component of the specified entity.
    #[allow(dead_code)]
    fn get_position_component(&self, entity_id: ::sillyecs::EntityId) -> Option<&PositionComponent> {
        let ear = self.entity_locations.get(&entity_id)?.clone();
        match ear.archetype {
            BackgroundObjectArchetype::ID => self.collection.background_object.get_position_component_at(ear.index),
            CubeArchetype::ID => self.collection.cube.get_position_component_at(ear.index),
            DynamicLightArchetype::ID => self.collection.dynamic_light.get_position_component_at(ear.index),
            TileArchetype::ID => self.collection.tile.get_position_component_at(ear.index),
            PlayerArchetype::ID => self.collection.player.get_position_component_at(ear.index),
            ForegroundObjectArchetype::ID => self.collection.foreground_object.get_position_component_at(ear.index),
            #[allow(dead_code)]
            _ => None
        }
    }

    /// Gets the `Skydome` component of the specified entity.
    #[allow(dead_code)]
    fn get_skydome_component(&self, entity_id: ::sillyecs::EntityId) -> Option<&SkydomeComponent> {
        let ear = self.entity_locations.get(&entity_id)?.clone();
        match ear.archetype {
            EnvironmentArchetype::ID => self.collection.environment.get_skydome_component_at(ear.index),
            #[allow(dead_code)]
            _ => None
        }
    }

    /// Gets the `Stars` component of the specified entity.
    #[allow(dead_code)]
    fn get_stars_component(&self, entity_id: ::sillyecs::EntityId) -> Option<&StarsComponent> {
        let ear = self.entity_locations.get(&entity_id)?.clone();
        match ear.archetype {
            EnvironmentArchetype::ID => self.collection.environment.get_stars_component_at(ear.index),
            #[allow(dead_code)]
            _ => None
        }
    }

    /// Gets the `Sun` component of the specified entity.
    #[allow(dead_code)]
    fn get_sun_component(&self, entity_id: ::sillyecs::EntityId) -> Option<&SunComponent> {
        let ear = self.entity_locations.get(&entity_id)?.clone();
        match ear.archetype {
            EnvironmentArchetype::ID => self.collection.environment.get_sun_component_at(ear.index),
            #[allow(dead_code)]
            _ => None
        }
    }

    /// Gets the `TextureSource` component of the specified entity.
    #[allow(dead_code)]
    fn get_texture_source_component(&self, entity_id: ::sillyecs::EntityId) -> Option<&TextureSourceComponent> {
        let ear = self.entity_locations.get(&entity_id)?.clone();
        match ear.archetype {
            WgpuTextureArchetype::ID => self.collection.wgpu_texture.get_texture_source_component_at(ear.index),
            #[allow(dead_code)]
            _ => None
        }
    }

    /// Gets the `Tile` component of the specified entity.
    #[allow(dead_code)]
    fn get_tile_component(&self, entity_id: ::sillyecs::EntityId) -> Option<&TileComponent> {
        let ear = self.entity_locations.get(&entity_id)?.clone();
        match ear.archetype {
            TileArchetype::ID => self.collection.tile.get_tile_component_at(ear.index),
            #[allow(dead_code)]
            _ => None
        }
    }

    /// Gets the `TimeOfDay` component of the specified entity.
    #[allow(dead_code)]
    fn get_time_of_day_component(&self, entity_id: ::sillyecs::EntityId) -> Option<&TimeOfDayComponent> {
        let ear = self.entity_locations.get(&entity_id)?.clone();
        match ear.archetype {
            EnvironmentArchetype::ID => self.collection.environment.get_time_of_day_component_at(ear.index),
            #[allow(dead_code)]
            _ => None
        }
    }

    /// Gets the `Transform` component of the specified entity.
    #[allow(dead_code)]
    fn get_transform_component(&self, entity_id: ::sillyecs::EntityId) -> Option<&TransformComponent> {
        let ear = self.entity_locations.get(&entity_id)?.clone();
        match ear.archetype {
            CubeArchetype::ID => self.collection.cube.get_transform_component_at(ear.index),
            #[allow(dead_code)]
            _ => None
        }
    }

    /// Gets the `UniformType` component of the specified entity.
    #[allow(dead_code)]
    fn get_uniform_type_component(&self, entity_id: ::sillyecs::EntityId) -> Option<&UniformTypeComponent> {
        let ear = self.entity_locations.get(&entity_id)?.clone();
        match ear.archetype {
            WgpuUniformBindGroupArchetype::ID => self.collection.wgpu_uniform_bind_group.get_uniform_type_component_at(ear.index),
            #[allow(dead_code)]
            _ => None
        }
    }

    /// Gets the `Velocity` component of the specified entity.
    #[allow(dead_code)]
    fn get_velocity_component(&self, entity_id: ::sillyecs::EntityId) -> Option<&VelocityComponent> {
        let ear = self.entity_locations.get(&entity_id)?.clone();
        match ear.archetype {
            CubeArchetype::ID => self.collection.cube.get_velocity_component_at(ear.index),
            PlayerArchetype::ID => self.collection.player.get_velocity_component_at(ear.index),
            #[allow(dead_code)]
            _ => None
        }
    }

    /// Gets the `WgpuBindGroup` component of the specified entity.
    #[allow(dead_code)]
    fn get_wgpu_bind_group_component(&self, entity_id: ::sillyecs::EntityId) -> Option<&WgpuBindGroupComponent> {
        let ear = self.entity_locations.get(&entity_id)?.clone();
        match ear.archetype {
            WgpuUniformBindGroupArchetype::ID => self.collection.wgpu_uniform_bind_group.get_wgpu_bind_group_component_at(ear.index),
            #[allow(dead_code)]
            _ => None
        }
    }

    /// Gets the `WgpuBindGroupLayout` component of the specified entity.
    #[allow(dead_code)]
    fn get_wgpu_bind_group_layout_component(&self, entity_id: ::sillyecs::EntityId) -> Option<&WgpuBindGroupLayoutComponent> {
        let ear = self.entity_locations.get(&entity_id)?.clone();
        match ear.archetype {
            WgpuUniformBindGroupArchetype::ID => self.collection.wgpu_uniform_bind_group.get_wgpu_bind_group_layout_component_at(ear.index),
            #[allow(dead_code)]
            _ => None
        }
    }

    /// Gets the `WgpuBuffer` component of the specified entity.
    #[allow(dead_code)]
    fn get_wgpu_buffer_component(&self, entity_id: ::sillyecs::EntityId) -> Option<&WgpuBufferComponent> {
        let ear = self.entity_locations.get(&entity_id)?.clone();
        match ear.archetype {
            WgpuUniformBindGroupArchetype::ID => self.collection.wgpu_uniform_bind_group.get_wgpu_buffer_component_at(ear.index),
            #[allow(dead_code)]
            _ => None
        }
    }

    /// Gets the `WgpuShader` component of the specified entity.
    #[allow(dead_code)]
    fn get_wgpu_shader_component(&self, entity_id: ::sillyecs::EntityId) -> Option<&WgpuShaderComponent> {
        let ear = self.entity_locations.get(&entity_id)?.clone();
        match ear.archetype {
            WgpuShaderArchetype::ID => self.collection.wgpu_shader.get_wgpu_shader_component_at(ear.index),
            #[allow(dead_code)]
            _ => None
        }
    }

    /// Gets the `WgpuTexture` component of the specified entity.
    #[allow(dead_code)]
    fn get_wgpu_texture_component(&self, entity_id: ::sillyecs::EntityId) -> Option<&WgpuTextureComponent> {
        let ear = self.entity_locations.get(&entity_id)?.clone();
        match ear.archetype {
            WgpuTextureArchetype::ID => self.collection.wgpu_texture.get_wgpu_texture_component_at(ear.index),
            #[allow(dead_code)]
            _ => None
        }
    }
}

//noinspection RsSortImplTraitMembers
impl ComponentAccessMut for MainWorldArchetypes {

    /// Mutably gets the `AmbientLight` component of the specified entity.
    #[allow(dead_code)]
    fn get_ambient_light_component_mut(&mut self, entity_id: ::sillyecs::EntityId) -> Option<&mut AmbientLightComponent> {
        let ear = self.entity_locations.get(&entity_id)?.clone();
        match ear.archetype {
            EnvironmentArchetype::ID => self.collection.environment.get_ambient_light_component_at_mut(ear.index),
            #[allow(dead_code)]
            _ => None
        }
    }

    /// Mutably gets the `BackgroundColor` component of the specified entity.
    #[allow(dead_code)]
    fn get_background_color_component_mut(&mut self, entity_id: ::sillyecs::EntityId) -> Option<&mut BackgroundColorComponent> {
        let ear = self.entity_locations.get(&entity_id)?.clone();
        match ear.archetype {
            SceneBackgroundArchetype::ID => self.collection.scene_background.get_background_color_component_at_mut(ear.index),
            #[allow(dead_code)]
            _ => None
        }
    }

    /// Mutably gets the `Camera` component of the specified entity.
    #[allow(dead_code)]
    fn get_camera_component_mut(&mut self, entity_id: ::sillyecs::EntityId) -> Option<&mut CameraComponent> {
        let ear = self.entity_locations.get(&entity_id)?.clone();
        match ear.archetype {
            CameraArchetype::ID => self.collection.camera.get_camera_component_at_mut(ear.index),
            #[allow(dead_code)]
            _ => None
        }
    }

    /// Mutably gets the `CameraFpsController` component of the specified entity.
    #[allow(dead_code)]
    fn get_camera_fps_controller_component_mut(&mut self, entity_id: ::sillyecs::EntityId) -> Option<&mut CameraFpsControllerComponent> {
        let ear = self.entity_locations.get(&entity_id)?.clone();
        match ear.archetype {
            CameraArchetype::ID => self.collection.camera.get_camera_fps_controller_component_at_mut(ear.index),
            #[allow(dead_code)]
            _ => None
        }
    }

    /// Mutably gets the `Collider` component of the specified entity.
    #[allow(dead_code)]
    fn get_collider_component_mut(&mut self, entity_id: ::sillyecs::EntityId) -> Option<&mut ColliderComponent> {
        let ear = self.entity_locations.get(&entity_id)?.clone();
        match ear.archetype {
            ForegroundObjectArchetype::ID => self.collection.foreground_object.get_collider_component_at_mut(ear.index),
            #[allow(dead_code)]
            _ => None
        }
    }

    /// Mutably gets the `Color` component of the specified entity.
    #[allow(dead_code)]
    fn get_color_component_mut(&mut self, entity_id: ::sillyecs::EntityId) -> Option<&mut ColorComponent> {
        let ear = self.entity_locations.get(&entity_id)?.clone();
        match ear.archetype {
            DynamicLightArchetype::ID => self.collection.dynamic_light.get_color_component_at_mut(ear.index),
            #[allow(dead_code)]
            _ => None
        }
    }

    /// Mutably gets the `DynamicLight` component of the specified entity.
    #[allow(dead_code)]
    fn get_dynamic_light_component_mut(&mut self, entity_id: ::sillyecs::EntityId) -> Option<&mut DynamicLightComponent> {
        let ear = self.entity_locations.get(&entity_id)?.clone();
        match ear.archetype {
            DynamicLightArchetype::ID => self.collection.dynamic_light.get_dynamic_light_component_at_mut(ear.index),
            #[allow(dead_code)]
            _ => None
        }
    }

    /// Mutably gets the `Fog` component of the specified entity.
    #[allow(dead_code)]
    fn get_fog_component_mut(&mut self, entity_id: ::sillyecs::EntityId) -> Option<&mut FogComponent> {
        let ear = self.entity_locations.get(&entity_id)?.clone();
        match ear.archetype {
            EnvironmentArchetype::ID => self.collection.environment.get_fog_component_at_mut(ear.index),
            #[allow(dead_code)]
            _ => None
        }
    }

    /// Mutably gets the `Health` component of the specified entity.
    #[allow(dead_code)]
    fn get_health_component_mut(&mut self, entity_id: ::sillyecs::EntityId) -> Option<&mut HealthComponent> {
        let ear = self.entity_locations.get(&entity_id)?.clone();
        match ear.archetype {
            PlayerArchetype::ID => self.collection.player.get_health_component_at_mut(ear.index),
            #[allow(dead_code)]
            _ => None
        }
    }

    /// Mutably gets the `Model` component of the specified entity.
    #[allow(dead_code)]
    fn get_model_component_mut(&mut self, entity_id: ::sillyecs::EntityId) -> Option<&mut ModelComponent> {
        let ear = self.entity_locations.get(&entity_id)?.clone();
        match ear.archetype {
            CubeArchetype::ID => self.collection.cube.get_model_component_at_mut(ear.index),
            #[allow(dead_code)]
            _ => None
        }
    }

    /// Mutably gets the `Position` component of the specified entity.
    #[allow(dead_code)]
    fn get_position_component_mut(&mut self, entity_id: ::sillyecs::EntityId) -> Option<&mut PositionComponent> {
        let ear = self.entity_locations.get(&entity_id)?.clone();
        match ear.archetype {
            BackgroundObjectArchetype::ID => self.collection.background_object.get_position_component_at_mut(ear.index),
            CubeArchetype::ID => self.collection.cube.get_position_component_at_mut(ear.index),
            DynamicLightArchetype::ID => self.collection.dynamic_light.get_position_component_at_mut(ear.index),
            TileArchetype::ID => self.collection.tile.get_position_component_at_mut(ear.index),
            PlayerArchetype::ID => self.collection.player.get_position_component_at_mut(ear.index),
            ForegroundObjectArchetype::ID => self.collection.foreground_object.get_position_component_at_mut(ear.index),
            #[allow(dead_code)]
            _ => None
        }
    }

    /// Mutably gets the `Skydome` component of the specified entity.
    #[allow(dead_code)]
    fn get_skydome_component_mut(&mut self, entity_id: ::sillyecs::EntityId) -> Option<&mut SkydomeComponent> {
        let ear = self.entity_locations.get(&entity_id)?.clone();
        match ear.archetype {
            EnvironmentArchetype::ID => self.collection.environment.get_skydome_component_at_mut(ear.index),
            #[allow(dead_code)]
            _ => None
        }
    }

    /// Mutably gets the `Stars` component of the specified entity.
    #[allow(dead_code)]
    fn get_stars_component_mut(&mut self, entity_id: ::sillyecs::EntityId) -> Option<&mut StarsComponent> {
        let ear = self.entity_locations.get(&entity_id)?.clone();
        match ear.archetype {
            EnvironmentArchetype::ID => self.collection.environment.get_stars_component_at_mut(ear.index),
            #[allow(dead_code)]
            _ => None
        }
    }

    /// Mutably gets the `Sun` component of the specified entity.
    #[allow(dead_code)]
    fn get_sun_component_mut(&mut self, entity_id: ::sillyecs::EntityId) -> Option<&mut SunComponent> {
        let ear = self.entity_locations.get(&entity_id)?.clone();
        match ear.archetype {
            EnvironmentArchetype::ID => self.collection.environment.get_sun_component_at_mut(ear.index),
            #[allow(dead_code)]
            _ => None
        }
    }

    /// Mutably gets the `TextureSource` component of the specified entity.
    #[allow(dead_code)]
    fn get_texture_source_component_mut(&mut self, entity_id: ::sillyecs::EntityId) -> Option<&mut TextureSourceComponent> {
        let ear = self.entity_locations.get(&entity_id)?.clone();
        match ear.archetype {
            WgpuTextureArchetype::ID => self.collection.wgpu_texture.get_texture_source_component_at_mut(ear.index),
            #[allow(dead_code)]
            _ => None
        }
    }

    /// Mutably gets the `Tile` component of the specified entity.
    #[allow(dead_code)]
    fn get_tile_component_mut(&mut self, entity_id: ::sillyecs::EntityId) -> Option<&mut TileComponent> {
        let ear = self.entity_locations.get(&entity_id)?.clone();
        match ear.archetype {
            TileArchetype::ID => self.collection.tile.get_tile_component_at_mut(ear.index),
            #[allow(dead_code)]
            _ => None
        }
    }

    /// Mutably gets the `TimeOfDay` component of the specified entity.
    #[allow(dead_code)]
    fn get_time_of_day_component_mut(&mut self, entity_id: ::sillyecs::EntityId) -> Option<&mut TimeOfDayComponent> {
        let ear = self.entity_locations.get(&entity_id)?.clone();
        match ear.archetype {
            EnvironmentArchetype::ID => self.collection.environment.get_time_of_day_component_at_mut(ear.index),
            #[allow(dead_code)]
            _ => None
        }
    }

    /// Mutably gets the `Transform` component of the specified entity.
    #[allow(dead_code)]
    fn get_transform_component_mut(&mut self, entity_id: ::sillyecs::EntityId) -> Option<&mut TransformComponent> {
        let ear = self.entity_locations.get(&entity_id)?.clone();
        match ear.archetype {
            CubeArchetype::ID => self.collection.cube.get_transform_component_at_mut(ear.index),
            #[allow(dead_code)]
            _ => None
        }
    }

    /// Mutably gets the `UniformType` component of the specified entity.
    #[allow(dead_code)]
    fn get_uniform_type_component_mut(&mut self, entity_id: ::sillyecs::EntityId) -> Option<&mut UniformTypeComponent> {
        let ear = self.entity_locations.get(&entity_id)?.clone();
        match ear.archetype {
            WgpuUniformBindGroupArchetype::ID => self.collection.wgpu_uniform_bind_group.get_uniform_type_component_at_mut(ear.index),
            #[allow(dead_code)]
            _ => None
        }
    }

    /// Mutably gets the `Velocity` component of the specified entity.
    #[allow(dead_code)]
    fn get_velocity_component_mut(&mut self, entity_id: ::sillyecs::EntityId) -> Option<&mut VelocityComponent> {
        let ear = self.entity_locations.get(&entity_id)?.clone();
        match ear.archetype {
            CubeArchetype::ID => self.collection.cube.get_velocity_component_at_mut(ear.index),
            PlayerArchetype::ID => self.collection.player.get_velocity_component_at_mut(ear.index),
            #[allow(dead_code)]
            _ => None
        }
    }

    /// Mutably gets the `WgpuBindGroup` component of the specified entity.
    #[allow(dead_code)]
    fn get_wgpu_bind_group_component_mut(&mut self, entity_id: ::sillyecs::EntityId) -> Option<&mut WgpuBindGroupComponent> {
        let ear = self.entity_locations.get(&entity_id)?.clone();
        match ear.archetype {
            WgpuUniformBindGroupArchetype::ID => self.collection.wgpu_uniform_bind_group.get_wgpu_bind_group_component_at_mut(ear.index),
            #[allow(dead_code)]
            _ => None
        }
    }

    /// Mutably gets the `WgpuBindGroupLayout` component of the specified entity.
    #[allow(dead_code)]
    fn get_wgpu_bind_group_layout_component_mut(&mut self, entity_id: ::sillyecs::EntityId) -> Option<&mut WgpuBindGroupLayoutComponent> {
        let ear = self.entity_locations.get(&entity_id)?.clone();
        match ear.archetype {
            WgpuUniformBindGroupArchetype::ID => self.collection.wgpu_uniform_bind_group.get_wgpu_bind_group_layout_component_at_mut(ear.index),
            #[allow(dead_code)]
            _ => None
        }
    }

    /// Mutably gets the `WgpuBuffer` component of the specified entity.
    #[allow(dead_code)]
    fn get_wgpu_buffer_component_mut(&mut self, entity_id: ::sillyecs::EntityId) -> Option<&mut WgpuBufferComponent> {
        let ear = self.entity_locations.get(&entity_id)?.clone();
        match ear.archetype {
            WgpuUniformBindGroupArchetype::ID => self.collection.wgpu_uniform_bind_group.get_wgpu_buffer_component_at_mut(ear.index),
            #[allow(dead_code)]
            _ => None
        }
    }

    /// Mutably gets the `WgpuShader` component of the specified entity.
    #[allow(dead_code)]
    fn get_wgpu_shader_component_mut(&mut self, entity_id: ::sillyecs::EntityId) -> Option<&mut WgpuShaderComponent> {
        let ear = self.entity_locations.get(&entity_id)?.clone();
        match ear.archetype {
            WgpuShaderArchetype::ID => self.collection.wgpu_shader.get_wgpu_shader_component_at_mut(ear.index),
            #[allow(dead_code)]
            _ => None
        }
    }

    /// Mutably gets the `WgpuTexture` component of the specified entity.
    #[allow(dead_code)]
    fn get_wgpu_texture_component_mut(&mut self, entity_id: ::sillyecs::EntityId) -> Option<&mut WgpuTextureComponent> {
        let ear = self.entity_locations.get(&entity_id)?.clone();
        match ear.archetype {
            WgpuTextureArchetype::ID => self.collection.wgpu_texture.get_wgpu_texture_component_at_mut(ear.index),
            #[allow(dead_code)]
            _ => None
        }
    }
}

impl<E, Q, U> WorldUserCommand for MainWorld<E, Q>
where
    Q: WorldUserCommand<UserCommand = U>,
    U: core::marker::Send + core::fmt::Debug
{
    type UserCommand = U;
}

/// An iterator over [`EntityId`](::sillyecs::EntityId) values.
#[allow(dead_code)]
pub type EntityIdIter<'a> = ::sillyecs::FlattenCopySlices<'a, ::sillyecs::EntityId>;

/// An iterator over all [`TimeOfDay`](TimeOfDayComponent) components, regardless of archetype.
#[allow(dead_code)]
// TODO: Simplify to ::core::slice::Iter<> if only one archetype has this component.
pub type TimeOfDayComponentIter<'a> = ::sillyecs::FlattenSlices<'a, TimeOfDayComponent>;

/// A mutable iterator over all [`TimeOfDay`](TimeOfDayComponent) components, regardless of archetype.
#[allow(dead_code)]
// TODO: Simplify to ::core::slice::IterMut<> if only one archetype has this component.
pub type TimeOfDayComponentIterMut<'a> = ::sillyecs::FlattenSlicesMut<'a, TimeOfDayComponent>;

/// A trait for types allowing to iterate all [`TimeOfDay`](TimeOfDayComponent) components, regardless of archetype.
#[allow(dead_code)]
pub trait IterTimeOfDayComponents<'a> {
    type Iterator: core::iter::Iterator<Item = &'a TimeOfDayComponent>;

    /// Iterates all [`TimeOfDay`](TimeOfDayComponent) components, regardless of archetype.
    fn iter_time_of_days(&'a self) -> Self::Iterator;
}

/// A trait for types allowing to iterate the [`EntityId`](::sillyecs::EntityId) values
/// of all [`TimeOfDay`](TimeOfDayComponent) components, regardless of archetype.
#[allow(dead_code)]
pub trait IterTimeOfDayEntities<'a> {
    type Iterator: core::iter::Iterator<Item = ::sillyecs::EntityId>;

    /// Iterates all entity IDs of [`TimeOfDay`](TimeOfDayComponent) components, regardless of archetype.
    fn iter_time_of_day_entities(&'a self) -> Self::Iterator;
}

/// A trait for types allowing to iterate all [`TimeOfDay`](TimeOfDayComponent) components, regardless of archetype.
#[allow(dead_code)]
pub trait IterMutTimeOfDayComponents<'a> {
    type IteratorMut: core::iter::Iterator<Item = &'a mut TimeOfDayComponent>;

    /// Iterates all [`TimeOfDay`](TimeOfDayComponent) components, regardless of archetype.
    fn iter_time_of_days_mut(&'a mut self) -> Self::IteratorMut;
}

/// An iterator over all [`Sun`](SunComponent) components, regardless of archetype.
#[allow(dead_code)]
// TODO: Simplify to ::core::slice::Iter<> if only one archetype has this component.
pub type SunComponentIter<'a> = ::sillyecs::FlattenSlices<'a, SunComponent>;

/// A mutable iterator over all [`Sun`](SunComponent) components, regardless of archetype.
#[allow(dead_code)]
// TODO: Simplify to ::core::slice::IterMut<> if only one archetype has this component.
pub type SunComponentIterMut<'a> = ::sillyecs::FlattenSlicesMut<'a, SunComponent>;

/// A trait for types allowing to iterate all [`Sun`](SunComponent) components, regardless of archetype.
#[allow(dead_code)]
pub trait IterSunComponents<'a> {
    type Iterator: core::iter::Iterator<Item = &'a SunComponent>;

    /// Iterates all [`Sun`](SunComponent) components, regardless of archetype.
    fn iter_suns(&'a self) -> Self::Iterator;
}

/// A trait for types allowing to iterate the [`EntityId`](::sillyecs::EntityId) values
/// of all [`Sun`](SunComponent) components, regardless of archetype.
#[allow(dead_code)]
pub trait IterSunEntities<'a> {
    type Iterator: core::iter::Iterator<Item = ::sillyecs::EntityId>;

    /// Iterates all entity IDs of [`Sun`](SunComponent) components, regardless of archetype.
    fn iter_sun_entities(&'a self) -> Self::Iterator;
}

/// A trait for types allowing to iterate all [`Sun`](SunComponent) components, regardless of archetype.
#[allow(dead_code)]
pub trait IterMutSunComponents<'a> {
    type IteratorMut: core::iter::Iterator<Item = &'a mut SunComponent>;

    /// Iterates all [`Sun`](SunComponent) components, regardless of archetype.
    fn iter_suns_mut(&'a mut self) -> Self::IteratorMut;
}

/// An iterator over all [`Skydome`](SkydomeComponent) components, regardless of archetype.
#[allow(dead_code)]
// TODO: Simplify to ::core::slice::Iter<> if only one archetype has this component.
pub type SkydomeComponentIter<'a> = ::sillyecs::FlattenSlices<'a, SkydomeComponent>;

/// A mutable iterator over all [`Skydome`](SkydomeComponent) components, regardless of archetype.
#[allow(dead_code)]
// TODO: Simplify to ::core::slice::IterMut<> if only one archetype has this component.
pub type SkydomeComponentIterMut<'a> = ::sillyecs::FlattenSlicesMut<'a, SkydomeComponent>;

/// A trait for types allowing to iterate all [`Skydome`](SkydomeComponent) components, regardless of archetype.
#[allow(dead_code)]
pub trait IterSkydomeComponents<'a> {
    type Iterator: core::iter::Iterator<Item = &'a SkydomeComponent>;

    /// Iterates all [`Skydome`](SkydomeComponent) components, regardless of archetype.
    fn iter_skydomes(&'a self) -> Self::Iterator;
}

/// A trait for types allowing to iterate the [`EntityId`](::sillyecs::EntityId) values
/// of all [`Skydome`](SkydomeComponent) components, regardless of archetype.
#[allow(dead_code)]
pub trait IterSkydomeEntities<'a> {
    type Iterator: core::iter::Iterator<Item = ::sillyecs::EntityId>;

    /// Iterates all entity IDs of [`Skydome`](SkydomeComponent) components, regardless of archetype.
    fn iter_skydome_entities(&'a self) -> Self::Iterator;
}

/// A trait for types allowing to iterate all [`Skydome`](SkydomeComponent) components, regardless of archetype.
#[allow(dead_code)]
pub trait IterMutSkydomeComponents<'a> {
    type IteratorMut: core::iter::Iterator<Item = &'a mut SkydomeComponent>;

    /// Iterates all [`Skydome`](SkydomeComponent) components, regardless of archetype.
    fn iter_skydomes_mut(&'a mut self) -> Self::IteratorMut;
}

/// An iterator over all [`Fog`](FogComponent) components, regardless of archetype.
#[allow(dead_code)]
// TODO: Simplify to ::core::slice::Iter<> if only one archetype has this component.
pub type FogComponentIter<'a> = ::sillyecs::FlattenSlices<'a, FogComponent>;

/// A mutable iterator over all [`Fog`](FogComponent) components, regardless of archetype.
#[allow(dead_code)]
// TODO: Simplify to ::core::slice::IterMut<> if only one archetype has this component.
pub type FogComponentIterMut<'a> = ::sillyecs::FlattenSlicesMut<'a, FogComponent>;

/// A trait for types allowing to iterate all [`Fog`](FogComponent) components, regardless of archetype.
#[allow(dead_code)]
pub trait IterFogComponents<'a> {
    type Iterator: core::iter::Iterator<Item = &'a FogComponent>;

    /// Iterates all [`Fog`](FogComponent) components, regardless of archetype.
    fn iter_fogs(&'a self) -> Self::Iterator;
}

/// A trait for types allowing to iterate the [`EntityId`](::sillyecs::EntityId) values
/// of all [`Fog`](FogComponent) components, regardless of archetype.
#[allow(dead_code)]
pub trait IterFogEntities<'a> {
    type Iterator: core::iter::Iterator<Item = ::sillyecs::EntityId>;

    /// Iterates all entity IDs of [`Fog`](FogComponent) components, regardless of archetype.
    fn iter_fog_entities(&'a self) -> Self::Iterator;
}

/// A trait for types allowing to iterate all [`Fog`](FogComponent) components, regardless of archetype.
#[allow(dead_code)]
pub trait IterMutFogComponents<'a> {
    type IteratorMut: core::iter::Iterator<Item = &'a mut FogComponent>;

    /// Iterates all [`Fog`](FogComponent) components, regardless of archetype.
    fn iter_fogs_mut(&'a mut self) -> Self::IteratorMut;
}

/// An iterator over all [`AmbientLight`](AmbientLightComponent) components, regardless of archetype.
#[allow(dead_code)]
// TODO: Simplify to ::core::slice::Iter<> if only one archetype has this component.
pub type AmbientLightComponentIter<'a> = ::sillyecs::FlattenSlices<'a, AmbientLightComponent>;

/// A mutable iterator over all [`AmbientLight`](AmbientLightComponent) components, regardless of archetype.
#[allow(dead_code)]
// TODO: Simplify to ::core::slice::IterMut<> if only one archetype has this component.
pub type AmbientLightComponentIterMut<'a> = ::sillyecs::FlattenSlicesMut<'a, AmbientLightComponent>;

/// A trait for types allowing to iterate all [`AmbientLight`](AmbientLightComponent) components, regardless of archetype.
#[allow(dead_code)]
pub trait IterAmbientLightComponents<'a> {
    type Iterator: core::iter::Iterator<Item = &'a AmbientLightComponent>;

    /// Iterates all [`AmbientLight`](AmbientLightComponent) components, regardless of archetype.
    fn iter_ambient_lights(&'a self) -> Self::Iterator;
}

/// A trait for types allowing to iterate the [`EntityId`](::sillyecs::EntityId) values
/// of all [`AmbientLight`](AmbientLightComponent) components, regardless of archetype.
#[allow(dead_code)]
pub trait IterAmbientLightEntities<'a> {
    type Iterator: core::iter::Iterator<Item = ::sillyecs::EntityId>;

    /// Iterates all entity IDs of [`AmbientLight`](AmbientLightComponent) components, regardless of archetype.
    fn iter_ambient_light_entities(&'a self) -> Self::Iterator;
}

/// A trait for types allowing to iterate all [`AmbientLight`](AmbientLightComponent) components, regardless of archetype.
#[allow(dead_code)]
pub trait IterMutAmbientLightComponents<'a> {
    type IteratorMut: core::iter::Iterator<Item = &'a mut AmbientLightComponent>;

    /// Iterates all [`AmbientLight`](AmbientLightComponent) components, regardless of archetype.
    fn iter_ambient_lights_mut(&'a mut self) -> Self::IteratorMut;
}

/// An iterator over all [`Stars`](StarsComponent) components, regardless of archetype.
#[allow(dead_code)]
// TODO: Simplify to ::core::slice::Iter<> if only one archetype has this component.
pub type StarsComponentIter<'a> = ::sillyecs::FlattenSlices<'a, StarsComponent>;

/// A mutable iterator over all [`Stars`](StarsComponent) components, regardless of archetype.
#[allow(dead_code)]
// TODO: Simplify to ::core::slice::IterMut<> if only one archetype has this component.
pub type StarsComponentIterMut<'a> = ::sillyecs::FlattenSlicesMut<'a, StarsComponent>;

/// A trait for types allowing to iterate all [`Stars`](StarsComponent) components, regardless of archetype.
#[allow(dead_code)]
pub trait IterStarsComponents<'a> {
    type Iterator: core::iter::Iterator<Item = &'a StarsComponent>;

    /// Iterates all [`Stars`](StarsComponent) components, regardless of archetype.
    fn iter_stars(&'a self) -> Self::Iterator;
}

/// A trait for types allowing to iterate the [`EntityId`](::sillyecs::EntityId) values
/// of all [`Stars`](StarsComponent) components, regardless of archetype.
#[allow(dead_code)]
pub trait IterStarsEntities<'a> {
    type Iterator: core::iter::Iterator<Item = ::sillyecs::EntityId>;

    /// Iterates all entity IDs of [`Stars`](StarsComponent) components, regardless of archetype.
    fn iter_stars_entities(&'a self) -> Self::Iterator;
}

/// A trait for types allowing to iterate all [`Stars`](StarsComponent) components, regardless of archetype.
#[allow(dead_code)]
pub trait IterMutStarsComponents<'a> {
    type IteratorMut: core::iter::Iterator<Item = &'a mut StarsComponent>;

    /// Iterates all [`Stars`](StarsComponent) components, regardless of archetype.
    fn iter_stars_mut(&'a mut self) -> Self::IteratorMut;
}

/// An iterator over all [`Camera`](CameraComponent) components, regardless of archetype.
#[allow(dead_code)]
// TODO: Simplify to ::core::slice::Iter<> if only one archetype has this component.
pub type CameraComponentIter<'a> = ::sillyecs::FlattenSlices<'a, CameraComponent>;

/// A mutable iterator over all [`Camera`](CameraComponent) components, regardless of archetype.
#[allow(dead_code)]
// TODO: Simplify to ::core::slice::IterMut<> if only one archetype has this component.
pub type CameraComponentIterMut<'a> = ::sillyecs::FlattenSlicesMut<'a, CameraComponent>;

/// A trait for types allowing to iterate all [`Camera`](CameraComponent) components, regardless of archetype.
#[allow(dead_code)]
pub trait IterCameraComponents<'a> {
    type Iterator: core::iter::Iterator<Item = &'a CameraComponent>;

    /// Iterates all [`Camera`](CameraComponent) components, regardless of archetype.
    fn iter_cameras(&'a self) -> Self::Iterator;
}

/// A trait for types allowing to iterate the [`EntityId`](::sillyecs::EntityId) values
/// of all [`Camera`](CameraComponent) components, regardless of archetype.
#[allow(dead_code)]
pub trait IterCameraEntities<'a> {
    type Iterator: core::iter::Iterator<Item = ::sillyecs::EntityId>;

    /// Iterates all entity IDs of [`Camera`](CameraComponent) components, regardless of archetype.
    fn iter_camera_entities(&'a self) -> Self::Iterator;
}

/// A trait for types allowing to iterate all [`Camera`](CameraComponent) components, regardless of archetype.
#[allow(dead_code)]
pub trait IterMutCameraComponents<'a> {
    type IteratorMut: core::iter::Iterator<Item = &'a mut CameraComponent>;

    /// Iterates all [`Camera`](CameraComponent) components, regardless of archetype.
    fn iter_cameras_mut(&'a mut self) -> Self::IteratorMut;
}

/// An iterator over all [`CameraFpsController`](CameraFpsControllerComponent) components, regardless of archetype.
#[allow(dead_code)]
// TODO: Simplify to ::core::slice::Iter<> if only one archetype has this component.
pub type CameraFpsControllerComponentIter<'a> = ::sillyecs::FlattenSlices<'a, CameraFpsControllerComponent>;

/// A mutable iterator over all [`CameraFpsController`](CameraFpsControllerComponent) components, regardless of archetype.
#[allow(dead_code)]
// TODO: Simplify to ::core::slice::IterMut<> if only one archetype has this component.
pub type CameraFpsControllerComponentIterMut<'a> = ::sillyecs::FlattenSlicesMut<'a, CameraFpsControllerComponent>;

/// A trait for types allowing to iterate all [`CameraFpsController`](CameraFpsControllerComponent) components, regardless of archetype.
#[allow(dead_code)]
pub trait IterCameraFpsControllerComponents<'a> {
    type Iterator: core::iter::Iterator<Item = &'a CameraFpsControllerComponent>;

    /// Iterates all [`CameraFpsController`](CameraFpsControllerComponent) components, regardless of archetype.
    fn iter_camera_fps_controllers(&'a self) -> Self::Iterator;
}

/// A trait for types allowing to iterate the [`EntityId`](::sillyecs::EntityId) values
/// of all [`CameraFpsController`](CameraFpsControllerComponent) components, regardless of archetype.
#[allow(dead_code)]
pub trait IterCameraFpsControllerEntities<'a> {
    type Iterator: core::iter::Iterator<Item = ::sillyecs::EntityId>;

    /// Iterates all entity IDs of [`CameraFpsController`](CameraFpsControllerComponent) components, regardless of archetype.
    fn iter_camera_fps_controller_entities(&'a self) -> Self::Iterator;
}

/// A trait for types allowing to iterate all [`CameraFpsController`](CameraFpsControllerComponent) components, regardless of archetype.
#[allow(dead_code)]
pub trait IterMutCameraFpsControllerComponents<'a> {
    type IteratorMut: core::iter::Iterator<Item = &'a mut CameraFpsControllerComponent>;

    /// Iterates all [`CameraFpsController`](CameraFpsControllerComponent) components, regardless of archetype.
    fn iter_camera_fps_controllers_mut(&'a mut self) -> Self::IteratorMut;
}

/// An iterator over all [`Position`](PositionComponent) components, regardless of archetype.
#[allow(dead_code)]
// TODO: Simplify to ::core::slice::Iter<> if only one archetype has this component.
pub type PositionComponentIter<'a> = ::sillyecs::FlattenSlices<'a, PositionComponent>;

/// A mutable iterator over all [`Position`](PositionComponent) components, regardless of archetype.
#[allow(dead_code)]
// TODO: Simplify to ::core::slice::IterMut<> if only one archetype has this component.
pub type PositionComponentIterMut<'a> = ::sillyecs::FlattenSlicesMut<'a, PositionComponent>;

/// A trait for types allowing to iterate all [`Position`](PositionComponent) components, regardless of archetype.
#[allow(dead_code)]
pub trait IterPositionComponents<'a> {
    type Iterator: core::iter::Iterator<Item = &'a PositionComponent>;

    /// Iterates all [`Position`](PositionComponent) components, regardless of archetype.
    fn iter_positions(&'a self) -> Self::Iterator;
}

/// A trait for types allowing to iterate the [`EntityId`](::sillyecs::EntityId) values
/// of all [`Position`](PositionComponent) components, regardless of archetype.
#[allow(dead_code)]
pub trait IterPositionEntities<'a> {
    type Iterator: core::iter::Iterator<Item = ::sillyecs::EntityId>;

    /// Iterates all entity IDs of [`Position`](PositionComponent) components, regardless of archetype.
    fn iter_position_entities(&'a self) -> Self::Iterator;
}

/// A trait for types allowing to iterate all [`Position`](PositionComponent) components, regardless of archetype.
#[allow(dead_code)]
pub trait IterMutPositionComponents<'a> {
    type IteratorMut: core::iter::Iterator<Item = &'a mut PositionComponent>;

    /// Iterates all [`Position`](PositionComponent) components, regardless of archetype.
    fn iter_positions_mut(&'a mut self) -> Self::IteratorMut;
}

/// An iterator over all [`Transform`](TransformComponent) components, regardless of archetype.
#[allow(dead_code)]
// TODO: Simplify to ::core::slice::Iter<> if only one archetype has this component.
pub type TransformComponentIter<'a> = ::sillyecs::FlattenSlices<'a, TransformComponent>;

/// A mutable iterator over all [`Transform`](TransformComponent) components, regardless of archetype.
#[allow(dead_code)]
// TODO: Simplify to ::core::slice::IterMut<> if only one archetype has this component.
pub type TransformComponentIterMut<'a> = ::sillyecs::FlattenSlicesMut<'a, TransformComponent>;

/// A trait for types allowing to iterate all [`Transform`](TransformComponent) components, regardless of archetype.
#[allow(dead_code)]
pub trait IterTransformComponents<'a> {
    type Iterator: core::iter::Iterator<Item = &'a TransformComponent>;

    /// Iterates all [`Transform`](TransformComponent) components, regardless of archetype.
    fn iter_transforms(&'a self) -> Self::Iterator;
}

/// A trait for types allowing to iterate the [`EntityId`](::sillyecs::EntityId) values
/// of all [`Transform`](TransformComponent) components, regardless of archetype.
#[allow(dead_code)]
pub trait IterTransformEntities<'a> {
    type Iterator: core::iter::Iterator<Item = ::sillyecs::EntityId>;

    /// Iterates all entity IDs of [`Transform`](TransformComponent) components, regardless of archetype.
    fn iter_transform_entities(&'a self) -> Self::Iterator;
}

/// A trait for types allowing to iterate all [`Transform`](TransformComponent) components, regardless of archetype.
#[allow(dead_code)]
pub trait IterMutTransformComponents<'a> {
    type IteratorMut: core::iter::Iterator<Item = &'a mut TransformComponent>;

    /// Iterates all [`Transform`](TransformComponent) components, regardless of archetype.
    fn iter_transforms_mut(&'a mut self) -> Self::IteratorMut;
}

/// An iterator over all [`Velocity`](VelocityComponent) components, regardless of archetype.
#[allow(dead_code)]
// TODO: Simplify to ::core::slice::Iter<> if only one archetype has this component.
pub type VelocityComponentIter<'a> = ::sillyecs::FlattenSlices<'a, VelocityComponent>;

/// A mutable iterator over all [`Velocity`](VelocityComponent) components, regardless of archetype.
#[allow(dead_code)]
// TODO: Simplify to ::core::slice::IterMut<> if only one archetype has this component.
pub type VelocityComponentIterMut<'a> = ::sillyecs::FlattenSlicesMut<'a, VelocityComponent>;

/// A trait for types allowing to iterate all [`Velocity`](VelocityComponent) components, regardless of archetype.
#[allow(dead_code)]
pub trait IterVelocityComponents<'a> {
    type Iterator: core::iter::Iterator<Item = &'a VelocityComponent>;

    /// Iterates all [`Velocity`](VelocityComponent) components, regardless of archetype.
    fn iter_velocities(&'a self) -> Self::Iterator;
}

/// A trait for types allowing to iterate the [`EntityId`](::sillyecs::EntityId) values
/// of all [`Velocity`](VelocityComponent) components, regardless of archetype.
#[allow(dead_code)]
pub trait IterVelocityEntities<'a> {
    type Iterator: core::iter::Iterator<Item = ::sillyecs::EntityId>;

    /// Iterates all entity IDs of [`Velocity`](VelocityComponent) components, regardless of archetype.
    fn iter_velocity_entities(&'a self) -> Self::Iterator;
}

/// A trait for types allowing to iterate all [`Velocity`](VelocityComponent) components, regardless of archetype.
#[allow(dead_code)]
pub trait IterMutVelocityComponents<'a> {
    type IteratorMut: core::iter::Iterator<Item = &'a mut VelocityComponent>;

    /// Iterates all [`Velocity`](VelocityComponent) components, regardless of archetype.
    fn iter_velocities_mut(&'a mut self) -> Self::IteratorMut;
}

/// An iterator over all [`Tile`](TileComponent) components, regardless of archetype.
#[allow(dead_code)]
// TODO: Simplify to ::core::slice::Iter<> if only one archetype has this component.
pub type TileComponentIter<'a> = ::sillyecs::FlattenSlices<'a, TileComponent>;

/// A mutable iterator over all [`Tile`](TileComponent) components, regardless of archetype.
#[allow(dead_code)]
// TODO: Simplify to ::core::slice::IterMut<> if only one archetype has this component.
pub type TileComponentIterMut<'a> = ::sillyecs::FlattenSlicesMut<'a, TileComponent>;

/// A trait for types allowing to iterate all [`Tile`](TileComponent) components, regardless of archetype.
#[allow(dead_code)]
pub trait IterTileComponents<'a> {
    type Iterator: core::iter::Iterator<Item = &'a TileComponent>;

    /// Iterates all [`Tile`](TileComponent) components, regardless of archetype.
    fn iter_tiles(&'a self) -> Self::Iterator;
}

/// A trait for types allowing to iterate the [`EntityId`](::sillyecs::EntityId) values
/// of all [`Tile`](TileComponent) components, regardless of archetype.
#[allow(dead_code)]
pub trait IterTileEntities<'a> {
    type Iterator: core::iter::Iterator<Item = ::sillyecs::EntityId>;

    /// Iterates all entity IDs of [`Tile`](TileComponent) components, regardless of archetype.
    fn iter_tile_entities(&'a self) -> Self::Iterator;
}

/// A trait for types allowing to iterate all [`Tile`](TileComponent) components, regardless of archetype.
#[allow(dead_code)]
pub trait IterMutTileComponents<'a> {
    type IteratorMut: core::iter::Iterator<Item = &'a mut TileComponent>;

    /// Iterates all [`Tile`](TileComponent) components, regardless of archetype.
    fn iter_tiles_mut(&'a mut self) -> Self::IteratorMut;
}

/// An iterator over all [`Health`](HealthComponent) components, regardless of archetype.
#[allow(dead_code)]
// TODO: Simplify to ::core::slice::Iter<> if only one archetype has this component.
pub type HealthComponentIter<'a> = ::sillyecs::FlattenSlices<'a, HealthComponent>;

/// A mutable iterator over all [`Health`](HealthComponent) components, regardless of archetype.
#[allow(dead_code)]
// TODO: Simplify to ::core::slice::IterMut<> if only one archetype has this component.
pub type HealthComponentIterMut<'a> = ::sillyecs::FlattenSlicesMut<'a, HealthComponent>;

/// A trait for types allowing to iterate all [`Health`](HealthComponent) components, regardless of archetype.
#[allow(dead_code)]
pub trait IterHealthComponents<'a> {
    type Iterator: core::iter::Iterator<Item = &'a HealthComponent>;

    /// Iterates all [`Health`](HealthComponent) components, regardless of archetype.
    fn iter_healths(&'a self) -> Self::Iterator;
}

/// A trait for types allowing to iterate the [`EntityId`](::sillyecs::EntityId) values
/// of all [`Health`](HealthComponent) components, regardless of archetype.
#[allow(dead_code)]
pub trait IterHealthEntities<'a> {
    type Iterator: core::iter::Iterator<Item = ::sillyecs::EntityId>;

    /// Iterates all entity IDs of [`Health`](HealthComponent) components, regardless of archetype.
    fn iter_health_entities(&'a self) -> Self::Iterator;
}

/// A trait for types allowing to iterate all [`Health`](HealthComponent) components, regardless of archetype.
#[allow(dead_code)]
pub trait IterMutHealthComponents<'a> {
    type IteratorMut: core::iter::Iterator<Item = &'a mut HealthComponent>;

    /// Iterates all [`Health`](HealthComponent) components, regardless of archetype.
    fn iter_healths_mut(&'a mut self) -> Self::IteratorMut;
}

/// An iterator over all [`Collider`](ColliderComponent) components, regardless of archetype.
#[allow(dead_code)]
// TODO: Simplify to ::core::slice::Iter<> if only one archetype has this component.
pub type ColliderComponentIter<'a> = ::sillyecs::FlattenSlices<'a, ColliderComponent>;

/// A mutable iterator over all [`Collider`](ColliderComponent) components, regardless of archetype.
#[allow(dead_code)]
// TODO: Simplify to ::core::slice::IterMut<> if only one archetype has this component.
pub type ColliderComponentIterMut<'a> = ::sillyecs::FlattenSlicesMut<'a, ColliderComponent>;

/// A trait for types allowing to iterate all [`Collider`](ColliderComponent) components, regardless of archetype.
#[allow(dead_code)]
pub trait IterColliderComponents<'a> {
    type Iterator: core::iter::Iterator<Item = &'a ColliderComponent>;

    /// Iterates all [`Collider`](ColliderComponent) components, regardless of archetype.
    fn iter_colliders(&'a self) -> Self::Iterator;
}

/// A trait for types allowing to iterate the [`EntityId`](::sillyecs::EntityId) values
/// of all [`Collider`](ColliderComponent) components, regardless of archetype.
#[allow(dead_code)]
pub trait IterColliderEntities<'a> {
    type Iterator: core::iter::Iterator<Item = ::sillyecs::EntityId>;

    /// Iterates all entity IDs of [`Collider`](ColliderComponent) components, regardless of archetype.
    fn iter_collider_entities(&'a self) -> Self::Iterator;
}

/// A trait for types allowing to iterate all [`Collider`](ColliderComponent) components, regardless of archetype.
#[allow(dead_code)]
pub trait IterMutColliderComponents<'a> {
    type IteratorMut: core::iter::Iterator<Item = &'a mut ColliderComponent>;

    /// Iterates all [`Collider`](ColliderComponent) components, regardless of archetype.
    fn iter_colliders_mut(&'a mut self) -> Self::IteratorMut;
}

/// An iterator over all [`Model`](ModelComponent) components, regardless of archetype.
#[allow(dead_code)]
// TODO: Simplify to ::core::slice::Iter<> if only one archetype has this component.
pub type ModelComponentIter<'a> = ::sillyecs::FlattenSlices<'a, ModelComponent>;

/// A mutable iterator over all [`Model`](ModelComponent) components, regardless of archetype.
#[allow(dead_code)]
// TODO: Simplify to ::core::slice::IterMut<> if only one archetype has this component.
pub type ModelComponentIterMut<'a> = ::sillyecs::FlattenSlicesMut<'a, ModelComponent>;

/// A trait for types allowing to iterate all [`Model`](ModelComponent) components, regardless of archetype.
#[allow(dead_code)]
pub trait IterModelComponents<'a> {
    type Iterator: core::iter::Iterator<Item = &'a ModelComponent>;

    /// Iterates all [`Model`](ModelComponent) components, regardless of archetype.
    fn iter_models(&'a self) -> Self::Iterator;
}

/// A trait for types allowing to iterate the [`EntityId`](::sillyecs::EntityId) values
/// of all [`Model`](ModelComponent) components, regardless of archetype.
#[allow(dead_code)]
pub trait IterModelEntities<'a> {
    type Iterator: core::iter::Iterator<Item = ::sillyecs::EntityId>;

    /// Iterates all entity IDs of [`Model`](ModelComponent) components, regardless of archetype.
    fn iter_model_entities(&'a self) -> Self::Iterator;
}

/// A trait for types allowing to iterate all [`Model`](ModelComponent) components, regardless of archetype.
#[allow(dead_code)]
pub trait IterMutModelComponents<'a> {
    type IteratorMut: core::iter::Iterator<Item = &'a mut ModelComponent>;

    /// Iterates all [`Model`](ModelComponent) components, regardless of archetype.
    fn iter_models_mut(&'a mut self) -> Self::IteratorMut;
}

/// An iterator over all [`Color`](ColorComponent) components, regardless of archetype.
#[allow(dead_code)]
// TODO: Simplify to ::core::slice::Iter<> if only one archetype has this component.
pub type ColorComponentIter<'a> = ::sillyecs::FlattenSlices<'a, ColorComponent>;

/// A mutable iterator over all [`Color`](ColorComponent) components, regardless of archetype.
#[allow(dead_code)]
// TODO: Simplify to ::core::slice::IterMut<> if only one archetype has this component.
pub type ColorComponentIterMut<'a> = ::sillyecs::FlattenSlicesMut<'a, ColorComponent>;

/// A trait for types allowing to iterate all [`Color`](ColorComponent) components, regardless of archetype.
#[allow(dead_code)]
pub trait IterColorComponents<'a> {
    type Iterator: core::iter::Iterator<Item = &'a ColorComponent>;

    /// Iterates all [`Color`](ColorComponent) components, regardless of archetype.
    fn iter_colors(&'a self) -> Self::Iterator;
}

/// A trait for types allowing to iterate the [`EntityId`](::sillyecs::EntityId) values
/// of all [`Color`](ColorComponent) components, regardless of archetype.
#[allow(dead_code)]
pub trait IterColorEntities<'a> {
    type Iterator: core::iter::Iterator<Item = ::sillyecs::EntityId>;

    /// Iterates all entity IDs of [`Color`](ColorComponent) components, regardless of archetype.
    fn iter_color_entities(&'a self) -> Self::Iterator;
}

/// A trait for types allowing to iterate all [`Color`](ColorComponent) components, regardless of archetype.
#[allow(dead_code)]
pub trait IterMutColorComponents<'a> {
    type IteratorMut: core::iter::Iterator<Item = &'a mut ColorComponent>;

    /// Iterates all [`Color`](ColorComponent) components, regardless of archetype.
    fn iter_colors_mut(&'a mut self) -> Self::IteratorMut;
}

/// An iterator over all [`BackgroundColor`](BackgroundColorComponent) components, regardless of archetype.
#[allow(dead_code)]
// TODO: Simplify to ::core::slice::Iter<> if only one archetype has this component.
pub type BackgroundColorComponentIter<'a> = ::sillyecs::FlattenSlices<'a, BackgroundColorComponent>;

/// A mutable iterator over all [`BackgroundColor`](BackgroundColorComponent) components, regardless of archetype.
#[allow(dead_code)]
// TODO: Simplify to ::core::slice::IterMut<> if only one archetype has this component.
pub type BackgroundColorComponentIterMut<'a> = ::sillyecs::FlattenSlicesMut<'a, BackgroundColorComponent>;

/// A trait for types allowing to iterate all [`BackgroundColor`](BackgroundColorComponent) components, regardless of archetype.
#[allow(dead_code)]
pub trait IterBackgroundColorComponents<'a> {
    type Iterator: core::iter::Iterator<Item = &'a BackgroundColorComponent>;

    /// Iterates all [`BackgroundColor`](BackgroundColorComponent) components, regardless of archetype.
    fn iter_background_colors(&'a self) -> Self::Iterator;
}

/// A trait for types allowing to iterate the [`EntityId`](::sillyecs::EntityId) values
/// of all [`BackgroundColor`](BackgroundColorComponent) components, regardless of archetype.
#[allow(dead_code)]
pub trait IterBackgroundColorEntities<'a> {
    type Iterator: core::iter::Iterator<Item = ::sillyecs::EntityId>;

    /// Iterates all entity IDs of [`BackgroundColor`](BackgroundColorComponent) components, regardless of archetype.
    fn iter_background_color_entities(&'a self) -> Self::Iterator;
}

/// A trait for types allowing to iterate all [`BackgroundColor`](BackgroundColorComponent) components, regardless of archetype.
#[allow(dead_code)]
pub trait IterMutBackgroundColorComponents<'a> {
    type IteratorMut: core::iter::Iterator<Item = &'a mut BackgroundColorComponent>;

    /// Iterates all [`BackgroundColor`](BackgroundColorComponent) components, regardless of archetype.
    fn iter_background_colors_mut(&'a mut self) -> Self::IteratorMut;
}

/// An iterator over all [`DynamicLight`](DynamicLightComponent) components, regardless of archetype.
#[allow(dead_code)]
// TODO: Simplify to ::core::slice::Iter<> if only one archetype has this component.
pub type DynamicLightComponentIter<'a> = ::sillyecs::FlattenSlices<'a, DynamicLightComponent>;

/// A mutable iterator over all [`DynamicLight`](DynamicLightComponent) components, regardless of archetype.
#[allow(dead_code)]
// TODO: Simplify to ::core::slice::IterMut<> if only one archetype has this component.
pub type DynamicLightComponentIterMut<'a> = ::sillyecs::FlattenSlicesMut<'a, DynamicLightComponent>;

/// A trait for types allowing to iterate all [`DynamicLight`](DynamicLightComponent) components, regardless of archetype.
#[allow(dead_code)]
pub trait IterDynamicLightComponents<'a> {
    type Iterator: core::iter::Iterator<Item = &'a DynamicLightComponent>;

    /// Iterates all [`DynamicLight`](DynamicLightComponent) components, regardless of archetype.
    fn iter_dynamic_lights(&'a self) -> Self::Iterator;
}

/// A trait for types allowing to iterate the [`EntityId`](::sillyecs::EntityId) values
/// of all [`DynamicLight`](DynamicLightComponent) components, regardless of archetype.
#[allow(dead_code)]
pub trait IterDynamicLightEntities<'a> {
    type Iterator: core::iter::Iterator<Item = ::sillyecs::EntityId>;

    /// Iterates all entity IDs of [`DynamicLight`](DynamicLightComponent) components, regardless of archetype.
    fn iter_dynamic_light_entities(&'a self) -> Self::Iterator;
}

/// A trait for types allowing to iterate all [`DynamicLight`](DynamicLightComponent) components, regardless of archetype.
#[allow(dead_code)]
pub trait IterMutDynamicLightComponents<'a> {
    type IteratorMut: core::iter::Iterator<Item = &'a mut DynamicLightComponent>;

    /// Iterates all [`DynamicLight`](DynamicLightComponent) components, regardless of archetype.
    fn iter_dynamic_lights_mut(&'a mut self) -> Self::IteratorMut;
}

/// An iterator over all [`WgpuShader`](WgpuShaderComponent) components, regardless of archetype.
#[allow(dead_code)]
// TODO: Simplify to ::core::slice::Iter<> if only one archetype has this component.
pub type WgpuShaderComponentIter<'a> = ::sillyecs::FlattenSlices<'a, WgpuShaderComponent>;

/// A mutable iterator over all [`WgpuShader`](WgpuShaderComponent) components, regardless of archetype.
#[allow(dead_code)]
// TODO: Simplify to ::core::slice::IterMut<> if only one archetype has this component.
pub type WgpuShaderComponentIterMut<'a> = ::sillyecs::FlattenSlicesMut<'a, WgpuShaderComponent>;

/// A trait for types allowing to iterate all [`WgpuShader`](WgpuShaderComponent) components, regardless of archetype.
#[allow(dead_code)]
pub trait IterWgpuShaderComponents<'a> {
    type Iterator: core::iter::Iterator<Item = &'a WgpuShaderComponent>;

    /// Iterates all [`WgpuShader`](WgpuShaderComponent) components, regardless of archetype.
    fn iter_wgpu_shaders(&'a self) -> Self::Iterator;
}

/// A trait for types allowing to iterate the [`EntityId`](::sillyecs::EntityId) values
/// of all [`WgpuShader`](WgpuShaderComponent) components, regardless of archetype.
#[allow(dead_code)]
pub trait IterWgpuShaderEntities<'a> {
    type Iterator: core::iter::Iterator<Item = ::sillyecs::EntityId>;

    /// Iterates all entity IDs of [`WgpuShader`](WgpuShaderComponent) components, regardless of archetype.
    fn iter_wgpu_shader_entities(&'a self) -> Self::Iterator;
}

/// A trait for types allowing to iterate all [`WgpuShader`](WgpuShaderComponent) components, regardless of archetype.
#[allow(dead_code)]
pub trait IterMutWgpuShaderComponents<'a> {
    type IteratorMut: core::iter::Iterator<Item = &'a mut WgpuShaderComponent>;

    /// Iterates all [`WgpuShader`](WgpuShaderComponent) components, regardless of archetype.
    fn iter_wgpu_shaders_mut(&'a mut self) -> Self::IteratorMut;
}

/// An iterator over all [`TextureSource`](TextureSourceComponent) components, regardless of archetype.
#[allow(dead_code)]
// TODO: Simplify to ::core::slice::Iter<> if only one archetype has this component.
pub type TextureSourceComponentIter<'a> = ::sillyecs::FlattenSlices<'a, TextureSourceComponent>;

/// A mutable iterator over all [`TextureSource`](TextureSourceComponent) components, regardless of archetype.
#[allow(dead_code)]
// TODO: Simplify to ::core::slice::IterMut<> if only one archetype has this component.
pub type TextureSourceComponentIterMut<'a> = ::sillyecs::FlattenSlicesMut<'a, TextureSourceComponent>;

/// A trait for types allowing to iterate all [`TextureSource`](TextureSourceComponent) components, regardless of archetype.
#[allow(dead_code)]
pub trait IterTextureSourceComponents<'a> {
    type Iterator: core::iter::Iterator<Item = &'a TextureSourceComponent>;

    /// Iterates all [`TextureSource`](TextureSourceComponent) components, regardless of archetype.
    fn iter_texture_sources(&'a self) -> Self::Iterator;
}

/// A trait for types allowing to iterate the [`EntityId`](::sillyecs::EntityId) values
/// of all [`TextureSource`](TextureSourceComponent) components, regardless of archetype.
#[allow(dead_code)]
pub trait IterTextureSourceEntities<'a> {
    type Iterator: core::iter::Iterator<Item = ::sillyecs::EntityId>;

    /// Iterates all entity IDs of [`TextureSource`](TextureSourceComponent) components, regardless of archetype.
    fn iter_texture_source_entities(&'a self) -> Self::Iterator;
}

/// A trait for types allowing to iterate all [`TextureSource`](TextureSourceComponent) components, regardless of archetype.
#[allow(dead_code)]
pub trait IterMutTextureSourceComponents<'a> {
    type IteratorMut: core::iter::Iterator<Item = &'a mut TextureSourceComponent>;

    /// Iterates all [`TextureSource`](TextureSourceComponent) components, regardless of archetype.
    fn iter_texture_sources_mut(&'a mut self) -> Self::IteratorMut;
}

/// An iterator over all [`WgpuTexture`](WgpuTextureComponent) components, regardless of archetype.
#[allow(dead_code)]
// TODO: Simplify to ::core::slice::Iter<> if only one archetype has this component.
pub type WgpuTextureComponentIter<'a> = ::sillyecs::FlattenSlices<'a, WgpuTextureComponent>;

/// A mutable iterator over all [`WgpuTexture`](WgpuTextureComponent) components, regardless of archetype.
#[allow(dead_code)]
// TODO: Simplify to ::core::slice::IterMut<> if only one archetype has this component.
pub type WgpuTextureComponentIterMut<'a> = ::sillyecs::FlattenSlicesMut<'a, WgpuTextureComponent>;

/// A trait for types allowing to iterate all [`WgpuTexture`](WgpuTextureComponent) components, regardless of archetype.
#[allow(dead_code)]
pub trait IterWgpuTextureComponents<'a> {
    type Iterator: core::iter::Iterator<Item = &'a WgpuTextureComponent>;

    /// Iterates all [`WgpuTexture`](WgpuTextureComponent) components, regardless of archetype.
    fn iter_wgpu_textures(&'a self) -> Self::Iterator;
}

/// A trait for types allowing to iterate the [`EntityId`](::sillyecs::EntityId) values
/// of all [`WgpuTexture`](WgpuTextureComponent) components, regardless of archetype.
#[allow(dead_code)]
pub trait IterWgpuTextureEntities<'a> {
    type Iterator: core::iter::Iterator<Item = ::sillyecs::EntityId>;

    /// Iterates all entity IDs of [`WgpuTexture`](WgpuTextureComponent) components, regardless of archetype.
    fn iter_wgpu_texture_entities(&'a self) -> Self::Iterator;
}

/// A trait for types allowing to iterate all [`WgpuTexture`](WgpuTextureComponent) components, regardless of archetype.
#[allow(dead_code)]
pub trait IterMutWgpuTextureComponents<'a> {
    type IteratorMut: core::iter::Iterator<Item = &'a mut WgpuTextureComponent>;

    /// Iterates all [`WgpuTexture`](WgpuTextureComponent) components, regardless of archetype.
    fn iter_wgpu_textures_mut(&'a mut self) -> Self::IteratorMut;
}

/// An iterator over all [`UniformType`](UniformTypeComponent) components, regardless of archetype.
#[allow(dead_code)]
// TODO: Simplify to ::core::slice::Iter<> if only one archetype has this component.
pub type UniformTypeComponentIter<'a> = ::sillyecs::FlattenSlices<'a, UniformTypeComponent>;

/// A mutable iterator over all [`UniformType`](UniformTypeComponent) components, regardless of archetype.
#[allow(dead_code)]
// TODO: Simplify to ::core::slice::IterMut<> if only one archetype has this component.
pub type UniformTypeComponentIterMut<'a> = ::sillyecs::FlattenSlicesMut<'a, UniformTypeComponent>;

/// A trait for types allowing to iterate all [`UniformType`](UniformTypeComponent) components, regardless of archetype.
#[allow(dead_code)]
pub trait IterUniformTypeComponents<'a> {
    type Iterator: core::iter::Iterator<Item = &'a UniformTypeComponent>;

    /// Iterates all [`UniformType`](UniformTypeComponent) components, regardless of archetype.
    fn iter_uniform_types(&'a self) -> Self::Iterator;
}

/// A trait for types allowing to iterate the [`EntityId`](::sillyecs::EntityId) values
/// of all [`UniformType`](UniformTypeComponent) components, regardless of archetype.
#[allow(dead_code)]
pub trait IterUniformTypeEntities<'a> {
    type Iterator: core::iter::Iterator<Item = ::sillyecs::EntityId>;

    /// Iterates all entity IDs of [`UniformType`](UniformTypeComponent) components, regardless of archetype.
    fn iter_uniform_type_entities(&'a self) -> Self::Iterator;
}

/// A trait for types allowing to iterate all [`UniformType`](UniformTypeComponent) components, regardless of archetype.
#[allow(dead_code)]
pub trait IterMutUniformTypeComponents<'a> {
    type IteratorMut: core::iter::Iterator<Item = &'a mut UniformTypeComponent>;

    /// Iterates all [`UniformType`](UniformTypeComponent) components, regardless of archetype.
    fn iter_uniform_types_mut(&'a mut self) -> Self::IteratorMut;
}

/// An iterator over all [`WgpuBindGroupLayout`](WgpuBindGroupLayoutComponent) components, regardless of archetype.
#[allow(dead_code)]
// TODO: Simplify to ::core::slice::Iter<> if only one archetype has this component.
pub type WgpuBindGroupLayoutComponentIter<'a> = ::sillyecs::FlattenSlices<'a, WgpuBindGroupLayoutComponent>;

/// A mutable iterator over all [`WgpuBindGroupLayout`](WgpuBindGroupLayoutComponent) components, regardless of archetype.
#[allow(dead_code)]
// TODO: Simplify to ::core::slice::IterMut<> if only one archetype has this component.
pub type WgpuBindGroupLayoutComponentIterMut<'a> = ::sillyecs::FlattenSlicesMut<'a, WgpuBindGroupLayoutComponent>;

/// A trait for types allowing to iterate all [`WgpuBindGroupLayout`](WgpuBindGroupLayoutComponent) components, regardless of archetype.
#[allow(dead_code)]
pub trait IterWgpuBindGroupLayoutComponents<'a> {
    type Iterator: core::iter::Iterator<Item = &'a WgpuBindGroupLayoutComponent>;

    /// Iterates all [`WgpuBindGroupLayout`](WgpuBindGroupLayoutComponent) components, regardless of archetype.
    fn iter_wgpu_bind_group_layouts(&'a self) -> Self::Iterator;
}

/// A trait for types allowing to iterate the [`EntityId`](::sillyecs::EntityId) values
/// of all [`WgpuBindGroupLayout`](WgpuBindGroupLayoutComponent) components, regardless of archetype.
#[allow(dead_code)]
pub trait IterWgpuBindGroupLayoutEntities<'a> {
    type Iterator: core::iter::Iterator<Item = ::sillyecs::EntityId>;

    /// Iterates all entity IDs of [`WgpuBindGroupLayout`](WgpuBindGroupLayoutComponent) components, regardless of archetype.
    fn iter_wgpu_bind_group_layout_entities(&'a self) -> Self::Iterator;
}

/// A trait for types allowing to iterate all [`WgpuBindGroupLayout`](WgpuBindGroupLayoutComponent) components, regardless of archetype.
#[allow(dead_code)]
pub trait IterMutWgpuBindGroupLayoutComponents<'a> {
    type IteratorMut: core::iter::Iterator<Item = &'a mut WgpuBindGroupLayoutComponent>;

    /// Iterates all [`WgpuBindGroupLayout`](WgpuBindGroupLayoutComponent) components, regardless of archetype.
    fn iter_wgpu_bind_group_layouts_mut(&'a mut self) -> Self::IteratorMut;
}

/// An iterator over all [`WgpuBindGroup`](WgpuBindGroupComponent) components, regardless of archetype.
#[allow(dead_code)]
// TODO: Simplify to ::core::slice::Iter<> if only one archetype has this component.
pub type WgpuBindGroupComponentIter<'a> = ::sillyecs::FlattenSlices<'a, WgpuBindGroupComponent>;

/// A mutable iterator over all [`WgpuBindGroup`](WgpuBindGroupComponent) components, regardless of archetype.
#[allow(dead_code)]
// TODO: Simplify to ::core::slice::IterMut<> if only one archetype has this component.
pub type WgpuBindGroupComponentIterMut<'a> = ::sillyecs::FlattenSlicesMut<'a, WgpuBindGroupComponent>;

/// A trait for types allowing to iterate all [`WgpuBindGroup`](WgpuBindGroupComponent) components, regardless of archetype.
#[allow(dead_code)]
pub trait IterWgpuBindGroupComponents<'a> {
    type Iterator: core::iter::Iterator<Item = &'a WgpuBindGroupComponent>;

    /// Iterates all [`WgpuBindGroup`](WgpuBindGroupComponent) components, regardless of archetype.
    fn iter_wgpu_bind_groups(&'a self) -> Self::Iterator;
}

/// A trait for types allowing to iterate the [`EntityId`](::sillyecs::EntityId) values
/// of all [`WgpuBindGroup`](WgpuBindGroupComponent) components, regardless of archetype.
#[allow(dead_code)]
pub trait IterWgpuBindGroupEntities<'a> {
    type Iterator: core::iter::Iterator<Item = ::sillyecs::EntityId>;

    /// Iterates all entity IDs of [`WgpuBindGroup`](WgpuBindGroupComponent) components, regardless of archetype.
    fn iter_wgpu_bind_group_entities(&'a self) -> Self::Iterator;
}

/// A trait for types allowing to iterate all [`WgpuBindGroup`](WgpuBindGroupComponent) components, regardless of archetype.
#[allow(dead_code)]
pub trait IterMutWgpuBindGroupComponents<'a> {
    type IteratorMut: core::iter::Iterator<Item = &'a mut WgpuBindGroupComponent>;

    /// Iterates all [`WgpuBindGroup`](WgpuBindGroupComponent) components, regardless of archetype.
    fn iter_wgpu_bind_groups_mut(&'a mut self) -> Self::IteratorMut;
}

/// An iterator over all [`WgpuBuffer`](WgpuBufferComponent) components, regardless of archetype.
#[allow(dead_code)]
// TODO: Simplify to ::core::slice::Iter<> if only one archetype has this component.
pub type WgpuBufferComponentIter<'a> = ::sillyecs::FlattenSlices<'a, WgpuBufferComponent>;

/// A mutable iterator over all [`WgpuBuffer`](WgpuBufferComponent) components, regardless of archetype.
#[allow(dead_code)]
// TODO: Simplify to ::core::slice::IterMut<> if only one archetype has this component.
pub type WgpuBufferComponentIterMut<'a> = ::sillyecs::FlattenSlicesMut<'a, WgpuBufferComponent>;

/// A trait for types allowing to iterate all [`WgpuBuffer`](WgpuBufferComponent) components, regardless of archetype.
#[allow(dead_code)]
pub trait IterWgpuBufferComponents<'a> {
    type Iterator: core::iter::Iterator<Item = &'a WgpuBufferComponent>;

    /// Iterates all [`WgpuBuffer`](WgpuBufferComponent) components, regardless of archetype.
    fn iter_wgpu_buffers(&'a self) -> Self::Iterator;
}

/// A trait for types allowing to iterate the [`EntityId`](::sillyecs::EntityId) values
/// of all [`WgpuBuffer`](WgpuBufferComponent) components, regardless of archetype.
#[allow(dead_code)]
pub trait IterWgpuBufferEntities<'a> {
    type Iterator: core::iter::Iterator<Item = ::sillyecs::EntityId>;

    /// Iterates all entity IDs of [`WgpuBuffer`](WgpuBufferComponent) components, regardless of archetype.
    fn iter_wgpu_buffer_entities(&'a self) -> Self::Iterator;
}

/// A trait for types allowing to iterate all [`WgpuBuffer`](WgpuBufferComponent) components, regardless of archetype.
#[allow(dead_code)]
pub trait IterMutWgpuBufferComponents<'a> {
    type IteratorMut: core::iter::Iterator<Item = &'a mut WgpuBufferComponent>;

    /// Iterates all [`WgpuBuffer`](WgpuBufferComponent) components, regardless of archetype.
    fn iter_wgpu_buffers_mut(&'a mut self) -> Self::IteratorMut;
}

#[allow(dead_code)]
impl<'a, E, Q> IterAmbientLightComponents<'a> for MainWorld<E, Q>
{
    type Iterator = AmbientLightComponentIter<'a>;

    /// Iterates over the [`AmbientLight`](AmbientLightComponent) component of the following archetypes, in that order:
    ///
    /// - [`Environment`](EnvironmentArchetype)
    fn iter_ambient_lights(&'a self) -> Self::Iterator {
        // TODO: Simplify to ::core::slice::Iter<> if only one archetype has this component.
        AmbientLightComponentIter::new([
            &self.archetypes.collection.environment.ambient_lights,
        ])
    }
}

#[allow(dead_code)]
impl<'a, E, Q> IterAmbientLightEntities<'a> for MainWorld<E, Q>
{
    type Iterator = EntityIdIter<'a>;

    /// Iterates all entity IDs of [`AmbientLight`](AmbientLightComponent) components, regardless of archetype.
    fn iter_ambient_light_entities(&'a self) -> Self::Iterator {
        // TODO: Simplify to ::core::slice::Iter<> if only one archetype has this component.
        EntityIdIter::new([
            &self.archetypes.collection.environment.entities,
        ])
    }
}

#[allow(dead_code)]
impl<'a, E, Q> IterMutAmbientLightComponents<'a> for MainWorld<E, Q>
{
    type IteratorMut = AmbientLightComponentIterMut<'a>;

    /// Mutably iterates over the [`AmbientLight`](AmbientLightComponent) component of the following archetypes, in that order:
    ///
    /// - [`Environment`](EnvironmentArchetype)
    fn iter_ambient_lights_mut(&'a mut self) -> Self::IteratorMut {
        // TODO: Simplify to ::core::slice::Iter<> if only one archetype has this component.
        AmbientLightComponentIterMut::new([
            &mut self.archetypes.collection.environment.ambient_lights,
        ])
    }
}

#[allow(dead_code)]
impl<'a, E, Q> IterBackgroundColorComponents<'a> for MainWorld<E, Q>
{
    type Iterator = BackgroundColorComponentIter<'a>;

    /// Iterates over the [`BackgroundColor`](BackgroundColorComponent) component of the following archetypes, in that order:
    ///
    /// - [`SceneBackground`](SceneBackgroundArchetype)
    fn iter_background_colors(&'a self) -> Self::Iterator {
        // TODO: Simplify to ::core::slice::Iter<> if only one archetype has this component.
        BackgroundColorComponentIter::new([
            &self.archetypes.collection.scene_background.background_colors,
        ])
    }
}

#[allow(dead_code)]
impl<'a, E, Q> IterBackgroundColorEntities<'a> for MainWorld<E, Q>
{
    type Iterator = EntityIdIter<'a>;

    /// Iterates all entity IDs of [`BackgroundColor`](BackgroundColorComponent) components, regardless of archetype.
    fn iter_background_color_entities(&'a self) -> Self::Iterator {
        // TODO: Simplify to ::core::slice::Iter<> if only one archetype has this component.
        EntityIdIter::new([
            &self.archetypes.collection.scene_background.entities,
        ])
    }
}

#[allow(dead_code)]
impl<'a, E, Q> IterMutBackgroundColorComponents<'a> for MainWorld<E, Q>
{
    type IteratorMut = BackgroundColorComponentIterMut<'a>;

    /// Mutably iterates over the [`BackgroundColor`](BackgroundColorComponent) component of the following archetypes, in that order:
    ///
    /// - [`SceneBackground`](SceneBackgroundArchetype)
    fn iter_background_colors_mut(&'a mut self) -> Self::IteratorMut {
        // TODO: Simplify to ::core::slice::Iter<> if only one archetype has this component.
        BackgroundColorComponentIterMut::new([
            &mut self.archetypes.collection.scene_background.background_colors,
        ])
    }
}

#[allow(dead_code)]
impl<'a, E, Q> IterCameraComponents<'a> for MainWorld<E, Q>
{
    type Iterator = CameraComponentIter<'a>;

    /// Iterates over the [`Camera`](CameraComponent) component of the following archetypes, in that order:
    ///
    /// - [`Camera`](CameraArchetype)
    fn iter_cameras(&'a self) -> Self::Iterator {
        // TODO: Simplify to ::core::slice::Iter<> if only one archetype has this component.
        CameraComponentIter::new([
            &self.archetypes.collection.camera.cameras,
        ])
    }
}

#[allow(dead_code)]
impl<'a, E, Q> IterCameraEntities<'a> for MainWorld<E, Q>
{
    type Iterator = EntityIdIter<'a>;

    /// Iterates all entity IDs of [`Camera`](CameraComponent) components, regardless of archetype.
    fn iter_camera_entities(&'a self) -> Self::Iterator {
        // TODO: Simplify to ::core::slice::Iter<> if only one archetype has this component.
        EntityIdIter::new([
            &self.archetypes.collection.camera.entities,
        ])
    }
}

#[allow(dead_code)]
impl<'a, E, Q> IterMutCameraComponents<'a> for MainWorld<E, Q>
{
    type IteratorMut = CameraComponentIterMut<'a>;

    /// Mutably iterates over the [`Camera`](CameraComponent) component of the following archetypes, in that order:
    ///
    /// - [`Camera`](CameraArchetype)
    fn iter_cameras_mut(&'a mut self) -> Self::IteratorMut {
        // TODO: Simplify to ::core::slice::Iter<> if only one archetype has this component.
        CameraComponentIterMut::new([
            &mut self.archetypes.collection.camera.cameras,
        ])
    }
}

#[allow(dead_code)]
impl<'a, E, Q> IterCameraFpsControllerComponents<'a> for MainWorld<E, Q>
{
    type Iterator = CameraFpsControllerComponentIter<'a>;

    /// Iterates over the [`CameraFpsController`](CameraFpsControllerComponent) component of the following archetypes, in that order:
    ///
    /// - [`Camera`](CameraArchetype)
    fn iter_camera_fps_controllers(&'a self) -> Self::Iterator {
        // TODO: Simplify to ::core::slice::Iter<> if only one archetype has this component.
        CameraFpsControllerComponentIter::new([
            &self.archetypes.collection.camera.camera_fps_controllers,
        ])
    }
}

#[allow(dead_code)]
impl<'a, E, Q> IterCameraFpsControllerEntities<'a> for MainWorld<E, Q>
{
    type Iterator = EntityIdIter<'a>;

    /// Iterates all entity IDs of [`CameraFpsController`](CameraFpsControllerComponent) components, regardless of archetype.
    fn iter_camera_fps_controller_entities(&'a self) -> Self::Iterator {
        // TODO: Simplify to ::core::slice::Iter<> if only one archetype has this component.
        EntityIdIter::new([
            &self.archetypes.collection.camera.entities,
        ])
    }
}

#[allow(dead_code)]
impl<'a, E, Q> IterMutCameraFpsControllerComponents<'a> for MainWorld<E, Q>
{
    type IteratorMut = CameraFpsControllerComponentIterMut<'a>;

    /// Mutably iterates over the [`CameraFpsController`](CameraFpsControllerComponent) component of the following archetypes, in that order:
    ///
    /// - [`Camera`](CameraArchetype)
    fn iter_camera_fps_controllers_mut(&'a mut self) -> Self::IteratorMut {
        // TODO: Simplify to ::core::slice::Iter<> if only one archetype has this component.
        CameraFpsControllerComponentIterMut::new([
            &mut self.archetypes.collection.camera.camera_fps_controllers,
        ])
    }
}

#[allow(dead_code)]
impl<'a, E, Q> IterColliderComponents<'a> for MainWorld<E, Q>
{
    type Iterator = ColliderComponentIter<'a>;

    /// Iterates over the [`Collider`](ColliderComponent) component of the following archetypes, in that order:
    ///
    /// - [`ForegroundObject`](ForegroundObjectArchetype)
    fn iter_colliders(&'a self) -> Self::Iterator {
        // TODO: Simplify to ::core::slice::Iter<> if only one archetype has this component.
        ColliderComponentIter::new([
            &self.archetypes.collection.foreground_object.colliders,
        ])
    }
}

#[allow(dead_code)]
impl<'a, E, Q> IterColliderEntities<'a> for MainWorld<E, Q>
{
    type Iterator = EntityIdIter<'a>;

    /// Iterates all entity IDs of [`Collider`](ColliderComponent) components, regardless of archetype.
    fn iter_collider_entities(&'a self) -> Self::Iterator {
        // TODO: Simplify to ::core::slice::Iter<> if only one archetype has this component.
        EntityIdIter::new([
            &self.archetypes.collection.foreground_object.entities,
        ])
    }
}

#[allow(dead_code)]
impl<'a, E, Q> IterMutColliderComponents<'a> for MainWorld<E, Q>
{
    type IteratorMut = ColliderComponentIterMut<'a>;

    /// Mutably iterates over the [`Collider`](ColliderComponent) component of the following archetypes, in that order:
    ///
    /// - [`ForegroundObject`](ForegroundObjectArchetype)
    fn iter_colliders_mut(&'a mut self) -> Self::IteratorMut {
        // TODO: Simplify to ::core::slice::Iter<> if only one archetype has this component.
        ColliderComponentIterMut::new([
            &mut self.archetypes.collection.foreground_object.colliders,
        ])
    }
}

#[allow(dead_code)]
impl<'a, E, Q> IterColorComponents<'a> for MainWorld<E, Q>
{
    type Iterator = ColorComponentIter<'a>;

    /// Iterates over the [`Color`](ColorComponent) component of the following archetypes, in that order:
    ///
    /// - [`DynamicLight`](DynamicLightArchetype)
    fn iter_colors(&'a self) -> Self::Iterator {
        // TODO: Simplify to ::core::slice::Iter<> if only one archetype has this component.
        ColorComponentIter::new([
            &self.archetypes.collection.dynamic_light.colors,
        ])
    }
}

#[allow(dead_code)]
impl<'a, E, Q> IterColorEntities<'a> for MainWorld<E, Q>
{
    type Iterator = EntityIdIter<'a>;

    /// Iterates all entity IDs of [`Color`](ColorComponent) components, regardless of archetype.
    fn iter_color_entities(&'a self) -> Self::Iterator {
        // TODO: Simplify to ::core::slice::Iter<> if only one archetype has this component.
        EntityIdIter::new([
            &self.archetypes.collection.dynamic_light.entities,
        ])
    }
}

#[allow(dead_code)]
impl<'a, E, Q> IterMutColorComponents<'a> for MainWorld<E, Q>
{
    type IteratorMut = ColorComponentIterMut<'a>;

    /// Mutably iterates over the [`Color`](ColorComponent) component of the following archetypes, in that order:
    ///
    /// - [`DynamicLight`](DynamicLightArchetype)
    fn iter_colors_mut(&'a mut self) -> Self::IteratorMut {
        // TODO: Simplify to ::core::slice::Iter<> if only one archetype has this component.
        ColorComponentIterMut::new([
            &mut self.archetypes.collection.dynamic_light.colors,
        ])
    }
}

#[allow(dead_code)]
impl<'a, E, Q> IterDynamicLightComponents<'a> for MainWorld<E, Q>
{
    type Iterator = DynamicLightComponentIter<'a>;

    /// Iterates over the [`DynamicLight`](DynamicLightComponent) component of the following archetypes, in that order:
    ///
    /// - [`DynamicLight`](DynamicLightArchetype)
    fn iter_dynamic_lights(&'a self) -> Self::Iterator {
        // TODO: Simplify to ::core::slice::Iter<> if only one archetype has this component.
        DynamicLightComponentIter::new([
            &self.archetypes.collection.dynamic_light.dynamic_lights,
        ])
    }
}

#[allow(dead_code)]
impl<'a, E, Q> IterDynamicLightEntities<'a> for MainWorld<E, Q>
{
    type Iterator = EntityIdIter<'a>;

    /// Iterates all entity IDs of [`DynamicLight`](DynamicLightComponent) components, regardless of archetype.
    fn iter_dynamic_light_entities(&'a self) -> Self::Iterator {
        // TODO: Simplify to ::core::slice::Iter<> if only one archetype has this component.
        EntityIdIter::new([
            &self.archetypes.collection.dynamic_light.entities,
        ])
    }
}

#[allow(dead_code)]
impl<'a, E, Q> IterMutDynamicLightComponents<'a> for MainWorld<E, Q>
{
    type IteratorMut = DynamicLightComponentIterMut<'a>;

    /// Mutably iterates over the [`DynamicLight`](DynamicLightComponent) component of the following archetypes, in that order:
    ///
    /// - [`DynamicLight`](DynamicLightArchetype)
    fn iter_dynamic_lights_mut(&'a mut self) -> Self::IteratorMut {
        // TODO: Simplify to ::core::slice::Iter<> if only one archetype has this component.
        DynamicLightComponentIterMut::new([
            &mut self.archetypes.collection.dynamic_light.dynamic_lights,
        ])
    }
}

#[allow(dead_code)]
impl<'a, E, Q> IterFogComponents<'a> for MainWorld<E, Q>
{
    type Iterator = FogComponentIter<'a>;

    /// Iterates over the [`Fog`](FogComponent) component of the following archetypes, in that order:
    ///
    /// - [`Environment`](EnvironmentArchetype)
    fn iter_fogs(&'a self) -> Self::Iterator {
        // TODO: Simplify to ::core::slice::Iter<> if only one archetype has this component.
        FogComponentIter::new([
            &self.archetypes.collection.environment.fogs,
        ])
    }
}

#[allow(dead_code)]
impl<'a, E, Q> IterFogEntities<'a> for MainWorld<E, Q>
{
    type Iterator = EntityIdIter<'a>;

    /// Iterates all entity IDs of [`Fog`](FogComponent) components, regardless of archetype.
    fn iter_fog_entities(&'a self) -> Self::Iterator {
        // TODO: Simplify to ::core::slice::Iter<> if only one archetype has this component.
        EntityIdIter::new([
            &self.archetypes.collection.environment.entities,
        ])
    }
}

#[allow(dead_code)]
impl<'a, E, Q> IterMutFogComponents<'a> for MainWorld<E, Q>
{
    type IteratorMut = FogComponentIterMut<'a>;

    /// Mutably iterates over the [`Fog`](FogComponent) component of the following archetypes, in that order:
    ///
    /// - [`Environment`](EnvironmentArchetype)
    fn iter_fogs_mut(&'a mut self) -> Self::IteratorMut {
        // TODO: Simplify to ::core::slice::Iter<> if only one archetype has this component.
        FogComponentIterMut::new([
            &mut self.archetypes.collection.environment.fogs,
        ])
    }
}

#[allow(dead_code)]
impl<'a, E, Q> IterHealthComponents<'a> for MainWorld<E, Q>
{
    type Iterator = HealthComponentIter<'a>;

    /// Iterates over the [`Health`](HealthComponent) component of the following archetypes, in that order:
    ///
    /// - [`Player`](PlayerArchetype)
    fn iter_healths(&'a self) -> Self::Iterator {
        // TODO: Simplify to ::core::slice::Iter<> if only one archetype has this component.
        HealthComponentIter::new([
            &self.archetypes.collection.player.healths,
        ])
    }
}

#[allow(dead_code)]
impl<'a, E, Q> IterHealthEntities<'a> for MainWorld<E, Q>
{
    type Iterator = EntityIdIter<'a>;

    /// Iterates all entity IDs of [`Health`](HealthComponent) components, regardless of archetype.
    fn iter_health_entities(&'a self) -> Self::Iterator {
        // TODO: Simplify to ::core::slice::Iter<> if only one archetype has this component.
        EntityIdIter::new([
            &self.archetypes.collection.player.entities,
        ])
    }
}

#[allow(dead_code)]
impl<'a, E, Q> IterMutHealthComponents<'a> for MainWorld<E, Q>
{
    type IteratorMut = HealthComponentIterMut<'a>;

    /// Mutably iterates over the [`Health`](HealthComponent) component of the following archetypes, in that order:
    ///
    /// - [`Player`](PlayerArchetype)
    fn iter_healths_mut(&'a mut self) -> Self::IteratorMut {
        // TODO: Simplify to ::core::slice::Iter<> if only one archetype has this component.
        HealthComponentIterMut::new([
            &mut self.archetypes.collection.player.healths,
        ])
    }
}

#[allow(dead_code)]
impl<'a, E, Q> IterModelComponents<'a> for MainWorld<E, Q>
{
    type Iterator = ModelComponentIter<'a>;

    /// Iterates over the [`Model`](ModelComponent) component of the following archetypes, in that order:
    ///
    /// - [`Cube`](CubeArchetype)
    fn iter_models(&'a self) -> Self::Iterator {
        // TODO: Simplify to ::core::slice::Iter<> if only one archetype has this component.
        ModelComponentIter::new([
            &self.archetypes.collection.cube.models,
        ])
    }
}

#[allow(dead_code)]
impl<'a, E, Q> IterModelEntities<'a> for MainWorld<E, Q>
{
    type Iterator = EntityIdIter<'a>;

    /// Iterates all entity IDs of [`Model`](ModelComponent) components, regardless of archetype.
    fn iter_model_entities(&'a self) -> Self::Iterator {
        // TODO: Simplify to ::core::slice::Iter<> if only one archetype has this component.
        EntityIdIter::new([
            &self.archetypes.collection.cube.entities,
        ])
    }
}

#[allow(dead_code)]
impl<'a, E, Q> IterMutModelComponents<'a> for MainWorld<E, Q>
{
    type IteratorMut = ModelComponentIterMut<'a>;

    /// Mutably iterates over the [`Model`](ModelComponent) component of the following archetypes, in that order:
    ///
    /// - [`Cube`](CubeArchetype)
    fn iter_models_mut(&'a mut self) -> Self::IteratorMut {
        // TODO: Simplify to ::core::slice::Iter<> if only one archetype has this component.
        ModelComponentIterMut::new([
            &mut self.archetypes.collection.cube.models,
        ])
    }
}

#[allow(dead_code)]
impl<'a, E, Q> IterPositionComponents<'a> for MainWorld<E, Q>
{
    type Iterator = PositionComponentIter<'a>;

    /// Iterates over the [`Position`](PositionComponent) component of the following archetypes, in that order:
    ///
    /// - [`DynamicLight`](DynamicLightArchetype)
    /// - [`Tile`](TileArchetype)
    /// - [`Cube`](CubeArchetype)
    /// - [`Player`](PlayerArchetype)
    /// - [`ForegroundObject`](ForegroundObjectArchetype)
    /// - [`BackgroundObject`](BackgroundObjectArchetype)
    fn iter_positions(&'a self) -> Self::Iterator {
        // TODO: Simplify to ::core::slice::Iter<> if only one archetype has this component.
        PositionComponentIter::new([
            &self.archetypes.collection.dynamic_light.positions,
            &self.archetypes.collection.tile.positions,
            &self.archetypes.collection.cube.positions,
            &self.archetypes.collection.player.positions,
            &self.archetypes.collection.foreground_object.positions,
            &self.archetypes.collection.background_object.positions,
        ])
    }
}

#[allow(dead_code)]
impl<'a, E, Q> IterPositionEntities<'a> for MainWorld<E, Q>
{
    type Iterator = EntityIdIter<'a>;

    /// Iterates all entity IDs of [`Position`](PositionComponent) components, regardless of archetype.
    fn iter_position_entities(&'a self) -> Self::Iterator {
        // TODO: Simplify to ::core::slice::Iter<> if only one archetype has this component.
        EntityIdIter::new([
            &self.archetypes.collection.dynamic_light.entities,
            &self.archetypes.collection.tile.entities,
            &self.archetypes.collection.cube.entities,
            &self.archetypes.collection.player.entities,
            &self.archetypes.collection.foreground_object.entities,
            &self.archetypes.collection.background_object.entities,
        ])
    }
}

#[allow(dead_code)]
impl<'a, E, Q> IterMutPositionComponents<'a> for MainWorld<E, Q>
{
    type IteratorMut = PositionComponentIterMut<'a>;

    /// Mutably iterates over the [`Position`](PositionComponent) component of the following archetypes, in that order:
    ///
    /// - [`DynamicLight`](DynamicLightArchetype)
    /// - [`Tile`](TileArchetype)
    /// - [`Cube`](CubeArchetype)
    /// - [`Player`](PlayerArchetype)
    /// - [`ForegroundObject`](ForegroundObjectArchetype)
    /// - [`BackgroundObject`](BackgroundObjectArchetype)
    fn iter_positions_mut(&'a mut self) -> Self::IteratorMut {
        // TODO: Simplify to ::core::slice::Iter<> if only one archetype has this component.
        PositionComponentIterMut::new([
            &mut self.archetypes.collection.dynamic_light.positions,
            &mut self.archetypes.collection.tile.positions,
            &mut self.archetypes.collection.cube.positions,
            &mut self.archetypes.collection.player.positions,
            &mut self.archetypes.collection.foreground_object.positions,
            &mut self.archetypes.collection.background_object.positions,
        ])
    }
}

#[allow(dead_code)]
impl<'a, E, Q> IterSkydomeComponents<'a> for MainWorld<E, Q>
{
    type Iterator = SkydomeComponentIter<'a>;

    /// Iterates over the [`Skydome`](SkydomeComponent) component of the following archetypes, in that order:
    ///
    /// - [`Environment`](EnvironmentArchetype)
    fn iter_skydomes(&'a self) -> Self::Iterator {
        // TODO: Simplify to ::core::slice::Iter<> if only one archetype has this component.
        SkydomeComponentIter::new([
            &self.archetypes.collection.environment.skydomes,
        ])
    }
}

#[allow(dead_code)]
impl<'a, E, Q> IterSkydomeEntities<'a> for MainWorld<E, Q>
{
    type Iterator = EntityIdIter<'a>;

    /// Iterates all entity IDs of [`Skydome`](SkydomeComponent) components, regardless of archetype.
    fn iter_skydome_entities(&'a self) -> Self::Iterator {
        // TODO: Simplify to ::core::slice::Iter<> if only one archetype has this component.
        EntityIdIter::new([
            &self.archetypes.collection.environment.entities,
        ])
    }
}

#[allow(dead_code)]
impl<'a, E, Q> IterMutSkydomeComponents<'a> for MainWorld<E, Q>
{
    type IteratorMut = SkydomeComponentIterMut<'a>;

    /// Mutably iterates over the [`Skydome`](SkydomeComponent) component of the following archetypes, in that order:
    ///
    /// - [`Environment`](EnvironmentArchetype)
    fn iter_skydomes_mut(&'a mut self) -> Self::IteratorMut {
        // TODO: Simplify to ::core::slice::Iter<> if only one archetype has this component.
        SkydomeComponentIterMut::new([
            &mut self.archetypes.collection.environment.skydomes,
        ])
    }
}

#[allow(dead_code)]
impl<'a, E, Q> IterStarsComponents<'a> for MainWorld<E, Q>
{
    type Iterator = StarsComponentIter<'a>;

    /// Iterates over the [`Stars`](StarsComponent) component of the following archetypes, in that order:
    ///
    /// - [`Environment`](EnvironmentArchetype)
    fn iter_stars(&'a self) -> Self::Iterator {
        // TODO: Simplify to ::core::slice::Iter<> if only one archetype has this component.
        StarsComponentIter::new([
            &self.archetypes.collection.environment.stars,
        ])
    }
}

#[allow(dead_code)]
impl<'a, E, Q> IterStarsEntities<'a> for MainWorld<E, Q>
{
    type Iterator = EntityIdIter<'a>;

    /// Iterates all entity IDs of [`Stars`](StarsComponent) components, regardless of archetype.
    fn iter_stars_entities(&'a self) -> Self::Iterator {
        // TODO: Simplify to ::core::slice::Iter<> if only one archetype has this component.
        EntityIdIter::new([
            &self.archetypes.collection.environment.entities,
        ])
    }
}

#[allow(dead_code)]
impl<'a, E, Q> IterMutStarsComponents<'a> for MainWorld<E, Q>
{
    type IteratorMut = StarsComponentIterMut<'a>;

    /// Mutably iterates over the [`Stars`](StarsComponent) component of the following archetypes, in that order:
    ///
    /// - [`Environment`](EnvironmentArchetype)
    fn iter_stars_mut(&'a mut self) -> Self::IteratorMut {
        // TODO: Simplify to ::core::slice::Iter<> if only one archetype has this component.
        StarsComponentIterMut::new([
            &mut self.archetypes.collection.environment.stars,
        ])
    }
}

#[allow(dead_code)]
impl<'a, E, Q> IterSunComponents<'a> for MainWorld<E, Q>
{
    type Iterator = SunComponentIter<'a>;

    /// Iterates over the [`Sun`](SunComponent) component of the following archetypes, in that order:
    ///
    /// - [`Environment`](EnvironmentArchetype)
    fn iter_suns(&'a self) -> Self::Iterator {
        // TODO: Simplify to ::core::slice::Iter<> if only one archetype has this component.
        SunComponentIter::new([
            &self.archetypes.collection.environment.suns,
        ])
    }
}

#[allow(dead_code)]
impl<'a, E, Q> IterSunEntities<'a> for MainWorld<E, Q>
{
    type Iterator = EntityIdIter<'a>;

    /// Iterates all entity IDs of [`Sun`](SunComponent) components, regardless of archetype.
    fn iter_sun_entities(&'a self) -> Self::Iterator {
        // TODO: Simplify to ::core::slice::Iter<> if only one archetype has this component.
        EntityIdIter::new([
            &self.archetypes.collection.environment.entities,
        ])
    }
}

#[allow(dead_code)]
impl<'a, E, Q> IterMutSunComponents<'a> for MainWorld<E, Q>
{
    type IteratorMut = SunComponentIterMut<'a>;

    /// Mutably iterates over the [`Sun`](SunComponent) component of the following archetypes, in that order:
    ///
    /// - [`Environment`](EnvironmentArchetype)
    fn iter_suns_mut(&'a mut self) -> Self::IteratorMut {
        // TODO: Simplify to ::core::slice::Iter<> if only one archetype has this component.
        SunComponentIterMut::new([
            &mut self.archetypes.collection.environment.suns,
        ])
    }
}

#[allow(dead_code)]
impl<'a, E, Q> IterTextureSourceComponents<'a> for MainWorld<E, Q>
{
    type Iterator = TextureSourceComponentIter<'a>;

    /// Iterates over the [`TextureSource`](TextureSourceComponent) component of the following archetypes, in that order:
    ///
    /// - [`WgpuTexture`](WgpuTextureArchetype)
    fn iter_texture_sources(&'a self) -> Self::Iterator {
        // TODO: Simplify to ::core::slice::Iter<> if only one archetype has this component.
        TextureSourceComponentIter::new([
            &self.archetypes.collection.wgpu_texture.texture_sources,
        ])
    }
}

#[allow(dead_code)]
impl<'a, E, Q> IterTextureSourceEntities<'a> for MainWorld<E, Q>
{
    type Iterator = EntityIdIter<'a>;

    /// Iterates all entity IDs of [`TextureSource`](TextureSourceComponent) components, regardless of archetype.
    fn iter_texture_source_entities(&'a self) -> Self::Iterator {
        // TODO: Simplify to ::core::slice::Iter<> if only one archetype has this component.
        EntityIdIter::new([
            &self.archetypes.collection.wgpu_texture.entities,
        ])
    }
}

#[allow(dead_code)]
impl<'a, E, Q> IterMutTextureSourceComponents<'a> for MainWorld<E, Q>
{
    type IteratorMut = TextureSourceComponentIterMut<'a>;

    /// Mutably iterates over the [`TextureSource`](TextureSourceComponent) component of the following archetypes, in that order:
    ///
    /// - [`WgpuTexture`](WgpuTextureArchetype)
    fn iter_texture_sources_mut(&'a mut self) -> Self::IteratorMut {
        // TODO: Simplify to ::core::slice::Iter<> if only one archetype has this component.
        TextureSourceComponentIterMut::new([
            &mut self.archetypes.collection.wgpu_texture.texture_sources,
        ])
    }
}

#[allow(dead_code)]
impl<'a, E, Q> IterTileComponents<'a> for MainWorld<E, Q>
{
    type Iterator = TileComponentIter<'a>;

    /// Iterates over the [`Tile`](TileComponent) component of the following archetypes, in that order:
    ///
    /// - [`Tile`](TileArchetype)
    fn iter_tiles(&'a self) -> Self::Iterator {
        // TODO: Simplify to ::core::slice::Iter<> if only one archetype has this component.
        TileComponentIter::new([
            &self.archetypes.collection.tile.tiles,
        ])
    }
}

#[allow(dead_code)]
impl<'a, E, Q> IterTileEntities<'a> for MainWorld<E, Q>
{
    type Iterator = EntityIdIter<'a>;

    /// Iterates all entity IDs of [`Tile`](TileComponent) components, regardless of archetype.
    fn iter_tile_entities(&'a self) -> Self::Iterator {
        // TODO: Simplify to ::core::slice::Iter<> if only one archetype has this component.
        EntityIdIter::new([
            &self.archetypes.collection.tile.entities,
        ])
    }
}

#[allow(dead_code)]
impl<'a, E, Q> IterMutTileComponents<'a> for MainWorld<E, Q>
{
    type IteratorMut = TileComponentIterMut<'a>;

    /// Mutably iterates over the [`Tile`](TileComponent) component of the following archetypes, in that order:
    ///
    /// - [`Tile`](TileArchetype)
    fn iter_tiles_mut(&'a mut self) -> Self::IteratorMut {
        // TODO: Simplify to ::core::slice::Iter<> if only one archetype has this component.
        TileComponentIterMut::new([
            &mut self.archetypes.collection.tile.tiles,
        ])
    }
}

#[allow(dead_code)]
impl<'a, E, Q> IterTimeOfDayComponents<'a> for MainWorld<E, Q>
{
    type Iterator = TimeOfDayComponentIter<'a>;

    /// Iterates over the [`TimeOfDay`](TimeOfDayComponent) component of the following archetypes, in that order:
    ///
    /// - [`Environment`](EnvironmentArchetype)
    fn iter_time_of_days(&'a self) -> Self::Iterator {
        // TODO: Simplify to ::core::slice::Iter<> if only one archetype has this component.
        TimeOfDayComponentIter::new([
            &self.archetypes.collection.environment.time_of_days,
        ])
    }
}

#[allow(dead_code)]
impl<'a, E, Q> IterTimeOfDayEntities<'a> for MainWorld<E, Q>
{
    type Iterator = EntityIdIter<'a>;

    /// Iterates all entity IDs of [`TimeOfDay`](TimeOfDayComponent) components, regardless of archetype.
    fn iter_time_of_day_entities(&'a self) -> Self::Iterator {
        // TODO: Simplify to ::core::slice::Iter<> if only one archetype has this component.
        EntityIdIter::new([
            &self.archetypes.collection.environment.entities,
        ])
    }
}

#[allow(dead_code)]
impl<'a, E, Q> IterMutTimeOfDayComponents<'a> for MainWorld<E, Q>
{
    type IteratorMut = TimeOfDayComponentIterMut<'a>;

    /// Mutably iterates over the [`TimeOfDay`](TimeOfDayComponent) component of the following archetypes, in that order:
    ///
    /// - [`Environment`](EnvironmentArchetype)
    fn iter_time_of_days_mut(&'a mut self) -> Self::IteratorMut {
        // TODO: Simplify to ::core::slice::Iter<> if only one archetype has this component.
        TimeOfDayComponentIterMut::new([
            &mut self.archetypes.collection.environment.time_of_days,
        ])
    }
}

#[allow(dead_code)]
impl<'a, E, Q> IterTransformComponents<'a> for MainWorld<E, Q>
{
    type Iterator = TransformComponentIter<'a>;

    /// Iterates over the [`Transform`](TransformComponent) component of the following archetypes, in that order:
    ///
    /// - [`Cube`](CubeArchetype)
    fn iter_transforms(&'a self) -> Self::Iterator {
        // TODO: Simplify to ::core::slice::Iter<> if only one archetype has this component.
        TransformComponentIter::new([
            &self.archetypes.collection.cube.transforms,
        ])
    }
}

#[allow(dead_code)]
impl<'a, E, Q> IterTransformEntities<'a> for MainWorld<E, Q>
{
    type Iterator = EntityIdIter<'a>;

    /// Iterates all entity IDs of [`Transform`](TransformComponent) components, regardless of archetype.
    fn iter_transform_entities(&'a self) -> Self::Iterator {
        // TODO: Simplify to ::core::slice::Iter<> if only one archetype has this component.
        EntityIdIter::new([
            &self.archetypes.collection.cube.entities,
        ])
    }
}

#[allow(dead_code)]
impl<'a, E, Q> IterMutTransformComponents<'a> for MainWorld<E, Q>
{
    type IteratorMut = TransformComponentIterMut<'a>;

    /// Mutably iterates over the [`Transform`](TransformComponent) component of the following archetypes, in that order:
    ///
    /// - [`Cube`](CubeArchetype)
    fn iter_transforms_mut(&'a mut self) -> Self::IteratorMut {
        // TODO: Simplify to ::core::slice::Iter<> if only one archetype has this component.
        TransformComponentIterMut::new([
            &mut self.archetypes.collection.cube.transforms,
        ])
    }
}

#[allow(dead_code)]
impl<'a, E, Q> IterUniformTypeComponents<'a> for MainWorld<E, Q>
{
    type Iterator = UniformTypeComponentIter<'a>;

    /// Iterates over the [`UniformType`](UniformTypeComponent) component of the following archetypes, in that order:
    ///
    /// - [`WgpuUniformBindGroup`](WgpuUniformBindGroupArchetype)
    fn iter_uniform_types(&'a self) -> Self::Iterator {
        // TODO: Simplify to ::core::slice::Iter<> if only one archetype has this component.
        UniformTypeComponentIter::new([
            &self.archetypes.collection.wgpu_uniform_bind_group.uniform_types,
        ])
    }
}

#[allow(dead_code)]
impl<'a, E, Q> IterUniformTypeEntities<'a> for MainWorld<E, Q>
{
    type Iterator = EntityIdIter<'a>;

    /// Iterates all entity IDs of [`UniformType`](UniformTypeComponent) components, regardless of archetype.
    fn iter_uniform_type_entities(&'a self) -> Self::Iterator {
        // TODO: Simplify to ::core::slice::Iter<> if only one archetype has this component.
        EntityIdIter::new([
            &self.archetypes.collection.wgpu_uniform_bind_group.entities,
        ])
    }
}

#[allow(dead_code)]
impl<'a, E, Q> IterMutUniformTypeComponents<'a> for MainWorld<E, Q>
{
    type IteratorMut = UniformTypeComponentIterMut<'a>;

    /// Mutably iterates over the [`UniformType`](UniformTypeComponent) component of the following archetypes, in that order:
    ///
    /// - [`WgpuUniformBindGroup`](WgpuUniformBindGroupArchetype)
    fn iter_uniform_types_mut(&'a mut self) -> Self::IteratorMut {
        // TODO: Simplify to ::core::slice::Iter<> if only one archetype has this component.
        UniformTypeComponentIterMut::new([
            &mut self.archetypes.collection.wgpu_uniform_bind_group.uniform_types,
        ])
    }
}

#[allow(dead_code)]
impl<'a, E, Q> IterVelocityComponents<'a> for MainWorld<E, Q>
{
    type Iterator = VelocityComponentIter<'a>;

    /// Iterates over the [`Velocity`](VelocityComponent) component of the following archetypes, in that order:
    ///
    /// - [`Cube`](CubeArchetype)
    /// - [`Player`](PlayerArchetype)
    fn iter_velocities(&'a self) -> Self::Iterator {
        // TODO: Simplify to ::core::slice::Iter<> if only one archetype has this component.
        VelocityComponentIter::new([
            &self.archetypes.collection.cube.velocities,
            &self.archetypes.collection.player.velocities,
        ])
    }
}

#[allow(dead_code)]
impl<'a, E, Q> IterVelocityEntities<'a> for MainWorld<E, Q>
{
    type Iterator = EntityIdIter<'a>;

    /// Iterates all entity IDs of [`Velocity`](VelocityComponent) components, regardless of archetype.
    fn iter_velocity_entities(&'a self) -> Self::Iterator {
        // TODO: Simplify to ::core::slice::Iter<> if only one archetype has this component.
        EntityIdIter::new([
            &self.archetypes.collection.cube.entities,
            &self.archetypes.collection.player.entities,
        ])
    }
}

#[allow(dead_code)]
impl<'a, E, Q> IterMutVelocityComponents<'a> for MainWorld<E, Q>
{
    type IteratorMut = VelocityComponentIterMut<'a>;

    /// Mutably iterates over the [`Velocity`](VelocityComponent) component of the following archetypes, in that order:
    ///
    /// - [`Cube`](CubeArchetype)
    /// - [`Player`](PlayerArchetype)
    fn iter_velocities_mut(&'a mut self) -> Self::IteratorMut {
        // TODO: Simplify to ::core::slice::Iter<> if only one archetype has this component.
        VelocityComponentIterMut::new([
            &mut self.archetypes.collection.cube.velocities,
            &mut self.archetypes.collection.player.velocities,
        ])
    }
}

#[allow(dead_code)]
impl<'a, E, Q> IterWgpuBindGroupComponents<'a> for MainWorld<E, Q>
{
    type Iterator = WgpuBindGroupComponentIter<'a>;

    /// Iterates over the [`WgpuBindGroup`](WgpuBindGroupComponent) component of the following archetypes, in that order:
    ///
    /// - [`WgpuUniformBindGroup`](WgpuUniformBindGroupArchetype)
    fn iter_wgpu_bind_groups(&'a self) -> Self::Iterator {
        // TODO: Simplify to ::core::slice::Iter<> if only one archetype has this component.
        WgpuBindGroupComponentIter::new([
            &self.archetypes.collection.wgpu_uniform_bind_group.wgpu_bind_groups,
        ])
    }
}

#[allow(dead_code)]
impl<'a, E, Q> IterWgpuBindGroupEntities<'a> for MainWorld<E, Q>
{
    type Iterator = EntityIdIter<'a>;

    /// Iterates all entity IDs of [`WgpuBindGroup`](WgpuBindGroupComponent) components, regardless of archetype.
    fn iter_wgpu_bind_group_entities(&'a self) -> Self::Iterator {
        // TODO: Simplify to ::core::slice::Iter<> if only one archetype has this component.
        EntityIdIter::new([
            &self.archetypes.collection.wgpu_uniform_bind_group.entities,
        ])
    }
}

#[allow(dead_code)]
impl<'a, E, Q> IterMutWgpuBindGroupComponents<'a> for MainWorld<E, Q>
{
    type IteratorMut = WgpuBindGroupComponentIterMut<'a>;

    /// Mutably iterates over the [`WgpuBindGroup`](WgpuBindGroupComponent) component of the following archetypes, in that order:
    ///
    /// - [`WgpuUniformBindGroup`](WgpuUniformBindGroupArchetype)
    fn iter_wgpu_bind_groups_mut(&'a mut self) -> Self::IteratorMut {
        // TODO: Simplify to ::core::slice::Iter<> if only one archetype has this component.
        WgpuBindGroupComponentIterMut::new([
            &mut self.archetypes.collection.wgpu_uniform_bind_group.wgpu_bind_groups,
        ])
    }
}

#[allow(dead_code)]
impl<'a, E, Q> IterWgpuBindGroupLayoutComponents<'a> for MainWorld<E, Q>
{
    type Iterator = WgpuBindGroupLayoutComponentIter<'a>;

    /// Iterates over the [`WgpuBindGroupLayout`](WgpuBindGroupLayoutComponent) component of the following archetypes, in that order:
    ///
    /// - [`WgpuUniformBindGroup`](WgpuUniformBindGroupArchetype)
    fn iter_wgpu_bind_group_layouts(&'a self) -> Self::Iterator {
        // TODO: Simplify to ::core::slice::Iter<> if only one archetype has this component.
        WgpuBindGroupLayoutComponentIter::new([
            &self.archetypes.collection.wgpu_uniform_bind_group.wgpu_bind_group_layouts,
        ])
    }
}

#[allow(dead_code)]
impl<'a, E, Q> IterWgpuBindGroupLayoutEntities<'a> for MainWorld<E, Q>
{
    type Iterator = EntityIdIter<'a>;

    /// Iterates all entity IDs of [`WgpuBindGroupLayout`](WgpuBindGroupLayoutComponent) components, regardless of archetype.
    fn iter_wgpu_bind_group_layout_entities(&'a self) -> Self::Iterator {
        // TODO: Simplify to ::core::slice::Iter<> if only one archetype has this component.
        EntityIdIter::new([
            &self.archetypes.collection.wgpu_uniform_bind_group.entities,
        ])
    }
}

#[allow(dead_code)]
impl<'a, E, Q> IterMutWgpuBindGroupLayoutComponents<'a> for MainWorld<E, Q>
{
    type IteratorMut = WgpuBindGroupLayoutComponentIterMut<'a>;

    /// Mutably iterates over the [`WgpuBindGroupLayout`](WgpuBindGroupLayoutComponent) component of the following archetypes, in that order:
    ///
    /// - [`WgpuUniformBindGroup`](WgpuUniformBindGroupArchetype)
    fn iter_wgpu_bind_group_layouts_mut(&'a mut self) -> Self::IteratorMut {
        // TODO: Simplify to ::core::slice::Iter<> if only one archetype has this component.
        WgpuBindGroupLayoutComponentIterMut::new([
            &mut self.archetypes.collection.wgpu_uniform_bind_group.wgpu_bind_group_layouts,
        ])
    }
}

#[allow(dead_code)]
impl<'a, E, Q> IterWgpuBufferComponents<'a> for MainWorld<E, Q>
{
    type Iterator = WgpuBufferComponentIter<'a>;

    /// Iterates over the [`WgpuBuffer`](WgpuBufferComponent) component of the following archetypes, in that order:
    ///
    /// - [`WgpuUniformBindGroup`](WgpuUniformBindGroupArchetype)
    fn iter_wgpu_buffers(&'a self) -> Self::Iterator {
        // TODO: Simplify to ::core::slice::Iter<> if only one archetype has this component.
        WgpuBufferComponentIter::new([
            &self.archetypes.collection.wgpu_uniform_bind_group.wgpu_buffers,
        ])
    }
}

#[allow(dead_code)]
impl<'a, E, Q> IterWgpuBufferEntities<'a> for MainWorld<E, Q>
{
    type Iterator = EntityIdIter<'a>;

    /// Iterates all entity IDs of [`WgpuBuffer`](WgpuBufferComponent) components, regardless of archetype.
    fn iter_wgpu_buffer_entities(&'a self) -> Self::Iterator {
        // TODO: Simplify to ::core::slice::Iter<> if only one archetype has this component.
        EntityIdIter::new([
            &self.archetypes.collection.wgpu_uniform_bind_group.entities,
        ])
    }
}

#[allow(dead_code)]
impl<'a, E, Q> IterMutWgpuBufferComponents<'a> for MainWorld<E, Q>
{
    type IteratorMut = WgpuBufferComponentIterMut<'a>;

    /// Mutably iterates over the [`WgpuBuffer`](WgpuBufferComponent) component of the following archetypes, in that order:
    ///
    /// - [`WgpuUniformBindGroup`](WgpuUniformBindGroupArchetype)
    fn iter_wgpu_buffers_mut(&'a mut self) -> Self::IteratorMut {
        // TODO: Simplify to ::core::slice::Iter<> if only one archetype has this component.
        WgpuBufferComponentIterMut::new([
            &mut self.archetypes.collection.wgpu_uniform_bind_group.wgpu_buffers,
        ])
    }
}

#[allow(dead_code)]
impl<'a, E, Q> IterWgpuShaderComponents<'a> for MainWorld<E, Q>
{
    type Iterator = WgpuShaderComponentIter<'a>;

    /// Iterates over the [`WgpuShader`](WgpuShaderComponent) component of the following archetypes, in that order:
    ///
    /// - [`WgpuShader`](WgpuShaderArchetype)
    fn iter_wgpu_shaders(&'a self) -> Self::Iterator {
        // TODO: Simplify to ::core::slice::Iter<> if only one archetype has this component.
        WgpuShaderComponentIter::new([
            &self.archetypes.collection.wgpu_shader.wgpu_shaders,
        ])
    }
}

#[allow(dead_code)]
impl<'a, E, Q> IterWgpuShaderEntities<'a> for MainWorld<E, Q>
{
    type Iterator = EntityIdIter<'a>;

    /// Iterates all entity IDs of [`WgpuShader`](WgpuShaderComponent) components, regardless of archetype.
    fn iter_wgpu_shader_entities(&'a self) -> Self::Iterator {
        // TODO: Simplify to ::core::slice::Iter<> if only one archetype has this component.
        EntityIdIter::new([
            &self.archetypes.collection.wgpu_shader.entities,
        ])
    }
}

#[allow(dead_code)]
impl<'a, E, Q> IterMutWgpuShaderComponents<'a> for MainWorld<E, Q>
{
    type IteratorMut = WgpuShaderComponentIterMut<'a>;

    /// Mutably iterates over the [`WgpuShader`](WgpuShaderComponent) component of the following archetypes, in that order:
    ///
    /// - [`WgpuShader`](WgpuShaderArchetype)
    fn iter_wgpu_shaders_mut(&'a mut self) -> Self::IteratorMut {
        // TODO: Simplify to ::core::slice::Iter<> if only one archetype has this component.
        WgpuShaderComponentIterMut::new([
            &mut self.archetypes.collection.wgpu_shader.wgpu_shaders,
        ])
    }
}

#[allow(dead_code)]
impl<'a, E, Q> IterWgpuTextureComponents<'a> for MainWorld<E, Q>
{
    type Iterator = WgpuTextureComponentIter<'a>;

    /// Iterates over the [`WgpuTexture`](WgpuTextureComponent) component of the following archetypes, in that order:
    ///
    /// - [`WgpuTexture`](WgpuTextureArchetype)
    fn iter_wgpu_textures(&'a self) -> Self::Iterator {
        // TODO: Simplify to ::core::slice::Iter<> if only one archetype has this component.
        WgpuTextureComponentIter::new([
            &self.archetypes.collection.wgpu_texture.wgpu_textures,
        ])
    }
}

#[allow(dead_code)]
impl<'a, E, Q> IterWgpuTextureEntities<'a> for MainWorld<E, Q>
{
    type Iterator = EntityIdIter<'a>;

    /// Iterates all entity IDs of [`WgpuTexture`](WgpuTextureComponent) components, regardless of archetype.
    fn iter_wgpu_texture_entities(&'a self) -> Self::Iterator {
        // TODO: Simplify to ::core::slice::Iter<> if only one archetype has this component.
        EntityIdIter::new([
            &self.archetypes.collection.wgpu_texture.entities,
        ])
    }
}

#[allow(dead_code)]
impl<'a, E, Q> IterMutWgpuTextureComponents<'a> for MainWorld<E, Q>
{
    type IteratorMut = WgpuTextureComponentIterMut<'a>;

    /// Mutably iterates over the [`WgpuTexture`](WgpuTextureComponent) component of the following archetypes, in that order:
    ///
    /// - [`WgpuTexture`](WgpuTextureArchetype)
    fn iter_wgpu_textures_mut(&'a mut self) -> Self::IteratorMut {
        // TODO: Simplify to ::core::slice::Iter<> if only one archetype has this component.
        WgpuTextureComponentIterMut::new([
            &mut self.archetypes.collection.wgpu_texture.wgpu_textures,
        ])
    }
}