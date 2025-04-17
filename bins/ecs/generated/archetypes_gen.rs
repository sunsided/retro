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
#[repr(u32)]
pub enum ArchetypeId {
    /// The [`WgpuShader`](WgpuShaderArchetype) archetype.
    WgpuShader = 1,
    /// The [`WgpuTexture`](WgpuTextureArchetype) archetype.
    WgpuTexture = 2,
    /// The [`Environment`](EnvironmentArchetype) archetype.
    Environment = 3,
    /// The [`Camera`](CameraArchetype) archetype.
    Camera = 4,
    /// The [`DynamicLight`](DynamicLightArchetype) archetype.
    DynamicLight = 5,
    /// The [`Tile`](TileArchetype) archetype.
    Tile = 6,
    /// A marching cube :)
    ///
    /// For details, see the [`CubeArchetype`](CubeArchetype) Struct.
    Cube = 7,
    /// The [`Player`](PlayerArchetype) archetype.
    Player = 8,
    /// The [`ForegroundObject`](ForegroundObjectArchetype) archetype.
    ForegroundObject = 9,
    /// The [`BackgroundObject`](BackgroundObjectArchetype) archetype.
    BackgroundObject = 10,
    /// The scene background.
    ///
    /// For details, see the [`SceneBackgroundArchetype`](SceneBackgroundArchetype) Struct.
    SceneBackground = 11,
}

impl ArchetypeId {
    /// A [`NonZeroU64`](core::num::NonZeroU64) representation of [`ArchetypeId::WgpuShader`](ArchetypeId::WgpuShader) archetype.
    pub const WGPU_SHADER_ID_VALUE: core::num::NonZeroU64 = core::num::NonZeroU64::new(1).expect("Error at ECS construction time");
    /// A [`NonZeroU64`](core::num::NonZeroU64) representation of [`ArchetypeId::WgpuTexture`](ArchetypeId::WgpuTexture) archetype.
    pub const WGPU_TEXTURE_ID_VALUE: core::num::NonZeroU64 = core::num::NonZeroU64::new(2).expect("Error at ECS construction time");
    /// A [`NonZeroU64`](core::num::NonZeroU64) representation of [`ArchetypeId::Environment`](ArchetypeId::Environment) archetype.
    pub const ENVIRONMENT_ID_VALUE: core::num::NonZeroU64 = core::num::NonZeroU64::new(3).expect("Error at ECS construction time");
    /// A [`NonZeroU64`](core::num::NonZeroU64) representation of [`ArchetypeId::Camera`](ArchetypeId::Camera) archetype.
    pub const CAMERA_ID_VALUE: core::num::NonZeroU64 = core::num::NonZeroU64::new(4).expect("Error at ECS construction time");
    /// A [`NonZeroU64`](core::num::NonZeroU64) representation of [`ArchetypeId::DynamicLight`](ArchetypeId::DynamicLight) archetype.
    pub const DYNAMIC_LIGHT_ID_VALUE: core::num::NonZeroU64 = core::num::NonZeroU64::new(5).expect("Error at ECS construction time");
    /// A [`NonZeroU64`](core::num::NonZeroU64) representation of [`ArchetypeId::Tile`](ArchetypeId::Tile) archetype.
    pub const TILE_ID_VALUE: core::num::NonZeroU64 = core::num::NonZeroU64::new(6).expect("Error at ECS construction time");
    /// A [`NonZeroU64`](core::num::NonZeroU64) representation of [`ArchetypeId::Cube`](ArchetypeId::Cube) archetype.
    pub const CUBE_ID_VALUE: core::num::NonZeroU64 = core::num::NonZeroU64::new(7).expect("Error at ECS construction time");
    /// A [`NonZeroU64`](core::num::NonZeroU64) representation of [`ArchetypeId::Player`](ArchetypeId::Player) archetype.
    pub const PLAYER_ID_VALUE: core::num::NonZeroU64 = core::num::NonZeroU64::new(8).expect("Error at ECS construction time");
    /// A [`NonZeroU64`](core::num::NonZeroU64) representation of [`ArchetypeId::ForegroundObject`](ArchetypeId::ForegroundObject) archetype.
    pub const FOREGROUND_OBJECT_ID_VALUE: core::num::NonZeroU64 = core::num::NonZeroU64::new(9).expect("Error at ECS construction time");
    /// A [`NonZeroU64`](core::num::NonZeroU64) representation of [`ArchetypeId::BackgroundObject`](ArchetypeId::BackgroundObject) archetype.
    pub const BACKGROUND_OBJECT_ID_VALUE: core::num::NonZeroU64 = core::num::NonZeroU64::new(10).expect("Error at ECS construction time");
    /// A [`NonZeroU64`](core::num::NonZeroU64) representation of [`ArchetypeId::SceneBackground`](ArchetypeId::SceneBackground) archetype.
    pub const SCENE_BACKGROUND_ID_VALUE: core::num::NonZeroU64 = core::num::NonZeroU64::new(11).expect("Error at ECS construction time");
}

#[allow(dead_code)]
impl ArchetypeId {
    /// Returns this ID as a [`NonZeroU64`](core::num::NonZeroU64) value.
    pub const fn as_nonzero_u64(&self) -> core::num::NonZeroU64 {
        match self {
            Self::WgpuShader => Self::WGPU_SHADER_ID_VALUE,
            Self::WgpuTexture => Self::WGPU_TEXTURE_ID_VALUE,
            Self::Environment => Self::ENVIRONMENT_ID_VALUE,
            Self::Camera => Self::CAMERA_ID_VALUE,
            Self::DynamicLight => Self::DYNAMIC_LIGHT_ID_VALUE,
            Self::Tile => Self::TILE_ID_VALUE,
            Self::Cube => Self::CUBE_ID_VALUE,
            Self::Player => Self::PLAYER_ID_VALUE,
            Self::ForegroundObject => Self::FOREGROUND_OBJECT_ID_VALUE,
            Self::BackgroundObject => Self::BACKGROUND_OBJECT_ID_VALUE,
            Self::SceneBackground => Self::SCENE_BACKGROUND_ID_VALUE,
        }
    }

    /// Returns this ID as a `u64` value.
    pub const fn as_u64(&self) -> u64 {
        self.as_nonzero_u64().get()
    }
}

impl core::hash::Hash for ArchetypeId {
    fn hash<H: core::hash::Hasher>(&self, state: &mut H) {
        self.as_u64().hash(state);
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
        match self {
            Self::WgpuShader => write!(f, "WgpuShader (ID 1)"),
            Self::WgpuTexture => write!(f, "WgpuTexture (ID 2)"),
            Self::Environment => write!(f, "Environment (ID 3)"),
            Self::Camera => write!(f, "Camera (ID 4)"),
            Self::DynamicLight => write!(f, "DynamicLight (ID 5)"),
            Self::Tile => write!(f, "Tile (ID 6)"),
            Self::Cube => write!(f, "Cube (ID 7)"),
            Self::Player => write!(f, "Player (ID 8)"),
            Self::ForegroundObject => write!(f, "ForegroundObject (ID 9)"),
            Self::BackgroundObject => write!(f, "BackgroundObject (ID 10)"),
            Self::SceneBackground => write!(f, "SceneBackground (ID 11)"),
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
    #[allow(dead_code)]
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
    /// An entity in the [`WgpuShader`](WgpuShaderArchetype) archetype.
    WgpuShader(WgpuShaderEntity),
    /// An entity in the [`WgpuTexture`](WgpuTextureArchetype) archetype.
    WgpuTexture(WgpuTextureEntity),
    /// An entity in the [`Environment`](EnvironmentArchetype) archetype.
    Environment(EnvironmentEntity),
    /// An entity in the [`Camera`](CameraArchetype) archetype.
    Camera(CameraEntity),
    /// An entity in the [`DynamicLight`](DynamicLightArchetype) archetype.
    DynamicLight(DynamicLightEntity),
    /// An entity in the [`Tile`](TileArchetype) archetype.
    Tile(TileEntity),
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
    /// An entity in the [`WgpuShader`](WgpuShaderArchetype) archetype.
    WgpuShader(WgpuShaderEntityData),
    /// An entity in the [`WgpuTexture`](WgpuTextureArchetype) archetype.
    WgpuTexture(WgpuTextureEntityData),
    /// An entity in the [`Environment`](EnvironmentArchetype) archetype.
    Environment(EnvironmentEntityData),
    /// An entity in the [`Camera`](CameraArchetype) archetype.
    Camera(CameraEntityData),
    /// An entity in the [`DynamicLight`](DynamicLightArchetype) archetype.
    DynamicLight(DynamicLightEntityData),
    /// An entity in the [`Tile`](TileArchetype) archetype.
    Tile(TileEntityData),
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
            Self::WgpuShader(_) => WgpuShaderArchetype::ID,
            Self::WgpuTexture(_) => WgpuTextureArchetype::ID,
            Self::Environment(_) => EnvironmentArchetype::ID,
            Self::Camera(_) => CameraArchetype::ID,
            Self::DynamicLight(_) => DynamicLightArchetype::ID,
            Self::Tile(_) => TileArchetype::ID,
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
            Self::WgpuShader(_) => WgpuShaderArchetype::ID,
            Self::WgpuTexture(_) => WgpuTextureArchetype::ID,
            Self::Environment(_) => EnvironmentArchetype::ID,
            Self::Camera(_) => CameraArchetype::ID,
            Self::DynamicLight(_) => DynamicLightArchetype::ID,
            Self::Tile(_) => TileArchetype::ID,
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
pub struct WgpuShaderArchetype {
    pub entities: Vec<EntityId>,
    pub wgpu_shaders: Vec<WgpuShaderComponent>,
}

/// An entity of the [`WgpuShaderArchetype`].
pub type WgpuShaderEntity = EntityWithIdAndData<WgpuShaderEntityComponents>;

/// The data of an entity of the [`WgpuShaderArchetype`].
#[derive(Debug, Clone)]
#[allow(dead_code)]
pub struct WgpuShaderEntityData {
    pub wgpu_shader: WgpuShaderData,
}

/// An entity of the [`WgpuShaderArchetype`].
#[derive(Debug, Clone)]
#[allow(dead_code)]
pub struct WgpuShaderEntityComponents {
    pub wgpu_shader: WgpuShaderComponent,
}

impl From<WgpuShaderEntityData> for WgpuShaderEntityComponents {
    fn from(value: WgpuShaderEntityData) -> Self {
        Self {
            wgpu_shader: value.wgpu_shader.into(),
        }
    }
}

impl EntityData for WgpuShaderEntityData {
    const ARCHETYPE_ID: ArchetypeId = WgpuShaderArchetype::ID;
}

impl EntityData for WgpuShaderEntityComponents {
    const ARCHETYPE_ID: ArchetypeId = WgpuShaderArchetype::ID;
}

/// A reference to an entity of the [`WgpuShaderArchetype`].
#[derive(Debug, Clone)]
#[allow(dead_code)]
pub struct WgpuShaderEntityRef<'archetype> {
    pub entity_id: EntityId,
    pub wgpu_shader: &'archetype WgpuShaderComponent,
}

/// A mutable reference to an entity of the [`WgpuShaderArchetype`].
#[derive(Debug)]
#[allow(dead_code)]
pub struct WgpuShaderEntityMut<'archetype> {
    pub entity_id: EntityId,
    pub wgpu_shader: &'archetype mut WgpuShaderComponent,
}

/// An iterator for entities in the [`WgpuShaderArchetype`].
pub struct WgpuShaderEntityIterator<'archetype> {
    archetype: &'archetype WgpuShaderArchetype,
    current: usize,
    length: usize,
}

impl<'archetype> WgpuShaderEntityIterator<'archetype> {
    fn new(archetype: &'archetype WgpuShaderArchetype) -> Self {
        Self {
            archetype,
            current: 0,
            length: archetype.len()
        }
    }
}

impl<'archetype> Iterator for WgpuShaderEntityIterator<'archetype> {
    type Item = WgpuShaderEntityRef<'archetype>;

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

impl WgpuShaderArchetype {
    /// Iterates all entities in this archetype.
    #[inline]
    #[allow(dead_code)]
    pub fn iter(&self) -> WgpuShaderEntityIterator {
        WgpuShaderEntityIterator::new(self)
    }

    /// Gets the `WgpuShader` component at the specified index.
    #[allow(dead_code)]
    #[inline]
    pub fn get_wgpu_shader_component_at(&self, index: usize) -> Option<&WgpuShaderComponent> {
        if index > self.len() {
            return None;
        }
        Some(unsafe { self.get_wgpu_shader_component_at_unchecked(index) })
    }

    /// Mutably gets the `WgpuShader` component at the specified index.
    #[allow(dead_code)]
    #[inline]
    pub fn get_wgpu_shader_component_at_mut(&mut self, index: usize) -> Option<&mut WgpuShaderComponent> {
        if index > self.len() {
            return None;
        }
        Some(unsafe { self.get_wgpu_shader_component_at_unchecked_mut(index) })
    }

    /// Gets the `WgpuShader` component at the specified index.
    #[allow(dead_code)]
    #[inline]
    pub unsafe fn get_wgpu_shader_component_at_unchecked(&self, index: usize) -> &WgpuShaderComponent {
        self.wgpu_shaders.get_unchecked(index)
    }

    /// Gets the `WgpuShader` component at the specified index.
    #[allow(dead_code)]
    #[inline]
    pub unsafe fn get_wgpu_shader_component_at_unchecked_mut(&mut self, index: usize) -> &mut WgpuShaderComponent {
        self.wgpu_shaders.get_unchecked_mut(index)
    }

    /// Gets the entity at the specified index.
    #[allow(dead_code)]
    pub fn get_entity_at(&self, index: usize) -> Option<WgpuShaderEntityRef> {
        if index > self.len() {
            return None;
        }
        Some(unsafe { self.get_entity_at_unchecked(index) })
    }

    /// Mutably gets the entity at the specified index.
    #[allow(dead_code)]
    pub fn get_entity_at_mut(&mut self, index: usize) -> Option<WgpuShaderEntityMut> {
        if index > self.len() {
            return None;
        }
        Some(unsafe { self.get_entity_at_unchecked_mut(index) })
    }

    /// Gets the entity at the specified index.
    #[allow(dead_code)]
    pub unsafe fn get_entity_at_unchecked(&self, index: usize) -> WgpuShaderEntityRef {
        WgpuShaderEntityRef {
            entity_id: *self.entities.get_unchecked(index),
            wgpu_shader: self.wgpu_shaders.get_unchecked(index),
        }
    }

    /// Gets the entity at the specified index.
    #[allow(dead_code)]
    pub unsafe fn get_entity_at_unchecked_mut(&mut self, index: usize) -> WgpuShaderEntityMut {
        WgpuShaderEntityMut {
            entity_id: *self.entities.get_unchecked(index),
            wgpu_shader: self.wgpu_shaders.get_unchecked_mut(index),
        }
    }
}

#[allow(dead_code)]
impl HasComponent<WgpuShaderComponent> for WgpuShaderEntity {
    #[inline]
    fn get(&self) -> &WgpuShaderComponent {
        &self.data.wgpu_shader
    }

