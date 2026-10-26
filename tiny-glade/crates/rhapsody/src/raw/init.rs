//! 候选初始化路径以实际能力查询建立设备，不推测原作排序名单。
pub use crate::{Error, Result};
use super::physical_device::{
    DeviceDriverVersion, DriverVendor, GraphicsDeviceKind, PerformanceInfo,
};
use super::{
    device::Device,
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
    /// 原 +0x99：设备类别字节（见 [`GraphicsDeviceKind`]），比较器与内存门槛都按它分派。
    pub kind: GraphicsDeviceKind,
    /// 原 +0x88：DEVICE_LOCAL 堆**大小**之和（0x1418b3300 里求和的是堆 size，不是 budget）。
    pub local_heap_bytes: u64,
    /// 原 +0x90：非 DEVICE_LOCAL 堆大小之和。
    pub host_heap_bytes: u64,
    /// 原 +0x78..0x88 的 16B 键（create_device 按它匹配；来自 PhysicalDevice+0x690，日志里叫 uuid）。
    pub device_uuid: [u8; 16],
    /// 原前 16B 的 `Option<u64>`（tag@0 / 值@8，`missing_devices` 与 DXGI 适配器 u64 比较）；是 LUID 为候选。
    pub luid: Option<u64>,
    pub driver: DeviceDriverVersion,
    pub memory: PerformanceInfo,
    /// 由堆大小得到的原门槛输入（`shared = kind != Discrete`，见 `0x140bda8f0` 的 `[+0x99]==0` 分支）。
    pub original_memory_metrics: Option<DeviceMemoryMetrics>,
    pub api_version: u32,
    pub timeline_semaphore: bool,
    pub graphics_families: Vec<u32>,
    pub present_families: Option<Vec<u32>>,
    pub extensions: Vec<String>,
    pub errors: Vec<String>,
}
impl AvailableGraphicsDevice {
    /// 排序键（0x1418b9130 内联比较器，O）：`base - (local_heap_bytes >> 30)`，升序、严格小于才前移。
    /// base 由类别字节跳表给出：Discrete -4000 / Integrated -3000（这两类若 emulated 则都是 -2000），
    /// Virtual -2000，Cpu -1000，Other 0。emulated = 名含 "Direct3D12" 或 "Microsoft Basic Render Driver"，
    /// 只对 Discrete/Integrated 检查。
    pub fn device_order_key(&self) -> i64 {
        let base: i64 = match self.kind {
            GraphicsDeviceKind::Discrete => {
                if self.is_emulated() {
                    -2000
                } else {
                    -4000
                }
            }
            GraphicsDeviceKind::Integrated => {
                if self.is_emulated() {
                    -2000
                } else {
                    -3000
                }
            }
            GraphicsDeviceKind::Virtual => -2000,
            GraphicsDeviceKind::Cpu => -1000,
            GraphicsDeviceKind::Other => 0,
        };
        base - (self.local_heap_bytes >> 30) as i64
    }
    /// `order_and_filter_devices` 的 retain 谓词（0x1418b5680，O）：只看堆大小，与驱动/emulated 无关。
    pub fn meets_memory_threshold(&self) -> bool {
        meets_original_memory_threshold(self.kind, self.local_heap_bytes, self.host_heap_bytes)
    }
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

/// 原内存门槛（0x1418b5680 / 0x140bda8f0 同一判据）：类别字节==0（Discrete）要求 local≥1.0GB 且 host≥400MB；
/// 其余类别要求 local+host≥1.4GB（十进制，`0x3b9aca00` / `0x17d78400` / `0x53724e00`）。
pub fn meets_original_memory_threshold(kind: GraphicsDeviceKind, local: u64, host: u64) -> bool {
    if kind == GraphicsDeviceKind::Discrete {
        local >= 1_000_000_000 && host >= 400_000_000
    } else {
        local.wrapping_add(host) >= 1_400_000_000
    }
}
/// 对设备表做原排序：稳定升序（比较器严格 `<`）。原版 `<0x15` 个用插入排序，≥0x15 个走
/// `FUN_1418bb250`——它分配暂存区并以 `eager_sort = len<0x41` 调 `FUN_14213bee0`，是 Rust std 的 driftsort
/// （稳定 `slice::sort_by`；**不是**文档所说的不稳定 introsort）。两条路径都稳定，因此 tie 保持枚举顺序，
/// 结果与实现无关，直接用（同为稳定的）`sort_by_key` 即得到同一排列。
pub fn sort_devices_original(devices: &mut [AvailableGraphicsDevice]) {
    devices.sort_by_key(|a| a.device_order_key());
}

/// 系统（DXGI）适配器：原 `missing_devices` 的输入（FUN_1418b29c0 枚举，0x28B 元素：
/// name String / u64@+0x18 / vendorId u32@+0x20 / u32@+0x24）。DXGI 是 Windows API，清单由调用方给出。
///
/// 第三批逐指令核对（0x1418b2ad0..0x1418b2c71）：`DXGI_ADAPTER_DESC`（0x130B，`memcpy` 到 `rbp-0x50`）只取三项：
/// `Description[128]`→`name`（+0x00..0x17 String）、`AdapterLuid{Low@+0x128, High@+0x12c}`→
/// `(High<<32)|Low`（`+0x18` qword，即 `luid`）、`VendorId@+0x100`→`+0x20` dword（`mov r15d,[rbp+0xb0]`）。
/// **`DeviceId`（DESC+0x104）原版没有取**；记录的 `+0x24..0x28` 是未写入的尾部填充。所以 `device_id` 只是
/// 本实现的候选字段，`missing_devices` 不读它。
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct DxgiAdapter {
    pub name: String,
    pub luid: u64,
    pub vendor_id: u32,
    /// 原版不保存（见上），仅作调用方信息。
    pub device_id: u32,
}
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum MissingVendor {
    /// 0x10de -> 低字节 0
    Nvidia,
    /// 0x1002 -> 低字节 1（原值 0x1001 的低字节）
    Amd,
}
/// 原 0xf0B 的 missing 条目：tag 2 = Vulkan 里找不到的 DXGI 适配器，tag 3 = 被内存门槛筛掉的设备。
#[derive(Clone, Debug)]
pub enum MissingGraphicsDevice {
    NotInVulkan { name: String, vendor: MissingVendor },
    InsufficientMemory(Box<AvailableGraphicsDevice>),
}
impl MissingGraphicsDevice {
    pub fn original_tag(&self) -> u64 {
        match self {
            Self::NotInVulkan { .. } => 2,
            Self::InsufficientMemory(_) => 3,
        }
    }
}
/// 原 `GraphicsDevicesForSurface::missing_devices`（0x140bdb090，O）。
/// 只看 vendor 0x1002/0x10de 的适配器；若某 Vulkan 设备满足 `luid==Some(adapter.luid)` 或
/// 名字逐字节相同，则该适配器不算缺失；其余记 `NotInVulkan`。其他厂商的适配器直接丢弃。
pub fn missing_devices(
    devices: &[AvailableGraphicsDevice],
    adapters: &[DxgiAdapter],
) -> Vec<MissingGraphicsDevice> {
    log::debug!("Enumerating DXGI devices");
    log::debug!("DXGI adapters:");
    let mut missing = Vec::new();
    for adapter in adapters {
        log::debug!("{adapter:?}");
        let vendor = match adapter.vendor_id {
            0x1002 => MissingVendor::Amd,
            0x10de => MissingVendor::Nvidia,
            _ => continue,
        };
        let found = devices
            .iter()
            .any(|d| d.luid == Some(adapter.luid) || d.name == adapter.name);
        if !found {
            missing.push(MissingGraphicsDevice::NotInVulkan {
                name: adapter.name.clone(),
                vendor,
            });
        }
    }
    missing
}
/// 原 `GraphicsDevicesForSurface::order_and_filter_devices`（0x140bdbc30）的字面形态：
/// 先稳定排序，再 `retain` 内存门槛；被筛掉的设备以 tag 3 追加进 `missing`（调用方应已先跑 `missing_devices`）。
pub fn order_and_filter_devices_original(
    devices: &mut Vec<AvailableGraphicsDevice>,
    missing: &mut Vec<MissingGraphicsDevice>,
) {
    if devices.len() >= 2 {
        sort_devices_original(devices);
    }
    let mut kept = Vec::with_capacity(devices.len());
    for device in devices.drain(..) {
        if device.meets_memory_threshold() {
            kept.push(device);
        } else {
            missing.push(MissingGraphicsDevice::InsufficientMemory(Box::new(device)));
        }
    }
    *devices = kept;
}

/// 原 `AvailableGraphicsDeviceInitState`：只有 `is_error`（0x140a9fa70，`cmp byte [rcx],1; sete al`）被证实，
/// 其余判别值/载荷未证实，故只保存判别字节。
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct AvailableGraphicsDeviceInitState {
    pub tag: u8,
}
impl AvailableGraphicsDeviceInitState {
    pub fn is_error(&self) -> bool {
        self.tag == 1
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
    /// 原 `available_devices`（0x140bdb820）返回物化的 `Vec<AvailableGraphicsDevice>`（逐项克隆，不是迭代器）。
    pub fn available_devices(&self) -> Vec<AvailableGraphicsDevice> {
        self.available
            .iter()
            .map(|index| self.devices[*index].clone())
            .collect()
    }
    /// 排序 + 过滤。排序是原比较器（稳定、`device_order_key` 升序，见 [`sort_devices_original`]），
    /// 原内存门槛（堆大小）对应原 retain；其余（api 版本/timeline/队列/surface/扩展/驱动）是候选策略，
    /// 原 `order_and_filter_devices` 并不做这些（驱动判定在其调用方）。
    pub fn order_and_filter_devices(&mut self) {
        self.available.clear();
        self.missing.clear();
        sort_devices_original(&mut self.devices);
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
            if !device.meets_memory_threshold() {
                // 原 retain 谓词（0x1418b5680）：被筛掉并记入 missing。
                reasons.push("原内存门槛不足（堆大小）".into());
            } else {
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
    }

}
pub struct GraphicsInstance {
    pub instance: Arc<VulkanInstance>,
    /// Enumerated adapter metadata used for selection and settings.
    pub devices: Vec<Arc<PhysicalDevice>>,
}

impl GraphicsInstance {
    pub fn new_with_allowlist(
        builder: &InstanceBuilder,
        allowlist: &GraphicsInstanceLayerAllowlist,
    ) -> Result<Self> {
        let instance = builder.build_with_allowlist(allowlist)?;
        let devices = Self::wrap_devices(enumerate_physical_devices(instance.clone())?);
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
            return Err(Error::invalid_usage("实例未启用surface扩展"));
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
            let kind = GraphicsDeviceKind::from_vk(physical.properties.device_type);
            let local_heap_bytes = physical.device_local_memory_bytes();
            let host_heap_bytes = physical.host_memory_bytes();
            let (device_uuid, luid) = physical.id_properties();
            candidates.push(AvailableGraphicsDevice {
                index,
                handle: physical.handle.as_raw(),
                name: physical.name(),
                device_type: physical.properties.device_type,
                kind,
                local_heap_bytes,
                host_heap_bytes,
                device_uuid,
                luid,
                driver: physical.driver_version(),
                memory,
                original_memory_metrics: Some(DeviceMemoryMetrics {
                    local: local_heap_bytes,
                    host: host_heap_bytes,
                    shared: kind != GraphicsDeviceKind::Discrete,
                }),
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
        let devices = Self::wrap_devices(enumerate_physical_devices(Arc::clone(&instance))?);
        Ok(Self { instance, devices })
    }
    fn wrap_devices(devices: Vec<PhysicalDevice>) -> Vec<Arc<PhysicalDevice>> {
        devices.into_iter().map(Arc::new).collect()
    }

}

pub fn create_headless_device() -> Result<Arc<Device>> {
    Device::new_headless(&crate::device::DeviceConfig::headless())
}

/// 创建离屏设备；拒绝 require_present，避免把没有 surface 的配置悄悄改写。
pub fn create_headless_device_configured(
    config: &crate::device::QueueConfig,
) -> Result<Arc<Device>> {
    Device::new_headless(&crate::device::DeviceConfig {
        queues: *config,
        ..crate::device::DeviceConfig::headless()
    })
}
