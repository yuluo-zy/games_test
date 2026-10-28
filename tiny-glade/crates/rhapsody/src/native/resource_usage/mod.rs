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
mod tests;
