//! 实际加载器 fallback、实例能力检查与显式 layer 白名单接口；原隐式 layer 注入控制仍未知。
use super::error::{Error, Result};
use ash::{Entry, vk};
use std::{
    ffi::{CStr, CString},
    sync::Arc,
};
pub struct VulkanInstance {
    pub entry: Entry,
    pub instance: ash::Instance,
    pub enabled_extensions: Vec<String>,
    pub layers: Vec<String>,
}
impl VulkanInstance {
    /// 原入口：已启用 layer 名称列表。
    pub fn layer_names(&self) -> &[String] {
        &self.layers
    }
}
#[derive(Clone, Debug)]
pub struct LoaderAttempt {
    pub candidate: String,
    pub error: Option<String>,
}
pub struct GraphicsInstanceLoader {
    pub entry: Entry,
    pub api_version: u32,
    pub extensions: Vec<String>,
    pub layers: Vec<super::init::GraphicsInstanceLayer>,
    pub attempts: Vec<LoaderAttempt>,
}
impl GraphicsInstanceLoader {
    pub fn new() -> Result<Self> {
        Self::from_candidates(&[])
    }
    /// 先尝试调用者提供的 loader，再回退到 ash 系统 loader；失败原因不被丢弃。
    pub fn from_candidates(candidates: &[std::path::PathBuf]) -> Result<Self> {
        let mut attempts = vec![];
        let mut selected = None;
        for candidate in candidates {
            match unsafe { Entry::load_from(candidate) } {
                Ok(entry) => {
                    attempts.push(LoaderAttempt {
                        candidate: candidate.to_string_lossy().into_owned(),
                        error: None,
                    });
                    selected = Some(entry);
                    break;
                }
                Err(error) => attempts.push(LoaderAttempt {
                    candidate: candidate.to_string_lossy().into_owned(),
                    error: Some(error.to_string()),
                }),
            }
        }
        let entry = if let Some(entry) = selected {
            entry
        } else {
            match unsafe { Entry::load() } {
                Ok(entry) => {
                    attempts.push(LoaderAttempt {
                        candidate: "system Vulkan loader".into(),
                        error: None,
                    });
                    entry
                }
                Err(error) => {
                    return Err(Error::unsupported(format!(
                        "{attempts:?}; system loader: {error}"
                    )));
                }
            }
        };
        let api_version =
            unsafe { entry.try_enumerate_instance_version() }?.unwrap_or(vk::API_VERSION_1_0);
        let extensions = unsafe { entry.enumerate_instance_extension_properties(None) }?
            .iter()
            .map(|p| {
                unsafe { CStr::from_ptr(p.extension_name.as_ptr()) }
                    .to_string_lossy()
                    .into_owned()
            })
            .collect();
        let layers = unsafe { entry.enumerate_instance_layer_properties() }?
            .iter()
            .map(|p| super::init::GraphicsInstanceLayer {
                name: unsafe { CStr::from_ptr(p.layer_name.as_ptr()) }
                    .to_string_lossy()
                    .into_owned(),
                description: unsafe { CStr::from_ptr(p.description.as_ptr()) }
                    .to_string_lossy()
                    .into_owned(),
            })
            .collect();
        Ok(Self {
            entry,
            api_version,
            extensions,
            layers,
            attempts,
        })
    }
    /// Vulkan枚举不能区分implicit/explicit来源；不把全部枚举层冒充implicit清单。
    pub fn implicit_layers(&self) -> Option<&[super::init::GraphicsInstanceLayer]> {
        None
    }
}
impl Drop for VulkanInstance {
    fn drop(&mut self) {
        unsafe {
            self.instance.destroy_instance(None);
        }
    }
}
pub struct InstanceBuilder {
    pub application_name: CString,
    pub api_version: u32,
    pub extensions: Vec<CString>,
    pub layers: Vec<CString>,
}
impl Default for InstanceBuilder {
    fn default() -> Self {
        Self {
            application_name: CString::new("rhapsody-rebuild").unwrap(),
            api_version: vk::API_VERSION_1_2,
            extensions: Vec::new(),
            layers: Vec::new(),
        }
    }
}
impl InstanceBuilder {
    pub fn build(&self) -> Result<Arc<VulkanInstance>> {
        self.build_with_loader(GraphicsInstanceLoader::new()?)
    }
    pub fn build_with_allowlist(
        &self,
        allowlist: &super::init::GraphicsInstanceLayerAllowlist,
    ) -> Result<Arc<VulkanInstance>> {
        for name in &self.layers {
            let layer = super::init::GraphicsInstanceLayer {
                name: name.to_string_lossy().into_owned(),
                description: String::new(),
            };
            if !allowlist.permits(&layer) {
                return Err(Error::unsupported(format!("layer不在白名单: {}", layer.name)));
            }
        }
        self.build()
    }
    pub fn build_with_loader(&self, loader: GraphicsInstanceLoader) -> Result<Arc<VulkanInstance>> {
        if loader.api_version < self.api_version {
            return Err(Error::unsupported("Vulkan 版本低于请求版本"));
        }
        for layer in &self.layers {
            if !loader
                .layers
                .iter()
                .any(|p| p.name == layer.to_string_lossy())
            {
                return Err(Error::unsupported(format!(
                    "请求layer不可用: {}",
                    layer.to_string_lossy()
                )));
            }
        }
        let entry = loader.entry;
        let mut available = unsafe { entry.enumerate_instance_extension_properties(None) }?;
        for layer in &self.layers {
            available.extend(unsafe {
                entry.enumerate_instance_extension_properties(Some(layer.as_c_str()))
            }?);
        }
        for extension in DEFAULT_INSTANCE_EXTENSIONS
            .iter()
            .map(|name| CString::new(*name).unwrap())
            .chain(self.extensions.iter().cloned())
        {
            if !available.iter().any(
                |p| unsafe { CStr::from_ptr(p.extension_name.as_ptr()) } == extension.as_c_str(),
            ) {
                return Err(Error::unsupported(format!(
                    "请求instance扩展不可用: {}",
                    extension.to_string_lossy()
                )));
            }
        }
        let extensions = effective_extensions(&self.extensions, &available);
        let extension_ptrs: Vec<_> = extensions.iter().map(|v| v.as_ptr()).collect();
        let layers: Vec<_> = self.layers.iter().map(|v| v.as_ptr()).collect();
        let app = vk::ApplicationInfo::default()
            .application_name(&self.application_name)
            .engine_name(c"rhapsody")
            .api_version(self.api_version);
        let info = vk::InstanceCreateInfo::default()
            .application_info(&app)
            .enabled_extension_names(&extension_ptrs)
            .enabled_layer_names(&layers);
        let instance = unsafe { entry.create_instance(&info, None) }?;
        Ok(Arc::new(VulkanInstance {
            entry,
            instance,
            enabled_extensions: extensions
                .iter()
                .map(|s| s.to_string_lossy().into_owned())
                .collect(),
            layers: self
                .layers
                .iter()
                .map(|s| s.to_string_lossy().into_owned())
                .collect(),
        }))
    }
    /// 启用 `VK_EXT_debug_utils`（debug marker/debug messenger 所需）。
    pub fn debug_markers(&mut self) {
        let marker = CString::new("VK_EXT_debug_utils").unwrap();
        if !self.extensions.iter().any(|e| e == &marker) {
            self.extensions.push(marker);
        }
    }
    /// 启用 Khronos 校验层 + `VK_EXT_debug_utils`（后者无论校验层是否存在都启用，供 [`ValidationMessenger`] 使用）。
    /// 校验层不可用时返回 `Ok(false)` 且不加入请求列表；这是测试/诊断选项，原版没有等价入口。
    pub fn enable_validation(&mut self) -> Result<bool> {
        let loader = GraphicsInstanceLoader::new()?;
        self.debug_markers();
        let layer = CString::new(KHRONOS_VALIDATION_LAYER).unwrap();
        if !loader.layers.iter().any(|l| l.name == KHRONOS_VALIDATION_LAYER) {
            return Ok(false);
        }
        if !self.layers.contains(&layer) {
            self.layers.push(layer);
        }
        Ok(true)
    }
    /// 从请求列表中移除指定层（若存在）。
    pub fn legacy_disable_layer(&mut self, name: &str) {
        self.layers.retain(|layer| layer.to_string_lossy() != name);
    }
}

