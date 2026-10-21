//! B `crates/systems/decorator/src/remove_ivy_under_windows.rs` — 窗下藤蔓清除。
//!
//! 本轮仅闭合实际ID→subtype+rank→canonical碰撞宽高的读取。
//! 原事件流、cached世界位置与删除传播仍未完整复现；下面旧遍历/坐标模型
//! 不因这次数据接口迁移而获得完整业务验收。
//!
//! 证据（fresh RE-MCP，decomp-140814860.txt，O 结构）：
//! - `remove_ivy_under_windows` 0x140814860：遍历存储记录（0x90B 段双表；
//!   记录 tag 判别 `tag-2 ∈ {…}`：1=墙记录、3=楼梯/屋顶记录、2/4=跳过），
//!   对窗类记录（subtype/rank 条件 + `DecoratorStorage::get` 内嵌 map 命中）
//!   计算清除盒 → `remove_ivy_at_location`（0x140999860，core O：
//!   `(commands, center, width, height, source_0, source_8)` → 0x30B
//!   `RemoveIvyCmd` 事件）；
//! - 盒计算（O）：center = (x, y - terrain_height(x,z), z)（`TerrainHeightsData::
//!   sample_at_world_xz` 0x140257ba0 浮点栅格），width/height = 窗墙空间尺寸
//!   （`DerivedDecoratorInfo` + `dims` 查表 0x14125cf90，未命中 fmt panic
//!   0x142c21c80/0x142c21c90）；source_0 = 装饰 id，source_8 = 墙 id
//!   （`DecoratorStorageAddr::try_as_wall` 0x140a1c290）。
//!
//! 系统参数面（I，待 reviewer 核对）：Res<DecoratorStorage>（记录遍历）+
//! Res<TerrainHeightsData> + EventWriter<RemoveIvyCmd> + 附加 ResMut
//! （tick@+0x1c，身份挂账 RESTORE.md）。

use bevy_ecs::prelude::*;
use country_core::resources::terrain_height::TerrainHeightsData;
use country_core::resources::walls::decorator::{DecoratorId, DecoratorInfo};
use country_core::resources::walls::decorator_storage::DecoratorStorage;
use country_core::resources::walls::decorator_subtype::{
    DecoratorLibKey, DerivedDecoratorInfoResources, DerivedDecoratorInfoView,
};
use country_core::resources::walls::WallId;
use country_core::systems::ivy::ivy_pruner::{remove_ivy_at_location, RemoveIvyCmd};
use glam::{Vec2, Vec3};

/// `remove_ivy_under_windows`（0x140814860，O 结构）：窗下藤蔓清除系统。
/// 对每个窗装饰发出 `RemoveIvyCmd`：盒中心贴地（y - 地形高）、尺寸按窗
/// 墙空间尺寸、源 = 装饰 id + 墙 id。
pub fn remove_ivy_under_windows(
    storage: Res<DecoratorStorage>,
    terrain: Res<TerrainHeightsData>,
    derived: DerivedDecoratorInfoResources,
    mut ivy_commands: EventWriter<RemoveIvyCmd>,
) {
    let derived = derived.as_view();
    // 墙段记录（0x90B；skip tag==2 / 非窗）。
    for (wall, decorators) in storage.iter_walls() {
        for (id, info) in decorators.decorators.iter() {
            if !is_window_for_ivy(info) {
                continue;
            }
            emit_remove_ivy(&mut ivy_commands, &terrain, derived, *id, info, wall);
        }
    }
    // 楼梯/屋顶段（记录 tag 3；窗判定同）。
    for decorators in storage.iter_stair_decorators() {
        for (id, info) in decorators.decorators.iter() {
            if !is_window_for_ivy(info) {
                continue;
            }
            emit_remove_ivy(
                &mut ivy_commands,
                &terrain,
                derived,
                *id,
                info,
                WallId(u64::MAX),
            );
        }
    }
}

/// 窗判定（O 结构）：记录 tag==1 分支（`dst tag < 2` 墙附着）+ 位置字节
/// `+0x10 == 0` 且 rank 字节 `+0x3a` 条件（I：`(b-2)>2` 门控窗族）。
fn is_window_for_ivy(info: &DecoratorInfo) -> bool {
    let t = info.decorator_type();
    // 窗族 {0x38,0x39,0x3a}（O：is_window 谓词 0x140833160）。
    t.is_window() && info.u45 == 0
}

