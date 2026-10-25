//! 第 0 层文件的挂载点（相对路径指向真实源文件，不拷贝）。
#[path = "@SRC@/render/view_constants.rs"]
pub mod view_constants;
#[path = "@SRC@/render/frame_constants.rs"]
pub mod frame_constants;
#[path = "@SRC@/render/dirty_index_range.rs"]
pub mod dirty_index_range;
#[path = "@SRC@/render/ssbo.rs"]
pub mod ssbo;
#[path = "@SRC@/render/shadowed_buffer.rs"]
pub mod shadowed_buffer;
#[path = "@SRC@/render/mesh.rs"]
pub mod mesh;
#[path = "@SRC@/render/vao.rs"]
pub mod vao;
#[path = "@SRC@/render/utils.rs"]
pub mod utils;
pub mod pass_builder {
    #[path = "@SRC@/render/pass_builder/shader_uri.rs"]
    pub mod shader_uri;
    #[path = "@SRC@/render/pass_builder/pass_resources.rs"]
    pub mod pass_resources;
    #[path = "@SRC@/render/pass_builder/uniform_binder.rs"]
    pub mod uniform_binder;
    #[path = "@SRC@/render/pass_builder/pass_resource_access.rs"]
    pub mod pass_resource_access;
    #[path = "@SRC@/render/pass_builder/rg_resources.rs"]
    pub mod rg_resources;
    #[path = "@SRC@/render/pass_builder/draw_list.rs"]
    pub mod draw_list;
    #[path = "@SRC@/render/pass_builder/bind_resources.rs"]
    pub mod bind_resources;
    #[path = "@SRC@/render/pass_builder/compute_pass_builder.rs"]
    pub mod compute_pass_builder;
    #[path = "@SRC@/render/pass_builder/framebuffer_binding.rs"]
    pub mod framebuffer_binding;
    #[path = "@SRC@/render/pass_builder/draw_stream.rs"]
    pub mod draw_stream;
    #[path = "@SRC@/render/pass_builder/draw_builder.rs"]
    pub mod draw_builder;
}
#[path = "@SRC@/render/directional_light.rs"]
pub mod directional_light;
#[path = "@SRC@/render/atmosphere.rs"]
pub mod atmosphere;
#[path = "@SRC@/render/rg_helpers.rs"]
pub mod rg_helpers;
#[path = "@SRC@/render/shader.rs"]
pub mod shader;
#[path = "@SRC@/render/render_config.rs"]
pub mod render_config;
#[path = "@SRC@/render/load_texture.rs"]
pub mod load_texture;
#[path = "@SRC@/render/sparse_instance_buffer.rs"]
pub mod sparse_instance_buffer;
#[path = "@SRC@/render/mesh_atlas.rs"]
pub mod mesh_atlas;
pub mod nani {
    pub mod gpu_world {
        #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
        #[repr(u8)]
        pub enum GpuWorldSubsetKind { Canopy, TreeTrunk, BackgroundTree, SolidVertexColor, SolidVertexColorWindow, SolidVertexColorWindowGlass, SolidVertexColorBricks, GothicWindowBricks, RoofTile, RoofFloor, Plant, Flower, Bush, WoodenPlank, PreplacedFlower, SolidAlbedoMap, SolidAlbedoMapEmissive, Wren, LanternEmissive, Plaster, Cloth, Smoke, BalconyDoor, Uber, CurvedHalfTimber, VelocityOnly, StairsFloor }
        impl GpuWorldSubsetKind {
            pub fn all() -> impl Iterator<Item = GpuWorldSubsetKind> {
                (0u8..27).map(|i| unsafe { core::mem::transmute::<u8, GpuWorldSubsetKind>(i) })
            }
        }
    }
    #[path = "@SRC@/render/nani/mesh_atlas.rs"]
    pub mod mesh_atlas;
}

#[path = "@SRC@/render/raster_single.rs"]
pub mod raster_single;
