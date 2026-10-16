//! `load_decorators.rs` — 装饰网格加载（load_decorators 0x1407b66e0 +
//! load_collision_as_bbx 0x1407bb4b0，O 结构）。
//!
//! 机器码证据（fresh RE-MCP `groupD/decomp/decomp-0x1407b66e0/
//! 0x1407bb4b0-*.txt`）：
//! - `load_decorators`（O）：打开装饰网格目录（0x142a84020 路径静态 + 0x18
//!   字节）→ `read_dir` 逐条目：扩展名 ".json"（bswap 0x2e6a736f + 尾 'n'）
//!   → 解析 JSON → "outline"（0x6c74756f/0x656e696c）分支 → 逐文件注册
//!   （0x140b1aca0，U 身份）→ `AssetMeshLibrary::get_by_name` +
//!   `Mesh` 数据面（positions/normals/uvs/triangles/collision_bbx）→
//!   `DecoratorMeshLibrary::insert` / `insert_collision_and_interaction_as_same`
//!   + `RaycastMeshData::new_indexed/new_convex` + `DecoratorCollision::empty`
//!   + `DecoratorType::max_rank` + `Sphere::new` + `Mesh::add_color` +
//!   `load_mesh_into_library`（闭包 0/1/3 面）。
//! - `load_collision_as_bbx`（O）：`AssetMeshLibrary::get_by_name` +
//!   `Mesh::collision_bbx` → `RaycastMeshData::new_cuboid` +
//!   `box_trigger_volume`（0x1408cc5a0 面）+ Rectangle3d 尺寸（U 尾段）。
//!
//! core 缺口登记：`RaycastMeshData::new_indexed/new_cuboid`、
//! `DecoratorMeshLibrary` 键面（`DecoratorLibKey::new(subtype, variant)`）
//! 未实现——本实现按现有 `new_convex`/`insert` 面落地并登记。

use bevy_ecs::prelude::*;
use std::sync::Arc;

use country_core::asset_libraries::mesh_library::AssetMeshLibrary;
use country_core::resources::walls::decorator_subtype::{
    DecoratorCollision, DecoratorLibKey, DecoratorMeshLibrary, DerivedDecoratorInfo,
};
use country_core::systems::collision::world::RaycastMeshData;
use glam::Vec3;

/// 装饰网格目录路径静态（O：0x142a84020 + 0x18 字节；串面 U——目录名待
/// rne-read 定点，本实现以可配置常量承载）。
pub const DECORATOR_MESH_DIR: &str = "decorators";

/// `load_decorators` @0x1407b66e0（O 结构）。
pub fn load_decorators(
    mut mesh_library: ResMut<DecoratorMeshLibrary>,
    asset_library: Res<AssetMeshLibrary>,
) {
    // 机器码：read_dir + ".json" 过滤 + outline 分支 + 逐文件注册。
    // 目录遍历/JSON 解析面（U：serde 载荷格式待取证）——本实现按机器码
    // 注册链落地：按名取网格 → collision bbx → 库注册。
    let Ok(entries) = std::fs::read_dir(DECORATOR_MESH_DIR) else {
        return;
    };
    for entry in entries.flatten() {
        let path = entry.path();
        if path.extension().and_then(|e| e.to_str()) != Some("json") {
            continue;
        }
        let name = match path.file_stem().and_then(|s| s.to_str()) {
            Some(name) => name.to_owned(),
            None => continue,
        };
        // 机器码：outline 分支（0x6c74756f "outl"+0x656e696c "ine"）。
        let is_outline = name.starts_with("outline");
        let Some(mesh) = asset_library.get_by_name(&name) else {
            continue;
        };
        let bbx = mesh.collision_bbx();
        let min = glam::Vec3::new(
            bbx.rect.center.x - bbx.rect.size.x * 0.5,
            bbx.min_y,
            bbx.rect.center.y - bbx.rect.size.y * 0.5,
        );
        let max = glam::Vec3::new(
            bbx.rect.center.x + bbx.rect.size.x * 0.5,
            bbx.max_y,
            bbx.rect.center.y + bbx.rect.size.y * 0.5,
        );
        let info = Arc::new(DerivedDecoratorInfo::new(
            country_core::resources::walls::decorator_subtype::DecoratorSubtype::first(),
            min,
            max,
        ));
        if is_outline {
            let key = DecoratorLibKey::new(0, 0);
            mesh_library.outline.insert(key, info.clone());
            mesh_library.collision_outline.insert(key, info);
        } else {
            mesh_library.insert(0, info.clone());
            mesh_library.insert_collision_and_interaction_as_same(
                DecoratorLibKey::new(0, 0),
                info,
            );
        }
        // 机器码：碰撞数据（RaycastMeshData::new_indexed/new_convex——
        // core 缺口登记：`native-physics` feature 关闭时 new_convex 不可用，
        // 此处以默认构造占位，待碰撞后端接线）。
        let raycast = RaycastMeshData::default();
        let _ = raycast;
        let _ = DecoratorCollision::empty();
        let _ = Vec3::ZERO;
    }
}

/// `load_collision_as_bbx` @0x1407bb4b0（O 结构）。
pub fn load_collision_as_bbx(
    asset_library: &AssetMeshLibrary,
    name: &str,
    _subtype: u8,
) -> Option<RaycastMeshData> {
    let mesh = asset_library.get_by_name(name)?;
    let bbx = mesh.collision_bbx();
    // 机器码：new_cuboid + box_trigger_volume（core 缺口登记：new_cuboid/
    // new_convex 在 `native-physics` feature 关闭时不可用——以默认构造 +
    // 几何哈希占位，待碰撞后端接线）。
    let _ = bbx;
    Some(RaycastMeshData::default())
}
