//! sheep_pet_animation（0x14231ac20，源码 15–；fresh 反编译证据 + RE-ANALYSIS §1.3 跳表解码）。
//!
//! 结构（O，fresh 14231ac20.c + 反汇编 0x14231ae78–0x14231afe2）：
//! - `from_iter<Vec<usize>, Map<EventIterator<PlaySheepPetAnimation>, closure_env$0>>` 收集
//!   事件目标实体。
//! - 取查询**首只羊**：`pet_target(+0x20)` 在事件目标列表 → 匹配路径（r11b=1）；否则未匹配路径。
//! - 两条路径均按 `phase(+0x18)` 经跳表 0x14304b178 分派 handler，随后函数返回（每帧一羊）。
//!   - 参数 XMM0/XMM1/XMM2 = 0.3(0x1429258d8)/1.0(0x142925904)/0.2(0x1429cd068)
//!     （dt 除数/阈值/除数）；r11b = 匹配标志（state0/state2 handler 读）。
//!
//! 跳表 0x14304b178 解码（RE-ANALYSIS §1.3，index=`*(uint*)(spot+0x18)`=phase，5 项）：
//! | index | 目标 | 语义 |
//! |---|---|---|
//! | 0 | 0x14231afe4 | `test r11b`：匹配 → `[+0x60]=1`、state→1、写音频槽；未匹配保持 |
//! | 1 | 0x14231b0b3 | `progress += dt/0.3`；`>1.0 → state=2,progress=0`；否则 state=1 保持；
//!   `[+0x58]=progress`（未钳制） |
//! | 2 | 0x14231b022 | `progress += dt/0.2`；`<=1.0 或 匹配 → 保持 state2`；否则 state→3、
//!   progress=0；`[+0x58]=1.0` |
//! | 3 | 0x14231b068 | `progress += dt/0.3`；`<=1.0 → 保持`；否则 state→4；`[+0x60]=0`；
//!   `[+0x58]=1.0-progress` |
//! | 4 | 0x14231b006 | state→0、`[+0x58]=0` |
//!
//! `advance_sheep_pet` 与解码 handler 逐语句一致（对照：phase0 匹配门/+0x60、phase1
//! progress 未钳制写 amplitude、phase2 `!(progress>1)||requested` 保持、phase3 +0x60=0 +
//! amplitude=1.0-progress、phase4 归零）。差异仅 phase0 额外写 `pet_progress=0`（二进制
//! h0 不写 +0x1c，随后被 phase1 覆盖，无行为差）。
//!
//! DIVERGE：二进制事件名 `PlaySheepPetAnimation`，crate 以 `PlaySheepPet` 承载（lib.rs
//! 事件注册仍旧名，待对齐）；音频槽写（`mov dword [rbp],ecx`）为查询列值，不建模。

use bevy_ecs::prelude::*;
use country_core::resources::time::GameTime;

use crate::components::Sheep;

/// 抚摸请求事件（宿主 UI 写）。
#[derive(Event, Clone, Copy, Debug, PartialEq)]
pub struct PlaySheepPet {
    pub target: u64,
}

/// sheep_pet_animation（0x14231ac20）：每帧取首只羊，按事件命中分派 pet 动画 handler。
pub fn sheep_pet_animation(
    time: Res<GameTime>,
    mut events: EventReader<PlaySheepPet>,
    mut query: Query<&mut Sheep>,
) {
    let dt = time.delta;
    let targets: Vec<u64> = events.read().map(|e| e.target).collect();
    if let Some(mut sheep) = query.iter_mut().next() {
        let matched = targets.contains(&sheep.pet_target);
        // 跳表 0x14304b178 按 phase 分派（未匹配/匹配均传 0.3/1.0/0.2 与匹配标志）。
        advance_sheep_pet(&mut sheep, matched, dt);
    }
}

/// 跳表 0x14304b178 handler（0x14231afe4/0x14231b0b3/0x14231b022/0x14231b068/0x14231b006）
/// 的五相位状态机（与 RE-ANALYSIS §1.3 逐语句判等）。
/// 相位 0=idle、1=起、2=保持、3=收、4=完成。参数 `requested`=匹配标志（r11b）。
/// 返回 (推进发生, 是否进入新阶段)。
pub fn advance_sheep_pet(sheep: &mut Sheep, requested: bool, delta: f32) -> (bool, bool) {
    let mut advanced = false;
    let mut entered = false;
    match sheep.pet_phase {
        0 => {
            // h0（0x14231afe4）：仅匹配才启动：`[+0x60]=1`、state→1。
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
            // h1（0x14231b0b3）：`progress += dt/0.3`；`>1.0 → state=2, progress=0`；
            // 否则 state=1 保持；`[+0x58]=progress`（未钳制）。
            let progress = delta / f32::from_bits(0x3e99999a) + sheep.pet_progress;
            sheep.pet_phase = if progress > 1.0 { 2 } else { 1 };
            sheep.pet_progress = if progress > 1.0 { 0.0 } else { progress };
            sheep.amplitude = progress;
            advanced = true;
        }
        2 => {
            // h2（0x14231b022）：`progress += dt/0.2`；`<=1.0 或 匹配 → 保持 state2`，
            // 否则 state→3、progress=0；`[+0x58]=1.0`。
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
            // h3（0x14231b068）：`progress += dt/0.3`；`<=1.0 → 保持`，否则 state→4；
            // `[+0x60]=0`；`[+0x58]=1.0-progress`。
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
            // h4（0x14231b006）：state→0、`[+0x58]=0`。
            sheep.pet_phase = 0;
            sheep.amplitude = 0.0;
            advanced = true;
            entered = true;
        }
        _ => return (false, false),
    }
    (advanced, entered)
}
