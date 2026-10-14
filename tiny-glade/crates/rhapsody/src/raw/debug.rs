//! Vulkan 调试名称和命令标签候选实现，原作命名策略尚未对照。
use crate::raw::device::VulkanDevice;
use ash::{vk, vk::Handle};
use std::{
    ffi::{CString, NulError},
    sync::Arc,
};

pub struct DebugUtils {
    pub device: Arc<VulkanDevice>,
    loader: ash::ext::debug_utils::Device,
}
impl DebugUtils {
    /// 仅在实例启用了 EXT_debug_utils 时创建。
    pub fn new(instance: &ash::Instance, device: Arc<VulkanDevice>) -> Self {
        let loader = ash::ext::debug_utils::Device::new(instance, &device.device);
        Self { device, loader }
    }
    /// # Safety
    /// handle 必须是本设备的存活对象；实例必须启用该扩展。
    pub unsafe fn set_object_name<T: Handle>(
        &self,
        handle: T,
        name: &str,
    ) -> Result<(), vk::Result> {
        let name = CString::new(name).map_err(|_| vk::Result::ERROR_INITIALIZATION_FAILED)?;
        let info = vk::DebugUtilsObjectNameInfoEXT::default()
            .object_handle(handle)
            .object_name(&name);
        unsafe { self.loader.set_debug_utils_object_name(&info) }
    }
    /// # Safety
    /// command 必须处于有效录制状态，且扩展已启用。
    pub unsafe fn begin_label(
        &self,
        command: vk::CommandBuffer,
        name: &str,
        color: [f32; 4],
    ) -> Result<(), NulError> {
        let name = CString::new(name)?;
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
