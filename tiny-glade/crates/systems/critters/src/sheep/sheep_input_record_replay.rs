//! sheep_input_record_replay（0x14231b720，源码 0–；closure$0 L29、closure$1 L41）。
//!
//! 证据（skeleton PDB 依赖注解）：InputStream::last_frame_mut（country-core
//! `input::input_stream`）、alias 0x140949970 / 0x141dbdec0 / 0x141dbf100。
//! 语义：把羊的输入（record/replay）经 InputStream 记录/回放（移动方向/速度）。
//! 宿主在 replay 会话注入 InputStream<SheepInputFrame>。

use bevy_ecs::prelude::*;
use country_core::input::input_stream::InputStream;
use country_core::resources::time::GameTime;
use glam::Vec3;

use crate::components::{Sheep, SheepPetControl, Velocity};

/// 羊输入流资源包装（core `InputStream<F>` 泛型非 Resource；装配层构造）。
#[derive(Resource, Clone, Debug, Default)]
pub struct SheepInputStream(pub InputStream<SheepInputFrame>);

/// 羊输入帧（record/replay 载荷；语义 I——方向 + 速度）。
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct SheepInputFrame {
    pub move_dir: Vec3,
    pub speed: f32,
}

/// 羊输入记录/回放事件（宿主按会话状态写；语义 U——record/replay 两阶段）。
#[derive(Event, Clone, Copy, Debug, PartialEq)]
pub struct SheepInputReplayCmd {
    pub replaying: bool,
}

/// sheep_input_record_replay（0x14231b720）。
/// `InputStream::last_frame_mut`：record 阶段把羊速度写入输入流；replay 阶段
/// 从输入流回放速度。
pub fn sheep_input_record_replay(
    time: Res<GameTime>,
    mut input_stream: ResMut<SheepInputStream>,
    mut events: EventReader<SheepInputReplayCmd>,
    mut query: Query<(&Sheep, &mut Velocity, &mut SheepPetControl)>,
) {
    let _ = &time;
    let mut replaying = false;
    for e in events.read() {
        replaying = e.replaying;
    }
    for (_sheep, mut velocity, _pet) in &mut query {
        if replaying {
            if let Some(frame) = input_stream.0.last_frame_mut() {
                velocity.0 = frame.move_dir * frame.speed.max(0.0);
            }
        } else if let Some(frame) = input_stream.0.last_frame_mut() {
            frame.move_dir = velocity.0.normalize_or_zero();
            frame.speed = velocity.0.length();
        }
    }
}
