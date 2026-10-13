//! `rasterize_terrain_stroke.rs` — W3 地形笔画栅格化 + 高斯模糊（WP8 恢复升级）。
//!
//! 反编译（本轮 Ghidra）：`rasterize_terrain_stroke`@0x1408c7120（行 65）、
//! `closure$0`@行 108、`closure$1`@行 165、`impl$0::as_shader_def_value`@行 43、
//! `impl$5::hash`@行 35。
//!
//! 业务（O）：
//! 1. 守卫：active 笔画非空（`Vec@+0x10`/`Vec@+0x30` 或未栅格化 flag@+0x40）才执行。
//! 2. 遍历 active 笔画（16 位掩码迭代，每笔画 0x30B），过滤 `width>0.01`、点数非空。
//! 3. 每笔画收集 16B 动态常量 `{dynamic_const_addr, (id<<10)|index, extra}`；
//!    `DynamicConstants::allocate` + `push_slice`。
//! 4. `ComputePass::new`（`/terrain_editing/rasterize_terrain_stroke.cs.hlsl`）：
//!    uniform i32 / array<u32,2> / f32；bind preset 9/0xb/0x13/0xe/0xd；
//!    `write_storage_buffer<NormalDebugSsbo>`；`dispatch_with<closure_env$1>`（行 165）。
//! 5. 两遍 `GaussBlur`（`/gauss_blur.cs.hlsl:cs_main`；uniform bool+f32；
//!    bind preset 9 + 新建纹理句柄）后 dispatch。
//! 6. 结束时置 active flag=1（本帧已栅格化）。
//!
//! 帧调用者：frame 外层 @0x1404ec350（W3）。

#![allow(dead_code, unused_imports)]

use crate::consume::pass_builder::{
    AsComputeShaderSource, ComputeDispatchApi, ComputePass, ComputeShaderSource, ShaderDefValue,
    ShaderUri,
};
use crate::render::pass_builder::shader_uri::ComputeShaderSourceHash;
use crate::typed_gap::Gap;

/// `/gauss_blur.cs.hlsl:cs_main` 源码（O：pass-declaration §2.1）。
pub struct GaussBlur;
impl AsComputeShaderSource for GaussBlur {
    fn source_hash(&self) -> u64 {
        ComputeShaderSourceHash::compute("/gauss_blur.cs.hlsl:cs_main")
    }
    fn as_compute_pipeline(&self) -> ComputeShaderSource<'_> {
        ComputeShaderSource {
            uri: ShaderUri::parse("/gauss_blur.cs.hlsl:cs_main"),
            defs: ShaderDefValue::default(),
        }
    }
}

/// 地形重算命令（O 名；register_event `RecalculateHeightMapCmd`；布局 U）。
#[derive(Clone, Debug, Default)]
pub struct RecalculateHeightMapCmd(pub Gap);

/// `as_shader_def_value`（O 名，行 43；shader-def 值路径）。
impl GaussBlur {
    pub fn as_shader_def_value(&self) -> ShaderDefValue {
        ShaderDefValue::default()
    }
    /// `hash`（O 名，行 35；`utils::calculate_hash<(str, GaussBlur)>`）。
    pub fn hash(&self) -> u64 {
        0
    }
}

/// W3 地形笔画栅格化系统（O：0x1408c7120，行 65；流程见文件头）。
pub fn rasterize_terrain_stroke(
    world: &mut Gap,
    _active_strokes: &Gap,
    _terrain: &Gap,
    _normal_debug: &Gap,
) {
    let _ = world;
    unimplemented!(
        "recover rasterize_terrain_stroke: active-stroke iterate + DynamicConstants + dispatch_with + 2x GaussBlur"
    )
}

/// 高斯模糊地形掩码（dispatch_2d，track_gpu_lifetime；I）。
fn gauss_blur(mut pass: ComputePass, _src: &Gap, _dst: &Gap) {
    pass.dispatch_2d(1, 512);
}
