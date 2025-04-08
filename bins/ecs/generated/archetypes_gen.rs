/// The ID of an entity.
#[derive(Debug, Copy, Clone, Ord, PartialOrd, Eq, PartialEq)]
pub struct EntityId(core::num::NonZeroU64);

#[allow(dead_code)]
impl EntityId {
    /// Returns a new, unique entity ID.
    ///
    /// Uniqueness is guaranteed by using a monotonically increasing `AtomicU64` counter
    /// for generating IDs, starting from 1.
    ///
    /// # Implementation
    /// This function uses a thread-safe counter with sequential consistency ordering
    /// to ensure unique IDs even under concurrent access.
    pub fn new() -> Self {
        static ENTITY_IDS: core::sync::atomic::AtomicU64 = core::sync::atomic::AtomicU64::new(1);
        let id = ENTITY_IDS.fetch_add(1, core::sync::atomic::Ordering::SeqCst);
        EntityId(core::num::NonZeroU64::new(id).expect("ID was zero"))
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

impl core::hash::Hash for EntityId {
    fn hash<H: core::hash::Hasher>(&self, state: &mut H) {
        self.0.hash(state);
    }
}

impl From<EntityId> for core::num::NonZeroU64 {
    fn from(value: EntityId) -> core::num::NonZeroU64 {
        value.as_nonzero_u64()
    }
}

impl From<EntityId> for u64 {
    fn from(value: EntityId) -> u64 {
        value.as_u64()
    }
}

impl core::fmt::Display for EntityId {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> Result<(), core::fmt::Error> {
        core::fmt::Display::fmt(&self.0.get(), f)
    }
}

/// The ID of an [`Archetype`].
#[derive(Debug, Copy, Clone, Ord, PartialOrd, Eq, PartialEq)]
pub struct ArchetypeId(core::num::NonZeroU64);

#[allow(dead_code)]
impl ArchetypeId {
    /// Returns this ID as a [`NonZeroU64`](core::num::NonZeroU64) value.
    pub const fn as_nonzero_u64(&self) -> core::num::NonZeroU64 {
        self.0
    }

    /// Returns this ID as a `u64` value.
    pub const fn as_u64(&self) -> u64 {
            self.0.get()
    }
}

impl core::hash::Hash for ArchetypeId {
    fn hash<H: core::hash::Hasher>(&self, state: &mut H) {
        self.0.hash(state);
    }
}

impl From<ArchetypeId> for core::num::NonZeroU64 {
    fn from(value: ArchetypeId) -> core::num::NonZeroU64 {
        value.as_nonzero_u64()
    }
}

impl From<ArchetypeId> for u64 {
    fn from(value: ArchetypeId) -> u64 {
        value.as_u64()
    }
}

impl core::fmt::Display for ArchetypeId {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> Result<(), core::fmt::Error> {
        match self.0.get() {
            1 => write!(f, "Camera (Archetype ID 1)"),
            2 => write!(f, "DynamicLight (Archetype ID 2)"),
            3 => write!(f, "Cube (Archetype ID 3)"),
            4 => write!(f, "Player (Archetype ID 4)"),
            5 => write!(f, "ForegroundObject (Archetype ID 5)"),
            6 => write!(f, "BackgroundObject (Archetype ID 6)"),
            7 => write!(f, "SceneBackground (Archetype ID 7)"),
            id => write!(f, "Archetype ID {id}")
        }
    }
}

/// A locator for an entity across archetypes.
#[derive(Debug, Clone, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct EntityArchetypeRef {
    /// The ID of the archetype the entity is in.
    pub archetype: ArchetypeId,

    /// The Entity's index within that archetype.
    pub index: usize
}

/// Spawns an entity into the world.
#[allow(dead_code)]
pub trait Spawn<E> {
    /// Spawn a new entity into the world.
    fn spawn(&mut self, data: E) -> EntityId;
}

/// Marker trait for archetypes.
#[allow(dead_code)]
pub trait Archetype: 'static + Send + Sync {
    /// The ID of this archetype.
    const ID: ArchetypeId;

    /// The ID of this archetype.
    #[inline]
    #[allow(dead_code)]
    fn id(&self) -> ArchetypeId {
        Self::ID
    }

    /// Returns the number of entities in this archetype.
    fn len(&self) -> usize;

    /// Indicates whether this archetype has no entities.
    fn is_empty(&self) -> bool {
        self.len() == 0
    }
}

/// Trait for archetypes containing the specified `Component`.
#[allow(dead_code)]
pub trait HasComponents<T: Component> {
    /// Returns a slice of all component values.
    fn get(&self) -> &[T];

    /// Returns a mutable slice of all component values.
    fn get_mut(&mut self) -> &mut [T];
}

/// Trait for entities containing the specified `Component`.
#[allow(dead_code)]
pub trait HasComponent<T: Component> {
    /// Returns the component value.
    fn get(&self) -> &T;

    /// Mutably returns the component value.
    fn get_mut(&mut self) -> &mut T;
}

/// An enumeration holding variants for entities.
#[derive(Debug, Clone)]
#[allow(dead_code)]
pub enum ArchetypeEntity {
    /// An entity in the [`Camera`](CameraArchetype) archetype.
    Camera(CameraEntity),
    /// An entity in the [`DynamicLight`](DynamicLightArchetype) archetype.
    DynamicLight(DynamicLightEntity),
    /// A marching cube :)
    Cube(CubeEntity),
    /// An entity in the [`Player`](PlayerArchetype) archetype.
    Player(PlayerEntity),
    /// An entity in the [`ForegroundObject`](ForegroundObjectArchetype) archetype.
    ForegroundObject(ForegroundObjectEntity),
    /// An entity in the [`BackgroundObject`](BackgroundObjectArchetype) archetype.
    BackgroundObject(BackgroundObjectEntity),
    /// The scene background.
    SceneBackground(SceneBackgroundEntity),
}

/// An enumeration holding variants for entity data, e.g. when spawning entities.
#[derive(Debug, Clone)]
#[allow(dead_code)]
pub enum ArchetypeEntityData {
    /// An entity in the [`Camera`](CameraArchetype) archetype.
    Camera(CameraEntityData),
    /// An entity in the [`DynamicLight`](DynamicLightArchetype) archetype.
    DynamicLight(DynamicLightEntityData),
    /// A marching cube :)
    Cube(CubeEntityData),
    /// An entity in the [`Player`](PlayerArchetype) archetype.
    Player(PlayerEntityData),
    /// An entity in the [`ForegroundObject`](ForegroundObjectArchetype) archetype.
    ForegroundObject(ForegroundObjectEntityData),
    /// An entity in the [`BackgroundObject`](BackgroundObjectArchetype) archetype.
    BackgroundObject(BackgroundObjectEntityData),
    /// The scene background.
    SceneBackground(SceneBackgroundEntityData),
}

#[allow(dead_code)]
impl ArchetypeEntity {
    /// Determines the [`ArchetypeId`] of this [`ArchetypeEntity`] variant.
    pub const fn archetype_id(&self) -> ArchetypeId {
        match self {
            Self::Camera(_) => CameraArchetype::ID,
            Self::DynamicLight(_) => DynamicLightArchetype::ID,
            Self::Cube(_) => CubeArchetype::ID,
            Self::Player(_) => PlayerArchetype::ID,
            Self::ForegroundObject(_) => ForegroundObjectArchetype::ID,
            Self::BackgroundObject(_) => BackgroundObjectArchetype::ID,
            Self::SceneBackground(_) => SceneBackgroundArchetype::ID,
        }
    }
}

#[allow(dead_code)]
impl ArchetypeEntityData {
    /// Determines the [`ArchetypeId`] of this [`ArchetypeEntityData`] variant.
    pub const fn archetype_id(&self) -> ArchetypeId {
        match self {
            Self::Camera(_) => CameraArchetype::ID,
            Self::DynamicLight(_) => DynamicLightArchetype::ID,
            Self::Cube(_) => CubeArchetype::ID,
            Self::Player(_) => PlayerArchetype::ID,
            Self::ForegroundObject(_) => ForegroundObjectArchetype::ID,
            Self::BackgroundObject(_) => BackgroundObjectArchetype::ID,
            Self::SceneBackground(_) => SceneBackgroundArchetype::ID,
        }
    }
}

/// A container for an entity with its associated data.
#[derive(Debug, Clone)]
#[allow(dead_code)]
pub struct EntityWithIdAndData<Data: EntityData> {
    pub id: EntityId,
    pub data: Data,
}

/// Marker trait for entity data.
pub trait EntityData {
    const ARCHETYPE_ID: ArchetypeId;
}

/// An archetype grouping entities with identical components.
#[derive(Debug, Default, Clone)]
pub struct CameraArchetype {
    pub entities: Vec<EntityId>,
    pub cameras: Vec<CameraComponent>,
    pub camera_fps_controllers: Vec<CameraFpsControllerComponent>,
}

/// An entity of the [`CameraArchetype`].
pub type CameraEntity = EntityWithIdAndData<CameraEntityData>;

/// An entity of the [`CameraArchetype`].
#[derive(Debug, Clone)]
#[allow(dead_code)]
pub struct CameraEntityData {
    pub camera: CameraComponent,
    pub camera_fps_controller: CameraFpsControllerComponent,
}

impl EntityData for CameraEntityData {
    const ARCHETYPE_ID: ArchetypeId = CameraArchetype::ID;
}

/// A reference to an entity of the [`CameraArchetype`].
#[derive(Debug, Clone)]
#[allow(dead_code)]
pub struct CameraEntityRef<'archetype> {
    pub entity_id: EntityId,
    pub camera: &'archetype CameraComponent,
    pub camera_fps_controller: &'archetype CameraFpsControllerComponent,
}

/// A mutable reference to an entity of the [`CameraArchetype`].
#[derive(Debug)]
#[allow(dead_code)]
pub struct CameraEntityMut<'archetype> {
    pub entity_id: EntityId,
    pub camera: &'archetype mut CameraComponent,
    pub camera_fps_controller: &'archetype mut CameraFpsControllerComponent,
}

/// An iterator for entities in the [`CameraArchetype`].
pub struct CameraEntityIterator<'archetype> {
    archetype: &'archetype CameraArchetype,
    current: usize,
    length: usize,
}

impl<'archetype> CameraEntityIterator<'archetype> {
    fn new(archetype: &'archetype CameraArchetype) -> Self {
        Self {
            archetype,
            current: 0,
            length: archetype.len()
        }
    }
}

impl<'archetype> Iterator for CameraEntityIterator<'archetype> {
    type Item = CameraEntityRef<'archetype>;

    #[inline]
    fn next(&mut self) -> Option<Self::Item> {
        if self.current >= self.length {
            return None;
        }

        let idx = self.current;
        self.current += 1;
        Some(unsafe { self.archetype.get_at_unchecked(idx) })
    }
}

impl CameraArchetype {
    /// Iterates all entities in this archetype.
    #[inline]
    #[allow(dead_code)]
    pub fn iter(&self) -> CameraEntityIterator {
        CameraEntityIterator::new(self)
    }

    /// Gets the entity at the specified index.
    #[allow(dead_code)]
    pub fn get_at(&self, index: usize) -> Option<CameraEntityRef> {
        if index > self.len() {
            return None;
        }
        Some(unsafe { self.get_at_unchecked(index) })
    }

