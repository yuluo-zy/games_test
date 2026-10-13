//! process_autoclutter_candidates 业务恢复草稿（B 0x1422ce260，size 2375）
//!
//! 证据：reva 反编译 + 全函数反汇编 + PDB 行号（源行 21–122；
//! closure$0=L40 filter_map、closure$1=L56 sort_by、closure$2=L82、closure$3=L98）。

use bevy_ecs::prelude::*;
use glam::Vec3;

use super::{AutoClutterCandidate, AutoClutterSpawn, AutoClutterState};

/// 有序候选（排序中间量；元素为 u16 源下标，2 字节 stride）。
#[derive(Clone, Copy, Debug)]
struct OrderedCandidate {
    /// 源候选 u16 下标。
    index: u16,
}

/// 候选筛选系统（B 0x1422ce260）。
/// 业务步骤（证据）：
/// 1. 读 state+0x20/+0x28 区域候选（stride 0x98）。
/// 2. from_iter<OrderedCandidate, FilterMap<Enumerate<Iter<Candidate>>, closure$0>>
///    （141cea460）枚举+过滤（closure$0=L40）。
/// 3. 排序：<0x15 用 insertion_sort_shift_left（141cf9b50），否则 driftsort_main（141cf4370），
///    比较器 closure$1=L56。
/// 4. 遍历有序候选：
///    - 源候选 byte@+0x30 == 6 → 强制接受，复制字段到输出（0x68 stride）。
///    - 否则 Rectangle3d::overlaps_rectangle3d（140c98580）对已接受输出做空间重叠剔除。
///    - 调试 DebugRectangle3d::new/color/show 可视化。
/// 5. 输出写 state+0x48（grow_one<WallPathSegment>，+0x58 计数）。
pub fn process_autoclutter_candidates(mut state: ResMut<AutoClutterState>) {
    // 证据：从 state.region_candidates 构建有序候选（u16 下标），过滤闭包 L40 条件 unknown。
    let mut ordered: Vec<OrderedCandidate> = state
        .region_candidates
        .iter()
        .enumerate()
        .filter_map(|(i, c)| filter_candidate(i, c))
        .map(|i| OrderedCandidate { index: i as u16 })
        .collect();
    ordered.sort_by(|a, b| cmp_candidate(&state, *a, *b));

    // 重叠剔除：对 !forced 的候选，用 Rectangle3d::overlaps 对已接受输出判重。
    let mut accepted: Vec<AutoClutterSpawn> = Vec::new();
    for oc in ordered {
        let src = &state.region_candidates[oc.index as usize];
        if src.forced() {
            // 强制接受路径：复制字段。
            accepted.push(spawn_from_candidate(src));
        } else if !accepted.iter().any(|s| rectangles_overlap(src, s)) {
            accepted.push(spawn_from_candidate(src));
        }
    }
    state.spawn_candidates = accepted;
}

/// 过滤闭包（PDB closure$0 L40；具体条件 unknown）。
fn filter_candidate(_i: usize, _c: &AutoClutterCandidate) -> Option<usize> {
    // TODO(evidence): 反编译 from_iter filter 闭包确认。
    Some(_i)
}

/// 排序比较器（PDB closure$1 L56；比较字段 unknown——距离/深度）。
fn cmp_candidate(_state: &AutoClutterState, a: OrderedCandidate, b: OrderedCandidate) -> std::cmp::Ordering {
    // TODO(evidence): driftsort_main 比较闭包字段 unknown。
    a.index.cmp(&b.index)
}

/// 强制接受判断（源候选 byte@+0x30 == 6）。
impl AutoClutterCandidate {
    fn forced(&self) -> bool {
        // evidence: process_candidates / on_windows 判 byte@+0x30 == 6。
        // 本草稿 AutoClutterCandidate 未保留 +0x30 字段，标记 unknown。
        false // TODO(evidence): 补 +0x30 字节字段。
    }
}

/// 输出记录组装（stride 0x68；字段复制证据见 add_rules / lantern / tree）。
fn spawn_from_candidate(_src: &AutoClutterCandidate) -> AutoClutterSpawn {
    // TODO(evidence): 16 字段复制精确映射（+0x0..+0x64）。
    AutoClutterSpawn {
        kind: 0,
        name: String::new(),
        key: 0,
        marker: 0,
        position: 0,
        unknown: [0; 8],
    }
}

/// 空间重叠判定（utils Rectangle3d::overlaps_rectangle3d，B 140c98580）。
fn rectangles_overlap(_a: &AutoClutterCandidate, _b: &AutoClutterSpawn) -> bool {
    // TODO(evidence): 0x20 stride 输入布局确认。
    false
}
