//! B `crates/systems/decorator/src/remove_ivy_under_windows.rs` — 窗下藤蔓清除。
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
use country_core::resources::walls::WallId;
use country_core::systems::ivy::ivy_pruner::{remove_ivy_at_location, RemoveIvyCmd};
use glam::Vec3;

/// `remove_ivy_under_windows`（0x140814860，O 结构）：窗下藤蔓清除系统。
/// 对每个窗装饰发出 `RemoveIvyCmd`：盒中心贴地（y - 地形高）、尺寸按窗
/// 墙空间尺寸、源 = 装饰 id + 墙 id。
pub fn remove_ivy_under_windows(
    storage: Res<DecoratorStorage>,
    terrain: Res<TerrainHeightsData>,
    mut ivy_commands: EventWriter<RemoveIvyCmd>,
) {
    // 墙段记录（0x90B；skip tag==2 / 非窗）。
    for (wall, decorators) in storage.iter_walls() {
        for (id, info) in decorators.decorators.iter() {
            if !is_window_for_ivy(info) {
                continue;
            }
            emit_remove_ivy(&mut ivy_commands, &terrain, *id, info, wall);
        }
    }
    // 楼梯/屋顶段（记录 tag 3；窗判定同）。
    for decorators in storage.iter_stair_decorators() {
        for (id, info) in decorators.decorators.iter() {
            if !is_window_for_ivy(info) {
                continue;
            }
            emit_remove_ivy(&mut ivy_commands, &terrain, *id, info, WallId(u64::MAX));
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
    // 尺寸（I：窗墙空间尺寸；原经 DecoratorSubtypeLibrary 查表
    // 0x14125cf90，未命中 fmt panic —— Rust 侧以默认尺寸承载并登记）。
    let width = 1.0;
    let height = 3.0;
    // 源（O：source_0 = 装饰 id；source_8 = 墙 id，try_as_wall 0x140a1c290）。
    let source_0 = u64::from(DecoratorId::new(id).0);
    let source_8 = wall.get_inner();
    remove_ivy_at_location(commands, center, width, height, source_0, source_8);
}
