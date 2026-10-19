//! B `decorator_visual/deco_differential_state.rs` — 装饰差分状态（D1）。
//!
//! 证据：fresh RE-MCP 反编译 0x14010ba60 / 0x14010b940（DecoDiff 构造器，1.9KB
//! 各，O 全文）+ 0x1412ad540 / 0x1412acd70（with_each_deco_diff 闭包对，24KB
//! 各）+ 0x1412ae1c0 / 0x1412add20（比较闭包对，12KB 各）+ 0x1412ae660 /
//! 0x1412af3a0（重装闭包对，31/26KB）+ 骨架 callees（O 类型名）：
//! DecoDiff::current、DecoDiffLog::with_each_deco_diff、DecoRemovalLog::with_removed。
//!
//! 机器码（O）：DecoDiff<C> 构造器（两泛型实例同构）——线程本地代际计数器
//! （未初始化 → {1,0..} + 代际 = FUN_140d64e00()）→ 96B 记录
//! `{header u32×6（常量 {74.48f32,1,0,0,0,0}）, 代际 u64, payload, {77.5f32,1},
//!  0, 0, ptr, 0}`。重装闭包（0x1412ae660 Chimney / 0x1412af3a0 DoorStairs）
//! 遍历存储 modified 装饰 → DecoDiffLog（差分日志）→ 逐装饰比较旧参数
//! （DecoDiff::current）→ 变化则重装砖/几何。
//!
//! 2026-10-19 补全（可复核部分）：
//! - [`compare_assembly_params`]：40B 参数块逐字段比较（`[u32; 10]`，
//!   布局 U，按 4B 字段直接比较）；
//! - [`WallAttachedDecoTracker`]：旧/新参数表 + `with_diffs` 仅对变化项回调
//!   （对应 0x1412ae660/0x1412af3a0 的 `with_diffs` 壳）；
//! - [`chimney_reassemble`] / [`door_stairs_reassemble`]：以 tracker 驱动，
//!   只有变化的装饰进入重装回调；砖列几何主体仍 U（回调由宿主提供）。
//!
//! U 登记：① 96B 记录各槽语义（header 常量 74.48/77.5 用途）② 重装闭包内部
//! 砖列装配主体 ③ with_each_deco_diff / with_removed 遍历准则。

use std::cell::Cell;
use std::collections::{HashMap, HashSet};

/// B DecoDiff<C>：装饰差分记录（96B；构造器 O）。
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct DecoDiff<C> {
    /// header u32×6（O 常量 {74.48f32 bits, 1, 0, 0, 0, 0}；U 语义）。
    pub header: [u32; 6],
    /// 代际（线程本地计数器，每次构造递增）。
    pub generation: u64,
    /// 参数载荷（ChimneyAssemblyParams / DoorStairsAssemblyParams）。
    pub payload: C,
    /// 第二默认槽（O 常量 {77.5f32 bits, 1, 0, 0}；U 语义）。
    pub defaults: [u64; 4],
}

// 代际计数器（线程本地，O：FUN_140d4d740 全局 cell）。
thread_local! {
    static DIFF_GENERATION: Cell<u64> = const { Cell::new(0) };
}

impl<C: Copy> DecoDiff<C> {
    /// 0x14010ba60 / 0x14010b940（O，两泛型实例同构）：构造差分记录。
    /// 代际 = 计数器递增；未初始化 cell 先写 {1,0..} + 初值。
    pub fn new(payload: C) -> Self {
        let generation = DIFF_GENERATION.with(|c| {
            let g = c.get();
            c.set(g.wrapping_add(1));
            g
        });
        Self {
            header: [0x4294f6e0, 1, 0, 0, 0, 0],
            generation,
            payload,
            defaults: [0x429b0010, 1, 0, 0],
        }
    }

    /// 行内联（I）：当前参数载荷。
    pub fn current(&self) -> C {
        self.payload
    }
}

/// 差分日志（DecoDiffLog，O 类型名）：逐装饰差分记录收集。
#[derive(Clone, Debug, Default)]
pub struct DecoDiffLog<C> {
    /// 装饰 id → 差分记录。
    pub entries: HashMap<u32, DecoDiff<C>>,
}

impl<C: Copy + PartialEq> DecoDiffLog<C> {
    /// with_each_deco_diff（0x1412ad540 / 0x1412acd70 闭包对，O 结构）：遍历
    /// 差分记录，逐条调用消费闭包。U：比较准则（仅变化条目？）待取证。
    pub fn with_each_deco_diff<F>(&self, mut f: F)
    where
        F: FnMut(u32, &DecoDiff<C>),
    {
        for (id, diff) in &self.entries {
            f(*id, diff);
        }
    }
}

/// 装饰移除日志（DecoRemovalLog，O 类型名）。
#[derive(Clone, Debug, Default)]
pub struct DecoRemovalLog {
    /// 已移除装饰 id 集。
    pub removed: HashSet<u32>,
}

impl DecoRemovalLog {
    /// with_removed（O）：遍历移除 id。
    pub fn with_removed<F>(&self, mut f: F)
    where
        F: FnMut(u32),
    {
        for id in &self.removed {
            f(*id);
        }
    }
}

