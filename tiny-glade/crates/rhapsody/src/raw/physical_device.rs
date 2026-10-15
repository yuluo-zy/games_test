//! 物理设备能力查询；不伪造原作偏好和兼容性名单。
use super::{error::Result, instance::VulkanInstance};
use arc_swap::ArcSwapOption;
use ash::vk;
use std::{
    ffi::CStr,
    sync::{Arc, OnceLock},
};
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
/// 原 AvailableGraphicsDevice+0x99 的设备类别字节（0x1418b3300 的转换：`0x302000104 >> (VkPhysicalDeviceType*8)`，
/// 类型值 ≥5 取 4）：Discrete=0, Integrated=1, Virtual=2, Cpu=3, Other=4。
/// 比较器（0x1418b9130 的跳表）与内存门槛（0x140bda8f0 / 0x1418b5680 的 `[+0x99]==0`）都按这个字节分派。
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum GraphicsDeviceKind {
    Discrete = 0,
    Integrated = 1,
    Virtual = 2,
    Cpu = 3,
    Other = 4,
}
impl GraphicsDeviceKind {
    pub fn from_vk(device_type: vk::PhysicalDeviceType) -> Self {
        match device_type {
            vk::PhysicalDeviceType::DISCRETE_GPU => Self::Discrete,
            vk::PhysicalDeviceType::INTEGRATED_GPU => Self::Integrated,
            vk::PhysicalDeviceType::VIRTUAL_GPU => Self::Virtual,
            vk::PhysicalDeviceType::CPU => Self::Cpu,
            _ => Self::Other,
        }
    }
}

