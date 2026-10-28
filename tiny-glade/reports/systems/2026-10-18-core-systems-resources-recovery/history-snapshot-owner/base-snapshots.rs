//! `history_snapshots.rs` — `History` 的快照表、起始编辑与空判定（B `resources::history`）。
//!
//! 作用在已有的 [`ObservedHistoryLedger`] 上（字段全是 pub），不改动 `history_transactions.rs`。证据：
//! 反编译 `history/decomp/decomp-0x140a84650.txt`、`decomp-0x140a844f0.txt` + 汇编核对。
//!
//! - **快照"最近者"选择**（`current_snapshot` 0x140a84650、`set_current_snapshot` 0x140a84760、
//!   `closest_resolved_snapshot` 0x140a84a50 共用）：快照按 `chain_index` 升序（元素 16 B：`Arc @0`、`chain_index i32 @8`）；
//!   无分支二分求 `p = (s[lo].idx < cursor) + lo`；`p < len` 时，`p != 0` 则比较 `|idx[p-1] - cursor|` 与
//!   `|idx[p] - cursor|`，**`dist[p] <= dist[p-1]` 取 `p`，否则取 `p-1`**（平局取后者，可能越过 `cursor`）；
//!   `p == 0` 取 0；`p == len` 取最后一个；
//! - `current_snapshot`：选出的快照 `chain_index == cursor` 才返回；
//! - `set_current_snapshot`：同一选择，若已有 `chain_index == cursor` 的快照 → **panic**（断言不替换，mod.rs:0x33），
//!   否则在 `p` 处有序插入；
//! - `closest_resolved_snapshot`：先按"已解析"谓词过滤（原谓词 0x140a84260 的规则未恢复，由调用方提供），再做同一选择，
//!   **不要求相等**；
//! - `insert_starting_edit` 0x140a844f0：断言 `cursor >= -1`（否则 panic "assertion failed: self.0 >= -1"），`cursor += 1`，
//!   在链首插入，随机 `nonce`，`observed_count_58 = 0`；
//! - `is_decorator_history_diff_empty` 0x140a84290 = `[+0x18] == 0`；`is_stairs_history_diff_empty` 0x140a842a0 =
//!   两个 Vec 的**容量**（`+0x30`、`+0x48`）都为 0（读容量而非长度）；
//! - `is_default_stair_width` 0x140b3f910 = `x == 1.0`（`cmpeqss`，NaN 为假），与 `is_default_lantern_brightness` 折叠同址。
// 公开重导出：system-tree `convert_preplaced_trees` 经本模块取 `HistoryEntry`
// （2026-10-17 并行落库的跨 crate 合同；不改变类型定义/逻辑）。
pub use crate::resources::history::transactions::HistoryEntry;
use crate::resources::history::transactions::ObservedHistoryLedger;
use crate::resources::history::transactions::ResolvedHistorySnapshot;

/// 无分支二分 + "取更近者"（原版 0x140a84650 的选择规则）。`chain_index(i)` 给出第 `i` 个快照的 `chain_index`。
fn nearest_index_by(len: usize, chain_index: impl Fn(usize) -> i32, cursor: i32) -> Option<usize> {
    if len == 0 {
        return None;
    }
    let mut lo = 0usize;
    let mut size = len;
    while size > 1 {
        let mid = (size >> 1) + lo;
        if chain_index(mid) < cursor {
            lo = mid;
        }
        size -= size >> 1;
    }
    let partition = usize::from(chain_index(lo) < cursor) + lo;
    if partition >= len {
        return Some(len - 1);
    }
    if partition == 0 {
        return Some(0);
    }
    let dist = |index: usize| chain_index(index).wrapping_sub(cursor).wrapping_abs();
    Some(if dist(partition) <= dist(partition - 1) {
        partition
    } else {
        partition - 1
    })
}

/// 在快照表上做"最近者"选择，返回被选中的下标。
pub fn nearest_snapshot_index<S>(
    snapshots: &[ResolvedHistorySnapshot<S>],
    cursor: i32,
) -> Option<usize> {
    nearest_index_by(snapshots.len(), |i| snapshots[i].chain_index, cursor)
}

