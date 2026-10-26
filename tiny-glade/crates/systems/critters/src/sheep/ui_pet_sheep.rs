//! ui_pet_sheep（0x142324f80，源码 29–；fresh 反编译证据）。
//!
//! 结构（O，fresh 142324f80.c）：
//! - UI 状态资源 gate：code==0x13 → `MainCamera::mode()==3` 才进；code==0xd 跳过；code==6 →
//!   `资源[1]低字节-1 > 1` 才进；其余码进。（DIVERGE：UI 状态资源身份未取证。）
//! - 逐羊：`MainCamera::mode()==3` → `dist(eye, pos) > 5.0`（0x40a00000）跳过。
//! - UI 接线（format "pet_sheep" + WidgetParams::new().sounds(UiSounds::empty())
//!   .no_history_recording() + UiState::add<String> + UiWidgetContext::intersecting/is_active +
//!   ws_to_cs + SignifierFactory::new + UiSignifierStream::write）→ 宿主 UI（未实现，同
//!   ui_pet_ducks 缺 country_core API）。
//! - 拖拽（`slider3d::into_drag_state`）：
//!   - 返回值字节-2==1（=3）：`[0x34]>0.5` → `AudioInstance::start`、`[0x34]=0`；
//!   - ==2（=4）：拖拽位 signifier 写 + 相机 mode==3 时 push decorator 宠物事件 +
//!     **写 Sheep+8=1（跟随）、+0xc=14.0（计时）**（0x40e0000000000001 = {1, 14.0}）。
//!
//! 本重建落确定段（UI 状态 gate 的控制流注释 + eye 距离门 + 目标记录）；UI 接线段
//! （intersecting/is_active/ws_to_cs/signifier/drag）未实现——`slider3d::into_drag_state`
//! 在 country-core 未暴露，宠物写入（drag 状态 4 分支）不得无条件伪造。

use bevy_ecs::prelude::*;
use country_core::systems::camera_rig::main_camera::MainCamera;
use glam::Vec3;

use crate::audio::AudioHandle;
use crate::components::{Sheep, SheepPetControl, Transform};

/// 抚摸目标（每帧更新；宿主 UI 消费；与 PetDuckTarget 同构；二进制无此资源——宿主缝）。
#[derive(Resource, Clone, Copy, Debug, Default, PartialEq)]
pub struct PetSheepTarget {
    pub entity: Option<Entity>,
    pub position: Vec3,
}

/// 羊抚摸音频（宿主缝）。
#[derive(Component, Debug, Default)]
pub struct SheepPetAudio {
    pub handle: AudioHandle,
}

/// ui_pet_sheep（0x142324f80）。
///
/// O（fresh）：相机 mode==3 时 `dist(eye, sheep) > 5.0` → 跳过该羊；可抚摸羊 → UI 命中 →
/// 拖拽宠物写 `Sheep.behavior_state=1`、`goal_timer=14.0`；drag 状态 3 且 `[0x34]>0.5` →
/// 播音频并清零。UI 状态资源 gate 与 format/WidgetParams/Signifier 段未实现（DIVERGE 见模块头）。
pub fn ui_pet_sheep(
    camera: Res<MainCamera>,
    mut target: ResMut<PetSheepTarget>,
    mut query: Query<(Entity, &Transform, ComponentWrite<Sheep>, &mut SheepPetControl, &mut SheepPetAudio)>,
) {
    // 未实现：UI 状态资源 code gate（0x13+mode3 / 0xd 跳过 / 6+子字节 / 其余；资源身份 U）。
    // 未实现：UI 命中（format!("pet_sheep{}", Sheep+0x20) / WidgetParams::new()
    // .sounds(UiSounds::empty()).no_history_recording() / UiState::add<String> /
    // UiWidgetContext::intersecting/is_active / eye 距离 / ws_to_cs(0.075 缩放) /
    // SignifierFactory::new + UiSignifierStream::write）。
    // 未实现：slider3d::into_drag_state（core 未暴露；DragState::is_some → WindowFocusEvent）。
    // 因此以下**可证段**仅为 eye 距离门与目标记录；宠物写入只在 drag 状态 4 分支发生，
    // 无 into_drag_state 时不得伪造（旧版无条件写 behavior_state=1/goal_timer=14.0 已删除）。
    let mut best: Option<(Entity, f32, Vec3)> = None;
    for (entity, transform, _sheep, _pet, _audio) in &mut query {
        let pos = transform.translation;
        if camera.mode_byte() == 3 {
            // O：`dist(eye, sheep) > 5.0` → 跳过（0x40a00000）。
            let eye = camera.eye_position();
            let d = (eye - pos).length();
            if d > 5.0 {
                continue;
            }
        }
        best = Some((entity, 0.0, pos));
        break;
    }
    target.entity = best.map(|(e, _, _)| e);
    target.position = best.map_or(Vec3::ZERO, |(_, _, p)| p);
}

use country_core::render::extract::ComponentWrite;
