/// The ID of a [`Component`].
#[derive(Debug, Copy, Clone, Ord, PartialOrd, Eq, PartialEq)]
#[repr(u32)]
pub enum ComponentId {
    /// The [`TimeOfDay`](TimeOfDayComponent) component.
    ///
    /// # Archetype Use
    /// This component is used by the following archetypes:
    /// 
    /// - [`EnvironmentArchetype`] ([`ArchetypeId::Environment`])
    ///
    /// # System Use
    /// This component is used by the following systems:
    /// 
    /// - [`TimeOfDaySystem`] ([`SystemId::TimeOfDay`])
    /// - [`SunSystem`] ([`SystemId::Sun`])
    /// - [`StarsSystem`] ([`SystemId::Stars`])
    /// - [`FrameGlobalsSystem`] ([`SystemId::FrameGlobals`])
    TimeOfDay = 1,
    /// The [`Sun`](SunComponent) component.
    ///
    /// # Archetype Use
    /// This component is used by the following archetypes:
    /// 
    /// - [`EnvironmentArchetype`] ([`ArchetypeId::Environment`])
    ///
    /// # System Use
    /// This component is used by the following systems:
    /// 
    /// - [`SunSystem`] ([`SystemId::Sun`])
    /// - [`FrameGlobalsSystem`] ([`SystemId::FrameGlobals`])
    Sun = 2,
    /// The [`Skydome`](SkydomeComponent) component.
    ///
    /// # Archetype Use
    /// This component is used by the following archetypes:
    /// 
    /// - [`EnvironmentArchetype`] ([`ArchetypeId::Environment`])
    ///
    /// # System Use
    /// This component is used by the following systems:
    /// 
    /// - [`SunSystem`] ([`SystemId::Sun`])
    /// - [`FrameGlobalsSystem`] ([`SystemId::FrameGlobals`])
    /// - [`RenderSkySystem`] ([`SystemId::RenderSky`])
    Skydome = 3,
    /// The [`AmbientLight`](AmbientLightComponent) component.
    ///
    /// # Archetype Use
    /// This component is used by the following archetypes:
    /// 
    /// - [`EnvironmentArchetype`] ([`ArchetypeId::Environment`])
    ///
    /// # System Use
    /// This component is used by the following systems:
    /// 
    /// - [`SunSystem`] ([`SystemId::Sun`])
    /// - [`FrameGlobalsSystem`] ([`SystemId::FrameGlobals`])
    AmbientLight = 4,
    /// The [`Stars`](StarsComponent) component.
    ///
    /// # Archetype Use
    /// This component is used by the following archetypes:
    /// 
    /// - [`EnvironmentArchetype`] ([`ArchetypeId::Environment`])
    ///
    /// # System Use
    /// This component is used by the following systems:
    /// 
    /// - [`StarsSystem`] ([`SystemId::Stars`])
    /// - [`FrameGlobalsSystem`] ([`SystemId::FrameGlobals`])
    Stars = 5,
    /// The [`Camera`](CameraComponent) component.
    ///
    /// # Archetype Use
    /// This component is used by the following archetypes:
    /// 
    /// - [`CameraArchetype`] ([`ArchetypeId::Camera`])
    ///
    /// # System Use
    /// This component is used by the following systems:
    /// 
    /// - [`CameraSystem`] ([`SystemId::Camera`])
    /// - [`CameraRenderSystem`] ([`SystemId::CameraRender`])
    Camera = 6,
    /// The [`CameraFpsController`](CameraFpsControllerComponent) component.
    ///
    /// # Archetype Use
    /// This component is used by the following archetypes:
    /// 
    /// - [`CameraArchetype`] ([`ArchetypeId::Camera`])
    ///
    /// # System Use
    /// This component is used by the following systems:
    /// 
    /// - [`CameraSystem`] ([`SystemId::Camera`])
    CameraFpsController = 7,
    /// The [`Position`](PositionComponent) component.
    ///
    /// # Archetype Use
    /// This component is used by the following archetypes:
    /// 
    /// - [`DynamicLightArchetype`] ([`ArchetypeId::DynamicLight`])
    /// - [`TileArchetype`] ([`ArchetypeId::Tile`])
    /// - [`CubeArchetype`] ([`ArchetypeId::Cube`])
    /// - [`PlayerArchetype`] ([`ArchetypeId::Player`])
    /// - [`ForegroundObjectArchetype`] ([`ArchetypeId::ForegroundObject`])
    /// - [`BackgroundObjectArchetype`] ([`ArchetypeId::BackgroundObject`])
    ///
    /// # System Use
    /// This component is used by the following systems:
    /// 
    /// - [`PhysicsSystem`] ([`SystemId::Physics`])
    /// - [`RenderSystem`] ([`SystemId::Render`])
    /// - [`TileRenderSystem`] ([`SystemId::TileRender`])
    Position = 8,
    /// The [`Transform`](TransformComponent) component.
    ///
    /// # Archetype Use
    /// This component is used by the following archetypes:
    /// 
    /// - [`CubeArchetype`] ([`ArchetypeId::Cube`])
    ///
    /// # System Use
    /// This component is used by the following systems:
    /// 
    /// - [`RenderSystem`] ([`SystemId::Render`])
    Transform = 9,
    /// The [`Velocity`](VelocityComponent) component.
    ///
    /// # Archetype Use
    /// This component is used by the following archetypes:
    /// 
    /// - [`CubeArchetype`] ([`ArchetypeId::Cube`])
    /// - [`PlayerArchetype`] ([`ArchetypeId::Player`])
    ///
    /// # System Use
    /// This component is used by the following systems:
    /// 
    /// - [`PhysicsSystem`] ([`SystemId::Physics`])
    Velocity = 10,
    /// The [`Tile`](TileComponent) component.
    ///
    /// # Archetype Use
    /// This component is used by the following archetypes:
    /// 
    /// - [`TileArchetype`] ([`ArchetypeId::Tile`])
    ///
    /// # System Use
    /// This component is used by the following systems:
    /// 
    /// - [`TileRenderSystem`] ([`SystemId::TileRender`])
    Tile = 11,
    /// The [`Health`](HealthComponent) component.
    ///
    /// # Archetype Use
    /// This component is used by the following archetypes:
    /// 
    /// - [`PlayerArchetype`] ([`ArchetypeId::Player`])
    ///
    /// # System Use
    /// This component is used by the following systems:
    /// 
    Health = 12,
    /// The [`Collider`](ColliderComponent) component.
    ///
    /// # Archetype Use
    /// This component is used by the following archetypes:
    /// 
    /// - [`ForegroundObjectArchetype`] ([`ArchetypeId::ForegroundObject`])
    ///
    /// # System Use
    /// This component is used by the following systems:
    /// 
    Collider = 13,
    /// The [`Model`](ModelComponent) component.
    ///
    /// # Archetype Use
    /// This component is used by the following archetypes:
    /// 
    /// - [`CubeArchetype`] ([`ArchetypeId::Cube`])
    ///
    /// # System Use
    /// This component is used by the following systems:
    /// 
    /// - [`RenderSystem`] ([`SystemId::Render`])
    Model = 14,
    /// The [`Color`](ColorComponent) component.
    ///
    /// # Archetype Use
    /// This component is used by the following archetypes:
    /// 
    /// - [`DynamicLightArchetype`] ([`ArchetypeId::DynamicLight`])
    ///
    /// # System Use
    /// This component is used by the following systems:
    /// 
    /// - [`DynamicLightSystem`] ([`SystemId::DynamicLight`])
    Color = 15,
    /// The background color used for clearing the render buffer
    ///
    /// For details, see the [`BackgroundColorComponent`](BackgroundColorComponent) Struct.
    ///
    /// # Archetype Use
    /// This component is used by the following archetypes:
    /// 
    /// - [`SceneBackgroundArchetype`] ([`ArchetypeId::SceneBackground`])
    ///
    /// # System Use
    /// This component is used by the following systems:
    /// 
    /// - [`BackgroundColorCycleSystem`] ([`SystemId::BackgroundColorCycle`])
    /// - [`RenderBackgroundSystem`] ([`SystemId::RenderBackground`])
    /// - [`VgaBlitSystem`] ([`SystemId::VgaBlit`])
    BackgroundColor = 16,
    /// A marker component for light entities
    ///
    /// For details, see the [`DynamicLightComponent`](DynamicLightComponent) Struct.
    ///
    /// # Archetype Use
    /// This component is used by the following archetypes:
    /// 
    /// - [`DynamicLightArchetype`] ([`ArchetypeId::DynamicLight`])
    ///
    /// # System Use
    /// This component is used by the following systems:
    /// 
    /// - [`DynamicLightSystem`] ([`SystemId::DynamicLight`])
    DynamicLight = 17,
}

