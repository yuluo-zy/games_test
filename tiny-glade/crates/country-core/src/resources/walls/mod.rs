pub mod height_rules;
pub mod dormers;
pub mod decorator;
pub mod decorator_archivist;
#[cfg(feature = "native-physics")]
pub mod decorator_collision;
pub mod decorator_storage;
pub mod decorator_subtype;
pub mod decorator_outline;
pub mod cached_decorator_transforms;
pub mod arches;
pub mod curved_halftimber;
pub mod elevation_support_stats;
pub mod material_stats;
pub mod plaster;
pub mod wall_path_segment;
pub mod wall_stone_columns;
pub mod wooden_gates;
pub mod inner_wall_state;
pub mod inner_walls;
pub mod public_walls;
pub mod public_wall_state;
pub mod wall_holes;
pub mod wall_trigger_volume;
pub mod shape_alignment;

pub use inner_wall_state::WallState;
pub use inner_walls::{InnerWalls, WallChanges};

/// B eraser multiplier read as one f32. Startup policy remains with its owner.
#[derive(bevy_ecs::resource::Resource, Clone, Copy, Debug, Default, PartialEq)]
pub struct EraserSizeMultiplier(pub f32);

/// Wall identity (B 0x140b34f20): zero is invalid.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Default)]
pub struct WallId(pub u64);
impl WallId {
    pub fn get_inner(self) -> u64 {
        self.0
    }
    pub fn is_valid(self) -> bool {
        self.0 != 0
    }
}

/// Native `resources::walls::impl$0::fmt` (0x140b34ec0): `"WallId({})"`.
/// Used by the `wall_height_edit_` signifier names and the
/// `"Tried deleting roof {} during Replay"` panic.
impl std::fmt::Display for WallId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "WallId({})", self.0)
    }
}

/// Only the highlighter fields consumed by the recovered render callbacks.
/// This is a safe Rust input representation, not a claim of full original ABI.
#[derive(bevy_ecs::resource::Resource, Clone, Copy, Debug, Default)]
pub struct WallHighlight {
    pub wall_entity: Option<bevy_ecs::entity::Entity>,
    pub wall_id: Option<WallId>,
    pub ty: u8,
}

/// 高亮颜色判别（O：`HighlightColor::to_rgb` 0x140b34f30，`resources::walls::HighlightColor`；
/// 1 字节判别值）。`to_rgb` 按判别值 `== 1` 二分支返回 RGB。
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct HighlightColor(pub u8);

impl HighlightColor {
    /// `HighlightColor::to_rgb`（0x140b34f30，O）：判别值 == 1 → `{0.1, 0.03, 0.0}`
    /// （`0x3dcccccd/0x3cf5c290`），否则 → `{0.07, 0.08, 0.07}`（`0x3d8f5c29` 等）。
    /// 返回 f32 三件套（O：XMM 写 + 第三分量）。
    pub fn to_rgb(self) -> [f32; 3] {
        if self.0 == 1 {
            [0.1, 0.03, 0.0]
        } else {
            [0.07, 0.08, 0.07]
        }
    }
}

/// 裙层高度序列（O：`resources::walls::split_height_into_skirts` 0x140b34f80）。
/// 高度 ≥ 5.9 时产出 `[4.5, 8.0, 11.5, …]`（首 4.5、步长 3.5、`height > f + 1.4` 才续）；
/// 否则空。返回 Vec `{ptr, cap, len}`（cap = 4）。
pub fn split_height_into_skirts(height: f32) -> Vec<f32> {
    let mut skirts = Vec::new();
    if height < 5.9 {
        return skirts;
    }
    skirts.push(4.5);
    let mut f = 4.5;
    loop {
        f += 3.5;
        if height <= f + 1.4 {
            break;
        }
        skirts.push(f);
    }
    skirts
}

pub mod color_selection;

pub mod height_cache;

pub mod publication_source;

pub mod freeform_height_cache;

pub mod gable_heights;

pub mod spatial_index;

pub mod publication_fields;
pub mod in_progress_wall;
