//! B OS-event producer and game-loop platform adapter.
//!
//! Source evidence: run_event_loop 0x14026e4a0, the inlined game callback at
//! 0x140557b50, and original RNE jump tables. The semantic event names below
//! are not a portable reconstruction of Windows enum ABI. Original branch
//! contracts and unresolved consumers are retained in the task-index work pack.
//! macOS uses EventLoop::run on the main thread; this is explicitly an OS
//! adaptation of the recovered producer/consumer logic.

use std::sync::Mutex;

/// `tiny_glade::event_loop::EguiEventIo`（B 类型；外壳布局未锚定，见 F1）。
pub struct EguiEventIo {
    state: Mutex<egui_winit::State>,
}

impl EguiEventIo {
    /// `EguiEventIo::new` `0x14026de50` —— 重建（包装 `egui_winit::State`）。
    pub fn new(state: egui_winit::State) -> Self {
        Self {
            state: Mutex::new(state),
        }
    }

    /// `EguiEventIo::take_egui_input` `0x14026dfd0` —— 胶水（锁 + puffin scope + 转发）。
    pub fn take_egui_input(&self, window: &winit::window::Window) -> egui::RawInput {
        puffin::profile_function!();
        let mut state = self.state.lock().unwrap();
        state.take_egui_input(window)
    }

    /// Feed window input into the same locked egui state that take_egui_input
    /// drains. This forwards the pinned egui-winit API rather than rebuilding it.
    pub fn on_window_event(
        &self,
        window: &winit::window::Window,
        event: &winit::event::WindowEvent,
    ) -> egui_winit::EventResponse {
        self.state.lock().unwrap().on_window_event(window, event)
    }

    /// B run_event_loop's platform-output branch calls this under the same
    /// mutex (0x14026e4a0, retained decomp line 498). Invoke on the OS thread.
    pub fn handle_platform_output(
        &self,
        window: &winit::window::Window,
        output: egui::PlatformOutput,
    ) {
        self.state
            .lock()
            .unwrap()
            .handle_platform_output(window, output);
    }
}

/// Semantic projection of the original game-bound variants. These names are
/// descriptive; this enum deliberately makes no claim about the Windows ABI.
#[derive(Debug)]
pub enum OsWindowEvent {
    Resized([u32; 2]),
    MonitorChanged,
    SurfaceStale,
    CloseRequested,
    ToggleFullscreen,
    Focused(bool),
    Keyboard(winit::event::KeyEvent),
    CursorAbsolute([f64; 2]),
    CursorRelative([f32; 2]),
    MouseButton(winit::event::ElementState, winit::event::MouseButton),
    CursorConfirmed(u64),
    LineMouseWheel([f32; 2]),
}

/// What the frame/event code needs from the OS loop. Implemented by winit's
/// window target on the main thread, and by [`ThreadTarget`] when the game
/// runs on its own thread and only forwarded events reach it.
pub trait LoopTarget {
    fn exit(&self);
    fn set_control_flow(&self, flow: winit::event_loop::ControlFlow);
    fn available_monitors(&self) -> Vec<winit::monitor::MonitorHandle>;
}

impl LoopTarget for winit::event_loop::EventLoopWindowTarget<()> {
    fn exit(&self) {
        winit::event_loop::EventLoopWindowTarget::exit(self)
    }
    fn set_control_flow(&self, flow: winit::event_loop::ControlFlow) {
        winit::event_loop::EventLoopWindowTarget::set_control_flow(self, flow)
    }
    fn available_monitors(&self) -> Vec<winit::monitor::MonitorHandle> {
        winit::event_loop::EventLoopWindowTarget::available_monitors(self).collect()
    }
}

/// Game-thread view of the OS loop. `exit` raises a flag and wakes the main
/// thread (a redraw request is an event it must handle), which then leaves its
/// loop. Control flow is the main thread's business and is ignored here.
pub(crate) struct ThreadTarget {
    window: std::sync::Arc<winit::window::Window>,
    exit: std::sync::Arc<std::sync::atomic::AtomicBool>,
}

impl LoopTarget for ThreadTarget {
    fn exit(&self) {
        self.exit.store(true, std::sync::atomic::Ordering::Release);
        self.window.request_redraw();
    }
    fn set_control_flow(&self, _flow: winit::event_loop::ControlFlow) {}
    fn available_monitors(&self) -> Vec<winit::monitor::MonitorHandle> {
        self.window.available_monitors().collect()
    }
}

/// Runs the actual winit loop on the OS main thread. B used pump_events on the
/// OS thread and a separate game thread. Main-thread rendering here is an
/// explicit macOS host adaptation, not recovered B thread scheduling.
pub fn run_event_loop(
    event_loop: winit::event_loop::EventLoop<()>,
    window: std::sync::Arc<winit::window::Window>,
    app: bevy_app::App,
    options: crate::game_loop::GameLoopOptions,
    shaders: std::sync::Arc<country_core::asset_libraries::shader_library::AssetShaderLibrary>,
) -> anyhow::Result<()> {
    crate::game_loop::GameLoop::new(app, window, options, shaders)?.run(event_loop)
}

/// `TG_GAME_THREAD=1` runs the game (update, render, present) on its own thread
/// while the OS main thread only pumps window events, like the original. Off by
/// default: window handling off the main thread is only exercised on X11, and
/// platforms such as macOS require main-thread window calls.
pub fn game_thread_enabled() -> bool {
    std::env::var_os("TG_GAME_THREAD").is_some_and(|value| value == "1")
}

/// Threaded variant of [`run_event_loop`]. `build` runs on the game thread and
/// creates every `!Send` object there (the `App`, renderer and `GameLoop`); only
/// forwarded winit events cross the channel. The first error from either side is
/// returned after the game thread has been joined.
pub fn run_event_loop_threaded(
    event_loop: winit::event_loop::EventLoop<()>,
    window: std::sync::Arc<winit::window::Window>,
    build: impl FnOnce(std::sync::Arc<winit::window::Window>) -> anyhow::Result<crate::game_loop::GameLoop>
        + Send
        + 'static,
) -> anyhow::Result<()> {
    use std::sync::{Arc, atomic::{AtomicBool, Ordering}, mpsc};
    use winit::event::{DeviceEvent, Event, WindowEvent};
    let (sender, receiver) = mpsc::channel::<Event<()>>();
    let exit = Arc::new(AtomicBool::new(false));
    let handle = std::thread::Builder::new()
        .name("tiny-glade game".into())
        .spawn({
            let window = window.clone();
            let exit = exit.clone();
            move || -> anyhow::Result<()> {
                let target = ThreadTarget { window: window.clone(), exit: exit.clone() };
                // However this thread ends, wake the main loop so it can leave.
                struct WakeMain<'a>(&'a ThreadTarget);
                impl Drop for WakeMain<'_> {
                    fn drop(&mut self) {
                        LoopTarget::exit(self.0);
                    }
                }
                let _wake = WakeMain(&target);
                let mut game = build(window)?;
                game.run_on_game_thread(receiver, &target)
            }
        })?;
    let mut handle = Some(handle);
    let game_result: Arc<std::sync::Mutex<Option<anyhow::Result<()>>>> = Arc::default();
    let result_slot = game_result.clone();
    let loop_result = event_loop.run(move |event, target| {
        target.set_control_flow(winit::event_loop::ControlFlow::Wait);
        if exit.load(Ordering::Acquire) {
            target.exit();
        }
        match event {
            // Frames are paced by the game thread, never by OS redraw requests.
            Event::WindowEvent { event: WindowEvent::RedrawRequested, .. } => {}
            Event::WindowEvent { .. }
            | Event::DeviceEvent { event: DeviceEvent::MouseMotion { .. }, .. }
            | Event::Suspended
            | Event::Resumed => {
                let _ = sender.send(event);
            }
            Event::LoopExiting => {
                let _ = sender.send(event);
                if let Some(handle) = handle.take() {
                    *result_slot.lock().unwrap() = Some(handle.join().unwrap_or_else(|panic| {
                        Err(anyhow::anyhow!("game thread panicked: {panic:?}"))
                    }));
                }
            }
            _ => {}
        }
    });
    let finished = game_result.lock().unwrap().take();
    match finished {
        Some(Err(error)) => Err(error),
        _ => loop_result.map_err(Into::into),
    }
}

