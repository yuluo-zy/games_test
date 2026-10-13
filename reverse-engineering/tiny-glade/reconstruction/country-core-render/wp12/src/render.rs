//! render + asset_libraries — typed-gap facades for the GPU-side types the wall
//! systems touch (`GraphicsDeviceResource`, `Asset*Library`, `InstancedWall`,
//! `SparseInstanceBuffer`, shader URI).

use bevy_ecs::prelude::{Component, Resource};

use crate::typed_gap::Gap;

/// Vulkan device resource.
#[derive(Resource, Clone, Copy, Debug, Default)]
pub struct GraphicsDeviceResource {
    pub raw: Gap,
}

/// Shader URI (wp1 recovered: `ShaderUri::parse`, `/wall/...` URIs).
#[derive(Clone, Debug, PartialEq, Eq, Hash, Default)]
pub struct ShaderUri {
    pub raw: Gap,
}
impl ShaderUri {
    pub fn parse(_s: &str) -> Self {
        Self { raw: Gap }
    }
}

pub mod mesh {
    use super::*;
    #[derive(Component, Clone, Debug, Default)]
    pub struct Mesh {
        pub raw: Gap,
    }
}

/// country-core asset-library handle (typed-gap newtype over a u64 index).
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct Handle<T> {
    pub index: u64,
    pub _p: std::marker::PhantomData<T>,
}
impl<T> Handle<T> {
    pub fn new(index: u64) -> Self {
        Self {
            index,
            _p: std::marker::PhantomData,
        }
    }
}
impl<T> Default for Handle<T> {
    fn default() -> Self {
        Self::new(0)
    }
}

pub mod mesh_library {
    use super::*;
    #[derive(Resource, Clone, Debug, Default)]
    pub struct AssetMeshLibrary {
        pub raw: Gap,
    }
}
pub mod shader_library {
    use super::*;
    #[derive(Resource, Clone, Debug, Default)]
    pub struct AssetShaderLibrary {
        pub raw: Gap,
    }
}
pub mod vao_library {
    use super::*;
    #[derive(Resource, Clone, Debug, Default)]
    pub struct AssetVAOLibrary {
        pub raw: Gap,
    }
}
pub mod ssbo_library {
    use super::*;
    use std::marker::PhantomData;

    #[derive(Resource, Clone, Debug, Default)]
    pub struct AssetSsboLibrary {
        pub raw: Gap,
    }
    impl AssetSsboLibrary {
        pub fn get<T>(&self, _id: SsboId) -> Option<&T> {
            None
        }
        pub fn get_dyn(&self, _name: &str) -> Option<&Gap> {
            None
        }
        pub fn get_mut<T>(&mut self, _id: SsboId) -> Option<&mut T> {
            None
        }
        pub fn get_mut_by_name<T>(&mut self, _name: &str) -> Option<&mut T> {
            None
        }
    }
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
    pub struct SsboId(pub u64);

    #[derive(Clone, Debug, Default)]
    pub struct BufferState<T> {
        pub raw: Gap,
        pub _p: PhantomData<T>,
    }
}

pub mod sparse_instance_buffer {
    use super::*;
    use std::marker::PhantomData;

    #[derive(Clone, Debug, Default)]
    pub struct SparseInstanceBuffer<T> {
        pub raw: Gap,
        pub _p: PhantomData<T>,
    }
    impl<T> SparseInstanceBuffer<T> {
        pub fn allocate(&mut self, _key: u64, _count: u32) -> u32 {
            0
        }
        pub fn gpu_flush(&mut self) {}
    }
}

pub mod geometry {
    pub mod instanced_wall {
        use super::super::*;
        /// Instanced wall brick transform record (WP11/later WPs). Typed gap.
        #[derive(Clone, Debug, Default)]
        pub struct BrickTransformSsbo {
            pub raw: Gap,
        }

        /// Per-wall instanced geometry (WP11). `+0x108` flag selects the compact
        /// vs. full dirty-segment upload path (O in `gpu_flush_brick_buffers`).
        #[derive(Component, Clone, Debug, Default)]
        pub struct InstancedWall {
            pub raw: Gap,
            pub compact: bool,
        }
        impl InstancedWall {
            pub fn upload_to_storage_buffer(&mut self) {}
        }
    }
}

pub mod asset_libraries {
    pub use super::mesh_library::*;
    pub use super::shader_library::*;
    pub use super::ssbo_library::*;
    pub use super::vao_library::*;
}
