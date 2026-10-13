//! 子分配：公开 gpu-allocator 经 `AllocationBackend` 接入。验证真的发生了子分配（块数远小于资源数）、
//! 偏移绑定与映射正确（逐字节读写）、释放归还区间、设备销毁时先于设备释放内存块。
//! 无 Vulkan 设备时跳过；本机可用 lavapipe。通过进程级开关显式选择模式，不依赖环境变量。
use ash::vk;
use rhapsody::raw::{
    allocator::{AllocationRequest, GpuAllocator, MemoryLocation},
    buffer::{Buffer, BufferDesc},
    device::VulkanDevice,
    init::create_headless_device,
    suballoc::{AllocationMode, SubAllocationBackend, set_default_allocation_mode},
    texture::{Texture, TextureDesc},
};
use std::sync::{Arc, Mutex, MutexGuard};

/// 默认模式是进程级全局：串行化本文件内的测试，避免互相改写。
static MODE: Mutex<()> = Mutex::new(());
fn device(mode: AllocationMode) -> Option<(MutexGuard<'static, ()>, Arc<VulkanDevice>)> {
    let guard = MODE.lock().unwrap_or_else(|e| e.into_inner());
    set_default_allocation_mode(mode);
    match create_headless_device() {
        Ok(device) => Some((guard, device)),
        Err(error) => {
            eprintln!("跳过：无可用 Vulkan 设备（{error:?}）");
            None
        }
    }
}
fn buffer(device: &Arc<VulkanDevice>, size: u64) -> Buffer {
    Buffer::new(
        device.clone(),
        &BufferDesc::new_cpu_to_gpu(size, vk::BufferUsageFlags::STORAGE_BUFFER),
    )
    .unwrap()
}

#[test]
fn many_small_resources_share_few_memory_blocks() {
    let Some((_g, device)) = device(AllocationMode::Suballocated) else { return };
    assert!(device.suballocation_enabled());
    let buffers: Vec<_> = (0..300).map(|_| buffer(&device, 4096)).collect();
    let logical = device.allocator_stats.report().allocations.len();
    let blocks = device.memory_block_count();
    assert_eq!(logical, 300, "登记表仍是每分配一条记录");
    assert!((1..=4).contains(&blocks), "300 个小缓冲应落在极少数大块内，实际 {blocks} 块");
    drop(buffers);
}

#[test]
fn dedicated_mode_still_uses_one_memory_object_per_resource() {
    let Some((_g, device)) = device(AllocationMode::Dedicated) else { return };
    assert!(!device.suballocation_enabled());
    let buffers: Vec<_> = (0..40).map(|_| buffer(&device, 4096)).collect();
    assert_eq!(device.memory_block_count(), 40);
    drop(buffers);
}

#[test]
fn neighbouring_suballocations_do_not_overlap_and_round_trip_independently() {
    let Some((_g, device)) = device(AllocationMode::Suballocated) else { return };
    // 大小互不相同，逼出不同的偏移与对齐；每个缓冲写入独有的字节模式后全部读回比对。
    let buffers: Vec<_> = (0..32).map(|i| buffer(&device, 64 + i * 97)).collect();
    for (i, b) in buffers.iter().enumerate() {
        let bytes = vec![(i as u8).wrapping_mul(7).wrapping_add(1); b.size as usize];
        b.write_bytes(0, &bytes).unwrap();
    }
    for (i, b) in buffers.iter().enumerate() {
        let mut out = vec![0u8; b.size as usize];
        b.read_bytes(0, &mut out).unwrap();
        let expected = (i as u8).wrapping_mul(7).wrapping_add(1);
        assert!(out.iter().all(|&x| x == expected), "缓冲 {i} 被邻居覆盖");
    }
}

#[test]
fn persistent_mapping_uses_the_shared_block_without_double_mapping() {
    let Some((_g, device)) = device(AllocationMode::Suballocated) else { return };
    let mut a = buffer(&device, 256);
    let mut b = buffer(&device, 256);
    a.mapped_slice_mut(0, 256).unwrap().fill(0xA5);
    b.mapped_slice_mut(0, 256).unwrap().fill(0x5A);
    // 两个缓冲大概率共享同一块 VkDeviceMemory；各自持久映射互不干扰，也不会触发重复 map 错误。
    let mut out = [0u8; 256];
    a.read_bytes(0, &mut out).unwrap();
    assert!(out.iter().all(|&x| x == 0xA5));
    b.read_bytes(0, &mut out).unwrap();
    assert!(out.iter().all(|&x| x == 0x5A));
    a.unmap();
    b.unmap();
}