    /// Gets the entity at the specified index.
    #[allow(dead_code)]
    pub unsafe fn get_at_unchecked(&self, index: usize) -> CameraEntityRef {
        CameraEntityRef {
            entity_id: *self.entities.get_unchecked(index),
            camera: self.cameras.get_unchecked(index),
            camera_fps_controller: self.camera_fps_controllers.get_unchecked(index),
        }
    }

    /// Gets the entity at the specified index.
    #[allow(dead_code)]
    pub unsafe fn get_at_unchecked_mut(&mut self, index: usize) -> CameraEntityMut {
        CameraEntityMut {
            entity_id: *self.entities.get_unchecked(index),
            camera: self.cameras.get_unchecked_mut(index),
            camera_fps_controller: self.camera_fps_controllers.get_unchecked_mut(index),
        }
    }
}

#[allow(dead_code)]
impl HasComponent<CameraComponent> for CameraEntity {
    #[inline]
    fn get(&self) -> &CameraComponent {
        &self.data.camera
    }

    #[inline]
    fn get_mut(&mut self) -> &mut CameraComponent {
        &mut self.data.camera
    }
}

#[allow(dead_code)]
impl HasComponent<CameraComponent> for CameraEntityData {
    #[inline]
    fn get(&self) -> &CameraComponent {
        &self.camera
    }

    #[inline]
    fn get_mut(&mut self) -> &mut CameraComponent {
        &mut self.camera
    }
}

#[allow(dead_code)]
impl HasComponent<CameraFpsControllerComponent> for CameraEntity {
    #[inline]
    fn get(&self) -> &CameraFpsControllerComponent {
        &self.data.camera_fps_controller
    }

    #[inline]
    fn get_mut(&mut self) -> &mut CameraFpsControllerComponent {
        &mut self.data.camera_fps_controller
    }
}

#[allow(dead_code)]
impl HasComponent<CameraFpsControllerComponent> for CameraEntityData {
    #[inline]
    fn get(&self) -> &CameraFpsControllerComponent {
        &self.camera_fps_controller
    }

    #[inline]
    fn get_mut(&mut self) -> &mut CameraFpsControllerComponent {
        &mut self.camera_fps_controller
    }
}
impl CameraEntityData {
    /// Spawn this entity into the given world.
    #[inline]
    #[allow(dead_code)]
    pub fn spawn_into<W>(self, world: &mut W) -> EntityId
    where
        W: Spawn<CameraEntityData>
    {
        world.spawn(self)
    }
}

impl core::ops::Deref for CameraEntity {
    type Target = CameraEntityData;

    fn deref(&self) -> &Self::Target {
        &self.data
    }
}

impl core::ops::DerefMut for CameraEntity {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.data
    }
}

#[allow(dead_code)]
impl CameraArchetype {
    /// The IDs of the components used by this archetype, in ascending order.
    pub const COMPONENT_IDS: [ComponentId; 2] = [
        ComponentId::new(1).expect("Invalid ID on ECS construction time"),
        ComponentId::new(2).expect("Invalid ID on ECS construction time"),
    ];

    /// Returns the number of entities in this archetype.
    #[inline(always)]
    pub fn len(&self) -> usize {
        self.entities.len()
    }

    /// Indicates whether this archetype has no entities.
    #[inline(always)]
    pub fn is_empty(&self) -> bool {
        self.entities.is_empty()
    }

    /// Returns the number of components used by this archetype.
    #[inline(always)]
    pub const fn num_components(&self) -> usize {
        2
    }

    /// Dynamically determines whether this archetype has a specific component.
    pub const fn has_components<Id>(&self, id: ComponentId) -> bool {
        let id = id.as_u64();
        id == 1 || id == 2 || false
    }

    /// Spawn a new entity into the world. Must be called through the world for proper bookkeeping.
    #[doc(hidden)]
    pub fn spawn_with<R>(
        &mut self,
        camera: CameraComponent,
        camera_fps_controller: CameraFpsControllerComponent,
        mut world_registry: R
    ) -> EntityId
    where
        R: WorldEntityRegistry
    {
        self.cameras.push(camera);
        self.camera_fps_controllers.push(camera_fps_controller);

        let entity_id = EntityId::new();

        let entity_index = self.entities.len();
        self.entities.push(entity_id);

        let reference = EntityArchetypeRef {
            archetype: Self::ID,
            index: entity_index
        };

        world_registry.register(entity_id, reference)
    }

    /// Drop an entity at the specified index. Called by the world upon de-spawning entities.
    ///
    /// Returns the ID of the entity that was moved into the hole, or [`None`] if the archetype is now empty.
    #[doc(hidden)]
    pub fn drop_at_index(&mut self, index: usize) -> Result<Option<EntityId>, usize> {
        if index > self.entities.len() {
            return Err(index);
        }
        self.entities.swap_remove(index);
        self.cameras.swap_remove(index);
        self.camera_fps_controllers.swap_remove(index);
        if self.entities.is_empty() {
            Ok(None)
        }
        else {
            Ok(Some(self.entities[index]))
        }
    }
}


#[automatically_derived]
impl HasComponents<CameraComponent> for CameraArchetype {
    /// Returns a slice of all [`CameraComponent`] component values.
    fn get(&self) -> &[CameraComponent] {
        self.cameras.as_slice()
    }

    /// Returns a mutable slice of all [`CameraComponent`] component values.
    fn get_mut(&mut self) -> &mut [CameraComponent] {
        self.cameras.as_mut_slice()
    }
}

#[automatically_derived]
impl HasComponents<CameraFpsControllerComponent> for CameraArchetype {
    /// Returns a slice of all [`CameraFpsControllerComponent`] component values.
    fn get(&self) -> &[CameraFpsControllerComponent] {
        self.camera_fps_controllers.as_slice()
    }

    /// Returns a mutable slice of all [`CameraFpsControllerComponent`] component values.
    fn get_mut(&mut self) -> &mut [CameraFpsControllerComponent] {
        self.camera_fps_controllers.as_mut_slice()
    }
}


#[automatically_derived]
#[allow(dead_code)]
impl Archetype for CameraArchetype {
    const ID: ArchetypeId = ArchetypeId(core::num::NonZeroU64::new(1).expect("A zero archetype ID was provided"));

    fn len(&self) -> usize {
        self.entities.len()
    }

    fn is_empty(&self) -> bool {
        self.entities.is_empty()
    }
}

impl PartialEq<ArchetypeId> for CameraArchetype {
    fn eq(&self, other: &ArchetypeId) -> bool {
        Self::ID.eq(other)
    }
}

/// An archetype grouping entities with identical components.
#[derive(Debug, Default, Clone)]
pub struct DynamicLightArchetype {
    pub entities: Vec<EntityId>,
    pub dynamic_lights: Vec<DynamicLightComponent>,
    pub colors: Vec<ColorComponent>,
    pub positions: Vec<PositionComponent>,
}

/// An entity of the [`DynamicLightArchetype`].
pub type DynamicLightEntity = EntityWithIdAndData<DynamicLightEntityData>;

/// An entity of the [`DynamicLightArchetype`].
#[derive(Debug, Clone)]
#[allow(dead_code)]
pub struct DynamicLightEntityData {
    pub dynamic_light: DynamicLightComponent,
    pub color: ColorComponent,
    pub position: PositionComponent,
}

impl EntityData for DynamicLightEntityData {
    const ARCHETYPE_ID: ArchetypeId = DynamicLightArchetype::ID;
}

/// A reference to an entity of the [`DynamicLightArchetype`].
#[derive(Debug, Clone)]
#[allow(dead_code)]
pub struct DynamicLightEntityRef<'archetype> {
    pub entity_id: EntityId,
    pub dynamic_light: &'archetype DynamicLightComponent,
    pub color: &'archetype ColorComponent,
    pub position: &'archetype PositionComponent,
}

/// A mutable reference to an entity of the [`DynamicLightArchetype`].
#[derive(Debug)]
#[allow(dead_code)]
pub struct DynamicLightEntityMut<'archetype> {
    pub entity_id: EntityId,
    pub dynamic_light: &'archetype mut DynamicLightComponent,
    pub color: &'archetype mut ColorComponent,
    pub position: &'archetype mut PositionComponent,
}

/// An iterator for entities in the [`DynamicLightArchetype`].
pub struct DynamicLightEntityIterator<'archetype> {
    archetype: &'archetype DynamicLightArchetype,
    current: usize,
    length: usize,
}

impl<'archetype> DynamicLightEntityIterator<'archetype> {
    fn new(archetype: &'archetype DynamicLightArchetype) -> Self {
        Self {
            archetype,
            current: 0,
            length: archetype.len()
        }
    }
}

impl<'archetype> Iterator for DynamicLightEntityIterator<'archetype> {
    type Item = DynamicLightEntityRef<'archetype>;

    #[inline]
    fn next(&mut self) -> Option<Self::Item> {
        if self.current >= self.length {
            return None;
        }

        let idx = self.current;
        self.current += 1;
        Some(unsafe { self.archetype.get_at_unchecked(idx) })
    }
}

impl DynamicLightArchetype {
    /// Iterates all entities in this archetype.
    #[inline]
    #[allow(dead_code)]
    pub fn iter(&self) -> DynamicLightEntityIterator {
        DynamicLightEntityIterator::new(self)
    }

    /// Gets the entity at the specified index.
    #[allow(dead_code)]
    pub fn get_at(&self, index: usize) -> Option<DynamicLightEntityRef> {
        if index > self.len() {
            return None;
        }
        Some(unsafe { self.get_at_unchecked(index) })
    }

    /// Gets the entity at the specified index.
    #[allow(dead_code)]
    pub unsafe fn get_at_unchecked(&self, index: usize) -> DynamicLightEntityRef {
        DynamicLightEntityRef {
            entity_id: *self.entities.get_unchecked(index),
            dynamic_light: self.dynamic_lights.get_unchecked(index),
            color: self.colors.get_unchecked(index),
            position: self.positions.get_unchecked(index),
        }
    }

    /// Gets the entity at the specified index.
    #[allow(dead_code)]
    pub unsafe fn get_at_unchecked_mut(&mut self, index: usize) -> DynamicLightEntityMut {
        DynamicLightEntityMut {
            entity_id: *self.entities.get_unchecked(index),
            dynamic_light: self.dynamic_lights.get_unchecked_mut(index),
            color: self.colors.get_unchecked_mut(index),
            position: self.positions.get_unchecked_mut(index),
        }
    }
}

#[allow(dead_code)]
impl HasComponent<DynamicLightComponent> for DynamicLightEntity {
    #[inline]
    fn get(&self) -> &DynamicLightComponent {
        &self.data.dynamic_light
    }

    #[inline]
    fn get_mut(&mut self) -> &mut DynamicLightComponent {
        &mut self.data.dynamic_light
    }
}

#[allow(dead_code)]
impl HasComponent<DynamicLightComponent> for DynamicLightEntityData {
    #[inline]
    fn get(&self) -> &DynamicLightComponent {
        &self.dynamic_light
    }

    #[inline]
    fn get_mut(&mut self) -> &mut DynamicLightComponent {
        &mut self.dynamic_light
    }
}

#[allow(dead_code)]
impl HasComponent<ColorComponent> for DynamicLightEntity {
    #[inline]
    fn get(&self) -> &ColorComponent {
        &self.data.color
    }

