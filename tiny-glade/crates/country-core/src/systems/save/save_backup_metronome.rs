//! B save_game 0x140a3fc15..0x140a3fd36, 0x140a4100b..0x140a410da.
//! 字段：counter u32@+0x140、累计 f32@+0x144、阈值 f32@+0x148。reviewer 复核：
//! **阈值门（+0x148 < 60.0 才累计）在累计块内**，不属 AutoSaveState 本体。

const INTERVALS: [u32; 4] = [300, 600, 3600, 7200];

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct SaveBackupMetronome {
    counter: u32,
    active_game_time: f32,
    /// +0x148 阈值；仅当 < 60.0 时累计活动时间。
    active_game_threshold: f32,
}

impl Default for SaveBackupMetronome {
    fn default() -> Self {
        Self::DEFAULT
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct BackupDecision {
    pub counter: u32,
    pub remove_counter: Option<u32>,
}

impl SaveBackupMetronome {
    /// 默认状态 = counter 0、无累计活动时间、阈值 0（默认总是累计）。
    pub const DEFAULT: Self = Self {
        counter: 0,
        active_game_time: 0.0,
        active_game_threshold: 0.0,
    };
    pub fn new_from_backup_list(list: &super::save_backup_list::SaveBackupList) -> Self {
        Self::from_backup_counter(list.backup_metronome_counter())
    }
    /// The backup-list owner computes max(auto filename counter + 1), or zero.
    pub const fn from_backup_counter(counter: u32) -> Self {
        Self {
            counter,
            active_game_time: 0.0,
            active_game_threshold: 0.0,
        }
    }
    pub const fn counter(&self) -> u32 {
        self.counter
    }
    pub const fn active_game_time(&self) -> f32 {
        self.active_game_time
    }
    /// 阈值访问（AutoSaveState 配置 +0x148）。
    pub const fn active_game_threshold(&self) -> f32 {
        self.active_game_threshold
    }
    /// 设置 +0x148 阈值。
    pub fn set_active_game_threshold(&mut self, threshold: f32) {
        self.active_game_threshold = threshold;
    }
    /// `AutoSaveState::close_file` 0x140a43840 的字段重置（O：fresh disasm 尾部
    /// `+0x140 = 0`、`+0x144..0x14c = 0x501502f900000000`）：
    /// - `+0x140` counter = 0；
    /// - `+0x144` 累计活动时间 f32 = 0.0（低 32 位 0）；
    /// - `+0x148` 阈值 f32 = `0x501502f9` = **1e10**（高 32 位；> 60.0 使阈值门关闭，
    ///   关闭文件后不再累计活动时间，直到下一次打开档重新配置）。
    pub fn reset_after_close(&mut self) {
        self.counter = 0;
        self.active_game_time = 0.0;
        self.active_game_threshold = f32::from_bits(0x5015_02f9);
    }
    pub fn add_active_game_time(&mut self, delta: f32) {
        // ucomiss/jb rejects negative and unordered inputs.
        assert!(delta >= 0.0, "active_game_delta_seconds >= 0.0");
        // 阈值门：仅当 阈值 < 60.0 时累计（reviewer 复核；NaN 无序比较语义待取证）。
        if !(self.active_game_threshold < 60.0) {
            return;
        }
        self.active_game_time += delta;
    }
    pub fn consider_backing_up(&mut self) -> Option<BackupDecision> {
        // Native MAXSS/MINSS preserve the input on equality and unordered.
        let time = self.active_game_time;
        let time = if 0.0 > time { 0.0 } else { time };
        self.active_game_time = if 8.0 < time { 8.0 } else { time };
        while self.active_game_time >= 1.0 {
            self.active_game_time -= 1.0;
            let counter = self.counter;
            self.counter = self.counter.wrapping_add(1);
            let Some(tier) = INTERVALS
                .iter()
                .rposition(|interval| counter % interval == 0)
            else {
                continue;
            };
            let interval = INTERVALS[tier];
            let mut candidate = counter;
            // Keep all coarser-tier backups. Walk backward by this tier's
            // interval until finding an unprotected older backup to replace.
            let remove_counter = loop {
                let Some(previous) = candidate.checked_sub(interval) else {
                    break None;
                };
                if !INTERVALS[tier + 1..]
                    .iter()
                    .any(|coarser| previous % coarser == 0)
                {
                    break Some(previous);
                }
                candidate = previous;
            };
            return Some(BackupDecision {
                counter,
                remove_counter,
            });
        }
        None
    }
}