#[test]
fn freed_ranges_are_returned_and_reused_without_growing_blocks() {
    let Some((_g, device)) = device(AllocationMode::Suballocated) else { return };
    let first: Vec<_> = (0..64).map(|_| buffer(&device, 8192)).collect();
    let blocks_after_first = device.memory_block_count();
    drop(first);
    device.wait_idle().unwrap();
    assert_eq!(device.allocator_stats.report().allocations.len(), 0);
    // 释放后再分配同样数量：复用已有块，不应新增块。
    let again: Vec<_> = (0..64).map(|_| buffer(&device, 8192)).collect();
    assert_eq!(device.memory_block_count(), blocks_after_first);
    drop(again);
}

#[test]
fn images_and_buffers_coexist_and_images_bind_at_their_offsets() {
    let Some((_g, device)) = device(AllocationMode::Suballocated) else { return };
    let keep: Vec<_> = (0..8).map(|_| buffer(&device, 1000)).collect();
    let textures: Vec<_> = (0..8)
        .map(|_| {
            let mut desc = TextureDesc::new_2d(16, 16, vk::Format::R8G8B8A8_UNORM);
            desc.usage = vk::ImageUsageFlags::TRANSFER_DST | vk::ImageUsageFlags::SAMPLED;
            Texture::new(device.clone(), &desc).unwrap()
        })
        .collect();
    assert!(device.memory_block_count() <= 4);
    drop(textures);
    drop(keep);
}

#[test]
fn texture_upload_round_trips_through_suballocated_staging_and_image() {
    use rhapsody::raw::setup::{init_texture_data, with_setup_cb};
    let Some((_g, device)) = device(AllocationMode::Suballocated) else { return };
    let mut desc = TextureDesc::new_2d(8, 4, vk::Format::R8G8B8A8_UNORM);
    desc.usage = vk::ImageUsageFlags::TRANSFER_DST
        | vk::ImageUsageFlags::TRANSFER_SRC
        | vk::ImageUsageFlags::SAMPLED;
    let texture = Texture::new(device.clone(), &desc).unwrap();
    let data: Vec<u8> = (0..8 * 4 * 4).map(|i| (i as u8).wrapping_mul(13)).collect();
    init_texture_data(&device, &texture, &data).unwrap();
    let readback = Buffer::new(
        device.clone(),
        &BufferDesc::new_gpu_to_cpu(data.len() as u64, vk::BufferUsageFlags::TRANSFER_DST),
    )
    .unwrap();
    with_setup_cb(&device, |cb| unsafe {
        let region = [vk::BufferImageCopy::default()
            .image_subresource(
                vk::ImageSubresourceLayers::default()
                    .aspect_mask(vk::ImageAspectFlags::COLOR)
                    .layer_count(1),
            )
            .image_extent(texture.desc.extent)];
        device.device.cmd_copy_image_to_buffer(
            cb,
            texture.handle,
            vk::ImageLayout::SHADER_READ_ONLY_OPTIMAL,
            readback.handle,
            &region,
        );
    })
    .unwrap();
    let mut out = vec![0u8; data.len()];
    readback.read_bytes(0, &mut out).unwrap();
    assert_eq!(out, data);
}

#[test]
fn the_allocation_backend_boundary_works_with_the_peak_wrapper() {
    let Some((_g, device)) = device(AllocationMode::Suballocated) else { return };
    let mut allocator = GpuAllocator::new(SubAllocationBackend { device: device.clone() }, true);
    let request = |name: &str, size: u64| AllocationRequest {
        name: name.into(),
        requirements: vk::MemoryRequirements {
            size,
            alignment: 16,
            memory_type_bits: u32::MAX,
        },
        location: MemoryLocation::CpuToGpu,
        flags: vk::MemoryAllocateFlags::empty(),
    };
    let a = allocator.allocate(&request("a", 1000)).unwrap();
    let b = allocator.allocate(&request("b", 3000)).unwrap();
    assert!(a.is_suballocated() && b.is_suballocated());
    assert!(a.host_ptr().is_some(), "宿主可见子分配应带持久映射指针");
    assert_eq!(allocator.peak_total_allocated_bytes(), 4000);
    allocator.free(a).unwrap();
    allocator.free(b).unwrap();
    assert_eq!(allocator.peak_total_allocated_bytes(), 4000, "峰值在释放后保留");
    let names: Vec<_> = allocator
        .peak_allocator_report()
        .unwrap()
        .allocations
        .iter()
        .map(|r| r.name.clone())
        .collect();
    assert_eq!(names, vec!["a".to_string(), "b".to_string()]);
}

