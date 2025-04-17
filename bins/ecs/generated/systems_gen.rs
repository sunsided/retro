/// The ID of a [`System`].
#[derive(Debug, Copy, Clone, Ord, PartialOrd, Eq, PartialEq)]
#[repr(u32)]
pub enum SystemId {
    /// The [`WgpuInitShader`](WgpuInitShaderSystem) system.
    ///
    /// ### Mutates
    /// 
    /// - [`WgpuShaderComponent`]
    ///
    /// # Archetype Use
    /// This component processes components in the following archetypes:
    /// 
    /// - [`WgpuShaderArchetype`] ([`ArchetypeId::WgpuShader`])
    WgpuInitShader = 1,
    /// The [`WgpuInitTexture`](WgpuInitTextureSystem) system.
    ///
    /// ### Reads
    /// 
    /// - [`TextureSourceComponent`]
    ///
    /// ### Mutates
    /// 
    /// - [`WgpuTextureComponent`]
    ///
    /// # Archetype Use
    /// This component processes components in the following archetypes:
    /// 
    /// - [`WgpuTextureArchetype`] ([`ArchetypeId::WgpuTexture`])
    WgpuInitTexture = 2,
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
    Camera = 3,
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
    TimeOfDay = 4,
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
    Sun = 5,
    /// The [`Fog`](FogSystem) system.
    ///
    /// ### Reads
    /// 
    /// - [`TimeOfDayComponent`]
    ///
    /// ### Mutates
    /// 
    /// - [`FogComponent`]
    ///
    /// # Archetype Use
    /// This component processes components in the following archetypes:
    /// 
    /// - [`EnvironmentArchetype`] ([`ArchetypeId::Environment`])
    Fog = 6,
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
    Stars = 7,
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
    Physics = 8,
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
    BackgroundColorCycle = 9,
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
    CameraRender = 10,
    /// The [`FrameGlobals`](FrameGlobalsSystem) system.
    ///
    /// ### Reads
    /// 
    /// - [`TimeOfDayComponent`]
    /// - [`SunComponent`]
    /// - [`FogComponent`]
    /// - [`AmbientLightComponent`]
    /// - [`StarsComponent`]
    /// - [`SkydomeComponent`]
    ///
    /// # Archetype Use
    /// This component processes components in the following archetypes:
    /// 
    /// - [`EnvironmentArchetype`] ([`ArchetypeId::Environment`])
    FrameGlobals = 11,
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
    RenderBackground = 12,
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
    RenderSky = 13,
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
    Render = 14,
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
    TileRender = 15,
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
    DynamicLight = 16,
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
    VgaBlit = 17,
}

impl SystemId {
    /// A [`NonZeroU64`](core::num::NonZeroU64) representation of [`SystemId::WgpuInitShader`](SystemId::WgpuInitShader) system.
    pub const WGPU_INIT_SHADER_ID_VALUE: core::num::NonZeroU64 = core::num::NonZeroU64::new(1).expect("Error at ECS construction time");
    /// A [`NonZeroU64`](core::num::NonZeroU64) representation of [`SystemId::WgpuInitTexture`](SystemId::WgpuInitTexture) system.
    pub const WGPU_INIT_TEXTURE_ID_VALUE: core::num::NonZeroU64 = core::num::NonZeroU64::new(2).expect("Error at ECS construction time");
    /// A [`NonZeroU64`](core::num::NonZeroU64) representation of [`SystemId::Camera`](SystemId::Camera) system.
    pub const CAMERA_ID_VALUE: core::num::NonZeroU64 = core::num::NonZeroU64::new(3).expect("Error at ECS construction time");
    /// A [`NonZeroU64`](core::num::NonZeroU64) representation of [`SystemId::TimeOfDay`](SystemId::TimeOfDay) system.
    pub const TIME_OF_DAY_ID_VALUE: core::num::NonZeroU64 = core::num::NonZeroU64::new(4).expect("Error at ECS construction time");
    /// A [`NonZeroU64`](core::num::NonZeroU64) representation of [`SystemId::Sun`](SystemId::Sun) system.
    pub const SUN_ID_VALUE: core::num::NonZeroU64 = core::num::NonZeroU64::new(5).expect("Error at ECS construction time");
    /// A [`NonZeroU64`](core::num::NonZeroU64) representation of [`SystemId::Fog`](SystemId::Fog) system.
    pub const FOG_ID_VALUE: core::num::NonZeroU64 = core::num::NonZeroU64::new(6).expect("Error at ECS construction time");
    /// A [`NonZeroU64`](core::num::NonZeroU64) representation of [`SystemId::Stars`](SystemId::Stars) system.
    pub const STARS_ID_VALUE: core::num::NonZeroU64 = core::num::NonZeroU64::new(7).expect("Error at ECS construction time");
    /// A [`NonZeroU64`](core::num::NonZeroU64) representation of [`SystemId::Physics`](SystemId::Physics) system.
    pub const PHYSICS_ID_VALUE: core::num::NonZeroU64 = core::num::NonZeroU64::new(8).expect("Error at ECS construction time");
    /// A [`NonZeroU64`](core::num::NonZeroU64) representation of [`SystemId::BackgroundColorCycle`](SystemId::BackgroundColorCycle) system.
    pub const BACKGROUND_COLOR_CYCLE_ID_VALUE: core::num::NonZeroU64 = core::num::NonZeroU64::new(9).expect("Error at ECS construction time");
    /// A [`NonZeroU64`](core::num::NonZeroU64) representation of [`SystemId::CameraRender`](SystemId::CameraRender) system.
    pub const CAMERA_RENDER_ID_VALUE: core::num::NonZeroU64 = core::num::NonZeroU64::new(10).expect("Error at ECS construction time");
    /// A [`NonZeroU64`](core::num::NonZeroU64) representation of [`SystemId::FrameGlobals`](SystemId::FrameGlobals) system.
    pub const FRAME_GLOBALS_ID_VALUE: core::num::NonZeroU64 = core::num::NonZeroU64::new(11).expect("Error at ECS construction time");
    /// A [`NonZeroU64`](core::num::NonZeroU64) representation of [`SystemId::RenderBackground`](SystemId::RenderBackground) system.
    pub const RENDER_BACKGROUND_ID_VALUE: core::num::NonZeroU64 = core::num::NonZeroU64::new(12).expect("Error at ECS construction time");
    /// A [`NonZeroU64`](core::num::NonZeroU64) representation of [`SystemId::RenderSky`](SystemId::RenderSky) system.
    pub const RENDER_SKY_ID_VALUE: core::num::NonZeroU64 = core::num::NonZeroU64::new(13).expect("Error at ECS construction time");
    /// A [`NonZeroU64`](core::num::NonZeroU64) representation of [`SystemId::Render`](SystemId::Render) system.
    pub const RENDER_ID_VALUE: core::num::NonZeroU64 = core::num::NonZeroU64::new(14).expect("Error at ECS construction time");
    /// A [`NonZeroU64`](core::num::NonZeroU64) representation of [`SystemId::TileRender`](SystemId::TileRender) system.
    pub const TILE_RENDER_ID_VALUE: core::num::NonZeroU64 = core::num::NonZeroU64::new(15).expect("Error at ECS construction time");
    /// A [`NonZeroU64`](core::num::NonZeroU64) representation of [`SystemId::DynamicLight`](SystemId::DynamicLight) system.
    pub const DYNAMIC_LIGHT_ID_VALUE: core::num::NonZeroU64 = core::num::NonZeroU64::new(16).expect("Error at ECS construction time");
    /// A [`NonZeroU64`](core::num::NonZeroU64) representation of [`SystemId::VgaBlit`](SystemId::VgaBlit) system.
    pub const VGA_BLIT_ID_VALUE: core::num::NonZeroU64 = core::num::NonZeroU64::new(17).expect("Error at ECS construction time");
}

