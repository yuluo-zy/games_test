//! 原版 `GraphicsDevice` 的资源创建/访问入口（句柄适配层）。
//!
//! 原版入口收发“设备持有的托管句柄”：先校验句柄属于本设备，再转交管理器。工作区的资源本体是
//! `Arc<Buffer/Texture>`，这里在 [`ManagedResources`] 的槽位存储之上按原名字/原参数形态提供入口，
//! 不改变内部所有权结构。依据：`create_buffer<D>` 0x140c03460 等、`create_texture<D>` 0x140c07950、
//! `get_vk_image_view` 0x140bfeea0、`update_buffer_bytes` 0x140beefa0（见 PLAN M3.1–M3.5）。
use super::{
    buffer::{Buffer, BufferDesc, MappedBufferParts, MemoryLocation, plan_buffer_update},
    device::GraphicsDevice,
    error::{Error, Result},
    managed::{
        buffer::ManagedBuffer,
        texture::{ManagedTexture, ManagedTextureView},
    },
    texture::{TextureDesc, TextureViewDesc},
};
use ash::vk;
use std::{ops::Range, sync::Arc};

/// `create_buffer<D>` 的三个实例：`Option<&[u8]>` / `&[u8]` / `()`。
pub trait BufferInitialData {
    fn initial_bytes(&self) -> Option<&[u8]>;
}
impl BufferInitialData for () {
    fn initial_bytes(&self) -> Option<&[u8]> {
        None
    }
}
impl BufferInitialData for &[u8] {
    fn initial_bytes(&self) -> Option<&[u8]> {
        Some(self)
    }
}
impl BufferInitialData for Option<&[u8]> {
    fn initial_bytes(&self) -> Option<&[u8]> {
        *self
    }
}
/// `create_texture<D>` 的 `&[u8]` / `()` 实例（`TextureInitialData` 枚举的变体未恢复，见 M2.3）。
pub trait TextureInitialBytes {
    fn initial_bytes(&self) -> Option<&[u8]>;
}
impl TextureInitialBytes for () {
    fn initial_bytes(&self) -> Option<&[u8]> {
        None
    }
}
impl TextureInitialBytes for &[u8] {
    fn initial_bytes(&self) -> Option<&[u8]> {
        Some(self)
    }
}

impl GraphicsDevice {
    fn owns(&self, device: &Arc<super::device::VulkanDevice>) -> Result<()> {
        if Arc::ptr_eq(device, &self.inner) {
            Ok(())
        } else {
            Err(Error::invalid_usage("句柄不属于本设备"))
        }
    }

    /// 原 `create_buffer`（0x140c03460）：
    /// 1. 非空初始数据时 `usage |= TRANSFER_DST`（只在这个条件下加，0x140c0348x）；
    /// 2. `create_buffer_impl` 建缓冲并命名；
    /// 3. 有数据：`assert!(initial_data.byte_count() <= desc.size)`（0x142b186a0）；目标已映射则直接
    ///    `memcpy`；否则建暂存缓冲 `{size = desc.size, usage = TRANSFER_SRC, alignment 0,
    ///    memory_location = 3}`，名字 `"buffer initial data"`，`memcpy` 后经 setup 命令缓冲拷贝，暂存随后丢弃。
    ///
    /// 与原版的差异：断言失败返回 `Err` 而非 panic；setup 命令缓冲是同步版本（见 `setup.rs`）。
    pub fn create_buffer<D: BufferInitialData>(
        &self,
        desc: &BufferDesc,
        name: &str,
        data: D,
    ) -> Result<ManagedBuffer> {
        let bytes = data.initial_bytes().filter(|b| !b.is_empty());
        let mut desc = *desc;
        if bytes.is_some() {
            desc.usage |= vk::BufferUsageFlags::TRANSFER_DST;
        }
        let buffer = Buffer::new(self.inner.clone(), &desc)?;
        self.set_debug_name(buffer.handle, name);
        if let Some(bytes) = bytes {
            if bytes.len() as u64 > desc.size {
                return Err(Error::invalid_usage(
                    "assertion failed: initial_data.byte_count() <= desc.size",
                ));
            }
            if let Some(mapped) = buffer.mapped_full_slice_mut() {
                // 目标本身可映射：原版直接 memcpy（不经暂存）。
                mapped[..bytes.len()].copy_from_slice(bytes);
                buffer.write_bytes_unsynchronized(0, bytes)?; // 非 coherent 内存补 flush
            } else {
                // 区域大小 = 数据长度（0x140c04840：`size = if data.ptr != 0 { data.len } else { 0 }`，
                // srcOffset = dstOffset = 0，恒单区域）；暂存缓冲大小 = desc.size。
                self.upload_via_staging(
                    &buffer,
                    bytes,
                    desc.size,
                    &[(0, 0, bytes.len() as u64)],
                    "buffer initial data",
                )?;
            }
        }
        Ok(self.resources.buffers.allocate(buffer))
    }

