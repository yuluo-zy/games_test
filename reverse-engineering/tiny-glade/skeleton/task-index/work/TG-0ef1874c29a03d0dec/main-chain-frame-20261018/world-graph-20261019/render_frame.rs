//! `tiny_glade::render_loop::render`（0x1404e8010，16279 B）与 `handle_screenshot_error`（0x1404ebfb0，337 B）。
//!
//! 属 tiny_glade 二进制 crate（`crates/tiny-glade/src/render_loop.rs`）。`render` 是整帧的外层：
//! 取资源 → 建绘制列表 → 编译渲染图（含 OOM 降分辨率重试）→ 执行 → 获取 swapchain 图像 → 展示图 → present →
//! 统计与 GPU 排队修剪。真实实现依赖 rhapsody（RenderGraphState/GraphExecutor/Swapchain 等，离线缺失），
//! 因此按 [`RenderHost`] 缝表达：每个方法对应原函数里的一个调用点（地址注释为 O），分支/常量/日志文案/panic 文案
//! 与数值逻辑（帧时间环、排队修剪时长）在本文件里**真实实现并有测试**。
//!
//! 标注：O = 机器码观测，I = 推断，U = 未知。

// Preserve the original instruction-level expression order/NaN policy, API call shapes, and evidence/test transcription. See README.md.
#![allow(clippy::collapsible_if, clippy::doc_lazy_continuation)]
use core::fmt::Debug;
use core::time::Duration;

/// puffin 作用域名（O：0x1429eb8f4，`"tiny_glade::render_loop::render::f"` 去掉 `::f` 的 31 字节）。
pub const SCOPE_NAME: &str = "tiny_glade::render_loop::render";
/// 日志 target（O：0x1429ebb70，0x17 字节）。
pub const LOG_TARGET: &str = "tiny_glade::render_loop";

/// 渲染图编译 OOM 降分辨率时的单步缩减（O：`addss xmm0, [0x1429eb6f8]` = -0.1f32，0x1404e8ac9）。
pub const RESOLUTION_SCALE_STEP: f32 = -0.1;
/// 兜底编译用的最小分辨率覆盖（O：`movabs rax, 0x2000000020`，0x1404e9376 → `[32, 32]`；
/// 随后被 `render_world` 的 `world_render_dims` 断言要求非零）。
pub const MIN_RESOLUTION_EXTENT: [u32; 2] = [32, 32];
/// 降级后写入"降分辨率状态"资源首个 u64 的值（O：`mov qword ptr [rcx], 2`，0x1404e90e3）。
pub const REDUCED_STATE_VALUE: u64 = 2;

/// 日志/panic 文案（O，.rdata 0x1429ebb00..0x1429ebed0）。
pub mod msgs {
    pub const FORCING_WORLD_RENDERING: &str = "Forcing world rendering because ";
    pub const GAME_CONFIG: &str = "GameConfig";
    pub const LOADING_NEEDS_SWAPCHAIN: &str = "The loading screen must have a swapchain";
    pub const LOADING_GRAPH: &str = "Loading screen render graph";
    pub const OOM_PREFIX: &str =
        "Out of memory while allocating render graph resources; reducing resolution scale ";
    pub const OOM_ARROW: &str = " -> ";
    pub const COMPILING_REDUCED: &str = "Compiling a render graph for the reduced resolution scale";
    pub const FALLBACK_FAILED: &str = "Minimum-resolution fallback render graph compile failed";
    pub const REDUCED_COMPILED: &str = "Reduced-resolution scale render graph compiled";
    pub const PRESENTATION_FAILED: &str = "Failed to compile presentation render graph: ";
    pub const OUT_OF_VRAM: &str = "Out of video memory. Swapchain info: ";
    pub const SCREENSHOT_ERROR: &str = "Error while capturing a screenshot: ";
    pub const DURATION_PANIC: &str =
        "cannot convert float seconds to Duration: value is either too big or NaN";
}

/// 日志等级（O：`log::Record` 的 level 字段：`1` = Error，`3` = Info）。
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Level {
    Error = 1,
    Info = 3,
}

/// `render` 的模式字节（第 8 个参数，O：`cmp dil, 2; sete r14b` / `test dil, dil; je loading`）。
/// 0 → 加载画面图；非 0 → 世界图；2 且 puffin 开启 → 本帧取 GPU profiler 输出缓冲（I）。
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RenderMode {
    LoadingScreen,
    World,
    /// 值 2（I：世界图 + GPU 剖析）。
    WorldGpuProfiled,
}

impl RenderMode {
    pub fn from_byte(b: u8) -> Self {
        match b {
            0 => Self::LoadingScreen,
            2 => Self::WorldGpuProfiled,
            _ => Self::World,
        }
    }
}

/// swapchain 格式 + 范围（O：`Swapchain::format` / `Swapchain::extent`，原函数缓存成 `Option<(format, extent)>`）。
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct SwapchainInfo {
    pub format: u32,
    pub extent: [u32; 2],
}

/// `validate_swapchain_and_acquire_image` 的结果（O：`cmp byte [rbp+0xa1e], 7`：7 = 本帧没有可用图像）。
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Acquire {
    /// 状态字节 == 7：跳过展示图与 present。
    Unavailable,
    Image,
}

/// `render` 的外部调用面（每个方法 = 一个调用点，顺序按 `render` 内出现顺序）。
pub trait RenderHost {
    type Compiled;
    type CompileError: Debug;