#[allow(dead_code)]
impl ComponentId {
    /// A [`NonZeroU64`](core::num::NonZeroU64) representation of [`ComponentId::TimeOfDay`](ComponentId::TimeOfDay) component.
    pub const TIME_OF_DAY_ID_VALUE: core::num::NonZeroU64 = core::num::NonZeroU64::new(1).expect("Error at ECS construction time");
    /// A [`NonZeroU64`](core::num::NonZeroU64) representation of [`ComponentId::Sun`](ComponentId::Sun) component.
    pub const SUN_ID_VALUE: core::num::NonZeroU64 = core::num::NonZeroU64::new(2).expect("Error at ECS construction time");
    /// A [`NonZeroU64`](core::num::NonZeroU64) representation of [`ComponentId::Skydome`](ComponentId::Skydome) component.
    pub const SKYDOME_ID_VALUE: core::num::NonZeroU64 = core::num::NonZeroU64::new(3).expect("Error at ECS construction time");
    /// A [`NonZeroU64`](core::num::NonZeroU64) representation of [`ComponentId::AmbientLight`](ComponentId::AmbientLight) component.
    pub const AMBIENT_LIGHT_ID_VALUE: core::num::NonZeroU64 = core::num::NonZeroU64::new(4).expect("Error at ECS construction time");
    /// A [`NonZeroU64`](core::num::NonZeroU64) representation of [`ComponentId::Stars`](ComponentId::Stars) component.
    pub const STARS_ID_VALUE: core::num::NonZeroU64 = core::num::NonZeroU64::new(5).expect("Error at ECS construction time");
    /// A [`NonZeroU64`](core::num::NonZeroU64) representation of [`ComponentId::Camera`](ComponentId::Camera) component.
    pub const CAMERA_ID_VALUE: core::num::NonZeroU64 = core::num::NonZeroU64::new(6).expect("Error at ECS construction time");
    /// A [`NonZeroU64`](core::num::NonZeroU64) representation of [`ComponentId::CameraFpsController`](ComponentId::CameraFpsController) component.
    pub const CAMERA_FPS_CONTROLLER_ID_VALUE: core::num::NonZeroU64 = core::num::NonZeroU64::new(7).expect("Error at ECS construction time");
    /// A [`NonZeroU64`](core::num::NonZeroU64) representation of [`ComponentId::Position`](ComponentId::Position) component.
    pub const POSITION_ID_VALUE: core::num::NonZeroU64 = core::num::NonZeroU64::new(8).expect("Error at ECS construction time");
    /// A [`NonZeroU64`](core::num::NonZeroU64) representation of [`ComponentId::Transform`](ComponentId::Transform) component.
    pub const TRANSFORM_ID_VALUE: core::num::NonZeroU64 = core::num::NonZeroU64::new(9).expect("Error at ECS construction time");
    /// A [`NonZeroU64`](core::num::NonZeroU64) representation of [`ComponentId::Velocity`](ComponentId::Velocity) component.
    pub const VELOCITY_ID_VALUE: core::num::NonZeroU64 = core::num::NonZeroU64::new(10).expect("Error at ECS construction time");
    /// A [`NonZeroU64`](core::num::NonZeroU64) representation of [`ComponentId::Tile`](ComponentId::Tile) component.
    pub const TILE_ID_VALUE: core::num::NonZeroU64 = core::num::NonZeroU64::new(11).expect("Error at ECS construction time");
    /// A [`NonZeroU64`](core::num::NonZeroU64) representation of [`ComponentId::Health`](ComponentId::Health) component.
    pub const HEALTH_ID_VALUE: core::num::NonZeroU64 = core::num::NonZeroU64::new(12).expect("Error at ECS construction time");
    /// A [`NonZeroU64`](core::num::NonZeroU64) representation of [`ComponentId::Collider`](ComponentId::Collider) component.
    pub const COLLIDER_ID_VALUE: core::num::NonZeroU64 = core::num::NonZeroU64::new(13).expect("Error at ECS construction time");
    /// A [`NonZeroU64`](core::num::NonZeroU64) representation of [`ComponentId::Model`](ComponentId::Model) component.
    pub const MODEL_ID_VALUE: core::num::NonZeroU64 = core::num::NonZeroU64::new(14).expect("Error at ECS construction time");
    /// A [`NonZeroU64`](core::num::NonZeroU64) representation of [`ComponentId::Color`](ComponentId::Color) component.
    pub const COLOR_ID_VALUE: core::num::NonZeroU64 = core::num::NonZeroU64::new(15).expect("Error at ECS construction time");
    /// A [`NonZeroU64`](core::num::NonZeroU64) representation of [`ComponentId::BackgroundColor`](ComponentId::BackgroundColor) component.
    pub const BACKGROUND_COLOR_ID_VALUE: core::num::NonZeroU64 = core::num::NonZeroU64::new(16).expect("Error at ECS construction time");
    /// A [`NonZeroU64`](core::num::NonZeroU64) representation of [`ComponentId::DynamicLight`](ComponentId::DynamicLight) component.
    pub const DYNAMIC_LIGHT_ID_VALUE: core::num::NonZeroU64 = core::num::NonZeroU64::new(17).expect("Error at ECS construction time");
}