impl crate::game_loop::GameLoop {
    /// Body of the game thread: the same event handling and frame code as the
    /// main-thread loop, driven by forwarded events plus a frame every iteration
    /// (the `RedrawRequested` the OS loop would otherwise request).
    pub(crate) fn run_on_game_thread(
        &mut self,
        events: std::sync::mpsc::Receiver<winit::event::Event<()>>,
        target: &ThreadTarget,
    ) -> anyhow::Result<()> {
        use std::sync::mpsc::RecvTimeoutError;
        use winit::event::{Event, WindowEvent};
        let lifecycle = self.lifecycle.clone();
        self.refresh_monitors(target);
        'run: loop {
            let background_pending = crate::loading_graph::is_background_preparation_pending(
                self.driver.app().world(),
            );
            // Suspended: nothing to render, block for the next OS event. Background
            // loading: at most one frame per display tick, woken early by events.
            // Otherwise poll, so frames are only paced by the limiter and present.
            let first = if self.suspended {
                events.recv().map_err(|_| RecvTimeoutError::Disconnected)
            } else {
                let wait = if background_pending {
                    std::time::Duration::from_millis(16)
                } else {
                    std::time::Duration::ZERO
                };
                events.recv_timeout(wait)
            };
            let mut batch = Vec::new();
            match first {
                Ok(event) => batch.push(event),
                Err(RecvTimeoutError::Timeout) => {}
                Err(RecvTimeoutError::Disconnected) => break 'run,
            }
            batch.extend(events.try_iter());
            let mut loop_exiting = false;
            for event in batch {
                loop_exiting |= matches!(event, Event::LoopExiting);
                if let Err(error) = self.main_loop(event, target) {
                    lifecycle.record_failure(error);
                    if let Err(shutdown) = self.shut_down() {
                        lifecycle.record_failure(shutdown);
                    }
                    LoopTarget::exit(target);
                    break 'run;
                }
            }
            if loop_exiting || !lifecycle.is_running() {
                break 'run;
            }
            if !self.suspended {
                let redraw = Event::WindowEvent {
                    window_id: self.window.id(),
                    event: WindowEvent::RedrawRequested,
                };
                if let Err(error) = self.main_loop(redraw, target) {
                    lifecycle.record_failure(error);
                    if let Err(shutdown) = self.shut_down() {
                        lifecycle.record_failure(shutdown);
                    }
                    LoopTarget::exit(target);
                    break 'run;
                }
                if !lifecycle.is_running() {
                    break 'run;
                }
            }
        }
        match lifecycle.take_failure() {
            Some(error) => Err(error),
            None => Ok(()),
        }
    }

    pub fn run(mut self, event_loop: winit::event_loop::EventLoop<()>) -> anyhow::Result<()> {
        let lifecycle = self.lifecycle.clone();
        self.refresh_monitors(&*event_loop);
        let loop_result = event_loop.run(move |event, target| {
            if let Err(error) = self.main_loop(event, target) {
                self.lifecycle.record_failure(error);
                if let Err(shutdown) = self.shut_down() {
                    self.lifecycle.record_failure(shutdown);
                }
                target.exit();
            }
        });
        let error = lifecycle.take_failure();
        match error {
            Some(error) => Err(error),
            None => loop_result.map_err(Into::into),
        }
    }

    /// The platform dispatcher feeds only the branches anchored in B's callback.
    /// Suspended/Resumed are OS/WSI adaptation, separately marked below.
    pub fn main_loop(
        &mut self,
        event: winit::event::Event<()>,
        target: &dyn LoopTarget,
    ) -> anyhow::Result<()> {
        if !self.lifecycle.is_running() {
            // exit() can leave already queued Redraw/Resumed/AboutToWait events.
            // They must not touch egui, the App, cursor mappings or the renderer.
            target.exit();
            return if matches!(event, winit::event::Event::LoopExiting) {
                self.shut_down()
            } else {
                Ok(())
            };
        }
        let lifecycle = self.lifecycle.clone();
        lifecycle.dispatch(|| self.dispatch_running_event(event, target))
    }

    fn dispatch_running_event(
        &mut self,
        event: winit::event::Event<()>,
        target: &dyn LoopTarget,
    ) -> anyhow::Result<()> {
        use winit::event::{Event, WindowEvent};
        match event {
            Event::WindowEvent { window_id, event } if window_id == self.window.id() => {
                if matches!(event, WindowEvent::RedrawRequested) {
                    if !self.suspended {
                        let previous_frame_count = self.frame_count;
                        let running = self.frame(target)?;
                        if self.renderer.any_present_successful() || self.frame_count > 1 {
                            self.window.set_visible(true);
                        }
                        if self.frame_count != previous_frame_count && self.frame_count % 30 == 0 {
                            log::info!(
                                "production frame progress: frame_count={} {:?}",
                                self.frame_count,
                                self.frame_progress()
                            );
                        }
                        let bounded_run_finished = self
                            .options
                            .frame_limit
                            .is_some_and(|limit| self.frame_count >= limit);
                        if !running || bounded_run_finished {
                            log::info!(
                                "production frame progress at exit: {:?}",
                                self.frame_progress()
                            );
                            self.shut_down()?;
                            target.exit();
                        }
                    }
                    return Ok(());
                }
                // B callback deliberately bypasses egui for physical Tab, leaving
                // navigation to Mei (physical key byte 0x3f at 0x140557b50).
                let bypass_egui = matches!(&event, WindowEvent::KeyboardInput { event, .. }
                    if event.physical_key == winit::keyboard::PhysicalKey::Code(winit::keyboard::KeyCode::Tab));
                if !bypass_egui && self.egui_io.on_window_event(&self.window, &event).consumed {
                    return Ok(());
                }
                match event {
                    WindowEvent::Resized(size) => {
                        self.queue_os_event(OsWindowEvent::Resized([size.width, size.height]))?
                    }
                    WindowEvent::Moved(_) => self.queue_os_event(OsWindowEvent::MonitorChanged)?,
                    WindowEvent::CloseRequested => {
                        if self.suspended || crate::loading_graph::is_background_preparation_pending(self.driver.app().world()) {
                            // 暂停时没有下一次 frame 消费事件队列；实际 OS 关闭必须
                            // 直接取消并 join 加载任务、关闭保存和 GPU，不能等待 Resumed。
                            // 这是主线程宿主适配，不运行暂停中的 App 或伪造关闭标记。
                            self.dispatch_os_event(OsWindowEvent::CloseRequested, target)?;
                            self.shut_down()?;
                            target.exit();
                        } else {
                            self.queue_os_event(OsWindowEvent::CloseRequested)?;
                        }
                    }
                    WindowEvent::Focused(focused) => {
                        self.queue_os_event(OsWindowEvent::Focused(focused))?
                    }
                    WindowEvent::ModifiersChanged(modifiers) => self.modifiers = modifiers.state(),
                    WindowEvent::KeyboardInput {
                        event,
                        is_synthetic,
                        ..
                    } => {
                        if is_synthetic && event.state == winit::event::ElementState::Pressed {
                            return Ok(());
                        }
                        let code = event.physical_key;
                        if event.state == winit::event::ElementState::Released
                            && (code
                                == winit::keyboard::PhysicalKey::Code(
                                    winit::keyboard::KeyCode::F11,
                                )
                                || code
                                    == winit::keyboard::PhysicalKey::Code(
                                        winit::keyboard::KeyCode::Enter,
                                    )
                                    && self.modifiers.alt_key())
                        {
                            self.queue_os_event(OsWindowEvent::ToggleFullscreen)?;
                        } else {
                            self.queue_os_event(OsWindowEvent::Keyboard(event))?;
                        }
                    }
                    WindowEvent::CursorMoved { position, .. } => {
                        if self.capture_uses_raw_motion {
                            return Ok(());
                        }
                        if let Some(anchor) = self.capture_anchor {
                            self.window.set_cursor_position(anchor)?;
                            let delta = if std::mem::take(&mut self.capture_needs_initial_event) {
                                [0.0, 0.0]
                            } else {
                                [
                                    (position.x - anchor.x) as f32,
                                    (position.y - anchor.y) as f32,
                                ]
                            };
                            self.queue_os_event(OsWindowEvent::CursorRelative(delta))?;
                        } else {
                            self.queue_os_event(OsWindowEvent::CursorAbsolute([
                                position.x, position.y,
                            ]))?;
                        }
                    }
                    WindowEvent::MouseInput { state, button, .. } => {
                        self.queue_os_event(OsWindowEvent::MouseButton(state, button))?
                    }
                    // B forwards the LineDelta variant (two f32s) and unit=0;
                    // the PixelDelta variant does not enter this producer path.
                    WindowEvent::MouseWheel {
                        delta: winit::event::MouseScrollDelta::LineDelta(x, y),
                        ..
                    } => self.queue_os_event(OsWindowEvent::LineMouseWheel([x, y]))?,
                    WindowEvent::ScaleFactorChanged { .. } => {
                        let size = self.window.inner_size();
                        self.queue_os_event(OsWindowEvent::Resized([size.width, size.height]))?;
                    }
                    _ => {}
                }
            }
            Event::DeviceEvent {
                event: winit::event::DeviceEvent::MouseMotion { delta },
                ..
            } if self.capture_uses_raw_motion && self.window.has_focus() => {
                // Explicit OS adaptation: macOS Locked cursor does not produce
                // the Windows warp-and-CursorMoved protocol. Feed its real raw
                // deltas into the same recovered relative-motion consumer.
                self.queue_os_event(OsWindowEvent::CursorRelative([
                    delta.0 as f32,
                    delta.1 as f32,
                ]))?;
            }
            Event::AboutToWait => {
                // OS main-thread adapter: redraw drives the same recovered
                // update/egui/render consumer, without a polling worker thread.
                let background_pending = crate::loading_graph::is_background_preparation_pending(
                    self.driver.app().world(),
                );
                if !self.suspended && background_pending {
                    // A bounded CPU loader has no native event proxy. Poll at
                    // most once per display tick while allowing OS input/close
                    // to wake the loop immediately; never spin empty redraws.
                    let deadline = *self.background_poll_deadline.get_or_insert_with(||
                        std::time::Instant::now() + std::time::Duration::from_millis(16));
                    target.set_control_flow(winit::event_loop::ControlFlow::WaitUntil(deadline));
                } else {
                    self.background_poll_deadline = None;
                    target.set_control_flow(winit::event_loop::ControlFlow::Wait);
                }
                if !self.suspended && !background_pending {
                    self.window.request_redraw();
                }
            }
            Event::NewEvents(winit::event::StartCause::ResumeTimeReached { .. }) => {
                self.background_poll_deadline = None;
                if !self.suspended {
                    self.window.request_redraw();
                }
            }
            Event::Suspended => {
                // Platform notification, not a reconstructed B gameplay event.
                self.suspended = true;
                self.renderer.suspend()?;
            }
            Event::Resumed => {
                self.renderer.resume()?;
                self.suspended = false;
                self.window.request_redraw();
            }
            Event::LoopExiting => {
                self.shut_down()?;
            }
            _ => {}
        }
        Ok(())
    }

    /// OS producer: retain the batch independently of either App. The mapped
    /// absolute cursor write remains in this callback before confirmation gates.
    pub(crate) fn queue_os_event(&mut self, event: OsWindowEvent) -> anyhow::Result<()> {
        if !self.lifecycle.is_running() {
            return Ok(());
        }
        if let OsWindowEvent::CursorAbsolute(position) = &event {
            self.update_custom_cursor_absolute(*position)?;
        }
        self.pending_os_events.push_back(event);
        Ok(())
    }

    pub(crate) fn dispatch_os_event(
        &mut self,
        event: OsWindowEvent,
        target: &dyn LoopTarget,
    ) -> anyhow::Result<()> {
        if !self.lifecycle.is_running() {
            return Ok(());
        }
        use bevy_ecs::entity::Entity;
        use bevy_input::{
            ButtonState,
            keyboard::KeyboardInput,
            mouse::{MouseButtonInput, MouseScrollUnit, MouseWheel},
        };
        use country_core::{
            game_config::GameConfig, resources::window::*, systems::backend::input,
        };
        match event {
            OsWindowEvent::Resized([width, height]) if width < 20_000 => {
                self.app_mut()
                    .world_mut()
                    .resource_mut::<WindowSize>()
                    .set(width, height);
                self.app_mut()
                    .world_mut()
                    .resource_mut::<GameConfig>()
                    .video
                    .display
                    .windowed_resolution = [width, height];
                // `FrameRenderConfig`'s three packed extents are "copied from WindowSize" at startup
                // (render_config.rs). The tail's display target reads `extent_8`, so a resized
                // window would keep presenting the old extent. HOST ADAPTATION (I): the original's
                // per-resize update point is not located; keep the copy semantics on resize.
                if let Some(mut frame) = self
                    .app_mut()
                    .world_mut()
                    .get_resource_mut::<country_core::render::render_config::FrameRenderConfig>()
                {
                    frame.extent_0 = [width, height];
                    frame.extent_8 = [width, height];
                    frame.extent_10 = [width, height];
                }
                self.renderer.resize([width, height])?;
            }
            OsWindowEvent::Resized(_) => {}
            OsWindowEvent::MonitorChanged => {
                self.refresh_monitors(target);
                let monitor = self
                    .window
                    .current_monitor()
                    .as_ref()
                    .map(crate::setup::FromWinit::from_winit);
                if self
                    .app()
                    .world()
                    .resource::<GameConfig>()
                    .video
                    .display
                    .monitor
                    == monitor
                {
                    return Ok(());
                }
                self.app_mut()
                    .world_mut()
                    .resource_mut::<GameConfig>()
                    .video
                    .display
                    .monitor = monitor;
                self.app_mut()
                    .world_mut()
                    .send_event(crate::app_config::WriteSettingsFileCmd);
            }
            OsWindowEvent::SurfaceStale => {
                let size = self.window.inner_size();
                self.renderer.resize([size.width, size.height])?;
            }
            OsWindowEvent::CloseRequested => {
                self.app_mut()
                    .world_mut()
                    .send_event(country_core::resources::events::ShutdownGameCmd);
                // Dispatch after a possible handoff so the current App owns
                // the real command. Its normal Update consumer owns the marker.
                self.window.request_redraw();
            }
            OsWindowEvent::ToggleFullscreen => {
                if self.app().world().get_resource::<crate::app_config::LaunchDisplayOverride>()
                    .is_some_and(|policy| policy.force_windowed)
                {
                    return Ok(());
                }
                let mut config = self.app_mut().world_mut().resource_mut::<GameConfig>();
                config.video.display.fullscreen = !config.video.display.fullscreen;
                *crate::game_loop::PENDING_VIDEO_SETTING_FLAGS
                    .lock()
                    .unwrap() |= 1;
            }
            OsWindowEvent::Focused(focused) => {
                self.renderer.set_focused(focused)?;
                self.app_mut()
                    .world_mut()
                    .send_event(WindowFocusEvent(u8::from(focused)));
            }
            OsWindowEvent::Keyboard(event) => {
                let key_code = crate::winit_bevy_glue::winit_to_bevy_keycode(&event);
                let state = if event.state == winit::event::ElementState::Pressed {
                    ButtonState::Pressed
                } else {
                    ButtonState::Released
                };
                let logical_key = logical_key_to_bevy(&event.logical_key);
                let keyboard_event = KeyboardInput {
                    key_code,
                    logical_key,
                    state,
                    text: event.text.as_ref().map(|s| s.as_str().into()),
                    repeat: event.repeat,
                    window: Entity::PLACEHOLDER,
                };
                // 原始事件交给 Bevy；核心的数字键码资源复用同一张物理键映射表。
                if let winit::keyboard::PhysicalKey::Code(code) = event.physical_key {
                    let code = code as u16 + 1;
                    if let Some(mut keys) =
                        self.app_mut()
                            .world_mut()
                            .get_resource_mut::<input::button_input::KeyButtonInput>()
                    {
                        if state == ButtonState::Pressed {
                            keys.press(code);
                        } else {
                            keys.release(code);
                        }
                    }
                }
                self.app_mut().world_mut().send_event(keyboard_event);
            }
            OsWindowEvent::CursorAbsolute(position) => {
                // The OS producer already wrote the mapped SSBO before this
                // batch was captured. Only the game event is confirmation-gated.
                if self.cursor_requests.is_fully_confirmed() {
                    self.app_mut().world_mut().send_event(
                        input::translate_input_to_cursor_moved::InputCursorMoved::Absolute(
                            [position[0] as f32, position[1] as f32].into(),
                        ),
                    );
                }
            }
            OsWindowEvent::CursorRelative(delta) => {
                self.app_mut().world_mut().send_event(
                    input::translate_input_to_cursor_moved::InputCursorMoved::Relative(
                        delta.into(),
                    ),
                );
            }
            OsWindowEvent::MouseButton(state, button) => {
                let (button, raw) = match button {
                    winit::event::MouseButton::Left => (bevy_input::mouse::MouseButton::Left, 0),
                    winit::event::MouseButton::Right => (bevy_input::mouse::MouseButton::Right, 1),
                    winit::event::MouseButton::Middle => {
                        (bevy_input::mouse::MouseButton::Middle, 2)
                    }
                    winit::event::MouseButton::Back => (bevy_input::mouse::MouseButton::Back, 3),
                    winit::event::MouseButton::Forward => {
                        (bevy_input::mouse::MouseButton::Forward, 4)
                    }
                    winit::event::MouseButton::Other(n) => {
                        (bevy_input::mouse::MouseButton::Other(n), n)
                    }
                };
                let state = if state == winit::event::ElementState::Pressed {
                    ButtonState::Pressed
                } else {
                    ButtonState::Released
                };
                if let Some(mut mouse) =
                    self.app_mut()
                        .world_mut()
                        .get_resource_mut::<input::button_input::MouseButtonInput>()
                {
                    if state == ButtonState::Pressed {
                        mouse.press(raw);
                    } else {
                        mouse.release(raw);
                    }
                }
                self.app_mut().world_mut().send_event(MouseButtonInput {
                    button,
                    state,
                    window: Entity::PLACEHOLDER,
                });
            }
            OsWindowEvent::CursorConfirmed(id) => self.cursor_requests.on_confirmation(id),
            OsWindowEvent::LineMouseWheel([x, y]) => {
                self.app_mut().world_mut().send_event(MouseWheel {
                    unit: MouseScrollUnit::Line,
                    x,
                    y,
                    window: Entity::PLACEHOLDER,
                });
                self.app_mut()
                    .world_mut()
                    .send_event(input::translate_input::MouseWheel { unit: 0, x, y });
            }
        }
        Ok(())
    }

    fn refresh_monitors(&mut self, target: &dyn LoopTarget) {
        let mut monitors = country_core::resources::window::AvailableMonitors::default();
        Self::extract_monitors_from_target(&mut monitors, target);
        self.app_mut().world_mut().insert_resource(monitors);
    }
}