    /// The current backend distinguishes allocation failure from declaration,
    /// capability, WSI and execution errors. Only allocation failure is eligible
    /// for the original reduced-resolution recovery policy.
    fn compile_error_is_out_of_memory(error: &Self::CompileError) -> bool;
    /// Drain the first backend error recorded by a void host callback. The
    /// original error value and its source chain must survive this boundary.
    fn take_render_error(&mut self) -> Option<Self::CompileError>;
    /// Cancel unfinished frame/acquisition ownership after an error. Keep the
    /// original error as the returned cause; report cleanup errors separately.
    fn abort_frame(&mut self);

    // ---- puffin / 时钟 ----
    fn puffin_scopes_on(&mut self) -> bool;
    fn scope_begin(&mut self, name: &str);
    fn scope_end(&mut self);
    /// `Instant::now()`，以"相对任意起点的 Duration"表示。
    fn now(&mut self) -> Duration;

    // ---- 资源（O：Components::get_resource_id + 存储表；缺失原版 `Option::unwrap`/`expect`） ----
    /// TypeId 0x2ed189581b92decf/0x15d3d99396065f38（`Arc<GraphicsDevice>` 的强引用 +1；缺失 → unwrap，行 642）。
    fn device_ready(&mut self) -> bool;
    /// TypeId 0x3d7b56e787d47c40/0x67b0b10a8d43bfc7（`GameConfig`/`VideoSettings`；缺失 → `expect("GameConfig")`，行 650）。
    /// 成功时克隆其中的字符串/Vec（`LastKnownGraphicsDevice`、`GraphicsInstanceLayerAllowlistItem`）。
    fn clone_video_settings(&mut self) -> bool;
    /// `GameConfig.ui.input_lag_reduction`。B 原始指令：0x1404e8666
    /// 从 `GameConfig+0xf0` 读 4 字节，0x1404e870e 存到 `[rbp+0x878]`；
    /// 0x1404ea052 比较 `[rbp+0x87a]`，即 `UiSettings+0xa`。
    fn gpu_pacing_enabled(&mut self) -> bool;

    // ---- 绘制列表 + swapchain ----
    /// `DrawLists::new` → `build_draw_lists` → `DrawLists::sort`（0x1409ba340/540/4b0）。
    fn build_draw_lists(&mut self);
    fn swapchain_info(&mut self) -> Option<SwapchainInfo>;

    // ---- 渲染图编译 ----
    /// 实例 0x1401647b0（加载画面图）。
    fn compile_loading_graph(
        &mut self,
        swapchain: &SwapchainInfo,
    ) -> Result<Self::Compiled, Self::CompileError>;
    /// 实例 0x140161cd0（世界图，内含 `render_world` 闭包）。`min_extent = Some([32,32])` 时为兜底编译。
    fn compile_world_graph(
        &mut self,
        swapchain: Option<&SwapchainInfo>,
        min_extent: Option<[u32; 2]>,
    ) -> Result<Self::Compiled, Self::CompileError>;

    // ---- OOM 降分辨率 ----
    /// 分辨率资源（TypeId 0xa4c02661e67b048b/0xeae761e6aa2a67ec）里 `+0x18` 的 f32 缩放；缺失 → unwrap（行 695）。
    fn resolution_scale(&mut self) -> Option<f32>;
    /// 写回缩放并把资源 changed_tick 设为 `world.change_tick`（O：`[rsi+0x5c] = [world+0x5b0]`）。
    fn set_resolution_scale(&mut self, scale: f32);
    /// `ResourcePool::clear`（0x140be76f0）。
    fn clear_resource_pool(&mut self);
    /// 把图状态里的两张 temporal 哈希表（`[state+0x160]`、`[state+0x250]`）换成带新随机种子的空表。
    fn reset_temporal_tables(&mut self);
    /// `GraphicsDevice::wait_idle_and_recycle_resources`（0x140bf8e60）。
    fn wait_idle_and_recycle_resources(&mut self);
    /// TypeId 0x3335386a5fbcee80/0x8f2a59f31a65ec60 的首个 u64 := 2（缺失则静默跳过，O：`je 0x1404e90ea`）。
    fn set_reduced_state(&mut self, value: u64);
    fn log(&mut self, level: Level, message: &str);

    // ---- 执行 ----
    /// 帧开始：`GraphExecutor::begin_frame`；`gpu_profiling` = 取一个 profiler 输出缓冲
    /// （池为空则 `create_gpu_profiler_output_buffer`）。
    fn begin_frame(&mut self, gpu_profiling: bool);
    /// 收集已完成帧的 profiler 结果（`retrieve_results` → `TimedFrame::new` → `send_to_puffin`）。
    fn drain_finished_frames(&mut self);
    /// `PassResources::new_from` + `execute_compiled` + `ExecutedRenderGraph::retire` + 取回导出的 swapchain 资源。
    fn execute_main_graph(&mut self, compiled: Self::Compiled);
    /// `GraphicsDevice::validate_swapchain_and_acquire_image`。
    fn acquire_swapchain_image(&mut self) -> Acquire;
    /// 展示图：`PassResources::new_from` + `GraphExecutionRecorder::execute<…execute_presentation_render_graph…>`
    /// + `TrackedSwapchainTextureLease::into_present_source`；原版里图缺失 → `Result::unwrap`（行 0x2e8，
    /// 文案 `msgs::PRESENTATION_FAILED`）。
    fn execute_presentation_graph(&mut self) -> Result<(), Self::CompileError>;
    fn end_frame(&mut self);
    /// `Swapchain::present`；swapchain 缺失 → `Option::unwrap`（行 0x35b）。
    fn present(&mut self);
    /// `tiny_glade::screenshot::finish_screenshot_capture`（0x14031ea20）。
    fn finish_screenshot_capture(&mut self);
    /// `RenderStats::snapshot_and_reset`，并把结果写到全局 `0x143634ba8`。
    fn snapshot_render_stats(&mut self);
    fn refresh_memory_budget(&mut self);
    /// `GraphExecutor::last_known_frame_duration_ns`（Some(ns)，O：返回 `(rax==1, rdx)`）。
    fn last_gpu_frame_ns(&mut self) -> Option<u64>;
    /// `GraphExecutor::wait_gpu_queue_trim(device, duration)`（0x140bea680）。
    fn wait_gpu_queue_trim(&mut self, wait: Duration);
}