    /// 原 `create_mapped_buffer`（0x140beea80）：`assert!(matches!(desc.memory_location,
    /// CpuToGpu | GpuToCpu))`；缓冲创建后必须有映射指针（否则 `expect("mapped memory")`）。
    /// 返回缓冲与整块映射 `(ptr, desc.size)`。
    pub fn create_mapped_buffer(
        &self,
        desc: &BufferDesc,
        name: &str,
    ) -> Result<(ManagedBuffer, MappedBufferParts)> {
        assert!(
            matches!(
                desc.memory_location,
                MemoryLocation::CpuToGpu | MemoryLocation::GpuToCpu
            ),
            "assertion failed: matches!(desc.memory_location, MemoryLocation::CpuToGpu | MemoryLocation::GpuToCpu)"
        );
        let managed = self.create_buffer(desc, name, ())?;
        let mapped = managed.mapped_full_slice_mut().expect("mapped memory");
        let parts = MappedBufferParts {
            ptr: mapped.as_mut_ptr(),
            len: mapped.len(),
        };
        Ok((managed, parts))
    }

    /// 原 `Buffer::mapped_slice_mut(&self, device)`（0x140bd4cd0）：全长切片，未映射返回 `None`。
    #[allow(clippy::mut_from_ref)]
    pub fn buffer_mapped_slice_mut<'a>(&self, buffer: &'a ManagedBuffer) -> Option<&'a mut [u8]> {
        self.owns(&buffer.device).ok()?;
        buffer.mapped_full_slice_mut()
    }

    /// 原 `Buffer::device_address(&self, device)`（0x140bd4c00）：三查后调 `vkGetBufferDeviceAddress`，
    /// 每次都走设备函数，没有缓存。
    pub fn buffer_device_address(&self, buffer: &ManagedBuffer) -> Result<vk::DeviceAddress> {
        self.owns(&buffer.device)?;
        buffer.device_address()
    }

    /// 原 `create_texture`（0x140c07950）：先 `assert!(desc.usage != empty)`；有非空初始数据时
    /// `usage |= TRANSFER_DST`；再 `assert!` create_info 的 extent 三维均 > 0；创建后同步上传。
    pub fn create_texture<D: TextureInitialBytes>(
        &self,
        desc: &TextureDesc,
        name: &str,
        data: D,
    ) -> Result<ManagedTexture> {
        assert!(
            desc.usage != vk::ImageUsageFlags::empty(),
            "assertion failed: desc.usage != vk::ImageUsageFlags::empty()"
        );
        let bytes = data.initial_bytes().filter(|b| !b.is_empty());
        let mut desc = *desc;
        if bytes.is_some() {
            desc.usage |= vk::ImageUsageFlags::TRANSFER_DST;
        }
        assert!(
            desc.width > 0,
            "assertion failed: create_info.extent.width > 0"
        );
        assert!(
            desc.height > 0,
            "assertion failed: create_info.extent.height > 0"
        );
        assert!(
            desc.depth > 0,
            "assertion failed: create_info.extent.depth > 0"
        );
        let texture = super::texture::Texture::new_with_initial_data(
            self.inner.clone(),
            &desc,
            bytes.is_some(),
        )?;
        self.set_debug_name(texture.handle, name);
        if let Some(bytes) = bytes {
            super::setup::init_texture_data(&self.inner, &texture, bytes)?;
        }
        Ok(self.resources.textures.allocate(texture))
    }

    /// `create_texture` 的完整 `TextureInitialData` 形态（`init_texture_data` 跳表 1/2/3 号目标，
    /// 见 [`super::setup::TextureInitialData`]）：多 mip、多层、块压缩的线性数据，或逐 mip 切片。
    /// 与带数据的 `create_texture` 一样加 `TRANSFER_DST`、`initialLayout = PREINITIALIZED`。
    pub fn create_texture_with_data(
        &self,
        desc: &TextureDesc,
        name: &str,
        data: super::setup::TextureInitialData<'_>,
    ) -> Result<ManagedTexture> {
        assert!(
            desc.usage != vk::ImageUsageFlags::empty(),
            "assertion failed: desc.usage != vk::ImageUsageFlags::empty()"
        );
        let mut desc = *desc;
        desc.usage |= vk::ImageUsageFlags::TRANSFER_DST;
        assert!(
            desc.width > 0,
            "assertion failed: create_info.extent.width > 0"
        );
        assert!(
            desc.height > 0,
            "assertion failed: create_info.extent.height > 0"
        );
        assert!(
            desc.depth > 0,
            "assertion failed: create_info.extent.depth > 0"
        );
        let texture =
            super::texture::Texture::new_with_initial_data(self.inner.clone(), &desc, true)?;
        self.set_debug_name(texture.handle, name);
        super::setup::init_texture_data_with(&self.inner, &texture, data)?;
        Ok(self.resources.textures.allocate(texture))
    }

    /// `create_texture` + `TextureInitialData` 跳表 1 号目标：整张图清零（见
    /// [`super::setup::init_texture_zero_fill`]）。与带数据的 `create_texture` 一样加 `TRANSFER_DST`、
    /// `initialLayout = PREINITIALIZED`。
    pub fn create_texture_zeroed(&self, desc: &TextureDesc, name: &str) -> Result<ManagedTexture> {
        assert!(
            desc.usage != vk::ImageUsageFlags::empty(),
            "assertion failed: desc.usage != vk::ImageUsageFlags::empty()"
        );
        let mut desc = *desc;
        desc.usage |= vk::ImageUsageFlags::TRANSFER_DST;
        assert!(
            desc.width > 0,
            "assertion failed: create_info.extent.width > 0"
        );
        assert!(
            desc.height > 0,
            "assertion failed: create_info.extent.height > 0"
        );
        assert!(
            desc.depth > 0,
            "assertion failed: create_info.extent.depth > 0"
        );
        let texture =
            super::texture::Texture::new_with_initial_data(self.inner.clone(), &desc, true)?;
        self.set_debug_name(texture.handle, name);
        super::setup::init_texture_zero_fill(&self.inner, &texture)?;
        Ok(self.resources.textures.allocate(texture))
    }

    /// 原 `create_render_pass`（0x140bfc0d0）：键是 0x98B 的 `RenderPassDesc`
    /// （颜色 `ArrayVec<_,8>` + 可选深度），相同描述命中 `+0x188` 缓存，返回同一个通道的新句柄引用；
    /// 未命中走 `RenderPass::from_desc`（无子通道依赖）。
    pub fn create_render_pass(
        &self,
        attachments: &[super::render_pass::RenderPassAttachmentDesc],
    ) -> Result<super::managed::render_pass::ManagedRenderPass> {
        self.create_render_pass_desc(super::render_pass::RenderPassDesc::from_flat(attachments))
    }

    /// 同 [`Self::create_render_pass`]，直接收原键。
    pub fn create_render_pass_desc(
        &self,
        desc: super::render_pass::RenderPassDesc,
    ) -> Result<super::managed::render_pass::ManagedRenderPass> {
        let mut cache = self.render_pass_cache.lock();
        if let Some(pass) = cache.get(&desc) {
            return Ok(pass.clone());
        }
        let pass = super::render_pass::RenderPass::from_desc(self.inner.clone(), &desc)?;
        let pass = self.resources.render_passes.allocate(pass);
        cache.insert(desc, pass.clone());
        Ok(pass)
    }

    /// 原 `create_framebuffer`：通道与附件视图都必须属于本设备。
    pub fn create_framebuffer(
        &self,
        pass: &super::managed::render_pass::ManagedRenderPass,
        views: Vec<std::sync::Arc<super::texture::TextureView>>,
        extent: vk::Extent2D,
        layers: u32,
    ) -> Result<super::managed::framebuffer::ManagedFramebuffer> {
        self.owns(&pass.device)?;
        let framebuffer =
            super::framebuffer::Framebuffer::new(pass.shared(), views, extent, layers)?;
        Ok(self.resources.framebuffers.allocate(framebuffer))
    }

    /// 原方法身份 `GraphicsDevice::bind_pipeline`（free fn 见 `raw::binding`）。
    ///
    /// # Safety
    /// `command_buffer` 必须处于录制状态且属于本设备。
    pub unsafe fn bind_pipeline(
        &self,
        command_buffer: vk::CommandBuffer,
        point: vk::PipelineBindPoint,
        pipeline: vk::Pipeline,
    ) {
        unsafe { super::binding::bind_pipeline(&self.inner, command_buffer, point, pipeline) }
    }

    /// 原方法身份 `GraphicsDevice::bind_descriptor_set`。
    ///
    /// # Safety
    /// 描述符集须与布局兼容，调用者保证其宿主与资源存活至提交完成。
    pub unsafe fn bind_descriptor_set(
        &self,
        command_buffer: vk::CommandBuffer,
        point: vk::PipelineBindPoint,
        layout: vk::PipelineLayout,
        first_set: u32,
        set: vk::DescriptorSet,
        dynamic_offsets: &[u32],
    ) {
        unsafe {
            super::binding::bind_descriptor_set(
                &self.inner,
                command_buffer,
                point,
                layout,
                first_set,
                set,
                dynamic_offsets,
            )
        }
    }

    /// 原方法身份 `GraphicsDevice::bind_persistent_descriptor_set`。
    ///
    /// # Safety
    /// 同 [`Self::bind_descriptor_set`]。
    pub unsafe fn bind_persistent_descriptor_set(
        &self,
        command_buffer: vk::CommandBuffer,
        point: vk::PipelineBindPoint,
        layout: vk::PipelineLayout,
        first_set: u32,
        sets: &[vk::DescriptorSet],
        dynamic_offsets: &[u32],
    ) {
        unsafe {
            super::binding::bind_persistent_descriptor_set(
                &self.inner,
                command_buffer,
                point,
                layout,
                first_set,
                sets,
                dynamic_offsets,
            )
        }
    }

    /// 原 `get_vk_buffer`（0x140bef470）：校验句柄属于本设备后取出原生句柄。
    pub fn get_vk_buffer(&self, buffer: &ManagedBuffer) -> Result<vk::Buffer> {
        self.owns(&buffer.device)?;
        Ok(buffer.handle)
    }

    /// 原 `get_vk_image_view`（0x140bfeea0）：校验后转 `ManagedTexture::view`（0x140bdc5d0）。
    pub fn get_vk_image_view(
        &self,
        texture: &ManagedTexture,
        desc: &TextureViewDesc,
    ) -> Result<ManagedTextureView> {
        self.owns(&texture.device)?;
        texture.image_view(desc)
    }

    /// 原 `update_buffer_bytes(buffer, data, ranges, offset)`（0x140beefa0）。
    ///
    /// 原版**没有**直写路径：判定顺序见 [`plan_buffer_update`]（句柄三查 → 逐区间断言 →
    /// `usage` 必须含 `TRANSFER_DST`（`inner+0x28 & 2`）→ 合计 > 0），然后一律建暂存缓冲
    /// `{size = total, usage = TRANSFER_SRC, memory_location = 3}`（名字 `"Buffer update data"`），
    /// 把各区间依次 `memcpy` 进去，再经 setup 命令缓冲发 `vkCmdCopyBuffer`，区域
    /// `{src = 累计, dst = range.start + offset, size}`。断言失败在原版是 panic，这里返回 `Err`，
    /// 且不会部分写入。
    pub fn update_buffer_bytes(
        &self,
        buffer: &ManagedBuffer,
        data: &[u8],
        ranges: &[Range<usize>],
        offset: usize,
    ) -> Result<()> {
        self.owns(&buffer.device)?;
        let record = buffer.record(0, 0, 0, 0);
        let slot = buffer.slot_view(0);
        let plan = plan_buffer_update(&record, 0, &slot, data.len(), ranges, offset as u64)
            .map_err(|e| Error::invalid_usage(e.static_text()))?;
        let mut gathered = Vec::with_capacity(plan.total as usize);
        for r in ranges {
            gathered.extend_from_slice(&data[r.clone()]);
        }
        let regions: Vec<(u64, u64, u64)> = plan
            .regions
            .iter()
            .map(|c| (c.src_offset, c.dst_offset, c.size))
            .collect();
        self.upload_via_staging(
            buffer,
            &gathered,
            plan.total,
            &regions,
            "Buffer update data",
        )
    }

    pub fn set_buffer_debug_name(&self, buffer: &ManagedBuffer, name: &str) {
        self.set_debug_name(buffer.handle, name);
    }
    pub fn set_texture_debug_name(&self, texture: &ManagedTexture, name: &str) {
        self.set_debug_name(texture.handle, name);
    }

    /// 实例未启用 debug_utils 时静默跳过（命名只影响诊断）。
    fn set_debug_name<T: ash::vk::Handle>(&self, handle: T, name: &str) {
        if self
            .inner
            .instance
            .enabled_extensions
            .iter()
            .any(|e| e == "VK_EXT_debug_utils")
        {
            let utils = self.debug_utils();
            let _ = unsafe { utils.set_object_name(handle, name) };
        }
    }

    /// 暂存上传：暂存缓冲 `{staging_size, TRANSFER_SRC, CpuToGpuLowPrio}`，`gathered` 从 0 起写入，
    /// `regions = (src, dst, size)` 经 setup 命令缓冲 `vkCmdCopyBuffer`。
    fn upload_via_staging(
        &self,
        target: &Buffer,
        gathered: &[u8],
        staging_size: u64,
        regions: &[(u64, u64, u64)],
        name: &str,
    ) -> Result<()> {
        if gathered.is_empty() {
            return Ok(());
        }
        let staging = Buffer::new(
            self.inner.clone(),
            &BufferDesc::new_cpu_to_gpu_low_prio(staging_size, vk::BufferUsageFlags::TRANSFER_SRC),
        )?;
        self.set_debug_name(staging.handle, name);
        staging.write_bytes_unsynchronized(0, gathered)?;
        let copies: Vec<vk::BufferCopy> = regions
            .iter()
            .map(|&(src, dst, size)| vk::BufferCopy {
                src_offset: src,
                dst_offset: dst,
                size,
            })
            .collect();
        self.with_setup_cb(|command| unsafe {
            self.inner
                .device
                .cmd_copy_buffer(command, staging.handle, target.handle, &copies);
        })?;
        Ok(())
    }
}
