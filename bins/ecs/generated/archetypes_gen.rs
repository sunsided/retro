/// The ID of an entity.
#[derive(Debug, Copy, Clone, Ord, PartialOrd, Eq, PartialEq)]
pub struct EntityId(std::num::NonZeroU64);

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
        static ENTITY_IDS: std::sync::atomic::AtomicU64 = std::sync::atomic::AtomicU64::new(1);
        let id = ENTITY_IDS.fetch_add(1, std::sync::atomic::Ordering::SeqCst);
        EntityId(std::num::NonZeroU64::new(id).expect("ID was zero"))
    }

    /// Returns this ID as a [`NonZeroU64`](std::num::NonZeroU64) value.
    pub const fn as_nonzero_u64(&self) -> std::num::NonZeroU64 {
        self.0
    }

    /// Returns this ID as a `u64` value.
    pub const fn as_u64(&self) -> u64 {
            self.0.get()
    }
}

impl std::hash::Hash for EntityId {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.0.hash(state);
    }
}

impl From<EntityId> for std::num::NonZeroU64 {
    fn from(value: EntityId) -> std::num::NonZeroU64 {
        value.as_nonzero_u64()
    }
}

impl From<EntityId> for u64 {
    fn from(value: EntityId) -> u64 {
        value.as_u64()
    }
}

/// The ID of an [`Archetype`].
#[derive(Debug, Copy, Clone, Ord, PartialOrd, Eq, PartialEq)]
pub struct ArchetypeId(std::num::NonZeroU64);

#[allow(dead_code)]
impl ArchetypeId {
    /// Returns this ID as a [`NonZeroU64`](std::num::NonZeroU64) value.
    pub const fn as_nonzero_u64(&self) -> std::num::NonZeroU64 {
        self.0
    }

    /// Returns this ID as a `u64` value.
    pub const fn as_u64(&self) -> u64 {
            self.0.get()
    }
}

impl std::hash::Hash for ArchetypeId {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.0.hash(state);
    }
}

impl From<ArchetypeId> for std::num::NonZeroU64 {
    fn from(value: ArchetypeId) -> std::num::NonZeroU64 {
        value.as_nonzero_u64()
    }
}

