//! `common_draw_params.rs` — 所有 draw pass 共用的逐帧绑定参数。
//!
//! PDB 里本模块 **没有独立过程**：`CommonDrawParams::new`（源码行 26）内联进每个 pass，`bind_common` 属
//! `render::pass_builder::draw_builder`（0x14090b460，A4 已恢复为 `DrawBuilder::bind_common`）。
//! 本文件承担：
//! - `CommonDrawParams::new(elapsed, flag)`：O 内联序列 = `get_resource_id(Time)`（TypeId 0x4bc362195b80f729）→
//!   `Duration::as_secs_f32`（`secs as f32 + nanos as f32 / 1e9`，`divss [0x1429a7640]` = 1.0e9）→ 与布尔字节一起组成
//!   `{ time: f32 @0, flag: u8 @4 }`（`mov byte [..+0x89c], 1/0`；shadow_pass 传 `cascade + 2`，见 `d_main_support::ItemMode::Shadow`）。
//! - `DrawBuilder::bind_brick_erase_highlight`（0x14090be10，1253 B；A4 标 U，PDB 前缀在 draw_builder 下，但它的调用者
//!   与世界资源布局属于本模块范围，故在此补实现）。
//! - 墙/石/木地板绑定器的薄封装（原函数在 `draw_builder`，A4 已实现）。
//!
//! 旧占位里的 `bind_roof_color_customization` / `bind_window_color_customization_textures` **没有对应 PDB 过程**
//! （只是源码行号 84/89 的 closure 内联），不在此虚构。

use core::time::Duration;

use glam::Vec3;

use crate::render::pass_builder::draw_builder::{
    BrickEraseHighlightInputs, CommonDrawFlag, CommonDrawParams, DrawBuilder,
};
use crate::render::pass_builder::pass_resource_access::SsboType;

impl CommonDrawParams {
    /// `CommonDrawParams::new`（inline）。
    pub fn new(elapsed: Duration, flag: impl Into<CommonDrawFlag>) -> CommonDrawParams {
        CommonDrawParams {
            time: elapsed.as_secs_f32(),
            flag: flag.into(),
        }
    }
}

/// `bind_brick_erase_highlight` 读取的四个世界资源值（O：四个 `get_resource_id` + `unwrap`；A4 的 `BrickEraseHighlightInputs`
/// 的上游形态）。`DrawBuilder::bind_brick_erase_highlight` 本体由 A4 实现（与本文件独立反汇编的结果逐项一致：
/// bool `0x203fd61c57064e54` = `!blocked && eraser_active`、Vec3 `0x06bdce4199980bf7` = 光标位置（未命中 0）、
/// f32 `0xb585ce21b6c728c4`），这里只做资源值 → A4 入参的转换。
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct BrickEraseInputs {
    /// 资源 1（TypeId 0x6efcda1c…）`+0xc` 字节 → `r14b`。
    pub eraser_active: bool,
    /// 资源 2（`CursorTerrainRaycast`，TypeId 0x062dfe8f…）：`+0` 的 bit0 = 命中标记，`+4` = 位置。
    pub cursor_hit: bool,
    pub cursor_pos: Vec3,
    /// 资源 3（TypeId 0xae77a86a…，`+0` 的 f32）。
    pub value: f32,
    /// 资源 4（TypeId 0x48b321ef…）`+0xfe` 字节 → `bl`。
    pub blocked: bool,
}

impl BrickEraseInputs {
    /// 转成 A4 `DrawBuilder::bind_brick_erase_highlight` 的入参。
    pub fn to_a4(&self) -> BrickEraseHighlightInputs {
        BrickEraseHighlightInputs {
            enabled: self.eraser_active,
            suppress: self.blocked,
            cursor_position: if self.cursor_hit { Some(self.cursor_pos) } else { None },
            radius: self.value,
        }
    }
}

/// `bind_brick_erase_highlight(builder, world)` 的 Rust 形态：资源值已读出。
pub fn bind_brick_erase_highlight<'p>(b: DrawBuilder<'p>, i: &BrickEraseInputs) -> DrawBuilder<'p> {
    b.bind_brick_erase_highlight(&i.to_a4())
}

/// 墙色绑定（A4 `bind_wall_color_customization<W>`）。
pub fn bind_wall_color_customization<'p, W: SsboType>(b: DrawBuilder<'p>) -> DrawBuilder<'p> {
    b.bind_wall_color_customization::<W>()
}

/// 石地板纹理（A4 `bind_stone_floor_textures`，12 张）。
pub fn bind_stone_floor_textures(b: DrawBuilder<'_>) -> DrawBuilder<'_> {
    b.bind_stone_floor_textures()
}

/// 木地板纹理（A4 `bind_wooden_floor_textures`，12 张）。
pub fn bind_wooden_floor_textures(b: DrawBuilder<'_>) -> DrawBuilder<'_> {
    b.bind_wooden_floor_textures()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn new_converts_duration_like_as_secs_f32() {
        let p = CommonDrawParams::new(Duration::new(3, 500_000_000), true);
        assert_eq!(p.time, 3.5);
        assert_eq!(p.flag, CommonDrawFlag::On);
        assert_eq!(
            CommonDrawParams::new(Duration::ZERO, CommonDrawFlag::ShadowCascade1).flag as u8,
            3
        );
        // 大值：secs as f32 + nanos as f32 / 1e9（不是 as_secs_f64 as f32）。
        let d = Duration::new(16_777_217, 1);
        assert_eq!(CommonDrawParams::new(d, false).time, 16_777_217u64 as f32 + 1.0 / 1.0e9);
    }

    #[test]
    fn brick_erase_inputs_map_to_the_a4_form() {
        let i = BrickEraseInputs {
            eraser_active: true,
            cursor_hit: false,
            cursor_pos: Vec3::new(1.0, 2.0, 3.0),
            value: 0.5,
            blocked: false,
        };
        let a = i.to_a4();
        assert!(a.enabled && !a.suppress);
        assert_eq!(a.cursor_position, None);
        assert_eq!(a.radius, 0.5);
    }
}