#[allow(dead_code)]
impl ComponentId {
    /// Compile-time constant lookup of [`ComponentId::TimeOfDay`](ComponentId::TimeOfDay).
    const COMPONENT_1: ComponentId = ComponentId::TimeOfDay;
    /// Compile-time constant lookup of [`ComponentId::Sun`](ComponentId::Sun).
    const COMPONENT_2: ComponentId = ComponentId::Sun;
    /// Compile-time constant lookup of [`ComponentId::Skydome`](ComponentId::Skydome).
    const COMPONENT_3: ComponentId = ComponentId::Skydome;
    /// Compile-time constant lookup of [`ComponentId::AmbientLight`](ComponentId::AmbientLight).
    const COMPONENT_4: ComponentId = ComponentId::AmbientLight;
    /// Compile-time constant lookup of [`ComponentId::Stars`](ComponentId::Stars).
    const COMPONENT_5: ComponentId = ComponentId::Stars;
    /// Compile-time constant lookup of [`ComponentId::Camera`](ComponentId::Camera).
    const COMPONENT_6: ComponentId = ComponentId::Camera;
    /// Compile-time constant lookup of [`ComponentId::CameraFpsController`](ComponentId::CameraFpsController).
    const COMPONENT_7: ComponentId = ComponentId::CameraFpsController;
    /// Compile-time constant lookup of [`ComponentId::Position`](ComponentId::Position).
    const COMPONENT_8: ComponentId = ComponentId::Position;
    /// Compile-time constant lookup of [`ComponentId::Transform`](ComponentId::Transform).
    const COMPONENT_9: ComponentId = ComponentId::Transform;
    /// Compile-time constant lookup of [`ComponentId::Velocity`](ComponentId::Velocity).
    const COMPONENT_10: ComponentId = ComponentId::Velocity;
    /// Compile-time constant lookup of [`ComponentId::Tile`](ComponentId::Tile).
    const COMPONENT_11: ComponentId = ComponentId::Tile;
    /// Compile-time constant lookup of [`ComponentId::Health`](ComponentId::Health).
    const COMPONENT_12: ComponentId = ComponentId::Health;
    /// Compile-time constant lookup of [`ComponentId::Collider`](ComponentId::Collider).
    const COMPONENT_13: ComponentId = ComponentId::Collider;
    /// Compile-time constant lookup of [`ComponentId::Model`](ComponentId::Model).
    const COMPONENT_14: ComponentId = ComponentId::Model;
    /// Compile-time constant lookup of [`ComponentId::Color`](ComponentId::Color).
    const COMPONENT_15: ComponentId = ComponentId::Color;
    /// Compile-time constant lookup of [`ComponentId::BackgroundColor`](ComponentId::BackgroundColor).
    const COMPONENT_16: ComponentId = ComponentId::BackgroundColor;
    /// Compile-time constant lookup of [`ComponentId::DynamicLight`](ComponentId::DynamicLight).
    const COMPONENT_17: ComponentId = ComponentId::DynamicLight;
}

#[allow(dead_code)]
impl ComponentId {
    /// Returns this ID as a [`NonZeroU64`](core::num::NonZeroU64) value.
    pub const fn as_nonzero_u64(&self) -> core::num::NonZeroU64 {
        match self {
            Self::TimeOfDay => Self::TIME_OF_DAY_ID_VALUE,
            Self::Sun => Self::SUN_ID_VALUE,
            Self::Skydome => Self::SKYDOME_ID_VALUE,
            Self::AmbientLight => Self::AMBIENT_LIGHT_ID_VALUE,
            Self::Stars => Self::STARS_ID_VALUE,
            Self::Camera => Self::CAMERA_ID_VALUE,
            Self::CameraFpsController => Self::CAMERA_FPS_CONTROLLER_ID_VALUE,
            Self::Position => Self::POSITION_ID_VALUE,
            Self::Transform => Self::TRANSFORM_ID_VALUE,
            Self::Velocity => Self::VELOCITY_ID_VALUE,
            Self::Tile => Self::TILE_ID_VALUE,
            Self::Health => Self::HEALTH_ID_VALUE,
            Self::Collider => Self::COLLIDER_ID_VALUE,
            Self::Model => Self::MODEL_ID_VALUE,
            Self::Color => Self::COLOR_ID_VALUE,
            Self::BackgroundColor => Self::BACKGROUND_COLOR_ID_VALUE,
            Self::DynamicLight => Self::DYNAMIC_LIGHT_ID_VALUE,
        }
    }

    /// Returns this ID as a `u64` value.
    pub const fn as_u64(&self) -> u64 {
        self.as_nonzero_u64().get()
    }
}

impl core::hash::Hash for ComponentId {
    fn hash<H: core::hash::Hasher>(&self, state: &mut H) {
        self.as_u64().hash(state);
    }
}

impl From<ComponentId> for core::num::NonZeroU64 {
    fn from(value: ComponentId) -> core::num::NonZeroU64 {
        value.as_nonzero_u64()
    }
}

impl From<ComponentId> for u64 {
    fn from(value: ComponentId) -> u64 {
        value.as_u64()
    }
}

impl core::fmt::Display for ComponentId {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> Result<(), core::fmt::Error> {
        match self {
            Self::TimeOfDay => write!(f, "TimeOfDay (ID 1)"),
            Self::Sun => write!(f, "Sun (ID 2)"),
            Self::Skydome => write!(f, "Skydome (ID 3)"),
            Self::AmbientLight => write!(f, "AmbientLight (ID 4)"),
            Self::Stars => write!(f, "Stars (ID 5)"),
            Self::Camera => write!(f, "Camera (ID 6)"),
            Self::CameraFpsController => write!(f, "CameraFpsController (ID 7)"),
            Self::Position => write!(f, "Position (ID 8)"),
            Self::Transform => write!(f, "Transform (ID 9)"),
            Self::Velocity => write!(f, "Velocity (ID 10)"),
            Self::Tile => write!(f, "Tile (ID 11)"),
            Self::Health => write!(f, "Health (ID 12)"),
            Self::Collider => write!(f, "Collider (ID 13)"),
            Self::Model => write!(f, "Model (ID 14)"),
            Self::Color => write!(f, "Color (ID 15)"),
            Self::BackgroundColor => write!(f, "BackgroundColor (ID 16)"),
            Self::DynamicLight => write!(f, "DynamicLight (ID 17)"),
        }
    }
}

/// Marker trait for components.
pub trait Component: 'static + Send + Sync {
    /// The ID of this component.
    const ID: ComponentId;

    /// The ID of this component.
    #[inline]
    #[allow(dead_code)]
    fn id(&self) -> ComponentId {
        Self::ID
    }
}

/// A `TimeOfDay` component.
///
/// See also [`TimeOfDayData`] for the actual data.
///
/// # Archetype Use
/// This component is used by the following archetypes:
/// 
/// - [`EnvironmentArchetype`] ([`ArchetypeId::Environment`])
///
/// # System Use
/// This component is used by the following systems:
/// 
/// - [`TimeOfDaySystem`] ([`SystemId::TimeOfDay`])
/// - [`SunSystem`] ([`SystemId::Sun`])
/// - [`StarsSystem`] ([`SystemId::Stars`])
/// - [`FrameGlobalsSystem`] ([`SystemId::FrameGlobals`])
#[derive(Debug, Clone)]
pub struct TimeOfDayComponent(TimeOfDayData);

#[allow(dead_code)]
impl TimeOfDayComponent {
    /// Creates a new [`TimeOfDayComponent`] instance from [`TimeOfDayData`].
    pub const fn new(data: TimeOfDayData) -> Self {
        Self(data)
    }

    /// Consumes self and returns the inner [`TimeOfDayData`].
    pub const fn into_inner(self) -> TimeOfDayData {
        self.0
    }
}

#[automatically_derived]
impl Component for TimeOfDayComponent {
    const ID: ComponentId = ComponentId::TimeOfDay;
}

impl PartialEq<ComponentId> for TimeOfDayComponent {
    fn eq(&self, other: &ComponentId) -> bool {
        Self::ID.eq(other)
    }
}

#[automatically_derived]
impl From<TimeOfDayData> for TimeOfDayComponent {
    fn from(data: TimeOfDayData) -> Self {
        Self(data)
    }
}

