//! setup 命令缓冲与纹理上传（候选）。
//!
//! 原作的 setup 命令缓冲是设备内的独占状态（`device+0x9f8`，锁 `+0x9f0`）：初始化类命令先写进它，
//! 提交前补一条 `ALL_COMMANDS→ALL_COMMANDS` 的全量内存屏障并 end，作为“一条命令、无 wait/signal”的
//! 提交信息排在图命令之前（见 evidence/render-recovery/p4-sync-setup-cb/FINDINGS.md）。
//! 这里保留“结尾全量屏障”的语义，但先提供**同步**版本：录制 → 屏障 → end → 提交 → 等 fence。
//! 与原作“并入下一次图提交、不等待”的流水线形态不同，属于已记录的候选偏差。
use super::{
    buffer::{Buffer, BufferDesc},
    command_buffer::VulkanCommandPool,
    device::VulkanDevice,
    error::{Error, Result},
    format::{observed, vk_format_info},
    texture::Texture,
    texture_upload::{
        TextureUpdateRegion, copy_format_params, linear_regions, per_mip_regions,
        plan_texture_copies,
    },
};
use ash::vk;
use std::sync::Arc;

/// 访问码（`access_table`，原 0x140c10dd0 三张表）驱动的整图屏障：与原 `update_texture_impl` /
/// zero-fill 闭包里 `get_image_memory_barrier` 的用法一致。三个码：`0` Nothing（无 stage/access，
/// layout 0）、`0x34` TransferWrite（TRANSFER/TRANSFER_WRITE/TRANSFER_DST_OPTIMAL）、
/// `0x22`（ALL_COMMANDS/SHADER_READ/SHADER_READ_ONLY_OPTIMAL，即公开版 AnyShaderReadSampledImage）。
pub(crate) fn code_image_barrier(
    previous: u8,
    next: u8,
    image: vk::Image,
    range: vk::ImageSubresourceRange,
) -> (
    vk::PipelineStageFlags,
    vk::PipelineStageFlags,
    vk::ImageMemoryBarrier<'static>,
) {
    use super::access_table::{BarrierLayout, ImageBarrier, get_image_memory_barrier};
    get_image_memory_barrier(&ImageBarrier {
        previous: &[previous],
        next: &[next],
        previous_layout: BarrierLayout::Optimal,
        next_layout: BarrierLayout::Optimal,
        discard_contents: false,
        src_queue_family_index: vk::QUEUE_FAMILY_IGNORED,
        dst_queue_family_index: vk::QUEUE_FAMILY_IGNORED,
        image,
        range,
    })
}

fn cmd_code_barrier(
    device: &VulkanDevice,
    cb: vk::CommandBuffer,
    (src, dst, barrier): (
        vk::PipelineStageFlags,
        vk::PipelineStageFlags,
        vk::ImageMemoryBarrier<'static>,
    ),
) {
    unsafe {
        device.device.cmd_pipeline_barrier(
            cb,
            src,
            dst,
            vk::DependencyFlags::empty(),
            &[],
            &[],
            &[barrier],
        );
    }
}