    #[inline]
    fn get_mut(&mut self) -> &mut WgpuShaderComponent {
        &mut self.data.wgpu_shader
    }
}

#[allow(dead_code)]
impl HasComponent<WgpuShaderComponent> for WgpuShaderEntityComponents {
    #[inline]
    fn get(&self) -> &WgpuShaderComponent {
        &self.wgpu_shader
    }

    #[inline]
    fn get_mut(&mut self) -> &mut WgpuShaderComponent {
        &mut self.wgpu_shader
    }
}
impl WgpuShaderEntityData {
    /// Spawn this entity into the given world.
    #[inline]
    #[allow(dead_code)]
    pub fn spawn_into<W>(self, world: &mut W) -> EntityId
    where
        W: Spawn<WgpuShaderEntityData>
    {
        world.spawn(self)
    }
}

impl WgpuShaderEntityComponents {
    /// Spawn this entity into the given world.
    #[inline]
    #[allow(dead_code)]
    pub fn spawn_into<W>(self, world: &mut W) -> EntityId
    where
        W: Spawn<WgpuShaderEntityComponents>
    {
        world.spawn(self)
    }
}

impl core::ops::Deref for WgpuShaderEntity {
    type Target = WgpuShaderEntityComponents;

    fn deref(&self) -> &Self::Target {
        &self.data
    }
}

impl core::ops::DerefMut for WgpuShaderEntity {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.data
    }
}

#[allow(dead_code)]
impl WgpuShaderArchetype {
    /// The IDs of the components used by this archetype, in ascending order.
    pub const COMPONENTS: [ComponentId; 1] = [
        ComponentId::COMPONENT_19,
    ];

    /// Returns the number of entities in this archetype.
    #[inline(always)]
    pub fn len(&self) -> usize {
        self.entities.len()
    }

    /// Indicates whether this archetype has no entities.
    #[inline(always)]
    #[allow(dead_code)]
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
        id == 19 || false
    }

    /// Spawn a new entity into the world. Must be called through the world for proper bookkeeping.
    #[doc(hidden)]
    pub fn spawn_with<R>(
        &mut self,
        wgpu_shader: WgpuShaderComponent,
        mut world_registry: R
    ) -> EntityId
    where
        R: WorldEntityRegistry
    {
        self.wgpu_shaders.push(wgpu_shader);

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
        self.wgpu_shaders.swap_remove(index);
        if self.entities.is_empty() {
            Ok(None)
        }
        else {
            Ok(Some(self.entities[index]))
        }
    }
}


#[automatically_derived]
impl HasComponents<WgpuShaderComponent> for WgpuShaderArchetype {
    /// Returns a slice of all [`WgpuShaderComponent`] component values.
    fn get(&self) -> &[WgpuShaderComponent] {
        self.wgpu_shaders.as_slice()
    }

    /// Returns a mutable slice of all [`WgpuShaderComponent`] component values.
    fn get_mut(&mut self) -> &mut [WgpuShaderComponent] {
        self.wgpu_shaders.as_mut_slice()
    }
}


#[automatically_derived]
#[allow(dead_code)]
impl Archetype for WgpuShaderArchetype {
    const ID: ArchetypeId = ArchetypeId::WgpuShader;

    fn len(&self) -> usize {
        self.entities.len()
    }

    fn is_empty(&self) -> bool {
        self.entities.is_empty()
    }
}

impl PartialEq<ArchetypeId> for WgpuShaderArchetype {
    fn eq(&self, other: &ArchetypeId) -> bool {
        Self::ID.eq(other)
    }
}

/// An archetype grouping entities with identical components.
#[derive(Debug, Default, Clone)]
pub struct WgpuTextureArchetype {
    pub entities: Vec<EntityId>,
    pub texture_sources: Vec<TextureSourceComponent>,
    pub wgpu_textures: Vec<WgpuTextureComponent>,
}

/// An entity of the [`WgpuTextureArchetype`].
pub type WgpuTextureEntity = EntityWithIdAndData<WgpuTextureEntityComponents>;

/// The data of an entity of the [`WgpuTextureArchetype`].
#[derive(Debug, Clone)]
#[allow(dead_code)]
pub struct WgpuTextureEntityData {
    pub texture_source: TextureSourceData,
    pub wgpu_texture: WgpuTextureData,
}

/// An entity of the [`WgpuTextureArchetype`].
#[derive(Debug, Clone)]
#[allow(dead_code)]
pub struct WgpuTextureEntityComponents {
    pub texture_source: TextureSourceComponent,
    pub wgpu_texture: WgpuTextureComponent,
}

impl From<WgpuTextureEntityData> for WgpuTextureEntityComponents {
    fn from(value: WgpuTextureEntityData) -> Self {
        Self {
            texture_source: value.texture_source.into(),
            wgpu_texture: value.wgpu_texture.into(),
        }
    }
}

impl EntityData for WgpuTextureEntityData {
    const ARCHETYPE_ID: ArchetypeId = WgpuTextureArchetype::ID;
}

impl EntityData for WgpuTextureEntityComponents {
    const ARCHETYPE_ID: ArchetypeId = WgpuTextureArchetype::ID;
}

/// A reference to an entity of the [`WgpuTextureArchetype`].
#[derive(Debug, Clone)]
#[allow(dead_code)]
pub struct WgpuTextureEntityRef<'archetype> {
    pub entity_id: EntityId,
    pub texture_source: &'archetype TextureSourceComponent,
    pub wgpu_texture: &'archetype WgpuTextureComponent,
}

/// A mutable reference to an entity of the [`WgpuTextureArchetype`].
#[derive(Debug)]
#[allow(dead_code)]
pub struct WgpuTextureEntityMut<'archetype> {
    pub entity_id: EntityId,
    pub texture_source: &'archetype mut TextureSourceComponent,
    pub wgpu_texture: &'archetype mut WgpuTextureComponent,
}

/// An iterator for entities in the [`WgpuTextureArchetype`].
pub struct WgpuTextureEntityIterator<'archetype> {
    archetype: &'archetype WgpuTextureArchetype,
    current: usize,
    length: usize,
}

impl<'archetype> WgpuTextureEntityIterator<'archetype> {
    fn new(archetype: &'archetype WgpuTextureArchetype) -> Self {
        Self {
            archetype,
            current: 0,
            length: archetype.len()
        }
    }
}

impl<'archetype> Iterator for WgpuTextureEntityIterator<'archetype> {
    type Item = WgpuTextureEntityRef<'archetype>;

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

impl WgpuTextureArchetype {
    /// Iterates all entities in this archetype.
    #[inline]
    #[allow(dead_code)]
    pub fn iter(&self) -> WgpuTextureEntityIterator {
        WgpuTextureEntityIterator::new(self)
    }

    /// Gets the `TextureSource` component at the specified index.
    #[allow(dead_code)]
    #[inline]
    pub fn get_texture_source_component_at(&self, index: usize) -> Option<&TextureSourceComponent> {
        if index > self.len() {
            return None;
        }
        Some(unsafe { self.get_texture_source_component_at_unchecked(index) })
    }

    /// Mutably gets the `TextureSource` component at the specified index.
    #[allow(dead_code)]
    #[inline]
    pub fn get_texture_source_component_at_mut(&mut self, index: usize) -> Option<&mut TextureSourceComponent> {
        if index > self.len() {
            return None;
        }
        Some(unsafe { self.get_texture_source_component_at_unchecked_mut(index) })
    }

    /// Gets the `TextureSource` component at the specified index.
    #[allow(dead_code)]
    #[inline]
    pub unsafe fn get_texture_source_component_at_unchecked(&self, index: usize) -> &TextureSourceComponent {
        self.texture_sources.get_unchecked(index)
    }

    /// Gets the `TextureSource` component at the specified index.
    #[allow(dead_code)]
    #[inline]
    pub unsafe fn get_texture_source_component_at_unchecked_mut(&mut self, index: usize) -> &mut TextureSourceComponent {
        self.texture_sources.get_unchecked_mut(index)
    }

    /// Gets the `WgpuTexture` component at the specified index.
    #[allow(dead_code)]
    #[inline]
    pub fn get_wgpu_texture_component_at(&self, index: usize) -> Option<&WgpuTextureComponent> {
        if index > self.len() {
            return None;
        }
        Some(unsafe { self.get_wgpu_texture_component_at_unchecked(index) })
    }

    /// Mutably gets the `WgpuTexture` component at the specified index.
    #[allow(dead_code)]
    #[inline]
    pub fn get_wgpu_texture_component_at_mut(&mut self, index: usize) -> Option<&mut WgpuTextureComponent> {
        if index > self.len() {
            return None;
        }
        Some(unsafe { self.get_wgpu_texture_component_at_unchecked_mut(index) })
    }

    /// Gets the `WgpuTexture` component at the specified index.
    #[allow(dead_code)]
    #[inline]
    pub unsafe fn get_wgpu_texture_component_at_unchecked(&self, index: usize) -> &WgpuTextureComponent {
        self.wgpu_textures.get_unchecked(index)
    }

    /// Gets the `WgpuTexture` component at the specified index.
    #[allow(dead_code)]
    #[inline]
    pub unsafe fn get_wgpu_texture_component_at_unchecked_mut(&mut self, index: usize) -> &mut WgpuTextureComponent {
        self.wgpu_textures.get_unchecked_mut(index)
    }

    /// Gets the entity at the specified index.
    #[allow(dead_code)]
    pub fn get_entity_at(&self, index: usize) -> Option<WgpuTextureEntityRef> {
        if index > self.len() {
            return None;
        }
        Some(unsafe { self.get_entity_at_unchecked(index) })
    }

    /// Mutably gets the entity at the specified index.
    #[allow(dead_code)]
    pub fn get_entity_at_mut(&mut self, index: usize) -> Option<WgpuTextureEntityMut> {
        if index > self.len() {
            return None;
        }
        Some(unsafe { self.get_entity_at_unchecked_mut(index) })
    }

    /// Gets the entity at the specified index.
    #[allow(dead_code)]
    pub unsafe fn get_entity_at_unchecked(&self, index: usize) -> WgpuTextureEntityRef {
        WgpuTextureEntityRef {
            entity_id: *self.entities.get_unchecked(index),
            texture_source: self.texture_sources.get_unchecked(index),
            wgpu_texture: self.wgpu_textures.get_unchecked(index),
        }
    }

    /// Gets the entity at the specified index.
    #[allow(dead_code)]
    pub unsafe fn get_entity_at_unchecked_mut(&mut self, index: usize) -> WgpuTextureEntityMut {
        WgpuTextureEntityMut {
            entity_id: *self.entities.get_unchecked(index),
            texture_source: self.texture_sources.get_unchecked_mut(index),
            wgpu_texture: self.wgpu_textures.get_unchecked_mut(index),
        }
    }
}

#[allow(dead_code)]
impl HasComponent<TextureSourceComponent> for WgpuTextureEntity {
    #[inline]
    fn get(&self) -> &TextureSourceComponent {
        &self.data.texture_source
    }

    #[inline]
    fn get_mut(&mut self) -> &mut TextureSourceComponent {
        &mut self.data.texture_source
    }
}

#[allow(dead_code)]
impl HasComponent<TextureSourceComponent> for WgpuTextureEntityComponents {
    #[inline]
    fn get(&self) -> &TextureSourceComponent {
        &self.texture_source
    }

    #[inline]
    fn get_mut(&mut self) -> &mut TextureSourceComponent {
        &mut self.texture_source
    }
}

#[allow(dead_code)]
impl HasComponent<WgpuTextureComponent> for WgpuTextureEntity {
    #[inline]
    fn get(&self) -> &WgpuTextureComponent {
        &self.data.wgpu_texture
    }

    #[inline]
    fn get_mut(&mut self) -> &mut WgpuTextureComponent {
        &mut self.data.wgpu_texture
    }
}

#[allow(dead_code)]
impl HasComponent<WgpuTextureComponent> for WgpuTextureEntityComponents {
    #[inline]
    fn get(&self) -> &WgpuTextureComponent {
        &self.wgpu_texture
    }

    #[inline]
    fn get_mut(&mut self) -> &mut WgpuTextureComponent {
        &mut self.wgpu_texture
    }
}
impl WgpuTextureEntityData {
    /// Spawn this entity into the given world.
    #[inline]
    #[allow(dead_code)]
    pub fn spawn_into<W>(self, world: &mut W) -> EntityId
    where
        W: Spawn<WgpuTextureEntityData>
    {
        world.spawn(self)
    }
}

impl WgpuTextureEntityComponents {
    /// Spawn this entity into the given world.
    #[inline]
    #[allow(dead_code)]
    pub fn spawn_into<W>(self, world: &mut W) -> EntityId
    where
        W: Spawn<WgpuTextureEntityComponents>
    {
        world.spawn(self)
    }
}

impl core::ops::Deref for WgpuTextureEntity {
    type Target = WgpuTextureEntityComponents;

    fn deref(&self) -> &Self::Target {
        &self.data
    }
}

impl core::ops::DerefMut for WgpuTextureEntity {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.data
    }
}

#[allow(dead_code)]
impl WgpuTextureArchetype {
    /// The IDs of the components used by this archetype, in ascending order.
    pub const COMPONENTS: [ComponentId; 2] = [
        ComponentId::COMPONENT_20,
        ComponentId::COMPONENT_21,
    ];

    /// Returns the number of entities in this archetype.
    #[inline(always)]
    pub fn len(&self) -> usize {
        self.entities.len()
    }

    /// Indicates whether this archetype has no entities.
    #[inline(always)]
    #[allow(dead_code)]
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
        id == 20 || id == 21 || false
    }

    /// Spawn a new entity into the world. Must be called through the world for proper bookkeeping.
    #[doc(hidden)]
    pub fn spawn_with<R>(
        &mut self,
        texture_source: TextureSourceComponent,
        wgpu_texture: WgpuTextureComponent,
        mut world_registry: R
    ) -> EntityId
    where
        R: WorldEntityRegistry
    {
        self.texture_sources.push(texture_source);
        self.wgpu_textures.push(wgpu_texture);

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
        self.texture_sources.swap_remove(index);
        self.wgpu_textures.swap_remove(index);
        if self.entities.is_empty() {
            Ok(None)
        }
        else {
            Ok(Some(self.entities[index]))
        }
    }
}


#[automatically_derived]
impl HasComponents<TextureSourceComponent> for WgpuTextureArchetype {
    /// Returns a slice of all [`TextureSourceComponent`] component values.
    fn get(&self) -> &[TextureSourceComponent] {
        self.texture_sources.as_slice()
    }

    /// Returns a mutable slice of all [`TextureSourceComponent`] component values.
    fn get_mut(&mut self) -> &mut [TextureSourceComponent] {
        self.texture_sources.as_mut_slice()
    }
}