    #[inline]
    fn get_mut(&mut self) -> &mut ColorComponent {
        &mut self.data.color
    }
}

#[allow(dead_code)]
impl HasComponent<ColorComponent> for DynamicLightEntityData {
    #[inline]
    fn get(&self) -> &ColorComponent {
        &self.color
    }

    #[inline]
    fn get_mut(&mut self) -> &mut ColorComponent {
        &mut self.color
    }
}

#[allow(dead_code)]
impl HasComponent<PositionComponent> for DynamicLightEntity {
    #[inline]
    fn get(&self) -> &PositionComponent {
        &self.data.position
    }

    #[inline]
    fn get_mut(&mut self) -> &mut PositionComponent {
        &mut self.data.position
    }
}

#[allow(dead_code)]
impl HasComponent<PositionComponent> for DynamicLightEntityData {
    #[inline]
    fn get(&self) -> &PositionComponent {
        &self.position
    }

    #[inline]
    fn get_mut(&mut self) -> &mut PositionComponent {
        &mut self.position
    }
}
impl DynamicLightEntityData {
    /// Spawn this entity into the given world.
    #[inline]
    #[allow(dead_code)]
    pub fn spawn_into<W>(self, world: &mut W) -> EntityId
    where
        W: Spawn<DynamicLightEntityData>
    {
        world.spawn(self)
    }
}

impl core::ops::Deref for DynamicLightEntity {
    type Target = DynamicLightEntityData;

    fn deref(&self) -> &Self::Target {
        &self.data
    }
}

impl core::ops::DerefMut for DynamicLightEntity {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.data
    }
}

#[allow(dead_code)]
impl DynamicLightArchetype {
    /// The IDs of the components used by this archetype, in ascending order.
    pub const COMPONENT_IDS: [ComponentId; 3] = [
        ComponentId::new(3).expect("Invalid ID on ECS construction time"),
        ComponentId::new(9).expect("Invalid ID on ECS construction time"),
        ComponentId::new(11).expect("Invalid ID on ECS construction time"),
    ];

    /// Returns the number of entities in this archetype.
    #[inline(always)]
    pub fn len(&self) -> usize {
        self.entities.len()
    }

    /// Indicates whether this archetype has no entities.
    #[inline(always)]
    pub fn is_empty(&self) -> bool {
        self.entities.is_empty()
    }

    /// Returns the number of components used by this archetype.
    #[inline(always)]
    pub const fn num_components(&self) -> usize {
        3
    }

    /// Dynamically determines whether this archetype has a specific component.
    pub const fn has_components<Id>(&self, id: ComponentId) -> bool {
        let id = id.as_u64();
        id == 3 || id == 9 || id == 11 || false
    }

    /// Spawn a new entity into the world. Must be called through the world for proper bookkeeping.
    #[doc(hidden)]
    pub fn spawn_with<R>(
        &mut self,
        dynamic_light: DynamicLightComponent,
        color: ColorComponent,
        position: PositionComponent,
        mut world_registry: R
    ) -> EntityId
    where
        R: WorldEntityRegistry
    {
        self.dynamic_lights.push(dynamic_light);
        self.colors.push(color);
        self.positions.push(position);

        let entity_id = EntityId::new();

        let entity_index = self.entities.len();
        self.entities.push(entity_id);

        let reference = EntityArchetypeRef {
            archetype: Self::ID,
            index: entity_index
        };

        world_registry.register(entity_id, reference)
    }

    /// Drop an entity at the specified index. Called by the world upon de-spawning entities.
    ///
    /// Returns the ID of the entity that was moved into the hole, or [`None`] if the archetype is now empty.
    #[doc(hidden)]
    pub fn drop_at_index(&mut self, index: usize) -> Result<Option<EntityId>, usize> {
        if index > self.entities.len() {
            return Err(index);
        }
        self.entities.swap_remove(index);
        self.dynamic_lights.swap_remove(index);
        self.colors.swap_remove(index);
        self.positions.swap_remove(index);
        if self.entities.is_empty() {
            Ok(None)
        }
        else {
            Ok(Some(self.entities[index]))
        }
    }
}


#[automatically_derived]
impl HasComponents<DynamicLightComponent> for DynamicLightArchetype {
    /// Returns a slice of all [`DynamicLightComponent`] component values.
    fn get(&self) -> &[DynamicLightComponent] {
        self.dynamic_lights.as_slice()
    }

    /// Returns a mutable slice of all [`DynamicLightComponent`] component values.
    fn get_mut(&mut self) -> &mut [DynamicLightComponent] {
        self.dynamic_lights.as_mut_slice()
    }
}

#[automatically_derived]
impl HasComponents<ColorComponent> for DynamicLightArchetype {
    /// Returns a slice of all [`ColorComponent`] component values.
    fn get(&self) -> &[ColorComponent] {
        self.colors.as_slice()
    }

    /// Returns a mutable slice of all [`ColorComponent`] component values.
    fn get_mut(&mut self) -> &mut [ColorComponent] {
        self.colors.as_mut_slice()
    }
}

#[automatically_derived]
impl HasComponents<PositionComponent> for DynamicLightArchetype {
    /// Returns a slice of all [`PositionComponent`] component values.
    fn get(&self) -> &[PositionComponent] {
        self.positions.as_slice()
    }

    /// Returns a mutable slice of all [`PositionComponent`] component values.
    fn get_mut(&mut self) -> &mut [PositionComponent] {
        self.positions.as_mut_slice()
    }
}


#[automatically_derived]
#[allow(dead_code)]
impl Archetype for DynamicLightArchetype {
    const ID: ArchetypeId = ArchetypeId(core::num::NonZeroU64::new(2).expect("A zero archetype ID was provided"));

    fn len(&self) -> usize {
        self.entities.len()
    }

    fn is_empty(&self) -> bool {
        self.entities.is_empty()
    }
}

impl PartialEq<ArchetypeId> for DynamicLightArchetype {
    fn eq(&self, other: &ArchetypeId) -> bool {
        Self::ID.eq(other)
    }
}

/// A marching cube :)
#[derive(Debug, Default, Clone)]
pub struct CubeArchetype {
    pub entities: Vec<EntityId>,
    pub positions: Vec<PositionComponent>,
    pub velocities: Vec<VelocityComponent>,
    pub transforms: Vec<TransformComponent>,
    pub models: Vec<ModelComponent>,
}

/// An entity of the [`CubeArchetype`].
pub type CubeEntity = EntityWithIdAndData<CubeEntityData>;

/// An entity of the [`CubeArchetype`].
#[derive(Debug, Clone)]
#[allow(dead_code)]
pub struct CubeEntityData {
    pub position: PositionComponent,
    pub velocity: VelocityComponent,
    pub transform: TransformComponent,
    pub model: ModelComponent,
}

impl EntityData for CubeEntityData {
    const ARCHETYPE_ID: ArchetypeId = CubeArchetype::ID;
}

/// A reference to an entity of the [`CubeArchetype`].
#[derive(Debug, Clone)]
#[allow(dead_code)]
pub struct CubeEntityRef<'archetype> {
    pub entity_id: EntityId,
    pub position: &'archetype PositionComponent,
    pub velocity: &'archetype VelocityComponent,
    pub transform: &'archetype TransformComponent,
    pub model: &'archetype ModelComponent,
}

/// A mutable reference to an entity of the [`CubeArchetype`].
#[derive(Debug)]
#[allow(dead_code)]
pub struct CubeEntityMut<'archetype> {
    pub entity_id: EntityId,
    pub position: &'archetype mut PositionComponent,
    pub velocity: &'archetype mut VelocityComponent,
    pub transform: &'archetype mut TransformComponent,
    pub model: &'archetype mut ModelComponent,
}

/// An iterator for entities in the [`CubeArchetype`].
pub struct CubeEntityIterator<'archetype> {
    archetype: &'archetype CubeArchetype,
    current: usize,
    length: usize,
}

impl<'archetype> CubeEntityIterator<'archetype> {
    fn new(archetype: &'archetype CubeArchetype) -> Self {
        Self {
            archetype,
            current: 0,
            length: archetype.len()
        }
    }
}

impl<'archetype> Iterator for CubeEntityIterator<'archetype> {
    type Item = CubeEntityRef<'archetype>;

    #[inline]
    fn next(&mut self) -> Option<Self::Item> {
        if self.current >= self.length {
            return None;
        }

        let idx = self.current;
        self.current += 1;
        Some(unsafe { self.archetype.get_at_unchecked(idx) })
    }
}

impl CubeArchetype {
    /// Iterates all entities in this archetype.
    #[inline]
    #[allow(dead_code)]
    pub fn iter(&self) -> CubeEntityIterator {
        CubeEntityIterator::new(self)
    }

    /// Gets the entity at the specified index.
    #[allow(dead_code)]
    pub fn get_at(&self, index: usize) -> Option<CubeEntityRef> {
        if index > self.len() {
            return None;
        }
        Some(unsafe { self.get_at_unchecked(index) })
    }

    /// Gets the entity at the specified index.
    #[allow(dead_code)]
    pub unsafe fn get_at_unchecked(&self, index: usize) -> CubeEntityRef {
        CubeEntityRef {
            entity_id: *self.entities.get_unchecked(index),
            position: self.positions.get_unchecked(index),
            velocity: self.velocities.get_unchecked(index),
            transform: self.transforms.get_unchecked(index),
            model: self.models.get_unchecked(index),
        }
    }

    /// Gets the entity at the specified index.
    #[allow(dead_code)]
    pub unsafe fn get_at_unchecked_mut(&mut self, index: usize) -> CubeEntityMut {
        CubeEntityMut {
            entity_id: *self.entities.get_unchecked(index),
            position: self.positions.get_unchecked_mut(index),
            velocity: self.velocities.get_unchecked_mut(index),
            transform: self.transforms.get_unchecked_mut(index),
            model: self.models.get_unchecked_mut(index),
        }
    }
}

#[allow(dead_code)]
impl HasComponent<PositionComponent> for CubeEntity {
    #[inline]
    fn get(&self) -> &PositionComponent {
        &self.data.position
    }

    #[inline]
    fn get_mut(&mut self) -> &mut PositionComponent {
        &mut self.data.position
    }
}

#[allow(dead_code)]
impl HasComponent<PositionComponent> for CubeEntityData {
    #[inline]
    fn get(&self) -> &PositionComponent {
        &self.position
    }

    #[inline]
    fn get_mut(&mut self) -> &mut PositionComponent {
        &mut self.position
    }
}

#[allow(dead_code)]
impl HasComponent<VelocityComponent> for CubeEntity {
    #[inline]
    fn get(&self) -> &VelocityComponent {
        &self.data.velocity
    }

    #[inline]
    fn get_mut(&mut self) -> &mut VelocityComponent {
        &mut self.data.velocity
    }
}

#[allow(dead_code)]
impl HasComponent<VelocityComponent> for CubeEntityData {
    #[inline]
    fn get(&self) -> &VelocityComponent {
        &self.velocity
    }

    #[inline]
    fn get_mut(&mut self) -> &mut VelocityComponent {
        &mut self.velocity
    }
}

#[allow(dead_code)]
impl HasComponent<TransformComponent> for CubeEntity {
    #[inline]
    fn get(&self) -> &TransformComponent {
        &self.data.transform
    }

    #[inline]
    fn get_mut(&mut self) -> &mut TransformComponent {
        &mut self.data.transform
    }
}