/// 40B 烟囱装配参数块（O：0x1412ae1c0/0x1412add20 比较面；逐字段布局 U）。
///
/// 原版比较闭包（FUN_1412accf0 ×2）对 40B 记录逐字段比较；这里以 10×u32
/// 原文承载，`PartialEq` 即逐字段等价。
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
#[repr(C)]
pub struct ChimneyAssemblyParams {
    pub words: [u32; 10],
}

/// 40B 门梯装配参数块（同构）。
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
#[repr(C)]
pub struct DoorStairsAssemblyParams {
    pub words: [u32; 10],
}

const _: () = assert!(std::mem::size_of::<ChimneyAssemblyParams>() == 40);
const _: () = assert!(std::mem::size_of::<DoorStairsAssemblyParams>() == 40);

/// 0x1412ae1c0 / 0x1412add20（O 结构）：40B 参数逐字段比较。
///
/// 返回 true 表示**发生变化**（with_diffs 的重装条件）。
pub fn compare_assembly_params<T: PartialEq>(old: &T, new: &T) -> bool {
    old != new
}

/// `with_diffs` 差分跟踪器（O：`WallAttachedDecoTracker<P>` 壳）。
///
/// 维护上一帧参数表；本帧参数与旧值逐项比较，仅变化项进入回调。
#[derive(Clone, Debug, Default)]
pub struct WallAttachedDecoTracker<P> {
    /// 旧参数（prev）。
    pub previous: HashMap<u32, P>,
}

impl<P: PartialEq + Clone> WallAttachedDecoTracker<P> {
    /// 以本帧参数表驱差分：`current` 为装饰 id → 新参数；`f` 只对
    /// “新增或参数变化”的条目调用；随后旧表替换为本帧表。
    ///
    /// O 结构：0x1412ae660/0x1412af3a0 遍历 modified 装饰 →
    /// `compare_assembly_params` → 变化才重装。
    pub fn with_diffs<F>(&mut self, current: &HashMap<u32, P>, mut f: F) -> usize
    where
        F: FnMut(u32, &P),
    {
        let mut changed = 0;
        for (id, new_params) in current {
            match self.previous.get(id) {
                Some(old) if !compare_assembly_params(old, new_params) => {}
                _ => {
                    f(*id, new_params);
                    changed += 1;
                }
            }
        }
        self.previous.clone_from(current);
        changed
    }
}

/// 0x1412ae660（O 结构，31KB）：Chimney 差分重装闭包。
///
/// 以旧参数表为基线，仅对变化/新增装饰调用 `rebuild`（烟囱砖列装配主体 U，
/// 由宿主回调提供）。返回重装条目数。
pub fn chimney_reassemble<F>(
    tracker: &mut WallAttachedDecoTracker<ChimneyAssemblyParams>,
    current: &HashMap<u32, ChimneyAssemblyParams>,
    mut rebuild: F,
) -> usize
where
    F: FnMut(u32, &ChimneyAssemblyParams),
{
    tracker.with_diffs(current, |id, params| rebuild(id, params))
}

/// 0x1412af3a0（O 结构，26KB）：DoorStairs 差分重装闭包。
pub fn door_stairs_reassemble<F>(
    tracker: &mut WallAttachedDecoTracker<DoorStairsAssemblyParams>,
    current: &HashMap<u32, DoorStairsAssemblyParams>,
    mut rebuild: F,
) -> usize
where
    F: FnMut(u32, &DoorStairsAssemblyParams),
{
    tracker.with_diffs(current, |id, params| rebuild(id, params))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn diff_generation_increments() {
        let a = DecoDiff::new(7u32);
        let b = DecoDiff::new(8u32);
        assert!(b.generation > a.generation);
        assert_eq!(a.header[0], 0x4294f6e0);
        assert_eq!(a.defaults[0], 0x429b0010);
    }

    #[test]
    fn compare_assembly_params_is_fieldwise() {
        let a = ChimneyAssemblyParams { words: [1, 2, 3, 4, 5, 6, 7, 8, 9, 10] };
        let mut b = a;
        assert!(!compare_assembly_params(&a, &b));
        b.words[9] = 11;
        assert!(compare_assembly_params(&a, &b));
    }

    #[test]
    fn with_diffs_only_rebuilds_changed() {
        let mut tracker = WallAttachedDecoTracker::<ChimneyAssemblyParams>::default();
        let mut current = HashMap::new();
        current.insert(1, ChimneyAssemblyParams { words: [1; 10] });
        current.insert(2, ChimneyAssemblyParams { words: [2; 10] });
        let mut rebuilt = Vec::new();
        assert_eq!(
            chimney_reassemble(&mut tracker, &current, |id, _| rebuilt.push(id)),
            2
        );
        assert_eq!(rebuilt.len(), 2);
        // 第二次：1 变化、2 不变。
        current.insert(1, ChimneyAssemblyParams { words: [9; 10] });
        rebuilt.clear();
        assert_eq!(
            chimney_reassemble(&mut tracker, &current, |id, _| rebuilt.push(id)),
            1
        );
        assert_eq!(rebuilt, vec![1]);
    }

    #[test]
    fn removal_log_iterates() {
        let mut log = DecoRemovalLog::default();
        log.removed.insert(4);
        log.removed.insert(5);
        let mut seen = Vec::new();
        log.with_removed(|id| seen.push(id));
        seen.sort();
        assert_eq!(seen, vec![4, 5]);
    }
}