#[automatically_derived]
impl HasComponents<WgpuTextureComponent> for WgpuTextureArchetype {
    /// Returns a slice of all [`WgpuTextureComponent`] component values.
    fn get(&self) -> &[WgpuTextureComponent] {
        self.wgpu_textures.as_slice()
    }

    /// Returns a mutable slice of all [`WgpuTextureComponent`] component values.
    fn get_mut(&mut self) -> &mut [WgpuTextureComponent] {
        self.wgpu_textures.as_mut_slice()
    }
}


#[automatically_derived]
#[allow(dead_code)]
impl Archetype for WgpuTextureArchetype {
    const ID: ArchetypeId = ArchetypeId::WgpuTexture;

    fn len(&self) -> usize {
        self.entities.len()
    }

    fn is_empty(&self) -> bool {
        self.entities.is_empty()
    }
}

impl PartialEq<ArchetypeId> for WgpuTextureArchetype {
    fn eq(&self, other: &ArchetypeId) -> bool {
        Self::ID.eq(other)
    }
}

/// An archetype grouping entities with identical components.
#[derive(Debug, Default, Clone)]
pub struct EnvironmentArchetype {
    pub entities: Vec<EntityId>,
    pub time_of_days: Vec<TimeOfDayComponent>,
    pub suns: Vec<SunComponent>,
    pub skydomes: Vec<SkydomeComponent>,
    pub ambient_lights: Vec<AmbientLightComponent>,
    pub stars: Vec<StarsComponent>,
    pub fogs: Vec<FogComponent>,
}

/// An entity of the [`EnvironmentArchetype`].
pub type EnvironmentEntity = EntityWithIdAndData<EnvironmentEntityComponents>;

/// The data of an entity of the [`EnvironmentArchetype`].
#[derive(Debug, Clone)]
#[allow(dead_code)]
pub struct EnvironmentEntityData {
    pub time_of_day: TimeOfDayData,
    pub sun: SunData,
    pub skydome: SkydomeData,
    pub ambient_light: AmbientLightData,
    pub stars: StarsData,
    pub fog: FogData,
}

/// An entity of the [`EnvironmentArchetype`].
#[derive(Debug, Clone)]
#[allow(dead_code)]
pub struct EnvironmentEntityComponents {
    pub time_of_day: TimeOfDayComponent,
    pub sun: SunComponent,
    pub skydome: SkydomeComponent,
    pub ambient_light: AmbientLightComponent,
    pub stars: StarsComponent,
    pub fog: FogComponent,
}

impl From<EnvironmentEntityData> for EnvironmentEntityComponents {
    fn from(value: EnvironmentEntityData) -> Self {
        Self {
            time_of_day: value.time_of_day.into(),
            sun: value.sun.into(),
            skydome: value.skydome.into(),
            ambient_light: value.ambient_light.into(),
            stars: value.stars.into(),
            fog: value.fog.into(),
        }
    }
}

impl EntityData for EnvironmentEntityData {
    const ARCHETYPE_ID: ArchetypeId = EnvironmentArchetype::ID;
}

impl EntityData for EnvironmentEntityComponents {
    const ARCHETYPE_ID: ArchetypeId = EnvironmentArchetype::ID;
}

/// A reference to an entity of the [`EnvironmentArchetype`].
#[derive(Debug, Clone)]
#[allow(dead_code)]
pub struct EnvironmentEntityRef<'archetype> {
    pub entity_id: EntityId,
    pub time_of_day: &'archetype TimeOfDayComponent,
    pub sun: &'archetype SunComponent,
    pub skydome: &'archetype SkydomeComponent,
    pub ambient_light: &'archetype AmbientLightComponent,
    pub stars: &'archetype StarsComponent,
    pub fog: &'archetype FogComponent,
}

/// A mutable reference to an entity of the [`EnvironmentArchetype`].
#[derive(Debug)]
#[allow(dead_code)]
pub struct EnvironmentEntityMut<'archetype> {
    pub entity_id: EntityId,
    pub time_of_day: &'archetype mut TimeOfDayComponent,
    pub sun: &'archetype mut SunComponent,
    pub skydome: &'archetype mut SkydomeComponent,
    pub ambient_light: &'archetype mut AmbientLightComponent,
    pub stars: &'archetype mut StarsComponent,
    pub fog: &'archetype mut FogComponent,
}

/// An iterator for entities in the [`EnvironmentArchetype`].
pub struct EnvironmentEntityIterator<'archetype> {
    archetype: &'archetype EnvironmentArchetype,
    current: usize,
    length: usize,
}

impl<'archetype> EnvironmentEntityIterator<'archetype> {
    fn new(archetype: &'archetype EnvironmentArchetype) -> Self {
        Self {
            archetype,
            current: 0,
            length: archetype.len()
        }
    }
}

impl<'archetype> Iterator for EnvironmentEntityIterator<'archetype> {
    type Item = EnvironmentEntityRef<'archetype>;

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

impl EnvironmentArchetype {
    /// Iterates all entities in this archetype.
    #[inline]
    #[allow(dead_code)]
    pub fn iter(&self) -> EnvironmentEntityIterator {
        EnvironmentEntityIterator::new(self)
    }

    /// Gets the `TimeOfDay` component at the specified index.
    #[allow(dead_code)]
    #[inline]
    pub fn get_time_of_day_component_at(&self, index: usize) -> Option<&TimeOfDayComponent> {
        if index > self.len() {
            return None;
        }
        Some(unsafe { self.get_time_of_day_component_at_unchecked(index) })
    }

    /// Mutably gets the `TimeOfDay` component at the specified index.
    #[allow(dead_code)]
    #[inline]
    pub fn get_time_of_day_component_at_mut(&mut self, index: usize) -> Option<&mut TimeOfDayComponent> {
        if index > self.len() {
            return None;
        }
        Some(unsafe { self.get_time_of_day_component_at_unchecked_mut(index) })
    }

    /// Gets the `TimeOfDay` component at the specified index.
    #[allow(dead_code)]
    #[inline]
    pub unsafe fn get_time_of_day_component_at_unchecked(&self, index: usize) -> &TimeOfDayComponent {
        self.time_of_days.get_unchecked(index)
    }

    /// Gets the `TimeOfDay` component at the specified index.
    #[allow(dead_code)]
    #[inline]
    pub unsafe fn get_time_of_day_component_at_unchecked_mut(&mut self, index: usize) -> &mut TimeOfDayComponent {
        self.time_of_days.get_unchecked_mut(index)
    }

    /// Gets the `Sun` component at the specified index.
    #[allow(dead_code)]
    #[inline]
    pub fn get_sun_component_at(&self, index: usize) -> Option<&SunComponent> {
        if index > self.len() {
            return None;
        }
        Some(unsafe { self.get_sun_component_at_unchecked(index) })
    }

    /// Mutably gets the `Sun` component at the specified index.
    #[allow(dead_code)]
    #[inline]
    pub fn get_sun_component_at_mut(&mut self, index: usize) -> Option<&mut SunComponent> {
        if index > self.len() {
            return None;
        }
        Some(unsafe { self.get_sun_component_at_unchecked_mut(index) })
    }

    /// Gets the `Sun` component at the specified index.
    #[allow(dead_code)]
    #[inline]
    pub unsafe fn get_sun_component_at_unchecked(&self, index: usize) -> &SunComponent {
        self.suns.get_unchecked(index)
    }

    /// Gets the `Sun` component at the specified index.
    #[allow(dead_code)]
    #[inline]
    pub unsafe fn get_sun_component_at_unchecked_mut(&mut self, index: usize) -> &mut SunComponent {
        self.suns.get_unchecked_mut(index)
    }

    /// Gets the `Skydome` component at the specified index.
    #[allow(dead_code)]
    #[inline]
    pub fn get_skydome_component_at(&self, index: usize) -> Option<&SkydomeComponent> {
        if index > self.len() {
            return None;
        }
        Some(unsafe { self.get_skydome_component_at_unchecked(index) })
    }

    /// Mutably gets the `Skydome` component at the specified index.
    #[allow(dead_code)]
    #[inline]
    pub fn get_skydome_component_at_mut(&mut self, index: usize) -> Option<&mut SkydomeComponent> {
        if index > self.len() {
            return None;
        }
        Some(unsafe { self.get_skydome_component_at_unchecked_mut(index) })
    }

    /// Gets the `Skydome` component at the specified index.
    #[allow(dead_code)]
    #[inline]
    pub unsafe fn get_skydome_component_at_unchecked(&self, index: usize) -> &SkydomeComponent {
        self.skydomes.get_unchecked(index)
    }

    /// Gets the `Skydome` component at the specified index.
    #[allow(dead_code)]
    #[inline]
    pub unsafe fn get_skydome_component_at_unchecked_mut(&mut self, index: usize) -> &mut SkydomeComponent {
        self.skydomes.get_unchecked_mut(index)
    }

    /// Gets the `AmbientLight` component at the specified index.
    #[allow(dead_code)]
    #[inline]
    pub fn get_ambient_light_component_at(&self, index: usize) -> Option<&AmbientLightComponent> {
        if index > self.len() {
            return None;
        }
        Some(unsafe { self.get_ambient_light_component_at_unchecked(index) })
    }

    /// Mutably gets the `AmbientLight` component at the specified index.
    #[allow(dead_code)]
    #[inline]
    pub fn get_ambient_light_component_at_mut(&mut self, index: usize) -> Option<&mut AmbientLightComponent> {
        if index > self.len() {
            return None;
        }
        Some(unsafe { self.get_ambient_light_component_at_unchecked_mut(index) })
    }

    /// Gets the `AmbientLight` component at the specified index.
    #[allow(dead_code)]
    #[inline]
    pub unsafe fn get_ambient_light_component_at_unchecked(&self, index: usize) -> &AmbientLightComponent {
        self.ambient_lights.get_unchecked(index)
    }

    /// Gets the `AmbientLight` component at the specified index.
    #[allow(dead_code)]
    #[inline]
    pub unsafe fn get_ambient_light_component_at_unchecked_mut(&mut self, index: usize) -> &mut AmbientLightComponent {
        self.ambient_lights.get_unchecked_mut(index)
    }

    /// Gets the `Stars` component at the specified index.
    #[allow(dead_code)]
    #[inline]
    pub fn get_stars_component_at(&self, index: usize) -> Option<&StarsComponent> {
        if index > self.len() {
            return None;
        }
        Some(unsafe { self.get_stars_component_at_unchecked(index) })
    }

    /// Mutably gets the `Stars` component at the specified index.
    #[allow(dead_code)]
    #[inline]
    pub fn get_stars_component_at_mut(&mut self, index: usize) -> Option<&mut StarsComponent> {
        if index > self.len() {
            return None;
        }
        Some(unsafe { self.get_stars_component_at_unchecked_mut(index) })
    }

    /// Gets the `Stars` component at the specified index.
    #[allow(dead_code)]
    #[inline]
    pub unsafe fn get_stars_component_at_unchecked(&self, index: usize) -> &StarsComponent {
        self.stars.get_unchecked(index)
    }

    /// Gets the `Stars` component at the specified index.
    #[allow(dead_code)]
    #[inline]
    pub unsafe fn get_stars_component_at_unchecked_mut(&mut self, index: usize) -> &mut StarsComponent {
        self.stars.get_unchecked_mut(index)
    }

    /// Gets the `Fog` component at the specified index.
    #[allow(dead_code)]
    #[inline]
    pub fn get_fog_component_at(&self, index: usize) -> Option<&FogComponent> {
        if index > self.len() {
            return None;
        }
        Some(unsafe { self.get_fog_component_at_unchecked(index) })
    }

    /// Mutably gets the `Fog` component at the specified index.
    #[allow(dead_code)]
    #[inline]
    pub fn get_fog_component_at_mut(&mut self, index: usize) -> Option<&mut FogComponent> {
        if index > self.len() {
            return None;
        }
        Some(unsafe { self.get_fog_component_at_unchecked_mut(index) })
    }

    /// Gets the `Fog` component at the specified index.
    #[allow(dead_code)]
    #[inline]
    pub unsafe fn get_fog_component_at_unchecked(&self, index: usize) -> &FogComponent {
        self.fogs.get_unchecked(index)
    }

    /// Gets the `Fog` component at the specified index.
    #[allow(dead_code)]
    #[inline]
    pub unsafe fn get_fog_component_at_unchecked_mut(&mut self, index: usize) -> &mut FogComponent {
        self.fogs.get_unchecked_mut(index)
    }

    /// Gets the entity at the specified index.
    #[allow(dead_code)]
    pub fn get_entity_at(&self, index: usize) -> Option<EnvironmentEntityRef> {
        if index > self.len() {
            return None;
        }
        Some(unsafe { self.get_entity_at_unchecked(index) })
    }

    /// Mutably gets the entity at the specified index.
    #[allow(dead_code)]
    pub fn get_entity_at_mut(&mut self, index: usize) -> Option<EnvironmentEntityMut> {
        if index > self.len() {
            return None;
        }
        Some(unsafe { self.get_entity_at_unchecked_mut(index) })
    }

    /// Gets the entity at the specified index.
    #[allow(dead_code)]
    pub unsafe fn get_entity_at_unchecked(&self, index: usize) -> EnvironmentEntityRef {
        EnvironmentEntityRef {
            entity_id: *self.entities.get_unchecked(index),
            time_of_day: self.time_of_days.get_unchecked(index),
            sun: self.suns.get_unchecked(index),
            skydome: self.skydomes.get_unchecked(index),
            ambient_light: self.ambient_lights.get_unchecked(index),
            stars: self.stars.get_unchecked(index),
            fog: self.fogs.get_unchecked(index),
        }
    }

    /// Gets the entity at the specified index.
    #[allow(dead_code)]
    pub unsafe fn get_entity_at_unchecked_mut(&mut self, index: usize) -> EnvironmentEntityMut {
        EnvironmentEntityMut {
            entity_id: *self.entities.get_unchecked(index),
            time_of_day: self.time_of_days.get_unchecked_mut(index),
            sun: self.suns.get_unchecked_mut(index),
            skydome: self.skydomes.get_unchecked_mut(index),
            ambient_light: self.ambient_lights.get_unchecked_mut(index),
            stars: self.stars.get_unchecked_mut(index),
            fog: self.fogs.get_unchecked_mut(index),
        }
    }
}

#[allow(dead_code)]
impl HasComponent<TimeOfDayComponent> for EnvironmentEntity {
    #[inline]
    fn get(&self) -> &TimeOfDayComponent {
        &self.data.time_of_day
    }

    #[inline]
    fn get_mut(&mut self) -> &mut TimeOfDayComponent {
        &mut self.data.time_of_day
    }
}

#[allow(dead_code)]
impl HasComponent<TimeOfDayComponent> for EnvironmentEntityComponents {
    #[inline]
    fn get(&self) -> &TimeOfDayComponent {
        &self.time_of_day
    }

