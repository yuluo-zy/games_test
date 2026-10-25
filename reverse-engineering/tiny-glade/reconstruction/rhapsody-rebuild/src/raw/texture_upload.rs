//! 帧内批量区域上传适配：只录制命令，不提交或等待；不是原作者源码。
use super::{
    buffer::Buffer,
    command_buffer::VulkanCommandBuffer,
    error::{Error, Result},
    texture::{Texture, TextureDesc},
};
use ash::vk;
use std::sync::Arc;

/// 独立检查区域与暂存数据范围，不访问设备或修改录制状态。
pub fn validate_texture_regions(
    desc: &TextureDesc,
    staging_size: u64,
    staging_usage: vk::BufferUsageFlags,
    regions: &[vk::BufferImageCopy],
    old_layout: vk::ImageLayout,
) -> Result<()> {
    if !matches!(
        old_layout,
        vk::ImageLayout::UNDEFINED | vk::ImageLayout::SHADER_READ_ONLY_OPTIMAL
    ) {
        return Err(Error::Invalid(
            "上传旧布局必须为 UNDEFINED 或 SHADER_READ_ONLY_OPTIMAL",
        ));
    }
    if desc.image_type != vk::ImageType::TYPE_2D
        || desc.extent.depth != 1
        || desc.mip_levels != 1
        || desc.array_layers != 1
        || desc.samples != vk::SampleCountFlags::TYPE_1
    {
        return Err(Error::Unsupported("区域上传只支持二维单采样单 mip 单层"));
    }
    if !matches!(
        desc.format,
        vk::Format::R8G8B8A8_UNORM | vk::Format::R8G8B8A8_SRGB
    ) {
        return Err(Error::Unsupported("区域上传只支持 RGBA8 UNORM/SRGB"));
    }
    if !desc
        .usage
        .contains(vk::ImageUsageFlags::TRANSFER_DST | vk::ImageUsageFlags::SAMPLED)
        || !staging_usage.contains(vk::BufferUsageFlags::TRANSFER_SRC)
    {
        return Err(Error::Invalid("区域上传缺少 transfer 或 sampled 用途"));
    }
    if regions.is_empty() || regions.len() > u32::MAX as usize {
        return Err(Error::Invalid("上传区域数量无效"));
    }
    for region in regions {
        let sub = region.image_subresource;
        let extent = region.image_extent;
        let offset = region.image_offset;
        if sub.aspect_mask != vk::ImageAspectFlags::COLOR
            || sub.mip_level != 0
            || sub.base_array_layer != 0
            || sub.layer_count != 1
            || offset.z != 0
            || extent.depth != 1
            || extent.width == 0
            || extent.height == 0
            || offset.x < 0
            || offset.y < 0
        {
            return Err(Error::Invalid("上传区域子资源或尺寸无效"));
        }
        if (offset.x as u32)
            .checked_add(extent.width)
            .is_none_or(|end| end > desc.extent.width)
            || (offset.y as u32)
                .checked_add(extent.height)
                .is_none_or(|end| end > desc.extent.height)
        {
            return Err(Error::Invalid("上传区域越过纹理边界"));
        }
        if region.buffer_offset % 4 != 0 {
            return Err(Error::Invalid("RGBA8 暂存偏移必须四字节对齐"));
        }
        let row = if region.buffer_row_length == 0 {
            extent.width
        } else {
            region.buffer_row_length
        };
        let height = if region.buffer_image_height == 0 {
            extent.height
        } else {
            region.buffer_image_height
        };
        if row < extent.width || height < extent.height || u64::from(row) * 4 > i32::MAX as u64 {
            return Err(Error::Invalid("暂存行长或图像高度无效"));
        }
        // 单层且深度一：最后一行后面的 padding 不被 Vulkan 读取。
        let required = u64::from(extent.height - 1)
            .checked_mul(u64::from(row))
            .and_then(|n| n.checked_add(u64::from(extent.width)))
            .and_then(|n| n.checked_mul(4))
            .and_then(|n| n.checked_add(region.buffer_offset));
        if required.is_none_or(|end| end > staging_size) {
            return Err(Error::Invalid("上传暂存范围越界或上溢"));
        }
    }
    Ok(())
}

