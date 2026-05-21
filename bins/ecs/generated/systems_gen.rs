/// The ID of a [`System`].
#[derive(Debug, Copy, Clone, Ord, PartialOrd, Eq, PartialEq)]
#[repr(u32)]
pub enum SystemId {
    /// The [`Camera`](CameraSystem) system.
    ///
    /// ### Mutates
    /// 
    /// - [`CameraComponent`]
    /// - [`CameraFpsControllerComponent`]
    ///
    /// # Archetype Use
    /// This component processes components in the following archetypes:
    /// 
    /// - [`CameraArchetype`] ([`ArchetypeId::Camera`])
    Camera = 1,
    /// The [`TimeOfDay`](TimeOfDaySystem) system.
    ///
    /// ### Mutates
    /// 
    /// - [`TimeOfDayComponent`]
    ///
    /// # Archetype Use
    /// This component processes components in the following archetypes:
    /// 
    /// - [`EnvironmentArchetype`] ([`ArchetypeId::Environment`])
    TimeOfDay = 2,
    /// The [`Sun`](SunSystem) system.
    ///
    /// ### Reads
    /// 
    /// - [`TimeOfDayComponent`]
    ///
    /// ### Mutates
    /// 
    /// - [`SunComponent`]
    /// - [`SkydomeComponent`]
    /// - [`AmbientLightComponent`]
    ///
    /// # Archetype Use
    /// This component processes components in the following archetypes:
    /// 
    /// - [`EnvironmentArchetype`] ([`ArchetypeId::Environment`])
    Sun = 3,
    /// The [`Stars`](StarsSystem) system.
    ///
    /// ### Reads
    /// 
    /// - [`TimeOfDayComponent`]
    ///
    /// ### Mutates
    /// 
    /// - [`StarsComponent`]
    ///
    /// # Archetype Use
    /// This component processes components in the following archetypes:
    /// 
    /// - [`EnvironmentArchetype`] ([`ArchetypeId::Environment`])
    Stars = 4,
    /// The [`Physics`](PhysicsSystem) system.
    ///
    /// ### Reads
    /// 
    /// - [`VelocityComponent`]
    ///
    /// ### Mutates
    /// 
    /// - [`PositionComponent`]
    ///
    /// # Archetype Use
    /// This component processes components in the following archetypes:
    /// 
    /// - [`CubeArchetype`] ([`ArchetypeId::Cube`])
    /// - [`PlayerArchetype`] ([`ArchetypeId::Player`])
    Physics = 5,
    /// The [`BackgroundColorCycle`](BackgroundColorCycleSystem) system.
    ///
    /// ### Mutates
    /// 
    /// - [`BackgroundColorComponent`]
    ///
    /// # Archetype Use
    /// This component processes components in the following archetypes:
    /// 
    /// - [`SceneBackgroundArchetype`] ([`ArchetypeId::SceneBackground`])
    BackgroundColorCycle = 6,
    /// The [`CameraRender`](CameraRenderSystem) system.
    ///
    /// ### Reads
    /// 
    /// - [`CameraComponent`]
    ///
    /// # Archetype Use
    /// This component processes components in the following archetypes:
    /// 
    /// - [`CameraArchetype`] ([`ArchetypeId::Camera`])
    CameraRender = 7,
    /// The [`FrameGlobals`](FrameGlobalsSystem) system.
    ///
    /// ### Reads
    /// 
    /// - [`TimeOfDayComponent`]
    ///
    /// ### Mutates
    /// 
    /// - [`SunComponent`]
    /// - [`AmbientLightComponent`]
    /// - [`StarsComponent`]
    /// - [`SkydomeComponent`]
    ///
    /// # Archetype Use
    /// This component processes components in the following archetypes:
    /// 
    /// - [`EnvironmentArchetype`] ([`ArchetypeId::Environment`])
    FrameGlobals = 8,
    /// The [`RenderBackground`](RenderBackgroundSystem) system.
    ///
    /// ### Reads
    /// 
    /// - [`BackgroundColorComponent`]
    ///
    /// # Archetype Use
    /// This component processes components in the following archetypes:
    /// 
    /// - [`SceneBackgroundArchetype`] ([`ArchetypeId::SceneBackground`])
    RenderBackground = 9,
    /// The [`RenderSky`](RenderSkySystem) system.
    ///
    /// ### Reads
    /// 
    /// - [`SkydomeComponent`]
    ///
    /// # Archetype Use
    /// This component processes components in the following archetypes:
    /// 
    /// - [`EnvironmentArchetype`] ([`ArchetypeId::Environment`])
    RenderSky = 10,
    /// The [`Render`](RenderSystem) system.
    ///
    /// ### Reads
    /// 
    /// - [`PositionComponent`]
    /// - [`TransformComponent`]
    /// - [`ModelComponent`]
    ///
    /// # Archetype Use
    /// This component processes components in the following archetypes:
    /// 
    /// - [`CubeArchetype`] ([`ArchetypeId::Cube`])
    Render = 11,
    /// The [`TileRender`](TileRenderSystem) system.
    ///
    /// ### Reads
    /// 
    /// - [`TileComponent`]
    /// - [`PositionComponent`]
    ///
    /// # Archetype Use
    /// This component processes components in the following archetypes:
    /// 
    /// - [`TileArchetype`] ([`ArchetypeId::Tile`])
    TileRender = 12,
    /// The [`DynamicLight`](DynamicLightSystem) system.
    ///
    /// ### Reads
    /// 
    /// - [`ColorComponent`]
    ///
    /// ### Mutates
    /// 
    /// - [`DynamicLightComponent`]
    ///
    /// # Archetype Use
    /// This component processes components in the following archetypes:
    /// 
    /// - [`DynamicLightArchetype`] ([`ArchetypeId::DynamicLight`])
    DynamicLight = 13,
    /// If retro rendering is enabled, this system blits the VGA offscreen buffer to the screen
    ///
    /// For details, see the [`VgaBlitSystem`](VgaBlitSystem) Struct.
    ///
    /// ### Reads
    /// 
    /// - [`BackgroundColorComponent`]
    ///
    /// # Archetype Use
    /// This component processes components in the following archetypes:
    /// 
    /// - [`SceneBackgroundArchetype`] ([`ArchetypeId::SceneBackground`])
    VgaBlit = 14,
}

impl SystemId {
    /// A [`NonZeroU64`](core::num::NonZeroU64) representation of [`SystemId::Camera`](SystemId::Camera) system.
    pub const CAMERA_ID_VALUE: core::num::NonZeroU64 = core::num::NonZeroU64::new(1).expect("Error at ECS construction time");
    /// A [`NonZeroU64`](core::num::NonZeroU64) representation of [`SystemId::TimeOfDay`](SystemId::TimeOfDay) system.
    pub const TIME_OF_DAY_ID_VALUE: core::num::NonZeroU64 = core::num::NonZeroU64::new(2).expect("Error at ECS construction time");
    /// A [`NonZeroU64`](core::num::NonZeroU64) representation of [`SystemId::Sun`](SystemId::Sun) system.
    pub const SUN_ID_VALUE: core::num::NonZeroU64 = core::num::NonZeroU64::new(3).expect("Error at ECS construction time");
    /// A [`NonZeroU64`](core::num::NonZeroU64) representation of [`SystemId::Stars`](SystemId::Stars) system.
    pub const STARS_ID_VALUE: core::num::NonZeroU64 = core::num::NonZeroU64::new(4).expect("Error at ECS construction time");
    /// A [`NonZeroU64`](core::num::NonZeroU64) representation of [`SystemId::Physics`](SystemId::Physics) system.
    pub const PHYSICS_ID_VALUE: core::num::NonZeroU64 = core::num::NonZeroU64::new(5).expect("Error at ECS construction time");
    /// A [`NonZeroU64`](core::num::NonZeroU64) representation of [`SystemId::BackgroundColorCycle`](SystemId::BackgroundColorCycle) system.
    pub const BACKGROUND_COLOR_CYCLE_ID_VALUE: core::num::NonZeroU64 = core::num::NonZeroU64::new(6).expect("Error at ECS construction time");
    /// A [`NonZeroU64`](core::num::NonZeroU64) representation of [`SystemId::CameraRender`](SystemId::CameraRender) system.
    pub const CAMERA_RENDER_ID_VALUE: core::num::NonZeroU64 = core::num::NonZeroU64::new(7).expect("Error at ECS construction time");
    /// A [`NonZeroU64`](core::num::NonZeroU64) representation of [`SystemId::FrameGlobals`](SystemId::FrameGlobals) system.
    pub const FRAME_GLOBALS_ID_VALUE: core::num::NonZeroU64 = core::num::NonZeroU64::new(8).expect("Error at ECS construction time");
    /// A [`NonZeroU64`](core::num::NonZeroU64) representation of [`SystemId::RenderBackground`](SystemId::RenderBackground) system.
    pub const RENDER_BACKGROUND_ID_VALUE: core::num::NonZeroU64 = core::num::NonZeroU64::new(9).expect("Error at ECS construction time");
    /// A [`NonZeroU64`](core::num::NonZeroU64) representation of [`SystemId::RenderSky`](SystemId::RenderSky) system.
    pub const RENDER_SKY_ID_VALUE: core::num::NonZeroU64 = core::num::NonZeroU64::new(10).expect("Error at ECS construction time");
    /// A [`NonZeroU64`](core::num::NonZeroU64) representation of [`SystemId::Render`](SystemId::Render) system.
    pub const RENDER_ID_VALUE: core::num::NonZeroU64 = core::num::NonZeroU64::new(11).expect("Error at ECS construction time");
    /// A [`NonZeroU64`](core::num::NonZeroU64) representation of [`SystemId::TileRender`](SystemId::TileRender) system.
    pub const TILE_RENDER_ID_VALUE: core::num::NonZeroU64 = core::num::NonZeroU64::new(12).expect("Error at ECS construction time");
    /// A [`NonZeroU64`](core::num::NonZeroU64) representation of [`SystemId::DynamicLight`](SystemId::DynamicLight) system.
    pub const DYNAMIC_LIGHT_ID_VALUE: core::num::NonZeroU64 = core::num::NonZeroU64::new(13).expect("Error at ECS construction time");
    /// A [`NonZeroU64`](core::num::NonZeroU64) representation of [`SystemId::VgaBlit`](SystemId::VgaBlit) system.
    pub const VGA_BLIT_ID_VALUE: core::num::NonZeroU64 = core::num::NonZeroU64::new(14).expect("Error at ECS construction time");
}

impl SystemId {
    /// Returns this ID as a [`NonZeroU64`](core::num::NonZeroU64) value.
    pub const fn as_nonzero_u64(&self) -> core::num::NonZeroU64 {
        match self {
            Self::Camera => Self::CAMERA_ID_VALUE,
            Self::TimeOfDay => Self::TIME_OF_DAY_ID_VALUE,
            Self::Sun => Self::SUN_ID_VALUE,
            Self::Stars => Self::STARS_ID_VALUE,
            Self::Physics => Self::PHYSICS_ID_VALUE,
            Self::BackgroundColorCycle => Self::BACKGROUND_COLOR_CYCLE_ID_VALUE,
            Self::CameraRender => Self::CAMERA_RENDER_ID_VALUE,
            Self::FrameGlobals => Self::FRAME_GLOBALS_ID_VALUE,
            Self::RenderBackground => Self::RENDER_BACKGROUND_ID_VALUE,
            Self::RenderSky => Self::RENDER_SKY_ID_VALUE,
            Self::Render => Self::RENDER_ID_VALUE,
            Self::TileRender => Self::TILE_RENDER_ID_VALUE,
            Self::DynamicLight => Self::DYNAMIC_LIGHT_ID_VALUE,
            Self::VgaBlit => Self::VGA_BLIT_ID_VALUE,
        }
    }

    /// Returns this ID as a `u64` value.
    pub const fn as_u64(&self) -> u64 {
        self.as_nonzero_u64().get()
    }
}