    #[inline]
    fn get_mut(&mut self) -> &mut TimeOfDayComponent {
        &mut self.time_of_day
    }
}

#[allow(dead_code)]
impl HasComponent<SunComponent> for EnvironmentEntity {
    #[inline]
    fn get(&self) -> &SunComponent {
        &self.data.sun
    }

    #[inline]
    fn get_mut(&mut self) -> &mut SunComponent {
        &mut self.data.sun
    }
}

#[allow(dead_code)]
impl HasComponent<SunComponent> for EnvironmentEntityComponents {
    #[inline]
    fn get(&self) -> &SunComponent {
        &self.sun
    }

    #[inline]
    fn get_mut(&mut self) -> &mut SunComponent {
        &mut self.sun
    }
}

#[allow(dead_code)]
impl HasComponent<SkydomeComponent> for EnvironmentEntity {
    #[inline]
    fn get(&self) -> &SkydomeComponent {
        &self.data.skydome
    }

    #[inline]
    fn get_mut(&mut self) -> &mut SkydomeComponent {
        &mut self.data.skydome
    }
}

#[allow(dead_code)]
impl HasComponent<SkydomeComponent> for EnvironmentEntityComponents {
    #[inline]
    fn get(&self) -> &SkydomeComponent {
        &self.skydome
    }

    #[inline]
    fn get_mut(&mut self) -> &mut SkydomeComponent {
        &mut self.skydome
    }
}

#[allow(dead_code)]
impl HasComponent<AmbientLightComponent> for EnvironmentEntity {
    #[inline]
    fn get(&self) -> &AmbientLightComponent {
        &self.data.ambient_light
    }

    #[inline]
    fn get_mut(&mut self) -> &mut AmbientLightComponent {
        &mut self.data.ambient_light
    }
}

#[allow(dead_code)]
impl HasComponent<AmbientLightComponent> for EnvironmentEntityComponents {
    #[inline]
    fn get(&self) -> &AmbientLightComponent {
        &self.ambient_light
    }

    #[inline]
    fn get_mut(&mut self) -> &mut AmbientLightComponent {
        &mut self.ambient_light
    }
}

#[allow(dead_code)]
impl HasComponent<StarsComponent> for EnvironmentEntity {
    #[inline]
    fn get(&self) -> &StarsComponent {
        &self.data.stars
    }

    #[inline]
    fn get_mut(&mut self) -> &mut StarsComponent {
        &mut self.data.stars
    }
}

#[allow(dead_code)]
impl HasComponent<StarsComponent> for EnvironmentEntityComponents {
    #[inline]
    fn get(&self) -> &StarsComponent {
        &self.stars
    }

    #[inline]
    fn get_mut(&mut self) -> &mut StarsComponent {
        &mut self.stars
    }
}

#[allow(dead_code)]
impl HasComponent<FogComponent> for EnvironmentEntity {
    #[inline]
    fn get(&self) -> &FogComponent {
        &self.data.fog
    }

    #[inline]
    fn get_mut(&mut self) -> &mut FogComponent {
        &mut self.data.fog
    }
}

#[allow(dead_code)]
impl HasComponent<FogComponent> for EnvironmentEntityComponents {
    #[inline]
    fn get(&self) -> &FogComponent {
        &self.fog
    }

    #[inline]
    fn get_mut(&mut self) -> &mut FogComponent {
        &mut self.fog
    }
}
impl EnvironmentEntityData {
    /// Spawn this entity into the given world.
    #[inline]
    #[allow(dead_code)]
    pub fn spawn_into<W>(self, world: &mut W) -> EntityId
    where
        W: Spawn<EnvironmentEntityData>
    {
        world.spawn(self)
    }
}

impl EnvironmentEntityComponents {
    /// Spawn this entity into the given world.
    #[inline]
    #[allow(dead_code)]
    pub fn spawn_into<W>(self, world: &mut W) -> EntityId
    where
        W: Spawn<EnvironmentEntityComponents>
    {
        world.spawn(self)
    }
}

impl core::ops::Deref for EnvironmentEntity {
    type Target = EnvironmentEntityComponents;

    fn deref(&self) -> &Self::Target {
        &self.data
    }
}

impl core::ops::DerefMut for EnvironmentEntity {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.data
    }
}

#[allow(dead_code)]
impl EnvironmentArchetype {
    /// The IDs of the components used by this archetype, in ascending order.
    pub const COMPONENTS: [ComponentId; 6] = [
        ComponentId::COMPONENT_1,
        ComponentId::COMPONENT_2,
        ComponentId::COMPONENT_3,
        ComponentId::COMPONENT_4,
        ComponentId::COMPONENT_5,
        ComponentId::COMPONENT_6,
    ];

    /// Returns the number of entities in this archetype.
    #[inline(always)]
    pub fn len(&self) -> usize {
        self.entities.len()
    }

    /// Indicates whether this archetype has no entities.
    #[inline(always)]
    #[allow(dead_code)]
    pub fn is_empty(&self) -> bool {
        self.entities.is_empty()
    }

    /// Returns the number of components used by this archetype.
    #[inline(always)]
    pub const fn num_components(&self) -> usize {
        6
    }

    /// Dynamically determines whether this archetype has a specific component.
    pub const fn has_components<Id>(&self, id: ComponentId) -> bool {
        let id = id.as_u64();
        id == 1 || id == 2 || id == 3 || id == 4 || id == 5 || id == 6 || false
    }

    /// Spawn a new entity into the world. Must be called through the world for proper bookkeeping.
    #[doc(hidden)]
    pub fn spawn_with<R>(
        &mut self,
        time_of_day: TimeOfDayComponent,
        sun: SunComponent,
        skydome: SkydomeComponent,
        ambient_light: AmbientLightComponent,
        stars: StarsComponent,
        fog: FogComponent,
        mut world_registry: R
    ) -> EntityId
    where
        R: WorldEntityRegistry
    {
        self.time_of_days.push(time_of_day);
        self.suns.push(sun);
        self.skydomes.push(skydome);
        self.ambient_lights.push(ambient_light);
        self.stars.push(stars);
        self.fogs.push(fog);

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
        self.time_of_days.swap_remove(index);
        self.suns.swap_remove(index);
        self.skydomes.swap_remove(index);
        self.ambient_lights.swap_remove(index);
        self.stars.swap_remove(index);
        self.fogs.swap_remove(index);
        if self.entities.is_empty() {
            Ok(None)
        }
        else {
            Ok(Some(self.entities[index]))
        }
    }
}


#[automatically_derived]
impl HasComponents<TimeOfDayComponent> for EnvironmentArchetype {
    /// Returns a slice of all [`TimeOfDayComponent`] component values.
    fn get(&self) -> &[TimeOfDayComponent] {
        self.time_of_days.as_slice()
    }

    /// Returns a mutable slice of all [`TimeOfDayComponent`] component values.
    fn get_mut(&mut self) -> &mut [TimeOfDayComponent] {
        self.time_of_days.as_mut_slice()
    }
}

#[automatically_derived]
impl HasComponents<SunComponent> for EnvironmentArchetype {
    /// Returns a slice of all [`SunComponent`] component values.
    fn get(&self) -> &[SunComponent] {
        self.suns.as_slice()
    }

    /// Returns a mutable slice of all [`SunComponent`] component values.
    fn get_mut(&mut self) -> &mut [SunComponent] {
        self.suns.as_mut_slice()
    }
}

#[automatically_derived]
impl HasComponents<SkydomeComponent> for EnvironmentArchetype {
    /// Returns a slice of all [`SkydomeComponent`] component values.
    fn get(&self) -> &[SkydomeComponent] {
        self.skydomes.as_slice()
    }

    /// Returns a mutable slice of all [`SkydomeComponent`] component values.
    fn get_mut(&mut self) -> &mut [SkydomeComponent] {
        self.skydomes.as_mut_slice()
    }
}

#[automatically_derived]
impl HasComponents<AmbientLightComponent> for EnvironmentArchetype {
    /// Returns a slice of all [`AmbientLightComponent`] component values.
    fn get(&self) -> &[AmbientLightComponent] {
        self.ambient_lights.as_slice()
    }

    /// Returns a mutable slice of all [`AmbientLightComponent`] component values.
    fn get_mut(&mut self) -> &mut [AmbientLightComponent] {
        self.ambient_lights.as_mut_slice()
    }
}

#[automatically_derived]
impl HasComponents<StarsComponent> for EnvironmentArchetype {
    /// Returns a slice of all [`StarsComponent`] component values.
    fn get(&self) -> &[StarsComponent] {
        self.stars.as_slice()
    }

    /// Returns a mutable slice of all [`StarsComponent`] component values.
    fn get_mut(&mut self) -> &mut [StarsComponent] {
        self.stars.as_mut_slice()
    }
}

#[automatically_derived]
impl HasComponents<FogComponent> for EnvironmentArchetype {
    /// Returns a slice of all [`FogComponent`] component values.
    fn get(&self) -> &[FogComponent] {
        self.fogs.as_slice()
    }

    /// Returns a mutable slice of all [`FogComponent`] component values.
    fn get_mut(&mut self) -> &mut [FogComponent] {
        self.fogs.as_mut_slice()
    }
}


#[automatically_derived]
#[allow(dead_code)]
impl Archetype for EnvironmentArchetype {
    const ID: ArchetypeId = ArchetypeId::Environment;

    fn len(&self) -> usize {
        self.entities.len()
    }

    fn is_empty(&self) -> bool {
        self.entities.is_empty()
    }
}

