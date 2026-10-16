//! rhapsody 统一错误模型。
//!
//! 公共 API 一律返回 [`Result<T>`]。错误按调用方能采取的处置分类：
//!
//! | 类别 | 含义 | 调用方处置 |
//! | --- | --- | --- |
//! | [`Error::DeviceLost`] | 设备不可用，所有后续 GPU 工作都会失败 | 重建设备或退出 |
//! | [`Error::OutOfDeviceMemory`] / [`Error::OutOfHostMemory`] | 分配失败 | 降级（降分辨率、清池）后重试 |
//! | [`Error::SurfaceOutOfDate`] | 交换链需要重建 | 重建交换链，跳过本帧 |
//! | [`Error::SurfaceLost`] | surface 已失效 | 重建 surface |
//! | [`Error::Timeout`] | 有界等待超时 | 稍后重试 |
//! | [`Error::Unsupported`] | 设备或驱动缺少所需能力 | 换路径或报告 |
//! | [`Error::InvalidUsage`] | 调用方违反 API 约定（越界、过期句柄、状态顺序错误等） | 修调用方代码 |
//! | [`Error::Vulkan`] | 其他未归类的 Vulkan 错误 | 记录并按致命处理 |
//!
//! 约定：用户可触发的误用一律返回 [`Error::InvalidUsage`]，不 panic；`debug_assert!` 只用于
//! rhapsody 内部不变量（违反即 rhapsody 自身的 bug）。
use ash::vk;
use std::borrow::Cow;

pub type Result<T, E = Error> = std::result::Result<T, E>;

/// 附带在 [`Error::InvalidUsage`] 上的下层细节（各模块自己的错误枚举）。
pub type Source = Box<dyn std::error::Error + Send + Sync + 'static>;

#[derive(Debug, thiserror::Error)]
#[non_exhaustive]
pub enum Error {
    #[error("GPU 设备丢失")]
    DeviceLost,
    #[error("显存不足")]
    OutOfDeviceMemory,
    #[error("主机内存不足")]
    OutOfHostMemory,
    #[error("交换链已过期，需要重建")]
    SurfaceOutOfDate,
    #[error("surface 已丢失")]
    SurfaceLost,
    #[error("等待超时")]
    Timeout,
    #[error("不支持：{0}")]
    Unsupported(Cow<'static, str>),
    #[error("用法错误：{what}")]
    InvalidUsage {
        what: Cow<'static, str>,
        #[source]
        source: Option<Source>,
    },
    #[error("Vulkan 错误：{0:?}")]
    Vulkan(vk::Result),
}

/// [`Error`] 的类别，便于 `match` 而不必关心携带的数据。
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum ErrorKind {
    DeviceLost,
    OutOfMemory,
    SurfaceOutOfDate,
    SurfaceLost,
    Timeout,
    Unsupported,
    InvalidUsage,
    Vulkan,
}

impl Error {
    pub fn kind(&self) -> ErrorKind {
        match self {
            Self::DeviceLost => ErrorKind::DeviceLost,
            Self::OutOfDeviceMemory | Self::OutOfHostMemory => ErrorKind::OutOfMemory,
            Self::SurfaceOutOfDate => ErrorKind::SurfaceOutOfDate,
            Self::SurfaceLost => ErrorKind::SurfaceLost,
            Self::Timeout => ErrorKind::Timeout,
            Self::Unsupported(_) => ErrorKind::Unsupported,
            Self::InvalidUsage { .. } => ErrorKind::InvalidUsage,
            Self::Vulkan(_) => ErrorKind::Vulkan,
        }
    }

    /// 设备已不可用：后续 GPU 工作必然失败，持有者应进入终止状态。
    pub fn is_device_lost(&self) -> bool {
        matches!(self, Self::DeviceLost)
    }

    pub fn unsupported(what: impl Into<Cow<'static, str>>) -> Self {
        Self::Unsupported(what.into())
    }

    /// 调用方误用。同时记一条 error 日志，避免误用在调用方吞掉错误后无声消失。
    pub fn invalid_usage(what: impl Into<Cow<'static, str>>) -> Self {
        let what = what.into();
        log::error!("rhapsody 用法错误：{what}");
        Self::InvalidUsage { what, source: None }
    }

    /// 带下层细节的误用。
    pub fn invalid_usage_with(
        what: impl Into<Cow<'static, str>>,
        source: impl std::error::Error + Send + Sync + 'static,
    ) -> Self {
        let what = what.into();
        log::error!("rhapsody 用法错误：{what}：{source}");
        Self::InvalidUsage {
            what,
            source: Some(Box::new(source)),
        }
    }
}

impl From<vk::Result> for Error {
    fn from(result: vk::Result) -> Self {
        match result {
            vk::Result::ERROR_DEVICE_LOST => Self::DeviceLost,
            vk::Result::ERROR_OUT_OF_DEVICE_MEMORY | vk::Result::ERROR_OUT_OF_POOL_MEMORY => {
                Self::OutOfDeviceMemory
            }
            vk::Result::ERROR_OUT_OF_HOST_MEMORY => Self::OutOfHostMemory,
            vk::Result::ERROR_OUT_OF_DATE_KHR
            | vk::Result::ERROR_FULL_SCREEN_EXCLUSIVE_MODE_LOST_EXT => Self::SurfaceOutOfDate,
            vk::Result::ERROR_SURFACE_LOST_KHR => Self::SurfaceLost,
            vk::Result::TIMEOUT => Self::Timeout,
            vk::Result::ERROR_FEATURE_NOT_PRESENT
            | vk::Result::ERROR_EXTENSION_NOT_PRESENT
            | vk::Result::ERROR_LAYER_NOT_PRESENT
            | vk::Result::ERROR_FORMAT_NOT_SUPPORTED
            | vk::Result::ERROR_INCOMPATIBLE_DRIVER => Self::Unsupported(format!("{result:?}").into()),
            other => Self::Vulkan(other),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn vulkan_results_map_to_actionable_categories() {
        let cases = [
            (vk::Result::ERROR_DEVICE_LOST, ErrorKind::DeviceLost),
            (vk::Result::ERROR_OUT_OF_DEVICE_MEMORY, ErrorKind::OutOfMemory),
            (vk::Result::ERROR_OUT_OF_HOST_MEMORY, ErrorKind::OutOfMemory),
            (vk::Result::ERROR_OUT_OF_DATE_KHR, ErrorKind::SurfaceOutOfDate),
            (vk::Result::ERROR_SURFACE_LOST_KHR, ErrorKind::SurfaceLost),
            (vk::Result::TIMEOUT, ErrorKind::Timeout),
            (vk::Result::ERROR_EXTENSION_NOT_PRESENT, ErrorKind::Unsupported),
            (vk::Result::ERROR_INITIALIZATION_FAILED, ErrorKind::Vulkan),
        ];
        for (result, kind) in cases {
            assert_eq!(Error::from(result).kind(), kind, "{result:?}");
        }
    }

    #[test]
    fn invalid_usage_keeps_its_source() {
        #[derive(Debug, thiserror::Error)]
        #[error("细节")]
        struct Detail;
        let error = Error::invalid_usage_with("越界", Detail);
        assert_eq!(error.kind(), ErrorKind::InvalidUsage);
        assert_eq!(std::error::Error::source(&error).unwrap().to_string(), "细节");
    }
}
