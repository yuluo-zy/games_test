//! raw-core 缓冲/纹理域测试：布局偏移、判据、边界；有原 EXE 时直接读机器码表作 oracle。
//! 需要 Vulkan 的用例在无设备时跳过；本机可用 lavapipe。
use ash::vk;
use rhapsody::raw::{
    buffer::{
        buffer_alignment, plan_buffer_update, BufferAccessError, BufferDesc, BufferRecord,
        BufferRegion, BufferSlotView, BufferUpdateError, MemoryLocation, SLOT_ALLOCATION_TAKEN,
    },
    device::GraphicsDevice,
    format::{observed, vk_format_info},
    init::create_headless_device,
    texture::{
        get_image_create_info_with_data, image_aspect_mask_from_format, OptionU32, TextureDesc,
        TextureKind, TextureRecord, TextureViewDescRaw,
    },
};
use std::mem::{offset_of, size_of};

const EXE: &str = "/home/ubuntu/test/games_test/tiny-glade.exe";

/// 最小 PE 读取：按 RVA 读原 EXE 的 .rdata 字节；EXE 不存在时返回 None（测试跳过）。
struct Pe {
    data: Vec<u8>,
    base: u64,
    sections: Vec<(u32, u32, u32)>,
}
impl Pe {
    fn open() -> Option<Self> {
        let data = std::fs::read(EXE).ok()?;
        let u32_at = |o: usize| u32::from_le_bytes(data[o..o + 4].try_into().unwrap());
        let u16_at = |o: usize| u16::from_le_bytes(data[o..o + 2].try_into().unwrap());
        let pe = u32_at(0x3c) as usize;
        let count = u16_at(pe + 6) as usize;
        let opt = u16_at(pe + 20) as usize;
        let base = u64::from_le_bytes(data[pe + 48..pe + 56].try_into().unwrap());
        let mut sections = Vec::new();
        let mut o = pe + 24 + opt;
        for _ in 0..count {
            let vs = u32_at(o + 8);
            let va = u32_at(o + 12);
            let rs = u32_at(o + 16);
            let rp = u32_at(o + 20);
            sections.push((va, vs.max(rs), rp));
            o += 40;
        }
        Some(Self { data, base, sections })
    }
    fn read(&self, va: u64, len: usize) -> &[u8] {
        let rva = (va - self.base) as u32;
        for &(sva, size, raw) in &self.sections {
            if rva >= sva && rva < sva + size {
                let off = (raw + rva - sva) as usize;
                return &self.data[off..off + len];
            }
        }
        panic!("VA {va:#x} 不在任何节内");
    }
    fn u32s(&self, va: u64, n: usize) -> Vec<u32> {
        self.read(va, n * 4)
            .chunks(4)
            .map(|c| u32::from_le_bytes(c.try_into().unwrap()))
            .collect()
    }
}

fn device() -> Option<std::sync::Arc<GraphicsDevice>> {
    create_headless_device().ok().map(GraphicsDevice::from_device)
}

// ---------------------------------------------------------------- Buffer：布局与判据

fn record(device_id: u16, ready: u8, offset: u32, size: u64, usage: u32) -> BufferRecord {
    BufferRecord {
        ctx: 0,
        slot: 0,
        offset,
        device_id,
        ready,
        _pad: 0,
        size,
        desc_tail: u64::from(usage) | (2u64 << 48),
    }
}
fn slot(offset: u32, capacity: u64, usage: u32) -> BufferSlotView {
    BufferSlotView {
        offset,
        capacity,
        desc_tail: u64::from(usage),
        vk_buffer: vk::Buffer::null(),
        mapped_ptr: std::ptr::null_mut(),
        block_size: capacity,
        allocation_state: 0,
    }
}

#[test]
fn alignment_is_max_of_requirement_desc_and_usage_floor() {
    // 0x140bee200：usage & 0x400 抬到 0x40，否则 0x10；再与 memreq.alignment、desc.alignment 取大。
    assert_eq!(buffer_alignment(4, 0, 0), 0x10);
    assert_eq!(buffer_alignment(4, 0, 0x400), 0x40);
    assert_eq!(buffer_alignment(0x100, 0x20, 0x400), 0x100);
    assert_eq!(buffer_alignment(4, 0x80, 0), 0x80);
    assert_eq!(buffer_alignment(0x10, 0, 0x20 | 0x2), 0x10, "非 0x400 位不抬高");
}