/// `apply_vulkan_blocklist`（0x140c0a530）里 16 个 `DISABLE_*` 环境变量，按原调用顺序。
/// 名字池 0x142b1951e.. 切片连续；`EOS_OVERLAY_DISABLE_VULKAN_WIN64` 是单个 32 字节名。
pub const VULKAN_LAYER_DISABLE_ENV: [&str; 16] = [
    "DISABLE_RTSS_LAYER",
    "DISABLE_VKBASALT",
    "VK_LAYER_bandicam_helper_DEBUG_1",
    "NODEVICE_SELECT",
    "DISABLE_GAMEPP_LAYER",
    "DISABLE_SAMPLE_LAYER",
    "DISABLE_LAYER_NV_OPTIMUS_1",
    "DISABLE_LAYER_AMD_SWITCHABLE_GRAPHICS_1",
    "EOS_OVERLAY_DISABLE_VULKAN_WIN64",
    "DISABLE_VULKAN_OW_OBS_CAPTURE",
    "DISABLE_VULKAN_OW_OVERLAY_LAYER",
    "DISABLE_VK_LAYER_reshade_1",
    "DISABLE_TWITCH_VULKAN_OVERLAY",
    "DISABLE_MANGOHUD",
    "DISABLE_LAYER",
    "DISABLE_VK_LAYER_gshade_1",
];

