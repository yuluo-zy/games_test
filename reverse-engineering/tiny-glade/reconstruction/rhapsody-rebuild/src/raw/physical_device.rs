//! 物理设备能力查询；不伪造原作偏好和兼容性名单。
use super::{error::Result, instance::VulkanInstance};
use ash::vk;
use std::{ffi::CStr, sync::Arc};
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DriverVendor {
    Nvidia,
    IntelWindows,
    IntelUnix,
    Amd,
    Qualcomm,
    Apple,
    Unknown,
}
impl DriverVendor {
    pub fn original_tag(self) -> u32 {
        match self {
            Self::Nvidia => 0,
            Self::IntelWindows => 1,
            Self::IntelUnix => 2,
            Self::Amd => 3,
            Self::Qualcomm => 4,
            Self::Apple => 5,
            Self::Unknown => 6,
        }
    }
    pub fn vendor_string(self) -> &'static str {
        match self {
            Self::Nvidia => "NVIDIA",
            Self::IntelWindows | Self::IntelUnix => "Intel",
            Self::Amd => "AMD",
            Self::Qualcomm => "Qualcomm",
            Self::Apple => "Apple",
            Self::Unknown => "Unknown",
        }
    }
}
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct DeviceDriverVersion {
    pub vendor: DriverVendor,
    pub components: [u32; 4],
    pub raw: u32,
    pub encoding_known: bool,
}
impl DeviceDriverVersion {
    pub fn from_parts(vendor: DriverVendor, components: [u32; 4]) -> Self {
        Self {
            vendor,
            components,
            raw: 0,
            encoding_known: true,
        }
    }
    pub fn vendor_string(self) -> &'static str {
        self.vendor.vendor_string()
    }
    pub fn is_amd(self) -> bool {
        self.vendor == DriverVendor::Amd
    }
    pub fn is_apple(self) -> bool {
        self.vendor == DriverVendor::Apple
    }
    /// 只对已识别驱动的公开编码解包；未知厂商/驱动不借用别家的版本格式。
    pub fn from_vulkan(vendor: u32, driver: vk::DriverId, raw: u32) -> Self {
        let standard = [
            vk::api_version_major(raw),
            vk::api_version_minor(raw),
            vk::api_version_patch(raw),
            0,
        ];
        let (kind, parts, known) = if vendor == 0x10de && driver == vk::DriverId::NVIDIA_PROPRIETARY
        {
            (
                DriverVendor::Nvidia,
                [raw >> 22, (raw >> 14) & 255, (raw >> 6) & 255, raw & 63],
                true,
            )
        } else if vendor == 0x8086 && driver == vk::DriverId::INTEL_PROPRIETARY_WINDOWS {
            (
                DriverVendor::IntelWindows,
                [raw >> 14, raw & 0x3fff, 0, 0],
                true,
            )
        } else if vendor == 0x8086 && driver == vk::DriverId::INTEL_OPEN_SOURCE_MESA {
            (DriverVendor::IntelUnix, standard, true)
        } else if vendor == 0x1002
            && (driver == vk::DriverId::AMD_PROPRIETARY || driver == vk::DriverId::AMD_OPEN_SOURCE)
        {
            (DriverVendor::Amd, standard, true)
        } else if vendor == 0x5143 {
            (DriverVendor::Qualcomm, standard, false)
        } else if vendor == 0x106b {
            (DriverVendor::Apple, standard, false)
        } else {
            (DriverVendor::Unknown, standard, false)
        };
        Self {
            vendor: kind,
            components: parts,
            raw,
            encoding_known: known,
        }
    }
}
impl std::fmt::Display for DeviceDriverVersion {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if !self.encoding_known {
            return write!(
                f,
                "{} raw:{:#x}（编码未知）",
                self.vendor_string(),
                self.raw
            );
        }
        let count = match self.vendor {
            DriverVendor::Nvidia => 4,
            DriverVendor::IntelWindows => 2,
            _ => 3,
        };
        write!(f, "{} ", self.vendor_string())?;
        for (i, value) in self.components[..count].iter().enumerate() {
            if i != 0 {
                f.write_str(".")?;
            }
            write!(f, "{value}")?;
        }
        Ok(())
    }
}
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct HeapBudget {
    pub index: u32,
    pub device_local: bool,
    pub size: u64,
    pub budget: Option<u64>,
    pub usage: Option<u64>,
}
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct PerformanceInfo {
    pub device_local_bytes: u64,
    pub host_bytes: u64,
    pub device_local_budget: Option<u64>,
    pub host_budget: Option<u64>,
    pub unified_memory: bool,
    pub heaps: Vec<HeapBudget>,
    pub vendor_specific: Option<String>,
}
impl PerformanceInfo {
    pub fn available_device_local_budget(&self) -> Option<u64> {
        self.device_local_budget?;
        self.heaps
            .iter()
            .filter(|h| h.device_local)
            .map(|h| h.budget.zip(h.usage).map(|(b, u)| b.saturating_sub(u)))
            .try_fold(0u64, |sum, v| v.map(|v| sum.saturating_add(v)))
    }
}
pub struct PhysicalDevice {
    pub instance: Arc<VulkanInstance>,
    pub handle: vk::PhysicalDevice,
    pub properties: vk::PhysicalDeviceProperties,
    pub memory: vk::PhysicalDeviceMemoryProperties,
    pub queues: Vec<vk::QueueFamilyProperties>,
}
impl PhysicalDevice {
    pub fn driver_version(&self) -> DeviceDriverVersion {
        let mut driver = vk::PhysicalDeviceDriverProperties::default();
        let mut properties = vk::PhysicalDeviceProperties2::default().push_next(&mut driver);
        unsafe {
            self.instance
                .instance
                .get_physical_device_properties2(self.handle, &mut properties);
        }
        DeviceDriverVersion::from_vulkan(
            self.properties.vendor_id,
            driver.driver_id,
            self.properties.driver_version,
        )
    }
    pub fn extension_names(&self) -> Result<Vec<String>> {
        Ok(unsafe {
            self.instance
                .instance
                .enumerate_device_extension_properties(self.handle)
        }?
        .iter()
        .map(|p| {
            unsafe { CStr::from_ptr(p.extension_name.as_ptr()) }
                .to_string_lossy()
                .into_owned()
        })
        .collect())
    }
    pub fn performance_info(&self) -> Result<PerformanceInfo> {
        let budget = self.refresh_memory_budget()?;
        Ok(self.memory_snapshot(budget.as_ref()))
    }
    pub(crate) fn memory_snapshot(
        &self,
        budget: Option<&vk::PhysicalDeviceMemoryBudgetPropertiesEXT<'_>>,
    ) -> PerformanceInfo {
        let heaps = self.memory.memory_heaps[..self.memory.memory_heap_count as usize]
            .iter()
            .enumerate()
            .map(|(i, h)| HeapBudget {
                index: i as u32,
                device_local: h.flags.contains(vk::MemoryHeapFlags::DEVICE_LOCAL),
                size: h.size,
                budget: budget.as_ref().map(|b| b.heap_budget[i]),
                usage: budget.as_ref().map(|b| b.heap_usage[i]),
            })
            .collect::<Vec<_>>();
        let sum = |local: bool| {
            heaps
                .iter()
                .filter(|h| h.device_local == local)
                .map(|h| h.size)
                .fold(0u64, u64::saturating_add)
        };
        let sum_budget = |local: bool| {
            budget.as_ref().map(|_| {
                heaps
                    .iter()
                    .filter(|h| h.device_local == local)
                    .map(|h| h.budget.unwrap())
                    .fold(0u64, u64::saturating_add)
            })
        };
        let unified = self.memory.memory_types[..self.memory.memory_type_count as usize]
            .iter()
            .any(|m| {
                m.property_flags.contains(
                    vk::MemoryPropertyFlags::DEVICE_LOCAL | vk::MemoryPropertyFlags::HOST_VISIBLE,
                )
            });
        PerformanceInfo {
            device_local_bytes: sum(true),
            host_bytes: sum(false),
            device_local_budget: sum_budget(true),
            host_budget: sum_budget(false),
            unified_memory: unified,
            heaps,
            vendor_specific: None,
        }
    }
    pub fn device_local_memory_budget_bytes(&self) -> Result<Option<u64>> {
        Ok(self.performance_info()?.device_local_budget)
    }
    pub fn host_memory_budget_bytes(&self) -> Result<Option<u64>> {
        Ok(self.performance_info()?.host_budget)
    }
    pub fn name(&self) -> String {
        unsafe { CStr::from_ptr(self.properties.device_name.as_ptr()) }
            .to_string_lossy()
            .into_owned()
    }
    pub fn device_local_memory_bytes(&self) -> u64 {
        self.memory.memory_heaps[..self.memory.memory_heap_count as usize]
            .iter()
            .filter(|h| h.flags.contains(vk::MemoryHeapFlags::DEVICE_LOCAL))
            .map(|h| h.size)
            .sum()
    }
    /// 未支持预算扩展时返回 None，不把堆大小当成预算。
    pub fn refresh_memory_budget(
        &self,
    ) -> Result<Option<vk::PhysicalDeviceMemoryBudgetPropertiesEXT<'static>>> {
        let extensions = unsafe {
            self.instance
                .instance
                .enumerate_device_extension_properties(self.handle)
        }?;
        if !extensions.iter().any(|e|unsafe {CStr::from_ptr(e.extension_name.as_ptr())}==ash::ext::memory_budget::NAME) {return Ok(None);}
        let mut budget = vk::PhysicalDeviceMemoryBudgetPropertiesEXT::default();
        let mut properties = vk::PhysicalDeviceMemoryProperties2::default().push_next(&mut budget);
        unsafe {
            self.instance
                .instance
                .get_physical_device_memory_properties2(self.handle, &mut properties);
        }
        Ok(Some(budget))
    }
    pub fn with_presentation_support(&self, surface: vk::SurfaceKHR) -> Result<Vec<u32>> {
        if surface == vk::SurfaceKHR::null() {
            return Err(super::error::Error::Invalid("呈现查询需要有效surface"));
        }
        let loader =
            ash::khr::surface::Instance::new(&self.instance.entry, &self.instance.instance);
        let mut output = Vec::new();
        for index in 0..self.queues.len() {
            if unsafe {
                loader.get_physical_device_surface_support(self.handle, index as u32, surface)
            }? {
                output.push(index as u32);
            }
        }
        Ok(output)
    }
    /// 总宿主内存（非 DEVICE_LOCAL 堆之和）。
    pub fn host_memory_bytes(&self) -> u64 {
        self.memory.memory_heaps[..self.memory.memory_heap_count as usize]
            .iter()
            .filter(|h| !h.flags.contains(vk::MemoryHeapFlags::DEVICE_LOCAL))
            .map(|h| h.size)
            .sum()
    }
    /// 按供应商字符串查找首个匹配物理设备。
    pub fn find_vendor_specific_device_handle(
        &self,
        vendors: &[&str],
    ) -> Option<vk::PhysicalDevice> {
        let name = self.name();
        vendors
            .iter()
            .any(|vendor| name.to_lowercase().contains(&vendor.to_lowercase()))
            .then_some(self.handle)
    }
}