#[automatically_derived]
impl core::ops::Deref for TimeOfDayComponent {
    type Target = TimeOfDayData;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

#[automatically_derived]
impl core::ops::DerefMut for TimeOfDayComponent {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

#[automatically_derived]
impl core::convert::AsRef<TimeOfDayData> for TimeOfDayComponent {
    fn as_ref(&self) -> &TimeOfDayData {
        &self.0
    }
}

#[automatically_derived]
impl core::convert::AsMut<TimeOfDayData> for TimeOfDayComponent {
    fn as_mut(&mut self) -> &mut TimeOfDayData {
        &mut self.0
    }
}

#[automatically_derived]
impl core::borrow::Borrow<TimeOfDayData> for TimeOfDayComponent {
    fn borrow(&self) -> &TimeOfDayData {
        &self.0
    }
}

#[automatically_derived]
impl core::borrow::BorrowMut<TimeOfDayData> for TimeOfDayComponent {
    fn borrow_mut(&mut self) -> &mut TimeOfDayData {
        &mut self.0
    }
}

/// A `Sun` component.
///
/// See also [`SunData`] for the actual data.
///
/// # Archetype Use
/// This component is used by the following archetypes:
/// 
/// - [`EnvironmentArchetype`] ([`ArchetypeId::Environment`])
///
/// # System Use
/// This component is used by the following systems:
/// 
/// - [`SunSystem`] ([`SystemId::Sun`])
/// - [`FrameGlobalsSystem`] ([`SystemId::FrameGlobals`])
#[derive(Debug, Clone)]
pub struct SunComponent(SunData);

#[allow(dead_code)]
impl SunComponent {
    /// Creates a new [`SunComponent`] instance from [`SunData`].
    pub const fn new(data: SunData) -> Self {
        Self(data)
    }

    /// Consumes self and returns the inner [`SunData`].
    pub const fn into_inner(self) -> SunData {
        self.0
    }
}

#[automatically_derived]
impl Component for SunComponent {
    const ID: ComponentId = ComponentId::Sun;
}

impl PartialEq<ComponentId> for SunComponent {
    fn eq(&self, other: &ComponentId) -> bool {
        Self::ID.eq(other)
    }
}

#[automatically_derived]
impl From<SunData> for SunComponent {
    fn from(data: SunData) -> Self {
        Self(data)
    }
}

#[automatically_derived]
impl core::ops::Deref for SunComponent {
    type Target = SunData;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

#[automatically_derived]
impl core::ops::DerefMut for SunComponent {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

#[automatically_derived]
impl core::convert::AsRef<SunData> for SunComponent {
    fn as_ref(&self) -> &SunData {
        &self.0
    }
}

#[automatically_derived]
impl core::convert::AsMut<SunData> for SunComponent {
    fn as_mut(&mut self) -> &mut SunData {
        &mut self.0
    }
}

#[automatically_derived]
impl core::borrow::Borrow<SunData> for SunComponent {
    fn borrow(&self) -> &SunData {
        &self.0
    }
}

#[automatically_derived]
impl core::borrow::BorrowMut<SunData> for SunComponent {
    fn borrow_mut(&mut self) -> &mut SunData {
        &mut self.0
    }
}

/// A `Skydome` component.
///
/// See also [`SkydomeData`] for the actual data.
///
/// # Archetype Use
/// This component is used by the following archetypes:
/// 
/// - [`EnvironmentArchetype`] ([`ArchetypeId::Environment`])
///
/// # System Use
/// This component is used by the following systems:
/// 
/// - [`SunSystem`] ([`SystemId::Sun`])
/// - [`FrameGlobalsSystem`] ([`SystemId::FrameGlobals`])
/// - [`RenderSkySystem`] ([`SystemId::RenderSky`])
#[derive(Debug, Clone)]
pub struct SkydomeComponent(SkydomeData);

#[allow(dead_code)]
impl SkydomeComponent {
    /// Creates a new [`SkydomeComponent`] instance from [`SkydomeData`].
    pub const fn new(data: SkydomeData) -> Self {
        Self(data)
    }

    /// Consumes self and returns the inner [`SkydomeData`].
    pub const fn into_inner(self) -> SkydomeData {
        self.0
    }
}

#[automatically_derived]
impl Component for SkydomeComponent {
    const ID: ComponentId = ComponentId::Skydome;
}

impl PartialEq<ComponentId> for SkydomeComponent {
    fn eq(&self, other: &ComponentId) -> bool {
        Self::ID.eq(other)
    }
}

#[automatically_derived]
impl From<SkydomeData> for SkydomeComponent {
    fn from(data: SkydomeData) -> Self {
        Self(data)
    }
}

#[automatically_derived]
impl core::ops::Deref for SkydomeComponent {
    type Target = SkydomeData;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

#[automatically_derived]
impl core::ops::DerefMut for SkydomeComponent {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

#[automatically_derived]
impl core::convert::AsRef<SkydomeData> for SkydomeComponent {
    fn as_ref(&self) -> &SkydomeData {
        &self.0
    }
}

#[automatically_derived]
impl core::convert::AsMut<SkydomeData> for SkydomeComponent {
    fn as_mut(&mut self) -> &mut SkydomeData {
        &mut self.0
    }
}

#[automatically_derived]
impl core::borrow::Borrow<SkydomeData> for SkydomeComponent {
    fn borrow(&self) -> &SkydomeData {
        &self.0
    }
}

#[automatically_derived]
impl core::borrow::BorrowMut<SkydomeData> for SkydomeComponent {
    fn borrow_mut(&mut self) -> &mut SkydomeData {
        &mut self.0
    }
}

/// A `AmbientLight` component.
///
/// See also [`AmbientLightData`] for the actual data.
///
/// # Archetype Use
/// This component is used by the following archetypes:
/// 
/// - [`EnvironmentArchetype`] ([`ArchetypeId::Environment`])
///
/// # System Use
/// This component is used by the following systems:
/// 
/// - [`SunSystem`] ([`SystemId::Sun`])
/// - [`FrameGlobalsSystem`] ([`SystemId::FrameGlobals`])
#[derive(Debug, Clone)]
pub struct AmbientLightComponent(AmbientLightData);

#[allow(dead_code)]
impl AmbientLightComponent {
    /// Creates a new [`AmbientLightComponent`] instance from [`AmbientLightData`].
    pub const fn new(data: AmbientLightData) -> Self {
        Self(data)
    }

