//! `save_file.rs` — 存档文件的已恢复纯逻辑：节表、影子文件脏块位图、追加编辑、读档升级与版本开关。
//!
//! 证据（RNE 机器码 + `session-save/decomp/`）：
//! - 节表：键长 @0x142ace328、键指针 @0x142ace378、填充 @0x142ace3c8，逐项实读（O）。第 9 项键是
//!   `"snapshots"`（9 字节），此前文档写成 `"snapshot"`。
//! - `mark_dirty` 0x14091a450：4096 B 块、`bitvec` 区间置位；位图不足先 `resize(end_block, false)`。
//! - `ensure_current_snapshot`（`add_history_edit_to_save_file`）0x140a439b0：**仅当 `edit_offsets` 推入前非空**才写
//!   `",\n"`（此前文档漏了这一条件），随后追加序列化后的编辑并标脏。
//! - 读档升级（`load_game` 0x14146d030 的两处独立块，此前文档混为一段）：
//!   位 48：子类型字节 == 0x3a 的 0x58 步长记录，颜色字段写 999（"window color IDs"）；
//!   位 40：颜色映射 `{0,3,6,7 不变; 1→2; 2→5; 4→8; 5→3; 8→4; 999 不变; 其余→1}`，作用于 16 B 条目的 `+8` 字段与
//!   子类型 0x38..=0x3a 的 0x58 步长记录（"arrow slits color IDs"）。
//! - 版本开关：`0x14362c308 = (v >= 0x1a)`、`0x14362c011 = (v > 0x1a)`（有符号比较，0x14146dc82..0x14146dc95）。
//!
//! 未恢复：备份节拍器（`save_game` 内联的 counter/除数循环，反编译不可靠）、`CurrentSaveFile` 的 0/1/2 三种形态、
//! 存档线程与快照 GC 的 I/O、`AsyncSaveCmd` 0/1/2。
use bitvec::prelude::{BitVec, Lsb0};

/// 存档节（O：键与填充取自二进制；变体名为 I）。
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
#[repr(u8)]
pub enum SaveFileSection {
    Version = 0,
    Identity = 1,
    BackwardCompatibility = 2,
    Camera = 3,
    TimeOfDay = 4,
    Sheep = 5,
    Colors = 6,
    ChainIndex = 7,
    HistoryStart = 8,
    Snapshots = 9,
}

impl SaveFileSection {
    pub const ALL: [Self; 10] = [
        Self::Version,
        Self::Identity,
        Self::BackwardCompatibility,
        Self::Camera,
        Self::TimeOfDay,
        Self::Sheep,
        Self::Colors,
        Self::ChainIndex,
        Self::HistoryStart,
        Self::Snapshots,
    ];

    /// JSON 键（O：0x142ace378 指向的串）。
    pub fn json_key(self) -> &'static str {
        match self {
            Self::Version => "Version",
            Self::Identity => "Identity",
            Self::BackwardCompatibility => "BackwardCompatibility",
            Self::Camera => "Camera",
            Self::TimeOfDay => "Time of day",
            Self::Sheep => "Sheep",
            Self::Colors => "Colors",
            Self::ChainIndex => "chain_index",
            Self::HistoryStart => "history_start",
            Self::Snapshots => "snapshots",
        }
    }

    /// 节载荷的预留字节数（O：0x142ace3c8）。
    pub fn padding(self) -> usize {
        match self {
            Self::Version | Self::BackwardCompatibility => 0,
            Self::Identity => 16,
            Self::Camera | Self::Sheep => 128,
            Self::TimeOfDay => 32,
            Self::Colors => 64,
            Self::ChainIndex | Self::HistoryStart => 8,
            Self::Snapshots => 2048,
        }
    }
}

/// 脏块粒度（O：`>> 12`）。
pub const DIRTY_BLOCK_SIZE: u64 = 4096;

/// 影子文件的脏块位图（原版为 `bitvec::BitVec`，字段 `+0x60/+0x68`）。
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct DirtyBlocks {
    bits: BitVec<usize, Lsb0>,
}

