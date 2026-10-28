//! B `cottage_dormers/maintain_dormer_tiles_draw.rs` — 老虎窗瓦绘制维护（D1）。
//!
//! 证据：fresh Ghidra shared 反编译 0x1407ce920（maintain_dormer_tiles，2KB，
//! `reports/state/decorator/review-evidence/fresh-decomp/0x1407ce920_*.c`，
//! reviewer 独立 diff 一致）。
//!
//! 机器码（O 全文）：param_1 = &mut 绘制状态（+0x00 回调注册表、+0x10 u32 槽指针
//! 实例 id 槽、+0x1c u32 值、+0x20 hasher）；param_2 = &{*mut u64 共享游标,
//! 双瓦列表 {ptr@+8/+0x28、len@+0x10/+0x30、count@+0x18/+0x38、+0x40 总计数}}。
//! 主体：prev = *游标；startA = prev - countA（钳 0）、startB = prev - countB；
//! 若 lenA < startA → A 无新瓦（空）；**游标循环前预写** = `总计数 - lenB' -
//! lenA'`；遍历 A[startA..] 链 B[startB..] 的 0x18B 瓦元素，**每瓦**（O fresh，
//! 全部四步）：
//!   1. `*游标 += 1`；
//!   2. 实例 id 单槽 store（`*[ids槽] = +0x1c 值`，R13 循环内不变、非 push）；
//!   3. `hash_one<RandomState, &GlobalId>`（0x1405b0d70，输入 = 元素+8）+
//!      `RawTable::remove_entry<u64, V>`（0x140053320）；
//!   4. 返回值非空时 `drop_in_place<SparseInstanceBufferAllocation<
//!      RoofTileSsbo>>`（0x141259000；瓦离开绘制集 → 释放稀疏分配）。
//! **末值 = total**（预写 total-lenA'-lenB' + 每瓦 +1 × (lenA'+lenB')）。
//!
//! ⚠ 2026-10-17 consume-types fresh asm 纠错：wave3 曾称 0x1405b0d70/
//! 0x140053320 为"虚构变换写 helper"；`get_functions` 签名证明二者分别为
//! `hash_one<RandomState, &GlobalId>` 与 `RawTable::remove_entry<u64,…>`，
//! 0x141259000 为 `drop_in_place<SparseInstanceBufferAllocation<RoofTileSsbo>>`。
//! 本文件按此三调用落码。
//!
//! U 登记：① `countA/countB` 语义（O 名未直接可见，按字段位置命名）；
//! ② 表值类型 PDB 记 `Box<dyn FnOnce(*mut c_void, bool)>`，但 drop 符号为
//! `SparseInstanceBufferAllocation<RoofTileSsbo>`（ICF 别名）——以分配类型承载。

/// 0x18B 瓦元素（O：{前 8B 数据, GlobalId u64 @+8, 后 8B}；decomp 每瓦
/// `hash_one<&GlobalId>` 的实参 = `元素+8`，见 fresh asm 1407cea19）。
#[derive(Clone, Copy, Debug, Default)]
pub struct DormerTileElem {
    pub data: [u8; 24],
}

impl DormerTileElem {
    /// O：`GlobalId` @元素+8（`hash_one<std::hash::random::RandomState,
    /// ref<country_core::systems::ui::GlobalId>>` 的输入；`GlobalId` 为
    /// `#[repr(transparent)] u64`）。
    pub fn global_id(&self) -> u64 {
        u64::from_le_bytes(self.data[8..16].try_into().unwrap())
    }
}

