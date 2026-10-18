//! B `crates/tiny-glade/src/event_loop.rs`（winit 事件泵 / egui 输入桥）。
//!
//! 恢复依据（Ghidra shared 2026-10-17，restorer-tinyglade-E fresh）：
//! - `run_event_loop` `0x14026e4a0`（1,102 行反编译，winit Windows 回调胶水）
//! - `EguiEventIo::new` `0x14026de50`（35 行）
//! - `EguiEventIo::take_egui_input` `0x14026dfd0`（155 行）
//!
//! ## `EguiEventIo::new`（重建，布局未锚定）
//! fresh：`memcpy(0x120 from arg)` → 写 `{1,1,0}` 到 +0x120/+0x128/+0x130 →
//! `rust_alloc(0x138)` → memcpy → 返回 `{rdx, alloc, r9}`（24 B 外壳）。
//! 0x120 与 `egui_winit::State` 大小一致；`{1,1,0}` 是包装层标志（语义 U）。
//! **分歧 F1**：`EguiEventIo` 的字段名/外壳 24 B 语义未在 PDB 锚定；本合同实现
//! 取 `Mutex<egui_winit::State>`（`take_egui_input` 的锁字段 @+0x10 与
//! `Mutex` 行为一致）。
//!
//! ## `EguiEventIo::take_egui_input`（胶水-判等）
//! fresh 逐语句：`Mutex::lock(state)`（fast path + `lock_contended`）→
//! poison 检查 `unwrap_failed` → `puffin::are_scopes_on()` 门内
//! `begin_scope(<source path>)` → `egui_winit::State::take_egui_input()` →
//! `end_scope` → unlock（旧 state==2 → `futex::wake`）。全部由
//! `Mutex` + `puffin::profile_function!()` + 上游转发提供。
//!
//! ## `run_event_loop`（未实现，平台胶水）
//! 1,102 行反编译是 winit Windows 事件循环的外层回调：`EventLoop::run` 闭包、
//! 设备丢失/挂起恢复（`GraphicsDevice` swapchain 重建）、窗口事件投递到
//! `GameEvent`/`OsWindowEvent`（PDB 类型名存在，字段未知）、`available_monitors`
//! 变化、egui 事件桥、`RedrawWindow`/SetCursorPos 等平台调用。
//! 依赖 fork 内部回调与 `GameLoop` 字段面 → 保持 `todo!`。

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
        self.state.lock().unwrap().handle_platform_output(window, output);
    }
}

/// `tiny_glade::event_loop::run_event_loop` `0x14026e4a0` —— 未实现。
pub fn run_event_loop() {
    todo!(
        "未实现: run_event_loop 0x14026e4a0 —— winit Windows 事件泵胶水（1,102 行），\
         GameEvent/OsWindowEvent 字段与 GameLoop 字段面未落库"
    )
}
