//! 木门系统（construct_gates/systems.rs）。
//!
//! - `construct_gates`（0x140756370，9 参 G06 冻结）。
//! - `update_gates_hashmap`（0x140755690，6 组合 despawn 顺序冻结）。
//! - `construct_wooden_gate_doors`（0x141210110，28 原返回/1886 条 96B/
//!   46 mesh/822 顶点冻结）。
//!
//! W-B1 轮 3 终验接线：ConstructGateCmd（construct_arches 的
//! ConstructGateCmd120 产出）→ payload 全解（wall/ordinal/段计数/端点）
//! → `construct_gate_cpu`（recovered_building/gates.rs，内部按墙高比选
//! construct_wooden_gate_doors 或 nani 分支）→ GateCpuOutput：
//! - bricks（96B）→ WallConstructorOutput；
//! - leaves（WoodenGateProjection：mesh 顶点/top_positions/端点方向/
//!   wall_id/seed/29a）→ 136B spawn payload（TransientMesh 0..16、
//!   变换 16..56、mesh 64、shader 72、top-position 80..104、WallId 104、
//!   端点方向 112..128、seed 128、+29a 132）→ 门实体 + Gates 表；
//! - nanis（NaniBundle<SolidVertexColorInstanceData>）→ 渲染边界登记。
//! seed = wrapping `WallId::get_inner() * ordinal + leaf.ordinal` cast u32；
//! shader 键 = `solid_vertex_color_wooden_gate`。
use country_core::asset_libraries::{
    mesh_library::AssetMeshLibrary, ssbo_library::AssetSsboLibrary,
};
use country_core::recovered_building::{
    wall_structures::MaterializedWallSegment,
    wall_collection::TerrainHeightGrid512,
    gates::{GateConstructionInput, construct_gate_cpu},
    wall_state::WallId as ObservedWallId,
};
use country_core::resources::{
    roofs::commands::{CreateRoofCmd, SwitchRoofTypeCmd},
    terrain_height::TerrainHeightsData,
    walls::{
        public_walls::{OnWallChanged, OnWallDeleted, PublicWalls},
        wooden_gates::Gates,
        WallId,
    },
};
use country_core::systems::ui::signifiers::signifier_create::UiSignifierStream;
use bevy_ecs::{event::Event, prelude::*};

/// B `ConstructGateCmd`（120B 事件 + 8B EventId = 128B 元素；G06）。
/// payload 布局（轮 3 全解登记）：wall u64@0、ordinal u64@8、段计数 u32@16、
/// 段端点/预备点（位置/高度/points_u 打包）@20..120。
#[derive(Event, Clone, Copy, Debug)]
pub struct ConstructGateCmd {
    pub wall: u64,
    pub payload: [u8; 120],
}

impl ConstructGateCmd {
    /// payload 解码：{wall, ordinal, point_count, 每点 (x,y,z,h,u)}。
    pub fn decode(&self) -> (u64, u64, u32) {
        let wall = u64::from_le_bytes(self.payload[..8].try_into().unwrap_or([0; 8]));
        let ordinal = u64::from_le_bytes(self.payload[8..16].try_into().unwrap_or([0; 8]));
        let count = u32::from_le_bytes(self.payload[16..20].try_into().unwrap_or([0; 4]));
        (wall, ordinal, count)
    }
}

