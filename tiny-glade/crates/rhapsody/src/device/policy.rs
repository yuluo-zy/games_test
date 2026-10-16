//! 执行策略：在设备初始化时，依据「已启用」能力一次性选定实现路径。
//!
//! # 设计
//!
//! 能力查询与策略选择分开：`caps.rs` 只回答“有没有/开没开”，本模块回答“因此该走哪条
//! 路”。策略在设备创建时算一次并固化进 [`ExecutionPolicy`]，帧热路径只读常量，不再重复
//! 探测或分支。
//!
//! 三条正交的轴：
//! - 描述符策略：`DescriptorBuffer` > `Bindless` > `PerSet`（`Auto` 时的优先级）。bindless
//!   依赖一整组 descriptor indexing 能力，缺一不可；`PerSet` 是永远可用的兜底。
//! - 录制策略：并行录制是显式 opt-in，默认串行（见 `REFACTOR_PLAN.md`）。
//! - 分配策略：通用分配默认子分配（受 crate 的 `suballocation` feature 门控）；专用/大块
//!   资源按 §4.5 分层分配单独决策，不在这里。

use crate::device::caps::{DeviceCaps, Feature, FeatureSet};
use crate::error::Result;

/// 描述符实现路径。
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum DescriptorStrategy {
    /// 全局描述符堆 + 逐资源 `BindlessIndex`（需要 descriptor indexing）。
    Bindless,
    /// `VK_EXT_descriptor_buffer`：描述符直接写 GPU 缓冲（可选加速）。
    DescriptorBuffer,
    /// 传统每 pass 描述符集。
    #[default]
    PerSet,
}

/// 录制策略。
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum RecordingPolicy {
    /// 单命令缓冲串行录制（默认，最简单可预期）。
    #[default]
    Serial,
    /// 按 pass 分块并行录制（显式 opt-in）。
    Parallel,
}

/// 通用分配的默认策略。
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum AllocationStrategy {
    /// 用 gpu-allocator 子分配（受 crate `suballocation` feature 门控）。
    #[default]
    Suballocated,
    /// 每个资源一块独立内存。
    Dedicated,
}

/// 描述符路径的偏好：既支持自动选择，也支持“必须某路径否则拒绝”。
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum DescriptorPreference {
    /// 按能力自动选择：DescriptorBuffer → Bindless → PerSet。
    #[default]
    Auto,
    /// 必须 bindless，否则返回 [`Error::Unsupported`]。
    RequireBindless,
    /// 必须 descriptor buffer，否则返回 [`Error::Unsupported`]。
    RequireDescriptorBuffer,
    /// 强制传统每 pass 描述符集。
    PerSet,
}

/// 策略偏好：不依赖能力、由调用方显式给出的选择。
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct PolicyPreferences {
    pub descriptor: DescriptorPreference,
    pub recording: RecordingPolicy,
    pub allocation: AllocationStrategy,
}

/// 初始化时固化的执行策略。
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct ExecutionPolicy {
    pub descriptor: DescriptorStrategy,
    pub recording: RecordingPolicy,
    pub allocation: AllocationStrategy,
}

/// bindless 依赖的整组 descriptor indexing 能力（缺一不可）。
const BINDLESS_REQUIRED: [Feature; 8] = [
    Feature::DescriptorIndexing,
    Feature::RuntimeDescriptorArray,
    Feature::DescriptorBindingPartiallyBound,
    Feature::DescriptorBindingVariableDescriptorCount,
    Feature::SampledImageUpdateAfterBind,
    Feature::StorageBufferUpdateAfterBind,
    Feature::ShaderSampledImageArrayNonUniformIndexing,
    Feature::DescriptorBindingUpdateUnusedWhilePending,
];

/// 依据「已启用」能力与偏好选定执行策略。
pub fn plan_policy(caps: &DeviceCaps, preferences: &PolicyPreferences) -> Result<ExecutionPolicy> {
    Ok(ExecutionPolicy {
        descriptor: select_descriptor(caps, preferences.descriptor)?,
        recording: preferences.recording,
        allocation: preferences.allocation,
    })
}

