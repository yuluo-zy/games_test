//! Vulkan 调试名称和命令标签候选实现，原作命名策略尚未对照。
use crate::device::Device;
use ash::{vk, vk::Handle};
use std::{ffi::CString, sync::Arc};

pub struct DebugUtils {
    pub(crate) device: Arc<Device>,
    loader: ash::ext::debug_utils::Device,
}

impl DebugUtils {
    /// The device that owns this object.
    pub fn device(&self) -> &Arc<Device> {
        &self.device
    }
}
impl DebugUtils {
    /// 仅在实例启用了 EXT_debug_utils 时创建。
    pub(crate) fn new(instance: &ash::Instance, device: Arc<Device>) -> Self {
        let loader = ash::ext::debug_utils::Device::new(instance, &device.core.device);
        Self { device, loader }
    }
    /// # Safety
    /// handle 必须是本设备的存活对象；实例必须启用该扩展。
    pub unsafe fn set_object_name<T: Handle>(&self, handle: T, name: &str) -> crate::Result<()> {
        let name = CString::new(name).map_err(|_| crate::Error::invalid_usage("调试名称含 NUL"))?;
        let info = vk::DebugUtilsObjectNameInfoEXT::default()
            .object_handle(handle)
            .object_name(&name);
        unsafe { self.loader.set_debug_utils_object_name(&info) }.map_err(Into::into)
    }
    /// # Safety
    /// command 必须处于有效录制状态，且扩展已启用。
    pub unsafe fn begin_label(
        &self,
        command: vk::CommandBuffer,
        name: &str,
        color: [f32; 4],
    ) -> crate::Result<()> {
        let name = CString::new(name).map_err(|_| crate::Error::invalid_usage("调试标签含 NUL"))?;
        let info = vk::DebugUtilsLabelEXT::default()
            .label_name(&name)
            .color(color);
        unsafe { self.loader.cmd_begin_debug_utils_label(command, &info) };
        Ok(())
    }
    /// # Safety
    /// command 中必须已有对应的开始标签。
    pub unsafe fn end_label(&self, command: vk::CommandBuffer) {
        unsafe { self.loader.cmd_end_debug_utils_label(command) }
    }
}
