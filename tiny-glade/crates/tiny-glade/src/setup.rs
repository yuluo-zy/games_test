//! B `crates/tiny-glade/src/setup.rs`（窗口建立 / 全屏模式选择 / winit 类型转换）。
//!
//! 恢复依据（Ghidra shared 2026-10-17，restorer-tinyglade-E fresh）：
//! - `setup_window` `0x1400436b0`（5,072 B）
//! - `find_best_fullscreen_video_mode` `0x1400453c0`（5,090 B）
//! - `impl$0::from_winit` `0x140175450`（796 B，`MonitorMatch <- MonitorHandle`）
//! - `impl$1::from_winit` `0x140175990`（91 B，`VideoMode <- winit::VideoMode`）
//!
//! ## `find_best_fullscreen_video_mode`（判等核心 + 平台等价面）
//!
//! fresh 反编译逐语句判等（0x1400453c0 + `disassemble_function`）：
//! 1. `get_monitor_info().unwrap()`；设备名 `decode_wide → to_string_lossy`（日志/匹配用）。
//! 2. 再次 `get_monitor_info()`：
//!    - Err → `log::warn!(target: "winit::platform_impl::platform::monitor",
//!      "Error from get_monitor_info: {err}")`（`max_level > Warn` 门），模式表为空；
//!    - Ok → 以设备名循环 `EnumDisplaySettingsExW(name, i, &dm, 0)`，`dm.dmSize=0xdc`，
//!      每项要求 `dmFields & 0x5c0000 == 0x5c0000`（缺则 `panic!`），
//!      `VideoMode{size=(dmPelsWidth,dmPelsHeight), bit_depth=dmBitsPerPel as u16,
//!      refresh_rate_millihertz=dmDisplayFrequency*1000}` 插入 `BTreeSet<winit::VideoMode>`。
//!      winit fork 的 `MonitorHandle::video_modes()` 内部正是该枚举 + BTreeSet
//!      （`platform_impl/windows/monitor.rs`），此处以公开等价面使用。
//! 3. 空表：`log::warn!("Monitor {name} @ {refresh} does not support any video modes")`
//!    → return `None`。`len>1` 时再次 `sort`（见分歧 D2）。
//! 4. `current == None` 时取 `EnumDisplaySettingsExW(.., ENUM_CURRENT_SETTINGS)` 的
//!    `dmDisplayFrequency*1000` 与监视器 rect 尺寸，在表中找完全相等的模式；
//!    命中则 `current = Some(VideoMode::from_winit(mode))`（公开等价面：
//!    `monitor.refresh_rate_millihertz()` + `monitor.size()`）。
//! 5. `current == Some(desired)` → `min_by_key`：
//!    `key = |w*h - desired.w*desired.h| + |refresh - desired.refresh|/10
//!           + |bit_depth - desired.bit_depth|*1000`（0x14004611c..0x14004616a 逐指令）；
//!    记 `log::debug!("Chosen video mode: {mode:?}")`。
//! 6. `current` 仍为 None → 记同一条 "does not support any video modes" warn，
//!    取表中第一个模式（0x140046333/0x1400461fe unwrap 路径）。
//!
//! **分歧 D1**：0x1400453c0 使用平台内部 `get_monitor_info`；本合同实现改用 fork
//! 公开等价面（`name/position/size/video_modes/refresh_rate_millihertz`），
//! `get_monitor_info` 失败分支（warn + 空表）在公开 API 下不可表达，未建模。
//! **分歧 D2**：B 在 BTreeSet→Vec 后另有一次 `sort`（PDB 里比较器被 devirtualize 成
//! `bool(*)(&VideoMode,&VideoMode)`，源码方向未锚定）；本实现按 BTreeSet 的 Ord
//! 顺序 `sort_by(|a,b| b.cmp(a))` 保留（同序，等价可观察行为）。
//! **分歧 D3**："does not support" 日志第二实参在 B 中为 i32（`fmt<i32>`），
//! 本实现取 `refresh_rate_millihertz`（mHz）。
//!
//! ## `setup_window`（重建，平台面）
//!
//! fresh 反编译控制流：
//! `puffin` scope → 期望文案 `"exclusive fullscreen requires a monitor"` →
//! `WindowBuilder::default()`（默认标题 `"winit window"`）→ 标题 `"Tiny Glade"` →
//! 监视器选择（`Desired monitor not found; using primary` /
//! `Attempting to place window on monitor: {monitor:?}`，0x140043... 的
//! `available_monitors().find(closure$0::closure$0)`）→ `DisplayConfig` 分支：
//! - `fullscreen && !allow_exclusive_fullscreen` → `Fullscreen::Borderless(Some(monitor))`；
//! - `fullscreen && allow_exclusive_fullscreen` → 取监视器信息（失败/缺失
//!   `expect("exclusive fullscreen requires a monitor")`）→
//!   `find_best_fullscreen_video_mode(&monitor, &mut None)`；Some →
//!   `Fullscreen::Exclusive(mode)` 并 `log::debug!("Chosen video mode: {mode:?}")`；
//!   None → borderless；
//! - 非全屏 → `windowed_resolution` 转 `PhysicalSize`，按监视器 rect 居中
//!   （源码两次 `get_monitor_info` + `dpi::from_f64`）。
//! 之后 `image::Reader::open("assets/window_icon.png") → decode → to_rgba8 →
//! Icon::from_rgba`，`window::init`（= `WindowBuilder::build`），
//! `RedrawWindow(.., RDW_INTERNALPAINT=2)`（= build 内部的 request_redraw）。
//! 返回 32 B：`Window` + `Option<winit::VideoMode>`。
//!
//! **分歧 D4**：参数 ABI 未锚定（rcx=sret 32B；rdx 事件循环 target；r8 指向
//! `DisplayConfig` 起始或其 monitor 槽）。本合同签名取
//! `(event_loop, &mut DisplayConfig) -> (Window, Option<winit::VideoMode>)`。
//! **分歧 D5**：`WindowAttributes` 的逐字段布尔写面（in_R8+0x60..+0x89 一带）未能
//! 与 fork 的 `WindowAttributes` 布局逐项对上；只恢复标题/尺寸/位置/全屏/图标五项，
//! 其余字段保持 `WindowBuilder` 默认。
//! **分歧 D6**：独占模式选择走 `find_best_fullscreen_video_mode`；
//! `expect_failed("exclusive fullscreen requires a monitor")` 的冷路径受
//! `local_258` 标志门控，该标志语义未锚定；默认配置
//! （`monitor=None` + `allow_exclusive_fullscreen=true`）在发行版中不得 panic，
//! 因此本实现不发出常开断言。`FullscreenConfig` 的分辨率载荷在反编译中未见消费点
//! （`local_1c0=0` 后被 `find_best` 填充），保持不消费。