#[test]
fn buffer_handle_checks_follow_original_order() {
    let s = slot(7, 64, 2);
    // 0x140bd4c00 / 0x140bd4cd0 / 0x140beefa0：设备代号 → flag==1 → 槽偏移。
    assert_eq!(
        record(1, 0, 9, 64, 2).validate(2, &s),
        Err(BufferAccessError::DeviceMismatch { handle: 1, device: 2 }),
        "设备代号先于 flag 与偏移"
    );
    assert_eq!(
        record(2, 0, 9, 64, 2).validate(2, &s),
        Err(BufferAccessError::NotReady { flag: 0 }),
        "flag 先于偏移"
    );
    assert_eq!(
        record(2, 1, 9, 64, 2).validate(2, &s),
        Err(BufferAccessError::StaleSlot { handle: 9, slot: 7 })
    );
    assert_eq!(record(2, 1, 7, 64, 2).validate(2, &s), Ok(()));
}

#[test]
fn device_address_is_not_cached_and_uses_slot_vk_buffer() {
    use ash::vk::Handle;
    let mut s = slot(7, 64, 2);
    s.vk_buffer = vk::Buffer::from_raw(0xabcd);
    let r = record(3, 1, 7, 64, 2);
    let mut calls = 0;
    let mut query = |b: vk::Buffer| {
        calls += 1;
        b.as_raw() + calls
    };
    assert_eq!(r.device_address(3, &s, &mut query), Ok(0xabcd + 1));
    assert_eq!(r.device_address(3, &s, &mut query), Ok(0xabcd + 2), "每次都走设备函数");
    assert!(r.device_address(4, &s, |_| 0).is_err(), "校验失败不调用设备函数");
}

#[test]
fn mapped_slice_parts_follow_0x140bd4cd0() {
    let mut backing = [0u8; 32];
    let r = record(1, 1, 7, 24, 2);
    let mut s = slot(7, 24, 2);
    // 不可映射：块指针为空 → None（不是错误）。
    assert_eq!(r.mapped_slice_parts(1, &s), Ok(None));
    s.mapped_ptr = backing.as_mut_ptr();
    s.block_size = 32;
    assert_eq!(r.mapped_slice_parts(1, &s), Ok(Some((backing.as_mut_ptr(), 24))), "全长 = Buffer+0x18");
    // 缓冲大小超过块大小：原版 slice_end_index_len_fail。
    s.block_size = 16;
    assert_eq!(
        r.mapped_slice_parts(1, &s),
        Err(BufferAccessError::SizeExceedsBlock { size: 24, block: 16 })
    );
    // inner+0x7c == 2：分配已被取走，原版直接 panic（先于指针检查）。
    s.allocation_state = SLOT_ALLOCATION_TAKEN;
    s.mapped_ptr = std::ptr::null_mut();
    assert_eq!(r.mapped_slice_parts(1, &s), Err(BufferAccessError::AllocationTaken));
}

#[test]
fn update_plan_matches_0x140beefa0_and_closure_0x140c03a00() {
    let usage_dst = vk::BufferUsageFlags::TRANSFER_DST.as_raw();
    let r = record(1, 1, 7, 64, usage_dst);
    let s = slot(7, 64, usage_dst);
    // 区域 = {src 累计, dst = start + dst_offset, size}；暂存大小 = 区间长度之和。
    let plan = plan_buffer_update(&r, 1, &s, 8, &[0..2, 4..6, 6..8], 10).unwrap();
    assert_eq!(plan.total, 6);
    assert_eq!(
        plan.regions,
        vec![
            BufferRegion { src_offset: 0, dst_offset: 10, size: 2 },
            BufferRegion { src_offset: 2, dst_offset: 14, size: 2 },
            BufferRegion { src_offset: 4, dst_offset: 16, size: 2 },
        ]
    );
    // 区间合法性：先 start<=end，再 end<=data.len。
    assert!(matches!(
        plan_buffer_update(&r, 1, &s, 8, &[5..3], 0),
        Err(BufferUpdateError::RangeInverted { start: 5, end: 3 })
    ));
    assert!(matches!(
        plan_buffer_update(&r, 1, &s, 4, &[0..5], 0),
        Err(BufferUpdateError::RangeOutOfData { end: 5, len: 4 })
    ));
    // total + dst_offset 超过容量。
    assert!(matches!(
        plan_buffer_update(&r, 1, &s, 8, &[0..8], 60),
        Err(BufferUpdateError::ExceedsCapacity { total: 8, dst_offset: 60, capacity: 64 })
    ));
    // 稀疏区间：原版只比较 total+offset，这里额外拒绝 dst_end 越界（有意超集）。
    assert!(matches!(
        plan_buffer_update(&r, 1, &s, 64, &[60..62], 8),
        Err(BufferUpdateError::ExceedsCapacity { .. })
    ));
    // inner+0x28 & 2 = usage.TRANSFER_DST；没有它 panic 文案 "must be created with ...TRANSFER_DST"。
    let no_dst = slot(7, 64, vk::BufferUsageFlags::STORAGE_BUFFER.as_raw());
    assert_eq!(
        plan_buffer_update(&r, 1, &no_dst, 8, &[0..2], 0),
        Err(BufferUpdateError::MissingTransferDst)
    );
    // 合计 0：`assert!(size > 0)`；顺序在 TRANSFER_DST 检查之后。
    assert_eq!(plan_buffer_update(&r, 1, &s, 8, &[], 0), Err(BufferUpdateError::Empty));
    assert_eq!(plan_buffer_update(&r, 1, &s, 8, &[3..3], 0), Err(BufferUpdateError::Empty));
    assert_eq!(
        plan_buffer_update(&r, 1, &no_dst, 8, &[], 0),
        Err(BufferUpdateError::MissingTransferDst)
    );
    // 句柄校验先于一切。
    assert!(matches!(
        plan_buffer_update(&r, 2, &s, 8, &[0..2], 0),
        Err(BufferUpdateError::Access(BufferAccessError::DeviceMismatch { .. }))
    ));
}

