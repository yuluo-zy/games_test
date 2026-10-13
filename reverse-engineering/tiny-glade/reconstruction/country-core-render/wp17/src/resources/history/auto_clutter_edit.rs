//! `resources::history::auto_clutter_edit` — 自动杂物编辑记录（WP17）。
//!
//! O（反编译 replay_auto_clutter 0x140aa4a60：判别 9 ReplayCmd 的
//! `Arc<AutoClutterEdit>`；serde serialize 0x140a18320 / deserialize
//! 0x140a18500）：自动杂物由墙/地形派生的装饰记录。

use crate::resources::history::{DecoratorId, WallId};
use crate::typed_gap::Gap;
use serde::{Deserialize, Serialize};

/// 自动杂物编辑负载（O：ReplayCmd 判别 9；负载字段 U）。
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
pub struct AutoClutterEdit {
    /// O：宿主墙（恢复/删除时插/删 HashMap<WallId>）。
    #[serde(default)]
    pub wall_id: Option<WallId>,
    /// O：派生装饰 id（DeletedAutoClutter erase (DecoratorId,DecoratorId)）。
    #[serde(default)]
    pub decorator_ids: Vec<DecoratorId>,
    /// U：杂物记录（dst/rank/seed 等）。
    #[serde(default)]
    pub aux: Gap,
}