    /// Consumes self and returns the inner [`AmbientLightData`].
    pub const fn into_inner(self) -> AmbientLightData {
        self.0
    }
}

#[automatically_derived]
impl Component for AmbientLightComponent {
    const ID: ComponentId = ComponentId::AmbientLight;
}

impl PartialEq<ComponentId> for AmbientLightComponent {
    fn eq(&self, other: &ComponentId) -> bool {
        Self::ID.eq(other)
    }
}

#[automatically_derived]
impl From<AmbientLightData> for AmbientLightComponent {
    fn from(data: AmbientLightData) -> Self {
        Self(data)
    }
}

#[automatically_derived]
impl core::ops::Deref for AmbientLightComponent {
    type Target = AmbientLightData;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

#[automatically_derived]
impl core::ops::DerefMut for AmbientLightComponent {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

#[automatically_derived]
impl core::convert::AsRef<AmbientLightData> for AmbientLightComponent {
    fn as_ref(&self) -> &AmbientLightData {
        &self.0
    }
}

#[automatically_derived]
impl core::convert::AsMut<AmbientLightData> for AmbientLightComponent {
    fn as_mut(&mut self) -> &mut AmbientLightData {
        &mut self.0
    }
}

#[automatically_derived]
impl core::borrow::Borrow<AmbientLightData> for AmbientLightComponent {
    fn borrow(&self) -> &AmbientLightData {
        &self.0
    }
}

#[automatically_derived]
impl core::borrow::BorrowMut<AmbientLightData> for AmbientLightComponent {
    fn borrow_mut(&mut self) -> &mut AmbientLightData {
        &mut self.0
    }
}

/// A `Stars` component.
///
/// See also [`StarsData`] for the actual data.
///
/// # Archetype Use
/// This component is used by the following archetypes:
/// 
/// - [`EnvironmentArchetype`] ([`ArchetypeId::Environment`])
///
/// # System Use
/// This component is used by the following systems:
/// 
/// - [`StarsSystem`] ([`SystemId::Stars`])
/// - [`FrameGlobalsSystem`] ([`SystemId::FrameGlobals`])
#[derive(Debug, Clone)]
pub struct StarsComponent(StarsData);

#[allow(dead_code)]
impl StarsComponent {
    /// Creates a new [`StarsComponent`] instance from [`StarsData`].
    pub const fn new(data: StarsData) -> Self {
        Self(data)
    }

    /// Consumes self and returns the inner [`StarsData`].
    pub const fn into_inner(self) -> StarsData {
        self.0
    }
}

#[automatically_derived]
impl Component for StarsComponent {
    const ID: ComponentId = ComponentId::Stars;
}

impl PartialEq<ComponentId> for StarsComponent {
    fn eq(&self, other: &ComponentId) -> bool {
        Self::ID.eq(other)
    }
}

#[automatically_derived]
impl From<StarsData> for StarsComponent {
    fn from(data: StarsData) -> Self {
        Self(data)
    }
}

#[automatically_derived]
impl core::ops::Deref for StarsComponent {
    type Target = StarsData;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

#[automatically_derived]
impl core::ops::DerefMut for StarsComponent {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

#[automatically_derived]
impl core::convert::AsRef<StarsData> for StarsComponent {
    fn as_ref(&self) -> &StarsData {
        &self.0
    }
}

#[automatically_derived]
impl core::convert::AsMut<StarsData> for StarsComponent {
    fn as_mut(&mut self) -> &mut StarsData {
        &mut self.0
    }
}

#[automatically_derived]
impl core::borrow::Borrow<StarsData> for StarsComponent {
    fn borrow(&self) -> &StarsData {
        &self.0
    }
}

#[automatically_derived]
impl core::borrow::BorrowMut<StarsData> for StarsComponent {
    fn borrow_mut(&mut self) -> &mut StarsData {
        &mut self.0
    }
}

/// A `Camera` component.
///
/// See also [`CameraData`] for the actual data.
///
/// # Archetype Use
/// This component is used by the following archetypes:
/// 
/// - [`CameraArchetype`] ([`ArchetypeId::Camera`])
///
/// # System Use
/// This component is used by the following systems:
/// 
/// - [`CameraSystem`] ([`SystemId::Camera`])
/// - [`CameraRenderSystem`] ([`SystemId::CameraRender`])
#[derive(Debug, Clone)]
pub struct CameraComponent(CameraData);

#[allow(dead_code)]
impl CameraComponent {
    /// Creates a new [`CameraComponent`] instance from [`CameraData`].
    pub const fn new(data: CameraData) -> Self {
        Self(data)
    }

    /// Consumes self and returns the inner [`CameraData`].
    pub const fn into_inner(self) -> CameraData {
        self.0
    }
}

#[automatically_derived]
impl Component for CameraComponent {
    const ID: ComponentId = ComponentId::Camera;
}

impl PartialEq<ComponentId> for CameraComponent {
    fn eq(&self, other: &ComponentId) -> bool {
        Self::ID.eq(other)
    }
}

#[automatically_derived]
impl From<CameraData> for CameraComponent {
    fn from(data: CameraData) -> Self {
        Self(data)
    }
}

#[automatically_derived]
impl core::ops::Deref for CameraComponent {
    type Target = CameraData;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

#[automatically_derived]
impl core::ops::DerefMut for CameraComponent {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

#[automatically_derived]
impl core::convert::AsRef<CameraData> for CameraComponent {
    fn as_ref(&self) -> &CameraData {
        &self.0
    }
}

#[automatically_derived]
impl core::convert::AsMut<CameraData> for CameraComponent {
    fn as_mut(&mut self) -> &mut CameraData {
        &mut self.0
    }
}

#[automatically_derived]
impl core::borrow::Borrow<CameraData> for CameraComponent {
    fn borrow(&self) -> &CameraData {
        &self.0
    }
}

#[automatically_derived]
impl core::borrow::BorrowMut<CameraData> for CameraComponent {
    fn borrow_mut(&mut self) -> &mut CameraData {
        &mut self.0
    }
}

/// A `CameraFpsController` component.
///
/// See also [`CameraFpsControllerData`] for the actual data.
///
/// # Archetype Use
/// This component is used by the following archetypes:
/// 
/// - [`CameraArchetype`] ([`ArchetypeId::Camera`])
///
/// # System Use
/// This component is used by the following systems:
/// 
/// - [`CameraSystem`] ([`SystemId::Camera`])
#[derive(Debug, Clone)]
pub struct CameraFpsControllerComponent(CameraFpsControllerData);

#[allow(dead_code)]
impl CameraFpsControllerComponent {
    /// Creates a new [`CameraFpsControllerComponent`] instance from [`CameraFpsControllerData`].
    pub const fn new(data: CameraFpsControllerData) -> Self {
        Self(data)
    }

    /// Consumes self and returns the inner [`CameraFpsControllerData`].
    pub const fn into_inner(self) -> CameraFpsControllerData {
        self.0
    }
}

#[automatically_derived]
impl Component for CameraFpsControllerComponent {
    const ID: ComponentId = ComponentId::CameraFpsController;
}

impl PartialEq<ComponentId> for CameraFpsControllerComponent {
    fn eq(&self, other: &ComponentId) -> bool {
        Self::ID.eq(other)
    }
}

#[automatically_derived]
impl From<CameraFpsControllerData> for CameraFpsControllerComponent {
    fn from(data: CameraFpsControllerData) -> Self {
        Self(data)
    }
}

#[automatically_derived]
impl core::ops::Deref for CameraFpsControllerComponent {
    type Target = CameraFpsControllerData;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

#[automatically_derived]
impl core::ops::DerefMut for CameraFpsControllerComponent {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

#[automatically_derived]
impl core::convert::AsRef<CameraFpsControllerData> for CameraFpsControllerComponent {
    fn as_ref(&self) -> &CameraFpsControllerData {
        &self.0
    }
}

#[automatically_derived]
impl core::convert::AsMut<CameraFpsControllerData> for CameraFpsControllerComponent {
    fn as_mut(&mut self) -> &mut CameraFpsControllerData {
        &mut self.0
    }
}

#[automatically_derived]
impl core::borrow::Borrow<CameraFpsControllerData> for CameraFpsControllerComponent {
    fn borrow(&self) -> &CameraFpsControllerData {
        &self.0
    }
}

#[automatically_derived]
impl core::borrow::BorrowMut<CameraFpsControllerData> for CameraFpsControllerComponent {
    fn borrow_mut(&mut self) -> &mut CameraFpsControllerData {
        &mut self.0
    }
}

/// A `Position` component.
///
/// See also [`PositionData`] for the actual data.
///
/// # Archetype Use
/// This component is used by the following archetypes:
/// 
/// - [`DynamicLightArchetype`] ([`ArchetypeId::DynamicLight`])
/// - [`TileArchetype`] ([`ArchetypeId::Tile`])
/// - [`CubeArchetype`] ([`ArchetypeId::Cube`])
/// - [`PlayerArchetype`] ([`ArchetypeId::Player`])
/// - [`ForegroundObjectArchetype`] ([`ArchetypeId::ForegroundObject`])
/// - [`BackgroundObjectArchetype`] ([`ArchetypeId::BackgroundObject`])
///
/// # System Use
/// This component is used by the following systems:
/// 
/// - [`PhysicsSystem`] ([`SystemId::Physics`])
/// - [`RenderSystem`] ([`SystemId::Render`])
/// - [`TileRenderSystem`] ([`SystemId::TileRender`])
#[derive(Debug, Clone)]
pub struct PositionComponent(PositionData);

#[allow(dead_code)]
impl PositionComponent {
    /// Creates a new [`PositionComponent`] instance from [`PositionData`].
    pub const fn new(data: PositionData) -> Self {
        Self(data)
    }

