//! Graphics instance layers and their allowlist.
use serde::{Deserialize, Serialize};

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
