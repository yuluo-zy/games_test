//! structure_outline.rs — 结构轮廓命令（组 A restorer 接线；BASELINE 参数面
//! `systems::wall::structure_outline::OutlineWallCmd` / `OutlineStairsCmd`——
//! system_color 颜色UI发射结构轮廓高亮。OutlineStairsCmd的16B结构已由
//! B writer@142291100、producer@1422b1538和consumer@14095a387交叉恢复。
//! OutlineWallCmd的载荷仍独立待恢复。

use bevy_ecs::event::Event;

/// 墙轮廓命令（O：`structure_outline::OutlineWallCmd`；载荷语义 U）。
#[derive(Event, Clone, Copy, Debug, Default)]
pub struct OutlineWallCmd {
    /// typed gap。
    pub _gap: u64,
}

use crate::resources::stairs::stairs_state::StairSegment;

#[repr(C)]
#[derive(Event, Clone, Copy, Debug)]
pub struct OutlineStairsCmd {
    pub segment: StairSegment,   // +00, two sorted endpoint words
    pub strength: f32,           // +08, consumer ignores <=0
    pub kind: u8,                // +0c, original flag (name inferred)
    pub include_connected: bool, // +0d, whole connected stairs if true
}
const _: () = assert!(core::mem::size_of::<OutlineStairsCmd>() == 16);
const _: () = assert!(core::mem::offset_of!(OutlineStairsCmd, strength) == 8);
const _: () = assert!(core::mem::offset_of!(OutlineStairsCmd, kind) == 12);
const _: () = assert!(core::mem::offset_of!(OutlineStairsCmd, include_connected) == 13);

impl OutlineStairsCmd {
    /// Explicit semantic factory for the observed UI producer; the native
    /// caller writes these fields directly, not a recovered original method.
    pub fn connected_color_hover(segment: StairSegment) -> Self {
        Self {
            segment,
            strength: 2.0,
            kind: 0,
            include_connected: true,
        }
    }
}