#[allow(dead_code)]
impl HasComponent<TransformComponent> for CubeEntityData {
    #[inline]
    fn get(&self) -> &TransformComponent {
        &self.transform
    }

    #[inline]
    fn get_mut(&mut self) -> &mut TransformComponent {
        &mut self.transform
    }
}

#[allow(dead_code)]
impl HasComponent<ModelComponent> for CubeEntity {
    #[inline]
    fn get(&self) -> &ModelComponent {
        &self.data.model
    }

    #[inline]
    fn get_mut(&mut self) -> &mut ModelComponent {
        &mut self.data.model
    }
}

#[allow(dead_code)]
impl HasComponent<ModelComponent> for CubeEntityData {
    #[inline]
    fn get(&self) -> &ModelComponent {
        &self.model
    }

    #[inline]
    fn get_mut(&mut self) -> &mut ModelComponent {
        &mut self.model
    }
}
impl CubeEntityData {
    /// Spawn this entity into the given world.
    #[inline]
    #[allow(dead_code)]
    pub fn spawn_into<W>(self, world: &mut W) -> EntityId
    where
        W: Spawn<CubeEntityData>
    {
        world.spawn(self)
    }
}

impl core::ops::Deref for CubeEntity {
    type Target = CubeEntityData;

    fn deref(&self) -> &Self::Target {
        &self.data
    }
}

impl core::ops::DerefMut for CubeEntity {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.data
    }
}

#[allow(dead_code)]
impl CubeArchetype {
    /// The IDs of the components used by this archetype, in ascending order.
    pub const COMPONENT_IDS: [ComponentId; 4] = [
        ComponentId::new(3).expect("Invalid ID on ECS construction time"),
        ComponentId::new(4).expect("Invalid ID on ECS construction time"),
        ComponentId::new(5).expect("Invalid ID on ECS construction time"),
        ComponentId::new(8).expect("Invalid ID on ECS construction time"),
    ];

    /// Returns the number of entities in this archetype.
    #[inline(always)]
    pub fn len(&self) -> usize {
        self.entities.len()
    }

    /// Indicates whether this archetype has no entities.
    #[inline(always)]
    pub fn is_empty(&self) -> bool {
        self.entities.is_empty()
    }

    /// Returns the number of components used by this archetype.
    #[inline(always)]
    pub const fn num_components(&self) -> usize {
        4
    }

    /// Dynamically determines whether this archetype has a specific component.
    pub const fn has_components<Id>(&self, id: ComponentId) -> bool {
        let id = id.as_u64();
        id == 3 || id == 4 || id == 5 || id == 8 || false
    }

    /// Spawn a new entity into the world. Must be called through the world for proper bookkeeping.
    #[doc(hidden)]
    pub fn spawn_with<R>(
        &mut self,
        position: PositionComponent,
        velocity: VelocityComponent,
        transform: TransformComponent,
        model: ModelComponent,
        mut world_registry: R
    ) -> EntityId
    where
        R: WorldEntityRegistry
    {
        self.positions.push(position);
        self.velocities.push(velocity);
        self.transforms.push(transform);
        self.models.push(model);

        let entity_id = EntityId::new();

        let entity_index = self.entities.len();
        self.entities.push(entity_id);

        let reference = EntityArchetypeRef {
            archetype: Self::ID,
            index: entity_index
        };

        world_registry.register(entity_id, reference)
    }

    /// Drop an entity at the specified index. Called by the world upon de-spawning entities.
    ///
    /// Returns the ID of the entity that was moved into the hole, or [`None`] if the archetype is now empty.
    #[doc(hidden)]
    pub fn drop_at_index(&mut self, index: usize) -> Result<Option<EntityId>, usize> {
        if index > self.entities.len() {
            return Err(index);
        }
        self.entities.swap_remove(index);
        self.positions.swap_remove(index);
        self.velocities.swap_remove(index);
        self.transforms.swap_remove(index);
        self.models.swap_remove(index);
        if self.entities.is_empty() {
            Ok(None)
        }
        else {
            Ok(Some(self.entities[index]))
        }
    }
}


#[automatically_derived]
impl HasComponents<PositionComponent> for CubeArchetype {
    /// Returns a slice of all [`PositionComponent`] component values.
    fn get(&self) -> &[PositionComponent] {
        self.positions.as_slice()
    }

    /// Returns a mutable slice of all [`PositionComponent`] component values.
    fn get_mut(&mut self) -> &mut [PositionComponent] {
        self.positions.as_mut_slice()
    }
}

#[automatically_derived]
impl HasComponents<VelocityComponent> for CubeArchetype {
    /// Returns a slice of all [`VelocityComponent`] component values.
    fn get(&self) -> &[VelocityComponent] {
        self.velocities.as_slice()
    }

    /// Returns a mutable slice of all [`VelocityComponent`] component values.
    fn get_mut(&mut self) -> &mut [VelocityComponent] {
        self.velocities.as_mut_slice()
    }
}

#[automatically_derived]
impl HasComponents<TransformComponent> for CubeArchetype {
    /// Returns a slice of all [`TransformComponent`] component values.
    fn get(&self) -> &[TransformComponent] {
        self.transforms.as_slice()
    }

    /// Returns a mutable slice of all [`TransformComponent`] component values.
    fn get_mut(&mut self) -> &mut [TransformComponent] {
        self.transforms.as_mut_slice()
    }
}

#[automatically_derived]
impl HasComponents<ModelComponent> for CubeArchetype {
    /// Returns a slice of all [`ModelComponent`] component values.
    fn get(&self) -> &[ModelComponent] {
        self.models.as_slice()
    }

    /// Returns a mutable slice of all [`ModelComponent`] component values.
    fn get_mut(&mut self) -> &mut [ModelComponent] {
        self.models.as_mut_slice()
    }
}


#[automatically_derived]
#[allow(dead_code)]
impl Archetype for CubeArchetype {
    const ID: ArchetypeId = ArchetypeId(core::num::NonZeroU64::new(3).expect("A zero archetype ID was provided"));

    fn len(&self) -> usize {
        self.entities.len()
    }

    fn is_empty(&self) -> bool {
        self.entities.is_empty()
    }
}

impl PartialEq<ArchetypeId> for CubeArchetype {
    fn eq(&self, other: &ArchetypeId) -> bool {
        Self::ID.eq(other)
    }
}

/// An archetype grouping entities with identical components.
#[derive(Debug, Default, Clone)]
pub struct PlayerArchetype {
    pub entities: Vec<EntityId>,
    pub positions: Vec<PositionComponent>,
    pub velocities: Vec<VelocityComponent>,
    pub healths: Vec<HealthComponent>,
}

/// An entity of the [`PlayerArchetype`].
pub type PlayerEntity = EntityWithIdAndData<PlayerEntityData>;

/// An entity of the [`PlayerArchetype`].
#[derive(Debug, Clone)]
#[allow(dead_code)]
pub struct PlayerEntityData {
    pub position: PositionComponent,
    pub velocity: VelocityComponent,
    pub health: HealthComponent,
}

impl EntityData for PlayerEntityData {
    const ARCHETYPE_ID: ArchetypeId = PlayerArchetype::ID;
}

/// A reference to an entity of the [`PlayerArchetype`].
#[derive(Debug, Clone)]
#[allow(dead_code)]
pub struct PlayerEntityRef<'archetype> {
    pub entity_id: EntityId,
    pub position: &'archetype PositionComponent,
    pub velocity: &'archetype VelocityComponent,
    pub health: &'archetype HealthComponent,
}

/// A mutable reference to an entity of the [`PlayerArchetype`].
#[derive(Debug)]
#[allow(dead_code)]
pub struct PlayerEntityMut<'archetype> {
    pub entity_id: EntityId,
    pub position: &'archetype mut PositionComponent,
    pub velocity: &'archetype mut VelocityComponent,
    pub health: &'archetype mut HealthComponent,
}

/// An iterator for entities in the [`PlayerArchetype`].
pub struct PlayerEntityIterator<'archetype> {
    archetype: &'archetype PlayerArchetype,
    current: usize,
    length: usize,
}

impl<'archetype> PlayerEntityIterator<'archetype> {
    fn new(archetype: &'archetype PlayerArchetype) -> Self {
        Self {
            archetype,
            current: 0,
            length: archetype.len()
        }
    }
}

impl<'archetype> Iterator for PlayerEntityIterator<'archetype> {
    type Item = PlayerEntityRef<'archetype>;

    #[inline]
    fn next(&mut self) -> Option<Self::Item> {
        if self.current >= self.length {
            return None;
        }

        let idx = self.current;
        self.current += 1;
        Some(unsafe { self.archetype.get_at_unchecked(idx) })
    }
}

impl PlayerArchetype {
    /// Iterates all entities in this archetype.
    #[inline]
    #[allow(dead_code)]
    pub fn iter(&self) -> PlayerEntityIterator {
        PlayerEntityIterator::new(self)
    }

    /// Gets the entity at the specified index.
    #[allow(dead_code)]
    pub fn get_at(&self, index: usize) -> Option<PlayerEntityRef> {
        if index > self.len() {
            return None;
        }
        Some(unsafe { self.get_at_unchecked(index) })
    }

    /// Gets the entity at the specified index.
    #[allow(dead_code)]
    pub unsafe fn get_at_unchecked(&self, index: usize) -> PlayerEntityRef {
        PlayerEntityRef {
            entity_id: *self.entities.get_unchecked(index),
            position: self.positions.get_unchecked(index),
            velocity: self.velocities.get_unchecked(index),
            health: self.healths.get_unchecked(index),
        }
    }

    /// Gets the entity at the specified index.
    #[allow(dead_code)]
    pub unsafe fn get_at_unchecked_mut(&mut self, index: usize) -> PlayerEntityMut {
        PlayerEntityMut {
            entity_id: *self.entities.get_unchecked(index),
            position: self.positions.get_unchecked_mut(index),
            velocity: self.velocities.get_unchecked_mut(index),
            health: self.healths.get_unchecked_mut(index),
        }
    }
}

#[allow(dead_code)]
impl HasComponent<PositionComponent> for PlayerEntity {
    #[inline]
    fn get(&self) -> &PositionComponent {
        &self.data.position
    }

    #[inline]
    fn get_mut(&mut self) -> &mut PositionComponent {
        &mut self.data.position
    }
}

#[allow(dead_code)]
impl HasComponent<PositionComponent> for PlayerEntityData {
    #[inline]
    fn get(&self) -> &PositionComponent {
        &self.position
    }

    #[inline]
    fn get_mut(&mut self) -> &mut PositionComponent {
        &mut self.position
    }
}

#[allow(dead_code)]
impl HasComponent<VelocityComponent> for PlayerEntity {
    #[inline]
    fn get(&self) -> &VelocityComponent {
        &self.data.velocity
    }

    #[inline]
    fn get_mut(&mut self) -> &mut VelocityComponent {
        &mut self.data.velocity
    }
}

#[allow(dead_code)]
impl HasComponent<VelocityComponent> for PlayerEntityData {
    #[inline]
    fn get(&self) -> &VelocityComponent {
        &self.velocity
    }

    #[inline]
    fn get_mut(&mut self) -> &mut VelocityComponent {
        &mut self.velocity
    }
}

