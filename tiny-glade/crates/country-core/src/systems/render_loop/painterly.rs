//! `painterly.rs` — painterly stylized stroke-reveal chain (progressive strokes).
//!
//! 全部来自机器码（O = 观测，I = 推断，U = 未知）。结构：业务逻辑写成对 [`PassHost`] 缝（seam）泛型的函数，
//! 缝里的每个方法 = 原函数里的一个外部调用点（rhapsody RG / ComputePass / DrawBuilder / 资源库 / log），
//! pass 的内容用描述符（[`ComputePassDesc`] / [`RasterPassDesc`]）按**调用顺序**逐项记录，宿主实现者
//! （集成层）把它翻译成真实 `ComputePass::new` / `RasterPass::new` + `DrawBuilder` 调用序列。
//! `vermeer.rs` 复用本文件里的描述符与 [`PassHost`]。
//!
//! 过程 → Rust：
//! - 0x1408a63d0 `PainterlyState::has_target`      → [`PainterlyState::has_target`]（O：`byte[+8] & 1`，即阶段 == Ready）
//! - 0x1408a63e0 `PainterlyPlugin::build`           → [`PainterlyPlugin::build`]（O：`insert_resource(state=2)` + PreStartup 系统）
//! - 0x1408a7fe0 `startup_painterly`                → [`startup_painterly`]（O：`load_shader_into_library` ×2）
//! - 0x1408a6410 `painterly_pass`（7118 B）          → [`painterly_pass`]
//!
//! `PainterlyState` 是 `enum2$`（O：判别在 `+8`，`Default` = 2 = 未启用）。变体 0/1 带载荷
//! `{progress: f64 @0, extent: [u32;2] @0xc, timer: f32 @0x14}`；这里用 struct + 阶段字段表达（载荷在 2 阶段不被读取）。

use std::fmt;

// ---------------------------------------------------------------------------------------------
// 机器码常量（O）
// ---------------------------------------------------------------------------------------------

/// `-x` 的 64 位补码（反汇编里的 `movabs r8, -0x…`）。
pub const fn neg(x: u64) -> u64 {
    0u64.wrapping_sub(x)
}

/// 笔触总数（O：`1024.0` 封顶、`BufferDesc::new(0x4000, 0x20, 1)` = 1024×16 B）。
pub const STROKE_COUNT: f64 = 1024.0;
/// 每秒新增笔触数（O：`f64 1638.4` = 1024 / 0.625 s）。
pub const STROKES_PER_SECOND: f64 = 1638.4;
/// 立即模式下的 timer uniform（O：`f32 100000.0`，0x142925954）。
pub const INSTANT_TIMER: f32 = 100000.0;
/// 颜色目标格式（O：`new_2d(extent, 0x25)`）。
pub const FORMAT_COLOR_TARGET: u32 = 0x25;
/// 深度目标格式（O：`new_2d(extent, 0x4c)`）。
pub const FORMAT_DEPTH_TARGET: u32 = 0x4c;
/// 笔触 SSBO 字节数（O：`mov edx, 0x4000`）。
pub const STROKES_BUFFER_BYTES: u32 = 0x4000;
/// `BufferDesc::new` 的对齐参数 / 标志（O：`r8d=0x20, r9b=1`）。
pub const BUFFER_ALIGN: u32 = 0x20;

/// `AppMode` 资源 TypeId（O：`get_resource_id(-0x2c72519e80486386, 0x22aadf5c5dad1eae)`）。
pub const TYPE_ID_APP_MODE: (u64, u64) = (neg(0x2c72519e80486386), 0x22aadf5c5dad1eae);
/// `PainterlyState` 资源 TypeId（O）。
pub const TYPE_ID_PAINTERLY_STATE: (u64, u64) = (0x1f18d9c9bdba6c60, neg(0x7ab86799b716f499));
/// 带字节 `+0xff` 的资源（O；类型名 U）。
pub const TYPE_ID_FLAG_FF: (u64, u64) = (0x48b321ef779dc301, neg(0x429a898e6b262238));
/// 带 u32 `+0x80` 的资源（O；类型名 U）。
pub const TYPE_ID_COUNT_80: (u64, u64) = (neg(0x702623e63868c436), 0x361ff4c5eb9b45fd);
/// 时间资源（O：`+0x40` f64 增量秒、`+0x58` f32 增量秒；与 vermeer 共用同一 TypeId）。
pub const TYPE_ID_TIME: (u64, u64) = (0x109ca79d5cccc2d1, 0x24eabc1830c9840c);
/// `AssetShaderLibrary` TypeId（O）。
pub const TYPE_ID_SHADER_LIBRARY: (u64, u64) = (neg(0x7a6811f8d84b9e4d), neg(0x74d6813a51ea0ea4));

/// `AppMode` 的 `state == 7 && flag == 1` 时整个系统直接返回（O：`cmp [mode],7 / cmp byte [mode+8],1`）。
pub const APP_MODE_SKIP_STATE: u32 = 7;

// 绑定名哈希（O：`movabs r8, …`；字符串本体 U，哈希来自 shader 反射名表）。
pub const KEY_ACQUIRE_COLOR: u64 = neg(0x266871abe31656a2);
pub const KEY_ACQUIRE_AUX_B: u64 = 0x4861579da049fbfc;
pub const KEY_ACQUIRE_AUX_C: u64 = neg(0x46f3e0806caff4a6);
pub const KEY_TARGET_OUT: u64 = neg(0x77de65c519676d4e);
pub const KEY_DEPTH_OUT: u64 = 0x7021fc7ae6e0ac34;
pub const KEY_CONVOLVE_IN: u64 = 0x11ba5697cc40b3dd;
pub const KEY_GEN_TARGET: u64 = neg(0x12d6329bc24d9912);
pub const KEY_GEN_DEPTH: u64 = neg(0x0c6214c3a1e40d3c);
/// 笔触 SSBO（generate = write，raster = read；O 同一哈希，vermeer 同）。
pub const KEY_STROKES_SSBO: u64 = neg(0x445f465f199124c8);
/// 光栅 pass 的分辨率 `optional_uniform<[u32;2]>`（O，vermeer 同）。
pub const KEY_RESOLUTION: u64 = 0x531bb9318eea4971;
/// 光栅 timer uniform（f32，O）。
pub const KEY_RASTER_TIMER: u64 = neg(0x5479a5b5ba14850f);
/// 磁盘纹理预设键（O：`TextureFromDisk(0x76ef6f6cc99f108b)`，painterly 与 vermeer 共用）与其绑定名哈希。
pub const PRESET_STROKE_TEXTURE: u64 = 0x76ef6f6cc99f108b;
pub const KEY_STROKE_TEXTURE: u64 = 0x7337daaff5b5b2d5;

