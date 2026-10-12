//! 额外Vulkan硬件RT适配，不属于本构建已证实的原作机制；原作软件BVH须复用原依赖另行接线。
//! 加速结构的实际 Vulkan 扩展适配。设备扩展与特性必须由调用者启用。
use super::device::VulkanDevice;
use ash::vk;
use std::sync::Arc;
pub struct AccelerationStructure {
    pub handle: vk::AccelerationStructureKHR,
    pub device: Arc<VulkanDevice>,
    loader: ash::khr::acceleration_structure::Device,
}
impl AccelerationStructure {
    /// # Safety
    /// info 中的存储缓冲须满足扩展有效性要求，并存活到加速结构销毁。
    pub unsafe fn new(
        device: Arc<VulkanDevice>,
        info: &vk::AccelerationStructureCreateInfoKHR<'_>,
    ) -> Result<Self, vk::Result> {
        let loader = ash::khr::acceleration_structure::Device::new(
            &device.instance.instance,
            &device.device,
        );
        let handle = unsafe { loader.create_acceleration_structure(info, None)? };
        Ok(Self {
            handle,
            device,
            loader,
        })
    }
    pub fn address(&self) -> u64 {
        let info = vk::AccelerationStructureDeviceAddressInfoKHR::default()
            .acceleration_structure(self.handle);
        unsafe { self.loader.get_acceleration_structure_device_address(&info) }
    }
    /// # Safety
    /// 几何体、scratch 与目标对象必须满足 Vulkan 构建约束及存活要求。
    pub unsafe fn build(
        &self,
        command: vk::CommandBuffer,
        infos: &[vk::AccelerationStructureBuildGeometryInfoKHR<'_>],
        ranges: &[&[vk::AccelerationStructureBuildRangeInfoKHR]],
    ) {
        unsafe {
            self.loader
                .cmd_build_acceleration_structures(command, infos, ranges)
        }
    }
}
impl Drop for AccelerationStructure {
    fn drop(&mut self) {
        let _ = self.device.wait_idle();
        unsafe {
            self.loader
                .destroy_acceleration_structure(self.handle, None)
        }
    }
}
