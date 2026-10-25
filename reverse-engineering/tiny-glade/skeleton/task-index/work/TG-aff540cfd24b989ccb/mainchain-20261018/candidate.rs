//! B `crates/tiny-glade/src/game_loop.rs`（winit 主循环 / 视频设置应用）。
//!
//! 恢复依据（Ghidra shared 2026-10-17，restorer-tinyglade-E fresh）：
//! - `GameLoop::new` `0x1404d8100`（327 B）
//! - `GameLoop::run` `0x1404d8250`（2,810 B）
//! - `GameLoop::main_loop` `0x1404d9580`（2,564 行反编译）
//! - `GameLoop::extract_available_monitors_info` `0x1404e2470`（481 行反编译）
//! - `impl$3::build` `0x1404e3370`（8 B）
//! - `gather_pending_video_setting_changes` `0x1404e3380`（102 行反编译）
//!
//! ## 逐函数结论
//!
//! ### `gather_pending_video_setting_changes` —— 重建（控制流逐语句判等）
//! fresh 0x1404e3380：
//! ```text
//! start = *cursor                       // 已消费全局序号
//! off1 = start.saturating_sub(seg1.start)      // +0x18
//! off2 = start.saturating_sub(seg2.start)      // +0x38
//! (ptr1,len1) = if seg1.end < off1 { (空,0) } else { (seg1.ptr+off1, seg1.end-off1) }
//! (ptr2,len2) = if seg2.end < off2 { (空,0) } else { (seg2.ptr+off2, seg2.end-off2) }
//! *cursor = (total - len2) - len1              // total @+0x40
//! for item in ptr1[..len1] then ptr2[..len2]:
//!     *cursor += 1
//!     Mutex<u8>::lock(); poison → unwrap_failed
//!     flags |= item.byte@+8
//!     unlock（旧 state==2 → futex::wake）
//! ```
//! 本实现用 `std::sync::Mutex<u8>`（poison unwrap + wake 语义由 std 提供），
//! 两个队列以 `(Vec, start)` 建模（B 布局：end@+0x10/+0x30、start@+0x18/+0x38、
//! total@+0x40，item stride 0x10、标志字节 @+8）。
//! **分歧 E1**：`PendingVideoSettingChanges`/`VideoSettingChange` 的字段名与真实
//! 类型未在 PDB 锚定（A 组在 `app_config` 留了占位）；本类型按上述语义在
//! game_loop 内重建，未改动 A 组 `app_config.rs`。标志全局类型取 `Mutex<u8>`
//! （B：state 字节 + poison 字节 + data 字节 = std Mutex<u8> 的静态布局）。
//!
//! ### `extract_available_monitors_info` —— 重建
//! fresh 0x1404e2470：`ResMut<AvailableMonitors>`（resource id hash
//! `0xe5e047cb9d6d98d7`）→ `clear()` → `available_monitors()` →
//! 逐个 monitor：
//! `MonitorMatch::from_winit`；`get_monitor_info` 失败 → warn
//! `Error from get_monitor_info`（target `winit::platform_impl::platform::monitor`）
//! 并留空模式表；成功 → `EnumDisplaySettingsExW` 枚举（dmSize=0xdc，
//! `dmFields & 0x5c0000` 全置位否则 panic）→ BTreeSet → Vec →
//! `sort` → map `VideoMode::from_winit` → `AvailableMonitor{ video_modes, monitor }`
//! push 到 `AvailableMonitors.monitors`。
//! 本实现用 fork 公开等价面 `MonitorHandle::video_modes()`（内部同一枚举）。
//! **分歧 E2**：同上 D2 的第二次 sort 方向未锚定，按 BTreeSet Ord 保留；
//! `get_monitor_info` 失败分支在公开 API 下不可表达（`name()` 返回 Option）。
//!
//! ### `GameLoop::new` / `run` / `main_loop` —— 未实现（诚实登记）
//! - `new`（0x1404d8100）：纯字段 memcpy，参数为 5 个大型按值结构
//!   （0xa8B / 0x658B / 0xa8B / 0x2b0B / 4×u64 …）。字段类型无独立证据
//!   （`AvailableMonitors` 已落库，但 `AvailableGraphicsDevices`、
//!   `SteamClientResource`、`RenderResourceCache`、`App` 等持有类型未锚定），
//!   按字节布局硬写会造类型 → 保持 `todo!`。
//! - `run`（0x1404d8250）：`main_loop()` → 从 window 的
//!   `Mutex<WindowState>` 取 pending fullscreen 并
//!   `EventLoopThreadExecutor::execute_in_thread(set_fullscreen)` → 取
//!   `GraphicsDevice`（resource hash `0x7c31cf58ebf489e1`）→ 峰值 GPU 内存日志
//!   （`Peak GPU memory use (device local)`）→ 序列化 input stream 到文件 →
//!   `shut_down_everything`（drop App/SubApp/事件通道/RenderResourceCache/
//!   RenderGraphs/Steam/EguiBackend）。依赖 GameLoop 字段与资源面 → `todo!`。
//! - `main_loop`（0x1404d9580）：2,564 行反编译；含 winit 事件泵、Bevy
//!   `App::update`、egui 输入、swapchain 重建/失焦、设备丢失恢复、监视器变化、
//!   `WindowSize::set`、`AvailableMonitors`、`WriteSettingsFileCmd`、
//!   `TgIced` GUI、音画同步等。未逐语句重建 → `todo!`。
//!
//! ### `impl$3::build` —— 胶水（未实现）
//! 反编译仅一行：`SubApp::add_systems<PostUpdate, EventReader<
//! ApplyVideoSettingsCmd>>()`；系统体 `apply_video_settings_cmd` 属 app_config
//! 且未恢复，注册空体系统会吞事件 → 保持未实现。

