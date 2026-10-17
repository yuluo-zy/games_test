//! setup 命令缓冲与纹理上传（候选）。
//!
//! 原作的 setup 命令缓冲是设备内的独占状态（`device+0x9f8`，锁 `+0x9f0`）：初始化类命令先写进它，
//! 提交前补一条 `ALL_COMMANDS→ALL_COMMANDS` 的全量内存屏障并 end，作为“一条命令、无 wait/signal”的
//! 提交信息排在图命令之前（见 evidence/render-recovery/p4-sync-setup-cb/FINDINGS.md）。
//! 这里保留“结尾全量屏障”的语义，但先提供**同步**版本：录制 → 屏障 → end → 提交 → 等设备时间线。
//! 与原作“并入下一次图提交、不等待”的流水线形态不同，属于已记录的候选偏差。
use super::{
    buffer::{Buffer, BufferDesc},
    command_buffer::{VulkanCommandBuffer, VulkanCommandPool},
    device::Device,
    error::{Error, Result},
    format::vk_format_info,
    resource_access::{buffer_access_borrowed, image_access_borrowed},
    texture::Texture,
    texture_upload::{
        TextureUpdateRegion, copy_format_params, linear_regions, per_mip_regions,
        plan_texture_copies,
    },
};
use crate::resource::range_state::AccessScope;
use ash::vk;
use std::sync::Arc;

/// 录制 `body` 到一条一次性命令缓冲，补全量屏障后提交并等待完成。
pub fn with_setup_resources<R>(
    device: &Arc<Device>,
    resources: &[&dyn crate::resource::GpuUse],
    body: impl FnOnce(vk::CommandBuffer) -> R,
) -> Result<R> {
    with_setup_command(device, |command| {
        for resource in resources {
            // The synchronous setup call retains these borrows until its real wait.
            unsafe {
                command.retain_borrowed_opaque(*resource)?;
            }
        }
        Ok(body(command.handle))
    })
}

/// Record initialization through the same raw range-access API as asynchronous
/// uploads, then submit and wait for this command's real completion point.
pub fn with_setup_command<R>(
    device: &Arc<Device>,
    body: impl FnOnce(&mut VulkanCommandBuffer) -> Result<R>,
) -> Result<R> {
    let pool = VulkanCommandPool::new(Arc::clone(device))?;
    let mut command = pool.allocate()?;
    command.begin()?;
    let result = body(&mut command)?;
    let barrier = vk::MemoryBarrier::default()
        .src_access_mask(vk::AccessFlags::MEMORY_READ | vk::AccessFlags::MEMORY_WRITE)
        .dst_access_mask(vk::AccessFlags::MEMORY_READ | vk::AccessFlags::MEMORY_WRITE);
    let mut scope = super::barrier::SynchronizationScope::new(
        vk::PipelineStageFlags::ALL_COMMANDS,
        vk::PipelineStageFlags::ALL_COMMANDS,
    );
    scope.memory.push(barrier);
    unsafe {
        super::barrier::BarrierDispatch::new(device).issue(device, command.handle, &scope)?;
    }
    command.end()?;
    let pending = command.submit()?;
    pending.wait(u64::MAX)?;
    Ok(result)
}

/// 原 `TextureInitialData` 判别（`init_texture_data` 跳表 0x142b174fc 的索引，`*param_4`）。
/// 0 号目标是 `unreachable!()`（texture.rs 0x288:44，"internal error: entered unreachable code"）——
/// `create_texture` 只在标签非 0 时才调用 `init_texture_data`，所以没有对应变体。
/// 变体名是候选（原名未从 PDB 取得）。
#[derive(Clone, Copy, Debug)]
pub enum TextureInitialData<'a> {
    /// 1 号目标（0x140bfe2c0）：整张图清零，见 [`init_texture_zero_fill`]。
    Zeroed,
    /// 2 号目标（0x140bfe30b）：每个 mip 一块数据，仅限非压缩格式
    /// （`0x1417390e0(format) == 1`，否则 `assert_eq!(.., 1, "TODO")`），切片数必须等于 `desc.mip_levels`。
    PerMip(&'a [&'a [u8]]),
    /// 3 号目标（0x140bfe3aa）：层主序、mip 次序的紧排线性数据，支持块压缩格式。
    Linear(&'a [u8]),
}

