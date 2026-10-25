//! sheep_floatie（0x142320cf0，fresh 反编译 + 符号实证 2026-10-17）。
//!
//! PDB 系统签名（`InfallibleSystemWrapper` 符号 0x1405e2cb0）：
//! `fn(Query<(&Transform, &CritterLocomotion, &mut FloatieHolder), With<Sheep>>,
//!     Query<(&mut SheepAccessory, &mut Transform), Without<Sheep>>,
//!     Commands, Res<AudioCore>, Res<GameTime>)`。
//!
//! 逐语句（fresh 142320cf0）：
//! - 羊实体（With<Sheep>）：`FloatieHolder+0x14 == 0` → `spawn_empty()` +
//!   `insert<DrawableMeshBundle>`（变换 = 羊变换，mesh/shader = holder 内两 Handle）+
//!   `insert<SheepAccessory>`（owner + 自身实体 + `visible = (locomotion==4)`）；
//!   写 holder[+0x10]=新实体、+0x14=1。
//! - 配件实体（Without<Sheep>）：整份 Transform 复制自 owner 羊（rotation/translation/
//!   scale 三轴）→ 读 owner locomotion（state==4 = 过渡态）→
//!   `(state==4 && !audio_started)` 或 `(state!=4 && audio_started)` → instantiate
//!   "event:/SFX/Sheep/UmbrellaClose"（0x14304b9b8，len 30）+ set_position +
//!   start_if_not_playing；`audio_started = (state==4)` 每帧回写；
//!   `progress = clamp(progress + dir*dt, 0, 1)`，`dir = state==4 ? +2 : -2`；
//!   `scale = BackOut.apply(progress) * 羊 scale.x` → 三轴写入配件 Transform；
//!   `visible = (state==4)`。
//!
//! Easing 变体实读：0x14304ba28 tag=**5 = BackOut**（旧按 CubicOut 的近似已纠正）。
//! DIVERGE：DrawableMeshBundle/mesh/shader 句柄由宿主经 FloatieHolder 注入（B 的
//! holder 前两 qword 即 Handle<Mesh>/Handle<RasterShaderProgram>）。

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

/// 浮水圈音频事件（B 字面量 0x14304b9b8，len 30）。
pub const FLOATIE_EVENT: &str = "event:/SFX/Sheep/UmbrellaClose";

/// 羊侧持有记录（B stride 0x18）：mesh@+0、shader@+8、spawned entity@+0x10、
/// initialized byte@+0x14。
#[derive(Component, Clone, Copy, Debug, Default)]
pub struct FloatieHolder {
    pub mesh: Option<Handle<Mesh>>,
    pub shader: Option<Handle<RasterShaderProgram>>,
    pub spawned: Option<Entity>,
    pub initialized: bool,
}

/// 配件实体组件（B `SheepAccessory`）：owner 绑定 + 展开进度/音频翻转位/可见位/实例。
#[derive(Component, Debug, Default)]
pub struct SheepAccessory {
    /// 归属羊（宿主投影；B 为两个 qword 的实体记录）。
    pub owner: Option<Entity>,
    /// +0：展开进度。
    pub progress: f32,
    /// +4：音频翻转位（`audio_started`）。
    pub audio_started: bool,
    /// +0x94：可见（= locomotion 4）。
    pub visible: bool,
    /// 音频实例（宿主缝；B 侧实例不参与查询）。
    pub audio: AudioHandle,
}

/// sheep_floaties（0x142320cf0）。
pub fn sheep_floaties(
    time: Res<GameTime>,
    audio_core: Option<Res<CritterAudioCore>>,
    mut commands: Commands,
    mut sheep_q: Query<
        (Entity, &Transform, &CritterLocomotion, &mut FloatieHolder),
        With<Sheep>,
    >,
    mut accessories: Query<(&mut SheepAccessory, ComponentWrite<Transform>), Without<Sheep>>,
) {
    let dt = time.delta;

    // ① 首次生成配件实体（O：holder+0x14==0）。
    for (entity, transform, locomotion, mut holder) in &mut sheep_q {
        if holder.initialized {
            continue;
        }
        let mut spawn = commands.spawn(SheepAccessory {
            owner: Some(entity),
            progress: 0.0,
            audio_started: false,
            visible: locomotion.0 == 4,
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

    // ② 配件更新（O：整份 Transform 复制 → 状态/音频/进度/缩放）。
    for (mut accessory, mut transform) in &mut accessories {
        let Some(owner) = accessory.owner else {
            continue;
        };
        let Ok((_, sheep_transform, locomotion, _)) = sheep_q.get_mut(owner) else {
            continue;
        };
        // 整份 Transform 复制（rotation/translation/scale）。
        *transform = *sheep_transform;
        let state = locomotion.0;
        let is_transition = state == 4;

        // 音频翻转（O：进入与离开过渡态各播一次同一事件）。
        if (is_transition && !accessory.audio_started)
            || (!is_transition && accessory.audio_started)
        {
            if let Some(core) = audio_core.as_ref() {
                let mut handle = core.instantiate(FLOATIE_EVENT);
                handle.set_position(transform.translation);
                handle.start_if_not_playing();
                accessory.audio = handle;
            }
            accessory.audio_started = is_transition;
        }

        // 展开/收缩（O：+2.0 / -2.0，clamp[0,1]）。
        let dir = if is_transition { 2.0 } else { -2.0 };
        accessory.progress = (accessory.progress + dir * dt).clamp(0.0, 1.0);

        // 缩放 = BackOut(progress) * 羊 scale.x（三轴同值）。
        let ease = EasingFunction::BackOut.apply(accessory.progress);
        let scale = ease * sheep_transform.scale.x;
        transform.scale = Vec3::splat(scale);
        accessory.visible = is_transition;
    }
}

/// 浮水圈音频宿主缝类型（历史 API）。
#[derive(Component, Debug, Default)]
pub struct FloatieAudio {
    pub handle: AudioHandle,
}

/// 展开进度（历史 API；B 字段并入 SheepAccessory）。
#[derive(Component, Clone, Copy, Debug, Default, PartialEq)]
pub struct FloatieEase {
    pub progress: f32,
    pub audio_started: bool,
}

/// 浮水圈（历史 API）。
#[derive(Component, Clone, Copy, Debug, Default, PartialEq)]
pub struct Floatie {
    pub pos: Vec3,
    pub phase: f32,
    pub owner: Option<Entity>,
}

use country_core::render::extract::ComponentWrite;
