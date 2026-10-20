//! 拱构造（construct_arches/mod.rs + profile.rs）。
//!
//! `construct_arches`（0x14075f680，149KB）。候选参数面（skeleton 富化）：
//! `(EventReader<OnWallChanged>, EventReader<CreateRoofCmd>,
//!  EventReader<SwitchRoofTypeCmd>, ResMut<PublicWalls>,
//!  ResMut<WallPathSegmentationMasks>, …)`。
//!
//! 机器码要点（building-arches findings G05 冻结 130 例/4159 条 96B）：
//! - 消费 136B 处理掩码输入；shape_tag 0/1；has_roof 关系；
//!   Pub294!=0 时选跨段最高 (height+groundY)；第三数组跨段 maxnum 累计；
//!   闭合轮廓 first→last 复制（无 −0.1125）。
//! - `if style == 2` → 跳过共享拱装配路径，直达 MaterialStats 记录
//!   （set_arches_segments_count/add_arches_length/add_gate_length/
//!   set_gates_segments_count）—— **shape2 分支未冻结**（C 版 wooden_arch_frame
//!   未做原码差分；reviewer 登记为未覆盖/边界）。
//! - 产出（W-B2 落盘）：96B WallBrickInstance 记录 + ConstructGateCmd120
//!   （→ construct_gates 消费）+ segment_count → 输出资源写回。
use bevy_ecs::prelude::*;
use country_core::asset_libraries::ssbo_library::AssetSsboLibrary;
use country_core::resources::{
    roofs::{
        Roof,
        commands::{CreateRoofCmd, SwitchRoofTypeCmd},
    },
    walls::public_walls::{OnWallChanged, PublicWalls},
};
use country_core::systems::wall::construction::segmentation::WallPathSegmentationMask;

pub mod profile;

/// 处理掩码资源（B `WallPathSegmentationMasks`：每墙 136B 记录的
/// 结构化投影；segmentation 族写回、arches/lintels 消费）。
#[derive(bevy_ecs::resource::Resource, Clone, Debug, Default)]
pub struct WallPathSegmentationMasks {
    /// 每墙 136B 掩码（分段系统产出；arches/lintels 逐段消费）。
    pub masks: hashbrown::HashMap<u64, Vec<WallPathSegmentationMask>>,
}

/// 构造输出（渲染边界消费；W-B2 落盘）。
#[derive(bevy_ecs::resource::Resource, Default)]
pub struct WallConstructorOutput {
    /// Actual per-source sparse owners; CPU vectors below are legacy producer
    /// seams and must not be treated as uploaded instances.
    pub gpu: crate::gpu_output::ConstructorGpuOutput,
    /// 96B 砖实例（arches/construct_walls 产出）。
    pub brick_instances: Vec<[u8; 96]>,
    /// 64B 木板实例（fences/lintels 产出）。
    pub plank_instances: Vec<[u8; 64]>,
    /// 门构造命令（arches 产出 → construct_gates 消费）。
    pub gate_commands: Vec<[u8; 128]>,
    /// 门扇 136B spawn payload（gates 产出 → 渲染边界）。
    pub door_spawn_payloads: Vec<[u8; 136]>,
}

/// B 0x14075f680。
pub fn construct_arches(
    mut changed: EventReader<OnWallChanged>,
    mut create_roof: EventReader<CreateRoofCmd>,
    mut switch_type: EventReader<SwitchRoofTypeCmd>,
    mut walls: ResMut<PublicWalls>,
    masks: Res<crate::construct_lintels::WallPathSegmentationMasksMinusStairs>,
    mut arch_segments: ResMut<country_core::resources::walls::arches::ArchSegments>,
    roofs: Query<&Roof>,
    mut previous: ResMut<country_core::resources::walls::arches::BuildingHadArchesLastFrame>,
    mut recalculate: EventWriter<country_core::resources::roofs::commands::RecalculateRoofCmd>,
    ssbo: Res<AssetSsboLibrary>,
    mut output: ResMut<WallConstructorOutput>,
    mut gates: EventWriter<crate::construct_gates::systems::ConstructGateCmd>,
) {
    let recalc: Vec<u64> = changed
        .read()
        .map(|e| e.0.0)
        .chain(create_roof.read().map(|e| e.0.wall_id().0))
        .chain(switch_type.read().map(|e| e.wall.0))
        .collect();
    for wall_id in recalc {
        let id = country_core::resources::walls::WallId(wall_id);
        // Native76047b calls PublicWalls::has_roof, including its failure-map
        // and generation test. Query validation belongs only to the tail below.
        let has_roof = walls.has_roof(id);
        let segments = masks
            .masks
            .get(&wall_id)
            .expect("arches require processed masks for the source wall");
        let wall = walls
            .get_mut(id)
            .expect("arch event must reference a published wall");
        let outcome = if wall.building_projection().observed_296() != 0 {
            country_core::systems::wall::construction::arches::WallArchesOutput::default()
        } else {
            wall.construct_arches(id, segments, has_roof)
                .expect("invalid or unrecovered authoritative arch input")
        };
        {
            let stats = &outcome.material_stats;
            wall.material_stats
                .set_arches_segments_count(stats.arches_segments_count());
            wall.material_stats.add_arches_length(stats.arches_length);
            wall.material_stats.add_gate_length(stats.gates_length);
            wall.material_stats
                .set_gates_segments_count(stats.gates_count as u32);
            wall.material_stats.add_sewer_length(stats.sewer_length);
            wall.material_stats
                .set_sewer_segments_count(stats.sewer_count as u32);
            arch_segments.0.insert(id, outcome.segments);
            // W-B2：96B 实例 + ConstructGateCmd120 落盘。
            output.gpu.replace_bricks(
                crate::gpu_output::ConstructorKey {
                    wall: wall_id,
                    part: crate::gpu_output::ConstructorPart::Arches,
                },
                outcome.instances,
            );
            for gate in &outcome.gate_commands {
                gates.write(crate::construct_gates::systems::ConstructGateCmd {
                    wall: gate.wall_id.0,
                    ordinal: gate.ordinal,
                    segment: gate.segment.clone(),
                });
            }
            let _ = outcome.segment_count;
        }
        // shape2 分支（style==2 → MaterialStats 直写）——未冻结边界登记。
        if segments
            .first()
            .and_then(|s| s.flags.first().copied())
            .unwrap_or(0)
            & 2
            != 0
        {
            let _ = ssbo;
        }
    }
    // Original762ce0..7630a5 scans all public rows every invocation, not only
    // this frame's event IDs. Only a live, non-flat associated roof participates.
    for (id, _) in walls.all() {
        let Some(entity) = walls
            .roof_entity(id)
            .filter(|entity| entity.to_bits() >> 32 != 0)
        else {
            continue;
        };
        let Ok(roof) = roofs.get(entity) else {
            continue;
        };
        if roof.is_flat() {
            continue;
        }
        let now = arch_segments
            .0
            .get(&id)
            .is_some_and(|segments| segments.len() >= 2);
        let before = previous.0.get(&id.0).copied().unwrap_or(false);
        if before != now {
            recalculate.write(
                country_core::resources::roofs::commands::RecalculateRoofCmd {
                    entity,
                    wall: id,
                    kind: 3,
                },
            );
        }
        previous.0.insert(id.0, now);
    }
}
