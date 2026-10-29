//! process_autoclutter_candidates（B 主基线；0x1422ce260，size 2375）。
//!
//! fresh 反编译（2026-10-17，--worker shared）控制流：
//! 1. 读 state+0x20/+0x28 区域候选（stride 0x98）；
//! 2. `from_iter<OrderedCandidate, FilterMap<Enumerate<Iter>, closure_env_0>>`（L40）枚举+过滤
//!    （带 u16 源下标 @+0xc）；
//! 3. 排序（closure_env_1 比较器；<0x15 用 insertion_sort_shift_left 否则 driftsort_main）；
//! 4. 遍历有序候选：源候选 `byte@+0x30 == 6` → 强制接受；否则对已接受缓冲（stride 0x20，
//!    存源候选 +0x68..+0x80 Rectangle3d）逐条 `Rectangle3d::overlaps_rectangle3d`
//!    （0x140c98580）剔除重叠；
//! 5. 接受路径 switch 源判别 `*puVar26`（3/4/5）：case3 截 u32、case4 Arc LOCK++、case5
//!    裸指针；复制字段写输出 state+0x48（stride 0x68，grow_one<WallPathSegment>）；
//! 6. debug `*(in_R9+0xf2)==1 && *(in_R9+0x102)==4` 时 DebugRectangle3d 可视化。
//!
//! 源(0x98)→输出(0x68)字段映射（O）：
//! +0x00 tag；+0x08 Arc/数据指针（case4 LOCK++、case5 源指针）；+0x10 键（case3/默认截 u32、
//! case4 全 u64）；+0x18=源+0x18；+0x20=源+0x20；+0x28=源+0x28；+0x30=源+0x30 byte；
//! +0x34=源+0x34；+0x38=源+0x38；+0x40=源+0x40；+0x48=源+0x48；+0x50=源+0x50；
//! +0x54=源+0x54 低 32；+0x58=源+0x54 高 32；+0x5c=源+0x5c；+0x60=源+0x60。

use bevy_ecs::prelude::*;
use country_core::systems::ui_clutter::DeletedAutoClutter;

use super::{AutoClutterCandidate, AutoClutterSpawn, AutoClutterState, SpawnShared};

/// 有序候选（16 B；u16 源下标 @+0xc）。
#[derive(Clone, Copy, Debug)]
struct OrderedCandidate {
    /// +0x00：键/哈希（enumerate 上下文，sort 后遍历不直接读）。
    region_key: u64,
    priority: u32,
    /// +0x0c：u16 源候选下标。
    index: u16,
}

/// 已接受缓冲（stride 0x20；Rectangle3d @+0x00 起）。
#[derive(Clone, Copy, Debug)]
struct AcceptedRecord {
    rect: utils::geometry::rectangle::Rectangle3d,
    tag: u64,
    shared: u64,
    key: u64,
}

/// process_autoclutter_candidates（0x1422ce260）。
pub fn process_autoclutter_candidates(mut state: ResMut<AutoClutterState>, deleted: Res<DeletedAutoClutter>) {
    // 1+2+3：枚举过滤（closure$0 L40）→ 稳定排序（closure$1 L56 比较器）。
    let mut ordered: Vec<OrderedCandidate> = state
        .region_candidates
        .iter()
        .enumerate()
        .filter_map(|(i, c)| (!deleted.is_deleted(c.f40)).then_some(OrderedCandidate {
            region_key: c.region_key, priority: c.priority, index: i as u16,
        }))
        .collect();
    // B 141cf9b50：优先级、区域键均降序，相等时维持原先顺序。
    ordered.sort_by(|a, b| b.priority.cmp(&a.priority).then_with(|| b.region_key.cmp(&a.region_key)));

    // 4+5：遍历有序候选 → 强制路径 / 重叠剔除 → 输出写。
    let mut accepted: Vec<AcceptedRecord> = Vec::new();
    let src = std::mem::take(&mut state.region_candidates);
    for oc in &ordered {
        // O：`panic_bounds_check`（len <= 源下标）。
        assert!(
            (oc.index as usize) < src.len(),
            "process_autoclutter_candidates: bounds check failed"
        );
        let candidate = &src[oc.index as usize];
        let mut accepted_spawn = None;
        if candidate.forced() {
            // 强制接受路径（byte@+0x30==6）：复制字段。
            accepted_spawn = Some(accept_candidate(candidate, &mut accepted));
        } else {
            // 贪心矩形重叠剔除（O：Rectangle3d::overlaps_rectangle3d）。
            let mut overlap = false;
            for acc in &accepted {
                if acc.rect.overlaps_rectangle3d(candidate.rect) {
                    overlap = true;
                    break;
                }
            }
            if !overlap {
                accepted_spawn = Some(accept_candidate(candidate, &mut accepted));
            }
        }
        if let Some(spawn) = accepted_spawn {
            state.spawn_candidates.push(spawn);
        }
    }
    state.region_candidates = src;
}

