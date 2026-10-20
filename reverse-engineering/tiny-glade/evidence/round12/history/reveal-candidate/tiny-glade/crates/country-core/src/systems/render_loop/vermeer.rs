//! `vermeer.rs` — Vermeer stylized stroke-reveal chain (session-start animation).
//!
//! 全部来自机器码（O/I/U 标注）。与 `painterly.rs` 同一套宿主缝（[`PassHost`] + 描述符），这里只加 Vermeer 专有的
//! 资源/静态变量缝（[`VermeerHost`]）。
//!
//! 过程 → Rust：
//! - 0x1408a8060 `VermeerState::activate`            → [`VermeerState::activate`]
//! - 0x1408a8140 `VermeerState::deactivate`          → [`VermeerState::deactivate`]
//! - 0x1408a81e0 `VermeerState::into_disable_hold`   → [`VermeerState::into_disable_hold`]（阶段 == 2 → panic "must be activated first"）
//! - 0x1408a8260 `is_active`                         → `phase != Disabled`
//! - 0x1408a8270 `should_freeze_camera`              → `phase != Disabled && freeze_camera`（`cmp [+0x48],2; setne; and al,[+0x55]`）
//! - 0x1408a8280 `VermeerPlugin::build`              → [`VermeerPlugin::build`]
//! - 0x1408a82f0 `vermeer_pass`（2679 B）             → [`vermeer_pass`]
//! - 0x1408a8d70 `vermeer_render_from_target`（6569 B）→ [`vermeer_render_from_target`]
//! - 0x1408aa720 `startup_vermeer`                   → [`startup_vermeer`]
//! - 0x1408aab70 `impl$7::as_compute_pipeline`       → [`GenerateStrokes`] 的 `AsComputeShaderSource`
//!
//! `VermeerState` 是 `enum2$`（O：判别在 `+0x48`：0 待采集 / 1 已采集 / 2 未启用）。`+8` 是 `Option<托管纹理>`
//! （`+0x3e == 7` = None；O：`Plugin::finish<MainCameraPlugin>` 只是 ICF 合并的空函数 `ret`，真正的动作是
//! `ManagedHandle::drop(+8)`，即释放这张"揭示源纹理"）。

use super::painterly::{
    ACCESS_COMPUTE_READ, ACCESS_COMPUTE_WRITE, ACCESS_DRAW_READ, BUFFER_ALIGN, BufferDesc,
    ComputeOp, ComputeSource, FORMAT_COLOR_TARGET, KEY_GEN_TARGET, KEY_RESOLUTION,
    KEY_STROKE_TEXTURE, KEY_STROKES_SSBO, LogLevel, PRESET_STROKE_TEXTURE, PassHost, RasterColor,
    RasterOp, RasterPassDesc, ShaderLoader, TexBinding, TexRef, TextureDesc, Uniform, compute_desc,
    neg, unwrap_temporal,
};
use crate::render::pass_builder::shader_uri::{
    AsComputeShaderSource, ComputePipelineDesc, ComputeShaderSource, ShaderDefValue, ShaderDefine,
    ShaderUri,
};

// ---------------------------------------------------------------------------------------------
// 机器码常量（O）
// ---------------------------------------------------------------------------------------------

/// 笔触数：photomode 0xa00，否则 0x8000（O：`cmovne r15, 0xa00`，初值 0x8000）。
pub const STROKES_PHOTOMODE: u32 = 0xa00;
pub const STROKES_DEFAULT: u32 = 0x8000;
/// 每帧 dt 上限 1/30 s（O：`minss`，f32 0x3d088889 = 0.033333335）。
pub const MAX_DT: f32 = 0.033333335;
/// 揭示时长 3 s（O：`divss 3.0`）。
pub const REVEAL_SECONDS: f32 = 3.0;
/// 前两帧不推进进度（O：`cmp r8, 2; setb dl`）。
pub const WARMUP_FRAMES: u64 = 2;
/// 纹理格式（O：`new_2d(.., 0x25)`）。
pub const FORMAT_TARGET: u32 = FORMAT_COLOR_TARGET;

/// 控制资源 TypeId（O：`VermeerDebugControl`，`get_resource_id(-0x41d183f500a337ed, -0x70b9a6b1dd0fe28a)`）。
pub const TYPE_ID_DEBUG_CONTROL: (u64, u64) = (neg(0x41d183f500a337ed), neg(0x70b9a6b1dd0fe28a));
/// `VermeerState` 资源 TypeId（O）。
pub const TYPE_ID_VERMEER_STATE: (u64, u64) = (neg(0x5d7beaa1974014c1), neg(0x7b5d4cbbe439b5c6));
/// `Events<StartupRevealFinished>` TypeId（O）。
pub const TYPE_ID_REVEAL_FINISHED_EVENTS: (u64, u64) = (neg(0x2706c2f9b15629e4), 0x6447617066d9832f);
/// `AssetTextureLibrary` TypeId（O）。
pub const TYPE_ID_TEXTURE_LIBRARY: (u64, u64) = (neg(0x597a9e4c6987b156), neg(0x0a028b19050b12ec));

/// 背景纹理名哈希（O：静态 `0x143634d90 != 0` ? 前者 : 后者）。
pub const BACKGROUND_NAME_ALT: u64 = 0x52c2c594db0dcfb0;
pub const BACKGROUND_NAME_DEFAULT: u64 = neg(0x08c92db3f13a4289);

// 绑定名哈希（O）。
pub const KEY_BG_TEXTURE: u64 = 0x7f41ce7531a06fe3;
pub const KEY_PYRAMID: u64 = KEY_GEN_TARGET; // -0x12d6329bc24d9912：generate/combine 里是金字塔，acquire 里是 vermeer_target 输出
pub const KEY_ACQUIRE_SOURCE: u64 = neg(0x266871abe31656a2);
pub const KEY_COMBINE_OUTPUT: u64 = neg(0x77de65c519676d4e);
pub const KEY_U_TIMER: u64 = neg(0x26ac9f8bb78239bb);
pub const KEY_U_STROKE_COUNT: u64 = 0x5dd2cf99e4b48447;
pub const KEY_U_PROGRESS: u64 = neg(0x2e6262d18a0e4303);
pub const KEY_U_PHOTOMODE: u64 = 0x15f142fdf7f6902c;
pub const KEY_U_REUSE: u64 = neg(0x565ef6489f1126fe);