#[test]
fn buffer_record_stays_40_bytes_and_desc_16() {
    assert_eq!(size_of::<BufferRecord>(), 40);
    assert_eq!(size_of::<BufferDesc>(), 16);
    assert_eq!(offset_of!(BufferDesc, alignment), 0x0c);
}

// ---------------------------------------------------------------- Texture：布局与描述符

#[test]
fn texture_record_is_0x38_with_flag_zero() {
    assert_eq!(size_of::<TextureRecord>(), 0x38);
    assert_eq!(offset_of!(TextureRecord, ctx), 0x00);
    assert_eq!(offset_of!(TextureRecord, slot), 0x08);
    assert_eq!(offset_of!(TextureRecord, offset), 0x10);
    assert_eq!(offset_of!(TextureRecord, device_id), 0x14);
    assert_eq!(offset_of!(TextureRecord, flag), 0x16);
    assert_eq!(offset_of!(TextureRecord, desc), 0x18);
    assert_eq!(size_of::<TextureDesc>(), 0x20);
}

#[test]
fn texture_desc_layout_matches_getters() {
    assert_eq!(offset_of!(TextureDesc, usage), 0x00);
    assert_eq!(offset_of!(TextureDesc, flags), 0x04);
    assert_eq!(offset_of!(TextureDesc, format), 0x08);
    assert_eq!(offset_of!(TextureDesc, width), 0x0c);
    assert_eq!(offset_of!(TextureDesc, height), 0x10);
    assert_eq!(offset_of!(TextureDesc, depth), 0x14);
    assert_eq!(offset_of!(TextureDesc, array_elements), 0x18);
    assert_eq!(offset_of!(TextureDesc, mip_levels), 0x1c);
    assert_eq!(offset_of!(TextureDesc, sample_count), 0x1d);
    assert_eq!(offset_of!(TextureDesc, kind), 0x1e);
}

