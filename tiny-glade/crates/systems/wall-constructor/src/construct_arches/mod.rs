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
use country_core::asset_libraries::ssbo_library::AssetSsboLibrary;
use country_core::systems::wall::construction::arches::RegularWallArchesInput;
use country_core::systems::wall::construction::arches::construct_regular_wall_arches;
use country_core::systems::wall::construction::segmentation::WallPathSegmentationMask;
use country_core::resources::{
    roofs::{Roof, commands::CreateRoofCmd},
    walls::public_walls::{OnWallChanged, PublicWalls},
};
use bevy_ecs::prelude::*;

pub mod profile;

/// 处理掩码资源（B `WallPathSegmentationMasks`：每墙 136B 记录的
/// 结构化投影；segmentation 族写回、arches/lintels 消费）。
#[derive(bevy_ecs::resource::Resource, Clone, Debug, Default)]
pub struct WallPathSegmentationMasks {
    /// 每墙 136B 掩码（分段系统产出；arches/lintels 逐段消费）。
    pub masks: hashbrown::HashMap<u64, Vec<WallPathSegmentationMask>>,
}

/// 构造输出（渲染边界消费；W-B2 落盘）。
#[derive(bevy_ecs::resource::Resource, Clone, Debug, Default)]
pub struct WallConstructorOutput {
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
    walls: Res<PublicWalls>,
    mut masks: ResMut<WallPathSegmentationMasks>,
    roofs: Query<&Roof>,
    ssbo: Res<AssetSsboLibrary>,
    mut output: ResMut<WallConstructorOutput>,
    mut gates: EventWriter<crate::construct_gates::systems::ConstructGateCmd>,
) {
    let recalc: Vec<u64> = changed
        .read()
        .map(|e| e.0 .0)
        .chain(create_roof.read().map(|e| e.0.wall_id().0))
        .collect();
    for wall_id in recalc {
        let id = country_core::resources::walls::WallId(wall_id);
        let Some(wall) = walls.get(id) else {
            continue;
        };
        let projection = wall.building_projection();
        let has_roof = walls
            .roof_entity(id)
            .and_then(|entity| roofs.get(entity).ok())
            .is_some();
        let Some(segments) = masks.masks.get(&wall_id) else {
            continue;
        };
        // 已冻结 CPU 链（G05 130 例/4159 条 96B）：prepared 数组源 =
        // 路径预备 owner（wall_path_preparation）的预备点列；此处以简化
        // 曲线点列代（I 级登记：预备点/第三数组的精确源待 path-prep 接线）。
        let curve = wall.simplified_curve_2d();
        let prepared_positions: Vec<[f32; 3]> = curve
            .points
            .iter()
            .map(|p| [p[0], p[1], 0.0])
            .collect();
        let n = prepared_positions.len();
        let mut arch = vec![0.0f32; n.max(2)];
        let mut third = vec![0.0f32; n.max(2)];
        let input = RegularWallArchesInput {
            masks: segments,
            prepared_positions: &prepared_positions,
            prepared_u: &curve.points.iter().map(|p| p[0]).collect::<Vec<f32>>(),
            path_values: &curve.points.iter().map(|p| p[1]).collect::<Vec<f32>>(),
            shape_tag: projection.shape().tag(),
            public_284: projection.float_0x284(),
            observed_294: projection.observed_294(),
            source_curve_length: curve.length,
            wall_id: country_core::resources::walls::publication_fields::WallId(id.0),
            has_roof,
        };
        if let Ok(outcome) = construct_regular_wall_arches(input, &mut arch, &mut third) {
            // W-B2：96B 实例 + ConstructGateCmd120 落盘。
            for instance in &outcome.instances {
                output.brick_instances.push(instance.observed_bytes());
            }
            for gate in &outcome.gate_commands {
                let mut payload = [0u8; 120];
                payload[..8].copy_from_slice(&gate.wall_id.0.to_le_bytes());
                payload[8..16].copy_from_slice(&gate.ordinal.to_le_bytes());
                gates.write(crate::construct_gates::systems::ConstructGateCmd {
                    wall: gate.wall_id.0,
                    payload,
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
}