pub const PASS_ACQUIRE: &str = "Vermeer acquire target";
pub const PATH_ACQUIRE: &str = "/vermeer/acquire_target.cs.hlsl";
pub const PASS_GENERATE: &str = "Vermeer generate strokes";
pub const PASS_RASTER_STROKES: &str = "Vermeer raster strokes";
/// 注意：combine pass 的名字在原版里就是 "Vermeer debug target"（O：0x142a9bd60，len 0x14）。
pub const PASS_COMBINE: &str = "Vermeer debug target";
pub const SHADER_RASTER_STROKES: &str = "vermeer_raster_strokes";
pub const SHADER_COMBINE: &str = "vermeer_combine";
pub const PATH_RASTER_STROKES: &str = "/vermeer/raster_strokes.raster.hlsl";
pub const PATH_COMBINE: &str = "/vermeer/combine.raster.hlsl";
pub const NAME_TARGET: &str = "vermeer_target";
pub const NAME_OUTPUT: &str = "vermeer_output";
pub const NAME_STROKES: &str = "vermeer_strokes";
pub const LOG_TARGET: &str = "country_core::systems::render_loop::vermeer";
pub const SOURCE_FILE: &str = "crates/country-core/src/systems/render_loop/vermeer.rs";
pub const LOG_ACQUIRING: &str = "Acquiring Vermeer target";
pub const LOG_REVEAL_COMPLETE: &str = "Game reveal complete.";
/// `into_disable_hold` 的 panic 文案（O：0x142a9ba78，len 0x17）。
pub const PANIC_MUST_ACTIVATE: &str = "must be activated first";
/// `GenerateStrokes` 的 shader URI（O：0x142a9bdb3，len 0x29）与宏名（O：`PERM_photomode`，len 0xe）。
pub const GENERATE_STROKES_URI: &str = "/vermeer/generate_strokes.cs.hlsl:cs_main";
pub const PERM_PHOTOMODE: &str = "PERM_photomode";

// ---------------------------------------------------------------------------------------------
// GenerateStrokes
// ---------------------------------------------------------------------------------------------

/// `GenerateStrokes` compute 来源（O：`ComputePass::new<GenerateStrokes>` 的第 4 个参数是 1 字节 =
/// `photomode`；`as_compute_pipeline` 用 `u8` 的 `Display` 格式化出 `PERM_photomode` 的值）。
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Default)]
pub struct GenerateStrokes {
    pub photomode: u8,
}

impl GenerateStrokes {
    /// O 0x1408aab70：`ComputePipelineDesc { path: "/vermeer/generate_strokes.cs.hlsl", entry: "cs_main",
    /// defines: [("PERM_photomode", photomode.to_string())] }`（`path`/`entry` 的 0x21 / 7 字节来自两次 `alloc`）。
    pub fn pipeline_desc(&self) -> ComputePipelineDesc {
        let uri = ShaderUri::parse(GENERATE_STROKES_URI);
        ComputePipelineDesc {
            path: uri.path.to_owned(),
            entry: uri.entry.unwrap_or("cs_main").to_owned(),
            defines: vec![(PERM_PHOTOMODE.to_owned(), self.photomode.to_string())],
        }
    }
}

impl AsComputeShaderSource for GenerateStrokes {
    /// O 0x1415fea40：`calculate_hash(&(URI(len 0x29), &GenerateStrokes))`（`derive(Hash)`，1 字节）。
    fn source_hash(&self) -> u64 {
        crate::render::utils::calculate_hash(&(GENERATE_STROKES_URI, self))
    }
    /// 特性值取 `photomode`（I：现有 trait 只能携带 `ShaderDefValue(u64)`；完整定义见 [`GenerateStrokes::pipeline_desc`]）。
    fn as_compute_pipeline(&self) -> ComputeShaderSource<'_> {
        ComputeShaderSource {
            uri: ShaderUri::parse(GENERATE_STROKES_URI),
            defs: ShaderDefValue(self.photomode as u64),
        }
    }
    /// `PERM_photomode`（O 0x1408aab70）经 `ComputePass::new` 进入 `ComputePipelineDesc.defines`（B 阶段 i_infra 接线）。
    fn pipeline_defines(&self) -> Vec<ShaderDefine> {
        self.pipeline_desc().defines
    }
}

// ---------------------------------------------------------------------------------------------
// 状态
// ---------------------------------------------------------------------------------------------

/// `VermeerState` 阶段（O：`u32 @+0x48`）。
#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
#[repr(u32)]
pub enum VermeerPhase {
    /// 0：已启用，目标尚未采集。
    #[default]
    Pending = 0,
    /// 1：目标已采集。
    Ready = 1,
    /// 2：未启用 / 已完成。
    Disabled = 2,
}

/// 托管纹理（O：`+8` 处的 `Option<Texture>`；`Drop` = `ManagedHandle::drop`）。
#[derive(Debug, PartialEq, Eq)]
pub struct ManagedTexture(pub TexRef);

/// Vermeer 揭示状态资源（0x58 B）。
#[derive(Debug, PartialEq, bevy_ecs::resource::Resource)]
pub struct VermeerState {
    /// `+0x00`：帧计数（每帧 +1；`activate` 清零）。
    pub frame_count: u64,
    /// `+0x08`：揭示源纹理（`+0x3e == 7` = None）。
    pub background: Option<ManagedTexture>,
    /// `+0x40`：揭示进度 0..1。
    pub progress: f32,
    /// `+0x44`：累计 dt。
    pub timer: f32,
    /// `+0x48`：阶段。
    pub phase: VermeerPhase,
    /// `+0x4c`：目标分辨率（仅 Ready 有效；O：`extent_2d(vermeer_target)` 的 u64）。
    pub target_extent: [u32; 2],
    /// `+0x54`：hold（为真时进度不推进、不会完成）。
    pub hold: bool,
    /// `+0x55`：冻结相机标志（`should_freeze_camera` 读它）。
    pub freeze_camera: bool,
}

impl Default for VermeerState {
    fn default() -> Self {
        Self {
            frame_count: 0,
            background: None,
            progress: 0.0,
            timer: 0.0,
            phase: VermeerPhase::Pending,
            target_extent: [0, 0],
            hold: false,
            freeze_camera: true,
        }
    }
}

impl VermeerState {
    /// O 0x1408a8060：`phase != Disabled && background.is_some()` → 先释放（`ManagedHandle::drop`），
    /// 然后 `frame=0, background=None, progress=timer=0, phase=Pending, hold=a, freeze_camera=b`。
    /// 注意参数顺序：`dl → +0x54(hold)`，`r8b → +0x55(freeze_camera)`。
    pub fn activate(&mut self, hold: bool, freeze_camera: bool) {
        if self.phase != VermeerPhase::Disabled {
            self.background = None; // drop
        }
        self.frame_count = 0;
        // 原版在 Disabled 时不 drop，只覆盖标签（Disabled 下 background 恒为 None，故等价）。
        self.background = None;
        self.progress = 0.0;
        self.timer = 0.0;
        self.phase = VermeerPhase::Pending;
        self.hold = hold;
        self.freeze_camera = freeze_camera;
    }

