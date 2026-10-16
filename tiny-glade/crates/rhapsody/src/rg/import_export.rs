//! 图资源导入与导出入口。所有权标记传入已有资源注册表消费者。
use super::{
    graph::{BuildError, Descriptor, OriginSource, RenderGraph},
    resource::{ResourceHandle, ResourceOrigin},
};
impl RenderGraph {
    pub fn import_unowned(
        &mut self,
        name: impl Into<String>,
        descriptor: Descriptor,
        token: u64,
        initial_access: u8,
    ) -> Result<ResourceHandle, BuildError> {
        self.create_raw_resource(
            name,
            ResourceOrigin {
                descriptor,
                source: OriginSource::ImportUnowned {
                    token,
                    initial_access,
                },
            },
        )
    }
    pub fn import_owned(
        &mut self,
        name: impl Into<String>,
        descriptor: Descriptor,
        token: u64,
        initial_access: u8,
    ) -> Result<ResourceHandle, BuildError> {
        self.create_raw_resource(
            name,
            ResourceOrigin {
                descriptor,
                source: OriginSource::ImportOwned {
                    token,
                    initial_access,
                },
            },
        )
    }
}

use super::{
    executor::ExportedResource,
    graph::{BufferDesc, TextureDesc},
    pass_bindings::{PassBinding, PassBindingError},
    resource::{Ref, TextureRef},
};
use crate::raw::texture::TextureViewDesc;
use ash::vk;

/// 原 trait `ImportExportToRenderGraph`（Texture / Buffer 两个 impl）。
/// 物理资源在这一层以 `(描述, token)` 表示，token 由后端资源表持有。
pub trait ImportExportToRenderGraph: Sized {
    fn descriptor(&self) -> Descriptor;
    fn token(&self) -> u64;
    /// 由图导出的物理资源重建载体（`from_owned_rg_resource`，0x140bd63f0）。
    fn from_owned_rg_resource(desc: Descriptor, exported: ExportedResource) -> Option<Self>;

    /// `import_unowned`：图只借用，物理对象的所有权留在调用方。
    fn import_unowned(
        &self,
        graph: &mut RenderGraph,
        name: impl Into<String>,
        access: u8,
    ) -> Result<ResourceHandle, BuildError> {
        graph.import_unowned(name, self.descriptor(), self.token(), access)
    }
    /// `import_owned`：物理对象的所有权移交给图。
    fn import_owned(
        self,
        graph: &mut RenderGraph,
        name: impl Into<String>,
        access: u8,
    ) -> Result<ResourceHandle, BuildError> {
        graph.import_owned(name, self.descriptor(), self.token(), access)
    }
    /// `export`：把图内资源按给定访问码导出。
    fn export(
        graph: &mut RenderGraph,
        handle: ResourceHandle,
        access: u8,
    ) -> Result<(), BuildError> {
        graph.export(handle, access)
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PhysicalTexture {
    pub desc: TextureDesc,
    pub token: u64,
}
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PhysicalBuffer {
    pub desc: BufferDesc,
    pub token: u64,
}
impl ImportExportToRenderGraph for PhysicalTexture {
    fn descriptor(&self) -> Descriptor {
        Descriptor::Image(self.desc)
    }
    fn token(&self) -> u64 {
        self.token
    }
    fn from_owned_rg_resource(desc: Descriptor, exported: ExportedResource) -> Option<Self> {
        match desc {
            Descriptor::Image(desc) => Some(Self {
                desc,
                token: exported.token,
            }),
            Descriptor::Buffer(_) => None,
        }
    }
}
impl ImportExportToRenderGraph for PhysicalBuffer {
    fn descriptor(&self) -> Descriptor {
        Descriptor::Buffer(self.desc)
    }
    fn token(&self) -> u64 {
        self.token
    }
    fn from_owned_rg_resource(desc: Descriptor, exported: ExportedResource) -> Option<Self> {
        match desc {
            Descriptor::Buffer(desc) => Some(Self {
                desc,
                token: exported.token,
            }),
            Descriptor::Image(_) => None,
        }
    }
}

/// `Buffer::import_unowned`（0x140bd4fc0）的返回值，20 字节：
/// `{size: u64 @0, desc_tail: u64 @8, resource_index: u32 @0x10}`。
/// 前两个 qword 取自 `BufferRecord` 的 `+0x18`（size）/`+0x20`（`BufferDesc` 尾 8 字节：usage、对齐、内存位置），
/// 末尾 u32 是新建图资源条目的下标（= `graph.resources.len()` 在 push 前的值）。
/// 同函数还把 `BufferRecord` 全体（`ctx` / `slot`，并对 slot 的引用计数 `lock inc [slot+8]`）搬进 0x48 字节的
/// `ResourceOrigin`（tag 1 = ImportUnowned，`+8` 字节 = {buffer=1, initial_access}），并向 `+0x30` 的访问 Vec
/// 追加一个 `{0xffffffff, 0}`（16 字节）空条目。
/// Texture 版 0x140bd60a0 同形，返回 36 字节 `{index u32, 4×u64}`；此处未建模。
///
/// ⚠️ 任务描述里"`+0xe = 5` 为未导入判别"在这两个函数的机器码里没有出现：`+0xe` 不在 20 字节返回值的字段边界上。
/// `5` 只在 `ResourceRegistry` 的后备资源 tag 里出现（5 = Empty，见 ALGORITHMS §1.2），未导出/已剥离的槽才是 5，
/// 与 `ImportedBuffer` 无关，所以这里不引入该判别。
#[repr(C, packed(4))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct ImportedBuffer {
    pub size: u64,
    pub desc_tail: u64,
    pub resource_index: u32,
}
const _: () = assert!(std::mem::size_of::<ImportedBuffer>() == 20);
impl ImportedBuffer {
    pub const ORIGINAL_BYTES: usize = 20;
    pub fn from_record(record: &crate::raw::buffer::BufferRecord, handle: ResourceHandle) -> Self {
        Self {
            size: record.size,
            desc_tail: record.desc_tail,
            resource_index: handle.raw(),
        }
    }
}

/// 原 trait 名 `IntoSampledRenderGraphTexture`（0x140bd6470 的 `into_sampled_rg_texture` 所属，
/// 不是 `ImportExportToRenderGraph`）。
pub trait IntoSampledRenderGraphTexture {
    fn into_sampled_rg_texture(
        self,
        view: TextureViewDesc,
        layout: vk::ImageLayout,
    ) -> Result<PassBinding, PassBindingError>;
}
impl IntoSampledRenderGraphTexture for Ref<'_, '_, TextureRef> {
    fn into_sampled_rg_texture(
        self,
        view: TextureViewDesc,
        layout: vk::ImageLayout,
    ) -> Result<PassBinding, PassBindingError> {
        Ref::into_sampled_rg_texture(self, view, layout)
    }
}

#[cfg(test)]
mod trait_tests {
    use super::*;

