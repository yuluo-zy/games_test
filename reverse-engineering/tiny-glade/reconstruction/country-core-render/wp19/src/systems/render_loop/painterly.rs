//! `painterly.rs` — painterly stylized stroke-reveal chain (progressive strokes).
//!
//! Business logic (contracts/painterly-vermeer.md §2):
//!   - `PainterlyPlugin::build` 0x1408a63e0: insert `PainterlyState` (Default);
//!     PreStartup `startup_painterly`.
//!   - `startup_painterly` 0x1408a7fe0: `load_shader_into_library` ×2.
//!   - `painterly_pass` 0x1408a6410: acquire/convolve target once, then each frame
//!     dispatch `generate_strokes` for the newly crossed stroke range
//!     (progress 0..1024) and raster the strokes.
//!
//! Shaders (I from `/painterly/` URI family): `painterly_acquire_target`,
//! `painterly_convolve_target`, `painterly_generate_strokes`,
//! `painterly_raster_strokes`, `painterly_debug_target`.

use crate::consume::pass_builder::{ComputePass, DrawBuilder, RasterPass};
use crate::consume::core::AssetShaderLibrary;
use crate::typed_gap::Gap;

/// Painterly 揭示状态资源（O 名称）。字段偏移为机器码观测（O）。
#[derive(Clone, Copy, Debug)]
#[repr(C)]
pub struct PainterlyState {
    /// +0x00 f64 progress（0..1024 = 笔触总数；每帧 `+step` 递增，封顶 1024）。
    pub progress: f64,
    /// +0x08 u32 state：0=待机 / 1=运行 / 2=完成（O）。
    pub state: u32,
    /// +0x0c temporal extent 句柄（O：`*(u64*)(+0x0c) = extent`）。
    pub temporal_handle: u64,
    /// +0x14 f32 timer（每帧 += dt@+0x58）。
    pub timer: f32,
    /// +0x18.. typed gap。
    pub _gap: [u8; 0x68],
}

impl PainterlyState {
    /// O 名称（0x1408a63d0，7B 常量函数）。真实签名 `-> bool`（PDB 占位 `void()`）。
    /// 语义 I：是否已有绘制目标。
    pub fn has_target(&self) -> bool {
        unimplemented!()
    }
}

/// Painterly bevy Plugin。
pub struct PainterlyPlugin(pub Gap);

impl PainterlyPlugin {
    /// O：0x1408a63e0（`impl$1::build`）。
    /// `insert_resource<PainterlyState>()` + `add_systems<PreStartup, startup_painterly>`。
    pub fn build(&mut self) {
        unimplemented!()
    }
}

/// O：0x1408a7fe0（系统 `(Res<GraphicsDeviceResource>, Res<AssetShaderLibrary>,
/// Res<ShaderWatch>)`）。`load_shader_into_library` ×2（I：painterly 光栅 shader）。
pub fn startup_painterly(_device: &Gap, _shaders: &AssetShaderLibrary, _watch: &Gap) {
    unimplemented!()
}

/// O：0x1408a6410。Painterly 驱动系统（5339B）。
/// 1. AppMode(hash 0xd38dae617fb79c7a) state==7 && byte@+0x08==1 → return。
/// 2. ResMut<PainterlyState>(hash 0x1f18d9c9bdba6c60)：state==2 → 重置。
/// 3. 目标可用标志 local_72（hash 0x48b321ef779dc301 byte@+0xff、0x8fd9dc19c7973bca int@+0x80）。
/// 4. 使能字节==0 → state=2、return。
/// 5. extent (w<<8)/h → get_or_create_temporal ×2；未获取时：
///    “Acquiring painterly target” 日志 → create_raw_resource(strokes 纹理 hash 0x42a9bc70)
///    → acquire_target compute → convolve_target compute → state=1。
/// 6. BufferDesc::new → get_or_create_temporal (strokes SSBO 对)。
/// 7. progress: step = has_target?1024.0 : rate@+0x40*1638.4；progress=min(+step,1024)。
///    floor 递增时：generate_strokes compute 覆盖 {start,end,timer} 区间 +
///    write_rg_storage_buffer(strokes_buf)。
/// 8. painterly_raster_strokes RasterPass：optional_uniform<[u32;2]> + uniforms +
///    bind TextureFromDisk(hash 0x76ef6f6cc99f108b) + read_rg_storage_buffer + draw_no_vao。
///    timer += dt@+0x58。
pub fn painterly_pass(world: &mut Gap) {
    let _ = world;
    unimplemented!()
}

/// `raster_strokes`（inline @270，O）：painterly 光栅笔触 DrawBuilder 组合。
pub fn raster_strokes(_target: &Gap) {
    unimplemented!()
}

/// `painterly_pass::closure$0`（inline @123，O）：dispatch 闭包。
pub fn painterly_dispatch_closure(_dispatch: (u32, u32)) {
    unimplemented!()
}
