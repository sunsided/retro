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
pub trait CreateSystem<S: System> {
    /// Create an instance of type `S`.
    fn create(&self) -> S;
}

/// Trait for constructing system instances.
pub trait CreateSystems: CreateSystem<CameraSystem> + CreateSystem<CameraRenderSystem> + CreateSystem<DynamicLightSystem> + CreateSystem<PhysicsSystem> + CreateSystem<BackgroundColorCycleSystem> + CreateSystem<RenderBackgroundSystem> + CreateSystem<RenderSystem>
{ }

#[automatically_derived]
impl<T> CreateSystems for T
where
    T: CreateSystem<CameraSystem>
     + CreateSystem<CameraRenderSystem>
     + CreateSystem<DynamicLightSystem>
     + CreateSystem<PhysicsSystem>
     + CreateSystem<BackgroundColorCycleSystem>
     + CreateSystem<RenderBackgroundSystem>
     + CreateSystem<RenderSystem>
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
/// The business logic is implemented via the [`ApplyCameraSystem`] trait.
/// See also [`CameraData`] for the systems-specific data.
///
/// ### Reads
/// 
///
/// ### Mutates
/// 
/// - [`CameraComponent`]
/// - [`CameraFpsControllerComponent`]
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
        ApplyCameraSystem::apply_many(
            self,
            input,
            wgpu_render,
            cameras,
            camera_fps_controllers,
        );
    }

    /// Applies this system to the [`CameraArchetype`].
    #[inline]
    #[deprecated]
    fn apply_to_camera(
        &mut self,
        archetype: &mut CameraArchetype,
        input: &InputState,
        wgpu_render: &WgpuRenderState,
    ) {
        self.apply_many(
            input,
            wgpu_render,
            &mut archetype.cameras,
            &mut archetype.camera_fps_controllers,
        );
    }
}

#[automatically_derived]
impl System for CameraSystem {
    const ID: SystemId = SystemId(core::num::NonZeroU64::new(1).expect("A zero system ID was provided"));
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
/// The business logic is implemented via the [`ApplyCameraRenderSystem`] trait.
/// See also [`CameraRenderData`] for the systems-specific data.
///
/// ### Reads
/// 
/// - [`CameraComponent`]
///
/// ### Mutates
/// 
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
        active_camera: &mut ActiveCameraState,
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
        active_camera: &mut ActiveCameraState,
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
        active_camera: &mut ActiveCameraState,
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
        ApplyCameraRenderSystem::apply_many(
            self,
            wgpu_render,
            active_camera,
            cameras,
        );
    }

    /// Applies this system to the [`CameraArchetype`].
    #[inline]
    #[deprecated]
    fn apply_to_camera(
        &mut self,
        archetype: &mut CameraArchetype,
        wgpu_render: &WgpuRenderState,
        active_camera: &mut ActiveCameraState,
    ) {
        self.apply_many(
            wgpu_render,
            active_camera,
            &archetype.cameras,
        );
    }
}

#[automatically_derived]
impl System for CameraRenderSystem {
    const ID: SystemId = SystemId(core::num::NonZeroU64::new(2).expect("A zero system ID was provided"));
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
        wgpu_render: &WgpuRenderState,
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
        wgpu_render: &WgpuRenderState,
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
        wgpu_render: &WgpuRenderState,
        color: &ColorComponent,
        dynamic_light: &mut DynamicLightComponent,
    ) {
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
        wgpu_render: &WgpuRenderState,
        colors: &[ColorComponent],
        dynamic_lights: &mut [DynamicLightComponent],
    ) {
        ApplyDynamicLightSystem::apply_many(
            self,
            context,
            wgpu_render,
            colors,
            dynamic_lights,
        );
    }

    /// Applies this system to the [`DynamicLightArchetype`].
    #[inline]
    #[deprecated]
    fn apply_to_dynamic_light(
        &mut self,
        archetype: &mut DynamicLightArchetype,
        context: &FrameContext,
        wgpu_render: &WgpuRenderState,
    ) {
        self.apply_many(
            &context,
            wgpu_render,
            &archetype.colors,
            &mut archetype.dynamic_lights,
        );
    }
}

#[automatically_derived]
impl System for DynamicLightSystem {
    const ID: SystemId = SystemId(core::num::NonZeroU64::new(3).expect("A zero system ID was provided"));
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
        ApplyPhysicsSystem::apply_many(
            self,
            context,
            velocities,
            positions,
        );
    }

    /// Applies this system to the [`ParticleArchetype`].
    #[inline]
    #[deprecated]
    fn apply_to_particle(
        &mut self,
        archetype: &mut ParticleArchetype,
        context: &FrameContext,
    ) {
        self.apply_many(
            &context,
            &archetype.velocities,
            &mut archetype.positions,
        );
    }

    /// Applies this system to the [`PlayerArchetype`].
    #[inline]
    #[deprecated]
    fn apply_to_player(
        &mut self,
        archetype: &mut PlayerArchetype,
        context: &FrameContext,
    ) {
        self.apply_many(
            &context,
            &archetype.velocities,
            &mut archetype.positions,
        );
    }
}