/// 绑定访问码（O：`r9b`）：compute 读 0x19 / 写 0x32；draw 0x1d。
pub const ACCESS_COMPUTE_READ: u8 = 0x19;
pub const ACCESS_COMPUTE_WRITE: u8 = 0x32;
pub const ACCESS_DRAW_READ: u8 = 0x1d;

/// `/painterly/*` 着色器与 pass 名（O：字符串长度与 `r8d` 一致）。
pub const SHADER_RASTER_STROKES: &str = "painterly_raster_strokes";
pub const SHADER_DEBUG_TARGET: &str = "painterly_debug_target";
pub const PATH_RASTER_STROKES: &str = "/painterly/raster_strokes.raster.hlsl";
pub const PATH_DEBUG_TARGET: &str = "/painterly/debug_target.raster.hlsl";
pub const PATH_ACQUIRE: &str = "/painterly/acquire_target.cs.hlsl";
pub const PATH_CONVOLVE: &str = "/painterly/convolve_target.cs.hlsl";
pub const PATH_GENERATE: &str = "/painterly/generate_strokes.cs.hlsl";
pub const PASS_ACQUIRE: &str = "Painterly acquire target";
pub const PASS_CONVOLVE: &str = "Painterly convolve target";
pub const PASS_GENERATE: &str = "Painterly generate strokes";
pub const PASS_RASTER: &str = "Painterly raster strokes";
/// log 目标 / 文案（O：`country_core::systems::render_loop::painterly`，len 0x2d）。
pub const LOG_TARGET: &str = "country_core::systems::render_loop::painterly";
pub const LOG_ACQUIRING: &str = "Acquiring painterly target";
/// 临时纹理名（O：`painterly_tmp`，len 0xd）。
pub const NAME_TMP: &str = "painterly_tmp";
pub const NAME_TARGET: &str = "painterly_target";
pub const NAME_DEPTH_TARGET: &str = "painterly_depth_target";
pub const NAME_STROKES: &str = "painterly_strokes";

// ---------------------------------------------------------------------------------------------
// 宿主缝：描述符与 PassHost
// ---------------------------------------------------------------------------------------------

/// RG 纹理句柄（资源号）。
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct TexRef(pub u32);
/// RG 缓冲句柄（资源号）。
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct BufRef(pub u32);
/// `AssetShaderLibrary::get_raster_handle_by_name` 返回的句柄（O：`rdx`）。
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct ShaderHandle(pub u64);

/// `TextureDesc::new_2d(extent, format)` + 可选 `.storage()` / `.sampled()`（O）。
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct TextureDesc {
    pub extent: [u32; 2],
    pub format: u32,
    pub storage: bool,
    pub sampled: bool,
}

/// `BufferDesc::new(size, align, flag)`（O：`edx / r8d / r9b`）。
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct BufferDesc {
    pub size: u32,
    pub align: u32,
    pub flag: bool,
}

/// `get_or_create_temporal` 的成功结果（O：`+0x504` 字节，2 = Err 的 niche；其余值 = 已复用）。
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct TemporalTexture {
    pub tex: TexRef,
    pub reused: bool,
}
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct TemporalBuffer {
    pub buf: BufRef,
    pub reused: bool,
}

/// `get_or_create_temporal` 的 `Err`（调用点 `Result::unwrap` → `core::result::unwrap_failed`）。
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct HostError(pub String);
impl fmt::Display for HostError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(&self.0)
    }
}

/// 时间资源视图（O：`+0x40` f64、`+0x58` f32）。
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct TimeView {
    pub delta_secs_f64: f64,
    pub delta_secs: f32,
}

