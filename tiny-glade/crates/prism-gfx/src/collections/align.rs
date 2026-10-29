//! Rounding of byte offsets and sizes up to an alignment.

/// Rounds `value` up to the next multiple of `alignment`; `None` when the result
/// overflows `u64`. `alignment` must be non-zero.
#[doc(hidden)]
pub fn align_up(value: u64, alignment: u64) -> Option<u64> {
    value
        .checked_add(alignment - 1)
        .map(|end| end / alignment * alignment)
}

/// Buffer offset alignment for copies of `block_bytes`-sized texel blocks: the
/// least common multiple of the block size and the 4-byte copy granularity.
#[doc(hidden)]
pub fn texel_block_alignment(block_bytes: u64) -> u64 {
    let (mut a, mut b) = (block_bytes, 4);
    while b != 0 {
        (a, b) = (b, a % b);
    }
    block_bytes / a * 4
}

#[cfg(test)]
mod tests {
    use super::align_up;

    #[test]
    fn rounds_to_non_power_of_two_and_power_of_two_alignments() {
        assert_eq!(align_up(13, 12), Some(24));
        assert_eq!(align_up(24, 12), Some(24));
        assert_eq!(align_up(0, 16), Some(0));
        assert_eq!(align_up(17, 16), Some(32));
    }

    #[test]
    fn texel_block_alignment_is_lcm_with_four() {
        for (block, expected) in [(1, 4), (2, 4), (3, 12), (4, 4), (8, 8), (12, 12), (16, 16)] {
            assert_eq!(super::texel_block_alignment(block), expected);
        }
    }

    #[test]
    fn reports_overflow() {
        assert_eq!(align_up(u64::MAX, 16), None);
        assert_eq!(align_up(u64::MAX - 14, 16), None);
        assert_eq!(align_up(u64::MAX - 15, 16), Some(u64::MAX - 15));
    }
}