#[automatically_derived]
impl System for PhysicsSystem {
    const ID: SystemId = SystemId(core::num::NonZeroU64::new(4).expect("A zero system ID was provided"));
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
/// ### Reads
/// 
///
/// ### Mutates
/// 
/// - [`BackgroundColorComponent`]
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
        ApplyBackgroundColorCycleSystem::apply_many(
            self,
            context,
            background_colors,
        );
    }

    /// Applies this system to the [`SceneBackgroundArchetype`].
    #[inline]
    #[deprecated]
    fn apply_to_scene_background(
        &mut self,
        archetype: &mut SceneBackgroundArchetype,
        context: &FrameContext,
    ) {
        self.apply_many(
            &context,
            &mut archetype.background_colors,
        );
    }
}

#[automatically_derived]
impl System for BackgroundColorCycleSystem {
    const ID: SystemId = SystemId(core::num::NonZeroU64::new(5).expect("A zero system ID was provided"));
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
/// The business logic is implemented via the [`ApplyRenderBackgroundSystem`] trait.
/// See also [`RenderBackgroundData`] for the systems-specific data.
///
/// ### Reads
/// 
/// - [`BackgroundColorComponent`]
///
/// ### Mutates
/// 
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
        ApplyRenderBackgroundSystem::apply_many(
            self,
            wgpu_render,
            background_colors,
        );
    }

    /// Applies this system to the [`SceneBackgroundArchetype`].
    #[inline]
    #[deprecated]
    fn apply_to_scene_background(
        &mut self,
        archetype: &mut SceneBackgroundArchetype,
        wgpu_render: &mut WgpuRenderState,
    ) {
        self.apply_many(
            wgpu_render,
            &archetype.background_colors,
        );
    }
}

#[automatically_derived]
impl System for RenderBackgroundSystem {
    const ID: SystemId = SystemId(core::num::NonZeroU64::new(6).expect("A zero system ID was provided"));
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
/// The business logic is implemented via the [`ApplyRenderSystem`] trait.
/// See also [`RenderData`] for the systems-specific data.
///
/// ### Reads
/// 
/// - [`PositionComponent`]
///
/// ### Mutates
/// 
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
    ///
    /// ### Mutates
    /// 
    #[allow(unused_variables)]
    fn apply_single(
        &mut self,
        wgpu_render: &mut WgpuRenderState,
        position: &PositionComponent,
    ) {
         // intentionally left blank
     }

    /// Applies the system's business logic to the given entity components.
    ///
    /// ### Reads
    /// 
    /// - `position`: A slice of the input components of type [`PositionComponent`].
    ///
    /// ### Mutates
    /// 
    fn apply_many(
        &mut self,
        wgpu_render: &mut WgpuRenderState,
        positions: &[PositionComponent],
    ) {
        let zipped_iter = positions;
        for position in zipped_iter {
            self.apply_single(
                wgpu_render,
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
        &mut self,
        wgpu_render: &mut WgpuRenderState,
        position: &PositionComponent,
    ) {
        ApplyRenderSystem::apply_single(
            self,
            wgpu_render,
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
        &mut self,
        wgpu_render: &mut WgpuRenderState,
        positions: &[PositionComponent],
    ) {
        ApplyRenderSystem::apply_many(
            self,
            wgpu_render,
            positions,
        );
    }

    /// Applies this system to the [`DynamicLightArchetype`].
    #[inline]
    #[deprecated]
    fn apply_to_dynamic_light(
        &mut self,
        archetype: &mut DynamicLightArchetype,
        wgpu_render: &mut WgpuRenderState,
    ) {
        self.apply_many(
            wgpu_render,
            &archetype.positions,
        );
    }

    /// Applies this system to the [`ParticleArchetype`].
    #[inline]
    #[deprecated]
    fn apply_to_particle(
        &mut self,
        archetype: &mut ParticleArchetype,
        wgpu_render: &mut WgpuRenderState,
    ) {
        self.apply_many(
            wgpu_render,
            &archetype.positions,
        );
    }

    /// Applies this system to the [`PlayerArchetype`].
    #[inline]
    #[deprecated]
    fn apply_to_player(
        &mut self,
        archetype: &mut PlayerArchetype,
        wgpu_render: &mut WgpuRenderState,
    ) {
        self.apply_many(
            wgpu_render,
            &archetype.positions,
        );
    }

    /// Applies this system to the [`ForegroundObjectArchetype`].
    #[inline]
    #[deprecated]
    fn apply_to_foreground_object(
        &mut self,
        archetype: &mut ForegroundObjectArchetype,
        wgpu_render: &mut WgpuRenderState,
    ) {
        self.apply_many(
            wgpu_render,
            &archetype.positions,
        );
    }

    /// Applies this system to the [`BackgroundObjectArchetype`].
    #[inline]
    #[deprecated]
    fn apply_to_background_object(
        &mut self,
        archetype: &mut BackgroundObjectArchetype,
        wgpu_render: &mut WgpuRenderState,
    ) {
        self.apply_many(
            wgpu_render,
            &archetype.positions,
        );
    }
}

#[automatically_derived]
impl System for RenderSystem {
    const ID: SystemId = SystemId(core::num::NonZeroU64::new(7).expect("A zero system ID was provided"));
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