    #[test]
    fn imported_buffer_is_twenty_bytes_with_the_original_offsets() {
        assert_eq!(
            std::mem::size_of::<ImportedBuffer>(),
            ImportedBuffer::ORIGINAL_BYTES
        );
        let record = crate::raw::buffer::BufferRecord {
            ctx: 1,
            slot: 2,
            offset: 3,
            device_id: 4,
            ready: 1,
            _pad: 0,
            size: 0x1234,
            desc_tail: 0xabcd,
        };
        let imported = ImportedBuffer::from_record(&record, ResourceHandle::from_raw(7));
        let bytes: [u8; 20] = unsafe { std::mem::transmute(imported) };
        assert_eq!(u64::from_le_bytes(bytes[0..8].try_into().unwrap()), 0x1234);
        assert_eq!(u64::from_le_bytes(bytes[8..16].try_into().unwrap()), 0xabcd);
        assert_eq!(u32::from_le_bytes(bytes[16..20].try_into().unwrap()), 7);
    }

    use crate::rg::graph::OriginSource;

    fn tex() -> PhysicalTexture {
        PhysicalTexture {
            desc: TextureDesc {
                words_at_00_through_18: [1, 2, 3, 4, 5, 6, 7],
                bytes_at_1c_through_1e: [0; 3],
            },
            token: 41,
        }
    }

    #[test]
    fn unowned_and_owned_imports_differ_only_in_origin_tag() {
        let mut graph = RenderGraph::new();
        let a = tex().import_unowned(&mut graph, "a", 0).unwrap();
        let b = tex().import_owned(&mut graph, "b", 0).unwrap();
        let sources: Vec<_> = graph
            .declaration
            .resources
            .iter()
            .map(|r| r.source)
            .collect();
        assert!(matches!(
            sources[a.raw() as usize],
            OriginSource::ImportUnowned { token: 41, .. }
        ));
        assert!(matches!(
            sources[b.raw() as usize],
            OriginSource::ImportOwned { token: 41, .. }
        ));
    }

    #[test]
    fn from_owned_rg_resource_checks_the_resource_kind() {
        let exported = ExportedResource::new(0, 9);
        let t = PhysicalTexture::from_owned_rg_resource(tex().descriptor(), exported).unwrap();
        assert_eq!(t.token, 9);
        assert!(PhysicalBuffer::from_owned_rg_resource(tex().descriptor(), exported).is_none());
    }
}