    /// Consumes self and returns the inner [`PositionData`].
    pub const fn into_inner(self) -> PositionData {
        self.0
    }
}

#[automatically_derived]
impl Component for PositionComponent {
    const ID: ComponentId = ComponentId::Position;
}

impl PartialEq<ComponentId> for PositionComponent {
    fn eq(&self, other: &ComponentId) -> bool {
        Self::ID.eq(other)
    }
}

#[automatically_derived]
impl From<PositionData> for PositionComponent {
    fn from(data: PositionData) -> Self {
        Self(data)
    }
}

#[automatically_derived]
impl core::ops::Deref for PositionComponent {
    type Target = PositionData;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

#[automatically_derived]
impl core::ops::DerefMut for PositionComponent {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

#[automatically_derived]
impl core::convert::AsRef<PositionData> for PositionComponent {
    fn as_ref(&self) -> &PositionData {
        &self.0
    }
}

#[automatically_derived]
impl core::convert::AsMut<PositionData> for PositionComponent {
    fn as_mut(&mut self) -> &mut PositionData {
        &mut self.0
    }
}

#[automatically_derived]
impl core::borrow::Borrow<PositionData> for PositionComponent {
    fn borrow(&self) -> &PositionData {
        &self.0
    }
}

#[automatically_derived]
impl core::borrow::BorrowMut<PositionData> for PositionComponent {
    fn borrow_mut(&mut self) -> &mut PositionData {
        &mut self.0
    }
}

/// A `Transform` component.
///
/// See also [`TransformData`] for the actual data.
///
/// # Archetype Use
/// This component is used by the following archetypes:
/// 
/// - [`CubeArchetype`] ([`ArchetypeId::Cube`])
///
/// # System Use
/// This component is used by the following systems:
/// 
/// - [`RenderSystem`] ([`SystemId::Render`])
#[derive(Debug, Clone)]
pub struct TransformComponent(TransformData);

#[allow(dead_code)]
impl TransformComponent {
    /// Creates a new [`TransformComponent`] instance from [`TransformData`].
    pub const fn new(data: TransformData) -> Self {
        Self(data)
    }

    /// Consumes self and returns the inner [`TransformData`].
    pub const fn into_inner(self) -> TransformData {
        self.0
    }
}

#[automatically_derived]
impl Component for TransformComponent {
    const ID: ComponentId = ComponentId::Transform;
}

impl PartialEq<ComponentId> for TransformComponent {
    fn eq(&self, other: &ComponentId) -> bool {
        Self::ID.eq(other)
    }
}

#[automatically_derived]
impl From<TransformData> for TransformComponent {
    fn from(data: TransformData) -> Self {
        Self(data)
    }
}

#[automatically_derived]
impl core::ops::Deref for TransformComponent {
    type Target = TransformData;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

#[automatically_derived]
impl core::ops::DerefMut for TransformComponent {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

#[automatically_derived]
impl core::convert::AsRef<TransformData> for TransformComponent {
    fn as_ref(&self) -> &TransformData {
        &self.0
    }
}

#[automatically_derived]
impl core::convert::AsMut<TransformData> for TransformComponent {
    fn as_mut(&mut self) -> &mut TransformData {
        &mut self.0
    }
}

#[automatically_derived]
impl core::borrow::Borrow<TransformData> for TransformComponent {
    fn borrow(&self) -> &TransformData {
        &self.0
    }
}

#[automatically_derived]
impl core::borrow::BorrowMut<TransformData> for TransformComponent {
    fn borrow_mut(&mut self) -> &mut TransformData {
        &mut self.0
    }
}

/// A `Velocity` component.
///
/// See also [`VelocityData`] for the actual data.
///
/// # Archetype Use
/// This component is used by the following archetypes:
/// 
/// - [`CubeArchetype`] ([`ArchetypeId::Cube`])
/// - [`PlayerArchetype`] ([`ArchetypeId::Player`])
///
/// # System Use
/// This component is used by the following systems:
/// 
/// - [`PhysicsSystem`] ([`SystemId::Physics`])
#[derive(Debug, Clone)]
pub struct VelocityComponent(VelocityData);

#[allow(dead_code)]
impl VelocityComponent {
    /// Creates a new [`VelocityComponent`] instance from [`VelocityData`].
    pub const fn new(data: VelocityData) -> Self {
        Self(data)
    }

    /// Consumes self and returns the inner [`VelocityData`].
    pub const fn into_inner(self) -> VelocityData {
        self.0
    }
}

#[automatically_derived]
impl Component for VelocityComponent {
    const ID: ComponentId = ComponentId::Velocity;
}

impl PartialEq<ComponentId> for VelocityComponent {
    fn eq(&self, other: &ComponentId) -> bool {
        Self::ID.eq(other)
    }
}

#[automatically_derived]
impl From<VelocityData> for VelocityComponent {
    fn from(data: VelocityData) -> Self {
        Self(data)
    }
}

#[automatically_derived]
impl core::ops::Deref for VelocityComponent {
    type Target = VelocityData;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

#[automatically_derived]
impl core::ops::DerefMut for VelocityComponent {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

#[automatically_derived]
impl core::convert::AsRef<VelocityData> for VelocityComponent {
    fn as_ref(&self) -> &VelocityData {
        &self.0
    }
}

#[automatically_derived]
impl core::convert::AsMut<VelocityData> for VelocityComponent {
    fn as_mut(&mut self) -> &mut VelocityData {
        &mut self.0
    }
}

#[automatically_derived]
impl core::borrow::Borrow<VelocityData> for VelocityComponent {
    fn borrow(&self) -> &VelocityData {
        &self.0
    }
}

#[automatically_derived]
impl core::borrow::BorrowMut<VelocityData> for VelocityComponent {
    fn borrow_mut(&mut self) -> &mut VelocityData {
        &mut self.0
    }
}

/// A `Tile` component.
///
/// See also [`TileData`] for the actual data.
///
/// # Archetype Use
/// This component is used by the following archetypes:
/// 
/// - [`TileArchetype`] ([`ArchetypeId::Tile`])
///
/// # System Use
/// This component is used by the following systems:
/// 
/// - [`TileRenderSystem`] ([`SystemId::TileRender`])
#[derive(Debug, Clone)]
pub struct TileComponent(TileData);

#[allow(dead_code)]
impl TileComponent {
    /// Creates a new [`TileComponent`] instance from [`TileData`].
    pub const fn new(data: TileData) -> Self {
        Self(data)
    }