/// B 0x14026ea28's display branch expressed through pinned winit APIs. The
/// original `try_set_fullscreen` fork returns Result; upstream setter does not.
pub(crate) fn apply_display_config(
    window: &winit::window::Window,
    display: &country_core::render::render_config::DisplayConfig,
) -> anyhow::Result<()> {
    use crate::setup::FromWinit;
    use country_core::render::render_config::{FullscreenConfig, MonitorMatch};
    use winit::window::Fullscreen;
    let monitor = window
        .available_monitors()
        .find(|handle| {
            display
                .monitor
                .as_ref()
                .is_some_and(|desired| &MonitorMatch::from_winit(handle) == desired)
        })
        .or_else(|| window.current_monitor());
    if !display.fullscreen {
        window.set_fullscreen(None);
        let [width, height] = display.windowed_resolution;
        let _ = window.request_inner_size(winit::dpi::PhysicalSize::new(width, height));
    } else if display.allow_exclusive_fullscreen
        && matches!(
            display.fullscreen_config,
            FullscreenConfig::Exclusive { .. }
        )
    {
        if let Some(monitor) = monitor {
            // Reuse the recovered desktop-mode selection. Do not invent a
            // bit depth/refresh rate to fill the original Windows VideoMode.
            let mut mode = None;
            if let Some(mode) = crate::setup::find_best_fullscreen_video_mode(&monitor, &mut mode) {
                window.set_fullscreen(Some(Fullscreen::Exclusive(mode)));
            } else {
                window.set_fullscreen(Some(Fullscreen::Borderless(Some(monitor))));
            }
        }
    } else {
        window.set_fullscreen(Some(Fullscreen::Borderless(monitor)));
    }
    Ok(())
}

