//! B `resources/history/decorator_edit.rs` — 装饰历史编辑/状态/差分类型（章节 O）。
//!
//! 2026-10-17 类型锚定（impl-history-difftypes，fresh decomp batch1）：
//! - `DecoratorHistoryState` 为原生 ~0x4fB 布局（`impl$1::from` 0x140b3f930）：
//!   DecoratorInfo 前 0x48B 直拷 + style 派生簇 + style 字节 @+0x4e。原 Rust
//!   `{id, style, color, lantern_brightness, stair_width}` 5 字段平面近似已被替换。
//! - `DecoratorStorageAddrHistoryState` / `DecoratorStorageHistoryDiff` 为
//!   `DecoratorArchivist::extract_diff`（0x140b2cb30）的输出/中间类型：先
//!   `from_iter<Addr,(AddrState,AddrState)>` 再 filter_map 成
//!   `HashMap<Addr, DecoratorStorageHistoryDiff>`。
//! - `DecoratorHistoryEdit::serialize`（0x1404f46e0）= 单 entry map
//!   `{"<id>": {before, after}}`（键=id 十进制串，值=nested diff）。
//!
//! 挂账（不编造）：state 内部 serde 原生为 style 分变体选字段写
//! （`convert_before_after_states_to_changes` 0x140b3fab0 /
//! `send_state_diff_events` 0x1417bdae0 的 `__SerializeWith`），Rust 侧暂为
//! 全字段 derive；`restore_decorators` 消费链在 walls/decorator 域实现。
use crate::resources::walls::decorator::{DecoratorId, DecoratorInfo, DeferredDecoratorOpEvents};
use crate::resources::walls::decorator_storage::DecoratorStorageAddr;
use serde::ser::SerializeMap;
use serde::{Deserialize, Deserializer, Serializer};
use std::collections::{HashMap, HashSet};

/// 装饰历史编辑（原生 0x1404f46e0：单 entry map `{"diffs": {<addr>: diff}}`）。
///
/// fresh serialize 0x1404f46e0：`{` + 1 个
/// `serialize_entry<str, __SerializeWith>` + `}`；键常量 @0x1429ec6c6 len 5 = `"diffs"`，
/// 值 = `vectorize::serialize<HashMap<DecoratorStorageAddr,
/// DecoratorStorageHistoryDiff>>`（0x1404be820）。即载荷是 **0x30B diff map**，
/// 不是旧的 `{id,before,after}` 平面结构。
#[derive(Clone, Debug, Default, PartialEq)]
pub struct DecoratorHistoryEdit {
    pub diffs: DecoratorHistoryDiffMap,
}

/// 装饰历史状态（原生 ~0x4fB；`impl$1::from` 0x140b3f930）。
///
/// 原生布局（fresh decomp-0x140b3f930 逐写确认）：DecoratorInfo 前 0x48B 字段
/// 直拷，+0x30..0x44 为 style 派生簇，+0x48..0x4e 为 rank/u45/ty/派生字节/style。
/// style 字节 = `DecoratorInfo.extra_style()`（@+0x41 = `u40` byte1）。
///
/// | style | 类型 | 派生簇 |
/// |---|---|---|
/// | 4 | Chimney{ f32,u8 } | +0x4d = u30 byte4（u8）；u30（f32）@+0x30 不落 state |
/// | 5 | Flag{ u8 } | +0x4c = u30 byte0 |
/// | 6 | Window{ u32 } | +0x34 = u30 low（u32） |
/// | 7 | Lantern{ f32 } | +0x38 = u30 low（f32） |
/// | 8+ / <2 | 默认簇 | +0x3c=u30 high、+0x40=u38、+0x44=u3c、+0x4b=u40 byte0、style@+0x4e |
/// | 2..7 | 其余 | +0x3c/+0x40/+0x44=0、+0x4b=0、style@+0x4e=0 |
#[derive(Clone, Copy, Debug, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct DecoratorHistoryState {
    /// +0x00：目标墙 id（DecoratorInfo.dst_wall）。
    pub dst_wall: u64,
    /// +0x08：目标次级 id。
    pub dst_secondary: u64,
    /// +0x10：目标载荷。
    pub dst_payload: u64,
    /// +0x18：墙空间数据。
    pub u18: u64,
    /// +0x20：u32（`DecoratorInfo.u20`）。
    pub u20: u32,
    /// +0x24：u32（`DecoratorInfo.u24`）。
    pub u24: u32,
    /// +0x28：u64（`DecoratorInfo.u28`）。
    pub u28: u64,
    /// +0x30：style 派生（style==4/8+ 或 <2 → `u30` low，否则 0）。
    pub u30: u32,
    /// +0x34：style==6（Window）→ `u30` low（u32），否则 0。
    pub u34: u32,
    /// +0x38：style==7（Lantern）→ `u30` low（f32），否则 1.0。
    pub u38: u32,
    /// +0x3c：style 8+ 或 <2 → `u30` high（f32），否则 1.0。
    pub u3c: u32,
    /// +0x40：style 8+ 或 <2 → `u38`，否则 0。
    pub u40: u32,
    /// +0x44：style 8+ 或 <2 → `u3c`，否则 0。
    pub u44: u32,
    /// +0x48：rank。
    pub rank: u8,
    /// +0x49：u45。
    pub u49: u8,
    /// +0x4a：DecoratorType。
    pub ty: u8,
    /// +0x4b：style 8+ 或 <2 → `u40` byte0，否则 0。
    pub u4b: u8,
    /// +0x4c：style==5（Flag）→ `u30` byte0（u8），否则 0。
    pub u4c: u8,
    /// +0x4d：style==4（Chimney）→ `u30` byte4（u8），否则 1。
    pub u4d: u8,
    /// +0x4e：style 字节（= `DecoratorInfo.extra_style()`，style 2..7 时为 0）。
    pub style: u8,
}

impl DecoratorHistoryState {
    /// `DecoratorHistoryState::from`（0x140b3f930）：DecoratorInfo → state。
    ///
    /// fresh 逐写映射见模块头布局表。入口：`info.state != 0` 视为 preview → panic
    /// "Attempting to capture a DecoratorHistoryState of a preview"。
    pub fn from_info(info: &DecoratorInfo) -> Self {
        Self::from(info)
    }
    /// `is_default_lantern_brightness`==`is_default_stair_width`==`x==1.0f`
    /// （ICF 0x140b3f910；`cmpeqss` NaN false）。lantern f32 @+0x38（style 7，
    /// 默认 1.0）。
    pub fn is_default_lantern_brightness(&self) -> bool {
        f32::from_bits(self.u38) == 1.0
    }
    /// stair width 字段偏移待取证（原生 ICF 与 lantern 同址，仅 `x==1.0`；
    /// 读 +0x3c 为最近拟，登记）。
    pub fn is_default_stair_width(&self) -> bool {
        f32::from_bits(self.u3c) == 1.0
    }

    /// B restore_snapshot 0x140a20f44..0x140a2107d chooses extra data by
    /// DecoratorType, including default-valued extras. It does not infer the
    /// variant from which optional field happens to differ from its default.
    ///
    /// Clutter（`ty < 0x38`）与未知类型在原生 `ty - 0x38` 夹取后走 index 6
    /// 分支（0x140a20d70 clamp+switch）：该分支保留 `u30/u38/u3c/u40` 原始槽，
    /// style 回读 `state.style`（Clutter 为 0）。不得把扩展簇清零或把 style
    /// 默认写成 3（round9 6/134 样本状态丢失的根因）。
    pub fn to_info(&self) -> DecoratorInfo {
        let style = match self.ty {
            0x38..=0x3a => 6,
            0x3b => 4,
            0x3c => 7,
            0x3d => 5,
            // `0x3f` 与 Clutter（<0x38）/未知类型共享 style 回读。
            _ => self.style,
        };
        let (u30, u38, u3c, extra_byte) = match self.ty {
            0x38..=0x3a => (u64::from(self.u34), 0, 0, 0),
            0x3b => (u64::from(self.u30) | (u64::from(self.u4d) << 32), 0, 0, 0),
            0x3c => (u64::from(self.u38), 0, 0, 0),
            0x3d => (u64::from(self.u4c), 0, 0, 0),
            // 0x3f 与 Clutter/未知：保留原始扩展簇槽。
            _ => (
                u64::from(self.u30) | (u64::from(self.u3c) << 32),
                self.u40,
                self.u44,
                self.u4b,
            ),
        };
        DecoratorInfo {
            dst_wall: self.dst_wall,
            dst_secondary: self.dst_secondary,
            dst_payload: self.dst_payload,
            u18: self.u18,
            u20: self.u20,
            u24: self.u24,
            u28: self.u28 as u32,
            decorator_id: (self.u28 >> 32) as u32,
            u30,
            u38,
            u3c,
            u40: (u32::from(style) << 8) | u32::from(extra_byte),
            rank: self.rank,
            u45: self.u49,
            ty: self.ty,
            state: 0,
        }
    }
}

