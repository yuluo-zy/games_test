//! 逻辑资源接口：资源句柄、来源与访问校验。
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
        name: impl Into<crate::resource::ResourceName>,
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
        name: impl Into<crate::resource::ResourceName>,
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
        self.current_versions[handle.raw() as usize].ok_or(
            super::graph::BuildError::Contract(ResourceContractError::VersionOverflow(handle.raw())),
        )
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
/// 不做引用计数/有效性检查的廉价复制。
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

/// 资源来源的公开表示。
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


use super::resource_registry::{Buffer, RegistryResource, Texture};
use std::sync::Arc;


pub enum BorrowedResource<'a, T, B, O> {
    Image(&'a Texture<T>),
    Buffer(&'a Buffer<B>),
    Other(&'a O),
}

impl<T, B, O> RegistryResource<T, B, Arc<O>> {
    /// 标签 0/1、2/3 直接借用载荷；标签 4 跳过 Arc 的两个引用计数。
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

/// 资源来源载体。
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

/// 逐字节复制 32 字节，包括填充字节。
pub fn descriptor_from_texture(input: &[u8; 32]) -> [u8; 32] {
    *input
}


/// 只允许在所属 builder 中生产绑定。
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



#[cfg(test)]
mod version_index_tests {
    use super::*;
    use crate::rg::graph::{BufferDesc, RenderGraph};
    fn scan(graph: &RenderGraph, handle: ResourceHandle) -> Option<u32> {
        let mut version = 0u32;
        for access in graph.declaration.passes.iter().flat_map(|p| &p.accesses)
            .filter(|a| a.resource_index == handle.raw()) {
            match access.version {
                AccessVersion::Write { current, .. } => version = current,
                AccessVersion::Automatic if crate::raw::access_table::is_write(access.access_code) => {
                    version = version.checked_add(1)?;
                }
                _ => {}
            }
        }
        Some(version)
    }
    #[test]
    fn committed_index_matches_scan_with_abandoned_passes_and_graph_roundtrip() {
        let mut graph = RenderGraph::new();
        let handles: Vec<_> = (0..31).map(|i| graph.create_buffer(format!("buffer {i}"), BufferDesc {
            word_at_00: 64, word_at_08: 0, bytes_at_0c_through_0e: [0; 3],
        }).unwrap().raw()).collect();
        let mut rng = 0x12345678u32;
        for step in 0..1024 {
            rng ^= rng << 13; rng ^= rng >> 17; rng ^= rng << 5;
            let handle = handles[rng as usize % handles.len()];
            let mut pass = graph.add_pass("random version");
            pass.add_resource_access(handle, if rng & 1 == 0 { 0x34 } else { 0x26 }, 0).unwrap();
            if step % 7 != 0 { pass.render(|_| true); } else { drop(pass); }
            for &h in &handles { assert_eq!(graph.current_version(h).ok(), scan(&graph, h)); }
        }
        let graph = graph.build().into_graph();
        for &h in &handles { assert_eq!(graph.current_version(h).ok(), scan(&graph, h)); }
        let mut other = RenderGraph::new();
        other.create_buffer("other", BufferDesc { word_at_00: 64, word_at_08: 0, bytes_at_0c_through_0e: [0; 3] }).unwrap();
        assert!(matches!(other.current_version(handles[0]), Err(crate::rg::graph::BuildError::WrongGraph { .. })));

        // Keep this a diagnostic, with no hardware-dependent timing assertion.
        let start = std::time::Instant::now();
        let indexed: u64 = (0..100_000).map(|i| graph.current_version(std::hint::black_box(handles[i % handles.len()])).unwrap() as u64).sum();
        let index_ns = start.elapsed().as_nanos();
        let start = std::time::Instant::now();
        let scanned: u64 = (0..100_000).map(|i| scan(&graph, std::hint::black_box(handles[i % handles.len()])).unwrap() as u64).sum();
        assert_eq!(indexed, scanned);
        eprintln!("VERSION_QUERY_DIAGNOSTIC queries=100000 passes=877 index_ns={index_ns} scan_ns={}", start.elapsed().as_nanos());
    }
    #[test]
    fn overflow_is_sticky_and_explicit_versions_are_indexed() {
        use crate::rg::graph::AccessDeclaration;
        let mut versions = [Some(0)];
        let mut access = AccessDeclaration { resource_index: 0, access_code: 0x34, ..Default::default() };
        access.version = AccessVersion::Write { previous: 0, current: u32::MAX };
        RenderGraph::advance_versions(&mut versions, &[access]);
        assert_eq!(versions, [Some(u32::MAX)]);
        access.version = AccessVersion::Automatic;
        RenderGraph::advance_versions(&mut versions, &[access]);
        assert_eq!(versions, [None]);
        access.version = AccessVersion::Write { previous: 0, current: 1 };
        RenderGraph::advance_versions(&mut versions, &[access]);
        assert_eq!(versions, [None]);
    }
}
