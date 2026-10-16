//! B `cottage_dormers/maintain_dormer_tiles_draw.rs` — 老虎窗瓦绘制维护（D1）。
//!
//! 证据：fresh RE-MCP 反编译 0x1407ce920（maintain_dormer_tiles，2KB，O 全文）。
//!
//! 机器码（O 全文）：param_1 = &mut 绘制状态（+0x00 状态基、+0x10 Vec<u32>
//! 实例 id 槽、+0x1c u32 值）；param_2 = &{*mut u64 共享游标, 双瓦列表
//! {ptr@+8/+0x28、len@+0x10/+0x30、count@+0x18/+0x38、+0x40 总计数}}。
//! 主体：prev = *游标；startA = prev - countA（钳 0）、startB = prev - countB；
//! 若 lenA < startA → A 无新瓦（空）；遍历 A[startA..] 链 B[startB..] 的
//! 0x18B 瓦元素 → `*游标 += 1`、实例 id 入 Vec、瓦变换写（FUN_1405b0d70(
//! 状态基+0x20, elem+8) + FUN_140053320 四参写入）；游标回写 =
//! `总计数 - (lenB - startB) - (lenA - startA)`（各钳 0）。
//! U 登记：① 0x18B 瓦元素布局 + 写入 helper（FUN_1405b0d70 / 0x140053320 目标
//! 缓冲类型）② countA/countB 语义。

/// 0x18B 瓦元素（D1 布局 I：{前 8B 变换系, ...}；U）。
#[derive(Clone, Copy, Debug, Default)]
pub struct DormerTileElem {
    pub data: [u8; 24],
}

/// 绘制状态（O 面：+0x10 Vec<u32>、+0x1c u32、+0x00 状态基）。
#[derive(Clone, Debug, Default)]
pub struct DormerTilesDrawState {
    /// +0x00：状态基（U）。
    pub base: u64,
    /// +0x10：实例 id 槽（Vec<u32>，每瓦 push +0x1c 值）。
    pub ids: Vec<u32>,
    /// +0x1c：实例 id 值。
    pub value: u32,
}

/// 瓦绘制缓冲视图（O 面：共享游标 + 双列表 + 总计数）。
#[derive(Clone, Debug, Default)]
pub struct DormerTileDrawBuffer {
    /// 共享游标（+0x00 指向 u64）。
    pub cursor: *mut u64,
    /// 列表 A（ptr@+8、len@+0x10、count@+0x18）。
    pub list_a: Vec<DormerTileElem>,
    /// 列表 B（ptr@+0x28、len@+0x30、count@+0x38）。
    pub list_b: Vec<DormerTileElem>,
    /// +0x40：总计数。
    pub total: u64,
    /// countA / countB（O 布局面）。
    pub count_a: u64,
    pub count_b: u64,
}

// SAFETY：cursor 为外部缓冲所有权（U），仅方法内使用。
unsafe impl Send for DormerTileDrawBuffer {}

/// 0x1407ce920（O 全文）：维护老虎窗瓦绘制。
pub fn maintain_dormer_tiles(
    state: &mut DormerTilesDrawState,
    buffer: &mut DormerTileDrawBuffer,
) {
    let prev = unsafe { *buffer.cursor };
    let start_a = prev.saturating_sub(buffer.count_a);
    let start_b = prev.saturating_sub(buffer.count_b);
    // O：lenA < startA → 空列表（lVar11 = 8 空哨兵，len' = 0）。
    let len_a = if buffer.list_a.len() as u64 >= start_a {
        buffer.list_a.len() as u64 - start_a
    } else {
        0
    };
    let len_b = if buffer.list_b.len() as u64 >= start_b {
        buffer.list_b.len() as u64 - start_b
    } else {
        0
    };
    // O：遍历 A[startA..] 链 B[startB..]。
    let iter = buffer
        .list_a
        .iter()
        .skip(start_a as usize)
        .chain(buffer.list_b.iter().skip(start_b as usize));
    for tile in iter {
        // O：*cursor += 1；实例 id 入 Vec。
        let cur = unsafe { &mut *buffer.cursor };
        *cur = cur.wrapping_add(1);
        state.ids.push(state.value);
        // U：瓦变换写入（FUN_1405b0d70(state.base+0x20, elem+8) +
        // FUN_140053320 四参面；目标缓冲类型 U）。
        let _ = tile;
    }
    // O：游标回写 = 总计数 - lenB' - lenA'。
    unsafe {
        *buffer.cursor = buffer.total.wrapping_sub(len_b).wrapping_sub(len_a);
    }
}
