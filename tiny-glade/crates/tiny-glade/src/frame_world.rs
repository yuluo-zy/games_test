//! Borrow the resources actually read by B `PassResources::new_from`.
//!
//! Original constants are identities, not layouts: a Rust source build cannot use
//! them as its compiler's `TypeId`. This adapter maps the six confirmed identities
//! to typed, live references. No resource is synthesized and no unrelated Bevy
//! resource is cast to an original memory layout. Additional `ResourceView`s must
//! be supplied as their exact reconstructed types by the corresponding producer.
//!
//! Evidence: B RNE SHA f507a6ce…; `new_from` 0x140909310 and the two `movabs`
//! instructions in each named insertion instance: VAO 0x14012153a/544,
//! placement 0x14012383a/844, constants 0x14011f88a/894, optional shader watch
//! 0x140115fe6/ff0. See TG-727475d65610eb845f/main-chain-frame-20261018.

use std::{collections::HashMap, rc::Rc, sync::Arc};

use country_core::{
    asset_libraries::{
        placement_data_library::AssetPlacementDataLibrary, shader_library::AssetShaderLibrary,
        ssbo_library::AssetSsboLibrary, texture_library::AssetTextureLibrary,
        vao_library::AssetVAOLibrary,
    },
    render::pass_builder::{
        compute_pass_builder::{ComputePassDevice, WorldAccess},
        pass_resources::{ResourceLookup, TypeId128, type_ids},
    },
    systems::render_loop::compute_passes::pass_kit::ResourceView,
};
use parking_lot::Mutex;
use rhapsody::resource::DynamicConstants;

/// Typed borrows of the six resources required by the original function.
/// `constants` is the mutex payload of the original `DynamicConstantsResource`;
/// this adapter does not expose or assert the old compiler's wrapper ABI.
pub struct FrameLibraries<'a> {
    pub shaders: &'a AssetShaderLibrary,
    pub textures: &'a AssetTextureLibrary,
    pub ssbos: &'a AssetSsboLibrary,
    pub vaos: &'a AssetVAOLibrary,
    pub placement: &'a AssetPlacementDataLibrary,
    pub constants: &'a Mutex<DynamicConstants>,
}

pub struct FrameWorldAccess<'a> {
    libraries: FrameLibraries<'a>,
    device: Arc<dyn ComputePassDevice>,
    views: HashMap<TypeId128, *const ()>,
    graph_factory: Option<Rc<dyn Fn() -> country_core::rhapsody_types::RenderGraph>>,
}

impl<'a> FrameWorldAccess<'a> {
    /// `device` must use the same handle tables as these libraries and the graph.
    pub fn new(libraries: FrameLibraries<'a>, device: Arc<dyn ComputePassDevice>) -> Self {
        Self {
            libraries,
            device,
            views: HashMap::new(),
            graph_factory: None,
        }
    }

    pub fn with_resource_graph_factory(
        mut self,
        factory: Rc<dyn Fn() -> country_core::rhapsody_types::RenderGraph>,
    ) -> Self {
        self.graph_factory = Some(factory);
        self
    }

    /// Borrow an exact resource view for this frame. Reject identity collisions
    /// instead of replacing a library with a pointer to a different layout.
    pub fn bind_view<T: ResourceView>(&mut self, value: &'a T) -> rhapsody::Result<()> {
        let id = T::TYPE_ID;
        if self.get_resource_ptr(id).is_some() {
            return Err(rhapsody::Error::invalid_usage(
                "duplicate frame resource identity",
            ));
        }
        self.views.insert(id, (value as *const T).cast());
        Ok(())
    }
}

impl ResourceLookup for FrameWorldAccess<'_> {
    fn get_resource_ptr(&self, id: TypeId128) -> Option<*const ()> {
        let l = &self.libraries;
        Some(match id {
            type_ids::SHADER_LIBRARY => (l.shaders as *const AssetShaderLibrary).cast(),
            type_ids::TEXTURE_LIBRARY => (l.textures as *const AssetTextureLibrary).cast(),
            type_ids::SSBO_LIBRARY => (l.ssbos as *const AssetSsboLibrary).cast(),
            type_ids::RESOURCE_E => (l.vaos as *const AssetVAOLibrary).cast(),
            type_ids::RESOURCE_G => (l.placement as *const AssetPlacementDataLibrary).cast(),
            type_ids::DYNAMIC_CONSTANTS => (l.constants as *const Mutex<DynamicConstants>).cast(),
            // ShaderWatch is optional in the original; it is absent unless a
            // correctly typed producer explicitly supplies its reconstructed view.
            _ => return self.views.get(&id).copied(),
        })
    }
}

impl WorldAccess for FrameWorldAccess<'_> {
    fn graphics_device(&self) -> Option<Arc<dyn ComputePassDevice>> {
        Some(self.device.clone())
    }
    fn resource_graph(&self) -> country_core::rhapsody_types::RenderGraph {
        self.graph_factory
            .as_ref()
            .expect("FrameWorldAccess must use its enclosing frame's real resource graph factory")(
        )
    }
}
