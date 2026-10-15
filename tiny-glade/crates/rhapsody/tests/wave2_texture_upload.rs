//! 第二批：纹理初值上传的剩余分支（`init_texture_data` 2/3 号目标 + `update_texture_impl`）。
//! 纯函数部分不需要设备；GPU 部分上传已知数据后经 `vkCmdCopyImageToBuffer` 读回逐字节比对（lavapipe 可跑）。
use ash::vk;
use rhapsody::raw::{
    device::GraphicsDevice,
    init::create_headless_device,
    setup::TextureInitialData,
    texture::TextureDesc,
    texture_upload::{
        linear_regions, per_mip_regions, plan_texture_copies, MipChainIter, MipLevelItem,
        TextureUpdateRegion,
    },
};
use std::sync::Arc;

const EXE: &str = "/home/ubuntu/test/games_test/tiny-glade.exe";

fn device() -> Option<Arc<GraphicsDevice>> {
    create_headless_device().ok().map(GraphicsDevice::from_device)
}

// ---------------------------------------------------------------- 原 EXE 锚点

fn exe_bytes(va: u64, len: usize) -> Option<Vec<u8>> {
    let data = std::fs::read(EXE).ok()?;
    let u32_at = |o: usize| u32::from_le_bytes(data[o..o + 4].try_into().unwrap());
    let u16_at = |o: usize| u16::from_le_bytes(data[o..o + 2].try_into().unwrap());
    let pe = u32_at(0x3c) as usize;
    let count = u16_at(pe + 6) as usize;
    let opt = u16_at(pe + 20) as usize;
    let base = u64::from_le_bytes(data[pe + 48..pe + 56].try_into().unwrap());
    let rva = (va - base) as u32;
    let mut o = pe + 24 + opt;
    for _ in 0..count {
        let (vs, sva, rs, rp) = (u32_at(o + 8), u32_at(o + 12), u32_at(o + 16), u32_at(o + 20));
        if rva >= sva && rva < sva + vs.max(rs) {
            let off = (rp + rva - sva) as usize;
            return Some(data[off..off + len].to_vec());
        }
        o += 40;
    }
    None
}

#[test]
fn panic_strings_and_names_match_original_binary() {
    let Some(_) = exe_bytes(0x142b0fb20, 1) else { return };
    for (va, text) in [
        (0x142b0fb20u64, "assertion failed: bytes_per_block > 0"),
        (0x142b0fb60, "assertion failed: block[0] > 0"),
        (0x142b0fb98, "assertion failed: block[1] > 0"),
        (0x142b0fbd0, "assertion failed: extent[0] > 0"),
        (0x142b0fc08, "assertion failed: extent[1] > 0"),
        (0x142b0fc40, "assertion failed: extent[2] > 0"),
        (0x142b0fd20, "Texture initial data underflow"),
        (0x142b0f830, "assertion failed: size > 0"),
        (0x142b17090, "TODO"),
        (0x142b17168, "texture initial data"),
        (0x142b1717c, "mapped texture staging buffer memory"),
        (0x142b0fae0, "assertion failed: !data.is_empty()"),
    ] {
        assert_eq!(exe_bytes(va, text.len()).unwrap(), text.as_bytes(), "{va:#x}");
    }
    // 数据过多的断言文案是一整条连续字符串（0x142b0fd70 起，75B）。
    let full = "Texture initial data has incorrect size; more bytes supplied than necessary";
    assert_eq!(exe_bytes(0x142b0fd70, full.len()).unwrap(), full.as_bytes());
}

