//! 设备能力模型：严格区分「硬件支持」与「实际启用」。
//!
//! # 设计
//!
//! Vulkan 上「驱动支持某特性」和「本次 `vkCreateDevice` 实际启用了它」是两回事。例如
//! 驱动可能支持 descriptor indexing，但设备创建时没有开启，此时 bindless 路径不可用。
//! 因此 [`DeviceCaps`] 同时保存 `supported` 与 `enabled` 两个 [`FeatureSet`]：
//! - `supported`：物理设备报告支持的能力（探测结果）；
//! - `enabled`：逻辑设备创建时真正开启的能力。
//!
//! 所有“按能力选择实现路径”的决策只看 `enabled`；`supported` 只用于诊断和一致性校验。
//! [`Feature`] 的判别值就是 [`FeatureSet`] 位集里的位下标（0..64），因此集合用单个 `u64`
//! 承载，`Copy`、无堆分配，适合在创建路径里反复比较。

use crate::error::{Error, Result};

/// 一个具名能力。`repr(u8)` 判别值即 [`FeatureSet`] 中的位下标。
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
#[repr(u8)]
pub enum Feature {
    /// `VK_KHR_timeline_semaphore`（Vulkan 1.2 核心）。
    TimelineSemaphore = 0,
    /// `VK_KHR_synchronization2`（Vulkan 1.3 核心）。
    Synchronization2 = 1,
    /// `VK_KHR_dynamic_rendering`（Vulkan 1.3 核心）。
    DynamicRendering = 2,
    /// `VK_KHR_buffer_device_address`（Vulkan 1.2 核心）。
    BufferDeviceAddress = 3,
    /// `VK_EXT_descriptor_indexing` 的运行时描述符数组。
    DescriptorIndexing = 4,
    /// update-after-bind 描述符在等待期间可被更新。
    DescriptorBindingUpdateUnusedWhilePending = 5,
    /// `VK_EXT_descriptor_buffer`：把描述符直接写入 GPU 缓冲。
    DescriptorBuffer = 6,
    /// `VK_EXT_graphics_pipeline_library`。
    GraphicsPipelineLibrary = 7,
    /// `VK_EXT_shader_object`。
    ShaderObject = 8,
    /// `VK_EXT_mesh_shader`。
    MeshShader = 9,
    /// `VK_KHR_present_id`。
    PresentId = 10,
    /// `VK_KHR_present_wait`。
    PresentWait = 11,
    /// `VK_EXT_memory_budget`。
    MemoryBudget = 12,
    /// `wideLines`（Vulkan 1.0 特性）。
    WideLines = 13,
    /// `runtimeDescriptorArray`。
    RuntimeDescriptorArray = 14,
    /// `descriptorBindingPartiallyBound`。
    DescriptorBindingPartiallyBound = 15,
    /// `descriptorBindingVariableDescriptorCount`。
    DescriptorBindingVariableDescriptorCount = 16,
    /// `descriptorBindingSampledImageUpdateAfterBind`。
    SampledImageUpdateAfterBind = 17,
    /// `descriptorBindingStorageBufferUpdateAfterBind`。
    StorageBufferUpdateAfterBind = 18,
    /// `shaderSampledImageArrayNonUniformIndexing`。
    ShaderSampledImageArrayNonUniformIndexing = 19,
    SwapchainMaintenance1 = 20,
    /// `descriptorBindingStorageImageUpdateAfterBind`.
    StorageImageUpdateAfterBind = 21,
    ShaderStorageImageArrayNonUniformIndexing = 22,
    ShaderStorageBufferArrayNonUniformIndexing = 23,
    /// Core `shaderStorageBufferArrayDynamicIndexing`.
    ShaderStorageBufferArrayDynamicIndexing = 24,
    /// Core `shaderSampledImageArrayDynamicIndexing`.
    ShaderSampledImageArrayDynamicIndexing = 25,
    /// Core `shaderStorageImageArrayDynamicIndexing`.
    ShaderStorageImageArrayDynamicIndexing = 26,
    /// Core `imageCubeArray` for cube-array views.
    ImageCubeArray = 27,
    /// Core `multiDrawIndirect`; more than one command per indirect call.
    MultiDrawIndirect = 28,
    /// Core `drawIndirectFirstInstance`; GPU commands may carry firstInstance.
    DrawIndirectFirstInstance = 29,
    /// Vulkan 1.2 `drawIndirectCount`; the count is read from a GPU buffer.
    DrawIndirectCount = 30,
    /// Vulkan 1.2 `separateDepthStencilLayouts`: depth and stencil aspects
    /// of one image may be in different layouts (`DEPTH_ATTACHMENT_OPTIMAL`,
    /// `STENCIL_READ_ONLY_OPTIMAL`, ...) and are tracked independently.
    SeparateDepthStencilLayouts = 31,
    /// Vulkan 1.3 `shaderDemoteToHelperInvocation`
    /// (`VK_EXT_shader_demote_to_helper_invocation`)：着色器可声明
    /// `DemoteToHelperInvocation` 能力（HLSL `discard` 经 DXC 产生 `OpDemoteToHelperInvocation`）。
    ShaderDemoteToHelperInvocation = 32,
    /// Vulkan 1.1 `shaderDrawParameters`: shaders may read the `DrawIndex`
    /// built-in (SPIR-V `DrawParameters`), which GPU-driven draws use to
    /// fetch per-draw parameters.
    ShaderDrawParameters = 33,
}