    /// O 0x1408a8140：`phase != Disabled && background.is_some()` → drop；`phase = Disabled`。
    pub fn deactivate(&mut self) {
        if self.phase != VermeerPhase::Disabled {
            self.background = None;
        }
        self.phase = VermeerPhase::Disabled;
    }

    /// O 0x1408a81e0：按值消费；`phase == Disabled` → `panic_fmt("must be activated first")`；
    /// 否则 `hold = false` 后整体复制（0x58 B）。
    pub fn into_disable_hold(mut self) -> Self {
        if self.phase == VermeerPhase::Disabled {
            panic!("{}", PANIC_MUST_ACTIVATE);
        }
        self.hold = false;
        self
    }

    /// O 0x1408a8260：`cmp [+0x48],2; setne`。
    pub fn is_active(&self) -> bool {
        self.phase != VermeerPhase::Disabled
    }

    /// O 0x1408a8270：`(phase != 2) & [+0x55]`。
    pub fn should_freeze_camera(&self) -> bool {
        self.phase != VermeerPhase::Disabled && self.freeze_camera
    }
}

/// Vermeer 调试控制资源（O：≤ 8 字节，`insert_resource(0)`；`+0` f32 冻结进度，`+4` restart，`+5` freeze）。
#[derive(Clone, Copy, Debug, Default, PartialEq, bevy_ecs::resource::Resource)]
pub struct VermeerDebugControl {
    pub progress: f32,
    pub restart: bool,
    pub freeze: bool,
}

/// Vermeer bevy Plugin（O：两个字节，`[0]` 决定初始阶段，`[1]` → `hold`）。
#[derive(Clone, Copy, Debug, Default)]
pub struct VermeerPlugin {
    pub enabled: bool,
    pub hold: bool,
}