/// `impl$1::from` 0x140b3f930（preview 判别 + style 分变体派生）。
impl From<&DecoratorInfo> for DecoratorHistoryState {
    fn from(info: &DecoratorInfo) -> Self {
        if info.state != 0 {
            panic!("Attempting to capture a DecoratorHistoryState of a preview");
        }
        let style = info.extra_style();
        // bVar12 = clamp(style-2, 0..=5) else 6（fresh：`(byte)(style-2)<6`）。
        let b12: u8 = {
            let d = (style as i16) - 2;
            if (0..6).contains(&d) { d as u8 } else { 6 }
        };
        let u30 = info.u30 as u32; // u32 @ +0x30（`u30` low）
        let u30_hi = (info.u30 >> 32) as u32; // u32 @ +0x34（`u30` high）
        // uVar14：b12==2（style==4）或 b12==6（style 8+/<2）→ u30，否则 0。
        let u30_field: u32 = if b12 == 2 || b12 == 6 { u30 } else { 0 };
        // 分支读：`(byte)(style-8) < 0xfa` ⟺ style ∈ {0,1} ∪ {8..=255}。
        let read_branch = style == 0 || style == 1 || style >= 8;
        let (u3c_field, u40_field, u4b_field, u44_field, style_field) = if read_branch {
            (u30_hi, info.u38, (info.u40 & 0xff) as u8, info.u3c, style)
        } else {
            (f32::to_bits(1.0), 0, 0, 0, 0)
        };
        // uVar19：style==7（Lantern f32）→ u30，否则 1.0。
        let u38_field: u32 = if style == 7 { u30 } else { f32::to_bits(1.0) };
        // uVar13：style==6（Window u32）→ u30，否则 0。
        let u34_field: u32 = if style == 6 { u30 } else { 0 };
        // uVar16：style==5（Flag u8）→ u30 byte0，否则 0。
        let u4c_field: u8 = if style == 5 { (u30 & 0xff) as u8 } else { 0 };
        // uVar18：style==4（Chimney u8）→ u30 byte4，否则 1。
        let u4d_field: u8 = if style == 4 { (u30_hi & 0xff) as u8 } else { 1 };
        Self {
            dst_wall: info.dst_wall,
            dst_secondary: info.dst_secondary,
            dst_payload: info.dst_payload,
            u18: info.u18,
            u20: info.u20,
            u24: info.u24,
            u28: ((info.decorator_id as u64) << 32) | u64::from(info.u28),
            u30: u30_field,
            u34: u34_field,
            u38: u38_field,
            u3c: u3c_field,
            u40: u40_field,
            u44: u44_field,
            rank: info.rank,
            u49: info.u45,
            ty: info.ty,
            u4b: u4b_field,
            u4c: u4c_field,
            u4d: u4d_field,
            style: style_field,
        }
    }
}

/// 装饰历史差异（单 id before/after；`replay_pop_bucket` 闭包持 `Arc<DecoratorHistoryDiff>`
/// 递减计数）。
#[derive(Clone, Debug, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct DecoratorHistoryDiff {
    pub before: DecoratorHistoryState,
    pub after: DecoratorHistoryState,
}

/// 装饰存储地址的历史状态（原生 `DecoratorStorageAddrHistoryState`）。
///
/// `DecoratorArchivist::extract_diff`（0x140b2cb30）的中间 map 值类型：
/// `from_iter<Addr,(AddrHistoryState,AddrHistoryState)>`。单地址（`Decorators` 桶）
/// 的状态由 `extract_before_after_impl`（0x140b2be70）经
/// `from_iter<DecoratorId, DecoratorHistoryState, Map<Iter<HashMap<DecoratorId,DecoratorInfo>>, impl$1::from>>`
/// 构建（逐条目 `DecoratorInfo → DecoratorHistoryState`）；removed 面（桶
/// `removed_ids`）为恢复语义所需，字段精确位置以类型面登记。
#[derive(Clone, Debug, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct DecoratorStorageAddrHistoryState {
    /// 该地址逐 id 的 DecoratorHistoryState。
    pub states: HashMap<DecoratorId, DecoratorHistoryState>,
    /// 该地址已移除的 id 集（桶 `removed_ids` 面；恢复时从 d2s 移除）。
    pub removed: HashSet<DecoratorId>,
}

/// 装饰存储地址的历史差分（原生 `DecoratorStorageHistoryDiff`）。
///
/// `extract_diff` 输出 `HashMap<DecoratorStorageAddr, DecoratorStorageHistoryDiff>`
/// （decomp：filter_map closure 将 `(Addr,(before,after))` 二元组收敛为具名结构）。
#[derive(Clone, Debug, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct DecoratorStorageHistoryDiff {
    pub before: DecoratorStorageAddrHistoryState,
    pub after: DecoratorStorageAddrHistoryState,
}

// ---------------------------------------------------------------------------
// diff 事件链（2026-10-17 restorer-diffevents，fresh 反编译）
//
// 证据：
// - `send_state_diff_events` 0x1417bdae0（restore_decorators 闭包 0x14155d13f 调用，
//   入参 RCX=&HashMap<DecoratorId, DecoratorBeforeAfterWithStorage>，RDX=捕获环境，
//   `[RDX]`=DeferredDecoratorOpEvents）。
// - `convert_before_after_states_to_changes` 0x140b3fab0（RCX=out changes，
//   RDX=before、R8=after，二者均指向 0x60B “存储槽 + 0x50B 状态”）。
// - 事件记录 0x88B，经 `DeferredDecoratorOpEvents::send`（0x1408336b0）入队。
// ---------------------------------------------------------------------------

/// 原生 `DecoratorHistoryDiff`：`extract_diff` 返回的 0x30B map
/// （`HashMap<DecoratorStorageAddr, DecoratorStorageHistoryDiff>`）。
///
/// fresh 0x140a854a0：`Arc<DecoratorHistoryDiff>` 载荷 0x30B（= `HashMap` 本体），
/// 与旧的 `{before, after}` 结构（0xa0）不同。旧结构保留给
/// `DecoratorHistoryEdit` 的 wire 形；本别名是原生 add_decorator_diff 面。
pub type DecoratorHistoryDiffMap = HashMap<DecoratorStorageAddr, DecoratorStorageHistoryDiff>;

/// `DecoratorBeforeAfterWithStorage` 的单侧：0x60B。
///
/// fresh `restore_decorators`（0x14155ba3f..0x14155d13f）与
/// `send_state_diff_events` 0x1417bdae0 逐字段确认：
/// - +0x00 `u64`：存储槽判别（`CMP R14,3/4`；`DecoratorStorageAddr` 的 0/1/2；
///   3 = 该侧存储缺失，4 = 终止标记）；
/// - +0x08 `u64`：存储槽载荷（Wall 变体为 WallId，Terrain/Stair 不比较）；
/// - +0x10 `DecoratorHistoryState`（0x50B，见 `impl$1::from`）。
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct DecoratorStateWithStorage {
    /// 存储槽判别（原生 +0x00）。
    pub storage_disc: u64,
    /// 存储槽载荷（原生 +0x08）。
    pub storage_payload: u64,
    /// 装饰状态（原生 +0x10..+0x5f）。
    pub state: DecoratorHistoryState,
}

impl DecoratorStateWithStorage {
    /// 原生逐字节布局（0x60B；状态尾部 +0x4f 为 padding）。
    pub fn to_bytes(self) -> [u8; 0x60] {
        let mut out = [0u8; 0x60];
        out[0x00..0x08].copy_from_slice(&self.storage_disc.to_le_bytes());
        out[0x08..0x10].copy_from_slice(&self.storage_payload.to_le_bytes());
        out[0x10..0x60].copy_from_slice(&self.state.to_bytes());
        out
    }
}

/// `DecoratorBeforeAfterWithStorage`：`HashMap<DecoratorId, _>` 的值（0xc0B）。
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct DecoratorBeforeAfterWithStorage {
    pub before: DecoratorStateWithStorage,
    pub after: DecoratorStateWithStorage,
}

