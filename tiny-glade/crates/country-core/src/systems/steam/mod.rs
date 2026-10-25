//! `systems/steam/mod.rs` — Steam 集成：客户端资源/覆盖层/截图/输入映射。
//!
//! 原文件（B）：`crates/country-core/src/systems/steam/mod.rs`（23 原函数：业务 16 /
//! 内联 2 / 胶水 5）。
//!
//! 本文件为逐函数 fresh 反编译后的重建（2026-10-17，rebuild-steam）。关键证据：
//! - `add_steam_integration` 0x140945320：bevy 插件 build——`insert_resource<SteamClientResource>`/
//!   `SteamScreenshotRequests`、`HookScreenshots`+`ScreenshotRequested` 回调、`PreStartup`
//!   `startup_steam`、`Update` `update_steam_inputs.in_set(GameSystemSet)`。
//! - `update_steam_inputs` 0x140ae9740（22KB，ReVa fresh 反编译 8158 行）：8 参数
//!   `(Res<SteamClientResource>, Local<PrevSteamInputState>, ResMut<SteamInput>,
//!   ResMut<ActionSetValues>, ResMut<ActiveInputModeTracker>,
//!   EventReader<InputCursorDelta>, Res<GameTime>, Res<EscMenuState>)`；
//!   EscMenuState 门 = `tag==2 || (tag==10 && flag!=1)`（与 translate_input 同）；
//!   逐手柄激活动作集 → read_state → is_any_active（模拟 `max(|x|,|y|)>=0.25`）→
//!   controller_deadline = now + 250ms；模拟 → ActionSetValues.current.analog
//!   （camera_pan += 左摇杆、camera_height += 右摇杆×0.1、camera_zoom.x +=
//!   delta×30×1.5×摇杆）；22 同构数字块沿变化簿记（+0x170 减 / +0x178 增）后压
//!   输入码按动作类型写入 `actions_by_type`；尾块消费 `InputCursorDelta` 作右键手势
//!   （阈值 3.0，写 current.digital[1]）。
//!
//! 库外 Steam SDK 绑定（原 `steamworks` 0.11.0 git 依赖）经
//! [`SteamSdk`](crate::resources::steam::SteamSdk) trait 注入；本模块只审调用语义。
//! 2026-10-24：原+160是动作源HashMap，旧“SIMD上帧状态表”解释已纠正。
//! 主/菜单所有映射、每帧OR、控制器门控与右键手势按完整原控制流恢复。

use std::sync::Arc;

use bevy_ecs::prelude::*;
use log::{error, info};

use crate::input::{action_set::ActionSetValues, ActiveInputModeTracker, INPUT_MODE_COOLDOWN};
use crate::resources::gui::EscMenuState;
use crate::resources::steam::{
    SteamActionSetDigitalAction, SteamInput, SteamMenuActionSetState, SteamMainActionSetState,
    SteamSdk,
};
use crate::resources::time::GameTime;
use crate::systems::backend::input::translate_input_to_cursor_moved::InputCursorDelta;

pub mod startup_steam;

/// 浮窗文本输入打开标志（原进程全局字节 `0x143634d94`，回调缓存）。
static FLOATING_GAMEPAD_TEXT_INPUT_OPEN: std::sync::atomic::AtomicBool =
    std::sync::atomic::AtomicBool::new(false);

/// Steam 客户端资源（`update_steam_inputs`/`startup_steam`/覆盖层方法的客户端面）。
#[derive(Resource, Default)]
pub struct SteamClientResource {
    /// 注入的 Steam 客户端（None = 无 Steam）。
    pub client: Option<Arc<dyn SteamSdk>>,
}