// Exhaustive dependency-variant forwarding. The game-facing physical key map
// remains the recovered winit_bevy_glue function; no new input algorithm lives
// in this conversion. Unmatched named keys retain the dependency's unknown key.
fn logical_key_to_bevy(key: &winit::keyboard::Key) -> bevy_input::keyboard::Key {
    use bevy_input::keyboard::{Key, NativeKey};
    match key {
        winit::keyboard::Key::Character(text) => Key::Character(text.as_str().into()),
        winit::keyboard::Key::Dead(value) => Key::Dead(*value),
        winit::keyboard::Key::Named(named) => match named {
            winit::keyboard::NamedKey::Alt => Key::Alt,
            winit::keyboard::NamedKey::AltGraph => Key::AltGraph,
            winit::keyboard::NamedKey::CapsLock => Key::CapsLock,
            winit::keyboard::NamedKey::Control => Key::Control,
            winit::keyboard::NamedKey::Fn => Key::Fn,
            winit::keyboard::NamedKey::FnLock => Key::FnLock,
            winit::keyboard::NamedKey::NumLock => Key::NumLock,
            winit::keyboard::NamedKey::ScrollLock => Key::ScrollLock,
            winit::keyboard::NamedKey::Shift => Key::Shift,
            winit::keyboard::NamedKey::Symbol => Key::Symbol,
            winit::keyboard::NamedKey::SymbolLock => Key::SymbolLock,
            winit::keyboard::NamedKey::Meta => Key::Meta,
            winit::keyboard::NamedKey::Hyper => Key::Hyper,
            winit::keyboard::NamedKey::Super => Key::Super,
            winit::keyboard::NamedKey::Enter => Key::Enter,
            winit::keyboard::NamedKey::Tab => Key::Tab,
            winit::keyboard::NamedKey::Space => Key::Space,
            winit::keyboard::NamedKey::ArrowDown => Key::ArrowDown,
            winit::keyboard::NamedKey::ArrowLeft => Key::ArrowLeft,
            winit::keyboard::NamedKey::ArrowRight => Key::ArrowRight,
            winit::keyboard::NamedKey::ArrowUp => Key::ArrowUp,
            winit::keyboard::NamedKey::End => Key::End,
            winit::keyboard::NamedKey::Home => Key::Home,
            winit::keyboard::NamedKey::PageDown => Key::PageDown,
            winit::keyboard::NamedKey::PageUp => Key::PageUp,
            winit::keyboard::NamedKey::Backspace => Key::Backspace,
            winit::keyboard::NamedKey::Clear => Key::Clear,
            winit::keyboard::NamedKey::Copy => Key::Copy,
            winit::keyboard::NamedKey::CrSel => Key::CrSel,
            winit::keyboard::NamedKey::Cut => Key::Cut,
            winit::keyboard::NamedKey::Delete => Key::Delete,
            winit::keyboard::NamedKey::EraseEof => Key::EraseEof,
            winit::keyboard::NamedKey::ExSel => Key::ExSel,
            winit::keyboard::NamedKey::Insert => Key::Insert,
            winit::keyboard::NamedKey::Paste => Key::Paste,
            winit::keyboard::NamedKey::Redo => Key::Redo,
            winit::keyboard::NamedKey::Undo => Key::Undo,
            winit::keyboard::NamedKey::Accept => Key::Accept,
            winit::keyboard::NamedKey::Again => Key::Again,
            winit::keyboard::NamedKey::Attn => Key::Attn,
            winit::keyboard::NamedKey::Cancel => Key::Cancel,
            winit::keyboard::NamedKey::ContextMenu => Key::ContextMenu,
            winit::keyboard::NamedKey::Escape => Key::Escape,
            winit::keyboard::NamedKey::Execute => Key::Execute,
            winit::keyboard::NamedKey::Find => Key::Find,
            winit::keyboard::NamedKey::Help => Key::Help,
            winit::keyboard::NamedKey::Pause => Key::Pause,
            winit::keyboard::NamedKey::Play => Key::Play,
            winit::keyboard::NamedKey::Props => Key::Props,
            winit::keyboard::NamedKey::Select => Key::Select,
            winit::keyboard::NamedKey::ZoomIn => Key::ZoomIn,
            winit::keyboard::NamedKey::ZoomOut => Key::ZoomOut,
            winit::keyboard::NamedKey::BrightnessDown => Key::BrightnessDown,
            winit::keyboard::NamedKey::BrightnessUp => Key::BrightnessUp,
            winit::keyboard::NamedKey::Eject => Key::Eject,
            winit::keyboard::NamedKey::LogOff => Key::LogOff,
            winit::keyboard::NamedKey::Power => Key::Power,
            winit::keyboard::NamedKey::PowerOff => Key::PowerOff,
            winit::keyboard::NamedKey::PrintScreen => Key::PrintScreen,
            winit::keyboard::NamedKey::Hibernate => Key::Hibernate,
            winit::keyboard::NamedKey::Standby => Key::Standby,
            winit::keyboard::NamedKey::WakeUp => Key::WakeUp,
            winit::keyboard::NamedKey::AllCandidates => Key::AllCandidates,
            winit::keyboard::NamedKey::Alphanumeric => Key::Alphanumeric,
            winit::keyboard::NamedKey::CodeInput => Key::CodeInput,
            winit::keyboard::NamedKey::Compose => Key::Compose,
            winit::keyboard::NamedKey::Convert => Key::Convert,
            winit::keyboard::NamedKey::FinalMode => Key::FinalMode,
            winit::keyboard::NamedKey::GroupFirst => Key::GroupFirst,
            winit::keyboard::NamedKey::GroupLast => Key::GroupLast,
            winit::keyboard::NamedKey::GroupNext => Key::GroupNext,
            winit::keyboard::NamedKey::GroupPrevious => Key::GroupPrevious,
            winit::keyboard::NamedKey::ModeChange => Key::ModeChange,
            winit::keyboard::NamedKey::NextCandidate => Key::NextCandidate,
            winit::keyboard::NamedKey::NonConvert => Key::NonConvert,
            winit::keyboard::NamedKey::PreviousCandidate => Key::PreviousCandidate,
            winit::keyboard::NamedKey::Process => Key::Process,
            winit::keyboard::NamedKey::SingleCandidate => Key::SingleCandidate,
            winit::keyboard::NamedKey::HangulMode => Key::HangulMode,
            winit::keyboard::NamedKey::HanjaMode => Key::HanjaMode,
            winit::keyboard::NamedKey::JunjaMode => Key::JunjaMode,
            winit::keyboard::NamedKey::Eisu => Key::Eisu,
            winit::keyboard::NamedKey::Hankaku => Key::Hankaku,
            winit::keyboard::NamedKey::Hiragana => Key::Hiragana,
            winit::keyboard::NamedKey::HiraganaKatakana => Key::HiraganaKatakana,
            winit::keyboard::NamedKey::KanaMode => Key::KanaMode,
            winit::keyboard::NamedKey::KanjiMode => Key::KanjiMode,
            winit::keyboard::NamedKey::Katakana => Key::Katakana,
            winit::keyboard::NamedKey::Romaji => Key::Romaji,
            winit::keyboard::NamedKey::Zenkaku => Key::Zenkaku,
            winit::keyboard::NamedKey::ZenkakuHankaku => Key::ZenkakuHankaku,
            winit::keyboard::NamedKey::Soft1 => Key::Soft1,
            winit::keyboard::NamedKey::Soft2 => Key::Soft2,
            winit::keyboard::NamedKey::Soft3 => Key::Soft3,
            winit::keyboard::NamedKey::Soft4 => Key::Soft4,
            winit::keyboard::NamedKey::ChannelDown => Key::ChannelDown,
            winit::keyboard::NamedKey::ChannelUp => Key::ChannelUp,
            winit::keyboard::NamedKey::Close => Key::Close,
            winit::keyboard::NamedKey::MailForward => Key::MailForward,
            winit::keyboard::NamedKey::MailReply => Key::MailReply,
            winit::keyboard::NamedKey::MailSend => Key::MailSend,
            winit::keyboard::NamedKey::MediaClose => Key::MediaClose,
            winit::keyboard::NamedKey::MediaFastForward => Key::MediaFastForward,
            winit::keyboard::NamedKey::MediaPause => Key::MediaPause,
            winit::keyboard::NamedKey::MediaPlay => Key::MediaPlay,
            winit::keyboard::NamedKey::MediaPlayPause => Key::MediaPlayPause,
            winit::keyboard::NamedKey::MediaRecord => Key::MediaRecord,
            winit::keyboard::NamedKey::MediaRewind => Key::MediaRewind,
            winit::keyboard::NamedKey::MediaStop => Key::MediaStop,
            winit::keyboard::NamedKey::MediaTrackNext => Key::MediaTrackNext,
            winit::keyboard::NamedKey::MediaTrackPrevious => Key::MediaTrackPrevious,
            winit::keyboard::NamedKey::New => Key::New,
            winit::keyboard::NamedKey::Open => Key::Open,
            winit::keyboard::NamedKey::Print => Key::Print,
            winit::keyboard::NamedKey::Save => Key::Save,
            winit::keyboard::NamedKey::SpellCheck => Key::SpellCheck,
            winit::keyboard::NamedKey::Key11 => Key::Key11,
            winit::keyboard::NamedKey::Key12 => Key::Key12,
            winit::keyboard::NamedKey::AudioBalanceLeft => Key::AudioBalanceLeft,
            winit::keyboard::NamedKey::AudioBalanceRight => Key::AudioBalanceRight,
            winit::keyboard::NamedKey::AudioBassBoostDown => Key::AudioBassBoostDown,
            winit::keyboard::NamedKey::AudioBassBoostToggle => Key::AudioBassBoostToggle,
            winit::keyboard::NamedKey::AudioBassBoostUp => Key::AudioBassBoostUp,
            winit::keyboard::NamedKey::AudioFaderFront => Key::AudioFaderFront,
            winit::keyboard::NamedKey::AudioFaderRear => Key::AudioFaderRear,
            winit::keyboard::NamedKey::AudioSurroundModeNext => Key::AudioSurroundModeNext,
            winit::keyboard::NamedKey::AudioTrebleDown => Key::AudioTrebleDown,
            winit::keyboard::NamedKey::AudioTrebleUp => Key::AudioTrebleUp,
            winit::keyboard::NamedKey::AudioVolumeDown => Key::AudioVolumeDown,
            winit::keyboard::NamedKey::AudioVolumeUp => Key::AudioVolumeUp,
            winit::keyboard::NamedKey::AudioVolumeMute => Key::AudioVolumeMute,
            winit::keyboard::NamedKey::MicrophoneToggle => Key::MicrophoneToggle,
            winit::keyboard::NamedKey::MicrophoneVolumeDown => Key::MicrophoneVolumeDown,
            winit::keyboard::NamedKey::MicrophoneVolumeUp => Key::MicrophoneVolumeUp,
            winit::keyboard::NamedKey::MicrophoneVolumeMute => Key::MicrophoneVolumeMute,
            winit::keyboard::NamedKey::SpeechCorrectionList => Key::SpeechCorrectionList,
            winit::keyboard::NamedKey::SpeechInputToggle => Key::SpeechInputToggle,
            winit::keyboard::NamedKey::LaunchApplication1 => Key::LaunchApplication1,
            winit::keyboard::NamedKey::LaunchApplication2 => Key::LaunchApplication2,
            winit::keyboard::NamedKey::LaunchCalendar => Key::LaunchCalendar,
            winit::keyboard::NamedKey::LaunchContacts => Key::LaunchContacts,
            winit::keyboard::NamedKey::LaunchMail => Key::LaunchMail,
            winit::keyboard::NamedKey::LaunchMediaPlayer => Key::LaunchMediaPlayer,
            winit::keyboard::NamedKey::LaunchMusicPlayer => Key::LaunchMusicPlayer,
            winit::keyboard::NamedKey::LaunchPhone => Key::LaunchPhone,
            winit::keyboard::NamedKey::LaunchScreenSaver => Key::LaunchScreenSaver,
            winit::keyboard::NamedKey::LaunchSpreadsheet => Key::LaunchSpreadsheet,
            winit::keyboard::NamedKey::LaunchWebBrowser => Key::LaunchWebBrowser,
            winit::keyboard::NamedKey::LaunchWebCam => Key::LaunchWebCam,
            winit::keyboard::NamedKey::LaunchWordProcessor => Key::LaunchWordProcessor,
            winit::keyboard::NamedKey::BrowserBack => Key::BrowserBack,
            winit::keyboard::NamedKey::BrowserFavorites => Key::BrowserFavorites,
            winit::keyboard::NamedKey::BrowserForward => Key::BrowserForward,
            winit::keyboard::NamedKey::BrowserHome => Key::BrowserHome,
            winit::keyboard::NamedKey::BrowserRefresh => Key::BrowserRefresh,
            winit::keyboard::NamedKey::BrowserSearch => Key::BrowserSearch,
            winit::keyboard::NamedKey::BrowserStop => Key::BrowserStop,
            winit::keyboard::NamedKey::AppSwitch => Key::AppSwitch,
            winit::keyboard::NamedKey::Call => Key::Call,
            winit::keyboard::NamedKey::Camera => Key::Camera,
            winit::keyboard::NamedKey::CameraFocus => Key::CameraFocus,
            winit::keyboard::NamedKey::EndCall => Key::EndCall,
            winit::keyboard::NamedKey::GoBack => Key::GoBack,
            winit::keyboard::NamedKey::GoHome => Key::GoHome,
            winit::keyboard::NamedKey::HeadsetHook => Key::HeadsetHook,
            winit::keyboard::NamedKey::LastNumberRedial => Key::LastNumberRedial,
            winit::keyboard::NamedKey::Notification => Key::Notification,
            winit::keyboard::NamedKey::MannerMode => Key::MannerMode,
            winit::keyboard::NamedKey::VoiceDial => Key::VoiceDial,
            winit::keyboard::NamedKey::TV => Key::TV,
            winit::keyboard::NamedKey::TV3DMode => Key::TV3DMode,
            winit::keyboard::NamedKey::TVAntennaCable => Key::TVAntennaCable,
            winit::keyboard::NamedKey::TVAudioDescription => Key::TVAudioDescription,
            winit::keyboard::NamedKey::TVAudioDescriptionMixDown => Key::TVAudioDescriptionMixDown,
            winit::keyboard::NamedKey::TVAudioDescriptionMixUp => Key::TVAudioDescriptionMixUp,
            winit::keyboard::NamedKey::TVContentsMenu => Key::TVContentsMenu,
            winit::keyboard::NamedKey::TVDataService => Key::TVDataService,
            winit::keyboard::NamedKey::TVInput => Key::TVInput,
            winit::keyboard::NamedKey::TVInputComponent1 => Key::TVInputComponent1,
            winit::keyboard::NamedKey::TVInputComponent2 => Key::TVInputComponent2,
            winit::keyboard::NamedKey::TVInputComposite1 => Key::TVInputComposite1,
            winit::keyboard::NamedKey::TVInputComposite2 => Key::TVInputComposite2,
            winit::keyboard::NamedKey::TVInputHDMI1 => Key::TVInputHDMI1,
            winit::keyboard::NamedKey::TVInputHDMI2 => Key::TVInputHDMI2,
            winit::keyboard::NamedKey::TVInputHDMI3 => Key::TVInputHDMI3,
            winit::keyboard::NamedKey::TVInputHDMI4 => Key::TVInputHDMI4,
            winit::keyboard::NamedKey::TVInputVGA1 => Key::TVInputVGA1,
            winit::keyboard::NamedKey::TVMediaContext => Key::TVMediaContext,
            winit::keyboard::NamedKey::TVNetwork => Key::TVNetwork,
            winit::keyboard::NamedKey::TVNumberEntry => Key::TVNumberEntry,
            winit::keyboard::NamedKey::TVPower => Key::TVPower,
            winit::keyboard::NamedKey::TVRadioService => Key::TVRadioService,
            winit::keyboard::NamedKey::TVSatellite => Key::TVSatellite,
            winit::keyboard::NamedKey::TVSatelliteBS => Key::TVSatelliteBS,
            winit::keyboard::NamedKey::TVSatelliteCS => Key::TVSatelliteCS,
            winit::keyboard::NamedKey::TVSatelliteToggle => Key::TVSatelliteToggle,
            winit::keyboard::NamedKey::TVTerrestrialAnalog => Key::TVTerrestrialAnalog,
            winit::keyboard::NamedKey::TVTerrestrialDigital => Key::TVTerrestrialDigital,
            winit::keyboard::NamedKey::TVTimer => Key::TVTimer,
            winit::keyboard::NamedKey::AVRInput => Key::AVRInput,
            winit::keyboard::NamedKey::AVRPower => Key::AVRPower,
            winit::keyboard::NamedKey::ColorF0Red => Key::ColorF0Red,
            winit::keyboard::NamedKey::ColorF1Green => Key::ColorF1Green,
            winit::keyboard::NamedKey::ColorF2Yellow => Key::ColorF2Yellow,
            winit::keyboard::NamedKey::ColorF3Blue => Key::ColorF3Blue,
            winit::keyboard::NamedKey::ColorF4Grey => Key::ColorF4Grey,
            winit::keyboard::NamedKey::ColorF5Brown => Key::ColorF5Brown,
            winit::keyboard::NamedKey::ClosedCaptionToggle => Key::ClosedCaptionToggle,
            winit::keyboard::NamedKey::Dimmer => Key::Dimmer,
            winit::keyboard::NamedKey::DisplaySwap => Key::DisplaySwap,
            winit::keyboard::NamedKey::DVR => Key::DVR,
            winit::keyboard::NamedKey::Exit => Key::Exit,
            winit::keyboard::NamedKey::FavoriteClear0 => Key::FavoriteClear0,
            winit::keyboard::NamedKey::FavoriteClear1 => Key::FavoriteClear1,
            winit::keyboard::NamedKey::FavoriteClear2 => Key::FavoriteClear2,
            winit::keyboard::NamedKey::FavoriteClear3 => Key::FavoriteClear3,
            winit::keyboard::NamedKey::FavoriteRecall0 => Key::FavoriteRecall0,
            winit::keyboard::NamedKey::FavoriteRecall1 => Key::FavoriteRecall1,
            winit::keyboard::NamedKey::FavoriteRecall2 => Key::FavoriteRecall2,
            winit::keyboard::NamedKey::FavoriteRecall3 => Key::FavoriteRecall3,
            winit::keyboard::NamedKey::FavoriteStore0 => Key::FavoriteStore0,
            winit::keyboard::NamedKey::FavoriteStore1 => Key::FavoriteStore1,
            winit::keyboard::NamedKey::FavoriteStore2 => Key::FavoriteStore2,
            winit::keyboard::NamedKey::FavoriteStore3 => Key::FavoriteStore3,
            winit::keyboard::NamedKey::Guide => Key::Guide,
            winit::keyboard::NamedKey::GuideNextDay => Key::GuideNextDay,
            winit::keyboard::NamedKey::GuidePreviousDay => Key::GuidePreviousDay,
            winit::keyboard::NamedKey::Info => Key::Info,
            winit::keyboard::NamedKey::InstantReplay => Key::InstantReplay,
            winit::keyboard::NamedKey::Link => Key::Link,
            winit::keyboard::NamedKey::ListProgram => Key::ListProgram,
            winit::keyboard::NamedKey::LiveContent => Key::LiveContent,
            winit::keyboard::NamedKey::Lock => Key::Lock,
            winit::keyboard::NamedKey::MediaApps => Key::MediaApps,
            winit::keyboard::NamedKey::MediaAudioTrack => Key::MediaAudioTrack,
            winit::keyboard::NamedKey::MediaLast => Key::MediaLast,
            winit::keyboard::NamedKey::MediaSkipBackward => Key::MediaSkipBackward,
            winit::keyboard::NamedKey::MediaSkipForward => Key::MediaSkipForward,
            winit::keyboard::NamedKey::MediaStepBackward => Key::MediaStepBackward,
            winit::keyboard::NamedKey::MediaStepForward => Key::MediaStepForward,
            winit::keyboard::NamedKey::MediaTopMenu => Key::MediaTopMenu,
            winit::keyboard::NamedKey::NavigateIn => Key::NavigateIn,
            winit::keyboard::NamedKey::NavigateNext => Key::NavigateNext,
            winit::keyboard::NamedKey::NavigateOut => Key::NavigateOut,
            winit::keyboard::NamedKey::NavigatePrevious => Key::NavigatePrevious,
            winit::keyboard::NamedKey::NextFavoriteChannel => Key::NextFavoriteChannel,
            winit::keyboard::NamedKey::NextUserProfile => Key::NextUserProfile,
            winit::keyboard::NamedKey::OnDemand => Key::OnDemand,
            winit::keyboard::NamedKey::Pairing => Key::Pairing,
            winit::keyboard::NamedKey::PinPDown => Key::PinPDown,
            winit::keyboard::NamedKey::PinPMove => Key::PinPMove,
            winit::keyboard::NamedKey::PinPToggle => Key::PinPToggle,
            winit::keyboard::NamedKey::PinPUp => Key::PinPUp,
            winit::keyboard::NamedKey::PlaySpeedDown => Key::PlaySpeedDown,
            winit::keyboard::NamedKey::PlaySpeedReset => Key::PlaySpeedReset,
            winit::keyboard::NamedKey::PlaySpeedUp => Key::PlaySpeedUp,
            winit::keyboard::NamedKey::RandomToggle => Key::RandomToggle,
            winit::keyboard::NamedKey::RcLowBattery => Key::RcLowBattery,
            winit::keyboard::NamedKey::RecordSpeedNext => Key::RecordSpeedNext,
            winit::keyboard::NamedKey::RfBypass => Key::RfBypass,
            winit::keyboard::NamedKey::ScanChannelsToggle => Key::ScanChannelsToggle,
            winit::keyboard::NamedKey::ScreenModeNext => Key::ScreenModeNext,
            winit::keyboard::NamedKey::Settings => Key::Settings,
            winit::keyboard::NamedKey::SplitScreenToggle => Key::SplitScreenToggle,
            winit::keyboard::NamedKey::STBInput => Key::STBInput,
            winit::keyboard::NamedKey::STBPower => Key::STBPower,
            winit::keyboard::NamedKey::Subtitle => Key::Subtitle,
            winit::keyboard::NamedKey::Teletext => Key::Teletext,
            winit::keyboard::NamedKey::VideoModeNext => Key::VideoModeNext,
            winit::keyboard::NamedKey::Wink => Key::Wink,
            winit::keyboard::NamedKey::ZoomToggle => Key::ZoomToggle,
            winit::keyboard::NamedKey::F1 => Key::F1,
            winit::keyboard::NamedKey::F2 => Key::F2,
            winit::keyboard::NamedKey::F3 => Key::F3,
            winit::keyboard::NamedKey::F4 => Key::F4,
            winit::keyboard::NamedKey::F5 => Key::F5,
            winit::keyboard::NamedKey::F6 => Key::F6,
            winit::keyboard::NamedKey::F7 => Key::F7,
            winit::keyboard::NamedKey::F8 => Key::F8,
            winit::keyboard::NamedKey::F9 => Key::F9,
            winit::keyboard::NamedKey::F10 => Key::F10,
            winit::keyboard::NamedKey::F11 => Key::F11,
            winit::keyboard::NamedKey::F12 => Key::F12,
            winit::keyboard::NamedKey::F13 => Key::F13,
            winit::keyboard::NamedKey::F14 => Key::F14,
            winit::keyboard::NamedKey::F15 => Key::F15,
            winit::keyboard::NamedKey::F16 => Key::F16,
            winit::keyboard::NamedKey::F17 => Key::F17,
            winit::keyboard::NamedKey::F18 => Key::F18,
            winit::keyboard::NamedKey::F19 => Key::F19,
            winit::keyboard::NamedKey::F20 => Key::F20,
            winit::keyboard::NamedKey::F21 => Key::F21,
            winit::keyboard::NamedKey::F22 => Key::F22,
            winit::keyboard::NamedKey::F23 => Key::F23,
            winit::keyboard::NamedKey::F24 => Key::F24,
            winit::keyboard::NamedKey::F25 => Key::F25,
            winit::keyboard::NamedKey::F26 => Key::F26,
            winit::keyboard::NamedKey::F27 => Key::F27,
            winit::keyboard::NamedKey::F28 => Key::F28,
            winit::keyboard::NamedKey::F29 => Key::F29,
            winit::keyboard::NamedKey::F30 => Key::F30,
            winit::keyboard::NamedKey::F31 => Key::F31,
            winit::keyboard::NamedKey::F32 => Key::F32,
            winit::keyboard::NamedKey::F33 => Key::F33,
            winit::keyboard::NamedKey::F34 => Key::F34,
            winit::keyboard::NamedKey::F35 => Key::F35,
            _ => Key::Unidentified(NativeKey::Unidentified),
        },
        winit::keyboard::Key::Unidentified(native) => Key::Unidentified(match native {
            winit::keyboard::NativeKey::Unidentified => NativeKey::Unidentified,
            winit::keyboard::NativeKey::Android(code) => NativeKey::Android(*code),
            winit::keyboard::NativeKey::MacOS(code) => NativeKey::MacOS(*code),
            winit::keyboard::NativeKey::Windows(code) => NativeKey::Windows(*code),
            winit::keyboard::NativeKey::Xkb(code) => NativeKey::Xkb(*code),
            winit::keyboard::NativeKey::Web(value) => NativeKey::Web(value.clone().into()),
        }),
    }
}
