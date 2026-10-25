//! lib.rs — WP6 证据束的 crate 根（镜像 `reconstruction/country-core` 的装配布局）。
//!
//! 本束只含 wp6 交付的文件 + 模块胶水；完整可编译装配在
//! `reconstruction/country-core/src/lib.rs`（集成 owner 组装，wp1–wp6）。

pub mod asset_libraries {
    // 复用 wp4 装配：`AssetSsboLibrary` / `AssetTextureLibrary`（本束不重新交付）。
    pub mod ssbo_library {
        pub use crate::asset_libraries_stub::ssbo::AssetSsboLibrary;
    }
    pub mod texture_library {
        pub use crate::asset_libraries_stub::texture::AssetTextureLibrary;
    }
}

/// wp4 装配类型占位（本束不交付 wp4；集成时指向真实 wp4 类型）。
pub mod asset_libraries_stub {
    pub mod ssbo {
        use crate::typed_gap::Gap;
        #[derive(Clone, Debug, Default)]
        pub struct AssetSsboLibrary(pub Gap);
    }
    pub mod texture {
        use crate::typed_gap::Gap;
        #[derive(Clone, Debug, Default)]
        pub struct AssetTextureLibrary(pub Gap);
    }
}

pub mod components;
pub mod resources;
pub mod systems;
pub mod utils;

pub mod render {
    pub mod mesh_atlas {
        // 复用 wp3 装配：`MeshAtlasPart`（集成时指向真实 wp3 类型）。
        pub use crate::render_stub::mesh_atlas::MeshAtlasPart;
    }
    pub mod nani {
        pub mod gpu_world {
            // 复用 wp3 装配：`GpuWorld` / `InstanceDataMarker` / `InstanceHandle` /
            // `InstanceKey` / `OwnedGpuWorldSubset` / `GpuWorldSubsetKind` /
            // `GpuWorldSubset`（集成时指向真实 wp3 类型）。
            pub use crate::render_stub::nani::gpu_world::*;
        }
        pub mod instance_data {
            // 复用 wp3 装配：全部 instance_data 类型（集成时指向真实 wp3 类型）。
            pub use crate::render_stub::nani::instance_data::*;
        }
        pub mod systems;
    }
}

pub mod render_stub {
    pub mod mesh_atlas {
        use crate::typed_gap::Gap;
        #[derive(Clone, Copy, Debug, Default)]
        pub struct MeshAtlasPart(pub Gap);
    }
    pub mod nani {
        pub mod gpu_world {
            use crate::typed_gap::Gap;
            #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
            #[repr(u8)]
            pub enum GpuWorldSubsetKind {
                Main = 0,
                Bush = 1,
                CurvedHalftimber = 2,
                Flowers = 3,
                Plaster = 4,
                RoofTile = 5,
                Smoke = 6,
                StairsFloor = 7,
                WoodenPlank = 8,
                Unknown9 = 9,
                Unknown10 = 10,
                Unknown11 = 11,
                Unknown12 = 12,
                Unknown13 = 13,
                Unknown14 = 14,
                Unknown15 = 15,
                Unknown16 = 16,
                Unknown17 = 17,
                Unknown18 = 18,
                Unknown19 = 19,
                Unknown20 = 20,
                Unknown21 = 21,
                Unknown22 = 22,
                Unknown23 = 23,
                Unknown24 = 24,
                Unknown25 = 25,
                Unknown26 = 26,
            }
            #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
            pub struct InstanceHandle {
                pub kind: GpuWorldSubsetKind,
                pub index: u32,
            }
            #[derive(Clone, Copy, Debug)]
            pub struct InstanceKey {
                pub chunk_id: u64,
                pub instance_index: u32,
                pub sort_key: f32,
            }
            #[derive(Clone, Copy, Debug, Default)]
            pub struct InstanceDataMarker<D>(pub core::marker::PhantomData<D>);
            pub enum GpuWorldSubset {
                Owned(OwnedGpuWorldSubset),
                External(ExternalGpuWorldSubset),
            }
            pub struct ExternalGpuWorldSubset(pub Gap);
            pub struct OwnedGpuWorldSubset {
                pub _gap: Gap,
            }
            impl OwnedGpuWorldSubset {
                pub fn extend_no_data(
                    &self,
                    _keys: &[InstanceKey],
                    _kind: GpuWorldSubsetKind,
                    _callback: impl FnMut(u64),
                ) {
                }
                pub fn update_instance_data<T>(&self, _key: InstanceKey, _data: T) {}
            }
            pub struct GpuWorld {
                pub _gap: Gap,
            }
            impl GpuWorld {
                pub fn owned_subset_mut(
                    &self,
                    _kind: GpuWorldSubsetKind,
                ) -> parking_lot::RwLock<Option<GpuWorldSubset>> {
                    parking_lot::RwLock::new(None)
                }
                pub fn remove_instance(&self, _key: InstanceKey) {}
                pub fn update_instance_mesh(&self, _entity: bevy_ecs::entity::Entity) {}
            }
        }
        pub mod instance_data {
            use crate::typed_gap::Gap;
            macro_rules! gap_ty {
                ($($t:ident)*) => {$(
                    #[derive(Clone, Copy, Default)]
                    pub struct $t(pub [Gap; 1]);
                )*};
            }
            gap_ty! {
                SolidVertexColorInstanceData
                SolidVertexColorWindowInstanceData
                SolidAlbedoMapInstanceData
                SolidAlbedoMapEmissiveInstanceData
                PreplacedFlowerInstanceData
                PlantInstanceData
                SolidVertexColorWindowGlassInstanceData
                SolidVertexColorBricksInstanceData
                GothicWindowBricksInstanceData
                CurvedHalfTimberInstanceData
                StoneFloorInstanceData
                WrenInstanceData
                LanternEmissiveInstanceData
                PlasterInstanceData
                ClothInstanceData
                SmokeInstanceData
                BalconyDoorInstanceData
                UberInstanceData
                VelocityOnlyInstanceData
                StairsFloorInstanceData
            }
        }
    }
}

/// 全局 typed-gap 标记（同 country-core `crate::Gap`）。
#[derive(Clone, Copy, Default, Debug, PartialEq, Eq, Hash)]
pub struct Gap;

pub mod typed_gap {
    pub use crate::Gap;
}

/// puffin env gap 契约（同 country-core `crate::puffin_contract`）。
pub mod puffin_contract {
    pub struct Scope;
    impl Drop for Scope {
        fn drop(&mut self) {}
    }
    pub fn scope(_name: &str) -> Scope {
        Scope
    }
}