#[test]
fn region_table_and_copy_closure_machine_code_anchors() {
    let Some(_) = exe_bytes(0x1418b4110, 1) else { return };
    // 0x1418b4080 逐区域闭包：`imul r15,r9,0x38`（0x38B 区域）、`add r14,0x30`（0x30B 表项）。
    assert_eq!(exe_bytes(0x1418b40f8, 4).unwrap(), [0x4d, 0x6b, 0xf9, 0x38]);
    assert_eq!(exe_bytes(0x1418b41e5, 4).unwrap(), [0x49, 0x83, 0xc6, 0x30]);
    // 0x140c08830 命令闭包：屏障访问码 0x34（TransferWrite），布局 7（TRANSFER_DST_OPTIMAL）：
    // `mov byte [rsp+0x47],0x34` 与 `mov r9d,7`。
    assert_eq!(exe_bytes(0x140c088d8, 5).unwrap(), [0xc6, 0x44, 0x24, 0x47, 0x34]);
    assert_eq!(exe_bytes(0x140c08988, 6).unwrap(), [0x41, 0xb9, 7, 0, 0, 0]);
    // 0x140bfe47d / 0x140bfe482：update_texture_impl 的两个访问码实参 0x22（目标态）与 0（初态）。
    assert_eq!(exe_bytes(0x140bfe47d, 10).unwrap(), [0xc6, 0x44, 0x24, 0x40, 0x22, 0xc6, 0x44, 0x24, 0x38, 0x00]);
    // 3 号目标：`array_elements < 0x10000`、`bytes_per_block < 0x100`。
    assert_eq!(exe_bytes(0x140bfe3dd, 7).unwrap(), [0x41, 0x81, 0xfc, 0, 0, 1, 0]);
    assert_eq!(exe_bytes(0x140bfe3ea, 7).unwrap(), [0x41, 0x81, 0xf9, 0, 1, 0, 0]);
    // 2 号目标：`block_bytes(format) == 1` 与 `slice 数 == mip_levels`。
    assert_eq!(exe_bytes(0x140bfe323, 4).unwrap(), [0x48, 0x83, 0xf8, 0x01]);
    assert_eq!(exe_bytes(0x140bfe33a, 3).unwrap(), [0x49, 0x39, 0xc4]);
}

// ---------------------------------------------------------------- MipChainIter / 区域生成（纯函数）

#[test]
fn mip_chain_halves_until_one_by_one_and_stops_early() {
    // 8x4x1 RGBA8：8x4, 4x2, 2x1, 1x1。mip_levels 给大了也只产出到 1x1x1。
    let chain: Vec<MipLevelItem> =
        MipChainIter::new([8, 4, 1], 10, 4, [1, 1]).unwrap().collect();
    let dims: Vec<[u32; 3]> = chain.iter().map(|i| i.extent).collect();
    assert_eq!(dims, [[8, 4, 1], [4, 2, 1], [2, 1, 1], [1, 1, 1]]);
    assert_eq!(chain.iter().map(|i| i.size).collect::<Vec<_>>(), [128, 32, 8, 4]);
    assert_eq!(chain.iter().map(|i| i.mip_level).collect::<Vec<_>>(), [0, 1, 2, 3]);
    // mip_levels 限制。
    assert_eq!(MipChainIter::new([8, 4, 1], 2, 4, [1, 1]).unwrap().count(), 2);
    assert_eq!(MipChainIter::new([8, 4, 1], 0, 4, [1, 1]).unwrap().count(), 0);
    // 3D：深度同样减半。
    let d: Vec<[u32; 3]> =
        MipChainIter::new([4, 4, 4], 8, 1, [1, 1]).unwrap().map(|i| i.extent).collect();
    assert_eq!(d, [[4, 4, 4], [2, 2, 2], [1, 1, 1]]);
}

#[test]
fn mip_chain_block_compressed_sizes_round_up_to_blocks() {
    // BC1 16x8：4x2 块 * 8B = 64；8x4 → 2x1 块 = 16；4x2 → 1 块 = 8；2x1 / 1x1 各 1 块 = 8。
    let sizes: Vec<u64> =
        MipChainIter::new([16, 8, 1], 5, 8, [4, 4]).unwrap().map(|i| i.size).collect();
    assert_eq!(sizes, [64, 16, 8, 8, 8]);
}