impl DecoratorHistoryState {
    /// 原生 `DecoratorHistoryState` 字节布局（0x50B；`+0x4e` style，`+0x4f` padding）。
    pub fn to_bytes(self) -> [u8; 0x50] {
        let mut out = [0u8; 0x50];
        out[0x00..0x08].copy_from_slice(&self.dst_wall.to_le_bytes());
        out[0x08..0x10].copy_from_slice(&self.dst_secondary.to_le_bytes());
        out[0x10..0x18].copy_from_slice(&self.dst_payload.to_le_bytes());
        out[0x18..0x20].copy_from_slice(&self.u18.to_le_bytes());
        out[0x20..0x24].copy_from_slice(&self.u20.to_le_bytes());
        out[0x24..0x28].copy_from_slice(&self.u24.to_le_bytes());
        out[0x28..0x30].copy_from_slice(&self.u28.to_le_bytes());
        out[0x30..0x34].copy_from_slice(&self.u30.to_le_bytes());
        out[0x34..0x38].copy_from_slice(&self.u34.to_le_bytes());
        out[0x38..0x3c].copy_from_slice(&self.u38.to_le_bytes());
        out[0x3c..0x40].copy_from_slice(&self.u3c.to_le_bytes());
        out[0x40..0x44].copy_from_slice(&self.u40.to_le_bytes());
        out[0x44..0x48].copy_from_slice(&self.u44.to_le_bytes());
        out[0x48] = self.rank;
        out[0x49] = self.u49;
        out[0x4a] = self.ty;
        out[0x4b] = self.u4b;
        out[0x4c] = self.u4c;
        out[0x4d] = self.u4d;
        out[0x4e] = self.style;
        out
    }
}

/// `convert_before_after_states_to_changes`（0x140b3fab0）输出（0x58B）。
///
/// 字段偏移严格按 fresh disasm：
/// `+0x00` 存储判别（未变更 = 3，否则 after 判别）、`+0x08` 存储载荷、
/// `+0x10..+0x2f` after 状态 `dst_wall/dst_secondary/dst_payload/u18`、
/// `+0x30/+0x34` after `u20/u24`、`+0x38` after rank、
/// `+0x40` u30 变更标志（CMPNEQSS：NaN 视为变更）、`+0x44` after u30 f32、
/// `+0x48` u3c 变更标志、`+0x4c` after u3c f32、
/// `+0x50` rank 变更、`+0x51` after rank、`+0x52` decorator_id 变更、
/// `+0x53` id 差低字节、`+0x54` u4c（相等时写 2）、`+0x55` 类型标记、
/// `+0x39..+0x3f`/`+0x56..+0x57` padding。
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct DecoratorStateChanges {
    pub storage_disc: u64,
    pub storage_payload: u64,
    pub dst_wall: u64,
    pub dst_secondary: u64,
    pub dst_payload: u64,
    pub u18: u64,
    pub u20: u32,
    pub u24: u32,
    pub rank_copy: u8,
    pub u30_changed: u32,
    pub u30: f32,
    pub u3c_changed: u32,
    pub u3c: f32,
    pub rank_changed: u8,
    pub rank_value: u8,
    pub id_changed: u8,
    pub id_delta: u8,
    pub u4c_or_2: u8,
    pub type_marker: u8,
}

impl DecoratorStateChanges {
    /// 原生大小（0x58B）。
    pub const SIZE: usize = 0x58;
    /// 按 fresh 偏移写出 0x58B。
    pub fn to_bytes(&self) -> [u8; 0x58] {
        let mut out = [0u8; 0x58];
        out[0x00..0x08].copy_from_slice(&self.storage_disc.to_le_bytes());
        out[0x08..0x10].copy_from_slice(&self.storage_payload.to_le_bytes());
        out[0x10..0x18].copy_from_slice(&self.dst_wall.to_le_bytes());
        out[0x18..0x20].copy_from_slice(&self.dst_secondary.to_le_bytes());
        out[0x20..0x28].copy_from_slice(&self.dst_payload.to_le_bytes());
        out[0x28..0x30].copy_from_slice(&self.u18.to_le_bytes());
        out[0x30..0x34].copy_from_slice(&self.u20.to_le_bytes());
        out[0x34..0x38].copy_from_slice(&self.u24.to_le_bytes());
        out[0x38] = self.rank_copy;
        out[0x40..0x44].copy_from_slice(&self.u30_changed.to_le_bytes());
        out[0x44..0x48].copy_from_slice(&self.u30.to_le_bytes());
        out[0x48..0x4c].copy_from_slice(&self.u3c_changed.to_le_bytes());
        out[0x4c..0x50].copy_from_slice(&self.u3c.to_le_bytes());
        out[0x50] = self.rank_changed;
        out[0x51] = self.rank_value;
        out[0x52] = self.id_changed;
        out[0x53] = self.id_delta;
        out[0x54] = self.u4c_or_2;
        out[0x55] = self.type_marker;
        out
    }
}

#[inline]
fn u32_at(bytes: &[u8], offset: usize) -> u32 {
    u32::from_le_bytes(bytes[offset..offset + 4].try_into().unwrap())
}
#[inline]
fn u64_at(bytes: &[u8], offset: usize) -> u64 {
    u64::from_le_bytes(bytes[offset..offset + 8].try_into().unwrap())
}
#[inline]
fn f32_at(bytes: &[u8], offset: usize) -> f32 {
    f32::from_bits(u32_at(bytes, offset))
}

/// 原生 dst-tag → selector：`(tag-2) < 3 ? tag-1 : 0`（tag∈{2,3,4} → 1..3）。
fn selector_before(tag: u8) -> u8 {
    if tag.wrapping_sub(2) < 3 {
        tag.wrapping_sub(1)
    } else {
        0
    }
}
/// 原生 after selector：`tag < 2 ? 0 : tag-1`。
fn selector_after(tag: u8) -> u8 {
    if tag < 2 { 0 } else { tag.wrapping_sub(1) }
}

/// 存储槽相等：判别低 32 位相等，且（判别 != 0 或载荷相等）。
fn storage_equal(before: &DecoratorStateWithStorage, after: &DecoratorStateWithStorage) -> bool {
    let disc = after.storage_disc as u32;
    (before.storage_disc as u32) == disc
        && (disc != 0 || before.storage_payload == after.storage_payload)
}

/// 变体相关的状态字段比较（fresh disasm 0x140b3fb80/0x140b3fcac/0x140b3fbd8）。
///
/// 浮点比较使用 `==`：NaN 比较为 false（等价 UCOMISS 的 ZF=0/PF=1 双跳）。
fn state_fields_equal(before: &[u8; 0x50], after: &[u8; 0x50]) -> bool {
    let sel_b = selector_before(before[0x22]);
    let sel_a = selector_after(after[0x22]);
    if sel_b != sel_a {
        return false;
    }
    match sel_b {
        // dst tag 2/3：半槽 +0x10/+0x14/+0x18 = 状态 +0x00/+0x04/+0x08。
        1 | 2 => {
            f32_at(before, 0x00) == f32_at(after, 0x00)
                && f32_at(before, 0x04) == f32_at(after, 0x04)
                && f32_at(before, 0x08) == f32_at(after, 0x08)
        }
        // dst tag 4：整数 @ 状态 +0x00/+0x04，f32 @ +0x08/+0x0c/+0x10。
        3 => {
            u32_at(before, 0x00) == u32_at(after, 0x00)
                && u32_at(before, 0x04) == u32_at(after, 0x04)
                && f32_at(before, 0x08) == f32_at(after, 0x08)
                && f32_at(before, 0x0c) == f32_at(after, 0x0c)
                && f32_at(before, 0x10) == f32_at(after, 0x10)
        }
        // 默认（tag 0/1 等）：f32 对 + qword + 字节 + tag 相等。
        _ => {
            f32_at(before, 0x00) == f32_at(after, 0x00)
                && f32_at(before, 0x04) == f32_at(after, 0x04)
                && f32_at(before, 0x10) == f32_at(after, 0x10)
                && f32_at(before, 0x14) == f32_at(after, 0x14)
                && f32_at(before, 0x18) == f32_at(after, 0x18)
                && f32_at(before, 0x1c) == f32_at(after, 0x1c)
                && before[0x20] == after[0x20]
                && u64_at(before, 0x08) == u64_at(after, 0x08)
                && before[0x22] == after[0x22]
                && before[0x21] == after[0x21]
        }
    }
}

/// 原生类型标记（+0x55）：`ty-0x38` 夹取（>=8 → 6）。
fn clamp_ty(ty: u8) -> u8 {
    let d = ty.wrapping_sub(0x38);
    if d >= 8 { 6 } else { d }
}