impl DirtyBlocks {
    /// `mark_dirty(start, end)`（0x14091a450）：`end <= start` 直接返回；块区间为
    /// `[start >> 12, ceil(end / 4096))`，位图不足时先补 `false` 到 `end_block` 位。
    pub fn mark_dirty(&mut self, start: u64, end: u64) {
        if end <= start {
            return;
        }
        let start_block = (start >> 12) as usize;
        let end_block = ((end >> 12) + 1 - u64::from(end & 0xfff == 0)) as usize;
        if self.bits.len() < end_block {
            self.bits.resize(end_block, false);
        }
        self.bits[start_block..end_block].fill(true);
    }

    /// 当前位图长度（块数）。
    pub fn len(&self) -> usize {
        self.bits.len()
    }

    pub fn is_empty(&self) -> bool {
        self.bits.is_empty()
    }

    pub fn is_dirty(&self, block: usize) -> bool {
        self.bits.get(block).is_some_and(|bit| *bit)
    }

    /// 所有脏块下标，升序。
    pub fn dirty_blocks(&self) -> impl Iterator<Item = usize> + '_ {
        self.bits.iter_ones()
    }
}

/// 影子文件的内存缓冲：数据（`+0x28/+0x30`）、追加偏移（`+0x38`）、脏位图，加上存档的编辑偏移表。
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct ShadowBuffer {
    pub data: Vec<u8>,
    pub append_offset: u64,
    pub edit_offsets: Vec<u64>,
    pub dirty: DirtyBlocks,
}

impl ShadowBuffer {
    /// 写入 `bytes` 到 `offset`：缓冲短于 `offset` 时零填充，长度取 `max(len, end)`，并把追加偏移推到 `end`。
    /// 与 0x140a439b0 里的"扩容 → 零填充 → 写入 → 更新 `+0x30/+0x38`"一致。
    fn write_at(&mut self, offset: u64, bytes: &[u8]) {
        let start = offset as usize;
        let end = start + bytes.len();
        if self.data.len() < end {
            self.data.resize(end, 0);
        }
        self.data[start..end].copy_from_slice(bytes);
        self.append_offset = end as u64;
    }

    /// `ensure_current_snapshot`（0x140a439b0）。
    ///
    /// 1. 把当前追加偏移推入 `edit_offsets`；
    /// 2. 若推入前 `edit_offsets` 非空，写 `",\n"`（0x0a2c）并 `mark_dirty(offset, offset + 2)`；
    /// 3. 用 `serialize` 填充一个初始容量 0x80 的缓冲，失败则返回错误（偏移表与分隔符已生效，不回滚）；
    /// 4. 追加序列化结果并 `mark_dirty(旧偏移, 新偏移)`。
    pub fn append_edit<E>(
        &mut self,
        serialize: impl FnOnce(&mut Vec<u8>) -> Result<(), E>,
    ) -> Result<(), E> {
        let had_edits = !self.edit_offsets.is_empty();
        self.edit_offsets.push(self.append_offset);
        if had_edits {
            let at = self.append_offset;
            self.write_at(at, b",\n");
            self.dirty.mark_dirty(at, at + 2);
        }
        let mut json = Vec::with_capacity(0x80);
        serialize(&mut json)?;
        let start = self.append_offset;
        if !json.is_empty() {
            self.write_at(start, &json);
        }
        self.dirty.mark_dirty(start, self.append_offset);
        Ok(())
    }
}

/// 读档升级标志（O：`load_game` 里 `(flags >> 0x28) & 1`、`(flags >> 0x30) & 1`）。
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct LoadUpgradeFlags(pub u64);

impl LoadUpgradeFlags {
    /// 位 40："Detected old version of arrow slits color IDs. Upgrading..."
    pub fn upgrade_arrow_slit_color_ids(self) -> bool {
        self.0 >> 40 & 1 != 0
    }

    /// 位 48："Detected old version of window color IDs. Upgrading..."
    pub fn upgrade_window_color_ids(self) -> bool {
        self.0 >> 48 & 1 != 0
    }
}

/// 窗口颜色 ID 升级用的记录子类型（O：`+0x52` 字节 == `':'`）。
pub const WINDOW_RECORD_SUBTYPE: u8 = 0x3a;
/// 窗口升级写入的颜色值（O）。
pub const WINDOW_UPGRADED_COLOR: u32 = 999;

/// 位 48 的记录改写：子类型为 `0x3a` 时颜色改为 999，其余不动。
pub fn upgrade_window_record_color(subtype: u8, color: &mut u32) {
    if subtype == WINDOW_RECORD_SUBTYPE {
        *color = WINDOW_UPGRADED_COLOR;
    }
}

