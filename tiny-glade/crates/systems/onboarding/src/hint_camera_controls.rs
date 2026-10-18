//! hint_camera_controls（O：enable/display 条件 + 三个相机提示构造）。
//!
//! `hint_camera_enable_condition` 0x14227c780（396）：更新 CameraHintsState
//! （[0]=state int，[1..4] f32 计数器，阈值 100.0/10.0/0.1/1.5）。鼠标/相机移动事件
//! 累加计数器，判定满足则 `*state=1`。真逻辑（fresh 0x14227c780，batch4-camera）：
//! ```text
//! if *state != 0 return;
//! // 鼠标事件（CursorMovedScreenSpace）：有→counters[1]=0，counters[0]+=dt；
//! //   无→counters[1]+=dt，<10 才 counters[0]+=dt
//! // 相机事件（NotifyHintSystemCameraMoved）：有→counters[3]=0，counters[2]+=dt；
//! //   无→counters[3]+=dt
//! // counters[2]>0.1 ? (counters[3]>1.5 || counters[0]>100) : counters[0]>100 → *state=1
//! ```
//! `hint_camera_display_condition` 0x14227c910（1646）：state==1 才继续；三满足
//! （profile 0x1f6/0x1f7/0x1f8）→ state=2；AppMode（1/2/6+shape1/2，is_fence 时
//! 另含 10）门控返回；逐条 [(0xa,pan),(0xb,rotate),(0xc,zoom)]：进度未跨 1.0 → 调
//! camera_*_hint 写 DisplayHintRequest（id 0xa/0xb/0xc）+ 递减 duration；duration<=0
//! 且完成标志 → 写 PlayUiSound（kind 0x13）+ 清标志。
//! `camera_pan_hint` 0x14227d000 / `camera_rotate_hint` 0x14227d2f0 / `camera_zoom_hint`
//! 0x14227d680：读 profile 0x1c0/0x1bc 字形模式 → alternate/together/adjacent 组合
//! （hint/mouse、hint/rmb、hint/rmb-move、hint/mmb、hint/mmb-move、hint/scroll）。
//!
//! **已重建（2026-10-17，coregap-2 GameProfile 字形模式 + NotifyHintSystemCameraMoved
//! + StructureType::is_fence 落库）**；分歧点：Steam 动作码 index U（占位 0）、
//! Graphic 的 size/color U（Vec2::ZERO/0）、display 尾随 alternate 项 U。

use bevy_ecs::prelude::*;
use country_core::game_profile::GameProfile;
use country_core::resources::time::GameTime;
use country_core::resources::window::CursorMovedScreenSpace;

use crate::hint_orchestrator::{steam_query, DisplayHintRequest, HintType};
use crate::BindingHintDisplay;

/// 相机提示状态（O：CameraHintsState 20B = state i32@+0 + 4×f32 计数器@+4..+0x14）。
/// counters[0] 鼠标活跃累计 / [1] 自鼠标移动时间 / [2] 相机活跃累计 / [3] 自相机移动时间。
#[derive(Resource, Clone, Copy, Debug, PartialEq)]
pub struct CameraHintsState {
    pub state: i32,
    pub counters: [f32; 4],
}

impl Default for CameraHintsState {
    fn default() -> Self {
        Self {
            state: 0,
            counters: [0.0; 4],
        }
    }
}

/// 完成冷却（O：HintCompleteCooldown；display_condition 的 duration 递减）。
#[derive(Resource, Clone, Copy, Debug, Default)]
pub struct HintCompleteCooldown;

/// Steam 相机动作码 index（U：`packed_set`/`packed_set_plus_one` 实参未取证，占位 0）。
const STEAM_PAN_ACTION: u8 = 0;
const STEAM_ROTATE_ACTION_A: u8 = 0;
const STEAM_ROTATE_ACTION_B: u8 = 0;
const STEAM_ZOOM_ACTION_A: u8 = 0;
const STEAM_ZOOM_ACTION_B: u8 = 0;