impl PartialEq<ArchetypeId> for EnvironmentArchetype {
    fn eq(&self, other: &ArchetypeId) -> bool {
        Self::ID.eq(other)
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
pub type CameraEntity = EntityWithIdAndData<CameraEntityComponents>;

/// The data of an entity of the [`CameraArchetype`].
#[derive(Debug, Clone)]
#[allow(dead_code)]
pub struct CameraEntityData {
    pub camera: CameraData,
    pub camera_fps_controller: CameraFpsControllerData,
}

/// An entity of the [`CameraArchetype`].
#[derive(Debug, Clone)]
#[allow(dead_code)]
pub struct CameraEntityComponents {
    pub camera: CameraComponent,
    pub camera_fps_controller: CameraFpsControllerComponent,
}

impl From<CameraEntityData> for CameraEntityComponents {
    fn from(value: CameraEntityData) -> Self {
        Self {
            camera: value.camera.into(),
            camera_fps_controller: value.camera_fps_controller.into(),
        }
    }
}

impl EntityData for CameraEntityData {
    const ARCHETYPE_ID: ArchetypeId = CameraArchetype::ID;
}

impl EntityData for CameraEntityComponents {
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

    /// Gets the `Camera` component at the specified index.
    #[allow(dead_code)]
    #[inline]
    pub fn get_camera_component_at(&self, index: usize) -> Option<&CameraComponent> {
        if index > self.len() {
            return None;
        }
        Some(unsafe { self.get_camera_component_at_unchecked(index) })
    }

    /// Mutably gets the `Camera` component at the specified index.
    #[allow(dead_code)]
    #[inline]
    pub fn get_camera_component_at_mut(&mut self, index: usize) -> Option<&mut CameraComponent> {
        if index > self.len() {
            return None;
        }
        Some(unsafe { self.get_camera_component_at_unchecked_mut(index) })
    }

    /// Gets the `Camera` component at the specified index.
    #[allow(dead_code)]
    #[inline]
    pub unsafe fn get_camera_component_at_unchecked(&self, index: usize) -> &CameraComponent {
        self.cameras.get_unchecked(index)
    }

    /// Gets the `Camera` component at the specified index.
    #[allow(dead_code)]
    #[inline]
    pub unsafe fn get_camera_component_at_unchecked_mut(&mut self, index: usize) -> &mut CameraComponent {
        self.cameras.get_unchecked_mut(index)
    }

    /// Gets the `CameraFpsController` component at the specified index.
    #[allow(dead_code)]
    #[inline]
    pub fn get_camera_fps_controller_component_at(&self, index: usize) -> Option<&CameraFpsControllerComponent> {
        if index > self.len() {
            return None;
        }
        Some(unsafe { self.get_camera_fps_controller_component_at_unchecked(index) })
    }

    /// Mutably gets the `CameraFpsController` component at the specified index.
    #[allow(dead_code)]
    #[inline]
    pub fn get_camera_fps_controller_component_at_mut(&mut self, index: usize) -> Option<&mut CameraFpsControllerComponent> {
        if index > self.len() {
            return None;
        }
        Some(unsafe { self.get_camera_fps_controller_component_at_unchecked_mut(index) })
    }

    /// Gets the `CameraFpsController` component at the specified index.
    #[allow(dead_code)]
    #[inline]
    pub unsafe fn get_camera_fps_controller_component_at_unchecked(&self, index: usize) -> &CameraFpsControllerComponent {
        self.camera_fps_controllers.get_unchecked(index)
    }

    /// Gets the `CameraFpsController` component at the specified index.
    #[allow(dead_code)]
    #[inline]
    pub unsafe fn get_camera_fps_controller_component_at_unchecked_mut(&mut self, index: usize) -> &mut CameraFpsControllerComponent {
        self.camera_fps_controllers.get_unchecked_mut(index)
    }

    /// Gets the entity at the specified index.
    #[allow(dead_code)]
    pub fn get_entity_at(&self, index: usize) -> Option<CameraEntityRef> {
        if index > self.len() {
            return None;
        }
        Some(unsafe { self.get_entity_at_unchecked(index) })
    }

    /// Mutably gets the entity at the specified index.
    #[allow(dead_code)]
    pub fn get_entity_at_mut(&mut self, index: usize) -> Option<CameraEntityMut> {
        if index > self.len() {
            return None;
        }
        Some(unsafe { self.get_entity_at_unchecked_mut(index) })
    }

    /// Gets the entity at the specified index.
    #[allow(dead_code)]
    pub unsafe fn get_entity_at_unchecked(&self, index: usize) -> CameraEntityRef {
        CameraEntityRef {
            entity_id: *self.entities.get_unchecked(index),
            camera: self.cameras.get_unchecked(index),
            camera_fps_controller: self.camera_fps_controllers.get_unchecked(index),
        }
    }

    /// Gets the entity at the specified index.
    #[allow(dead_code)]
    pub unsafe fn get_entity_at_unchecked_mut(&mut self, index: usize) -> CameraEntityMut {
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
impl HasComponent<CameraComponent> for CameraEntityComponents {
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
impl HasComponent<CameraFpsControllerComponent> for CameraEntityComponents {
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

impl CameraEntityComponents {
    /// Spawn this entity into the given world.
    #[inline]
    #[allow(dead_code)]
    pub fn spawn_into<W>(self, world: &mut W) -> EntityId
    where
        W: Spawn<CameraEntityComponents>
    {
        world.spawn(self)
    }
}

impl core::ops::Deref for CameraEntity {
    type Target = CameraEntityComponents;

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
    pub const COMPONENTS: [ComponentId; 2] = [
        ComponentId::COMPONENT_7,
        ComponentId::COMPONENT_8,
    ];

    /// Returns the number of entities in this archetype.
    #[inline(always)]
    pub fn len(&self) -> usize {
        self.entities.len()
    }

    /// Indicates whether this archetype has no entities.
    #[inline(always)]
    #[allow(dead_code)]
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
        id == 7 || id == 8 || false
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
    const ID: ArchetypeId = ArchetypeId::Camera;

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
pub type DynamicLightEntity = EntityWithIdAndData<DynamicLightEntityComponents>;

/// The data of an entity of the [`DynamicLightArchetype`].
#[derive(Debug, Clone)]
#[allow(dead_code)]
pub struct DynamicLightEntityData {
    pub dynamic_light: DynamicLightData,
    pub color: ColorData,
    pub position: PositionData,
}

/// An entity of the [`DynamicLightArchetype`].
#[derive(Debug, Clone)]
#[allow(dead_code)]
pub struct DynamicLightEntityComponents {
    pub dynamic_light: DynamicLightComponent,
    pub color: ColorComponent,
    pub position: PositionComponent,
}

impl From<DynamicLightEntityData> for DynamicLightEntityComponents {
    fn from(value: DynamicLightEntityData) -> Self {
        Self {
            dynamic_light: value.dynamic_light.into(),
            color: value.color.into(),
            position: value.position.into(),
        }
    }
}

impl EntityData for DynamicLightEntityData {
    const ARCHETYPE_ID: ArchetypeId = DynamicLightArchetype::ID;
}

impl EntityData for DynamicLightEntityComponents {
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

    /// Gets the `DynamicLight` component at the specified index.
    #[allow(dead_code)]
    #[inline]
    pub fn get_dynamic_light_component_at(&self, index: usize) -> Option<&DynamicLightComponent> {
        if index > self.len() {
            return None;
        }
        Some(unsafe { self.get_dynamic_light_component_at_unchecked(index) })
    }

    /// Mutably gets the `DynamicLight` component at the specified index.
    #[allow(dead_code)]
    #[inline]
    pub fn get_dynamic_light_component_at_mut(&mut self, index: usize) -> Option<&mut DynamicLightComponent> {
        if index > self.len() {
            return None;
        }
        Some(unsafe { self.get_dynamic_light_component_at_unchecked_mut(index) })
    }

    /// Gets the `DynamicLight` component at the specified index.
    #[allow(dead_code)]
    #[inline]
    pub unsafe fn get_dynamic_light_component_at_unchecked(&self, index: usize) -> &DynamicLightComponent {
        self.dynamic_lights.get_unchecked(index)
    }

    /// Gets the `DynamicLight` component at the specified index.
    #[allow(dead_code)]
    #[inline]
    pub unsafe fn get_dynamic_light_component_at_unchecked_mut(&mut self, index: usize) -> &mut DynamicLightComponent {
        self.dynamic_lights.get_unchecked_mut(index)
    }

    /// Gets the `Color` component at the specified index.
    #[allow(dead_code)]
    #[inline]
    pub fn get_color_component_at(&self, index: usize) -> Option<&ColorComponent> {
        if index > self.len() {
            return None;
        }
        Some(unsafe { self.get_color_component_at_unchecked(index) })
    }

    /// Mutably gets the `Color` component at the specified index.
    #[allow(dead_code)]
    #[inline]
    pub fn get_color_component_at_mut(&mut self, index: usize) -> Option<&mut ColorComponent> {
        if index > self.len() {
            return None;
        }
        Some(unsafe { self.get_color_component_at_unchecked_mut(index) })
    }

    /// Gets the `Color` component at the specified index.
    #[allow(dead_code)]
    #[inline]
    pub unsafe fn get_color_component_at_unchecked(&self, index: usize) -> &ColorComponent {
        self.colors.get_unchecked(index)
    }

    /// Gets the `Color` component at the specified index.
    #[allow(dead_code)]
    #[inline]
    pub unsafe fn get_color_component_at_unchecked_mut(&mut self, index: usize) -> &mut ColorComponent {
        self.colors.get_unchecked_mut(index)
    }

    /// Gets the `Position` component at the specified index.
    #[allow(dead_code)]
    #[inline]
    pub fn get_position_component_at(&self, index: usize) -> Option<&PositionComponent> {
        if index > self.len() {
            return None;
        }
        Some(unsafe { self.get_position_component_at_unchecked(index) })
    }

    /// Mutably gets the `Position` component at the specified index.
    #[allow(dead_code)]
    #[inline]
    pub fn get_position_component_at_mut(&mut self, index: usize) -> Option<&mut PositionComponent> {
        if index > self.len() {
            return None;
        }
        Some(unsafe { self.get_position_component_at_unchecked_mut(index) })
    }

    /// Gets the `Position` component at the specified index.
    #[allow(dead_code)]
    #[inline]
    pub unsafe fn get_position_component_at_unchecked(&self, index: usize) -> &PositionComponent {
        self.positions.get_unchecked(index)
    }

    /// Gets the `Position` component at the specified index.
    #[allow(dead_code)]
    #[inline]
    pub unsafe fn get_position_component_at_unchecked_mut(&mut self, index: usize) -> &mut PositionComponent {
        self.positions.get_unchecked_mut(index)
    }

    /// Gets the entity at the specified index.
    #[allow(dead_code)]
    pub fn get_entity_at(&self, index: usize) -> Option<DynamicLightEntityRef> {
        if index > self.len() {
            return None;
        }
        Some(unsafe { self.get_entity_at_unchecked(index) })
    }

    /// Mutably gets the entity at the specified index.
    #[allow(dead_code)]
    pub fn get_entity_at_mut(&mut self, index: usize) -> Option<DynamicLightEntityMut> {
        if index > self.len() {
            return None;
        }
        Some(unsafe { self.get_entity_at_unchecked_mut(index) })
    }

    /// Gets the entity at the specified index.
    #[allow(dead_code)]
    pub unsafe fn get_entity_at_unchecked(&self, index: usize) -> DynamicLightEntityRef {
        DynamicLightEntityRef {
            entity_id: *self.entities.get_unchecked(index),
            dynamic_light: self.dynamic_lights.get_unchecked(index),
            color: self.colors.get_unchecked(index),
            position: self.positions.get_unchecked(index),
        }
    }

    /// Gets the entity at the specified index.
    #[allow(dead_code)]
    pub unsafe fn get_entity_at_unchecked_mut(&mut self, index: usize) -> DynamicLightEntityMut {
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
impl HasComponent<DynamicLightComponent> for DynamicLightEntityComponents {
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
impl HasComponent<ColorComponent> for DynamicLightEntityComponents {
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
impl HasComponent<PositionComponent> for DynamicLightEntityComponents {
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

impl DynamicLightEntityComponents {
    /// Spawn this entity into the given world.
    #[inline]
    #[allow(dead_code)]
    pub fn spawn_into<W>(self, world: &mut W) -> EntityId
    where
        W: Spawn<DynamicLightEntityComponents>
    {
        world.spawn(self)
    }
}

impl core::ops::Deref for DynamicLightEntity {
    type Target = DynamicLightEntityComponents;

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
    pub const COMPONENTS: [ComponentId; 3] = [
        ComponentId::COMPONENT_9,
        ComponentId::COMPONENT_16,
        ComponentId::COMPONENT_18,
    ];

    /// Returns the number of entities in this archetype.
    #[inline(always)]
    pub fn len(&self) -> usize {
        self.entities.len()
    }

    /// Indicates whether this archetype has no entities.
    #[inline(always)]
    #[allow(dead_code)]
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
        id == 9 || id == 16 || id == 18 || false
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
    const ID: ArchetypeId = ArchetypeId::DynamicLight;

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

/// An archetype grouping entities with identical components.
#[derive(Debug, Default, Clone)]
pub struct TileArchetype {
    pub entities: Vec<EntityId>,
    pub tiles: Vec<TileComponent>,
    pub positions: Vec<PositionComponent>,
}

/// An entity of the [`TileArchetype`].
pub type TileEntity = EntityWithIdAndData<TileEntityComponents>;

/// The data of an entity of the [`TileArchetype`].
#[derive(Debug, Clone)]
#[allow(dead_code)]
pub struct TileEntityData {
    pub tile: TileData,
    pub position: PositionData,
}

/// An entity of the [`TileArchetype`].
#[derive(Debug, Clone)]
#[allow(dead_code)]
pub struct TileEntityComponents {
    pub tile: TileComponent,
    pub position: PositionComponent,
}

impl From<TileEntityData> for TileEntityComponents {
    fn from(value: TileEntityData) -> Self {
        Self {
            tile: value.tile.into(),
            position: value.position.into(),
        }
    }
}

impl EntityData for TileEntityData {
    const ARCHETYPE_ID: ArchetypeId = TileArchetype::ID;
}

impl EntityData for TileEntityComponents {
    const ARCHETYPE_ID: ArchetypeId = TileArchetype::ID;
}

/// A reference to an entity of the [`TileArchetype`].
#[derive(Debug, Clone)]
#[allow(dead_code)]
pub struct TileEntityRef<'archetype> {
    pub entity_id: EntityId,
    pub tile: &'archetype TileComponent,
    pub position: &'archetype PositionComponent,
}

/// A mutable reference to an entity of the [`TileArchetype`].
#[derive(Debug)]
#[allow(dead_code)]
pub struct TileEntityMut<'archetype> {
    pub entity_id: EntityId,
    pub tile: &'archetype mut TileComponent,
    pub position: &'archetype mut PositionComponent,
}

/// An iterator for entities in the [`TileArchetype`].
pub struct TileEntityIterator<'archetype> {
    archetype: &'archetype TileArchetype,
    current: usize,
    length: usize,
}

impl<'archetype> TileEntityIterator<'archetype> {
    fn new(archetype: &'archetype TileArchetype) -> Self {
        Self {
            archetype,
            current: 0,
            length: archetype.len()
        }
    }
}

impl<'archetype> Iterator for TileEntityIterator<'archetype> {
    type Item = TileEntityRef<'archetype>;

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

impl TileArchetype {
    /// Iterates all entities in this archetype.
    #[inline]
    #[allow(dead_code)]
    pub fn iter(&self) -> TileEntityIterator {
        TileEntityIterator::new(self)
    }

    /// Gets the `Tile` component at the specified index.
    #[allow(dead_code)]
    #[inline]
    pub fn get_tile_component_at(&self, index: usize) -> Option<&TileComponent> {
        if index > self.len() {
            return None;
        }
        Some(unsafe { self.get_tile_component_at_unchecked(index) })
    }

    /// Mutably gets the `Tile` component at the specified index.
    #[allow(dead_code)]
    #[inline]
    pub fn get_tile_component_at_mut(&mut self, index: usize) -> Option<&mut TileComponent> {
        if index > self.len() {
            return None;
        }
        Some(unsafe { self.get_tile_component_at_unchecked_mut(index) })
    }

    /// Gets the `Tile` component at the specified index.
    #[allow(dead_code)]
    #[inline]
    pub unsafe fn get_tile_component_at_unchecked(&self, index: usize) -> &TileComponent {
        self.tiles.get_unchecked(index)
    }

    /// Gets the `Tile` component at the specified index.
    #[allow(dead_code)]
    #[inline]
    pub unsafe fn get_tile_component_at_unchecked_mut(&mut self, index: usize) -> &mut TileComponent {
        self.tiles.get_unchecked_mut(index)
    }

    /// Gets the `Position` component at the specified index.
    #[allow(dead_code)]
    #[inline]
    pub fn get_position_component_at(&self, index: usize) -> Option<&PositionComponent> {
        if index > self.len() {
            return None;
        }
        Some(unsafe { self.get_position_component_at_unchecked(index) })
    }

    /// Mutably gets the `Position` component at the specified index.
    #[allow(dead_code)]
    #[inline]
    pub fn get_position_component_at_mut(&mut self, index: usize) -> Option<&mut PositionComponent> {
        if index > self.len() {
            return None;
        }
        Some(unsafe { self.get_position_component_at_unchecked_mut(index) })
    }

    /// Gets the `Position` component at the specified index.
    #[allow(dead_code)]
    #[inline]
    pub unsafe fn get_position_component_at_unchecked(&self, index: usize) -> &PositionComponent {
        self.positions.get_unchecked(index)
    }

    /// Gets the `Position` component at the specified index.
    #[allow(dead_code)]
    #[inline]
    pub unsafe fn get_position_component_at_unchecked_mut(&mut self, index: usize) -> &mut PositionComponent {
        self.positions.get_unchecked_mut(index)
    }

    /// Gets the entity at the specified index.
    #[allow(dead_code)]
    pub fn get_entity_at(&self, index: usize) -> Option<TileEntityRef> {
        if index > self.len() {
            return None;
        }
        Some(unsafe { self.get_entity_at_unchecked(index) })
    }

    /// Mutably gets the entity at the specified index.
    #[allow(dead_code)]
    pub fn get_entity_at_mut(&mut self, index: usize) -> Option<TileEntityMut> {
        if index > self.len() {
            return None;
        }
        Some(unsafe { self.get_entity_at_unchecked_mut(index) })
    }

    /// Gets the entity at the specified index.
    #[allow(dead_code)]
    pub unsafe fn get_entity_at_unchecked(&self, index: usize) -> TileEntityRef {
        TileEntityRef {
            entity_id: *self.entities.get_unchecked(index),
            tile: self.tiles.get_unchecked(index),
            position: self.positions.get_unchecked(index),
        }
    }

    /// Gets the entity at the specified index.
    #[allow(dead_code)]
    pub unsafe fn get_entity_at_unchecked_mut(&mut self, index: usize) -> TileEntityMut {
        TileEntityMut {
            entity_id: *self.entities.get_unchecked(index),
            tile: self.tiles.get_unchecked_mut(index),
            position: self.positions.get_unchecked_mut(index),
        }
    }
}

#[allow(dead_code)]
impl HasComponent<TileComponent> for TileEntity {
    #[inline]
    fn get(&self) -> &TileComponent {
        &self.data.tile
    }

    #[inline]
    fn get_mut(&mut self) -> &mut TileComponent {
        &mut self.data.tile
    }
}

#[allow(dead_code)]
impl HasComponent<TileComponent> for TileEntityComponents {
    #[inline]
    fn get(&self) -> &TileComponent {
        &self.tile
    }

    #[inline]
    fn get_mut(&mut self) -> &mut TileComponent {
        &mut self.tile
    }
}

#[allow(dead_code)]
impl HasComponent<PositionComponent> for TileEntity {
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
impl HasComponent<PositionComponent> for TileEntityComponents {
    #[inline]
    fn get(&self) -> &PositionComponent {
        &self.position
    }

    #[inline]
    fn get_mut(&mut self) -> &mut PositionComponent {
        &mut self.position
    }
}
impl TileEntityData {
    /// Spawn this entity into the given world.
    #[inline]
    #[allow(dead_code)]
    pub fn spawn_into<W>(self, world: &mut W) -> EntityId
    where
        W: Spawn<TileEntityData>
    {
        world.spawn(self)
    }
}

impl TileEntityComponents {
    /// Spawn this entity into the given world.
    #[inline]
    #[allow(dead_code)]
    pub fn spawn_into<W>(self, world: &mut W) -> EntityId
    where
        W: Spawn<TileEntityComponents>
    {
        world.spawn(self)
    }
}

impl core::ops::Deref for TileEntity {
    type Target = TileEntityComponents;

    fn deref(&self) -> &Self::Target {
        &self.data
    }
}

impl core::ops::DerefMut for TileEntity {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.data
    }
}

#[allow(dead_code)]
impl TileArchetype {
    /// The IDs of the components used by this archetype, in ascending order.
    pub const COMPONENTS: [ComponentId; 2] = [
        ComponentId::COMPONENT_9,
        ComponentId::COMPONENT_12,
    ];

    /// Returns the number of entities in this archetype.
    #[inline(always)]
    pub fn len(&self) -> usize {
        self.entities.len()
    }

    /// Indicates whether this archetype has no entities.
    #[inline(always)]
    #[allow(dead_code)]
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
        id == 9 || id == 12 || false
    }

    /// Spawn a new entity into the world. Must be called through the world for proper bookkeeping.
    #[doc(hidden)]
    pub fn spawn_with<R>(
        &mut self,
        tile: TileComponent,
        position: PositionComponent,
        mut world_registry: R
    ) -> EntityId
    where
        R: WorldEntityRegistry
    {
        self.tiles.push(tile);
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
        self.tiles.swap_remove(index);
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
impl HasComponents<TileComponent> for TileArchetype {
    /// Returns a slice of all [`TileComponent`] component values.
    fn get(&self) -> &[TileComponent] {
        self.tiles.as_slice()
    }

    /// Returns a mutable slice of all [`TileComponent`] component values.
    fn get_mut(&mut self) -> &mut [TileComponent] {
        self.tiles.as_mut_slice()
    }
}

#[automatically_derived]
impl HasComponents<PositionComponent> for TileArchetype {
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
impl Archetype for TileArchetype {
    const ID: ArchetypeId = ArchetypeId::Tile;

    fn len(&self) -> usize {
        self.entities.len()
    }

    fn is_empty(&self) -> bool {
        self.entities.is_empty()
    }
}

impl PartialEq<ArchetypeId> for TileArchetype {
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
pub type CubeEntity = EntityWithIdAndData<CubeEntityComponents>;

/// The data of an entity of the [`CubeArchetype`].
#[derive(Debug, Clone)]
#[allow(dead_code)]
pub struct CubeEntityData {
    pub position: PositionData,
    pub velocity: VelocityData,
    pub transform: TransformData,
    pub model: ModelData,
}

/// An entity of the [`CubeArchetype`].
#[derive(Debug, Clone)]
#[allow(dead_code)]
pub struct CubeEntityComponents {
    pub position: PositionComponent,
    pub velocity: VelocityComponent,
    pub transform: TransformComponent,
    pub model: ModelComponent,
}

impl From<CubeEntityData> for CubeEntityComponents {
    fn from(value: CubeEntityData) -> Self {
        Self {
            position: value.position.into(),
            velocity: value.velocity.into(),
            transform: value.transform.into(),
            model: value.model.into(),
        }
    }
}

impl EntityData for CubeEntityData {
    const ARCHETYPE_ID: ArchetypeId = CubeArchetype::ID;
}

impl EntityData for CubeEntityComponents {
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

    /// Gets the `Position` component at the specified index.
    #[allow(dead_code)]
    #[inline]
    pub fn get_position_component_at(&self, index: usize) -> Option<&PositionComponent> {
        if index > self.len() {
            return None;
        }
        Some(unsafe { self.get_position_component_at_unchecked(index) })
    }

    /// Mutably gets the `Position` component at the specified index.
    #[allow(dead_code)]
    #[inline]
    pub fn get_position_component_at_mut(&mut self, index: usize) -> Option<&mut PositionComponent> {
        if index > self.len() {
            return None;
        }
        Some(unsafe { self.get_position_component_at_unchecked_mut(index) })
    }

    /// Gets the `Position` component at the specified index.
    #[allow(dead_code)]
    #[inline]
    pub unsafe fn get_position_component_at_unchecked(&self, index: usize) -> &PositionComponent {
        self.positions.get_unchecked(index)
    }

    /// Gets the `Position` component at the specified index.
    #[allow(dead_code)]
    #[inline]
    pub unsafe fn get_position_component_at_unchecked_mut(&mut self, index: usize) -> &mut PositionComponent {
        self.positions.get_unchecked_mut(index)
    }

    /// Gets the `Velocity` component at the specified index.
    #[allow(dead_code)]
    #[inline]
    pub fn get_velocity_component_at(&self, index: usize) -> Option<&VelocityComponent> {
        if index > self.len() {
            return None;
        }
        Some(unsafe { self.get_velocity_component_at_unchecked(index) })
    }

    /// Mutably gets the `Velocity` component at the specified index.
    #[allow(dead_code)]
    #[inline]
    pub fn get_velocity_component_at_mut(&mut self, index: usize) -> Option<&mut VelocityComponent> {
        if index > self.len() {
            return None;
        }
        Some(unsafe { self.get_velocity_component_at_unchecked_mut(index) })
    }

    /// Gets the `Velocity` component at the specified index.
    #[allow(dead_code)]
    #[inline]
    pub unsafe fn get_velocity_component_at_unchecked(&self, index: usize) -> &VelocityComponent {
        self.velocities.get_unchecked(index)
    }

    /// Gets the `Velocity` component at the specified index.
    #[allow(dead_code)]
    #[inline]
    pub unsafe fn get_velocity_component_at_unchecked_mut(&mut self, index: usize) -> &mut VelocityComponent {
        self.velocities.get_unchecked_mut(index)
    }

    /// Gets the `Transform` component at the specified index.
    #[allow(dead_code)]
    #[inline]
    pub fn get_transform_component_at(&self, index: usize) -> Option<&TransformComponent> {
        if index > self.len() {
            return None;
        }
        Some(unsafe { self.get_transform_component_at_unchecked(index) })
    }

    /// Mutably gets the `Transform` component at the specified index.
    #[allow(dead_code)]
    #[inline]
    pub fn get_transform_component_at_mut(&mut self, index: usize) -> Option<&mut TransformComponent> {
        if index > self.len() {
            return None;
        }
        Some(unsafe { self.get_transform_component_at_unchecked_mut(index) })
    }

    /// Gets the `Transform` component at the specified index.
    #[allow(dead_code)]
    #[inline]
    pub unsafe fn get_transform_component_at_unchecked(&self, index: usize) -> &TransformComponent {
        self.transforms.get_unchecked(index)
    }

    /// Gets the `Transform` component at the specified index.
    #[allow(dead_code)]
    #[inline]
    pub unsafe fn get_transform_component_at_unchecked_mut(&mut self, index: usize) -> &mut TransformComponent {
        self.transforms.get_unchecked_mut(index)
    }

    /// Gets the `Model` component at the specified index.
    #[allow(dead_code)]
    #[inline]
    pub fn get_model_component_at(&self, index: usize) -> Option<&ModelComponent> {
        if index > self.len() {
            return None;
        }
        Some(unsafe { self.get_model_component_at_unchecked(index) })
    }

    /// Mutably gets the `Model` component at the specified index.
    #[allow(dead_code)]
    #[inline]
    pub fn get_model_component_at_mut(&mut self, index: usize) -> Option<&mut ModelComponent> {
        if index > self.len() {
            return None;
        }
        Some(unsafe { self.get_model_component_at_unchecked_mut(index) })
    }

    /// Gets the `Model` component at the specified index.
    #[allow(dead_code)]
    #[inline]
    pub unsafe fn get_model_component_at_unchecked(&self, index: usize) -> &ModelComponent {
        self.models.get_unchecked(index)
    }

    /// Gets the `Model` component at the specified index.
    #[allow(dead_code)]
    #[inline]
    pub unsafe fn get_model_component_at_unchecked_mut(&mut self, index: usize) -> &mut ModelComponent {
        self.models.get_unchecked_mut(index)
    }

    /// Gets the entity at the specified index.
    #[allow(dead_code)]
    pub fn get_entity_at(&self, index: usize) -> Option<CubeEntityRef> {
        if index > self.len() {
            return None;
        }
        Some(unsafe { self.get_entity_at_unchecked(index) })
    }

    /// Mutably gets the entity at the specified index.
    #[allow(dead_code)]
    pub fn get_entity_at_mut(&mut self, index: usize) -> Option<CubeEntityMut> {
        if index > self.len() {
            return None;
        }
        Some(unsafe { self.get_entity_at_unchecked_mut(index) })
    }

    /// Gets the entity at the specified index.
    #[allow(dead_code)]
    pub unsafe fn get_entity_at_unchecked(&self, index: usize) -> CubeEntityRef {
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
    pub unsafe fn get_entity_at_unchecked_mut(&mut self, index: usize) -> CubeEntityMut {
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
impl HasComponent<PositionComponent> for CubeEntityComponents {
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
impl HasComponent<VelocityComponent> for CubeEntityComponents {
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
impl HasComponent<TransformComponent> for CubeEntityComponents {
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
impl HasComponent<ModelComponent> for CubeEntityComponents {
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

impl CubeEntityComponents {
    /// Spawn this entity into the given world.
    #[inline]
    #[allow(dead_code)]
    pub fn spawn_into<W>(self, world: &mut W) -> EntityId
    where
        W: Spawn<CubeEntityComponents>
    {
        world.spawn(self)
    }
}

impl core::ops::Deref for CubeEntity {
    type Target = CubeEntityComponents;

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
    pub const COMPONENTS: [ComponentId; 4] = [
        ComponentId::COMPONENT_9,
        ComponentId::COMPONENT_10,
        ComponentId::COMPONENT_11,
        ComponentId::COMPONENT_15,
    ];

    /// Returns the number of entities in this archetype.
    #[inline(always)]
    pub fn len(&self) -> usize {
        self.entities.len()
    }

    /// Indicates whether this archetype has no entities.
    #[inline(always)]
    #[allow(dead_code)]
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
        id == 9 || id == 10 || id == 11 || id == 15 || false
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
    const ID: ArchetypeId = ArchetypeId::Cube;

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
pub type PlayerEntity = EntityWithIdAndData<PlayerEntityComponents>;

/// The data of an entity of the [`PlayerArchetype`].
#[derive(Debug, Clone)]
#[allow(dead_code)]
pub struct PlayerEntityData {
    pub position: PositionData,
    pub velocity: VelocityData,
    pub health: HealthData,
}

/// An entity of the [`PlayerArchetype`].
#[derive(Debug, Clone)]
#[allow(dead_code)]
pub struct PlayerEntityComponents {
    pub position: PositionComponent,
    pub velocity: VelocityComponent,
    pub health: HealthComponent,
}

impl From<PlayerEntityData> for PlayerEntityComponents {
    fn from(value: PlayerEntityData) -> Self {
        Self {
            position: value.position.into(),
            velocity: value.velocity.into(),
            health: value.health.into(),
        }
    }
}

impl EntityData for PlayerEntityData {
    const ARCHETYPE_ID: ArchetypeId = PlayerArchetype::ID;
}

impl EntityData for PlayerEntityComponents {
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

    /// Gets the `Position` component at the specified index.
    #[allow(dead_code)]
    #[inline]
    pub fn get_position_component_at(&self, index: usize) -> Option<&PositionComponent> {
        if index > self.len() {
            return None;
        }
        Some(unsafe { self.get_position_component_at_unchecked(index) })
    }

    /// Mutably gets the `Position` component at the specified index.
    #[allow(dead_code)]
    #[inline]
    pub fn get_position_component_at_mut(&mut self, index: usize) -> Option<&mut PositionComponent> {
        if index > self.len() {
            return None;
        }
        Some(unsafe { self.get_position_component_at_unchecked_mut(index) })
    }

    /// Gets the `Position` component at the specified index.
    #[allow(dead_code)]
    #[inline]
    pub unsafe fn get_position_component_at_unchecked(&self, index: usize) -> &PositionComponent {
        self.positions.get_unchecked(index)
    }

    /// Gets the `Position` component at the specified index.
    #[allow(dead_code)]
    #[inline]
    pub unsafe fn get_position_component_at_unchecked_mut(&mut self, index: usize) -> &mut PositionComponent {
        self.positions.get_unchecked_mut(index)
    }

    /// Gets the `Velocity` component at the specified index.
    #[allow(dead_code)]
    #[inline]
    pub fn get_velocity_component_at(&self, index: usize) -> Option<&VelocityComponent> {
        if index > self.len() {
            return None;
        }
        Some(unsafe { self.get_velocity_component_at_unchecked(index) })
    }

    /// Mutably gets the `Velocity` component at the specified index.
    #[allow(dead_code)]
    #[inline]
    pub fn get_velocity_component_at_mut(&mut self, index: usize) -> Option<&mut VelocityComponent> {
        if index > self.len() {
            return None;
        }
        Some(unsafe { self.get_velocity_component_at_unchecked_mut(index) })
    }

    /// Gets the `Velocity` component at the specified index.
    #[allow(dead_code)]
    #[inline]
    pub unsafe fn get_velocity_component_at_unchecked(&self, index: usize) -> &VelocityComponent {
        self.velocities.get_unchecked(index)
    }

    /// Gets the `Velocity` component at the specified index.
    #[allow(dead_code)]
    #[inline]
    pub unsafe fn get_velocity_component_at_unchecked_mut(&mut self, index: usize) -> &mut VelocityComponent {
        self.velocities.get_unchecked_mut(index)
    }

    /// Gets the `Health` component at the specified index.
    #[allow(dead_code)]
    #[inline]
    pub fn get_health_component_at(&self, index: usize) -> Option<&HealthComponent> {
        if index > self.len() {
            return None;
        }
        Some(unsafe { self.get_health_component_at_unchecked(index) })
    }

    /// Mutably gets the `Health` component at the specified index.
    #[allow(dead_code)]
    #[inline]
    pub fn get_health_component_at_mut(&mut self, index: usize) -> Option<&mut HealthComponent> {
        if index > self.len() {
            return None;
        }
        Some(unsafe { self.get_health_component_at_unchecked_mut(index) })
    }

    /// Gets the `Health` component at the specified index.
    #[allow(dead_code)]
    #[inline]
    pub unsafe fn get_health_component_at_unchecked(&self, index: usize) -> &HealthComponent {
        self.healths.get_unchecked(index)
    }

    /// Gets the `Health` component at the specified index.
    #[allow(dead_code)]
    #[inline]
    pub unsafe fn get_health_component_at_unchecked_mut(&mut self, index: usize) -> &mut HealthComponent {
        self.healths.get_unchecked_mut(index)
    }

    /// Gets the entity at the specified index.
    #[allow(dead_code)]
    pub fn get_entity_at(&self, index: usize) -> Option<PlayerEntityRef> {
        if index > self.len() {
            return None;
        }
        Some(unsafe { self.get_entity_at_unchecked(index) })
    }

    /// Mutably gets the entity at the specified index.
    #[allow(dead_code)]
    pub fn get_entity_at_mut(&mut self, index: usize) -> Option<PlayerEntityMut> {
        if index > self.len() {
            return None;
        }
        Some(unsafe { self.get_entity_at_unchecked_mut(index) })
    }

    /// Gets the entity at the specified index.
    #[allow(dead_code)]
    pub unsafe fn get_entity_at_unchecked(&self, index: usize) -> PlayerEntityRef {
        PlayerEntityRef {
            entity_id: *self.entities.get_unchecked(index),
            position: self.positions.get_unchecked(index),
            velocity: self.velocities.get_unchecked(index),
            health: self.healths.get_unchecked(index),
        }
    }

    /// Gets the entity at the specified index.
    #[allow(dead_code)]
    pub unsafe fn get_entity_at_unchecked_mut(&mut self, index: usize) -> PlayerEntityMut {
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
impl HasComponent<PositionComponent> for PlayerEntityComponents {
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
impl HasComponent<VelocityComponent> for PlayerEntityComponents {
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
impl HasComponent<HealthComponent> for PlayerEntityComponents {
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

impl PlayerEntityComponents {
    /// Spawn this entity into the given world.
    #[inline]
    #[allow(dead_code)]
    pub fn spawn_into<W>(self, world: &mut W) -> EntityId
    where
        W: Spawn<PlayerEntityComponents>
    {
        world.spawn(self)
    }
}

impl core::ops::Deref for PlayerEntity {
    type Target = PlayerEntityComponents;

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
    pub const COMPONENTS: [ComponentId; 3] = [
        ComponentId::COMPONENT_9,
        ComponentId::COMPONENT_11,
        ComponentId::COMPONENT_13,
    ];

    /// Returns the number of entities in this archetype.
    #[inline(always)]
    pub fn len(&self) -> usize {
        self.entities.len()
    }

    /// Indicates whether this archetype has no entities.
    #[inline(always)]
    #[allow(dead_code)]
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
        id == 9 || id == 11 || id == 13 || false
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
    const ID: ArchetypeId = ArchetypeId::Player;

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
pub type ForegroundObjectEntity = EntityWithIdAndData<ForegroundObjectEntityComponents>;

/// The data of an entity of the [`ForegroundObjectArchetype`].
#[derive(Debug, Clone)]
#[allow(dead_code)]
pub struct ForegroundObjectEntityData {
    pub position: PositionData,
    pub collider: ColliderData,
}

/// An entity of the [`ForegroundObjectArchetype`].
#[derive(Debug, Clone)]
#[allow(dead_code)]
pub struct ForegroundObjectEntityComponents {
    pub position: PositionComponent,
    pub collider: ColliderComponent,
}

impl From<ForegroundObjectEntityData> for ForegroundObjectEntityComponents {
    fn from(value: ForegroundObjectEntityData) -> Self {
        Self {
            position: value.position.into(),
            collider: value.collider.into(),
        }
    }
}

impl EntityData for ForegroundObjectEntityData {
    const ARCHETYPE_ID: ArchetypeId = ForegroundObjectArchetype::ID;
}

impl EntityData for ForegroundObjectEntityComponents {
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

    /// Gets the `Position` component at the specified index.
    #[allow(dead_code)]
    #[inline]
    pub fn get_position_component_at(&self, index: usize) -> Option<&PositionComponent> {
        if index > self.len() {
            return None;
        }
        Some(unsafe { self.get_position_component_at_unchecked(index) })
    }

    /// Mutably gets the `Position` component at the specified index.
    #[allow(dead_code)]
    #[inline]
    pub fn get_position_component_at_mut(&mut self, index: usize) -> Option<&mut PositionComponent> {
        if index > self.len() {
            return None;
        }
        Some(unsafe { self.get_position_component_at_unchecked_mut(index) })
    }

    /// Gets the `Position` component at the specified index.
    #[allow(dead_code)]
    #[inline]
    pub unsafe fn get_position_component_at_unchecked(&self, index: usize) -> &PositionComponent {
        self.positions.get_unchecked(index)
    }

    /// Gets the `Position` component at the specified index.
    #[allow(dead_code)]
    #[inline]
    pub unsafe fn get_position_component_at_unchecked_mut(&mut self, index: usize) -> &mut PositionComponent {
        self.positions.get_unchecked_mut(index)
    }

    /// Gets the `Collider` component at the specified index.
    #[allow(dead_code)]
    #[inline]
    pub fn get_collider_component_at(&self, index: usize) -> Option<&ColliderComponent> {
        if index > self.len() {
            return None;
        }
        Some(unsafe { self.get_collider_component_at_unchecked(index) })
    }

    /// Mutably gets the `Collider` component at the specified index.
    #[allow(dead_code)]
    #[inline]
    pub fn get_collider_component_at_mut(&mut self, index: usize) -> Option<&mut ColliderComponent> {
        if index > self.len() {
            return None;
        }
        Some(unsafe { self.get_collider_component_at_unchecked_mut(index) })
    }

    /// Gets the `Collider` component at the specified index.
    #[allow(dead_code)]
    #[inline]
    pub unsafe fn get_collider_component_at_unchecked(&self, index: usize) -> &ColliderComponent {
        self.colliders.get_unchecked(index)
    }

    /// Gets the `Collider` component at the specified index.
    #[allow(dead_code)]
    #[inline]
    pub unsafe fn get_collider_component_at_unchecked_mut(&mut self, index: usize) -> &mut ColliderComponent {
        self.colliders.get_unchecked_mut(index)
    }

    /// Gets the entity at the specified index.
    #[allow(dead_code)]
    pub fn get_entity_at(&self, index: usize) -> Option<ForegroundObjectEntityRef> {
        if index > self.len() {
            return None;
        }
        Some(unsafe { self.get_entity_at_unchecked(index) })
    }

    /// Mutably gets the entity at the specified index.
    #[allow(dead_code)]
    pub fn get_entity_at_mut(&mut self, index: usize) -> Option<ForegroundObjectEntityMut> {
        if index > self.len() {
            return None;
        }
        Some(unsafe { self.get_entity_at_unchecked_mut(index) })
    }

    /// Gets the entity at the specified index.
    #[allow(dead_code)]
    pub unsafe fn get_entity_at_unchecked(&self, index: usize) -> ForegroundObjectEntityRef {
        ForegroundObjectEntityRef {
            entity_id: *self.entities.get_unchecked(index),
            position: self.positions.get_unchecked(index),
            collider: self.colliders.get_unchecked(index),
        }
    }

    /// Gets the entity at the specified index.
    #[allow(dead_code)]
    pub unsafe fn get_entity_at_unchecked_mut(&mut self, index: usize) -> ForegroundObjectEntityMut {
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
impl HasComponent<PositionComponent> for ForegroundObjectEntityComponents {
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
impl HasComponent<ColliderComponent> for ForegroundObjectEntityComponents {
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

impl ForegroundObjectEntityComponents {
    /// Spawn this entity into the given world.
    #[inline]
    #[allow(dead_code)]
    pub fn spawn_into<W>(self, world: &mut W) -> EntityId
    where
        W: Spawn<ForegroundObjectEntityComponents>
    {
        world.spawn(self)
    }
}

impl core::ops::Deref for ForegroundObjectEntity {
    type Target = ForegroundObjectEntityComponents;

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
    pub const COMPONENTS: [ComponentId; 2] = [
        ComponentId::COMPONENT_9,
        ComponentId::COMPONENT_14,
    ];

    /// Returns the number of entities in this archetype.
    #[inline(always)]
    pub fn len(&self) -> usize {
        self.entities.len()
    }

    /// Indicates whether this archetype has no entities.
    #[inline(always)]
    #[allow(dead_code)]
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
        id == 9 || id == 14 || false
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
    const ID: ArchetypeId = ArchetypeId::ForegroundObject;

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
pub type BackgroundObjectEntity = EntityWithIdAndData<BackgroundObjectEntityComponents>;

/// The data of an entity of the [`BackgroundObjectArchetype`].
#[derive(Debug, Clone)]
#[allow(dead_code)]
pub struct BackgroundObjectEntityData {
    pub position: PositionData,
}

/// An entity of the [`BackgroundObjectArchetype`].
#[derive(Debug, Clone)]
#[allow(dead_code)]
pub struct BackgroundObjectEntityComponents {
    pub position: PositionComponent,
}

impl From<BackgroundObjectEntityData> for BackgroundObjectEntityComponents {
    fn from(value: BackgroundObjectEntityData) -> Self {
        Self {
            position: value.position.into(),
        }
    }
}

impl EntityData for BackgroundObjectEntityData {
    const ARCHETYPE_ID: ArchetypeId = BackgroundObjectArchetype::ID;
}

impl EntityData for BackgroundObjectEntityComponents {
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

    /// Gets the `Position` component at the specified index.
    #[allow(dead_code)]
    #[inline]
    pub fn get_position_component_at(&self, index: usize) -> Option<&PositionComponent> {
        if index > self.len() {
            return None;
        }
        Some(unsafe { self.get_position_component_at_unchecked(index) })
    }

    /// Mutably gets the `Position` component at the specified index.
    #[allow(dead_code)]
    #[inline]
    pub fn get_position_component_at_mut(&mut self, index: usize) -> Option<&mut PositionComponent> {
        if index > self.len() {
            return None;
        }
        Some(unsafe { self.get_position_component_at_unchecked_mut(index) })
    }

    /// Gets the `Position` component at the specified index.
    #[allow(dead_code)]
    #[inline]
    pub unsafe fn get_position_component_at_unchecked(&self, index: usize) -> &PositionComponent {
        self.positions.get_unchecked(index)
    }

    /// Gets the `Position` component at the specified index.
    #[allow(dead_code)]
    #[inline]
    pub unsafe fn get_position_component_at_unchecked_mut(&mut self, index: usize) -> &mut PositionComponent {
        self.positions.get_unchecked_mut(index)
    }

    /// Gets the entity at the specified index.
    #[allow(dead_code)]
    pub fn get_entity_at(&self, index: usize) -> Option<BackgroundObjectEntityRef> {
        if index > self.len() {
            return None;
        }
        Some(unsafe { self.get_entity_at_unchecked(index) })
    }

    /// Mutably gets the entity at the specified index.
    #[allow(dead_code)]
    pub fn get_entity_at_mut(&mut self, index: usize) -> Option<BackgroundObjectEntityMut> {
        if index > self.len() {
            return None;
        }
        Some(unsafe { self.get_entity_at_unchecked_mut(index) })
    }

    /// Gets the entity at the specified index.
    #[allow(dead_code)]
    pub unsafe fn get_entity_at_unchecked(&self, index: usize) -> BackgroundObjectEntityRef {
        BackgroundObjectEntityRef {
            entity_id: *self.entities.get_unchecked(index),
            position: self.positions.get_unchecked(index),
        }
    }

    /// Gets the entity at the specified index.
    #[allow(dead_code)]
    pub unsafe fn get_entity_at_unchecked_mut(&mut self, index: usize) -> BackgroundObjectEntityMut {
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
impl HasComponent<PositionComponent> for BackgroundObjectEntityComponents {
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

impl BackgroundObjectEntityComponents {
    /// Spawn this entity into the given world.
    #[inline]
    #[allow(dead_code)]
    pub fn spawn_into<W>(self, world: &mut W) -> EntityId
    where
        W: Spawn<BackgroundObjectEntityComponents>
    {
        world.spawn(self)
    }
}

impl core::ops::Deref for BackgroundObjectEntity {
    type Target = BackgroundObjectEntityComponents;

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
    pub const COMPONENTS: [ComponentId; 1] = [
        ComponentId::COMPONENT_9,
    ];

    /// Returns the number of entities in this archetype.
    #[inline(always)]
    pub fn len(&self) -> usize {
        self.entities.len()
    }

    /// Indicates whether this archetype has no entities.
    #[inline(always)]
    #[allow(dead_code)]
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
        id == 9 || false
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
    const ID: ArchetypeId = ArchetypeId::BackgroundObject;

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
pub type SceneBackgroundEntity = EntityWithIdAndData<SceneBackgroundEntityComponents>;

/// The data of an entity of the [`SceneBackgroundArchetype`].
#[derive(Debug, Clone)]
#[allow(dead_code)]
pub struct SceneBackgroundEntityData {
    pub background_color: BackgroundColorData,
}

/// An entity of the [`SceneBackgroundArchetype`].
#[derive(Debug, Clone)]
#[allow(dead_code)]
pub struct SceneBackgroundEntityComponents {
    pub background_color: BackgroundColorComponent,
}

impl From<SceneBackgroundEntityData> for SceneBackgroundEntityComponents {
    fn from(value: SceneBackgroundEntityData) -> Self {
        Self {
            background_color: value.background_color.into(),
        }
    }
}

impl EntityData for SceneBackgroundEntityData {
    const ARCHETYPE_ID: ArchetypeId = SceneBackgroundArchetype::ID;
}

impl EntityData for SceneBackgroundEntityComponents {
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

    /// Gets the `BackgroundColor` component at the specified index.
    #[allow(dead_code)]
    #[inline]
    pub fn get_background_color_component_at(&self, index: usize) -> Option<&BackgroundColorComponent> {
        if index > self.len() {
            return None;
        }
        Some(unsafe { self.get_background_color_component_at_unchecked(index) })
    }

    /// Mutably gets the `BackgroundColor` component at the specified index.
    #[allow(dead_code)]
    #[inline]
    pub fn get_background_color_component_at_mut(&mut self, index: usize) -> Option<&mut BackgroundColorComponent> {
        if index > self.len() {
            return None;
        }
        Some(unsafe { self.get_background_color_component_at_unchecked_mut(index) })
    }

    /// Gets the `BackgroundColor` component at the specified index.
    #[allow(dead_code)]
    #[inline]
    pub unsafe fn get_background_color_component_at_unchecked(&self, index: usize) -> &BackgroundColorComponent {
        self.background_colors.get_unchecked(index)
    }

    /// Gets the `BackgroundColor` component at the specified index.
    #[allow(dead_code)]
    #[inline]
    pub unsafe fn get_background_color_component_at_unchecked_mut(&mut self, index: usize) -> &mut BackgroundColorComponent {
        self.background_colors.get_unchecked_mut(index)
    }

    /// Gets the entity at the specified index.
    #[allow(dead_code)]
    pub fn get_entity_at(&self, index: usize) -> Option<SceneBackgroundEntityRef> {
        if index > self.len() {
            return None;
        }
        Some(unsafe { self.get_entity_at_unchecked(index) })
    }

    /// Mutably gets the entity at the specified index.
    #[allow(dead_code)]
    pub fn get_entity_at_mut(&mut self, index: usize) -> Option<SceneBackgroundEntityMut> {
        if index > self.len() {
            return None;
        }
        Some(unsafe { self.get_entity_at_unchecked_mut(index) })
    }

    /// Gets the entity at the specified index.
    #[allow(dead_code)]
    pub unsafe fn get_entity_at_unchecked(&self, index: usize) -> SceneBackgroundEntityRef {
        SceneBackgroundEntityRef {
            entity_id: *self.entities.get_unchecked(index),
            background_color: self.background_colors.get_unchecked(index),
        }
    }

    /// Gets the entity at the specified index.
    #[allow(dead_code)]
    pub unsafe fn get_entity_at_unchecked_mut(&mut self, index: usize) -> SceneBackgroundEntityMut {
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
impl HasComponent<BackgroundColorComponent> for SceneBackgroundEntityComponents {
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

impl SceneBackgroundEntityComponents {
    /// Spawn this entity into the given world.
    #[inline]
    #[allow(dead_code)]
    pub fn spawn_into<W>(self, world: &mut W) -> EntityId
    where
        W: Spawn<SceneBackgroundEntityComponents>
    {
        world.spawn(self)
    }
}

impl core::ops::Deref for SceneBackgroundEntity {
    type Target = SceneBackgroundEntityComponents;

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
    pub const COMPONENTS: [ComponentId; 1] = [
        ComponentId::COMPONENT_17,
    ];

    /// Returns the number of entities in this archetype.
    #[inline(always)]
    pub fn len(&self) -> usize {
        self.entities.len()
    }

    /// Indicates whether this archetype has no entities.
    #[inline(always)]
    #[allow(dead_code)]
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
        id == 17 || false
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
    const ID: ArchetypeId = ArchetypeId::SceneBackground;

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

    #[allow(dead_code)]
    fn is_empty(&self) -> bool;

    #[allow(clippy::needless_lifetimes, dead_code)]
    fn get_at<'archetype>(&'archetype self, index: usize) -> Option<Self::EntityRef<'archetype>>;

    #[allow(clippy::needless_lifetimes, dead_code)]
    unsafe fn get_at_unchecked<'archetype>(&'archetype self, index: usize) -> Self::EntityRef<'archetype>;
}

impl core::ops::Index<usize> for WgpuShaderArchetype {
    type Output = EntityId;

    fn index(&self, index: usize) -> &Self::Output {
        &self.entities[index]
    }
}

impl core::ops::Index<usize> for WgpuTextureArchetype {
    type Output = EntityId;

    fn index(&self, index: usize) -> &Self::Output {
        &self.entities[index]
    }
}

impl core::ops::Index<usize> for EnvironmentArchetype {
    type Output = EntityId;

    fn index(&self, index: usize) -> &Self::Output {
        &self.entities[index]
    }
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

impl core::ops::Index<usize> for TileArchetype {
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
        if Archetype::is_empty(self) {
            return 0;
        }

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

impl GetEntityRef for WgpuShaderArchetype {
    type EntityRef<'archetype> = WgpuShaderEntityRef<'archetype>;

    #[allow(dead_code)]
    fn is_empty(&self) -> bool {
       self.is_empty()
    }

    /// See [`WgpuShaderArchetype::get_at`](WgpuShaderArchetype::get_at).
    #[inline(always)]
    #[allow(clippy::needless_lifetimes, dead_code)]
    fn get_at<'archetype>(&'archetype self, index: usize) -> Option<Self::EntityRef<'archetype>> {
        self.get_entity_at(index)
    }

    /// See [`WgpuShaderArchetype::get_at_unchecked`](WgpuShaderArchetype::get_at_unchecked).
    #[inline(always)]
    #[allow(clippy::needless_lifetimes, dead_code)]
    unsafe fn get_at_unchecked<'archetype>(&'archetype self, index: usize) -> Self::EntityRef<'archetype> {
        unsafe { self.get_entity_at_unchecked(index) }
    }
}

impl GetEntityRef for WgpuTextureArchetype {
    type EntityRef<'archetype> = WgpuTextureEntityRef<'archetype>;

    #[allow(dead_code)]
    fn is_empty(&self) -> bool {
       self.is_empty()
    }

    /// See [`WgpuTextureArchetype::get_at`](WgpuTextureArchetype::get_at).
    #[inline(always)]
    #[allow(clippy::needless_lifetimes, dead_code)]
    fn get_at<'archetype>(&'archetype self, index: usize) -> Option<Self::EntityRef<'archetype>> {
        self.get_entity_at(index)
    }

    /// See [`WgpuTextureArchetype::get_at_unchecked`](WgpuTextureArchetype::get_at_unchecked).
    #[inline(always)]
    #[allow(clippy::needless_lifetimes, dead_code)]
    unsafe fn get_at_unchecked<'archetype>(&'archetype self, index: usize) -> Self::EntityRef<'archetype> {
        unsafe { self.get_entity_at_unchecked(index) }
    }
}

impl GetEntityRef for EnvironmentArchetype {
    type EntityRef<'archetype> = EnvironmentEntityRef<'archetype>;

    #[allow(dead_code)]
    fn is_empty(&self) -> bool {
       self.is_empty()
    }

    /// See [`EnvironmentArchetype::get_at`](EnvironmentArchetype::get_at).
    #[inline(always)]
    #[allow(clippy::needless_lifetimes, dead_code)]
    fn get_at<'archetype>(&'archetype self, index: usize) -> Option<Self::EntityRef<'archetype>> {
        self.get_entity_at(index)
    }

    /// See [`EnvironmentArchetype::get_at_unchecked`](EnvironmentArchetype::get_at_unchecked).
    #[inline(always)]
    #[allow(clippy::needless_lifetimes, dead_code)]
    unsafe fn get_at_unchecked<'archetype>(&'archetype self, index: usize) -> Self::EntityRef<'archetype> {
        unsafe { self.get_entity_at_unchecked(index) }
    }
}

impl GetEntityRef for CameraArchetype {
    type EntityRef<'archetype> = CameraEntityRef<'archetype>;

    #[allow(dead_code)]
    fn is_empty(&self) -> bool {
       self.is_empty()
    }

    /// See [`CameraArchetype::get_at`](CameraArchetype::get_at).
    #[inline(always)]
    #[allow(clippy::needless_lifetimes, dead_code)]
    fn get_at<'archetype>(&'archetype self, index: usize) -> Option<Self::EntityRef<'archetype>> {
        self.get_entity_at(index)
    }

    /// See [`CameraArchetype::get_at_unchecked`](CameraArchetype::get_at_unchecked).
    #[inline(always)]
    #[allow(clippy::needless_lifetimes, dead_code)]
    unsafe fn get_at_unchecked<'archetype>(&'archetype self, index: usize) -> Self::EntityRef<'archetype> {
        unsafe { self.get_entity_at_unchecked(index) }
    }
}

impl GetEntityRef for DynamicLightArchetype {
    type EntityRef<'archetype> = DynamicLightEntityRef<'archetype>;

    #[allow(dead_code)]
    fn is_empty(&self) -> bool {
       self.is_empty()
    }

    /// See [`DynamicLightArchetype::get_at`](DynamicLightArchetype::get_at).
    #[inline(always)]
    #[allow(clippy::needless_lifetimes, dead_code)]
    fn get_at<'archetype>(&'archetype self, index: usize) -> Option<Self::EntityRef<'archetype>> {
        self.get_entity_at(index)
    }

    /// See [`DynamicLightArchetype::get_at_unchecked`](DynamicLightArchetype::get_at_unchecked).
    #[inline(always)]
    #[allow(clippy::needless_lifetimes, dead_code)]
    unsafe fn get_at_unchecked<'archetype>(&'archetype self, index: usize) -> Self::EntityRef<'archetype> {
        unsafe { self.get_entity_at_unchecked(index) }
    }
}

impl GetEntityRef for TileArchetype {
    type EntityRef<'archetype> = TileEntityRef<'archetype>;

    #[allow(dead_code)]
    fn is_empty(&self) -> bool {
       self.is_empty()
    }

    /// See [`TileArchetype::get_at`](TileArchetype::get_at).
    #[inline(always)]
    #[allow(clippy::needless_lifetimes, dead_code)]
    fn get_at<'archetype>(&'archetype self, index: usize) -> Option<Self::EntityRef<'archetype>> {
        self.get_entity_at(index)
    }

    /// See [`TileArchetype::get_at_unchecked`](TileArchetype::get_at_unchecked).
    #[inline(always)]
    #[allow(clippy::needless_lifetimes, dead_code)]
    unsafe fn get_at_unchecked<'archetype>(&'archetype self, index: usize) -> Self::EntityRef<'archetype> {
        unsafe { self.get_entity_at_unchecked(index) }
    }
}

impl GetEntityRef for CubeArchetype {
    type EntityRef<'archetype> = CubeEntityRef<'archetype>;

    #[allow(dead_code)]
    fn is_empty(&self) -> bool {
       self.is_empty()
    }

    /// See [`CubeArchetype::get_at`](CubeArchetype::get_at).
    #[inline(always)]
    #[allow(clippy::needless_lifetimes, dead_code)]
    fn get_at<'archetype>(&'archetype self, index: usize) -> Option<Self::EntityRef<'archetype>> {
        self.get_entity_at(index)
    }

    /// See [`CubeArchetype::get_at_unchecked`](CubeArchetype::get_at_unchecked).
    #[inline(always)]
    #[allow(clippy::needless_lifetimes, dead_code)]
    unsafe fn get_at_unchecked<'archetype>(&'archetype self, index: usize) -> Self::EntityRef<'archetype> {
        unsafe { self.get_entity_at_unchecked(index) }
    }
}

impl GetEntityRef for PlayerArchetype {
    type EntityRef<'archetype> = PlayerEntityRef<'archetype>;

    #[allow(dead_code)]
    fn is_empty(&self) -> bool {
       self.is_empty()
    }

    /// See [`PlayerArchetype::get_at`](PlayerArchetype::get_at).
    #[inline(always)]
    #[allow(clippy::needless_lifetimes, dead_code)]
    fn get_at<'archetype>(&'archetype self, index: usize) -> Option<Self::EntityRef<'archetype>> {
        self.get_entity_at(index)
    }

    /// See [`PlayerArchetype::get_at_unchecked`](PlayerArchetype::get_at_unchecked).
    #[inline(always)]
    #[allow(clippy::needless_lifetimes, dead_code)]
    unsafe fn get_at_unchecked<'archetype>(&'archetype self, index: usize) -> Self::EntityRef<'archetype> {
        unsafe { self.get_entity_at_unchecked(index) }
    }
}

impl GetEntityRef for ForegroundObjectArchetype {
    type EntityRef<'archetype> = ForegroundObjectEntityRef<'archetype>;

    #[allow(dead_code)]
    fn is_empty(&self) -> bool {
       self.is_empty()
    }

    /// See [`ForegroundObjectArchetype::get_at`](ForegroundObjectArchetype::get_at).
    #[inline(always)]
    #[allow(clippy::needless_lifetimes, dead_code)]
    fn get_at<'archetype>(&'archetype self, index: usize) -> Option<Self::EntityRef<'archetype>> {
        self.get_entity_at(index)
    }

    /// See [`ForegroundObjectArchetype::get_at_unchecked`](ForegroundObjectArchetype::get_at_unchecked).
    #[inline(always)]
    #[allow(clippy::needless_lifetimes, dead_code)]
    unsafe fn get_at_unchecked<'archetype>(&'archetype self, index: usize) -> Self::EntityRef<'archetype> {
        unsafe { self.get_entity_at_unchecked(index) }
    }
}

impl GetEntityRef for BackgroundObjectArchetype {
    type EntityRef<'archetype> = BackgroundObjectEntityRef<'archetype>;

    #[allow(dead_code)]
    fn is_empty(&self) -> bool {
       self.is_empty()
    }

    /// See [`BackgroundObjectArchetype::get_at`](BackgroundObjectArchetype::get_at).
    #[inline(always)]
    #[allow(clippy::needless_lifetimes, dead_code)]
    fn get_at<'archetype>(&'archetype self, index: usize) -> Option<Self::EntityRef<'archetype>> {
        self.get_entity_at(index)
    }

    /// See [`BackgroundObjectArchetype::get_at_unchecked`](BackgroundObjectArchetype::get_at_unchecked).
    #[inline(always)]
    #[allow(clippy::needless_lifetimes, dead_code)]
    unsafe fn get_at_unchecked<'archetype>(&'archetype self, index: usize) -> Self::EntityRef<'archetype> {
        unsafe { self.get_entity_at_unchecked(index) }
    }
}

