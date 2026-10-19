//! 图资源导入与导出入口。所有权标记传入已有资源注册表消费者。
use super::{
    graph::{BuildError, Descriptor, OriginSource, RenderGraph},
    resource::{ResourceHandle, ResourceOrigin},
};
impl RenderGraph {
    pub fn import_unowned_with_sharing(
        &mut self,
        name: impl Into<String>,
        descriptor: Descriptor,
        token: u64,
        initial_access: u8,
        sharing: crate::resource::ResourceSharing,
    ) -> Result<ResourceHandle, BuildError> {
        let handle = self.import_unowned(name, descriptor, token, initial_access)?;
        self.set_resource_sharing(handle, sharing)?;
        Ok(handle)
    }
    pub fn import_owned_with_sharing(
        &mut self,
        name: impl Into<String>,
        descriptor: Descriptor,
        token: u64,
        initial_access: u8,
        sharing: crate::resource::ResourceSharing,
    ) -> Result<ResourceHandle, BuildError> {
        let handle = self.import_owned(name, descriptor, token, initial_access)?;
        self.set_resource_sharing(handle, sharing)?;
        Ok(handle)
    }
    pub fn set_resource_sharing(
        &mut self,
        handle: ResourceHandle,
        sharing: crate::resource::ResourceSharing,
    ) -> Result<(), BuildError> {
        self.check_handle(handle)?;
        self.declaration.resources[handle.raw() as usize].sharing = sharing;
        Ok(())
    }
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

/// 物理资源与渲染图资源之间的导入/导出（Texture / Buffer 两个 impl）。
/// 物理资源在这一层以 `(描述, token)` 表示，token 由后端资源表持有。
pub trait ImportExportToRenderGraph: Sized {
    fn descriptor(&self) -> Descriptor;
    fn token(&self) -> u64;
    /// 由图导出的物理资源重建载体。
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

/// 可作为采样纹理导入渲染图的类型（`into_sampled_rg_texture`，
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
