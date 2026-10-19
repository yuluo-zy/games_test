//! Texture region uploads: command recording helpers (no submit or wait) and
//! pure region generation for initial data and updates.
use super::{
    buffer::Buffer,
    command_buffer::VulkanCommandBuffer,
    error::{Error, Result},
    format::vk_format_info,
    resource_access::{buffer_access_borrowed, image_access_borrowed},
    texture::{Texture, TextureDesc},
};
use crate::{resource::range_state::AccessScope, sync::QueueId};
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
        return Err(Error::invalid_usage(
            "上传旧布局必须为 UNDEFINED 或 SHADER_READ_ONLY_OPTIMAL",
        ));
    }
    if desc.image_type() != vk::ImageType::TYPE_2D
        || desc.extent().depth != 1
        || desc.mip_levels != 1
        || desc.array_layers()? != 1
        || desc.samples() != vk::SampleCountFlags::TYPE_1
    {
        return Err(Error::unsupported("区域上传只支持二维单采样单 mip 单层"));
    }
    if !matches!(
        desc.format,
        vk::Format::R8G8B8A8_UNORM | vk::Format::R8G8B8A8_SRGB
    ) {
        return Err(Error::unsupported("区域上传只支持 RGBA8 UNORM/SRGB"));
    }
    if !desc
        .usage
        .contains(vk::ImageUsageFlags::TRANSFER_DST | vk::ImageUsageFlags::SAMPLED)
        || !staging_usage.contains(vk::BufferUsageFlags::TRANSFER_SRC)
    {
        return Err(Error::invalid_usage(
            "区域上传缺少 transfer 或 sampled 用途",
        ));
    }
    if regions.is_empty() || regions.len() > u32::MAX as usize {
        return Err(Error::invalid_usage("上传区域数量无效"));
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
            return Err(Error::invalid_usage("上传区域子资源或尺寸无效"));
        }
        if (offset.x as u32)
            .checked_add(extent.width)
            .is_none_or(|end| end > desc.extent().width)
            || (offset.y as u32)
                .checked_add(extent.height)
                .is_none_or(|end| end > desc.extent().height)
        {
            return Err(Error::invalid_usage("上传区域越过纹理边界"));
        }
        if region.buffer_offset % 4 != 0 {
            return Err(Error::invalid_usage("RGBA8 暂存偏移必须四字节对齐"));
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
            return Err(Error::invalid_usage("暂存行长或图像高度无效"));
        }
        // 单层且深度一：最后一行后面的 padding 不被 Vulkan 读取。
        let required = u64::from(extent.height - 1)
            .checked_mul(u64::from(row))
            .and_then(|n| n.checked_add(u64::from(extent.width)))
            .and_then(|n| n.checked_mul(4))
            .and_then(|n| n.checked_add(region.buffer_offset));
        if required.is_none_or(|end| end > staging_size) {
            return Err(Error::invalid_usage("上传暂存范围越界或上溢"));
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
    queue: QueueId,
    texture: Arc<Texture>,
    staging: Arc<Buffer>,
    regions: &[vk::BufferImageCopy],
    old_layout: vk::ImageLayout,
) -> Result<()> {
    if !command.is_recording() {
        return Err(Error::invalid_usage("区域上传要求命令缓冲正在录制"));
    }
    if !Arc::ptr_eq(&texture.device, &command.pool.device)
        || !Arc::ptr_eq(&staging.device, &command.pool.device)
    {
        return Err(Error::invalid_usage("上传资源和命令缓冲必须属于同一设备"));
    }
    validate_texture_regions(
        &texture.desc,
        staging.size,
        staging.usage,
        regions,
        old_layout,
    )?;
    unsafe {
        command.retain_recorded(Arc::clone(&texture))?;
        command.retain_recorded(Arc::clone(&staging))?;
    }
    let range = Texture::ownership_range(&texture.desc)?;
    for region in regions {
        let row = if region.buffer_row_length == 0 {
            region.image_extent.width
        } else {
            region.buffer_row_length
        };
        let size = (u64::from(region.image_extent.height - 1) * u64::from(row)
            + u64::from(region.image_extent.width))
            * 4;
        buffer_access_borrowed(
            command,
            &staging,
            queue,
            region.buffer_offset,
            size,
            AccessScope {
                stages: vk::PipelineStageFlags::TRANSFER,
                access: vk::AccessFlags::TRANSFER_READ,
                write: false,
            },
        )?;
    }
    image_access_borrowed(
        command,
        &texture,
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
        command.pool.device.device.cmd_copy_buffer_to_image(
            command.handle,
            staging.handle,
            texture.handle,
            vk::ImageLayout::TRANSFER_DST_OPTIMAL,
            regions,
        );
    }
    image_access_borrowed(
        command,
        &texture,
        queue,
        range,
        crate::transfer::texture::IMAGE_CONSUMERS,
        vk::ImageLayout::SHADER_READ_ONLY_OPTIMAL,
        Some(vk::ImageLayout::TRANSFER_DST_OPTIMAL),
    )
}

