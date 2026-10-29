//! add_autoclutter_around_gates（B 主基线；0x1422cc390，size 2225）。
//!
//! 2026-10-17 restorer-clutter-remaining：按 fresh 反编译 + 逐指令反汇编重建
//! （`evidence/decomp/0x1422cc390_*.c`、`evidence/disasm6_1.txt`）。
//!
//! 控制流（O）：
//! - 宿主记录 stride **0x38**：`+0x08` 曲线数据、`+0x10` 曲线点数、`+0x18` WallId、
//!   `+0x20` 方向 Vec2、`+0x28` 方向 Vec2、`+0x30` u32、`+0x34` u8。
//! - 曲线点数 `> 2` 才放置（O：`2 < hiVar`）；首点用于 `gate_right`、末点用于
//!   `gate_left`（各 +2.0 y）。
//! - 每放置：方向 `dir = 宿主 +0x20/+0x28`；`normal = (-dy, dx)/|dir|`；
//!   区域角点 = 端点 + normal×side + dir − 旋转尺寸对角线×0.5。
//! - 键 `calculate_hash<tuple<&str,u32,WallId>>`（名字、+0x30 u32、+0x18 WallId）。
//! - 区域记录：kind=3、候选索引 **0xffff**、`+0x34` 字节；每门 2 个 `add_region`。
//!
//! 原指令 1422cc91c..ccaed 确认上述计算、轴 dir、尺寸(2,2)和 version 的字节偏移；
//! 数值原侧差分尚未运行。

use bevy_ecs::prelude::*;
use glam::{Vec2, Vec3};

use super::{AutoClutterRegion, AutoClutterState, ClutterCandidateIndex, ClutterRegionId};
use country_core::resources::walls::WallId;

/// 门放置宿主记录（stride 0x38；宿主装配时填充）。
pub use country_core::components::WoodenGate as GateHost;

/// 门放置名（O：`gate_right` len 10 / `gate_left` len 9，左右顺序固定）。
pub const GATE_RIGHT: &str = "gate_right";
pub const GATE_LEFT: &str = "gate_left";

/// add_autoclutter_around_gates（0x1422cc390）。
/// 每门：曲线 >2 点才放置；首/末点各 1 区域候选。
pub fn add_autoclutter_around_gates(
    mut state: ResMut<AutoClutterState>,
    mut query: Query<&GateHost>,
) {
    for gate in &mut query {
        // O：曲线点数 > 2 才处理。
        if gate.top_positions.len() <= 2 {
            continue;
        }
        let first = Vec3::from_array(gate.top_positions[0]);
        let last = Vec3::from_array(gate.top_positions[gate.top_positions.len() - 1]);
        for (i, (name, dir, point, side)) in [
            (GATE_RIGHT, Vec2::from_array(gate.endpoint_directions[0]), first + Vec3::Y * 2.0, -1.0f32),
            (GATE_LEFT, Vec2::from_array(gate.endpoint_directions[1]), last + Vec3::Y * 2.0, 1.0f32),
        ]
        .into_iter()
        .enumerate()
        {
            // O：normal = (-dy, dx)/|d|。
            let len = (dir.x * dir.x + dir.y * dir.y).sqrt();
            // 原指令对零长度也执行 1/sqrt，不跳过无序结果。
            let inv = 1.0 / len;
            let normal = Vec2::new(-dir.y * inv, dir.x * inv);
            let rot = Vec2::new((dir.x + dir.x) - (dir.y + dir.y),
                (dir.y + dir.y) + (dir.x + dir.x));
            // B 1422cca26..1422cca85：区域原点是端点加侧向偏移、
            // 方向项，再减去 2×2 矩形旋转对角线的一半。
            let pos = Vec3::new(
                (point.x + ((normal.x + normal.x) * 0.5) * side)
                    + (dir.x + dir.x) * 0.5 - rot.x * 0.5,
                point.y,
                (point.z + ((normal.y + normal.y) * 0.5) * side)
                    + (dir.y + dir.y) * 0.5 - rot.y * 0.5,
            );
            // O：键 tuple<&str,u32,WallId>。
            let key = hash_gate_key(name, gate.seed, gate.wall.get_inner());
            let region = AutoClutterRegion {
                // B 1422cca：tag3、墙 ID、门 seed 是区域来源，而非区域哈希。
                source: Some(country_core::components::drawable::AutoClutterSource::Id {
                    tag: country_core::components::drawable::AutoClutterSourceIdTag::Native3,
                    native_u64_08: gate.wall.0,
                    native_u32_10: gate.seed,
                }),
                kind: 3,
                key: ClutterRegionId(key),
                center: pos,
                basis: [dir, Vec2::splat(2.0)],
                f34: 0,
                b38: 0,
                b39: gate.clutter_version,
                // O：候选索引初值 0xffff。
                candidate_index: ClutterCandidateIndex::NONE,
                epoch: state.epoch,
            };
            state.add_region(ClutterRegionId(key), &region);
            let _ = i;
        }
    }
}

/// 门放置键（`calculate_hash<tuple<&str,u32,WallId>>` 0x141cf7440 语义）。
pub fn hash_gate_key(name: &str, u32_field: u32, wall: u64) -> u64 {
    utils::calculate_hash(&(name, u32_field, WallId(wall)))
}

#[cfg(test)]
mod canonical_gate_tests {
    use super::*;

    #[test]
    fn constructor_component_identity_reaches_real_scatter_query_without_a_mirror() {
        let mut world = World::new();
        world.insert_resource(AutoClutterState {
            lazy_grid: vec![Vec::new(); 4096],
            ..Default::default()
        });
        let gate = world.spawn(country_core::components::WoodenGate {
            top_positions: vec![[1.0, 2.0, 3.0], [2.0, 2.0, 3.0], [3.0, 2.0, 3.0]],
            wall: WallId(5),
            endpoint_directions: [[1.0, 0.0], [1.0, 0.0]],
            seed: 17,
            clutter_version: 23,
        }).id();
        let mut schedule = Schedule::default();
        schedule.add_systems(add_autoclutter_around_gates);
        schedule.run(&mut world);
        let state = world.resource::<AutoClutterState>();
        assert_eq!(state.regions.len(), 2);
        for name in [GATE_RIGHT, GATE_LEFT] {
            let region = &state.regions[&ClutterRegionId(hash_gate_key(name, 17, 5))];
            assert_eq!(region.kind, 3);
            assert_eq!(region.b38, 0);
            assert_eq!(region.b39, 23);
            assert_eq!(region.basis, [Vec2::X, Vec2::splat(2.0)]);
            let center = region.rectangle2d().center;
            assert_eq!(center, if name == GATE_RIGHT { Vec2::new(2.0, 2.0) } else { Vec2::new(4.0, 4.0) });
            assert!(matches!(&region.source,
                Some(country_core::components::drawable::AutoClutterSource::Id {
                    tag: country_core::components::drawable::AutoClutterSourceIdTag::Native3,
                    native_u64_08: 5, native_u32_10: 17,
                })));
        }
        assert!(world.get::<GateHost>(gate).is_some());
        world.despawn(gate);
        assert!(world.get::<GateHost>(gate).is_none());
    }
}