/// 原 `update_texture_impl`（0x140bfeac0）：把区域表上传到纹理。
/// 暂存缓冲 `{size = Σ region.len, usage = TRANSFER_SRC, alignment 0, memory_location = 3}`，
/// 名 `"texture initial data"`（0x142b17168，20B；这里没有设备命名面，不命名），区域依次紧排，
/// 经 setup 命令缓冲：整图（level/layer 全量）屏障 → `vkCmdCopyBufferToImage`（TRANSFER_DST_OPTIMAL）→ 屏障
/// （0x140c08830）。同步版本，见模块说明。
pub fn upload_regions(
    device: &Arc<Device>,
    texture: &Texture,
    regions: &[TextureUpdateRegion<'_>],
    old_layout: vk::ImageLayout,
) -> Result<()> {
    if regions.is_empty()
        || !texture
            .desc
            .usage
            .contains(vk::ImageUsageFlags::TRANSFER_DST)
        || texture.desc.samples() != vk::SampleCountFlags::TYPE_1
    {
        return Err(Error::invalid_usage(
            "texture upload requires regions and a single-sampled transfer destination",
        ));
    }
    let (block_bytes, aspect) = copy_format_params(texture.desc.format)?;
    if !matches!(
        old_layout,
        vk::ImageLayout::UNDEFINED | vk::ImageLayout::SHADER_READ_ONLY_OPTIMAL
    ) {
        return Err(Error::invalid_usage(
            "texture upload old layout is unsupported",
        ));
    }
    let info = vk_format_info(texture.desc.format)
        .ok_or_else(|| Error::unsupported("texture format has no copy layout"))?;
    let layers = texture.desc.array_layers()?;
    for region in regions {
        if region.mip_level >= u32::from(texture.desc.mip_levels) || region.array_layer >= layers {
            return Err(Error::invalid_usage(
                "texture upload subresource is out of range",
            ));
        }
        let full = [texture.desc.width, texture.desc.height, texture.desc.depth]
            .map(|size| (size >> region.mip_level).max(1));
        if region.extent.contains(&0)
            || region.extent.iter().zip(full).zip(info.block_extent).any(
                |((&size, full), block)| size > full || size != full && !size.is_multiple_of(block),
            )
        {
            return Err(Error::invalid_usage(
                "texture upload extent violates texel-block boundaries",
            ));
        }
        let required = region
            .extent
            .iter()
            .zip(info.block_extent)
            .try_fold(u64::from(info.block_bytes), |bytes, (&size, block)| {
                bytes.checked_mul(u64::from(size.div_ceil(block)))
            })
            .ok_or_else(|| Error::invalid_usage("texture upload size overflows"))?;
        if region.data.len() as u64 != required {
            return Err(Error::invalid_usage(
                "texture upload byte count differs from its region",
            ));
        }
    }
    let plan = plan_texture_copies(regions, block_bytes, aspect)?;
    let staging = Buffer::new(
        Arc::clone(device),
        &BufferDesc::new_cpu_to_gpu_low_prio(plan.total, vk::BufferUsageFlags::TRANSFER_SRC),
    )?;
    // 暂存缓冲刚创建，GPU 尚未访问，写入无需等待在飞工作。
    for (region, copy) in regions.iter().zip(&plan.copies) {
        staging.write_bytes_unsynchronized(copy.buffer_offset, region.data)?;
    }
    let queue = device.queue(crate::device::QueueClass::Graphics)?;
    let range = Texture::ownership_range(&texture.desc)?;
    with_setup_command(device, |command| {
        for (region, copy) in regions.iter().zip(&plan.copies) {
            buffer_access_borrowed(
                command,
                &staging,
                queue,
                copy.buffer_offset,
                region.data.len() as u64,
                AccessScope {
                    stages: vk::PipelineStageFlags::TRANSFER,
                    access: vk::AccessFlags::TRANSFER_READ,
                    write: false,
                },
            )?;
        }
        image_access_borrowed(
            command,
            texture,
            queue,
            range,
            AccessScope {
                stages: vk::PipelineStageFlags::TRANSFER,
                access: vk::AccessFlags::TRANSFER_WRITE,
                write: true,
            },
            vk::ImageLayout::TRANSFER_DST_OPTIMAL,
            Some(old_layout),
        )?;
        unsafe {
            device.device.cmd_copy_buffer_to_image(
                command.handle,
                staging.handle,
                texture.handle,
                vk::ImageLayout::TRANSFER_DST_OPTIMAL,
                &plan.copies,
            );
        }
        image_access_borrowed(
            command,
            texture,
            queue,
            range,
            AccessScope {
                stages: vk::PipelineStageFlags::ALL_COMMANDS,
                access: vk::AccessFlags::SHADER_READ,
                write: false,
            },
            vk::ImageLayout::SHADER_READ_ONLY_OPTIMAL,
            Some(vk::ImageLayout::TRANSFER_DST_OPTIMAL),
        )
    })
}

/// 线性数据（3 号目标）的区域生成，参数取自纹理描述与原格式表 `vk_format_info`（0x141739060）：
/// `{bytes_per_block = word04, block = (byte08, byte09)}`。
fn linear_regions_for<'a>(
    texture: &Texture,
    bytes: &'a [u8],
) -> Result<Vec<TextureUpdateRegion<'a>>> {
    let desc = &texture.desc;
    let info = vk_format_info(desc.format)
        .ok_or_else(|| Error::unsupported("texture format has no copy layout"))?;
    copy_format_params(desc.format)?;
    let block = [
        u8::try_from(info.block_extent[0])
            .map_err(|_| Error::unsupported("copy block width is unsupported"))?,
        u8::try_from(info.block_extent[1])
            .map_err(|_| Error::unsupported("copy block height is unsupported"))?,
    ];
    linear_regions(
        bytes,
        [desc.width, desc.height, desc.depth],
        desc.mip_levels,
        desc.array_layers()?,
        info.block_bytes,
        block,
    )
}