/// 4 项环缓冲（O：两组全局，`[0x143634bb8 + 8*(n&3)]`/计数 `0x143634bd8` 与 `[0x143634be0 + 8*(n&3)]`/计数 `0x143634c00`）。
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct Ring4 {
    values: [f64; 4],
    count: u64,
}

impl Ring4 {
    pub fn push(&mut self, v: f64) {
        self.values[(self.count & 3) as usize] = v;
        self.count = self.count.wrapping_add(1);
    }
    /// 有效项数 = `min(count, 4)`（O：逐项展开的 `minsd/maxsd` 链按 `count` 提前退出）。
    pub fn len(&self) -> usize {
        self.count.min(4) as usize
    }
    pub fn is_empty(&self) -> bool {
        self.count == 0
    }
    fn valid(&self) -> &[f64] {
        &self.values[..self.len()]
    }
    /// 最小值（空 → `f64::MAX`，O：`movsd xmm0, [0x14292fee8]` = 1.797e308）。
    pub fn min(&self) -> f64 {
        self.valid().iter().copied().fold(f64::MAX, f64::min)
    }
    /// 最大值（空 → `-f64::MAX`，O：`[0x1429eb700]` = -1.797e308）。
    pub fn max(&self) -> f64 {
        self.valid().iter().copied().fold(-f64::MAX, f64::max)
    }
}

/// 帧节奏状态（进程全局，O）：GPU/CPU 帧时间各 4 帧窗口，以及上一帧结束时刻（`0x143634bb0` + `0x14362b0e8`）。
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct FramePacer {
    pub gpu: Ring4,
    pub cpu: Ring4,
    /// 上一帧结束时刻（O：纳秒字段 `== 1_000_000_000` 为 `None` niche）。
    pub last_frame_end: Option<Duration>,
}

impl FramePacer {
    /// 记录一帧：GPU 时间 `gpu_ns / 1e9`；CPU 时间 = `since_prev_frame + exec_elapsed`（O：两个 Duration 先分别转 f64 再相加）。
    pub fn record(&mut self, gpu_ns: u64, since_prev_frame: Duration, exec_elapsed: Duration) {
        self.gpu.push(gpu_ns as f64 / 1e9);
        self.cpu
            .push(since_prev_frame.as_secs_f64() + exec_elapsed.as_secs_f64());
    }
    /// 需要等待的秒数：`max(min(gpu 窗口) - max(cpu 窗口), 0.0)`（O：0x1404ea2e4..2ec，`maxsd xmm0, 0`）。
    pub fn queue_trim_seconds(&self) -> f64 {
        (self.gpu.min() - self.cpu.max()).max(0.0)
    }
}

/// `Duration::from_secs_f64`（O：0x1404ea2f0..44a 内联；失败 → `from_secs_f64::panic_cold_display`，文案
/// [`msgs::DURATION_PANIC`]）。
pub fn duration_from_secs_f64(secs: f64) -> Duration {
    Duration::try_from_secs_f64(secs).unwrap_or_else(|_| panic!("{}", msgs::DURATION_PANIC))
}

/// OOM 重试决策：返回新缩放；`new <= 0.0` 为"显存不足"终止条件（O：`xorps xmm1,xmm1; ucomiss xmm1,xmm0; jae panic`）。
pub fn reduced_scale(old: f32) -> Result<f32, f32> {
    let new = old + RESOLUTION_SCALE_STEP;
    if 0.0 >= new { Err(new) } else { Ok(new) }
}

/// OOM 日志文案（O：两段字面量 + 两个 `f32` 的 `Display`：旧值（资源里的值）、新值）。
pub fn oom_message(old: f32, new: f32) -> String {
    format!("{}{old}{}{new}", msgs::OOM_PREFIX, msgs::OOM_ARROW)
}

/// `render` 的 panic：显存不足（O：`panic_fmt`，参数是 `Option<(vk::Format, vk::Extent2D)>` 的 Debug）。
///
/// FIX-A1（reviewer-A-tinyglade）：原版是
/// `core::option::impl$46::fmt<tuple<ash::vk::Format,array<u32,2>>>`，即用 `{:?}`
/// 打印整个 `Option<(vk::Format, [u32; 2])>`；ash 0.38 的 `Format` Debug 输出
/// 变体名（如 `B8G8R8A8_SRGB`），不是数值。这里把内部 u32 还原成
/// `vk::Format::from_raw` 后整体 Debug 格式化，保持原生 Debug 形状。
pub fn out_of_video_memory(info: Option<&SwapchainInfo>) -> String {
    let info = info.map(|i| (ash::vk::Format::from_raw(i.format as i32), i.extent));
    format!("{}{info:?}", msgs::OUT_OF_VRAM)
}

