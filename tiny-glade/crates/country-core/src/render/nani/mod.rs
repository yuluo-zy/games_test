//! render/nani.rs — nani GPU-driven 渲染模块根（WP3）。
//!
//! 源 SHA256 `abf304771afe0ae305af45e5f05d879cd9be470def1994d8e8b068f5b522d7e4`；
//! PDB 0 procedure_instances（纯模块声明 + 常量 + 类型别名）。
//! 子模块与 skeleton 树一一对应；各子模块业务契约见 contracts/。

#![allow(dead_code, unused_imports)]

/// 游戏 `utils` 的 warn-once 宏展开（O：gpu_world `extend_no_data`/`create_instances` 与 systems
/// `nani_maintain_instances` 各内联一份；每个展开点自带一张 `static Mutex<BTreeMap<String, ()>>`，
/// `expect("Mutex was poisonned")`，去重键 = `"module_path! ()" + "$crate :: Level :: Warn" + 消息`）。
/// `log` crate 没有 warn-once 能力，所以只能是游戏自有逻辑；两处共用本辅助，避免两份手写。
/// 返回 `true` = 首次出现（并已 `log::warn!`）。
pub(crate) fn warn_once_in(
    table: &std::sync::Mutex<alloc::collections::BTreeMap<String, ()>>,
    target: &'static str,
    message: &str,
) -> bool {
    let key = warn_once_key(message);
    let fresh = table
        .lock()
        .expect("Mutex was poisonned")
        .insert(key, ())
        .is_none();
    if fresh {
        log::warn!(target: target, "{}", message);
    }
    fresh
}

/// 去重键（O：`"module_path! ()"` 15B + `"$crate :: Level :: Warn"` + 消息）。
pub fn warn_once_key(message: &str) -> String {
    let mut key = String::from("module_path! ()");
    key.push_str("$crate :: Level :: Warn");
    key.push_str(message);
    key
}

/// 展开点：自带 `static` 去重表（与原版宏展开一致）。
macro_rules! nani_warn_once {
    ($target:expr, $message:expr) => {{
        static TABLE: std::sync::Mutex<alloc::collections::BTreeMap<String, ()>> =
            std::sync::Mutex::new(alloc::collections::BTreeMap::new());
        $crate::render::nani::warn_once_in(&TABLE, $target, &$message)
    }};
}

pub mod buffers;
pub mod cull_shaders;
pub mod draw_ext;
pub mod gpu_world;
pub mod instance_data;
pub mod instance_updates;
pub mod mesh_atlas;
pub mod systems;
// `systems.rs`：maintain / 20 个 `nani_update_*` / flush（行视图 + 纯函数，见 R8）；载荷构建在 `instance_updates.rs`。
pub mod trimesh;

/// 编译期开关：强制细粒度排序（public-symbols `NANI_FORCE_NO_FINE_GRAINED_SORTING`，O）。
pub const NANI_FORCE_FINE_GRAINED_SORTING: bool = false;
/// 编译期开关：禁止细粒度排序（O）。
pub const NANI_FORCE_NO_FINE_GRAINED_SORTING: bool = false;

/// `render::nani` 对外重导出（contracts 提及的共享类型）。
pub use gpu_world::{
    GpuWorld, GpuWorldSubsetKind, InstanceHandle, InstanceKey, NaniInstanceData,
    OwnedGpuWorldSubset,
};
pub use mesh_atlas::{MeshAtlas, MeshAtlasPart, NaniMeshAtlasPartInfo};

/// Native `GpuDrivenDrawable` writes the same mesh, shader and transform
/// components as this existing bundle (write_components@14200fa70). Nani's
/// indirect behavior is selected by the raster shader's draw-list callback.
pub type GpuDrivenDrawable = crate::components::drawable::DrawableMeshBundle;

pub mod solid_vertex_color;