use country_core::render::render_config::{DisplayConfig, MonitorMatch, VideoMode};
use winit::dpi::{PhysicalPosition, PhysicalSize};
use winit::event_loop::EventLoopWindowTarget;
use winit::monitor::MonitorHandle;
use winit::window::{Fullscreen, Icon, Window, WindowBuilder};

/// `tiny_glade::setup::FromWinit`（原 trait；`from_winit(&T)` 按引用取值）。
pub trait FromWinit<T> {
    fn from_winit(value: &T) -> Self;
}

impl FromWinit<winit::monitor::MonitorHandle> for MonitorMatch {
    /// `impl$0::from_winit` 0x140175450 —— 判等（平台公开等价面）。
    ///
    /// 反编译：`get_monitor_info` → `decode_wide` → `to_string_lossy` → `String`；
    /// 再次 `get_monitor_info` → rect → `[left, top]` / `[right-left, bottom-top]`；
    /// 输出 40 B（`Option<String>` 24 + position 8 + size 8）。fork 的
    /// `name()/position()/size()` 与 `get_monitor_info` 同源。
    fn from_winit(handle: &winit::monitor::MonitorHandle) -> Self {
        let position = handle.position();
        let size = handle.size();
        Self {
            name: handle.name(),
            physical_position: [position.x, position.y],
            physical_size: [size.width, size.height],
        }
    }
}