pub(crate) fn record_zero_fill_borrowed(
    command: &mut VulkanCommandBuffer,
    texture: &Texture,
    queue: QueueId,
    expected_layout: Option<vk::ImageLayout>,
) -> Result<()> {
    let format = vk_format_info(texture.desc.format)
        .ok_or_else(|| Error::unsupported("texture format has no clear layout"))?;
    if !texture
        .desc
        .usage
        .contains(vk::ImageUsageFlags::TRANSFER_DST | vk::ImageUsageFlags::SAMPLED)
        || texture.desc.samples() != vk::SampleCountFlags::TYPE_1
        || format.block_extent != [1, 1, 1]
    {
        return Err(Error::unsupported(
            "texture clear requires an uncompressed single-sampled transfer destination",
        ));
    }
    let aspect = format.aspect;
    let range = Texture::ownership_range(&texture.desc)?;
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
        expected_layout,
    )?;
    let native = vk::ImageSubresourceRange::default()
        .aspect_mask(aspect)
        .level_count(u32::from(texture.desc.mip_levels))
        .layer_count(texture.desc.array_layers()?);
    unsafe {
        if aspect.intersects(vk::ImageAspectFlags::DEPTH | vk::ImageAspectFlags::STENCIL) {
            command.pool.device.device.cmd_clear_depth_stencil_image(
                command.handle,
                texture.handle,
                vk::ImageLayout::TRANSFER_DST_OPTIMAL,
                &vk::ClearDepthStencilValue::default(),
                &[native],
            );
        } else {
            command.pool.device.device.cmd_clear_color_image(
                command.handle,
                texture.handle,
                vk::ImageLayout::TRANSFER_DST_OPTIMAL,
                &vk::ClearColorValue::default(),
                &[native],
            );
        }
    }
    image_access_borrowed(
        command,
        texture,
        queue,
        range,
        crate::transfer::texture::IMAGE_CONSUMERS,
        vk::ImageLayout::SHADER_READ_ONLY_OPTIMAL,
        Some(vk::ImageLayout::TRANSFER_DST_OPTIMAL),
    )
}

/// Clear every image subresource and leave it ready for shader sampling.
///
/// # Safety
/// Record outside rendering on a transfer/graphics capable queue. The image
/// must support transfer destination and sampled access; submit on `queue`.
pub unsafe fn record_texture_zero_fill(
    command: &mut VulkanCommandBuffer,
    texture: Arc<Texture>,
    queue: QueueId,
) -> Result<()> {
    if !texture
        .desc
        .usage
        .contains(vk::ImageUsageFlags::TRANSFER_DST | vk::ImageUsageFlags::SAMPLED)
    {
        return Err(Error::invalid_usage(
            "texture clear requires transfer destination and sampled usage",
        ));
    }
    unsafe {
        command.retain_recorded(texture.clone())?;
    }
    record_zero_fill_borrowed(command, &texture, queue, None)?;
    Ok(())
}

// ======================================================================================
// Region generation for initial data and updates (pure, no device access).
// ======================================================================================

/// One tightly packed copy region: `data` covers `extent` of one mip level of
/// one array layer (cube faces are layers).
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct TextureUpdateRegion<'a> {
    pub data: &'a [u8],
    pub extent: [u32; 3],
    pub mip_level: u32,
    pub array_layer: u32,
}

/// One mip level of a chain: byte size, extent and level index.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct MipLevelItem {
    /// `ceil(w/block[0]) * ceil(h/block[1]) * d * bytes_per_block`。
    pub size: u64,
    pub extent: [u32; 3],
    pub mip_level: u32,
}

/// Iterator over the mip levels of an extent with texel-block sizes.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct MipChainIter {
    extent: [u16; 3],
    block: [u8; 2],
    cur: u8,
    count: u8,
    bytes_per_block: u8,
}

