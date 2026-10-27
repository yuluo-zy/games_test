use crate::{Error, Result};
use crate::device::Device;
use crate::resource::managed::buffer::ManagedBuffer;
use crate::resource::managed::texture::{ManagedTexture, ManagedTextureView};
use crate::resource::texture::TextureViewDesc;
use ash::vk;
use std::sync::Arc;

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
    pub(crate) fn owns(&self, device: &Arc<crate::device::Device>) -> Result<()> {
        if Arc::ptr_eq(device, &self.shared()) {
            Ok(())
        } else {
            Err(Error::invalid_usage("句柄不属于本设备"))
        }
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

    pub fn set_buffer_debug_name(&self, buffer: &ManagedBuffer, name: &str) {
        self.set_debug_name(buffer.handle, name);
    }
    pub fn set_texture_debug_name(&self, texture: &ManagedTexture, name: &str) {
        self.set_debug_name(texture.handle, name);
    }

    /// 实例未启用 debug_utils 时静默跳过（命名只影响诊断）。
    pub(crate) fn set_debug_name<T: ash::vk::Handle>(&self, handle: T, name: &str) {
        if self
            .core
            .instance
            .enabled_extensions
            .iter()
            .any(|e| e == "VK_EXT_debug_utils")
        {
            let utils = self.debug_utils();
            let _ = unsafe { utils.set_object_name(handle, name) };
        }
    }

}

impl Device {
    /// 创建 GPU 分析输出缓冲。
    pub fn create_gpu_profiler_output_buffer(
        &self,
        size: u64,
    ) -> crate::error::Result<crate::resource::buffer::Buffer> {
        crate::resource::buffer::Buffer::new(
            self.shared(),
            &crate::resource::buffer::BufferDesc::new_gpu_to_cpu(
                size,
                vk::BufferUsageFlags::STORAGE_BUFFER | vk::BufferUsageFlags::TRANSFER_DST,
            ),
        )
    }
}
