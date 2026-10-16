//! sheep_floatie（B 主基线；0x142320cf0，源码 15–）。
//!
//! 证据（skeleton PDB 依赖注解）：AudioCore::instantiate_audio、AudioInstance::
//! start_if_not_playing、EasingFunction::apply、Commands::spawn_empty、
//! option::unwrap_failed/result::unwrap_failed。
//! 语义：羊进入深水 → **生成**浮水圈实体（spawn_empty + instantiate_audio）并播放音频；
//! 出水 → 移除（despawn）。

use bevy_ecs::prelude::*;
use country_core::resources::time::GameTime;
use country_core::resources::world_raster::WorldRaster;
use glam::Vec3;
use utils::easing_functions::EasingFunction;

use crate::audio::{AudioHandle, CritterAudioCore};
use crate::components::{Sheep, Transform};
use crate::sheep::sheep_terrain::{SheepTerrainState, STATE_GROUNDED};

/// 浮水圈（组件；宿主渲染域消费；EasingFunction::apply 驱动浮沉）。
#[derive(Component, Clone, Copy, Debug, Default, PartialEq)]
pub struct Floatie {
    pub pos: Vec3,
    pub phase: f32,
    /// 归属羊（生成后绑定；羊出水时移除）。
    pub owner: Option<Entity>,
}

/// 浮水圈音频（宿主缝）。
#[derive(Component, Debug, Default)]
pub struct FloatieAudio {
    pub handle: AudioHandle,
}

/// sheep_floaties（0x142320cf0）：深水羊 ↔ 浮水圈实体同步。
/// 生成：深水羊无浮水圈 → `Commands::spawn_empty` + `AudioCore::instantiate_audio`
/// （"event:/SFX/Sheep/Float"）+ start_if_not_playing；出水 → despawn。
pub fn sheep_floaties(
    time: Res<GameTime>,
    water: Res<WorldRaster<512>>,
    audio_core: Option<Res<CritterAudioCore>>,
    mut commands: Commands,
    mut floaties: Query<(Entity, &mut Floatie, &mut FloatieAudio)>,
    sheep: Query<(Entity, &Transform, &Sheep, &SheepTerrainState)>,
) {
    let dt = time.delta;
    let w = (*water).as_ref();
    // 推进现有浮水圈（EasingFunction::apply 浮沉相位）。
    let mut remove = Vec::new();
    for (entity, mut floatie, mut audio) in &mut floaties {
        floatie.phase += dt;
        let ease = EasingFunction::CubicOut.apply((floatie.phase * 2.0).fract());
        floatie.pos.y += ease * dt;
        audio.handle.start_if_not_playing();
        // 出水 → 移除（O：羊出水浮圈消失）。
        let in_water = w.bilinear_sample_at_world_xz(
            floatie.pos.x,
            floatie.pos.z,
            130.0,
            130.0,
        ) > 0.5;
        if !in_water {
            remove.push(entity);
        }
    }
    for entity in remove {
        commands.entity(entity).despawn();
    }
    // 生成：深水且非地面态的羊 → spawn 浮水圈（每羊 1 个，owner 绑定）。
    let existing: Vec<Entity> = floaties
        .iter()
        .filter_map(|(_, f, _)| f.owner)
        .collect();
    for (entity, transform, _sheep, state) in &sheep {
        if state.0 == STATE_GROUNDED {
            continue;
        }
        let in_water =
            w.bilinear_sample_at_world_xz(
                transform.translation.x,
                transform.translation.z,
                130.0,
                130.0,
            ) > 0.5;
        if in_water && !existing.contains(&entity) {
            let handle = match &audio_core {
                Some(core) => core.instantiate("event:/SFX/Sheep/Float"),
                None => AudioHandle::Disabled,
            };
            commands.spawn((
                Floatie {
                    pos: transform.translation,
                    phase: 0.0,
                    owner: Some(entity),
                },
                FloatieAudio { handle },
            ));
        }
    }
}