#[test]
fn texture_view_desc_raw_layout_and_builders() {
    assert_eq!(size_of::<TextureViewDescRaw>(), 0x3c);
    assert_eq!(size_of::<OptionU32>(), 8);
    assert_eq!(offset_of!(TextureViewDescRaw, view_type), 0x00);
    assert_eq!(offset_of!(TextureViewDescRaw, usage), 0x08);
    assert_eq!(offset_of!(TextureViewDescRaw, format), 0x10);
    assert_eq!(offset_of!(TextureViewDescRaw, aspect_mask), 0x18);
    assert_eq!(offset_of!(TextureViewDescRaw, level_count), 0x20);
    assert_eq!(offset_of!(TextureViewDescRaw, layer_count), 0x28);
    assert_eq!(offset_of!(TextureViewDescRaw, base_mip_level), 0x30);
    assert_eq!(offset_of!(TextureViewDescRaw, base_layer), 0x34);
    assert_eq!(offset_of!(TextureViewDescRaw, bind_individual_mips), 0x38);
    let d = TextureViewDescRaw::new().format(37).level_count(3).base_layer(2).bind_individual_mips(true);
    assert_eq!(d.format, OptionU32 { tag: 1, value: 37 });
    assert_eq!(d.level_count, OptionU32 { tag: 1, value: 3 });
    assert_eq!(d.view_type.get(), None);
    assert_eq!((d.base_layer, d.bind_individual_mips), (2, 1));
    // 候选解析：缺省的层数取 base 之后的剩余。
    let tex = TextureDesc::new_2d_array(8, 8, 4, vk::Format::R8G8B8A8_UNORM).mip_levels(3);
    let r = d.resolve(&tex);
    assert_eq!((r.level_count, r.layer_count, r.base_layer), (3, 2, 2));
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
    let up = e.div_up_extent_by([8, 8, 1]);
    assert_eq!((up.width, up.height, up.depth), (3, 1, 1));
    let dn = e.div_extent_by([8, 8, 2]);
    assert_eq!((dn.width, dn.height, dn.depth), (2, 1, 1), "floor(3/8)=0 → 1，floor(1/2)=0 → 1");
    // 0x140bd54a0：max(bitlen(w), bitlen(h), bitlen(depth))。
    assert_eq!(TextureDesc::new_3d(4, 4, 64, vk::Format::R8_UNORM).all_mip_levels().mip_levels, 7);
    assert_eq!(TextureDesc::new_2d(1024, 512, vk::Format::R8_UNORM).all_mip_levels().mip_levels, 11);
    // 0x140bd5540：只有 kind ∈ {1,3,6} 可以 array_elements > 1（掩码 0x4a）。
    for k in 0..7u8 {
        let kind = TextureKind::from_raw(k).unwrap();
        assert_eq!(kind.is_array(), (0x4au32 >> k) & 1 != 0);
    }
}

#[test]
#[should_panic(expected = "divide by zero")]
fn div_up_extent_panics_on_zero_divisor() {
    let _ = TextureDesc::new_2d(4, 4, vk::Format::R8_UNORM).div_up_extent_by([2, 0, 1]);
}

#[test]
fn image_create_info_follows_0x140bd5ed0() {
    let f = vk::Format::R8G8B8A8_UNORM;
    // initialLayout = has_data << 3（PREINITIALIZED = 8）。
    let d2 = TextureDesc::new_2d(16, 8, f).sampled();
    assert_eq!(get_image_create_info_with_data(&d2, false).initial_layout, vk::ImageLayout::UNDEFINED);
    assert_eq!(get_image_create_info_with_data(&d2, true).initial_layout.as_raw(), 8);
    // extent 随 kind：1D → (w,1,1)；2D → (w,h,1)；3D → (w,h,d)。
    let d1 = TextureDesc::new(vk::Extent3D { width: 9, height: 7, depth: 5 }, TextureKind::D1, f);
    let e = get_image_create_info_with_data(&d1, false).extent;
    assert_eq!((e.width, e.height, e.depth), (9, 1, 1));
    let e = get_image_create_info_with_data(&d2, false).extent;
    assert_eq!((e.width, e.height, e.depth), (16, 8, 1));
    let d3 = TextureDesc::new_3d(4, 5, 6, f);
    let info3 = get_image_create_info_with_data(&d3, false);
    assert_eq!((info3.extent.depth, info3.image_type), (6, vk::ImageType::TYPE_3D));
    // Cube = 6 层，CubeArray = n*6。
    let cube = TextureDesc::new(vk::Extent3D { width: 4, height: 4, depth: 1 }, TextureKind::Cube, f);
    assert_eq!(get_image_create_info_with_data(&cube, false).array_layers, 6);
    let carr = TextureDesc::new(vk::Extent3D { width: 4, height: 4, depth: 1 }, TextureKind::CubeArray, f)
        .array_elements(3);
    assert_eq!(get_image_create_info_with_data(&carr, false).array_layers, 18);
    // 其余固定字段：tiling OPTIMAL、sharing EXCLUSIVE、sType 14。
    assert_eq!(info3.tiling, vk::ImageTiling::OPTIMAL);
    assert_eq!(info3.sharing_mode, vk::SharingMode::EXCLUSIVE);
    assert_eq!(info3.s_type.as_raw(), 14);
}