/// 录制 `body` 到一条一次性命令缓冲，补全量屏障后提交并等待完成。
pub fn with_setup_cb<R>(
    device: &Arc<VulkanDevice>,
    body: impl FnOnce(vk::CommandBuffer) -> R,
) -> Result<R> {
    let pool = VulkanCommandPool::new(Arc::clone(device))?;
    let mut command = pool.allocate()?;
    command.begin()?;
    let result = body(command.handle);
    let barrier = [vk::MemoryBarrier::default()
        .src_access_mask(vk::AccessFlags::MEMORY_READ | vk::AccessFlags::MEMORY_WRITE)
        .dst_access_mask(vk::AccessFlags::MEMORY_READ | vk::AccessFlags::MEMORY_WRITE)];
    unsafe {
        device.device.cmd_pipeline_barrier(
            command.handle,
            vk::PipelineStageFlags::ALL_COMMANDS,
            vk::PipelineStageFlags::ALL_COMMANDS,
            vk::DependencyFlags::empty(),
            &barrier,
            &[],
            &[],
        );
    }
    command.end()?;
    let fence = unsafe {
        device
            .device
            .create_fence(&vk::FenceCreateInfo::default(), None)?
    };
    let submitted = device.submit_prepared(
        device.queue,
        &[super::submit::PreparedSubmit {
            commands: vec![command.handle],
            ..Default::default()
        }],
        fence,
    );
    let waited = submitted.and_then(|_| {
        unsafe { device.device.wait_for_fences(&[fence], true, u64::MAX) }.map_err(Error::from)
    });
    unsafe { device.device.destroy_fence(fence, None) };
    waited?;
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
    device: &Arc<VulkanDevice>,
    texture: &Texture,
    regions: &[TextureUpdateRegion<'_>],
    old_layout: vk::ImageLayout,
) -> Result<()> {
    let (block_bytes, aspect) = copy_format_params(texture.desc.format)?;
    let plan = plan_texture_copies(regions, block_bytes, aspect)?;
    let staging = Buffer::new(
        Arc::clone(device),
        &BufferDesc::new_cpu_to_gpu_low_prio(plan.total, vk::BufferUsageFlags::TRANSFER_SRC),
    )?;
    // 暂存缓冲刚创建，GPU 尚未访问，写入无需等待在飞工作。
    let mut offset = 0u64;
    for region in regions {
        staging.write_bytes_unsynchronized(offset, region.data)?;
        offset += region.data.len() as u64;
    }
    let range = vk::ImageSubresourceRange::default()
        .aspect_mask(aspect)
        .base_mip_level(0)
        .level_count(vk::REMAINING_MIP_LEVELS)
        .base_array_layer(0)
        .layer_count(vk::REMAINING_ARRAY_LAYERS);
    with_setup_cb(device, |cb| unsafe {
        // 访问码 old(0 或 0x22) → 0x34；old 非 UNDEFINED 时用 0x22（静止布局 SHADER_READ_ONLY_OPTIMAL）。
        let old_code = if old_layout == vk::ImageLayout::UNDEFINED {
            0
        } else {
            0x22
        };
        cmd_code_barrier(
            device,
            cb,
            code_image_barrier(old_code, 0x34, texture.handle, range),
        );
        device.device.cmd_copy_buffer_to_image(
            cb,
            staging.handle,
            texture.handle,
            vk::ImageLayout::TRANSFER_DST_OPTIMAL,
            &plan.copies,
        );
        cmd_code_barrier(
            device,
            cb,
            code_image_barrier(0x34, 0x22, texture.handle, range),
        );
    })
}

/// 线性数据（3 号目标）的区域生成，参数取自纹理描述与原格式表 `vk_format_info`（0x141739060）：
/// `{bytes_per_block = word04, block = (byte08, byte09)}`。
fn linear_regions_for<'a>(
    texture: &Texture,
    bytes: &'a [u8],
) -> Result<Vec<TextureUpdateRegion<'a>>> {
    let desc = &texture.desc;
    let info = observed::try_vk_format_info(desc.format)
        .map_err(|_| Error::unsupported("原构建不支持该格式的 info 查询"))?;
    linear_regions(
        bytes,
        [desc.width, desc.height, desc.depth],
        desc.mip_levels,
        desc.array_elements,
        info.word_at_04(),
        [info.byte_at_08(), info.byte_at_09()],
    )
}