use std::sync::Mutex;

use bevy_app::App;
use bevy_ecs::world::World;

use country_core::render::render_config::{MonitorMatch, VideoMode};
use country_core::resources::window::{AvailableMonitor, AvailableMonitors};
use winit::event_loop::EventLoopWindowTarget;

use crate::setup::FromWinit;
use crate::egui_backend::EguiBackend;
use crate::render_loop::{FramePacer, RenderHost, RenderMode};

/// Builds the existing core RenderHost against the world *after* App::update.
/// The host can borrow that exact World and the EguiBackend for this frame.
/// Implementations must supply the actual frame/pass/WSI adapter; this trait
/// does not declare the missing production adapter implemented.
pub trait GameRenderHostFactory {
    type Host<'a>: RenderHost
    where
        Self: 'a;

    fn host<'a>(
        &'a mut self,
        world: &'a mut World,
        egui: &'a mut EguiBackend,
    ) -> Self::Host<'a>;
}

/// B 0x1404de6b4..0x1404de7da. An open, throttleable menu limits
/// positive configured FPS to 60; disabled/NaN limits become 60 in that branch.
/// Profiling bypasses that menu cap. Missing menu/commander resources have
/// different behavior: no menu means no cap, no commander means not profiling.
pub fn frame_target_fps(world: &World) -> f32 {
    use country_core::{game_config::GameConfig, meta_commander::MetaCommander,
        resources::gui::EscMenuState};

    let fps = world.resource::<GameConfig>().video.fps_limit;
    let menu_limits = world.get_resource::<EscMenuState>()
        .is_some_and(|menu| menu.is_open() && menu.can_limit_frame_rate());
    let profiling = world.get_resource::<MetaCommander>()
        .is_some_and(MetaCommander::is_profiling);
    apply_menu_fps_limit(fps, menu_limits, profiling)
}

/// B 0x1404de4f9..0x1404de587 tests the *presence* of MainCamera
/// (native TypeId bba451ee6a08a3ff/8d008d15febcf29d), then doubles that
/// boolean to form render's mode byte. No guessed AppLoadingState tag is used.
pub fn frame_render_mode(world: &World) -> RenderMode {
    if world.contains_resource::<country_core::systems::camera_rig::main_camera::MainCamera>() {
        RenderMode::WorldGpuProfiled
    } else {
        RenderMode::LoadingScreen
    }
}

fn apply_menu_fps_limit(fps: f32, menu_limits: bool, profiling: bool) -> f32 {
    if menu_limits && !profiling {
        if fps > 0.0 { fps.min(60.0) } else { 60.0 }
    } else {
        fps
    }
}

/// The recovered update/render section of main_loop, separated from its still
/// unresolved OS-event/device-recovery section. This owns real Bevy/egui state;
/// it is not a native-layout mirror of the 0x1118-byte GameLoop.
pub struct GameFrameDriver {
    app: App,
    egui: EguiBackend,
    pacer: FramePacer,
    limiter: crate::fps_limiter::FpsLimiter,
}

impl GameFrameDriver {
    pub fn new(mut app: App, context: egui::Context) -> Self {
        // GUI consumers and the backend must see the same Context identity.
        app.world_mut().insert_resource(
            country_core::resources::gui::EguiContextResource(context.clone()),
        );
        Self {
            app,
            egui: EguiBackend::new(context),
            pacer: FramePacer::default(),
            limiter: crate::fps_limiter::FpsLimiter::new(),
        }
    }

