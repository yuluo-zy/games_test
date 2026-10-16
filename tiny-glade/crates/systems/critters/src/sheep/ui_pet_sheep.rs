//! ui_pet_sheep（0x142324f80，源码 29–；closure$0 L106、closure$1 L149）。
//!
//! 证据（skeleton PDB 依赖注解）：MainCamera::eye_position/mode、UiWidgetContext::
//! intersecting/is_active、UiSounds::empty、WidgetParams::new/no_history_recording/sounds、
//! SignifierFactory::new、UiSignifierStream::write、AudioInstance::start、
//! geometry::ws_to_cs。语义：羊互动 UI（抚摸/喂食）；signifier 流写归宿主 UI 接线。

use bevy_ecs::prelude::*;
use country_core::resources::cursor_terrain_raycast::CursorTerrainRaycast;
use country_core::systems::camera_rig::main_camera::MainCamera;
use glam::Vec3;

use crate::audio::AudioHandle;
use crate::components::{Sheep, SheepPetControl, Transform};

/// 抚摸目标（每帧更新；宿主 UI 消费；与 PetDuckTarget 同构）。
#[derive(Resource, Clone, Copy, Debug, Default, PartialEq)]
pub struct PetSheepTarget {
    pub entity: Option<Entity>,
    pub position: Vec3,
}

/// ui_pet_sheep（0x142324f80）：相机/光标附近、空闲羊可被抚摸。
pub fn ui_pet_sheep(
    camera: Res<MainCamera>,
    cursor: Res<CursorTerrainRaycast>,
    mut target: ResMut<PetSheepTarget>,
    mut query: Query<(Entity, &Transform, &Sheep, &mut SheepPetControl, &mut SheepPetAudio)>,
) {
    let focus = if camera.mode_byte() == 3 {
        camera.eye_position()
    } else {
        cursor.position()
    };
    let mut best: Option<(Entity, f32, Vec3)> = None;
    for (entity, transform, _sheep, _pet, _audio) in &mut query {
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

/// 羊抚摸音频（宿主缝）。
#[derive(Component, Debug, Default)]
pub struct SheepPetAudio {
    pub handle: AudioHandle,
}