/// `tiny_glade::render_loop::render`（0x1404e8010）。
pub fn render<H: RenderHost>(
    host: &mut H, pacer: &mut FramePacer, mode: RenderMode,
) -> Result<(), H::CompileError> {
    let scopes_on = host.puffin_scopes_on();
    if scopes_on {
        host.scope_begin(SCOPE_NAME);
    }
    // Commit pacing history only after all fallible frame operations succeed.
    let mut next_pacer = *pacer;
    let result = render_inner(host, &mut next_pacer, mode, scopes_on);
    if result.is_err() {
        host.abort_frame();
    } else {
        *pacer = next_pacer;
    }
    if scopes_on {
        host.scope_end();
    }
    result
}

fn check_host_error<H: RenderHost>(host: &mut H) -> Result<(), H::CompileError> {
    match host.take_render_error() {
        Some(error) => Err(error),
        None => Ok(()),
    }
}

fn render_inner<H: RenderHost>(
    host: &mut H, pacer: &mut FramePacer, mode: RenderMode, scopes_on: bool,
) -> Result<(), H::CompileError> {
    // 0x1404e82c3..e8c..：设备（unwrap，行 642）、GameConfig（expect，行 650）
    let device_ready = host.device_ready();
    check_host_error(host)?;
    assert!(device_ready, "called `Option::unwrap()` on a `None` value");
    let video_ready = host.clone_video_settings();
    check_host_error(host)?;
    assert!(video_ready, "{}", msgs::GAME_CONFIG);

    // 0x1404e8799：绘制列表
    host.build_draw_lists();
    check_host_error(host)?;
    let swapchain = host.swapchain_info();

    // 0x1404e883b：模式字节分支
    let (compiled, primary_ok) = if mode == RenderMode::LoadingScreen {
        let sc = swapchain.as_ref().expect(msgs::LOADING_NEEDS_SWAPCHAIN);
        let compiled = host.compile_loading_graph(sc)?;
        (compiled, true)
    } else {
        match host.compile_world_graph(swapchain.as_ref(), None) {
            Ok(c) => (c, true),
            Err(error) if H::compile_error_is_out_of_memory(&error) => {
                (recompile_at_reduced_scale(host, swapchain.as_ref())?, false)
            }
            Err(error) => return Err(error),
        }
    };

    // 0x1404e954f：上一帧结束到现在的间隔（`None` → 0）
    let since_prev_frame = pacer
        .last_frame_end
        .map(|prev| host.now().saturating_sub(prev))
        .unwrap_or(Duration::ZERO);
    let gpu_profiling = mode == RenderMode::WorldGpuProfiled && scopes_on;
    host.begin_frame(gpu_profiling);
    check_host_error(host)?;
    host.drain_finished_frames();
    check_host_error(host)?;

    // 0x1404e9a40：执行段起点
    let exec_start = host.now();
    host.execute_main_graph(compiled);
    check_host_error(host)?;

    // 0x1404e9bc2：只有主编译成功且有 swapchain 才取图像/展示（`!primary_ok || swapchain.is_none()` → 跳过）
    let mut present = false;
    if primary_ok && swapchain.is_some() {
        let acquired = host.acquire_swapchain_image();
        check_host_error(host)?;
        if acquired == Acquire::Image {
            host.execute_presentation_graph()?;
            present = true;
        }
    }
    host.end_frame();
    check_host_error(host)?;
    if present {
        host.present();
        check_host_error(host)?;
    }
    host.finish_screenshot_capture();
    check_host_error(host)?;
    host.snapshot_render_stats();
    check_host_error(host)?;
    host.refresh_memory_budget();
    check_host_error(host)?;

    // 0x1404ea052：GPU 帧节奏
    if host.gpu_pacing_enabled() {
        let exec_elapsed = host.now().saturating_sub(exec_start);
        if let Some(gpu_ns) = host.last_gpu_frame_ns() {
            pacer.record(gpu_ns, since_prev_frame, exec_elapsed);
            host.wait_gpu_queue_trim(duration_from_secs_f64(pacer.queue_trim_seconds()));
            check_host_error(host)?;
        }
    }
    // 0x1404ea47b：记录本帧结束时刻
    pacer.last_frame_end = Some(host.now());

    Ok(())
}

/// 世界图首次编译失败后的恢复（O：0x1404e8a3d..0x1404e9516）。只重试一次，且重试的结果不再展示。
fn recompile_at_reduced_scale<H: RenderHost>(
    host: &mut H,
    swapchain: Option<&SwapchainInfo>,
) -> Result<H::Compiled, H::CompileError> {
    let old = host.resolution_scale().unwrap();
    let new = match reduced_scale(old) {
        Ok(n) => n,
        Err(n) => {
            // 日志在 panic 之前已发出（O：log 调用在 `ucomiss` 之前）
            host.log(Level::Error, &oom_message(old, n));
            panic!("{}", out_of_video_memory(swapchain));
        }
    };
    host.log(Level::Error, &oom_message(old, new));
    host.set_resolution_scale(new);
    check_host_error(host)?;
    host.clear_resource_pool();
    check_host_error(host)?;
    host.reset_temporal_tables();
    check_host_error(host)?;
    host.wait_idle_and_recycle_resources();
    check_host_error(host)?;
    host.set_reduced_state(REDUCED_STATE_VALUE);
    check_host_error(host)?;
    host.log(Level::Info, msgs::COMPILING_REDUCED);
    let compiled = host.compile_world_graph(swapchain, Some(MIN_RESOLUTION_EXTENT))?;
    host.log(Level::Info, msgs::REDUCED_COMPILED);
    Ok(compiled)
}

