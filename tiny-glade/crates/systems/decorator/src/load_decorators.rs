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

use std::path::Path;

use country_core::asset_libraries::mesh_library::AssetMeshLibrary;
use country_core::render::mesh::Mesh;
use country_core::systems::collision::world::RaycastMeshData;
use country_core::utils::load_json::load_json_as_mesh;
use glam::Vec3;

/// 装饰网格目录路径静态（O：0x142a84020 + 0x18 字节；串面 U——目录名待
/// rne-read 定点，本实现以可配置常量承载）。
pub const DECORATOR_MESH_DIR: &str = "decorators";

/// 网格注册宿主面（B `country_core::startup::load_mesh_into_library`
/// 0x140b1aca0；core 现仅有 trait 形态调用点、无具体资源实现 → 宿主注入）。
pub trait DecoratorMeshHost {
    /// 注册一个从 JSON 加载的网格（`startup_garden_brush` 同型宿主面）。
    fn load_mesh_into_library(&mut self, name: &str, mesh: Mesh);
}

/// `load_decorators` @0x1407b66e0（O，fresh 2026-10-17 逐分支）。
///
/// 行为：
/// 1. `read_dir(dir)`；失败 → anyhow context + `unwrap_failed`（panic）；
/// 2. 迭代条目：`entry.unwrap()`、`file_name().to_str().unwrap()`（B 的
///    `try_from` 失败即 `unwrap_failed`）；
/// 3. 扩展名门：文件名字节长度 > 4，末 5 字节 bswap 后 == ".jso" 且末字节
///    == 'n'（即 `ends_with(".json")`，且要求至少 5 字节）；
/// 4. `entry.path()` → `load_json_as_mesh`（读/解析失败 panic）；
/// 5. 去扩展名后的 stem == "outline" → `Mesh::add_color(Vec3::ONE)`；
/// 6. `load_mesh_into_library(stem, mesh)`（宿主面；U：core 缺具体实现）。
///
/// 早期实现直接 `DecoratorMeshLibrary::insert` 且静默跳过错误，与机器码
/// 不符，已按上述重写；注册叶以 [`DecoratorMeshHost`] 注入。
pub fn load_decorators<H: DecoratorMeshHost>(dir: &Path, host: &mut H) {
    let entries = std::fs::read_dir(dir)
        .unwrap_or_else(|error| panic!("failed to read decorator mesh dir {dir:?}: {error}"));
    for entry in entries {
        let entry = entry.unwrap_or_else(|error| panic!("failed to read dir entry: {error}"));
        let file_name = entry.file_name();
        let name = file_name
            .to_str()
            .unwrap_or_else(|| panic!("decorator mesh file name is not UTF-8"));
        // O：`4 < len` + 末 5 字节 ".json" 判定。
        if name.len() <= 4 || !name.ends_with(".json") {
            continue;
        }
        let path = entry.path();
        let path_str = path
            .to_str()
            .unwrap_or_else(|| panic!("decorator mesh path is not UTF-8"));
        let mut mesh = load_json_as_mesh(path_str);
        // O：stem（去掉 5 字节扩展名）== "outline" → 顶点色置白。
        let stem = name.strip_suffix(".json").unwrap();
        if stem == "outline" {
            mesh.add_color(Vec3::ONE);
        }
        // O：format context + `load_mesh_into_library`（0x140b1aca0）。
        host.load_mesh_into_library(stem, mesh);
    }
}

/// `load_collision_as_bbx` @0x1407bb4b0（O 结构）。
pub fn load_collision_as_bbx(
    asset_library: &AssetMeshLibrary,
    name: &str,
    _subtype: u8,
) -> Option<RaycastMeshData> {
    let mesh = asset_library.get_by_name(name)?;
    let _bbx = mesh.collision_bbx();
    // 机器码：`new_cuboid(bbx 半extents, 几何哈希)` + box_trigger_volume。
    // core `new_cuboid` 位于 `native-physics` feature（system-decorator 不启用），
    // 故**显式未恢复**返回 None，不再以 `RaycastMeshData::default()` 冒充。
    None
}
