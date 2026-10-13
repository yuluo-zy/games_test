//! `gpu_lut.rs` — GPU LUT generation resources (`/lut/brdf_fg.cs.hlsl`,
//! `/lut/water_noise.cs.hlsl`, I).
//!
//! Business logic (contracts/lut.md §1):
//!   - `GpuLutPlugin::build` 0x14090e080: build `GpuLutTracker` pre-loaded with
//!     2 `GpuLutApi` entries (1 in demo mode), `insert_resource`.
//!   - `GpuLutTracker::render_luts` 0x14090e280: iterate the 0x30-stride entry
//!     Vec (+0x08 ptr, +0x10 len); per un-rendered entry call vtable+0x28.
//!   - `compute_impl` 0x14090e310 + build closures: per-LUT compute via
//!     `ComputePass::new<&dyn AsComputeShaderSource>`.

use crate::consume::pass_builder::{AsComputeShaderSource, ComputePass, ShaderUri};
use crate::typed_gap::Gap;

/// GPU LUT 生成器 trait（O 名称）。trait 对象 vtable 槽 +0x18（compute 调用）/
/// +0x28（render_luts 内执行，O）。
pub trait GpuLutApi {
    /// O：compute 生成 LUT（inline 宿主 `impl$1::compute` @63）。
    fn compute(&mut self, _pass: &mut ComputePass);
}

/// LUT 描述（O 名称；字段 U）。`GpuLutDesc::new`（inline @80）。
pub struct GpuLutDesc(pub Gap);

/// GPU LUT 追踪器资源（O 名称；布局 O 偏移，语义 I）。
#[derive(Clone, Copy, Debug, Default)]
#[repr(C)]
pub struct GpuLutTracker {
    /// +0x00 typed gap。
    pub _gap0: u64,
    /// +0x08 Vec 数据指针（stride 0x30 条目）。
    pub entries: *mut Gap,
    /// +0x10 条目数。
    pub len: u64,
}

impl GpuLutTracker {
    /// O：0x14090e280。对每条未渲染条目（`+0x28==0`）调 vtable+0x28，标记 +0x28=1。
    pub fn render_luts(&mut self) {
        unimplemented!()
    }
}

/// GpuLut bevy Plugin。
pub struct GpuLutPlugin(pub Gap);

impl GpuLutPlugin {
    /// O：0x14090e080（`impl$0::build`）。构造 2 条目（brdf_fg/water_noise，I）并
    /// `insert_resource<GpuLutTracker>()`。
    pub fn build(&mut self) {
        unimplemented!()
    }
}

/// `impl$3::render_luts::impl$0::compute_impl`（O，0x14090e310）：vtable+0x18 +
/// `ComputePass::new<&dyn AsComputeShaderSource>` + 每 LUT compute。
pub fn compute_impl(_api: &mut dyn GpuLutApi) {
    unimplemented!()
}