/// `bevy_app::App` 缝：`VermeerPlugin::build` 的四个调用（O 顺序）。
pub trait VermeerApp {
    fn insert_vermeer_state(&mut self, state: VermeerState);
    fn insert_vermeer_debug_control(&mut self, control: VermeerDebugControl);
    fn add_pre_startup_system(&mut self, name: &'static str);
    /// `add_systems(OnExit<AppLoadingState>(variant), closure(ResMut<VermeerState>))`；O：`mov dl, 1`。
    fn add_on_exit_app_loading_system(&mut self, state_variant: u8, name: &'static str);
}

impl VermeerPlugin {
    /// O 0x1408a8280：状态初值 `phase = (enabled ^ 1) * 2`（enabled → 0，否则 2），`hold = plugin[1]`，
    /// `freeze_camera = true`（`byte [+0x55] = 1`），`background = None`。
    pub fn build<A: VermeerApp>(&self, app: &mut A) {
        let phase = if self.enabled {
            VermeerPhase::Pending
        } else {
            VermeerPhase::Disabled
        };
        app.insert_vermeer_state(VermeerState {
            phase,
            hold: self.hold,
            freeze_camera: true,
            ..Default::default()
        });
        app.insert_vermeer_debug_control(VermeerDebugControl::default());
        app.add_pre_startup_system("startup_vermeer");
        app.add_on_exit_app_loading_system(1, "deactivate");
    }
}

/// O 0x1408aa720：按调用序加载两个光栅 shader。
pub fn startup_vermeer<L: ShaderLoader>(loader: &mut L) {
    loader.load_shader_into_library(PATH_RASTER_STROKES, SHADER_RASTER_STROKES);
    loader.load_shader_into_library(PATH_COMBINE, SHADER_COMBINE);
}

// ---------------------------------------------------------------------------------------------
// 宿主缝
// ---------------------------------------------------------------------------------------------

/// Vermeer 专有的外部调用点。
pub trait VermeerHost: PassHost {
    /// `VermeerDebugControl` 资源（缺失 → `Option::unwrap` panic）。
    fn debug_control(&self) -> Option<VermeerDebugControl>;
    /// 静态 `0x143634d90`（O：只读；决定背景纹理名哈希，I：调试开关）。
    fn alt_background(&self) -> bool;
    /// `AssetTextureLibrary::get_by_name(hash)` + `into_sampled_rg_texture`（O 0x14093ca90 / 0x140bd6470）。
    fn library_texture(&mut self, name_hash: u64) -> TexRef;
    /// `compute_passes::blur_pyramid::blur_pyramid(world, graph, input)`（O 0x1409ee1e0）。
    fn blur_pyramid(&mut self, input: TexRef) -> TexRef;
    /// `Events<StartupRevealFinished>::send`（O：`EventInstance` 追加 + 计数 +1；资源缺失 → `None` → unwrap panic）。
    fn send_startup_reveal_finished(&mut self) -> Option<()>;
}

/// 静态 `0x143634c9c`（O：`vermeer_pass` 里读后置 1；禁用且无 restart 时置 0）。
#[derive(bevy_ecs::prelude::Resource, Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct VermeerPassLocals {
    pub prev_active: bool,
}

/// `vermeer_pass` 的输入（O：`r8` / `r9b` / 栈参数）。
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct VermeerPassParams {
    /// `r8`：场景目标纹理（extent 来源 + acquire 的输入 + combine 的颜色附件）。
    pub target: TexRef,
    /// `r9b`：photomode（→ `GenerateStrokes.photomode`、笔触数、两个 bool uniform）。
    pub photomode: bool,
    /// 栈参数（O：`[rbp+0x400]`，原样传给 `render_from_target` 的 `arg9`）：真则跳过 generate strokes pass。
    pub skip_generate: bool,
}

/// `vermeer_render_from_target` 的输入。
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct VermeerRenderParams {
    /// `r8`：`state.background`（Some → 绑定它；None → 绑定金字塔）。
    pub background: Option<TexRef>,
    /// `r9`：金字塔来源（Some → 直接用；None → 按名取库纹理）。
    pub source: Option<TexRef>,
    /// 栈参数 5：场景目标。
    pub target: TexRef,
    /// 栈参数 6：累计 timer。
    pub timer: f32,
    /// 栈参数 7：揭示进度。
    pub progress: f32,
    /// 栈参数 8：photomode。
    pub photomode: bool,
    /// 栈参数 9：跳过 generate pass。
    pub skip_generate: bool,
    /// 栈参数 10：`prev_active & 目标 temporal 已复用`（→ `-0x565e…` uniform 与 笔触 temporal 已复用 做 AND）。
    pub reuse: bool,
}

/// 笔触数。
pub fn max_stroke_count(photomode: bool) -> u32 {
    if photomode {
        STROKES_PHOTOMODE
    } else {
        STROKES_DEFAULT
    }
}

/// O 0x1408a8d70：Vermeer 绘制链（generate strokes → raster strokes → combine）。
///
/// O 序：
/// 1. 笔触数；金字塔来源（缺省 → 库纹理，名哈希由静态开关选）→ `blur_pyramid`。
/// 2. `vermeer_output`（fmt 0x25，extent = target）原始资源；`vermeer_strokes` temporal（`count << 4` B）。
/// 3. `!skip_generate`：compute `GenerateStrokes{photomode}`。
/// 4. raster "Vermeer raster strokes"（`6 × count` 顶点）→ 5. raster "Vermeer debug target"（combine，3 顶点）。
pub fn vermeer_render_from_target<H: VermeerHost>(host: &mut H, p: VermeerRenderParams) {
    let stroke_count = max_stroke_count(p.photomode);
    // 1.
    let pyramid_input = match p.source {
        Some(t) => t,
        None => {
            let name = if host.alt_background() {
                BACKGROUND_NAME_ALT
            } else {
                BACKGROUND_NAME_DEFAULT
            };
            host.library_texture(name)
        }
    };
    let pyramid = host.blur_pyramid(pyramid_input);
    // 2.
    let extent = host.texture_extent(p.target);
    let output = host.create_raw_texture(
        NAME_OUTPUT,
        TextureDesc {
            extent,
            format: FORMAT_TARGET,
            storage: false,
            sampled: false,
        },
    );
    let strokes = unwrap_temporal(host.temporal_buffer(
        NAME_STROKES,
        BufferDesc {
            size: stroke_count << 4,
            align: BUFFER_ALIGN,
            flag: true,
        },
    ));
    // 3.
    if !p.skip_generate {
        let ops = vec![
            ComputeOp::BindTexture {
                key: KEY_BG_TEXTURE,
                access: ACCESS_COMPUTE_READ,
                tex: p.background.unwrap_or(pyramid),
                write: false,
            },
            ComputeOp::BindTexture {
                key: KEY_PYRAMID,
                access: ACCESS_COMPUTE_READ,
                tex: pyramid,
                write: false,
            },
            ComputeOp::WriteRgStorageBuffer {
                key: KEY_STROKES_SSBO,
                buf: strokes.buf,
            },
            ComputeOp::WriteUniform {
                key: KEY_U_TIMER,
                value: Uniform::F32(p.timer),
            },
            ComputeOp::WriteUniform {
                key: KEY_U_STROKE_COUNT,
                value: Uniform::I32(stroke_count as i32),
            },
            ComputeOp::WriteUniform {
                key: KEY_U_PROGRESS,
                value: Uniform::F32(p.progress),
            },
            ComputeOp::WriteUniform {
                key: KEY_U_PHOTOMODE,
                value: Uniform::Bool(p.photomode),
            },
            ComputeOp::WriteUniform {
                key: KEY_U_REUSE,
                value: Uniform::Bool(p.reuse & strokes.reused),
            },
        ];
        host.submit_compute(compute_desc(
            PASS_GENERATE,
            ComputeSource::GenerateStrokes {
                photomode: p.photomode as u8,
            },
            ops,
            [stroke_count, 1, 1],
        ));
    }
    // 4. raster strokes：附件 = vermeer_output 视图，视口 = output 的 extent。
    let shader = host.raster_shader(SHADER_RASTER_STROKES).unwrap();
    host.submit_raster(RasterPassDesc {
        name: PASS_RASTER_STROKES,
        shader_name: SHADER_RASTER_STROKES,
        shader,
        color: RasterColor::View(output),
        viewport: extent,
        ops: vec![
            RasterOp::OptionalUniform {
                key: KEY_RESOLUTION,
                value: extent,
            },
            RasterOp::Uniform {
                key: KEY_U_PROGRESS,
                value: Uniform::F32(p.progress),
            },
            RasterOp::Uniform {
                key: KEY_U_PHOTOMODE,
                value: Uniform::Bool(p.photomode),
            },
            RasterOp::Uniform {
                key: KEY_U_STROKE_COUNT,
                value: Uniform::I32(stroke_count as i32),
            },
            RasterOp::BindTexture {
                key: KEY_STROKE_TEXTURE,
                access: ACCESS_DRAW_READ,
                tex: TexBinding::FromDisk(PRESET_STROKE_TEXTURE),
            },
            RasterOp::ReadRgStorageBuffer {
                key: KEY_STROKES_SSBO,
                buf: strokes.buf,
            },
        ],
        raster_state_edit: true,
        vertex_count: stroke_count * 6,
    });
    // 5. combine：附件 = 场景目标。
    let shader = host.raster_shader(SHADER_COMBINE).unwrap();
    host.submit_raster(RasterPassDesc {
        name: PASS_COMBINE,
        shader_name: SHADER_COMBINE,
        shader,
        color: RasterColor::Texture(p.target),
        viewport: extent,
        ops: vec![
            RasterOp::OptionalUniform {
                key: KEY_RESOLUTION,
                value: extent,
            },
            RasterOp::Uniform {
                key: KEY_U_PROGRESS,
                value: Uniform::F32(p.progress),
            },
            RasterOp::Uniform {
                key: KEY_U_PHOTOMODE,
                value: Uniform::Bool(p.photomode),
            },
            RasterOp::Uniform {
                key: KEY_U_STROKE_COUNT,
                value: Uniform::I32(stroke_count as i32),
            },
            RasterOp::BindTexture {
                key: KEY_PYRAMID,
                access: ACCESS_DRAW_READ,
                tex: TexBinding::Rg(pyramid),
            },
            RasterOp::BindTexture {
                key: KEY_COMBINE_OUTPUT,
                access: ACCESS_DRAW_READ,
                tex: TexBinding::Rg(output),
            },
        ],
        raster_state_edit: true,
        vertex_count: 3,
    });
}

/// O 0x1408a82f0：揭示驱动系统。返回值 = 进入时阶段 != Disabled（O：`cmp [rbp+0x374],2; setne`）。
///
/// O 序：
/// - 阶段 Disabled：控制.restart → 重置（`frame=0, background=None, progress=timer=0, phase=Pending, hold=freeze=0`），
///   否则 `prev_active = false`；返回 false。
/// - 否则：`prev_active=true`；`vermeer_target` temporal（fmt 0x25 storage+sampled，extent = target）；Trace 日志；
///   compute "Vermeer acquire target"（读 target @-0x2668、写 vermeer_target @-0x12d6，线程 = target extent）；
///   阶段 = Ready，`target_extent = extent(vermeer_target)`。
/// - dt = min(delta_f32, 1/30)；冻结 → progress = 控制.progress；否则 `(frame < 2 || hold)` 不动，否则 `+= dt/3`。
/// - `frame += 1; timer += dt`；`vermeer_render_from_target`。
/// - `progress >= 1.0 && !控制.freeze && !hold` → 释放 background、phase = Disabled、发 `StartupRevealFinished`、
///   Info 日志 "Game reveal complete."。
pub fn vermeer_pass<H: VermeerHost>(
    host: &mut H,
    locals: &mut VermeerPassLocals,
    state: &mut VermeerState,
    p: VermeerPassParams,
) -> bool {
    let control = host.debug_control().unwrap();
    let orig_phase = state.phase;
    if orig_phase == VermeerPhase::Disabled {
        if control.restart {
            state.frame_count = 0;
            state.background = None;
            state.progress = 0.0;
            state.timer = 0.0;
            state.phase = VermeerPhase::Pending;
            state.hold = false;
            state.freeze_camera = false;
        } else {
            locals.prev_active = false;
        }
        return false;
    }

    let extent = host.texture_extent(p.target);
    let prev_active = locals.prev_active;
    locals.prev_active = true;
    let target = unwrap_temporal::<super::painterly::TemporalTexture>(host.temporal_texture(
        NAME_TARGET,
        TextureDesc {
            extent,
            format: FORMAT_TARGET,
            storage: true,
            sampled: true,
        },
    ));
    host.log(LogLevel::Trace, LOG_TARGET, SOURCE_FILE, LOG_ACQUIRING);
    host.submit_compute(compute_desc(
        PASS_ACQUIRE,
        ComputeSource::Uri(PATH_ACQUIRE),
        vec![
            ComputeOp::BindTexture {
                key: KEY_ACQUIRE_SOURCE,
                access: ACCESS_COMPUTE_READ,
                tex: p.target,
                write: false,
            },
            ComputeOp::BindTexture {
                key: KEY_PYRAMID,
                access: ACCESS_COMPUTE_WRITE,
                tex: target.tex,
                write: true,
            },
        ],
        [extent[0], extent[1], 1],
    ));
    state.phase = VermeerPhase::Ready;
    state.target_extent = host.texture_extent(target.tex);

    let time = host.time().unwrap();
    let dt = if time.delta_secs < MAX_DT {
        time.delta_secs
    } else {
        MAX_DT
    };
    if control.freeze {
        state.progress = control.progress;
    } else if !(state.frame_count < WARMUP_FRAMES || state.hold) {
        state.progress += dt / REVEAL_SECONDS;
    }
    state.frame_count += 1;
    state.timer += dt;

    vermeer_render_from_target(
        host,
        VermeerRenderParams {
            background: state.background.as_ref().map(|b| b.0),
            source: Some(target.tex),
            target: p.target,
            timer: state.timer,
            progress: state.progress,
            photomode: p.photomode,
            skip_generate: p.skip_generate,
            reuse: prev_active & target.reused,
        },
    );

    if state.progress >= 1.0 && !control.freeze && !state.hold {
        if state.phase != VermeerPhase::Disabled {
            state.background = None; // ManagedHandle::drop
        }
        state.phase = VermeerPhase::Disabled;
        host.send_startup_reveal_finished().unwrap();
        host.log(LogLevel::Info, LOG_TARGET, SOURCE_FILE, LOG_REVEAL_COMPLETE);
    }
    orig_phase != VermeerPhase::Disabled
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::systems::render_loop::painterly::{
        AppModeView, BufRef, ComputePassDesc, HostError, ShaderHandle, TemporalBuffer,
        TemporalTexture, TimeView,
    };

    #[derive(Default)]
    struct Rec {
        calls: Vec<String>,
        control: Option<VermeerDebugControl>,
        delta: f32,
        extent: [u32; 2],
        alt: bool,
        reuse_target: bool,
        reuse_strokes: bool,
        compute: Vec<ComputePassDesc>,
        raster: Vec<RasterPassDesc>,
        events: u32,
        no_events: bool,
    }

    impl PassHost for Rec {
        fn app_mode(&self) -> Option<AppModeView> {
            None
        }
        fn time(&self) -> Option<TimeView> {
            Some(TimeView {
                delta_secs_f64: self.delta as f64,
                delta_secs: self.delta,
            })
        }
        fn raster_shader(&self, name: &str) -> Option<ShaderHandle> {
            Some(ShaderHandle(name.len() as u64))
        }
        fn texture_extent(&self, _t: TexRef) -> [u32; 2] {
            self.extent
        }
        fn temporal_texture(
            &mut self,
            name: &str,
            d: TextureDesc,
        ) -> Result<TemporalTexture, HostError> {
            self.calls.push(format!(
                "temporal_tex {name} {:?} {:#x} {} {}",
                d.extent, d.format, d.storage, d.sampled
            ));
            Ok(TemporalTexture {
                tex: TexRef(30),
                reused: self.reuse_target,
            })
        }
        fn temporal_buffer(
            &mut self,
            name: &str,
            d: BufferDesc,
        ) -> Result<TemporalBuffer, HostError> {
            self.calls.push(format!(
                "temporal_buf {name} {} {} {}",
                d.size, d.align, d.flag
            ));
            Ok(TemporalBuffer {
                buf: BufRef(40),
                reused: self.reuse_strokes,
            })
        }
        fn create_raw_texture(&mut self, name: &str, d: TextureDesc) -> TexRef {
            self.calls.push(format!(
                "raw {name} {:?} {:#x} {} {}",
                d.extent, d.format, d.storage, d.sampled
            ));
            TexRef(31)
        }
        fn texture_desc(&self, _t: TexRef) -> TextureDesc {
            unreachable!()
        }
        fn log(&mut self, l: LogLevel, t: &str, _f: &str, m: &str) {
            self.calls.push(format!("log {l:?} {t} {m}"));
        }
        fn submit_compute(&mut self, p: ComputePassDesc) {
            self.calls.push(format!("compute {}", p.name));
            self.compute.push(p);
        }
        fn submit_raster(&mut self, p: RasterPassDesc) {
            self.calls.push(format!("raster {}", p.name));
            self.raster.push(p);
        }
    }

    impl VermeerHost for Rec {
        fn debug_control(&self) -> Option<VermeerDebugControl> {
            self.control
        }
        fn alt_background(&self) -> bool {
            self.alt
        }
        fn library_texture(&mut self, h: u64) -> TexRef {
            self.calls.push(format!("library {h:#x}"));
            TexRef(32)
        }
        fn blur_pyramid(&mut self, input: TexRef) -> TexRef {
            self.calls.push(format!("blur_pyramid {}", input.0));
            TexRef(33)
        }
        fn send_startup_reveal_finished(&mut self) -> Option<()> {
            if self.no_events {
                return None;
            }
            self.events += 1;
            self.calls.push("event StartupRevealFinished".into());
            Some(())
        }
    }

    fn host() -> Rec {
        Rec {
            control: Some(VermeerDebugControl::default()),
            delta: 0.016,
            extent: [1280, 720],
            reuse_target: true,
            reuse_strokes: true,
            ..Default::default()
        }
    }

    fn params(photomode: bool) -> VermeerPassParams {
        VermeerPassParams {
            target: TexRef(1),
            photomode,
            skip_generate: false,
        }
    }

    fn active(frame: u64) -> VermeerState {
        VermeerState {
            frame_count: frame,
            phase: VermeerPhase::Pending,
            ..Default::default()
        }
    }

    #[test]
    fn state_machine_activate_deactivate_flags() {
        let mut s = VermeerState {
            background: Some(ManagedTexture(TexRef(9))),
            progress: 0.7,
            timer: 1.0,
            frame_count: 9,
            phase: VermeerPhase::Ready,
            ..Default::default()
        };
        assert!(s.is_active());
        s.activate(true, false);
        assert_eq!(
            (
                s.frame_count,
                s.progress,
                s.timer,
                s.phase,
                s.hold,
                s.freeze_camera
            ),
            (0, 0.0, 0.0, VermeerPhase::Pending, true, false)
        );
        assert!(s.background.is_none());
        assert!(!s.should_freeze_camera());
        s.activate(false, true);
        assert!(s.should_freeze_camera());
        s.background = Some(ManagedTexture(TexRef(1)));
        s.deactivate();
        assert_eq!(s.phase, VermeerPhase::Disabled);
        assert!(s.background.is_none());
        assert!(!s.is_active());
        assert!(!s.should_freeze_camera());
        // 已禁用时 deactivate 幂等
        s.deactivate();
        assert_eq!(s.phase, VermeerPhase::Disabled);
    }

    #[test]
    fn into_disable_hold_clears_hold_and_keeps_everything_else() {
        let s = VermeerState {
            frame_count: 4,
            progress: 0.4,
            hold: true,
            phase: VermeerPhase::Ready,
            target_extent: [3, 4],
            ..Default::default()
        };
        let t = s.into_disable_hold();
        assert!(!t.hold);
        assert_eq!(
            (t.frame_count, t.progress, t.phase, t.target_extent),
            (4, 0.4, VermeerPhase::Ready, [3, 4])
        );
    }

    #[test]
    #[should_panic(expected = "must be activated first")]
    fn into_disable_hold_panics_when_disabled() {
        VermeerState {
            phase: VermeerPhase::Disabled,
            ..Default::default()
        }
        .into_disable_hold();
    }

    #[derive(Default)]
    struct App(Vec<String>);
    impl VermeerApp for App {
        fn insert_vermeer_state(&mut self, s: VermeerState) {
            self.0.push(format!(
                "state {:?} hold={} freeze={}",
                s.phase, s.hold, s.freeze_camera
            ));
        }
        fn insert_vermeer_debug_control(&mut self, c: VermeerDebugControl) {
            self.0.push(format!("control {c:?}"));
        }
        fn add_pre_startup_system(&mut self, n: &'static str) {
            self.0.push(format!("pre_startup {n}"));
        }
        fn add_on_exit_app_loading_system(&mut self, v: u8, n: &'static str) {
            self.0.push(format!("on_exit {v} {n}"));
        }
    }

    #[test]
    fn plugin_build_initial_phase_follows_enabled_flag() {
        let mut a = App::default();
        VermeerPlugin {
            enabled: true,
            hold: true,
        }
        .build(&mut a);
        assert_eq!(
            a.0,
            [
                "state Pending hold=true freeze=true",
                "control VermeerDebugControl { progress: 0.0, restart: false, freeze: false }",
                "pre_startup startup_vermeer",
                "on_exit 1 deactivate"
            ]
        );
        let mut b = App::default();
        VermeerPlugin {
            enabled: false,
            hold: false,
        }
        .build(&mut b);
        assert_eq!(b.0[0], "state Disabled hold=false freeze=true");
    }

    #[test]
    fn startup_loads_two_shaders() {
        #[derive(Default)]
        struct L(Vec<(&'static str, &'static str)>);
        impl ShaderLoader for L {
            fn load_shader_into_library(&mut self, p: &'static str, n: &'static str) {
                self.0.push((p, n));
            }
        }
        let mut l = L::default();
        startup_vermeer(&mut l);
        assert_eq!(
            l.0[0],
            (
                "/vermeer/raster_strokes.raster.hlsl",
                "vermeer_raster_strokes"
            )
        );
        assert_eq!(l.0[1], ("/vermeer/combine.raster.hlsl", "vermeer_combine"));
        assert_eq!(
            (PATH_RASTER_STROKES.len(), SHADER_RASTER_STROKES.len()),
            (0x23, 0x16)
        );
        assert_eq!((PATH_COMBINE.len(), SHADER_COMBINE.len()), (0x1c, 0xf));
    }

    #[test]
    fn generate_strokes_pipeline_defines_photomode_permutation() {
        let d = GenerateStrokes { photomode: 1 }.pipeline_desc();
        assert_eq!(d.path, "/vermeer/generate_strokes.cs.hlsl");
        assert_eq!(d.path.len(), 0x21);
        assert_eq!(d.entry, "cs_main");
        assert_eq!(
            d.defines,
            vec![("PERM_photomode".to_string(), "1".to_string())]
        );
        assert_eq!(GENERATE_STROKES_URI.len(), 0x29);
        assert_eq!(PERM_PHOTOMODE.len(), 0xe);
        let a = GenerateStrokes { photomode: 0 };
        let b = GenerateStrokes { photomode: 1 };
        assert_ne!(a.source_hash(), b.source_hash());
        assert_eq!(b.as_compute_pipeline().uri.entry, Some("cs_main"));
        assert_eq!(b.as_compute_pipeline().defs, ShaderDefValue(1));
    }

    #[test]
    fn stroke_count_toggle() {
        assert_eq!(max_stroke_count(false), 0x8000);
        assert_eq!(max_stroke_count(true), 0xa00);
    }

    #[test]
    fn disabled_without_restart_clears_prev_active_and_returns_false() {
        let mut h = host();
        let mut l = VermeerPassLocals { prev_active: true };
        let mut s = VermeerState {
            phase: VermeerPhase::Disabled,
            ..Default::default()
        };
        assert!(!vermeer_pass(&mut h, &mut l, &mut s, params(false)));
        assert!(!l.prev_active);
        assert!(h.calls.is_empty());
    }

    #[test]
    fn disabled_with_restart_resets_state_but_keeps_prev_active() {
        let mut h = host();
        h.control = Some(VermeerDebugControl {
            restart: true,
            ..Default::default()
        });
        let mut l = VermeerPassLocals { prev_active: true };
        let mut s = VermeerState {
            phase: VermeerPhase::Disabled,
            frame_count: 50,
            progress: 1.0,
            timer: 3.0,
            hold: true,
            ..Default::default()
        };
        assert!(!vermeer_pass(&mut h, &mut l, &mut s, params(false)));
        assert!(l.prev_active);
        assert_eq!(
            (
                s.frame_count,
                s.progress,
                s.timer,
                s.phase,
                s.hold,
                s.freeze_camera
            ),
            (0, 0.0, 0.0, VermeerPhase::Pending, false, false)
        );
        assert!(h.calls.is_empty());
    }

    #[test]
    fn active_frame_runs_acquire_then_chain_in_machine_order() {
        let mut h = host();
        let mut l = VermeerPassLocals::default();
        let mut s = active(5);
        assert!(vermeer_pass(&mut h, &mut l, &mut s, params(false)));
        assert!(l.prev_active);
        assert_eq!(
            h.calls,
            [
                "temporal_tex vermeer_target [1280, 720] 0x25 true true",
                "log Trace country_core::systems::render_loop::vermeer Acquiring Vermeer target",
                "compute Vermeer acquire target",
                "blur_pyramid 30", // source = 刚采集的 vermeer_target（id 30），不查库
                "raw vermeer_output [1280, 720] 0x25 false false",
                "temporal_buf vermeer_strokes 524288 32 true",
                "compute Vermeer generate strokes",
                "raster Vermeer raster strokes",
                "raster Vermeer debug target",
            ]
        );
        // 阶段 / extent / 计数
        assert_eq!(
            (s.phase, s.target_extent, s.frame_count),
            (VermeerPhase::Ready, [1280, 720], 6)
        );
        // acquire：绑定与线程数
        let acq = &h.compute[0];
        assert_eq!(acq.dispatch_threads, [1280, 720, 1]);
        assert_eq!(
            acq.ops[0],
            ComputeOp::BindTexture {
                key: neg(0x266871abe31656a2),
                access: 0x19,
                tex: TexRef(1),
                write: false
            }
        );
        assert_eq!(
            acq.ops[1],
            ComputeOp::BindTexture {
                key: neg(0x12d6329bc24d9912),
                access: 0x32,
                tex: TexRef(30),
                write: true
            }
        );
    }

    #[test]
    fn progress_rules_warmup_hold_freeze_and_dt_clamp() {
        // 前两帧不推进
        let mut h = host();
        let mut l = VermeerPassLocals::default();
        let mut s = active(1);
        vermeer_pass(&mut h, &mut l, &mut s, params(false));
        assert_eq!(s.progress, 0.0);
        // 第 2 帧开始推进 dt/3
        let mut s = active(2);
        vermeer_pass(&mut h, &mut l, &mut s, params(false));
        assert_eq!(s.progress, 0.016 / 3.0);
        assert_eq!(s.timer, 0.016);
        // hold 不推进
        let mut s = VermeerState {
            hold: true,
            ..active(10)
        };
        vermeer_pass(&mut h, &mut l, &mut s, params(false));
        assert_eq!(s.progress, 0.0);
        // dt 被夹到 1/30
        h.delta = 1.0;
        let mut s = active(10);
        vermeer_pass(&mut h, &mut l, &mut s, params(false));
        assert_eq!(s.progress, MAX_DT / 3.0);
        assert_eq!(s.timer, MAX_DT);
        // 冻结：进度取控制资源，且不会完成
        h.control = Some(VermeerDebugControl {
            progress: 2.0,
            restart: false,
            freeze: true,
        });
        let mut s = active(10);
        vermeer_pass(&mut h, &mut l, &mut s, params(false));
        assert_eq!(s.progress, 2.0);
        assert_eq!(s.phase, VermeerPhase::Ready);
        assert_eq!(h.events, 0);
    }

    #[test]
    fn completion_releases_background_sends_event_and_logs_info() {
        let mut h = host();
        let mut l = VermeerPassLocals::default();
        let mut s = VermeerState {
            background: Some(ManagedTexture(TexRef(7))),
            progress: 0.999,
            ..active(10)
        };
        h.delta = 0.03;
        assert!(vermeer_pass(&mut h, &mut l, &mut s, params(false)));
        assert_eq!(s.phase, VermeerPhase::Disabled);
        assert!(s.background.is_none());
        assert_eq!(h.events, 1);
        let n = h.calls.len();
        assert_eq!(h.calls[n - 2], "event StartupRevealFinished");
        assert_eq!(
            h.calls[n - 1],
            "log Info country_core::systems::render_loop::vermeer Game reveal complete."
        );
        // 下一帧：Disabled -> 返回 false，不重复发布 Finished。
        assert!(!vermeer_pass(&mut h, &mut l, &mut s, params(false)));
        assert_eq!(h.events, 1);
    }

    #[test]
    fn hold_blocks_completion() {
        let mut h = host();
        let mut l = VermeerPassLocals::default();
        let mut s = VermeerState {
            hold: true,
            progress: 5.0,
            ..active(10)
        };
        vermeer_pass(&mut h, &mut l, &mut s, params(false));
        assert_eq!(s.phase, VermeerPhase::Ready);
        assert_eq!(h.events, 0);
    }

    #[test]
    #[should_panic(expected = "called `Option::unwrap()` on a `None` value")]
    fn missing_reveal_finished_event_resource_panics() {
        let mut h = host();
        h.no_events = true;
        let mut l = VermeerPassLocals::default();
        let mut s = VermeerState {
            progress: 2.0,
            ..active(10)
        };
        vermeer_pass(&mut h, &mut l, &mut s, params(false));
    }

    #[test]
    fn prev_active_and_reused_target_feed_the_reuse_uniform() {
        let mut h = host();
        let mut l = VermeerPassLocals { prev_active: true };
        let mut s = active(10);
        vermeer_pass(&mut h, &mut l, &mut s, params(false));
        let genp = h.compute.iter().find(|c| c.name == PASS_GENERATE).unwrap();
        assert_eq!(
            genp.ops[7],
            ComputeOp::WriteUniform {
                key: neg(0x565ef6489f1126fe),
                value: Uniform::Bool(true)
            }
        );
        // 第一次（prev_active=false）→ false
        let mut h = host();
        let mut l = VermeerPassLocals::default();
        let mut s = active(10);
        vermeer_pass(&mut h, &mut l, &mut s, params(false));
        let genp = h.compute.iter().find(|c| c.name == PASS_GENERATE).unwrap();
        assert_eq!(
            genp.ops[7],
            ComputeOp::WriteUniform {
                key: neg(0x565ef6489f1126fe),
                value: Uniform::Bool(false)
            }
        );
    }

    fn render(p: VermeerRenderParams) -> Rec {
        let mut h = host();
        vermeer_render_from_target(&mut h, p);
        h
    }

    fn rp() -> VermeerRenderParams {
        VermeerRenderParams {
            background: None,
            source: None,
            target: TexRef(1),
            timer: 1.5,
            progress: 0.25,
            photomode: false,
            skip_generate: false,
            reuse: true,
        }
    }

    #[test]
    fn render_without_source_uses_library_name_selected_by_static_flag() {
        let h = render(rp());
        assert_eq!(
            h.calls[0],
            format!("library {:#x}", BACKGROUND_NAME_DEFAULT)
        );
        let mut h2 = host();
        h2.alt = true;
        vermeer_render_from_target(&mut h2, rp());
        assert_eq!(h2.calls[0], format!("library {:#x}", BACKGROUND_NAME_ALT));
        // source 提供时不查库
        let h3 = render(VermeerRenderParams {
            source: Some(TexRef(5)),
            ..rp()
        });
        assert_eq!(h3.calls[0], "blur_pyramid 5");
    }

    #[test]
    fn generate_ops_and_uniform_values() {
        let h = render(VermeerRenderParams {
            photomode: true,
            ..rp()
        });
        let genp = &h.compute[0];
        assert_eq!(genp.source, ComputeSource::GenerateStrokes { photomode: 1 });
        assert_eq!(genp.dispatch_threads, [0xa00, 1, 1]);
        // 无 background → 两次都绑金字塔（id 33）
        assert_eq!(
            genp.ops[0],
            ComputeOp::BindTexture {
                key: 0x7f41ce7531a06fe3,
                access: 0x19,
                tex: TexRef(33),
                write: false
            }
        );
        assert_eq!(
            genp.ops[1],
            ComputeOp::BindTexture {
                key: neg(0x12d6329bc24d9912),
                access: 0x19,
                tex: TexRef(33),
                write: false
            }
        );
        assert_eq!(
            genp.ops[2],
            ComputeOp::WriteRgStorageBuffer {
                key: neg(0x445f465f199124c8),
                buf: BufRef(40)
            }
        );
        assert_eq!(
            genp.ops[3],
            ComputeOp::WriteUniform {
                key: neg(0x26ac9f8bb78239bb),
                value: Uniform::F32(1.5)
            }
        );
        assert_eq!(
            genp.ops[4],
            ComputeOp::WriteUniform {
                key: 0x5dd2cf99e4b48447,
                value: Uniform::I32(0xa00)
            }
        );
        assert_eq!(
            genp.ops[5],
            ComputeOp::WriteUniform {
                key: neg(0x2e6262d18a0e4303),
                value: Uniform::F32(0.25)
            }
        );
        assert_eq!(
            genp.ops[6],
            ComputeOp::WriteUniform {
                key: 0x15f142fdf7f6902c,
                value: Uniform::Bool(true)
            }
        );
        // 有 background → 第一个绑它
        let h = render(VermeerRenderParams {
            background: Some(TexRef(77)),
            ..rp()
        });
        assert_eq!(
            h.compute[0].ops[0],
            ComputeOp::BindTexture {
                key: 0x7f41ce7531a06fe3,
                access: 0x19,
                tex: TexRef(77),
                write: false
            }
        );
        // strokes buffer = count << 4
        assert!(
            h.calls
                .iter()
                .any(|c| c == "temporal_buf vermeer_strokes 524288 32 true")
        );
    }

    #[test]
    fn skip_generate_omits_the_compute_pass_only() {
        let h = render(VermeerRenderParams {
            skip_generate: true,
            ..rp()
        });
        assert!(h.compute.is_empty());
        assert_eq!(h.raster.len(), 2);
    }

    #[test]
    fn raster_strokes_and_combine_descriptors() {
        let h = render(rp());
        let rs = &h.raster[0];
        assert_eq!(rs.name, "Vermeer raster strokes");
        assert_eq!(rs.color, RasterColor::View(TexRef(31)));
        assert_eq!(rs.vertex_count, 0x8000 * 6);
        assert!(rs.raster_state_edit);
        assert_eq!(
            rs.ops[0],
            RasterOp::OptionalUniform {
                key: 0x531bb9318eea4971,
                value: [1280, 720]
            }
        );
        assert_eq!(
            rs.ops[4],
            RasterOp::BindTexture {
                key: 0x7337daaff5b5b2d5,
                access: 0x1d,
                tex: TexBinding::FromDisk(0x76ef6f6cc99f108b)
            }
        );
        assert_eq!(
            rs.ops[5],
            RasterOp::ReadRgStorageBuffer {
                key: neg(0x445f465f199124c8),
                buf: BufRef(40)
            }
        );
        let c = &h.raster[1];
        assert_eq!(c.name, "Vermeer debug target");
        assert_eq!(c.shader_name, "vermeer_combine");
        assert_eq!(c.color, RasterColor::Texture(TexRef(1)));
        assert_eq!(c.vertex_count, 3);
        assert_eq!(
            c.ops[4],
            RasterOp::BindTexture {
                key: neg(0x12d6329bc24d9912),
                access: 0x1d,
                tex: TexBinding::Rg(TexRef(33))
            }
        );
        assert_eq!(
            c.ops[5],
            RasterOp::BindTexture {
                key: neg(0x77de65c519676d4e),
                access: 0x1d,
                tex: TexBinding::Rg(TexRef(31))
            }
        );
        assert_eq!(c.ops.len(), 6);
    }
}