impl From<ArchetypeId> for u64 {
    fn from(value: ArchetypeId) -> u64 {
        value.as_u64()
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
pub trait HasComponent<T: Component> {
    /// Returns a slice of all component values.
    fn get(&self) -> &[T];

    /// Returns a mutable slice of all component values.
    fn get_mut(&mut self) -> &mut [T];
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
    pub position: PositionData,
    pub velocity: VelocityData,
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

impl std::ops::Deref for ParticleEntity {
    type Target = ParticleEntityData;

    fn deref(&self) -> &Self::Target {
        &self.data
    }
}

impl std::ops::DerefMut for ParticleEntity {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.data
    }
}

#[allow(dead_code)]
impl ParticleArchetype {
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

    /// Spawn a new entity into the world.
    pub fn spawn_with(
        &mut self,
        position: PositionData,
        velocity: VelocityData,
    ) -> (EntityId, EntityArchetypeRef) {
        self.positions.push(PositionComponent::new(position));
        self.velocities.push(VelocityComponent::new(velocity));

        let entity_id = EntityId::new();

        let entity_index = self.entities.len();
        self.entities.push(entity_id);

        let reference = EntityArchetypeRef {
            archetype: Self::ID,
            index: entity_index
        };
        (entity_id, reference)
    }
}


#[automatically_derived]
impl HasComponent<PositionComponent> for ParticleArchetype {
    /// Returns a slice of all [`PositionComponent`] component values.
    fn get(&self) -> &[PositionComponent] {
        &self.positions
    }

    /// Returns a mutable slice of all [`PositionComponent`] component values.
    fn get_mut(&mut self) -> &mut [PositionComponent] {
        &mut self.positions
    }
}

#[automatically_derived]
impl HasComponent<VelocityComponent> for ParticleArchetype {
    /// Returns a slice of all [`VelocityComponent`] component values.
    fn get(&self) -> &[VelocityComponent] {
        &self.velocities
    }

    /// Returns a mutable slice of all [`VelocityComponent`] component values.
    fn get_mut(&mut self) -> &mut [VelocityComponent] {
        &mut self.velocities
    }
}


#[automatically_derived]
#[allow(dead_code)]
impl Archetype for ParticleArchetype {
    const ID: ArchetypeId = ArchetypeId(std::num::NonZeroU64::new(1).expect("A zero archetype ID was provided"));

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
    pub position: PositionData,
    pub velocity: VelocityData,
    pub health: HealthData,
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

impl std::ops::Deref for PlayerEntity {
    type Target = PlayerEntityData;

    fn deref(&self) -> &Self::Target {
        &self.data
    }
}

impl std::ops::DerefMut for PlayerEntity {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.data
    }
}

#[allow(dead_code)]
impl PlayerArchetype {
    /// The IDs of the components used by this archetype, in ascending order.
    pub const COMPONENT_IDS: [ComponentId; 3] = [
        ComponentId::new(1).expect("Invalid ID on ECS construction time"),
        ComponentId::new(2).expect("Invalid ID on ECS construction time"),
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
        3
    }

    /// Dynamically determines whether this archetype has a specific component.
    pub const fn has_components<Id>(&self, id: ComponentId) -> bool {
        let id = id.as_u64();
        id == 1 || id == 2 || id == 3 || false
    }

    /// Spawn a new entity into the world.
    pub fn spawn_with(
        &mut self,
        position: PositionData,
        velocity: VelocityData,
        health: HealthData,
    ) -> (EntityId, EntityArchetypeRef) {
        self.positions.push(PositionComponent::new(position));
        self.velocities.push(VelocityComponent::new(velocity));
        self.healths.push(HealthComponent::new(health));

        let entity_id = EntityId::new();

        let entity_index = self.entities.len();
        self.entities.push(entity_id);

        let reference = EntityArchetypeRef {
            archetype: Self::ID,
            index: entity_index
        };
        (entity_id, reference)
    }
}


#[automatically_derived]
impl HasComponent<PositionComponent> for PlayerArchetype {
    /// Returns a slice of all [`PositionComponent`] component values.
    fn get(&self) -> &[PositionComponent] {
        &self.positions
    }

    /// Returns a mutable slice of all [`PositionComponent`] component values.
    fn get_mut(&mut self) -> &mut [PositionComponent] {
        &mut self.positions
    }
}

#[automatically_derived]
impl HasComponent<VelocityComponent> for PlayerArchetype {
    /// Returns a slice of all [`VelocityComponent`] component values.
    fn get(&self) -> &[VelocityComponent] {
        &self.velocities
    }

    /// Returns a mutable slice of all [`VelocityComponent`] component values.
    fn get_mut(&mut self) -> &mut [VelocityComponent] {
        &mut self.velocities
    }
}

#[automatically_derived]
impl HasComponent<HealthComponent> for PlayerArchetype {
    /// Returns a slice of all [`HealthComponent`] component values.
    fn get(&self) -> &[HealthComponent] {
        &self.healths
    }

    /// Returns a mutable slice of all [`HealthComponent`] component values.
    fn get_mut(&mut self) -> &mut [HealthComponent] {
        &mut self.healths
    }
}


#[automatically_derived]
#[allow(dead_code)]
impl Archetype for PlayerArchetype {
    const ID: ArchetypeId = ArchetypeId(std::num::NonZeroU64::new(2).expect("A zero archetype ID was provided"));

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
    pub position: PositionData,
    pub collider: ColliderData,
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

impl std::ops::Deref for ForegroundObjectEntity {
    type Target = ForegroundObjectEntityData;

    fn deref(&self) -> &Self::Target {
        &self.data
    }
}

impl std::ops::DerefMut for ForegroundObjectEntity {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.data
    }
}

#[allow(dead_code)]
impl ForegroundObjectArchetype {
    /// The IDs of the components used by this archetype, in ascending order.
    pub const COMPONENT_IDS: [ComponentId; 2] = [
        ComponentId::new(1).expect("Invalid ID on ECS construction time"),
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
        id == 1 || id == 4 || false
    }

    /// Spawn a new entity into the world.
    pub fn spawn_with(
        &mut self,
        position: PositionData,
        collider: ColliderData,
    ) -> (EntityId, EntityArchetypeRef) {
        self.positions.push(PositionComponent::new(position));
        self.colliders.push(ColliderComponent::new(collider));

        let entity_id = EntityId::new();

        let entity_index = self.entities.len();
        self.entities.push(entity_id);

        let reference = EntityArchetypeRef {
            archetype: Self::ID,
            index: entity_index
        };
        (entity_id, reference)
    }

    /// Promotes this [`ForegroundObjectArchetype`] to [`BackgroundObjectArchetype`].
    pub fn promote_to_background_objects(
        self,
    ) -> BackgroundObjectArchetype {
        // TODO: Move dropped arrays to a pool? Create a trait with "recycle_<component vector>".
        BackgroundObjectArchetype {
            entities: self.entities,
            positions: self.positions,
        }
    }
}


#[automatically_derived]
impl HasComponent<PositionComponent> for ForegroundObjectArchetype {
    /// Returns a slice of all [`PositionComponent`] component values.
    fn get(&self) -> &[PositionComponent] {
        &self.positions
    }

    /// Returns a mutable slice of all [`PositionComponent`] component values.
    fn get_mut(&mut self) -> &mut [PositionComponent] {
        &mut self.positions
    }
}

#[automatically_derived]
impl HasComponent<ColliderComponent> for ForegroundObjectArchetype {
    /// Returns a slice of all [`ColliderComponent`] component values.
    fn get(&self) -> &[ColliderComponent] {
        &self.colliders
    }

    /// Returns a mutable slice of all [`ColliderComponent`] component values.
    fn get_mut(&mut self) -> &mut [ColliderComponent] {
        &mut self.colliders
    }
}


#[automatically_derived]
#[allow(dead_code)]
impl Archetype for ForegroundObjectArchetype {
    const ID: ArchetypeId = ArchetypeId(std::num::NonZeroU64::new(3).expect("A zero archetype ID was provided"));

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
    pub position: PositionData,
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

impl std::ops::Deref for BackgroundObjectEntity {
    type Target = BackgroundObjectEntityData;

    fn deref(&self) -> &Self::Target {
        &self.data
    }
}

impl std::ops::DerefMut for BackgroundObjectEntity {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.data
    }
}

#[allow(dead_code)]
impl BackgroundObjectArchetype {
    /// The IDs of the components used by this archetype, in ascending order.
    pub const COMPONENT_IDS: [ComponentId; 1] = [
        ComponentId::new(1).expect("Invalid ID on ECS construction time"),
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
        id == 1 || false
    }

    /// Spawn a new entity into the world.
    pub fn spawn_with(
        &mut self,
        position: PositionData,
    ) -> (EntityId, EntityArchetypeRef) {
        self.positions.push(PositionComponent::new(position));

        let entity_id = EntityId::new();

        let entity_index = self.entities.len();
        self.entities.push(entity_id);

        let reference = EntityArchetypeRef {
            archetype: Self::ID,
            index: entity_index
        };
        (entity_id, reference)
    }

    /// Promotes this [`BackgroundObjectArchetype`] to [`ForegroundObjectArchetype`].
    pub fn promote_to_foreground_objects(
        self,
        colliders: Vec<ColliderComponent>,
    ) -> ForegroundObjectArchetype {
        // TODO: Move dropped arrays to a pool? Create a trait with "recycle_<component vector>".
        ForegroundObjectArchetype {
            entities: self.entities,
            colliders,
            positions: self.positions,
        }
    }
}


#[automatically_derived]
impl HasComponent<PositionComponent> for BackgroundObjectArchetype {
    /// Returns a slice of all [`PositionComponent`] component values.
    fn get(&self) -> &[PositionComponent] {
        &self.positions
    }

    /// Returns a mutable slice of all [`PositionComponent`] component values.
    fn get_mut(&mut self) -> &mut [PositionComponent] {
        &mut self.positions
    }
}


#[automatically_derived]
#[allow(dead_code)]
impl Archetype for BackgroundObjectArchetype {
    const ID: ArchetypeId = ArchetypeId(std::num::NonZeroU64::new(4).expect("A zero archetype ID was provided"));

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