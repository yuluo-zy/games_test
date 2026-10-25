//! `logo_animation.rs` — animated logo (5-stage reveal) + loading handshake.
//!
//! Business logic (contracts/menu-session.md §4):
//!   - `LogoAnimationPlugin::build` 0x140b19d40: `insert_resource<LogoAnimation>()`;
//!     Startup `startup_animated_logo`; Update `animate_logo` with
//!     `(ResMut<LogoAnimation>, Res<Time<()>>, Res<AudioCore>)`.
//!   - `startup_animated_logo` 0x140b19d90: load `tiny_glade_logo` + `logo_branches`
//!     meshes (O strings) from JSON, watch, register shaders, spawn 2 entities
//!     with `(KeepOnLevelUnload, DrawableMeshBundle, DesignatedRenderPass,
//!     DontCastShadow)`.
//!   - `animate_logo` 0x140b1a6b0: 5-stage progression; per stage start play
//!     `event:/UI/Logo/Scribble` (stage 0) / `event:/UI/Logo/Branches` (stage 2);
//!     write the current stage value into the GPU uniform target (+0x10).

use crate::consume::core::{AssetMeshLibrary, AssetShaderLibrary, AudioCore};
use crate::typed_gap::Gap;

/// Logo 动画资源（O 名称）。观测字段：
/// +0x00 u64、+0x08 u64、+0x0c f32 wait 进度（`finish_wait` 置 1.0）、+0x10 u32——
/// 其余（5 阶段 + 5 时长，I）为 typed gap。
#[derive(Clone, Copy, Debug, Default)]
#[repr(C)]
pub struct LogoAnimation {
    /// +0x00（`play` 清零）。
    pub _f0: u64,
    /// +0x08（`play` 清零）。
    pub _f1: u64,
    /// +0x0c wait 进度（`finish_wait` 置 1.0）。
    pub wait_progress: f32,
    /// +0x10（`play` 清零）。
    pub _f2: u32,
    /// 5 阶段进度 + 5 时长（O：animate_logo `pfVar1[0..4]`/`pfVar1[5..9]`；布局 U）。
    pub _stages: Gap,
}

impl LogoAnimation {
    /// O：0x140b19d00。重置动画（清零 +0x00/+0x08/+0x10）。
    pub fn play(&mut self) {
        unimplemented!()
    }

    /// O 名称（0x140b19d10，16B 常量函数）。真实签名 `-> bool`。语义 I：wait 未完成。
    pub fn is_waiting(&self) -> bool {
        unimplemented!()
    }

    /// O 名称（0x140b19d20，16B 常量函数）。语义 I：wait 已完成（+0x0c >= 1.0）。
    pub fn is_finished(&self) -> bool {
        unimplemented!()
    }

    /// O：0x140b19d30。结束等待（+0x0c = 1.0）。
    pub fn finish_wait(&mut self) {
        unimplemented!()
    }
}

/// 动画 logo 实体组件（O 名称；布局 U）。
pub struct AnimatedLogo(pub Gap);

/// LogoAnimation bevy Plugin。
pub struct LogoAnimationPlugin(pub Gap);

impl LogoAnimationPlugin {
    /// O：0x140b19d40（`impl$2::build`）。
    pub fn build(&mut self) {
        // insert_resource<LogoAnimation>()
        // add_systems<Startup, startup_animated_logo>
        // add_systems<Update, animate_logo>
        unimplemented!()
    }
}

/// O：0x140b19d90（Startup 系统，6 参）。生成 logo + branches 网格实体。
pub fn startup_animated_logo(
    _device: &Gap,
    _meshes: &AssetMeshLibrary,
    _mesh_watch: &Gap,
    _shaders: &AssetShaderLibrary,
    _shader_watch: &Gap,
) {
    // O: load_json_as_mesh<&str> + MeshWatch::watch; add("tiny_glade_logo")/add("logo_branches")
    //     load_shader_into_library; spawn entities with (KeepOnLevelUnload,
    //     DrawableMeshBundle, DesignatedRenderPass, DontCastShadow)
    unimplemented!()
}

/// O：0x140b1a6b0（Update 系统 `(ResMut<LogoAnimation>, Res<Time<()>>, Res<AudioCore>)`）。
/// 5 阶段进度；阶段开始播音频；写当前值到 GPU uniform 目标。
pub fn animate_logo(_logo: &mut LogoAnimation, _time: &Gap, _audio: &AudioCore) {
    // O: dt = min(Time::delta_seconds@+0x58, 1)
    //     first stage[i] < 1.0 → stage[i] = clamp(stage[i] + dt/duration[i], eps, 1.0)
    //     stage[i]==0.0 → play "event:/UI/Logo/Scribble" | "event:/UI/Logo/Branches"
    //     *(target_ptr@+0x10) = current stage value
    unimplemented!()
}
