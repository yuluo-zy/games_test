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
                    return Err(Error::Loader(format!(
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
                return Err(Error::Loader(format!("layer不在白名单: {}", layer.name)));
            }
        }
        self.build()
    }
    pub fn build_with_loader(&self, loader: GraphicsInstanceLoader) -> Result<Arc<VulkanInstance>> {
        if loader.api_version < self.api_version {
            return Err(Error::Unsupported("Vulkan 版本低于请求版本"));
        }
        for layer in &self.layers {
            if !loader
                .layers
                .iter()
                .any(|p| p.name == layer.to_string_lossy())
            {
                return Err(Error::Loader(format!(
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
        for extension in &self.extensions {
            if !available.iter().any(
                |p| unsafe { CStr::from_ptr(p.extension_name.as_ptr()) } == extension.as_c_str(),
            ) {
                return Err(Error::Loader(format!(
                    "请求instance扩展不可用: {}",
                    extension.to_string_lossy()
                )));
            }
        }
        let extensions: Vec<_> = self.extensions.iter().map(|v| v.as_ptr()).collect();
        let layers: Vec<_> = self.layers.iter().map(|v| v.as_ptr()).collect();
        let app = vk::ApplicationInfo::default()
            .application_name(&self.application_name)
            .engine_name(c"rhapsody")
            .api_version(self.api_version);
        let info = vk::InstanceCreateInfo::default()
            .application_info(&app)
            .enabled_extension_names(&extensions)
            .enabled_layer_names(&layers);
        let instance = unsafe { entry.create_instance(&info, None) }?;
        Ok(Arc::new(VulkanInstance {
            entry,
            instance,
            enabled_extensions: self
                .extensions
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
    /// 从请求列表中移除指定层（若存在）。
    pub fn legacy_disable_layer(&mut self, name: &str) {
        self.layers.retain(|layer| layer.to_string_lossy() != name);
    }
}

/// 通用层黑名单：保留下不在 `blocked` 中的层。
pub fn apply_vulkan_blocklist<'a>(layers: &'a [String], blocked: &[&str]) -> Vec<&'a str> {
    layers
        .iter()
        .map(String::as_str)
        .filter(|name| !blocked.contains(name))
        .collect()
}

/// 现代驱动黑名单（关闭已知冲突层）；具体条目为候选，不声称覆盖原驱动表。
pub fn apply_vulkan_modern_blocklist(layers: &[String]) -> Vec<&str> {
    apply_vulkan_blocklist(layers, &[])
}

/// 旧驱动黑名单（关闭已废弃层）。
pub fn apply_vulkan_legacy_blocklist(layers: &[String]) -> Vec<&str> {
    apply_vulkan_blocklist(layers, &["VK_LAYER_LUNARG_standard_validation"])
}

/// 双击/重复层黑名单（合并同供应商重复实例）。
pub fn apply_vulkan_doubletap_blocklist(layers: &[String]) -> Vec<&str> {
    let mut seen = std::collections::BTreeSet::new();
    layers
        .iter()
        .map(String::as_str)
        .filter(|name| seen.insert(name.to_owned()))
        .collect()
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
        return Err(Error::Unsupported("实例未启用 VK_EXT_debug_utils"));
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