/// 位 40 的颜色映射（O：switch，`0,3,6,7` 与 `999` 原样，`1→2 2→5 4→8 5→3 8→4`，其余 → 1）。
pub fn upgrade_arrow_slit_color_id(id: u32) -> u32 {
    match id {
        0 | 3 | 6 | 7 | 999 => id,
        1 => 2,
        2 => 5,
        4 => 8,
        5 => 3,
        8 => 4,
        _ => 1,
    }
}

/// 位 40 作用到 0x58 步长记录的条件：`subtype - 0x38 < 3`。
pub fn arrow_slit_record_applies(subtype: u8) -> bool {
    subtype.wrapping_sub(0x38) < 3
}

/// 位 40 的整张颜色表升级（16 B 条目的 `+8` 字段）。
pub fn upgrade_arrow_slit_color_table(colors: &mut [u32]) {
    for color in colors {
        *color = upgrade_arrow_slit_color_id(*color);
    }
}

/// 存档版本 → 两个全局开关（O：0x14146dc82..0x14146dc95，有符号比较）。
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct LoadVersionGates {
    /// `0x14362c308`：`CENTERED_PATH`，`version >= 0x1a`。
    pub centered_path: bool,
    /// `0x14362c011`：楼梯 checksum 校验开关，`version > 0x1a`。
    pub stairs_checksum: bool,
}

impl LoadVersionGates {
    /// 把 `centered_path` 写进浮点格采样的全局开关（`load_game` 的 `setge byte [0x143638fac]`）。
    /// 目标由调用方给出（生产代码传 `utils::float_grid::CENTERED_PATH`），便于不污染全局地测试。
    pub fn apply_centered_path(self, target: &core::sync::atomic::AtomicBool) {
        target.store(self.centered_path, core::sync::atomic::Ordering::Relaxed);
    }

