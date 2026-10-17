use super::{
    buffer::{Buffer, BufferDesc, MappedBufferParts, MemoryLocation, plan_buffer_update},
    device::Device,
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

impl Device {
    fn owns(&self, device: &Arc<super::device::Device>) -> Result<()> {
        if Arc::ptr_eq(device, &self.shared()) {
            Ok(())
        } else {
            Err(Error::invalid_usage("句柄不属于本设备"))
        }
    }

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
        if bytes.is_some_and(|bytes| bytes.len() as u64 > desc.size) {
            return Err(Error::invalid_usage("缓冲初始数据超过声明大小"));
        }
        let buffer = Buffer::new(self.shared(), &desc)?;
        self.set_debug_name(buffer.handle, name);
        if let Some(bytes) = bytes {
            if buffer.persistent_host_ptr().is_some() {
                buffer.write_bytes_unsynchronized(0, bytes)?;
            } else {
                self.upload_via_staging(
                    &buffer,
                    bytes,
                    desc.size,
                    &[(0, 0, bytes.len() as u64)],
                    "buffer initial data",
                )?;
            }
        }
        self.resources.buffers.allocate(buffer)
    }

    pub fn create_mapped_buffer(
        &self,
        desc: &BufferDesc,
        name: &str,
    ) -> Result<(ManagedBuffer, MappedBufferParts)> {
        if !matches!(
            desc.memory_location,
            MemoryLocation::CpuToGpu | MemoryLocation::GpuToCpu
        ) {
            return Err(Error::invalid_usage(
                "映射缓冲需要 CpuToGpu 或 GpuToCpu 内存",
            ));
        }
        let managed = self.create_buffer(desc, name, ())?;
        let mapped = managed
            .persistent_host_ptr()
            .ok_or_else(|| Error::invalid_usage("缓冲内存未映射"))?;
        let parts = MappedBufferParts {
            ptr: mapped,
            len: managed.size as usize,
        };
        Ok((managed, parts))
    }

    /// Borrow this buffer's persistent host mapping.
    ///
    /// # Safety
    /// The caller must prevent host aliases and GPU accesses for the entire
    /// returned borrow, and satisfy non-coherent memory visibility rules.
    #[allow(clippy::mut_from_ref)]
    pub unsafe fn buffer_mapped_slice_mut<'a>(
        &self,
        buffer: &'a ManagedBuffer,
    ) -> Option<&'a mut [u8]> {
        self.owns(&buffer.device).ok()?;
        unsafe { buffer.mapped_full_slice_mut() }
    }

    pub fn buffer_device_address(&self, buffer: &ManagedBuffer) -> Result<vk::DeviceAddress> {
        self.owns(&buffer.device)?;
        buffer.device_address()
    }

    pub fn create_texture<D: TextureInitialBytes>(
        &self,
        desc: &TextureDesc,
        name: &str,
        data: D,
    ) -> Result<ManagedTexture> {
        desc.validate()?;
        let bytes = data.initial_bytes().filter(|b| !b.is_empty());
        let mut desc = *desc;
        if bytes.is_some() {
            desc.usage |= vk::ImageUsageFlags::TRANSFER_DST;
        }
        let texture =
            super::texture::Texture::new_with_initial_data(self.shared(), &desc, bytes.is_some())?;
        self.set_debug_name(texture.handle, name);
        if let Some(bytes) = bytes {
            super::setup::init_texture_data(&self.shared(), &texture, bytes)?;
        }
        self.resources.textures.allocate(texture)
    }

    /// `create_texture` 的完整 `TextureInitialData` 形态（`init_texture_data` 跳表 1/2/3 号目标，
    /// 见 [`super::setup::TextureInitialData`]）：多 mip、多层、块压缩的线性数据，或逐 mip 切片。
    /// 与带数据的 `create_texture` 一样加 `TRANSFER_DST`，创建后从 UNDEFINED 上传。
    pub fn create_texture_with_data(
        &self,
        desc: &TextureDesc,
        name: &str,
        data: super::setup::TextureInitialData<'_>,
    ) -> Result<ManagedTexture> {
        desc.validate()?;
        let mut desc = *desc;
        desc.usage |= vk::ImageUsageFlags::TRANSFER_DST;
        let texture = super::texture::Texture::new_with_initial_data(self.shared(), &desc, true)?;
        self.set_debug_name(texture.handle, name);
        super::setup::init_texture_data_with(&self.shared(), &texture, data)?;
        self.resources.textures.allocate(texture)
    }

    /// `create_texture` + `TextureInitialData` 跳表 1 号目标：整张图清零（见
    /// [`super::setup::init_texture_zero_fill`]）。与带数据的 `create_texture` 一样加 `TRANSFER_DST`、
    /// 创建布局为 UNDEFINED，随后转换到传输目标执行清零。
    pub fn create_texture_zeroed(&self, desc: &TextureDesc, name: &str) -> Result<ManagedTexture> {
        desc.validate()?;
        let mut desc = *desc;
        desc.usage |= vk::ImageUsageFlags::TRANSFER_DST;
        let texture = super::texture::Texture::new_with_initial_data(self.shared(), &desc, true)?;
        self.set_debug_name(texture.handle, name);
        super::setup::init_texture_zero_fill(&self.shared(), &texture)?;
        self.resources.textures.allocate(texture)
    }

    /// 原方法身份 `Device::bind_pipeline`（free fn 见 `raw::binding`）。
    ///
    /// # Safety
    /// `command_buffer` 必须处于录制状态且属于本设备。
    pub unsafe fn bind_pipeline(
        &self,
        command_buffer: vk::CommandBuffer,
        point: vk::PipelineBindPoint,
        pipeline: vk::Pipeline,
    ) {
        unsafe { super::binding::bind_pipeline(&self.shared(), command_buffer, point, pipeline) }
    }

    /// 原方法身份 `Device::bind_descriptor_set`。
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
                &self.shared(),
                command_buffer,
                point,
                layout,
                first_set,
                set,
                dynamic_offsets,
            )
        }
    }

    /// 原方法身份 `Device::bind_persistent_descriptor_set`。
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
                &self.shared(),
                command_buffer,
                point,
                layout,
                first_set,
                sets,
                dynamic_offsets,
            )
        }
    }

    pub fn get_vk_buffer(&self, buffer: &ManagedBuffer) -> Result<vk::Buffer> {
        self.owns(&buffer.device)?;
        Ok(buffer.handle)
    }

    pub fn get_vk_image_view(
        &self,
        texture: &ManagedTexture,
        desc: &TextureViewDesc,
    ) -> Result<ManagedTextureView> {
        self.owns(&texture.device)?;
        texture.image_view(desc)
    }

    pub fn update_buffer_bytes(
        &self,
        buffer: &ManagedBuffer,
        data: &[u8],
        ranges: &[Range<usize>],
        offset: usize,
    ) -> Result<()> {
        self.owns(&buffer.device)?;
        let plan = plan_buffer_update(&buffer.desc, data.len(), ranges, offset as u64)?;
        let mut gathered = Vec::new();
        gathered
            .try_reserve(usize::try_from(plan.total).map_err(|_| Error::OutOfHostMemory)?)
            .map_err(|_| Error::OutOfHostMemory)?;
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
            self.shared(),
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
        super::setup::with_setup_resources(
            &self.shared(),
            &[target, &staging],
            |command| unsafe {
                self.device
                    .cmd_copy_buffer(command, staging.handle, target.handle, &copies);
            },
        )?;
        Ok(())
    }
}
