//! `replay_decorator.rs` — 装饰回放（0x14080c8d0，fresh Ghidra shared 反编译
//! `work/wave3-decorator-visual/jobs/decomp_0x14080c8d0.c`，997B）。
//!
//! 参数面（O，寄存器）：in_RCX = ResMut<GameTime>（delta `*+0x28`）、in_RDX =
//! ResMut<ReplayBuffer>（speed `*+0x28`、tick 回写 +0x10/+0x1c）、in_R8 =
//! ResMut<DecoratorArchivist>（cull-allow 集合 + tick 回写 +0x10/+0x1c）。
//!
//! 行为（O，fresh）：
//! 1. 取 ReplayBuffer 前端桶（`front_mut_bucket`）；
//! 2. 取桶 `+0x18/+0x20` 的 **`Arc<[DecoratorId]>` cull-allow id 表**（指针置
//!    0 = 取出），逐 id 入 `HashMap<DecoratorId,()>`（archivist cull 集合）→
//!    Arc drop；
//! 3. 扫描命令（桶 `+0x8` 指针 / `+0x10` 长度，**每条 0x30B**，布局
//!    `{tag u8@0, flags@+8, duration f32@+0xc, payload@+0x10, elapsed
//!    f32@+0x18}`；**跳过非 tag-7 命令**，只处理 tag==7 装饰命令）：
//!    - `flags bit0==0` → 立即完成（记移除索引）；
//!    - 否则 `required = max(duration/speed, 0.05)`（**0.05 为 O floor**——
//!      fresh `if (fVar22 <= 0.05) fVar22 = 0.05`，旧模型 0.1 不符已修）；
//!      `required < elapsed` → 完成记索引；
//!    - 未完成命令 `elapsed += delta`；
//! 4. 完成索引**反转**后逐条 `Vec::remove`（0x30B memmove 右移），按 tag 判别
//!    释放载荷（Arc dec/drop 跳表）：0/1/2=RasterEdit Arc、3=TerrainEdit Arc、
//!    4=`drop_in_place<WallReplayCmd>`、5=RoofEdit Arc、6/7=
//!    DecoratorHistoryEdit Arc、8=TreeEdit Arc、9=CrashMiscInfo Mutex、
//!    default=winit PhysicalSize Mutex。
//!
//! core 缺口登记（未实现，逐点）：
//! - core `ReplayCommand` 枚举无 **Decorator 变体**（bucket 展开
//!   `DecoratorV5` → `UnrestoredReplayOwner`）——tag-7 命令无可承载类型；
//! - core `ReplayBucket`（{entry_index, undo, commands, timing}）**无
//!   `Arc<[DecoratorId]>` cull-allow 表字段**（二进制桶 `+0x18/+0x20`）；
//! - 二进制命令时序内嵌于 0x30B 记录（+8/+0xc/+0x18），非 core
//!   `ReplayBucket.timing: Vec<CommandTiming>` 分离模型。
//! 上述缺口闭合后（core 接线 Decorator 命令 + cull 表）本系统即可重建为
//! 判等；当前以结构模型承载并诚实登记，不伪造 tag-7 处理。

use bevy_ecs::prelude::*;

use country_core::resources::history::ReplayBuffer;
use country_core::resources::walls::decorator_archivist::DecoratorArchivist;
use country_core::resources::walls::decorator_storage::DecoratorStorage;
/// 装饰回放命令（0x30B ReplayCmd 布局；O 读取面）。
#[derive(Clone, Debug, PartialEq)]
pub struct DecoratorReplayCmd {
    /// +0x00：tag（7 = 装饰命令，扫描目标；O）。
    pub tag: u8,
    /// +0x08：标志字节（bit0==0 → 立即完成；O）。
    pub flags: u8,
    /// +0x0c：动画时长（秒）。
    pub duration: f32,
    /// +0x10：装饰 id（u32；cull-allow 目标）。
    pub decorator_id: u32,
    /// +0x18：已推进动画时间。
    pub elapsed: f32,
    /// +0x1c：undo 标志。
    pub undo: bool,
}

impl DecoratorReplayCmd {
    /// 装饰命令 tag（O）。
    pub const TAG_DECORATOR: u8 = 7;
    /// 动画门控下限（**O = 0.05**：fresh `replay_decorator` 0x14080c8d0
    /// `if (fVar22 <= 0.05) fVar22 = 0.05`；旧模型 0.1 不符，2026-10-17 修正）。
    pub const DURATION_FLOOR: f32 = 0.05;

    /// 完成判定 + 推进（O：`required = max(duration/speed, 0.05)`；
    /// `required < elapsed` → 完成；未完成 `elapsed += delta`）。
    pub fn advance(&mut self, dt: f32, speed: f32) -> bool {
        if self.flags & 1 == 0 {
            return true;
        }
        let required = (self.duration / speed).max(Self::DURATION_FLOOR);
        let done = required < self.elapsed;
        self.elapsed += dt;
        done
    }
}

/// `replay_decorator` @0x14080c8d0（O 结构，fresh 反编译）。
///
/// core 缺口登记：命令载荷按 [`DecoratorReplayCmd`] 模型推进；`ReplayCommand`
/// Decorator 变体与桶内 `Arc<[DecoratorId]>` cull-allow 表均未在 core 接线
/// （见文件头）——本系统对前端桶按该模型处理，未识别的命令原样保留。
/// **未实现**（core-type-gap，诚实登记）：tag-7 处理/cull-allow/0x30B 命令
/// 布局依赖 core ReplayCommand Decorator 变体 + 桶 cull 表；当前为结构壳。
pub fn replay_decorator<C: Send + Sync + 'static>(
    mut replay: ResMut<ReplayBuffer>,
    mut storage: ResMut<DecoratorStorage>,
    mut archivist: ResMut<DecoratorArchivist<C>>,
    time: Res<bevy_time::Time>,
) {
    // 机器码：取前端桶前先读速度（E0502：桶可变借用期间避免 replay 借用）。
    let speed = replay.speed;
    let Some(bucket) = replay.front_mut_bucket() else {
        return;
    };
    let _ = storage;
    // 命令推进（O 结构；装饰命令模型见 DecoratorReplayCmd）。
    let mut done_indices = Vec::new();
    for (index, command) in bucket.commands.iter_mut().enumerate() {
        let Some(mut cmd) = decorator_cmd_of(command) else {
            continue;
        };
        // 机器码：逐命令 cull-allow（archivist+0x30 集合 insert；U 集合布局）。
        archivist.culling.allowed_ids.insert(cmd.decorator_id);
        if cmd.advance(time.delta_secs_f64() as f32, speed) {
            done_indices.push(index);
        }
    }
    // 完成命令移除（机器码索引反转后逐条 0x30B memmove 移除）。
    for index in done_indices.into_iter().rev() {
        bucket.commands.remove(index);
    }
}

/// 从 core ReplayCommand 提取装饰命令（core 变体缺口登记：当前恒 None，
/// 见文件头；结构按 DecoratorReplayCmd 模型保留）。
fn decorator_cmd_of(_command: &country_core::resources::history::ReplayCommand) -> Option<DecoratorReplayCmd> {
    None
}

const LOG_TARGET: &str = "system_decorator::replay_decorator";