/// 通用 Vulkan 枚举助手：先查 count，再取数组。
pub fn enumerate_vk_data<T: Clone>(
    mut fill: impl FnMut(&mut u32, &mut Vec<T>) -> vk::Result,
) -> Result<Vec<T>> {
    let mut count = 0;
    if fill(&mut count, &mut vec![]) != vk::Result::SUCCESS {
        return Err(super::error::Error::Invalid("枚举数量查询失败"));
    }
    let mut output = Vec::with_capacity(count as usize);
    if fill(&mut count, &mut output) != vk::Result::SUCCESS {
        return Err(super::error::Error::Invalid("枚举数据查询失败"));
    }
    output.truncate(count as usize);
    Ok(output)
}

/// 以空字符结尾的 C 字符数组 → String。
pub fn string_from_c_array(bytes: &[std::ffi::c_char]) -> String {
    let length = bytes
        .iter()
        .position(|byte| *byte == 0)
        .unwrap_or(bytes.len());
    unsafe { std::str::from_utf8_unchecked(std::slice::from_raw_parts(bytes.as_ptr().cast(), length)) }
        .to_owned()
}

/// Vulkan 驱动版本编码：`major<<22 | minor<<12 | patch`。
pub fn parse_driver_version(raw: u32) -> [u32; 3] {
    [raw >> 22, (raw >> 12) & 0x3ff, raw & 0xfff]
}

pub fn enumerate_physical_devices(instance: Arc<VulkanInstance>) -> Result<Vec<PhysicalDevice>> {
    let handles = unsafe { instance.instance.enumerate_physical_devices() }?;
    Ok(handles
        .into_iter()
        .map(|handle| PhysicalDevice {
            properties: unsafe { instance.instance.get_physical_device_properties(handle) },
            memory: unsafe {
                instance
                    .instance
                    .get_physical_device_memory_properties(handle)
            },
            queues: unsafe {
                instance
                    .instance
                    .get_physical_device_queue_family_properties(handle)
            },
            instance: Arc::clone(&instance),
            handle,
        })
        .collect())
}
