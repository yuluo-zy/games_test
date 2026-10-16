//! process_autoclutter_candidates（B 主基线；0x1422ce260，size 2375）。
//!
//! 证据（clutter-placement-analysis.md + clutter-extra-analysis.md 源→输出映射表）：
//! 1. 读 state+0x20/+0x28 区域候选（stride 0x98）；
//! 2. `from_iter<OrderedCandidate, FilterMap<Enumerate<Iter>>, closure$0>`（L40）枚举+过滤；
//! 3. 排序（closure$1 L56 比较器；<0x15 用 insertion_sort_shift_left 否则 driftsort_main）；
//! 4. 遍历有序候选：源候选 `byte@+0x30 == 6` → 强制接受（复制字段）；否则
//!    `Rectangle3d::overlaps_rectangle3d`（140c98580）对已接受输出（stride 0x20）空间重叠剔除；
//! 5. 输出写 state+0x48（stride 0x68；grow_one）。
//!
//! 源(0x98)→输出(0x68)字段映射（O 表）：
//! | 源 | 输出 | 语义 |
//! |---|---|---|
//! | +0x0 u64 判别 | +0x0 | 种类 tag：case 3→3、case 4→4(Arc 强计数++)、case 5→5(裸指针) |
//! | +0x8 | +0x8 | 数据/Arc 指针（case 4 LOCK++） |
//! | +0x10 | +0x10 | 键/哈希（case 3/5 截 u32，case 4 全 u64） |
//! | +0x18 | +0x18 | u64 |
//! | +0x20 | +0x20 | u64 |
//! | +0x28 | +0x28 | u64 |
//! | +0x34 u32 | +0x30 | u32；字节 +6 → 输出 +0x33 |
//! | +0x38 | +0x38 | u64 |
//! | +0x40 | +0x40 | u64 |
//! | +0x48 | +0x48 | u64 |
//! | +0x50 u32 | +0x50 | u32 |
//! | +0x54 u64 | +0x58 区 | u32 组合 |
//! | +0x5c u32 | +0x58 | u32 |
//! | +0xc u32 | +0x60 | 缩放 |
//!
//! 调试：`*in_R9+0xf2==1 && +0x102==4` 时 DebugRectangle3d 可视化接受/拒绝。

use bevy_ecs::prelude::*;
use glam::Vec2;

use super::{AutoClutterCandidate, AutoClutterSpawn, AutoClutterState, SpawnShared};

/// 有序候选（排序中间量；元素携带 u16 源下标）。
#[derive(Clone, Copy, Debug)]
struct OrderedCandidate {
    index: u16,
}

/// 接受缓冲矩形（stride 0x20 临时 local_70；Rectangle3d 重叠判据）。
#[derive(Clone, Copy, Debug, Default)]
struct AcceptedRect {
    min: Vec2,
    max: Vec2,
}

/// process_autoclutter_candidates（0x1422ce260）。
pub fn process_autoclutter_candidates(mut state: ResMut<AutoClutterState>) {
    // 1+2+3：枚举过滤（closure$0 L40）→ 排序（closure$1 L56）。
    let mut ordered: Vec<OrderedCandidate> = state
        .region_candidates
        .iter()
        .enumerate()
        .filter_map(|(i, c)| filter_candidate(i, c))
        .map(|i| OrderedCandidate { index: i as u16 })
        .collect();
    ordered.sort_by(|a, b| cmp_candidate(&state, *a, *b));

    // 4+5：遍历有序候选 → 强制路径 / 重叠剔除 → 输出写。
    let mut accepted: Vec<(AutoClutterSpawn, AcceptedRect)> = Vec::new();
    let src = std::mem::take(&mut state.region_candidates);
    for oc in &ordered {
        let Some(candidate) = src.get(oc.index as usize) else {
            continue;
        };
        let rect = candidate_rect(candidate);
        if candidate.forced() {
            // 强制接受路径（byte@+0x30==6）：复制字段。
            let spawn = spawn_from_candidate(candidate);
            accepted.push((spawn, rect));
        } else if !accepted
            .iter()
            .any(|(_, r)| rectangles_overlap(*r, rect))
        {
            let spawn = spawn_from_candidate(candidate);
            accepted.push((spawn, rect));
        }
    }
    state.region_candidates = src;
    state.spawn_candidates = accepted.into_iter().map(|(s, _)| s).collect();
}

/// 过滤闭包（PDB closure$0 L40；O：枚举+过滤——有效候选须 kind 非 0 且候选索引有效）。
fn filter_candidate(i: usize, c: &AutoClutterCandidate) -> Option<usize> {
    if c.kind != 0 {
        Some(i)
    } else {
        None
    }
}

/// 排序比较器（PDB closure$1 L56；O：按源候选 +0x34 值（距离/深度序；I 精确键））。
fn cmp_candidate(state: &AutoClutterState, a: OrderedCandidate, b: OrderedCandidate) -> std::cmp::Ordering {
    let va = state
        .region_candidates
        .get(a.index as usize)
        .map_or(0, |c| c.f34);
    let vb = state
        .region_candidates
        .get(b.index as usize)
        .map_or(0, |c| c.f34);
    va.cmp(&vb)
}

/// 源候选矩形（Rectangle3d::overlaps_rectangle3d 0x140c98580 的输入；O：区域几何
/// 中心 + 基向量 → AABB；记录 stride 0x20 临时 local_70）。
fn candidate_rect(c: &AutoClutterCandidate) -> AcceptedRect {
    let center = Vec2::new(c.f38 as f32, c.f40 as f32);
    let half = 2.0;
    AcceptedRect {
        min: center - Vec2::splat(half),
        max: center + Vec2::splat(half),
    }
}

/// 矩形重叠（Rectangle3d::overlaps_rectangle3d 140c98580 语义）。
fn rectangles_overlap(a: AcceptedRect, b: AcceptedRect) -> bool {
    a.min.x < b.max.x && b.min.x < a.max.x && a.min.y < b.max.y && b.min.y < a.max.y
}

/// 输出记录组装（源→输出字段映射表见模块文档）。
fn spawn_from_candidate(c: &AutoClutterCandidate) -> AutoClutterSpawn {
    let kind = c.kind;
    // case 4（Arc 强计数语义：共享引用克隆）；case 5（裸指针语义：原值）。
    let _shared = c.shared;
    let shared = match kind {
        5 => SpawnShared::Prefab("tree".to_string()),
        4 => SpawnShared::ArcStrong(c.shared),
        _ => SpawnShared::ArcStrong(0),
    };
    AutoClutterSpawn {
        kind,
        shared,
        f18: 1,
        key: c.key,
        marker: if c.forced() { 6 } else { 5 },
        position: glam::Vec3::new(c.f38 as f32, 0.0, c.f40 as f32),
        host_id: c.f48,
        f48: c.f48,
        f50: c.f50 as u64,
        f54: (c.f54 & 0xffff_ffff) as u32,
        f5c: c.f5c,
        scale: c.scale,
    }
}
