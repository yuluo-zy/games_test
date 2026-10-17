//! 逻辑资源接口：依据 PDB 函数名及汇编重建，公开 Rust 签名仍是候选。
use super::graph::{Descriptor, OriginSource};

/// A resource index only has meaning within the graph that allocated it.
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct GraphId(u64);
impl GraphId {
    pub(crate) fn next() -> Self {
        use std::sync::atomic::{AtomicU64, Ordering};
        static NEXT: AtomicU64 = AtomicU64::new(1);
        Self(NEXT.fetch_add(1, Ordering::Relaxed))
    }
    pub const fn raw(self) -> u64 {
        self.0
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct ResourceHandle {
    graph: GraphId,
    index: u32,
}
impl ResourceHandle {
    /// Historical index projection. It is deliberately not a live graph handle.
    /// Use `RenderGraph::resource_handle` to resolve an index at a legacy boundary.
    pub const fn from_raw(raw: u32) -> Self {
        Self {
            graph: GraphId(0),
            index: raw,
        }
    }
    pub(crate) const fn scoped(graph: GraphId, index: u32) -> Self {
        Self { graph, index }
    }
    pub const fn graph_id(self) -> GraphId {
        self.graph
    }
    pub const fn raw(self) -> u32 {
        self.index
    }
}

macro_rules! typed_resource {
    ($name:ident) => {
        #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
        pub struct $name {
            pub(crate) handle: ResourceHandle,
            pub(crate) version: u32,
        }
        impl $name {
            pub const fn raw(self) -> ResourceHandle {
                self.handle
            }
            pub const fn version(self) -> u32 {
                self.version
            }
        }
    };
}
typed_resource!(TextureId);
typed_resource!(BufferId);

impl super::graph::RenderGraph {
    pub fn create_texture(
        &mut self,
        name: impl Into<String>,
        descriptor: super::graph::TextureDesc,
    ) -> Result<TextureId, super::graph::BuildError> {
        let handle = self.create_raw_resource(
            name,
            ResourceOrigin {
                descriptor: Descriptor::Image(descriptor),
                source: OriginSource::Create,
            },
        )?;
        self.texture_id(handle)
    }
    pub fn create_buffer(
        &mut self,
        name: impl Into<String>,
        descriptor: super::graph::BufferDesc,
    ) -> Result<BufferId, super::graph::BuildError> {
        let handle = self.create_raw_resource(
            name,
            ResourceOrigin {
                descriptor: Descriptor::Buffer(descriptor),
                source: OriginSource::Create,
            },
        )?;
        self.buffer_id(handle)
    }
    pub fn texture_id(
        &self,
        handle: ResourceHandle,
    ) -> Result<TextureId, super::graph::BuildError> {
        self.check_handle(handle)?;
        if !matches!(
            self.declaration.resources[handle.raw() as usize].descriptor,
            Descriptor::Image(_)
        ) {
            return Err(super::graph::BuildError::ResourceKindMismatch(handle.raw()));
        }
        Ok(TextureId {
            handle,
            version: self.current_version(handle)?,
        })
    }
    pub fn buffer_id(&self, handle: ResourceHandle) -> Result<BufferId, super::graph::BuildError> {
        self.check_handle(handle)?;
        if !matches!(
            self.declaration.resources[handle.raw() as usize].descriptor,
            Descriptor::Buffer(_)
        ) {
            return Err(super::graph::BuildError::ResourceKindMismatch(handle.raw()));
        }
        Ok(BufferId {
            handle,
            version: self.current_version(handle)?,
        })
    }
    pub fn export_buffer_range(
        &mut self,
        buffer: BufferId,
        access_code: u8,
        range: AccessRange,
    ) -> Result<(), super::graph::BuildError> {
        let current = self.buffer_id(buffer.raw())?;
        self.check_export_version(buffer.raw(), current.version(), buffer.version())?;
        self.export_range(buffer.raw(), access_code, range)
    }
    pub fn export_texture_range(
        &mut self,
        texture: TextureId,
        access_code: u8,
        range: AccessRange,
    ) -> Result<(), super::graph::BuildError> {
        let current = self.texture_id(texture.raw())?;
        self.check_export_version(texture.raw(), current.version(), texture.version())?;
        self.export_range(texture.raw(), access_code, range)
    }
    fn check_export_version(
        &self,
        resource: ResourceHandle,
        expected: u32,
        actual: u32,
    ) -> Result<(), super::graph::BuildError> {
        if expected != actual {
            return Err(super::graph::BuildError::Contract(
                ResourceContractError::StaleVersion {
                    resource: resource.raw(),
                    expected,
                    actual,
                },
            ));
        }
        Ok(())
    }
    pub(crate) fn current_version(
        &self,
        handle: ResourceHandle,
    ) -> Result<u32, super::graph::BuildError> {
        self.check_handle(handle)?;
        let mut version = 0u32;
        for access in self
            .declaration
            .passes
            .iter()
            .flat_map(|pass| &pass.accesses)
            .filter(|access| access.resource_index == handle.raw())
        {
            match access.version {
                AccessVersion::Write { current, .. } => version = current,
                AccessVersion::Automatic
                    if crate::raw::access_table::is_write(access.access_code) =>
                {
                    version = version
                        .checked_add(1)
                        .ok_or(super::graph::BuildError::Contract(
                            ResourceContractError::VersionOverflow(handle.raw()),
                        ))?;
                }
                _ => {}
            }
        }
        Ok(version)
    }
}

/// `Whole` is resolved using the declaration descriptor, never Vulkan handles.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Hash)]
pub enum AccessRange {
    #[default]
    Whole,
    Buffer {
        offset: u64,
        size: u64,
    },
    Texture {
        aspect_mask: u32,
        base_mip_level: u32,
        level_count: u32,
        base_array_layer: u32,
        layer_count: u32,
    },
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RangeError {
    WrongKind,
    InvalidBounds,
}

impl AccessRange {
    pub fn validate(self, descriptor: Descriptor) -> Result<(), RangeError> {
        if self == Self::Whole {
            return Ok(());
        }
        let whole = Self::Whole.normalized(descriptor)?;
        match (self, whole) {
            (Self::Buffer { offset, size }, Self::Buffer { size: limit, .. }) => {
                if size != 0 && offset.checked_add(size).is_some_and(|end| end <= limit) {
                    Ok(())
                } else {
                    Err(RangeError::InvalidBounds)
                }
            }
            (
                Self::Texture {
                    aspect_mask,
                    base_mip_level,
                    level_count,
                    base_array_layer,
                    layer_count,
                },
                Self::Texture {
                    aspect_mask: aspects,
                    level_count: mips,
                    layer_count: layers,
                    ..
                },
            ) => {
                if aspect_mask != 0
                    && aspect_mask & !aspects == 0
                    && level_count != 0
                    && layer_count != 0
                    && base_mip_level
                        .checked_add(level_count)
                        .is_some_and(|end| end <= mips)
                    && base_array_layer
                        .checked_add(layer_count)
                        .is_some_and(|end| end <= layers)
                {
                    Ok(())
                } else {
                    Err(RangeError::InvalidBounds)
                }
            }
            _ => Err(RangeError::WrongKind),
        }
    }

    pub fn normalized(self, descriptor: Descriptor) -> Result<Self, RangeError> {
        if self != Self::Whole {
            self.validate(descriptor)?;
            return Ok(self);
        }
        match descriptor {
            Descriptor::Buffer(desc) if desc.word_at_00 != 0 => Ok(Self::Buffer {
                offset: 0,
                size: desc.word_at_00,
            }),
            Descriptor::Image(desc) => {
                let mip_count = u32::from(desc.bytes_at_1c_through_1e[0]);
                let layers = match desc.bytes_at_1c_through_1e[2] {
                    0 | 2 | 4 => Some(1),
                    1 | 3 => Some(desc.words_at_00_through_18[6]),
                    5 => Some(6),
                    6 => desc.words_at_00_through_18[6].checked_mul(6),
                    _ => None,
                }
                .filter(|layers| *layers != 0)
                .ok_or(RangeError::InvalidBounds)?;
                let aspect_mask = crate::raw::format::vk_format_all_aspects(
                    ash::vk::Format::from_raw(desc.words_at_00_through_18[2] as i32),
                )
                .as_raw();
                if mip_count == 0 || aspect_mask == 0 {
                    return Err(RangeError::InvalidBounds);
                }
                Ok(Self::Texture {
                    aspect_mask,
                    base_mip_level: 0,
                    level_count: mip_count,
                    base_array_layer: 0,
                    layer_count: layers,
                })
            }
            _ => Err(RangeError::InvalidBounds),
        }
    }

    pub fn contains(self, other: Self) -> bool {
        match (self, other) {
            (Self::Whole, _) => true,
            (Self::Buffer { offset: a, size: n }, Self::Buffer { offset: b, size: m }) => {
                a <= b
                    && b.checked_add(m)
                        .zip(a.checked_add(n))
                        .is_some_and(|(be, ae)| be <= ae)
            }
            (
                Self::Texture {
                    aspect_mask: a,
                    base_mip_level: am,
                    level_count: an,
                    base_array_layer: al,
                    layer_count: ac,
                },
                Self::Texture {
                    aspect_mask: b,
                    base_mip_level: bm,
                    level_count: bn,
                    base_array_layer: bl,
                    layer_count: bc,
                },
            ) => {
                b & !a == 0
                    && am <= bm
                    && al <= bl
                    && bm
                        .checked_add(bn)
                        .zip(am.checked_add(an))
                        .is_some_and(|(be, ae)| be <= ae)
                    && bl
                        .checked_add(bc)
                        .zip(al.checked_add(ac))
                        .is_some_and(|(be, ae)| be <= ae)
            }
            _ => false,
        }
    }

    pub fn overlaps(self, other: Self) -> bool {
        match (self, other) {
            (Self::Whole, _) | (_, Self::Whole) => true,
            (Self::Buffer { offset: a, size: n }, Self::Buffer { offset: b, size: m }) => {
                a.checked_add(n).is_some_and(|end| b < end)
                    && b.checked_add(m).is_some_and(|end| a < end)
            }
            (
                Self::Texture {
                    aspect_mask: a,
                    base_mip_level: am,
                    level_count: an,
                    base_array_layer: al,
                    layer_count: ac,
                },
                Self::Texture {
                    aspect_mask: b,
                    base_mip_level: bm,
                    level_count: bn,
                    base_array_layer: bl,
                    layer_count: bc,
                },
            ) => {
                a & b != 0
                    && am.checked_add(an).is_some_and(|end| bm < end)
                    && bm.checked_add(bn).is_some_and(|end| am < end)
                    && al.checked_add(ac).is_some_and(|end| bl < end)
                    && bl.checked_add(bc).is_some_and(|end| al < end)
            }
            _ => false,
        }
    }
}

/// Automatic is only an input adapter for the existing raw access-code API.
/// The planner resolves it before checking or caching a plan.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Hash)]
pub enum AccessVersion {
    #[default]
    Automatic,
    Read(u32),
    Write {
        previous: u32,
        current: u32,
    },
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum ResourceContractError {
    WrongRangeKind {
        resource: u32,
    },
    InvalidRange {
        resource: u32,
    },
    InvalidAccessCode(u8),
    WrongAccessMode {
        resource: u32,
        access_code: u8,
    },
    UninitializedRead {
        resource: u32,
        pass: u32,
    },
    StaleVersion {
        resource: u32,
        expected: u32,
        actual: u32,
    },
    InvalidWriteVersion {
        resource: u32,
        previous: u32,
        current: u32,
    },
    VersionOverflow(u32),
}

/// 携带描述符的资源句柄；`desc()` 返回声明期描述符，`clone_unchecked()` 不做
/// 引用计数/有效性检查地复制（原 197 处内联实例的廉价克隆语义）。
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Handle {
    pub resource: ResourceHandle,
    pub descriptor: Descriptor,
}
impl Handle {
    pub fn new(resource: ResourceHandle, descriptor: Descriptor) -> Self {
        Self {
            resource,
            descriptor,
        }
    }
    pub fn desc(&self) -> &Descriptor {
        &self.descriptor
    }
    pub fn raw(&self) -> ResourceHandle {
        self.resource
    }
    pub fn clone_unchecked(&self) -> Self {
        Self {
            resource: self.resource,
            descriptor: self.descriptor,
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ResourceType {
    Image,
    Buffer,
    Other,
}

/// 候选公开表示；原作 ResourceOrigin 的 72 字节布局另有证据记录。
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct ResourceOrigin {
    pub descriptor: Descriptor,
    pub source: OriginSource,
}
impl ResourceOrigin {
    pub fn ty(self) -> ResourceType {
        match self.descriptor {
            Descriptor::Image(_) => ResourceType::Image,
            Descriptor::Buffer(_) => ResourceType::Buffer,
        }
    }
}

/// ✅ 原版空操作：PDB 公共符号 `<Texture as Resource>::validate_read_access`（h83678d6fcc0889d6）与
/// `<Buffer as Resource>::validate_read_access`（hfc3e91621b7af5dd）被 ICF 折叠到同一地址 0x14003bb80，
/// 该地址的机器码只有一条 `ret`（C3，后跟 int3 填充）。故不存在读访问合法性断言/panic 文案；保留空操作。
pub fn validate_read_access(_access: u8) {}

use super::resource_registry::{Buffer, RegistryResource, Texture};
use std::sync::Arc;

pub type RuntimeResource<T, B, O> = RegistryResource<T, B, Arc<O>>;

pub enum BorrowedResource<'a, T, B, O> {
    Image(&'a Texture<T>),
    Buffer(&'a Buffer<B>),
    Other(&'a O),
}

impl<T, B, O> RegistryResource<T, B, Arc<O>> {
    /// 原入口标签 0/1、2/3 直接借用载荷；标签 4 跳过 Arc 的两个引用计数。
    pub fn borrow(&self) -> BorrowedResource<'_, T, B, O> {
        match self {
            Self::OwnedTexture(value) | Self::UnownedTexture(value) => {
                BorrowedResource::Image(value)
            }
            Self::OwnedBuffer(value) | Self::UnownedBuffer(value) => {
                BorrowedResource::Buffer(value)
            }
            Self::Other(value) => BorrowedResource::Other(value.as_ref()),
            Self::Empty => panic!("不能借用空资源"),
        }
    }
}

/// 原作 ResourceOrigin 的取证载体。只暴露已观察字段，不猜测其余字段名。
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(C)]
pub struct ResourceOriginLayout {
    pub tag: u64,
    pub payload: [u8; 64],
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum OriginTypeError {
    DescriptorKind8,
    InvalidOrigin(u64),
}

impl ResourceOriginLayout {
    /// 标签 0 的描述符从对象 +24 开始，描述符判别字节位于 +54。
    /// 标签 1/2 的类型码直接取对象 +8；其他类型码保留原值。
    pub fn ty(&self) -> Result<u8, OriginTypeError> {
        match self.tag {
            0 => match self.payload[46] {
                7 => Ok(1),
                8 => Err(OriginTypeError::DescriptorKind8),
                _ => Ok(0),
            },
            1 | 2 => Ok(self.payload[0]),
            tag => Err(OriginTypeError::InvalidOrigin(tag)),
        }
    }
}

/// 对应原转换入口 +0xbd6ce0：逐字节复制 32 字节，包括填充字节。
pub fn descriptor_from_texture(input: &[u8; 32]) -> [u8; 32] {
    *input
}

/// 对应原转换入口 +0xbd6d00：只写前 16 字节及第 30 字节。
/// 未写入区保持原值，不能错误地补零或复制全部 32 字节。
pub fn descriptor_from_buffer(input: &[u8; 16], output: &mut [u8; 32]) {
    output[..16].copy_from_slice(input);
    output[30] = 7;
}

/// 只允许在所属 builder 中生产绑定；原 Ref 的完整泛型 ABI 尚未恢复。
pub struct Ref<'borrow, 'graph, Kind> {
    pub(crate) builder: &'borrow mut super::pass_builder::PassBuilder<'graph>,
    pub(crate) resource: ResourceHandle,
    pub(crate) access_code: u8,
    pub(crate) group: u32,
    pub(crate) kind: std::marker::PhantomData<Kind>,
}
pub enum BufferRef {}
pub enum TextureRef {}
impl<K> Ref<'_, '_, K> {
    pub fn raw(&self) -> ResourceHandle {
        self.resource
    }
}
impl Ref<'_, '_, BufferRef> {
    pub fn bind_uniform_buffer(
        self,
        offset: u64,
        size: u64,
    ) -> Result<super::pass_bindings::PassBinding, super::pass_bindings::PassBindingError> {
        self.builder.add_resource_access_range(
            self.resource,
            self.access_code,
            self.group,
            AccessRange::Buffer { offset, size },
        )?;
        Ok(super::pass_bindings::PassBinding::buffer(
            self.resource,
            offset,
            size,
            true,
        ))
    }
    pub fn bind_storage_buffer(
        self,
        offset: u64,
        size: u64,
    ) -> Result<super::pass_bindings::PassBinding, super::pass_bindings::PassBindingError> {
        self.builder.add_resource_access_range(
            self.resource,
            self.access_code,
            self.group,
            AccessRange::Buffer { offset, size },
        )?;
        Ok(super::pass_bindings::PassBinding::buffer(
            self.resource,
            offset,
            size,
            false,
        ))
    }
}
impl Ref<'_, '_, TextureRef> {
    /// view 与布局在 callback 解析真实纹理时验证，绑定自动登记本 pass 的资源访问。
    pub fn bind_view(
        self,
        view: crate::raw::texture::TextureViewDesc,
        layout: ash::vk::ImageLayout,
        storage: bool,
    ) -> Result<super::pass_bindings::PassBinding, super::pass_bindings::PassBindingError> {
        self.builder.add_resource_access_range(
            self.resource,
            self.access_code,
            self.group,
            AccessRange::Texture {
                aspect_mask: view.aspect.as_raw(),
                base_mip_level: view.base_mip_level,
                level_count: view.level_count,
                base_array_layer: view.base_layer,
                layer_count: view.layer_count,
            },
        )?;
        Ok(super::pass_bindings::PassBinding::image(
            self.resource,
            view,
            layout,
            storage,
        ))
    }
    pub fn into_sampled_rg_texture(
        self,
        view: crate::raw::texture::TextureViewDesc,
        layout: ash::vk::ImageLayout,
    ) -> Result<super::pass_bindings::PassBinding, super::pass_bindings::PassBindingError> {
        self.bind_view(view, layout, false)
    }
}

/// 原 view carrier 的限定字节写入合同；不把未知 60 字节强行映射到候选 view 描述符。
pub fn original_ref_view_binding(
    view: &[u8; 60],
    logical: ResourceHandle,
    role: u8,
    output: &mut [u8; 68],
) {
    output[..60].copy_from_slice(view);
    output[60..64].copy_from_slice(&logical.raw().to_le_bytes());
    output[64] = role;
}
/// 原 buffer carrier：uniform=3，另一 bind 实例=4；未知尾部不参与恢复。
pub fn original_ref_buffer_binding(logical: ResourceHandle, uniform: bool) -> [u32; 2] {
    [if uniform { 3 } else { 4 }, logical.raw()]
}