/// 2 号目标的区域生成：非压缩格式（`block_bytes == 1`）才允许。
fn per_mip_regions_for<'a>(
    texture: &Texture,
    slices: &[&'a [u8]],
) -> Result<Vec<TextureUpdateRegion<'a>>> {
    let desc = &texture.desc;
    let info = vk_format_info(desc.format)
        .ok_or_else(|| Error::unsupported("texture format has no copy layout"))?;
    if info.block_extent != [1, 1, 1] || desc.array_layers()? != 1 {
        return Err(Error::unsupported(
            "per-mip slices require an uncompressed single layer; use linear data for compressed/cube/array textures",
        ));
    }
    let texel = vk_format_info(desc.format)
        .filter(|info| info.aspect == vk::ImageAspectFlags::COLOR)
        .map(|info| u64::from(info.block_bytes));
    per_mip_regions(
        slices,
        [desc.width, desc.height, desc.depth],
        desc.mip_levels,
        |item| {
            texel.map(|bytes| {
                u64::from(item.extent[0])
                    * u64::from(item.extent[1])
                    * u64::from(item.extent[2])
                    * bytes
            })
        },
    )
}

/// `init_texture_data`（0x140bfe270）入口断言 `desc.usage.contains(vk::ImageUsageFlags::SAMPLED)`
/// （`test byte [desc], 4`，0x142b16ff8）；失败在原版是 panic，这里返回错误。
fn require_sampled(texture: &Texture) -> Result<()> {
    if texture.desc.usage.contains(vk::ImageUsageFlags::SAMPLED) {
        Ok(())
    } else {
        Err(Error::invalid_usage(
            "assertion failed: desc.usage.contains(vk::ImageUsageFlags::SAMPLED)",
        ))
    }
}

/// 首次上传（`TextureInitialData::Linear`，3 号目标）：图像当前布局为 UNDEFINED，完成后进入
/// SHADER_READ_ONLY_OPTIMAL。多 mip、多层、块压缩纹理按原版 `MipChainIter` / 层主序切片。
pub fn init_texture_data(device: &Arc<Device>, texture: &Texture, bytes: &[u8]) -> Result<()> {
    init_texture_data_with(device, texture, TextureInitialData::Linear(bytes))
}