#[allow(dead_code)]
impl HasComponent<HealthComponent> for PlayerEntity {
    #[inline]
    fn get(&self) -> &HealthComponent {
        &self.data.health
    }

    #[inline]
    fn get_mut(&mut self) -> &mut HealthComponent {
        &mut self.data.health
    }
}

#[allow(dead_code)]
impl HasComponent<HealthComponent> for PlayerEntityData {
    #[inline]
    fn get(&self) -> &HealthComponent {
        &self.health
    }

    #[inline]
    fn get_mut(&mut self) -> &mut HealthComponent {
        &mut self.health
    }
}
impl PlayerEntityData {
    /// Spawn this entity into the given world.
    #[inline]
    #[allow(dead_code)]
    pub fn spawn_into<W>(self, world: &mut W) -> EntityId
    where
        W: Spawn<PlayerEntityData>
    {
        world.spawn(self)
    }
}

impl core::ops::Deref for PlayerEntity {
    type Target = PlayerEntityData;

    fn deref(&self) -> &Self::Target {
        &self.data
    }
}

impl core::ops::DerefMut for PlayerEntity {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.data
    }
}

#[allow(dead_code)]
impl PlayerArchetype {
    /// The IDs of the components used by this archetype, in ascending order.
    pub const COMPONENT_IDS: [ComponentId; 3] = [
        ComponentId::new(3).expect("Invalid ID on ECS construction time"),
        ComponentId::new(5).expect("Invalid ID on ECS construction time"),
        ComponentId::new(6).expect("Invalid ID on ECS construction time"),
    ];

    /// Returns the number of entities in this archetype.
    #[inline(always)]
    pub fn len(&self) -> usize {
        self.entities.len()
    }

    /// Indicates whether this archetype has no entities.
    #[inline(always)]
    pub fn is_empty(&self) -> bool {
        self.entities.is_empty()
    }

    /// Returns the number of components used by this archetype.
    #[inline(always)]
    pub const fn num_components(&self) -> usize {
        3
    }

    /// Dynamically determines whether this archetype has a specific component.
    pub const fn has_components<Id>(&self, id: ComponentId) -> bool {
        let id = id.as_u64();
        id == 3 || id == 5 || id == 6 || false
    }

    /// Spawn a new entity into the world. Must be called through the world for proper bookkeeping.
    #[doc(hidden)]
    pub fn spawn_with<R>(
        &mut self,
        position: PositionComponent,
        velocity: VelocityComponent,
        health: HealthComponent,
        mut world_registry: R
    ) -> EntityId
    where
        R: WorldEntityRegistry
    {
        self.positions.push(position);
        self.velocities.push(velocity);
        self.healths.push(health);

        let entity_id = EntityId::new();

        let entity_index = self.entities.len();
        self.entities.push(entity_id);

        let reference = EntityArchetypeRef {
            archetype: Self::ID,
            index: entity_index
        };

        world_registry.register(entity_id, reference)
    }

    /// Drop an entity at the specified index. Called by the world upon de-spawning entities.
    ///
    /// Returns the ID of the entity that was moved into the hole, or [`None`] if the archetype is now empty.
    #[doc(hidden)]
    pub fn drop_at_index(&mut self, index: usize) -> Result<Option<EntityId>, usize> {
        if index > self.entities.len() {
            return Err(index);
        }
        self.entities.swap_remove(index);
        self.positions.swap_remove(index);
        self.velocities.swap_remove(index);
        self.healths.swap_remove(index);
        if self.entities.is_empty() {
            Ok(None)
        }
        else {
            Ok(Some(self.entities[index]))
        }
    }
}


#[automatically_derived]
impl HasComponents<PositionComponent> for PlayerArchetype {
    /// Returns a slice of all [`PositionComponent`] component values.
    fn get(&self) -> &[PositionComponent] {
        self.positions.as_slice()
    }

    /// Returns a mutable slice of all [`PositionComponent`] component values.
    fn get_mut(&mut self) -> &mut [PositionComponent] {
        self.positions.as_mut_slice()
    }
}

#[automatically_derived]
impl HasComponents<VelocityComponent> for PlayerArchetype {
    /// Returns a slice of all [`VelocityComponent`] component values.
    fn get(&self) -> &[VelocityComponent] {
        self.velocities.as_slice()
    }

    /// Returns a mutable slice of all [`VelocityComponent`] component values.
    fn get_mut(&mut self) -> &mut [VelocityComponent] {
        self.velocities.as_mut_slice()
    }
}

#[automatically_derived]
impl HasComponents<HealthComponent> for PlayerArchetype {
    /// Returns a slice of all [`HealthComponent`] component values.
    fn get(&self) -> &[HealthComponent] {
        self.healths.as_slice()
    }

    /// Returns a mutable slice of all [`HealthComponent`] component values.
    fn get_mut(&mut self) -> &mut [HealthComponent] {
        self.healths.as_mut_slice()
    }
}


#[automatically_derived]
#[allow(dead_code)]
impl Archetype for PlayerArchetype {
    const ID: ArchetypeId = ArchetypeId(core::num::NonZeroU64::new(4).expect("A zero archetype ID was provided"));

    fn len(&self) -> usize {
        self.entities.len()
    }

    fn is_empty(&self) -> bool {
        self.entities.is_empty()
    }
}

impl PartialEq<ArchetypeId> for PlayerArchetype {
    fn eq(&self, other: &ArchetypeId) -> bool {
        Self::ID.eq(other)
    }
}

/// An archetype grouping entities with identical components.
#[derive(Debug, Default, Clone)]
pub struct ForegroundObjectArchetype {
    pub entities: Vec<EntityId>,
    pub positions: Vec<PositionComponent>,
    pub colliders: Vec<ColliderComponent>,
}

/// An entity of the [`ForegroundObjectArchetype`].
pub type ForegroundObjectEntity = EntityWithIdAndData<ForegroundObjectEntityData>;

/// An entity of the [`ForegroundObjectArchetype`].
#[derive(Debug, Clone)]
#[allow(dead_code)]
pub struct ForegroundObjectEntityData {
    pub position: PositionComponent,
    pub collider: ColliderComponent,
}

impl EntityData for ForegroundObjectEntityData {
    const ARCHETYPE_ID: ArchetypeId = ForegroundObjectArchetype::ID;
}

/// A reference to an entity of the [`ForegroundObjectArchetype`].
#[derive(Debug, Clone)]
#[allow(dead_code)]
pub struct ForegroundObjectEntityRef<'archetype> {
    pub entity_id: EntityId,
    pub position: &'archetype PositionComponent,
    pub collider: &'archetype ColliderComponent,
}

/// A mutable reference to an entity of the [`ForegroundObjectArchetype`].
#[derive(Debug)]
#[allow(dead_code)]
pub struct ForegroundObjectEntityMut<'archetype> {
    pub entity_id: EntityId,
    pub position: &'archetype mut PositionComponent,
    pub collider: &'archetype mut ColliderComponent,
}

/// An iterator for entities in the [`ForegroundObjectArchetype`].
pub struct ForegroundObjectEntityIterator<'archetype> {
    archetype: &'archetype ForegroundObjectArchetype,
    current: usize,
    length: usize,
}

impl<'archetype> ForegroundObjectEntityIterator<'archetype> {
    fn new(archetype: &'archetype ForegroundObjectArchetype) -> Self {
        Self {
            archetype,
            current: 0,
            length: archetype.len()
        }
    }
}

impl<'archetype> Iterator for ForegroundObjectEntityIterator<'archetype> {
    type Item = ForegroundObjectEntityRef<'archetype>;

    #[inline]
    fn next(&mut self) -> Option<Self::Item> {
        if self.current >= self.length {
            return None;
        }

        let idx = self.current;
        self.current += 1;
        Some(unsafe { self.archetype.get_at_unchecked(idx) })
    }
}

impl ForegroundObjectArchetype {
    /// Iterates all entities in this archetype.
    #[inline]
    #[allow(dead_code)]
    pub fn iter(&self) -> ForegroundObjectEntityIterator {
        ForegroundObjectEntityIterator::new(self)
    }

    /// Gets the entity at the specified index.
    #[allow(dead_code)]
    pub fn get_at(&self, index: usize) -> Option<ForegroundObjectEntityRef> {
        if index > self.len() {
            return None;
        }
        Some(unsafe { self.get_at_unchecked(index) })
    }

    /// Gets the entity at the specified index.
    #[allow(dead_code)]
    pub unsafe fn get_at_unchecked(&self, index: usize) -> ForegroundObjectEntityRef {
        ForegroundObjectEntityRef {
            entity_id: *self.entities.get_unchecked(index),
            position: self.positions.get_unchecked(index),
            collider: self.colliders.get_unchecked(index),
        }
    }

    /// Gets the entity at the specified index.
    #[allow(dead_code)]
    pub unsafe fn get_at_unchecked_mut(&mut self, index: usize) -> ForegroundObjectEntityMut {
        ForegroundObjectEntityMut {
            entity_id: *self.entities.get_unchecked(index),
            position: self.positions.get_unchecked_mut(index),
            collider: self.colliders.get_unchecked_mut(index),
        }
    }
}

#[allow(dead_code)]
impl HasComponent<PositionComponent> for ForegroundObjectEntity {
    #[inline]
    fn get(&self) -> &PositionComponent {
        &self.data.position
    }

    #[inline]
    fn get_mut(&mut self) -> &mut PositionComponent {
        &mut self.data.position
    }
}

#[allow(dead_code)]
impl HasComponent<PositionComponent> for ForegroundObjectEntityData {
    #[inline]
    fn get(&self) -> &PositionComponent {
        &self.position
    }

    #[inline]
    fn get_mut(&mut self) -> &mut PositionComponent {
        &mut self.position
    }
}

#[allow(dead_code)]
impl HasComponent<ColliderComponent> for ForegroundObjectEntity {
    #[inline]
    fn get(&self) -> &ColliderComponent {
        &self.data.collider
    }

    #[inline]
    fn get_mut(&mut self) -> &mut ColliderComponent {
        &mut self.data.collider
    }
}

#[allow(dead_code)]
impl HasComponent<ColliderComponent> for ForegroundObjectEntityData {
    #[inline]
    fn get(&self) -> &ColliderComponent {
        &self.collider
    }

    #[inline]
    fn get_mut(&mut self) -> &mut ColliderComponent {
        &mut self.collider
    }
}
impl ForegroundObjectEntityData {
    /// Spawn this entity into the given world.
    #[inline]
    #[allow(dead_code)]
    pub fn spawn_into<W>(self, world: &mut W) -> EntityId
    where
        W: Spawn<ForegroundObjectEntityData>
    {
        world.spawn(self)
    }
}

impl core::ops::Deref for ForegroundObjectEntity {
    type Target = ForegroundObjectEntityData;

    fn deref(&self) -> &Self::Target {
        &self.data
    }
}

impl core::ops::DerefMut for ForegroundObjectEntity {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.data
    }
}

#[allow(dead_code)]
impl ForegroundObjectArchetype {
    /// The IDs of the components used by this archetype, in ascending order.
    pub const COMPONENT_IDS: [ComponentId; 2] = [
        ComponentId::new(3).expect("Invalid ID on ECS construction time"),
        ComponentId::new(7).expect("Invalid ID on ECS construction time"),
    ];

    /// Returns the number of entities in this archetype.
    #[inline(always)]
    pub fn len(&self) -> usize {
        self.entities.len()
    }