/// `convert_before_after_states_to_changes`（0x140b3fab0）逐分支实现。
pub fn convert_before_after_states_to_changes(
    before: &DecoratorStateWithStorage,
    after: &DecoratorStateWithStorage,
) -> DecoratorStateChanges {
    let sb = before.state.to_bytes();
    let sa = after.state.to_bytes();
    let unchanged = storage_equal(before, after) && state_fields_equal(&sb, &sa);

    let before_id = u32_at(&sb, 0x2c);
    let after_id = u32_at(&sa, 0x2c);
    let before_ty = sb[0x4a];
    let after_ty = sa[0x4a];
    let type_marker = if clamp_ty(before_ty) != clamp_ty(after_ty) {
        after_ty
    } else if clamp_ty(before_ty) != 6 || before_ty == after_ty {
        // 0x40 = “类型未变/仅低 3 位类型族相同”。
        0x40
    } else {
        after_ty
    };

    DecoratorStateChanges {
        storage_disc: if unchanged { 3 } else { after.storage_disc },
        storage_payload: after.storage_payload,
        dst_wall: u64_at(&sa, 0x00),
        dst_secondary: u64_at(&sa, 0x08),
        dst_payload: u64_at(&sa, 0x10),
        u18: u64_at(&sa, 0x18),
        u20: u32_at(&sa, 0x20),
        u24: u32_at(&sa, 0x24),
        rank_copy: sa[0x48],
        // CMPNEQSS：单精度不相等（无序/NaN 也视为变更）。
        u30_changed: u32::from(f32_at(&sa, 0x30) != f32_at(&sb, 0x30)),
        u30: f32_at(&sa, 0x30),
        u3c_changed: u32::from(f32_at(&sa, 0x3c) != f32_at(&sb, 0x3c)),
        u3c: f32_at(&sa, 0x3c),
        rank_changed: u8::from(sa[0x48] != sb[0x48]),
        rank_value: sa[0x48],
        id_changed: u8::from(after_id != before_id),
        id_delta: after_id.wrapping_sub(before_id) as u8,
        u4c_or_2: if sa[0x4c] == sb[0x4c] { 2 } else { sa[0x4c] },
        type_marker,
    }
}

/// 0x88B 装饰 diff 事件记录（`DeferredDecoratorOpEvents` 队列元素）。
///
/// 布局（fresh `send_state_diff_events` 三条写出路径）：
/// `+0x00` 事件种类（0/1/2 = 变更时存储判别；3 = 新增；4 = 移除）、
/// `+0x08` 存储判别、`+0x10` 存储载荷、`+0x10..+0x68` 变更/状态负载、
/// `+0x68` DecoratorId、`+0x7c` =1、`+0x81` DecoratorType。
/// 原生未写出的 padding 字节（`+0x6c..+0x70` 等）按 0 建模并登记 U。
pub use crate::resources::walls::decorator::DecoratorOpEvent as DecoratorDiffEvent;

impl DecoratorDiffEvent {
    pub const SIZE: usize = 0x88;

    fn zeroed() -> Self {
        Self([0u8; 0x88])
    }
    fn put_u64(&mut self, offset: usize, value: u64) {
        self.0[offset..offset + 8].copy_from_slice(&value.to_le_bytes());
    }
    fn put_u32(&mut self, offset: usize, value: u32) {
        self.0[offset..offset + 4].copy_from_slice(&value.to_le_bytes());
    }
    /// +0x00：事件种类。
    pub fn kind(&self) -> u32 {
        u32_at(&self.0, 0x00)
    }
    /// 事件记录 id：按判别取不同偏移（fresh `DecoratorOpEventKind::id`
    /// 0x140833570 同式：disc 3 → +0x40，disc 4 → +0x60，其余 → +0x68）。
    pub fn decorator_id(&self) -> u32 {
        self.id()
    }
    /// +0x81：DecoratorType。
    pub fn decorator_ty(&self) -> u8 {
        self.0[0x81]
    }

    /// 两侧存储都在：`+0x00 = before 判别`，`+0x10 = changes(0x58)`。
    pub fn from_changes(
        key: DecoratorId,
        before: &DecoratorStateWithStorage,
        changes: &DecoratorStateChanges,
    ) -> Self {
        let mut event = Self::zeroed();
        event.put_u64(0x00, before.storage_disc);
        event.put_u64(0x08, before.storage_payload);
        event.0[0x10..0x10 + DecoratorStateChanges::SIZE].copy_from_slice(&changes.to_bytes());
        event.put_u32(0x68, key.0);
        event.0[0x7c] = 1;
        event.0[0x80] = 0;
        event.0[0x81] = before.state.ty;
        event
    }

    /// before 存储缺失（新增）：`+0x00 = 3`，负载为 after 状态前 0x28B + 键。
    pub fn from_added(key: DecoratorId, after: &DecoratorStateWithStorage) -> Self {
        let state = after.state.to_bytes();
        let mut event = Self::zeroed();
        event.put_u64(0x00, 3);
        event.put_u64(0x08, after.storage_disc);
        event.put_u64(0x10, after.storage_payload);
        event.0[0x18..0x38].copy_from_slice(&state[0x00..0x20]);
        event.0[0x38..0x40].copy_from_slice(&state[0x20..0x28]);
        event.put_u32(0x40, key.0);
        event.0[0x44] = 1;
        event.0[0x45] = state[0x48];
        event.0[0x7c] = 1;
        event.0[0x81] = state[0x4a];
        event
    }

    /// after 存储缺失（移除）：`+0x00 = 4`，按 before.type 的 0x38..0x3f 分派
    /// 选择风格簇字段（fresh jump table 0x142ce2360 case 0..7）。
    pub fn from_removed(key: DecoratorId, before: &DecoratorStateWithStorage) -> Self {
        // after 侧 u20 低字节：原生事件 +0x4c 在 case 0/1/2/4/5/6 使用
        // EDI（= after.state +0x20 的低 32 位）；after 状态字节由调用方提供。
        Self::from_removed_with_after(key, before, &DecoratorHistoryState::default())
    }

    /// 同上，但显式给出 after 状态（原生 `[0x1e0] = EDI = after +0x20`）。
    pub fn from_removed_with_after(
        key: DecoratorId,
        before: &DecoratorStateWithStorage,
        after_state: &DecoratorHistoryState,
    ) -> Self {
        let b = before.state.to_bytes();
        let a = after_state.to_bytes();
        let ty = b[0x4a];
        let case = clamp_ty(ty);
        let after_u20_low = a[0x20];

        let (field_48, byte_4c, tag_59) = match case {
            // 0/1/2：字段 = before.u34；+0x4c = after.u20 低字节；tag = 6。
            0 | 1 | 2 => (u32_at(&b, 0x34), after_u20_low, 6u8),
            // 3：字段 = before.u30；+0x4c = before.u4d；tag = 4。
            3 => (u32_at(&b, 0x30), b[0x4d], 4u8),
            // 4：字段 = before.u38（f32 位）；+0x4c = after.u20 低字节；tag = 7。
            4 => (u32_at(&b, 0x38), after_u20_low, 7u8),
            // 5：字段 = before.u4c（u8）；+0x4c = after.u20 低字节；tag = 5。
            5 => (u32::from(b[0x4c]), after_u20_low, 5u8),
            // 6：字段 = before.u34；+0x4c = after.u20 低字节；tag = 3。
            6 => (u32_at(&b, 0x34), after_u20_low, 3u8),
            // 7：字段 = before.u30（f32 位）；+0x4c = before.u3c 低字节；
            //    tag = before.style（低字节）。
            _ => (u32_at(&b, 0x30), b[0x3c], b[0x4e]),
        };

        let mut event = Self::zeroed();
        event.put_u32(0x00, 4);
        event.put_u64(0x08, before.storage_disc);
        event.put_u64(0x10, before.storage_payload);
        event.0[0x18..0x38].copy_from_slice(&b[0x00..0x20]);
        event.0[0x38..0x40].copy_from_slice(&b[0x20..0x28]);
        event.put_u32(0x40, u32_at(&b, 0x28));
        event.put_u32(0x44, u32_at(&b, 0x2c));
        event.put_u32(0x48, field_48);
        event.0[0x4c] = byte_4c;
        event.put_u32(0x50, u32_at(&b, 0x40));
        event.put_u32(0x54, u32_at(&b, 0x44));
        event.0[0x58] = b[0x4b];
        event.0[0x59] = tag_59;
        event.0[0x5c] = b[0x48];
        event.0[0x5d] = b[0x49];
        event.0[0x5e] = ty;
        event.put_u32(0x60, key.0);
        event.0[0x7c] = 1;
        event.0[0x81] = ty;
        event
    }
}

