use super::*;

/// The previous bit-twiddling formulation, kept as an exhaustive oracle.
fn buffer_reference(mask: u32) -> u32 {
    let mut flags = ((mask & 0x6) << 5) | ((mask & 0x1) << 8);
    flags |= (mask & 0x8) << 1;
    if mask & 0x18060 != 0 {
        flags |= BufferUsageFlags::STORAGE_BUFFER.as_raw();
    }
    flags | ((mask >> 11) & 0x3)
}

#[test]
fn buffer_usage_matches_the_bitwise_mapping_for_every_low_access_bit_combination() {
    for mask in 0..(1u32 << 17) {
        assert_eq!(
            buffer_usage_for_access(A::from_raw(mask)).as_raw(),
            buffer_reference(mask),
            "{mask:#x}"
        );
    }
}

#[test]
fn image_usage_accepts_exactly_single_usage_accesses() {
    for (mask, usage) in [
        (0, ImageUsageFlags::empty()),
        (0x20, ImageUsageFlags::SAMPLED),
        (0x40, ImageUsageFlags::STORAGE),
        (0x18000, ImageUsageFlags::STORAGE),
        (0x80, ImageUsageFlags::COLOR_ATTACHMENT),
        (0x100, ImageUsageFlags::COLOR_ATTACHMENT),
        (0x180, ImageUsageFlags::COLOR_ATTACHMENT),
        (0x200, ImageUsageFlags::DEPTH_STENCIL_ATTACHMENT),
        (0x400, ImageUsageFlags::DEPTH_STENCIL_ATTACHMENT),
        (0x600, ImageUsageFlags::DEPTH_STENCIL_ATTACHMENT),
        (
            0x220,
            ImageUsageFlags::DEPTH_STENCIL_ATTACHMENT | ImageUsageFlags::SAMPLED,
        ),
        (0x800, ImageUsageFlags::TRANSFER_SRC),
        (0x1000, ImageUsageFlags::TRANSFER_DST),
    ] {
        assert_eq!(image_usage_for_access(A::from_raw(mask)).unwrap(), usage);
    }
    assert!(image_usage_for_access(A::INDEX_READ).is_err());
    assert!(image_usage_for_access(A::SHADER_READ | A::SHADER_WRITE).is_err());
}