/// `init_texture_data`（0x140bfe270）的完整分派：先 `desc.usage.contains(SAMPLED)`，再按
/// `TextureInitialData` 判别走 1/2/3 号目标。
pub fn init_texture_data_with(
    device: &Arc<Device>,
    texture: &Texture,
    data: TextureInitialData<'_>,
) -> Result<()> {
    require_sampled(texture)?;
    match data {
        TextureInitialData::Zeroed => init_texture_zero_fill(device, texture),
        TextureInitialData::PerMip(slices) => {
            let regions = per_mip_regions_for(texture, slices)?;
            upload_regions(device, texture, &regions, vk::ImageLayout::UNDEFINED)
        }
        TextureInitialData::Linear(bytes) => {
            let regions = linear_regions_for(texture, bytes)?;
            upload_regions(device, texture, &regions, vk::ImageLayout::UNDEFINED)
        }
    }
}

/// `init_texture_data` 跳表 1 号目标（0x140bfe2c0，闭包 0x140c08560）：**整张图清零**。
/// 机器码：`aspect = 0x141738fe0(format)`（格式 → aspect 表 0x142ccd890）；setup 命令缓冲里先做
/// 访问码 `0 → 0x34(TransferWrite)` 的整图屏障，`aspect & 6 == 0` 用 `vkCmdClearColorImage`
/// （`device+0x3a8`）否则 `vkCmdClearDepthStencilImage`（`+0x3b0`），布局 7（TRANSFER_DST_OPTIMAL），
/// 清除值全 0，范围 `{aspect, mip 0.., level_count = desc.mip_levels, layer 0.., layer_count}`，
/// 最后 `0x34 → 0x22` 屏障。
///
/// 偏差：原版 `layer_count` 取 `desc.array_elements`，这里取 `array_layers()`（cube 才不会漏面）。
pub fn init_texture_zero_fill(device: &Arc<Device>, texture: &Texture) -> Result<()> {
    require_sampled(texture)?;
    let queue = device.queue(crate::device::QueueClass::Graphics)?;
    with_setup_command(device, |command| {
        super::texture_upload::record_zero_fill_borrowed(
            command,
            texture,
            queue,
            Some(vk::ImageLayout::UNDEFINED),
        )
    })
}

/// 更新已初始化纹理：前提是图像处于 SHADER_READ_ONLY_OPTIMAL（`init_texture_data` 之后的静止布局）。
/// `bytes` 与首次上传的线性数据同形（整张纹理、层主序、mip 次序）；缺少 TRANSFER_DST 用途直接拒绝
/// （原版 `update_texture_impl` 不查，Vulkan 要求）。
pub fn update_texture(device: &Arc<Device>, texture: &Texture, bytes: &[u8]) -> Result<()> {
    if !texture
        .desc
        .usage
        .contains(vk::ImageUsageFlags::TRANSFER_DST)
    {
        return Err(Error::invalid_usage("纹理缺少 TRANSFER_DST 用途"));
    }
    let regions = linear_regions_for(texture, bytes)?;
    upload_regions(
        device,
        texture,
        &regions,
        vk::ImageLayout::SHADER_READ_ONLY_OPTIMAL,
    )
}

/// 用显式区域表更新已初始化纹理（`update_texture_impl` 的直接入口，旧布局 SHADER_READ_ONLY_OPTIMAL）。
pub fn update_texture_regions(
    device: &Arc<Device>,
    texture: &Texture,
    regions: &[TextureUpdateRegion<'_>],
) -> Result<()> {
    if !texture
        .desc
        .usage
        .contains(vk::ImageUsageFlags::TRANSFER_DST)
    {
        return Err(Error::invalid_usage("纹理缺少 TRANSFER_DST 用途"));
    }
    upload_regions(
        device,
        texture,
        regions,
        vk::ImageLayout::SHADER_READ_ONLY_OPTIMAL,
    )
}
