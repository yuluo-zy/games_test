use super::*;
use ash::vk::{self, Handle};
use std::sync::Arc;

#[test]
fn range_barriers_map_ranges_and_reject_the_wrong_resource_kind() {
    let image_range = crate::tracking::OwnershipRange::Image {
        aspects: vk::ImageAspectFlags::COLOR.as_raw(),
        base_mip: 2,
        mip_count: 3,
        base_layer: 4,
        layer_count: 5,
    };
    let barrier = RangeBarrier::local(
        image_range,
        vk::AccessFlags::SHADER_WRITE,
        vk::AccessFlags::SHADER_READ,
        vk::ImageLayout::GENERAL,
        vk::ImageLayout::SHADER_READ_ONLY_OPTIMAL,
    );
    let image = barrier.image(vk::Image::from_raw(7)).unwrap();
    assert_eq!(image.src_queue_family_index, vk::QUEUE_FAMILY_IGNORED);
    assert_eq!(image.dst_queue_family_index, vk::QUEUE_FAMILY_IGNORED);
    assert_eq!(
        (image.old_layout, image.new_layout),
        (
            vk::ImageLayout::GENERAL,
            vk::ImageLayout::SHADER_READ_ONLY_OPTIMAL
        )
    );
    let range = image.subresource_range;
    assert_eq!(
        (
            range.base_mip_level,
            range.level_count,
            range.base_array_layer,
            range.layer_count
        ),
        (2, 3, 4, 5)
    );
    assert!(barrier.buffer(vk::Buffer::from_raw(7)).is_err());
    let mut scope = SynchronizationScope::default();
    assert!(
        scope
            .push(BarrierTarget::Buffer(vk::Buffer::from_raw(7)), &barrier)
            .is_err()
    );
    assert!(scope.is_empty());
    scope
        .push(BarrierTarget::Image(vk::Image::from_raw(7)), &barrier)
        .unwrap();
    assert_eq!(scope.images.len(), 1);
}

#[test]
fn conversion_preserves_accesses_layouts_ranges_and_ownership_pairs() {
    let stats = Arc::new(crate::accounting::AllocatorStats::default());
    let mut scope = SynchronizationScope::new(
        vk::PipelineStageFlags::TRANSFER | vk::PipelineStageFlags::HOST,
        vk::PipelineStageFlags::COMPUTE_SHADER,
    );
    scope.memory.push(
        vk::MemoryBarrier::default()
            .src_access_mask(vk::AccessFlags::TRANSFER_WRITE)
            .dst_access_mask(vk::AccessFlags::SHADER_READ),
    );
    scope.buffers.push(
        vk::BufferMemoryBarrier::default()
            .src_access_mask(vk::AccessFlags::TRANSFER_WRITE)
            .dst_access_mask(vk::AccessFlags::SHADER_READ)
            .src_queue_family_index(1)
            .dst_queue_family_index(3)
            .buffer(vk::Buffer::from_raw(101))
            .offset(123)
            .size(456),
    );
    scope.images.push(
        vk::ImageMemoryBarrier::default()
            .src_access_mask(vk::AccessFlags::SHADER_WRITE)
            .dst_access_mask(vk::AccessFlags::TRANSFER_READ)
            .old_layout(vk::ImageLayout::GENERAL)
            .new_layout(vk::ImageLayout::TRANSFER_SRC_OPTIMAL)
            .src_queue_family_index(3)
            .dst_queue_family_index(1)
            .image(vk::Image::from_raw(202))
            .subresource_range(
                vk::ImageSubresourceRange::default()
                    .aspect_mask(vk::ImageAspectFlags::DEPTH | vk::ImageAspectFlags::STENCIL)
                    .base_mip_level(1)
                    .level_count(2)
                    .base_array_layer(3)
                    .layer_count(4),
            ),
    );
    let converted = Barrier2Scope::new(&scope, &stats).unwrap();
    assert_eq!(
        converted.memory[0].src_stage_mask.as_raw(),
        u64::from(scope.source_stages.as_raw())
    );
    assert_eq!(
        converted.memory[0].dst_stage_mask,
        vk::PipelineStageFlags2::COMPUTE_SHADER
    );
    assert_eq!(
        converted.memory[0].src_access_mask,
        vk::AccessFlags2::TRANSFER_WRITE
    );
    assert_eq!(
        converted.memory[0].dst_access_mask,
        vk::AccessFlags2::SHADER_READ
    );
    let buffer = converted.buffers[0];
    assert_eq!(buffer.buffer.as_raw(), 101);
    assert_eq!(
        (buffer.src_queue_family_index, buffer.dst_queue_family_index),
        (1, 3)
    );
    assert_eq!((buffer.offset, buffer.size), (123, 456));
    assert_eq!(buffer.src_access_mask, vk::AccessFlags2::TRANSFER_WRITE);
    assert_eq!(buffer.dst_access_mask, vk::AccessFlags2::SHADER_READ);
    let image = converted.images[0];
    assert_eq!(image.image.as_raw(), 202);
    assert_eq!(
        (image.src_queue_family_index, image.dst_queue_family_index),
        (3, 1)
    );
    assert_eq!(
        (image.old_layout, image.new_layout),
        (
            vk::ImageLayout::GENERAL,
            vk::ImageLayout::TRANSFER_SRC_OPTIMAL
        )
    );
    assert_eq!(image.src_access_mask, vk::AccessFlags2::SHADER_WRITE);
    assert_eq!(image.dst_access_mask, vk::AccessFlags2::TRANSFER_READ);
    let range = image.subresource_range;
    assert_eq!(
        range.aspect_mask,
        vk::ImageAspectFlags::DEPTH | vk::ImageAspectFlags::STENCIL
    );
    assert_eq!(
        (
            range.base_mip_level,
            range.level_count,
            range.base_array_layer,
            range.layer_count
        ),
        (1, 2, 3, 4)
    );
    assert_eq!(
        stats.cpu_memory_usage().total(),
        0,
        "common barriers remain inline"
    );
}

