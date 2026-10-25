//! `vermeer.rs` — Vermeer stylized stroke-reveal chain (session-start animation).
//!
//! Business logic (contracts/painterly-vermeer.md §3):
//!   - `VermeerPlugin::build` 0x1408a8280: insert `VermeerState` +
//!     `VermeerDebugControl`; PreStartup `startup_vermeer`;
//!     OnExit<AppLoadingState> `deactivate`.
//!   - `startup_vermeer` 0x1408aa720: load 2 raster shaders into the library
//!     (I: `vermeer_raster_strokes`, `vermeer_combine`).
//!   - `vermeer_pass` 0x1408a82f0: reveal driver — acquire/compute the reveal
//!     target, then per-frame `vermeer_render_from_target`, progress 0..1, on
//!     completion release the main camera + send `WriteSettingsFileCmd`, log
//!     "Game reveal complete".
//!   - `vermeer_render_from_target` 0x1408a8d70: `GenerateStrokes` compute
//!     (`/vermeer/generate_strokes.cs.hlsl:cs_main`, O) writing the strokes SSBO,
//!     then `vermeer_raster_strokes` + `vermeer_combine` raster passes.
//!
//! Shader names (O): `vermeer_raster_strokes`, `vermeer_combine` (decompiled
//! string args). Stroke count: 0x8000 (32768) / 0xa00 (2560) toggle (O).

use crate::consume::pass_builder::{AsComputeShaderSource, ComputePass, DrawBuilder, RasterPass, ShaderUri};
use crate::consume::core::AssetShaderLibrary;
use crate::typed_gap::Gap;

/// Vermeer 揭示状态资源（O 名称）。字段偏移为机器码观测（O）；未观测区间 typed gap。
#[derive(Clone, Copy, Debug)]
#[repr(C)]
pub struct VermeerState {
    /// +0x00 帧计数（`vermeer_pass` 每帧 +1；`activate` 清零）。
    pub frame_count: u64,
    /// +0x08..+0x3d typed gap（`into_disable_hold` 复制 0x58B 含此区间）。
    pub _gap0: [u8; 0x36],
    /// +0x3e 主相机释放标记（==7 已释放；activate/deactivate 守卫）。
    pub camera_released: u8,
    /// +0x3f typed gap。
    pub _gap1: u8,
    /// +0x40 揭示进度 0..1（`+0x40 += dt/3` 或冻结时取自控制资源）。
    pub progress: f32,
    /// +0x44 累计 dt。
    pub timer: f32,
    /// +0x48 状态：0=待机 / 1=运行 / 2=完成（O）。
    pub state: u32,
    /// +0x4c 目标句柄（8B，`vermeer_pass` 写入；布局 U）。
    pub handle: u64,
    /// +0x54 hold 标志（`activate` 参数；`into_disable_hold` 清零）。
    pub hold: u8,
    /// +0x55 `activate` 第二参数。
    pub param_b: u8,
    /// +0x56..+0x57 typed gap。
    pub _gap2: [u8; 2],
}

impl VermeerState {
    /// O：0x1408a8060。启动揭示：释放主相机（若未释放）并重置状态。
    /// 参数：`hold` 标志、`param_b`（语义 U）。
    pub fn activate(&mut self, _hold: u8, _param_b: u8) {
        // if state(+0x48)!=2 && +0x3e!=7 → Plugin::finish<MainCameraPlugin> + drop
        // self.frame_count=0; self.camera_released=7; self.progress=0.0; self.state=0;
        // self.hold=hold; self.param_b=param_b;
        unimplemented!()
    }

    /// O：0x1408a8140。完成揭示（state=2）；未释放相机则先释放。
    pub fn deactivate(&mut self) {
        // if state!=2 && +0x3e!=7 → release camera; state=2
        unimplemented!()
    }

    /// O：0x1408a81e0。复制 `other` 88B 到 self 并把 +0x54 hold 清零；
    /// state==2 时 panic。
    pub fn into_disable_hold(&mut self, _other: &VermeerState) {
        // debug_assert state != 2; self.hold = 0; copy other's payload
        unimplemented!()
    }

    /// O 名称（0x1408a8260，16B 常量函数）。真实签名 `-> bool`（`void()` 为 PDB 占位）。
    /// 语义 I：`state != 2`。
    pub fn is_active(&self) -> bool {
        unimplemented!()
    }

    /// O 名称（0x1408a8270，16B 常量函数）；被 `main_camera_update@97` inline。
    /// 语义 I：`state == 1`（揭示运行期间冻结相机）。
    pub fn should_freeze_camera(&self) -> bool {
        unimplemented!()
    }
}

/// Vermeer 调试控制资源（O 名称；插入于 build；布局/用途 U）。
pub struct VermeerDebugControl(pub Gap);

/// `GenerateStrokes` 计算 pass 类型；`AsComputeShaderSource` URI =
/// `/vermeer/generate_strokes.cs.hlsl:cs_main`（O，shader_uri.rs §3）。
/// 结构体字段 U（typed gap）。
pub struct GenerateStrokes(pub Gap);