/// 接受路径（O：switch 源判别 3/4/5 + 写已接受缓冲 + 输出字段复制）。
fn accept_candidate(
    c: &AutoClutterCandidate,
    accepted: &mut Vec<AcceptedRecord>,
) -> AutoClutterSpawn {
    // O：已接受缓冲 = 源候选 +0x68..+0x80 Rectangle3d。
    accepted.push(AcceptedRecord {
        rect: c.rect,
        tag: 0,
        shared: 0,
        key: 0,
    });
    // source 的 Arc/指针所有权仍待上游提供；此处不能用整数制造 Arc。
    let (tag, shared) = match c.kind {
        4 => {
            // case4：Arc 强计数++（LOCK 语义；**U 登记缝**——Arc payload 合同 U，登记不执行）。
            (4u8, c.shared)
        }
        5 => {
            // case5：裸指针语义（源候选指针；以 shared 承载）。
            (5u8, c.shared)
        }
        _ => {
            // case3 / default：截 u32。
            (c.kind, c.shared)
        }
    };
    AutoClutterSpawn {
        kind: tag,
        shared: if let Some(object) = &c.resolved {
            assert_eq!(object.source.native_tag(), c.kind, "自动杂物来源判别不一致");
            SpawnShared::Object(object.clone())
        } else if tag == 4 {
            SpawnShared::ArcStrong(shared)
        } else {
            SpawnShared::Prefab(String::new())
        },
        f18: (c.f18 & 0xffff_ffff) as u32,
        key: c.f40,
        marker: c.forced,
        // B 输出 +0x48..+0x53：原 candidate 的完整 xyz。
        position: glam::Vec3::new(
            f32::from_bits(c.f48 as u32),
            f32::from_bits((c.f48 >> 32) as u32),
            f32::from_bits(c.f50),
        ),
        host_id: c.f48,
        f48: c.f48,
        f50: c.f50 as u64,
        f54: (c.f54 & 0xffff_ffff) as u32,
        f58: ((c.f54 >> 32) & 0xffff_ffff) as u32,
        f5c: c.f5c,
        scale: f32::from_bits(c.f60),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn stale_region_removes_grid_links_and_its_candidate() {
        use crate::autoclutter::{AutoClutterRegion, ClutterCandidateIndex, ClutterRegionId};
        let key = ClutterRegionId(91);
        let mut state = AutoClutterState { lazy_grid: vec![vec![]; 4096], epoch: 4, ..Default::default() };
        let region = AutoClutterRegion { key, epoch: 3, center: glam::Vec3::ZERO,
            basis: [glam::Vec2::X, glam::Vec2::ONE], ..Default::default() };
        state.add_region(key, &region);
        state.regions.get_mut(&key).unwrap().epoch = 3;
        state.regions.get_mut(&key).unwrap().candidate_index = ClutterCandidateIndex(0);
        state.region_candidates.push(AutoClutterCandidate { region_key: key.0, ..Default::default() });
        assert!(state.lazy_grid.iter().any(|cell| cell.contains(&key.0)));
        state.remove_stale_regions();
        state.flush_candidates_pending_removal();
        assert!(state.regions.is_empty());
        assert!(state.region_candidates.is_empty());
        assert!(state.lazy_grid.iter().all(|cell| !cell.contains(&key.0)));
    }

    #[test]
    fn deleted_keys_are_filtered_before_priority_order_and_full_pose_is_copied() {
        let mut world = World::new();
        let candidate = |id, region_key, priority, x: f32| AutoClutterCandidate {
            kind: 3,
            f40: id,
            f48: u64::from(x.to_bits()) | (u64::from(2.0f32.to_bits()) << 32),
            f50: 3.0f32.to_bits(),
            forced: 6,
            region_key,
            priority,
            f60: 1.0f32.to_bits(),
            ..Default::default()
        };
        world.insert_resource(AutoClutterState {
            region_candidates: vec![candidate(0x1_0000_0001, 1, 2, 1.0),
                candidate(7, 9, 2, 4.0), candidate(8, 2, 3, 5.0),
                candidate(9, 3, 9, 6.0)],
            ..Default::default()
        });
        let mut deleted = DeletedAutoClutter::default();
        deleted.0.insert(9);
        world.insert_resource(deleted);
        let mut schedule = Schedule::default();
        schedule.add_systems(process_autoclutter_candidates);
        schedule.run(&mut world);
        let output = &world.resource::<AutoClutterState>().spawn_candidates;
        assert_eq!(output.iter().map(|s| s.key).collect::<Vec<_>>(), [8, 7, 0x1_0000_0001]);
        assert_eq!(output[2].position, glam::Vec3::new(1.0, 2.0, 3.0));
    }
}