#[test]
fn mip_chain_constructor_assertions_in_machine_code_order() {
    let msg = |r: rhapsody::raw::error::Result<MipChainIter>| r.unwrap_err().to_string();
    assert_eq!(msg(MipChainIter::new([4, 4, 1], 1, 0, [1, 1])), "assertion failed: bytes_per_block > 0");
    assert_eq!(msg(MipChainIter::new([4, 4, 1], 1, 4, [0, 1])), "assertion failed: block[0] > 0");
    assert_eq!(msg(MipChainIter::new([4, 4, 1], 1, 4, [1, 0])), "assertion failed: block[1] > 0");
    assert_eq!(msg(MipChainIter::new([0, 4, 1], 1, 4, [1, 1])), "assertion failed: extent[0] > 0");
    assert_eq!(msg(MipChainIter::new([4, 0, 1], 1, 4, [1, 1])), "assertion failed: extent[1] > 0");
    assert_eq!(msg(MipChainIter::new([4, 4, 0], 1, 4, [1, 1])), "assertion failed: extent[2] > 0");
    // 优先级：bytes_per_block 先于 extent。
    assert_eq!(msg(MipChainIter::new([0, 0, 0], 1, 0, [0, 0])), "assertion failed: bytes_per_block > 0");
    // extent >= 0x10000 → u16::try_from(..).unwrap() 失败。
    assert!(MipChainIter::new([0x10000, 1, 1], 1, 4, [1, 1]).is_err());
    assert!(MipChainIter::new([0xffff, 1, 1], 1, 4, [1, 1]).is_ok());
}

#[test]
fn linear_regions_are_layer_major_then_mip() {
    // 4x2 RGBA8，2 个 mip（4x2=32B, 2x1=8B），3 层：每层 40B，共 120B。
    let data: Vec<u8> = (0..120).map(|i| i as u8).collect();
    let regions = linear_regions(&data, [4, 2, 1], 2, 3, 4, [1, 1]).unwrap();
    let order: Vec<(u32, u32, usize, [u32; 3])> = regions
        .iter()
        .map(|r| (r.array_layer, r.mip_level, r.data.len(), r.extent))
        .collect();
    assert_eq!(
        order,
        [
            (0, 0, 32, [4, 2, 1]),
            (0, 1, 8, [2, 1, 1]),
            (1, 0, 32, [4, 2, 1]),
            (1, 1, 8, [2, 1, 1]),
            (2, 0, 32, [4, 2, 1]),
            (2, 1, 8, [2, 1, 1]),
        ]
    );
    // 数据从前往后依次切，无缝紧排。
    assert_eq!(regions[0].data[0], 0);
    assert_eq!(regions[1].data[0], 32);
    assert_eq!(regions[2].data[0], 40);
    assert_eq!(regions[5].data[7], 119);
}

#[test]
fn linear_regions_data_size_checks_match_original_messages() {
    let ok = vec![0u8; 40];
    assert!(linear_regions(&ok, [4, 2, 1], 2, 1, 4, [1, 1]).is_ok());
    // 少一个字节：underflow。
    let err = linear_regions(&ok[..39], [4, 2, 1], 2, 1, 4, [1, 1]).unwrap_err().to_string();
    assert_eq!(err, "Texture initial data underflow");
    // 多一个字节：断言文案。
    let long = vec![0u8; 41];
    let err = linear_regions(&long, [4, 2, 1], 2, 1, 4, [1, 1]).unwrap_err().to_string();
    assert_eq!(err, "Texture initial data has incorrect size; more bytes supplied than necessary");
    // 空数据。
    assert_eq!(
        linear_regions(&[], [4, 2, 1], 1, 1, 4, [1, 1]).unwrap_err().to_string(),
        "assertion failed: !data.is_empty()"
    );
    // array_elements >= 0x10000、bytes_per_block >= 0x100 都是 unwrap 失败。
    assert!(linear_regions(&ok, [4, 2, 1], 1, 0x10000, 4, [1, 1]).is_err());
    assert!(linear_regions(&ok, [4, 2, 1], 1, 1, 0x100, [1, 1]).is_err());
    // mip_levels 大于可用 mip：迭代器提前结束，数据照实消耗。
    let buf = vec![0u8; 32 + 8 + 4];
    let r = linear_regions(&buf, [4, 2, 1], 9, 1, 4, [1, 1]).unwrap();
    assert_eq!(r.len(), 3);
}

