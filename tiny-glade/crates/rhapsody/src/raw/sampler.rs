//! 原 built-in sampler 的策略未恢复；此接口接受明确 Vulkan 描述符。
use crate::Result;
use super::device::Device;
use ash::vk;
use std::sync::Arc;
pub struct Sampler {
    pub(crate) last_use: Arc<crate::resource::SyncCell>,
    pub(crate) device: Arc<Device>,
    pub(crate) handle: vk::Sampler,
    _shared_storage: Option<Arc<SamplerStorage>>,
}

impl Sampler {
    /// The device that owns this object.
    pub fn device(&self) -> &Arc<Device> {
        &self.device
    }
    /// Interop escape hatch: the native handle. It stays owned by `self`; never destroy it or use it after `self` is dropped.
    pub fn raw_handle(&self) -> vk::Sampler {
        self.handle
    }
}
pub(crate) struct SamplerStorage {
    _memory: super::allocator::CpuMemoryReservation,
}
impl Sampler {
    pub(crate) fn new_builtin(
        device: Arc<Device>,
        info: &vk::SamplerCreateInfo<'_>,
    ) -> Result<(Arc<Self>, Arc<SamplerStorage>)> {
        if device.is_device_lost() {
            return Err(crate::Error::DeviceLost);
        }
        let memory = super::allocator::reserve_shared_cpu_metadata::<Self>(
            &device.memory.stats,
            super::allocator::shared_cpu_bytes::<SamplerStorage>()?,
        )?;
        let storage = Arc::new(SamplerStorage { _memory: memory });
        let mut sampler = Self::new(device, info)?;
        sampler._shared_storage = Some(storage.clone());
        Ok((Arc::new(sampler), storage))
    }

    pub fn new(device: Arc<Device>, info: &vk::SamplerCreateInfo<'_>) -> Result<Self> {
        if device.is_device_lost() {
            return Err(crate::Error::DeviceLost);
        }
        let sync_storage = crate::resource::SyncCell::reserve_arc_storage(&device.memory.stats)?;
        let handle = crate::error::observe_vulkan(&device, unsafe {
            device.core.device.create_sampler(info, None)
        })?;
        Ok(Self {
            last_use: crate::resource::SyncCell::for_device(device.generation())
                .into_reserved_arc(sync_storage),
            device,
            handle,
            _shared_storage: None,
        })
    }
}
impl Drop for Sampler {
    fn drop(&mut self) {
        self.device.retire(
            self.last_use.snapshot(),
            super::graveyard::Doomed::Sampler(self.handle),
        );
    }
}

/// 发行版 404 个编译 shader 的反射里出现的内置采样器名（`inc/samplers.hlsl`，名字/set 来自
/// raw-per-bin 反射，O）。原版 `Device` 在 `+0x680` 持有 `HashMap<name, vk::Sampler>`，
/// `get_built_in_sampler`（0x140bfc5b0）按名字查找，缺失时 `panic!("Sampler not found: {name}")`
/// （sampler.rs:201）。创建参数见 `BUILTIN_SAMPLER_SPECS`（`Device::create`
/// 0x140bf2560 内联的 create_samplers，名字池 0x142b16b42，状态逐字节读自 0x140bf5208..0x140bf5823）。
/// 本数组只列出 shader 反射里出现的 9 个名字；原版实际创建 13 个（另有 4 个 `_iso` 变体）。
pub const BUILTIN_SAMPLER_NAMES: [&str; 9] = [
    "sampler_lnc",
    "sampler_llr",
    "sampler_nnc",
    "sampler_llc",
    "sampler_llb",
    "sampler_default",
    "sampler_mipmap",
    "sampler_shadow_nearest",
    "sampler_shadow_linear",
];