impl SystemId {
    /// Returns this ID as a [`NonZeroU64`](core::num::NonZeroU64) value.
    pub const fn as_nonzero_u64(&self) -> core::num::NonZeroU64 {
        match self {
            Self::WgpuInitShader => Self::WGPU_INIT_SHADER_ID_VALUE,
            Self::WgpuInitTexture => Self::WGPU_INIT_TEXTURE_ID_VALUE,
            Self::Camera => Self::CAMERA_ID_VALUE,
            Self::TimeOfDay => Self::TIME_OF_DAY_ID_VALUE,
            Self::Sun => Self::SUN_ID_VALUE,
            Self::Fog => Self::FOG_ID_VALUE,
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
            Self::WgpuInitShader => write!(f, "WgpuInitShader (ID 1)"),
            Self::WgpuInitTexture => write!(f, "WgpuInitTexture (ID 2)"),
            Self::Camera => write!(f, "Camera (ID 3)"),
            Self::TimeOfDay => write!(f, "TimeOfDay (ID 4)"),
            Self::Sun => write!(f, "Sun (ID 5)"),
            Self::Fog => write!(f, "Fog (ID 6)"),
            Self::Stars => write!(f, "Stars (ID 7)"),
            Self::Physics => write!(f, "Physics (ID 8)"),
            Self::BackgroundColorCycle => write!(f, "BackgroundColorCycle (ID 9)"),
            Self::CameraRender => write!(f, "CameraRender (ID 10)"),
            Self::FrameGlobals => write!(f, "FrameGlobals (ID 11)"),
            Self::RenderBackground => write!(f, "RenderBackground (ID 12)"),
            Self::RenderSky => write!(f, "RenderSky (ID 13)"),
            Self::Render => write!(f, "Render (ID 14)"),
            Self::TileRender => write!(f, "TileRender (ID 15)"),
            Self::DynamicLight => write!(f, "DynamicLight (ID 16)"),
            Self::VgaBlit => write!(f, "VgaBlit (ID 17)"),
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
    /// The `WgpuReinit` system phase.
    ///
    /// This phase is executed exactly once per frame.
    WgpuReinit = 1,
    /// The `FixedUpdate` system phase.
    ///
    /// This phase runs using a **fixed-time loop** of approximately 60.0 Hz (i.e., every 0.0167 seconds) and may iterate zero or multiple times per frame.
    FixedUpdate = 2,
    /// The `Update` system phase.
    ///
    /// This phase is executed exactly once per frame.
    Update = 3,
    /// The `Render` system phase.
    ///
    /// This phase is executed exactly once per frame.
    Render = 4,
}

#[allow(dead_code)]
impl SystemPhase {
    /// The number of system phases.
    pub const COUNT: usize = 5;

    /// The number of seconds the [`FixedUpdate`](SystemPhase::FixedUpdate) fixed-time step should run.
    pub const FIXED_UPDATE_SECS: f32 = 0.01666666753590107;

    /// The frequency of the [`FixedUpdate`](SystemPhase::FixedUpdate) fixed-time step.
    pub const FIXED_UPDATE_HZ: f32 = 60.0;

    /// Returns the number of seconds between the fixed time steps if this phase
    /// is configured to run in fixed time, otherwise [`None`].
    pub const fn fixed_secs(&self) -> Option<f32> {
        match self {
            Self::Startup => None,
            Self::WgpuReinit => None,
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
            Self::WgpuReinit => None,
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
pub trait CreateSystems: CreateSystem<WgpuInitShaderSystem> + CreateSystem<WgpuInitTextureSystem> + CreateSystem<CameraSystem> + CreateSystem<TimeOfDaySystem> + CreateSystem<SunSystem> + CreateSystem<FogSystem> + CreateSystem<StarsSystem> + CreateSystem<PhysicsSystem> + CreateSystem<BackgroundColorCycleSystem> + CreateSystem<CameraRenderSystem> + CreateSystem<FrameGlobalsSystem> + CreateSystem<RenderBackgroundSystem> + CreateSystem<RenderSkySystem> + CreateSystem<RenderSystem> + CreateSystem<TileRenderSystem> + CreateSystem<DynamicLightSystem> + CreateSystem<VgaBlitSystem>
{ }

#[automatically_derived]
impl<T> CreateSystems for T
where
    T: CreateSystem<WgpuInitShaderSystem>
     + CreateSystem<WgpuInitTextureSystem>
     + CreateSystem<CameraSystem>
     + CreateSystem<TimeOfDaySystem>
     + CreateSystem<SunSystem>
     + CreateSystem<FogSystem>
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
pub trait SystemPhaseEvents: SystemStartupPhaseEvents + SystemWgpuReinitPhaseEvents + SystemFixedUpdatePhaseEvents + SystemUpdatePhaseEvents + SystemRenderPhaseEvents
{
}

#[automatically_derived]
impl<T> SystemPhaseEvents for T
where
    T: SystemStartupPhaseEvents
     + SystemWgpuReinitPhaseEvents
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
        _context: &FrameContext,
    ) -> Result<(), Self::Error> {
        Ok(())
    }

    #[inline]
    fn on_end_phase(
        &mut self,
        _result: Option<Self::Result>,
        _context: &FrameContext,
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
        context: &FrameContext,
    ) -> Result<Self::Result, Self::Error>;

    /// Executed at the end of the phase (after all systems).
    ///
    /// The result from [`SystemStartupPhaseEvents::on_begin_phase`] is passed in
    /// as [`Some`] value, if the executed succeeded. If the function failed, the value will be [`None`].
    fn on_end_phase(
        &mut self,
        result: Option<Self::Result>,
        context: &FrameContext,
    ) -> Result<(), Self::Error>;
}

impl SystemWgpuReinitPhaseEvents for NoOpPhaseEvents {
    type Result = ();
    type Error = core::convert::Infallible;

    #[inline]
    fn on_begin_phase(
        &mut self,
        _context: &FrameContext,
    ) -> Result<(), Self::Error> {
        Ok(())
    }

    #[inline]
    fn on_end_phase(
        &mut self,
        _result: Option<Self::Result>,
        _context: &FrameContext,
    ) -> Result<(), Self::Error> {
        Ok(())
    }
}

pub trait SystemWgpuReinitPhaseEvents {
    type Result;
    type Error: core::error::Error;

    /// The name of the phase.
    const NAME: &'static str = "WgpuReinit";

    /// Executed at the beginning of the phase (before all systems).
    fn on_begin_phase(
        &mut self,
        context: &FrameContext,
    ) -> Result<Self::Result, Self::Error>;

    /// Executed at the end of the phase (after all systems).
    ///
    /// The result from [`SystemWgpuReinitPhaseEvents::on_begin_phase`] is passed in
    /// as [`Some`] value, if the executed succeeded. If the function failed, the value will be [`None`].
    fn on_end_phase(
        &mut self,
        result: Option<Self::Result>,
        context: &FrameContext,
    ) -> Result<(), Self::Error>;
}

impl SystemFixedUpdatePhaseEvents for NoOpPhaseEvents {
    type Result = ();
    type Error = core::convert::Infallible;

    #[inline]
    fn on_begin_phase(
        &mut self,
        _context: &FrameContext,
    ) -> Result<(), Self::Error> {
        Ok(())
    }

    #[inline]
    fn on_end_phase(
        &mut self,
        _result: Option<Self::Result>,
        _context: &FrameContext,
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
        context: &FrameContext,
    ) -> Result<Self::Result, Self::Error>;

    /// Executed at the end of the phase (after all systems).
    ///
    /// The result from [`SystemFixedUpdatePhaseEvents::on_begin_phase`] is passed in
    /// as [`Some`] value, if the executed succeeded. If the function failed, the value will be [`None`].
    fn on_end_phase(
        &mut self,
        result: Option<Self::Result>,
        context: &FrameContext,
    ) -> Result<(), Self::Error>;
}

impl SystemUpdatePhaseEvents for NoOpPhaseEvents {
    type Result = ();
    type Error = core::convert::Infallible;

    #[inline]
    fn on_begin_phase(
        &mut self,
        _context: &FrameContext,
    ) -> Result<(), Self::Error> {
        Ok(())
    }

    #[inline]
    fn on_end_phase(
        &mut self,
        _result: Option<Self::Result>,
        _context: &FrameContext,
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
        context: &FrameContext,
    ) -> Result<Self::Result, Self::Error>;

    /// Executed at the end of the phase (after all systems).
    ///
    /// The result from [`SystemUpdatePhaseEvents::on_begin_phase`] is passed in
    /// as [`Some`] value, if the executed succeeded. If the function failed, the value will be [`None`].
    fn on_end_phase(
        &mut self,
        result: Option<Self::Result>,
        context: &FrameContext,
    ) -> Result<(), Self::Error>;
}

impl SystemRenderPhaseEvents for NoOpPhaseEvents {
    type Result = ();
    type Error = core::convert::Infallible;

    #[inline]
    fn on_begin_phase(
        &mut self,
        _context: &FrameContext,
                _wgpu_render: &mut WgpuRenderState,
    ) -> Result<(), Self::Error> {
        Ok(())
    }

    #[inline]
    fn on_end_phase(
        &mut self,
        _result: Option<Self::Result>,
        _context: &FrameContext,
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
        context: &FrameContext,
                wgpu_render: &mut WgpuRenderState,
    ) -> Result<Self::Result, Self::Error>;

    /// Executed at the end of the phase (after all systems).
    ///
    /// The result from [`SystemRenderPhaseEvents::on_begin_phase`] is passed in
    /// as [`Some`] value, if the executed succeeded. If the function failed, the value will be [`None`].
    fn on_end_phase(
        &mut self,
        result: Option<Self::Result>,
        context: &FrameContext,
                wgpu_render: &mut WgpuRenderState,
    ) -> Result<(), Self::Error>;
}

/// A system operating on multiple [`Component`]s.
///
/// The business logic is implemented via the [`ApplyWgpuInitShaderSystem`] trait.
/// See also [`WgpuInitShaderData`] for the systems-specific data.
///
/// ### Mutates
/// 
/// - [`WgpuShaderComponent`]
///
/// # Archetype Use
/// This component processes components in the following archetypes:
/// 
/// - [`WgpuShaderArchetype`] ([`ArchetypeId::WgpuShader`])
#[derive(Debug)]
pub struct WgpuInitShaderSystem(WgpuInitShaderSystemData);

/// Implements the business logic for the [`WgpuInitShaderSystem`] system.
#[allow(dead_code)]
pub trait ApplyWgpuInitShaderSystem: System {
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
                
                resource_manager: &mut ResourceManagerState,
    ) -> Result<(), Self::Error> {
        Ok(())
    }

    /// Inspects all components after the actual operation finished.
    ///
    /// This function can be called multiple times, depending on the number of archetypes providing
    /// the necessary components.
    ///
    /// ### Reads
    /// 
    ///
    /// ### Reads output to be mutated by the system
    /// 
    /// - `wgpu_shader`: A mutable slice of the components of type [`WgpuShaderComponent`].
    #[allow(unused_variables)]
    fn postflight(
        &mut self,
                
                resource_manager: &mut ResourceManagerState,
        entities: &[EntityId],
        wgpu_shaders: &[WgpuShaderComponent],
    );

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
    /// - `wgpu_shader`: A mutable reference to the component of type [`WgpuShaderComponent`].
    #[allow(unused_variables)]
    fn apply_single(
        &mut self,
                wgpu_render: &mut WgpuRenderState,
                
        entity: EntityId,
        wgpu_shader: &mut WgpuShaderComponent,
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
    /// - `wgpu_shader`: A mutable slice of the components of type [`WgpuShaderComponent`].
    fn apply_many(
        &mut self,
                wgpu_render: &mut WgpuRenderState,
                
        entities: &[EntityId],
        wgpu_shaders: &mut [WgpuShaderComponent],
    ) {
        let zipped_iter = entities.iter().zip(wgpu_shaders.iter_mut());
        for (entity, wgpu_shader) in zipped_iter {
            self.apply_single(
                        wgpu_render,
                        
                *entity,
                wgpu_shader,
            );
        }
    }
}

#[allow(dead_code)]
impl WgpuInitShaderSystem {
    /// Apply the system to the given entity components
    #[inline]
    pub fn apply_single(
        &mut self,
                wgpu_render: &mut WgpuRenderState,
                
        entity: EntityId,
        wgpu_shader: &mut WgpuShaderComponent,
    ) {
        // Force the implementation of the ApplyWgpuInitShaderSystem trait.
        ApplyWgpuInitShaderSystem::apply_single(
            self,
                    wgpu_render,
                    
            entity,
            wgpu_shader,
        );
    }

    /// Applies the system's business logic to the given entity components.
    ///
    /// ### Reads
    /// 
    ///
    /// ### Mutates
    /// 
    /// - `wgpu_shader`: A mutable slice of the components of type [`WgpuShaderComponent`].
    #[inline]
    fn apply_many(
        &mut self,
                wgpu_render: &mut WgpuRenderState,
                
        entities: &[EntityId],
        wgpu_shaders: &mut [WgpuShaderComponent],
    ) {
        // Force the implementation of the ApplyWgpuInitShaderSystem trait.
        ApplyWgpuInitShaderSystem::apply_many(
            self,
                    wgpu_render,
                    
            entities,
            wgpu_shaders,
        );
    }
}

#[automatically_derived]
impl System for WgpuInitShaderSystem {
    const ID: SystemId = SystemId::WgpuInitShader;
}

impl PartialEq<SystemId> for WgpuInitShaderSystem {
    fn eq(&self, other: &SystemId) -> bool {
        Self::ID.eq(other)
    }
}

#[automatically_derived]
impl From<WgpuInitShaderSystemData> for WgpuInitShaderSystem {
    fn from(data: WgpuInitShaderSystemData) -> Self {
        Self(data)
    }
}

#[automatically_derived]
impl core::ops::Deref for WgpuInitShaderSystem {
    type Target = WgpuInitShaderSystemData;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

#[automatically_derived]
impl core::ops::DerefMut for WgpuInitShaderSystem {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

/// A system operating on multiple [`Component`]s.
///
/// The business logic is implemented via the [`ApplyWgpuInitTextureSystem`] trait.
/// See also [`WgpuInitTextureData`] for the systems-specific data.
///
/// ### Reads
/// 
/// - [`TextureSourceComponent`]
///
/// ### Mutates
/// 
/// - [`WgpuTextureComponent`]
///
/// # Archetype Use
/// This component processes components in the following archetypes:
/// 
/// - [`WgpuTextureArchetype`] ([`ArchetypeId::WgpuTexture`])
#[derive(Debug)]
pub struct WgpuInitTextureSystem(WgpuInitTextureSystemData);

/// Implements the business logic for the [`WgpuInitTextureSystem`] system.
#[allow(dead_code)]
pub trait ApplyWgpuInitTextureSystem: System {
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
                
                resource_manager: &mut ResourceManagerState,
    ) -> Result<(), Self::Error> {
        Ok(())
    }

    /// Inspects all components after the actual operation finished.
    ///
    /// This function can be called multiple times, depending on the number of archetypes providing
    /// the necessary components.
    ///
    /// ### Reads
    /// 
    /// - `texture_source`: A slice of the input components of type [`TextureSourceComponent`].
    ///
    /// ### Reads output to be mutated by the system
    /// 
    /// - `wgpu_texture`: A mutable slice of the components of type [`WgpuTextureComponent`].
    #[allow(unused_variables)]
    fn postflight(
        &mut self,
                
                resource_manager: &mut ResourceManagerState,
        entities: &[EntityId],
        texture_sources: &[TextureSourceComponent],
        wgpu_textures: &[WgpuTextureComponent],
    );

    /// Applies the system's business logic to the given entity components.
    ///
    /// ### Performance Considerations
    /// Prefer implementing [`apply_many`](Self::apply_many) directly.
    ///
    /// ### Reads
    /// 
    /// - `texture_source`: A reference to the input component of type [`TextureSourceComponent`].
    ///
    /// ### Mutates
    /// 
    /// - `wgpu_texture`: A mutable reference to the component of type [`WgpuTextureComponent`].
    #[allow(unused_variables)]
    fn apply_single(
        &mut self,
                wgpu_render: &mut WgpuRenderState,
                
        entity: EntityId,
        texture_source: &TextureSourceComponent,
        wgpu_texture: &mut WgpuTextureComponent,
    ) {
         // intentionally left blank
     }

    /// Applies the system's business logic to the given entity components.
    ///
    /// ### Reads
    /// 
    /// - `texture_source`: A slice of the input components of type [`TextureSourceComponent`].
    ///
    /// ### Mutates
    /// 
    /// - `wgpu_texture`: A mutable slice of the components of type [`WgpuTextureComponent`].
    fn apply_many(
        &mut self,
                wgpu_render: &mut WgpuRenderState,
                
        entities: &[EntityId],
        texture_sources: &[TextureSourceComponent],
        wgpu_textures: &mut [WgpuTextureComponent],
    ) {
        let zipped_iter = entities.iter().zip(texture_sources.iter().zip(wgpu_textures.iter_mut()));
        for (entity, (texture_source, wgpu_texture)) in zipped_iter {
            self.apply_single(
                        wgpu_render,
                        
                *entity,
                texture_source,
                wgpu_texture,
            );
        }
    }
}

#[allow(dead_code)]
impl WgpuInitTextureSystem {
    /// Apply the system to the given entity components
    #[inline]
    pub fn apply_single(
        &mut self,
                wgpu_render: &mut WgpuRenderState,
                
        entity: EntityId,
        texture_source: &TextureSourceComponent,
        wgpu_texture: &mut WgpuTextureComponent,
    ) {
        // Force the implementation of the ApplyWgpuInitTextureSystem trait.
        ApplyWgpuInitTextureSystem::apply_single(
            self,
                    wgpu_render,
                    
            entity,
            texture_source,
            wgpu_texture,
        );
    }

    /// Applies the system's business logic to the given entity components.
    ///
    /// ### Reads
    /// 
    /// - `texture_source`: A slice of the input components of type [`TextureSourceComponent`].
    ///
    /// ### Mutates
    /// 
    /// - `wgpu_texture`: A mutable slice of the components of type [`WgpuTextureComponent`].
    #[inline]
    fn apply_many(
        &mut self,
                wgpu_render: &mut WgpuRenderState,
                
        entities: &[EntityId],
        texture_sources: &[TextureSourceComponent],
        wgpu_textures: &mut [WgpuTextureComponent],
    ) {
        // Force the implementation of the ApplyWgpuInitTextureSystem trait.
        ApplyWgpuInitTextureSystem::apply_many(
            self,
                    wgpu_render,
                    
            entities,
            texture_sources,
            wgpu_textures,
        );
    }
}

#[automatically_derived]
impl System for WgpuInitTextureSystem {
    const ID: SystemId = SystemId::WgpuInitTexture;
}

impl PartialEq<SystemId> for WgpuInitTextureSystem {
    fn eq(&self, other: &SystemId) -> bool {
        Self::ID.eq(other)
    }
}

#[automatically_derived]
impl From<WgpuInitTextureSystemData> for WgpuInitTextureSystem {
    fn from(data: WgpuInitTextureSystemData) -> Self {
        Self(data)
    }
}

#[automatically_derived]
impl core::ops::Deref for WgpuInitTextureSystem {
    type Target = WgpuInitTextureSystemData;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

#[automatically_derived]
impl core::ops::DerefMut for WgpuInitTextureSystem {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
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
}

#[allow(dead_code)]
impl CameraSystem {
    /// Apply the system to the given entity components
    #[inline]
    pub fn apply_single(
        &mut self,
                input: &InputState,
                wgpu_render: &WgpuRenderState,
        camera: &mut CameraComponent,
        camera_fps_controller: &mut CameraFpsControllerComponent,
    ) {
        // Force the implementation of the ApplyCameraSystem trait.
        ApplyCameraSystem::apply_single(
            self,
                    input,
                    wgpu_render,
            camera,
            camera_fps_controller,
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
    #[inline]
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
        context: &FrameContext,
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
        context: &FrameContext,
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
        context: &FrameContext,
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
        context: &FrameContext,
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
        context: &FrameContext,
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
}

#[allow(dead_code)]
impl TimeOfDaySystem {
    /// Apply the system to the given entity components
    #[inline]
    pub fn apply_single(
        &mut self,
        context: &FrameContext,
        time_of_day: &mut TimeOfDayComponent,
    ) {
        // Force the implementation of the ApplyTimeOfDaySystem trait.
        ApplyTimeOfDaySystem::apply_single(
            self,
            context,
            time_of_day,
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
    #[inline]
    fn apply_many(
        &mut self,
        context: &FrameContext,
        time_of_days: &mut [TimeOfDayComponent],
    ) {
        // Force the implementation of the ApplyTimeOfDaySystem trait.
        ApplyTimeOfDaySystem::apply_many(
            self,
            context,
            time_of_days,
        );
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
}

#[allow(dead_code)]
impl SunSystem {
    /// Apply the system to the given entity components
    #[inline]
    pub fn apply_single(
        &mut self,
        time_of_day: &TimeOfDayComponent,
        sun: &mut SunComponent,
        skydome: &mut SkydomeComponent,
        ambient_light: &mut AmbientLightComponent,
    ) {
        // Force the implementation of the ApplySunSystem trait.
        ApplySunSystem::apply_single(
            self,
            time_of_day,
            sun,
            skydome,
            ambient_light,
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
    #[inline]
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
/// The business logic is implemented via the [`ApplyFogSystem`] trait.
/// See also [`FogData`] for the systems-specific data.
///
/// ### Reads
/// 
/// - [`TimeOfDayComponent`]
///
/// ### Mutates
/// 
/// - [`FogComponent`]
///
/// # Archetype Use
/// This component processes components in the following archetypes:
/// 
/// - [`EnvironmentArchetype`] ([`ArchetypeId::Environment`])
#[derive(Debug)]
pub struct FogSystem(FogSystemData);

/// Implements the business logic for the [`FogSystem`] system.
#[allow(dead_code)]
pub trait ApplyFogSystem: System {
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
    /// - `fog`: A mutable reference to the component of type [`FogComponent`].
    #[allow(unused_variables)]
    fn apply_single(
        &mut self,
        time_of_day: &TimeOfDayComponent,
        fog: &mut FogComponent,
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
    /// - `fog`: A mutable slice of the components of type [`FogComponent`].
    fn apply_many(
        &mut self,
        time_of_days: &[TimeOfDayComponent],
        fogs: &mut [FogComponent],
    ) {
        let zipped_iter = time_of_days.iter().zip(fogs.iter_mut());
        for (time_of_day, fog) in zipped_iter {
            self.apply_single(
                time_of_day,
                fog,
            );
        }
    }
}

#[allow(dead_code)]
impl FogSystem {
    /// Apply the system to the given entity components
    #[inline]
    pub fn apply_single(
        &mut self,
        time_of_day: &TimeOfDayComponent,
        fog: &mut FogComponent,
    ) {
        // Force the implementation of the ApplyFogSystem trait.
        ApplyFogSystem::apply_single(
            self,
            time_of_day,
            fog,
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
    /// - `fog`: A mutable slice of the components of type [`FogComponent`].
    #[inline]
    fn apply_many(
        &mut self,
        time_of_days: &[TimeOfDayComponent],
        fogs: &mut [FogComponent],
    ) {
        // Force the implementation of the ApplyFogSystem trait.
        ApplyFogSystem::apply_many(
            self,
            time_of_days,
            fogs,
        );
    }
}

#[automatically_derived]
impl System for FogSystem {
    const ID: SystemId = SystemId::Fog;
}

impl PartialEq<SystemId> for FogSystem {
    fn eq(&self, other: &SystemId) -> bool {
        Self::ID.eq(other)
    }
}

#[automatically_derived]
impl From<FogSystemData> for FogSystem {
    fn from(data: FogSystemData) -> Self {
        Self(data)
    }
}

#[automatically_derived]
impl core::ops::Deref for FogSystem {
    type Target = FogSystemData;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

#[automatically_derived]
impl core::ops::DerefMut for FogSystem {
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
}

#[allow(dead_code)]
impl StarsSystem {
    /// Apply the system to the given entity components
    #[inline]
    pub fn apply_single(
        &mut self,
        time_of_day: &TimeOfDayComponent,
        stars: &mut StarsComponent,
    ) {
        // Force the implementation of the ApplyStarsSystem trait.
        ApplyStarsSystem::apply_single(
            self,
            time_of_day,
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
    #[inline]
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
        context: &FrameContext,
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
        context: &FrameContext,
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
        context: &FrameContext,
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
        context: &FrameContext,
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
        context: &FrameContext,
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
}

#[allow(dead_code)]
impl PhysicsSystem {
    /// Apply the system to the given entity components
    #[inline]
    pub fn apply_single(
        &mut self,
        context: &FrameContext,
        velocity: &VelocityComponent,
        position: &mut PositionComponent,
    ) {
        // Force the implementation of the ApplyPhysicsSystem trait.
        ApplyPhysicsSystem::apply_single(
            self,
            context,
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
        &mut self,
        context: &FrameContext,
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
        context: &FrameContext,
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
        context: &FrameContext,
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
        context: &FrameContext,
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
        context: &FrameContext,
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
        context: &FrameContext,
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
}

#[allow(dead_code)]
impl BackgroundColorCycleSystem {
    /// Apply the system to the given entity components
    #[inline]
    pub fn apply_single(
        &mut self,
        context: &FrameContext,
        background_color: &mut BackgroundColorComponent,
    ) {
        // Force the implementation of the ApplyBackgroundColorCycleSystem trait.
        ApplyBackgroundColorCycleSystem::apply_single(
            self,
            context,
            background_color,
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
    #[inline]
    fn apply_many(
        &mut self,
        context: &FrameContext,
        background_colors: &mut [BackgroundColorComponent],
    ) {
        // Force the implementation of the ApplyBackgroundColorCycleSystem trait.
        ApplyBackgroundColorCycleSystem::apply_many(
            self,
            context,
            background_colors,
        );
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
                active_camera: &ActiveCameraState,
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
                active_camera: &ActiveCameraState,
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
                active_camera: &ActiveCameraState,
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
}

#[allow(dead_code)]
impl CameraRenderSystem {
    /// Apply the system to the given entity components
    #[inline]
    pub fn apply_single(
        &mut self,
                wgpu_render: &WgpuRenderState,
                active_camera: &mut ActiveCameraState,
        camera: &CameraComponent,
    ) {
        // Force the implementation of the ApplyCameraRenderSystem trait.
        ApplyCameraRenderSystem::apply_single(
            self,
                    wgpu_render,
                    active_camera,
            camera,
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
    #[inline]
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
/// - [`SunComponent`]
/// - [`FogComponent`]
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
    /// - `time_of_day`: A reference to the input component of type [`TimeOfDayComponent`].
    /// - `sun`: A reference to the input component of type [`SunComponent`].
    /// - `fog`: A reference to the input component of type [`FogComponent`].
    /// - `ambient_light`: A reference to the input component of type [`AmbientLightComponent`].
    /// - `stars`: A reference to the input component of type [`StarsComponent`].
    /// - `skydome`: A reference to the input component of type [`SkydomeComponent`].
    ///
    /// ### Mutates
    /// 
    #[allow(unused_variables)]
    fn apply_single(
        &mut self,
                wgpu_render: &mut WgpuRenderState,
        time_of_day: &TimeOfDayComponent,
        sun: &SunComponent,
        fog: &FogComponent,
        ambient_light: &AmbientLightComponent,
        stars: &StarsComponent,
        skydome: &SkydomeComponent,
    ) {
         // intentionally left blank
     }

    /// Applies the system's business logic to the given entity components.
    ///
    /// ### Reads
    /// 
    /// - `time_of_day`: A slice of the input components of type [`TimeOfDayComponent`].
    /// - `sun`: A slice of the input components of type [`SunComponent`].
    /// - `fog`: A slice of the input components of type [`FogComponent`].
    /// - `ambient_light`: A slice of the input components of type [`AmbientLightComponent`].
    /// - `stars`: A slice of the input components of type [`StarsComponent`].
    /// - `skydome`: A slice of the input components of type [`SkydomeComponent`].
    ///
    /// ### Mutates
    /// 
    fn apply_many(
        &mut self,
                wgpu_render: &mut WgpuRenderState,
        time_of_days: &[TimeOfDayComponent],
        suns: &[SunComponent],
        fogs: &[FogComponent],
        ambient_lights: &[AmbientLightComponent],
        stars: &[StarsComponent],
        skydomes: &[SkydomeComponent],
    ) {
        let zipped_iter = time_of_days.iter().zip(suns.iter().zip(fogs.iter().zip(ambient_lights.iter().zip(stars.iter().zip(skydomes.iter())))));
        for (time_of_day, (sun, (fog, (ambient_light, (stars, skydome))))) in zipped_iter {
            self.apply_single(
                        wgpu_render,
                time_of_day,
                sun,
                fog,
                ambient_light,
                stars,
                skydome,
            );
        }
    }
}

#[allow(dead_code)]
impl FrameGlobalsSystem {
    /// Apply the system to the given entity components
    #[inline]
    pub fn apply_single(
        &mut self,
                wgpu_render: &mut WgpuRenderState,
        time_of_day: &TimeOfDayComponent,
        sun: &SunComponent,
        fog: &FogComponent,
        ambient_light: &AmbientLightComponent,
        stars: &StarsComponent,
        skydome: &SkydomeComponent,
    ) {
        // Force the implementation of the ApplyFrameGlobalsSystem trait.
        ApplyFrameGlobalsSystem::apply_single(
            self,
                    wgpu_render,
            time_of_day,
            sun,
            fog,
            ambient_light,
            stars,
            skydome,
        );
    }

    /// Applies the system's business logic to the given entity components.
    ///
    /// ### Reads
    /// 
    /// - `time_of_day`: A slice of the input components of type [`TimeOfDayComponent`].
    /// - `sun`: A slice of the input components of type [`SunComponent`].
    /// - `fog`: A slice of the input components of type [`FogComponent`].
    /// - `ambient_light`: A slice of the input components of type [`AmbientLightComponent`].
    /// - `stars`: A slice of the input components of type [`StarsComponent`].
    /// - `skydome`: A slice of the input components of type [`SkydomeComponent`].
    ///
    /// ### Mutates
    /// 
    #[inline]
    fn apply_many(
        &mut self,
                wgpu_render: &mut WgpuRenderState,
        time_of_days: &[TimeOfDayComponent],
        suns: &[SunComponent],
        fogs: &[FogComponent],
        ambient_lights: &[AmbientLightComponent],
        stars: &[StarsComponent],
        skydomes: &[SkydomeComponent],
    ) {
        // Force the implementation of the ApplyFrameGlobalsSystem trait.
        ApplyFrameGlobalsSystem::apply_many(
            self,
                    wgpu_render,
            time_of_days,
            suns,
            fogs,
            ambient_lights,
            stars,
            skydomes,
        );
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
}

#[allow(dead_code)]
impl RenderBackgroundSystem {
    /// Apply the system to the given entity components
    #[inline]
    pub fn apply_single(
        &mut self,
                wgpu_render: &mut WgpuRenderState,
        background_color: &BackgroundColorComponent,
    ) {
        // Force the implementation of the ApplyRenderBackgroundSystem trait.
        ApplyRenderBackgroundSystem::apply_single(
            self,
                    wgpu_render,
            background_color,
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
    #[inline]
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
}

#[allow(dead_code)]
impl RenderSkySystem {
    /// Apply the system to the given entity components
    #[inline]
    pub fn apply_single(
        &mut self,
                wgpu_render: &mut WgpuRenderState,
        skydome: &SkydomeComponent,
    ) {
        // Force the implementation of the ApplyRenderSkySystem trait.
        ApplyRenderSkySystem::apply_single(
            self,
                    wgpu_render,
            skydome,
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
    #[inline]
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
}

#[allow(dead_code)]
impl RenderSystem {
    /// Apply the system to the given entity components
    #[inline]
    pub fn apply_single(
        &mut self,
                wgpu_render: &mut WgpuRenderState,
        position: &PositionComponent,
        transform: &TransformComponent,
        model: &ModelComponent,
    ) {
        // Force the implementation of the ApplyRenderSystem trait.
        ApplyRenderSystem::apply_single(
            self,
                    wgpu_render,
            position,
            transform,
            model,
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
    #[inline]
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
}

#[allow(dead_code)]
impl TileRenderSystem {
    /// Apply the system to the given entity components
    #[inline]
    pub fn apply_single(
        &mut self,
                wgpu_render: &mut WgpuRenderState,
        tile: &TileComponent,
        position: &PositionComponent,
    ) {
        // Force the implementation of the ApplyTileRenderSystem trait.
        ApplyTileRenderSystem::apply_single(
            self,
                    wgpu_render,
            tile,
            position,
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
    #[inline]
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
        context: &FrameContext,
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
        context: &FrameContext,
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
        context: &FrameContext,
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
    /// - `color`: A reference to the input component of type [`ColorComponent`].
    ///
    /// ### Mutates
    /// 
    /// - `dynamic_light`: A mutable reference to the component of type [`DynamicLightComponent`].
    #[allow(unused_variables)]
    fn apply_single(
        &mut self,
        context: &FrameContext,
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
        context: &FrameContext,
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
}

#[allow(dead_code)]
impl DynamicLightSystem {
    /// Apply the system to the given entity components
    #[inline]
    pub fn apply_single(
        &mut self,
        context: &FrameContext,
                wgpu_render: &mut WgpuRenderState,
        color: &ColorComponent,
        dynamic_light: &mut DynamicLightComponent,
    ) {
        // Force the implementation of the ApplyDynamicLightSystem trait.
        ApplyDynamicLightSystem::apply_single(
            self,
            context,
                    wgpu_render,
            color,
            dynamic_light,
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
    #[inline]
    fn apply_many(
        &mut self,
        context: &FrameContext,
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
}

#[allow(dead_code)]
impl VgaBlitSystem {
    /// Apply the system to the given entity components
    #[inline]
    pub fn apply_single(
        &mut self,
                wgpu_render: &mut WgpuRenderState,
        background_color: &BackgroundColorComponent,
    ) {
        // Force the implementation of the ApplyVgaBlitSystem trait.
        ApplyVgaBlitSystem::apply_single(
            self,
                    wgpu_render,
            background_color,
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
    #[inline]
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