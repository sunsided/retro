/// The ID of a [`Component`].
#[derive(Debug, Copy, Clone, Ord, PartialOrd, Eq, PartialEq)]
pub struct ComponentId(core::num::NonZeroU64);

#[allow(dead_code)]
impl ComponentId {
    pub const fn new(id: u64) -> Option<Self> {
        if let Some(id) = core::num::NonZeroU64::new(id) {
            Some(Self(id))
        } else {
            None
        }
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

impl core::hash::Hash for ComponentId {
    fn hash<H: core::hash::Hasher>(&self, state: &mut H) {
        self.0.hash(state);
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

/// Marker trait for components.
pub trait Component: 'static + Send + Sync {
    /// The ID of this component.
    const ID: ComponentId;
}

/// A component.
///
/// See also [`PositionData`] for the actual data.
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
    const ID: ComponentId = ComponentId(core::num::NonZeroU64::new(1).expect("A zero component ID was provided"));
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

/// A component.
///
/// See also [`VelocityData`] for the actual data.
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
    const ID: ComponentId = ComponentId(core::num::NonZeroU64::new(2).expect("A zero component ID was provided"));
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

/// A component.
///
/// See also [`HealthData`] for the actual data.
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
    const ID: ComponentId = ComponentId(core::num::NonZeroU64::new(3).expect("A zero component ID was provided"));
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

/// A component.
///
/// See also [`ColliderData`] for the actual data.
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
    const ID: ComponentId = ComponentId(core::num::NonZeroU64::new(4).expect("A zero component ID was provided"));
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

/// The background color used for clearing the render buffer
///
/// See also [`BackgroundColorData`] for the actual data.
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
    const ID: ComponentId = ComponentId(core::num::NonZeroU64::new(5).expect("A zero component ID was provided"));
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