impl SteamClientResource {
    /// show_floating_gamepad_text_input 0x140ae9390：全局未打开时才弹浮窗
    /// （`ShowFloatingGamepadTextInput(KeyboardMode, 0, 0, 0x500, 400)`），
    /// 注册 Dismissed 回调并把结果缓存到全局标志。
    pub fn show_floating_gamepad_text_input(&self) {
        if FLOATING_GAMEPAD_TEXT_INPUT_OPEN.load(std::sync::atomic::Ordering::Relaxed) {
            return;
        }
        let Some(sdk) = &self.client else {
            return;
        };
        // steamworks `utils::show_floating_gamepad_text_input`：mode 1 = KeyboardMode，
        // 位置 (0,0)，尺寸 (0x500=1280, 400)。回调注册 + Dismissed 缓存为库外 SDK 职责。
        sdk.show_floating_gamepad_text_input(1, 0, 0, 0x500, 400);
        // 原 `SteamAPI_ISteamUtils_ShowFloatingGamepadTextInput` 返回 bool → 缓存。
        FLOATING_GAMEPAD_TEXT_INPUT_OPEN.store(true, std::sync::atomic::Ordering::Relaxed);
    }

    /// is_floating_gamepad_text_input_open 0x140ae9470：读全局字节（RNE：`movzx eax,
    /// byte [0x143634d94]`）。
    pub fn is_floating_gamepad_text_input_open(&self) -> bool {
        FLOATING_GAMEPAD_TEXT_INPUT_OPEN.load(std::sync::atomic::Ordering::Relaxed)
    }

    /// input_show_binding_panel 0x140ae9480：客户端非空 → `ShowBindingPanel(handle)`。
    pub fn input_show_binding_panel(&self, handle: u64) {
        if let Some(sdk) = &self.client {
            sdk.show_binding_panel(handle);
        }
    }
}

/// AppSteamIntegration：steamworks 客户端封装（`client` 字段 @+0）。
pub struct AppSteamIntegration {
    pub client: Arc<dyn SteamSdk>,
}

impl AppSteamIntegration {
    /// try_new 0x140ae94b0：解析 app id 常量 2198150 → info 日志 →
    /// `Client::init_app(app_id)`（库外 SDK 创建，经 `sdk_factory` 注入）。
    pub fn try_new(
        sdk_factory: impl FnOnce(u32) -> Option<Arc<dyn SteamSdk>>,
    ) -> anyhow::Result<Self> {
        let app_id: u32 = "2198150".parse().expect("steam app id is a decimal constant");
        info!("Initializing Steam API with app id {}", app_id);
        let client = sdk_factory(app_id)
            .ok_or_else(|| anyhow::anyhow!("steamworks Client::init_app failed"))?;
        Ok(Self { client })
    }

    /// client 0x14067dd80：`mov rax, rcx`（client 字段 @+0，按引用返回）。
    pub fn client(&self) -> &Arc<dyn SteamSdk> {
        &self.client
    }

    /// user_id 0x140ae96f0：每次现算 `client.user().steam_id()`（Arc 临时 clone+drop）。
    pub fn user_id(&self) -> u64 {
        self.client.user_steam_id()
    }
}

/// `Local<PrevSteamInputState>`（update_steam_inputs 的本地状态）。
#[derive(Default)]
pub struct PrevSteamInputState {
    /// 帧计数（@0x30；每帧 +1）。
    pub frame_count: u64,
    /// 右键手势移动累加器（@0x38）。
    pub mouse_moved: f32,
    /// 右键按住标志（@0x3c）。
    pub right_click_held: bool,
    /// 本帧是否有控制器产生输入（尾块 `local_158`）。
    pub controller_active: bool,
    /// 本帧产生输入的控制器（尾块 `local_178`）。
    pub active_controller: u64,
}

/// 截图完成槽的内容（原 `Mutex<Option<Result<(PathBuf,[u32;2]),Box<dyn Error+Send+Sync>>>>`；
/// 0x14031af10 分配时初始为 None（niche 0x8000000000000001），完成回调 0x1402ce120 填入）。
pub type SteamScreenshotResult =
    Result<(std::path::PathBuf, [u32; 2]), Box<dyn std::error::Error + Send + Sync>>;
