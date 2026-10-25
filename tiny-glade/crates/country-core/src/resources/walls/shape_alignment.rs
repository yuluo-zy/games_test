//! Downstream hosts for the two alignment record streams written by the wall
//! shape editors (fresh 2026-10-17).
//!
//! Evidence:
//! - `edit_rectangle` 0x14097f620 pushes a `0x20`-byte element through the
//!   `grow_one` symbol named
//!   `tuple<DecoratorId, Vec<system_decorator::decorator_visual::stairs::
//!   PreferredNodeAlignment>>`; the observed store is
//!   `{q0 @0, q1 @8, q2 @0x10, u32 @0x18, byte @0x1c, byte @0x1d}`.
//! - `adjust_wall_height_impl` 0x140ac1f20 writes the same `0x20` record with
//!   `{byte@0x1c = 0, byte@0x1d = bool}`.
//! - Both also write a `0x28`-byte `indexmap::Bucket<(DecoratorId,
//!   DecoratorId), AlignKind>`-shaped record. Producers reuse the bucket as a
//!   hand-written record (`ui_edit_terrain` documents the same pattern), so the
//!   value bytes are kept opaque instead of being reinterpreted as
//!   [`AlignKind`](crate::resources::stairs::AlignKind).
//!
//! This owner is the main-tree host face. The dense `InnerWalls` record words
//! (`+8`, `+0x28`, `+0x30`) and the decorator-id counter (`+0x40`) needed to
//! fill the records are not modelled by the semantic `WallState`; the editors
//! therefore push through these vectors only once that source is available.

use crate::resources::stairs::PreferredNodeAlignment;
use bevy_ecs::resource::Resource;

/// One `indexmap::Bucket<(DecoratorId, DecoratorId), AlignKind>`-shaped record
/// (native stride 0x28). The 24-byte value area is producer-specific.
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct AlignKindMapEntry {
    /// +0x00 indexmap hash slot (producers reuse it as a counter/hash).
    pub hash: u64,
    /// +0x08 key: two `DecoratorId` (`u32`) values packed as one qword.
    pub key: u64,
    /// +0x10..+0x27 value bytes. Native producers write
    /// `{qword@0, u32@8, f32@0xc, u8 tag@0x10, pad[7]}`; kept opaque.
    pub value: [u8; 24],
}

impl AlignKindMapEntry {
    /// Native bucket stride.
    pub const NATIVE_SIZE: usize = 0x28;
    /// `DecoratorId` pair decoded from the `+0x08` qword.
    pub fn decorator_ids(&self) -> (u32, u32) {
        (self.key as u32, (self.key >> 32) as u32)
    }
}

/// Main-tree host for the wall shape editors' alignment writes.
#[derive(Resource, Default, Debug)]
pub struct WallShapeAlignmentWrites {
    /// `Vec<(DecoratorId, Vec<PreferredNodeAlignment>)>`-shaped elements
    /// (observed 0x20-byte records).
    pub preferred_node_alignments: Vec<PreferredNodeAlignment>,
    /// `indexmap::Bucket<(DecoratorId, DecoratorId), AlignKind>`-shaped
    /// records (observed 0x28-byte records).
    pub align_kind_entries: Vec<AlignKindMapEntry>,
}

impl WallShapeAlignmentWrites {
    pub fn push_preferred_node_alignment(&mut self, record: PreferredNodeAlignment) {
        self.preferred_node_alignments.push(record);
    }

    pub fn push_align_kind_entry(&mut self, entry: AlignKindMapEntry) {
        self.align_kind_entries.push(entry);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn alignment_record_sizes_match_fresh_strides() {
        assert_eq!(
            core::mem::size_of::<PreferredNodeAlignment>(),
            0x20
        );
        assert_eq!(core::mem::size_of::<AlignKindMapEntry>(), 0x28);
        assert_eq!(
            core::mem::offset_of!(AlignKindMapEntry, key),
            0x08
        );
        assert_eq!(
            core::mem::offset_of!(AlignKindMapEntry, value),
            0x10
        );
    }

    #[test]
    fn decorator_pair_decodes_both_halves() {
        let entry = AlignKindMapEntry {
            key: (3u64) | (7u64 << 32),
            ..Default::default()
        };
        assert_eq!(entry.decorator_ids(), (3, 7));
    }
}