/// 绘制状态（O：+0x00 回调注册表、+0x10 id 输出槽指针、+0x1c u32、+0x20 hasher）。
#[derive(Debug)]
pub struct DormerTilesDrawState {
    /// +0x00：瓦 → GPU 稀疏分配退役表（O：`RawTable<u64, …>`；键 = `GlobalId`，
    /// 每瓦 `RawTable::remove_entry` 后条件 `drop_in_place<…Allocation<RoofTileSsbo>>`）。
    /// 值类型由 drop 符号 0x141259000 锚定（PDB 的 `Box<dyn FnOnce>` 为 ICF
    /// 别名，不据此造回调面）。
    pub retire: hashbrown::HashMap<
        u64,
        country_core::render::sparse_instance_buffer::SparseInstanceBufferAllocation<
            country_core::resources::roofs::roof_tile_ssbo::RoofTileSsbo,
        >,
    >,
    /// +0x10：实例 id 输出槽（O：`*mut u32`，R13 循环内只加载一次、永不递增；
    /// 每瓦 `*slot = +0x1c 值`——**单槽 store，非 Vec push**，wave4 G3）。
    pub id_slot: *mut u32,
    /// +0x1c：实例 id 值（写入 `id_slot`）。
    pub value: u32,
}

impl Default for DormerTilesDrawState {
    fn default() -> Self {
        Self {
            retire: hashbrown::HashMap::new(),
            // O：槽指针由宿主提供；null 时下方跳过 store（Rust 侧安全 seam）。
            id_slot: core::ptr::null_mut(),
            value: 0,
        }
    }
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

/// 0x1407ce920（O，fresh 复核 2026-10-17）：维护老虎窗瓦绘制。
///
/// 游标推进/id 单槽 store/双列表遍历 **判等**；游标写回 **写序修正（FIX-E，reviewer
/// wave3-toolbar-decorator）**——二进制**循环前**写 `*cursor = total - lenB' -
/// lenA'`、每瓦 `*cursor += 1` → **末值 = total**。
///
/// **每瓦 GPU 退役（FIX-2 已落）**：fresh asm 1407cea19-1407cea58 逐指令：
///   1. `hash_one<RandomState, &GlobalId>`（0x1405b0d70，GlobalId = 元素+8）；
///   2. `RawTable::remove_entry<u64, V>`（0x140053320）；
///   3. 返回值非空 → `drop_in_place<SparseInstanceBufferAllocation<RoofTileSsbo>>`
///      （0x141259000）。
/// Rust 以 `hashbrown::HashMap<u64, SparseInstanceBufferAllocation<RoofTileSsbo>>`
/// 的 `remove` + 作用域 drop 承载（`remove_entry` 的返回值即被 drop 的分配）。
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
    // O（fresh 0x1407ce920 行 47）：游标**循环前**预写 = 总计数 - lenB' - lenA'，
    // 每瓦 `*cursor += 1` → **末值 = total**。
    unsafe {
        *buffer.cursor = buffer.total.wrapping_sub(len_b).wrapping_sub(len_a);
    }
    // O：遍历 A[startA..] 链 B[startB..]。
    let iter = buffer
        .list_a
        .iter()
        .skip(start_a as usize)
        .chain(buffer.list_b.iter().skip(start_b as usize));
    for tile in iter {
        // O：每瓦四步——① *cursor += 1 ② id 单槽 store ③ hash_one +
        // remove_entry ④ 返回值非空时 drop 稀疏分配。
        let cur = unsafe { &mut *buffer.cursor };
        *cur = cur.wrapping_add(1);
        // O：`MOV [R13],EBP`——R13 = state+0x10 指向的 u32 槽，循环内不变、
        // 不递增、无容量检查（wave4 G3：单槽 store，不是 Vec push）。
        if !state.id_slot.is_null() {
            unsafe { *state.id_slot = state.value };
        }
        // O：键 = 元素+8 的 `GlobalId`（hash_one 输入）。
        let global_id = tile.global_id();
        if let Some(allocation) = state.retire.remove(&global_id) {
            // O：`local_50 != 0` → `drop_in_place<SparseInstanceBufferAllocation<
            // RoofTileSsbo>>`（0x141259000）——句柄归还未用 chunk。
            drop(allocation);
        }
    }
}