/// `InstanceBuilder::build`（0x140c08f10）无条件启用的两个 instance 扩展：`Vec<&str>` 初值，
/// 指针 0x142b1909b / 0x142b190c2（字符串池读出）。其后按 builder 标志追加 `VK_EXT_debug_utils`（0x142b190e3），
/// 再接调用方给的扩展表。
pub const DEFAULT_INSTANCE_EXTENSIONS: [&str; 2] = [
    "VK_KHR_get_physical_device_properties2",
    "VK_KHR_get_surface_capabilities2",
];
/// 原版在扩展可用性检查里对 `VK_EXT_layer_settings`（22 字节含 NUL 的 memcmp，常量 0x142b18de0/df0）做探测：
/// 可用则 info 日志并追加启用（0x142b19210），不可用则 info 日志跳过，二者都不致命。
pub const LAYER_SETTINGS_EXTENSION: &str = "VK_EXT_layer_settings";

/// 默认扩展 + 调用方扩展（去重，保序）+ 探测到的 `VK_EXT_layer_settings`。
pub fn effective_extensions(requested: &[CString], available: &[vk::ExtensionProperties]) -> Vec<CString> {
    let mut out: Vec<CString> = Vec::new();
    for name in DEFAULT_INSTANCE_EXTENSIONS
        .iter()
        .map(|name| CString::new(*name).unwrap())
        .chain(requested.iter().cloned())
    {
        if !out.contains(&name) {
            out.push(name);
        }
    }
    let layer_settings = CString::new(LAYER_SETTINGS_EXTENSION).unwrap();
    let present = available
        .iter()
        .any(|p| unsafe { CStr::from_ptr(p.extension_name.as_ptr()) } == layer_settings.as_c_str());
    if present {
        log::info!("{LAYER_SETTINGS_EXTENSION} is available, enabling it");
        if !out.contains(&layer_settings) {
            out.push(layer_settings);
        }
    } else {
        log::info!("{LAYER_SETTINGS_EXTENSION} is not available");
    }
    out
}

/// `VK_LOADER_LAYERS_ALLOW` 的值（0x140c0a530 前半段，机器码证实）：入参是 `GraphicsInstanceLayerAllowlist`
/// 的条目表（32B 元素 `{tag@0, String@8}`，`tag & 1`：0 = Layer，1 = Vendor；与 `contains_layer/contains_vendor` 的
/// 判别一致）。逐条：`bit0 == 1`（Vendor）→ `format!("VK_LAYER_{vendor}*")`（pieces `"VK_LAYER_"`,`"*"` @0x142b19478），
/// `bit0 == 0`（Layer）→ 层名原样克隆；再用 `","` 连接（FUN_1418cd6c0）。
/// 注：旧候选把入参当“发现的层名”并按 `VK_LAYER_` 前缀过滤，与机器码不符，已纠正。
pub fn allow_list_value(allowlist: &super::init::GraphicsInstanceLayerAllowlist) -> String {
    use super::init::GraphicsInstanceLayerAllowlistEntry as Entry;
    allowlist
        .entries
        .iter()
        .map(|entry| match entry {
            Entry::Vendor(vendor) => format!("VK_LAYER_{vendor}*"),
            Entry::Layer(name) => name.clone(),
        })
        .collect::<Vec<_>>()
        .join(",")
}

