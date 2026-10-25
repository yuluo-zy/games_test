//! 玩家楼梯支柱提议处理（construct_playermade_stairs/pillars_process_proposals.rs）。
//!
//! `process_playermade_stair_pillars_proposals`（宿主 0x140178a70，re-analysis
//! hardpoints-interaction §3.2 已攻克）：bevy `impl$3::call_mut` 系统包装，
//! 6 参 SystemParam 坐实：
//! `(ResMut<StairsPillarProposals>, Res<UiSignifierStream>, ResMut<RaycastWorld>,
//!  ResMut<StairColliderRegistry>, ResMut<RtWorld>,
//!  EventWriter<UpdateRtWorldEvent>)`。
//!
//! 宿主反编译（`ghidra-0x140178a70-*.c.txt`，47KB）已解出业务块全序：
//! 1. `from_iter<Drain>` 全量取出 `HashMap<StairGraphEdgeIdentity,
//!    Vec<StairsPillarProposal>>`；
//! 2. 每 key 桶内**合并附近提案**（距离 < 0.01 合并，size_x 取 max / size_z
//!    取 min，NaN 感知；kind 字节 == 0x02 终止本桶）；—— 本文件已按
//!    反编译逐语句重建为 `merge_nearby_proposals`；
//! 3. 合并结果 `HashMap::insert` 回写（新表）；
//! 4. 每提案 `StairGraphEdge::hash_one` → RawTable 查 `StairColliderRegistry`
//!    （命中直用 / 未命中 `reserve_rehash` 插入空 `Vec<Vec3>`）；
//! 5. `Rectangle3d::new`（size = {(size_x − size_z) + 0.3, +0x18 − 0.5}，
//!    常量 0x3f0f5c29）+ `box_trigger_volume` + closure_env_0<true>
//!    `spec_extend<Vec3>`（Triangle → 三顶点）写入 collider Vec；
//! 6. `RtWorld::get_mut<RtMeshHandle>` 追加现有 collider 点 +
//!    `EventWriter::write<UpdateRtWorldEvent>`。
//!
//! **44B 提议记录布局**（stride 0x2c，反编译逐字节）：`{pos x/z @0..8,
//!  f32 @8, f32 @0xc, u64 @0x10, u64 @0x18, u64 @0x20, kind u8 @0x28,
//!  u16 @0x29, u8 @0x2b}`。
//!
//! **未实现登记（2026-10-17 restorer-remaining-wallconstructor core 复核）**：
//! 第 4-6 步依赖五个类型，逐项核实 country-core：
//! - ✅ `RtWorld`：`resources/rt.rs`（`RtWorld::get_mut` / `RtMeshHandle` /
//!   `RtMesh` 已落库，含 `alloc_prefab_handle`）；
//! - ✅ `RtMeshHandle`：同上（16B `{tag,payload}` 键）；
//! - ✅ `StairGraphEdge`：`resources/stairs/stairs_state.rs`（u32 对，min/max）；
//! - ❌ `StairColliderRegistry`：全 workspace 无（`resources/stairs/
//!   stairs_collider.rs` 仅有标记资源 `StairsCollider`）——**缺**；
//! - ❌ `UpdateRtWorldEvent`：全 workspace 无（`resources/events.rs` 未注册）
//!   ——**缺**。
//!
//! 系统参数面按证据登记，接线待上述 2 个类型落地；不自行补造缺失类型
//! （硬标准）。解锁条件：core 落 `StairColliderRegistry`
//! （`RawTable<StairGraphEdge, Vec<Vec3>>` 语义：hash_one→探测→命中直用/
//! 未命中 reserve_rehash 插入空 Vec）与 `UpdateRtWorldEvent`（Event 载荷）后，
//! 按宿主反编译全序实现第 4-6 步。
use bevy_ecs::prelude::*;
use ::utils::bytes::{read_f32_le as read_f32, write_f32_le as write_f32};
use country_core::resources::stairs_walls::{StairsPillarProposal, StairsPillarProposals};
use country_core::systems::collision::world::RaycastWorld;
use country_core::systems::ui::signifiers::signifier_create::UiSignifierStream;

/// 提议记录内字段偏移（stride 0x2c）。
const POS: std::ops::Range<usize> = 0..8; // pos x/z（2×f32）
const EXT_X: std::ops::Range<usize> = 8..12; // size_x（合并取 max）
const EXT_Z: std::ops::Range<usize> = 12..16; // size_z（合并取 min）
const KIND: usize = 0x28; // kind 字节（==0x02 终止桶内处理）
/// 合并距离阈值（`while (0.01 <= dist)`；合并当 dist < 0.01）。
const MERGE_DIST: f32 = 0.01;

