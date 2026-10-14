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
    format::vk_format_info,
    texture::Texture,
};
use ash::vk;
use std::sync::Arc;

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
    let commands = [command.handle];
    let submits = [vk::SubmitInfo::default().command_buffers(&commands)];
    let submitted = {
        let _guard = device.queue_lock.lock();
        device.note_submit();
        unsafe { device.device.queue_submit(device.queue, &submits, fence) }
    };
    let waited =
        submitted.and_then(|_| unsafe { device.device.wait_for_fences(&[fence], true, u64::MAX) });
    unsafe { device.device.destroy_fence(fence, None) };
    waited?;
    Ok(result)
}

fn upload_size(texture: &Texture) -> Result<u64> {
    let desc = &texture.desc;
    if desc.mip_levels != 1 || desc.array_layers != 1 {
        return Err(Error::Unsupported("纹理上传暂只支持单 mip、单层"));
    }
    if !desc.usage.contains(vk::ImageUsageFlags::TRANSFER_DST) {
        return Err(Error::Invalid("纹理缺少 TRANSFER_DST 用途"));
    }
    let info = vk_format_info(desc.format).ok_or(Error::Invalid("未知纹理格式"))?;
    let blocks = |extent: u32, block: u32| u64::from(extent.div_ceil(block.max(1)));
    Ok(blocks(desc.extent.width, info.block_extent[0])
        * blocks(desc.extent.height, info.block_extent[1])
        * blocks(desc.extent.depth, info.block_extent[2])
        * u64::from(info.block_bytes))
}

fn upload(
    device: &Arc<VulkanDevice>,
    texture: &Texture,
    bytes: &[u8],
    old_layout: vk::ImageLayout,
) -> Result<()> {
    let expected = upload_size(texture)?;
    if bytes.len() as u64 != expected {
        return Err(Error::Invalid("上传数据长度与纹理不符"));
    }
    let staging = Buffer::new(
        Arc::clone(device),
        &BufferDesc::new_cpu_to_gpu(expected, vk::BufferUsageFlags::TRANSFER_SRC),
    )?;
    // 暂存缓冲刚创建，GPU 尚未访问，写入无需等待在飞工作。
    staging.write_bytes_unsynchronized(0, bytes)?;
    let aspect = vk_format_info(texture.desc.format)
        .map(|info| info.aspect)
        .unwrap_or(vk::ImageAspectFlags::COLOR);
    let range = vk::ImageSubresourceRange::default()
        .aspect_mask(aspect)
        .level_count(1)
        .layer_count(1);
    with_setup_cb(device, |cb| unsafe {
        let to_dst = [vk::ImageMemoryBarrier::default()
            .src_access_mask(vk::AccessFlags::SHADER_READ)
            .dst_access_mask(vk::AccessFlags::TRANSFER_WRITE)
            .old_layout(old_layout)
            .new_layout(vk::ImageLayout::TRANSFER_DST_OPTIMAL)
            .image(texture.handle)
            .subresource_range(range)];
        device.device.cmd_pipeline_barrier(
            cb,
            vk::PipelineStageFlags::ALL_COMMANDS,
            vk::PipelineStageFlags::TRANSFER,
            vk::DependencyFlags::empty(),
            &[],
            &[],
            &to_dst,
        );
        let region = [vk::BufferImageCopy::default()
            .image_subresource(
                vk::ImageSubresourceLayers::default()
                    .aspect_mask(aspect)
                    .layer_count(1),
            )
            .image_extent(texture.desc.extent)];
        device.device.cmd_copy_buffer_to_image(
            cb,
            staging.handle,
            texture.handle,
            vk::ImageLayout::TRANSFER_DST_OPTIMAL,
            &region,
        );
        let to_read = [vk::ImageMemoryBarrier::default()
            .src_access_mask(vk::AccessFlags::TRANSFER_WRITE)
            .dst_access_mask(vk::AccessFlags::SHADER_READ)
            .old_layout(vk::ImageLayout::TRANSFER_DST_OPTIMAL)
            .new_layout(vk::ImageLayout::SHADER_READ_ONLY_OPTIMAL)
            .image(texture.handle)
            .subresource_range(range)];
        device.device.cmd_pipeline_barrier(
            cb,
            vk::PipelineStageFlags::TRANSFER,
            vk::PipelineStageFlags::ALL_COMMANDS,
            vk::DependencyFlags::empty(),
            &[],
            &[],
            &to_read,
        );
    })
}

/// 首次上传：图像当前布局为 UNDEFINED，完成后进入 SHADER_READ_ONLY_OPTIMAL。
pub fn init_texture_data(
    device: &Arc<VulkanDevice>,
    texture: &Texture,
    bytes: &[u8],
) -> Result<()> {
    upload(device, texture, bytes, vk::ImageLayout::UNDEFINED)
}

/// 更新已初始化纹理：前提是图像处于 SHADER_READ_ONLY_OPTIMAL（`init_texture_data` 之后的静止布局）。
pub fn update_texture(device: &Arc<VulkanDevice>, texture: &Texture, bytes: &[u8]) -> Result<()> {
    upload(
        device,
        texture,
        bytes,
        vk::ImageLayout::SHADER_READ_ONLY_OPTIMAL,
    )
}
