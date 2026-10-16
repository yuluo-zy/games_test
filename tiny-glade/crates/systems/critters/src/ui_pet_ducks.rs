//! ui_pet_ducks（0x142322e30，源码 29–）：鸭子互动 UI（抚摸）。
//!
//! 证据（skeleton PDB 依赖注解）：MainCamera::eye_position/mode、UiWidgetContext::
//! intersecting/is_active、UiSounds::empty、WidgetParams::new/no_history_recording/sounds、
//! SignifierFactory::new、UiSignifierStream::write、AudioInstance::start。
//! 本重建实现状态面（选中目标 + 音频启动）；signifier 流写入归宿主 UI 接线（toolbar 批次）。

use bevy_ecs::prelude::*;
use country_core::resources::cursor_terrain_raycast::CursorTerrainRaycast;
use country_core::systems::camera_rig::main_camera::MainCamera;
use glam::Vec3;

use crate::audio::AudioHandle;
use crate::components::{Duck, DuckState, Transform, Velocity};

/// 抚摸目标（每帧更新；宿主 UI 消费）。
#[derive(Resource, Clone, Copy, Debug, Default, PartialEq)]
pub struct PetDuckTarget {
    pub entity: Option<Entity>,
    pub position: Vec3,
}

/// ui_pet_ducks（0x142322e30）：相机/光标附近、空闲（DuckState==2）的鸭子可被抚摸。
pub fn ui_pet_ducks(
    camera: Res<MainCamera>,
    cursor: Res<CursorTerrainRaycast>,
    mut target: ResMut<PetDuckTarget>,
    mut query: Query<(Entity, &Transform, &Duck, &DuckState, &mut Velocity, &mut DuckAudioMut)>,
) {
    let focus = if camera.mode_byte() == 3 {
        camera.eye_position()
    } else {
        cursor.position()
    };
    let mut best: Option<(Entity, f32, Vec3)> = None;
    for (entity, transform, _duck, state, _velocity, _audio) in &mut query {
        if state.to_byte() != 2 {
            continue;
        }
        let d = (transform.translation - focus).length();
        if d < 3.0 {
            if best.map_or(true, |(_, bd, _)| d < bd) {
                best = Some((entity, d, transform.translation));
            }
        }
    }
    target.entity = best.map(|(e, _, p)| e);
    target.position = best.map_or(Vec3::ZERO, |(_, _, p)| p);
}

/// 鸭子音频可变句柄（pet 时启动 Quack；宿主缝，见 audio 模块）。
#[derive(Component, Debug, Default)]
pub struct DuckAudioMut {
    pub handle: AudioHandle,
}

impl DuckAudioMut {
    pub fn quack(&mut self) {
        self.handle.start_if_not_playing();
    }
}