/// B 0x140178a70 业务块步骤 2（merge）：同一 key 桶内把距离 < 0.01 的
/// 支柱提议合并（size_x 取 max、size_z 取 min，均 NaN 感知：existing 为
/// NaN 时取 new）。kind 字节 == 0x02 的提议**终止**本桶剩余处理（不并入）。
///
/// 宿主反编译逐语句（`ghidra-0x140178a70-*.c.txt` 行 373-445）：
/// - 距离 = `sqrt((ex_pos.x - new_pos.x)² + (ex_pos.z - new_pos.z)²)`，
///   用 pos 字节 `@0..8`（LE 两个 f32）；
/// - 命中合并：只写回 existing 的 `+8`（
///   `ex.is_nan() ? new : (new <= ex ? ex : new)`）与 `+0xc`（
///   `ex.is_nan() ? new : (ex <= new ? ex : new)`）；其余字段保留 existing。
/// - 未命中：把 new 完整追加（44B 逐字段拷）。
pub fn merge_nearby_proposals(bucket: &[StairsPillarProposal]) -> Vec<StairsPillarProposal> {
    let mut merged: Vec<StairsPillarProposal> = Vec::new();
    for proposal in bucket {
        let new = proposal.observed();
        // kind == 0x02：终止本桶剩余处理（宿主 `if (cVar37 == '\x02') break;`）。
        if new[KIND] == 0x02 {
            break;
        }
        let nx = read_f32(new, POS.start);
        let nz = read_f32(new, POS.start + 4);
        let mut matched: Option<usize> = None;
        for (i, existing) in merged.iter().enumerate() {
            let ex = existing.observed();
            let ex_x = read_f32(ex, POS.start);
            let ex_z = read_f32(ex, POS.start + 4);
            let dx = ex_x - nx;
            let dz = ex_z - nz;
            let dist = (dx * dx + dz * dz).sqrt();
            if dist < MERGE_DIST {
                matched = Some(i);
                break;
            }
        }
        match matched {
            None => {
                // 未命中：完整追加（宿主 append 逐字段拷 44B）。
                merged.push(*proposal);
            }
            Some(i) => {
                // 命中：只更新 existing 的 size_x（max）/ size_z（min），
                // 其余字段保留 existing（宿主只写 `*existing + 8/+0xc`）。
                let existing = merged[i].observed();
                let mut out = *existing;
                let ex_x = read_f32(existing, EXT_X.start);
                let new_x = read_f32(new, EXT_X.start);
                let ex_z = read_f32(existing, EXT_Z.start);
                let new_z = read_f32(new, EXT_Z.start);
                // 宿主 +8：`ex.is_nan() ? new : (new <= ex ? ex : new)`（max）。
                let merged_x = if ex_x.is_nan() {
                    new_x
                } else if new_x <= ex_x {
                    ex_x
                } else {
                    new_x
                };
                // 宿主 +0xc：`ex.is_nan() ? new : (ex <= new ? ex : new)`（min）。
                let merged_z = if ex_z.is_nan() {
                    new_z
                } else if ex_z <= new_z {
                    ex_z
                } else {
                    new_z
                };
                write_f32(&mut out, EXT_X.start, merged_x);
                write_f32(&mut out, EXT_Z.start, merged_z);
                merged[i] = StairsPillarProposal::from_observed(out);
            }
        }
    }
    merged
}

/// B 支柱提议处理（0x140178a70）。**未实现**：完整系统需第 4-6 步依赖的
/// `StairColliderRegistry`/`UpdateRtWorldEvent` 类型（core-type-gap，2026-10-17
/// 复核：`RtWorld`/`RtMeshHandle`/`StairGraphEdge` 已在 country-core，
/// 上述 2 类型仍缺）。合并块（步骤 2）已重建为 `merge_nearby_proposals`，
/// 接线待类型落地后按宿主反编译全序实现。真实系统签名 6 参（见模块文档），
/// 此处仅以可编译参数面占位。
pub fn process_playermade_stair_pillars_proposals(
    _proposals: ResMut<StairsPillarProposals>,
    _signifiers: Res<UiSignifierStream>,
    _world: ResMut<RaycastWorld>,
) {
    todo!(
        "未实现: process_playermade_stair_pillars_proposals 宿主 0x140178a70（缺 StairColliderRegistry/RtWorld/RtMeshHandle/UpdateRtWorldEvent/StairGraphEdge core 类型）；merge 块已落 merge_nearby_proposals"
    )
}


#[cfg(test)]
mod tests {
    use super::*;

    fn proposal(pos: [f32; 2], size_x: f32, size_z: f32, kind: u8) -> StairsPillarProposal {
        let mut bytes = [0u8; 44];
        write_f32(&mut bytes, 0, pos[0]);
        write_f32(&mut bytes, 4, pos[1]);
        write_f32(&mut bytes, 8, size_x);
        write_f32(&mut bytes, 12, size_z);
        bytes[0x28] = kind;
        StairsPillarProposal::from_observed(bytes)
    }

    fn pos(record: &StairsPillarProposal) -> (f32, f32) {
        let b = record.observed();
        (read_f32(b, 0), read_f32(b, 4))
    }
    fn ext(record: &StairsPillarProposal) -> (f32, f32) {
        let b = record.observed();
        (read_f32(b, 8), read_f32(b, 12))
    }

    #[test]
    fn merge_within_threshold() {
        // dist 0.005 < 0.01 → 合并；size_x 取 max、size_z 取 min，pos 保留 existing。
        let p1 = proposal([0.0, 0.0], 1.0, 2.0, 0);
        let p2 = proposal([0.005, 0.0], 3.0, 1.0, 0);
        let merged = merge_nearby_proposals(&[p1, p2]);
        assert_eq!(merged.len(), 1);
        assert_eq!(pos(&merged[0]), (0.0, 0.0));
        assert_eq!(ext(&merged[0]), (3.0, 1.0)); // max/min
    }

    #[test]
    fn far_proposals_stay_separate() {
        let p1 = proposal([0.0, 0.0], 1.0, 2.0, 0);
        let p2 = proposal([0.5, 0.0], 3.0, 1.0, 0);
        let merged = merge_nearby_proposals(&[p1, p2]);
        assert_eq!(merged.len(), 2);
        assert_eq!(pos(&merged[0]), (0.0, 0.0));
        assert_eq!(pos(&merged[1]), (0.5, 0.0));
    }

    #[test]
    fn kind_2_terminates_bucket() {
        let p1 = proposal([0.0, 0.0], 1.0, 2.0, 2); // kind==2 → 终止
        let p2 = proposal([0.01, 0.0], 3.0, 1.0, 0);
        let merged = merge_nearby_proposals(&[p1, p2]);
        assert!(merged.is_empty());
    }
}
