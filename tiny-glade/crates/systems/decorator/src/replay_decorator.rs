//! `replay_decorator.rs` — 装饰回放（0x14080c8d0，O）。
//!
//! 机器码证据（fresh RE-MCP `groupD/decomp/decomp-0x14080c8d0-*.txt`）：
//! `(ResMut<ReplayBuffer>, ResMut<DecoratorStorage/Archivist>, ResMut<Time>)`。
//!
//! 行为（O）：
//! 1. 取 ReplayBuffer 前端桶（`front_mut_bucket` 0x1409f1a80）；
//! 2. 取出桶的命令 Vec（Arc drop 语义），逐条取命令 `+0x10` 的 u32 decorator
//!    id → `DecoratorArchivist::decorator_cull_allow`（FUN_1402c9200 =
//!    HashSet insert；archivist+0x30 区域，U：cull 集合布局）；
//! 3. 扫描命令（0x30B ReplayCmd 布局 `{tag@0, flags@+8, duration f32@+0xc,
//!    payload@+0x10, elapsed f32@+0x18, undo@+0x1c}`；tag==7 = 装饰命令）：
//!    flags bit0==0 → 立即完成（记移除索引）；否则 `required =
//!    max(duration/speed, 0.1)`（_UNK_142930d70 floor）；required < elapsed →
//!    完成记索引；随后 `elapsed += dt`；
//! 4. 完成索引列表反转后逐条移除命令（0x30B memmove 右移），按 tag 判别
//!    释放载荷（Arc dec/drop 跳表 tag 0..9+）；
//! 5. 释放索引列表。
//!
//! core 缺口登记：core `ReplayCommand` 无 Decorator 变体（bucket 展开
//! `DecoratorV5` → `UnrestoredReplayOwner`）；本系统按核心 ReplayBuffer API
//! 面实现结构（front_mut_bucket + 命令推进），装饰命令变体由集成 owner 在
//! core 接线后启用。

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
    /// 动画门控下限（B `_UNK_142930d70`，O 读取面；数值 0.1 与 stairs 族一致）。
    pub const DURATION_FLOOR: f32 = 0.1;

    /// 完成判定 + 推进（O：`required = max(duration/speed, 0.1)`；
    /// `required < elapsed` → 完成；`elapsed += dt`）。
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

/// `replay_decorator` @0x14080c8d0（O 结构）。
///
/// core 缺口登记：命令载荷按 [`DecoratorReplayCmd`] 模型推进；`ReplayCommand`
/// Decorator 变体未在 core 接线（见文件头）——本系统对前端桶按该模型处理，
/// 未识别的命令原样保留。
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