    /// Indicates whether this archetype has no entities.
    #[inline(always)]
    pub fn is_empty(&self) -> bool {
        self.entities.is_empty()
    }

    /// Returns the number of components used by this archetype.
    #[inline(always)]
    pub const fn num_components(&self) -> usize {
        2
    }

    /// Dynamically determines whether this archetype has a specific component.
    pub const fn has_components<Id>(&self, id: ComponentId) -> bool {
        let id = id.as_u64();
        id == 3 || id == 7 || false
    }

    /// Spawn a new entity into the world. Must be called through the world for proper bookkeeping.
    #[doc(hidden)]
    pub fn spawn_with<R>(
        &mut self,
        position: PositionComponent,
        collider: ColliderComponent,
        mut world_registry: R
    ) -> EntityId
    where
        R: WorldEntityRegistry
    {
        self.positions.push(position);
        self.colliders.push(collider);

        let entity_id = EntityId::new();

        let entity_index = self.entities.len();
        self.entities.push(entity_id);

        let reference = EntityArchetypeRef {
            archetype: Self::ID,
            index: entity_index
        };

        world_registry.register(entity_id, reference)
    }

    /// Drop an entity at the specified index. Called by the world upon de-spawning entities.
    ///
    /// Returns the ID of the entity that was moved into the hole, or [`None`] if the archetype is now empty.
    #[doc(hidden)]
    pub fn drop_at_index(&mut self, index: usize) -> Result<Option<EntityId>, usize> {
        if index > self.entities.len() {
            return Err(index);
        }
        self.entities.swap_remove(index);
        self.positions.swap_remove(index);
        self.colliders.swap_remove(index);
        if self.entities.is_empty() {
            Ok(None)
        }
        else {
            Ok(Some(self.entities[index]))
        }
    }
}


#[automatically_derived]
impl HasComponents<PositionComponent> for ForegroundObjectArchetype {
    /// Returns a slice of all [`PositionComponent`] component values.
    fn get(&self) -> &[PositionComponent] {
        self.positions.as_slice()
    }

    /// Returns a mutable slice of all [`PositionComponent`] component values.
    fn get_mut(&mut self) -> &mut [PositionComponent] {
        self.positions.as_mut_slice()
    }
}

#[automatically_derived]
impl HasComponents<ColliderComponent> for ForegroundObjectArchetype {
    /// Returns a slice of all [`ColliderComponent`] component values.
    fn get(&self) -> &[ColliderComponent] {
        self.colliders.as_slice()
    }

    /// Returns a mutable slice of all [`ColliderComponent`] component values.
    fn get_mut(&mut self) -> &mut [ColliderComponent] {
        self.colliders.as_mut_slice()
    }
}


#[automatically_derived]
#[allow(dead_code)]
impl Archetype for ForegroundObjectArchetype {
    const ID: ArchetypeId = ArchetypeId(core::num::NonZeroU64::new(5).expect("A zero archetype ID was provided"));

    fn len(&self) -> usize {
        self.entities.len()
    }

    fn is_empty(&self) -> bool {
        self.entities.is_empty()
    }
}

impl PartialEq<ArchetypeId> for ForegroundObjectArchetype {
    fn eq(&self, other: &ArchetypeId) -> bool {
        Self::ID.eq(other)
    }
}

/// An archetype grouping entities with identical components.
#[derive(Debug, Default, Clone)]
pub struct BackgroundObjectArchetype {
    pub entities: Vec<EntityId>,
    pub positions: Vec<PositionComponent>,
}

/// An entity of the [`BackgroundObjectArchetype`].
pub type BackgroundObjectEntity = EntityWithIdAndData<BackgroundObjectEntityData>;

/// An entity of the [`BackgroundObjectArchetype`].
#[derive(Debug, Clone)]
#[allow(dead_code)]
pub struct BackgroundObjectEntityData {
    pub position: PositionComponent,
}

impl EntityData for BackgroundObjectEntityData {
    const ARCHETYPE_ID: ArchetypeId = BackgroundObjectArchetype::ID;
}

/// A reference to an entity of the [`BackgroundObjectArchetype`].
#[derive(Debug, Clone)]
#[allow(dead_code)]
pub struct BackgroundObjectEntityRef<'archetype> {
    pub entity_id: EntityId,
    pub position: &'archetype PositionComponent,
}

/// A mutable reference to an entity of the [`BackgroundObjectArchetype`].
#[derive(Debug)]
#[allow(dead_code)]
pub struct BackgroundObjectEntityMut<'archetype> {
    pub entity_id: EntityId,
    pub position: &'archetype mut PositionComponent,
}

/// An iterator for entities in the [`BackgroundObjectArchetype`].
pub struct BackgroundObjectEntityIterator<'archetype> {
    archetype: &'archetype BackgroundObjectArchetype,
    current: usize,
    length: usize,
}

impl<'archetype> BackgroundObjectEntityIterator<'archetype> {
    fn new(archetype: &'archetype BackgroundObjectArchetype) -> Self {
        Self {
            archetype,
            current: 0,
            length: archetype.len()
        }
    }
}

impl<'archetype> Iterator for BackgroundObjectEntityIterator<'archetype> {
    type Item = BackgroundObjectEntityRef<'archetype>;

    #[inline]
    fn next(&mut self) -> Option<Self::Item> {
        if self.current >= self.length {
            return None;
        }

        let idx = self.current;
        self.current += 1;
        Some(unsafe { self.archetype.get_at_unchecked(idx) })
    }
}

impl BackgroundObjectArchetype {
    /// Iterates all entities in this archetype.
    #[inline]
    #[allow(dead_code)]
    pub fn iter(&self) -> BackgroundObjectEntityIterator {
        BackgroundObjectEntityIterator::new(self)
    }

    /// Gets the entity at the specified index.
    #[allow(dead_code)]
    pub fn get_at(&self, index: usize) -> Option<BackgroundObjectEntityRef> {
        if index > self.len() {
            return None;
        }
        Some(unsafe { self.get_at_unchecked(index) })
    }

    /// Gets the entity at the specified index.
    #[allow(dead_code)]
    pub unsafe fn get_at_unchecked(&self, index: usize) -> BackgroundObjectEntityRef {
        BackgroundObjectEntityRef {
            entity_id: *self.entities.get_unchecked(index),
            position: self.positions.get_unchecked(index),
        }
    }

    /// Gets the entity at the specified index.
    #[allow(dead_code)]
    pub unsafe fn get_at_unchecked_mut(&mut self, index: usize) -> BackgroundObjectEntityMut {
        BackgroundObjectEntityMut {
            entity_id: *self.entities.get_unchecked(index),
            position: self.positions.get_unchecked_mut(index),
        }
    }
}

#[allow(dead_code)]
impl HasComponent<PositionComponent> for BackgroundObjectEntity {
    #[inline]
    fn get(&self) -> &PositionComponent {
        &self.data.position
    }

    #[inline]
    fn get_mut(&mut self) -> &mut PositionComponent {
        &mut self.data.position
    }
}

#[allow(dead_code)]
impl HasComponent<PositionComponent> for BackgroundObjectEntityData {
    #[inline]
    fn get(&self) -> &PositionComponent {
        &self.position
    }

    #[inline]
    fn get_mut(&mut self) -> &mut PositionComponent {
        &mut self.position
    }
}
impl BackgroundObjectEntityData {
    /// Spawn this entity into the given world.
    #[inline]
    #[allow(dead_code)]
    pub fn spawn_into<W>(self, world: &mut W) -> EntityId
    where
        W: Spawn<BackgroundObjectEntityData>
    {
        world.spawn(self)
    }
}

impl core::ops::Deref for BackgroundObjectEntity {
    type Target = BackgroundObjectEntityData;

    fn deref(&self) -> &Self::Target {
        &self.data
    }
}

impl core::ops::DerefMut for BackgroundObjectEntity {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.data
    }
}

#[allow(dead_code)]
impl BackgroundObjectArchetype {
    /// The IDs of the components used by this archetype, in ascending order.
    pub const COMPONENT_IDS: [ComponentId; 1] = [
        ComponentId::new(3).expect("Invalid ID on ECS construction time"),
    ];

    /// Returns the number of entities in this archetype.
    #[inline(always)]
    pub fn len(&self) -> usize {
        self.entities.len()
    }

    /// Indicates whether this archetype has no entities.
    #[inline(always)]
    pub fn is_empty(&self) -> bool {
        self.entities.is_empty()
    }

    /// Returns the number of components used by this archetype.
    #[inline(always)]
    pub const fn num_components(&self) -> usize {
        1
    }

    /// Dynamically determines whether this archetype has a specific component.
    pub const fn has_components<Id>(&self, id: ComponentId) -> bool {
        let id = id.as_u64();
        id == 3 || false
    }

    /// Spawn a new entity into the world. Must be called through the world for proper bookkeeping.
    #[doc(hidden)]
    pub fn spawn_with<R>(
        &mut self,
        position: PositionComponent,
        mut world_registry: R
    ) -> EntityId
    where
        R: WorldEntityRegistry
    {
        self.positions.push(position);

        let entity_id = EntityId::new();

        let entity_index = self.entities.len();
        self.entities.push(entity_id);

        let reference = EntityArchetypeRef {
            archetype: Self::ID,
            index: entity_index
        };

        world_registry.register(entity_id, reference)
    }

    /// Drop an entity at the specified index. Called by the world upon de-spawning entities.
    ///
    /// Returns the ID of the entity that was moved into the hole, or [`None`] if the archetype is now empty.
    #[doc(hidden)]
    pub fn drop_at_index(&mut self, index: usize) -> Result<Option<EntityId>, usize> {
        if index > self.entities.len() {
            return Err(index);
        }
        self.entities.swap_remove(index);
        self.positions.swap_remove(index);
        if self.entities.is_empty() {
            Ok(None)
        }
        else {
            Ok(Some(self.entities[index]))
        }
    }
}


#[automatically_derived]
impl HasComponents<PositionComponent> for BackgroundObjectArchetype {
    /// Returns a slice of all [`PositionComponent`] component values.
    fn get(&self) -> &[PositionComponent] {
        self.positions.as_slice()
    }

    /// Returns a mutable slice of all [`PositionComponent`] component values.
    fn get_mut(&mut self) -> &mut [PositionComponent] {
        self.positions.as_mut_slice()
    }
}


#[automatically_derived]
#[allow(dead_code)]
impl Archetype for BackgroundObjectArchetype {
    const ID: ArchetypeId = ArchetypeId(core::num::NonZeroU64::new(6).expect("A zero archetype ID was provided"));

    fn len(&self) -> usize {
        self.entities.len()
    }

    fn is_empty(&self) -> bool {
        self.entities.is_empty()
    }
}

impl PartialEq<ArchetypeId> for BackgroundObjectArchetype {
    fn eq(&self, other: &ArchetypeId) -> bool {
        Self::ID.eq(other)
    }
}

/// The scene background.
#[derive(Debug, Default, Clone)]
pub struct SceneBackgroundArchetype {
    pub entities: Vec<EntityId>,
    pub background_colors: Vec<BackgroundColorComponent>,
}

/// An entity of the [`SceneBackgroundArchetype`].
pub type SceneBackgroundEntity = EntityWithIdAndData<SceneBackgroundEntityData>;