impl GetEntityRef for SceneBackgroundArchetype {
    type EntityRef<'archetype> = SceneBackgroundEntityRef<'archetype>;

    #[allow(dead_code)]
    fn is_empty(&self) -> bool {
       self.is_empty()
    }

    /// See [`SceneBackgroundArchetype::get_at`](SceneBackgroundArchetype::get_at).
    #[inline(always)]
    #[allow(clippy::needless_lifetimes, dead_code)]
    fn get_at<'archetype>(&'archetype self, index: usize) -> Option<Self::EntityRef<'archetype>> {
        self.get_entity_at(index)
    }

    /// See [`SceneBackgroundArchetype::get_at_unchecked`](SceneBackgroundArchetype::get_at_unchecked).
    #[inline(always)]
    #[allow(clippy::needless_lifetimes, dead_code)]
    unsafe fn get_at_unchecked<'archetype>(&'archetype self, index: usize) -> Self::EntityRef<'archetype> {
        unsafe { self.get_entity_at_unchecked(index) }
    }
}

impl FrontloadEntities for WgpuShaderArchetype {
    #[inline(always)]
    #[doc(hidden)]
    fn swap_entities(&mut self, first: usize, second: usize) {
        self.entities.swap(first, second);
        self.wgpu_shaders.swap(first, second);
    }
}