/// 原 `refresh_memory_budget` 发布的 0x110B 快照：就是一份 `VkPhysicalDeviceMemoryBudgetPropertiesEXT`
/// （sType@+0 / pNext@+8 / heapBudget[16]@+0x10 / heapUsage[16]@+0x90，每项 u64）。
/// 旧文档的“{budget u128, usage u128} 步长 0x20”不对：`device_local_memory_budget_bytes`（0x140c006a0）
/// 标量循环以 8B 步长读 `base+0x20+8i`（= 载荷+0x10+8i），即 heapBudget 数组。
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct MemoryBudgetSnapshot {
    pub heap_budget: [u64; MAX_MEMORY_HEAPS],
    pub heap_usage: [u64; MAX_MEMORY_HEAPS],
}
/// 原版断言 `memory_heap_count <= 16`（0x140c006a0 里 `0x10 < count` 走 slice 越界 panic）。
pub const MAX_MEMORY_HEAPS: usize = 16;
/// 快照载荷字节数（`func_142923c00(.., 0x110)`，ArcSwap 分配 0x120B = 2×u64 计数 + 载荷）。
pub const BUDGET_SNAPSHOT_BYTES: usize = 0x110;
impl MemoryBudgetSnapshot {
    pub fn from_vk(budget: &vk::PhysicalDeviceMemoryBudgetPropertiesEXT<'_>) -> Self {
        Self {
            heap_budget: budget.heap_budget,
            heap_usage: budget.heap_usage,
        }
    }
    pub fn to_vk(self) -> vk::PhysicalDeviceMemoryBudgetPropertiesEXT<'static> {
        let mut out = vk::PhysicalDeviceMemoryBudgetPropertiesEXT::default();
        out.heap_budget = self.heap_budget;
        out.heap_usage = self.heap_usage;
        out
    }
    /// 16 个同构钳位块：只钳 budget，usage 原样取驱动值。
    pub fn clamped_against(mut self, old: &Self) -> Self {
        for i in 0..MAX_MEMORY_HEAPS {
            self.heap_budget[i] = clamp_budget_hysteresis(old.heap_budget[i], self.heap_budget[i]);
        }
        self
    }
    /// getter 的求和：`want_device_local` 为 true 取 DEVICE_LOCAL 堆，否则取其余堆（0x140c006a0 / 0x140c00890）。
    pub fn sum_budget(&self, heaps: &[vk::MemoryHeap], want_device_local: bool) -> u64 {
        heaps
            .iter()
            .take(MAX_MEMORY_HEAPS)
            .zip(self.heap_budget)
            .filter(|(h, _)| h.flags.contains(vk::MemoryHeapFlags::DEVICE_LOCAL) == want_device_local)
            .fold(0u64, |sum, (_, b)| sum.wrapping_add(b))
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct PerformanceInfo {
    /// 原快照布局（驱动支持 VK_EXT_memory_budget 时才有）。
    pub snapshot: Option<MemoryBudgetSnapshot>,
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
/// `refresh_memory_budget` 的单堆钳位：`delta = max(1, old >> 3)`，
/// `lo = old - delta`（不足则 0），`hi = old + delta`，结果 `clamp(new, lo, hi)`。
pub fn clamp_budget_hysteresis(old: u64, new: u64) -> u64 {
    let delta = (old >> 3).max(1);
    let lo = old.saturating_sub(delta);
    let hi = old.saturating_add(delta);
    new.max(lo).min(hi)
}
pub struct PhysicalDevice {
    pub instance: Arc<VulkanInstance>,
    pub handle: vk::PhysicalDevice,
    pub properties: vk::PhysicalDeviceProperties,
    pub memory: vk::PhysicalDeviceMemoryProperties,
    pub queues: Vec<vk::QueueFamilyProperties>,
    /// 最近一次预算快照。原作 `refresh_memory_budget` 经 `ArcSwapAny::store/swap` 写入，
    /// `*_memory_budget_bytes` 经 `ArcSwapAny::load` 读取（PDB 注解）；快照的字段内容是候选。
    budget_snapshot: ArcSwapOption<PerformanceInfo>,
    /// 设备扩展集合在物理设备生命周期内不变，只枚举一次。
    memory_budget_extension: OnceLock<bool>,
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
    /// 向驱动重新查询并发布新快照。
    pub fn performance_info(&self) -> Result<PerformanceInfo> {
        let budget = self.refresh_memory_budget()?;
        let info = self.memory_snapshot(budget.as_ref());
        self.budget_snapshot.store(Some(Arc::new(info.clone())));
        Ok(info)
    }
    /// 无锁读取最近一次快照；从未刷新过则为 `None`。
    pub fn cached_performance_info(&self) -> Option<Arc<PerformanceInfo>> {
        self.budget_snapshot.load_full()
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
                    .fold(0u64, u64::wrapping_add)
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
            snapshot: budget.map(MemoryBudgetSnapshot::from_vk),
            device_local_bytes: sum(true),
            host_bytes: sum(false),
            device_local_budget: sum_budget(true),
            host_budget: sum_budget(false),
            unified_memory: unified,
            heaps,
            vendor_specific: None,
        }
    }
    /// 读快照；只有从未刷新过才会查询驱动。要最新值先调 `performance_info`/`refresh_memory_budget`。
    fn snapshot_or_refresh(&self) -> Result<Arc<PerformanceInfo>> {
        match self.cached_performance_info() {
            Some(info) => Ok(info),
            None => {
                self.performance_info()?;
                Ok(self.cached_performance_info().expect("刚发布过快照"))
            }
        }
    }
    pub fn device_local_memory_budget_bytes(&self) -> Result<Option<u64>> {
        Ok(self.snapshot_or_refresh()?.device_local_budget)
    }
    pub fn host_memory_budget_bytes(&self) -> Result<Option<u64>> {
        Ok(self.snapshot_or_refresh()?.host_budget)
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
        if !self.supports_memory_budget()? {
            return Ok(None);
        }
        let mut budget = vk::PhysicalDeviceMemoryBudgetPropertiesEXT::default();
        let mut properties = vk::PhysicalDeviceMemoryProperties2::default().push_next(&mut budget);
        unsafe {
            self.instance
                .instance
                .get_physical_device_memory_properties2(self.handle, &mut properties);
        }
        // 逐堆滞回（0x140c01620，16 处同构钳位块）：新预算限制在上一份快照的
        // `[old - max(1, old>>3), old + max(1, old>>3)]` 内（只钳 budget；usage 取驱动值）。
        // 首次没有旧快照则原样采用（原版在枚举阶段先放入一份初始快照，这里以“无旧快照”等价）。
        if let Some(old) = self.cached_performance_info().and_then(|p| p.snapshot) {
            let clamped = MemoryBudgetSnapshot::from_vk(&budget).clamped_against(&old);
            budget.heap_budget = clamped.heap_budget;
        }
        Ok(Some(budget))
    }
    fn supports_memory_budget(&self) -> Result<bool> {
        if let Some(&supported) = self.memory_budget_extension.get() {
            return Ok(supported);
        }
        let extensions = unsafe {
            self.instance
                .instance
                .enumerate_device_extension_properties(self.handle)
        }?;
        let supported = extensions.iter().any(|e| {
            let name = unsafe { CStr::from_ptr(e.extension_name.as_ptr()) };
            name == ash::ext::memory_budget::NAME
        });
        Ok(*self.memory_budget_extension.get_or_init(|| supported))
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
    /// 原 `find_vendor_specific_device_handle`（0x140c01e80，O）：仅 vendorID==0x10de 才继续；
    /// 遍历厂商库（NVAPI，FUN_1418b1340）枚举出的物理 GPU 清单，对每项取 PCI 标识（FUN_1418b17b0）：
    /// 取不到 → 记 warn "NVAPI: No PCI identifier" 并跳过；`pci.device_id == VkPhysicalDeviceProperties.deviceID`
    /// → 记 info "Matched to NVAPI device" 并返回该项句柄。非 NVIDIA / 无匹配 → None
    /// （原版此时记 "No vendor-specific info" 后返回 Err 判别 1；日志级别未证实）。
    /// NVAPI 是 Windows 专有库，清单由调用方提供，这里不重新实现厂商库。
    pub fn find_vendor_specific_device_handle(&self, nvapi_devices: &[VendorDeviceEntry]) -> Option<u64> {
        match_vendor_device_handle(self.properties.vendor_id, self.properties.device_id, nvapi_devices)
    }
    /// 设备 LUID（`VkPhysicalDeviceIDProperties`），无效时 None。原 AvailableGraphicsDevice 前 16B 是
    /// `Option<u64>`（tag@0 / 值@8），`missing_devices` 以它与 DXGI 适配器的 u64@+0x18 比较；
    /// “这个 u64 是 LUID”是候选（证据只到“同一个 8B 标识”）。
    pub fn luid(&self) -> Option<u64> {
        self.id_properties().1
    }
    /// (deviceUUID, LUID)。原 PhysicalDevice+0x690 的 16B 键（日志里叫 uuid）取 deviceUUID 是候选：
    /// 证据只到“16B、create_device 按它匹配、日志标 uuid”。
    pub fn id_properties(&self) -> ([u8; 16], Option<u64>) {
        let mut id = vk::PhysicalDeviceIDProperties::default();
        let mut properties = vk::PhysicalDeviceProperties2::default().push_next(&mut id);
        unsafe {
            self.instance
                .instance
                .get_physical_device_properties2(self.handle, &mut properties);
        }
        (
            id.device_uuid,
            (id.device_luid_valid != 0).then(|| u64::from_le_bytes(id.device_luid)),
        )
    }
    /// 原 `performance_info`（0x140c00a80，O）：**不是**堆预算，而是厂商库（NVAPI）遥测文本。
    /// 没有厂商特定信息（原 `[info+0x10]&1==0`）→ "Unsupported"；有则最多 6 段、以换行连接。
    /// 6 次厂商查询的值文本由厂商库类型的 Debug 生成，这里由调用方提供。
    pub fn vendor_performance_info(&self, queries: Option<&VendorPerformanceQueries>) -> String {
        format_vendor_performance_info(queries)
    }
}

/// 厂商库（NVAPI）枚举出的一个物理 GPU：句柄 + PCI 标识查询结果（None = 查询失败）。
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct VendorDeviceEntry {
    pub handle: u64,
    pub pci_device_id: Option<u32>,
}
/// `find_vendor_specific_device_handle` 的纯判定部分。
pub fn match_vendor_device_handle(
    vendor_id: u32,
    device_id: u32,
    entries: &[VendorDeviceEntry],
) -> Option<u64> {
    if vendor_id != 0x10de {
        return None;
    }
    for entry in entries {
        match entry.pci_device_id {
            None => log::warn!("NVAPI: No PCI identifier"),
            Some(id) if id == device_id => {
                log::info!("Matched to NVAPI device");
                return Some(entry.handle);
            }
            Some(_) => {}
        }
    }
    None
}
/// `performance_info` 的 6 次厂商查询结果（顺序固定，None = 该次查询失败/不适用而被跳过）：
/// FUN_1418b1a40 -> "Clocks: "；FUN_1418b1bb0（返回 0 才输出）-> "P-state: "；FUN_1418b1cd0 -> "Utilization: "；
/// FUN_1418b1e40 -> "Power: "；FUN_1418b18f0 -> "Memory: "（NVAPI MemoryInfo 的 Debug）；FUN_1418b1fb0 -> "Sensors: "。
/// 值格式化闭包 FUN_1418c03b0/c02b0/b6390/bffd70/b6310 未逐个读，故值文本由调用方给出。
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct VendorPerformanceQueries {
    pub clocks: Option<String>,
    pub p_state: Option<String>,
    pub utilization: Option<String>,
    pub power: Option<String>,
    pub memory: Option<String>,
    pub sensors: Option<String>,
}
pub fn format_vendor_performance_info(queries: Option<&VendorPerformanceQueries>) -> String {
    let Some(q) = queries else {
        return "Unsupported".to_owned();
    };
    [
        ("Clocks: ", &q.clocks),
        ("P-state: ", &q.p_state),
        ("Utilization: ", &q.utilization),
        ("Power: ", &q.power),
        ("Memory: ", &q.memory),
        ("Sensors: ", &q.sensors),
    ]
    .iter()
    .filter_map(|(label, value)| value.as_ref().map(|v| format!("{label}{v}")))
    .collect::<Vec<_>>()
    .join("\n")
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
    unsafe {
        std::str::from_utf8_unchecked(std::slice::from_raw_parts(bytes.as_ptr().cast(), length))
    }
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
            budget_snapshot: ArcSwapOption::empty(),
            memory_budget_extension: OnceLock::new(),
            handle,
        })
        .collect())
}

#[cfg(test)]
mod hysteresis_tests {
    use super::clamp_budget_hysteresis as clamp;

    #[test]
    fn eighth_window_with_unit_floor() {
        assert_eq!(clamp(800, 10_000), 900);
        assert_eq!(clamp(800, 0), 700);
        assert_eq!(clamp(800, 850), 850);
        // old < 8: delta 取 1，而不是 old/8 == 0。
        assert_eq!(clamp(5, 100), 6);
        assert_eq!(clamp(5, 0), 4);
        assert_eq!(clamp(0, 100), 1);
        assert_eq!(clamp(u64::MAX, 0), u64::MAX - (u64::MAX >> 3));
    }
}
