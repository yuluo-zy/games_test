//! lib.rs — WP8 证据束的 crate 根（镜像 `reconstruction/country-core` 的装配布局）。
//!
//! 本束只含 wp8 交付的文件 + 模块胶水；完整可编译装配在
//! `reconstruction/country-core/src/lib.rs`（集成 owner 组装，wp1–wp8）。
//! 本束检查通过：`cargo check --offline` 0 错误（见 README.md 与 evidence.json）。

#![allow(dead_code, unused_imports)]

pub mod typed_gap {
    /// Typed-gap marker（与装配 crate 同构）。
    #[derive(Clone, Copy, Default, Debug, PartialEq, Eq, Hash)]
    pub struct Gap;
}

/// wp1 pass 声明面占位（本束不交付 wp1；集成时指向真实 `render::pass_builder`）。
pub mod consume {
    pub mod pass_builder {
        use crate::typed_gap::Gap;

        #[derive(Clone, Debug, Default)]
        pub struct ComputePass(pub Gap);
        impl ComputePass {
            pub fn dispatch_2d(self, _x: u32, _y: u32) -> ComputePass {
                self
            }
            pub fn dispatch_1d(self, _x: u32) -> ComputePass {
                self
            }
        }
        #[derive(Clone, Debug, Default)]
        pub struct ShaderDefValue(pub Gap);
        #[derive(Clone, Debug, Default)]
        pub struct ShaderUri<'a>(pub Gap, pub std::marker::PhantomData<&'a ()>);
        impl<'a> ShaderUri<'a> {
            pub fn parse(_uri: &str) -> ShaderUri<'static> {
                ShaderUri(Gap, std::marker::PhantomData)
            }
        }
        #[derive(Clone, Debug, Default)]
        pub struct ComputeShaderSource<'a> {
            pub uri: ShaderUri<'a>,
            pub defs: ShaderDefValue,
        }
        #[derive(Clone, Debug, Default)]
        pub struct ComputeDispatchApi(pub Gap);
        pub trait AsComputeShaderSource {
            fn source_hash(&self) -> u64;
            fn as_compute_pipeline(&self) -> ComputeShaderSource<'_>;
        }
    }
}

pub mod render {
    pub mod pass_builder {
        pub mod shader_uri {
            #[derive(Clone, Copy, Debug, Default)]
            pub struct ComputeShaderSourceHash(pub u64);

            impl ComputeShaderSourceHash {
                pub fn compute(_uri: &str) -> u64 {
                    0
                }
            }
            pub use crate::consume::pass_builder::ShaderUri;
        }
    }
}

/// wp4 装配类型占位（本束不交付 wp4）。
pub mod asset_libraries {
    pub mod ssbo_library {
        use crate::typed_gap::Gap;
        #[derive(Clone, Debug, Default)]
        pub struct AssetSsboLibrary(pub Gap);
    }
    pub mod texture_library {
        use crate::typed_gap::Gap;
        #[derive(Clone, Debug, Default)]
        pub struct AssetTextureLibrary(pub Gap);
    }
}

pub mod resources {
    use crate::typed_gap::Gap;

    /// `resources::world_raster` 占位（本束不交付；集成时指向装配 crate 的 `RasterBufferRef`）。
    pub mod world_raster {
        #[derive(Clone, Copy, Debug, Default)]
        pub struct RasterBufferRef<const N: usize>(pub crate::typed_gap::Gap);
    }

    pub mod active_raster_stroke;
    pub mod cursor_terrain_raycast;
    pub mod mouse_terrain_raycast;
    pub mod rocky_terrain;
    pub mod terrain_editing;
    pub mod terrain_height;
    pub mod terrain_rock_mask;
    pub mod terrain_stroke;
    pub mod world_raster_defs;
}

pub mod systems {
    pub mod render_loop {
        pub mod compute_passes {
            pub mod displace_rocky_terrain;
            pub mod rasterize_terrain_stroke;
            pub mod terrain_lighting_texture;
        }
    }
    pub mod terrain;
}
