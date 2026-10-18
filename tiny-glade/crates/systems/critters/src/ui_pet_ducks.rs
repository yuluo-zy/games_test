//! ui_pet_ducks（0x142322e30，源码 29–）：鸭子互动 UI（抚摸）。
//!
//! fresh 反编译（evidence/restorer-eco-critters/decomp/142322e30.c）的真实结构：
//!
//! 1) **宠物计时衰减**（无条件，先于 UI gate）：每鸭 `Duck+0xf8`（pet timer f32）按
//!    `clamp(f_f8 ± dt / 0.4, 0, 1)` 推进——`Duck+0xfc`（frozen）!=0 → `+dt/0.4`、
//!    ==0 → `−dt/0.4`；当 `frozen && f_f8 >= 1.0` → 释放（frozen=0，并写 per-entity u32 槽）。
//!    两条 fetch 路径（table / archetype，`*(query+0x108)` 分支）条件等价（frozen 恒 0/1）。
//! 2) **UI 交互区**（LAB_1423231d0，受 UI 状态资源 gate：`*(int*)*res` code 0x13/0xd/6 +
//!    `MainCamera::mode()==3`）：
//!    - `format!("pet_duck{entity}")` → `WidgetParams::new().sounds(UiSounds::empty())
//!      .no_history_recording()` → `UiState::add<String>` → `UiWidgetContext::intersecting()/is_active()`
//!      （mode==3 时与 `eye_position` 距离 >6.0 跳过）；
//!    - 激活 → 相机基四元数（pet 状态块 `+0xc0..+0xe8`）→ `SignifierFactory::new` +
//!      closure_0（0x142324290 "pet_hand"）/ closure_1（0x142324460 "duck_head" +
//!      `insert<LookAtCamera>`）→ `UiSignifierStream::write`；
//!    - `slider3d::into_drag_state()`：返回值字节−2==1（=3）→ `frozen=1` +
//!      `AudioInstance::start_if_not_playing`；==2（=4）→ 拖拽 signifier 写 + 音频。
//!
//! 第 1) 段常量/控制流与本实现判等（0.4、clamp[0,1]、frozen 语义）。第 2) 段归宿主 UI 接线：
//! country_core 目前缺 `UiWidgetContext::intersecting` / `SignifierEntityBuilder` /
//! `slider3d::into_drag_state` 三个 API，UI 状态 gate 资源身份未取证，故 2) 未实现（DIVERGE 见下）。

use bevy_ecs::prelude::*;
use country_core::resources::time::GameTime;

use crate::components::Duck;

/// ui_pet_ducks（0x142322e30）：宠物计时衰减段（O，判等）。
///
/// 每帧对所有鸭子推进 `Duck+0xf8`（pet timer）：frozen 时 `+dt/0.4`、非 frozen 时 `−dt/0.4`，
/// `clamp[0,1]`；`frozen && timer >= 1.0` → 释放 frozen。原码每鸭同时写一个 per-entity u32 槽
/// （值源 = query state +0x14，U —— 本重建无对应组件，DIVERGE）。UI 交互段（format "pet_duck" /
/// WidgetParams / UiState::add / intersect / is_active / signifier / drag-state）未实现，见模块头注释。
pub fn ui_pet_ducks(time: Res<GameTime>, mut query: Query<&mut Duck>) {
    let dt = time.delta;
    for mut duck in &mut query {
        // O（142322e30 L148–157）：`fVar32 = dt`；frozen==0 → `fVar32 = -fVar32`；
        // `f_f8 = clamp(fVar32/0.4 + f_f8, 0, 1)`（min/max 展开 = clamp）。
        let step = if duck.frozen { dt } else { -dt } / 0.4;
        duck.f_f8 = (duck.f_f8 + step).clamp(0.0, 1.0);
        // O（L143–146/L189–192）：`frozen != 0 && f_f8 >= 1.0` → 释放（frozen=0）。
        if duck.frozen && duck.f_f8 >= 1.0 {
            duck.frozen = false;
        }
    }
}
