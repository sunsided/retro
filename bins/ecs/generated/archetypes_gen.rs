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
            2 => write!(f, "Particle (Archetype ID 2)"),
            3 => write!(f, "Player (Archetype ID 3)"),
            4 => write!(f, "ForegroundObject (Archetype ID 4)"),
            5 => write!(f, "BackgroundObject (Archetype ID 5)"),
            6 => write!(f, "SceneBackground (Archetype ID 6)"),
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
    /// A particle system particle.
    Particle(ParticleEntity),
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
    /// A particle system particle.
    Particle(ParticleEntityData),
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
            Self::Particle(_) => ParticleArchetype::ID,
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
            Self::Particle(_) => ParticleArchetype::ID,
            Self::Player(_) => PlayerArchetype::ID,
            Self::ForegroundObject(_) => ForegroundObjectArchetype::ID,
            Self::BackgroundObject(_) => BackgroundObjectArchetype::ID,
            Self::SceneBackground(_) => SceneBackgroundArchetype::ID,
        }
    }
}

/// An archetype grouping entities with identical components.
#[derive(Debug, Default, Clone)]
pub struct CameraArchetype {
    pub entities: Vec<EntityId>,
    pub cameras: Vec<CameraComponent>,
    pub camera_fps_controllers: Vec<CameraFpsControllerComponent>,
}

/// An entity of the [`CameraArchetype`].
#[derive(Debug, Clone)]
#[allow(dead_code)]
pub struct CameraEntity {
    pub id: EntityId,
    pub data: CameraEntityData,
}

/// An entity of the [`CameraArchetype`].
#[derive(Debug, Clone)]
#[allow(dead_code)]
pub struct CameraEntityData {
    pub camera: CameraComponent,
    pub camera_fps_controller: CameraFpsControllerComponent,
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

/// A particle system particle.
#[derive(Debug, Default, Clone)]
pub struct ParticleArchetype {
    pub entities: Vec<EntityId>,
    pub positions: Vec<PositionComponent>,
    pub velocities: Vec<VelocityComponent>,
}

/// An entity of the [`ParticleArchetype`].
#[derive(Debug, Clone)]
#[allow(dead_code)]
pub struct ParticleEntity {
    pub id: EntityId,
    pub data: ParticleEntityData,
}

/// An entity of the [`ParticleArchetype`].
#[derive(Debug, Clone)]
#[allow(dead_code)]
pub struct ParticleEntityData {
    pub position: PositionComponent,
    pub velocity: VelocityComponent,
}

#[allow(dead_code)]
impl HasComponent<PositionComponent> for ParticleEntity {
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
impl HasComponent<PositionComponent> for ParticleEntityData {
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
impl HasComponent<VelocityComponent> for ParticleEntity {
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
impl HasComponent<VelocityComponent> for ParticleEntityData {
    #[inline]
    fn get(&self) -> &VelocityComponent {
        &self.velocity
    }

    #[inline]
    fn get_mut(&mut self) -> &mut VelocityComponent {
        &mut self.velocity
    }
}
impl ParticleEntityData {
    /// Spawn this entity into the given world.
    #[inline]
    #[allow(dead_code)]
    pub fn spawn_into<W>(self, world: &mut W) -> EntityId
    where
        W: Spawn<ParticleEntityData>
    {
        world.spawn(self)
    }
}

impl core::ops::Deref for ParticleEntity {
    type Target = ParticleEntityData;

    fn deref(&self) -> &Self::Target {
        &self.data
    }
}

impl core::ops::DerefMut for ParticleEntity {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.data
    }
}

#[allow(dead_code)]
impl ParticleArchetype {
    /// The IDs of the components used by this archetype, in ascending order.
    pub const COMPONENT_IDS: [ComponentId; 2] = [
        ComponentId::new(3).expect("Invalid ID on ECS construction time"),
        ComponentId::new(4).expect("Invalid ID on ECS construction time"),
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
        id == 3 || id == 4 || false
    }

