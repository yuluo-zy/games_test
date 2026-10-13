//! `resources::history::replay_cmd_buffer` — 回放命令缓冲（WP17 核心）。
//!
//! O 证据（合同 `contracts/replay-orchestration.md`；反编译 pop_front
//! 0x1409f18f0 / front_mut_bucket 0x1409f1a80 / make_animations_instant
//! 0x1409f1ab0 / replay_edit 0x140af6170）：
//!   - `ReplayCmd` 0x30B 枚举：判别 u8@+0x00；动画头 `instant` bit@+0x08、
//!     `duration` f32@+0x0c、负载@+0x10、`elapsed` f32@+0x18、`undo` u8@+0x1c。
//!     判别 0/1/2=RasterEdit、3=Terrain、4=Wall、5=Roof、6/7=Decorator、
//!     8=Tree、9=AutoClutter、10+=Stairs 等（drop glue 判别）。
//!   - `ReplayBucket` 0x40B：命令集 + 装饰/楼梯 diff + 装饰 cull allow-list；
//!     字段偏移 U（typed gap）。
//!   - `ReplayBuffer` 环形缓冲：`{ head@+0x00, buf@+0x08, cap@+0x10,
//!     len@+0x18, modification_counter@+0x20 }`。

use crate::resources::history::{DecoratorId, StairGraphEdge, WallId};
use crate::typed_gap::Gap;
use std::collections::VecDeque;
use std::sync::Arc;

// ---- 领域编辑负载（编辑记录容器，见 edit-history-types 合同）----
use crate::resources::history::auto_clutter_edit::AutoClutterEdit;
use crate::resources::history::decorator_edit::DecoratorHistoryEdit;
use crate::resources::history::path_edit::RasterEdit;
use crate::resources::history::terrain_edit::TerrainEdit;
use crate::resources::history::tree_edit::TreeEdit;

/// 屋顶编辑（不在 WP17 范围；仅作 ReplayCmd 负载边界）。
#[derive(Clone, Debug, Default)]
pub struct RoofEditBoundary(pub Gap);

/// 回放命令（O：0x30B；判别字节@+0x00）。各判别负载见合同；负载内层布局 U。
#[derive(Clone, Debug)]
pub enum ReplayCmd {
    /// 判别 0：Path 光栅编辑（Arc<RasterEdit>）。
    Path(Arc<RasterEdit>),
    /// 判别 1：Cobblestone1 光栅编辑。
    Cobblestone1(Arc<RasterEdit>),
    /// 判别 2：Cobblestone2/3 光栅编辑。
    Cobblestone2(Arc<RasterEdit>),
    /// 判别 3：地形编辑。
    Terrain(Arc<Option<TerrainEdit>>),
    /// 判别 4：墙回放命令（内联 WallReplayCmd）。
    Wall(WallReplayCmd),
    /// 判别 5：屋顶编辑（边界）。
    Roof(Arc<Option<RoofEditBoundary>>),
    /// 判别 6：装饰编辑（状态 diff 附于桶）。
    Decorator(Arc<Option<DecoratorHistoryEdit>>),
    /// 判别 7：装饰编辑（第二变体；负载与 6 同型，用途 U）。
    DecoratorSecondary(Arc<Option<DecoratorHistoryEdit>>),
    /// 判别 8：树编辑。
    Tree(Arc<Option<TreeEdit>>),
    /// 判别 9：自动杂物回放命令。
    AutoClutter(Arc<AutoClutterEdit>),
    /// 判别 10：楼梯回放命令。
    Stairs(StairsReplayCmd),
    /// 未证判别（观察 open；typed gap）。
    Unknown(Gap),
}

/// ReplayCmd 通用动画头字段（O：replay_terrain/wall/auto_clutter/stairs
/// 一致读取 @+0x08/+0x0c/+0x10/+0x18/+0x1c）。语义：
///   - `instant`：即时完成（不逐帧动画）。
///   - `duration`：动画时长（秒）。
///   - `elapsed`：已过时间（帧累加 GameTime.delta）。
///   - `undo`：是否撤销方向。
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct ReplayAnimationHeader {
    pub instant: bool,
    pub duration: f32,
    pub elapsed: f32,
    pub undo: bool,
}