/// 插入点（原版的 `p`，不含"取更近者"这一步）。
fn partition_index<S>(snapshots: &[ResolvedHistorySnapshot<S>], cursor: i32) -> usize {
    let len = snapshots.len();
    let mut lo = 0usize;
    let mut size = len;
    if size > 1 {
        while size > 1 {
            let mid = (size >> 1) + lo;
            if snapshots[mid].chain_index < cursor {
                lo = mid;
            }
            size -= size >> 1;
        }
    }
    if len == 0 {
        0
    } else {
        usize::from(snapshots[lo].chain_index < cursor) + lo
    }
}

/// `History::current_snapshot`（0x140a84650）。
pub fn current_snapshot<S>(snapshots: &[ResolvedHistorySnapshot<S>], cursor: i32) -> Option<&S> {
    let chosen = &snapshots[nearest_snapshot_index(snapshots, cursor)?];
    (chosen.chain_index == cursor).then_some(&chosen.value)
}

/// `History::set_current_snapshot`（0x140a84760）：该游标已有快照时 panic，否则有序插入。
pub fn set_current_snapshot<S>(
    snapshots: &mut Vec<ResolvedHistorySnapshot<S>>,
    cursor: i32,
    value: S,
) {
    if let Some(index) = nearest_snapshot_index(snapshots, cursor) {
        assert!(
            snapshots[index].chain_index != cursor,
            "assertion failed: a snapshot already exists at the current history cursor"
        );
    }
    let at = partition_index(snapshots, cursor);
    snapshots.insert(
        at,
        ResolvedHistorySnapshot {
            chain_index: cursor,
            value,
        },
    );
}

/// 原生 `impl$9::try_from`（0x140a84260）的 "已解析" 判别上限。
///
/// fresh：`uVar1 = *(u64 *)(*in_RDX + 0x10); ... *out = (uVar1 < 2)`——
/// 快照载荷 +0x10 的 u64 状态值 `< 2` 才算 resolved；`>= 2` 时
/// `TryFrom` 失败（filter_map 丢弃该快照）。
pub const RESOLVED_SNAPSHOT_STATE_LIMIT: u64 = 2;

/// 原生 `HistoryChainSnapshot` 载荷的解析状态面（`impl$9::try_from` 0x140a84260）。
///
/// 原生谓词只读载荷 `+0x10` 的 u64 并与 2 比较；具体载荷类型归各 owner。
/// 本 trait 把该规则固定下来，替代此前"由调用方传入 is_resolved"的近似。
pub trait HistoryChainSnapshotPayload {
    /// 原生载荷 `+0x10` 的 u64（解析状态判别）。
    fn resolution_state(&self) -> u64;
    /// 原生 `impl$9::try_from`：`state < 2` 才 resolved。
    fn is_resolved(&self) -> bool {
        self.resolution_state() < RESOLVED_SNAPSHOT_STATE_LIMIT
    }
}

/// `HistoryTransactions<S=()>` 的占位载荷没有可观测状态：视为已解析
/// （原生空快照载荷 `+0x10` 为 0）。
impl HistoryChainSnapshotPayload for () {
    fn resolution_state(&self) -> u64 {
        0
    }
}

/// 以原始 u64 状态表示载荷（供测试与宿主把 `+0x10` 值原样喂入）。
impl HistoryChainSnapshotPayload for u64 {
    fn resolution_state(&self) -> u64 {
        *self
    }
}

/// 原生 `TryFrom<&HistoryChainSnapshot>` 形状：resolved 时产出
/// `(载荷, chain_index)`，否则 `None`（filter_map 元素）。
pub fn resolve_snapshot<S: HistoryChainSnapshotPayload>(
    snapshot: &ResolvedHistorySnapshot<S>,
) -> Option<(&S, i32)> {
    snapshot
        .value
        .is_resolved()
        .then_some((&snapshot.value, snapshot.chain_index))
}

/// `History::closest_resolved_snapshot`（0x140a84a50）：先按原生 `TryFrom`
/// 谓词（`0x140a84260`，`[payload+0x10] < 2`）过滤，再做同一最近者选择。
pub fn closest_resolved_snapshot<S: HistoryChainSnapshotPayload>(
    snapshots: &[ResolvedHistorySnapshot<S>],
    cursor: i32,
) -> Option<&ResolvedHistorySnapshot<S>> {
    let resolved: Vec<&ResolvedHistorySnapshot<S>> =
        snapshots.iter().filter(|s| s.value.is_resolved()).collect();
    let chosen = nearest_index_by(resolved.len(), |i| resolved[i].chain_index, cursor)?;
    Some(resolved[chosen])
}