    pub fn app(&self) -> &App { &self.app }
    pub fn app_mut(&mut self) -> &mut App { &mut self.app }

    /// B begin_frame@0x1404de115 → App::update@0x1404de349 →
    /// end_frame@0x1404de3d2 → platform output send@0x1404de464 →
    /// render@0x1404de5bc → first-frame log@0x1404de640 → limiter@0x1404de7da.
    ///
    /// input=None follows the original no-egui branch: update/render still run,
    /// without a begin/end pair or a platform message. OS events and Steam
    /// callbacks must be resolved by the full host loop.
    pub fn frame<F: GameRenderHostFactory>(
        &mut self,
        input: Option<egui::RawInput>,
        factory: &mut F,
        publish_platform_output: impl FnOnce(egui::PlatformOutput),
    ) {
        let has_gui_input = input.is_some();
        if let Some(input) = input {
            self.egui.context().begin_frame(input);
        }
        {
            puffin::profile_scope!("App update");
            self.app.update();
        }
        if has_gui_input {
            publish_platform_output(self.egui.end_frame());
        }
        {
            let mode = frame_render_mode(self.app.world());
            let mut host = factory.host(self.app.world_mut(), &mut self.egui);
            crate::render_loop::render(&mut host, &mut self.pacer, mode);
        }
        crate::main::log_time_to_first_frame();
        self.limiter.frame(frame_target_fps(self.app.world()));
    }
}

/// B `tiny_glade::game_loop::GameLoop`（字段布局见模块注释；这里只保留已
/// 明确的启动计时字段，其余以 `todo!` 函数表达，不伪造结构）。
#[derive(Debug)]
pub struct GameLoop {
    /// `+0x10b8`：`Option<Instant>` 首帧计时（`nanos == 1e9` 为 None）；
    /// `log_time_to_first_frame` 消费的全局由同一时刻写入（见 `main.rs`）。
    pub startup_instant: Option<std::time::Instant>,
}

impl GameLoop {
    /// `GameLoop::new` `0x1404d8100` —— 未实现（字段类型未锚定）。
    pub fn new() -> Self {
        todo!(
            "未实现: GameLoop::new 0x1404d8100 —— 纯字段 memcpy 已解，但 GameLoop \
             字段类型/AvailableGraphicsDevices 等持有类型未落库"
        );
    }

    /// `GameLoop::run` `0x1404d8250` —— 未实现。
    pub fn run(&mut self) {
        todo!(
            "未实现: GameLoop::run 0x1404d8250 —— main_loop + pending fullscreen + \
             shut_down_everything 顺序已解，依赖 GameLoop 字段/资源面"
        )
    }

    /// `GameLoop::main_loop` `0x1404d9580` —— 未实现。
    pub fn main_loop(&mut self) {
        todo!("未实现: GameLoop::main_loop 0x1404d9580 —— 2,564 行 winit/Bevy 事件泵体")
    }

    /// `GameLoop::extract_available_monitors_info` `0x1404e2470` —— 重建。
    ///
    /// 原为 Bevy 系统（`ResMut<AvailableMonitors>` + 事件循环资源）；本合同把
    /// 两个输入显式传入，保留逐语句控制流。
    pub fn extract_available_monitors_info(
        available: &mut AvailableMonitors,
        event_loop: &EventLoopWindowTarget<()>,
    ) {
        available.clear();
        for handle in event_loop.available_monitors() {
            let monitor = MonitorMatch::from_winit(&handle);
            let mut modes: Vec<winit::monitor::VideoMode> = handle.video_modes().collect();
            // E2：B 的第二次 sort（方向未锚定），按 BTreeSet Ord 顺序保留。
            if modes.len() > 1 {
                modes.sort_by(|a, b| b.cmp(a));
            }
            let video_modes: Vec<VideoMode> =
                modes.iter().map(VideoMode::from_winit).collect();
            available.monitors.push(AvailableMonitor {
                video_modes,
                monitor,
            });
        }
    }
}

impl Default for GameLoop {
    fn default() -> Self {
        Self {
            startup_instant: None,
        }
    }
}

/// `gather_pending_video_setting_changes` 的队列元素（B item stride 0x10，
/// 标志字节 @+8；前 8 字节为命令载荷）。
#[derive(Debug)]
pub struct VideoSettingChange {
    /// `ApplyVideoSettingsCmd`（B 事件；字段未知，见 app_config）。
    pub command: crate::app_config::ApplyVideoSettingsCmd,
    /// `+0x8`：按位或进全局标志的字节。
    pub flags: u8,
}

