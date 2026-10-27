//! raw-core 缓冲/纹理域测试：判据、边界与描述符构造。
//! 需要 Vulkan 的用例在无设备时跳过；本机可用 lavapipe。
// 区间参数刻意传单元素切片与反向区间，用来覆盖原版的区间校验。
#![allow(clippy::single_range_in_vec_init, clippy::reversed_empty_ranges)]
use ash::vk;
use rhapsody::device::Device;
use rhapsody::resource::buffer::{BufferDesc, BufferRegion, MemoryLocation, buffer_alignment, plan_buffer_update};
use rhapsody::resource::texture::{TextureDesc, TextureKind, TextureViewDesc, get_image_create_info_with_data, image_aspect_mask_from_format};
use rhapsody::device::selection::create_headless_device;

fn device() -> Option<std::sync::Arc<Device>> {
    create_headless_device().ok()
}

// ---------------------------------------------------------------- Buffer：判据

#[test]
fn alignment_is_max_of_requirement_desc_and_usage_floor() {
    // 0x140bee200：usage & 0x400 抬到 0x40，否则 0x10；再与 memreq.alignment、desc.alignment 取大。
    assert_eq!(buffer_alignment(4, 0, 0), 0x10);
    assert_eq!(buffer_alignment(4, 0, 0x400), 0x40);
    assert_eq!(buffer_alignment(0x100, 0x20, 0x400), 0x100);
    assert_eq!(buffer_alignment(4, 0x80, 0), 0x80);
    assert_eq!(
        buffer_alignment(0x10, 0, 0x20 | 0x2),
        0x10,
        "非 0x400 位不抬高"
    );
}

#[test]
fn sparse_update_plan_preserves_destinations_and_rejects_invalid_ranges() {
    let desc = BufferDesc::new_cpu_to_gpu(64, vk::BufferUsageFlags::TRANSFER_DST);
    let plan = plan_buffer_update(&desc, 8, &[0..2, 4..6, 6..8], 10).unwrap();
    assert_eq!(plan.total, 6);
    assert_eq!(
        plan.regions,
        vec![
            BufferRegion {
                src_offset: 0,
                dst_offset: 10,
                size: 2
            },
            BufferRegion {
                src_offset: 2,
                dst_offset: 14,
                size: 2
            },
            BufferRegion {
                src_offset: 4,
                dst_offset: 16,
                size: 2
            },
        ]
    );
    let no_dst = BufferDesc::new_cpu_to_gpu(64, vk::BufferUsageFlags::STORAGE_BUFFER);
    for error in [
        plan_buffer_update(&desc, 8, &[5..3], 0).unwrap_err(),
        plan_buffer_update(&desc, 4, &[0..5], 0).unwrap_err(),
        plan_buffer_update(&desc, 8, &[0..8], 60).unwrap_err(),
        plan_buffer_update(&desc, 64, &[60..62], 8).unwrap_err(),
        plan_buffer_update(&desc, 8, &[0..2], u64::MAX).unwrap_err(),
        plan_buffer_update(&no_dst, 8, &[0..2], 0).unwrap_err(),
        plan_buffer_update(&desc, 8, &[], 0).unwrap_err(),
        plan_buffer_update(&desc, 8, &[3..3], 0).unwrap_err(),
    ] {
        assert_eq!(error.kind(), rhapsody::ErrorKind::InvalidUsage);
    }
    // Empty source ranges never reach vkCmdCopyBuffer as zero-size regions.
    let plan = plan_buffer_update(&desc, 4, &[0..0, 0..4, 4..4], 0).unwrap();
    assert_eq!(plan.regions.len(), 1);
    assert_eq!(plan.total, 4);
}