/// 原函数的环境变量写入点（`FUN_140d6c690` = `std::env::set_var` 包装）。
/// 任何一次写入失败，原函数都以 `failed to set environment variable` panic 结束。
pub fn apply_vulkan_blocklist_with(
    allowlist: &super::init::GraphicsInstanceLayerAllowlist,
    mut set_var: impl FnMut(&str, &str) -> std::io::Result<()>,
) {
    let mut set = |name: &str, value: &str| {
        if let Err(error) = set_var(name, value) {
            panic!("failed to set environment variable `{name}`: {error}");
        }
    };
    // 1. 先整体禁用 implicit 层。
    set("VK_LOADER_LAYERS_DISABLE", "~implicit~");
    // 2. 白名单 → 逗号连接后选择性重新启用（原版此处记 "Selectively enabling implicit Vulkan layers: {allow_list}"）。
    log::debug!("Selectively enabling implicit Vulkan layers: {}", allow_list_value(allowlist));
    set("VK_LOADER_LAYERS_ALLOW", &allow_list_value(allowlist));
    // 3. 16 个覆盖层各自写 "1"（原版每个前有 "[legacy] Force-disabling a Vulkan layer via {env}"）。
    for name in VULKAN_LAYER_DISABLE_ENV {
        log::debug!("[legacy] Force-disabling a Vulkan layer via {name}");
        set(name, "1");
    }
}

/// 真实入口：写进程环境变量。须在创建 Vulkan 实例之前、且尚无其他线程读环境时调用。
pub fn apply_vulkan_blocklist(allowlist: &super::init::GraphicsInstanceLayerAllowlist) {
    apply_vulkan_blocklist_with(allowlist, |name, value| {
        // SAFETY: 与原版同样的前置条件——实例创建前由初始化线程单次调用。
        unsafe { std::env::set_var(name, value) };
        Ok(())
    });
}

/// 创建 Vulkan debug messenger。`enabled_extensions` 必须已启用 `VK_EXT_debug_utils`。
/// 返回 loader 与 messenger 句柄；调用者负责用 loader 销毁。
pub fn vulkan_debug_messenger(
    instance: &VulkanInstance,
    severity: vk::DebugUtilsMessageSeverityFlagsEXT,
    types: vk::DebugUtilsMessageTypeFlagsEXT,
) -> Result<(ash::ext::debug_utils::Instance, vk::DebugUtilsMessengerEXT)> {
    let loader = ash::ext::debug_utils::Instance::new(&instance.entry, &instance.instance);
    if !instance
        .enabled_extensions
        .iter()
        .any(|name| name == "VK_EXT_debug_utils")
    {
        return Err(Error::unsupported("实例未启用 VK_EXT_debug_utils"));
    }
    // ash 需要函数指针；使用 no-op 回调，实际诊断仍经 marker/读回路径。
    unsafe extern "system" fn callback(
        _severity: vk::DebugUtilsMessageSeverityFlagsEXT,
        _types: vk::DebugUtilsMessageTypeFlagsEXT,
        _data: *const vk::DebugUtilsMessengerCallbackDataEXT,
        _user: *mut std::ffi::c_void,
    ) -> vk::Bool32 {
        vk::FALSE
    }
    let info = vk::DebugUtilsMessengerCreateInfoEXT::default()
        .message_severity(severity)
        .message_type(types)
        .pfn_user_callback(Some(callback));
    let handle = unsafe { loader.create_debug_utils_messenger(&info, None) }?;
    Ok((loader, handle))
}

pub const KHRONOS_VALIDATION_LAYER: &str = "VK_LAYER_KHRONOS_validation";