/// `PendingVideoSettingChanges` 的队列段。
///
/// B 布局（0x1404e3380）：`ptr@+8`、`end(len)@+0x10`、`start@+0x18`
/// （第二段 +0x28/+0x30/+0x38，总计 `total@+0x40`）。`start` 是该段在全局
/// 序号中的起始位置。
#[derive(Debug, Default)]
pub struct PendingVideoSettingChanges {
    /// 已消费的全局序号（解引用后：`*cursor`）。
    pub consumed: usize,
    pub first: Vec<VideoSettingChange>,
    pub first_start: usize,
    pub second: Vec<VideoSettingChange>,
    pub second_start: usize,
    pub total: usize,
}

/// 全局待处理视频设置标志（B：std `Mutex<u8>` 静态；state/poison/data 三字节）。
pub static PENDING_VIDEO_SETTING_FLAGS: Mutex<u8> = Mutex::new(0);

/// `gather_pending_video_setting_changes` `0x1404e3380` —— 重建。
///
/// 控制流逐语句对应 fresh 反编译（`wrapping_*` 保留 release 语义的无符号回绕，
/// 与原机器码一致；锁的 poison→panic 与 unlock 时的 wake 由 std Mutex 提供）。
pub fn gather_pending_video_setting_changes(changes: &mut PendingVideoSettingChanges) {
    let start = changes.consumed;

    let off1 = start.saturating_sub(changes.first_start);
    let off2 = start.saturating_sub(changes.second_start);

    let len1_total = changes.first.len();
    let len2_total = changes.second.len();

    let (first, rem1) = if len1_total < off1 {
        (&[][..], 0)
    } else {
        (&changes.first[off1..], len1_total - off1)
    };
    let (second, rem2) = if len2_total < off2 {
        (&[][..], 0)
    } else {
        (&changes.second[off2..], len2_total - off2)
    };

    changes.consumed = changes
        .total
        .wrapping_sub(rem2)
        .wrapping_sub(rem1);

    for item in first.iter().chain(second.iter()) {
        changes.consumed = changes.consumed.wrapping_add(1);
        let mut flags = PENDING_VIDEO_SETTING_FLAGS.lock().unwrap();
        *flags |= item.flags;
    }
}

/// `tiny_glade::game_loop::impl$3::build` `0x1404e3370` —— 胶水（未实现）。
///
/// 反编译仅 `add_systems(PostUpdate, EventReader<ApplyVideoSettingsCmd>)`；
/// 被注册的系统体属 app_config，尚未恢复。
pub fn register_settings_application_system() {
    todo!("未实现: GameLoopPlugin::build 0x1404e3370 —— apply_video_settings_cmd 系统体未恢复")
}

#[cfg(test)]
mod frame_tests {
    use super::*;
    use country_core::{game_config::GameConfig, resources::gui::EscMenuState};

    #[test]
    fn menu_cap_preserves_normal_game_limits_and_handles_disabled_or_nan_limits() {
        for fps in [0.0, -0.0, -1.0, 30.0, 60.0, 144.0, f32::INFINITY, f32::NAN] {
            assert_eq!(apply_menu_fps_limit(fps, false, false).to_bits(), fps.to_bits());
            assert_eq!(apply_menu_fps_limit(fps, true, true).to_bits(), fps.to_bits());
            let expected = if fps > 0.0 && fps < 60.0 { fps } else { 60.0 };
            assert_eq!(apply_menu_fps_limit(fps, true, false), expected);
        }
    }

    #[test]
    fn menu_cap_reads_the_current_world_resources() {
        let mut world = World::new();
        let mut config = GameConfig::default();
        config.video.fps_limit = 144.0;
        world.insert_resource(config);
        assert_eq!(frame_target_fps(&world), 144.0);
        world.insert_resource(EscMenuState::Main);
        assert_eq!(frame_target_fps(&world), 60.0);
        world.insert_resource(EscMenuState::Closed);
        assert_eq!(frame_target_fps(&world), 144.0);
        world.resource_mut::<GameConfig>().video.fps_limit = 30.0;
        world.insert_resource(EscMenuState::Main);
        assert_eq!(frame_target_fps(&world), 30.0);
    }

    #[test]
    fn camera_resource_presence_selects_the_native_frame_mode() {
        let mut world = World::new();
        assert_eq!(frame_render_mode(&world), RenderMode::LoadingScreen);
        world.insert_resource(country_core::systems::camera_rig::main_camera::MainCamera::default());
        assert_eq!(frame_render_mode(&world), RenderMode::WorldGpuProfiled);
        world.remove_resource::<country_core::systems::camera_rig::main_camera::MainCamera>();
        assert_eq!(frame_render_mode(&world), RenderMode::LoadingScreen);
    }
}