#[test]
fn texture_view_ranges_check_exact_remainders_zero_and_overflow() {
    let texture = TextureDesc::new_2d_array(8, 8, 4, vk::Format::R8G8B8A8_UNORM).mip_levels(3).sampled();
    let full = TextureViewDesc::new(&texture).unwrap();
    assert!(full.validate(&texture).is_ok());
    let mut view = full;
    view.base_layer = 2;
    view.layer_count = 2;
    view.base_mip_level = 1;
    view.level_count = 2;
    assert!(view.validate(&texture).is_ok());
    for invalid in [
        TextureViewDesc {
            layer_count: 0,
            ..view
        },
        TextureViewDesc {
            level_count: 0,
            ..view
        },
        TextureViewDesc {
            base_layer: u32::MAX,
            ..view
        },
        TextureViewDesc {
            base_mip_level: u32::MAX,
            ..view
        },
        TextureViewDesc {
            layer_count: 3,
            ..view
        },
        TextureViewDesc {
            level_count: 3,
            ..view
        },
    ] {
        assert_eq!(
            invalid.validate(&texture).unwrap_err().kind(),
            rhapsody::ErrorKind::InvalidUsage
        );
    }
}

#[test]
fn desc_builders_follow_machine_code() {
    let base = TextureDesc::new_2d(7, 5, vk::Format::R8G8B8A8_UNORM);
    // 0x140bd5410：usage |= SAMPLED(4)，flags |= 0x108。
    let d = base.sampled_with_different_format();
    assert_eq!(d.usage.as_raw(), 4);
    assert_eq!(d.flags.as_raw(), 0x108);
    // 0x140bd55a0：sample_count 写原始数值。
    assert_eq!(base.sample_count(4).samples(), vk::SampleCountFlags::TYPE_4);
    // 0x140bd5730 half_res：三个维度都是 x - (x>>1)，0 → 1。depth 也减半。
    let h = TextureDesc::new_3d(7, 5, 9, vk::Format::R8_UNORM).half_res();
    assert_eq!((h.width, h.height, h.depth), (4, 3, 5));
    let h2 = TextureDesc::new_2d(1, 1, vk::Format::R8_UNORM).half_res();
    assert_eq!((h2.width, h2.height, h2.depth), (1, 1, 1));
    // 0x140bd55e0 div_up_extent：ceil，至少 1；0x140bd56a0 div_extent：floor，至少 1。
    let e = TextureDesc::new_3d(17, 3, 1, vk::Format::R8_UNORM);
    let up = e.div_up_extent_by([8, 8, 1]).unwrap();
    assert_eq!((up.width, up.height, up.depth), (3, 1, 1));
    let dn = e.div_extent_by([8, 8, 2]).unwrap();
    assert_eq!(
        (dn.width, dn.height, dn.depth),
        (2, 1, 1),
        "floor(3/8)=0 → 1，floor(1/2)=0 → 1"
    );
    // 0x140bd54a0：max(bitlen(w), bitlen(h), bitlen(depth))。
    assert_eq!(
        TextureDesc::new_3d(4, 4, 64, vk::Format::R8_UNORM)
            .all_mip_levels()
            .mip_levels,
        7
    );
    assert_eq!(
        TextureDesc::new_2d(1024, 512, vk::Format::R8_UNORM)
            .all_mip_levels()
            .mip_levels,
        11
    );
    // 0x140bd5540：只有 kind ∈ {1,3,6} 可以 array_elements > 1（掩码 0x4a）。
    for k in 0..7u8 {
        let kind = TextureKind::from_raw(k).unwrap();
        assert_eq!(kind.is_array(), (0x4au32 >> k) & 1 != 0);
    }
}

#[test]
fn invalid_texture_builder_inputs_return_invalid_usage() {
    let desc = TextureDesc::new_2d(4, 4, vk::Format::R8_UNORM);
    let errors = [
        desc.div_up_extent_by([2, 0, 1]).unwrap_err(),
        desc.div_extent_by([0, 1, 1]).unwrap_err(),
        desc.div_up_extent(0).unwrap_err(),
        desc.div_extent(0).unwrap_err(),
        desc.array_elements(2).unwrap_err(),
        desc.array_elements(0).unwrap_err(),
        TextureDesc::new(desc.extent(), TextureKind::CubeArray, desc.format)
            .array_elements(u32::MAX)
            .unwrap_err(),
    ];
    for error in errors {
        assert_eq!(error.kind(), rhapsody::ErrorKind::InvalidUsage);
    }
}