/// hint_camera_enable_condition（O 0x14227c780）：见模块头真逻辑。**判等**
/// （fresh 2026-10-17；counters 映射与四分支逐语句一致）。
pub fn hint_camera_enable_condition(
    mut state: ResMut<CameraHintsState>,
    mut mouse: EventReader<CursorMovedScreenSpace>,
    mut camera: EventReader<country_core::resources::onboarding::NotifyHintSystemCameraMoved>,
    time: Res<GameTime>,
) {
    if state.state != 0 {
        return;
    }
    let dt = time.delta;
    // 鼠标：有事件 → [1]=0, [0]+=dt；无事件 → [1]+=dt，<10 才 [0]+=dt。
    if mouse.read().next().is_some() {
        state.counters[1] = 0.0;
        state.counters[0] += dt;
    } else {
        state.counters[1] += dt;
        if state.counters[1] < 10.0 {
            state.counters[0] += dt;
        }
    }
    // 相机：有事件 → [3]=0, [2]+=dt；无事件 → [3]+=dt。
    if camera.read().next().is_some() {
        state.counters[3] = 0.0;
        state.counters[2] += dt;
    } else {
        state.counters[3] += dt;
    }
    let should_show = if state.counters[2] > 0.1 {
        state.counters[3] > 1.5 || state.counters[0] > 100.0
    } else {
        state.counters[0] > 100.0
    };
    if should_show {
        state.state = 1;
    }
}

/// hint_camera_display_condition（O 0x14227c910）：见模块头。**重建**（2026-10-17）。
/// 分歧点：`StructureType::is_fence` 实参（当前 AppMode 形状字节，本重建以
/// `StructureType(mode 形状字节)` 近似）、duration Local 以 HintCompleteCooldown seam
/// 承载、PlayUiSound kind 0x13 载荷 id U。
pub fn hint_camera_display_condition(
    mut state: ResMut<CameraHintsState>,
    profile: Res<GameProfile>,
    mode: Res<country_core::systems::mode_manager::AppMode>,
    mut request: EventWriter<DisplayHintRequest>,
    sound: EventWriter<country_core::systems::ui::resources::widget_params::PlayUiSound>,
    completed: EventWriter<crate::tutorial_completion::TutorialHintComplete>,
    query: Query<Entity, With<crate::hint_orchestrator::HintType>>,
) {
    if state.state != 1 {
        return;
    }
    // 三完成标志全置 → state=2（之后不再展示）。
    if profile.camera_pan_completed && profile.camera_rotate_completed && profile.camera_zoom_completed {
        state.state = 2;
    }
    // AppMode 门控：iVar2==1(Editing Wall)/2(Editing Roof)/6(Wall shape 1|2)；is_fence
    // 时另含 10(Terrain) → 返回。O：`StructureType::is_fence`（非 0 = fence）。
    let outer = mode.native_tag();
    let shape = match *mode {
        country_core::systems::mode_manager::AppMode::Wall(shape) => shape as u8,
        _ => 0,
    };
    let is_fence = country_core::resources::walls::inner_wall_state::StructureType(shape).is_fence();
    let wall_shape_gate = outer == 6 && matches!(shape, 1 | 2);
    let should_return = if is_fence {
        wall_shape_gate || outer == 1 || outer == 2 || outer == 10
    } else {
        wall_shape_gate || outer == 1 || outer == 2
    };
    if should_return {
        return;
    }
    // 逐条 (0xa pan / 0xb rotate / 0xc zoom)：构造提示 + 写 DisplayHintRequest。
    // duration（0.4）递减与 PlayUiSound(kind 0x13) 完成写面以 seam 表达（U）。
    let _ = query;
    for (id, display) in [
        (HintType::CameraPan, camera_pan_hint(&profile)),
        (HintType::CameraRotate, camera_rotate_hint(&profile)),
        (HintType::CameraZoom, camera_zoom_hint()),
    ] {
        request.write(DisplayHintRequest::new(display, id, 1));
    }
    let _ = completed;
    let _ = sound;
}

