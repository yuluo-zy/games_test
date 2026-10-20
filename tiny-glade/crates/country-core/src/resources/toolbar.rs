//! B Toolbar constructor (0x140af8c50), contains (0x140af89d0),
//! switch_mode (0x140af9850), and cycling closures (0x140af9f30..0x140afaeE0).
use crate::systems::mode_manager::{AppMode, DecoratorType, EditSubject, EraserLayer, WallShape};
use bevy_ecs::resource::Resource;
use serde::{Deserialize, Serialize};

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[repr(u8)]
pub enum ToolbarCategory {
    #[default]
    Building = 0,
    Path = 1,
    Decorators = 2,
    Customization = 3,
    Eraser = 4,
    Tree = 5,
    Terrain = 6,
    Editing = 7,
    Water = 8,
    Garden = 9,
    Photomode = 10,
    TimeOfDay = 11,
    PhotomodeFolder = 12,
    PhotomodeCamera = 13,
    PhotomodeFov = 14,
    PhotomodeFocus = 15,
    PhotomodeDof = 16,
    PhotomodeTakePic = 17,
    PhotomodeEvShift = 18,
    PhotomodeLuts = 19,
    PhotomodeArtistic = 20,
    PhotomodeBack = 21,
}
impl ToolbarCategory {
    pub fn from_index(index: usize) -> Option<Self> {
        let all: [ToolbarCategory; 22] = [
            Self::Building,
            Self::Path,
            Self::Decorators,
            Self::Customization,
            Self::Eraser,
            Self::Tree,
            Self::Terrain,
            Self::Editing,
            Self::Water,
            Self::Garden,
            Self::Photomode,
            Self::TimeOfDay,
            Self::PhotomodeFolder,
            Self::PhotomodeCamera,
            Self::PhotomodeFov,
            Self::PhotomodeFocus,
            Self::PhotomodeDof,
            Self::PhotomodeTakePic,
            Self::PhotomodeEvShift,
            Self::PhotomodeLuts,
            Self::PhotomodeArtistic,
            Self::PhotomodeBack,
        ];
        all.get(index).copied()
    }
}

/// 工具栏内的单类别模式组；实际 ECS 资源是持有所有类别的 Toolbar。
#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
pub struct ToolbarCategoryModes {
    pub modes: Vec<AppMode>,
    pub current: usize,
}
impl ToolbarCategoryModes {
    pub fn contains(&self, mode: &AppMode) -> bool {
        self.modes.contains(mode)
    }
    pub fn last_selected_mode(&self) -> Option<AppMode> {
        self.modes.get(self.current).copied()
    }
    pub fn next_mode(&mut self) -> Option<AppMode> {
        if self.modes.is_empty() {
            return None;
        }
        self.current = self.current.wrapping_add(1) % self.modes.len();
        Some(self.modes[self.current])
    }
}

