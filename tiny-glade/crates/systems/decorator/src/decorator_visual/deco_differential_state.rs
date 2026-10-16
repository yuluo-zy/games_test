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
//! U 登记：① 96B 记录各槽语义（header 常量 74.48/77.5 用途）② 重装闭包内部
//! 砖列装配主体 ③ with_each_deco_diff / with_removed 遍历准则。

use std::cell::Cell;

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
    pub entries: std::collections::HashMap<u32, DecoDiff<C>>,
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
    pub removed: std::collections::HashSet<u32>,
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

/// 0x1412ae660（O 结构，31KB）：Chimney 差分重装闭包。遍历存储 modified
/// 装饰 → DecoDiffLog<ChimneyAssemblyParams> → 逐装饰比较旧参数
/// （DecoDiff::current）→ 变化则按 brick 装配链重装（U 内部）。
pub fn chimney_reassemble(
    _context_a: u64,
    _context_b: u64,
    _context_c: u64,
    _context_e: u64,
    _context_g: u64,
) {
    // U：重装主体待取证（差分比较 + 烟囱砖列装配）。
}

/// 0x1412af3a0（O 结构，26KB）：DoorStairs 差分重装闭包。
pub fn door_stairs_reassemble(
    _context_a: u64,
    _context_b: u64,
    _context_d: u64,
) {
    // U：重装主体待取证。
}

/// 0x1412ae1c0 / 0x1412add20（O 结构，12KB 对）：参数比较闭包（FUN_1412accf0
/// ×2 + DecoratorStorageAddr::as_wall 0x140a1c240）。U：比较面待取证。
pub fn compare_assembly_params(
    _a: u64,
    _b: u64,
    _c: u64,
) -> bool {
    false // U。
}
