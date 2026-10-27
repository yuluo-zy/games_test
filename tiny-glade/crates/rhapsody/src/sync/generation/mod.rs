//! 设备代际：标识设备的一次生命周期，拒绝跨代际/跨设备的资源混用。
//!
//! # 设计
//!
//! 设备每次创建（或从 `DeviceLost` 恢复后重建）都拿到一个新的 [`DeviceGeneration`]，进程
//! 内单调递增、绝不重复。句柄/资源用 [`Generationed<T>`] 携带“值 + 代际”，访问时与当前
//! 设备代际比对，不匹配即返回 [`Error::InvalidUsage`]。
//!
//! 这样把“上一个设备的资源不能塞进新设备的帧实例”从文档约定变成运行时硬约束：即使某个
//! 旧句柄的数值恰好与新一代里的槽位下标相同，代际不同也会被拒绝，不会误读新资源。

use std::sync::atomic::{AtomicU64, Ordering};

/// 进程级代际发放器。从 1 开始，`fetch_add` 返回旧值，因此首个代际为 1。
static NEXT_GENERATION: AtomicU64 = AtomicU64::new(1);

/// 设备的一次生命周期标识。
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct DeviceGeneration(u64);

impl DeviceGeneration {
    /// 0 保留为“无效/未初始化”，不是任何真实代际。
    pub const INVALID: Self = Self(0);

    /// 发放一个新代际。进程内唯一且单调递增。
    pub fn next() -> Self {
        Self(NEXT_GENERATION.fetch_add(1, Ordering::Relaxed))
    }

    pub const fn value(self) -> u64 {
        self.0
    }

    pub const fn is_valid(self) -> bool {
        self.0 != 0
    }
}

/// 携带设备代际的值：访问前必须确认代际匹配。
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct Generationed<T> {
    value: T,
    generation: DeviceGeneration,
}

impl<T> Generationed<T> {
    pub const fn new(value: T, generation: DeviceGeneration) -> Self {
        Self { value, generation }
    }

    pub const fn generation(&self) -> DeviceGeneration {
        self.generation
    }

    /// 代际匹配才交出只读引用；否则返回 [`crate::Error::InvalidUsage`]。
    pub fn check(&self, current: DeviceGeneration) -> crate::Result<&T> {
        if current.is_valid() && self.generation == current {
            Ok(&self.value)
        } else {
            Err(crate::Error::invalid_usage("资源代际与当前设备不符"))
        }
    }

    /// 代际匹配才交出所有权；否则返回 [`crate::Error::InvalidUsage`]。
    pub fn into_value(self, current: DeviceGeneration) -> crate::Result<T> {
        if current.is_valid() && self.generation == current {
            Ok(self.value)
        } else {
            Err(crate::Error::invalid_usage("资源代际与当前设备不符"))
        }
    }
}

#[cfg(test)]
mod tests;
