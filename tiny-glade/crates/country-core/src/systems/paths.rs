//! `paths.rs` — 路径绘制状态（B `systems::paths::DrawState::update`，0x140ae4bd0，汇编逐条核对）。
//!
//! 纠错：原函数有**三个**参数 `(&mut mode, mode_id: u8, raw: u8)`；`raw` 在 `r ∈ {1, 2}` 分支里被写入 `mode`
//! （`movzx edx, r8b; cmovae eax, edx`），文档里"写入 `r`"与机器码不符。
//!
//! 规则（`r = min(mode_id - 2 (u8 回绕，不是饱和减), 3)`，所以 `mode_id` 为 0/1 时 `r = 3`）：
//! - `r == 0`：`mode = 0`；
//! - `r ∈ {1, 2}`：`mode = if mode - 1 (u8 回绕) >= 2 { raw } else { 2 }`；
//! - `r == 3`：`mode = if mode - 1 (u8 回绕) < 2 { 3 } else { 0 }`。
//!
//! 常量（O，`draw_path` 0x140ae4d00）：笔刷缩放 `0.5525`（0x142a95dd0）、笔刷矩形半径系数 `0.8`（0x142a16cb8）。

/// 路径绘制模式字节。
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct DrawState(pub u8);

impl DrawState {
    /// `DrawState::update`（0x140ae4bd0）。
    pub fn update(&mut self, mode_id: u8, raw: u8) {
        let r = mode_id.wrapping_sub(2).min(3);
        let shifted = self.0.wrapping_sub(1);
        self.0 = match r {
            0 => 0,
            1 | 2 => {
                if shifted >= 2 {
                    raw
                } else {
                    2
                }
            }
            _ => {
                if shifted < 2 {
                    3
                } else {
                    0
                }
            }
        };
    }
}

/// `draw_path` 里 `eraser` 模式的笔刷缩放（O：0x142a95dd0）。
pub const ERASER_BRUSH_SCALE: f32 = 0.5525;
/// `draw_path` 笔刷矩形尺寸系数（O：0x142a16cb8）。
pub const BRUSH_RECT_SCALE: f32 = 0.8;

#[cfg(test)]
mod tests {
    use super::*;

    fn run(mode: u8, mode_id: u8, raw: u8) -> u8 {
        let mut s = DrawState(mode);
        s.update(mode_id, raw);
        s.0
    }

    #[test]
    fn only_mode_id_two_resets_to_zero() {
        assert_eq!(run(2, 2, 9), 0);
        assert_eq!(run(3, 2, 9), 0);
    }

    #[test]
    fn mode_ids_zero_and_one_wrap_around_and_behave_like_the_toggle_branch() {
        for id in [0u8, 1] {
            assert_eq!(run(1, id, 9), 3, "mode_id {id}");
            assert_eq!(run(0, id, 9), 0, "mode_id {id}");
        }
    }

    #[test]
    fn mode_ids_three_and_four_take_raw_unless_the_state_is_one_or_two() {
        for id in [3u8, 4] {
            assert_eq!(run(0, id, 9), 9);
            assert_eq!(run(3, id, 9), 9);
            assert_eq!(run(1, id, 9), 2);
            assert_eq!(run(2, id, 9), 2);
        }
    }

    #[test]
    fn mode_ids_five_and_above_toggle_between_three_and_zero() {
        for id in [5u8, 6, 200] {
            assert_eq!(run(1, id, 9), 3);
            assert_eq!(run(2, id, 9), 3);
            assert_eq!(run(0, id, 9), 0);
            assert_eq!(run(3, id, 9), 0);
        }
    }

    #[test]
    fn state_zero_wraps_to_255_and_counts_as_not_one_or_two() {
        assert_eq!(run(0, 3, 7), 7);
        assert_eq!(run(0, 5, 7), 0);
    }
}

// ---------------------------------------------------------------- 组 A 接线

/// 路径栅格资源束（O：`systems::paths::PathRastersResources`；森林/树的
/// 候选评估以 `as_float_grid`（0x140ae4c20）采样四路径栅格。字段 @+0x00/+0x20/
/// +0x40/+0x60（render/nani/instance_updates.rs 注）——语义 = 四栅格引用）。
#[derive(bevy_ecs::system::SystemParam)]
pub struct PathRastersResources<'w> {
    path: bevy_ecs::prelude::ResMut<'w, crate::resources::world_raster_defs::PathRaster>,
    cobblestone1: bevy_ecs::prelude::ResMut<'w, crate::resources::world_raster_defs::Cobblestone1Raster>,
    cobblestone2: bevy_ecs::prelude::ResMut<'w, crate::resources::world_raster_defs::Cobblestone2Raster>,
    cobblestone3: bevy_ecs::prelude::ResMut<'w, crate::resources::world_raster_defs::Cobblestone3Raster>,
}

impl PathRastersResources<'_> {
    /// `as_float_grid`（0x140ae4c20，O）：按索引取四路径栅格之一的浮点栅格引用。
    pub fn as_float_grid(&self, index: u8) -> crate::resources::world_raster::RasterBufferRef<'_, 512> {
        match index {
            0 => (**self.path).as_ref(),
            1 => (**self.cobblestone1).as_ref(),
            2 => (**self.cobblestone2).as_ref(),
            _ => (**self.cobblestone3).as_ref(),
        }
    }
}