impl FromWinit<winit::monitor::VideoMode> for VideoMode {
    /// `impl$1::from_winit` 0x140175990 —— 判等。
    ///
    /// 反编译逐指令：`size()` 打包 `w | h<<32` 写 +0；`*(u32*)(src+0x18)` 写 +8
    /// （refresh_rate_millihertz）；`*(u16*)(src+0x1c)` 写 +0xc（bit_depth）。
    fn from_winit(mode: &winit::monitor::VideoMode) -> Self {
        let size = mode.size();
        Self {
            resolution: [size.width, size.height],
            bit_depth: mode.bit_depth(),
            refresh_rate_millihertz: mode.refresh_rate_millihertz(),
        }
    }
}

/// `tiny_glade::setup::find_best_fullscreen_video_mode` `0x1400453c0`。
///
/// `current` 为输入/输出槽：`None` 时表示未指定、由当前桌面模式填充；返回值是
/// 选中的 winit 模式（用于 `Fullscreen::Exclusive`）。逐语句依据见模块注释。
pub fn find_best_fullscreen_video_mode(
    monitor: &MonitorHandle,
    current: &mut Option<VideoMode>,
) -> Option<winit::monitor::VideoMode> {
    let monitor_name = monitor.name().unwrap_or_default();

    let mut modes: Vec<winit::monitor::VideoMode> = monitor.video_modes().collect();
    if modes.is_empty() {
        log_unsupported(&monitor_name, monitor.refresh_rate_millihertz());
        return None;
    }
    if modes.len() > 1 {
        // D2：B 的第二次 sort；方向未锚定，按 BTreeSet 的 Ord 顺序保留。
        modes.sort_by(|a, b| b.cmp(a));
    }

    if current.is_none() {
        // B：EnumDisplaySettingsExW(ENUM_CURRENT_SETTINGS) 与监视器 rect 匹配。
        // 公开等价面：当前桌面刷新率 + 监视器尺寸。
        if let Some(refresh) = monitor.refresh_rate_millihertz() {
            let size = monitor.size();
            if let Some(mode) = modes
                .iter()
                .find(|m| m.refresh_rate_millihertz() == refresh && m.size() == size)
            {
                *current = Some(VideoMode::from_winit(mode));
            }
        }
    }

    let chosen = match *current {
        Some(desired) => {
            // 0x14004611c..0x14004616a：area_diff + refresh_diff/10 + depth_diff*1000。
            let best = modes
                .iter()
                .min_by_key(|mode| {
                    let size = mode.size();
                    let area_diff = (size.width as i64 * size.height as i64
                        - desired.resolution[0] as i64 * desired.resolution[1] as i64)
                        .abs();
                    let refresh_diff = (mode.refresh_rate_millihertz() as i64
                        - desired.refresh_rate_millihertz as i64)
                        .abs()
                        / 10;
                    let depth_diff = (mode.bit_depth() as i64 - desired.bit_depth as i64).abs()
                        * 1000;
                    area_diff + refresh_diff + depth_diff
                })
                .expect("video_modes is non-empty");
            if log::max_level() >= log::LevelFilter::Debug {
                log::debug!("Chosen video mode: {best:?}");
            }
            best.clone()
        }
        None => {
            // 0x140046333 → get_monitor_info Ok → warn；0x1400461fe 取首个模式。
            log_unsupported(&monitor_name, monitor.refresh_rate_millihertz());
            modes[0].clone()
        }
    };
    Some(chosen)
}

