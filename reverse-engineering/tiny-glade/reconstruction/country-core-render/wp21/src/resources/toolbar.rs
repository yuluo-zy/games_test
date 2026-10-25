//! toolbar.rs — 工具栏（WP21）。
//!
//! O：`Toolbar::new`（0xaf8c50）：11 类模式表（demo 精简）；`ToolbarCategoryModes::contains`
//!（0xaf89d0）；`Toolbar::switch_mode`（0xaf9850，未找到 panic）。模式项 0x10B：
//! `{cat: u32, mode: u8, arg: u64}`。

use bevy_ecs::resource::Resource;

use crate::typed_gap::Gap;

/// `ToolbarCategory`（O：类别枚举；id 6/7/8/0xb/0xc/9/0x10/10/0xf/0x11/0x13）。
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ToolbarCategory {
    Buildings = 6,
    Walls = 7,
    Roofs = 8,
    Decor = 0xb,
    Misc = 0xc,
    Paths = 9,
    Trees = 0x10,
    Water = 10,
    Meadow = 0xf,
    Animals = 0x11,
    Terrain = 0x13,
}

impl ToolbarCategory {
    /// O：id→枚举。
    pub fn from_id(id: u32) -> Option<Self> {
        match id {
            6 => Some(ToolbarCategory::Buildings),
            7 => Some(ToolbarCategory::Walls),
            8 => Some(ToolbarCategory::Roofs),
            0xb => Some(ToolbarCategory::Decor),
            0xc => Some(ToolbarCategory::Misc),
            9 => Some(ToolbarCategory::Paths),
            0x10 => Some(ToolbarCategory::Trees),
            10 => Some(ToolbarCategory::Water),
            0xf => Some(ToolbarCategory::Meadow),
            0x11 => Some(ToolbarCategory::Animals),
            0x13 => Some(ToolbarCategory::Terrain),
            _ => None,
        }
    }
}

/// `ToolbarCategoryModes`（O：类别模式集；contains 0xaf89d0）。
#[derive(Clone, Debug, Default)]
pub struct ToolbarCategoryModes {
    /// 模式项（O：stride 0x10 = `{cat:u32, mode:u8, arg:u64}`）。
    pub modes: Vec<(u32, u8, u64)>,
}

impl ToolbarCategoryModes {
    /// `contains`（O 0xaf89d0）。
    pub fn contains(&self, category: u32, mode: u8) -> bool {
        self.modes
            .iter()
            .any(|(c, m, _)| *c == category && (*m == mode || (0x38 <= *m && 0x3d >= *m)))
    }
    /// `next_mode` / `last_selected_mode`（O 名，13 内联）。
    pub fn next_mode(&self, _cur: u8) -> u8 {
        0
    }
}

/// `Toolbar`（O：工具栏资源）。
#[derive(Resource, Clone, Debug)]
pub struct Toolbar {
    /// 各类别模式集（O：`Toolbar::new` 建 11 类）。
    pub categories: Vec<(ToolbarCategory, ToolbarCategoryModes)>,
    /// 当前模式索引（O：`switch_mode` 写 +0x18）。
    pub current: u64,
}

impl Default for Toolbar {
    fn default() -> Self {
        Self::new(false)
    }
}

impl Toolbar {
    /// `new`（O 0xaf8c50）：默认类别/模式表。
    pub fn new(demo: bool) -> Self {
        let mut categories = Vec::new();
        // 类 6 × [1,2,0]
        categories.push((ToolbarCategory::Buildings, ToolbarCategoryModes {
            modes: vec![(6, 1, 0), (6, 2, 0), (6, 0, 0)],
        }));
        // 类 0xb × [0x38..0x3d]（demo 只 0x38/0x3c）
        categories.push((ToolbarCategory::Decor, ToolbarCategoryModes {
            modes: if demo {
                vec![(0xb, 0x38, 0), (0xb, 0x3c, 0)]
            } else {
                vec![
                    (0xb, 0x38, 0),
                    (0xb, 0x39, 0),
                    (0xb, 0x3a, 0),
                    (0xb, 0x3c, 0),
                    (0xb, 0x3d, 0),
                    (0xb, 0x3b, 0),
                ]
            },
        }));
        // 类 9 × [1,2,3,5,6,4]（demo [1,2,5,3]）
        categories.push((ToolbarCategory::Paths, ToolbarCategoryModes {
            modes: if demo {
                vec![(9, 1, 0), (9, 2, 0), (9, 5, 0), (9, 3, 0)]
            } else {
                vec![(9, 1, 0), (9, 2, 0), (9, 3, 0), (9, 5, 0), (9, 6, 0), (9, 4, 0)]
            },
        }));
        let _ = demo;
        Self { categories, current: 0 }
    }
    /// `switch_mode`（O 0xaf9850）。
    pub fn switch_mode(&mut self, category: u32, mode: u8) {
        for (i, (cat, modes)) in self.categories.iter().enumerate() {
            if cat.to_owned() as u32 == category && modes.contains(category, mode) {
                self.current = i as u64;
                return;
            }
        }
        panic!("Switched to mode but no toolbar category contains it");
    }
    /// `switch_category`（O 名，13 内联）。
    pub fn switch_category(&mut self, _category: ToolbarCategory) {}
}

/// `ToolbarUiIsInFocus`（O：UI 焦点标志）。
#[derive(Resource, Clone, Copy, Debug, Default, PartialEq)]
pub struct ToolbarUiIsInFocus(pub bool);

/// `ResetToolbar`（O：重置事件）。
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct ResetToolbar(pub Gap);