/// 收集到的一条 debug utils 消息。
#[derive(Clone, Debug)]
pub struct DebugMessage {
    pub severity: vk::DebugUtilsMessageSeverityFlagsEXT,
    pub types: vk::DebugUtilsMessageTypeFlagsEXT,
    pub id_name: String,
    pub message: String,
}

/// 可查询的消息列表；回调线程安全地追加。
#[derive(Default)]
pub struct ValidationCollector {
    messages: parking_lot::Mutex<Vec<DebugMessage>>,
}
impl ValidationCollector {
    pub fn all(&self) -> Vec<DebugMessage> {
        self.messages.lock().clone()
    }
    pub fn with_severity(&self, severity: vk::DebugUtilsMessageSeverityFlagsEXT) -> Vec<DebugMessage> {
        self.messages.lock().iter().filter(|m| m.severity.contains(severity)).cloned().collect()
    }
    pub fn errors(&self) -> Vec<DebugMessage> {
        self.with_severity(vk::DebugUtilsMessageSeverityFlagsEXT::ERROR)
    }
    pub fn warnings(&self) -> Vec<DebugMessage> {
        self.with_severity(vk::DebugUtilsMessageSeverityFlagsEXT::WARNING)
    }
    pub fn clear(&self) {
        self.messages.lock().clear();
    }
}

/// 把 debug utils 消息收集进 [`ValidationCollector`] 的 messenger；Drop 时销毁（先于实例）。
pub struct ValidationMessenger {
    loader: ash::ext::debug_utils::Instance,
    handle: vk::DebugUtilsMessengerEXT,
    pub collector: Arc<ValidationCollector>,
    _instance: Arc<VulkanInstance>,
}
impl ValidationMessenger {
    /// 收集 WARNING 与 ERROR（含 VERBOSE/INFO 以外的全部类型）。实例须已 `debug_markers()`。
    pub fn new(instance: &Arc<VulkanInstance>) -> Result<Self> {
        if !instance.enabled_extensions.iter().any(|name| name == "VK_EXT_debug_utils") {
            return Err(Error::unsupported("实例未启用 VK_EXT_debug_utils"));
        }
        let loader = ash::ext::debug_utils::Instance::new(&instance.entry, &instance.instance);
        let collector = Arc::new(ValidationCollector::default());
        unsafe extern "system" fn callback(
            severity: vk::DebugUtilsMessageSeverityFlagsEXT,
            types: vk::DebugUtilsMessageTypeFlagsEXT,
            data: *const vk::DebugUtilsMessengerCallbackDataEXT<'_>,
            user: *mut std::ffi::c_void,
        ) -> vk::Bool32 {
            if data.is_null() || user.is_null() {
                return vk::FALSE;
            }
            // SAFETY: user 是 `Arc<ValidationCollector>` 的裸指针，messenger 在 collector 之前销毁。
            let collector = unsafe { &*(user as *const ValidationCollector) };
            let data = unsafe { &*data };
            let text = |p: *const std::ffi::c_char| {
                if p.is_null() { String::new() } else { unsafe { CStr::from_ptr(p) }.to_string_lossy().into_owned() }
            };
            collector.messages.lock().push(DebugMessage {
                severity,
                types,
                id_name: text(data.p_message_id_name),
                message: text(data.p_message),
            });
            vk::FALSE
        }
        let info = vk::DebugUtilsMessengerCreateInfoEXT::default()
            .message_severity(
                vk::DebugUtilsMessageSeverityFlagsEXT::WARNING | vk::DebugUtilsMessageSeverityFlagsEXT::ERROR,
            )
            .message_type(
                vk::DebugUtilsMessageTypeFlagsEXT::GENERAL
                    | vk::DebugUtilsMessageTypeFlagsEXT::VALIDATION
                    | vk::DebugUtilsMessageTypeFlagsEXT::PERFORMANCE,
            )
            .pfn_user_callback(Some(callback))
            .user_data(Arc::as_ptr(&collector) as *mut std::ffi::c_void);
        let handle = unsafe { loader.create_debug_utils_messenger(&info, None) }?;
        Ok(Self { loader, handle, collector, _instance: instance.clone() })
    }
    /// 主动提交一条消息（经 loader，无需校验层）；用来验证收集器本身。
    pub fn submit(&self, severity: vk::DebugUtilsMessageSeverityFlagsEXT, id: &CStr, text: &CStr) {
        let data = vk::DebugUtilsMessengerCallbackDataEXT::default().message_id_name(id).message(text);
        unsafe {
            self.loader.submit_debug_utils_message(severity, vk::DebugUtilsMessageTypeFlagsEXT::GENERAL, &data)
        };
    }
}
impl Drop for ValidationMessenger {
    fn drop(&mut self) {
        unsafe { self.loader.destroy_debug_utils_messenger(self.handle, None) };
    }
}

