//! hint_camera_controls（O：enable/display 条件 + 三个相机提示构造）。
//!
//! `hint_camera_enable_condition` 0x14227c780（396）：更新 CameraHintsState
//! （[0]=state int，[1..4] f32 计数器，阈值 100.0/10.0/0.1/1.5）。鼠标/相机移动事件
//! 累加计数器，判定满足则 `*state=1`。真逻辑：
//! ```text
//! if *state != 0 return;
//! // 读鼠标事件（CursorMovedScreenSpace）：有→[2]=0；无→[2]+=dt，<10 才 [1]+=dt
//! // 读相机事件（NotifyHintSystemCameraMoved）：有→[4]=0,[3]+=dt；无→[4]+=dt
//! // (0.1<[3] && ([4]>1.5 || [1]>100)) || (0.1>= [3] && [1]>100) → *state=1
//! ```
//! `hint_camera_display_condition` 0x14227c910（1646）：三满足组合（profile 0x1f6/0x1f7/
//! 0x1f8）→ state=2；AppMode（6/2/1/10）+ StructureType::is_fence 门控；构造
//! [(0xa,pan),(0xb,rotate),(0xc,zoom)] 逐条判定，duration-=dt，写 DisplayHintRequest
//! 与完成事件（tag 0x13）。
//! `camera_pan_hint` 0x14227d000 / `camera_rotate_hint` 0x14227d2f0 / `camera_zoom_hint`
//! 0x14227d680：读 profile 0x1c0/0x1bc 字形模式 → alternate/together/adjacent 组合
//! （hint/mouse、hint/rmb、hint/rmb-move、hint/mmb、hint/mmb-move、hint/scroll）。
//!
//! **未实现**：GameProfile 字形模式/完成标志未落库、NotifyHintSystemCameraMoved 未落库、
//! SteamInput 未落库 → 各函数标未实现，逻辑已登记。

use bevy_ecs::prelude::*;

/// 相机提示状态（O：CameraHintsState 20B = state i32@+0 + 4×f32 计数器@+4..+0x14）。
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

/// hint_camera_enable_condition（O 0x14227c780）：见模块头真逻辑。
/// **未实现**：NotifyHintSystemCameraMoved 未落库；骨架保留。
pub fn hint_camera_enable_condition(
    mut _state: ResMut<CameraHintsState>,
    _mouse: EventReader<country_core::resources::window::CursorMovedScreenSpace>,
    _time: Res<country_core::resources::time::GameTime>,
) {
    // 未实现：见模块头分歧点。
}

/// hint_camera_display_condition（O 0x14227c910）：见模块头真逻辑。
/// **未实现**：GameProfile 相机完成标志 + AppMode/StructureType 门控 + DisplayHintRequest
/// 写面依赖未落库。
pub fn hint_camera_display_condition(
    _state: ResMut<CameraHintsState>,
    _profile: Res<country_core::game_profile::GameProfile>,
    _mode: Res<country_core::systems::mode_manager::AppMode>,
    _query: Query<(Entity, &crate::hint_orchestrator::HintType)>,
) {
    // 未实现：见模块头分歧点。
}

/// camera_pan_hint（O 0x14227d000）：profile 0x1c0 字形模式 → 组合：
/// ==1 → alternate([Graphic("hint/mouse"),Graphic("hint/rmb"),Graphic("hint/rmb-move")])；
/// ==2 → [mouse,mmb,mmb-move]；其它 → alternate(MouseButton)。
/// **未实现**：profile 0x1c0 + SteamInput + lang 串未落库。
pub fn camera_pan_hint(
    _profile: Res<country_core::game_profile::GameProfile>,
) -> crate::BindingHintDisplay {
    crate::BindingHintDisplay::Empty
}

/// camera_rotate_hint（O 0x14227d2f0）：profile 0x1bc 字形模式 → together(steam,
/// alternate([...])) → adjacent(Option<KeyCode>) → alternate。
/// **未实现**：同上。
pub fn camera_rotate_hint(
    _profile: Res<country_core::game_profile::GameProfile>,
) -> crate::BindingHintDisplay {
    crate::BindingHintDisplay::Empty
}

/// camera_zoom_hint（O 0x14227d680）：adjacent(steam,steam) → alternate（滚轮缩放）。
/// **未实现**：同上。
pub fn camera_zoom_hint() -> crate::BindingHintDisplay {
    crate::BindingHintDisplay::Empty
}
