//! sheep_umbrella（B 主基线；0x142321980，源码 0–）。
//!
//! 证据（skeleton PDB 依赖注解，与 sheep_floaties 同构）：AudioCore::instantiate_audio、
//! AudioInstance::start_if_not_playing、EasingFunction::apply、Commands::spawn_empty、
//! option::unwrap_failed/result::unwrap_failed。
//! 语义：雨天羊 → **生成**伞实体（spawn_empty + instantiate_audio）并播放音频；雨停 →
//! 移除（despawn）。

use bevy_ecs::prelude::*;
use country_core::resources::time::GameTime;
use glam::Vec3;
use utils::easing_functions::EasingFunction;

use crate::audio::{AudioHandle, CritterAudioCore};
use crate::components::{Sheep, Transform};

/// 羊伞（组件；宿主渲染域消费）。
#[derive(Component, Clone, Copy, Debug, Default, PartialEq)]
pub struct SheepUmbrella {
    pub pos: Vec3,
    pub phase: f32,
    /// 归属羊。
    pub owner: Option<Entity>,
}

/// 羊伞音频（宿主缝）。
#[derive(Component, Debug, Default)]
pub struct UmbrellaAudio {
    pub handle: AudioHandle,
}

/// 雨状态（宿主天气系统注入）。
#[derive(Resource, Clone, Copy, Debug, Default, PartialEq)]
pub struct Raining(pub bool);

/// sheep_umbrellas（0x142321980）：雨天羊 ↔ 伞实体同步。
/// 生成：雨天羊无伞 → spawn_empty + instantiate_audio + start_if_not_playing；
/// 雨停 → 全部 despawn。
pub fn sheep_umbrellas(
    time: Res<GameTime>,
    raining: Res<Raining>,
    audio_core: Option<Res<CritterAudioCore>>,
    mut commands: Commands,
    mut umbrellas: Query<(Entity, &mut SheepUmbrella, &mut UmbrellaAudio)>,
    sheep: Query<(Entity, &Transform, &Sheep)>,
) {
    let dt = time.delta;
    if !raining.0 {
        // 雨停 → 移除（O）。
        let mut done = Vec::new();
        for (entity, _, _) in &mut umbrellas {
            done.push(entity);
        }
        for entity in done {
            commands.entity(entity).despawn();
        }
        return;
    }
    // 推进（EasingFunction::apply 展开相位）。
    for (_, mut umbrella, mut audio) in &mut umbrellas {
        umbrella.phase += dt;
        let ease = EasingFunction::CubicOut.apply((umbrella.phase * 2.0).fract());
        umbrella.pos.y += ease * dt;
        audio.handle.start_if_not_playing();
    }
    // 生成：雨天羊 → spawn 伞（每羊 1 个）。
    let existing: Vec<Entity> = umbrellas
        .iter()
        .filter_map(|(_, u, _)| u.owner)
        .collect();
    for (entity, transform, _sheep) in &sheep {
        if existing.contains(&entity) {
            continue;
        }
        let handle = match &audio_core {
            Some(core) => core.instantiate("event:/SFX/Sheep/Umbrella"),
            None => AudioHandle::Disabled,
        };
        commands.spawn((
            SheepUmbrella {
                pos: transform.translation,
                phase: 0.0,
                owner: Some(entity),
            },
            UmbrellaAudio { handle },
        ));
    }
}