#[test]
fn aspect_mask_table_matches_binary_and_boundaries() {
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
    let Some(pe) = Pe::open() else { return };
    let table = pe.u32s(0x142b0ffb8, 7);
    for (i, &v) in table.iter().enumerate() {
        assert_eq!(aspect(0x7c + i as i32), v, "0x142b0ffb8[{i}]");
    }
    // 机器码：add ecx,-0x7c; cmp ecx,6; ja → mov eax,1。
    assert_eq!(pe.read(0x140bd6630, 11), &[0x83, 0xc1, 0x84, 0xb8, 1, 0, 0, 0, 0x83, 0xf9, 0x06]);
}

#[test]
fn format_tables_equal_original_binary_tables() {
    let Some(pe) = Pe::open() else { return };
    // 0x141738fe0：format-1 < 0xb8 → 表 0x142ccd890（aspect）。
    let aspects = pe.u32s(0x142ccd890, 0xb8);
    for f in 1..=0xb8u32 {
        assert_eq!(
            observed::try_vk_format_valid_aspect_flags(vk::Format::from_raw(f as i32)).unwrap().as_raw(),
            aspects[(f - 1) as usize],
            "aspect 表 format {f}"
        );
    }
    // 0x141739060：format < 0x93 → 三张表（0x142ccdb70 word00 / 0x142ccddbc word04 / 0x142cce008 byte08）。
    let w0 = pe.u32s(0x142ccdb70, 0x93);
    let w4 = pe.u32s(0x142ccddbc, 0x93);
    let b8 = pe.read(0x142cce008, 0x93);
    for f in 0..0x93u32 {
        let info = observed::vk_format_info(vk::Format::from_raw(f as i32));
        assert_eq!(info.word_at_00(), w0[f as usize], "word00 format {f}");
        assert_eq!(info.word_at_04(), w4[f as usize], "word04 format {f}");
        assert_eq!(info.byte_at_08(), b8[f as usize], "byte08 format {f}");
        assert_eq!(info.byte_at_09(), b8[f as usize], "byte09 format {f}");
    }
}

#[test]
fn public_registry_agrees_with_original_block_tables_where_original_has_entries() {
    let Some(pe) = Pe::open() else { return };
    // 原表 word04 = 块字节数，byte08 = 块边长（压缩格式 4，其余 1）；公开 vk.xml 注册表应一致。
    let w4 = pe.u32s(0x142ccddbc, 0x93);
    let b8 = pe.read(0x142cce008, 0x93);
    let mut checked = 0;
    for f in 1..0x93u32 {
        let Some(info) = vk_format_info(vk::Format::from_raw(f as i32)) else { continue };
        // 原表对 D16S8 / D24S8 / D32S8 等深度模板格式记的是 depth 部分的宽度，公开表按整纹素；
        // 只比较色彩格式与压缩格式（aspect == COLOR）。
        if info.aspect != vk::ImageAspectFlags::COLOR {
            continue;
        }
        assert_eq!(info.block_bytes, w4[f as usize], "block_bytes format {f}");
        assert_eq!(info.block_extent[0], u32::from(b8[f as usize]), "block 边长 format {f}");
        checked += 1;
    }
    assert!(checked > 100, "实际比较了 {checked} 个格式");
}

#[test]
fn init_texture_data_machine_code_anchors() {
    let Some(pe) = Pe::open() else { return };
    // 0x140bfe293：test byte [r8], 4 —— usage 必须含 SAMPLED(4)。
    assert_eq!(pe.read(0x140bfe293, 4), &[0x41, 0xf6, 0x00, 0x04]);
    // 跳表 0x142b174fc：4 个有符号偏移，目标 = 表基址 + 偏移。
    let targets: Vec<u64> = pe
        .u32s(0x142b174fc, 4)
        .iter()
        .map(|&o| 0x142b174fcu64.wrapping_add(o as i32 as i64 as u64))
        .collect();
    assert_eq!(targets, vec![0x140bfe5af, 0x140bfe2c0, 0x140bfe30b, 0x140bfe3aa]);
    // create_texture：get_image_create_info 第三参 has_data 写进 +0x50 = has_data << 3。
    assert_eq!(pe.read(0x140bd5f89, 8), &[0x45, 0x0f, 0xb6, 0xc0, 0x41, 0xc1, 0xe0, 0x03]);
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
    assert_eq!(&gd.buffer_mapped_slice_mut(&mapped).unwrap()[..4], init);
    // 数据超过 desc.size：断言失败。
    let too_big = vec![0u8; 33];
    assert!(gd.create_buffer(&desc, "big", too_big.as_slice()).is_err());
    // GpuOnly：走暂存 + setup 命令缓冲，缓冲本身不可映射。
    let gpu = BufferDesc::new_gpu_only(16, vk::BufferUsageFlags::STORAGE_BUFFER);
    let g = gd.create_buffer(&gpu, "gpu", init).unwrap();
    // lavapipe/UMA 的 DEVICE_LOCAL 内存同时 HOST_VISIBLE，所以只断言“有无映射 ⇔ 有无切片”。
    assert_eq!(gd.buffer_mapped_slice_mut(&g).is_none(), g.slot_view(0).mapped_ptr.is_null());
}