#[test]
fn malformed_public_layer_fields_are_rejected_by_every_descriptor_entry() {
    let mut desc = TextureDesc::new(
        vk::Extent3D {
            width: 4,
            height: 4,
            depth: 1,
        },
        TextureKind::CubeArray,
        vk::Format::R8G8B8A8_UNORM,
    )
    .sampled();
    desc.array_elements = u32::MAX / 6;
    assert_eq!(desc.array_layers().unwrap(), (u32::MAX / 6) * 6);
    desc.array_elements += 1;
    for error in [
        desc.array_layers().unwrap_err(),
        desc.validate().unwrap_err(),
        get_image_create_info_with_data(&desc, false).unwrap_err(),
        rhapsody::resource::texture::TextureViewDesc::new(&desc).unwrap_err(),
    ] {
        assert_eq!(error.kind(), rhapsody::ErrorKind::InvalidUsage);
    }
    desc.array_elements = 0;
    assert_eq!(
        desc.array_layers().unwrap_err().kind(),
        rhapsody::ErrorKind::InvalidUsage
    );
}

#[test]
fn optimal_image_creation_uses_staging_compatible_layout_and_expected_shape() {
    let f = vk::Format::R8G8B8A8_UNORM;
    // Initial pixels are staged after creation; OPTIMAL images always begin UNDEFINED.
    let d2 = TextureDesc::new_2d(16, 8, f).sampled();
    assert_eq!(
        get_image_create_info_with_data(&d2, false)
            .unwrap()
            .initial_layout,
        vk::ImageLayout::UNDEFINED
    );
    assert_eq!(
        get_image_create_info_with_data(&d2, true)
            .unwrap()
            .initial_layout,
        vk::ImageLayout::UNDEFINED
    );
    // extent 随 kind：1D → (w,1,1)；2D → (w,h,1)；3D → (w,h,d)。
    let d1 = TextureDesc::new(
        vk::Extent3D {
            width: 9,
            height: 7,
            depth: 5,
        },
        TextureKind::D1,
        f,
    );
    let e = get_image_create_info_with_data(&d1, false).unwrap().extent;
    assert_eq!((e.width, e.height, e.depth), (9, 1, 1));
    let e = get_image_create_info_with_data(&d2, false).unwrap().extent;
    assert_eq!((e.width, e.height, e.depth), (16, 8, 1));
    let d3 = TextureDesc::new_3d(4, 5, 6, f);
    let info3 = get_image_create_info_with_data(&d3, false).unwrap();
    assert_eq!(
        (info3.extent.depth, info3.image_type),
        (6, vk::ImageType::TYPE_3D)
    );
    // Cube = 6 层，CubeArray = n*6。
    let cube = TextureDesc::new(
        vk::Extent3D {
            width: 4,
            height: 4,
            depth: 1,
        },
        TextureKind::Cube,
        f,
    );
    assert_eq!(
        get_image_create_info_with_data(&cube, false)
            .unwrap()
            .array_layers,
        6
    );
    let carr = TextureDesc::new(
        vk::Extent3D {
            width: 4,
            height: 4,
            depth: 1,
        },
        TextureKind::CubeArray,
        f,
    )
    .array_elements(3)
    .unwrap();
    assert_eq!(
        get_image_create_info_with_data(&carr, false)
            .unwrap()
            .array_layers,
        18
    );
    // 其余固定字段：tiling OPTIMAL、sharing EXCLUSIVE、sType 14。
    assert_eq!(info3.tiling, vk::ImageTiling::OPTIMAL);
    assert_eq!(info3.sharing_mode, vk::SharingMode::EXCLUSIVE);
    assert_eq!(info3.s_type.as_raw(), 14);
}

#[test]
fn aspect_mask_boundaries() {
    // 边界：format-0x7c > 6 → COLOR。
    let aspect = |raw: i32| image_aspect_mask_from_format(vk::Format::from_raw(raw)).as_raw();
    assert_eq!(aspect(0), 1);
    assert_eq!(aspect(0x7b), 1);
    assert_eq!(aspect(0x7c), 2); // D16
    assert_eq!(aspect(0x7d), 2); // X8_D24
    assert_eq!(aspect(0x7e), 2); // D32
    assert_eq!(aspect(0x7f), 4); // S8
    assert_eq!(aspect(0x80), 6);
    assert_eq!(aspect(0x82), 6); // D32S8
    assert_eq!(aspect(0x83), 1);
}