#[test]
fn cube_uses_array_elements_not_vulkan_layers() {
    // 原版层数取 desc.array_elements（Cube 恒为 1），所以只切出一层。
    let desc = TextureDesc::new_cube(4, vk::Format::R8G8B8A8_UNORM);
    assert_eq!(desc.array_layers(), 6);
    let data = vec![0u8; 64];
    let r = linear_regions(&data, [4, 4, 1], 1, desc.array_elements, 4, [1, 1]).unwrap();
    assert_eq!(r.len(), 1);
}

#[test]
fn per_mip_regions_require_matching_slice_count_and_zip_with_chain() {
    let a = [1u8; 32];
    let b = [2u8; 8];
    let c = [3u8; 4];
    let slices: [&[u8]; 3] = [&a, &b, &c];
    let r = per_mip_regions(&slices, [4, 2, 1], 3, |_| None).unwrap();
    assert_eq!(
        r.iter().map(|r| (r.mip_level, r.extent, r.array_layer, r.data.len())).collect::<Vec<_>>(),
        [(0, [4, 2, 1], 0, 32), (1, [2, 1, 1], 0, 8), (2, [1, 1, 1], 0, 4)]
    );
    assert!(per_mip_regions(&slices[..2], [4, 2, 1], 3, |_| None).is_err());
    // 额外的最小长度校验（有意偏差）。
    assert!(per_mip_regions(&slices, [4, 2, 1], 3, |_| Some(1000)).is_err());
}

#[test]
fn copy_plan_packs_tightly_and_asserts_alignment_and_nonzero_total() {
    let a = [0u8; 16];
    let b = [0u8; 8];
    let regions = [
        TextureUpdateRegion { data: &a, extent: [4, 4, 1], mip_level: 0, array_layer: 1 },
        TextureUpdateRegion { data: &b, extent: [2, 2, 1], mip_level: 1, array_layer: 1 },
    ];
    let plan = plan_texture_copies(&regions, 8, vk::ImageAspectFlags::COLOR).unwrap();
    assert_eq!(plan.total, 24);
    assert_eq!(plan.copies[0].buffer_offset, 0);
    assert_eq!(plan.copies[1].buffer_offset, 16);
    for c in &plan.copies {
        assert_eq!((c.buffer_row_length, c.buffer_image_height), (0, 0));
        assert_eq!((c.image_offset.x, c.image_offset.y, c.image_offset.z), (0, 0, 0));
        assert_eq!(c.image_subresource.layer_count, 1);
        assert_eq!(c.image_subresource.base_array_layer, 1);
    }
    assert_eq!(plan.copies[1].image_subresource.mip_level, 1);
    assert_eq!(plan.copies[1].image_extent.width, 2);
    // 偏移 16 对 block_bytes=16 OK，对 32 不是整数倍（第二块起点 16 % 32 != 0）。
    assert!(plan_texture_copies(&regions, 16, vk::ImageAspectFlags::COLOR).is_ok());
    assert!(plan_texture_copies(&regions, 32, vk::ImageAspectFlags::COLOR).is_err());
    // 总长 0：`assert!(size > 0)`。
    let empty = [TextureUpdateRegion { data: &[], extent: [1, 1, 1], mip_level: 0, array_layer: 0 }];
    assert_eq!(
        plan_texture_copies(&empty, 1, vk::ImageAspectFlags::COLOR).unwrap_err().to_string(),
        "assertion failed: size > 0"
    );
}

// ---------------------------------------------------------------- GPU 读回