/// `AppMode` 视图（O：`u32 @0`、`u8 @8`）。
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct AppModeView {
    pub state: u32,
    pub flag: u8,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum LogLevel {
    Info = 3,
    Trace = 5,
}

/// `write_uniform<T>` / `optional_uniform<T>` 的值（O：i32 / f32 / bool / [u32;2]）。
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Uniform {
    F32(f32),
    I32(i32),
    Bool(bool),
    U32x2([u32; 2]),
}

/// 纹理绑定的来源。
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TexBinding {
    /// RG 句柄（`bind_texture_to_*<&Handle<Texture>>`）。
    Rg(TexRef),
    /// 磁盘纹理预设（`bind_texture_to_draw_builder<TextureFromDisk>`，键 = 预设哈希）。
    FromDisk(u64),
}

/// ComputePass 描述符里的一步（按调用顺序）。
#[derive(Clone, Debug, PartialEq)]
pub enum ComputeOp {
    /// `bind_texture_to_compute_pass`：`(key, access, tex)`；`write` = 以 `&mut` 绑定（访问 0x32）。
    BindTexture {
        key: u64,
        access: u8,
        tex: TexRef,
        write: bool,
    },
    /// `ComputePass::write_rg_storage_buffer(key, &buf)`。
    WriteRgStorageBuffer { key: u64, buf: BufRef },
    /// `UniformBinder::write_uniform`（块存在才写；是否存在由宿主实现判断）。
    WriteUniform { key: u64, value: Uniform },
}

/// ComputePass 的 shader 来源。
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum ComputeSource {
    /// `ComputePass::new<&str>`：URI（无 `:entry` → 默认 `cs_main`）。
    Uri(&'static str),
    /// `ComputePass::new<GenerateStrokes>`（vermeer）：`photomode` 字节。
    GenerateStrokes { photomode: u8 },
}

/// 一个 compute pass 的完整声明（`ComputePass::new` + 绑定 + `dispatch_*` → `PassBuilder::render`）。
#[derive(Clone, Debug, PartialEq)]
pub struct ComputePassDesc {
    pub name: &'static str,
    pub source: ComputeSource,
    pub ops: Vec<ComputeOp>,
    /// push constants 原始字节（O：generate strokes 的 12 B；其余为空）。
    pub push_constants: Vec<u8>,
    /// 线程总数 `dispatch_1d/2d`（O：渲染记录 `[mode=1, x, y, z]`；缺省维 = 1）。
    pub dispatch_threads: [u32; 3],
}

/// 光栅颜色附件（O：`ColorAttachment` 0x20 B 枚举，tag 0 引用 / tag 1 带视图）。
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RasterColor {
    /// tag 0：纹理引用。
    Texture(TexRef),
    /// tag 1：RG 资源视图（vermeer 笔触 pass 写 `vermeer_output`）。
    View(TexRef),
}

/// DrawBuilder 上的一步（按调用顺序）。
#[derive(Clone, Debug, PartialEq)]
pub enum RasterOp {
    /// `optional_uniform<[u32;2]>(key, extent)`。
    OptionalUniform { key: u64, value: [u32; 2] },
    /// `write_uniform(key, value)`。
    Uniform { key: u64, value: Uniform },
    /// `bind_texture_to_draw_builder(binding, access)`。
    BindTexture {
        key: u64,
        access: u8,
        tex: TexBinding,
    },
    /// `read_rg_storage_buffer(key, &buf)`。
    ReadRgStorageBuffer { key: u64, buf: BufRef },
}

/// 一个 raster pass（`RasterPass::new` + `DrawPacket::new` + `DrawBuilder::new(variant 0)` + 绑定 + `draw_no_vao` + `render`）。
#[derive(Clone, Debug, PartialEq)]
pub struct RasterPassDesc {
    pub name: &'static str,
    /// `get_raster_handle_by_name` 用的 shader 名。
    pub shader_name: &'static str,
    pub shader: ShaderHandle,
    pub color: RasterColor,
    /// 视口 = 附件纹理 `extent_2d`（O：`RasterPass::new_impl` 的 `[rsp+0x30]`）。
    pub viewport: [u32; 2],
    pub ops: Vec<RasterOp>,
    /// 原版在 draw 前改写光栅状态位（O：`byte0 &= 0x7f; byte3 = (byte3 & 0x9f) | 0x40`）。
    pub raster_state_edit: bool,
    /// `draw_no_vao(vertex_count)`。
    pub vertex_count: u32,
}

/// 宿主缝：原函数里的每个外部调用点。
pub trait PassHost {
    /// `Components::get_resource_id` + 取 `AppMode`（缺失 → `Option::unwrap` panic，由调用方 unwrap）。
    fn app_mode(&self) -> Option<AppModeView>;
    /// 时间资源（缺失 → unwrap panic）。
    fn time(&self) -> Option<TimeView>;
    /// `AssetShaderLibrary::get_raster_handle_by_name`（返回 `al & 1` 为 0 → `Option::unwrap` panic）。
    fn raster_shader(&self, name: &str) -> Option<ShaderHandle>;
    /// `TextureDesc::extent_2d`。
    fn texture_extent(&self, tex: TexRef) -> [u32; 2];
    /// `get_or_create_temporal(graph, name, &desc)`。
    fn temporal_texture(
        &mut self,
        name: &str,
        desc: TextureDesc,
    ) -> Result<TemporalTexture, HostError>;
    fn temporal_buffer(
        &mut self,
        name: &str,
        desc: BufferDesc,
    ) -> Result<TemporalBuffer, HostError>;
    /// `RenderGraph::create_raw_resource(name, desc.into())`。
    fn create_raw_texture(&mut self, name: &str, desc: TextureDesc) -> TexRef;
    /// `TextureDesc` 取自已有 RG 纹理（O：tmp 纹理复制 `painterly_target` 的描述）。
    fn texture_desc(&self, tex: TexRef) -> TextureDesc;
    /// `log::__private_api::log`（宿主按 `log::max_level` 过滤；原版 Trace 守卫 `max_level > 4`）。
    fn log(&mut self, level: LogLevel, target: &str, file: &str, message: &str);
    fn submit_compute(&mut self, pass: ComputePassDesc);
    fn submit_raster(&mut self, pass: RasterPassDesc);
}

/// 帧外辅助：`ComputePass` 描述符构造。
pub fn compute_desc(
    name: &'static str,
    source: ComputeSource,
    ops: Vec<ComputeOp>,
    dispatch_threads: [u32; 3],
) -> ComputePassDesc {
    ComputePassDesc {
        name,
        source,
        ops,
        push_constants: Vec::new(),
        dispatch_threads,
    }
}

pub fn unwrap_temporal<T>(r: Result<T, HostError>) -> T {
    // O：`core::result::unwrap_failed("called `Result::unwrap()` on an `Err` value", &err)`
    r.unwrap()
}

/// 源码文件（O：`Location::file()`）。
pub const SOURCE_FILE: &str = "crates/country-core/src/systems/render_loop/painterly.rs";

// ---------------------------------------------------------------------------------------------
// 状态与 Plugin
// ---------------------------------------------------------------------------------------------

/// `PainterlyState` 的阶段（O：`u32 @+8`）。
#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
#[repr(u32)]
pub enum PainterlyPhase {
    /// 0：已启用、目标尚未采集。
    Pending = 0,
    /// 1：目标已采集（`has_target`）。
    Ready = 1,
    /// 2：未启用（`Default`）。
    #[default]
    Disabled = 2,
}

/// Painterly 揭示状态资源。
#[derive(Clone, Copy, Debug, PartialEq, Default, bevy_ecs::resource::Resource)]
pub struct PainterlyState {
    /// `+0x00` f64：已揭示的笔触数（0..=1024）。
    pub progress: f64,
    /// `+0x08` 阶段。
    pub phase: PainterlyPhase,
    /// `+0x0c` `[u32;2]`：目标分辨率（采集后写入；`Pending` 时 U/过期，这里为 0）。
    pub extent: [u32; 2],
    /// `+0x14` f32：累计时间。
    pub timer: f32,
}

impl PainterlyState {
    /// O 0x1408a63d0：`movzx eax, byte [rcx+8]; and al, 1` → 阶段 == Ready。
    pub fn has_target(&self) -> bool {
        (self.phase as u32) & 1 != 0
    }

    /// 重置为 Pending（O：`[+0]=0, [+8]=0, [+0x14]=0`；`+0xc` 不动）。
    fn reset_pending(&mut self) {
        self.progress = 0.0;
        self.phase = PainterlyPhase::Pending;
        self.timer = 0.0;
    }
}

/// `bevy_app::App` 缝：`PainterlyPlugin::build` 里的两个调用。
pub trait PainterlyApp {
    /// `App::insert_resource<PainterlyState>(Disabled)`（O：栈上 `dword [rsp+0x30] = 2`）。
    fn insert_painterly_state(&mut self, state: PainterlyState);
    /// `SubApp::add_systems(PreStartup, startup_painterly)`。
    fn add_pre_startup_system(&mut self, name: &'static str);
}

/// Painterly bevy Plugin（O：零大小）。
#[derive(Clone, Copy, Debug, Default)]
pub struct PainterlyPlugin;

impl PainterlyPlugin {
    /// O 0x1408a63e0（42 B）。
    pub fn build<A: PainterlyApp>(&self, app: &mut A) {
        app.insert_painterly_state(PainterlyState::default());
        app.add_pre_startup_system("startup_painterly");
    }
}

/// `load_shader_into_library(device, path, name, library, watch)` 缝（O 0x140b3fe40）。
pub trait ShaderLoader {
    fn load_shader_into_library(&mut self, path: &'static str, name: &'static str);
}

/// O 0x1408a7fe0：按调用序加载两个光栅 shader。
pub fn startup_painterly<L: ShaderLoader>(loader: &mut L) {
    loader.load_shader_into_library(PATH_RASTER_STROKES, SHADER_RASTER_STROKES);
    loader.load_shader_into_library(PATH_DEBUG_TARGET, SHADER_DEBUG_TARGET);
}

// ---------------------------------------------------------------------------------------------
// painterly_pass
// ---------------------------------------------------------------------------------------------

/// `painterly_pass` 的附加输入（O：`r8 / r9 / [rsp+0x20] / [rsp+0x28]`）。
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PainterlyInputs {
    /// `r8`：场景颜色目标（既是 acquire 的输入，又是 raster 的颜色附件；extent 来源）。
    pub color: TexRef,
    /// `r9`（绑定键 `0x4861579da049fbfc`；语义 U）。
    pub aux_b: TexRef,
    /// 第 5 个参数（绑定键 `-0x46f3e0806caff4a6`；语义 U）。
    pub aux_c: TexRef,
    /// 第 6 个参数（O：`[rbp+0xae8]`；0 → 阶段置 Disabled 并返回）。
    pub enabled: bool,
}

/// 目标分辨率：`(w << 8) / h` × 256（O：`shl eax,8; div ecx`；`h == 0` → `panic_const_div_by_zero`）。
pub fn painterly_extent(color_extent: [u32; 2]) -> [u32; 2] {
    let (w, h) = (color_extent[0], color_extent[1]);
    if h == 0 {
        panic!("attempt to divide by zero");
    }
    [(w << 8) / h, 256]
}

/// `f64 → u64` 饱和转换（O：`cvttsd2si` + 符号修正 + 上下界 `cmov`，等价于 Rust 的 `as u64`；NaN → 0）。
fn floor_u64(x: f64) -> u64 {
    x as u64
}

/// O 0x1408a6410：Painterly 驱动系统（7118 B）。
///
/// 流程（O）：
/// 1. `AppMode.state == 7 && flag == 1` → 返回。
/// 2. `instant` = 资源(0xff).byte != 0 ? true : 资源(0x80).u32 == 0。
/// 3. `!enabled` → 阶段 = Disabled，返回。
/// 4. 阶段 Disabled → 重置 Pending（progress=0、timer=0）。
/// 5. `instant` → 再重置 Pending 并按目标分辨率重算；否则 Ready 时沿用状态里的 extent。
/// 6. `painterly_target`（fmt 0x25，storage+sampled）、`painterly_depth_target`（fmt 0x4c，同）两个 temporal；
///    阶段 != Pending 且两者都已复用 → 跳过采集；否则 `painterly_tmp` 原始资源（复制 target 的描述）+ Trace 日志
///    + acquire/convolve 两个 compute pass，阶段 = Ready、extent = target 纹理 extent。
/// 7. `painterly_strokes` SSBO（0x4000 B）temporal；未复用 → progress = 0。
/// 8. step = instant ? 1024 : delta_f64 × 1638.4；progress' = min(progress+step, 1024)；
///    floor 差 ≠ 0 → generate strokes（push constants `[start, count, timer]`，线程数 count）。
/// 9. 写回 progress；raster pass 画 `6 × floor(progress')` 个顶点；`timer += delta_f32`。
pub fn painterly_pass<H: PassHost>(
    host: &mut H,
    state: &mut PainterlyState,
    io: PainterlyInputs,
    flag_ff: impl FnOnce() -> Option<bool>,
    count_80: impl FnOnce() -> Option<u32>,
) {
    // 1. AppMode
    let mode = host.app_mode().unwrap();
    if mode.state == APP_MODE_SKIP_STATE && mode.flag == 1 {
        return;
    }
    // 2. instant（资源 0xff 先查；为 0 才查 0x80 的资源）
    let instant = if flag_ff().unwrap() {
        true
    } else {
        count_80().unwrap() == 0
    };
    // 3.
    if !io.enabled {
        state.phase = PainterlyPhase::Disabled;
        return;
    }
    // 4.
    if state.phase == PainterlyPhase::Disabled {
        state.reset_pending();
    }
    // 5.
    let extent = if instant {
        state.reset_pending();
        painterly_extent(host.texture_extent(io.color))
    } else if state.phase == PainterlyPhase::Ready {
        state.extent
    } else {
        painterly_extent(host.texture_extent(io.color))
    };
    // 6.
    let target_desc = TextureDesc {
        extent,
        format: FORMAT_COLOR_TARGET,
        storage: true,
        sampled: true,
    };
    let target = unwrap_temporal(host.temporal_texture(NAME_TARGET, target_desc));
    let depth_desc = TextureDesc {
        extent,
        format: FORMAT_DEPTH_TARGET,
        storage: true,
        sampled: true,
    };
    let depth = unwrap_temporal(host.temporal_texture(NAME_DEPTH_TARGET, depth_desc));
    if state.phase == PainterlyPhase::Pending || !(target.reused && depth.reused) {
        acquire_and_convolve(host, state, io, extent, target, depth);
    }
    // 7.
    let strokes_desc = BufferDesc {
        size: STROKES_BUFFER_BYTES,
        align: BUFFER_ALIGN,
        flag: true,
    };
    let strokes = unwrap_temporal(host.temporal_buffer(NAME_STROKES, strokes_desc));
    if !strokes.reused {
        state.progress = 0.0;
    }
    // 8.
    let time = host.time().unwrap();
    let old = state.progress;
    let old_floor = floor_u64(old);
    let step = if instant {
        STROKE_COUNT
    } else {
        time.delta_secs_f64 * STROKES_PER_SECOND
    };
    let new = (old + step).min(STROKE_COUNT);
    let new_floor = floor_u64(new);
    let count = new_floor.wrapping_sub(old_floor);
    if count != 0 {
        let start = old_floor as u32;
        let count32 = count as u32;
        let mut pc = Vec::with_capacity(12);
        pc.extend_from_slice(&start.to_le_bytes());
        pc.extend_from_slice(&count32.to_le_bytes());
        pc.extend_from_slice(&state.timer.to_le_bytes());
        let mut pass = compute_desc(
            PASS_GENERATE,
            ComputeSource::Uri(PATH_GENERATE),
            vec![
                ComputeOp::BindTexture {
                    key: KEY_GEN_TARGET,
                    access: ACCESS_COMPUTE_READ,
                    tex: target.tex,
                    write: false,
                },
                ComputeOp::BindTexture {
                    key: KEY_GEN_DEPTH,
                    access: ACCESS_COMPUTE_READ,
                    tex: depth.tex,
                    write: false,
                },
                ComputeOp::WriteRgStorageBuffer {
                    key: KEY_STROKES_SSBO,
                    buf: strokes.buf,
                },
            ],
            [count32, 1, 1],
        );
        pass.push_constants = pc;
        host.submit_compute(pass);
    }
    // 9.
    state.progress = new;
    let timer_value = if instant { INSTANT_TIMER } else { state.timer };
    let shader = host.raster_shader(SHADER_RASTER_STROKES).unwrap();
    let viewport = host.texture_extent(io.color);
    let ops = vec![
        RasterOp::OptionalUniform {
            key: KEY_RESOLUTION,
            value: viewport,
        },
        RasterOp::Uniform {
            key: KEY_RASTER_TIMER,
            value: Uniform::F32(timer_value),
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
    ];
    host.submit_raster(RasterPassDesc {
        name: PASS_RASTER,
        shader_name: SHADER_RASTER_STROKES,
        shader,
        color: RasterColor::Texture(io.color),
        viewport,
        ops,
        raster_state_edit: true,
        vertex_count: (new_floor as u32).wrapping_mul(6),
    });
    state.timer += time.delta_secs;
}

/// 采集分支（O 0x1408a6849..0x1408a6f4f）。
fn acquire_and_convolve<H: PassHost>(
    host: &mut H,
    state: &mut PainterlyState,
    io: PainterlyInputs,
    extent: [u32; 2],
    target: TemporalTexture,
    depth: TemporalTexture,
) {
    let tmp_desc = host.texture_desc(target.tex);
    let tmp = host.create_raw_texture(NAME_TMP, tmp_desc);
    host.log(LogLevel::Trace, LOG_TARGET, SOURCE_FILE, LOG_ACQUIRING);
    let acquire = compute_desc(
        PASS_ACQUIRE,
        ComputeSource::Uri(PATH_ACQUIRE),
        vec![
            ComputeOp::BindTexture {
                key: KEY_ACQUIRE_COLOR,
                access: ACCESS_COMPUTE_READ,
                tex: io.color,
                write: false,
            },
            ComputeOp::BindTexture {
                key: KEY_ACQUIRE_AUX_B,
                access: ACCESS_COMPUTE_READ,
                tex: io.aux_b,
                write: false,
            },
            ComputeOp::BindTexture {
                key: KEY_ACQUIRE_AUX_C,
                access: ACCESS_COMPUTE_READ,
                tex: io.aux_c,
                write: false,
            },
            ComputeOp::BindTexture {
                key: KEY_TARGET_OUT,
                access: ACCESS_COMPUTE_WRITE,
                tex: tmp,
                write: true,
            },
            ComputeOp::BindTexture {
                key: KEY_DEPTH_OUT,
                access: ACCESS_COMPUTE_WRITE,
                tex: depth.tex,
                write: true,
            },
        ],
        [extent[0], extent[1], 1],
    );
    host.submit_compute(acquire);
    let convolve = compute_desc(
        PASS_CONVOLVE,
        ComputeSource::Uri(PATH_CONVOLVE),
        vec![
            ComputeOp::BindTexture {
                key: KEY_CONVOLVE_IN,
                access: ACCESS_COMPUTE_READ,
                tex: tmp,
                write: false,
            },
            ComputeOp::BindTexture {
                key: KEY_TARGET_OUT,
                access: ACCESS_COMPUTE_WRITE,
                tex: target.tex,
                write: true,
            },
        ],
        [extent[0], extent[1], 1],
    );
    host.submit_compute(convolve);
    state.phase = PainterlyPhase::Ready;
    state.extent = host.texture_extent(target.tex);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[derive(Default)]
    pub struct Rec {
        pub calls: Vec<String>,
        pub mode: Option<AppModeView>,
        pub time: Option<TimeView>,
        pub color_extent: [u32; 2],
        pub target_extent: [u32; 2],
        pub reuse_target: bool,
        pub reuse_depth: bool,
        pub reuse_strokes: bool,
        pub compute: Vec<ComputePassDesc>,
        pub raster: Vec<RasterPassDesc>,
        pub temporal_err: bool,
        pub shaders: bool,
    }

    impl PassHost for Rec {
        fn app_mode(&self) -> Option<AppModeView> {
            self.mode
        }
        fn time(&self) -> Option<TimeView> {
            self.time
        }
        fn raster_shader(&self, name: &str) -> Option<ShaderHandle> {
            if self.shaders {
                Some(ShaderHandle(name.len() as u64))
            } else {
                None
            }
        }
        fn texture_extent(&self, tex: TexRef) -> [u32; 2] {
            if tex.0 == 1 {
                self.color_extent
            } else {
                self.target_extent
            }
        }
        fn temporal_texture(
            &mut self,
            name: &str,
            desc: TextureDesc,
        ) -> Result<TemporalTexture, HostError> {
            self.calls.push(format!(
                "temporal_tex {name} {:?} fmt={:#x} s={} m={}",
                desc.extent, desc.format, desc.storage, desc.sampled
            ));
            if self.temporal_err {
                return Err(HostError("boom".into()));
            }
            let (id, reused) = match name {
                NAME_TARGET => (10, self.reuse_target),
                _ => (11, self.reuse_depth),
            };
            Ok(TemporalTexture {
                tex: TexRef(id),
                reused,
            })
        }
        fn temporal_buffer(
            &mut self,
            name: &str,
            desc: BufferDesc,
        ) -> Result<TemporalBuffer, HostError> {
            self.calls.push(format!("temporal_buf {name} {desc:?}"));
            Ok(TemporalBuffer {
                buf: BufRef(20),
                reused: self.reuse_strokes,
            })
        }
        fn create_raw_texture(&mut self, name: &str, desc: TextureDesc) -> TexRef {
            self.calls.push(format!("raw {name} {:?}", desc.extent));
            TexRef(12)
        }
        fn texture_desc(&self, tex: TexRef) -> TextureDesc {
            TextureDesc {
                extent: [tex.0, tex.0],
                format: FORMAT_COLOR_TARGET,
                storage: true,
                sampled: true,
            }
        }
        fn log(&mut self, level: LogLevel, target: &str, _file: &str, message: &str) {
            self.calls.push(format!("log {level:?} {target} {message}"));
        }
        fn submit_compute(&mut self, pass: ComputePassDesc) {
            self.calls.push(format!("compute {}", pass.name));
            self.compute.push(pass);
        }
        fn submit_raster(&mut self, pass: RasterPassDesc) {
            self.calls.push(format!("raster {}", pass.name));
            self.raster.push(pass);
        }
    }

    fn host() -> Rec {
        Rec {
            mode: Some(AppModeView { state: 3, flag: 0 }),
            time: Some(TimeView {
                delta_secs_f64: 0.25,
                delta_secs: 0.25,
            }),
            color_extent: [1920, 1080],
            target_extent: [455, 256],
            shaders: true,
            ..Default::default()
        }
    }

    fn io(enabled: bool) -> PainterlyInputs {
        PainterlyInputs {
            color: TexRef(1),
            aux_b: TexRef(2),
            aux_c: TexRef(3),
            enabled,
        }
    }

    fn run(h: &mut Rec, st: &mut PainterlyState, enabled: bool, ff: bool, c80: u32) {
        painterly_pass(h, st, io(enabled), || Some(ff), || Some(c80));
    }

    #[test]
    fn has_target_is_phase_bit0() {
        let mut s = PainterlyState::default();
        assert!(!s.has_target());
        s.phase = PainterlyPhase::Pending;
        assert!(!s.has_target());
        s.phase = PainterlyPhase::Ready;
        assert!(s.has_target());
        assert_eq!(PainterlyPhase::Disabled as u32, 2);
    }

    #[test]
    fn plugin_inserts_disabled_state_then_registers_startup() {
        #[derive(Default)]
        struct App(Vec<String>);
        impl PainterlyApp for App {
            fn insert_painterly_state(&mut self, s: PainterlyState) {
                self.0.push(format!("insert {:?}", s.phase));
            }
            fn add_pre_startup_system(&mut self, n: &'static str) {
                self.0.push(format!("pre_startup {n}"));
            }
        }
        let mut a = App::default();
        PainterlyPlugin.build(&mut a);
        assert_eq!(a.0, ["insert Disabled", "pre_startup startup_painterly"]);
    }

    #[test]
    fn startup_loads_two_raster_shaders_in_order() {
        #[derive(Default)]
        struct L(Vec<(&'static str, &'static str)>);
        impl ShaderLoader for L {
            fn load_shader_into_library(&mut self, p: &'static str, n: &'static str) {
                self.0.push((p, n));
            }
        }
        let mut l = L::default();
        startup_painterly(&mut l);
        assert_eq!(
            l.0,
            [
                (
                    "/painterly/raster_strokes.raster.hlsl",
                    "painterly_raster_strokes"
                ),
                (
                    "/painterly/debug_target.raster.hlsl",
                    "painterly_debug_target"
                )
            ]
        );
        // 长度与反汇编 r8d / [rsp+0x20] 一致
        assert_eq!(
            (PATH_RASTER_STROKES.len(), SHADER_RASTER_STROKES.len()),
            (0x25, 0x18)
        );
        assert_eq!(
            (PATH_DEBUG_TARGET.len(), SHADER_DEBUG_TARGET.len()),
            (0x23, 0x16)
        );
    }

    #[test]
    fn app_mode_7_with_flag_returns_before_anything() {
        let mut h = host();
        h.mode = Some(AppModeView { state: 7, flag: 1 });
        let mut st = PainterlyState::default();
        run(&mut h, &mut st, true, false, 0);
        assert!(h.calls.is_empty());
        assert_eq!(st.phase, PainterlyPhase::Disabled);
        // flag != 1 不跳过
        h.mode = Some(AppModeView { state: 7, flag: 0 });
        run(&mut h, &mut st, false, false, 0);
        assert_eq!(st.phase, PainterlyPhase::Disabled);
    }

    #[test]
    fn disabled_flag_sets_phase_2_and_stops() {
        let mut h = host();
        let mut st = PainterlyState {
            phase: PainterlyPhase::Ready,
            progress: 5.0,
            ..Default::default()
        };
        run(&mut h, &mut st, false, false, 1);
        assert_eq!(st.phase, PainterlyPhase::Disabled);
        assert!(h.calls.is_empty());
        assert_eq!(st.progress, 5.0);
    }

    #[test]
    fn extent_formula_and_div_by_zero() {
        assert_eq!(painterly_extent([1920, 1080]), [455, 256]);
        assert_eq!(painterly_extent([100, 100]), [256, 256]);
    }

    #[test]
    #[should_panic(expected = "attempt to divide by zero")]
    fn zero_height_target_panics() {
        painterly_extent([10, 0]);
    }

    #[test]
    fn first_frame_acquires_then_generates_and_rasters_in_machine_order() {
        let mut h = host();
        let mut st = PainterlyState::default(); // Disabled -> Pending
        run(&mut h, &mut st, true, false, 7);
        assert_eq!(
            h.calls,
            [
                "temporal_tex painterly_target [455, 256] fmt=0x25 s=true m=true",
                "temporal_tex painterly_depth_target [455, 256] fmt=0x4c s=true m=true",
                "raw painterly_tmp [10, 10]",
                "log Trace country_core::systems::render_loop::painterly Acquiring painterly target",
                "compute Painterly acquire target",
                "compute Painterly convolve target",
                "temporal_buf painterly_strokes BufferDesc { size: 16384, align: 32, flag: true }",
                "compute Painterly generate strokes",
                "raster Painterly raster strokes",
            ]
        );
        assert_eq!(st.phase, PainterlyPhase::Ready);
        assert_eq!(st.extent, [455, 256]); // `extent_2d(painterly_target 描述)`
    }

    #[test]
    fn acquire_binds_and_dispatch_match_asm() {
        let mut h = host();
        let mut st = PainterlyState::default();
        run(&mut h, &mut st, true, false, 7);
        let acq = &h.compute[0];
        assert_eq!(
            acq.source,
            ComputeSource::Uri("/painterly/acquire_target.cs.hlsl")
        );
        assert_eq!(acq.dispatch_threads, [455, 256, 1]);
        assert_eq!(acq.ops.len(), 5);
        assert_eq!(
            acq.ops[3],
            ComputeOp::BindTexture {
                key: neg(0x77de65c519676d4e),
                access: 0x32,
                tex: TexRef(12),
                write: true
            }
        );
        assert_eq!(
            acq.ops[4],
            ComputeOp::BindTexture {
                key: 0x7021fc7ae6e0ac34,
                access: 0x32,
                tex: TexRef(11),
                write: true
            }
        );
        let conv = &h.compute[1];
        assert_eq!(conv.name, "Painterly convolve target");
        assert_eq!(
            conv.ops[0],
            ComputeOp::BindTexture {
                key: 0x11ba5697cc40b3dd,
                access: 0x19,
                tex: TexRef(12),
                write: false
            }
        );
        assert_eq!(
            conv.ops[1],
            ComputeOp::BindTexture {
                key: neg(0x77de65c519676d4e),
                access: 0x32,
                tex: TexRef(10),
                write: true
            }
        );
    }

    #[test]
    fn progress_advances_by_dt_times_1638_4_and_generates_delta_strokes() {
        let mut h = host();
        h.reuse_target = true;
        h.reuse_depth = true;
        h.reuse_strokes = true;
        let mut st = PainterlyState {
            phase: PainterlyPhase::Ready,
            progress: 100.5,
            extent: [455, 256],
            timer: 2.0,
        };
        run(&mut h, &mut st, true, false, 7);
        // 无采集
        assert!(!h.calls.iter().any(|c| c.contains("acquire")));
        let new = 100.5 + 0.25 * 1638.4;
        assert_eq!(st.progress, new);
        let genp = &h.compute[0];
        let count = new as u64 - 100;
        assert_eq!(genp.dispatch_threads, [count as u32, 1, 1]);
        let mut pc = Vec::new();
        pc.extend_from_slice(&100u32.to_le_bytes());
        pc.extend_from_slice(&(count as u32).to_le_bytes());
        pc.extend_from_slice(&2.0f32.to_le_bytes());
        assert_eq!(genp.push_constants, pc);
        assert_eq!(
            genp.ops[2],
            ComputeOp::WriteRgStorageBuffer {
                key: neg(0x445f465f199124c8),
                buf: BufRef(20)
            }
        );
        let r = &h.raster[0];
        assert_eq!(r.vertex_count, new as u32 * 6);
        assert!(r.raster_state_edit);
        assert_eq!(
            r.ops[1],
            RasterOp::Uniform {
                key: neg(0x5479a5b5ba14850f),
                value: Uniform::F32(2.0)
            }
        );
        assert_eq!(
            r.ops[2],
            RasterOp::BindTexture {
                key: 0x7337daaff5b5b2d5,
                access: 0x1d,
                tex: TexBinding::FromDisk(0x76ef6f6cc99f108b)
            }
        );
        assert_eq!(r.viewport, [1920, 1080]);
        assert_eq!(r.color, RasterColor::Texture(TexRef(1)));
        // timer += delta_f32
        assert_eq!(st.timer, 2.25);
    }

    #[test]
    fn progress_is_capped_at_1024_and_no_generate_when_floor_unchanged() {
        let mut h = host();
        h.reuse_target = true;
        h.reuse_depth = true;
        h.reuse_strokes = true;
        let mut st = PainterlyState {
            phase: PainterlyPhase::Ready,
            progress: 1024.0,
            extent: [1, 1],
            timer: 0.0,
        };
        run(&mut h, &mut st, true, false, 7);
        assert_eq!(st.progress, 1024.0);
        assert!(h.compute.is_empty());
        assert_eq!(h.raster[0].vertex_count, 6144);
    }

    #[test]
    fn instant_mode_reveals_everything_with_timer_100000() {
        let mut h = host();
        h.reuse_target = true;
        h.reuse_depth = true;
        h.reuse_strokes = true;
        let mut st = PainterlyState {
            phase: PainterlyPhase::Ready,
            progress: 300.0,
            extent: [1, 1],
            timer: 9.0,
        };
        run(&mut h, &mut st, true, true, 5); // flag_ff set -> instant
        // 重置为 Pending（progress=0,timer=0）后全量生成
        assert_eq!(st.progress, 1024.0);
        assert_eq!(
            h.compute.iter().filter(|c| c.name == PASS_GENERATE).count(),
            1
        );
        let genp = h.compute.iter().find(|c| c.name == PASS_GENERATE).unwrap();
        assert_eq!(genp.dispatch_threads, [1024, 1, 1]);
        assert_eq!(&genp.push_constants[0..4], &0u32.to_le_bytes());
        let r = h.raster.last().unwrap();
        assert_eq!(
            r.ops[1],
            RasterOp::Uniform {
                key: neg(0x5479a5b5ba14850f),
                value: Uniform::F32(100000.0)
            }
        );
        // 重置 -> Pending -> 采集分支必走（Pending）
        assert!(
            h.calls
                .iter()
                .any(|c| c == "compute Painterly acquire target")
        );
    }

    #[test]
    fn count_80_zero_also_means_instant_but_only_when_flag_ff_clear() {
        let mut h = host();
        h.reuse_target = true;
        h.reuse_depth = true;
        h.reuse_strokes = true;
        let mut st = PainterlyState {
            phase: PainterlyPhase::Ready,
            progress: 0.0,
            extent: [1, 1],
            timer: 0.0,
        };
        run(&mut h, &mut st, true, false, 0);
        assert_eq!(st.progress, 1024.0);
        // flag_ff = true 时不应再查 count_80（闭包 FnOnce 不被调用）
        let mut h2 = host();
        let mut st2 = PainterlyState::default();
        painterly_pass(
            &mut h2,
            &mut st2,
            io(true),
            || Some(true),
            || panic!("count_80 must not be queried"),
        );
    }

    #[test]
    fn unreused_strokes_buffer_resets_progress_before_stepping() {
        let mut h = host();
        h.reuse_target = true;
        h.reuse_depth = true;
        h.reuse_strokes = false;
        let mut st = PainterlyState {
            phase: PainterlyPhase::Ready,
            progress: 500.0,
            extent: [1, 1],
            timer: 0.0,
        };
        run(&mut h, &mut st, true, false, 7);
        assert_eq!(st.progress, 0.25 * 1638.4);
        assert_eq!(h.compute[0].push_constants[0..4], 0u32.to_le_bytes());
    }

    #[test]
    fn ready_but_not_reused_target_reacquires() {
        let mut h = host();
        h.reuse_target = true;
        h.reuse_depth = false;
        h.reuse_strokes = true;
        let mut st = PainterlyState {
            phase: PainterlyPhase::Ready,
            progress: 0.0,
            extent: [64, 256],
            timer: 0.0,
        };
        run(&mut h, &mut st, true, false, 7);
        assert!(
            h.calls
                .iter()
                .any(|c| c == "compute Painterly acquire target")
        );
        // 沿用状态里的 extent（Ready 且非 instant）
        assert!(h.calls[0].contains("[64, 256]"));
    }

    #[test]
    #[should_panic(expected = "called `Result::unwrap()` on an `Err` value")]
    fn temporal_failure_unwraps() {
        let mut h = host();
        h.temporal_err = true;
        let mut st = PainterlyState::default();
        run(&mut h, &mut st, true, false, 7);
    }

    #[test]
    #[should_panic(expected = "called `Option::unwrap()` on a `None` value")]
    fn missing_raster_shader_unwraps() {
        let mut h = host();
        h.shaders = false;
        h.reuse_target = true;
        h.reuse_depth = true;
        h.reuse_strokes = true;
        let mut st = PainterlyState {
            phase: PainterlyPhase::Ready,
            ..Default::default()
        };
        run(&mut h, &mut st, true, false, 7);
    }
}