/// `send_state_diff_events<closure_env>`（0x1417bdae0）。
///
/// 由 `DecoratorStorage::restore_decorators` 的闭包（0x14155d13f）调用：
/// 遍历 `HashMap<DecoratorId, DecoratorBeforeAfterWithStorage>`，逐条按
/// before/after 的存储判别分派并 `send` 0x88B 事件：
/// - before 判别 4 → 立即返回（迭代终止标记）；
/// - before 3 且 after 3 → `unreachable!()`（decorator_edit.rs，0x142ce1364）；
/// - before 3、after≠3 → 新增事件（种类 3）；
/// - after 3、before≠3 → 移除事件（种类 4，按 type 分派）；
/// - 否则 → `convert_before_after_states_to_changes` + 变更事件（种类 = before 判别）。
pub fn send_state_diff_events(
    diffs: &HashMap<DecoratorId, DecoratorBeforeAfterWithStorage>,
    events: &DeferredDecoratorOpEvents,
) {
    for (key, value) in diffs {
        let before = &value.before;
        let after = &value.after;
        if before.storage_disc == 4 {
            return;
        }
        if before.storage_disc == 3 {
            if after.storage_disc == 3 {
                unreachable!("decorator diff: both sides have no storage");
            }
            events.send(DecoratorDiffEvent::from_added(*key, after).0);
            continue;
        }
        if after.storage_disc == 3 {
            events.send(DecoratorDiffEvent::from_removed_with_after(*key, before, &after.state).0);
            continue;
        }
        let changes = convert_before_after_states_to_changes(before, after);
        events.send(DecoratorDiffEvent::from_changes(*key, before, &changes).0);
    }
}

impl<S>
    crate::resources::history::transactions::HistoryTransactions<
        crate::resources::history::EditType,
        S,
    >
{
    /// `History::add_decorator_diff`（0x140a854a0）。
    ///
    /// 原生 4 参：`History`（RCX）、`&DecoratorHistoryDiff`（RDX，0x30B map）、
    /// `&StairsHistoryDiff`（R8，0x60B）、`&mut HashMap<StairGraphEdge, ()>`
    /// （R9）。流程：
    /// 1. pending 未开始 → panic "Tried writing decorator history..."；
    /// 2. `cursor - committed < -2` → `nuke_future_states` 并重建空 MultiEdit；
    /// 3. `VERIFY_STAIR_CHECKSUMS`（原生全局字节 0x14362c011）且 stairs diff
    ///    校验和与最近 Stairs 条目记录不符 → log + panic
    ///    "Stairs graph history recording..."；
    /// 4. 第一边对循环（原生 Vec@+0x40）：边在集合中 → push 0xE/变体 0
    ///    （SupportsDelete）；第二循环（Vec@+0x28）：从集合移除成功 → push
    ///    0xE/变体 1（SupportsRecreate）；
    /// 5. 克隆替换 `History +0x48` Arc<DecoratorHistoryDiff> 与
    ///    `History +0x50` Arc<StairsHistoryDiff>。
    ///
    /// 登记 U（跨域）：`VERIFY_STAIR_CHECKSUMS` 原生为 stairs 域可变全局；
    /// 本函数以参数注入。原生 StairsHistoryDiff 为 0x60B（两 Vec @+0x28/+0x40
    /// + 判别 @+0x00 + checksum @+0x08），当前 stairs 域类型为近似结构，
    /// Vec 字段与判别/校验和的精确对应由 stairs 域后续对齐。
    pub fn add_decorator_diff(
        &mut self,
        decorator_diff: &DecoratorHistoryDiffMap,
        stairs_diff: &crate::resources::stairs::stairs_state::StairsHistoryDiff,
        stairs_edges: &mut HashSet<crate::resources::stairs::stairs_state::StairGraphEdge>,
        verify_stair_checksums: bool,
    ) {
        use crate::resources::history::stairs_history_edit::StairsHistoryEdit;
        use crate::resources::stairs::stairs_state::StairGraphEdge;

        // 1. pending 未开始 → panic（原生 panic_fmt @0x142ad83c0）。
        if self.pending().is_none() {
            panic!("Tried writing decorator history while not recording");
        }

        // 2. cursor - committed < -2 → nuke_future_states + 重建空 pending。
        let committed = self.ledger().entries.len() as i32;
        if self.ledger().cursor.wrapping_sub(committed) < -2 {
            self.nuke_future_states_with(&mut || fastrand::u64(..))
                .expect("nuke_future_states commits the new nonce");
            self.set_pending(Some(
                crate::resources::history::transactions::HistoryEntry::from_edits(Vec::new()),
            ));
        }

        // 3. checksum 校验（原生：倒序找最近 tag==1 记录，比较 record+0x28）。
        if verify_stair_checksums {
            let stored =
                self.ledger()
                    .entries
                    .iter()
                    .rev()
                    .find(|entry| {
                        entry.edits.iter().any(|edit| {
                            matches!(edit, crate::resources::history::EditType::Stairs(_))
                        })
                    })
                    .and_then(|entry| entry.observed_stairs_checksum);
            if let Some(stored) = stored {
                if stored != stairs_diff.after_checksum {
                    log::error!("Stairs diff {:?}", stairs_diff);
                    panic!("Stairs graph history recording checksum mismatch");
                }
            }
        }

        // 4. 两 u32 边对循环 → tag 0xE 记录。
        // 原生第一循环（Vec@+0x40）只 probe，命中 → SupportsDelete（变体 0）。
        // 原生第二循环（Vec@+0x28）remove 命中 → SupportsRecreate（变体 1）。
        // Vec 与当前 stairs 类型字段的对应登记 U（见函数文档）。
        let mut records: Vec<crate::resources::history::EditType> = Vec::new();
        for &(a, b) in &stairs_diff.added {
            let edge = StairGraphEdge::new(a, b);
            if stairs_edges.contains(&edge) {
                records.push(crate::resources::history::EditType::Stairs(
                    std::sync::Arc::new(StairsHistoryEdit::SupportsDelete(edge)),
                ));
            }
        }
        for &(a, b) in &stairs_diff.removed {
            let edge = StairGraphEdge::new(a, b);
            if stairs_edges.remove(&edge) {
                records.push(crate::resources::history::EditType::Stairs(
                    std::sync::Arc::new(StairsHistoryEdit::SupportsRecreate(edge)),
                ));
            }
        }
        if let Some(pending) = self.pending_mut() {
            pending.edits.extend(records);
            // 原生记录 checksum 位于记录 +0x28；建模位置见 HistoryEntry 字段。
            pending.observed_stairs_checksum = Some(stairs_diff.after_checksum);
        }

        // 5. Arc 替换（原生先 decorator +0x48，再 stairs +0x50）。
        let old = self
            .decorator_diff
            .replace(std::sync::Arc::new(decorator_diff.clone()));
        drop(old);
        let old = self
            .stairs_diff
            .replace(std::sync::Arc::new(stairs_diff.clone()));
        drop(old);
        let auxiliary = crate::resources::history::transactions::HistoryEntryAux {
            decorator_diff: self.decorator_diff.clone(),
            stairs_diff: self.stairs_diff.clone(),
        };
        // 建模：原生 MultiEdit 的空判定读 3 个 aux owner 计数；装饰/楼梯 diff
        // 非空即让 pending 非空（否则 decorator-only diff 不会提交条目）。
        if let Some(pending) = self.pending_mut() {
            pending.auxiliary = auxiliary;
            pending.observed_auxiliary_counts[0] = usize::from(!decorator_diff.is_empty());
            pending.observed_auxiliary_counts[1] = usize::from(!stairs_diff.is_empty());
        }
    }
}

/// The tag10 edit carries the native 32-byte operation record. Replay uses
/// its entry-owned storage diff; serialization intentionally writes only the
/// operation marker (the original saved Place/Change/Remove payload is {}).
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct DecoratorV5Edit {
    pub kind: u64,
    pub fields: [u64; 3],
}
impl Default for DecoratorV5Edit {
    fn default() -> Self {
        Self {
            kind: 3,
            fields: [0; 3],
        }
    }
}
impl serde::Serialize for DecoratorV5Edit {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        let name = match self.kind {
            3 => "Change",
            4 => "Place",
            5 => "Remove",
            other => {
                return Err(serde::ser::Error::custom(format!(
                    "unrestored decorator operation {other}"
                )));
            }
        };
        use serde::ser::SerializeStructVariant;
        serializer
            .serialize_struct_variant("DecoratorV5", self.kind as u32, name, 0)?
            .end()
    }
}
impl<'de> serde::Deserialize<'de> for DecoratorV5Edit {
    fn deserialize<D: Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        #[derive(serde::Deserialize)]
        enum Marker {
            Place {},
            Change {},
            Remove {},
        }
        let kind = match Marker::deserialize(deserializer)? {
            Marker::Change {} => 3,
            Marker::Place {} => 4,
            Marker::Remove {} => 5,
        };
        Ok(Self {
            kind,
            fields: [0; 3],
        })
    }
}