/// 把（mip, layer）的子资源读回成连续字节：屏障 → `vkCmdCopyImageToBuffer` → 读缓冲。
fn read_back(
    gd: &GraphicsDevice,
    texture: &rhapsody::raw::texture::Texture,
    mip: u32,
    layer: u32,
    extent: [u32; 3],
    bytes: u64,
) -> Vec<u8> {
    use rhapsody::raw::buffer::{Buffer, BufferDesc};
    let buffer = Buffer::new(
        gd.inner.clone(),
        &BufferDesc::new_gpu_to_cpu(bytes, vk::BufferUsageFlags::TRANSFER_DST),
    )
    .unwrap();
    let range = vk::ImageSubresourceRange::default()
        .aspect_mask(vk::ImageAspectFlags::COLOR)
        .level_count(vk::REMAINING_MIP_LEVELS)
        .layer_count(vk::REMAINING_ARRAY_LAYERS);
    gd.with_setup_cb(|cb| unsafe {
        let device = &gd.inner.device;
        let to_src = [vk::ImageMemoryBarrier::default()
            .src_access_mask(vk::AccessFlags::SHADER_READ)
            .dst_access_mask(vk::AccessFlags::TRANSFER_READ)
            .old_layout(vk::ImageLayout::SHADER_READ_ONLY_OPTIMAL)
            .new_layout(vk::ImageLayout::TRANSFER_SRC_OPTIMAL)
            .image(texture.handle)
            .subresource_range(range)];
        device.cmd_pipeline_barrier(
            cb,
            vk::PipelineStageFlags::ALL_COMMANDS,
            vk::PipelineStageFlags::TRANSFER,
            vk::DependencyFlags::empty(),
            &[],
            &[],
            &to_src,
        );
        let region = [vk::BufferImageCopy::default()
            .image_subresource(vk::ImageSubresourceLayers {
                aspect_mask: vk::ImageAspectFlags::COLOR,
                mip_level: mip,
                base_array_layer: layer,
                layer_count: 1,
            })
            .image_extent(vk::Extent3D { width: extent[0], height: extent[1], depth: extent[2] })];
        device.cmd_copy_image_to_buffer(
            cb,
            texture.handle,
            vk::ImageLayout::TRANSFER_SRC_OPTIMAL,
            buffer.handle,
            &region,
        );
        let back = [vk::ImageMemoryBarrier::default()
            .src_access_mask(vk::AccessFlags::TRANSFER_READ)
            .dst_access_mask(vk::AccessFlags::SHADER_READ)
            .old_layout(vk::ImageLayout::TRANSFER_SRC_OPTIMAL)
            .new_layout(vk::ImageLayout::SHADER_READ_ONLY_OPTIMAL)
            .image(texture.handle)
            .subresource_range(range)];
        device.cmd_pipeline_barrier(
            cb,
            vk::PipelineStageFlags::TRANSFER,
            vk::PipelineStageFlags::ALL_COMMANDS,
            vk::DependencyFlags::empty(),
            &[],
            &[],
            &back,
        );
    })
    .unwrap();
    let mut out = vec![0u8; bytes as usize];
    buffer.read_bytes(0, &mut out).unwrap();
    out
}

fn pattern(len: usize, seed: u8) -> Vec<u8> {
    (0..len).map(|i| (i as u8).wrapping_mul(31).wrapping_add(seed)).collect()
}

fn readable() -> vk::ImageUsageFlags {
    vk::ImageUsageFlags::SAMPLED | vk::ImageUsageFlags::TRANSFER_SRC
}

#[test]
fn gpu_multi_mip_multi_layer_linear_upload_reads_back_exactly() {
    let Some(gd) = device() else { return };
    let f = vk::Format::R8G8B8A8_UNORM;
    // 8x4 RGBA8，3 个 mip（8x4, 4x2, 2x1），4 层。
    let desc = TextureDesc::new_2d_array(8, 4, 4, f).mip_levels(3).usage(readable());
    let mips: [([u32; 3], usize); 3] = [([8, 4, 1], 128), ([4, 2, 1], 32), ([2, 1, 1], 8)];
    let mut data = Vec::new();
    let mut expect = Vec::new();
    for layer in 0..4u32 {
        for (mip, (extent, len)) in mips.iter().enumerate() {
            let chunk = pattern(*len, (layer * 16 + mip as u32) as u8);
            data.extend_from_slice(&chunk);
            expect.push((mip as u32, layer, *extent, chunk));
        }
    }
    let t = gd
        .create_texture_with_data(&desc, "multi", TextureInitialData::Linear(&data))
        .unwrap();
    for (mip, layer, extent, chunk) in expect {
        let got = read_back(&gd, &t, mip, layer, extent, chunk.len() as u64);
        assert_eq!(got, chunk, "mip {mip} layer {layer}");
    }
}