// ---------------------------------------------------------------- 需要 Vulkan 的行为

#[test]
fn create_buffer_initial_data_paths() {
    let Some(gd) = device() else { return };
    // 目标可映射：直接 memcpy，且只在有数据时加 TRANSFER_DST。
    let desc = BufferDesc::new_cpu_to_gpu(32, vk::BufferUsageFlags::STORAGE_BUFFER);
    let no_data = gd.create_buffer(&desc, "plain", ()).unwrap();
    assert!(!no_data.usage.contains(vk::BufferUsageFlags::TRANSFER_DST));
    let init: &[u8] = &[1, 2, 3, 4];
    let mapped = gd.create_buffer(&desc, "mapped", init).unwrap();
    assert!(mapped.usage.contains(vk::BufferUsageFlags::TRANSFER_DST));
    assert_eq!(
        &unsafe { gd.buffer_mapped_slice_mut(&mapped) }.unwrap()[..4],
        init
    );
    // 数据超过 desc.size：断言失败。
    let too_big = vec![0u8; 33];
    assert!(gd.create_buffer(&desc, "big", too_big.as_slice()).is_err());
    // GpuOnly：走暂存 + setup 命令缓冲，缓冲本身不可映射。
    let gpu = BufferDesc::new_gpu_only(16, vk::BufferUsageFlags::STORAGE_BUFFER);
    let g = gd.create_buffer(&gpu, "gpu", init).unwrap();
    // lavapipe/UMA 的 DEVICE_LOCAL 内存同时 HOST_VISIBLE，所以只断言“有无映射 ⇔ 有无切片”。
    assert_eq!(
        unsafe { gd.buffer_mapped_slice_mut(&g) }.is_none(),
        g.persistent_host_ptr().is_none()
    );
}

#[test]
fn mapped_slice_is_full_length_and_follows_slot_pointer() {
    let Some(gd) = device() else { return };
    let cpu = BufferDesc::new_cpu_to_gpu(48, vk::BufferUsageFlags::UNIFORM_BUFFER);
    let b = gd.create_buffer(&cpu, "m", ()).unwrap();
    let slice = unsafe { gd.buffer_mapped_slice_mut(&b) }.unwrap();
    assert_eq!(slice.len(), 48, "全长 = desc.size");
    slice.fill(0x5a);
    let mut out = [0u8; 48];
    b.read_bytes(0, &mut out).unwrap();
    assert!(out.iter().all(|&v| v == 0x5a));
    let gpu = BufferDesc::new_gpu_only(48, vk::BufferUsageFlags::UNIFORM_BUFFER);
    let g = gd.create_buffer(&gpu, "g", ()).unwrap();
    assert_eq!(
        unsafe { gd.buffer_mapped_slice_mut(&g) }.is_none(),
        g.persistent_host_ptr().is_none()
    );
}

#[test]
fn canonical_managed_host_locations_support_visible_roundtrips() {
    let Some(gd) = device() else { return };
    for location in [MemoryLocation::CpuToGpu, MemoryLocation::GpuToCpu, MemoryLocation::CpuToGpuLowPrio] {
        let desc = BufferDesc::new(64, vk::BufferUsageFlags::TRANSFER_SRC, location);
        let buffer = gd.create_buffer(&desc, "managed mapped roundtrip", ()).unwrap();
        assert!(buffer.persistent_host_ptr().is_some());
        assert_eq!(buffer.size, 64);
        buffer.write_bytes(0, &[37; 64]).unwrap();
        let mut bytes = [0; 64];
        buffer.read_bytes(0, &mut bytes).unwrap();
        assert_eq!(bytes, [37; 64]);
    }
}