/// 原 `create_samplers` 构造的单个采样器配置（O：0x140bf5208..0x140bf5823 的 13 个 0x60B
/// `(name: &str, VkSamplerCreateInfo)` 元素，元素 +0x00 名字指针/+0x08 长度/+0x10 起为 CreateInfo）。
/// 所有元素的 `flags=0`、`mipLodBias=0`、`unnormalizedCoordinates=0`、`pNext=null`
/// （来自序言里清零的栈模板，0x140bf5064..0x140bf51fd）。
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct BuiltinSamplerSpec {
    pub name: &'static str,
    pub mag_filter: vk::Filter,
    pub min_filter: vk::Filter,
    pub mipmap_mode: vk::SamplerMipmapMode,
    pub address_mode: vk::SamplerAddressMode,
    pub anisotropy_enable: bool,
    pub max_anisotropy: f32,
    pub compare_enable: bool,
    pub compare_op: vk::CompareOp,
    pub min_lod: f32,
    pub max_lod: f32,
    pub border_color: vk::BorderColor,
}

impl BuiltinSamplerSpec {
    /// 还原 `VkSamplerCreateInfo`（三个 address mode 恒相同，原机器码 U=V=W）。
    pub fn create_info(&self) -> vk::SamplerCreateInfo<'static> {
        vk::SamplerCreateInfo::default()
            .mag_filter(self.mag_filter)
            .min_filter(self.min_filter)
            .mipmap_mode(self.mipmap_mode)
            .address_mode_u(self.address_mode)
            .address_mode_v(self.address_mode)
            .address_mode_w(self.address_mode)
            .mip_lod_bias(0.0)
            .anisotropy_enable(self.anisotropy_enable)
            .max_anisotropy(self.max_anisotropy)
            .compare_enable(self.compare_enable)
            .compare_op(self.compare_op)
            .min_lod(self.min_lod)
            .max_lod(self.max_lod)
            .border_color(self.border_color)
            .unnormalized_coordinates(false)
    }
}

// 内建采样器表逐项列出原表字段，参数一一对应，不另造参数结构。
#[allow(clippy::too_many_arguments)]
const fn spec(
    name: &'static str,
    mag_min: vk::Filter,
    mipmap_mode: vk::SamplerMipmapMode,
    address_mode: vk::SamplerAddressMode,
    aniso: bool,
    max_anisotropy: f32,
    compare: Option<vk::CompareOp>,
    max_lod: f32,
    border_color: vk::BorderColor,
) -> BuiltinSamplerSpec {
    BuiltinSamplerSpec {
        name,
        mag_filter: mag_min,
        min_filter: mag_min,
        mipmap_mode,
        address_mode,
        anisotropy_enable: aniso,
        // 原值：前五个（lnc/llr/nnc/llc/llb）恒 4.0f（0x40800000，含关闭各向异性的 nnc），
        // 其余 1.0f（0x3f800000）。
        max_anisotropy,
        compare_enable: compare.is_some(),
        compare_op: match compare {
            Some(op) => op,
            None => vk::CompareOp::NEVER,
        },
        min_lod: 0.0,
        max_lod,
        border_color,
    }
}

use vk::{
    BorderColor as B, CompareOp as C, Filter as F, SamplerAddressMode as A, SamplerMipmapMode as M,
};