#[test]
fn gpu_create_texture_with_plain_slice_is_linear_and_rejects_wrong_sizes() {
    let Some(gd) = device() else { return };
    let f = vk::Format::R8G8B8A8_UNORM;
    let desc = TextureDesc::new_2d(4, 4, f).mip_levels(3).usage(readable());
    let full = pattern(64 + 16 + 4, 7);
    let t = gd.create_texture(&desc, "linear", &full[..]).unwrap();
    assert_eq!(read_back(&gd, &t, 1, 0, [2, 2, 1], 16), full[64..80]);
    assert_eq!(read_back(&gd, &t, 2, 0, [1, 1, 1], 4), full[80..84]);
    assert!(gd.create_texture(&desc, "short", &full[..83]).is_err());
    assert!(gd.create_texture(&desc, "long", &pattern(85, 0)[..]).is_err());
}

#[test]
fn gpu_per_mip_slices_upload_reads_back_exactly() {
    let Some(gd) = device() else { return };
    let f = vk::Format::R8G8B8A8_UNORM;
    let desc = TextureDesc::new_2d(8, 8, f).mip_levels(4).usage(readable());
    let sizes = [256usize, 64, 16, 4];
    let chunks: Vec<Vec<u8>> = sizes.iter().enumerate().map(|(i, &n)| pattern(n, 40 + i as u8)).collect();
    let slices: Vec<&[u8]> = chunks.iter().map(Vec::as_slice).collect();
    let t = gd
        .create_texture_with_data(&desc, "permip", TextureInitialData::PerMip(&slices))
        .unwrap();
    let dims = [[8, 8, 1], [4, 4, 1], [2, 2, 1], [1, 1, 1]];
    for (mip, (extent, chunk)) in dims.iter().zip(&chunks).enumerate() {
        assert_eq!(read_back(&gd, &t, mip as u32, 0, *extent, chunk.len() as u64), *chunk, "mip {mip}");
    }
    // 切片数必须等于 mip_levels。
    assert!(gd
        .create_texture_with_data(&desc, "bad", TextureInitialData::PerMip(&slices[..3]))
        .is_err());
    // 过短切片被拒绝（有意偏差）。
    let short = [&chunks[0][..255], &chunks[1][..], &chunks[2][..], &chunks[3][..]];
    assert!(gd
        .create_texture_with_data(&desc, "short", TextureInitialData::PerMip(&short))
        .is_err());
}

#[test]
fn gpu_per_mip_rejects_block_compressed_formats() {
    let Some(gd) = device() else { return };
    let desc = TextureDesc::new_2d(8, 8, vk::Format::BC1_RGB_UNORM_BLOCK).usage(readable());
    if gd.create_texture(&desc, "probe", ()).is_err() {
        return; // 设备不支持 BC：跳过。
    }
    let data = vec![0u8; 32];
    let slices: [&[u8]; 1] = [&data];
    assert!(gd
        .create_texture_with_data(&desc, "bc-permip", TextureInitialData::PerMip(&slices))
        .is_err());
}

#[test]
fn gpu_block_compressed_mip_chain_uploads_and_reads_back_exactly() {
    let Some(gd) = device() else { return };
    // 原 block_bytes 表只放行 BC1_RGB(8B) / BC3(16B) / BC4_UNORM(8B) / BC5(16B) / BC7(16B)。
    for (format, block_bytes) in [
        (vk::Format::BC1_RGB_UNORM_BLOCK, 8usize),
        (vk::Format::BC3_UNORM_BLOCK, 16),
        (vk::Format::BC7_UNORM_BLOCK, 16),
    ] {
        let desc = TextureDesc::new_2d(16, 8, format).mip_levels(3).usage(readable());
        if gd.create_texture(&desc, "probe", ()).is_err() {
            eprintln!("跳过 {format:?}：设备不支持");
            continue; // 设备不支持该 BC 格式：跳过。
        }
        // 16x8：4x2 块；8x4：2x1 块；4x2：1 块。
        let blocks = [8usize, 2, 1];
        let extents = [[16, 8, 1], [8, 4, 1], [4, 2, 1]];
        let chunks: Vec<Vec<u8>> =
            blocks.iter().enumerate().map(|(i, &b)| pattern(b * block_bytes, 90 + i as u8)).collect();
        let data: Vec<u8> = chunks.concat();
        eprintln!("BC 读回：{format:?}");
        let t = gd
            .create_texture_with_data(&desc, "bc", TextureInitialData::Linear(&data))
            .unwrap();
        for (mip, (extent, chunk)) in extents.iter().zip(&chunks).enumerate() {
            let got = read_back(&gd, &t, mip as u32, 0, *extent, chunk.len() as u64);
            assert_eq!(got, *chunk, "{format:?} mip {mip}");
        }
    }
}

