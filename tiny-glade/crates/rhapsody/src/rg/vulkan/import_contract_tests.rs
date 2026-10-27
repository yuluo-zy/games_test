use super::*;
use crate::resource::texture;
use crate::rg::graph;
use ash::vk;
use crate::resource::buffer;
use crate::rg::vulkan::resource_spec::validate_import_descriptor;

fn rejected(declared: graph::Descriptor, actual: graph::Descriptor) {
    assert_eq!(
        validate_import_descriptor(declared, actual)
            .unwrap_err()
            .kind(),
        crate::ErrorKind::InvalidUsage
    );
}

#[test]
fn import_accepts_extra_physical_usage_but_not_missing_usage_or_wrong_kind() {
    let declared = ResourceSpec::Buffer(buffer::BufferDesc::new_gpu_only(
        64,
        vk::BufferUsageFlags::TRANSFER_SRC,
    ))
    .descriptor();
    let actual = declared.with_usage(
        (vk::BufferUsageFlags::TRANSFER_SRC | vk::BufferUsageFlags::STORAGE_BUFFER).as_raw(),
    );
    assert!(validate_import_descriptor(declared, actual).is_ok());
    rejected(actual, declared);
    let image = ResourceSpec::Image(texture::TextureDesc::new_2d(
        4,
        4,
        vk::Format::R8G8B8A8_UNORM,
    ))
    .descriptor();
    rejected(image, declared);
    rejected(declared, image);
}

#[test]
fn import_rejects_buffer_extent_alignment_and_memory_location_mismatches() {
    let desc = buffer::BufferDesc::new_gpu_only(64, vk::BufferUsageFlags::STORAGE_BUFFER);
    let actual = ResourceSpec::Buffer(desc).descriptor();
    let mut variants = [desc; 3];
    variants[0].size = 128;
    variants[1].alignment = 256;
    variants[2].memory_location = buffer::MemoryLocation::CpuToGpu;
    for changed in variants {
        rejected(ResourceSpec::Buffer(changed).descriptor(), actual);
    }
}

#[test]
fn import_rejects_image_shape_format_and_subresource_mismatches() {
    let desc = texture::TextureDesc::new_2d(16, 16, vk::Format::R8G8B8A8_UNORM);
    let actual = ResourceSpec::Image(desc).descriptor();
    let mut variants = [desc; 9];
    variants[0].width = 32;
    variants[1].height = 32;
    variants[2].depth = 2;
    variants[3].array_elements = 2;
    variants[4].mip_levels = 2;
    variants[5].sample_count = 4;
    variants[6].format = vk::Format::B8G8R8A8_UNORM;
    variants[7].kind = texture::TextureKind::Cube;
    variants[8].flags = vk::ImageCreateFlags::MUTABLE_FORMAT;
    for changed in variants {
        rejected(ResourceSpec::Image(changed).descriptor(), actual);
    }
}
