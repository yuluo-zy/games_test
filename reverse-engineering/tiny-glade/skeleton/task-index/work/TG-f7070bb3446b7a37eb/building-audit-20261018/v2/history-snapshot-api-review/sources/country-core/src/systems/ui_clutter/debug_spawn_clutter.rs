//! `debug_spawn_clutter.rs` — 调试批量生成 clutter（B 0x1409a1f50，O）。
//!
//! 证据：fresh 反编译（ui_clutter-0x1409a1f50）+ re-analyst-interaction：
//! - 消费 `DebugSpawnClutterCmd` / `DebugSpawnAllClutterCmd` 双事件队列；
//! - **两循环都调 `History::start_input_recording()`**（早期 Rust 完全缺）；
//! - 单条：推 0x48 B spawn 事件（tag=2、subtype=3 @+0x29、目标 entity@+0x40）；**位置来自
//!   事件载荷（`uStack_9a`），不采地形高度**（早期 Rust 用 `(0, height.sample, 0)`）；
//! - 批量：螺旋 0x2a=42 个——`r = sqrt(n)`、`θ = n * _UNK_142abbcec`、cos/sin 定位，
//!   逐项类型字节 `(&UNK_142aa0013)[n]` + 地形高度采样（`FUN_1409e9640` =
//!   `TerrainHeightsData::sample_at_world_xz`），推 0x48 B 事件（@+0x30/0x38 = 位置、
//!   **@+0x2c 旋转 = 0**——早期 Rust 写 `n·SPIRAL_ANGLE` 错）。
//!
//! 0x48 B spawn 事件的消费端（deco-clutter 生成）依赖装饰存储（shared building modules，
//! 宿主）；本实现产出 crate 自有 [`SpawnAutoClutter`] 事件与螺旋位置。
use bevy_ecs::prelude::*;
use glam::Vec3;

use crate::{
    resources::{history::History, terrain_height::{TerrainHeightsData, TerrainHeightSsbo}},
    systems::mode_manager::{AppMode, ClutterMeshTy},
};

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

/// 单条调试生成命令（I：载荷 = 类型 + 旋转 + 位置；原生事件在 0x48 B spawn 记录的
/// `+0x2a..+0x39` 一次性写入 rotation/x/y/z（`uStack_9a` 两段 8 B 写），位置**不采**
/// 地形高度）。
#[derive(Event, Clone, Copy, Debug)]
pub struct DebugSpawnClutterCmd {
    pub ty: ClutterMeshTy,
    pub position: Vec3,
    /// O：单条路径 rotation @+0x2c 来自事件载荷（与批量路径的固定 0 不同）。
    pub rotation: f32,
}

/// 批量调试生成命令（I：空载荷）。
#[derive(Event, Clone, Copy, Debug, Default)]
pub struct DebugSpawnAllClutterCmd;

/// 0x48 B spawn 事件（O 布局：tag=2@0、subtype=3@+0x29、rotation@+0x2c=0、
/// position@+0x30/+0x38、type@+0x3c、entity@+0x40）。
#[derive(Event, Clone, Copy, Debug)]
pub struct SpawnAutoClutter {
    pub entity: u64,
    pub ty: ClutterMeshTy,
    pub position: Vec3,
    pub rotation: f32,
}

/// 螺旋位置（O：`r = sqrt(n)`、`θ = n * _UNK_142abbcec`、cos/sin）。
pub fn spiral_position(index: usize) -> (f32, f32) {
    let n = index as f32;
    let r = n.sqrt();
    let theta = n * SPIRAL_ANGLE;
    (r * theta.cos(), r * theta.sin())
}

/// `debug_spawn_all_clutter` @0x1409a1f50（O 结构）。
pub fn debug_spawn_all_clutter<S: Send + Sync + 'static>(
    mut single: EventReader<DebugSpawnClutterCmd>,
    mut all: EventReader<DebugSpawnAllClutterCmd>,
    height: Res<TerrainHeightsData<TerrainHeightSsbo>>,
    mode: Res<AppMode>,
    mut history: ResMut<History<S>>,
    mut spawned: EventWriter<SpawnAutoClutter>,
    mut next_entity: ResMut<DebugClutterEntityCounter>,
) {
    for cmd in single.read() {
        // O：每事件先 `History::start_input_recording()`（早期 Rust 缺）。
        let _ = history.start_input_recording(mode.native_tag());
        let entity = next_entity.0;
        next_entity.0 = next_entity.0.wrapping_add(1);
        // **单条位置/旋转来自事件载荷（uStack_9a），不采地形高度**。
        spawned.write(SpawnAutoClutter {
            entity,
            ty: cmd.ty,
            position: cmd.position,
            rotation: cmd.rotation,
        });
    }
    for _ in all.read() {
        let _ = history.start_input_recording(mode.native_tag());
        for n in 0..DEBUG_SPAWN_COUNT {
            let (x, z) = spiral_position(n);
            let entity = next_entity.0;
            next_entity.0 = next_entity.0.wrapping_add(1);
            // 类型字节表 `&UNK_142aa0013`（O，re-analyst 直读）；高度经 `FUN_1409e9640`
            // = `TerrainHeightsData::sample_at_world_xz` 采样。
            let ty = ClutterMeshTy::from_native_tag(DEBUG_SPAWN_TYPES[n])
                .unwrap_or(ClutterMeshTy::PlantPotV1);
            let height = height.sample_at_world_xz(x, z);
            spawned.write(SpawnAutoClutter {
                entity,
                ty,
                position: Vec3::new(x, height, z),
                // **旋转 @+0x2c = 0**（早期 Rust 写 `n·SPIRAL_ANGLE` 错）。
                rotation: 0.0,
            });
        }
    }
}

/// 调试生成实体计数（I：宿主递增；原生用 Events 内部计数 @+0x40，早期 Rust
/// 独立计数器，标注 I）。
#[derive(Resource, Clone, Copy, Debug, Default)]
pub struct DebugClutterEntityCounter(pub u64);