#[cfg(test)]
mod blocklist_tests {
    use super::*;

    #[test]
    fn env_chain_matches_original_write_order() {
        let mut writes = Vec::new();
        use crate::raw::init::{GraphicsInstanceLayerAllowlist, GraphicsInstanceLayerAllowlistEntry as E};
        let allowlist = GraphicsInstanceLayerAllowlist {
            version: 1,
            entries: vec![
                E::Layer("VK_LAYER_foo".into()),
                E::Vendor("NV".into()),
                E::Layer("other".into()),
            ],
        };
        apply_vulkan_blocklist_with(
            &allowlist,
            |name, value| {
                writes.push((name.to_owned(), value.to_owned()));
                Ok(())
            },
        );
        assert_eq!(writes.len(), 18);
        assert_eq!(writes[0], ("VK_LOADER_LAYERS_DISABLE".into(), "~implicit~".into()));
        assert_eq!(
            writes[1],
            ("VK_LOADER_LAYERS_ALLOW".into(), "VK_LAYER_foo,VK_LAYER_NV*,other".into())
        );
        assert!(writes[2..].iter().all(|(_, v)| v == "1"));
        assert_eq!(writes[10].0, "EOS_OVERLAY_DISABLE_VULKAN_WIN64");
        assert_eq!(writes[10].0.len(), 32);
        assert_eq!(writes[17].0, "DISABLE_VK_LAYER_gshade_1");
    }

    #[test]
    fn env_name_lengths_match_the_string_pool() {
        let lens: Vec<usize> = VULKAN_LAYER_DISABLE_ENV.iter().map(|n| n.len()).collect();
        assert_eq!(lens, [18, 16, 32, 15, 20, 20, 26, 39, 32, 29, 31, 26, 29, 16, 13, 25]);
    }

    #[test]
    #[should_panic(expected = "failed to set environment variable `VK_LOADER_LAYERS_DISABLE`")]
    fn set_var_failure_panics() {
        apply_vulkan_blocklist_with(&Default::default(), |_, _| Err(std::io::Error::other("boom")));
    }
}

#[cfg(test)]
mod extension_tests {
    use super::*;

    fn props(name: &str) -> vk::ExtensionProperties {
        let mut p = vk::ExtensionProperties::default();
        for (slot, b) in p.extension_name.iter_mut().zip(name.bytes()) {
            *slot = b as _;
        }
        p
    }

    #[test]
    fn defaults_first_then_requested_deduped_and_layer_settings_only_when_available() {
        let requested = [CString::new("VK_EXT_debug_utils").unwrap(), CString::new(DEFAULT_INSTANCE_EXTENSIONS[0]).unwrap()];
        let names = |v: &[CString]| v.iter().map(|c| c.to_str().unwrap().to_owned()).collect::<Vec<_>>();
        let without = effective_extensions(&requested, &[]);
        assert_eq!(
            names(&without),
            ["VK_KHR_get_physical_device_properties2", "VK_KHR_get_surface_capabilities2", "VK_EXT_debug_utils"]
        );
        let with = effective_extensions(&requested, &[props("VK_EXT_layer_settings")]);
        assert_eq!(names(&with).last().unwrap(), "VK_EXT_layer_settings");
        // 前缀相同但更长的名字不算命中（原版按整个 22 字节含 NUL 比较）。
        assert_eq!(effective_extensions(&[], &[props("VK_EXT_layer_settings2")]).len(), 2);
    }
}
