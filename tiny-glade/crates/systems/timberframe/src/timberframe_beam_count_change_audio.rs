//! timberframe_beam_count_change_audio.rs — 梁数变化→音频事件（B 0x1421b6610，restorer-state 重建）。
//!
//! ## fresh 反编译（2026-10-17，`evidence/restorer-state-timberframe/decomp-timberframe_beam_count_change_audio.txt`）
//!
//! 4 参（O，寄存器面）：RCX=`EventReader<ConstructTimberframeCmd>`、RDX=`ResMut<TimberframeBeamCount>`
//! （+0x1c tick 回写）、R8=`Res<GameTime>`（只读）、R9=`EventWriter<PlayTimberframeSound>`。
//!
//! 逐语句控制流（O）：
//! 1. 事件游标算术求「新事件数」；`if (无新事件) return;`（游标已推进）。
//! 2. `frame = *(int*)(*game_time + 0x2c)` —— 机器码 = `GameClock.frame_count`（章节 O：u32 @+0x2c）。
//! 3. `plVar3 = *beam_count`；判定
//!    `(frame - (int)plVar3[2] == 1) && (*plVar3 != plVar3[1])` ——
//!    **`(frame - last_frame == 1) && (last_count != count)`**（非空 = 梁数变化过）。
//! 4. 触发：`ShapeParameters::center()`（0x1411ff2a0）→ `utils::swizzlers::x0y()` → 构造
//!    `PlayTimberframeSound`（kind 低字节 = 6，0x20B 记录）→ `Events::push`（0x20 步长、
//!    +0x40 事件 id 递增）。`get_building_size_01()`（0x1417d3f10）结果写入事件 +0x10 ——
//!    **country-core `PlayTimberframeSound` 无 size/id 字段，GAP**。
//! 5. 回写：`*(int*)(BeamCount+0x10) = frame`、`*(i64*)(BeamCount+0) = *(i64*)(BeamCount+8)`、
//!    ResMut tick 回写（bevy 自动）。
//!
//! ## TimberframeBeamCount 布局（O，fresh 交叉）
//! - `+0x00` i64 `last_count`：音频上次看到的梁数（`*plVar3 != plVar3[1]` 非空比较；回写 `*plVar3=plVar3[1]`）。
//! - `+0x08` i64 `count`：矩形/圆形构造系统写入的当前梁数（`*(longlong*)(BeamCount+8)=lVar19`）。
//! - `+0x10` i32 `last_frame`：音频上次看到的帧计数（`*(int*)(plVar3+2)=iVar10`）。
//!
//! 旧 `{current, previous, non_empty}` 布局为 I 级近似，已按机器码面修正。

use bevy_ecs::prelude::*;
use glam::Vec3;

use country_core::resources::events::ConstructTimberframeCmd;
use country_core::resources::time::GameTime;
use country_core::systems::audio::PlayTimberframeSound;
use country_core::systems::wall::valid_enclosure::ShapeParameters;

/// 梁计数资源（O 布局：`last_count` i64 @+0x00、`count` i64 @+0x08、`last_frame` i32 @+0x10）。
#[derive(Resource, Clone, Copy, Debug, Default)]
pub struct TimberframeBeamCount {
    /// +0x00：音频上次看到的梁数（非空比较 `last_count != count`）。
    pub last_count: i64,
    /// +0x08：构造系统写入的当前梁数。
    pub count: i64,
    /// +0x10：音频上次看到的帧计数。
    pub last_frame: i32,
}

/// 机器码读面 `*(int*)(GameTime+0x2c)` 的裸读（O）。当前 country-core `GameClock` 结构体
/// 实际布局 `frame_count` 字段 @+0x28（reviewer-state 核验；章节证据 @0x2c 待校正）——故不依赖
/// 字段名，按机器码读面取 +0x2c 的 4 字节；该偏移处原结构字段语义 U（跨域 GAP）。
fn game_time_int_at_0x2c(time: &GameTime) -> i32 {
    // SAFETY: `GameClock` 为整块可读内存的 POD，+0x2c..+0x30 在该值边界内；与机器码读面一致。
    unsafe {
        let base = core::ptr::addr_of!(*time).cast::<u8>();
        core::ptr::read_unaligned(base.add(0x2c) as *const i32)
    }
}

/// `timberframe_beam_count_change_audio` @0x1421b6610（重建，主体判等）。
///
/// 触发条件（O）：存在新 `ConstructTimberframeCmd` 事件 **且** `frame - last_frame == 1`
/// **且** `last_count != count` → 投递 `PlayTimberframeSound { kind=6, position=x0y(center) }`。
///
/// 分歧点（GAP，登记不伪造）：
/// - `get_building_size_01`（0x1417d3f10，audio/building_audio 域）未在 country-core 提供
///   → 事件 size 字段（+0x10）未写出；`PlayTimberframeSound` core 类型缺 size/id 字段。
/// - 触发用事件的形状索引：机器码取当前事件指针（本实现取首个未读事件，I）。
/// - 帧读面：机器码读 `*(int*)(GameTime+0x2c)`（O）；reviewer-state 核验 country-core `GameClock`
///   结构体实际布局 `frame_count` @+0x28（≠+0x2c；既有章节证据 @0x2c 待校正）——本实现按机器码
///   面**裸读 +0x2c**，不依赖字段名；原结构 +0x2c 字段语义 U（跨域 GAP）。
pub fn timberframe_beam_count_change_audio(
    mut constructs: EventReader<ConstructTimberframeCmd>,
    mut beams: ResMut<TimberframeBeamCount>,
    time: Res<GameTime>,
    mut sounds: EventWriter<PlayTimberframeSound>,
) {
    // O：无新事件 → 返回（read() 已推进游标）。
    let Some(cmd) = constructs.read().next() else {
        return;
    };

    // O：`*(int*)(*param_3 + 0x2c)` = GameClock 数据 +0x2c 处的 int（帧/计数读面）。
    let frame = game_time_int_at_0x2c(&*time);

    // O：`(frame - last_frame == 1) && (last_count != count)`。
    if frame.wrapping_sub(beams.last_frame) == 1 && beams.last_count != beams.count {
        // O：`ShapeParameters::center`（0x1411ff2a0）+ `x0y()` → 位置（kind=6）。
        // GAP：`get_building_size_01`（0x1417d3f10）→ 事件 size 字段，core 未提供。
        let center = (cmd.shape.len() >= 28)
            .then(|| {
                ShapeParameters::from_observed(cmd.shape[..28].try_into().unwrap()).center()
            })
            .unwrap_or_default();
        sounds.write(PlayTimberframeSound {
            kind: 6,
            position: Vec3::new(center.x, 0.0, center.y),
            // 原生 0x20B 记录：kind@+0、pos@+8、size@+0x10、id@+0x18。core 现以
            // {flag@+1, speed_source@+4, value@+0x14} 承载；本调用点不写这些字段
            // （size 由 get_building_size_01 提供，GAP 未解锁）→ 0。
            flag: 0,
            speed_source: 0.0,
            value: 0.0,
        });
    }

    // O：`*(int*)(BeamCount+0x10) = frame`；`*(i64*)(BeamCount+0) = *(i64*)(BeamCount+8)`。
    beams.last_frame = frame;
    beams.last_count = beams.count;
}