impl Feature {
    /// 能力对应的位掩码。
    pub const fn bit(self) -> u64 {
        1u64 << (self as u8)
    }

    /// 全部已建模能力。位下标必须与 `repr(u8)` 判别值一一对应。
    pub const ALL: [Feature; 34] = [
        Self::TimelineSemaphore,
        Self::Synchronization2,
        Self::DynamicRendering,
        Self::BufferDeviceAddress,
        Self::DescriptorIndexing,
        Self::DescriptorBindingUpdateUnusedWhilePending,
        Self::DescriptorBuffer,
        Self::GraphicsPipelineLibrary,
        Self::ShaderObject,
        Self::MeshShader,
        Self::PresentId,
        Self::PresentWait,
        Self::MemoryBudget,
        Self::WideLines,
        Self::RuntimeDescriptorArray,
        Self::DescriptorBindingPartiallyBound,
        Self::DescriptorBindingVariableDescriptorCount,
        Self::SampledImageUpdateAfterBind,
        Self::StorageBufferUpdateAfterBind,
        Self::ShaderSampledImageArrayNonUniformIndexing,
        Self::SwapchainMaintenance1,
        Self::StorageImageUpdateAfterBind,
        Self::ShaderStorageImageArrayNonUniformIndexing,
        Self::ShaderStorageBufferArrayNonUniformIndexing,
        Self::ShaderStorageBufferArrayDynamicIndexing,
        Self::ShaderSampledImageArrayDynamicIndexing,
        Self::ShaderStorageImageArrayDynamicIndexing,
        Self::ImageCubeArray,
        Self::MultiDrawIndirect,
        Self::DrawIndirectFirstInstance,
        Self::DrawIndirectCount,
        Self::SeparateDepthStencilLayouts,
        Self::ShaderDemoteToHelperInvocation,
        Self::ShaderDrawParameters,
    ];
}

/// 一组能力，位集表示。`Copy`、8 字节、无堆分配。
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Hash)]
pub struct FeatureSet(u64);

impl FeatureSet {
    pub const EMPTY: Self = Self(0);

    pub const fn empty() -> Self {
        Self(0)
    }

    pub const fn from_bits(bits: u64) -> Self {
        Self(bits)
    }

    pub const fn bits(self) -> u64 {
        self.0
    }

    pub const fn contains(self, feature: Feature) -> bool {
        self.0 & feature.bit() != 0
    }

    pub const fn with(self, feature: Feature) -> Self {
        Self(self.0 | feature.bit())
    }

    pub const fn union(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }

    pub const fn intersection(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }

    /// `self` 中有、`other` 中没有的能力。
    pub const fn difference(self, other: Self) -> Self {
        Self(self.0 & !other.0)
    }

    /// `self` 是否包含 `other` 的全部能力。
    pub const fn is_superset(self, other: Self) -> bool {
        (self.0 & other.0) == other.0
    }

    pub const fn is_empty(self) -> bool {
        self.0 == 0
    }

    /// 集合内能力数量。
    pub const fn len(self) -> u32 {
        self.0.count_ones()
    }

    /// 遍历集合内能力。
    pub fn iter(self) -> impl Iterator<Item = Feature> {
        Feature::ALL
            .into_iter()
            .filter(move |&feature| self.contains(feature))
    }
}

/// 设备能力：区分「支持」与「已启用」。
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct DeviceCaps {
    supported: FeatureSet,
    enabled: FeatureSet,
}

impl DeviceCaps {
    pub const fn new(supported: FeatureSet, enabled: FeatureSet) -> Self {
        Self { supported, enabled }
    }

    pub const fn supported(self) -> FeatureSet {
        self.supported
    }

    pub const fn enabled(self) -> FeatureSet {
        self.enabled
    }

    pub const fn is_supported(self, feature: Feature) -> bool {
        self.supported.contains(feature)
    }

    pub const fn is_enabled(self, feature: Feature) -> bool {
        self.enabled.contains(feature)
    }

    /// 已启用 ⊆ 已支持；不满足说明能力模型被喂了矛盾输入。
    pub const fn is_consistent(self) -> bool {
        self.supported.is_superset(self.enabled)
    }

    /// 要求硬件支持某项能力；不满足返回 [`Error::Unsupported`]。
    pub fn require_supported(self, feature: Feature) -> Result<()> {
        if self.is_supported(feature) {
            Ok(())
        } else {
            Err(Error::unsupported(format!("设备不支持 {feature:?}")))
        }
    }

    /// 要求设备已启用某项能力；不满足返回 [`Error::Unsupported`]。
    pub fn require_enabled(self, feature: Feature) -> Result<()> {
        if self.is_enabled(feature) {
            Ok(())
        } else {
            Err(Error::unsupported(format!("设备未启用 {feature:?}")))
        }
    }

    /// 一次性要求多个已启用能力；错误消息列出缺失项。
    pub fn require_all_enabled(self, features: FeatureSet) -> Result<()> {
        if self.enabled.is_superset(features) {
            return Ok(());
        }
        let missing = features.difference(self.enabled);
        let names = missing
            .iter()
            .map(|feature| format!("{feature:?}"))
            .collect::<Vec<_>>()
            .join(", ");
        Err(Error::unsupported(format!("缺少已启用能力：{names}")))
    }
}

#[cfg(test)]
mod tests;
