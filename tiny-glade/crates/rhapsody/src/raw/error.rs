//! 候选 Vulkan 错误接口；原错误枚举 ABI 尚未恢复。
use ash::vk;
#[derive(Debug)]
pub enum Error {
    Vulkan(vk::Result),
    Loader(String),
    Unsupported(&'static str),
    Invalid(&'static str),
    /// 设备缺少原版要求的扩展/特性/上限；消息带具体名字。
    Missing(String),
}
pub type Result<T> = std::result::Result<T, Error>;
impl From<vk::Result> for Error {
    fn from(v: vk::Result) -> Self {
        Self::Vulkan(v)
    }
}
impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Vulkan(e) => write!(f, "Vulkan: {e:?}"),
            Self::Loader(e) | Self::Missing(e) => f.write_str(e),
            Self::Unsupported(e) | Self::Invalid(e) => f.write_str(e),
        }
    }
}
impl std::error::Error for Error {}