#[test]
fn update_buffer_bytes_requires_transfer_dst_and_always_stages() {
    let Some(gd) = device() else { return };
    // 没有 TRANSFER_DST 的缓冲：原版 panic（"must be created with ... TRANSFER_DST"），这里 Err，且不写入。
    let desc = BufferDesc::new_cpu_to_gpu(16, vk::BufferUsageFlags::STORAGE_BUFFER);
    let plain = gd.create_buffer(&desc, "plain", ()).unwrap();
    assert!(gd.update_buffer_bytes(&plain, &[9, 9], &[0..2], 0).is_err());
    let mut out = [0u8; 16];
    plain.read_bytes(0, &mut out).unwrap();
    assert_eq!(out, [0u8; 16]);
    // 有 TRANSFER_DST：多区间按 {累计 src, start+offset} 写入；空更新（合计 0）被拒绝。
    let dst = BufferDesc::new_cpu_to_gpu(
        16,
        vk::BufferUsageFlags::STORAGE_BUFFER | vk::BufferUsageFlags::TRANSFER_DST,
    );
    let b = gd.create_buffer(&dst, "dst", ()).unwrap();
    gd.update_buffer_bytes(&b, &[1, 2, 3, 4, 5, 6], &[0..2, 4..6], 4)
        .unwrap();
    b.read_bytes(0, &mut out).unwrap();
    assert_eq!(&out[4..6], &[1, 2]);
    assert_eq!(&out[8..10], &[5, 6]);
    assert!(gd.update_buffer_bytes(&b, &[1], &[], 0).is_err());
    assert!(gd.update_buffer_bytes(&b, &[1], &[0..0], 0).is_err());
    // GpuOnly + TRANSFER_DST 也能更新（暂存路径）。
    let gpu = BufferDesc::new_gpu_only(
        16,
        vk::BufferUsageFlags::STORAGE_BUFFER | vk::BufferUsageFlags::TRANSFER_DST,
    );
    let g = gd.create_buffer(&gpu, "gpu", ()).unwrap();
    gd.update_buffer_bytes(&g, &[7, 7, 7, 7], &[0..4], 8)
        .unwrap();
}

#[test]
fn buffer_device_address_is_forwarded_every_time() {
    let Some(gd) = device() else { return };
    let desc = BufferDesc::new_gpu_only(
        64,
        vk::BufferUsageFlags::STORAGE_BUFFER | vk::BufferUsageFlags::SHADER_DEVICE_ADDRESS,
    );
    let Ok(buffer) = gd.create_buffer(&desc, "bda", ()) else {
        return;
    };
    let a = gd.buffer_device_address(&buffer).unwrap();
    assert_ne!(a, 0);
    assert_eq!(a, gd.buffer_device_address(&buffer).unwrap());
    // 另一设备的句柄被拒绝（设备代号校验）。
    if let Some(other) = device() {
        assert!(other.buffer_device_address(&buffer).is_err());
    }
}

#[test]
fn create_texture_zeroed_and_sampled_gate() {
    let Some(gd) = device() else { return };
    let f = vk::Format::R8G8B8A8_UNORM;
    let desc = TextureDesc::new_2d(8, 8, f).mip_levels(3).sampled();
    let t = gd.create_texture_zeroed(&desc, "zero").unwrap();
    assert!(t.desc.usage.contains(vk::ImageUsageFlags::TRANSFER_DST));
    // 深度模板走 vkCmdClearDepthStencilImage。
    let ds = TextureDesc::new_2d(8, 8, vk::Format::D32_SFLOAT)
        .sampled()
        .depth_stencil_attachment();
    gd.create_texture_zeroed(&ds, "ds").unwrap();
    // 没有 SAMPLED 的纹理不能带初始数据（`desc.usage.contains(SAMPLED)` 断言）。
    let storage = TextureDesc::new_2d(4, 4, f).storage();
    assert!(gd.create_texture(&storage, "bad", &[0u8; 64][..]).is_err());
    assert!(gd.create_texture_zeroed(&storage, "bad").is_err());
    // 带线性数据：沿用既有单 mip 单层路径。
    let ok = TextureDesc::new_2d(4, 4, f).sampled();
    gd.create_texture(&ok, "data", &[1u8; 64][..]).unwrap();
    // 空数据断言（!data.is_empty()）在 create_texture 里被当作“无初始数据”，不会进入上传。
    gd.create_texture(&ok, "empty", &[][..]).unwrap();
}