    /// Consumes self and returns the inner [`TileData`].
    pub const fn into_inner(self) -> TileData {
        self.0
    }
}

#[automatically_derived]
impl Component for TileComponent {
    const ID: ComponentId = ComponentId::Tile;
}

impl PartialEq<ComponentId> for TileComponent {
    fn eq(&self, other: &ComponentId) -> bool {
        Self::ID.eq(other)
    }
}

#[automatically_derived]
impl From<TileData> for TileComponent {
    fn from(data: TileData) -> Self {
        Self(data)
    }
}

#[automatically_derived]
impl core::ops::Deref for TileComponent {
    type Target = TileData;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

#[automatically_derived]
impl core::ops::DerefMut for TileComponent {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

#[automatically_derived]
impl core::convert::AsRef<TileData> for TileComponent {
    fn as_ref(&self) -> &TileData {
        &self.0
    }
}

#[automatically_derived]
impl core::convert::AsMut<TileData> for TileComponent {
    fn as_mut(&mut self) -> &mut TileData {
        &mut self.0
    }
}

#[automatically_derived]
impl core::borrow::Borrow<TileData> for TileComponent {
    fn borrow(&self) -> &TileData {
        &self.0
    }
}

#[automatically_derived]
impl core::borrow::BorrowMut<TileData> for TileComponent {
    fn borrow_mut(&mut self) -> &mut TileData {
        &mut self.0
    }
}

/// A `Health` component.
///
/// See also [`HealthData`] for the actual data.
///
/// # Archetype Use
/// This component is used by the following archetypes:
/// 
/// - [`PlayerArchetype`] ([`ArchetypeId::Player`])
///
/// # System Use
/// This component is used by the following systems:
/// 
#[derive(Debug, Clone)]
pub struct HealthComponent(HealthData);

#[allow(dead_code)]
impl HealthComponent {
    /// Creates a new [`HealthComponent`] instance from [`HealthData`].
    pub const fn new(data: HealthData) -> Self {
        Self(data)
    }

    /// Consumes self and returns the inner [`HealthData`].
    pub const fn into_inner(self) -> HealthData {
        self.0
    }
}

#[automatically_derived]
impl Component for HealthComponent {
    const ID: ComponentId = ComponentId::Health;
}

impl PartialEq<ComponentId> for HealthComponent {
    fn eq(&self, other: &ComponentId) -> bool {
        Self::ID.eq(other)
    }
}

#[automatically_derived]
impl From<HealthData> for HealthComponent {
    fn from(data: HealthData) -> Self {
        Self(data)
    }
}

#[automatically_derived]
impl core::ops::Deref for HealthComponent {
    type Target = HealthData;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

#[automatically_derived]
impl core::ops::DerefMut for HealthComponent {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

#[automatically_derived]
impl core::convert::AsRef<HealthData> for HealthComponent {
    fn as_ref(&self) -> &HealthData {
        &self.0
    }
}

#[automatically_derived]
impl core::convert::AsMut<HealthData> for HealthComponent {
    fn as_mut(&mut self) -> &mut HealthData {
        &mut self.0
    }
}

#[automatically_derived]
impl core::borrow::Borrow<HealthData> for HealthComponent {
    fn borrow(&self) -> &HealthData {
        &self.0
    }
}

#[automatically_derived]
impl core::borrow::BorrowMut<HealthData> for HealthComponent {
    fn borrow_mut(&mut self) -> &mut HealthData {
        &mut self.0
    }
}

/// A `Collider` component.
///
/// See also [`ColliderData`] for the actual data.
///
/// # Archetype Use
/// This component is used by the following archetypes:
/// 
/// - [`ForegroundObjectArchetype`] ([`ArchetypeId::ForegroundObject`])
///
/// # System Use
/// This component is used by the following systems:
/// 
#[derive(Debug, Clone)]
pub struct ColliderComponent(ColliderData);

#[allow(dead_code)]
impl ColliderComponent {
    /// Creates a new [`ColliderComponent`] instance from [`ColliderData`].
    pub const fn new(data: ColliderData) -> Self {
        Self(data)
    }

    /// Consumes self and returns the inner [`ColliderData`].
    pub const fn into_inner(self) -> ColliderData {
        self.0
    }
}

#[automatically_derived]
impl Component for ColliderComponent {
    const ID: ComponentId = ComponentId::Collider;
}

impl PartialEq<ComponentId> for ColliderComponent {
    fn eq(&self, other: &ComponentId) -> bool {
        Self::ID.eq(other)
    }
}

#[automatically_derived]
impl From<ColliderData> for ColliderComponent {
    fn from(data: ColliderData) -> Self {
        Self(data)
    }
}

#[automatically_derived]
impl core::ops::Deref for ColliderComponent {
    type Target = ColliderData;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

#[automatically_derived]
impl core::ops::DerefMut for ColliderComponent {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

#[automatically_derived]
impl core::convert::AsRef<ColliderData> for ColliderComponent {
    fn as_ref(&self) -> &ColliderData {
        &self.0
    }
}

#[automatically_derived]
impl core::convert::AsMut<ColliderData> for ColliderComponent {
    fn as_mut(&mut self) -> &mut ColliderData {
        &mut self.0
    }
}

#[automatically_derived]
impl core::borrow::Borrow<ColliderData> for ColliderComponent {
    fn borrow(&self) -> &ColliderData {
        &self.0
    }
}

#[automatically_derived]
impl core::borrow::BorrowMut<ColliderData> for ColliderComponent {
    fn borrow_mut(&mut self) -> &mut ColliderData {
        &mut self.0
    }
}

/// A `Model` component.
///
/// See also [`ModelData`] for the actual data.
///
/// # Archetype Use
/// This component is used by the following archetypes:
/// 
/// - [`CubeArchetype`] ([`ArchetypeId::Cube`])
///
/// # System Use
/// This component is used by the following systems:
/// 
/// - [`RenderSystem`] ([`SystemId::Render`])
#[derive(Debug, Clone)]
pub struct ModelComponent(ModelData);

#[allow(dead_code)]
impl ModelComponent {
    /// Creates a new [`ModelComponent`] instance from [`ModelData`].
    pub const fn new(data: ModelData) -> Self {
        Self(data)
    }

    /// Consumes self and returns the inner [`ModelData`].
    pub const fn into_inner(self) -> ModelData {
        self.0
    }
}

#[automatically_derived]
impl Component for ModelComponent {
    const ID: ComponentId = ComponentId::Model;
}

impl PartialEq<ComponentId> for ModelComponent {
    fn eq(&self, other: &ComponentId) -> bool {
        Self::ID.eq(other)
    }
}

#[automatically_derived]
impl From<ModelData> for ModelComponent {
    fn from(data: ModelData) -> Self {
        Self(data)
    }
}

#[automatically_derived]
impl core::ops::Deref for ModelComponent {
    type Target = ModelData;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

#[automatically_derived]
impl core::ops::DerefMut for ModelComponent {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

#[automatically_derived]
impl core::convert::AsRef<ModelData> for ModelComponent {
    fn as_ref(&self) -> &ModelData {
        &self.0
    }
}

#[automatically_derived]
impl core::convert::AsMut<ModelData> for ModelComponent {
    fn as_mut(&mut self) -> &mut ModelData {
        &mut self.0
    }
}

#[automatically_derived]
impl core::borrow::Borrow<ModelData> for ModelComponent {
    fn borrow(&self) -> &ModelData {
        &self.0
    }
}

#[automatically_derived]
impl core::borrow::BorrowMut<ModelData> for ModelComponent {
    fn borrow_mut(&mut self) -> &mut ModelData {
        &mut self.0
    }
}

/// A `Color` component.
///
/// See also [`ColorData`] for the actual data.
///
/// # Archetype Use
/// This component is used by the following archetypes:
/// 
/// - [`DynamicLightArchetype`] ([`ArchetypeId::DynamicLight`])
///
/// # System Use
/// This component is used by the following systems:
/// 
/// - [`DynamicLightSystem`] ([`SystemId::DynamicLight`])
#[derive(Debug, Clone)]
pub struct ColorComponent(ColorData);

#[allow(dead_code)]
impl ColorComponent {
    /// Creates a new [`ColorComponent`] instance from [`ColorData`].
    pub const fn new(data: ColorData) -> Self {
        Self(data)
    }