pub type SteamScreenshotSlot = parking_lot::Mutex<Option<SteamScreenshotResult>>;

/// 原 `enum SteamScreenshotRequest`（`Vec<_>` 元素 8 B，空指针 niche）：
/// `Requested` = Steam `ScreenshotRequested` 回调放入的空占位（0x14091ad50 `push(None)`）；
/// `Captured(slot)` = gather 取走占位拍照后推入的槽，截图写盘完成后由完成回调填充，
/// 再由 `add_captured_screenshots_to_steam_library`（0x1402c13d0）消费。
#[derive(Clone)]
pub enum SteamScreenshotRequest {
    Requested,
    Captured(Arc<SteamScreenshotSlot>),
}

/// Steam 截图请求资源（原 `Arc<RwLock<Vec<SteamScreenshotRequest>>>`，Vec 在 +0x20/+0x28；
/// `add_steam_integration` 0x140945320 新建，`ScreenshotRequested` 回调与 gather 共享）。
#[derive(Resource, Clone, Default)]
pub struct SteamScreenshotRequests {
    pub requests: Arc<parking_lot::RwLock<Vec<SteamScreenshotRequest>>>,
}

impl SteamScreenshotRequests {
    /// `ScreenshotRequested` 回调体 0x14091ad50（0x14091ae36..ae77）：写锁内 `Vec.push(None)`。
    pub fn on_screenshot_requested(&self) {
        self.requests.write().push(SteamScreenshotRequest::Requested);
    }

    /// gather 0x14031af10 开头循环（0x14031b05x..b1b3）：写锁内找第一个空占位，
    /// `swap_remove` 取走并返回 true；无占位返回 false。
    pub fn take_requested(&self) -> bool {
        let mut requests = self.requests.write();
        match requests
            .iter()
            .position(|request| matches!(request, SteamScreenshotRequest::Requested))
        {
            Some(index) => {
                requests.swap_remove(index);
                true
            }
            None => false,
        }
    }

    /// gather 拍照时（Steam 存在）：新槽的一份推入该 Vec（写锁，0x14031b3f7..b4c1）。
    pub fn push_captured(&self, slot: Arc<SteamScreenshotSlot>) {
        self.requests.write().push(SteamScreenshotRequest::Captured(slot));
    }
}

pub use crate::resources::window::OsWindowCmd;

/// 覆盖层资源面（原 `SteamOverlayOpenResources` SystemParam，含
/// `Option<Res<SteamClientResource>>` 客户端 @+0x20 + `Events<OsWindowCmd>`）。
#[derive(Resource, Default)]
pub struct SteamOverlayOpenResources {
    /// 客户端（None = 无 Steam → 直开浏览器）。
    pub client: Option<Arc<dyn SteamSdk>>,
    /// 浏览器打开回调（库外 webbrowser 注入）。
    pub open_browser: Option<Arc<dyn Fn(&str) -> Result<(), String> + Send + Sync>>,
    /// 成功开浏览器后发事件（原 `Events<OsWindowCmd>`）。
    pub browser_opened_events: bevy_ecs::event::Events<OsWindowCmd>,
}

impl SteamOverlayOpenResources {
    /// can_use_overlay 0x140aef1a0：客户端非空 → `IsSteamRunningOnSteamDeck()`；
    /// 否则 false。
    pub fn can_use_overlay(&self) -> bool {
        match &self.client {
            Some(sdk) => sdk.is_steam_running_on_steam_deck(),
            None => false,
        }
    }