/// An entity of the [`SceneBackgroundArchetype`].
#[derive(Debug, Clone)]
#[allow(dead_code)]
pub struct SceneBackgroundEntityData {
    pub background_color: BackgroundColorComponent,
}

impl EntityData for SceneBackgroundEntityData {
    const ARCHETYPE_ID: ArchetypeId = SceneBackgroundArchetype::ID;
}

/// A reference to an entity of the [`SceneBackgroundArchetype`].
#[derive(Debug, Clone)]
#[allow(dead_code)]
pub struct SceneBackgroundEntityRef<'archetype> {
    pub entity_id: EntityId,
    pub background_color: &'archetype BackgroundColorComponent,
}

/// A mutable reference to an entity of the [`SceneBackgroundArchetype`].
#[derive(Debug)]
#[allow(dead_code)]
pub struct SceneBackgroundEntityMut<'archetype> {
    pub entity_id: EntityId,
    pub background_color: &'archetype mut BackgroundColorComponent,
}

/// An iterator for entities in the [`SceneBackgroundArchetype`].
pub struct SceneBackgroundEntityIterator<'archetype> {
    archetype: &'archetype SceneBackgroundArchetype,
    current: usize,
    length: usize,
}

impl<'archetype> SceneBackgroundEntityIterator<'archetype> {
    fn new(archetype: &'archetype SceneBackgroundArchetype) -> Self {
        Self {
            archetype,
            current: 0,
            length: archetype.len()
        }
    }
}

impl<'archetype> Iterator for SceneBackgroundEntityIterator<'archetype> {
    type Item = SceneBackgroundEntityRef<'archetype>;

    #[inline]
    fn next(&mut self) -> Option<Self::Item> {
        if self.current >= self.length {
            return None;
        }

        let idx = self.current;
        self.current += 1;
        Some(unsafe { self.archetype.get_at_unchecked(idx) })
    }
}

impl SceneBackgroundArchetype {
    /// Iterates all entities in this archetype.
    #[inline]
    #[allow(dead_code)]
    pub fn iter(&self) -> SceneBackgroundEntityIterator {
        SceneBackgroundEntityIterator::new(self)
    }

    /// Gets the entity at the specified index.
    #[allow(dead_code)]
    pub fn get_at(&self, index: usize) -> Option<SceneBackgroundEntityRef> {
        if index > self.len() {
            return None;
        }
        Some(unsafe { self.get_at_unchecked(index) })
    }

    /// Gets the entity at the specified index.
    #[allow(dead_code)]
    pub unsafe fn get_at_unchecked(&self, index: usize) -> SceneBackgroundEntityRef {
        SceneBackgroundEntityRef {
            entity_id: *self.entities.get_unchecked(index),
            background_color: self.background_colors.get_unchecked(index),
        }
    }

    /// Gets the entity at the specified index.
    #[allow(dead_code)]
    pub unsafe fn get_at_unchecked_mut(&mut self, index: usize) -> SceneBackgroundEntityMut {
        SceneBackgroundEntityMut {
            entity_id: *self.entities.get_unchecked(index),
            background_color: self.background_colors.get_unchecked_mut(index),
        }
    }
}

#[allow(dead_code)]
impl HasComponent<BackgroundColorComponent> for SceneBackgroundEntity {
    #[inline]
    fn get(&self) -> &BackgroundColorComponent {
        &self.data.background_color
    }

    #[inline]
    fn get_mut(&mut self) -> &mut BackgroundColorComponent {
        &mut self.data.background_color
    }
}

#[allow(dead_code)]
impl HasComponent<BackgroundColorComponent> for SceneBackgroundEntityData {
    #[inline]
    fn get(&self) -> &BackgroundColorComponent {
        &self.background_color
    }

    #[inline]
    fn get_mut(&mut self) -> &mut BackgroundColorComponent {
        &mut self.background_color
    }
}
impl SceneBackgroundEntityData {
    /// Spawn this entity into the given world.
    #[inline]
    #[allow(dead_code)]
    pub fn spawn_into<W>(self, world: &mut W) -> EntityId
    where
        W: Spawn<SceneBackgroundEntityData>
    {
        world.spawn(self)
    }
}

impl core::ops::Deref for SceneBackgroundEntity {
    type Target = SceneBackgroundEntityData;

    fn deref(&self) -> &Self::Target {
        &self.data
    }
}

impl core::ops::DerefMut for SceneBackgroundEntity {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.data
    }
}

#[allow(dead_code)]
impl SceneBackgroundArchetype {
    /// The IDs of the components used by this archetype, in ascending order.
    pub const COMPONENT_IDS: [ComponentId; 1] = [
        ComponentId::new(10).expect("Invalid ID on ECS construction time"),
    ];

    /// Returns the number of entities in this archetype.
    #[inline(always)]
    pub fn len(&self) -> usize {
        self.entities.len()
    }

    /// Indicates whether this archetype has no entities.
    #[inline(always)]
    pub fn is_empty(&self) -> bool {
        self.entities.is_empty()
    }

    /// Returns the number of components used by this archetype.
    #[inline(always)]
    pub const fn num_components(&self) -> usize {
        1
    }

    /// Dynamically determines whether this archetype has a specific component.
    pub const fn has_components<Id>(&self, id: ComponentId) -> bool {
        let id = id.as_u64();
        id == 10 || false
    }

    /// Spawn a new entity into the world. Must be called through the world for proper bookkeeping.
    #[doc(hidden)]
    pub fn spawn_with<R>(
        &mut self,
        background_color: BackgroundColorComponent,
        mut world_registry: R
    ) -> EntityId
    where
        R: WorldEntityRegistry
    {
        self.background_colors.push(background_color);

        let entity_id = EntityId::new();

        let entity_index = self.entities.len();
        self.entities.push(entity_id);

        let reference = EntityArchetypeRef {
            archetype: Self::ID,
            index: entity_index
        };

        world_registry.register(entity_id, reference)
    }

    /// Drop an entity at the specified index. Called by the world upon de-spawning entities.
    ///
    /// Returns the ID of the entity that was moved into the hole, or [`None`] if the archetype is now empty.
    #[doc(hidden)]
    pub fn drop_at_index(&mut self, index: usize) -> Result<Option<EntityId>, usize> {
        if index > self.entities.len() {
            return Err(index);
        }
        self.entities.swap_remove(index);
        self.background_colors.swap_remove(index);
        if self.entities.is_empty() {
            Ok(None)
        }
        else {
            Ok(Some(self.entities[index]))
        }
    }
}


#[automatically_derived]
impl HasComponents<BackgroundColorComponent> for SceneBackgroundArchetype {
    /// Returns a slice of all [`BackgroundColorComponent`] component values.
    fn get(&self) -> &[BackgroundColorComponent] {
        self.background_colors.as_slice()
    }

    /// Returns a mutable slice of all [`BackgroundColorComponent`] component values.
    fn get_mut(&mut self) -> &mut [BackgroundColorComponent] {
        self.background_colors.as_mut_slice()
    }
}


#[automatically_derived]
#[allow(dead_code)]
impl Archetype for SceneBackgroundArchetype {
    const ID: ArchetypeId = ArchetypeId(core::num::NonZeroU64::new(7).expect("A zero archetype ID was provided"));

    fn len(&self) -> usize {
        self.entities.len()
    }

    fn is_empty(&self) -> bool {
        self.entities.is_empty()
    }
}

impl PartialEq<ArchetypeId> for SceneBackgroundArchetype {
    fn eq(&self, other: &ArchetypeId) -> bool {
        Self::ID.eq(other)
    }
}

pub trait GetEntityRef {
    type EntityRef<'archetype>
    where
        Self: 'archetype;

    #[allow(clippy::needless_lifetimes, dead_code)]
    fn get_at<'archetype>(&'archetype self, index: usize) -> Option<Self::EntityRef<'archetype>>;

    #[allow(clippy::needless_lifetimes, dead_code)]
    unsafe fn get_at_unchecked<'archetype>(&'archetype self, index: usize) -> Self::EntityRef<'archetype>;
}

impl core::ops::Index<usize> for CameraArchetype {
    type Output = EntityId;

    fn index(&self, index: usize) -> &Self::Output {
        &self.entities[index]
    }
}

impl core::ops::Index<usize> for DynamicLightArchetype {
    type Output = EntityId;

    fn index(&self, index: usize) -> &Self::Output {
        &self.entities[index]
    }
}

impl core::ops::Index<usize> for CubeArchetype {
    type Output = EntityId;

    fn index(&self, index: usize) -> &Self::Output {
        &self.entities[index]
    }
}

impl core::ops::Index<usize> for PlayerArchetype {
    type Output = EntityId;

    fn index(&self, index: usize) -> &Self::Output {
        &self.entities[index]
    }
}

impl core::ops::Index<usize> for ForegroundObjectArchetype {
    type Output = EntityId;

    fn index(&self, index: usize) -> &Self::Output {
        &self.entities[index]
    }
}

impl core::ops::Index<usize> for BackgroundObjectArchetype {
    type Output = EntityId;

    fn index(&self, index: usize) -> &Self::Output {
        &self.entities[index]
    }
}

impl core::ops::Index<usize> for SceneBackgroundArchetype {
    type Output = EntityId;

    fn index(&self, index: usize) -> &Self::Output {
        &self.entities[index]
    }
}

#[allow(dead_code)]
pub trait FrontloadEntities:
    Archetype
    + core::ops::Index<usize, Output = EntityId>
{
    /// Frontloads entities provided by their IDs (e.g., from quadtree results).
    /// Returns the count of frontloaded entities.
    fn frontload(
        &mut self,
        entity_lookup: &mut EntityLocationMap<EntityId, EntityArchetypeRef>,
        entities_to_frontload: &[EntityId],
        previous_frontload_pivot: Option<usize>
    ) -> usize {
        // Collect and filter valid indices
        let mut indices_to_frontload: Vec<usize> = entities_to_frontload
            .iter()
            .filter_map(|id| entity_lookup.get(id))
            .filter(|id| id.archetype == Self::ID)
            .map(|id| id.index)
            .collect();

        // Sort indices to achieve linear access patterns
        // TODO: Measure; might be overkill for small numbers
        indices_to_frontload.sort_unstable();

        self.frontload_by_indices_sorted(entity_lookup, indices_to_frontload, previous_frontload_pivot)
    }

    /// Frontloads entities provided by their IDs (e.g., from quadtree results).
    ///
    /// This function operates directly on indices and assumes that the index list is sorted
    /// for coherent cache access. It works with unsorted lists but may have degraded performance.
    ///
    /// Returns the count of frontloaded entities.
    fn frontload_by_indices_sorted<I>(
        &mut self,
        entity_lookup: &mut EntityLocationMap<EntityId, EntityArchetypeRef>,
        indices_to_frontload: I,
        previous_frontload_pivot: Option<usize>
    ) -> usize
    where
        I: AsRef<[usize]>
    {
        let mut frontload_target_idx = previous_frontload_pivot.unwrap_or_default();
        let mut swaps_made = Vec::new();

        for &current_idx in indices_to_frontload.as_ref() {
            // If already frontloaded, skip
            if current_idx < frontload_target_idx {
                continue;
            }

            // If entity is exactly where it should be, no swap needed
            if current_idx == frontload_target_idx {
                frontload_target_idx += 1;
                continue;
            }

            // Swap the entity into the target frontload position
            self.swap_entities(current_idx, frontload_target_idx);

            // Track swapped entity IDs for lookup update
            let swapped_entity_id = self[current_idx];
            let frontloaded_entity_id = self[frontload_target_idx];

            swaps_made.push((frontloaded_entity_id, frontload_target_idx));
            swaps_made.push((swapped_entity_id, current_idx));

            frontload_target_idx += 1;
        }

        // Update only swapped entity positions in the lookup map
        for (eid, index) in swaps_made {
            let reference = EntityArchetypeRef {
                archetype: Self::ID,
                index
            };
            entity_lookup.insert(eid, reference);
        }

        frontload_target_idx
    }

    /// Swaps two entities in the archetype.
    ///
    /// Note that this function is meant to be called internally and does **not** perform any bookkeeping
    /// such as updating the world's entity location map.
    #[doc(hidden)]
    fn swap_entities(&mut self, current_idx: usize, new_idx: usize);
}

