//! Shared bit layout for the text shader's packed shape programs.
use crate::{TextError, TextResult};
use std::ops::Range;

pub(crate) const GLYPH_STREAM: u32 = 0;
pub(crate) const SOLID_STREAM: u32 = u32::MAX;
pub(crate) const MAX_SHAPE_WORDS: usize = u8::MAX as usize;
pub(crate) const OPCODE_MASK: u32 = u8::MAX as u32;
pub(crate) const FILL_OPCODE: u32 = 0;
pub(crate) const IMAGE_OPCODE: u32 = 1;
pub(crate) const RADIUS_OPCODE: u32 = 2;
pub(crate) const ARGUMENT_SHIFT: u32 = 8;
pub(crate) const SCALING_SHIFT: u32 = 24;
pub(crate) const IMAGE_INDEX_MASK: u32 = u16::MAX as u32;
const MAX_WORD_OFFSET: usize = 0x00ff_ffff;

/// A nonempty word range. Neither reserved primitive stream is a shape packet.
pub(crate) struct ShapePacket {
    pub(crate) word_offset: usize,
    pub(crate) word_count: usize,
}
impl ShapePacket {
    pub(crate) fn pack(self) -> TextResult<u32> {
        let Self {
            word_offset,
            word_count,
        } = self;
        if word_offset > MAX_WORD_OFFSET || word_count == 0 || word_count > MAX_SHAPE_WORDS {
            return Err(TextError::InvalidBatch(
                "shape packet exceeds packed range".into(),
            ));
        }
        let packed = ((word_offset as u32) << ARGUMENT_SHIFT) | word_count as u32;
        if packed == SOLID_STREAM {
            return Err(TextError::InvalidBatch(
                "shape packet collides with solid rectangle encoding".into(),
            ));
        }
        Ok(packed)
    }
    pub(crate) fn unpack(packed: u32) -> TextResult<Range<usize>> {
        let word_offset = (packed >> ARGUMENT_SHIFT) as usize;
        let word_count = (packed & OPCODE_MASK) as usize;
        if packed == SOLID_STREAM || word_count == 0 {
            return Err(TextError::InvalidBatch(
                "reserved primitive stream is not a shape packet".into(),
            ));
        }
        // A 24-bit offset plus an 8-bit count fits usize even on 32-bit hosts.
        Ok(word_offset..word_offset + word_count)
    }
}
