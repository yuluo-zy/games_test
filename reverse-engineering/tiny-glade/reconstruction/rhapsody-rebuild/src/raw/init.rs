//! 候选初始化路径以实际能力查询建立设备，不推测原作排序名单。
pub use super::instance::GraphicsInstanceLoader;
use super::physical_device::{DeviceDriverVersion, DriverVendor, PerformanceInfo};
use super::{
    device::VulkanDevice,
    error::{Error, Result},
    instance::{InstanceBuilder, VulkanInstance},
    physical_device::{PhysicalDevice, enumerate_physical_devices},
};
use ash::vk::{self, Handle};
use serde::{Deserialize, Serialize};
use std::sync::Arc;

#[derive(Clone, Debug)]
pub struct GraphicsInstanceLayer {
    pub name: String,
    pub description: String,
}
impl GraphicsInstanceLayer {
    /// 原入口先要求 VK_LAYER_，再取后缀首个下划线之前的非空 vendor；仅有后缀返回 None。
    pub fn vendor(&self) -> Option<&str> {
        let rest = self.name.strip_prefix("VK_LAYER_")?;
        let (vendor, _) = rest.split_once('_')?;
        (!vendor.is_empty()).then_some(vendor)
    }
}
#[derive(Clone, Debug, PartialEq, Eq, Deserialize, Serialize)]
pub enum GraphicsInstanceLayerAllowlistEntry {
    Layer(String),
    Vendor(String),
}
#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct GraphicsInstanceLayerAllowlist {
    pub version: u32,
    pub entries: Vec<GraphicsInstanceLayerAllowlistEntry>,
}
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct AllowlistMigration {
    pub version: u32,
    pub add: Vec<GraphicsInstanceLayerAllowlistEntry>,
}
impl GraphicsInstanceLayerAllowlist {
    pub fn contains_layer(&self, name: &str) -> bool {
        self.entries
            .iter()
            .any(|e| matches!(e,GraphicsInstanceLayerAllowlistEntry::Layer(v) if v==name))
    }
    pub fn contains_vendor(&self, name: &str) -> bool {
        self.entries
            .iter()
            .any(|e| matches!(e,GraphicsInstanceLayerAllowlistEntry::Vendor(v) if v==name))
    }
    pub fn permits(&self, layer: &GraphicsInstanceLayer) -> bool {
        self.contains_layer(&layer.name) || layer.vendor().is_some_and(|v| self.contains_vendor(v))
    }
    /// 直接复用原 Ron fork；既有用户项保留，只追加更高版本 migration。
    pub fn update_from_ron(&mut self, input: &str) -> std::result::Result<(), String> {
        let migrations: Vec<AllowlistMigration> =
            ron::from_str(input).map_err(|e| e.to_string())?;
        if migrations.windows(2).any(|w| w[0].version >= w[1].version) {
            return Err("白名单migration版本必须严格递增".into());
        }
        for migration in migrations {
            if migration.version > self.version {
                self.entries.extend(migration.add);
                self.version = migration.version;
            }
        }
        Ok(())
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum DriverSupport {
    Supported,
    BelowMinimum {
        recommended: &'static str,
        old_intel: bool,
    },
    KnownIssue {
        recommended: &'static str,
    },
    Emulated,
    Unknown,
}
impl DriverSupport {
    pub fn original_tag(&self) -> u32 {
        match self {
            Self::Supported => 5,
            Self::BelowMinimum {
                old_intel: true, ..
            } => 1,
            Self::BelowMinimum { .. } => 0,
            Self::KnownIssue { .. } => 4,
            Self::Emulated => 2,
            Self::Unknown => 3,
        }
    }
}
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum DeviceDecision {
    Yes,
    No(String),
    Unknown(String),
}
#[derive(Clone, Copy, Debug)]
pub struct DeviceMemoryMetrics {
    pub local: u64,
    pub host: u64,
    pub shared: bool,
}
#[derive(Clone, Debug)]
pub struct AvailableGraphicsDevice {
    pub index: usize,
    pub handle: u64,
    pub name: String,
    pub device_type: vk::PhysicalDeviceType,
    pub driver: DeviceDriverVersion,
    pub memory: PerformanceInfo,
    pub original_memory_metrics: Option<DeviceMemoryMetrics>,
    pub api_version: u32,
    pub timeline_semaphore: bool,
    pub graphics_families: Vec<u32>,
    pub present_families: Option<Vec<u32>>,
    pub extensions: Vec<String>,
    pub errors: Vec<String>,
}
impl AvailableGraphicsDevice {
    pub fn is_emulated(&self) -> bool {
        self.name.contains("Direct3D12") || self.name.contains("Microsoft Basic Render Driver")
    }
    pub fn is_old_intel(&self) -> bool {
        self.driver.vendor == DriverVendor::IntelWindows
            && (self.name.contains("HD Graphics") || self.name.contains("Iris(R)"))
            && ![
                "Xe",
                "11th",
                "Gen11",
                "12th",
                "Gen12",
                "13th",
                "Gen13",
                "Graphics 7",
                "Graphics P7",
            ]
            .iter()
            .any(|name| self.name.contains(name))
    }
    /// 原限定跳表与版本比较；编码尚未确认时返回 Unknown，不能按厂商猜通过。
    pub fn driver_support(&self) -> DriverSupport {
        if self.is_emulated() {
            return DriverSupport::Emulated;
        }
        if !self.driver.encoding_known {
            return DriverSupport::Unknown;
        }
        let c = self.driver.components;
        let old = self.is_old_intel();
        let (minimum, recommended) = match self.driver.vendor {
            DriverVendor::Nvidia => {
                if c[0] == 580 && (c[1] == 88 || c[1] == 65) && self.name.contains("RTX 50") {
                    return DriverSupport::KnownIssue {
                        recommended: if c[1] == 88 { "580.97" } else { "580.95" },
                    };
                }
                if c[0] < 500 {
                    ([472, 12, 0, 0], "472.12")
                } else {
                    ([511, 23, 0, 0], "511.23")
                }
            }
            DriverVendor::IntelWindows => {
                if old {
                    ([101, 2111, 0, 0], "101.2111")
                } else {
                    ([101, 5522, 0, 0], "101.5522")
                }
            }
            DriverVendor::IntelUnix => ([24, 1, 5, 0], "24.1.5"),
            DriverVendor::Amd => ([2, 0, 179, 0], "21.3.1"),
            DriverVendor::Qualcomm => ([512, 798, 6, 0], "512.798.6"),
            DriverVendor::Apple => ([0, 2, 2108, 0], "0.2.2108"),
            DriverVendor::Unknown => return DriverSupport::Unknown,
        };
        if c < minimum {
            DriverSupport::BelowMinimum {
                recommended,
                old_intel: old,
            }
        } else {
            DriverSupport::Supported
        }
    }
    pub fn is_compatible(&self) -> DeviceDecision {
        match self.driver_support() {
            DriverSupport::Unknown => {
                return DeviceDecision::Unknown("原驱动版本编码/支持条件未确认".into());
            }
            DriverSupport::Supported => {}
            other => return DeviceDecision::No(format!("驱动不满足原条件: {other:?}")),
        }
        let Some(memory) = self.original_memory_metrics else {
            return DeviceDecision::Unknown(
                "原可用内存指标尚未取得；不以heap size冒充budget".into(),
            );
        };
        let enough = if memory.shared {
            memory.local.wrapping_add(memory.host) >= 1_400_000_000
        } else {
            memory.local >= 1_000_000_000 && memory.host >= 400_000_000
        };
        if enough {
            DeviceDecision::Yes
        } else {
            DeviceDecision::No("原内存门槛不足".into())
        }
    }
    pub fn is_supported(&self) -> DeviceDecision {
        if self.is_old_intel() {
            DeviceDecision::No("原支持规则排除旧Intel（兼容性另行保留）".into())
        } else {
            self.is_compatible()
        }
    }
}

#[derive(Clone, Debug)]
pub struct DeviceSelectionPolicy {
    pub allow_unknown_original_policy: bool,
    pub allow_compatible: bool,
    pub required_extensions: Vec<String>,
    pub requires_surface: bool,
}
impl Default for DeviceSelectionPolicy {
    // 保留候选基本渲染路径；Unknown仍然以Unknown返回，选项明确记录fallback而非宣称原支持。
    fn default() -> Self {
        Self {
            allow_unknown_original_policy: true,
            allow_compatible: true,
            required_extensions: vec![],
            requires_surface: false,
        }
    }
}
#[derive(Clone, Debug)]
pub struct GraphicsDevicesForSurface {
    pub devices: Vec<AvailableGraphicsDevice>,
    pub policy: DeviceSelectionPolicy,
    pub available: Vec<usize>,
    pub missing: Vec<(usize, Vec<String>)>,
}
impl GraphicsDevicesForSurface {
    pub fn refresh_memory_budgets(&mut self, graphics: &GraphicsInstance) -> Result<()> {
        for candidate in &mut self.devices {
            let physical = graphics
                .devices
                .get(candidate.index)
                .filter(|p| p.handle.as_raw() == candidate.handle)
                .ok_or(Error::Invalid("刷新预算时设备清单身份改变"))?;
            candidate.memory = physical.performance_info()?;
            candidate.original_memory_metrics = candidate
                .memory
                .device_local_budget
                .zip(candidate.memory.host_budget)
                .map(|(local, host)| DeviceMemoryMetrics {
                    local,
                    host,
                    shared: candidate.memory.unified_memory,
                });
        }
        self.order_and_filter_devices();
        Ok(())
    }
    pub fn from_candidates(
        devices: Vec<AvailableGraphicsDevice>,
        policy: DeviceSelectionPolicy,
    ) -> Self {
        let mut result = Self {
            devices,
            policy,
            available: vec![],
            missing: vec![],
        };
        result.order_and_filter_devices();
        result
    }
    pub fn missing_devices(&self) -> &[(usize, Vec<String>)] {
        &self.missing
    }
    pub fn available_devices(&self) -> impl Iterator<Item = &AvailableGraphicsDevice> {
        self.available.iter().map(|index| &self.devices[*index])
    }
    pub fn order_and_filter_devices(&mut self) {
        self.available.clear();
        self.missing.clear();
        for (index, device) in self.devices.iter().enumerate() {
            let mut reasons = device.errors.clone();
            if device.api_version < vk::API_VERSION_1_2 {
                reasons.push("缺少Vulkan1.2".into());
            }
            if !device.timeline_semaphore {
                reasons.push("缺少timeline semaphore".into());
            }
            if device.graphics_families.is_empty() {
                reasons.push("缺少graphics queue".into());
            }
            if self.policy.requires_surface
                && !device.present_families.as_ref().is_some_and(|families| {
                    families
                        .iter()
                        .any(|i| device.graphics_families.contains(i))
                })
            {
                reasons.push("无graphics+presentation队列".into());
            }
            for extension in &self.policy.required_extensions {
                if !device.extensions.contains(extension) {
                    reasons.push(format!("缺少device扩展 {extension}"));
                }
            }
            let compatible = device.is_compatible();
            match device.is_supported() {
                DeviceDecision::Yes => {}
                DeviceDecision::No(reason) => {
                    // compatible 判定按策略可豁免 is_supported 的拒绝原因。
                    let waived = (self.policy.allow_compatible
                        && compatible == DeviceDecision::Yes)
                        || (self.policy.allow_unknown_original_policy
                            && matches!(compatible, DeviceDecision::Unknown(_)));
                    if !waived {
                        reasons.push(reason)
                    }
                }
                DeviceDecision::Unknown(reason) => {
                    if !self.policy.allow_unknown_original_policy {
                        reasons.push(reason)
                    }
                }
            }
            if device.is_emulated() {
                reasons.push("原emulated设备过滤".into());
            }
            if reasons.is_empty() {
                self.available.push(index);
            } else {
                self.missing.push((index, reasons));
            }
        }
        // 原sort比较器未恢复；候选明确排序：原支持/兼容优先，再按设备类型与真实budget/heap size稳定排序。
        self.available.sort_by_key(|index| {
            let d = &self.devices[*index];
            (
                std::cmp::Reverse(match d.is_supported() {
                    DeviceDecision::Yes => 2,
                    _ => usize::from(d.is_compatible() == DeviceDecision::Yes),
                }),
                std::cmp::Reverse(match d.device_type {
                    vk::PhysicalDeviceType::DISCRETE_GPU => 3,
                    vk::PhysicalDeviceType::INTEGRATED_GPU => 2,
                    vk::PhysicalDeviceType::VIRTUAL_GPU => 1,
                    _ => 0,
                }),
                std::cmp::Reverse(d.memory.available_device_local_budget().unwrap_or(0)),
                std::cmp::Reverse(d.memory.device_local_bytes),
                d.index,
            )
        });
    }
    pub fn create_device(
        &self,
        graphics: &GraphicsInstance,
    ) -> std::result::Result<Arc<VulkanDevice>, Vec<String>> {
        let mut errors = self
            .missing
            .iter()
            .map(|(i, reasons)| format!("{}: {}", self.devices[*i].name, reasons.join("; ")))
            .collect::<Vec<_>>();
        for candidate in self.available_devices() {
            let Some(physical) = graphics
                .devices
                .get(candidate.index)
                .filter(|p| p.handle.as_raw() == candidate.handle)
            else {
                errors.push("设备清单身份已变".into());
                continue;
            };
            let family = candidate
                .graphics_families
                .iter()
                .copied()
                .find(|family| {
                    !self.policy.requires_surface
                        || candidate
                            .present_families
                            .as_ref()
                            .is_some_and(|p| p.contains(family))
                })
                .unwrap();
            let strings = self
                .policy
                .required_extensions
                .iter()
                .map(|s| std::ffi::CString::new(s.as_str()))
                .collect::<std::result::Result<Vec<_>, _>>()
                .map_err(|e| vec![e.to_string()])?;
            let names = strings.iter().map(|s| s.as_ptr()).collect::<Vec<_>>();
            match graphics.create_device(physical, family, &names) {
                Ok(device) => return Ok(device),
                Err(error) => errors.push(format!("{}: {error}", candidate.name)),
            }
        }
        Err(errors)
    }
}
pub struct GraphicsInstance {
    pub instance: Arc<VulkanInstance>,
    pub devices: Vec<PhysicalDevice>,
}
impl GraphicsInstance {
    pub fn new_with_allowlist(
        builder: &InstanceBuilder,
        allowlist: &GraphicsInstanceLayerAllowlist,
    ) -> Result<Self> {
        let instance = builder.build_with_allowlist(allowlist)?;
        let devices = enumerate_physical_devices(instance.clone())?;
        Ok(Self { instance, devices })
    }
    pub fn available_for_surface(
        &self,
        surface: Option<vk::SurfaceKHR>,
        mut policy: DeviceSelectionPolicy,
    ) -> Result<GraphicsDevicesForSurface> {
        if surface.is_some()
            && !self
                .instance
                .enabled_extensions
                .iter()
                .any(|e| e == "VK_KHR_surface")
        {
            return Err(Error::Invalid("实例未启用surface扩展"));
        }
        policy.requires_surface = surface.is_some();
        if surface.is_some()
            && !policy
                .required_extensions
                .iter()
                .any(|e| e == "VK_KHR_swapchain")
        {
            policy.required_extensions.push("VK_KHR_swapchain".into());
        }
        let mut candidates = vec![];
        for (index, physical) in self.devices.iter().enumerate() {
            let mut supported = vk::PhysicalDeviceVulkan12Features::default();
            let mut features = vk::PhysicalDeviceFeatures2::default().push_next(&mut supported);
            unsafe {
                self.instance
                    .instance
                    .get_physical_device_features2(physical.handle, &mut features);
            }
            let mut errors = vec![];
            let memory = match physical.performance_info() {
                Ok(memory) => memory,
                Err(error) => {
                    errors.push(format!("预算查询失败: {error}"));
                    physical.memory_snapshot(None)
                }
            };
            let present_families = surface
                .map(|s| physical.with_presentation_support(s))
                .transpose()
                .unwrap_or_else(|error| {
                    errors.push(format!("surface查询失败: {error}"));
                    Some(vec![])
                });
            let extensions = physical.extension_names().unwrap_or_else(|error| {
                errors.push(format!("扩展查询失败: {error}"));
                vec![]
            });
            let original_memory_metrics =
                memory
                    .device_local_budget
                    .zip(memory.host_budget)
                    .map(|(local, host)| DeviceMemoryMetrics {
                        local,
                        host,
                        shared: memory.unified_memory,
                    });
            candidates.push(AvailableGraphicsDevice {
                index,
                handle: physical.handle.as_raw(),
                name: physical.name(),
                device_type: physical.properties.device_type,
                driver: physical.driver_version(),
                memory,
                original_memory_metrics,
                api_version: physical.properties.api_version,
                timeline_semaphore: supported.timeline_semaphore != 0,
                graphics_families: physical
                    .queues
                    .iter()
                    .enumerate()
                    .filter(|(_, q)| {
                        q.queue_count > 0 && q.queue_flags.contains(vk::QueueFlags::GRAPHICS)
                    })
                    .map(|(i, _)| i as u32)
                    .collect(),
                present_families,
                extensions,
                errors,
            });
        }
        Ok(GraphicsDevicesForSurface::from_candidates(
            candidates, policy,
        ))
    }
    pub fn new() -> Result<Self> {
        let instance = InstanceBuilder::default().build()?;
        let devices = enumerate_physical_devices(Arc::clone(&instance))?;
        Ok(Self { instance, devices })
    }
    pub fn create_device(
        &self,
        physical: &PhysicalDevice,
        queue_family: u32,
        extensions: &[*const std::ffi::c_char],
    ) -> Result<Arc<VulkanDevice>> {
        if physical.properties.api_version < vk::API_VERSION_1_2 {
            return Err(Error::Unsupported("设备不支持 Vulkan 1.2"));
        }
        let mut supported = vk::PhysicalDeviceVulkan12Features::default();
        let mut features = vk::PhysicalDeviceFeatures2::default().push_next(&mut supported);
        unsafe {
            self.instance
                .instance
                .get_physical_device_features2(physical.handle, &mut features);
        }
        let core =
            vk::PhysicalDeviceFeatures::default().wide_lines(features.features.wide_lines != 0);
        if supported.timeline_semaphore == vk::FALSE {
            return Err(Error::Unsupported("设备缺少时间线信号量"));
        }
        if physical
            .queues
            .get(queue_family as usize)
            .is_none_or(|q| q.queue_count == 0)
        {
            return Err(Error::Invalid("请求的队列族不存在"));
        }
        let priorities = [1.0];
        let mut queues = vec![
            vk::DeviceQueueCreateInfo::default()
                .queue_family_index(queue_family)
                .queue_priorities(&priorities),
        ];
        // 追加独立 compute 队列；没有此能力时不伪造辅助队列。
        if let Some((family, _)) = physical.queues.iter().enumerate().find(|(family, q)| {
            *family != queue_family as usize
                && q.queue_count > 0
                && q.queue_flags.contains(vk::QueueFlags::COMPUTE)
                && !q.queue_flags.contains(vk::QueueFlags::GRAPHICS)
        }) {
            queues.push(
                vk::DeviceQueueCreateInfo::default()
                    .queue_family_index(family as u32)
                    .queue_priorities(&priorities),
            );
        }
        let mut enabled = vk::PhysicalDeviceVulkan12Features::default()
            .timeline_semaphore(true)
            .buffer_device_address(supported.buffer_device_address != 0)
            .descriptor_indexing(supported.descriptor_indexing != 0)
            .runtime_descriptor_array(supported.runtime_descriptor_array != 0)
            .descriptor_binding_partially_bound(supported.descriptor_binding_partially_bound != 0)
            .descriptor_binding_variable_descriptor_count(
                supported.descriptor_binding_variable_descriptor_count != 0,
            )
            .descriptor_binding_sampled_image_update_after_bind(
                supported.descriptor_binding_sampled_image_update_after_bind != 0,
            )
            .descriptor_binding_storage_buffer_update_after_bind(
                supported.descriptor_binding_storage_buffer_update_after_bind != 0,
            )
            .shader_sampled_image_array_non_uniform_indexing(
                supported.shader_sampled_image_array_non_uniform_indexing != 0,
            )
            .shader_storage_buffer_array_non_uniform_indexing(
                supported.shader_storage_buffer_array_non_uniform_indexing != 0,
            );
        let info = vk::DeviceCreateInfo::default()
            .queue_create_infos(&queues)
            .enabled_extension_names(extensions)
            .enabled_features(&core)
            .push_next(&mut enabled);
        Ok(VulkanDevice::create(
            Arc::clone(&self.instance),
            physical.handle,
            queue_family,
            &info,
        )?)
    }
}
pub fn create_headless_device() -> Result<Arc<VulkanDevice>> {
    let graphics = GraphicsInstance::new()?;
    graphics
        .available_for_surface(None, DeviceSelectionPolicy::default())?
        .create_device(&graphics)
        .map_err(|errors| Error::Loader(errors.join("\n")))
}
