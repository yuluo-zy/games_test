use std::collections::HashMap;

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
#[repr(C)]
pub struct TextureDesc {
    pub words_at_00_through_18: [u32; 7],
    pub bytes_at_1c_through_1e: [u8; 3],
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
#[repr(C)]
pub struct BufferDesc {
    pub word_at_00: u64,
    pub word_at_08: u32,
    pub bytes_at_0c_through_0e: [u8; 3],
}

#[derive(Debug, PartialEq, Eq)]
pub struct Texture<Handle> {
    pub handle: Handle,
    pub desc: TextureDesc,
}

#[derive(Debug, PartialEq, Eq)]
pub struct Buffer<Handle> {
    pub handle: Handle,
    pub desc: BufferDesc,
}

pub struct ResourcePool<TextureHandle, BufferHandle> {
    textures: HashMap<TextureDesc, Vec<Texture<TextureHandle>>>,
    buffers: HashMap<BufferDesc, Vec<Buffer<BufferHandle>>>,
}

impl<TextureHandle, BufferHandle> Default for ResourcePool<TextureHandle, BufferHandle> {
    fn default() -> Self {
        Self {
            textures: HashMap::new(),
            buffers: HashMap::new(),
        }
    }
}

impl<TextureHandle, BufferHandle> ResourcePool<TextureHandle, BufferHandle> {
    /// Move only idle pool entries out; in-flight frame registries are separate.
    pub fn drain(&mut self) -> (Vec<Texture<TextureHandle>>, Vec<Buffer<BufferHandle>>) {
        let textures = self
            .textures
            .drain()
            .flat_map(|(_, entries)| entries)
            .collect();
        let buffers = self
            .buffers
            .drain()
            .flat_map(|(_, entries)| entries)
            .collect();
        (textures, buffers)
    }
    pub fn take_texture_where(
        &mut self,
        desc: &TextureDesc,
        mut accepts: impl FnMut(&TextureHandle) -> bool,
    ) -> Option<Texture<TextureHandle>> {
        let entries = self.textures.get_mut(desc)?;
        let index = entries.iter().rposition(|entry| accepts(&entry.handle))?;
        let found = entries.swap_remove(index);
        let empty = entries.is_empty();
        if empty { self.textures.remove(desc); }
        Some(found)
    }
    pub fn take_buffer_where(
        &mut self,
        desc: &BufferDesc,
        mut accepts: impl FnMut(&BufferHandle) -> bool,
    ) -> Option<Buffer<BufferHandle>> {
        let entries = self.buffers.get_mut(desc)?;
        let index = entries.iter().rposition(|entry| accepts(&entry.handle))?;
        let found = entries.swap_remove(index);
        let empty = entries.is_empty();
        if empty { self.buffers.remove(desc); }
        Some(found)
    }

    pub fn insert_texture(&mut self, texture: Texture<TextureHandle>) {
        self.textures.entry(texture.desc).or_default().push(texture);
    }

    pub fn take_buffer(&mut self, desc: &BufferDesc) -> Option<Buffer<BufferHandle>> {
        self.take_buffer_where(desc, |_| true)
    }

    pub fn insert_buffer(&mut self, buffer: Buffer<BufferHandle>) {
        self.buffers.entry(buffer.desc).or_default().push(buffer);
    }

    pub fn clear(&mut self) {
        self.textures.clear();
        self.buffers.clear();
    }
    /// Pooled entries.
    pub fn len(&self) -> usize {
        self.textures.values().map(Vec::len).sum::<usize>()
            + self.buffers.values().map(Vec::len).sum::<usize>()
    }
    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }
}
impl<TextureHandle: PartialEq, BufferHandle: PartialEq> ResourcePool<TextureHandle, BufferHandle> {
    /// Forget entries whose resource the backend already released.
    pub fn forget_handles(&mut self, textures: &[TextureHandle], buffers: &[BufferHandle]) {
        for entries in self.textures.values_mut() {
            entries.retain(|entry| !textures.contains(&entry.handle));
        }
        for entries in self.buffers.values_mut() {
            entries.retain(|entry| !buffers.contains(&entry.handle));
        }
        self.textures.retain(|_, entries| !entries.is_empty());
        self.buffers.retain(|_, entries| !entries.is_empty());
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(C)]
pub struct ResourceReuseInfo {
    pub backing_index: u16,
    pub reused: bool,
}

impl Default for ResourceReuseInfo {
    fn default() -> Self {
        Self {
            backing_index: u16::MAX,
            reused: false,
        }
    }
}

#[derive(Debug, PartialEq, Eq)]
pub enum RegistryResource<TextureHandle, BufferHandle, Other> {
    OwnedTexture(Texture<TextureHandle>),
    UnownedTexture(Texture<TextureHandle>),
    OwnedBuffer(Buffer<BufferHandle>),
    UnownedBuffer(Buffer<BufferHandle>),
    Other(Other),
    Empty,
}

pub struct ResourceRegistry<TextureHandle, BufferHandle, Other> {
    pub resources: Vec<ResourceReuseInfo>,
    pub resource_initial_access: Vec<u8>,
    pub backing_resources: Vec<RegistryResource<TextureHandle, BufferHandle, Other>>,
    pub exported_logical_handles: Vec<u16>,
}

impl<TextureHandle, BufferHandle, Other> Default
    for ResourceRegistry<TextureHandle, BufferHandle, Other>
{
    fn default() -> Self {
        Self {
            resources: Vec::new(),
            resource_initial_access: Vec::new(),
            backing_resources: Vec::new(),
            exported_logical_handles: Vec::new(),
        }
    }
}


impl<TextureHandle, BufferHandle, Other> ResourceRegistry<TextureHandle, BufferHandle, Other> {
    /// 逻辑资源是否已由 population 分配物理槽；未出现在生命周期事件里的资源（如只导出、
    /// 从未被访问的 Create 资源）保持 `backing_index = u16::MAX`，此时 `resource()` 等访问器会越界。
    pub fn is_populated(&self, logical: u32) -> bool {
        self.resources
            .get(logical as usize)
            .is_some_and(|reuse| usize::from(reuse.backing_index) < self.backing_resources.len())
    }

    fn backing(&self, logical: u32) -> &RegistryResource<TextureHandle, BufferHandle, Other> {
        let reuse = self.resources[logical as usize];
        &self.backing_resources[usize::from(reuse.backing_index)]
    }

    /// Access a populated physical resource after graph preparation.
    ///
    /// # Panics
    /// Panics for an invalid logical handle or a resource not populated by
    /// this frame. Use `is_populated` when inspecting a culled declaration.
    pub fn resource(
        &self,
        logical: u32,
    ) -> (
        &RegistryResource<TextureHandle, BufferHandle, Other>,
        u8,
        bool,
    ) {
        let backing = self.backing(logical);
        let initial_access = self.resource_initial_access[logical as usize];
        (
            backing,
            initial_access,
            self.resources[logical as usize].reused,
        )
    }

    pub fn image(&self, logical: u32) -> &Texture<TextureHandle> {
        match self.backing(logical) {
            RegistryResource::OwnedTexture(texture) | RegistryResource::UnownedTexture(texture) => {
                texture
            }
            _ => panic!("resource is not a texture"),
        }
    }

    pub fn buffer(&self, logical: u32) -> &Buffer<BufferHandle> {
        match self.backing(logical) {
            RegistryResource::OwnedBuffer(buffer) | RegistryResource::UnownedBuffer(buffer) => {
                buffer
            }
            _ => panic!("resource is not a buffer"),
        }
    }

}