#[allow(dead_code)]
pub trait FrontloadEntitiesScan:
    Archetype
    + FrontloadEntities
    + GetEntityRef
    + core::ops::Index<usize, Output = EntityId>
{
    /// Frontloads entities satisfying `should_frontload`.
    /// Returns the number of entities frontloaded.
    fn frontload_scan<F>(
        &mut self,
        entity_lookup: &mut EntityLocationMap<EntityId, EntityArchetypeRef>,
        mut should_frontload: F,
    ) -> usize
    where
        for<'archetype> F: FnMut(&<Self as GetEntityRef>::EntityRef<'archetype>) -> bool,
    {
        let mut left = 0;
        let mut right = self.len().wrapping_sub(1);
        let mut swaps_made = Vec::new();

        loop {
            // Move left pointer forward until an invalid entry is found
            while left <= right {
                let reference = unsafe { self.get_at_unchecked(left) };
                if should_frontload(&reference) {
                    left += 1;
                } else {
                    break;
                }
            }

            // Move right pointer backward until a valid entry is found
            while left <= right {
                let reference = unsafe { self.get_at_unchecked(right) };
                if !should_frontload(&reference) {
                    right = right.wrapping_sub(1);
                } else {
                    break;
                }
            }

            if left >= right {
                break;
            }

            // Swap entities and record swapped indices
            self.swap_entities(left, right);
            swaps_made.push((left, self[left]));
            swaps_made.push((right, self[right]));

            left += 1;
            right = right.wrapping_sub(1);
        }

        // Only update indices for swapped entities
        for (index, entity_id) in swaps_made {
            let reference = EntityArchetypeRef {
                archetype: Self::ID,
                index,
            };
            entity_lookup.insert(entity_id, reference);
        }

        // At this point, `left` indicates how many entities are valid (frontloaded)
        left
    }
}

impl GetEntityRef for CameraArchetype {
    type EntityRef<'archetype> = CameraEntityRef<'archetype>;

    /// See [`CameraArchetype::get_at`](CameraArchetype::get_at).
    #[inline(always)]
    #[allow(clippy::needless_lifetimes, dead_code)]
    fn get_at<'archetype>(&'archetype self, index: usize) -> Option<Self::EntityRef<'archetype>> {
        self.get_at(index)
    }

    /// See [`CameraArchetype::get_at_unchecked`](CameraArchetype::get_at_unchecked).
    #[inline(always)]
    #[allow(clippy::needless_lifetimes, dead_code)]
    unsafe fn get_at_unchecked<'archetype>(&'archetype self, index: usize) -> Self::EntityRef<'archetype> {
        unsafe { self.get_at_unchecked(index) }
    }
}

impl GetEntityRef for DynamicLightArchetype {
    type EntityRef<'archetype> = DynamicLightEntityRef<'archetype>;

    /// See [`DynamicLightArchetype::get_at`](DynamicLightArchetype::get_at).
    #[inline(always)]
    #[allow(clippy::needless_lifetimes, dead_code)]
    fn get_at<'archetype>(&'archetype self, index: usize) -> Option<Self::EntityRef<'archetype>> {
        self.get_at(index)
    }

    /// See [`DynamicLightArchetype::get_at_unchecked`](DynamicLightArchetype::get_at_unchecked).
    #[inline(always)]
    #[allow(clippy::needless_lifetimes, dead_code)]
    unsafe fn get_at_unchecked<'archetype>(&'archetype self, index: usize) -> Self::EntityRef<'archetype> {
        unsafe { self.get_at_unchecked(index) }
    }
}

impl GetEntityRef for CubeArchetype {
    type EntityRef<'archetype> = CubeEntityRef<'archetype>;

    /// See [`CubeArchetype::get_at`](CubeArchetype::get_at).
    #[inline(always)]
    #[allow(clippy::needless_lifetimes, dead_code)]
    fn get_at<'archetype>(&'archetype self, index: usize) -> Option<Self::EntityRef<'archetype>> {
        self.get_at(index)
    }

    /// See [`CubeArchetype::get_at_unchecked`](CubeArchetype::get_at_unchecked).
    #[inline(always)]
    #[allow(clippy::needless_lifetimes, dead_code)]
    unsafe fn get_at_unchecked<'archetype>(&'archetype self, index: usize) -> Self::EntityRef<'archetype> {
        unsafe { self.get_at_unchecked(index) }
    }
}

impl GetEntityRef for PlayerArchetype {
    type EntityRef<'archetype> = PlayerEntityRef<'archetype>;

    /// See [`PlayerArchetype::get_at`](PlayerArchetype::get_at).
    #[inline(always)]
    #[allow(clippy::needless_lifetimes, dead_code)]
    fn get_at<'archetype>(&'archetype self, index: usize) -> Option<Self::EntityRef<'archetype>> {
        self.get_at(index)
    }

    /// See [`PlayerArchetype::get_at_unchecked`](PlayerArchetype::get_at_unchecked).
    #[inline(always)]
    #[allow(clippy::needless_lifetimes, dead_code)]
    unsafe fn get_at_unchecked<'archetype>(&'archetype self, index: usize) -> Self::EntityRef<'archetype> {
        unsafe { self.get_at_unchecked(index) }
    }
}

impl GetEntityRef for ForegroundObjectArchetype {
    type EntityRef<'archetype> = ForegroundObjectEntityRef<'archetype>;

    /// See [`ForegroundObjectArchetype::get_at`](ForegroundObjectArchetype::get_at).
    #[inline(always)]
    #[allow(clippy::needless_lifetimes, dead_code)]
    fn get_at<'archetype>(&'archetype self, index: usize) -> Option<Self::EntityRef<'archetype>> {
        self.get_at(index)
    }

    /// See [`ForegroundObjectArchetype::get_at_unchecked`](ForegroundObjectArchetype::get_at_unchecked).
    #[inline(always)]
    #[allow(clippy::needless_lifetimes, dead_code)]
    unsafe fn get_at_unchecked<'archetype>(&'archetype self, index: usize) -> Self::EntityRef<'archetype> {
        unsafe { self.get_at_unchecked(index) }
    }
}

impl GetEntityRef for BackgroundObjectArchetype {
    type EntityRef<'archetype> = BackgroundObjectEntityRef<'archetype>;

    /// See [`BackgroundObjectArchetype::get_at`](BackgroundObjectArchetype::get_at).
    #[inline(always)]
    #[allow(clippy::needless_lifetimes, dead_code)]
    fn get_at<'archetype>(&'archetype self, index: usize) -> Option<Self::EntityRef<'archetype>> {
        self.get_at(index)
    }

    /// See [`BackgroundObjectArchetype::get_at_unchecked`](BackgroundObjectArchetype::get_at_unchecked).
    #[inline(always)]
    #[allow(clippy::needless_lifetimes, dead_code)]
    unsafe fn get_at_unchecked<'archetype>(&'archetype self, index: usize) -> Self::EntityRef<'archetype> {
        unsafe { self.get_at_unchecked(index) }
    }
}

impl GetEntityRef for SceneBackgroundArchetype {
    type EntityRef<'archetype> = SceneBackgroundEntityRef<'archetype>;

    /// See [`SceneBackgroundArchetype::get_at`](SceneBackgroundArchetype::get_at).
    #[inline(always)]
    #[allow(clippy::needless_lifetimes, dead_code)]
    fn get_at<'archetype>(&'archetype self, index: usize) -> Option<Self::EntityRef<'archetype>> {
        self.get_at(index)
    }

    /// See [`SceneBackgroundArchetype::get_at_unchecked`](SceneBackgroundArchetype::get_at_unchecked).
    #[inline(always)]
    #[allow(clippy::needless_lifetimes, dead_code)]
    unsafe fn get_at_unchecked<'archetype>(&'archetype self, index: usize) -> Self::EntityRef<'archetype> {
        unsafe { self.get_at_unchecked(index) }
    }
}

impl FrontloadEntities for CameraArchetype {
    #[inline(always)]
    #[doc(hidden)]
    fn swap_entities(&mut self, first: usize, second: usize) {
        self.entities.swap(first, second);
        self.cameras.swap(first, second);
        self.camera_fps_controllers.swap(first, second);
    }
}

impl FrontloadEntitiesScan for CameraArchetype { }

impl FrontloadEntities for DynamicLightArchetype {
    #[inline(always)]
    #[doc(hidden)]
    fn swap_entities(&mut self, first: usize, second: usize) {
        self.entities.swap(first, second);
        self.dynamic_lights.swap(first, second);
        self.colors.swap(first, second);
        self.positions.swap(first, second);
    }
}

impl FrontloadEntitiesScan for DynamicLightArchetype { }

impl FrontloadEntities for CubeArchetype {
    #[inline(always)]
    #[doc(hidden)]
    fn swap_entities(&mut self, first: usize, second: usize) {
        self.entities.swap(first, second);
        self.positions.swap(first, second);
        self.velocities.swap(first, second);
        self.transforms.swap(first, second);
        self.models.swap(first, second);
    }
}

impl FrontloadEntitiesScan for CubeArchetype { }

impl FrontloadEntities for PlayerArchetype {
    #[inline(always)]
    #[doc(hidden)]
    fn swap_entities(&mut self, first: usize, second: usize) {
        self.entities.swap(first, second);
        self.positions.swap(first, second);
        self.velocities.swap(first, second);
        self.healths.swap(first, second);
    }
}

impl FrontloadEntitiesScan for PlayerArchetype { }

impl FrontloadEntities for ForegroundObjectArchetype {
    #[inline(always)]
    #[doc(hidden)]
    fn swap_entities(&mut self, first: usize, second: usize) {
        self.entities.swap(first, second);
        self.positions.swap(first, second);
        self.colliders.swap(first, second);
    }
}

impl FrontloadEntitiesScan for ForegroundObjectArchetype { }

impl FrontloadEntities for BackgroundObjectArchetype {
    #[inline(always)]
    #[doc(hidden)]
    fn swap_entities(&mut self, first: usize, second: usize) {
        self.entities.swap(first, second);
        self.positions.swap(first, second);
    }
}

impl FrontloadEntitiesScan for BackgroundObjectArchetype { }

impl FrontloadEntities for SceneBackgroundArchetype {
    #[inline(always)]
    #[doc(hidden)]
    fn swap_entities(&mut self, first: usize, second: usize) {
        self.entities.swap(first, second);
        self.background_colors.swap(first, second);
    }
}

impl FrontloadEntitiesScan for SceneBackgroundArchetype { }