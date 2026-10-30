//! ui_pet_sheep（0x142324f80，源码 29–；fresh 反编译证据）。
//!
//! 结构（O，fresh 142324f80.c）：
//! - UI 状态资源 gate：code==0x13 → `MainCamera::mode()==3` 才进；code==0xd 跳过；code==6 →
//!   `资源[1]低字节-1 > 1` 才进；其余码进。（DIVERGE：UI 状态资源身份未取证。）
//! - 逐羊：`MainCamera::mode()==3` → `dist(eye, pos) > 5.0`（0x40a00000）跳过。
//! - UI 接线：widget 注册、命中读取和拖拽状态已恢复；原版 signifier、
//!   WindowFocusEvent 与 decorator 宠物事件仍需补全。
//! - 拖拽（`slider3d::into_drag_state`）：
//!   - 返回值字节-2==1（=3）：`[0x34]>0.5` → `AudioInstance::start`、`[0x34]=0`；
//!   - ==2（=4）：拖拽位 signifier 写 + 相机 mode==3 时 push decorator 宠物事件 +
//!     **写 Sheep+8=1（跟随）、+0xc=14.0（计时）**（0x40e0000000000001 = {1, 14.0}）。
//!
//! drag==3 用出生时实际装配的 SheepAudio.elapsed；drag==4 写行为和计时。

use bevy_ecs::prelude::*;
use country_core::systems::camera_rig::main_camera::MainCamera;
use country_core::systems::mode_manager::{AppMode, WallShape};
use country_core::systems::ui::resources::ui_state::UiState;
use country_core::systems::ui::resources::widget_params::{UiSounds, WidgetParams, WidgetVolumeShape};
use glam::Vec3;

use crate::components::{Sheep, Transform};
use crate::sheep::sheep_behavior::SheepAudio;

/// 抚摸目标（每帧更新；宿主 UI 消费；与 PetDuckTarget 同构；二进制无此资源——宿主缝）。
#[derive(Resource, Clone, Copy, Debug, Default, PartialEq)]
pub struct PetSheepTarget {
    pub entity: Option<Entity>,
    pub position: Vec3,
}

/// ui_pet_sheep（0x142324f80）。
///
/// O（fresh）：相机 mode==3 时 `dist(eye, sheep) > 5.0` → 跳过该羊；可抚摸羊 → UI 命中 →
/// 拖拽宠物写 `Sheep.behavior_state=1`、`goal_timer=14.0`；drag 状态 3 且音频
/// `[0x34]>0.5` 时启动并清零。
pub fn ui_pet_sheep(
    camera: Res<MainCamera>,
    mode: Res<AppMode>,
    ui: Res<UiState>,
    mut target: ResMut<PetSheepTarget>,
    mut query: Query<(Entity, &Transform, ComponentWrite<Sheep>, &mut SheepAudio)>,
) {
    // 0x142325044: Wall 子判别字节按 wrapping_sub(1)>1 比较。
    let permitted = match *mode {
        AppMode::Photomode => camera.mode_byte() == 3,
        AppMode::Replay => false,
        AppMode::Wall(shape) => matches!(shape, WallShape::Freeform),
        _ => true,
    };
    if !permitted {
        target.entity = None;
        target.position = Vec3::ZERO;
        return;
    }
    // signifier、WindowFocusEvent 和 decorator 事件仍未恢复。
    let mut active = None;
    for (entity, transform, mut sheep, mut audio) in &mut query {
        let pos = transform.translation;
        if camera.mode_byte() == 3 {
            // O：`dist(eye, sheep) > 5.0` → 跳过（0x40a00000）。
            let eye = camera.eye_position();
            let d = (eye - pos).length();
            if d > 5.0 {
                continue;
            }
        }
        let widget = ui.add(
            &format!("pet_sheep{}", sheep.pet_target),
            WidgetParams::new(
                WidgetVolumeShape::sphere(pos + Vec3::new(0.0, 0.8, 0.0), 1.0),
                [0; 2],
            )
            .sounds(UiSounds::empty())
            .no_history_recording(),
        );
        let _intersection = widget.intersecting();
        if widget.is_active() {
            active = Some((entity, pos));
        }
        let drag = widget.into_drag_state();
        if drag.0 == 3 && audio.elapsed > 0.5 {
            audio.handle.start();
            audio.elapsed = 0.0;
        } else if drag.0 == 4 {
            sheep.behavior_state = 1;
            sheep.goal_timer = 14.0;
        }
    }
    target.entity = active.map(|(e, _)| e);
    target.position = active.map_or(Vec3::ZERO, |(_, p)| p);
}

use country_core::render::extract::ComponentWrite;

#[cfg(test)]
mod original_pet_interaction_tests {
    use super::*;
    use crate::audio::{AudioHandle, CritterAudioSink};
    use country_core::resources::critters::sheep::SheepMoveState;
    use country_core::systems::ui::resources::ui_state::{UiInteraction, WidgetId};
    use std::sync::{Arc, atomic::{AtomicUsize, Ordering}};

    struct Starts(Arc<AtomicUsize>);
    impl CritterAudioSink for Starts {
        fn start(&mut self) { self.0.fetch_add(1, Ordering::SeqCst); }
        fn start_if_not_playing(&mut self) { panic!("sheep pet uses start"); }
        fn stop(&mut self) { panic!("sheep pet does not stop"); }
        fn set_position(&mut self, _: Vec3) { panic!("sheep pet does not move audio"); }
        fn set_volume(&mut self, _: f32) { panic!("sheep pet does not set volume"); }
        fn set_parameter(&mut self, _: &str, _: f32) { panic!("sheep pet does not set parameters"); }
    }

    #[test]
    fn native_drag_phases_use_the_spawned_sheep_audio_and_follow_state() {
        let mut world = World::new();
        world.insert_resource(MainCamera::default());
        world.insert_resource(AppMode::default());
        world.insert_resource(PetSheepTarget::default());
        let widget = WidgetId::from_name("pet_sheep0");
        world.insert_resource(UiState::new(UiInteraction { native_tag: 2, native_phase: 3, widget }));
        let starts = Arc::new(AtomicUsize::new(0));
        let entity = world.spawn((
            Transform::default(),
            Sheep { behavior_state: 0, goal_timer: 0.0, pet_phase: 0, pet_progress: 0.0,
                pet_target: 0, move_state: SheepMoveState::Idle { timeout_s: 0.0 },
                amplitude: 0.0, control_flag: 1 },
            SheepAudio { handle: AudioHandle::Active(Box::new(Starts(starts.clone()))),
                startup_pitch: 0.0, elapsed: 0.6 },
        )).id();
        let mut schedule = Schedule::default();
        schedule.add_systems(ui_pet_sheep);
        schedule.run(&mut world);
        assert_eq!(starts.load(Ordering::SeqCst), 1);
        assert_eq!(world.get::<SheepAudio>(entity).unwrap().elapsed, 0.0);
        assert_eq!(world.get::<Sheep>(entity).unwrap().behavior_state, 0);
        world.resource_mut::<UiState>().replace_interaction(UiInteraction { native_tag: 2, native_phase: 4, widget });
        schedule.run(&mut world);
        let sheep = world.get::<Sheep>(entity).unwrap();
        assert_eq!((sheep.behavior_state, sheep.goal_timer), (1, 14.0));
        assert_eq!(world.resource::<PetSheepTarget>().entity, Some(entity));
    }
}