/// `History::insert_starting_edit`（0x140a844f0）。`nonce` 由调用方提供（原版取 `fastrand` u64）。
pub fn insert_starting_edit<M, S>(
    ledger: &mut ObservedHistoryLedger<M, S>,
    entry: HistoryEntry<M>,
    nonce: u64,
) {
    assert!(ledger.cursor >= -1, "assertion failed: self.0 >= -1");
    ledger.cursor += 1;
    ledger.entries.insert(0, entry);
    ledger.mutation_nonce = nonce;
    ledger.observed_count_58 = 0;
}

/// 0x140a84290：装饰 diff 的 `+0x18` 字段为 0。
pub fn is_decorator_history_diff_empty(item_count: u64) -> bool {
    item_count == 0
}

/// 0x140a842a0：两个 Vec 的**容量**都是 0（原版读 `+0x30`、`+0x48`）。
pub fn is_stairs_history_diff_empty(edges_a_capacity: usize, edges_b_capacity: usize) -> bool {
    edges_a_capacity == 0 && edges_b_capacity == 0
}

/// 0x140b3f910：`x == 1.0`。
pub fn is_default_stair_width(width: f32) -> bool {
    width == 1.0
}

#[cfg(test)]
mod tests {
    use super::*;

    fn snaps(indices: &[i32]) -> Vec<ResolvedHistorySnapshot<&'static str>> {
        indices
            .iter()
            .map(|i| ResolvedHistorySnapshot {
                chain_index: *i,
                value: "s",
            })
            .collect()
    }

    #[test]
    fn nearest_picks_the_exact_match_when_present() {
        let s = snaps(&[1, 4, 9, 12]);
        for (cursor, expect) in [(1, 0), (4, 1), (9, 2), (12, 3)] {
            assert_eq!(
                nearest_snapshot_index(&s, cursor),
                Some(expect),
                "cursor {cursor}"
            );
        }
    }

    #[test]
    fn ties_go_to_the_later_snapshot_even_past_the_cursor() {
        let s = snaps(&[2, 6]);
        assert_eq!(nearest_snapshot_index(&s, 4), Some(1));
        assert_eq!(nearest_snapshot_index(&s, 3), Some(0));
        assert_eq!(nearest_snapshot_index(&s, 5), Some(1));
    }

    #[test]
    fn out_of_range_cursors_clamp_to_the_ends() {
        let s = snaps(&[3, 5, 7]);
        assert_eq!(nearest_snapshot_index(&s, -10), Some(0));
        assert_eq!(nearest_snapshot_index(&s, 100), Some(2));
        assert_eq!(nearest_snapshot_index::<()>(&[], 0), None);
    }

    #[test]
    fn current_snapshot_requires_an_exact_chain_index() {
        let s = snaps(&[1, 4, 9]);
        assert!(current_snapshot(&s, 4).is_some());
        assert!(current_snapshot(&s, 5).is_none());
        assert!(current_snapshot(&snaps(&[]), 0).is_none());
    }

    #[test]
    fn set_current_snapshot_inserts_in_order() {
        let mut s = snaps(&[1, 9]);
        set_current_snapshot(&mut s, 4, "mid");
        set_current_snapshot(&mut s, 12, "tail");
        set_current_snapshot(&mut s, -1, "head");
        let order: Vec<_> = s.iter().map(|x| x.chain_index).collect();
        assert_eq!(order, [-1, 1, 4, 9, 12]);
        assert_eq!(s[2].value, "mid");
    }

    #[test]
    #[should_panic(expected = "snapshot already exists")]
    fn set_current_snapshot_does_not_replace() {
        let mut s = snaps(&[1, 4]);
        set_current_snapshot(&mut s, 4, "again");
    }

    #[test]
    fn closest_resolved_filters_first_and_does_not_need_equality() {
        let s = vec![
            ResolvedHistorySnapshot {
                chain_index: 2,
                value: 1,
            },
            ResolvedHistorySnapshot {
                chain_index: 5,
                // 原生 +0x10 状态值 2 → `2 < 2` 为假：未解析，被 filter_map 丢弃。
                value: 2,
            },
            ResolvedHistorySnapshot {
                chain_index: 9,
                value: 1,
            },
        ];
        // 过滤后只剩 2 和 9；游标 5 时 |2-5| = 3 < |9-5| = 4，选 2。
        assert_eq!(closest_resolved_snapshot(&s, 5).unwrap().chain_index, 2);
        // 游标 6：|2-6| = 4 > |9-6| = 3，选 9。
        assert_eq!(closest_resolved_snapshot(&s, 6).unwrap().chain_index, 9);
        // 未解析快照（chain 5）永远不会被选中：游标 5 也只能落到最近的已解析项。
        assert_eq!(closest_resolved_snapshot(&s, 5).unwrap().chain_index, 2);
    }

    #[test]
    fn closest_resolved_predicate_is_state_less_than_two() {
        // 原生 impl$9::try_from 0x140a84260：`*(u64*)(payload+0x10) < 2`。
        assert!(0u64.is_resolved());
        assert!(1u64.is_resolved());
        assert!(!2u64.is_resolved());
        assert!(!u64::MAX.is_resolved());
        assert_eq!(u64::MAX.resolution_state(), u64::MAX);
        // `()` 占位载荷状态为 0（resolved）。
        assert!(().is_resolved());
        let s = vec![ResolvedHistorySnapshot {
            chain_index: 0,
            value: 0u64,
        }];
        assert_eq!(resolve_snapshot(&s[0]), Some((&0u64, 0)));
        let unresolved = vec![ResolvedHistorySnapshot {
            chain_index: 0,
            value: 2u64,
        }];
        assert_eq!(resolve_snapshot(&unresolved[0]), None);
    }

    #[test]
    fn closest_resolved_is_none_when_nothing_is_resolved() {
        let s = vec![
            ResolvedHistorySnapshot {
                chain_index: 1,
                value: 2u64,
            },
            ResolvedHistorySnapshot {
                chain_index: 2,
                value: 3u64,
            },
        ];
        assert!(closest_resolved_snapshot(&s, 1).is_none());
    }

    #[test]
    fn insert_starting_edit_shifts_the_cursor_and_resets_the_count() {
        let mut ledger = ObservedHistoryLedger::<u8, ()> {
            entries: vec![HistoryEntry::from_edits(vec![1])],
            cursor: 0,
            undo_anchor: -1,
            resolved_snapshots: vec![],
            observed_count_58: 7,
            mutation_nonce: 1,
        };
        insert_starting_edit(&mut ledger, HistoryEntry::from_edits(vec![9]), 42);
        assert_eq!(ledger.cursor, 1);
        assert_eq!(ledger.entries[0].edits, vec![9]);
        assert_eq!(ledger.entries[1].edits, vec![1]);
        assert_eq!((ledger.observed_count_58, ledger.mutation_nonce), (0, 42));
    }

    #[test]
    #[should_panic(expected = "self.0 >= -1")]
    fn insert_starting_edit_asserts_the_cursor_floor() {
        let mut ledger = ObservedHistoryLedger::<u8, ()> {
            entries: vec![],
            cursor: -2,
            undo_anchor: -1,
            resolved_snapshots: vec![],
            observed_count_58: 0,
            mutation_nonce: 0,
        };
        insert_starting_edit(&mut ledger, HistoryEntry::from_edits(vec![]), 0);
    }

    #[test]
    fn ledger_mut_wires_insert_starting_edit_through_history_transactions() {
        use crate::resources::history::transactions::HistoryTransactions;
        let mut history = HistoryTransactions::<u8>::after_observed_future_clear();
        insert_starting_edit(history.ledger_mut(), HistoryEntry::from_edits(vec![7]), 99);
        assert_eq!(history.ledger().cursor, 0);
        assert_eq!(history.ledger().entries[0].edits, vec![7]);
        assert_eq!(history.ledger().mutation_nonce, 99);
    }

    #[test]
    fn small_predicates_match_the_machine_code() {
        assert!(is_decorator_history_diff_empty(0) && !is_decorator_history_diff_empty(3));
        assert!(is_stairs_history_diff_empty(0, 0));
        assert!(!is_stairs_history_diff_empty(4, 0) && !is_stairs_history_diff_empty(0, 4));
        assert!(is_default_stair_width(1.0));
        assert!(!is_default_stair_width(1.0001) && !is_default_stair_width(f32::NAN));
    }
}