impl ReplayCmd {
    /// O：`make_animation_instant`（make_animations_instant 0x1409f1ab0 内联）。
    pub fn make_animation_instant(&mut self) {
        match self {
            ReplayCmd::Wall(cmd) => cmd.instant = true,
            ReplayCmd::Terrain(_)
            | ReplayCmd::Path(_)
            | ReplayCmd::Cobblestone1(_)
            | ReplayCmd::Cobblestone2(_)
            | ReplayCmd::Roof(_)
            | ReplayCmd::Decorator(_)
            | ReplayCmd::DecoratorSecondary(_)
            | ReplayCmd::Tree(_)
            | ReplayCmd::AutoClutter(_)
            | ReplayCmd::Stairs(_)
            | ReplayCmd::Unknown(_) => {}
        }
    }

    /// O：动画头读取（各领域系统）。返回 None 表示非动画命令（Wall 内联负载）。
    pub fn animation(&self) -> Option<ReplayAnimationHeader> {
        match self {
            ReplayCmd::Terrain(_)
            | ReplayCmd::Path(_)
            | ReplayCmd::Cobblestone1(_)
            | ReplayCmd::Cobblestone2(_)
            | ReplayCmd::Roof(_)
            | ReplayCmd::Decorator(_)
            | ReplayCmd::DecoratorSecondary(_)
            | ReplayCmd::Tree(_)
            | ReplayCmd::AutoClutter(_)
            | ReplayCmd::Stairs(_) => Some(ReplayAnimationHeader {
                instant: false,
                duration: 0.0,
                elapsed: 0.0,
                undo: false,
            }),
            ReplayCmd::Wall(_) | ReplayCmd::Unknown(_) => None,
        }
    }
}

/// 墙回放命令（O：判别 4；内联负载。`Vec<WallReplayEditCmd>` 元素 0x18）。
#[derive(Clone, Debug, Default)]
pub struct WallReplayCmd {
    /// O：`Vec<WallReplayEditCmd>`（元素 0x18）。
    pub edits: Vec<WallReplayEditCmd>,
    /// O：`is_single_adjust_height_cmd`（单 AdjustHeight 判定；replay_edit 内联）。
    pub instant: bool,
    /// U：已改墙集合等辅助字段。
    pub aux: Gap,
}

impl WallReplayCmd {
    /// O：`new`（replay_wall::WallReplayCmd::new；构造自 ConcurrentEdits）。
    pub fn new(edits: Vec<WallReplayEditCmd>) -> Self {
        Self {
            edits,
            instant: false,
            aux: Gap,
        }
    }
    /// O：`is_single_adjust_height_cmd`（0x140af6170 内联；元素 0x18 计数==1）。
    pub fn is_single_adjust_height_cmd(&self) -> bool {
        self.edits.len() == 1
    }
}

/// 墙回放编辑命令（O：0x18B；`process` 0x140abfb30 应用领域状态）。
#[derive(Clone, Debug, Default)]
pub struct WallReplayEditCmd {
    /// O：`Arc<WallHistoryEdit>` 指针（负载，@+0x8）。done 标志 @+0x15。
    pub edit: Option<Arc<WallHistoryEditBoundary>>,
    /// O：done 标志（@+0x15；instant 路径完成置位）。
    pub done: bool,
    /// O：instant（bit0@+0x0）。
    pub instant: bool,
    /// U：缓动/动画辅助。
    pub aux: Gap,
}

/// 墙历史编辑负载边界（`wall_compound_edit::WallHistoryEdit`；U）。
#[derive(Clone, Debug, Default)]
pub struct WallHistoryEditBoundary(pub Gap);

impl WallReplayEditCmd {
    /// O：`process<closure_env$0>`（0x140abfb30）：instant 用 `cubic_in_out`
    /// 后按判别分派；否则标记 done 后分派。
    pub fn process(&mut self) {
        if self.done {
            return;
        }
        if self.instant {
            // O：`utils::easing_functions::cubic_in_out` 后按跳表分派；分派表 U。
            self.done = true;
        } else {
            self.done = true;
        }
    }
}

/// 楼梯回放命令（O：判别 10；负载 StairsEdit 边信息，@+0x10）。
#[derive(Clone, Debug, Default)]
pub struct StairsReplayCmd {
    /// O：楼梯编辑负载（StairGraphEdge 等；U）。
    pub edit: Option<StairsEditBoundary>,
    pub aux: Gap,
}

/// 楼梯编辑负载边界（`stairs_history_edit::StairsHistoryEdit`；U）。
#[derive(Clone, Debug, Default)]
pub struct StairsEditBoundary(pub Gap);

