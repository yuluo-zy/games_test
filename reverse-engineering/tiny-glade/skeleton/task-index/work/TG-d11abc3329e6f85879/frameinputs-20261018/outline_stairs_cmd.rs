//! Candidate replacement of only canonical OutlineStairsCmd in structure_outline.
//! Native EventWriter@142291100 copies16B; UI producer@1422b1538..1568 and
//! structure_outline@14095a387 consume these exact offsets.
use crate::resources::stairs::stairs_state::StairSegment;
use bevy_ecs::event::Event;

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