    /// open_overlay_or_browser_to_webpage 0x14026d3c0/0x14026d580：客户端非空 →
    /// 覆盖层 `activate_game_overlay_to_web_page(url)`；否则开浏览器。Deck 检查
    /// 在该路径执行但结果不参与分支（按反编译）。
    pub fn open_overlay_or_browser_to_webpage(&mut self, url: &str) {
        if let Some(sdk) = &self.client {
            let _on_deck = sdk.is_steam_running_on_steam_deck();
            info!("Opening overlay to {}", url);
            sdk.activate_game_overlay_to_web_page(url);
        } else {
            self.open_browser_to_webpage(url);
        }
    }

    /// open_browser_to_webpage 0x14026d080：webbrowser::open(url)；成功 → 发
    /// `OsWindowCmd` 事件；失败 → warn 日志。
    pub fn open_browser_to_webpage(&mut self, url: &str) {
        match self.open_browser.as_deref() {
            Some(open) => match open(url) {
                Ok(()) => {
                    self.browser_opened_events.send(OsWindowCmd);
                }
                Err(err) => {
                    error!("Failed opening browser window: {}", err);
                }
            },
            None => {
                error!("Failed opening browser window: no opener injected");
            }
        }
    }
}

/// Deliver host-opener results to the same command stream drained by main_loop.
pub fn forward_browser_window_commands(
    mut overlay: ResMut<SteamOverlayOpenResources>,
    mut window: EventWriter<OsWindowCmd>,
) {
    for command in overlay.browser_opened_events.drain() {
        window.write(command);
    }
}

/// 原完整控制流的22个主集映射，不能按汇编相邻常量配对：LLVM合并了公共尾部。
const DIGITAL_BLOCKS: [usize; 22] = [4,5,6,7,9,8,10,11,12,14,13,2,3,0,19,20,22,23,24,21,18,15];
const DIGITAL_TARGETS: [usize; 22] = [4,5,6,7,8,9,10,11,12,13,14,16,17,0,18,19,20,21,22,23,27,39];
const DIGITAL_CODES: [SteamActionSetDigitalAction; 22] = [
    SteamActionSetDigitalAction::ToolBuilding, SteamActionSetDigitalAction::ToolPath,
    SteamActionSetDigitalAction::ToolDecorator, SteamActionSetDigitalAction::ToolCustomize,
    SteamActionSetDigitalAction::ToolTerrain, SteamActionSetDigitalAction::ToolEraser,
    SteamActionSetDigitalAction::ToolGarden, SteamActionSetDigitalAction::ToolTree,
    SteamActionSetDigitalAction::ToolWater, SteamActionSetDigitalAction::Photomode,
    SteamActionSetDigitalAction::TimeOfDay, SteamActionSetDigitalAction::Undo,
    SteamActionSetDigitalAction::Redo, SteamActionSetDigitalAction::LeftClick,
    SteamActionSetDigitalAction::UiCancel, SteamActionSetDigitalAction::UiConfirm,
    SteamActionSetDigitalAction::UiLeft, SteamActionSetDigitalAction::UiRight,
    SteamActionSetDigitalAction::UiDown, SteamActionSetDigitalAction::UiUp,
    SteamActionSetDigitalAction::MainMenu, SteamActionSetDigitalAction::ToggleSnapping,
];
const MENU_TARGETS: [usize; 9] = [27,18,19,23,20,21,22,24,25];
const MENU_CODES: [SteamActionSetDigitalAction; 9] = [
    SteamActionSetDigitalAction::MenuMainMenu, SteamActionSetDigitalAction::MenuUiCancel,
    SteamActionSetDigitalAction::MenuUiConfirm, SteamActionSetDigitalAction::MenuUiUp,
    SteamActionSetDigitalAction::MenuUiLeft, SteamActionSetDigitalAction::MenuUiRight,
    SteamActionSetDigitalAction::MenuUiDown, SteamActionSetDigitalAction::MenuUiTabPrev,
    SteamActionSetDigitalAction::MenuUiTabNext,
];