/// 2 号目标的区域生成：非压缩格式（`block_bytes == 1`）才允许。
fn per_mip_regions_for<'a>(
    texture: &Texture,
    slices: &[&'a [u8]],
) -> Result<Vec<TextureUpdateRegion<'a>>> {
    let desc = &texture.desc;
    let block_bytes = observed::try_vk_format_block_bytes(desc.format)
        .map_err(|_| Error::unsupported("原构建不支持该格式的 block_bytes 查询"))?;
    if block_bytes != 1 {
        return Err(Error::invalid_usage(
            "assertion `left == right` failed: TODO（逐 mip 数据不支持块压缩格式）",
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
pub fn init_texture_data(
    device: &Arc<VulkanDevice>,
    texture: &Texture,
    bytes: &[u8],
) -> Result<()> {
    init_texture_data_with(device, texture, TextureInitialData::Linear(bytes))
}

/// `init_texture_data`（0x140bfe270）的完整分派：先 `desc.usage.contains(SAMPLED)`，再按
/// `TextureInitialData` 判别走 1/2/3 号目标。
pub fn init_texture_data_with(
    device: &Arc<VulkanDevice>,
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
pub fn init_texture_zero_fill(device: &Arc<VulkanDevice>, texture: &Texture) -> Result<()> {
    require_sampled(texture)?;
    let desc = &texture.desc;
    let aspect = vk_format_info(desc.format)
        .map(|info| info.aspect)
        .unwrap_or(vk::ImageAspectFlags::COLOR);
    let range = vk::ImageSubresourceRange::default()
        .aspect_mask(aspect)
        .base_mip_level(0)
        .level_count(u32::from(desc.mip_levels))
        .base_array_layer(0)
        .layer_count(desc.array_layers());
    with_setup_cb(device, |cb| unsafe {
        cmd_code_barrier(
            device,
            cb,
            code_image_barrier(0, 0x34, texture.handle, range),
        );
        if !aspect.intersects(vk::ImageAspectFlags::DEPTH | vk::ImageAspectFlags::STENCIL) {
            device.device.cmd_clear_color_image(
                cb,
                texture.handle,
                vk::ImageLayout::TRANSFER_DST_OPTIMAL,
                &vk::ClearColorValue::default(),
                &[range],
            );
        } else {
            device.device.cmd_clear_depth_stencil_image(
                cb,
                texture.handle,
                vk::ImageLayout::TRANSFER_DST_OPTIMAL,
                &vk::ClearDepthStencilValue::default(),
                &[range],
            );
        }
        cmd_code_barrier(
            device,
            cb,
            code_image_barrier(0x34, 0x22, texture.handle, range),
        );
    })
}

/// 更新已初始化纹理：前提是图像处于 SHADER_READ_ONLY_OPTIMAL（`init_texture_data` 之后的静止布局）。
/// `bytes` 与首次上传的线性数据同形（整张纹理、层主序、mip 次序）；缺少 TRANSFER_DST 用途直接拒绝
/// （原版 `update_texture_impl` 不查，Vulkan 要求）。
pub fn update_texture(device: &Arc<VulkanDevice>, texture: &Texture, bytes: &[u8]) -> Result<()> {
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
    device: &Arc<VulkanDevice>,
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

#[cfg(test)]
mod access_code_tests {
    use super::*;

    fn range() -> vk::ImageSubresourceRange {
        vk::ImageSubresourceRange::default()
            .aspect_mask(vk::ImageAspectFlags::COLOR)
            .level_count(vk::REMAINING_MIP_LEVELS)
            .layer_count(vk::REMAINING_ARRAY_LAYERS)
    }

    /// 访问码 0 → 0x34：TOP_OF_PIPE → TRANSFER，UNDEFINED → TRANSFER_DST_OPTIMAL，src access 空。
    #[test]
    fn code_0_to_0x34_is_undefined_to_transfer_dst() {
        let (src, dst, b) = code_image_barrier(0, 0x34, vk::Image::null(), range());
        assert_eq!(src, vk::PipelineStageFlags::TOP_OF_PIPE);
        assert_eq!(dst, vk::PipelineStageFlags::TRANSFER);
        assert_eq!(b.src_access_mask, vk::AccessFlags::empty());
        assert_eq!(b.dst_access_mask, vk::AccessFlags::TRANSFER_WRITE);
        assert_eq!(b.old_layout, vk::ImageLayout::UNDEFINED);
        assert_eq!(b.new_layout, vk::ImageLayout::TRANSFER_DST_OPTIMAL);
    }

    /// 0x34 → 0x22：TRANSFER → ALL_COMMANDS，TRANSFER_WRITE → SHADER_READ，7 → SHADER_READ_ONLY_OPTIMAL。
    #[test]
    fn code_0x34_to_0x22_is_transfer_to_shader_read() {
        let (src, dst, b) = code_image_barrier(0x34, 0x22, vk::Image::null(), range());
        assert_eq!(src, vk::PipelineStageFlags::TRANSFER);
        assert_eq!(dst, vk::PipelineStageFlags::ALL_COMMANDS);
        assert_eq!(b.src_access_mask, vk::AccessFlags::TRANSFER_WRITE);
        assert_eq!(b.dst_access_mask, vk::AccessFlags::SHADER_READ);
        assert_eq!(b.old_layout, vk::ImageLayout::TRANSFER_DST_OPTIMAL);
        assert_eq!(b.new_layout, vk::ImageLayout::SHADER_READ_ONLY_OPTIMAL);
    }

    /// 0x22 → 0x34（带旧内容的更新）：ALL_COMMANDS → TRANSFER，读不产生 src access。
    #[test]
    fn code_0x22_to_0x34_has_no_src_access() {
        let (src, dst, b) = code_image_barrier(0x22, 0x34, vk::Image::null(), range());
        assert_eq!(src, vk::PipelineStageFlags::ALL_COMMANDS);
        assert_eq!(dst, vk::PipelineStageFlags::TRANSFER);
        assert_eq!(b.src_access_mask, vk::AccessFlags::empty());
        assert_eq!(b.old_layout, vk::ImageLayout::SHADER_READ_ONLY_OPTIMAL);
    }
}
