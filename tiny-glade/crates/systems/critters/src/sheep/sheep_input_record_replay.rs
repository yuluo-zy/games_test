//! sheep_input_record_replay（0x14231b720，源码 0–；fresh 反编译证据）。
//!
//! 结构（O，fresh 14231b720.c）：
//! - 会话状态（`*(u64*)(*res+0x40) ^ 0x8000000000000000`，取 min 档；DIVERGE 资源身份未取证）：
//!   - `档 < 2` → 记录：`from_iter<Vec<Vec3>, QueryIter<Transform,With<Sheep>>>` 收集羊位 +
//!     `from_iter<Vec<f32>, …>` 收集航向 → 写 `InputStream::last_frame_mut`；
//!   - `档-3 > 1`（即 >4）且输入门字节 → 回放：逐羊从帧读 `(Vec3 位, f32 航向)` →
//!     `Transform.translation` + `glam::from_euler_angles` 写旋转。
//!
//! 本重建以 `SheepInputStream(InputStream<SheepInputFrame>)` 承载帧；`SheepInputReplayCmd`
//! 事件作会话状态宿主注入（二进制为资源 +0x40 字段，身份未取证 → DIVERGE）。记录语义从
//! `move_dir/speed` 改为二进制实际的 `positions[] / yaws[]` 数组帧。

use bevy_ecs::prelude::*;
use country_core::input::input_stream::InputStream;
use country_core::resources::time::GameTime;
use glam::{EulerRot, Quat, Vec3};

use crate::components::{Sheep, Transform};

/// 羊输入流资源包装（core `InputStream<F>` 泛型非 Resource；装配层构造）。
#[derive(Resource, Clone, Debug, Default)]
pub struct SheepInputStream(pub InputStream<SheepInputFrame>);

/// 羊输入帧（二进制一帧 = 全羊 `Vec<Vec3>` 位 + `Vec<f32>` 航向）。
#[derive(Clone, Debug, Default, PartialEq)]
pub struct SheepInputFrame {
    pub positions: Vec<Vec3>,
    pub yaws: Vec<f32>,
}

/// 羊输入记录/回放事件（宿主按会话状态写；二进制门为资源 +0x40 字段，DIVERGE）。
#[derive(Event, Clone, Copy, Debug, PartialEq)]
pub struct SheepInputReplayCmd {
    pub replaying: bool,
}

/// sheep_input_record_replay（0x14231b720）。
pub fn sheep_input_record_replay(
    time: Res<GameTime>,
    mut input_stream: ResMut<SheepInputStream>,
    mut events: EventReader<SheepInputReplayCmd>,
    mut query: Query<(Entity, &mut Transform, &Sheep)>,
) {
    let _ = &time;
    let mut replaying = false;
    for e in events.read() {
        replaying = e.replaying;
    }
    if !replaying {
        // 记录（O：两 from_iter 收集全羊 → last_frame_mut）。
        let frame = match input_stream.0.last_frame_mut() {
            Some(f) => f,
            None => return,
        };
        frame.positions.clear();
        frame.yaws.clear();
        for (_e, transform, _sheep) in &query {
            frame.positions.push(transform.translation);
            // 航向：绕 Y 轴 euler（O from_euler_angles；f32@帧载荷）。
            let (_, y, _) = transform.rotation.to_euler(EulerRot::XYZ);
            frame.yaws.push(y);
        }
    } else if let Some(frame) = input_stream.0.last_frame_mut() {
        // 回放（O：逐羊写 translation + from_euler_angles 旋转）。
        for (i, (_e, mut transform, _sheep)) in query.iter_mut().enumerate() {
            if let Some(&pos) = frame.positions.get(i) {
                transform.translation = pos;
            }
            if let Some(&yaw) = frame.yaws.get(i) {
                transform.rotation = Quat::from_euler(EulerRot::XYZ, 0.0, yaw, 0.0);
            }
        }
    }
}