/// 13 个内置采样器，**按原创建顺序**（名字池顺序 = 元素顺序）。逐项来自机器码常量：
/// address 0=REPEAT/2=CLAMP_TO_EDGE/3=CLAMP_TO_BORDER；border 2=FLOAT_OPAQUE_BLACK/4=FLOAT_OPAQUE_WHITE；
/// compareOp 3=LESS_OR_EQUAL。名字缩写（I 推断，与数值吻合）：第一字母 mag/min 过滤 l/n，第二字母
/// mip 过滤（`llr` 的 mip=LINEAR；`lnc` 的 mip=NEAREST），第三字母 c=clamp-to-edge/r=repeat/b=clamp-to-border；
/// `_iso` = 关闭各向异性。`shadow_nearest` 的 compareEnable 原值为 0（与名字直觉不同，机器码为准）。
pub const BUILTIN_SAMPLER_SPECS: [BuiltinSamplerSpec; 13] = [
    spec(
        "sampler_lnc",
        F::LINEAR,
        M::NEAREST,
        A::CLAMP_TO_EDGE,
        true,
        4.0,
        None,
        1000.0,
        B::FLOAT_TRANSPARENT_BLACK,
    ),
    spec(
        "sampler_llr",
        F::LINEAR,
        M::LINEAR,
        A::REPEAT,
        true,
        4.0,
        None,
        1000.0,
        B::FLOAT_TRANSPARENT_BLACK,
    ),
    spec(
        "sampler_nnc",
        F::NEAREST,
        M::NEAREST,
        A::CLAMP_TO_EDGE,
        false,
        4.0,
        None,
        1000.0,
        B::FLOAT_TRANSPARENT_BLACK,
    ),
    spec(
        "sampler_llc",
        F::LINEAR,
        M::LINEAR,
        A::CLAMP_TO_EDGE,
        true,
        4.0,
        None,
        1000.0,
        B::FLOAT_TRANSPARENT_BLACK,
    ),
    spec(
        "sampler_llb",
        F::LINEAR,
        M::LINEAR,
        A::CLAMP_TO_BORDER,
        true,
        4.0,
        None,
        1000.0,
        B::FLOAT_OPAQUE_BLACK,
    ),
    spec(
        "sampler_lnc_iso",
        F::LINEAR,
        M::NEAREST,
        A::CLAMP_TO_EDGE,
        false,
        1.0,
        None,
        1000.0,
        B::FLOAT_TRANSPARENT_BLACK,
    ),
    spec(
        "sampler_llr_iso",
        F::LINEAR,
        M::LINEAR,
        A::REPEAT,
        false,
        1.0,
        None,
        1000.0,
        B::FLOAT_TRANSPARENT_BLACK,
    ),
    spec(
        "sampler_llc_iso",
        F::LINEAR,
        M::LINEAR,
        A::CLAMP_TO_EDGE,
        false,
        1.0,
        None,
        1000.0,
        B::FLOAT_TRANSPARENT_BLACK,
    ),
    spec(
        "sampler_llb_iso",
        F::LINEAR,
        M::LINEAR,
        A::CLAMP_TO_BORDER,
        false,
        1.0,
        None,
        1000.0,
        B::FLOAT_OPAQUE_BLACK,
    ),
    spec(
        "sampler_default",
        F::LINEAR,
        M::NEAREST,
        A::CLAMP_TO_EDGE,
        false,
        1.0,
        None,
        0.0,
        B::FLOAT_TRANSPARENT_BLACK,
    ),
    spec(
        "sampler_mipmap",
        F::LINEAR,
        M::LINEAR,
        A::REPEAT,
        false,
        1.0,
        None,
        1000.0,
        B::FLOAT_TRANSPARENT_BLACK,
    ),
    spec(
        "sampler_shadow_nearest",
        F::NEAREST,
        M::NEAREST,
        A::CLAMP_TO_BORDER,
        false,
        1.0,
        None,
        0.0,
        B::FLOAT_OPAQUE_WHITE,
    ),
    spec(
        "sampler_shadow_linear",
        F::LINEAR,
        M::NEAREST,
        A::CLAMP_TO_BORDER,
        false,
        1.0,
        Some(C::LESS_OR_EQUAL),
        0.0,
        B::FLOAT_OPAQUE_WHITE,
    ),
];

/// 按名字查规格（原 `HashMap<name, vk::Sampler>` 的键集）。
pub fn builtin_sampler_spec(name: &str) -> Option<&'static BuiltinSamplerSpec> {
    BUILTIN_SAMPLER_SPECS.iter().find(|s| s.name == name)
}

impl crate::resource::GpuUse for Sampler {
    fn visit_uses(&self, visitor: &mut crate::resource::UseVisitor<'_>) -> crate::Result<()> {
        crate::resource::tracked::visit_cells(visitor, [&self.last_use])
    }
}
