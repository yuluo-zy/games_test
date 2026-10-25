//! Resource usage flags implied by a declared access.
use ash::vk::{AccessFlags as A, BufferUsageFlags, ImageUsageFlags};

/// Buffer usage needed for `access`; bits without a buffer usage are ignored.
pub const fn buffer_usage_for_access(access: A) -> BufferUsageFlags {
    let access = access.as_raw();
    let mut usage = 0;
    if access & A::INDIRECT_COMMAND_READ.as_raw() != 0 {
        usage |= BufferUsageFlags::INDIRECT_BUFFER.as_raw();
    }
    if access & A::INDEX_READ.as_raw() != 0 {
        usage |= BufferUsageFlags::INDEX_BUFFER.as_raw();
    }
    if access & A::VERTEX_ATTRIBUTE_READ.as_raw() != 0 {
        usage |= BufferUsageFlags::VERTEX_BUFFER.as_raw();
    }
    if access & A::UNIFORM_READ.as_raw() != 0 {
        usage |= BufferUsageFlags::UNIFORM_BUFFER.as_raw();
    }
    if access
        & (A::SHADER_READ.as_raw()
            | A::SHADER_WRITE.as_raw()
            | A::MEMORY_READ.as_raw()
            | A::MEMORY_WRITE.as_raw())
        != 0
    {
        usage |= BufferUsageFlags::STORAGE_BUFFER.as_raw();
    }
    if access & A::TRANSFER_READ.as_raw() != 0 {
        usage |= BufferUsageFlags::TRANSFER_SRC.as_raw();
    }
    if access & A::TRANSFER_WRITE.as_raw() != 0 {
        usage |= BufferUsageFlags::TRANSFER_DST.as_raw();
    }
    BufferUsageFlags::from_raw(usage)
}

/// Image usage needed for `access`. Accesses without a single image usage
/// are invalid declarations, not device failures.
pub fn image_usage_for_access(access: A) -> crate::Result<ImageUsageFlags> {
    const DEPTH_STENCIL: A = A::from_raw(
        A::DEPTH_STENCIL_ATTACHMENT_READ.as_raw() | A::DEPTH_STENCIL_ATTACHMENT_WRITE.as_raw(),
    );
    const COLOR: A =
        A::from_raw(A::COLOR_ATTACHMENT_READ.as_raw() | A::COLOR_ATTACHMENT_WRITE.as_raw());
    const MEMORY: A = A::from_raw(A::MEMORY_READ.as_raw() | A::MEMORY_WRITE.as_raw());
    const SAMPLED_DEPTH: A =
        A::from_raw(A::DEPTH_STENCIL_ATTACHMENT_READ.as_raw() | A::SHADER_READ.as_raw());
    Ok(match access {
        a if a.is_empty() => ImageUsageFlags::empty(),
        A::SHADER_READ => ImageUsageFlags::SAMPLED,
        A::SHADER_WRITE | MEMORY => ImageUsageFlags::STORAGE,
        A::COLOR_ATTACHMENT_READ | A::COLOR_ATTACHMENT_WRITE | COLOR => {
            ImageUsageFlags::COLOR_ATTACHMENT
        }
        A::DEPTH_STENCIL_ATTACHMENT_READ | A::DEPTH_STENCIL_ATTACHMENT_WRITE | DEPTH_STENCIL => {
            ImageUsageFlags::DEPTH_STENCIL_ATTACHMENT
        }
        SAMPLED_DEPTH => ImageUsageFlags::DEPTH_STENCIL_ATTACHMENT | ImageUsageFlags::SAMPLED,
        A::TRANSFER_READ => ImageUsageFlags::TRANSFER_SRC,
        A::TRANSFER_WRITE => ImageUsageFlags::TRANSFER_DST,
        _ => {
            return Err(crate::Error::invalid_usage(format!(
                "unsupported image access mask: {access:?}"
            )));
        }
    })
}

#[cfg(test)]
mod tests {
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
}
