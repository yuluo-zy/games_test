//! sheep_umbrella（0x142321980，fresh 反编译 + 常量实读 2026-10-17）。
//!
//! 与 sheep_floaties 同构（fresh 142321980）：两个查询
//! `Query<(&Transform, &CritterLocomotion, &mut UmbrellaHolder), With<Sheep>>` +
//! `Query<(&mut SheepAccessory, &mut Transform), Without<Sheep>>` + Commands +
//! Res<AudioCore> + Res<GameTime>。
//!
//! 逐语句：
//! - 羊首帧（holder+0x14==0）→ spawn_empty + insert<DrawableMeshBundle>
//!   （变换 = 羊变换）+ insert<SheepAccessory{visible = mask}>；holder 记实体。
//! - 配件：整份 Transform 复制自 owner；`mask = 0x1a >> clamp(state-2, 0..3) & 1`
//!   （state∈{0,1,3,5,6,7+} → 1；{2,4} → 0；u32 回绕后 clamp）；mask 与 audio_started
//!   不等 → instantiate "event:/SFX/Sheep/UmbrellaClose"（0x14304bac8，len 30）+
//!   set_position + start_if_not_playing；`audio_started = mask`；
//!   `progress = clamp(progress + (mask ? +2 : -2)*dt, 0, 1)`；
//!   `scale = **BackOut**.apply(progress) * 羊 scale.x`（0x14304bb38 tag=5 实读）三轴；
//!   `visible = mask`。
//!
//! DIVERGE：宿主 bundle 句柄经 UmbrellaHolder 注入；关系以 owner 实体表达。

use bevy_ecs::prelude::*;
use country_core::asset_libraries::Handle;
use country_core::components::drawable::DrawableMeshBundle;
use country_core::render::mesh::Mesh;
use country_core::render::shader::RasterShaderProgram;
use country_core::resources::time::GameTime;
use glam::Vec3;
use utils::easing_functions::EasingFunction;

use crate::audio::{AudioHandle, CritterAudioCore};
use crate::components::{CritterLocomotion, Sheep, Transform};
use crate::sheep::sheep_floatie::SheepAccessory;

/// 伞音频事件（B 字面量 0x14304bac8，len 30）。
pub const UMBRELLA_EVENT: &str = "event:/SFX/Sheep/UmbrellaClose";

/// 羊侧伞持有记录（B stride 0x18：mesh/shader/实体/初始化字节）。
#[derive(Component, Clone, Copy, Debug, Default)]
pub struct UmbrellaHolder {
    pub mesh: Option<Handle<Mesh>>,
    pub shader: Option<Handle<RasterShaderProgram>>,
    pub spawned: Option<Entity>,
    pub initialized: bool,
}

/// 展开掩码（O 142321980 L179–184）：`0x1a >> clamp(state-2, ..,3) & 1`（u32 回绕）。
fn umbrella_mask(state: u32) -> bool {
    let mut v = state.wrapping_sub(2);
    if 4 < v {
        v = 3;
    }
    (0x1a_u32 >> (v & 0x1f)) & 1 == 1
}

/// sheep_umbrellas（0x142321980）。
pub fn sheep_umbrellas(
    time: Res<GameTime>,
    audio_core: Option<Res<CritterAudioCore>>,
    mut commands: Commands,
    mut sheep_q: Query<
        (Entity, &Transform, &CritterLocomotion, &mut UmbrellaHolder),
        With<Sheep>,
    >,
    mut accessories: Query<(&mut SheepAccessory, &mut Transform), Without<Sheep>>,
) {
    let dt = time.delta;

    // ① 首帧生成伞配件实体。
    for (entity, transform, locomotion, mut holder) in &mut sheep_q {
        if holder.initialized {
            continue;
        }
        let mask = umbrella_mask(locomotion.0);
        let mut spawn = commands.spawn(SheepAccessory {
            owner: Some(entity),
            progress: 0.0,
            audio_started: mask,
            visible: mask,
            audio: AudioHandle::Disabled,
        });
        if let (Some(mesh), Some(shader)) = (holder.mesh, holder.shader) {
            // DrawableMeshBundle 自带 Transform（Bundle 展开），不可再插独立 Transform。
            spawn.insert(DrawableMeshBundle {
                mesh,
                shader,
                transform: *transform,
            });
        } else {
            spawn.insert(*transform);
        }
        holder.spawned = Some(spawn.id());
        holder.initialized = true;
    }

    // ② 配件更新。
    for (mut accessory, mut transform) in &mut accessories {
        let Some(owner) = accessory.owner else {
            continue;
        };
        let Ok((_, sheep_transform, locomotion, _)) = sheep_q.get_mut(owner) else {
            continue;
        };
        *transform = *sheep_transform;
        let mask = umbrella_mask(locomotion.0);

        // 翻转音效（O：mask != audio_started）。
        if mask != accessory.audio_started {
            if let Some(core) = audio_core.as_ref() {
                let mut handle = core.instantiate(UMBRELLA_EVENT);
                handle.set_position(transform.translation);
                handle.start_if_not_playing();
                accessory.audio = handle;
            }
            accessory.audio_started = mask;
        }

        let dir = if mask { 2.0 } else { -2.0 };
        accessory.progress = (accessory.progress + dir * dt).clamp(0.0, 1.0);
        let ease = EasingFunction::BackOut.apply(accessory.progress);
        let scale = ease * sheep_transform.scale.x;
        transform.scale = Vec3::splat(scale);
        accessory.visible = mask;
    }
}

/// 雨状态（历史宿主缝资源；B 实际为羊状态驱动，非雨门控）。
#[derive(Resource, Clone, Copy, Debug, Default, PartialEq)]
pub struct Raining(pub bool);

/// 伞音频宿主缝（历史 API）。
#[derive(Component, Debug, Default)]
pub struct UmbrellaAudio {
    pub handle: AudioHandle,
}

/// 展开进度（历史 API；B 字段并入 SheepAccessory）。
#[derive(Component, Clone, Copy, Debug, Default, PartialEq)]
pub struct UmbrellaEase {
    pub progress: f32,
    pub audio_started: bool,
}

/// 羊伞（历史 API）。
#[derive(Component, Clone, Copy, Debug, Default, PartialEq)]
pub struct SheepUmbrella {
    pub pos: Vec3,
    pub phase: f32,
    pub owner: Option<Entity>,
}