impl core::hash::Hash for SystemId {
    fn hash<H: core::hash::Hasher>(&self, state: &mut H) {
        self.as_u64().hash(state);
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

impl core::fmt::Display for SystemId {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> Result<(), core::fmt::Error> {
        match self {
            Self::Camera => write!(f, "Camera (ID 1)"),
            Self::TimeOfDay => write!(f, "TimeOfDay (ID 2)"),
            Self::Sun => write!(f, "Sun (ID 3)"),
            Self::Stars => write!(f, "Stars (ID 4)"),
            Self::Physics => write!(f, "Physics (ID 5)"),
            Self::BackgroundColorCycle => write!(f, "BackgroundColorCycle (ID 6)"),
            Self::CameraRender => write!(f, "CameraRender (ID 7)"),
            Self::FrameGlobals => write!(f, "FrameGlobals (ID 8)"),
            Self::RenderBackground => write!(f, "RenderBackground (ID 9)"),
            Self::RenderSky => write!(f, "RenderSky (ID 10)"),
            Self::Render => write!(f, "Render (ID 11)"),
            Self::TileRender => write!(f, "TileRender (ID 12)"),
            Self::DynamicLight => write!(f, "DynamicLight (ID 13)"),
            Self::VgaBlit => write!(f, "VgaBlit (ID 14)"),
        }
    }
}

/// Marker trait for systems.
pub trait System: 'static + Send + Sync {
    /// The ID of this system.
    const ID: SystemId;

    /// The ID of this system.
    #[inline]
    #[allow(dead_code)]
    fn id(&self) -> SystemId {
        Self::ID
    }
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
pub trait CreateSystem<S: System> {
    /// Create an instance of type `S`.
    fn create(&self) -> S;
}

/// Trait for constructing system instances.
pub trait CreateSystems: CreateSystem<CameraSystem> + CreateSystem<TimeOfDaySystem> + CreateSystem<SunSystem> + CreateSystem<StarsSystem> + CreateSystem<PhysicsSystem> + CreateSystem<BackgroundColorCycleSystem> + CreateSystem<CameraRenderSystem> + CreateSystem<FrameGlobalsSystem> + CreateSystem<RenderBackgroundSystem> + CreateSystem<RenderSkySystem> + CreateSystem<RenderSystem> + CreateSystem<TileRenderSystem> + CreateSystem<DynamicLightSystem> + CreateSystem<VgaBlitSystem>
{ }

#[automatically_derived]
impl<T> CreateSystems for T
where
    T: CreateSystem<CameraSystem>
     + CreateSystem<TimeOfDaySystem>
     + CreateSystem<SunSystem>
     + CreateSystem<StarsSystem>
     + CreateSystem<PhysicsSystem>
     + CreateSystem<BackgroundColorCycleSystem>
     + CreateSystem<CameraRenderSystem>
     + CreateSystem<FrameGlobalsSystem>
     + CreateSystem<RenderBackgroundSystem>
     + CreateSystem<RenderSkySystem>
     + CreateSystem<RenderSystem>
     + CreateSystem<TileRenderSystem>
     + CreateSystem<DynamicLightSystem>
     + CreateSystem<VgaBlitSystem>
{
}

/// Trait for constructing system instances.
pub trait SystemPhaseEvents: SystemStartupPhaseEvents + SystemFixedUpdatePhaseEvents + SystemUpdatePhaseEvents + SystemRenderPhaseEvents
{
}

#[automatically_derived]
impl<T> SystemPhaseEvents for T
where
    T: SystemStartupPhaseEvents
     + SystemFixedUpdatePhaseEvents
     + SystemUpdatePhaseEvents
     + SystemRenderPhaseEvents
{
}

/// A no-op event struct.
pub struct NoOpPhaseEvents;

impl SystemStartupPhaseEvents for NoOpPhaseEvents {
    type Result = ();
    type Error = core::convert::Infallible;

    #[inline]
    fn on_begin_phase(
        &mut self,
        _context: &::sillyecs::FrameContext,
    ) -> Result<(), Self::Error> {
        Ok(())
    }

    #[inline]
    fn on_end_phase(
        &mut self,
        _result: Option<Self::Result>,
        _context: &::sillyecs::FrameContext,
    ) -> Result<(), Self::Error> {
        Ok(())
    }
}

pub trait SystemStartupPhaseEvents {
    type Result;
    type Error: core::error::Error;

    /// The name of the phase.
    const NAME: &'static str = "Startup";

    /// Executed at the beginning of the phase (before all systems).
    fn on_begin_phase(
        &mut self,
        context: &::sillyecs::FrameContext,
    ) -> Result<Self::Result, Self::Error>;

    /// Executed at the end of the phase (after all systems).
    ///
    /// The result from [`SystemStartupPhaseEvents::on_begin_phase`] is passed in
    /// as [`Some`] value, if the executed succeeded. If the function failed, the value will be [`None`].
    fn on_end_phase(
        &mut self,
        result: Option<Self::Result>,
        context: &::sillyecs::FrameContext,
    ) -> Result<(), Self::Error>;
}

impl SystemFixedUpdatePhaseEvents for NoOpPhaseEvents {
    type Result = ();
    type Error = core::convert::Infallible;

    #[inline]
    fn on_begin_phase(
        &mut self,
        _context: &::sillyecs::FrameContext,
    ) -> Result<(), Self::Error> {
        Ok(())
    }

    #[inline]
    fn on_end_phase(
        &mut self,
        _result: Option<Self::Result>,
        _context: &::sillyecs::FrameContext,
    ) -> Result<(), Self::Error> {
        Ok(())
    }
}

pub trait SystemFixedUpdatePhaseEvents {
    type Result;
    type Error: core::error::Error;

    /// The name of the phase.
    const NAME: &'static str = "FixedUpdate";

    /// Executed at the beginning of the phase (before all systems).
    fn on_begin_phase(
        &mut self,
        context: &::sillyecs::FrameContext,
    ) -> Result<Self::Result, Self::Error>;

    /// Executed at the end of the phase (after all systems).
    ///
    /// The result from [`SystemFixedUpdatePhaseEvents::on_begin_phase`] is passed in
    /// as [`Some`] value, if the executed succeeded. If the function failed, the value will be [`None`].
    fn on_end_phase(
        &mut self,
        result: Option<Self::Result>,
        context: &::sillyecs::FrameContext,
    ) -> Result<(), Self::Error>;
}

impl SystemUpdatePhaseEvents for NoOpPhaseEvents {
    type Result = ();
    type Error = core::convert::Infallible;

    #[inline]
    fn on_begin_phase(
        &mut self,
        _context: &::sillyecs::FrameContext,
    ) -> Result<(), Self::Error> {
        Ok(())
    }

    #[inline]
    fn on_end_phase(
        &mut self,
        _result: Option<Self::Result>,
        _context: &::sillyecs::FrameContext,
    ) -> Result<(), Self::Error> {
        Ok(())
    }
}

pub trait SystemUpdatePhaseEvents {
    type Result;
    type Error: core::error::Error;

    /// The name of the phase.
    const NAME: &'static str = "Update";

    /// Executed at the beginning of the phase (before all systems).
    fn on_begin_phase(
        &mut self,
        context: &::sillyecs::FrameContext,
    ) -> Result<Self::Result, Self::Error>;

    /// Executed at the end of the phase (after all systems).
    ///
    /// The result from [`SystemUpdatePhaseEvents::on_begin_phase`] is passed in
    /// as [`Some`] value, if the executed succeeded. If the function failed, the value will be [`None`].
    fn on_end_phase(
        &mut self,
        result: Option<Self::Result>,
        context: &::sillyecs::FrameContext,
    ) -> Result<(), Self::Error>;
}

impl SystemRenderPhaseEvents for NoOpPhaseEvents {
    type Result = ();
    type Error = core::convert::Infallible;

    #[inline]
    fn on_begin_phase(
        &mut self,
        _context: &::sillyecs::FrameContext,
                _wgpu_render: &mut WgpuRenderState,
    ) -> Result<(), Self::Error> {
        Ok(())
    }

    #[inline]
    fn on_end_phase(
        &mut self,
        _result: Option<Self::Result>,
        _context: &::sillyecs::FrameContext,
                _wgpu_render: &mut WgpuRenderState,
    ) -> Result<(), Self::Error> {
        Ok(())
    }
}

pub trait SystemRenderPhaseEvents {
    type Result;
    type Error: core::error::Error;

    /// The name of the phase.
    const NAME: &'static str = "Render";

    /// Executed at the beginning of the phase (before all systems).
    fn on_begin_phase(
        &mut self,
        context: &::sillyecs::FrameContext,
                wgpu_render: &mut WgpuRenderState,
    ) -> Result<Self::Result, Self::Error>;

    /// Executed at the end of the phase (after all systems).
    ///
    /// The result from [`SystemRenderPhaseEvents::on_begin_phase`] is passed in
    /// as [`Some`] value, if the executed succeeded. If the function failed, the value will be [`None`].
    fn on_end_phase(
        &mut self,
        result: Option<Self::Result>,
        context: &::sillyecs::FrameContext,
                wgpu_render: &mut WgpuRenderState,
    ) -> Result<(), Self::Error>;
}

/// A system operating on multiple [`Component`]s.
///
/// The business logic is implemented via the [`ApplyCameraSystem`] trait.
/// See also [`CameraData`] for the systems-specific data.
///
/// ### Mutates
/// 
/// - [`CameraComponent`]
/// - [`CameraFpsControllerComponent`]
///
/// # Archetype Use
/// This component processes components in the following archetypes:
/// 
/// - [`CameraArchetype`] ([`ArchetypeId::Camera`])
#[derive(Debug)]
pub struct CameraSystem(CameraSystemData);

/// Implements the business logic for the [`CameraSystem`] system.
#[allow(dead_code)]
pub trait ApplyCameraSystem: System {
    type Error: core::error::Error;

    /// Indicates whether this system is ready for action.
    ///
    /// Always called once per phase before all other functions of this system. If it returns
    /// `true`, the remaining functions are executed, otherwise the system skips the phase.
    #[inline]
    #[allow(unused_variables)]
    fn is_ready(
        &self,
                input: &InputState,
                wgpu_render: &WgpuRenderState,
    ) -> bool {
        true
    }

    /// Executed at the beginning of the phase (for this system).
    ///
    /// All calls to [`apply_many`](Self::apply_many) and/or [`apply_single`](Self::apply_single) happen after this
    /// function was executed.
    #[inline]
    #[allow(unused_variables)]
    fn on_begin_phase(
        &mut self,
                input: &InputState,
                wgpu_render: &WgpuRenderState,
    ) -> Result<(), Self::Error> {
        Ok(())
    }

    /// Executed at the end of the phase (for this system).
    ///
    /// All calls to [`apply_many`](Self::apply_many) and/or [`apply_single`](Self::apply_single) happen before this
    /// function is executed.
    #[inline]
    #[allow(unused_variables)]
    fn on_end_phase(
        &mut self,
                input: &InputState,
                wgpu_render: &WgpuRenderState,
    ) -> Result<(), Self::Error> {
        Ok(())
    }

    /// Applies the system's business logic to the given entity components.
    ///
    /// ### Performance Considerations
    /// Prefer implementing [`apply_many`](Self::apply_many) directly.
    ///
    /// ### Reads
    /// 
    ///
    /// ### Mutates
    /// 
    /// - `camera`: A mutable reference to the component of type [`CameraComponent`].
    /// - `camera_fps_controller`: A mutable reference to the component of type [`CameraFpsControllerComponent`].
    #[allow(unused_variables)]
    fn apply_single(
        &mut self,
                input: &InputState,
                wgpu_render: &WgpuRenderState,
        camera: &mut CameraComponent,
        camera_fps_controller: &mut CameraFpsControllerComponent,
    ) {
         // intentionally left blank
     }

    /// Applies the system's business logic to the given entity components.
    ///
    /// ### Reads
    /// 
    ///
    /// ### Mutates
    /// 
    /// - `camera`: A mutable slice of the components of type [`CameraComponent`].
    /// - `camera_fps_controller`: A mutable slice of the components of type [`CameraFpsControllerComponent`].
    fn apply_many(
        &mut self,
                input: &InputState,
                wgpu_render: &WgpuRenderState,
        cameras: &mut [CameraComponent],
        camera_fps_controllers: &mut [CameraFpsControllerComponent],
    ) {
        let zipped_iter = cameras.iter_mut().zip(camera_fps_controllers.iter_mut());
        for (camera, camera_fps_controller) in zipped_iter {
            self.apply_single(
                        input,
                        wgpu_render,
                camera,
                camera_fps_controller,
            );
        }
    }

    /// Applies the system's business logic to the given entity components.
    ///
    /// ### Reads
    /// 
    ///
    /// ### Mutates
    /// 
    /// - `camera`: A mutable slice of the components of type [`CameraComponent`].
    /// - `camera_fps_controller`: A mutable slice of the components of type [`CameraFpsControllerComponent`].
    #[allow(unused_mut)]
    #[inline]
    fn apply_all(
        &mut self,
                input: &InputState,
                wgpu_render: &WgpuRenderState,
        mut cameras: [&mut [CameraComponent]; 1],
        mut camera_fps_controllers: [&mut [CameraFpsControllerComponent]; 1],
    ) {
        let zipped_iter = cameras.iter_mut().zip(camera_fps_controllers.iter_mut());
        for (camera, camera_fps_controller) in zipped_iter {
            ApplyCameraSystem::apply_many(
                self,
                        input,
                        wgpu_render,
                camera,
                camera_fps_controller,
            );
        }
    }
}

#[allow(dead_code)]
impl CameraSystem {
    /// Applies the system's business logic to the given entity components.
    ///
    /// ### Reads
    /// 
    ///
    /// ### Mutates
    /// 
    /// - `camera`: A mutable slice of the components of type [`CameraComponent`].
    /// - `camera_fps_controller`: A mutable slice of the components of type [`CameraFpsControllerComponent`].
    #[inline(always)]
    fn apply_many(
        &mut self,
        input: &InputState,
        wgpu_render: &WgpuRenderState,
        cameras: &mut [CameraComponent],
        camera_fps_controllers: &mut [CameraFpsControllerComponent],
    ) {
        // Force the implementation of the ApplyCameraSystem trait.
        ApplyCameraSystem::apply_many(
            self,
            input,
            wgpu_render,
            cameras,
            camera_fps_controllers,
        );
    }

    /// Applies the system's business logic to the given entity components.
    ///
    /// ### Reads
    /// 
    ///
    /// ### Mutates
    /// 
    /// - `camera`: A mutable slice of the components of type [`CameraComponent`].
    /// - `camera_fps_controller`: A mutable slice of the components of type [`CameraFpsControllerComponent`].
    #[allow(unused_mut)]
    #[inline]
    fn apply_all(
        &mut self,
        input: &InputState,
        wgpu_render: &WgpuRenderState,
        mut cameras: [&mut [CameraComponent]; 1],
        mut camera_fps_controllers: [&mut [CameraFpsControllerComponent]; 1],
    ) {
        let zipped_iter = cameras.iter_mut().zip(camera_fps_controllers.iter_mut());
        for (camera, camera_fps_controller) in zipped_iter {
            self.apply_many(
                input,
                wgpu_render,
                camera,
                camera_fps_controller,
            );
        }
    }
}

#[automatically_derived]
impl System for CameraSystem {
    const ID: SystemId = SystemId::Camera;
}

impl PartialEq<SystemId> for CameraSystem {
    fn eq(&self, other: &SystemId) -> bool {
        Self::ID.eq(other)
    }
}

#[automatically_derived]
impl From<CameraSystemData> for CameraSystem {
    fn from(data: CameraSystemData) -> Self {
        Self(data)
    }
}

#[automatically_derived]
impl core::ops::Deref for CameraSystem {
    type Target = CameraSystemData;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

#[automatically_derived]
impl core::ops::DerefMut for CameraSystem {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

/// A system operating on multiple [`Component`]s.
///
/// The business logic is implemented via the [`ApplyTimeOfDaySystem`] trait.
/// See also [`TimeOfDayData`] for the systems-specific data.
///
/// ### Mutates
/// 
/// - [`TimeOfDayComponent`]
///
/// # Archetype Use
/// This component processes components in the following archetypes:
/// 
/// - [`EnvironmentArchetype`] ([`ArchetypeId::Environment`])
#[derive(Debug)]
pub struct TimeOfDaySystem(TimeOfDaySystemData);

/// Implements the business logic for the [`TimeOfDaySystem`] system.
#[allow(dead_code)]
pub trait ApplyTimeOfDaySystem: System {
    type Error: core::error::Error;

    /// Indicates whether this system is ready for action.
    ///
    /// Always called once per phase before all other functions of this system. If it returns
    /// `true`, the remaining functions are executed, otherwise the system skips the phase.
    #[inline]
    #[allow(unused_variables)]
    fn is_ready(
        &self,
        context: &::sillyecs::FrameContext,
    ) -> bool {
        true
    }

    /// Executed at the beginning of the phase (for this system).
    ///
    /// All calls to [`apply_many`](Self::apply_many) and/or [`apply_single`](Self::apply_single) happen after this
    /// function was executed.
    #[inline]
    #[allow(unused_variables)]
    fn on_begin_phase(
        &mut self,
        context: &::sillyecs::FrameContext,
    ) -> Result<(), Self::Error> {
        Ok(())
    }

    /// Executed at the end of the phase (for this system).
    ///
    /// All calls to [`apply_many`](Self::apply_many) and/or [`apply_single`](Self::apply_single) happen before this
    /// function is executed.
    #[inline]
    #[allow(unused_variables)]
    fn on_end_phase(
        &mut self,
        context: &::sillyecs::FrameContext,
    ) -> Result<(), Self::Error> {
        Ok(())
    }

    /// Applies the system's business logic to the given entity components.
    ///
    /// ### Performance Considerations
    /// Prefer implementing [`apply_many`](Self::apply_many) directly.
    ///
    /// ### Reads
    /// 
    ///
    /// ### Mutates
    /// 
    /// - `time_of_day`: A mutable reference to the component of type [`TimeOfDayComponent`].
    #[allow(unused_variables)]
    fn apply_single(
        &mut self,
        context: &::sillyecs::FrameContext,
        time_of_day: &mut TimeOfDayComponent,
    ) {
         // intentionally left blank
     }

    /// Applies the system's business logic to the given entity components.
    ///
    /// ### Reads
    /// 
    ///
    /// ### Mutates
    /// 
    /// - `time_of_day`: A mutable slice of the components of type [`TimeOfDayComponent`].
    fn apply_many(
        &mut self,
        context: &::sillyecs::FrameContext,
        time_of_days: &mut [TimeOfDayComponent],
    ) {
        let zipped_iter = time_of_days;
        for time_of_day in zipped_iter {
            self.apply_single(
                &context,
                time_of_day,
            );
        }
    }

    /// Applies the system's business logic to the given entity components.
    ///
    /// ### Reads
    /// 
    ///
    /// ### Mutates
    /// 
    /// - `time_of_day`: A mutable slice of the components of type [`TimeOfDayComponent`].
    #[allow(unused_mut)]
    #[inline]
    fn apply_all(
        &mut self,
        context: &::sillyecs::FrameContext,
        mut time_of_days: [&mut [TimeOfDayComponent]; 1],
    ) {
        let zipped_iter = time_of_days;
        for time_of_day in zipped_iter {
            ApplyTimeOfDaySystem::apply_many(
                self,
                context,
                time_of_day,
            );
        }
    }
}

#[allow(dead_code)]
impl TimeOfDaySystem {
    /// Applies the system's business logic to the given entity components.
    ///
    /// ### Reads
    /// 
    ///
    /// ### Mutates
    /// 
    /// - `time_of_day`: A mutable slice of the components of type [`TimeOfDayComponent`].
    #[inline(always)]
    fn apply_many(
        &mut self,
        context: &::sillyecs::FrameContext,
        time_of_days: &mut [TimeOfDayComponent],
    ) {
        // Force the implementation of the ApplyTimeOfDaySystem trait.
        ApplyTimeOfDaySystem::apply_many(
            self,
            context,
            time_of_days,
        );
    }

    /// Applies the system's business logic to the given entity components.
    ///
    /// ### Reads
    /// 
    ///
    /// ### Mutates
    /// 
    /// - `time_of_day`: A mutable slice of the components of type [`TimeOfDayComponent`].
    #[allow(unused_mut)]
    #[inline]
    fn apply_all(
        &mut self,
        context: &::sillyecs::FrameContext,
        mut time_of_days: [&mut [TimeOfDayComponent]; 1],
    ) {
        let zipped_iter = time_of_days;
        for time_of_day in zipped_iter {
            self.apply_many(
                context,
                time_of_day,
            );
        }
    }
}

#[automatically_derived]
impl System for TimeOfDaySystem {
    const ID: SystemId = SystemId::TimeOfDay;
}

impl PartialEq<SystemId> for TimeOfDaySystem {
    fn eq(&self, other: &SystemId) -> bool {
        Self::ID.eq(other)
    }
}

#[automatically_derived]
impl From<TimeOfDaySystemData> for TimeOfDaySystem {
    fn from(data: TimeOfDaySystemData) -> Self {
        Self(data)
    }
}

#[automatically_derived]
impl core::ops::Deref for TimeOfDaySystem {
    type Target = TimeOfDaySystemData;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

#[automatically_derived]
impl core::ops::DerefMut for TimeOfDaySystem {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

/// A system operating on multiple [`Component`]s.
///
/// The business logic is implemented via the [`ApplySunSystem`] trait.
/// See also [`SunData`] for the systems-specific data.
///
/// ### Reads
/// 
/// - [`TimeOfDayComponent`]
///
/// ### Mutates
/// 
/// - [`SunComponent`]
/// - [`SkydomeComponent`]
/// - [`AmbientLightComponent`]
///
/// # Archetype Use
/// This component processes components in the following archetypes:
/// 
/// - [`EnvironmentArchetype`] ([`ArchetypeId::Environment`])
#[derive(Debug)]
pub struct SunSystem(SunSystemData);

/// Implements the business logic for the [`SunSystem`] system.
#[allow(dead_code)]
pub trait ApplySunSystem: System {
    type Error: core::error::Error;

    /// Indicates whether this system is ready for action.
    ///
    /// Always called once per phase before all other functions of this system. If it returns
    /// `true`, the remaining functions are executed, otherwise the system skips the phase.
    #[inline]
    #[allow(unused_variables)]
    fn is_ready(
        &self,
    ) -> bool {
        true
    }

    /// Executed at the beginning of the phase (for this system).
    ///
    /// All calls to [`apply_many`](Self::apply_many) and/or [`apply_single`](Self::apply_single) happen after this
    /// function was executed.
    #[inline]
    #[allow(unused_variables)]
    fn on_begin_phase(
        &mut self,
    ) -> Result<(), Self::Error> {
        Ok(())
    }

    /// Executed at the end of the phase (for this system).
    ///
    /// All calls to [`apply_many`](Self::apply_many) and/or [`apply_single`](Self::apply_single) happen before this
    /// function is executed.
    #[inline]
    #[allow(unused_variables)]
    fn on_end_phase(
        &mut self,
    ) -> Result<(), Self::Error> {
        Ok(())
    }

    /// Applies the system's business logic to the given entity components.
    ///
    /// ### Performance Considerations
    /// Prefer implementing [`apply_many`](Self::apply_many) directly.
    ///
    /// ### Reads
    /// 
    /// - `time_of_day`: A reference to the input component of type [`TimeOfDayComponent`].
    ///
    /// ### Mutates
    /// 
    /// - `sun`: A mutable reference to the component of type [`SunComponent`].
    /// - `skydome`: A mutable reference to the component of type [`SkydomeComponent`].
    /// - `ambient_light`: A mutable reference to the component of type [`AmbientLightComponent`].
    #[allow(unused_variables)]
    fn apply_single(
        &mut self,
        time_of_day: &TimeOfDayComponent,
        sun: &mut SunComponent,
        skydome: &mut SkydomeComponent,
        ambient_light: &mut AmbientLightComponent,
    ) {
         // intentionally left blank
     }

    /// Applies the system's business logic to the given entity components.
    ///
    /// ### Reads
    /// 
    /// - `time_of_day`: A slice of the input components of type [`TimeOfDayComponent`].
    ///
    /// ### Mutates
    /// 
    /// - `sun`: A mutable slice of the components of type [`SunComponent`].
    /// - `skydome`: A mutable slice of the components of type [`SkydomeComponent`].
    /// - `ambient_light`: A mutable slice of the components of type [`AmbientLightComponent`].
    fn apply_many(
        &mut self,
        time_of_days: &[TimeOfDayComponent],
        suns: &mut [SunComponent],
        skydomes: &mut [SkydomeComponent],
        ambient_lights: &mut [AmbientLightComponent],
    ) {
        let zipped_iter = time_of_days.iter().zip(suns.iter_mut().zip(skydomes.iter_mut().zip(ambient_lights.iter_mut())));
        for (time_of_day, (sun, (skydome, ambient_light))) in zipped_iter {
            self.apply_single(
                time_of_day,
                sun,
                skydome,
                ambient_light,
            );
        }
    }

    /// Applies the system's business logic to the given entity components.
    ///
    /// ### Reads
    /// 
    /// - `time_of_day`: A slice of the input components of type [`TimeOfDayComponent`].
    ///
    /// ### Mutates
    /// 
    /// - `sun`: A mutable slice of the components of type [`SunComponent`].
    /// - `skydome`: A mutable slice of the components of type [`SkydomeComponent`].
    /// - `ambient_light`: A mutable slice of the components of type [`AmbientLightComponent`].
    #[allow(unused_mut)]
    #[inline]
    fn apply_all(
        &mut self,
        time_of_days: [&[TimeOfDayComponent]; 1],
        mut suns: [&mut [SunComponent]; 1],
        mut skydomes: [&mut [SkydomeComponent]; 1],
        mut ambient_lights: [&mut [AmbientLightComponent]; 1],
    ) {
        let zipped_iter = time_of_days.iter().zip(suns.iter_mut().zip(skydomes.iter_mut().zip(ambient_lights.iter_mut())));
        for (time_of_day, (sun, (skydome, ambient_light))) in zipped_iter {
            ApplySunSystem::apply_many(
                self,
                time_of_day,
                sun,
                skydome,
                ambient_light,
            );
        }
    }
}

#[allow(dead_code)]
impl SunSystem {
    /// Applies the system's business logic to the given entity components.
    ///
    /// ### Reads
    /// 
    /// - `time_of_day`: A slice of the input components of type [`TimeOfDayComponent`].
    ///
    /// ### Mutates
    /// 
    /// - `sun`: A mutable slice of the components of type [`SunComponent`].
    /// - `skydome`: A mutable slice of the components of type [`SkydomeComponent`].
    /// - `ambient_light`: A mutable slice of the components of type [`AmbientLightComponent`].
    #[inline(always)]
    fn apply_many(
        &mut self,
        time_of_days: &[TimeOfDayComponent],
        suns: &mut [SunComponent],
        skydomes: &mut [SkydomeComponent],
        ambient_lights: &mut [AmbientLightComponent],
    ) {
        // Force the implementation of the ApplySunSystem trait.
        ApplySunSystem::apply_many(
            self,
            time_of_days,
            suns,
            skydomes,
            ambient_lights,
        );
    }

    /// Applies the system's business logic to the given entity components.
    ///
    /// ### Reads
    /// 
    /// - `time_of_day`: A slice of the input components of type [`TimeOfDayComponent`].
    ///
    /// ### Mutates
    /// 
    /// - `sun`: A mutable slice of the components of type [`SunComponent`].
    /// - `skydome`: A mutable slice of the components of type [`SkydomeComponent`].
    /// - `ambient_light`: A mutable slice of the components of type [`AmbientLightComponent`].
    #[allow(unused_mut)]
    #[inline]
    fn apply_all(
        &mut self,
        time_of_days: [&[TimeOfDayComponent]; 1],
        mut suns: [&mut [SunComponent]; 1],
        mut skydomes: [&mut [SkydomeComponent]; 1],
        mut ambient_lights: [&mut [AmbientLightComponent]; 1],
    ) {
        let zipped_iter = time_of_days.iter().zip(suns.iter_mut().zip(skydomes.iter_mut().zip(ambient_lights.iter_mut())));
        for (time_of_day, (sun, (skydome, ambient_light))) in zipped_iter {
            self.apply_many(
                time_of_day,
                sun,
                skydome,
                ambient_light,
            );
        }
    }
}

#[automatically_derived]
impl System for SunSystem {
    const ID: SystemId = SystemId::Sun;
}

impl PartialEq<SystemId> for SunSystem {
    fn eq(&self, other: &SystemId) -> bool {
        Self::ID.eq(other)
    }
}

#[automatically_derived]
impl From<SunSystemData> for SunSystem {
    fn from(data: SunSystemData) -> Self {
        Self(data)
    }
}

#[automatically_derived]
impl core::ops::Deref for SunSystem {
    type Target = SunSystemData;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

#[automatically_derived]
impl core::ops::DerefMut for SunSystem {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

/// A system operating on multiple [`Component`]s.
///
/// The business logic is implemented via the [`ApplyStarsSystem`] trait.
/// See also [`StarsData`] for the systems-specific data.
///
/// ### Reads
/// 
/// - [`TimeOfDayComponent`]
///
/// ### Mutates
/// 
/// - [`StarsComponent`]
///
/// # Archetype Use
/// This component processes components in the following archetypes:
/// 
/// - [`EnvironmentArchetype`] ([`ArchetypeId::Environment`])
#[derive(Debug)]
pub struct StarsSystem(StarsSystemData);

/// Implements the business logic for the [`StarsSystem`] system.
#[allow(dead_code)]
pub trait ApplyStarsSystem: System {
    type Error: core::error::Error;

    /// Indicates whether this system is ready for action.
    ///
    /// Always called once per phase before all other functions of this system. If it returns
    /// `true`, the remaining functions are executed, otherwise the system skips the phase.
    #[inline]
    #[allow(unused_variables)]
    fn is_ready(
        &self,
    ) -> bool {
        true
    }

    /// Executed at the beginning of the phase (for this system).
    ///
    /// All calls to [`apply_many`](Self::apply_many) and/or [`apply_single`](Self::apply_single) happen after this
    /// function was executed.
    #[inline]
    #[allow(unused_variables)]
    fn on_begin_phase(
        &mut self,
    ) -> Result<(), Self::Error> {
        Ok(())
    }

    /// Executed at the end of the phase (for this system).
    ///
    /// All calls to [`apply_many`](Self::apply_many) and/or [`apply_single`](Self::apply_single) happen before this
    /// function is executed.
    #[inline]
    #[allow(unused_variables)]
    fn on_end_phase(
        &mut self,
    ) -> Result<(), Self::Error> {
        Ok(())
    }

    /// Applies the system's business logic to the given entity components.
    ///
    /// ### Performance Considerations
    /// Prefer implementing [`apply_many`](Self::apply_many) directly.
    ///
    /// ### Reads
    /// 
    /// - `time_of_day`: A reference to the input component of type [`TimeOfDayComponent`].
    ///
    /// ### Mutates
    /// 
    /// - `stars`: A mutable reference to the component of type [`StarsComponent`].
    #[allow(unused_variables)]
    fn apply_single(
        &mut self,
        time_of_day: &TimeOfDayComponent,
        stars: &mut StarsComponent,
    ) {
         // intentionally left blank
     }

    /// Applies the system's business logic to the given entity components.
    ///
    /// ### Reads
    /// 
    /// - `time_of_day`: A slice of the input components of type [`TimeOfDayComponent`].
    ///
    /// ### Mutates
    /// 
    /// - `stars`: A mutable slice of the components of type [`StarsComponent`].
    fn apply_many(
        &mut self,
        time_of_days: &[TimeOfDayComponent],
        stars: &mut [StarsComponent],
    ) {
        let zipped_iter = time_of_days.iter().zip(stars.iter_mut());
        for (time_of_day, stars) in zipped_iter {
            self.apply_single(
                time_of_day,
                stars,
            );
        }
    }

    /// Applies the system's business logic to the given entity components.
    ///
    /// ### Reads
    /// 
    /// - `time_of_day`: A slice of the input components of type [`TimeOfDayComponent`].
    ///
    /// ### Mutates
    /// 
    /// - `stars`: A mutable slice of the components of type [`StarsComponent`].
    #[allow(unused_mut)]
    #[inline]
    fn apply_all(
        &mut self,
        time_of_days: [&[TimeOfDayComponent]; 1],
        mut stars: [&mut [StarsComponent]; 1],
    ) {
        let zipped_iter = time_of_days.iter().zip(stars.iter_mut());
        for (time_of_day, stars) in zipped_iter {
            ApplyStarsSystem::apply_many(
                self,
                time_of_day,
                stars,
            );
        }
    }
}

#[allow(dead_code)]
impl StarsSystem {
    /// Applies the system's business logic to the given entity components.
    ///
    /// ### Reads
    /// 
    /// - `time_of_day`: A slice of the input components of type [`TimeOfDayComponent`].
    ///
    /// ### Mutates
    /// 
    /// - `stars`: A mutable slice of the components of type [`StarsComponent`].
    #[inline(always)]
    fn apply_many(
        &mut self,
        time_of_days: &[TimeOfDayComponent],
        stars: &mut [StarsComponent],
    ) {
        // Force the implementation of the ApplyStarsSystem trait.
        ApplyStarsSystem::apply_many(
            self,
            time_of_days,
            stars,
        );
    }

    /// Applies the system's business logic to the given entity components.
    ///
    /// ### Reads
    /// 
    /// - `time_of_day`: A slice of the input components of type [`TimeOfDayComponent`].
    ///
    /// ### Mutates
    /// 
    /// - `stars`: A mutable slice of the components of type [`StarsComponent`].
    #[allow(unused_mut)]
    #[inline]
    fn apply_all(
        &mut self,
        time_of_days: [&[TimeOfDayComponent]; 1],
        mut stars: [&mut [StarsComponent]; 1],
    ) {
        let zipped_iter = time_of_days.iter().zip(stars.iter_mut());
        for (time_of_day, stars) in zipped_iter {
            self.apply_many(
                time_of_day,
                stars,
            );
        }
    }
}

#[automatically_derived]
impl System for StarsSystem {
    const ID: SystemId = SystemId::Stars;
}

impl PartialEq<SystemId> for StarsSystem {
    fn eq(&self, other: &SystemId) -> bool {
        Self::ID.eq(other)
    }
}

#[automatically_derived]
impl From<StarsSystemData> for StarsSystem {
    fn from(data: StarsSystemData) -> Self {
        Self(data)
    }
}

#[automatically_derived]
impl core::ops::Deref for StarsSystem {
    type Target = StarsSystemData;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

#[automatically_derived]
impl core::ops::DerefMut for StarsSystem {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
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
///
/// # Archetype Use
/// This component processes components in the following archetypes:
/// 
/// - [`CubeArchetype`] ([`ArchetypeId::Cube`])
/// - [`PlayerArchetype`] ([`ArchetypeId::Player`])
#[derive(Debug)]
pub struct PhysicsSystem(PhysicsSystemData);

/// Implements the business logic for the [`PhysicsSystem`] system.
#[allow(dead_code)]
pub trait ApplyPhysicsSystem: System {
    type Error: core::error::Error;

    /// Indicates whether this system is ready for action.
    ///
    /// Always called once per phase before all other functions of this system. If it returns
    /// `true`, the remaining functions are executed, otherwise the system skips the phase.
    #[inline]
    #[allow(unused_variables)]
    fn is_ready(
        &self,
        context: &::sillyecs::FrameContext,
    ) -> bool {
        true
    }

    /// Executed at the beginning of the phase (for this system).
    ///
    /// All calls to [`apply_many`](Self::apply_many) and/or [`apply_single`](Self::apply_single) happen after this
    /// function was executed.
    #[inline]
    #[allow(unused_variables)]
    fn on_begin_phase(
        &mut self,
        context: &::sillyecs::FrameContext,
    ) -> Result<(), Self::Error> {
        Ok(())
    }

    /// Executed at the end of the phase (for this system).
    ///
    /// All calls to [`apply_many`](Self::apply_many) and/or [`apply_single`](Self::apply_single) happen before this
    /// function is executed.
    #[inline]
    #[allow(unused_variables)]
    fn on_end_phase(
        &mut self,
        context: &::sillyecs::FrameContext,
    ) -> Result<(), Self::Error> {
        Ok(())
    }

    /// Applies the system's business logic to the given entity components.
    ///
    /// ### Performance Considerations
    /// Prefer implementing [`apply_many`](Self::apply_many) directly.
    ///
    /// ### Reads
    /// 
    /// - `velocity`: A reference to the input component of type [`VelocityComponent`].
    ///
    /// ### Mutates
    /// 
    /// - `position`: A mutable reference to the component of type [`PositionComponent`].
    #[allow(unused_variables)]
    fn apply_single(
        &mut self,
        context: &::sillyecs::FrameContext,
        velocity: &VelocityComponent,
        position: &mut PositionComponent,
    ) {
         // intentionally left blank
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
    fn apply_many(
        &mut self,
        context: &::sillyecs::FrameContext,
        velocities: &[VelocityComponent],
        positions: &mut [PositionComponent],
    ) {
        let zipped_iter = velocities.iter().zip(positions.iter_mut());
        for (velocity, position) in zipped_iter {
            self.apply_single(
                &context,
                velocity,
                position,
            );
        }
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
    #[allow(unused_mut)]
    #[inline]
    fn apply_all(
        &mut self,
        context: &::sillyecs::FrameContext,
        velocities: [&[VelocityComponent]; 2],
        mut positions: [&mut [PositionComponent]; 2],
    ) {
        let zipped_iter = velocities.iter().zip(positions.iter_mut());
        for (velocity, position) in zipped_iter {
            ApplyPhysicsSystem::apply_many(
                self,
                context,
                velocity,
                position,
            );
        }
    }
}

#[allow(dead_code)]
impl PhysicsSystem {
    /// Applies the system's business logic to the given entity components.
    ///
    /// ### Reads
    /// 
    /// - `velocity`: A slice of the input components of type [`VelocityComponent`].
    ///
    /// ### Mutates
    /// 
    /// - `position`: A mutable slice of the components of type [`PositionComponent`].
    #[inline(always)]
    fn apply_many(
        &mut self,
        context: &::sillyecs::FrameContext,
        velocities: &[VelocityComponent],
        positions: &mut [PositionComponent],
    ) {
        // Force the implementation of the ApplyPhysicsSystem trait.
        ApplyPhysicsSystem::apply_many(
            self,
            context,
            velocities,
            positions,
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
    #[allow(unused_mut)]
    #[inline]
    fn apply_all(
        &mut self,
        context: &::sillyecs::FrameContext,
        velocities: [&[VelocityComponent]; 2],
        mut positions: [&mut [PositionComponent]; 2],
    ) {
        let zipped_iter = velocities.iter().zip(positions.iter_mut());
        for (velocity, position) in zipped_iter {
            self.apply_many(
                context,
                velocity,
                position,
            );
        }
    }
}

#[automatically_derived]
impl System for PhysicsSystem {
    const ID: SystemId = SystemId::Physics;
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
/// The business logic is implemented via the [`ApplyBackgroundColorCycleSystem`] trait.
/// See also [`BackgroundColorCycleData`] for the systems-specific data.
///
/// ### Mutates
/// 
/// - [`BackgroundColorComponent`]
///
/// # Archetype Use
/// This component processes components in the following archetypes:
/// 
/// - [`SceneBackgroundArchetype`] ([`ArchetypeId::SceneBackground`])
#[derive(Debug)]
pub struct BackgroundColorCycleSystem(BackgroundColorCycleSystemData);

/// Implements the business logic for the [`BackgroundColorCycleSystem`] system.
#[allow(dead_code)]
pub trait ApplyBackgroundColorCycleSystem: System {
    type Error: core::error::Error;

    /// Indicates whether this system is ready for action.
    ///
    /// Always called once per phase before all other functions of this system. If it returns
    /// `true`, the remaining functions are executed, otherwise the system skips the phase.
    #[inline]
    #[allow(unused_variables)]
    fn is_ready(
        &self,
        context: &::sillyecs::FrameContext,
    ) -> bool {
        true
    }

    /// Executed at the beginning of the phase (for this system).
    ///
    /// All calls to [`apply_many`](Self::apply_many) and/or [`apply_single`](Self::apply_single) happen after this
    /// function was executed.
    #[inline]
    #[allow(unused_variables)]
    fn on_begin_phase(
        &mut self,
        context: &::sillyecs::FrameContext,
    ) -> Result<(), Self::Error> {
        Ok(())
    }

    /// Executed at the end of the phase (for this system).
    ///
    /// All calls to [`apply_many`](Self::apply_many) and/or [`apply_single`](Self::apply_single) happen before this
    /// function is executed.
    #[inline]
    #[allow(unused_variables)]
    fn on_end_phase(
        &mut self,
        context: &::sillyecs::FrameContext,
    ) -> Result<(), Self::Error> {
        Ok(())
    }

    /// Applies the system's business logic to the given entity components.
    ///
    /// ### Performance Considerations
    /// Prefer implementing [`apply_many`](Self::apply_many) directly.
    ///
    /// ### Reads
    /// 
    ///
    /// ### Mutates
    /// 
    /// - `background_color`: A mutable reference to the component of type [`BackgroundColorComponent`].
    #[allow(unused_variables)]
    fn apply_single(
        &mut self,
        context: &::sillyecs::FrameContext,
        background_color: &mut BackgroundColorComponent,
    ) {
         // intentionally left blank
     }

    /// Applies the system's business logic to the given entity components.
    ///
    /// ### Reads
    /// 
    ///
    /// ### Mutates
    /// 
    /// - `background_color`: A mutable slice of the components of type [`BackgroundColorComponent`].
    fn apply_many(
        &mut self,
        context: &::sillyecs::FrameContext,
        background_colors: &mut [BackgroundColorComponent],
    ) {
        let zipped_iter = background_colors;
        for background_color in zipped_iter {
            self.apply_single(
                &context,
                background_color,
            );
        }
    }

    /// Applies the system's business logic to the given entity components.
    ///
    /// ### Reads
    /// 
    ///
    /// ### Mutates
    /// 
    /// - `background_color`: A mutable slice of the components of type [`BackgroundColorComponent`].
    #[allow(unused_mut)]
    #[inline]
    fn apply_all(
        &mut self,
        context: &::sillyecs::FrameContext,
        mut background_colors: [&mut [BackgroundColorComponent]; 1],
    ) {
        let zipped_iter = background_colors;
        for background_color in zipped_iter {
            ApplyBackgroundColorCycleSystem::apply_many(
                self,
                context,
                background_color,
            );
        }
    }
}

#[allow(dead_code)]
impl BackgroundColorCycleSystem {
    /// Applies the system's business logic to the given entity components.
    ///
    /// ### Reads
    /// 
    ///
    /// ### Mutates
    /// 
    /// - `background_color`: A mutable slice of the components of type [`BackgroundColorComponent`].
    #[inline(always)]
    fn apply_many(
        &mut self,
        context: &::sillyecs::FrameContext,
        background_colors: &mut [BackgroundColorComponent],
    ) {
        // Force the implementation of the ApplyBackgroundColorCycleSystem trait.
        ApplyBackgroundColorCycleSystem::apply_many(
            self,
            context,
            background_colors,
        );
    }

    /// Applies the system's business logic to the given entity components.
    ///
    /// ### Reads
    /// 
    ///
    /// ### Mutates
    /// 
    /// - `background_color`: A mutable slice of the components of type [`BackgroundColorComponent`].
    #[allow(unused_mut)]
    #[inline]
    fn apply_all(
        &mut self,
        context: &::sillyecs::FrameContext,
        mut background_colors: [&mut [BackgroundColorComponent]; 1],
    ) {
        let zipped_iter = background_colors;
        for background_color in zipped_iter {
            self.apply_many(
                context,
                background_color,
            );
        }
    }
}

#[automatically_derived]
impl System for BackgroundColorCycleSystem {
    const ID: SystemId = SystemId::BackgroundColorCycle;
}

impl PartialEq<SystemId> for BackgroundColorCycleSystem {
    fn eq(&self, other: &SystemId) -> bool {
        Self::ID.eq(other)
    }
}

#[automatically_derived]
impl From<BackgroundColorCycleSystemData> for BackgroundColorCycleSystem {
    fn from(data: BackgroundColorCycleSystemData) -> Self {
        Self(data)
    }
}

#[automatically_derived]
impl core::ops::Deref for BackgroundColorCycleSystem {
    type Target = BackgroundColorCycleSystemData;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

#[automatically_derived]
impl core::ops::DerefMut for BackgroundColorCycleSystem {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

/// A system operating on multiple [`Component`]s.
///
/// The business logic is implemented via the [`ApplyCameraRenderSystem`] trait.
/// See also [`CameraRenderData`] for the systems-specific data.
///
/// ### Reads
/// 
/// - [`CameraComponent`]
///
/// # Archetype Use
/// This component processes components in the following archetypes:
/// 
/// - [`CameraArchetype`] ([`ArchetypeId::Camera`])
#[derive(Debug)]
pub struct CameraRenderSystem(CameraRenderSystemData);

/// Implements the business logic for the [`CameraRenderSystem`] system.
#[allow(dead_code)]
pub trait ApplyCameraRenderSystem: System {
    type Error: core::error::Error;

    /// Indicates whether this system is ready for action.
    ///
    /// Always called once per phase before all other functions of this system. If it returns
    /// `true`, the remaining functions are executed, otherwise the system skips the phase.
    #[inline]
    #[allow(unused_variables)]
    fn is_ready(
        &self,
                wgpu_render: &WgpuRenderState,
                
    ) -> bool {
        true
    }

    /// Executed at the beginning of the phase (for this system).
    ///
    /// All calls to [`apply_many`](Self::apply_many) and/or [`apply_single`](Self::apply_single) happen after this
    /// function was executed.
    #[inline]
    #[allow(unused_variables)]
    fn on_begin_phase(
        &mut self,
                wgpu_render: &WgpuRenderState,
                
    ) -> Result<(), Self::Error> {
        Ok(())
    }

    /// Executed at the end of the phase (for this system).
    ///
    /// All calls to [`apply_many`](Self::apply_many) and/or [`apply_single`](Self::apply_single) happen before this
    /// function is executed.
    #[inline]
    #[allow(unused_variables)]
    fn on_end_phase(
        &mut self,
                wgpu_render: &WgpuRenderState,
                
    ) -> Result<(), Self::Error> {
        Ok(())
    }

    /// Applies the system's business logic to the given entity components.
    ///
    /// ### Performance Considerations
    /// Prefer implementing [`apply_many`](Self::apply_many) directly.
    ///
    /// ### Reads
    /// 
    /// - `camera`: A reference to the input component of type [`CameraComponent`].
    ///
    /// ### Mutates
    /// 
    #[allow(unused_variables)]
    fn apply_single(
        &mut self,
                wgpu_render: &WgpuRenderState,
                active_camera: &mut ActiveCameraState,
        camera: &CameraComponent,
    ) {
         // intentionally left blank
     }

    /// Applies the system's business logic to the given entity components.
    ///
    /// ### Reads
    /// 
    /// - `camera`: A slice of the input components of type [`CameraComponent`].
    ///
    /// ### Mutates
    /// 
    fn apply_many(
        &mut self,
                wgpu_render: &WgpuRenderState,
                active_camera: &mut ActiveCameraState,
        cameras: &[CameraComponent],
    ) {
        let zipped_iter = cameras;
        for camera in zipped_iter {
            self.apply_single(
                        wgpu_render,
                        active_camera,
                camera,
            );
        }
    }

    /// Applies the system's business logic to the given entity components.
    ///
    /// ### Reads
    /// 
    /// - `camera`: A slice of the input components of type [`CameraComponent`].
    ///
    /// ### Mutates
    /// 
    #[allow(unused_mut)]
    #[inline]
    fn apply_all(
        &mut self,
                wgpu_render: &WgpuRenderState,
                active_camera: &mut ActiveCameraState,
        cameras: [&[CameraComponent]; 1],
    ) {
        let zipped_iter = cameras;
        for camera in zipped_iter {
            ApplyCameraRenderSystem::apply_many(
                self,
                        wgpu_render,
                        active_camera,
                camera,
            );
        }
    }
}

#[allow(dead_code)]
impl CameraRenderSystem {
    /// Applies the system's business logic to the given entity components.
    ///
    /// ### Reads
    /// 
    /// - `camera`: A slice of the input components of type [`CameraComponent`].
    ///
    /// ### Mutates
    /// 
    #[inline(always)]
    fn apply_many(
        &mut self,
        wgpu_render: &WgpuRenderState,
        active_camera: &mut ActiveCameraState,
        cameras: &[CameraComponent],
    ) {
        // Force the implementation of the ApplyCameraRenderSystem trait.
        ApplyCameraRenderSystem::apply_many(
            self,
            wgpu_render,
            active_camera,
            cameras,
        );
    }

    /// Applies the system's business logic to the given entity components.
    ///
    /// ### Reads
    /// 
    /// - `camera`: A slice of the input components of type [`CameraComponent`].
    ///
    /// ### Mutates
    /// 
    #[allow(unused_mut)]
    #[inline]
    fn apply_all(
        &mut self,
        wgpu_render: &WgpuRenderState,
        active_camera: &mut ActiveCameraState,
        cameras: [&[CameraComponent]; 1],
    ) {
        let zipped_iter = cameras;
        for camera in zipped_iter {
            self.apply_many(
                wgpu_render,
                active_camera,
                camera,
            );
        }
    }
}

#[automatically_derived]
impl System for CameraRenderSystem {
    const ID: SystemId = SystemId::CameraRender;
}

impl PartialEq<SystemId> for CameraRenderSystem {
    fn eq(&self, other: &SystemId) -> bool {
        Self::ID.eq(other)
    }
}

#[automatically_derived]
impl From<CameraRenderSystemData> for CameraRenderSystem {
    fn from(data: CameraRenderSystemData) -> Self {
        Self(data)
    }
}

#[automatically_derived]
impl core::ops::Deref for CameraRenderSystem {
    type Target = CameraRenderSystemData;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

#[automatically_derived]
impl core::ops::DerefMut for CameraRenderSystem {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

/// A system operating on multiple [`Component`]s.
///
/// The business logic is implemented via the [`ApplyFrameGlobalsSystem`] trait.
/// See also [`FrameGlobalsData`] for the systems-specific data.
///
/// ### Reads
/// 
/// - [`TimeOfDayComponent`]
///
/// ### Mutates
/// 
/// - [`SunComponent`]
/// - [`AmbientLightComponent`]
/// - [`StarsComponent`]
/// - [`SkydomeComponent`]
///
/// # Archetype Use
/// This component processes components in the following archetypes:
/// 
/// - [`EnvironmentArchetype`] ([`ArchetypeId::Environment`])
#[derive(Debug)]
pub struct FrameGlobalsSystem(FrameGlobalsSystemData);

/// Implements the business logic for the [`FrameGlobalsSystem`] system.
#[allow(dead_code)]
pub trait ApplyFrameGlobalsSystem: System {
    type Error: core::error::Error;

    /// Indicates whether this system is ready for action.
    ///
    /// Always called once per phase before all other functions of this system. If it returns
    /// `true`, the remaining functions are executed, otherwise the system skips the phase.
    #[inline]
    #[allow(unused_variables)]
    fn is_ready(
        &self,
                wgpu_render: &mut WgpuRenderState,
    ) -> bool {
        true
    }

    /// Executed at the beginning of the phase (for this system).
    ///
    /// All calls to [`apply_many`](Self::apply_many) and/or [`apply_single`](Self::apply_single) happen after this
    /// function was executed.
    #[inline]
    #[allow(unused_variables)]
    fn on_begin_phase(
        &mut self,
                wgpu_render: &mut WgpuRenderState,
    ) -> Result<(), Self::Error> {
        Ok(())
    }

    /// Executed at the end of the phase (for this system).
    ///
    /// All calls to [`apply_many`](Self::apply_many) and/or [`apply_single`](Self::apply_single) happen before this
    /// function is executed.
    #[inline]
    #[allow(unused_variables)]
    fn on_end_phase(
        &mut self,
                wgpu_render: &mut WgpuRenderState,
    ) -> Result<(), Self::Error> {
        Ok(())
    }

    /// Applies the system's business logic to the given entity components.
    ///
    /// ### Performance Considerations
    /// Prefer implementing [`apply_many`](Self::apply_many) directly.
    ///
    /// ### Reads
    /// 
    /// - `time_of_day`: A reference to the input component of type [`TimeOfDayComponent`].
    ///
    /// ### Mutates
    /// 
    /// - `sun`: A mutable reference to the component of type [`SunComponent`].
    /// - `ambient_light`: A mutable reference to the component of type [`AmbientLightComponent`].
    /// - `stars`: A mutable reference to the component of type [`StarsComponent`].
    /// - `skydome`: A mutable reference to the component of type [`SkydomeComponent`].
    #[allow(unused_variables)]
    fn apply_single(
        &mut self,
                wgpu_render: &mut WgpuRenderState,
        time_of_day: &TimeOfDayComponent,
        sun: &mut SunComponent,
        ambient_light: &mut AmbientLightComponent,
        stars: &mut StarsComponent,
        skydome: &mut SkydomeComponent,
    ) {
         // intentionally left blank
     }

    /// Applies the system's business logic to the given entity components.
    ///
    /// ### Reads
    /// 
    /// - `time_of_day`: A slice of the input components of type [`TimeOfDayComponent`].
    ///
    /// ### Mutates
    /// 
    /// - `sun`: A mutable slice of the components of type [`SunComponent`].
    /// - `ambient_light`: A mutable slice of the components of type [`AmbientLightComponent`].
    /// - `stars`: A mutable slice of the components of type [`StarsComponent`].
    /// - `skydome`: A mutable slice of the components of type [`SkydomeComponent`].
    fn apply_many(
        &mut self,
                wgpu_render: &mut WgpuRenderState,
        time_of_days: &[TimeOfDayComponent],
        suns: &mut [SunComponent],
        ambient_lights: &mut [AmbientLightComponent],
        stars: &mut [StarsComponent],
        skydomes: &mut [SkydomeComponent],
    ) {
        let zipped_iter = time_of_days.iter().zip(suns.iter_mut().zip(ambient_lights.iter_mut().zip(stars.iter_mut().zip(skydomes.iter_mut()))));
        for (time_of_day, (sun, (ambient_light, (stars, skydome)))) in zipped_iter {
            self.apply_single(
                        wgpu_render,
                time_of_day,
                sun,
                ambient_light,
                stars,
                skydome,
            );
        }
    }

    /// Applies the system's business logic to the given entity components.
    ///
    /// ### Reads
    /// 
    /// - `time_of_day`: A slice of the input components of type [`TimeOfDayComponent`].
    ///
    /// ### Mutates
    /// 
    /// - `sun`: A mutable slice of the components of type [`SunComponent`].
    /// - `ambient_light`: A mutable slice of the components of type [`AmbientLightComponent`].
    /// - `stars`: A mutable slice of the components of type [`StarsComponent`].
    /// - `skydome`: A mutable slice of the components of type [`SkydomeComponent`].
    #[allow(unused_mut)]
    #[inline]
    fn apply_all(
        &mut self,
                wgpu_render: &mut WgpuRenderState,
        time_of_days: [&[TimeOfDayComponent]; 1],
        mut suns: [&mut [SunComponent]; 1],
        mut ambient_lights: [&mut [AmbientLightComponent]; 1],
        mut stars: [&mut [StarsComponent]; 1],
        mut skydomes: [&mut [SkydomeComponent]; 1],
    ) {
        let zipped_iter = time_of_days.iter().zip(suns.iter_mut().zip(ambient_lights.iter_mut().zip(stars.iter_mut().zip(skydomes.iter_mut()))));
        for (time_of_day, (sun, (ambient_light, (stars, skydome)))) in zipped_iter {
            ApplyFrameGlobalsSystem::apply_many(
                self,
                        wgpu_render,
                time_of_day,
                sun,
                ambient_light,
                stars,
                skydome,
            );
        }
    }
}

#[allow(dead_code)]
impl FrameGlobalsSystem {
    /// Applies the system's business logic to the given entity components.
    ///
    /// ### Reads
    /// 
    /// - `time_of_day`: A slice of the input components of type [`TimeOfDayComponent`].
    ///
    /// ### Mutates
    /// 
    /// - `sun`: A mutable slice of the components of type [`SunComponent`].
    /// - `ambient_light`: A mutable slice of the components of type [`AmbientLightComponent`].
    /// - `stars`: A mutable slice of the components of type [`StarsComponent`].
    /// - `skydome`: A mutable slice of the components of type [`SkydomeComponent`].
    #[inline(always)]
    fn apply_many(
        &mut self,
        wgpu_render: &mut WgpuRenderState,
        time_of_days: &[TimeOfDayComponent],
        suns: &mut [SunComponent],
        ambient_lights: &mut [AmbientLightComponent],
        stars: &mut [StarsComponent],
        skydomes: &mut [SkydomeComponent],
    ) {
        // Force the implementation of the ApplyFrameGlobalsSystem trait.
        ApplyFrameGlobalsSystem::apply_many(
            self,
            wgpu_render,
            time_of_days,
            suns,
            ambient_lights,
            stars,
            skydomes,
        );
    }

    /// Applies the system's business logic to the given entity components.
    ///
    /// ### Reads
    /// 
    /// - `time_of_day`: A slice of the input components of type [`TimeOfDayComponent`].
    ///
    /// ### Mutates
    /// 
    /// - `sun`: A mutable slice of the components of type [`SunComponent`].
    /// - `ambient_light`: A mutable slice of the components of type [`AmbientLightComponent`].
    /// - `stars`: A mutable slice of the components of type [`StarsComponent`].
    /// - `skydome`: A mutable slice of the components of type [`SkydomeComponent`].
    #[allow(unused_mut)]
    #[inline]
    fn apply_all(
        &mut self,
        wgpu_render: &mut WgpuRenderState,
        time_of_days: [&[TimeOfDayComponent]; 1],
        mut suns: [&mut [SunComponent]; 1],
        mut ambient_lights: [&mut [AmbientLightComponent]; 1],
        mut stars: [&mut [StarsComponent]; 1],
        mut skydomes: [&mut [SkydomeComponent]; 1],
    ) {
        let zipped_iter = time_of_days.iter().zip(suns.iter_mut().zip(ambient_lights.iter_mut().zip(stars.iter_mut().zip(skydomes.iter_mut()))));
        for (time_of_day, (sun, (ambient_light, (stars, skydome)))) in zipped_iter {
            self.apply_many(
                wgpu_render,
                time_of_day,
                sun,
                ambient_light,
                stars,
                skydome,
            );
        }
    }
}

#[automatically_derived]
impl System for FrameGlobalsSystem {
    const ID: SystemId = SystemId::FrameGlobals;
}

impl PartialEq<SystemId> for FrameGlobalsSystem {
    fn eq(&self, other: &SystemId) -> bool {
        Self::ID.eq(other)
    }
}

#[automatically_derived]
impl From<FrameGlobalsSystemData> for FrameGlobalsSystem {
    fn from(data: FrameGlobalsSystemData) -> Self {
        Self(data)
    }
}

#[automatically_derived]
impl core::ops::Deref for FrameGlobalsSystem {
    type Target = FrameGlobalsSystemData;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

#[automatically_derived]
impl core::ops::DerefMut for FrameGlobalsSystem {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

/// A system operating on multiple [`Component`]s.
///
/// The business logic is implemented via the [`ApplyRenderBackgroundSystem`] trait.
/// See also [`RenderBackgroundData`] for the systems-specific data.
///
/// ### Reads
/// 
/// - [`BackgroundColorComponent`]
///
/// # Archetype Use
/// This component processes components in the following archetypes:
/// 
/// - [`SceneBackgroundArchetype`] ([`ArchetypeId::SceneBackground`])
#[derive(Debug)]
pub struct RenderBackgroundSystem(RenderBackgroundSystemData);

/// Implements the business logic for the [`RenderBackgroundSystem`] system.
#[allow(dead_code)]
pub trait ApplyRenderBackgroundSystem: System {
    type Error: core::error::Error;

    /// Indicates whether this system is ready for action.
    ///
    /// Always called once per phase before all other functions of this system. If it returns
    /// `true`, the remaining functions are executed, otherwise the system skips the phase.
    #[inline]
    #[allow(unused_variables)]
    fn is_ready(
        &self,
                wgpu_render: &mut WgpuRenderState,
    ) -> bool {
        true
    }

    /// Executed at the beginning of the phase (for this system).
    ///
    /// All calls to [`apply_many`](Self::apply_many) and/or [`apply_single`](Self::apply_single) happen after this
    /// function was executed.
    #[inline]
    #[allow(unused_variables)]
    fn on_begin_phase(
        &mut self,
                wgpu_render: &mut WgpuRenderState,
    ) -> Result<(), Self::Error> {
        Ok(())
    }

    /// Executed at the end of the phase (for this system).
    ///
    /// All calls to [`apply_many`](Self::apply_many) and/or [`apply_single`](Self::apply_single) happen before this
    /// function is executed.
    #[inline]
    #[allow(unused_variables)]
    fn on_end_phase(
        &mut self,
                wgpu_render: &mut WgpuRenderState,
    ) -> Result<(), Self::Error> {
        Ok(())
    }

    /// Applies the system's business logic to the given entity components.
    ///
    /// ### Performance Considerations
    /// Prefer implementing [`apply_many`](Self::apply_many) directly.
    ///
    /// ### Reads
    /// 
    /// - `background_color`: A reference to the input component of type [`BackgroundColorComponent`].
    ///
    /// ### Mutates
    /// 
    #[allow(unused_variables)]
    fn apply_single(
        &mut self,
                wgpu_render: &mut WgpuRenderState,
        background_color: &BackgroundColorComponent,
    ) {
         // intentionally left blank
     }

    /// Applies the system's business logic to the given entity components.
    ///
    /// ### Reads
    /// 
    /// - `background_color`: A slice of the input components of type [`BackgroundColorComponent`].
    ///
    /// ### Mutates
    /// 
    fn apply_many(
        &mut self,
                wgpu_render: &mut WgpuRenderState,
        background_colors: &[BackgroundColorComponent],
    ) {
        let zipped_iter = background_colors;
        for background_color in zipped_iter {
            self.apply_single(
                        wgpu_render,
                background_color,
            );
        }
    }

    /// Applies the system's business logic to the given entity components.
    ///
    /// ### Reads
    /// 
    /// - `background_color`: A slice of the input components of type [`BackgroundColorComponent`].
    ///
    /// ### Mutates
    /// 
    #[allow(unused_mut)]
    #[inline]
    fn apply_all(
        &mut self,
                wgpu_render: &mut WgpuRenderState,
        background_colors: [&[BackgroundColorComponent]; 1],
    ) {
        let zipped_iter = background_colors;
        for background_color in zipped_iter {
            ApplyRenderBackgroundSystem::apply_many(
                self,
                        wgpu_render,
                background_color,
            );
        }
    }
}

#[allow(dead_code)]
impl RenderBackgroundSystem {
    /// Applies the system's business logic to the given entity components.
    ///
    /// ### Reads
    /// 
    /// - `background_color`: A slice of the input components of type [`BackgroundColorComponent`].
    ///
    /// ### Mutates
    /// 
    #[inline(always)]
    fn apply_many(
        &mut self,
        wgpu_render: &mut WgpuRenderState,
        background_colors: &[BackgroundColorComponent],
    ) {
        // Force the implementation of the ApplyRenderBackgroundSystem trait.
        ApplyRenderBackgroundSystem::apply_many(
            self,
            wgpu_render,
            background_colors,
        );
    }

    /// Applies the system's business logic to the given entity components.
    ///
    /// ### Reads
    /// 
    /// - `background_color`: A slice of the input components of type [`BackgroundColorComponent`].
    ///
    /// ### Mutates
    /// 
    #[allow(unused_mut)]
    #[inline]
    fn apply_all(
        &mut self,
        wgpu_render: &mut WgpuRenderState,
        background_colors: [&[BackgroundColorComponent]; 1],
    ) {
        let zipped_iter = background_colors;
        for background_color in zipped_iter {
            self.apply_many(
                wgpu_render,
                background_color,
            );
        }
    }
}

#[automatically_derived]
impl System for RenderBackgroundSystem {
    const ID: SystemId = SystemId::RenderBackground;
}

impl PartialEq<SystemId> for RenderBackgroundSystem {
    fn eq(&self, other: &SystemId) -> bool {
        Self::ID.eq(other)
    }
}

#[automatically_derived]
impl From<RenderBackgroundSystemData> for RenderBackgroundSystem {
    fn from(data: RenderBackgroundSystemData) -> Self {
        Self(data)
    }
}

#[automatically_derived]
impl core::ops::Deref for RenderBackgroundSystem {
    type Target = RenderBackgroundSystemData;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

#[automatically_derived]
impl core::ops::DerefMut for RenderBackgroundSystem {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

/// A system operating on multiple [`Component`]s.
///
/// The business logic is implemented via the [`ApplyRenderSkySystem`] trait.
/// See also [`RenderSkyData`] for the systems-specific data.
///
/// ### Reads
/// 
/// - [`SkydomeComponent`]
///
/// # Archetype Use
/// This component processes components in the following archetypes:
/// 
/// - [`EnvironmentArchetype`] ([`ArchetypeId::Environment`])
#[derive(Debug)]
pub struct RenderSkySystem(RenderSkySystemData);

/// Implements the business logic for the [`RenderSkySystem`] system.
#[allow(dead_code)]
pub trait ApplyRenderSkySystem: System {
    type Error: core::error::Error;

    /// Indicates whether this system is ready for action.
    ///
    /// Always called once per phase before all other functions of this system. If it returns
    /// `true`, the remaining functions are executed, otherwise the system skips the phase.
    #[inline]
    #[allow(unused_variables)]
    fn is_ready(
        &self,
                wgpu_render: &mut WgpuRenderState,
    ) -> bool {
        true
    }

    /// Executed at the beginning of the phase (for this system).
    ///
    /// All calls to [`apply_many`](Self::apply_many) and/or [`apply_single`](Self::apply_single) happen after this
    /// function was executed.
    #[inline]
    #[allow(unused_variables)]
    fn on_begin_phase(
        &mut self,
                wgpu_render: &mut WgpuRenderState,
    ) -> Result<(), Self::Error> {
        Ok(())
    }

    /// Executed at the end of the phase (for this system).
    ///
    /// All calls to [`apply_many`](Self::apply_many) and/or [`apply_single`](Self::apply_single) happen before this
    /// function is executed.
    #[inline]
    #[allow(unused_variables)]
    fn on_end_phase(
        &mut self,
                wgpu_render: &mut WgpuRenderState,
    ) -> Result<(), Self::Error> {
        Ok(())
    }

    /// Applies the system's business logic to the given entity components.
    ///
    /// ### Performance Considerations
    /// Prefer implementing [`apply_many`](Self::apply_many) directly.
    ///
    /// ### Reads
    /// 
    /// - `skydome`: A reference to the input component of type [`SkydomeComponent`].
    ///
    /// ### Mutates
    /// 
    #[allow(unused_variables)]
    fn apply_single(
        &mut self,
                wgpu_render: &mut WgpuRenderState,
        skydome: &SkydomeComponent,
    ) {
         // intentionally left blank
     }

    /// Applies the system's business logic to the given entity components.
    ///
    /// ### Reads
    /// 
    /// - `skydome`: A slice of the input components of type [`SkydomeComponent`].
    ///
    /// ### Mutates
    /// 
    fn apply_many(
        &mut self,
                wgpu_render: &mut WgpuRenderState,
        skydomes: &[SkydomeComponent],
    ) {
        let zipped_iter = skydomes;
        for skydome in zipped_iter {
            self.apply_single(
                        wgpu_render,
                skydome,
            );
        }
    }

    /// Applies the system's business logic to the given entity components.
    ///
    /// ### Reads
    /// 
    /// - `skydome`: A slice of the input components of type [`SkydomeComponent`].
    ///
    /// ### Mutates
    /// 
    #[allow(unused_mut)]
    #[inline]
    fn apply_all(
        &mut self,
                wgpu_render: &mut WgpuRenderState,
        skydomes: [&[SkydomeComponent]; 1],
    ) {
        let zipped_iter = skydomes;
        for skydome in zipped_iter {
            ApplyRenderSkySystem::apply_many(
                self,
                        wgpu_render,
                skydome,
            );
        }
    }
}

#[allow(dead_code)]
impl RenderSkySystem {
    /// Applies the system's business logic to the given entity components.
    ///
    /// ### Reads
    /// 
    /// - `skydome`: A slice of the input components of type [`SkydomeComponent`].
    ///
    /// ### Mutates
    /// 
    #[inline(always)]
    fn apply_many(
        &mut self,
        wgpu_render: &mut WgpuRenderState,
        skydomes: &[SkydomeComponent],
    ) {
        // Force the implementation of the ApplyRenderSkySystem trait.
        ApplyRenderSkySystem::apply_many(
            self,
            wgpu_render,
            skydomes,
        );
    }

    /// Applies the system's business logic to the given entity components.
    ///
    /// ### Reads
    /// 
    /// - `skydome`: A slice of the input components of type [`SkydomeComponent`].
    ///
    /// ### Mutates
    /// 
    #[allow(unused_mut)]
    #[inline]
    fn apply_all(
        &mut self,
        wgpu_render: &mut WgpuRenderState,
        skydomes: [&[SkydomeComponent]; 1],
    ) {
        let zipped_iter = skydomes;
        for skydome in zipped_iter {
            self.apply_many(
                wgpu_render,
                skydome,
            );
        }
    }
}

#[automatically_derived]
impl System for RenderSkySystem {
    const ID: SystemId = SystemId::RenderSky;
}

impl PartialEq<SystemId> for RenderSkySystem {
    fn eq(&self, other: &SystemId) -> bool {
        Self::ID.eq(other)
    }
}

#[automatically_derived]
impl From<RenderSkySystemData> for RenderSkySystem {
    fn from(data: RenderSkySystemData) -> Self {
        Self(data)
    }
}

#[automatically_derived]
impl core::ops::Deref for RenderSkySystem {
    type Target = RenderSkySystemData;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

#[automatically_derived]
impl core::ops::DerefMut for RenderSkySystem {
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
/// - [`TransformComponent`]
/// - [`ModelComponent`]
///
/// # Archetype Use
/// This component processes components in the following archetypes:
/// 
/// - [`CubeArchetype`] ([`ArchetypeId::Cube`])
#[derive(Debug)]
pub struct RenderSystem(RenderSystemData);

/// Implements the business logic for the [`RenderSystem`] system.
#[allow(dead_code)]
pub trait ApplyRenderSystem: System {
    type Error: core::error::Error;

    /// Indicates whether this system is ready for action.
    ///
    /// Always called once per phase before all other functions of this system. If it returns
    /// `true`, the remaining functions are executed, otherwise the system skips the phase.
    #[inline]
    #[allow(unused_variables)]
    fn is_ready(
        &self,
                wgpu_render: &mut WgpuRenderState,
    ) -> bool {
        true
    }

    /// Executed at the beginning of the phase (for this system).
    ///
    /// All calls to [`apply_many`](Self::apply_many) and/or [`apply_single`](Self::apply_single) happen after this
    /// function was executed.
    #[inline]
    #[allow(unused_variables)]
    fn on_begin_phase(
        &mut self,
                wgpu_render: &mut WgpuRenderState,
    ) -> Result<(), Self::Error> {
        Ok(())
    }

    /// Executed at the end of the phase (for this system).
    ///
    /// All calls to [`apply_many`](Self::apply_many) and/or [`apply_single`](Self::apply_single) happen before this
    /// function is executed.
    #[inline]
    #[allow(unused_variables)]
    fn on_end_phase(
        &mut self,
                wgpu_render: &mut WgpuRenderState,
    ) -> Result<(), Self::Error> {
        Ok(())
    }

    /// Applies the system's business logic to the given entity components.
    ///
    /// ### Performance Considerations
    /// Prefer implementing [`apply_many`](Self::apply_many) directly.
    ///
    /// ### Reads
    /// 
    /// - `position`: A reference to the input component of type [`PositionComponent`].
    /// - `transform`: A reference to the input component of type [`TransformComponent`].
    /// - `model`: A reference to the input component of type [`ModelComponent`].
    ///
    /// ### Mutates
    /// 
    #[allow(unused_variables)]
    fn apply_single(
        &mut self,
                wgpu_render: &mut WgpuRenderState,
        position: &PositionComponent,
        transform: &TransformComponent,
        model: &ModelComponent,
    ) {
         // intentionally left blank
     }

    /// Applies the system's business logic to the given entity components.
    ///
    /// ### Reads
    /// 
    /// - `position`: A slice of the input components of type [`PositionComponent`].
    /// - `transform`: A slice of the input components of type [`TransformComponent`].
    /// - `model`: A slice of the input components of type [`ModelComponent`].
    ///
    /// ### Mutates
    /// 
    fn apply_many(
        &mut self,
                wgpu_render: &mut WgpuRenderState,
        positions: &[PositionComponent],
        transforms: &[TransformComponent],
        models: &[ModelComponent],
    ) {
        let zipped_iter = positions.iter().zip(transforms.iter().zip(models.iter()));
        for (position, (transform, model)) in zipped_iter {
            self.apply_single(
                        wgpu_render,
                position,
                transform,
                model,
            );
        }
    }

    /// Applies the system's business logic to the given entity components.
    ///
    /// ### Reads
    /// 
    /// - `position`: A slice of the input components of type [`PositionComponent`].
    /// - `transform`: A slice of the input components of type [`TransformComponent`].
    /// - `model`: A slice of the input components of type [`ModelComponent`].
    ///
    /// ### Mutates
    /// 
    #[allow(unused_mut)]
    #[inline]
    fn apply_all(
        &mut self,
                wgpu_render: &mut WgpuRenderState,
        positions: [&[PositionComponent]; 1],
        transforms: [&[TransformComponent]; 1],
        models: [&[ModelComponent]; 1],
    ) {
        let zipped_iter = positions.iter().zip(transforms.iter().zip(models.iter()));
        for (position, (transform, model)) in zipped_iter {
            ApplyRenderSystem::apply_many(
                self,
                        wgpu_render,
                position,
                transform,
                model,
            );
        }
    }
}

#[allow(dead_code)]
impl RenderSystem {
    /// Applies the system's business logic to the given entity components.
    ///
    /// ### Reads
    /// 
    /// - `position`: A slice of the input components of type [`PositionComponent`].
    /// - `transform`: A slice of the input components of type [`TransformComponent`].
    /// - `model`: A slice of the input components of type [`ModelComponent`].
    ///
    /// ### Mutates
    /// 
    #[inline(always)]
    fn apply_many(
        &mut self,
        wgpu_render: &mut WgpuRenderState,
        positions: &[PositionComponent],
        transforms: &[TransformComponent],
        models: &[ModelComponent],
    ) {
        // Force the implementation of the ApplyRenderSystem trait.
        ApplyRenderSystem::apply_many(
            self,
            wgpu_render,
            positions,
            transforms,
            models,
        );
    }

    /// Applies the system's business logic to the given entity components.
    ///
    /// ### Reads
    /// 
    /// - `position`: A slice of the input components of type [`PositionComponent`].
    /// - `transform`: A slice of the input components of type [`TransformComponent`].
    /// - `model`: A slice of the input components of type [`ModelComponent`].
    ///
    /// ### Mutates
    /// 
    #[allow(unused_mut)]
    #[inline]
    fn apply_all(
        &mut self,
        wgpu_render: &mut WgpuRenderState,
        positions: [&[PositionComponent]; 1],
        transforms: [&[TransformComponent]; 1],
        models: [&[ModelComponent]; 1],
    ) {
        let zipped_iter = positions.iter().zip(transforms.iter().zip(models.iter()));
        for (position, (transform, model)) in zipped_iter {
            self.apply_many(
                wgpu_render,
                position,
                transform,
                model,
            );
        }
    }
}

#[automatically_derived]
impl System for RenderSystem {
    const ID: SystemId = SystemId::Render;
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

/// A system operating on multiple [`Component`]s.
///
/// The business logic is implemented via the [`ApplyTileRenderSystem`] trait.
/// See also [`TileRenderData`] for the systems-specific data.
///
/// ### Reads
/// 
/// - [`TileComponent`]
/// - [`PositionComponent`]
///
/// # Archetype Use
/// This component processes components in the following archetypes:
/// 
/// - [`TileArchetype`] ([`ArchetypeId::Tile`])
#[derive(Debug)]
pub struct TileRenderSystem(TileRenderSystemData);

/// Implements the business logic for the [`TileRenderSystem`] system.
#[allow(dead_code)]
pub trait ApplyTileRenderSystem: System {
    type Error: core::error::Error;

    /// Indicates whether this system is ready for action.
    ///
    /// Always called once per phase before all other functions of this system. If it returns
    /// `true`, the remaining functions are executed, otherwise the system skips the phase.
    #[inline]
    #[allow(unused_variables)]
    fn is_ready(
        &self,
                wgpu_render: &mut WgpuRenderState,
    ) -> bool {
        true
    }

    /// Executed at the beginning of the phase (for this system).
    ///
    /// All calls to [`apply_many`](Self::apply_many) and/or [`apply_single`](Self::apply_single) happen after this
    /// function was executed.
    #[inline]
    #[allow(unused_variables)]
    fn on_begin_phase(
        &mut self,
                wgpu_render: &mut WgpuRenderState,
    ) -> Result<(), Self::Error> {
        Ok(())
    }

    /// Executed at the end of the phase (for this system).
    ///
    /// All calls to [`apply_many`](Self::apply_many) and/or [`apply_single`](Self::apply_single) happen before this
    /// function is executed.
    #[inline]
    #[allow(unused_variables)]
    fn on_end_phase(
        &mut self,
                wgpu_render: &mut WgpuRenderState,
    ) -> Result<(), Self::Error> {
        Ok(())
    }

    /// Applies the system's business logic to the given entity components.
    ///
    /// ### Performance Considerations
    /// Prefer implementing [`apply_many`](Self::apply_many) directly.
    ///
    /// ### Reads
    /// 
    /// - `tile`: A reference to the input component of type [`TileComponent`].
    /// - `position`: A reference to the input component of type [`PositionComponent`].
    ///
    /// ### Mutates
    /// 
    #[allow(unused_variables)]
    fn apply_single(
        &mut self,
                wgpu_render: &mut WgpuRenderState,
        tile: &TileComponent,
        position: &PositionComponent,
    ) {
         // intentionally left blank
     }

    /// Applies the system's business logic to the given entity components.
    ///
    /// ### Reads
    /// 
    /// - `tile`: A slice of the input components of type [`TileComponent`].
    /// - `position`: A slice of the input components of type [`PositionComponent`].
    ///
    /// ### Mutates
    /// 
    fn apply_many(
        &mut self,
                wgpu_render: &mut WgpuRenderState,
        tiles: &[TileComponent],
        positions: &[PositionComponent],
    ) {
        let zipped_iter = tiles.iter().zip(positions.iter());
        for (tile, position) in zipped_iter {
            self.apply_single(
                        wgpu_render,
                tile,
                position,
            );
        }
    }

    /// Applies the system's business logic to the given entity components.
    ///
    /// ### Reads
    /// 
    /// - `tile`: A slice of the input components of type [`TileComponent`].
    /// - `position`: A slice of the input components of type [`PositionComponent`].
    ///
    /// ### Mutates
    /// 
    #[allow(unused_mut)]
    #[inline]
    fn apply_all(
        &mut self,
                wgpu_render: &mut WgpuRenderState,
        tiles: [&[TileComponent]; 1],
        positions: [&[PositionComponent]; 1],
    ) {
        let zipped_iter = tiles.iter().zip(positions.iter());
        for (tile, position) in zipped_iter {
            ApplyTileRenderSystem::apply_many(
                self,
                        wgpu_render,
                tile,
                position,
            );
        }
    }
}

#[allow(dead_code)]
impl TileRenderSystem {
    /// Applies the system's business logic to the given entity components.
    ///
    /// ### Reads
    /// 
    /// - `tile`: A slice of the input components of type [`TileComponent`].
    /// - `position`: A slice of the input components of type [`PositionComponent`].
    ///
    /// ### Mutates
    /// 
    #[inline(always)]
    fn apply_many(
        &mut self,
        wgpu_render: &mut WgpuRenderState,
        tiles: &[TileComponent],
        positions: &[PositionComponent],
    ) {
        // Force the implementation of the ApplyTileRenderSystem trait.
        ApplyTileRenderSystem::apply_many(
            self,
            wgpu_render,
            tiles,
            positions,
        );
    }

    /// Applies the system's business logic to the given entity components.
    ///
    /// ### Reads
    /// 
    /// - `tile`: A slice of the input components of type [`TileComponent`].
    /// - `position`: A slice of the input components of type [`PositionComponent`].
    ///
    /// ### Mutates
    /// 
    #[allow(unused_mut)]
    #[inline]
    fn apply_all(
        &mut self,
        wgpu_render: &mut WgpuRenderState,
        tiles: [&[TileComponent]; 1],
        positions: [&[PositionComponent]; 1],
    ) {
        let zipped_iter = tiles.iter().zip(positions.iter());
        for (tile, position) in zipped_iter {
            self.apply_many(
                wgpu_render,
                tile,
                position,
            );
        }
    }
}

#[automatically_derived]
impl System for TileRenderSystem {
    const ID: SystemId = SystemId::TileRender;
}

impl PartialEq<SystemId> for TileRenderSystem {
    fn eq(&self, other: &SystemId) -> bool {
        Self::ID.eq(other)
    }
}

#[automatically_derived]
impl From<TileRenderSystemData> for TileRenderSystem {
    fn from(data: TileRenderSystemData) -> Self {
        Self(data)
    }
}

#[automatically_derived]
impl core::ops::Deref for TileRenderSystem {
    type Target = TileRenderSystemData;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

#[automatically_derived]
impl core::ops::DerefMut for TileRenderSystem {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

/// A system operating on multiple [`Component`]s.
///
/// The business logic is implemented via the [`ApplyDynamicLightSystem`] trait.
/// See also [`DynamicLightData`] for the systems-specific data.
///
/// ### Reads
/// 
/// - [`ColorComponent`]
///
/// ### Mutates
/// 
/// - [`DynamicLightComponent`]
///
/// # Archetype Use
/// This component processes components in the following archetypes:
/// 
/// - [`DynamicLightArchetype`] ([`ArchetypeId::DynamicLight`])
#[derive(Debug)]
pub struct DynamicLightSystem(DynamicLightSystemData);

/// Implements the business logic for the [`DynamicLightSystem`] system.
#[allow(dead_code)]
pub trait ApplyDynamicLightSystem: System {
    type Error: core::error::Error;

    /// Indicates whether this system is ready for action.
    ///
    /// Always called once per phase before all other functions of this system. If it returns
    /// `true`, the remaining functions are executed, otherwise the system skips the phase.
    #[inline]
    #[allow(unused_variables)]
    fn is_ready(
        &self,
        context: &::sillyecs::FrameContext,
                wgpu_render: &mut WgpuRenderState,
    ) -> bool {
        true
    }

    /// Executed at the beginning of the phase (for this system).
    ///
    /// All calls to [`apply_many`](Self::apply_many) and/or [`apply_single`](Self::apply_single) happen after this
    /// function was executed.
    #[inline]
    #[allow(unused_variables)]
    fn on_begin_phase(
        &mut self,
        context: &::sillyecs::FrameContext,
                wgpu_render: &mut WgpuRenderState,
    ) -> Result<(), Self::Error> {
        Ok(())
    }

    /// Executed at the end of the phase (for this system).
    ///
    /// All calls to [`apply_many`](Self::apply_many) and/or [`apply_single`](Self::apply_single) happen before this
    /// function is executed.
    #[inline]
    #[allow(unused_variables)]
    fn on_end_phase(
        &mut self,
        context: &::sillyecs::FrameContext,
                wgpu_render: &mut WgpuRenderState,
    ) -> Result<(), Self::Error> {
        Ok(())
    }

    /// Applies the system's business logic to the given entity components.
    ///
    /// ### Performance Considerations
    /// Prefer implementing [`apply_many`](Self::apply_many) directly.
    ///
    /// ### Reads
    /// 
    /// - `color`: A reference to the input component of type [`ColorComponent`].
    ///
    /// ### Mutates
    /// 
    /// - `dynamic_light`: A mutable reference to the component of type [`DynamicLightComponent`].
    #[allow(unused_variables)]
    fn apply_single(
        &mut self,
        context: &::sillyecs::FrameContext,
                wgpu_render: &mut WgpuRenderState,
        color: &ColorComponent,
        dynamic_light: &mut DynamicLightComponent,
    ) {
         // intentionally left blank
     }

    /// Applies the system's business logic to the given entity components.
    ///
    /// ### Reads
    /// 
    /// - `color`: A slice of the input components of type [`ColorComponent`].
    ///
    /// ### Mutates
    /// 
    /// - `dynamic_light`: A mutable slice of the components of type [`DynamicLightComponent`].
    fn apply_many(
        &mut self,
        context: &::sillyecs::FrameContext,
                wgpu_render: &mut WgpuRenderState,
        colors: &[ColorComponent],
        dynamic_lights: &mut [DynamicLightComponent],
    ) {
        let zipped_iter = colors.iter().zip(dynamic_lights.iter_mut());
        for (color, dynamic_light) in zipped_iter {
            self.apply_single(
                &context,
                        wgpu_render,
                color,
                dynamic_light,
            );
        }
    }

    /// Applies the system's business logic to the given entity components.
    ///
    /// ### Reads
    /// 
    /// - `color`: A slice of the input components of type [`ColorComponent`].
    ///
    /// ### Mutates
    /// 
    /// - `dynamic_light`: A mutable slice of the components of type [`DynamicLightComponent`].
    #[allow(unused_mut)]
    #[inline]
    fn apply_all(
        &mut self,
        context: &::sillyecs::FrameContext,
                wgpu_render: &mut WgpuRenderState,
        colors: [&[ColorComponent]; 1],
        mut dynamic_lights: [&mut [DynamicLightComponent]; 1],
    ) {
        let zipped_iter = colors.iter().zip(dynamic_lights.iter_mut());
        for (color, dynamic_light) in zipped_iter {
            ApplyDynamicLightSystem::apply_many(
                self,
                context,
                        wgpu_render,
                color,
                dynamic_light,
            );
        }
    }
}

#[allow(dead_code)]
impl DynamicLightSystem {
    /// Applies the system's business logic to the given entity components.
    ///
    /// ### Reads
    /// 
    /// - `color`: A slice of the input components of type [`ColorComponent`].
    ///
    /// ### Mutates
    /// 
    /// - `dynamic_light`: A mutable slice of the components of type [`DynamicLightComponent`].
    #[inline(always)]
    fn apply_many(
        &mut self,
        context: &::sillyecs::FrameContext,
        wgpu_render: &mut WgpuRenderState,
        colors: &[ColorComponent],
        dynamic_lights: &mut [DynamicLightComponent],
    ) {
        // Force the implementation of the ApplyDynamicLightSystem trait.
        ApplyDynamicLightSystem::apply_many(
            self,
            context,
            wgpu_render,
            colors,
            dynamic_lights,
        );
    }

    /// Applies the system's business logic to the given entity components.
    ///
    /// ### Reads
    /// 
    /// - `color`: A slice of the input components of type [`ColorComponent`].
    ///
    /// ### Mutates
    /// 
    /// - `dynamic_light`: A mutable slice of the components of type [`DynamicLightComponent`].
    #[allow(unused_mut)]
    #[inline]
    fn apply_all(
        &mut self,
        context: &::sillyecs::FrameContext,
        wgpu_render: &mut WgpuRenderState,
        colors: [&[ColorComponent]; 1],
        mut dynamic_lights: [&mut [DynamicLightComponent]; 1],
    ) {
        let zipped_iter = colors.iter().zip(dynamic_lights.iter_mut());
        for (color, dynamic_light) in zipped_iter {
            self.apply_many(
                context,
                wgpu_render,
                color,
                dynamic_light,
            );
        }
    }
}

#[automatically_derived]
impl System for DynamicLightSystem {
    const ID: SystemId = SystemId::DynamicLight;
}

impl PartialEq<SystemId> for DynamicLightSystem {
    fn eq(&self, other: &SystemId) -> bool {
        Self::ID.eq(other)
    }
}

#[automatically_derived]
impl From<DynamicLightSystemData> for DynamicLightSystem {
    fn from(data: DynamicLightSystemData) -> Self {
        Self(data)
    }
}

#[automatically_derived]
impl core::ops::Deref for DynamicLightSystem {
    type Target = DynamicLightSystemData;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

#[automatically_derived]
impl core::ops::DerefMut for DynamicLightSystem {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}


/// If retro rendering is enabled, this system blits the VGA offscreen buffer to the screen
///
/// The business logic is implemented via the [`ApplyVgaBlitSystem`] trait.
/// See also [`VgaBlitData`] for the systems-specific data.
///
/// ### Reads
/// 
/// - [`BackgroundColorComponent`]
///
/// # Archetype Use
/// This component processes components in the following archetypes:
/// 
/// - [`SceneBackgroundArchetype`] ([`ArchetypeId::SceneBackground`])
#[derive(Debug)]
pub struct VgaBlitSystem(VgaBlitSystemData);

/// Implements the business logic for the [`VgaBlitSystem`] system.
#[allow(dead_code)]
pub trait ApplyVgaBlitSystem: System {
    type Error: core::error::Error;

    /// Indicates whether this system is ready for action.
    ///
    /// Always called once per phase before all other functions of this system. If it returns
    /// `true`, the remaining functions are executed, otherwise the system skips the phase.
    #[inline]
    #[allow(unused_variables)]
    fn is_ready(
        &self,
                wgpu_render: &mut WgpuRenderState,
    ) -> bool {
        true
    }

    /// Executed at the beginning of the phase (for this system).
    ///
    /// All calls to [`apply_many`](Self::apply_many) and/or [`apply_single`](Self::apply_single) happen after this
    /// function was executed.
    #[inline]
    #[allow(unused_variables)]
    fn on_begin_phase(
        &mut self,
                wgpu_render: &mut WgpuRenderState,
    ) -> Result<(), Self::Error> {
        Ok(())
    }

    /// Executed at the end of the phase (for this system).
    ///
    /// All calls to [`apply_many`](Self::apply_many) and/or [`apply_single`](Self::apply_single) happen before this
    /// function is executed.
    #[inline]
    #[allow(unused_variables)]
    fn on_end_phase(
        &mut self,
                wgpu_render: &mut WgpuRenderState,
    ) -> Result<(), Self::Error> {
        Ok(())
    }

    /// Applies the system's business logic to the given entity components.
    ///
    /// ### Performance Considerations
    /// Prefer implementing [`apply_many`](Self::apply_many) directly.
    ///
    /// ### Reads
    /// 
    /// - `background_color`: A reference to the input component of type [`BackgroundColorComponent`].
    ///
    /// ### Mutates
    /// 
    #[allow(unused_variables)]
    fn apply_single(
        &mut self,
                wgpu_render: &mut WgpuRenderState,
        background_color: &BackgroundColorComponent,
    ) {
         // intentionally left blank
     }

    /// Applies the system's business logic to the given entity components.
    ///
    /// ### Reads
    /// 
    /// - `background_color`: A slice of the input components of type [`BackgroundColorComponent`].
    ///
    /// ### Mutates
    /// 
    fn apply_many(
        &mut self,
                wgpu_render: &mut WgpuRenderState,
        background_colors: &[BackgroundColorComponent],
    ) {
        let zipped_iter = background_colors;
        for background_color in zipped_iter {
            self.apply_single(
                        wgpu_render,
                background_color,
            );
        }
    }

    /// Applies the system's business logic to the given entity components.
    ///
    /// ### Reads
    /// 
    /// - `background_color`: A slice of the input components of type [`BackgroundColorComponent`].
    ///
    /// ### Mutates
    /// 
    #[allow(unused_mut)]
    #[inline]
    fn apply_all(
        &mut self,
                wgpu_render: &mut WgpuRenderState,
        background_colors: [&[BackgroundColorComponent]; 1],
    ) {
        let zipped_iter = background_colors;
        for background_color in zipped_iter {
            ApplyVgaBlitSystem::apply_many(
                self,
                        wgpu_render,
                background_color,
            );
        }
    }
}

#[allow(dead_code)]
impl VgaBlitSystem {
    /// Applies the system's business logic to the given entity components.
    ///
    /// ### Reads
    /// 
    /// - `background_color`: A slice of the input components of type [`BackgroundColorComponent`].
    ///
    /// ### Mutates
    /// 
    #[inline(always)]
    fn apply_many(
        &mut self,
        wgpu_render: &mut WgpuRenderState,
        background_colors: &[BackgroundColorComponent],
    ) {
        // Force the implementation of the ApplyVgaBlitSystem trait.
        ApplyVgaBlitSystem::apply_many(
            self,
            wgpu_render,
            background_colors,
        );
    }

    /// Applies the system's business logic to the given entity components.
    ///
    /// ### Reads
    /// 
    /// - `background_color`: A slice of the input components of type [`BackgroundColorComponent`].
    ///
    /// ### Mutates
    /// 
    #[allow(unused_mut)]
    #[inline]
    fn apply_all(
        &mut self,
        wgpu_render: &mut WgpuRenderState,
        background_colors: [&[BackgroundColorComponent]; 1],
    ) {
        let zipped_iter = background_colors;
        for background_color in zipped_iter {
            self.apply_many(
                wgpu_render,
                background_color,
            );
        }
    }
}

#[automatically_derived]
impl System for VgaBlitSystem {
    const ID: SystemId = SystemId::VgaBlit;
}

impl PartialEq<SystemId> for VgaBlitSystem {
    fn eq(&self, other: &SystemId) -> bool {
        Self::ID.eq(other)
    }
}

#[automatically_derived]
impl From<VgaBlitSystemData> for VgaBlitSystem {
    fn from(data: VgaBlitSystemData) -> Self {
        Self(data)
    }
}

#[automatically_derived]
impl core::ops::Deref for VgaBlitSystem {
    type Target = VgaBlitSystemData;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

#[automatically_derived]
impl core::ops::DerefMut for VgaBlitSystem {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}