    pub fn from_version(version: i32) -> Self {
        Self {
            centered_path: version >= 0x1a,
            stairs_checksum: version > 0x1a,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn section_table_matches_the_binary() {
        let keys: Vec<_> = SaveFileSection::ALL.iter().map(|s| s.json_key()).collect();
        assert_eq!(
            keys,
            [
                "Version",
                "Identity",
                "BackwardCompatibility",
                "Camera",
                "Time of day",
                "Sheep",
                "Colors",
                "chain_index",
                "history_start",
                "snapshots"
            ]
        );
        let lens: Vec<_> = keys.iter().map(|k| k.len()).collect();
        assert_eq!(lens, [7, 8, 21, 6, 11, 5, 6, 11, 13, 9]);
        let padding: Vec<_> = SaveFileSection::ALL.iter().map(|s| s.padding()).collect();
        assert_eq!(padding, [0, 16, 0, 128, 32, 128, 64, 8, 8, 2048]);
        for (i, s) in SaveFileSection::ALL.iter().enumerate() {
            assert_eq!(*s as usize, i);
        }
    }

    #[test]
    fn mark_dirty_uses_4k_blocks_with_ceiling_end() {
        let mut d = DirtyBlocks::default();
        d.mark_dirty(0, 1);
        assert_eq!(d.dirty_blocks().collect::<Vec<_>>(), [0]);
        let mut d = DirtyBlocks::default();
        d.mark_dirty(4095, 4097);
        assert_eq!(d.dirty_blocks().collect::<Vec<_>>(), [0, 1]);
        let mut d = DirtyBlocks::default();
        d.mark_dirty(4096, 8192);
        assert_eq!(d.dirty_blocks().collect::<Vec<_>>(), [1]);
        assert_eq!(d.len(), 2);
    }

    #[test]
    fn mark_dirty_ignores_empty_ranges_and_never_shrinks() {
        let mut d = DirtyBlocks::default();
        d.mark_dirty(10, 10);
        d.mark_dirty(10, 5);
        assert!(d.is_empty());
        d.mark_dirty(0, 3 * 4096);
        d.mark_dirty(0, 1);
        assert_eq!(d.len(), 3);
        assert_eq!(d.dirty_blocks().count(), 3);
    }

    #[test]
    fn first_edit_has_no_separator_and_later_edits_do() {
        let mut buf = ShadowBuffer::default();
        buf.append_edit::<()>(|out| {
            out.extend_from_slice(b"{\"a\":1}");
            Ok(())
        })
        .unwrap();
        assert_eq!(buf.data, b"{\"a\":1}");
        assert_eq!(buf.edit_offsets, [0]);
        buf.append_edit::<()>(|out| {
            out.extend_from_slice(b"{\"b\":2}");
            Ok(())
        })
        .unwrap();
        assert_eq!(buf.data, b"{\"a\":1},\n{\"b\":2}");
        assert_eq!(buf.edit_offsets, [0, 7]);
        assert_eq!(buf.append_offset, 16);
        assert_eq!(buf.dirty.dirty_blocks().collect::<Vec<_>>(), [0]);
    }

    #[test]
    fn serialization_error_keeps_the_offset_and_separator() {
        let mut buf = ShadowBuffer::default();
        buf.append_edit::<()>(|out| {
            out.extend_from_slice(b"{}");
            Ok(())
        })
        .unwrap();
        let err = buf.append_edit(|_| Err("boom"));
        assert_eq!(err, Err("boom"));
        assert_eq!(buf.edit_offsets, [0, 2]);
        assert_eq!(buf.data, b"{},\n");
        assert_eq!(buf.append_offset, 4);
    }

    #[test]
    fn append_after_a_gap_zero_fills_and_marks_the_right_blocks() {
        let mut buf = ShadowBuffer {
            append_offset: 4090,
            ..Default::default()
        };
        buf.append_edit::<()>(|out| {
            out.extend_from_slice(&[b'x'; 20]);
            Ok(())
        })
        .unwrap();
        assert_eq!(buf.data.len(), 4110);
        assert!(buf.data[..4090].iter().all(|b| *b == 0));
        assert_eq!(buf.dirty.dirty_blocks().collect::<Vec<_>>(), [0, 1]);
    }

    #[test]
    fn color_upgrade_map_matches_the_switch() {
        let mapped: Vec<_> = (0..10).map(upgrade_arrow_slit_color_id).collect();
        assert_eq!(mapped, [0, 2, 5, 3, 8, 3, 6, 7, 4, 1]);
        assert_eq!(upgrade_arrow_slit_color_id(999), 999);
        assert_eq!(upgrade_arrow_slit_color_id(1000), 1);
        let mut table = [1, 2, 4, 5, 8, 0, 999, 42];
        upgrade_arrow_slit_color_table(&mut table);
        assert_eq!(table, [2, 5, 8, 3, 4, 0, 999, 1]);
    }

    #[test]
    fn upgrade_flags_and_record_filters() {
        assert!(LoadUpgradeFlags(1 << 40).upgrade_arrow_slit_color_ids());
        assert!(!LoadUpgradeFlags(1 << 40).upgrade_window_color_ids());
        assert!(LoadUpgradeFlags(1 << 48).upgrade_window_color_ids());
        assert!(
            LoadUpgradeFlags(1 << 56).0 != 0
                && !LoadUpgradeFlags(1 << 56).upgrade_window_color_ids()
        );
        assert!(arrow_slit_record_applies(0x38) && arrow_slit_record_applies(0x3a));
        assert!(!arrow_slit_record_applies(0x37) && !arrow_slit_record_applies(0x3b));
        let mut color = 5;
        upgrade_window_record_color(0x39, &mut color);
        assert_eq!(color, 5);
        upgrade_window_record_color(0x3a, &mut color);
        assert_eq!(color, 999);
    }

    #[test]
    fn applying_the_gates_writes_the_centered_path_flag() {
        let flag = core::sync::atomic::AtomicBool::new(false);
        LoadVersionGates::from_version(0x1a).apply_centered_path(&flag);
        assert!(flag.load(core::sync::atomic::Ordering::Relaxed));
        LoadVersionGates::from_version(0x19).apply_centered_path(&flag);
        assert!(!flag.load(core::sync::atomic::Ordering::Relaxed));
    }

    #[test]
    fn version_gates_split_at_0x1a() {
        let g = LoadVersionGates::from_version;
        assert_eq!(
            g(0x19),
            LoadVersionGates {
                centered_path: false,
                stairs_checksum: false
            }
        );
        assert_eq!(
            g(0x1a),
            LoadVersionGates {
                centered_path: true,
                stairs_checksum: false
            }
        );
        assert_eq!(
            g(0x1b),
            LoadVersionGates {
                centered_path: true,
                stairs_checksum: true
            }
        );
        assert!(!g(-1).centered_path);
    }
}