const EXTENT_ERR: &str = "texture extent and layer count must fit in 16 bits";
const BLOCK_ERR: &str = "texel block byte size must fit in 8 bits";

impl MipChainIter {
    /// Requires nonzero block size, block extent and extent, and an extent that
    /// fits in 16 bits per dimension.
    pub fn new(
        extent: [u32; 3],
        mip_levels: u8,
        bytes_per_block: u8,
        block: [u8; 2],
    ) -> Result<Self> {
        if bytes_per_block == 0 {
            return Err(Error::invalid_usage(
                "texel block byte size must be nonzero",
            ));
        }
        if block[0] == 0 {
            return Err(Error::invalid_usage("texel block width must be nonzero"));
        }
        if block[1] == 0 {
            return Err(Error::invalid_usage("texel block height must be nonzero"));
        }
        if extent[0] == 0 {
            return Err(Error::invalid_usage("texture width must be nonzero"));
        }
        if extent[1] == 0 {
            return Err(Error::invalid_usage("texture height must be nonzero"));
        }
        if extent[2] == 0 {
            return Err(Error::invalid_usage("texture depth must be nonzero"));
        }
        let mut narrowed = [0u16; 3];
        for (dst, src) in narrowed.iter_mut().zip(extent) {
            *dst = u16::try_from(src).map_err(|_| Error::invalid_usage(EXTENT_ERR))?;
        }
        Ok(Self {
            extent: narrowed,
            block,
            cur: 0,
            count: mip_levels,
            bytes_per_block,
        })
    }
}

impl Iterator for MipChainIter {
    type Item = MipLevelItem;
    /// Yields the current level, then halves every dimension (at least 1); the
    /// chain ends after `count` levels or after the 1x1x1 level.
    fn next(&mut self) -> Option<MipLevelItem> {
        if self.cur >= self.count {
            return None;
        }
        let [w, h, d] = self.extent;
        let level = u32::from(self.cur);
        let wb = u64::from(w.div_ceil(u16::from(self.block[0])));
        let hb = u64::from(h.div_ceil(u16::from(self.block[1])));
        let size = u64::from(self.bytes_per_block) * u64::from(d) * hb * wb;
        let (nw, nh, nd) = (w >> 1, h >> 1, d >> 1);
        if nw | nh | nd == 0 {
            self.cur = 0xff;
        } else {
            self.extent = [nw.max(1), nh.max(1), nd.max(1)];
            self.cur += 1;
        }
        Some(MipLevelItem {
            size,
            extent: [u32::from(w), u32::from(h), u32::from(d)],
            mip_level: level,
        })
    }
}

/// Regions of linear data: layer-major, mip-minor, each level sliced in order
/// with its [`MipChainIter`] size. Fails on empty, short or excess data, or on
/// layer/block counts that do not fit the iterator.
///
/// The caller supplies actual Vulkan layers, including every face of cube arrays.
pub fn linear_regions(
    data: &[u8],
    extent: [u32; 3],
    mip_levels: u8,
    array_elements: u32,
    bytes_per_block: u32,
    block: [u8; 2],
) -> Result<Vec<TextureUpdateRegion<'_>>> {
    let layers = u16::try_from(array_elements).map_err(|_| Error::invalid_usage(EXTENT_ERR))?;
    let bytes_per_block =
        u8::try_from(bytes_per_block).map_err(|_| Error::invalid_usage(BLOCK_ERR))?;
    if data.is_empty() {
        return Err(Error::invalid_usage("texture initial data is empty"));
    }
    let template = MipChainIter::new(extent, mip_levels, bytes_per_block, block)?;
    let mut rest = data;
    let mut regions = Vec::new();
    for layer in 0..layers.max(1) {
        for item in template {
            if rest.is_empty() {
                return Err(Error::invalid_usage("Texture initial data underflow"));
            }
            let Some(len) = usize::try_from(item.size).ok().filter(|&n| n <= rest.len()) else {
                return Err(Error::invalid_usage("Texture initial data underflow"));
            };
            let (head, tail) = rest.split_at(len);
            rest = tail;
            regions.push(TextureUpdateRegion {
                data: head,
                extent: item.extent,
                mip_level: item.mip_level,
                array_layer: u32::from(layer),
            });
        }
    }
    if !rest.is_empty() {
        return Err(Error::invalid_usage(
            "Texture initial data has incorrect size; more bytes supplied than necessary",
        ));
    }
    Ok(regions)
}