/// 权威输入每帧做OR；提示映射记录动作源，不以按键是否按下或发生沿变化为门控。
fn project_digital_actions(
    input: &mut SteamInput, current: &mut crate::input::action_set::MainActionSetSerde,
    controller: u64, states: &[bool], sources: &[usize], targets: &[usize], codes: &[SteamActionSetDigitalAction],
) {
    for ((&source,&target),&code) in sources.iter().zip(targets).zip(codes) {
        current.digital[target] |= states[source];
        if input.active_controller == controller {
            input.actions_by_type.entry(target as u16).or_default().push(code);
        }
    }
}

/// update_steam_inputs 0x140ae9740（22KB，ReVa fresh 反编译 8158 行；O 结构全解）。
///
/// 参数序即 `add_steam_integration` 0x140945320 in_set 签名（PDB）。EscMenuState 门
/// `tag==2 || (tag==10 && flag!=1)` 与 translate_input 相同（0x140ae9740 行 887）。
///
/// 原HashMap键为核心动作、值为Steam动作源；不是只有按下沿才更新的位图。
pub fn update_steam_inputs(
    client: Res<SteamClientResource>,
    mut prev_state: Local<PrevSteamInputState>,
    mut steam_input: ResMut<SteamInput>,
    mut action_set_values: ResMut<ActionSetValues>,
    mut input_mode: ResMut<ActiveInputModeTracker>,
    mut input_cursor_delta: EventReader<InputCursorDelta>,
    time: Res<GameTime>,
    elapsed: Res<bevy_time::Time>,
    menu: Res<EscMenuState>,
) {
    // Arc 克隆门：无客户端 → 直接返回（原 `*local_140 == 0` panic 等价为 Option 门）。
    let Some(sdk) = client.client.clone() else {
        return;
    };
    prev_state.frame_count = prev_state.frame_count.wrapping_add(1);

    // 每帧清空动作队列与簿记。
    steam_input.actions_by_type.clear();

    sdk.run_frame();
    let controllers = sdk.connected_controllers();
    if steam_input.active_controller != 0 && !controllers.contains(&steam_input.active_controller) {
        steam_input.active_controller = 0;
        steam_input.last_controller = 0;
    }
    if steam_input.main_action_set.loaded == 0 { steam_input.main_action_set.try_load(sdk.as_ref()); }
    if steam_input.menu_action_set.loaded == 0 { steam_input.menu_action_set.try_load(sdk.as_ref()); }
    if controllers.is_empty() { return; }
    let view=menu.view();
    let esc_gate=view.tag==2 || (view.tag==10 && view.flag!=1);
    let mut controller_active=false;
    let mut active_controller=steam_input.active_controller;
    let mut right_down=false;
    for controller in controllers {
        if esc_gate {
            if steam_input.main_action_set.loaded != 1 {
                prev_state.right_click_held=false;
                prev_state.mouse_moved=0.;
                return;
            }
            sdk.activate_action_set(steam_input.main_action_set.action_set_handle,controller);
            let mut state=SteamMainActionSetState::default();
            steam_input.main_action_set.read_state(sdk.as_ref(),controller,&mut state);
            if state.is_any_active() {
                input_mode.controller_deadline=elapsed.elapsed().saturating_add(INPUT_MODE_COOLDOWN);
                controller_active=true;active_controller=controller;
            }
            let current=&mut action_set_values.current;
            current.analog[2]+=state.analog[0];
            current.analog[3]+=state.analog[2]*0.1;
            let step=time.delta*30.;
            if state.digital[16] { current.analog[4].x+=step*1.5; }
            if state.digital[17] { current.analog[4].x+=step*-1.5; }
            project_digital_actions(&mut steam_input,current,controller,&state.digital,&DIGITAL_BLOCKS,&DIGITAL_TARGETS,&DIGITAL_CODES);
            right_down|=state.digital[1];
        } else {
            if steam_input.menu_action_set.loaded != 1 { return; }
            sdk.activate_action_set(steam_input.menu_action_set.action_set_handle,controller);
            let mut state=SteamMenuActionSetState::default();
            steam_input.menu_action_set.read_state(sdk.as_ref(),controller,&mut state);
            if state.is_any_active() {
                input_mode.controller_deadline=elapsed.elapsed().saturating_add(INPUT_MODE_COOLDOWN);
                controller_active=true;active_controller=controller;
            }
            project_digital_actions(&mut steam_input,&mut action_set_values.current,controller,&state.digital,
                &[0,1,2,3,4,5,6,7,8],&MENU_TARGETS,&MENU_CODES);
        }
    }
    let movement=input_cursor_delta.read().last().map_or(0.,|event|event.0.length());
    if right_down && !prev_state.right_click_held { prev_state.mouse_moved=0.; }
    else if right_down { prev_state.mouse_moved+=movement; }
    if !right_down && prev_state.right_click_held && prev_state.mouse_moved<3. {
        action_set_values.current.digital[1]=true;
    }
    action_set_values.current.digital[2]|=right_down;
    prev_state.right_click_held=right_down;
    prev_state.controller_active=controller_active;
    prev_state.active_controller=active_controller;
    if controller_active {
        steam_input.last_controller=steam_input.active_controller;
        steam_input.active_controller=active_controller;
    }
}