#[test]
fn mapped_slice_is_full_length_and_follows_slot_pointer() {
    let Some(gd) = device() else { return };
    let cpu = BufferDesc::new_cpu_to_gpu(48, vk::BufferUsageFlags::UNIFORM_BUFFER);
    let b = gd.create_buffer(&cpu, "m", ()).unwrap();
    let slice = gd.buffer_mapped_slice_mut(&b).unwrap();
    assert_eq!(slice.len(), 48, "全长 = desc.size");
    slice.fill(0x5a);
    let mut out = [0u8; 48];
    b.read_bytes(0, &mut out).unwrap();
    assert!(out.iter().all(|&v| v == 0x5a));
    let gpu = BufferDesc::new_gpu_only(48, vk::BufferUsageFlags::UNIFORM_BUFFER);
    let g = gd.create_buffer(&gpu, "g", ()).unwrap();
    assert_eq!(gd.buffer_mapped_slice_mut(&g).is_none(), g.slot_view(0).mapped_ptr.is_null());
}

#[test]
fn create_mapped_buffer_requires_host_visible_locations() {
    let Some(gd) = device() else { return };
    for location in [MemoryLocation::CpuToGpu, MemoryLocation::GpuToCpu] {
        let desc = BufferDesc::new(64, vk::BufferUsageFlags::TRANSFER_SRC, location);
        let (buffer, parts) = gd.create_mapped_buffer(&desc, "mapped").unwrap();
        assert_eq!(parts.len, 64);
        assert!(!parts.ptr.is_null());
        assert_eq!(buffer.size, 64);
    }
    let low = BufferDesc::new_cpu_to_gpu_low_prio(8, vk::BufferUsageFlags::TRANSFER_SRC);
    let gpu = BufferDesc::new_gpu_only(8, vk::BufferUsageFlags::TRANSFER_SRC);
    for bad in [low, gpu] {
        let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
            let _ = gd.create_mapped_buffer(&bad, "bad");
        }));
        assert!(result.is_err(), "memory_location ∉ {{2,4}} 必须断言失败");
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
    let dst = BufferDesc::new_cpu_to_gpu(16, vk::BufferUsageFlags::STORAGE_BUFFER | vk::BufferUsageFlags::TRANSFER_DST);
    let b = gd.create_buffer(&dst, "dst", ()).unwrap();
    gd.update_buffer_bytes(&b, &[1, 2, 3, 4, 5, 6], &[0..2, 4..6], 4).unwrap();
    b.read_bytes(0, &mut out).unwrap();
    assert_eq!(&out[4..6], &[1, 2]);
    assert_eq!(&out[8..10], &[5, 6]);
    assert!(gd.update_buffer_bytes(&b, &[1], &[], 0).is_err());
    assert!(gd.update_buffer_bytes(&b, &[1], &[0..0], 0).is_err());
    // GpuOnly + TRANSFER_DST 也能更新（暂存路径）。
    let gpu = BufferDesc::new_gpu_only(16, vk::BufferUsageFlags::STORAGE_BUFFER | vk::BufferUsageFlags::TRANSFER_DST);
    let g = gd.create_buffer(&gpu, "gpu", ()).unwrap();
    gd.update_buffer_bytes(&g, &[7, 7, 7, 7], &[0..4], 8).unwrap();
}

#[test]
fn buffer_device_address_is_forwarded_every_time() {
    let Some(gd) = device() else { return };
    let desc = BufferDesc::new_gpu_only(
        64,
        vk::BufferUsageFlags::STORAGE_BUFFER | vk::BufferUsageFlags::SHADER_DEVICE_ADDRESS,
    );
    let Ok(buffer) = gd.create_buffer(&desc, "bda", ()) else { return };
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
    let ds = TextureDesc::new_2d(8, 8, vk::Format::D32_SFLOAT).sampled().depth_stencil_attachment();
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