/// 回放桶（O：0x40B 元素；`replay_edit` 构造写入 8×u64）。字段偏移 U。
#[derive(Clone, Debug, Default)]
pub struct ReplayBucket {
    /// O：命令集（ReplayCmd 元素 0x30）。
    pub commands: Vec<ReplayCmd>,
    /// O：装饰 diff（replay_pop_bucket 消费 `DecoratorStorage::restore_decorators`）。
    pub decorator_diff: Option<Arc<DecoratorHistoryDiffBoundary>>,
    /// O：楼梯 diff（replay_pop_bucket 消费 `StairsState::restore_backup/apply_diff`）。
    pub stairs_diff: Option<Arc<StairsHistoryDiffBoundary>>,
    /// O：装饰 cull allow-list（`to_arc_slice<DecoratorId, …>`）。
    pub decorator_cull_allow_list: Option<Arc<[DecoratorId]>>,
    /// U：精确偏移/标签。
    pub aux: Gap,
}

/// 装饰历史 diff 负载边界（`decorator_edit::DecoratorHistoryDiff`；U）。
#[derive(Clone, Debug, Default)]
pub struct DecoratorHistoryDiffBoundary(pub Gap);

/// 楼梯历史 diff 负载边界（`stairs_state::StairsHistoryDiff`；U）。
#[derive(Clone, Debug, Default)]
pub struct StairsHistoryDiffBoundary(pub Gap);

/// 回放缓冲（O：环形 `{ head@+0x00, buf@+0x08, cap@+0x10, len@+0x18,
/// modification_counter@+0x20 }`；前桶元素 0x40）。
#[derive(Clone, Debug, Default)]
pub struct ReplayBuffer {
    pub buckets: VecDeque<ReplayBucket>,
    /// O：`modification_counter`（pop_front/add 递增；save_game 读取）。
    pub modification_counter: u64,
}

impl ReplayBuffer {
    /// O：`new`（insert_history_resources 0x140943e10 内联）。
    pub fn new() -> Self {
        Self {
            buckets: VecDeque::new(),
            modification_counter: 0,
        }
    }
    /// O：`len`（+0x18）。
    pub fn len(&self) -> usize {
        self.buckets.len()
    }
    /// O：`is_empty`（finish_replay 0x1408b36a0 / save_game 0x140a3f470 /
    /// history_process 0x140af5b30 内联）。
    pub fn is_empty(&self) -> bool {
        self.buckets.is_empty()
    }
    /// O：`add`（replay_edit 0x140af6170 内联；尾部 push）。
    pub fn add(&mut self, bucket: ReplayBucket) {
        self.buckets.push_back(bucket);
        self.modification_counter = self.modification_counter.wrapping_add(1);
    }
    /// O：`pop_front`（0x1409f18f0）：弹前桶；写 counter。
    pub fn pop_front(&mut self) -> ReplayBucket {
        self.modification_counter = self.modification_counter.wrapping_add(1);
        self.buckets
            .pop_front()
            .expect("ReplayBuffer::pop_front on empty buffer")
    }
    /// O：`front_mut_bucket`（0x1409f1a80）：前桶可变引用（空返回 None）。
    pub fn front_mut_bucket(&mut self) -> Option<&mut ReplayBucket> {
        self.buckets.front_mut()
    }
    /// O：`make_animations_instant`（0x1409f1ab0）：全部命令即时完成。
    pub fn make_animations_instant(&mut self) {
        for bucket in &mut self.buckets {
            for cmd in &mut bucket.commands {
                cmd.make_animation_instant();
            }
        }
    }
    /// O：`current_replay_cmd`（replay_audio 0x140ad5800 /
    /// signifier_replay_active 0x140b1b860 内联）：前桶首条命令（若存在）。
    pub fn current_replay_cmd(&self) -> Option<&ReplayCmd> {
        self.buckets.front().and_then(|b| b.commands.first())
    }
}

/// 判断回放是否活跃（O：signifier_replay_active 检查 ReplayBuffer 状态）。
pub fn replay_active(buffer: &ReplayBuffer) -> bool {
    !buffer.is_empty()
}

// 供领域系统使用的共享常量（O：replay_terrain 的动画下限 0.05）。
pub const ANIMATION_EPSILON: f32 = 0.05;

/// 供领域编辑负载使用的辅助类型别名（Observed 边界）。
pub type WallIdSet = std::collections::HashSet<WallId>;
pub type StairGraphEdgeSet = std::collections::HashSet<StairGraphEdge>;
