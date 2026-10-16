//! 第二批：纹理初值上传的剩余分支（`init_texture_data` 2/3 号目标 + `update_texture_impl`）。
//! 纯函数部分不需要设备；GPU 部分上传已知数据后经 `vkCmdCopyImageToBuffer` 读回逐字节比对（lavapipe 可跑）。
use ash::vk;
use rhapsody::ErrorKind;
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

fn device() -> Option<Arc<GraphicsDevice>> {
    create_headless_device().ok().map(GraphicsDevice::from_device)
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
fn linear_regions_reject_wrong_data_sizes_as_invalid_usage() {
    let ok = vec![0u8; 40];
    assert!(linear_regions(&ok, [4, 2, 1], 2, 1, 4, [1, 1]).is_ok());
    let invalid = |r: rhapsody::Result<Vec<TextureUpdateRegion<'_>>>| {
        assert_eq!(r.unwrap_err().kind(), ErrorKind::InvalidUsage);
    };
    // 少一个字节、多一个字节、空数据。
    invalid(linear_regions(&ok[..39], [4, 2, 1], 2, 1, 4, [1, 1]));
    invalid(linear_regions(&vec![0u8; 41], [4, 2, 1], 2, 1, 4, [1, 1]));
    invalid(linear_regions(&[], [4, 2, 1], 1, 1, 4, [1, 1]));
    // 层数 >= 0x10000、块字节数 >= 0x100、零尺寸。
    invalid(linear_regions(&ok, [4, 2, 1], 1, 0x10000, 4, [1, 1]));
    invalid(linear_regions(&ok, [4, 2, 1], 1, 1, 0x100, [1, 1]));
    invalid(linear_regions(&ok, [0, 2, 1], 1, 1, 4, [1, 1]));
    // mip_levels 大于可用 mip：迭代器提前结束，数据照实消耗。
    let buf = vec![0u8; 32 + 8 + 4];
    let r = linear_regions(&buf, [4, 2, 1], 9, 1, 4, [1, 1]).unwrap();
    assert_eq!(r.len(), 3);
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
fn copy_plan_packs_tightly_and_rejects_misalignment_and_empty_total() {
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
    assert_eq!(
        plan_texture_copies(&regions, 32, vk::ImageAspectFlags::COLOR).unwrap_err().kind(),
        ErrorKind::InvalidUsage
    );
    // 总长 0。
    let empty = [TextureUpdateRegion { data: &[], extent: [1, 1, 1], mip_level: 0, array_layer: 0 }];
    assert_eq!(
        plan_texture_copies(&empty, 1, vk::ImageAspectFlags::COLOR).unwrap_err().kind(),
        ErrorKind::InvalidUsage
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
    // 所有 BC 格式都按格式表的块字节数切片（含原版块字节表漏掉的 BC1_RGBA、BC2、BC4_SNORM、BC6H）。
    for (format, block_bytes) in [
        (vk::Format::BC1_RGB_UNORM_BLOCK, 8usize),
        (vk::Format::BC1_RGBA_UNORM_BLOCK, 8),
        (vk::Format::BC2_UNORM_BLOCK, 16),
        (vk::Format::BC3_UNORM_BLOCK, 16),
        (vk::Format::BC4_SNORM_BLOCK, 8),
        (vk::Format::BC6H_UFLOAT_BLOCK, 16),
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
fn gpu_cube_linear_upload_writes_all_six_faces() {
    let Some(gd) = device() else { return };
    let desc = TextureDesc::new_cube(4, vk::Format::R8G8B8A8_UNORM).usage(readable());
    let faces: Vec<Vec<u8>> = (0..6).map(|face| pattern(64, 10 + face as u8)).collect();
    let data = faces.concat();
    let t = gd
        .create_texture_with_data(&desc, "cube", TextureInitialData::Linear(&data))
        .unwrap();
    for (face, chunk) in faces.iter().enumerate() {
        assert_eq!(read_back(&gd, &t, 0, face as u32, [4, 4, 1], 64), *chunk, "face {face}");
    }
    // 只给一个面的数据是不足。
    assert_eq!(
        gd.create_texture_with_data(&desc, "cube-short", TextureInitialData::Linear(&faces[0]))
            .unwrap_err()
            .kind(),
        ErrorKind::InvalidUsage
    );
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