#[test]
fn stage_only_scope_retains_a_real_execution_dependency() {
    let stats = Arc::new(crate::accounting::AllocatorStats::default());
    let scope = SynchronizationScope::new(
        vk::PipelineStageFlags::ALL_COMMANDS,
        vk::PipelineStageFlags::ALL_COMMANDS,
    );
    let converted = Barrier2Scope::new(&scope, &stats).unwrap();
    assert_eq!(converted.memory.len(), 1);
    assert_eq!(
        converted.memory[0].src_stage_mask,
        vk::PipelineStageFlags2::ALL_COMMANDS
    );
    assert_eq!(
        converted.memory[0].dst_stage_mask,
        vk::PipelineStageFlags2::ALL_COMMANDS
    );
    assert!(converted.memory[0].src_access_mask.is_empty());
    assert!(converted.memory[0].dst_access_mask.is_empty());
}

#[test]
fn synchronization2_preserves_empty_stage_scopes() {
    let stats = Arc::new(crate::accounting::AllocatorStats::default());
    let scope = SynchronizationScope::new(vk::PipelineStageFlags::empty(), vk::PipelineStageFlags::empty());
    let converted = Barrier2Scope::new(&scope, &stats).unwrap();
    assert!(converted.memory[0].src_stage_mask.is_empty());
    assert!(converted.memory[0].dst_stage_mask.is_empty());
}

#[test]
fn later_spill_admission_failure_releases_already_prepared_capacity() {
    let stats = Arc::new(crate::accounting::AllocatorStats::default());
    let mut scope = SynchronizationScope::default();
    scope
        .buffers
        .extend([vk::BufferMemoryBarrier::default(); 5]);
    scope.images.extend([vk::ImageMemoryBarrier::default(); 5]);
    stats
        .set_total_budget((5 * size_of::<vk::BufferMemoryBarrier2<'static>>()) as u64)
        .unwrap();
    assert!(matches!(
        Barrier2Scope::new(&scope, &stats),
        Err(crate::Error::OutOfHostMemory)
    ));
    assert_eq!(stats.cpu_memory_usage().total(), 0);
    stats.set_total_budget(4096).unwrap();
    let converted = Barrier2Scope::new(&scope, &stats).unwrap();
    assert!(
        stats.cpu_memory_usage().live
            >= (5
                * (size_of::<vk::BufferMemoryBarrier2<'static>>()
                    + size_of::<vk::ImageMemoryBarrier2<'static>>())) as u64
    );
    drop(converted);
    assert_eq!(stats.cpu_memory_usage().total(), 0);
}