impl FrontloadEntitiesScan for WgpuShaderArchetype { }

impl FrontloadEntities for WgpuTextureArchetype {
    #[inline(always)]
    #[doc(hidden)]
    fn swap_entities(&mut self, first: usize, second: usize) {
        self.entities.swap(first, second);
        self.texture_sources.swap(first, second);
        self.wgpu_textures.swap(first, second);
    }
}

impl FrontloadEntitiesScan for WgpuTextureArchetype { }

impl FrontloadEntities for EnvironmentArchetype {
    #[inline(always)]
    #[doc(hidden)]
    fn swap_entities(&mut self, first: usize, second: usize) {
        self.entities.swap(first, second);
        self.time_of_days.swap(first, second);
        self.suns.swap(first, second);
        self.skydomes.swap(first, second);
        self.ambient_lights.swap(first, second);
        self.stars.swap(first, second);
        self.fogs.swap(first, second);
    }
}

impl FrontloadEntitiesScan for EnvironmentArchetype { }

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

impl FrontloadEntities for TileArchetype {
    #[inline(always)]
    #[doc(hidden)]
    fn swap_entities(&mut self, first: usize, second: usize) {
        self.entities.swap(first, second);
        self.tiles.swap(first, second);
        self.positions.swap(first, second);
    }
}

impl FrontloadEntitiesScan for TileArchetype { }

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