/// Original vectorize codec: address enum + named before/after state maps.
/// State conversion is shared with save snapshots, preserving the same variant
/// defaults and style-specific fields instead of serializing native byte slots.
impl serde::Serialize for DecoratorHistoryEdit {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        use crate::systems::save::snapshot::{
            decorator_history_state_json, decorator_storage_addr_json,
        };
        let encode =
            |state: &DecoratorStorageAddrHistoryState| -> Result<serde_json::Value, S::Error> {
                let mut out = serde_json::Map::new();
                for (id, value) in &state.states {
                    out.insert(
                        id.0.to_string(),
                        decorator_history_state_json(value).map_err(serde::ser::Error::custom)?,
                    );
                }
                Ok(serde_json::Value::Object(out))
            };
        let mut diffs = Vec::with_capacity(self.diffs.len());
        for (addr, diff) in &self.diffs {
            diffs.push(serde_json::json!([decorator_storage_addr_json(*addr), {
                "before": encode(&diff.before)?, "after": encode(&diff.after)?
            }]));
        }
        let mut map = serializer.serialize_map(Some(1))?;
        map.serialize_entry("diffs", &diffs)?;
        map.end()
    }
}
impl<'de> serde::Deserialize<'de> for DecoratorHistoryEdit {
    fn deserialize<D: Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use crate::systems::save::snapshot::{DecoratorAddrJson, DecoratorInfoJson};
        #[derive(serde::Deserialize)]
        struct Diff {
            before: HashMap<u32, DecoratorInfoJson>,
            after: HashMap<u32, DecoratorInfoJson>,
        }
        #[derive(serde::Deserialize)]
        struct Input {
            diffs: Vec<(DecoratorAddrJson, Diff)>,
        }
        let decode = |input: HashMap<u32, DecoratorInfoJson>| -> Result<DecoratorStorageAddrHistoryState, D::Error> {
            let mut out = DecoratorStorageAddrHistoryState::default();
            for (id, state) in input {
                out.states.insert(DecoratorId(id), state.to_history_state().map_err(serde::de::Error::custom)?);
            }
            Ok(out)
        };
        let mut diffs = DecoratorHistoryDiffMap::new();
        for (addr, diff) in Input::deserialize(deserializer)?.diffs {
            diffs.insert(
                addr.to_storage_addr(),
                DecoratorStorageHistoryDiff {
                    before: decode(diff.before)?,
                    after: decode(diff.after)?,
                },
            );
        }
        Ok(Self { diffs })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn info(style: u8, state: u8) -> DecoratorInfo {
        DecoratorInfo {
            dst_wall: 1,
            dst_secondary: 2,
            dst_payload: 3,
            u18: 4,
            u20: 5,
            u24: 6,
            u28: 7,
            decorator_id: 42,
            u30: 0x1234_5678_9abc_def0,
            u38: 0x11223344,
            u3c: 0x55667788,
            u40: ((style as u32) << 8) | 0x99,
            rank: 3,
            u45: 4,
            ty: 5,
            state,
        }
    }

    #[test]
    fn from_info_copies_plain_fields_and_derives_per_style() {
        // style 7（Lantern）：u38 = u30 low。
        let s = DecoratorHistoryState::from(&info(7, 0));
        assert_eq!(s.dst_wall, 1);
        assert_eq!(s.dst_secondary, 2);
        assert_eq!(s.dst_payload, 3);
        assert_eq!(s.u18, 4);
        assert_eq!(s.u20, 5);
        assert_eq!(s.u24, 6);
        // +0x28 为 u64 = (decorator_id<<32)|u28（原生 in_RDX[5] 直拷）。
        assert_eq!(s.u28, (42u64 << 32) | 7);
        assert_eq!(s.rank, 3);
        assert_eq!(s.u49, 4);
        assert_eq!(s.ty, 5);
        // style==7：u38 = u30 low（Lantern f32）。
        assert_eq!(s.u38, 0x9abc_def0);
        assert_eq!(f32::from_bits(s.u38), f32::from_bits(0x9abc_def0));
        // u34（Window 面）= 0，u4c（Flag 面）= 0，u4d（Chimney 面）= 1。
        assert_eq!(s.u34, 0);
        assert_eq!(s.u4c, 0);
        assert_eq!(s.u4d, 1);
        // style 2..7：u3c/u40/u44=0、u4b=0、style@+0x4e=0。
        assert_eq!(s.u3c, f32::to_bits(1.0));
        assert_eq!(s.u40, 0);
        assert_eq!(s.u44, 0);
        assert_eq!(s.u4b, 0);
        assert_eq!(s.style, 0);
        assert!(s.is_default_stair_width()); // +0x3c = 1.0
    }

    #[test]
    fn from_info_style_6_window_and_style_5_flag() {
        let w = DecoratorHistoryState::from(&info(6, 0));
        assert_eq!(w.u34, 0x9abc_def0); // Window u32
        assert_eq!(w.u4c, 0);
        let f = DecoratorHistoryState::from(&info(5, 0));
        assert_eq!(f.u4c, 0xf0); // Flag u8 = u30 byte0
        assert_eq!(f.u34, 0);
        let c = DecoratorHistoryState::from(&info(4, 0));
        assert_eq!(c.u4d, 0x78); // Chimney u8 = u30 byte4（0x1234_5678_9abc_def0 的 byte4）
        assert_eq!(c.u4c, 0);
    }

    #[test]
    #[should_panic(expected = "Attempting to capture a DecoratorHistoryState of a preview")]
    fn from_info_preview_panics() {
        let _ = DecoratorHistoryState::from(&info(7, 1));
    }

    #[test]
    fn lantern_brightness_default_is_1() {
        // 非 Lantern（style != 7）：u38 = 1.0。
        let s = DecoratorHistoryState::from(&info(6, 0));
        assert!(s.is_default_lantern_brightness());
    }

    #[test]
    fn to_info_roundtrips_per_style_fields() {
        fn valid_info(style: u8) -> DecoratorInfo {
            let mut src = info(style, 0);
            src.ty = match style {
                4 => 0x3b,
                5 => 0x3d,
                6 => 0x38,
                7 => 0x3c,
                _ => 0x3f,
            };
            src
        }
        // 各 style：from → to_info 收敛回 DecoratorInfo 的直拷簇 + style 字节。
        // 派生簇为**有损**（原生 state 只保留 Chimney/Flag/Window/Lantern 子面，
        // 未保留 u30 高字节）——只对非有损面断言全量回填。
        for style in [4u8, 5, 6, 7, 8, 0] {
            let src = valid_info(style);
            let state = DecoratorHistoryState::from(&src);
            let back = state.to_info();
            // 直拷簇完整回填。
            assert_eq!(back.dst_wall, src.dst_wall, "style {style}");
            assert_eq!(back.dst_secondary, src.dst_secondary, "style {style}");
            assert_eq!(back.dst_payload, src.dst_payload, "style {style}");
            assert_eq!(back.u18, src.u18, "style {style}");
            assert_eq!(back.u20, src.u20, "style {style}");
            assert_eq!(back.u24, src.u24, "style {style}");
            assert_eq!(back.u28, src.u28, "style {style}");
            assert_eq!(back.decorator_id, src.decorator_id, "style {style}");
            assert_eq!(back.rank, src.rank, "style {style}");
            assert_eq!(back.u45, src.u45, "style {style}");
            assert_eq!(back.ty, src.ty, "style {style}");
            // style 字节 = 原 u40 byte1。
            assert_eq!(back.u40 >> 8, (src.u40 >> 8) & 0xff, "style {style}");
        }
        // 非有损默认簇（style 8+/<2）：u30/u38/u3c 全量回填。
        for style in [8u8, 0] {
            let src = valid_info(style);
            let back = DecoratorHistoryState::from(&src).to_info();
            assert_eq!(back.u30, src.u30, "style {style}");
            assert_eq!(back.u38, src.u38, "style {style}");
            assert_eq!(back.u3c, src.u3c, "style {style}");
        }
        // 有损子面（style 4/5/6/7）：捕获子字节回填到 u30 对应字节。
        let c = DecoratorHistoryState::from(&valid_info(4)).to_info();
        assert_eq!(c.u30 >> 32, 0x78); // Chimney u8 @ u30 byte4
        let f = DecoratorHistoryState::from(&valid_info(5)).to_info();
        assert_eq!(f.u30 & 0xff, 0xf0); // Flag u8 @ u30 byte0
        let w = DecoratorHistoryState::from(&valid_info(6)).to_info();
        assert_eq!(w.u30, 0x9abc_def0); // Window u32
        let l = DecoratorHistoryState::from(&valid_info(7)).to_info();
        assert_eq!(l.u30, 0x9abc_def0); // Lantern f32
    }