#[derive(Resource, Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct Toolbar {
    pub categories: Vec<ToolbarCategoryModes>,
    pub current: ToolbarCategory,
}
impl Default for Toolbar {
    fn default() -> Self {
        Self::new(false)
    }
}
impl Toolbar {
    /// The original reads the global demo flag. Its value is supplied explicitly
    /// here so both editions use the same constructor without a guessed global.
    pub fn new(demo: bool) -> Self {
        use AppMode as A;
        use DecoratorType as D;
        use EraserLayer as E;
        use WallShape as W;
        let buildings = if demo {
            vec![
                A::Wall(W::Circle),
                A::Wall(W::Rectangle),
                A::Wall(W::Freeform),
            ]
        } else {
            vec![
                A::Wall(W::Circle),
                A::Wall(W::Rectangle),
                A::Decorator(D::Stairs),
                A::Wall(W::Freeform),
                A::Fences,
            ]
        };
        let paths = if demo {
            vec![A::Path]
        } else {
            vec![
                A::Path,
                A::Cobblestone(1),
                A::Cobblestone(2),
                A::Cobblestone(3),
            ]
        };
        let decorators = if demo {
            vec![A::Decorator(D::CottageWindow), A::Decorator(D::Lantern)]
        } else {
            vec![
                A::Decorator(D::CottageWindow),
                A::Decorator(D::GothicWindow),
                A::Decorator(D::ArrowSlit),
                A::Decorator(D::Lantern),
                A::Decorator(D::Flag),
                A::Decorator(D::Chimney),
            ]
        };
        let erasers = if demo {
            vec![E::Wall, E::Path, E::Garden, E::Bulldoze]
        } else {
            vec![E::Wall, E::Path, E::Bulldoze, E::Garden, E::Tree, E::Water]
        };
        let groups = vec![
            buildings,
            paths,
            decorators,
            vec![A::Color],
            erasers.into_iter().map(A::Eraser).collect(),
            if demo { vec![] } else { vec![A::Tree] },
            vec![A::Terrain],
            vec![A::Editing(EditSubject::None)],
            if demo { vec![] } else { vec![A::Water] },
            vec![A::Garden],
            vec![A::Photomode],
        ];
        Self {
            categories: groups
                .into_iter()
                .map(|modes| ToolbarCategoryModes { modes, current: 0 })
                .collect(),
            current: ToolbarCategory::Building,
        }
    }
    /// Select the category, cycling only when it was already selected. Empty
    /// and absent categories preserve both remembered indices and current.
    pub fn advance_category(&mut self, category: ToolbarCategory) -> Option<AppMode> {
        let modes = self.categories.get_mut(category as usize)?;
        let mode = if self.current == category {
            modes.next_mode()
        } else {
            modes.last_selected_mode()
        };
        if mode.is_some() {
            self.current = category;
        }
        mode
    }
    /// Select the first category and entry equal to the requested full mode.
    /// The original panics if the mode is not represented in this toolbar.
    pub fn switch_mode(&mut self, mode: &AppMode) {
        let (category, index) = self
            .categories
            .iter()
            .enumerate()
            .find_map(|(category, modes)| {
                modes
                    .modes
                    .iter()
                    .position(|candidate| candidate == mode)
                    .map(|index| (category, index))
            })
            .unwrap_or_else(|| panic!("mode {mode:?} has no toolbar category"));
        assert!(category < 11, "internal error: entered unreachable code");
        self.categories[category].current = index;
        self.current = ToolbarCategory::from_index(category).unwrap();
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn original_edition_tables_and_cycles() {
        let mut full = Toolbar::new(false);
        let demo = Toolbar::new(true);
        assert_eq!(
            full.categories
                .iter()
                .map(|x| x.modes.len())
                .collect::<Vec<_>>(),
            [5, 4, 6, 1, 6, 1, 1, 1, 1, 1, 1]
        );
        assert_eq!(
            demo.categories
                .iter()
                .map(|x| x.modes.len())
                .collect::<Vec<_>>(),
            [3, 1, 2, 1, 4, 0, 1, 1, 0, 1, 1]
        );
        assert_eq!(
            full.advance_category(ToolbarCategory::Building),
            Some(AppMode::Wall(WallShape::Rectangle))
        );
        assert_eq!(
            full.advance_category(ToolbarCategory::Path),
            Some(AppMode::Path)
        );
        full.switch_mode(&AppMode::Cobblestone(3));
        assert_eq!(full.categories[1].current, 3);
        assert_eq!(
            full.advance_category(ToolbarCategory::Path),
            Some(AppMode::Path)
        );
    }
    #[test]
    fn equality_compares_payload_and_demo_empty_category_is_unchanged() {
        let mut demo = Toolbar::new(true);
        assert_eq!(demo.advance_category(ToolbarCategory::Water), None);
        assert_eq!(demo.current, ToolbarCategory::Building);
        assert!(!demo.categories[7].contains(&AppMode::Editing(EditSubject::Roof(4))));
        assert!(demo.categories[0].contains(&AppMode::Wall(WallShape::Rectangle)));
    }
}
/// Original toolbar focus publication: a frame emits one event when any
/// toolbar item has a nonzero keep byte (B ui_notify_hints_if_toolbar_in_focus).
#[derive(bevy_ecs::event::Event, Clone, Copy, Debug, Default)]
pub struct ToolbarUiIsInFocus;