/// Regions of one slice per mip level (layer 0). Requires one slice per level;
/// slices shorter than `min_len` reports for their level are rejected.
pub fn per_mip_regions<'a>(
    slices: &[&'a [u8]],
    extent: [u32; 3],
    mip_levels: u8,
    min_len: impl Fn(&MipLevelItem) -> Option<u64>,
) -> Result<Vec<TextureUpdateRegion<'a>>> {
    if slices.len() != usize::from(mip_levels) {
        return Err(Error::invalid_usage(
            "per-mip slice count differs from the mip level count",
        ));
    }
    let chain = MipChainIter::new(extent, mip_levels, 1, [1, 1])?;
    let mut regions = Vec::with_capacity(slices.len());
    for (slice, item) in slices.iter().zip(chain) {
        if let Some(need) = min_len(&item)
            && (slice.len() as u64) < need
        {
            return Err(Error::invalid_usage("per-mip slice is shorter than its level"));
        }
        regions.push(TextureUpdateRegion {
            data: slice,
            extent: item.extent,
            mip_level: item.mip_level,
            array_layer: 0,
        });
    }
    Ok(regions)
}

/// `update_texture_impl` 的拷贝计划：暂存缓冲总长与 `vkCmdCopyBufferToImage` 区域表。
#[derive(Clone, Debug)]
pub struct TextureCopyPlan {
    pub total: u64,
    pub copies: Vec<vk::BufferImageCopy>,
}

/// 为每个区域生成一个紧密行布局的 Vulkan copy，并按 texel block 与四字节要求
/// 对齐各区域的暂存偏移。`total` 包含对齐填充；调用方须按返回的 offset 写入数据。
/// combined depth/stencil 和多平面格式须先拆成单 aspect 数据，不能在此隐式混合。
pub fn plan_texture_copies(
    regions: &[TextureUpdateRegion<'_>],
    block_bytes: u32,
    aspect: vk::ImageAspectFlags,
) -> Result<TextureCopyPlan> {
    if regions.is_empty() || block_bytes == 0 || aspect.as_raw().count_ones() != 1 {
        return Err(Error::invalid_usage(
            "texture copies require data, a texel block and one aspect",
        ));
    }
    let gcd = |mut a: u64, mut b: u64| {
        while b != 0 {
            let next = a % b;
            a = b;
            b = next;
        }
        a
    };
    let alignment = u64::from(block_bytes) / gcd(u64::from(block_bytes), 4) * 4;
    let mut offset = 0u64;
    let mut copies = Vec::with_capacity(regions.len());
    for region in regions {
        if region.data.is_empty() || region.extent.contains(&0) {
            return Err(Error::invalid_usage("texture copy region is empty"));
        }
        offset = offset
            .checked_add(alignment - 1)
            .and_then(|n| (n / alignment).checked_mul(alignment))
            .ok_or_else(|| Error::invalid_usage("texture copy offset overflows"))?;
        copies.push(
            vk::BufferImageCopy::default()
                .buffer_offset(offset)
                .image_subresource(vk::ImageSubresourceLayers {
                    aspect_mask: aspect,
                    mip_level: region.mip_level,
                    base_array_layer: region.array_layer,
                    layer_count: 1,
                })
                .image_extent(vk::Extent3D {
                    width: region.extent[0],
                    height: region.extent[1],
                    depth: region.extent[2],
                }),
        );
        offset = offset
            .checked_add(region.data.len() as u64)
            .ok_or_else(|| Error::invalid_usage("texture staging size overflows"))?;
    }
    Ok(TextureCopyPlan {
        total: offset,
        copies,
    })
}

/// Public Vulkan texel-block size and a single copy aspect, including every BC format.
pub fn copy_format_params(format: vk::Format) -> Result<(u32, vk::ImageAspectFlags)> {
    let info = vk_format_info(format)
        .ok_or_else(|| Error::unsupported("texture format has no copy layout"))?;
    if !super::format::vk_format_planes(format).is_empty()
        || info.aspect.as_raw().count_ones() != 1
        || info.block_bytes == 0
        || info.block_extent[2] != 1
    {
        return Err(Error::unsupported(
            "texture upload requires one aspect and two-dimensional texel blocks",
        ));
    }
    Ok((info.block_bytes, info.aspect))
}