/// 注册 Steam 资源与系统（原 `impl$4::add_steam_integration` 0x140945320 插件 build 的
/// 库内部分；`GameSystemSet` 接线由宿主调度）。
pub fn add_steam_integration(world: &mut World) {
    world.init_resource::<SteamClientResource>();
    world.init_resource::<SteamScreenshotRequests>();
    world.init_resource::<SteamOverlayOpenResources>();
    world.init_resource::<bevy_ecs::event::Events<OsWindowCmd>>();
    // SteamInput 由 startup_steam 经 Commands 插入（无 Default/FromWorld），这里不预置。
}

/// 把 22 个输入码的压入序暴露给录制/回放域（O：0x140ae9740）。
pub fn steam_input_codes() -> &'static [SteamActionSetDigitalAction; 22] {
    &DIGITAL_CODES
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn digital_blocks_cover_all_22_codes_in_push_order() {
        let codes = [
            0x400, 0x500, 0x600, 0x700, 0x900, 0x800, 0xa00, 0xb00, 0xc00, 0xe00, 0xd00, 0x200,
            0x300, 0, 0x1300, 0x1400, 0x1600, 0x1700, 0x1800, 0x1500, 0x1200, 0xf00,
        ];
        assert_eq!(DIGITAL_CODES.len(), 22);
        for (code, action) in codes.iter().zip(DIGITAL_CODES.iter()) {
            assert_eq!(action.code(), *code);
        }
    }

    #[test]
    fn packed_conversion_helpers_are_the_small_from_thunks() {
        // impl$30/31/37::from 的机器码：x<<8 / (x<<8)+1 / a+(b<<8)+1。
        use crate::resources::steam::packed_set;
        assert_eq!(packed_set(0x12), 0x1200);
    }

    #[test]
    fn screenshot_callback_pushes_empty_placeholder_and_gather_takes_one() {
        let requests = SteamScreenshotRequests::default();
        assert!(!requests.take_requested());
        requests.on_screenshot_requested();
        let slot = Arc::new(SteamScreenshotSlot::new(None));
        requests.push_captured(slot.clone());
        requests.on_screenshot_requested();
        // 取走占位用 swap_remove：不会碰 Captured 槽，数量递减。
        assert!(requests.take_requested());
        assert!(requests.take_requested());
        assert!(!requests.take_requested());
        let remaining = requests.requests.read();
        assert_eq!(remaining.len(), 1);
        assert!(matches!(&remaining[0], SteamScreenshotRequest::Captured(s) if Arc::ptr_eq(s, &slot)));
    }
}
