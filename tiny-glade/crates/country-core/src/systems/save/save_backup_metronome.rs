//! B save_game 0x140a3fc15..0x140a3fd36, 0x140a4100b..0x140a410da.
//! The +0x148 inactivity timer belongs to AutoSaveState, not this metronome.

const INTERVALS: [u32; 4] = [300, 600, 3600, 7200];

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct SaveBackupMetronome {
    counter: u32,
    active_game_time: f32,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct BackupDecision {
    pub counter: u32,
    pub remove_counter: Option<u32>,
}

impl SaveBackupMetronome {
    pub fn new_from_backup_list(list: &super::save_backup_list::SaveBackupList) -> Self {
        Self::from_backup_counter(list.backup_metronome_counter())
    }
    /// The backup-list owner computes max(auto filename counter + 1), or zero.
    pub const fn from_backup_counter(counter: u32) -> Self {
        Self {
            counter,
            active_game_time: 0.0,
        }
    }
    pub const fn counter(&self) -> u32 {
        self.counter
    }
    pub const fn active_game_time(&self) -> f32 {
        self.active_game_time
    }
    pub fn add_active_game_time(&mut self, delta: f32) {
        // ucomiss/jb rejects negative and unordered inputs.
        assert!(delta >= 0.0, "active_game_delta_seconds >= 0.0");
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