impl AsComputeShaderSource for GenerateStrokes {
    fn source_hash(&self) -> u64 {
        // O：`as_compute_pipeline` 产出 ShaderUri `/vermeer/generate_strokes.cs.hlsl:cs_main`
        unimplemented!()
    }
    fn as_compute_pipeline(&self) -> ShaderUri {
        // URI (O): `/vermeer/generate_strokes.cs.hlsl:cs_main`
        unimplemented!()
    }
}

/// Vermeer bevy Plugin。
pub struct VermeerPlugin(pub Gap);

impl VermeerPlugin {
    /// O：0x1408a8280（`impl$1::build`）。
    /// `insert_resource<VermeerState>()` + `insert_resource<VermeerDebugControl>()` +
    /// `add_systems<PreStartup, startup_vermeer>` +
    /// `add_systems<OnExit<AppLoadingState>, deactivate>`。
    pub fn build(&mut self) {
        unimplemented!()
    }
}

/// O：0x1408aa720（系统 `(Res<GraphicsDeviceResource>, Res<AssetShaderLibrary>,
/// Res<ShaderWatch>)`）。`load_shader_into_library` ×2（I：`vermeer_raster_strokes`,
/// `vermeer_combine`）。
pub fn startup_vermeer(_device: &Gap, _shaders: &AssetShaderLibrary, _watch: &Gap) {
    unimplemented!()
}

/// O：0x1408a82f0。揭示驱动系统：取 VermeerState + 控制资源（hash
/// `0xbe2e7c0aff5cc813`，名称 U），state==2 走重置/关闭分支，否则：
/// acquire 目标 temporal 对 → by-name compute（I：`vermeer_acquire_target`）→
/// state=1 → 累加 progress/timer → `vermeer_render_from_target` → 完成判定。
pub fn vermeer_pass(world: &mut Gap) {
    // O: get_resource_id(0xbe2e7c0aff5cc813) + (0xa284155e68bfeb3f)
    // state@+0x48 == 2 → reset-or-DAT_143634c9c=0; else DAT_143634c9c=1
    // TextureDesc::new_2d+storage+sampled → get_or_create_temporal → ComputePass::new<&str>
    //   bind textures → PassBuilder::render
    // dt = min(Time@+0x58, 1/30); progress += dt/3 | freeze
    // frame+=1; timer+=dt; vermeer_render_from_target()
    // progress>=1.0 && !frozen && !hold → finish<MainCameraPlugin>; state=2;
    //   WriteSettingsFileCmd event; log "Game reveal complete"
    let _ = world;
    unimplemented!()
}

/// O：0x1408a8d70。Vermeer 绘制链主体（6580B）。
/// 参数（I）：`target` 纹理句柄、分辨率、background 标志、附加字节。
pub fn vermeer_render_from_target(
    _target: &Gap,
    _background: Option<&Gap>,
    _resolution: u32,
    _flags: VermeerRenderFlags,
) {
    // O:
    // 1. stroke_count = 0x8000 | 0xa00（background 标志切换，O）
    // 2. background 为空 → AssetTextureLibrary::get_by_name(hash 0xa68561b396784eaa)
    //    → into_sampled_rg_texture + blur_pyramid()
    // 3. extent_2d(target+4) → create_raw_resource(name hash 0x42a9bc70 = strokes 纹理)
    //    → BufferDesc::new → get_or_create_temporal (strokes SSBO 对)
    // 4. ComputePass::new<GenerateStrokes> → bind background texture (可选) +
    //    write_rg_storage_buffer(strokes_buf) + uniforms f32/i32/f32/bool/bool
    //    → PassBuilder::render
    // 5. get_raster_handle_by_name("vermeer_raster_strokes") → RasterPass + DrawBuilder:
    //    optional_uniform<[u32;2]>(extent) + uniforms + bind TextureFromDisk
    //    (hash 0x76ef6f6cc99f108b) + read_rg_storage_buffer(strokes_buf) + draw_no_vao
    // 6. get_raster_handle_by_name("vermeer_combine") → RasterPass(target) + DrawBuilder:
    //    bind 2×&Handle<Texture> + uniforms + draw_no_vao
    unimplemented!()
}

/// `max_stroke_count`（inline @254，O）：background 存在与否 → 0x8000 | 0xa00。
#[allow(non_snake_case)]
pub fn max_stroke_count(_background: bool) -> u32 {
    unimplemented!()
}

/// `raster_strokes`（inline @347，O）：`vermeer_raster_strokes` DrawBuilder 组合。
pub fn raster_strokes(_target: &Gap) {
    unimplemented!()
}

/// `raster_combine`（inline @395，O）：`vermeer_combine` DrawBuilder 组合。
pub fn raster_combine(_target: &Gap) {
    unimplemented!()
}

/// 揭示渲染旗标（I：background 标志与附加字节；布局 U）。
#[derive(Clone, Copy, Debug, Default)]
pub struct VermeerRenderFlags {
    pub background: bool,
    pub extra: u8,
}