fn select_descriptor(
    caps: &DeviceCaps,
    preference: DescriptorPreference,
) -> Result<DescriptorStrategy> {
    match preference {
        DescriptorPreference::PerSet => Ok(DescriptorStrategy::PerSet),
        DescriptorPreference::RequireDescriptorBuffer => {
            caps.require_enabled(Feature::DescriptorBuffer)?;
            Ok(DescriptorStrategy::DescriptorBuffer)
        }
        DescriptorPreference::RequireBindless => {
            require_bindless(caps)?;
            Ok(DescriptorStrategy::Bindless)
        }
        DescriptorPreference::Auto => {
            if caps.is_enabled(Feature::DescriptorBuffer) {
                Ok(DescriptorStrategy::DescriptorBuffer)
            } else if bindless_available(caps) {
                Ok(DescriptorStrategy::Bindless)
            } else {
                Ok(DescriptorStrategy::PerSet)
            }
        }
    }
}

fn bindless_available(caps: &DeviceCaps) -> bool {
    BINDLESS_REQUIRED
        .iter()
        .all(|&feature| caps.is_enabled(feature))
}

fn require_bindless(caps: &DeviceCaps) -> Result<()> {
    let required = BINDLESS_REQUIRED
        .iter()
        .fold(FeatureSet::EMPTY, |set, &feature| set.with(feature));
    caps.require_all_enabled(required)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::error::ErrorKind;

    fn caps_with(enabled: &[Feature]) -> DeviceCaps {
        let enabled = enabled
            .iter()
            .fold(FeatureSet::EMPTY, |set, &feature| set.with(feature));
        DeviceCaps::new(enabled, enabled)
    }

    fn prefs(descriptor: DescriptorPreference) -> PolicyPreferences {
        PolicyPreferences {
            descriptor,
            ..PolicyPreferences::default()
        }
    }

    #[test]
    fn per_set_is_always_available() {
        let policy = plan_policy(&caps_with(&[]), &prefs(DescriptorPreference::PerSet)).unwrap();
        assert_eq!(policy.descriptor, DescriptorStrategy::PerSet);
    }

    #[test]
    fn auto_prefers_descriptor_buffer_then_bindless() {
        let with_buffer = caps_with(&[Feature::DescriptorBuffer]);
        assert_eq!(
            plan_policy(&with_buffer, &prefs(DescriptorPreference::Auto))
                .unwrap()
                .descriptor,
            DescriptorStrategy::DescriptorBuffer
        );

        let with_bindless = caps_with(&BINDLESS_REQUIRED);
        assert_eq!(
            plan_policy(&with_bindless, &prefs(DescriptorPreference::Auto))
                .unwrap()
                .descriptor,
            DescriptorStrategy::Bindless
        );
    }

    #[test]
    fn auto_falls_back_to_per_set_without_bindless_capabilities() {
        let partial = caps_with(&[Feature::DescriptorIndexing]);
        assert_eq!(
            plan_policy(&partial, &prefs(DescriptorPreference::Auto))
                .unwrap()
                .descriptor,
            DescriptorStrategy::PerSet
        );
    }

    #[test]
    fn required_bindless_rejects_incomplete_capabilities() {
        let partial = caps_with(&[Feature::DescriptorIndexing]);
        let error =
            plan_policy(&partial, &prefs(DescriptorPreference::RequireBindless)).unwrap_err();
        assert_eq!(error.kind(), ErrorKind::Unsupported);

        let full = caps_with(&BINDLESS_REQUIRED);
        assert_eq!(
            plan_policy(&full, &prefs(DescriptorPreference::RequireBindless))
                .unwrap()
                .descriptor,
            DescriptorStrategy::Bindless
        );
    }

    #[test]
    fn required_descriptor_buffer_rejects_when_not_enabled() {
        let error = plan_policy(
            &caps_with(&[]),
            &prefs(DescriptorPreference::RequireDescriptorBuffer),
        )
        .unwrap_err();
        assert_eq!(error.kind(), ErrorKind::Unsupported);
    }

    #[test]
    fn policy_carries_recording_and_allocation_preferences() {
        let policy = plan_policy(
            &caps_with(&[]),
            &PolicyPreferences {
                recording: RecordingPolicy::Parallel,
                allocation: AllocationStrategy::Dedicated,
                ..PolicyPreferences::default()
            },
        )
        .unwrap();
        assert_eq!(policy.recording, RecordingPolicy::Parallel);
        assert_eq!(policy.allocation, AllocationStrategy::Dedicated);
    }
}