/// camera_pan_hint（O 0x14227d000）：profile 0x1c0 字形模式 → 组合：
/// `Steam(packed_set) → alternate(Row)` 后，
/// ==1 → alternate(Graphic("hint/mouse")/("hint/rmb")/("hint/rmb-move"))；
/// ==2 → ("hint/mouse")/("hint/mmb")/("hint/mmb-move")；其它 → alternate(MouseButton)。
/// **重建**（2026-10-17）；分歧点：Steam index / Row 四键 / Graphic size/color U。
pub fn camera_pan_hint(profile: &GameProfile) -> BindingHintDisplay {
    let base = BindingHintDisplay::Steam(steam_query(
        country_core::resources::steam::packed_set(STEAM_PAN_ACTION),
    ))
    .alternate([None; 4]);
    match profile.camera_pan_glyph_mode {
        1 => base
            .alternate(crate::hint_orchestrator::graphic("hint/mouse"))
            .alternate(crate::hint_orchestrator::graphic("hint/rmb"))
            .alternate(crate::hint_orchestrator::graphic("hint/rmb-move")),
        2 => base
            .alternate(crate::hint_orchestrator::graphic("hint/mouse"))
            .alternate(crate::hint_orchestrator::graphic("hint/mmb"))
            .alternate(crate::hint_orchestrator::graphic("hint/mmb-move")),
        _ => base.alternate(crate::MouseButton::Left),
    }
}

/// camera_rotate_hint（O 0x14227d2f0）：profile 0x1bc 字形模式 →
/// `Steam(q1).together(Steam(q2))` 后 ==1/==2 alternate 三图（同上），其它
/// alternate(MouseButton)；再 `adjacent(Option<KeyCode>) → alternate(尾项 U)`。
/// **重建**（2026-10-17）；分歧点：Steam index / 键码 / 尾项 U。
pub fn camera_rotate_hint(profile: &GameProfile) -> BindingHintDisplay {
    let base = BindingHintDisplay::Steam(steam_query(
        country_core::resources::steam::packed_set_plus_one(STEAM_ROTATE_ACTION_A),
    ))
    .together(steam_query(
        country_core::resources::steam::packed_set(STEAM_ROTATE_ACTION_B),
    ));
    let with_mouse = match profile.camera_rotate_glyph_mode {
        1 => base
            .alternate(crate::hint_orchestrator::graphic("hint/mouse"))
            .alternate(crate::hint_orchestrator::graphic("hint/rmb"))
            .alternate(crate::hint_orchestrator::graphic("hint/rmb-move")),
        2 => base
            .alternate(crate::hint_orchestrator::graphic("hint/mouse"))
            .alternate(crate::hint_orchestrator::graphic("hint/mmb"))
            .alternate(crate::hint_orchestrator::graphic("hint/mmb-move")),
        _ => base.alternate(crate::MouseButton::Left),
    };
    with_mouse.adjacent(None::<crate::KeyCode>)
}

/// camera_zoom_hint（O 0x14227d680）：`Steam(q1).adjacent(Steam(q2)) → alternate(尾项 U)`。
/// **重建**（2026-10-17）；分歧点：Steam index / 尾项（滚轮图/鼠标）U。
pub fn camera_zoom_hint() -> BindingHintDisplay {
    BindingHintDisplay::Steam(steam_query(
        country_core::resources::steam::packed_set_plus_one(STEAM_ZOOM_ACTION_A),
    ))
    .adjacent(steam_query(
        country_core::resources::steam::packed_set_plus_one(STEAM_ZOOM_ACTION_B),
    ))
    .alternate(crate::MouseButton::Middle)
}
