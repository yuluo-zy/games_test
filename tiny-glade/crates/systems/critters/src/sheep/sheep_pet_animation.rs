//! sheep_pet_animation（0x14231ac20，源码 15–）。
//!
//! 证据（skeleton PDB 依赖注解：alias 0x141dbeac0；reconstruction/critters/sheep_pet.rs
//! 五阶段状态机 C14293a350）：pet 事件 → 五阶段源突变（phase 0..4）→ 运动谓词
//! （being_petted 门控 walk animation）。相位/进度/幅度记录在 Sheep 组件
//! （+0x18/+0x1c/+0x58/+0x60，stride 0x68）。

use bevy_ecs::prelude::*;
use country_core::resources::time::GameTime;

use crate::components::Sheep;

/// 抚摸请求事件（宿主 UI 写）。
#[derive(Event, Clone, Copy, Debug, PartialEq)]
pub struct PlaySheepPet {
    pub target: u64,
}

/// sheep_pet_animation（0x14231ac20）：逐羊按请求推进五阶段 pet 动画。
pub fn sheep_pet_animation(
    time: Res<GameTime>,
    mut events: EventReader<PlaySheepPet>,
    mut query: Query<&mut Sheep>,
) {
    let dt = time.delta;
    let targets: Vec<u64> = events.read().map(|e| e.target).collect();
    for mut sheep in &mut query {
        let requested = targets.contains(&sheep.pet_target);
        let _ = advance_sheep_pet(&mut sheep, requested, dt);
    }
}

/// 五阶段状态机核心（reconstruction/critters/sheep_pet.rs::advance_sheep_pet 一致；
/// 相位 0=idle、1=起、2=保持、3=收、4=完成；0x58 幅度、0x60 控制字节）。
/// 返回 (推进发生, 是否进入新阶段)。
pub fn advance_sheep_pet(sheep: &mut Sheep, requested: bool, delta: f32) -> (bool, bool) {
    let mut advanced = false;
    let mut entered = false;
    match sheep.pet_phase {
        0 => {
            if !requested {
                return (false, false);
            }
            sheep.control_flag = 1;
            sheep.pet_phase = 1;
            sheep.pet_progress = 0.0;
            advanced = true;
            entered = true;
        }
        1 => {
            let progress = delta / f32::from_bits(0x3e99999a) + sheep.pet_progress;
            sheep.pet_phase = if progress > 1.0 { 2 } else { 1 };
            sheep.pet_progress = if progress > 1.0 { 0.0 } else { progress };
            sheep.amplitude = progress;
            advanced = true;
        }
        2 => {
            let progress = delta / f32::from_bits(0x3e4ccccd) + sheep.pet_progress;
            let retain = !(progress > 1.0) || requested;
            sheep.pet_phase = if retain { 2 } else { 3 };
            sheep.pet_progress = if retain { progress } else { 0.0 };
            sheep.amplitude = 1.0;
            advanced = true;
            if !retain {
                entered = true;
            }
        }
        3 => {
            let progress = delta / f32::from_bits(0x3e99999a) + sheep.pet_progress;
            sheep.pet_phase = if progress > 1.0 { 4 } else { 3 };
            if !(progress > 1.0) {
                sheep.pet_progress = progress;
            }
            sheep.control_flag = 0;
            sheep.amplitude = 1.0 - progress;
            advanced = true;
        }
        4 => {
            sheep.pet_phase = 0;
            sheep.amplitude = 0.0;
            advanced = true;
            entered = true;
        }
        _ => return (false, false),
    }
    (advanced, entered)
}