/// `Monitor {} @ {} does not support any video modes`（0x14292daf0 区段文案）。
fn log_unsupported(name: &str, refresh: Option<u32>) {
    if log::max_level() >= log::LevelFilter::Warn {
        log::warn!(
            "Monitor {} @ {} does not support any video modes",
            name,
            refresh.unwrap_or(0) as i32
        );
    }
}

/// `tiny_glade::setup::setup_window` `0x1400436b0` —— 重建（平台面，见 D4–D6）。
///
/// 返回 `(Window, Option<winit::VideoMode>)`；第二个值在独占全屏时是实际应用的
/// 视频模式（B 的 sret 32 B 输出）。
pub fn setup_window(
    event_loop: &EventLoopWindowTarget<()>,
    display: &mut DisplayConfig,
) -> (Window, Option<winit::monitor::VideoMode>) {
    puffin::profile_function!();

    // 1. 监视器选择：desired → available_monitors().find(...) → fallback primary。
    let monitor = match display.monitor.as_ref() {
        Some(desired) => {
            log::debug!("Attempting to place window on monitor: {desired:?}");
            match event_loop
                .available_monitors()
                .find(|m| MonitorMatch::from_winit(m) == *desired)
            {
                Some(m) => m,
                None => {
                    log::warn!("Desired monitor not found; using primary");
                    event_loop
                        .primary_monitor()
                        .expect("no primary monitor available")
                }
            }
        }
        None => event_loop
            .primary_monitor()
            .expect("no primary monitor available"),
    };
    // 把实际使用的监视器写回配置（B 写 40 B MonitorMatch）。
    display.monitor = Some(MonitorMatch::from_winit(&monitor));

    // 2. 面板图标：assets/window_icon.png → decode → to_rgba8 → Icon::from_rgba。
    let icon = image::io::Reader::open("assets/window_icon.png")
        .ok()
        .and_then(|reader| reader.decode().ok())
        .map(|image| {
            let rgba = image.to_rgba8();
            let (w, h) = rgba.dimensions();
            Icon::from_rgba(rgba.into_raw(), w, h).ok()
        })
        .flatten();

    // 3. 窗口属性 + 全屏模式。
    let mut builder = WindowBuilder::new().with_title("Tiny Glade");
    if let Some(icon) = icon {
        builder = builder.with_window_icon(Some(icon));
    }

    let mut chosen_mode = None;
    if display.fullscreen {
        if !display.allow_exclusive_fullscreen {
            builder = builder.with_fullscreen(Some(Fullscreen::Borderless(Some(monitor.clone()))));
        } else {
            // B 在独占分支保留了 `expect_failed("exclusive fullscreen requires a
            // monitor")` 的冷路径（受 local_258 标志门控）；该标志的语义未能锚定。
            // 默认配置（monitor=None + allow_exclusive=true）在发行版中不得 panic，
            // 故此处不发出常开断言（见 D4/D6）。
            let mut current = None;
            if let Some(mode) = find_best_fullscreen_video_mode(&monitor, &mut current) {
                if log::max_level() >= log::LevelFilter::Debug {
                    log::debug!("Chosen video mode: {mode:?}");
                }
                chosen_mode = Some(mode.clone());
                builder = builder.with_fullscreen(Some(Fullscreen::Exclusive(mode)));
            } else {
                builder = builder.with_fullscreen(Some(Fullscreen::Borderless(Some(
                    monitor.clone(),
                ))));
            }
        }
    } else {
        let size = PhysicalSize::new(display.windowed_resolution[0], display.windowed_resolution[1]);
        let mpos = monitor.position();
        let msize = monitor.size();
        let x = mpos.x + (msize.width as i32 - size.width as i32) / 2;
        let y = mpos.y + (msize.height as i32 - size.height as i32) / 2;
        builder = builder
            .with_inner_size(size)
            .with_position(PhysicalPosition::new(x, y));
    }

    let window = builder
        .build(event_loop)
        .expect("failed to create window");
    // B: RedrawWindow(hwnd, null, null, RDW_INTERNALPAINT)。
    window.request_redraw();
    (window, chosen_mode)
}