    #[test]
    fn to_info_type_selects_default_extras_and_live_state() {
        // 0x3e/未知与 Clutter（<0x38）在原生 `ty-0x38` 夹取后走 index 6 分支：
        // style 回读 state.style（默认 0），不得默认写 3。
        for (ty, expected_style) in [
            (0x38, 6),
            (0x39, 6),
            (0x3a, 6),
            (0x3b, 4),
            (0x3c, 7),
            (0x3d, 5),
            (0x3e, 0),
            (0x10, 0),
        ] {
            let state = DecoratorHistoryState {
                ty,
                u38: 1.0_f32.to_bits(),
                u3c: 1.0_f32.to_bits(),
                u4d: 1,
                ..DecoratorHistoryState::default()
            };
            let actual = state.to_info();
            assert_eq!(actual.extra_style(), expected_style, "type {ty:#x}");
            assert_eq!(actual.state, 0);
            assert_eq!(DecoratorHistoryState::from_info(&actual).ty, ty);
        }
    }

    // ---- diff 事件链（0x140b3fab0 / 0x1417bdae0 / 0x140a854a0）----

    fn state_with(dst_tag: u8, u30: f32) -> DecoratorHistoryState {
        DecoratorHistoryState {
            dst_wall: 0x1122_3344_5566_7788,
            dst_secondary: 7,
            dst_payload: 9,
            u18: 11,
            u20: (u32::from(dst_tag) << 16) | 0x0042,
            u24: 13,
            u28: 0x00aa_00bb,
            u30: u30.to_bits(),
            u34: 0,
            u38: 0,
            u3c: f32::to_bits(1.5),
            u40: 0,
            u44: 0,
            rank: 3,
            u49: 4,
            ty: 0x3b,
            u4b: 5,
            u4c: 6,
            u4d: 7,
            style: 8,
        }
    }

    fn slot(disc: u64, payload: u64, state: DecoratorHistoryState) -> DecoratorStateWithStorage {
        DecoratorStateWithStorage {
            storage_disc: disc,
            storage_payload: payload,
            state,
        }
    }

    #[test]
    fn state_and_slot_native_sizes() {
        assert_eq!(std::mem::size_of::<DecoratorHistoryState>(), 0x50);
        assert_eq!(std::mem::size_of::<DecoratorStateWithStorage>(), 0x60);
        assert_eq!(std::mem::size_of::<DecoratorBeforeAfterWithStorage>(), 0xc0);
        assert_eq!(DecoratorDiffEvent::SIZE, 0x88);
    }

    #[test]
    fn convert_marks_equal_state_as_3_and_type_marker_40() {
        let s = state_with(2, 2.5);
        let before = slot(0, 42, s);
        let after = slot(0, 42, s);
        let changes = convert_before_after_states_to_changes(&before, &after);
        assert_eq!(changes.storage_disc, 3);
        assert_eq!(changes.type_marker, 0x40);
        assert_eq!(changes.rank_changed, 0);
        assert_eq!(changes.id_changed, 0);
        // 相等时 +0x54 写 2。
        assert_eq!(changes.u4c_or_2, 2);
        let bytes = changes.to_bytes();
        assert_eq!(u64_at(&bytes, 0x00), 3);
        assert_eq!(u64_at(&bytes, 0x08), 42);
        assert_eq!(u32_at(&bytes, 0x40), 0);
        assert_eq!(u32_at(&bytes, 0x48), 0);
        assert_eq!(bytes[0x50], 0);
        assert_eq!(bytes[0x51], 3);
        assert_eq!(bytes[0x55], 0x40);
    }

    #[test]
    fn convert_storage_payload_compared_only_for_wall_disc() {
        let s = state_with(2, 1.0);
        // disc 1（Terrain）不比较 payload。
        let a = convert_before_after_states_to_changes(&slot(1, 1, s), &slot(1, 2, s));
        assert_eq!(a.storage_disc, 3);
        // disc 0（Wall）比较 payload。
        let b = convert_before_after_states_to_changes(&slot(0, 1, s), &slot(0, 2, s));
        assert_ne!(b.storage_disc, 3);
        assert_eq!(b.storage_disc, 0);
        assert_eq!(b.storage_payload, 2);
    }

    #[test]
    fn convert_nan_is_a_change_and_uses_after_value() {
        let a = slot(0, 1, state_with(2, f32::NAN));
        let b = slot(0, 1, state_with(2, f32::NAN));
        let changes = convert_before_after_states_to_changes(&a, &b);
        // u30 不在 tag 2 的相等谓词内 → 状态仍判“未变更”。
        assert_eq!(changes.storage_disc, 3);
        // CMPNEQSS：无序 → 变更标志 1，载荷为 after 的 NaN 位。
        assert_eq!(changes.u30_changed, 1);
        assert!(changes.u30.is_nan());

        // 比较字段内的 NaN：位型不同 → 不相等（UCOMISS 无序）。
        let mut n0 = state_with(2, 1.0);
        n0.dst_wall = 0x7fc0_0001_7fc0_0000;
        let mut n1 = n0;
        n1.dst_wall = 0x7fc0_0002_7fc0_0000;
        let changed = convert_before_after_states_to_changes(&slot(0, 0, n0), &slot(0, 0, n1));
        assert_ne!(changed.storage_disc, 3);
    }

    #[test]
    fn convert_variant_tag4_compares_wall_words_as_integers() {
        // tag 4 的 +0x10/+0x14 用整数 dword 比较：NaN 位型不同也视为变更。
        let mut s0 = state_with(4, 1.0);
        s0.dst_wall = 0x7fc0_0000_7fc0_0000; // 两个 NaN 位型
        let mut s1 = s0;
        s1.dst_wall = 0x7fc0_0001_7fc0_0000;
        let a = convert_before_after_states_to_changes(&slot(0, 0, s0), &slot(0, 0, s1));
        assert_ne!(a.storage_disc, 3);
        // 对照 tag 2/3 分支（f32 比较）下同样的 NaN 位型 == 自身。
        let a0 = convert_before_after_states_to_changes(&slot(0, 0, s0), &slot(0, 0, s0));
        assert_eq!(a0.storage_disc, 3);
    }

    #[test]
    fn convert_reports_rank_id_and_type_changes() {
        let mut before = state_with(2, 1.0);
        let mut after = before;
        before.rank = 1;
        after.rank = 2;
        after.u28 = 0x00aa_00bb;
        // decorator_id 在状态 +0x2c（`impl$1::from` 写 u28 高 32 位）。
        before.u28 = 0x0000_0000_0000_00bb;
        after.u28 = 0x0000_0005_0000_00bb;
        after.ty = 0x40; // clamp 6，与 before 0x3b→3 不同 → marker=after ty
        let changes =
            convert_before_after_states_to_changes(&slot(0, 0, before), &slot(0, 0, after));
        assert_eq!(changes.rank_changed, 1);
        assert_eq!(changes.rank_value, 2);
        assert_eq!(changes.id_changed, 1);
        assert_eq!(changes.id_delta, 5);
        assert_eq!(changes.type_marker, 0x40);
    }

    #[test]
    fn changed_event_layout_matches_fresh_offsets() {
        let key = DecoratorId(77);
        let before = slot(1, 0, state_with(2, 2.0));
        let changes = convert_before_after_states_to_changes(&before, &before);
        let event = DecoratorDiffEvent::from_changes(key, &before, &changes);
        assert_eq!(u64_at(&event.0, 0x00), 1);
        assert_eq!(u64_at(&event.0, 0x08), 0);
        // +0x10 起是 changes。
        assert_eq!(&event.0[0x10..0x10 + 0x58], &changes.to_bytes()[..]);
        assert_eq!(event.decorator_id(), 77);
        assert_eq!(event.0[0x7c], 1);
        assert_eq!(event.decorator_ty(), before.state.ty);
    }

    #[test]
    fn added_event_uses_kind_3_and_after_state_prefix() {
        let after = slot(2, 0, state_with(3, 1.0));
        let bytes = after.state.to_bytes();
        let event = DecoratorDiffEvent::from_added(DecoratorId(9), &after);
        assert_eq!(event.kind(), 3);
        assert_eq!(u64_at(&event.0, 0x08), 2);
        assert_eq!(&event.0[0x18..0x38], &bytes[0x00..0x20]);
        assert_eq!(&event.0[0x38..0x40], &bytes[0x20..0x28]);
        assert_eq!(event.0[0x44], 1);
        assert_eq!(event.0[0x45], bytes[0x48]);
        assert_eq!(event.decorator_ty(), bytes[0x4a]);
    }