    /// Spawn a new entity into the world. Must be called through the world for proper bookkeeping.
    #[doc(hidden)]
    pub fn spawn_with<R>(
        &mut self,
        position: PositionComponent,
        velocity: VelocityComponent,
        mut world_registry: R
    ) -> EntityId
    where
        R: WorldEntityRegistry
    {
        self.positions.push(position);
        self.velocities.push(velocity);

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
        if self.entities.is_empty() {
            Ok(None)
        }
        else {
            Ok(Some(self.entities[index]))
        }
    }
}


#[automatically_derived]
impl HasComponents<PositionComponent> for ParticleArchetype {
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
impl HasComponents<VelocityComponent> for ParticleArchetype {
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
#[allow(dead_code)]
impl Archetype for ParticleArchetype {
    const ID: ArchetypeId = ArchetypeId(core::num::NonZeroU64::new(2).expect("A zero archetype ID was provided"));

    fn len(&self) -> usize {
        self.entities.len()
    }

    fn is_empty(&self) -> bool {
        self.entities.is_empty()
    }
}

impl PartialEq<ArchetypeId> for ParticleArchetype {
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
#[derive(Debug, Clone)]
#[allow(dead_code)]
pub struct PlayerEntity {
    pub id: EntityId,
    pub data: PlayerEntityData,
}

/// An entity of the [`PlayerArchetype`].
#[derive(Debug, Clone)]
#[allow(dead_code)]
pub struct PlayerEntityData {
    pub position: PositionComponent,
    pub velocity: VelocityComponent,
    pub health: HealthComponent,
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
        ComponentId::new(4).expect("Invalid ID on ECS construction time"),
        ComponentId::new(5).expect("Invalid ID on ECS construction time"),
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
        id == 3 || id == 4 || id == 5 || false
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
    const ID: ArchetypeId = ArchetypeId(core::num::NonZeroU64::new(3).expect("A zero archetype ID was provided"));

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
#[derive(Debug, Clone)]
#[allow(dead_code)]
pub struct ForegroundObjectEntity {
    pub id: EntityId,
    pub data: ForegroundObjectEntityData,
}

/// An entity of the [`ForegroundObjectArchetype`].
#[derive(Debug, Clone)]
#[allow(dead_code)]
pub struct ForegroundObjectEntityData {
    pub position: PositionComponent,
    pub collider: ColliderComponent,
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
        2
    }

    /// Dynamically determines whether this archetype has a specific component.
    pub const fn has_components<Id>(&self, id: ComponentId) -> bool {
        let id = id.as_u64();
        id == 3 || id == 6 || false
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
    const ID: ArchetypeId = ArchetypeId(core::num::NonZeroU64::new(4).expect("A zero archetype ID was provided"));

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
#[derive(Debug, Clone)]
#[allow(dead_code)]
pub struct BackgroundObjectEntity {
    pub id: EntityId,
    pub data: BackgroundObjectEntityData,
}

/// An entity of the [`BackgroundObjectArchetype`].
#[derive(Debug, Clone)]
#[allow(dead_code)]
pub struct BackgroundObjectEntityData {
    pub position: PositionComponent,
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
    const ID: ArchetypeId = ArchetypeId(core::num::NonZeroU64::new(5).expect("A zero archetype ID was provided"));

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
#[derive(Debug, Clone)]
#[allow(dead_code)]
pub struct SceneBackgroundEntity {
    pub id: EntityId,
    pub data: SceneBackgroundEntityData,
}

/// An entity of the [`SceneBackgroundArchetype`].
#[derive(Debug, Clone)]
#[allow(dead_code)]
pub struct SceneBackgroundEntityData {
    pub background_color: BackgroundColorComponent,
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
        1
    }

    /// Dynamically determines whether this archetype has a specific component.
    pub const fn has_components<Id>(&self, id: ComponentId) -> bool {
        let id = id.as_u64();
        id == 7 || false
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
    const ID: ArchetypeId = ArchetypeId(core::num::NonZeroU64::new(6).expect("A zero archetype ID was provided"));

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