#[test]
fn dropping_the_device_releases_blocks_before_the_device_is_destroyed() {
    let Some((_g, device)) = device(AllocationMode::Suballocated) else { return };
    let kept: Vec<_> = (0..16).map(|_| buffer(&device, 4096)).collect();
    drop(kept);
    // 仍有延迟销毁项时销毁设备：顺序错误会在这里崩溃或触发泄漏断言。
    drop(device);
}

#[test]
fn gpu_accesses_each_buffer_at_its_own_offset_not_the_start_of_the_block() {
    use rhapsody::raw::setup::with_setup_cb;
    let Some((_g, device)) = device(AllocationMode::Suballocated) else { return };
    // GPU 侧经 vkBindBufferMemory 的偏移访问缓冲；宿主读写走 gpu-allocator 的映射指针，
    // 不经绑定偏移，所以必须用 GPU 拷贝才能发现“偏移被忽略”。
    let usage = vk::BufferUsageFlags::TRANSFER_SRC | vk::BufferUsageFlags::TRANSFER_DST;
    let count = 16u64;
    let size = 256u64;
    let sources: Vec<_> = (0..count)
        .map(|_| Buffer::new(device.clone(), &BufferDesc::new_cpu_to_gpu(size, usage)).unwrap())
        .collect();
    let readback = Buffer::new(
        device.clone(),
        &BufferDesc::new_gpu_to_cpu(size * count, usage),
    )
    .unwrap();
    assert!(device.memory_block_count() <= 2, "缓冲应共享块，否则测不到偏移");
    for (i, b) in sources.iter().enumerate() {
        b.write_bytes(0, &vec![0x10 + i as u8; size as usize]).unwrap();
    }
    with_setup_cb(&device, |cb| unsafe {
        for (i, b) in sources.iter().enumerate() {
            device.device.cmd_copy_buffer(
                cb,
                b.handle,
                readback.handle,
                &[vk::BufferCopy {
                    src_offset: 0,
                    dst_offset: i as u64 * size,
                    size,
                }],
            );
        }
    })
    .unwrap();
    let mut out = vec![0u8; (size * count) as usize];
    readback.read_bytes(0, &mut out).unwrap();
    for i in 0..count as usize {
        let chunk = &out[i * size as usize..(i + 1) * size as usize];
        assert!(
            chunk.iter().all(|&x| x == 0x10 + i as u8),
            "缓冲 {i} 在 GPU 侧读到的不是它自己的数据（偏移被忽略或区间重叠）"
        );
    }
}

#[test]
fn host_visible_suballocations_are_aligned_and_sized_to_the_non_coherent_atom() {
    // lavapipe 只有一种一致性内存类型，刷新/失效路径在本机不会被执行；但“范围恰好覆盖本分配、
    // 且按粒度对齐”是可以直接检查的前提：真显卡的非一致性内存正是依赖它才能只刷新自己的范围。
    let Some((_g, device)) = device(AllocationMode::Suballocated) else { return };
    let atom = device.suballocator.as_ref().unwrap().non_coherent_atom_size;
    assert!(atom.is_power_of_two() || atom >= 1);
    let buffers: Vec<_> = (0..24)
        .map(|i| {
            Buffer::new(
                device.clone(),
                &BufferDesc::new_cpu_to_gpu_low_prio(
                    1 + i * 37,
                    vk::BufferUsageFlags::STORAGE_BUFFER,
                ),
            )
            .unwrap()
        })
        .collect();
    let mut ranges = Vec::new();
    for b in &buffers {
        let (offset, size) = b.host_range_for_test();
        assert_eq!(offset % atom, 0, "范围起点必须按非一致性粒度对齐");
        assert_eq!(size % atom, 0, "范围大小必须是非一致性粒度的整数倍");
        assert!(size >= b.size, "范围必须覆盖整个缓冲");
        ranges.push((offset, offset + size));
    }
    // 同一块内存里各分配的刷新范围互不相交：刷新自己不会波及邻居。
    ranges.sort_unstable();
    for pair in ranges.windows(2) {
        assert!(pair[0].1 <= pair[1].0, "刷新范围重叠：{pair:?}");
    }
}
