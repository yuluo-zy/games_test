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
//! | [`Error::Exhausted`] | 有界容量耗尽（槽位表、描述符池、arena、缓存） | 释放/回收后重试，或扩大配置容量 |
//! | [`Error::Unsupported`] | 设备或驱动缺少所需能力 | 换路径或报告 |
//! | [`Error::InvalidUsage`] | 调用方违反 API 约定（越界、过期句柄、状态顺序错误等） | 修调用方代码 |
//! | [`Error::Vulkan`] | 其他未归类的 Vulkan 错误 | 记录并按致命处理 |
//!
//! 约定：用户可触发的误用一律返回 [`Error::InvalidUsage`]，不 panic；`debug_assert!` 只用于
//! rhapsody 内部不变量（违反即 rhapsody 自身的 bug）。
use crate::native_types::BackendResult;
use std::borrow::Cow;

pub type Result<T, E = Error> = std::result::Result<T, E>;

/// 附带在 [`Error::InvalidUsage`] 上的下层细节（各模块自己的错误枚举）。
pub type Source = Box<dyn std::error::Error + Send + Sync + 'static>;

// Deliberately NOT `#[non_exhaustive]`. Inside `rhapsody` this enum was matched exhaustively on purpose
// (`command_buffer/retention.rs`, `transfer/queue/mod.rs`): adding a variant must be a compile error there.
// `non_exhaustive` only lets the defining crate match exhaustively, so once the enum moved to its own crate
// it would have forced `_ =>` arms and silently dropped that guarantee. Cost: adding a variant now breaks
// downstream matches too; every downstream crate is in this workspace (`publish = false`).
#[derive(Debug, thiserror::Error)]
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
    #[error("容量耗尽：{0}")]
    Exhausted(Cow<'static, str>),
    #[error("不支持：{0}")]
    Unsupported(Cow<'static, str>),
    #[error("用法错误：{what}")]
    InvalidUsage {
        what: Cow<'static, str>,
        #[source]
        source: Option<Source>,
    },
    /// A backend status code that has no neutral class. With `native-types` it is the Vulkan result code.
    #[cfg(feature = "native-types")]
    #[error("Vulkan 错误：{0:?}")]
    Vulkan(BackendResult),
    /// A backend status code that has no neutral class (a non-Vulkan backend).
    #[cfg(not(feature = "native-types"))]
    #[error("后端错误：{0:?}")]
    Backend(BackendResult),
}

/// Cloneable diagnostic identity for protocols which implement Eq. Failures
/// are not part of graph-plan cache keys.
#[derive(Clone, Debug)]
pub struct SharedError(std::sync::Arc<Error>);
impl SharedError {
    pub fn into_owned(self) -> Error {
        self.0.into()
    }
}
impl From<Error> for SharedError {
    fn from(error: Error) -> Self {
        Self(std::sync::Arc::new(error))
    }
}
impl PartialEq for SharedError {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.0, &other.0)
    }
}
impl Eq for SharedError {}
impl std::fmt::Display for SharedError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.0.as_ref(), f)
    }
}
impl std::error::Error for SharedError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        Some(self.0.as_ref())
    }
}

/// [`Error`] 的类别，便于 `match` 而不必关心携带的数据。
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum ErrorKind {
    DeviceLost,
    OutOfMemory,
    SurfaceOutOfDate,
    SurfaceLost,
    Timeout,
    Exhausted,
    Unsupported,
    InvalidUsage,
    /// An unmapped backend status code. Named `Vulkan` because existing code matches on it; it covers
    /// `Error::Backend` too and can be renamed once those matches are migrated.
    Vulkan,
}
impl From<std::sync::Arc<Error>> for Error {
    fn from(error: std::sync::Arc<Error>) -> Self {
        match error.as_ref() {
            Self::DeviceLost => Self::DeviceLost,
            Self::OutOfDeviceMemory => Self::OutOfDeviceMemory,
            Self::OutOfHostMemory => Self::OutOfHostMemory,
            Self::SurfaceOutOfDate => Self::SurfaceOutOfDate,
            Self::SurfaceLost => Self::SurfaceLost,
            Self::Timeout => Self::Timeout,
            Self::Exhausted(what) => Self::Exhausted(what.clone()),
            Self::Unsupported(what) => Self::Unsupported(what.clone()),
            #[cfg(feature = "native-types")]
            Self::Vulkan(code) => Self::Vulkan(*code),
            #[cfg(not(feature = "native-types"))]
            Self::Backend(code) => Self::Backend(*code),
            Self::InvalidUsage { what, .. } => Self::InvalidUsage {
                what: what.clone(),
                source: Some(Box::new(error.clone())),
            },
        }
    }
}

impl Error {
    pub fn kind(&self) -> ErrorKind {
        match self {
            Self::DeviceLost => ErrorKind::DeviceLost,
            Self::OutOfDeviceMemory | Self::OutOfHostMemory => ErrorKind::OutOfMemory,
            Self::SurfaceOutOfDate => ErrorKind::SurfaceOutOfDate,
            Self::SurfaceLost => ErrorKind::SurfaceLost,
            Self::Timeout => ErrorKind::Timeout,
            Self::Exhausted(_) => ErrorKind::Exhausted,
            Self::Unsupported(_) => ErrorKind::Unsupported,
            Self::InvalidUsage { .. } => ErrorKind::InvalidUsage,
            #[cfg(feature = "native-types")]
            Self::Vulkan(_) => ErrorKind::Vulkan,
            #[cfg(not(feature = "native-types"))]
            Self::Backend(_) => ErrorKind::Vulkan,
        }
    }

    /// 设备已不可用：后续 GPU 工作必然失败，持有者应进入终止状态。
    pub fn is_device_lost(&self) -> bool {
        matches!(self, Self::DeviceLost)
    }

    pub fn unsupported(what: impl Into<Cow<'static, str>>) -> Self {
        Self::Unsupported(what.into())
    }

    /// 有界容量耗尽；与 [`Error::Timeout`] 区分，调用方应先回收再重试。
    pub fn exhausted(what: impl Into<Cow<'static, str>>) -> Self {
        Self::Exhausted(what.into())
    }

    /// 调用方误用。记一条 debug 日志：预期内的拒绝（如重试循环）不应刷屏，
    /// 调用方必须传播或显式处理返回的错误。
    pub fn invalid_usage(what: impl Into<Cow<'static, str>>) -> Self {
        let what = what.into();
        log::debug!("rhapsody 用法错误：{what}");
        Self::InvalidUsage { what, source: None }
    }

    /// 带下层细节的误用。
    pub fn invalid_usage_with(
        what: impl Into<Cow<'static, str>>,
        source: impl std::error::Error + Send + Sync + 'static,
    ) -> Self {
        let what = what.into();
        log::debug!("rhapsody 用法错误：{what}：{source}");
        Self::InvalidUsage {
            what,
            source: Some(Box::new(source)),
        }
    }
}

#[cfg(all(test, feature = "native-types"))]
mod tests;
