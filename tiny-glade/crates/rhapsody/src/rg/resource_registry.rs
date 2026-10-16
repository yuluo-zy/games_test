// 候选实现迁移来源：reconstruction/render/work-packages/registry-pool/src/lib.rs.
// 这是行为重建，不是原作者源码。
use std::collections::{HashMap, HashSet};

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
    pub fn take_texture(&mut self, desc: &TextureDesc) -> Option<Texture<TextureHandle>> {
        self.textures.get_mut(desc).and_then(Vec::pop)
    }

    pub fn insert_texture(&mut self, texture: Texture<TextureHandle>) {
        self.textures.entry(texture.desc).or_default().push(texture);
    }

    pub fn take_buffer(&mut self, desc: &BufferDesc) -> Option<Buffer<BufferHandle>> {
        self.buffers.get_mut(desc).and_then(Vec::pop)
    }

    pub fn insert_buffer(&mut self, buffer: Buffer<BufferHandle>) {
        self.buffers.entry(buffer.desc).or_default().push(buffer);
    }

    pub fn clear(&mut self) {
        self.textures.clear();
        self.buffers.clear();
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
    pub exported_logical_handles: HashSet<u16>,
}

pub struct ResourceExports<TextureHandle, BufferHandle, Other> {
    pub remaining: Vec<RegistryResource<TextureHandle, BufferHandle, Other>>,
    pub exported: HashMap<u16, RegistryResource<TextureHandle, BufferHandle, Other>>,
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

    pub fn image_from_raw_handle(&self, logical: u32) -> &Texture<TextureHandle> {
        match self.backing(logical) {
            RegistryResource::OwnedTexture(texture) | RegistryResource::UnownedTexture(texture) => {
                texture
            }
            _ => panic!("resource is not a texture"),
        }
    }

    /// 原入口名；与 `image_from_raw_handle` 同一实现，保留原名供消费者按 PDB 名字调用。
    pub fn image(&self, logical: u32) -> &Texture<TextureHandle> {
        self.image_from_raw_handle(logical)
    }

    /// 原入口按 managed 视图解析；本通用 registry 只持有 `Texture { handle, desc }`，
    /// 不拥有独立的 image-view 载体。返回纹理 handle 作为可解析的物理对象，
    /// 视图创建仍由 raw 层 `TextureViewDesc`/后端完成。
    pub fn image_view(&self, logical: u32) -> &Texture<TextureHandle> {
        self.image_from_raw_handle(logical)
    }

    pub fn buffer(&self, logical: u32) -> &Buffer<BufferHandle> {
        match self.backing(logical) {
            RegistryResource::OwnedBuffer(buffer) | RegistryResource::UnownedBuffer(buffer) => {
                buffer
            }
            _ => panic!("resource is not a buffer"),
        }
    }

    pub fn into_exports(mut self) -> ResourceExports<TextureHandle, BufferHandle, Other> {
        let exported = self
            .exported_logical_handles
            .into_iter()
            .filter_map(|logical| {
                // 未填充的导出资源没有物理槽可取（执行器已以 ResourceUnavailable 拒绝该图）。
                let physical = self.resources.get(usize::from(logical))?.backing_index;
                let slot = self.backing_resources.get_mut(usize::from(physical))?;
                Some((logical, std::mem::replace(slot, RegistryResource::Empty)))
            })
            .collect();
        self.backing_resources
            .retain(|resource| !matches!(resource, RegistryResource::Empty));
        ResourceExports {
            remaining: self.backing_resources,
            exported,
        }
    }
}