#[test]
fn gpu_block_compressed_formats_outside_original_block_bytes_table_are_rejected() {
    let Some(gd) = device() else { return };
    // BC2 / BC1_RGBA / BC4_SNORM / BC6H 不在 0x1417390e0 放行的子集内（原版 panic）。
    for format in [
        vk::Format::BC2_UNORM_BLOCK,
        vk::Format::BC1_RGBA_UNORM_BLOCK,
        vk::Format::BC4_SNORM_BLOCK,
        vk::Format::BC6H_UFLOAT_BLOCK,
    ] {
        let desc = TextureDesc::new_2d(4, 4, format).usage(readable());
        if gd.create_texture(&desc, "probe", ()).is_err() {
            continue;
        }
        let data = vec![0u8; 16];
        assert!(gd.create_texture(&desc, "bad", &data[..]).is_err(), "{format:?}");
    }
}

#[test]
fn gpu_update_texture_regions_overwrites_only_the_given_mip_and_layer() {
    let Some(gd) = device() else { return };
    let f = vk::Format::R8G8B8A8_UNORM;
    let desc = TextureDesc::new_2d_array(4, 4, 2, f).mip_levels(2).usage(readable());
    let base = pattern(2 * (64 + 16), 1);
    let t = gd.create_texture(&desc, "upd", &base[..]).unwrap();
    let patch = pattern(16, 200);
    rhapsody::raw::setup::update_texture_regions(
        &gd.inner,
        &t,
        &[TextureUpdateRegion { data: &patch, extent: [2, 2, 1], mip_level: 1, array_layer: 1 }],
    )
    .unwrap();
    // 被更新的（mip 1, layer 1）变成补丁，其余保持原值。
    assert_eq!(read_back(&gd, &t, 1, 1, [2, 2, 1], 16), patch);
    assert_eq!(read_back(&gd, &t, 1, 0, [2, 2, 1], 16), base[64..80]);
    assert_eq!(read_back(&gd, &t, 0, 1, [4, 4, 1], 64), base[80..144]);
    // update_texture（整张线性）也支持多 mip 多层。
    let all = pattern(2 * (64 + 16), 77);
    gd.update_texture(&t, &all).unwrap();
    assert_eq!(read_back(&gd, &t, 1, 1, [2, 2, 1], 16), all[144..160]);
}

// ---------------------------------------------------------------- create 失败文案

#[test]
fn create_failure_expect_messages_match_original_binary() {
    use rhapsody::raw::{buffer::create_failure as buffer, texture::create_failure as texture};
    let Some(_) = exe_bytes(0x142b157f0, 1) else { return };
    for (va, text) in [
        (0x142b18af8u64, texture::CREATE_IMAGE),
        (0x142b18b70, texture::BIND_IMAGE_MEMORY),
        (0x142b18b38, texture::SET_DEBUG_UTILS_OBJECT_NAME),
        (0x142b157f0, buffer::CREATE_BUFFER),
        (0x142b15888, buffer::BIND_BUFFER_MEMORY),
        (0x142b15850, buffer::SET_DEBUG_UTILS_OBJECT_NAME),
    ] {
        assert_eq!(exe_bytes(va, text.len()).unwrap(), text.as_bytes(), "{va:#x}");
    }
    assert_eq!(
        rhapsody::raw::texture::create_failure_text(texture::CREATE_IMAGE, vk::Result::ERROR_OUT_OF_DEVICE_MEMORY),
        "create_image: ERROR_OUT_OF_DEVICE_MEMORY"
    );
}