/// 将所有区域在现有命令缓冲中一次录制，最终进入 shader-read 布局。
///
/// # Safety
/// 调用者保证不在 render pass 内，图像实际旧布局与参数一致，且当前队列族拥有图像和暂存缓冲；
/// 队列必须支持图形录制。不得与其他 GPU 工作或宿主暂存写入竞争。
/// UNDEFINED 会丢弃整个图像内容；仅部分初始化时未写区域不能采样。
/// 调用者负责提交同步与完成凭据；本函数持有资源，但不提交、等待或释放完成拥有者。
pub unsafe fn record_texture_regions(
    command: &mut VulkanCommandBuffer,
    texture: Arc<Texture>,
    staging: Arc<Buffer>,
    regions: &[vk::BufferImageCopy],
    old_layout: vk::ImageLayout,
) -> Result<()> {
    if !command.is_recording() {
        return Err(Error::Invalid("区域上传要求命令缓冲正在录制"));
    }
    if !Arc::ptr_eq(&texture.device, &command.pool.device)
        || !Arc::ptr_eq(&staging.device, &command.pool.device)
    {
        return Err(Error::Invalid("上传资源和命令缓冲必须属于同一设备"));
    }
    validate_texture_regions(
        &texture.desc,
        staging.size,
        staging.usage,
        regions,
        old_layout,
    )?;
    // 在发出第一条命令前持有资源，直到对应命令的 GPU 生命周期完成。
    command.keep_alive(Arc::clone(&texture));
    command.keep_alive(Arc::clone(&staging));
    let range = vk::ImageSubresourceRange::default()
        .aspect_mask(vk::ImageAspectFlags::COLOR)
        .level_count(1)
        .layer_count(1);
    let first = [vk::ImageMemoryBarrier::default()
        .src_access_mask(if old_layout == vk::ImageLayout::UNDEFINED {
            vk::AccessFlags::empty()
        } else {
            vk::AccessFlags::SHADER_READ
        })
        .dst_access_mask(vk::AccessFlags::TRANSFER_WRITE)
        .old_layout(old_layout)
        .new_layout(vk::ImageLayout::TRANSFER_DST_OPTIMAL)
        .src_queue_family_index(vk::QUEUE_FAMILY_IGNORED)
        .dst_queue_family_index(vk::QUEUE_FAMILY_IGNORED)
        .image(texture.handle)
        .subresource_range(range)];
    let last = [vk::ImageMemoryBarrier::default()
        .src_access_mask(vk::AccessFlags::TRANSFER_WRITE)
        .dst_access_mask(vk::AccessFlags::SHADER_READ)
        .old_layout(vk::ImageLayout::TRANSFER_DST_OPTIMAL)
        .new_layout(vk::ImageLayout::SHADER_READ_ONLY_OPTIMAL)
        .src_queue_family_index(vk::QUEUE_FAMILY_IGNORED)
        .dst_queue_family_index(vk::QUEUE_FAMILY_IGNORED)
        .image(texture.handle)
        .subresource_range(range)];
    let device = &command.pool.device.device;
    unsafe {
        device.cmd_pipeline_barrier(
            command.handle,
            if old_layout == vk::ImageLayout::UNDEFINED {
                vk::PipelineStageFlags::TOP_OF_PIPE
            } else {
                vk::PipelineStageFlags::ALL_COMMANDS
            },
            vk::PipelineStageFlags::TRANSFER,
            vk::DependencyFlags::empty(),
            &[],
            &[],
            &first,
        );
        device.cmd_copy_buffer_to_image(
            command.handle,
            staging.handle,
            texture.handle,
            vk::ImageLayout::TRANSFER_DST_OPTIMAL,
            regions,
        );
        device.cmd_pipeline_barrier(
            command.handle,
            vk::PipelineStageFlags::TRANSFER,
            vk::PipelineStageFlags::ALL_COMMANDS,
            vk::DependencyFlags::empty(),
            &[],
            &[],
            &last,
        );
    }
    Ok(())
}
