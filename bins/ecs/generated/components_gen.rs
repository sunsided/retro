/// The ID of a [`Component`].
#[derive(Debug, Copy, Clone, Ord, PartialOrd, Eq, PartialEq)]
#[repr(u32)]
pub enum ComponentId {
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
    Camera = 1,
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
    CameraFpsController = 2,
    /// The [`Position`](PositionComponent) component.
    ///
    /// # Archetype Use
    /// This component is used by the following archetypes:
    /// 
    /// - [`DynamicLightArchetype`] ([`ArchetypeId::DynamicLight`])
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
    Position = 3,
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
    Transform = 4,
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
    Velocity = 5,
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
    Health = 6,
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
    Collider = 7,
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
    Model = 8,
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
    Color = 9,
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
    BackgroundColor = 10,
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
    DynamicLight = 11,
}

#[allow(dead_code)]
impl ComponentId {
    /// A [`NonZeroU64`](core::num::NonZeroU64) representation of [`ComponentId::Camera`](ComponentId::Camera) component.
    pub const CAMERA_ID_VALUE: core::num::NonZeroU64 = core::num::NonZeroU64::new(1).expect("Error at ECS construction time");
    /// A [`NonZeroU64`](core::num::NonZeroU64) representation of [`ComponentId::CameraFpsController`](ComponentId::CameraFpsController) component.
    pub const CAMERA_FPS_CONTROLLER_ID_VALUE: core::num::NonZeroU64 = core::num::NonZeroU64::new(2).expect("Error at ECS construction time");
    /// A [`NonZeroU64`](core::num::NonZeroU64) representation of [`ComponentId::Position`](ComponentId::Position) component.
    pub const POSITION_ID_VALUE: core::num::NonZeroU64 = core::num::NonZeroU64::new(3).expect("Error at ECS construction time");
    /// A [`NonZeroU64`](core::num::NonZeroU64) representation of [`ComponentId::Transform`](ComponentId::Transform) component.
    pub const TRANSFORM_ID_VALUE: core::num::NonZeroU64 = core::num::NonZeroU64::new(4).expect("Error at ECS construction time");
    /// A [`NonZeroU64`](core::num::NonZeroU64) representation of [`ComponentId::Velocity`](ComponentId::Velocity) component.
    pub const VELOCITY_ID_VALUE: core::num::NonZeroU64 = core::num::NonZeroU64::new(5).expect("Error at ECS construction time");
    /// A [`NonZeroU64`](core::num::NonZeroU64) representation of [`ComponentId::Health`](ComponentId::Health) component.
    pub const HEALTH_ID_VALUE: core::num::NonZeroU64 = core::num::NonZeroU64::new(6).expect("Error at ECS construction time");
    /// A [`NonZeroU64`](core::num::NonZeroU64) representation of [`ComponentId::Collider`](ComponentId::Collider) component.
    pub const COLLIDER_ID_VALUE: core::num::NonZeroU64 = core::num::NonZeroU64::new(7).expect("Error at ECS construction time");
    /// A [`NonZeroU64`](core::num::NonZeroU64) representation of [`ComponentId::Model`](ComponentId::Model) component.
    pub const MODEL_ID_VALUE: core::num::NonZeroU64 = core::num::NonZeroU64::new(8).expect("Error at ECS construction time");
    /// A [`NonZeroU64`](core::num::NonZeroU64) representation of [`ComponentId::Color`](ComponentId::Color) component.
    pub const COLOR_ID_VALUE: core::num::NonZeroU64 = core::num::NonZeroU64::new(9).expect("Error at ECS construction time");
    /// A [`NonZeroU64`](core::num::NonZeroU64) representation of [`ComponentId::BackgroundColor`](ComponentId::BackgroundColor) component.
    pub const BACKGROUND_COLOR_ID_VALUE: core::num::NonZeroU64 = core::num::NonZeroU64::new(10).expect("Error at ECS construction time");
    /// A [`NonZeroU64`](core::num::NonZeroU64) representation of [`ComponentId::DynamicLight`](ComponentId::DynamicLight) component.
    pub const DYNAMIC_LIGHT_ID_VALUE: core::num::NonZeroU64 = core::num::NonZeroU64::new(11).expect("Error at ECS construction time");
}

#[allow(dead_code)]
impl ComponentId {
    /// Compile-time constant lookup of [`ComponentId::Camera`](ComponentId::Camera).
    const COMPONENT_1: ComponentId = ComponentId::Camera;
    /// Compile-time constant lookup of [`ComponentId::CameraFpsController`](ComponentId::CameraFpsController).
    const COMPONENT_2: ComponentId = ComponentId::CameraFpsController;
    /// Compile-time constant lookup of [`ComponentId::Position`](ComponentId::Position).
    const COMPONENT_3: ComponentId = ComponentId::Position;
    /// Compile-time constant lookup of [`ComponentId::Transform`](ComponentId::Transform).
    const COMPONENT_4: ComponentId = ComponentId::Transform;
    /// Compile-time constant lookup of [`ComponentId::Velocity`](ComponentId::Velocity).
    const COMPONENT_5: ComponentId = ComponentId::Velocity;
    /// Compile-time constant lookup of [`ComponentId::Health`](ComponentId::Health).
    const COMPONENT_6: ComponentId = ComponentId::Health;
    /// Compile-time constant lookup of [`ComponentId::Collider`](ComponentId::Collider).
    const COMPONENT_7: ComponentId = ComponentId::Collider;
    /// Compile-time constant lookup of [`ComponentId::Model`](ComponentId::Model).
    const COMPONENT_8: ComponentId = ComponentId::Model;
    /// Compile-time constant lookup of [`ComponentId::Color`](ComponentId::Color).
    const COMPONENT_9: ComponentId = ComponentId::Color;
    /// Compile-time constant lookup of [`ComponentId::BackgroundColor`](ComponentId::BackgroundColor).
    const COMPONENT_10: ComponentId = ComponentId::BackgroundColor;
    /// Compile-time constant lookup of [`ComponentId::DynamicLight`](ComponentId::DynamicLight).
    const COMPONENT_11: ComponentId = ComponentId::DynamicLight;
}

#[allow(dead_code)]
impl ComponentId {
    /// Returns this ID as a [`NonZeroU64`](core::num::NonZeroU64) value.
    pub const fn as_nonzero_u64(&self) -> core::num::NonZeroU64 {
        match self {
            Self::Camera => Self::CAMERA_ID_VALUE,
            Self::CameraFpsController => Self::CAMERA_FPS_CONTROLLER_ID_VALUE,
            Self::Position => Self::POSITION_ID_VALUE,
            Self::Transform => Self::TRANSFORM_ID_VALUE,
            Self::Velocity => Self::VELOCITY_ID_VALUE,
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
            Self::Camera => write!(f, "Camera (ID 1)"),
            Self::CameraFpsController => write!(f, "CameraFpsController (ID 2)"),
            Self::Position => write!(f, "Position (ID 3)"),
            Self::Transform => write!(f, "Transform (ID 4)"),
            Self::Velocity => write!(f, "Velocity (ID 5)"),
            Self::Health => write!(f, "Health (ID 6)"),
            Self::Collider => write!(f, "Collider (ID 7)"),
            Self::Model => write!(f, "Model (ID 8)"),
            Self::Color => write!(f, "Color (ID 9)"),
            Self::BackgroundColor => write!(f, "BackgroundColor (ID 10)"),
            Self::DynamicLight => write!(f, "DynamicLight (ID 11)"),
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