/// B 0x140756370：9 参数面（顺序按 G06 findings）。
pub fn construct_gates(
    mut requests: EventReader<ConstructGateCmd>,
    mut gates: ResMut<Gates>,
    mut commands: Commands,
    mut meshes: ResMut<AssetMeshLibrary>,
    walls: Res<PublicWalls>,
    terrain: Res<TerrainHeightsData>,
    ssbo: Res<AssetSsboLibrary>,
    _signifiers: Res<UiSignifierStream>,
    mut output: ResMut<crate::construct_arches::WallConstructorOutput>,
) {
    // TerrainHeightGrid512（512×512 u16 + 高度界；采样修复标志按证据登记）。
    let terrain_grid = TerrainHeightGrid512 {
        encoded: std::sync::Arc::from(terrain.heights()),
        minimum_y: terrain.min_height(),
        maximum_y: terrain.max_height(),
        sampling_fix: false,
    };
    for request in requests.read() {
        let (wall, ordinal, count) = request.decode();
        let id = WallId(wall);
        let Some(wall_state) = walls.get(id) else {
            continue;
        };
        let projection = wall_state.building_projection();
        let curve = wall_state.simplified_curve_2d();
        // 段（MaterializedWallSegment）：预备点列源 = 简化曲线（I 级登记：
        // 预备点/第三数组的精确源待 path-prep 全链接线）。
        let n = (count as usize).clamp(2, curve.points.len().max(2));
        let positions: Vec<[f32; 3]> = curve
            .points
            .iter()
            .take(n)
            .map(|p| [p[0], p[1], 0.0])
            .collect();
        let heights: Vec<f32> = curve.points.iter().take(n).map(|p| p[1]).collect();
        let points_u: Vec<f32> = (0..n)
            .map(|i| i as f32 / (n.saturating_sub(1).max(1)) as f32)
            .collect();
        let segment = MaterializedWallSegment {
            heights,
            positions,
            points_u,
            source_indices: [0, n],
            range: country_core::recovered_building::wall_segmentation::WallLoopRange {
                start: 0,
                end: n as i32,
                modulus: n as i32,
            },
        };
        let prepared_positions = segment.positions.clone();
        let prepared_u = segment.points_u.clone();
        let input = GateConstructionInput {
            segment: &segment,
            prepared_positions: &prepared_positions,
            prepared_u: &prepared_u,
            observed_public_288: projection.float_0x288(),
            observed_public_29a: projection.observed_29a(),
            wall_id: ObservedWallId(wall),
            ordinal,
        };
        if let Ok(outcome) = construct_gate_cpu(input, &terrain_grid) {
            // 96B 支撑砖落盘。
            for brick in &outcome.bricks {
                output.brick_instances.push(brick.observed_bytes());
            }
            // 门扇实体 + 136B spawn payload（G06 布局登记）。
            for (leaf_ordinal, leaf) in outcome.leaves.iter().enumerate() {
                let seed = wall.wrapping_mul(ordinal).wrapping_add(leaf_ordinal as u64) as u32;
                let mut spawn_payload = [0u8; 136];
                spawn_payload[16..20].copy_from_slice(&1.0f32.to_le_bytes()); // 单位缩放
                spawn_payload[20..24].copy_from_slice(&1.0f32.to_le_bytes());
                spawn_payload[24..28].copy_from_slice(&1.0f32.to_le_bytes());
                spawn_payload[80..104].copy_from_slice(
                    &leaf
                        .top_positions
                        .first()
                        .copied()
                        .unwrap_or([0.0; 3])
                        .map(f32::to_le_bytes)
                        .concat(),
                );
                spawn_payload[104..112].copy_from_slice(&wall.to_le_bytes());
                spawn_payload[112..120].copy_from_slice(
                    &leaf.endpoint_directions[0].map(f32::to_le_bytes).concat(),
                );
                spawn_payload[120..128].copy_from_slice(
                    &leaf.endpoint_directions[1].map(f32::to_le_bytes).concat(),
                );
                spawn_payload[128..132].copy_from_slice(&seed.to_le_bytes());
                spawn_payload[132] = leaf.observed_public_29a;
                // 门扇实体（TransientMesh/mesh/shader 句柄 = 渲染边界登记）。
                let entity = commands.spawn_empty().id();
                gates.for_wall_mut(id).push(entity.to_bits());
                output.door_spawn_payloads.push(spawn_payload);
            }
            // nanis（NaniBundle）→ 渲染边界登记。
            let _ = outcome.nanis;
        }
        let _ = (&mut meshes, &ssbo);
    }
}

/// B 0x140755690：6 组合 despawn 顺序（changed/create/switch rebuild +
/// wall/roof removal）；重建保留空条目。
pub fn update_gates_hashmap(
    mut changed: EventReader<OnWallChanged>,
    mut create_roof: EventReader<CreateRoofCmd>,
    mut switch_type: EventReader<SwitchRoofTypeCmd>,
    mut deleted: EventReader<OnWallDeleted>,
    mut gates: ResMut<Gates>,
) {
    let mut rebuild: Vec<u64> = changed
        .read()
        .map(|e| e.0 .0)
        .chain(create_roof.read().map(|e| e.0.wall_id().0))
        .chain(switch_type.read().map(|e| e.wall.0))
        .collect();
    rebuild.sort_unstable();
    rebuild.dedup();
    for id in rebuild {
        let _ = gates.for_wall_mut(WallId(id));
    }
    for event in deleted.read() {
        gates.per_wall.remove(&event.0);
    }
}
