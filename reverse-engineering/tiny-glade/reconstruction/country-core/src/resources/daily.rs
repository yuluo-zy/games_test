//! daily.rs — 每日主题（WP21）。
//!
//! O：`DailyThemeInfo`（default 0x83c480 置两个 u64=0x8000000000000000 哨兵、
//! `is_weekly` 0x83c440 用 SystemTime::duration_since、`get_name` 0x83c240）。
//! `CurrentDaily` / `DailyThemeCredits`。

use std::time::SystemTime;

use bevy_ecs::resource::Resource;

use crate::typed_gap::Gap;

/// `DailyThemeInfo`（O：两个 SystemTime u64 哨兵 + 名称）。
#[derive(Clone, Debug)]
pub struct DailyThemeInfo {
    /// O：default 置 0x8000000000000000×2（前 epoch 哨兵）。
    pub start: SystemTime,
    pub _gap: Gap,
}

impl Default for DailyThemeInfo {
    fn default() -> Self {
        Self { start: SystemTime::UNIX_EPOCH, _gap: Gap::default() }
    }
}

impl DailyThemeInfo {
    /// `default`（O 0x83c480）。
    pub fn new() -> Self {
        Self::default()
    }
    /// `get_name`（O 0x83c240）：主题名（String，U 内容）。
    pub fn get_name(&self) -> String {
        String::new()
    }
    /// `is_weekly`（O 0x83c440）：`SystemTime::duration_since` 判周限。
    pub fn is_weekly(&self) -> bool {
        match self.start.elapsed() {
            Ok(d) => d.as_secs() >= 7 * 86400,
            Err(_) => false,
        }
    }
}

/// `CurrentDaily`（O：当前每日主题资源）。
#[derive(Resource, Clone, Debug, Default)]
pub struct CurrentDaily(pub Gap);

/// `DailyThemeCredits`（O：主题致谢，U）。
#[derive(Resource, Clone, Debug, Default)]
pub struct DailyThemeCredits(pub Gap);