/// 单窗 → RemoveIvyCmd（O：中心/尺寸/源计算）。
fn emit_remove_ivy(
    commands: &mut EventWriter<RemoveIvyCmd>,
    terrain: &TerrainHeightsData,
    derived: DerivedDecoratorInfoView<'_>,
    id: u32,
    info: &DecoratorInfo,
    wall: WallId,
) {
    // 位置（记录 +0x2c/+0x34 打包；I：精确字段语义见 remove_ivy 反编译）。
    let x = f32::from_bits(info.u24);
    let z = f32::from_bits(info.u28);
    // 地形高（O：sample_at_world_xz；盒中心贴地）。
    let ground = terrain.sample_at_world_xz(x, z);
    let center = Vec3::new(x, -ground, z);
    // 尺寸（O：`DecoratorLibKey::new` → 库查表 0x14125cf90；未命中
    // fmt panic 0x142c21c80/0x142c21c90，不回落默认尺寸）。
    let dims = collision_clearance_dimensions(derived, id, info.rank());
    let width = dims.x;
    let height = dims.y;
    // 源（O：source_0 = 装饰 id；source_8 = 墙 id，try_as_wall 0x140a1c290）。
    let source_0 = u64::from(DecoratorId::new(id).0);
    let source_8 = wall.get_inner();
    remove_ivy_at_location(commands, center, width, height, source_0, source_8);
}

/// 原a62/da6通过实际ID取subtype，a94/dd7的逻辑尺寸按原+0/+1算序扩张。
fn collision_clearance_dimensions(
    derived: DerivedDecoratorInfoView<'_>,
    id: u32,
    rank: u8,
) -> Vec2 {
    let subtype = derived.subtype(DecoratorId(id));
    let collision = derived.mesh_library().collision_record(DecoratorLibKey::new(subtype.0, rank));
    Vec2::new(collision.bounds.footprint.size.x + 0.0,
        (collision.bounds.top - collision.bounds.bottom) + 1.0)
}

#[cfg(test)]
mod canonical_ivy_dimensions_regressions {
    use super::*;
    use std::sync::Arc;
    use country_core::resources::walls::decorator_subtype::{
        DecoratorCollision, DecoratorCollisionOutline, DecoratorMeshLibrary,
        DecoratorOutline, DecoratorSubtype, DerivedDecoratorSubtypes,
    };
    use country_core::systems::collision::world::RaycastMeshData;
    use country_core::systems::wall::shapes::rectangle::ui_edit_rectangle_orientation::box_trigger_volume;
    use country_core::utils::components::transform::Transform;
    use utils::geometry::Rectangle3d;

    #[test]
    fn actual_id_rank_and_bounds_supply_ivy_clearance_without_mesh_cloning() {
        let mut subtypes = DerivedDecoratorSubtypes::default();
        subtypes.insert(17, DecoratorSubtype(3));
        let mut library = DecoratorMeshLibrary::default();
        let size = Vec3::new(2.0, 4.0, 7.0);
        let center = Vec3::new(5.0, 13.0, -2.0);
        let transform = Transform { translation: center, ..Transform::identity() };
        let mesh = Arc::new(RaycastMeshData::new_cuboid_at(size, center));
        let probe = Arc::clone(&mesh);
        library.insert_collision_and_interaction_as_same(DecoratorLibKey::new(3, 2), DecoratorCollision {
            triangles: box_trigger_volume(size.x, size.y, size.z, &transform),
            mesh,
            bounds: Rectangle3d::new(center, size, Vec2::Y),
        }, DecoratorOutline::None, DecoratorCollisionOutline::None);
        let count = Arc::strong_count(&probe);
        let derived = DerivedDecoratorInfoView::new(&subtypes, &library);
        assert_eq!(collision_clearance_dimensions(derived, 17, 2), Vec2::new(2.0, 5.0));
        assert_eq!(Arc::strong_count(&probe), count);
        for (id, rank) in [(17, 1), (18, 2)] {
            assert!(std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
                collision_clearance_dimensions(derived, id, rank)
            })).is_err());
        }
        library.collision.clear();
        assert!(std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
            collision_clearance_dimensions(DerivedDecoratorInfoView::new(&subtypes, &library), 17, 2)
        })).is_err());
    }
}