    #[test]
    fn removed_event_dispatch_cases_capture_style_cluster_fields() {
        let mut before = state_with(0x3b, 1.0); // case 3
        before.u4d = 0x5a;
        let after_state = state_with(0x3b, 1.0);
        let event = DecoratorDiffEvent::from_removed_with_after(
            DecoratorId(1),
            &slot(1, 0, before),
            &after_state,
        );
        assert_eq!(event.kind(), 4);
        assert_eq!(event.0[0x48..0x4c], before.u30.to_le_bytes());
        assert_eq!(event.0[0x4c], 0x5a); // case 3 → before.u4d
        assert_eq!(event.0[0x59], 4);
        assert_eq!(event.0[0x5e], 0x3b);

        let mut case7 = state_with(0x3f, 3.0);
        case7.ty = 0x3f;
        case7.u3c = f32::to_bits(2.0);
        case7.style = 11;
        let event7 = DecoratorDiffEvent::from_removed_with_after(
            DecoratorId(2),
            &slot(1, 0, case7),
            &after_state,
        );
        assert_eq!(event7.0[0x48..0x4c], case7.u30.to_le_bytes());
        assert_eq!(event7.0[0x4c], (case7.u3c & 0xff) as u8);
        assert_eq!(event7.0[0x59], 11);
    }

    #[test]
    fn send_state_diff_events_dispatches_all_three_kinds() {
        use crate::resources::walls::decorator::DeferredDecoratorOpEvents;
        let mut map = HashMap::new();
        map.insert(
            DecoratorId(1),
            DecoratorBeforeAfterWithStorage {
                before: slot(0, 10, state_with(2, 1.0)),
                after: slot(0, 10, state_with(2, 2.0)),
            },
        );
        map.insert(
            DecoratorId(2),
            DecoratorBeforeAfterWithStorage {
                before: slot(3, 0, state_with(2, 1.0)),
                after: slot(1, 0, state_with(2, 1.0)),
            },
        );
        map.insert(
            DecoratorId(3),
            DecoratorBeforeAfterWithStorage {
                before: slot(2, 0, state_with(0x3b, 1.0)),
                after: slot(3, 0, state_with(0x3b, 1.0)),
            },
        );
        let events = DeferredDecoratorOpEvents::default();
        send_state_diff_events(&map, &events);
        let mut out = Vec::new();
        events.unleash(&mut out);
        assert_eq!(out.len(), 3);
        let kinds: Vec<u32> = out
            .iter()
            .map(|e| u32::from_le_bytes(e[0..4].try_into().unwrap()))
            .collect();
        assert!(kinds.contains(&0)); // changed（before disc 0）
        assert!(kinds.contains(&3)); // added
        assert!(kinds.contains(&4)); // removed
        let ids: Vec<u32> = out
            .iter()
            .map(|e| {
                let mut record = [0u8; 0x88];
                record.copy_from_slice(&e[..0x88]);
                DecoratorDiffEvent(record).decorator_id()
            })
            .collect();
        assert!(ids.contains(&1) && ids.contains(&2) && ids.contains(&3));
    }

    #[test]
    fn add_decorator_diff_pushes_tag_e_records_and_replaces_arcs() {
        use crate::resources::history::transactions::ObservedHistoryLedger;
        use crate::resources::history::{EditType, History};
        use crate::resources::stairs::stairs_state::{StairGraphEdge, StairsHistoryDiff};
        let ledger = ObservedHistoryLedger::<EditType, ()> {
            entries: Vec::new(),
            cursor: -1,
            undo_anchor: -1,
            resolved_snapshots: Vec::new(),
            observed_count_58: 0,
            mutation_nonce: 0,
        };
        let mut history = History::from_observed(ledger);
        history.start_input_recording(0).unwrap();
        let stairs = StairsHistoryDiff {
            before_checksum: 1,
            after_checksum: 2,
            removed: vec![(9, 8)],
            added: vec![(7, 3)],
        };
        let mut edges = HashSet::new();
        edges.insert(StairGraphEdge::new(7, 3));
        edges.insert(StairGraphEdge::new(8, 9));
        let diff = DecoratorHistoryDiffMap::default();
        history.add_decorator_diff(&diff, &stairs, &mut edges, false);
        let pending = history.pending().expect("recording stays open");
        assert_eq!(pending.edits.len(), 2);
        assert!(matches!(
            &pending.edits[0],
            EditType::Stairs(edit) if matches!(&**edit, crate::resources::history::stairs_history_edit::StairsHistoryEdit::SupportsDelete(edge) if edge.nodes() == (3, 7))
        ));
        assert!(matches!(
            &pending.edits[1],
            EditType::Stairs(edit) if matches!(&**edit, crate::resources::history::stairs_history_edit::StairsHistoryEdit::SupportsRecreate(edge) if edge.nodes() == (8, 9))
        ));
        assert!(history.decorator_diff.is_some());
        assert_eq!(history.stairs_diff.as_deref(), Some(&stairs));
    }

    #[test]
    #[should_panic(expected = "checksum mismatch")]
    fn add_decorator_diff_panics_on_checksum_mismatch() {
        use crate::resources::history::transactions::HistoryEntry;
        use crate::resources::history::transactions::ObservedHistoryLedger;
        use crate::resources::history::{EditType, History, stairs_history_edit};
        use crate::resources::stairs::stairs_state::{StairGraphEdge, StairsHistoryDiff};
        let entry = HistoryEntry {
            auxiliary: Default::default(),
            edits: vec![EditType::Stairs(std::sync::Arc::new(
                stairs_history_edit::StairsHistoryEdit::SupportsDelete(StairGraphEdge::new(1, 2)),
            ))],
            observed_auxiliary_counts: [0; 3],
            observed_stairs_checksum: Some(100),
        };
        let ledger = ObservedHistoryLedger::<EditType, ()> {
            entries: vec![entry],
            cursor: 0,
            undo_anchor: -1,
            resolved_snapshots: Vec::new(),
            observed_count_58: 1,
            mutation_nonce: 0,
        };
        let mut history = History::from_observed(ledger);
        history.start_input_recording(0).unwrap();
        let stairs = StairsHistoryDiff {
            before_checksum: 1,
            after_checksum: 999,
            removed: Vec::new(),
            added: Vec::new(),
        };
        let mut edges = HashSet::new();
        history.add_decorator_diff(
            &DecoratorHistoryDiffMap::default(),
            &stairs,
            &mut edges,
            true,
        );
    }

    #[test]
    #[should_panic(expected = "Tried writing decorator history")]
    fn add_decorator_diff_requires_recording() {
        use crate::resources::history::transactions::ObservedHistoryLedger;
        use crate::resources::history::{EditType, History};
        use crate::resources::stairs::stairs_state::StairsHistoryDiff;
        let ledger = ObservedHistoryLedger::<EditType, ()> {
            entries: Vec::new(),
            cursor: -1,
            undo_anchor: -1,
            resolved_snapshots: Vec::new(),
            observed_count_58: 0,
            mutation_nonce: 0,
        };
        let mut history = History::from_observed(ledger);
        let stairs = StairsHistoryDiff {
            before_checksum: 0,
            after_checksum: 0,
            removed: Vec::new(),
            added: Vec::new(),
        };
        history.add_decorator_diff(
            &DecoratorHistoryDiffMap::default(),
            &stairs,
            &mut HashSet::new(),
            false,
        );
    }

    #[test]
    fn clutter_state_round_trips_extension_slots_through_to_info() {
        // Clutter（ty<0x38）：from_info 写入的扩展簇槽必须被 to_info 保留，
        // style 回读 0（round9 6/134 样本丢失面）。
        let info = DecoratorInfo {
            decorator_id: 0x21,
            ty: 0x10,
            u30: 0x1122_3344_5566_7788,
            u38: 0x99aa_bbcc,
            u3c: 0x0d0e_0f10,
            u40: 0x11,
            rank: 3,
            u45: 5,
            ..DecoratorInfo::default()
        };
        let state = DecoratorHistoryState::from_info(&info);
        assert_eq!(state.style, 0);
        let back = state.to_info();
        assert_eq!(back.u30, info.u30);
        assert_eq!(back.u38, info.u38);
        assert_eq!(back.u3c, info.u3c);
        assert_eq!(back.u40 & 0xff, info.u40 & 0xff);
        assert_eq!(back.u40 >> 8 & 0xff, 0);
        assert_eq!(back.ty, info.ty);
    }
}