/// `tiny_glade::render_loop::handle_screenshot_error`（0x1404ebfb0）：`Some(err)` 且日志等级 ≥ Error 时
/// 记一条 Error 日志 `"Error while capturing a screenshot: {err}"`（target `tiny_glade::render_loop`）；
/// `None` 什么都不做。返回要记录的 `(level, message)`，由调用方送入日志后端。
pub fn handle_screenshot_error<E: core::fmt::Display>(
    err: Option<E>,
    log_enabled: bool,
) -> Option<(Level, String)> {
    let err = err?;
    log_enabled.then(|| (Level::Error, format!("{}{err}", msgs::SCREENSHOT_ERROR)))
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::cell::Cell;

    struct Host {
        log: Vec<String>,
        scopes_on: bool,
        device: bool,
        config: bool,
        swapchain: Option<SwapchainInfo>,
        world_results: Vec<Result<u32, &'static str>>,
        loading_result: Result<u32, &'static str>,
        scale: Option<f32>,
        acquire: Acquire,
        presentation: Result<(), &'static str>,
        pacing: bool,
        gpu_ns: Option<u64>,
        clock: Cell<u64>,
        fail_on: Option<&'static str>,
        pending_error: Option<&'static str>,
    }

    impl Host {
        fn new() -> Self {
            Self {
                log: vec![],
                scopes_on: false,
                device: true,
                config: true,
                swapchain: Some(SwapchainInfo {
                    format: 50,
                    extent: [1920, 1080],
                }),
                world_results: vec![Ok(1)],
                loading_result: Ok(2),
                scale: Some(1.0),
                acquire: Acquire::Image,
                presentation: Ok(()),
                pacing: false,
                gpu_ns: None,
                clock: Cell::new(1_000_000_000),
                fail_on: None,
                pending_error: None,
            }
        }
        fn p(&mut self, s: impl Into<String>) {
            let s = s.into();
            if self.fail_on == Some(s.as_str()) && self.pending_error.is_none() {
                self.pending_error = Some("callback failure");
            }
            self.log.push(s);
        }
    }

    impl RenderHost for Host {
        type Compiled = u32;
        type CompileError = &'static str;
        fn compile_error_is_out_of_memory(error: &&'static str) -> bool {
            *error == "oom"
        }
        fn take_render_error(&mut self) -> Option<Self::CompileError> {
            self.pending_error.take()
        }
        fn abort_frame(&mut self) {
            self.p("abort_frame");
        }
        fn puffin_scopes_on(&mut self) -> bool {
            self.scopes_on
        }
        fn scope_begin(&mut self, n: &str) {
            self.p(format!("scope_begin {n}"));
        }
        fn scope_end(&mut self) {
            self.p("scope_end");
        }
        fn now(&mut self) -> Duration {
            // 每次调用前进 1ms
            let t = self.clock.get();
            self.clock.set(t + 1_000_000);
            Duration::from_nanos(t)
        }
        fn device_ready(&mut self) -> bool {
            self.device
        }
        fn clone_video_settings(&mut self) -> bool {
            self.config
        }
        fn gpu_pacing_enabled(&mut self) -> bool {
            self.pacing
        }
        fn build_draw_lists(&mut self) {
            self.p("draw_lists");
        }
        fn swapchain_info(&mut self) -> Option<SwapchainInfo> {
            self.swapchain
        }
        fn compile_loading_graph(&mut self, _sc: &SwapchainInfo) -> Result<u32, &'static str> {
            self.p("compile_loading");
            self.loading_result
        }
        fn compile_world_graph(
            &mut self,
            _sc: Option<&SwapchainInfo>,
            min: Option<[u32; 2]>,
        ) -> Result<u32, &'static str> {
            self.p(format!("compile_world min={min:?}"));
            self.world_results.remove(0)
        }
        fn resolution_scale(&mut self) -> Option<f32> {
            self.scale
        }
        fn set_resolution_scale(&mut self, s: f32) {
            self.p(format!("set_scale {s}"));
        }
        fn clear_resource_pool(&mut self) {
            self.p("clear_pool");
        }
        fn reset_temporal_tables(&mut self) {
            self.p("reset_temporal");
        }
        fn wait_idle_and_recycle_resources(&mut self) {
            self.p("wait_idle");
        }
        fn set_reduced_state(&mut self, v: u64) {
            self.p(format!("reduced_state {v}"));
        }
        fn log(&mut self, l: Level, m: &str) {
            self.p(format!("log {:?}/{} {m}", l, l as u8));
        }
        fn begin_frame(&mut self, gpu: bool) {
            self.p(format!("begin_frame gpu={gpu}"));
        }
        fn drain_finished_frames(&mut self) {
            self.p("drain_frames");
        }
        fn execute_main_graph(&mut self, c: u32) {
            self.p(format!("execute {c}"));
        }
        fn acquire_swapchain_image(&mut self) -> Acquire {
            self.p("acquire");
            self.acquire
        }
        fn execute_presentation_graph(&mut self) -> Result<(), &'static str> {
            self.p("presentation");
            self.presentation
        }
        fn end_frame(&mut self) {
            self.p("end_frame");
        }
        fn present(&mut self) {
            self.p("present");
        }
        fn finish_screenshot_capture(&mut self) {
            self.p("screenshot");
        }
        fn snapshot_render_stats(&mut self) {
            self.p("stats");
        }
        fn refresh_memory_budget(&mut self) {
            self.p("memory_budget");
        }
        fn last_gpu_frame_ns(&mut self) -> Option<u64> {
            self.gpu_ns
        }
        fn wait_gpu_queue_trim(&mut self, w: Duration) {
            self.p(format!("trim {:?}", w));
        }
    }

    fn run(h: &mut Host, mode: RenderMode) -> FramePacer {
        let mut pacer = FramePacer::default();
        render(h, &mut pacer, mode).unwrap();
        pacer
    }

    #[test]
    fn world_frame_happy_path_order() {
        let mut h = Host::new();
        let pacer = run(&mut h, RenderMode::World);
        assert_eq!(
            h.log,
            [
                "draw_lists",
                "compile_world min=None",
                "begin_frame gpu=false",
                "drain_frames",
                "execute 1",
                "acquire",
                "presentation",
                "end_frame",
                "present",
                "screenshot",
                "stats",
                "memory_budget",
            ]
        );
        assert!(pacer.last_frame_end.is_some());
    }

    #[test]
    fn puffin_scope_wraps_frame_when_on() {
        let mut h = Host::new();
        h.scopes_on = true;
        run(&mut h, RenderMode::WorldGpuProfiled);
        assert_eq!(
            h.log.first().unwrap(),
            "scope_begin tiny_glade::render_loop::render"
        );
        assert_eq!(h.log.last().unwrap(), "scope_end");
        assert!(h.log.contains(&"begin_frame gpu=true".to_string()));
        // 模式 2 但 puffin 关 → 不取 profiler 缓冲
        let mut h = Host::new();
        run(&mut h, RenderMode::WorldGpuProfiled);
        assert!(h.log.contains(&"begin_frame gpu=false".to_string()));
    }

    #[test]
    fn loading_screen_uses_loading_graph() {
        let mut h = Host::new();
        run(&mut h, RenderMode::LoadingScreen);
        assert!(h.log.contains(&"compile_loading".to_string()));
        assert!(!h.log.iter().any(|l| l.starts_with("compile_world")));
        assert!(h.log.contains(&"execute 2".to_string()));
        assert!(h.log.contains(&"present".to_string()));
    }

    #[test]
    #[should_panic(expected = "The loading screen must have a swapchain")]
    fn loading_screen_without_swapchain_panics() {
        let mut h = Host::new();
        h.swapchain = None;
        run(&mut h, RenderMode::LoadingScreen);
    }

    #[test]
    fn loading_graph_compile_error_preserves_cause_and_aborts() {
        let mut h = Host::new();
        h.loading_result = Err("bad");
        let mut pacer = FramePacer::default();
        assert_eq!(render(&mut h, &mut pacer, RenderMode::LoadingScreen), Err("bad"));
        assert_eq!(h.log, ["draw_lists", "compile_loading", "abort_frame"]);
        assert!(pacer.last_frame_end.is_none());
    }

    #[test]
    #[should_panic(expected = "GameConfig")]
    fn missing_game_config_panics() {
        let mut h = Host::new();
        h.config = false;
        run(&mut h, RenderMode::World);
    }

    #[test]
    #[should_panic(expected = "called `Option::unwrap()` on a `None` value")]
    fn missing_device_panics() {
        let mut h = Host::new();
        h.device = false;
        run(&mut h, RenderMode::World);
    }

    #[test]
    fn world_without_swapchain_compiles_but_does_not_present() {
        let mut h = Host::new();
        h.swapchain = None;
        run(&mut h, RenderMode::World);
        assert!(!h.log.contains(&"acquire".to_string()));
        assert!(!h.log.contains(&"present".to_string()));
        assert!(h.log.contains(&"end_frame".to_string()));
    }

    #[test]
    fn unavailable_swapchain_image_skips_presentation_and_present() {
        let mut h = Host::new();
        h.acquire = Acquire::Unavailable;
        run(&mut h, RenderMode::World);
        assert!(h.log.contains(&"acquire".to_string()));
        assert!(!h.log.contains(&"presentation".to_string()));
        assert!(!h.log.contains(&"present".to_string()));
        assert!(h.log.contains(&"end_frame".to_string()));
        assert!(h.log.contains(&"screenshot".to_string()));
    }

    #[test]
    fn oom_recompiles_once_with_reduced_scale_and_skips_presentation() {
        let mut h = Host::new();
        h.world_results = vec![Err("oom"), Ok(9)];
        h.scale = Some(1.0);
        run(&mut h, RenderMode::World);
        let head: Vec<_> = h.log.iter().take(11).cloned().collect();
        assert_eq!(
            head,
            [
                "draw_lists",
                "compile_world min=None",
                "log Error/1 Out of memory while allocating render graph resources; reducing resolution scale 1 -> 0.9",
                "set_scale 0.9",
                "clear_pool",
                "reset_temporal",
                "wait_idle",
                "reduced_state 2",
                "log Info/3 Compiling a render graph for the reduced resolution scale",
                "compile_world min=Some([32, 32])",
                "log Info/3 Reduced-resolution scale render graph compiled",
            ]
        );
        assert!(h.log.contains(&"execute 9".to_string()));
        // 降级路径不取 swapchain 图像、不 present
        assert!(!h.log.contains(&"acquire".to_string()));
        assert!(!h.log.contains(&"present".to_string()));
    }

    #[test]
    #[should_panic(
        expected = "Out of video memory. Swapchain info: Some((B8G8R8A8_SRGB, [1920, 1080]))"
    )]
    fn oom_at_scale_floor_panics_with_swapchain_info() {
        let mut h = Host::new();
        h.world_results = vec![Err("oom")];
        h.scale = Some(0.1);
        run(&mut h, RenderMode::World);
    }

    #[test]
    fn oom_at_floor_logs_before_panicking() {
        let mut h = Host::new();
        h.world_results = vec![Err("oom")];
        h.scale = Some(0.05);
        let r = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
            run(&mut h, RenderMode::World)
        }));
        assert!(r.is_err());
        assert!(
            h.log
                .iter()
                .any(|l| l.starts_with("log Error/1 Out of memory"))
        );
        assert!(!h.log.iter().any(|l| l.starts_with("set_scale")));
    }

    #[test]
    fn fallback_compile_failure_preserves_cause() {
        let mut h = Host::new();
        h.world_results = vec![Err("oom"), Err("again")];
        let mut pacer = FramePacer::default();
        assert_eq!(render(&mut h, &mut pacer, RenderMode::World), Err("again"));
        assert_eq!(h.log.last().unwrap(), "abort_frame");
        assert!(!h.log.iter().any(|line| line.starts_with("begin_frame")));
        assert!(pacer.last_frame_end.is_none());
    }

    #[test]
    #[should_panic(expected = "called `Option::unwrap()` on a `None` value")]
    fn missing_resolution_resource_panics_on_oom() {
        let mut h = Host::new();
        h.world_results = vec![Err("oom")];
        h.scale = None;
        run(&mut h, RenderMode::World);
    }

    #[test]
    fn presentation_failure_preserves_cause_and_skips_success_effects() {
        let mut h = Host::new();
        h.presentation = Err("nope");
        let mut pacer = FramePacer::default();
        assert_eq!(render(&mut h, &mut pacer, RenderMode::World), Err("nope"));
        assert_eq!(h.log.last().unwrap(), "abort_frame");
        assert!(!h.log.iter().any(|line| matches!(line.as_str(), "present" | "stats" | "screenshot")));
        assert!(pacer.last_frame_end.is_none());
    }

    #[test]
    fn non_oom_compile_errors_do_not_mutate_resolution_or_temporal_state() {
        for cause in ["invalid resource", "timeout", "unsupported", "device lost", "exhausted"] {
            let mut h = Host::new();
            h.scopes_on = true;
            h.world_results = vec![Err(cause)];
            let mut pacer = FramePacer::default();
            assert_eq!(render(&mut h, &mut pacer, RenderMode::World), Err(cause));
            assert_eq!(h.scale, Some(1.0));
            assert_eq!(h.log, [
                "scope_begin tiny_glade::render_loop::render", "draw_lists",
                "compile_world min=None", "abort_frame", "scope_end",
            ]);
            assert_eq!(pacer, FramePacer::default());
        }
    }

    #[test]
    fn recorded_callback_errors_stop_at_the_actual_failure() {
        for stage in ["draw_lists", "begin_frame gpu=false", "drain_frames", "execute 1",
                      "acquire", "end_frame", "present", "screenshot", "stats", "memory_budget"] {
            let mut h = Host::new();
            h.fail_on = Some(stage);
            let mut pacer = FramePacer::default();
            assert_eq!(render(&mut h, &mut pacer, RenderMode::World), Err("callback failure"), "{stage}");
            let failed = h.log.iter().position(|line| line == stage).unwrap();
            assert_eq!(&h.log[failed + 1..], &["abort_frame"], "{stage}");
            assert_eq!(pacer, FramePacer::default(), "{stage}");
        }
    }

    #[test]
    fn pacing_waits_when_gpu_slower_than_cpu_window() {
        let mut h = Host::new();
        h.pacing = true;
        h.gpu_ns = Some(20_000_000); // 20 ms
        let pacer = run(&mut h, RenderMode::World);
        // gpu 窗口 [0.02]，cpu 窗口 = 间隔(0) + 执行耗时(若干 ms) → 等待 = 0.02 - cpu
        let wait = h
            .log
            .iter()
            .find_map(|l| l.strip_prefix("trim "))
            .expect("trim");
        assert!(wait.ends_with("ms") || wait.ends_with("µs"), "{wait}");
        assert_eq!(pacer.gpu.len(), 1);
        assert_eq!(pacer.cpu.len(), 1);
    }

    #[test]
    fn pacing_skipped_when_disabled_or_gpu_time_unknown() {
        let mut h = Host::new();
        h.pacing = false;
        h.gpu_ns = Some(1);
        run(&mut h, RenderMode::World);
        assert!(!h.log.iter().any(|l| l.starts_with("trim")));
        let mut h = Host::new();
        h.pacing = true;
        h.gpu_ns = None;
        let p = run(&mut h, RenderMode::World);
        assert!(!h.log.iter().any(|l| l.starts_with("trim")));
        assert!(p.gpu.is_empty());
    }

    #[test]
    fn second_frame_measures_gap_since_previous_frame_end() {
        let mut h = Host::new();
        h.pacing = true;
        h.gpu_ns = Some(5_000_000);
        h.world_results = vec![Ok(1), Ok(1)];
        let mut pacer = FramePacer::default();
        render(&mut h, &mut pacer, RenderMode::World).unwrap();
        let first_end = pacer.last_frame_end.unwrap();
        render(&mut h, &mut pacer, RenderMode::World).unwrap();
        assert!(pacer.last_frame_end.unwrap() > first_end);
        assert_eq!(pacer.cpu.len(), 2);
        // 第二帧 cpu 时间含与上一帧结束的间隔，必然 > 第一帧
        assert!(pacer.cpu.values[1] > pacer.cpu.values[0]);
    }

    #[test]
    fn ring4_window_and_extrema() {
        let mut r = Ring4::default();
        assert!(r.is_empty());
        assert_eq!(r.min(), f64::MAX);
        assert_eq!(r.max(), -f64::MAX);
        for v in [5.0, 3.0, 9.0, 4.0] {
            r.push(v);
        }
        assert_eq!((r.min(), r.max(), r.len()), (3.0, 9.0, 4));
        // 第 5 次覆盖槽 0（`count & 3`）
        r.push(1.0);
        assert_eq!((r.min(), r.max()), (1.0, 9.0));
        r.push(0.5);
        r.push(0.25);
        r.push(0.125); // 9.0 被覆盖
        assert_eq!((r.min(), r.max()), (0.125, 1.0));
    }

    #[test]
    fn queue_trim_is_clamped_at_zero_and_uses_min_gpu_max_cpu() {
        let mut p = FramePacer::default();
        p.record(
            10_000_000,
            Duration::from_millis(2),
            Duration::from_millis(3),
        ); // gpu 10ms, cpu 5ms
        assert!((p.queue_trim_seconds() - 0.005).abs() < 1e-9);
        p.record(
            8_000_000,
            Duration::from_millis(1),
            Duration::from_millis(10),
        ); // gpu 8ms, cpu 11ms
        // min(gpu)=8ms, max(cpu)=11ms → 负 → 0
        assert_eq!(p.queue_trim_seconds(), 0.0);
        // 空窗口（原版不会出现：先 push 再读）：MAX - (-MAX) = inf，随后 Duration 转换 panic
        assert!(FramePacer::default().queue_trim_seconds().is_infinite());
    }

    #[test]
    fn duration_conversion_matches_std_and_panics_like_original() {
        assert_eq!(duration_from_secs_f64(0.5), Duration::from_millis(500));
        assert_eq!(duration_from_secs_f64(0.0), Duration::ZERO);
        let r = std::panic::catch_unwind(|| duration_from_secs_f64(f64::NAN));
        let msg = *r.unwrap_err().downcast::<String>().unwrap();
        assert!(
            msg.contains("cannot convert float seconds to Duration"),
            "{msg}"
        );
        // 巨大值（> u64::MAX 秒）同样 panic
        assert!(std::panic::catch_unwind(|| duration_from_secs_f64(1e30)).is_err());
    }

    #[test]
    fn reduced_scale_steps_by_point_one_and_stops_at_zero() {
        assert!((reduced_scale(1.0).unwrap() - 0.9).abs() < 1e-6);
        assert!(reduced_scale(0.15).is_ok());
        assert!(reduced_scale(0.1).is_err()); // 0.1 + (-0.1) 刚好 ≤ 0
        assert!(reduced_scale(0.05).is_err());
        assert_eq!(RESOLUTION_SCALE_STEP, -0.1f32);
        assert_eq!(MIN_RESOLUTION_EXTENT, [32, 32]);
        assert_eq!(u64::from(0x0000_0020u32) | (0x20u64 << 32), 0x2000000020);
    }

    #[test]
    fn oom_message_and_vram_message_formats() {
        assert_eq!(
            oom_message(1.0, 0.9),
            "Out of memory while allocating render graph resources; reducing resolution scale 1 -> 0.9"
        );
        assert_eq!(
            out_of_video_memory(None),
            "Out of video memory. Swapchain info: None"
        );
        // FIX-A1：格式按 ash `vk::Format` 的 Debug 变体名输出（50 = B8G8R8A8_SRGB）。
        let info = SwapchainInfo {
            format: 50,
            extent: [1920, 1080],
        };
        assert_eq!(
            out_of_video_memory(Some(&info)),
            "Out of video memory. Swapchain info: Some((B8G8R8A8_SRGB, [1920, 1080]))"
        );
    }

    #[test]
    fn render_mode_byte_mapping() {
        assert_eq!(RenderMode::from_byte(0), RenderMode::LoadingScreen);
        assert_eq!(RenderMode::from_byte(1), RenderMode::World);
        assert_eq!(RenderMode::from_byte(2), RenderMode::WorldGpuProfiled);
    }

    #[test]
    fn screenshot_error_logging() {
        assert_eq!(handle_screenshot_error::<String>(None, true), None);
        assert_eq!(handle_screenshot_error(Some("disk full"), false), None);
        assert_eq!(
            handle_screenshot_error(Some("disk full"), true),
            Some((
                Level::Error,
                "Error while capturing a screenshot: disk full".to_string()
            ))
        );
        assert_eq!(Level::Error as u8, 1);
        assert_eq!(Level::Info as u8, 3);
    }
}
