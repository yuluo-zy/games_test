//! `debug_spawn_clutter.rs` — 调试批量生成 clutter（B 0x1409a1f50，O）。
//!
//! 证据：`chapters/systems/ui_clutter/debug_spawn_clutter.rs.md`、re-analyst-interaction：
//! - 消费 `DebugSpawnClutterCmd` / `DebugSpawnAllClutterCmd` 双事件队列；
//! - 单条：推 0x48 B spawn 事件（tag=2、subtype=3 @+0x29、目标 entity@+0x40）；
//! - 批量：螺旋 0x2a=42 个——`r = sqrt(n)`、`θ = n * _UNK_142abbcec`、cos/sin 定位，
//!   逐项类型字节 `(&UNK_142aa0013)[n]` + 地形高度采样（`FUN_1409e9640` =
//!   `TerrainHeightsData::sample_at_world_xz`），推 0x48 B 事件（@+0x30/0x38 = 位置、
//!   @+0x2c = 旋转）。
//!
//! 0x48 B spawn 事件的消费端（deco-clutter 生成）依赖装饰存储（recovered_building，
//! 宿主）；本实现产出 crate 自有 [`SpawnAutoClutter`] 事件与螺旋位置。
use bevy_ecs::prelude::*;
use glam::Vec3;

use crate::systems::mode_manager::ClutterMeshTy;

/// 螺旋角度常数（O：`_UNK_142abbcec` = 2.399827718734741，re-analyst RNE 直读）。
const SPIRAL_ANGLE: f32 = 2.399_827_7;

/// 批量数量（O：0x2a = 42）。
pub const DEBUG_SPAWN_COUNT: usize = 0x2a;

/// 42 项类型字节表（O：RNE 直读 `&UNK_142aa0013`，实测字节
/// `[0,15,43,19,1,2,3,4,5,6,7,8,9,10,11,13,12,16,17,18,20,21,22,37,38,39,40,54,23,24,41,
/// 55,25,26,44,45,46,47,14,49,48,50]`，证据
/// reports/interaction/ui_clutter/evidence/read-memory-0x142aa0013-types-0x142abbcec-angle.json）。
pub const DEBUG_SPAWN_TYPES: [u8; DEBUG_SPAWN_COUNT] = [
    0, 15, 43, 19, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 13, 12, 16, 17, 18, 20, 21, 22, 37,
    38, 39, 40, 54, 23, 24, 41, 55, 25, 26, 44, 45, 46, 47, 14, 49, 48, 50,
];

/// 单条调试生成命令（I：载荷 = 类型）。
#[derive(Event, Clone, Copy, Debug)]
pub struct DebugSpawnClutterCmd(pub ClutterMeshTy);

/// 批量调试生成命令（I：空载荷）。
#[derive(Event, Clone, Copy, Debug, Default)]
pub struct DebugSpawnAllClutterCmd;

/// 0x48 B spawn 事件（O 布局：tag=2@0、subtype=3@+0x29、position@+0x30/+0x38、
/// rotation@+0x2c、type@+0x3c、entity@+0x40）。
#[derive(Event, Clone, Copy, Debug)]
pub struct SpawnAutoClutter {
    pub entity: u64,
    pub ty: ClutterMeshTy,
    pub position: Vec3,
    pub rotation: f32,
}

/// 地形高度采样适配（I：TerrainHeightsData 非 Resource，宿主从 CPU 高度数据拷贝）。
#[derive(Resource, Clone, Copy, Debug, Default)]
pub struct DebugTerrainHeight {
    pub sample: f32,
}

/// 螺旋位置（O：`r = sqrt(n)`、`θ = n * _UNK_142abbcec`、cos/sin）。
pub fn spiral_position(index: usize) -> (f32, f32) {
    let n = index as f32;
    let r = n.sqrt();
    let theta = n * SPIRAL_ANGLE;
    (r * theta.cos(), r * theta.sin())
}

/// `debug_spawn_all_clutter` @0x1409a1f50（O 结构）。
pub fn debug_spawn_all_clutter(
    mut single: EventReader<DebugSpawnClutterCmd>,
    mut all: EventReader<DebugSpawnAllClutterCmd>,
    height: Res<DebugTerrainHeight>,
    mut spawned: EventWriter<SpawnAutoClutter>,
    mut next_entity: ResMut<DebugClutterEntityCounter>,
) {
    for cmd in single.read() {
        let entity = next_entity.0;
        next_entity.0 = next_entity.0.wrapping_add(1);
        spawned.write(SpawnAutoClutter {
            entity,
            ty: cmd.0,
            position: Vec3::new(0.0, height.sample, 0.0),
            rotation: 0.0,
        });
    }
    for _ in all.read() {
        for n in 0..DEBUG_SPAWN_COUNT {
            let (x, z) = spiral_position(n);
            let entity = next_entity.0;
            next_entity.0 = next_entity.0.wrapping_add(1);
            // 类型字节表 `&UNK_142aa0013`（O，re-analyst 直读）；高度经 `FUN_1409e9640`
            // 采样（I：宿主已把最近采样放入 [`DebugTerrainHeight`]）。
            let ty = ClutterMeshTy::from_native_tag(DEBUG_SPAWN_TYPES[n])
                .unwrap_or(ClutterMeshTy::PlantPotV1);
            spawned.write(SpawnAutoClutter {
                entity,
                ty,
                position: Vec3::new(x, height.sample, z),
                rotation: theta_for_spiral(n),
            });
        }
    }
}

/// 批量项旋转（I：@+0x2c = 旋转；本实现取螺旋角）。
pub fn theta_for_spiral(index: usize) -> f32 {
    index as f32 * SPIRAL_ANGLE
}

/// 调试生成实体计数（I：宿主递增；spawn 端实体位集待取证）。
#[derive(Resource, Clone, Copy, Debug, Default)]
pub struct DebugClutterEntityCounter(pub u64);