    /// Consumes self and returns the inner [`ColorData`].
    pub const fn into_inner(self) -> ColorData {
        self.0
    }
}

#[automatically_derived]
impl Component for ColorComponent {
    const ID: ComponentId = ComponentId::Color;
}

impl PartialEq<ComponentId> for ColorComponent {
    fn eq(&self, other: &ComponentId) -> bool {
        Self::ID.eq(other)
    }
}

#[automatically_derived]
impl From<ColorData> for ColorComponent {
    fn from(data: ColorData) -> Self {
        Self(data)
    }
}

#[automatically_derived]
impl core::ops::Deref for ColorComponent {
    type Target = ColorData;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

#[automatically_derived]
impl core::ops::DerefMut for ColorComponent {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

#[automatically_derived]
impl core::convert::AsRef<ColorData> for ColorComponent {
    fn as_ref(&self) -> &ColorData {
        &self.0
    }
}

#[automatically_derived]
impl core::convert::AsMut<ColorData> for ColorComponent {
    fn as_mut(&mut self) -> &mut ColorData {
        &mut self.0
    }
}

#[automatically_derived]
impl core::borrow::Borrow<ColorData> for ColorComponent {
    fn borrow(&self) -> &ColorData {
        &self.0
    }
}

#[automatically_derived]
impl core::borrow::BorrowMut<ColorData> for ColorComponent {
    fn borrow_mut(&mut self) -> &mut ColorData {
        &mut self.0
    }
}

/// The background color used for clearing the render buffer
///
/// See also [`BackgroundColorData`] for the actual data.
///
/// # Archetype Use
/// This component is used by the following archetypes:
/// 
/// - [`SceneBackgroundArchetype`] ([`ArchetypeId::SceneBackground`])
///
/// # System Use
/// This component is used by the following systems:
/// 
/// - [`BackgroundColorCycleSystem`] ([`SystemId::BackgroundColorCycle`])
/// - [`RenderBackgroundSystem`] ([`SystemId::RenderBackground`])
/// - [`VgaBlitSystem`] ([`SystemId::VgaBlit`])
#[derive(Debug, Clone)]
pub struct BackgroundColorComponent(BackgroundColorData);

#[allow(dead_code)]
impl BackgroundColorComponent {
    /// Creates a new [`BackgroundColorComponent`] instance from [`BackgroundColorData`].
    pub const fn new(data: BackgroundColorData) -> Self {
        Self(data)
    }

    /// Consumes self and returns the inner [`BackgroundColorData`].
    pub const fn into_inner(self) -> BackgroundColorData {
        self.0
    }
}

#[automatically_derived]
impl Component for BackgroundColorComponent {
    const ID: ComponentId = ComponentId::BackgroundColor;
}

impl PartialEq<ComponentId> for BackgroundColorComponent {
    fn eq(&self, other: &ComponentId) -> bool {
        Self::ID.eq(other)
    }
}

#[automatically_derived]
impl From<BackgroundColorData> for BackgroundColorComponent {
    fn from(data: BackgroundColorData) -> Self {
        Self(data)
    }
}

#[automatically_derived]
impl core::ops::Deref for BackgroundColorComponent {
    type Target = BackgroundColorData;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

#[automatically_derived]
impl core::ops::DerefMut for BackgroundColorComponent {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

#[automatically_derived]
impl core::convert::AsRef<BackgroundColorData> for BackgroundColorComponent {
    fn as_ref(&self) -> &BackgroundColorData {
        &self.0
    }
}

#[automatically_derived]
impl core::convert::AsMut<BackgroundColorData> for BackgroundColorComponent {
    fn as_mut(&mut self) -> &mut BackgroundColorData {
        &mut self.0
    }
}

#[automatically_derived]
impl core::borrow::Borrow<BackgroundColorData> for BackgroundColorComponent {
    fn borrow(&self) -> &BackgroundColorData {
        &self.0
    }
}

#[automatically_derived]
impl core::borrow::BorrowMut<BackgroundColorData> for BackgroundColorComponent {
    fn borrow_mut(&mut self) -> &mut BackgroundColorData {
        &mut self.0
    }
}

/// A marker component for light entities
///
/// See also [`DynamicLightData`] for the actual data.
///
/// # Archetype Use
/// This component is used by the following archetypes:
/// 
/// - [`DynamicLightArchetype`] ([`ArchetypeId::DynamicLight`])
///
/// # System Use
/// This component is used by the following systems:
/// 
/// - [`DynamicLightSystem`] ([`SystemId::DynamicLight`])
#[derive(Debug, Clone)]
pub struct DynamicLightComponent(DynamicLightData);

#[allow(dead_code)]
impl DynamicLightComponent {
    /// Creates a new [`DynamicLightComponent`] instance from [`DynamicLightData`].
    pub const fn new(data: DynamicLightData) -> Self {
        Self(data)
    }

    /// Consumes self and returns the inner [`DynamicLightData`].
    pub const fn into_inner(self) -> DynamicLightData {
        self.0
    }
}

#[automatically_derived]
impl Component for DynamicLightComponent {
    const ID: ComponentId = ComponentId::DynamicLight;
}

impl PartialEq<ComponentId> for DynamicLightComponent {
    fn eq(&self, other: &ComponentId) -> bool {
        Self::ID.eq(other)
    }
}

#[automatically_derived]
impl From<DynamicLightData> for DynamicLightComponent {
    fn from(data: DynamicLightData) -> Self {
        Self(data)
    }
}

#[automatically_derived]
impl core::ops::Deref for DynamicLightComponent {
    type Target = DynamicLightData;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

#[automatically_derived]
impl core::ops::DerefMut for DynamicLightComponent {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

#[automatically_derived]
impl core::convert::AsRef<DynamicLightData> for DynamicLightComponent {
    fn as_ref(&self) -> &DynamicLightData {
        &self.0
    }
}

#[automatically_derived]
impl core::convert::AsMut<DynamicLightData> for DynamicLightComponent {
    fn as_mut(&mut self) -> &mut DynamicLightData {
        &mut self.0
    }
}

#[automatically_derived]
impl core::borrow::Borrow<DynamicLightData> for DynamicLightComponent {
    fn borrow(&self) -> &DynamicLightData {
        &self.0
    }
}

#[automatically_derived]
impl core::borrow::BorrowMut<DynamicLightData> for DynamicLightComponent {
    fn borrow_mut(&mut self) -> &mut DynamicLightData {
        &mut self.0
    }
}