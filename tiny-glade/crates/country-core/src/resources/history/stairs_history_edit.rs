//! B `resources/history/stairs_history_edit.rs` — 楼梯编辑负载（章节 O；12 函数）。
//!
//! `StairsHistoryEdit`/`Diff`：边对 before/after + hash；运行时 `StairsDiffTable`
//! 96B = 两 Vec<u32> 边对（cap-first）。serde 0x140259af0 / 0x140a18ee0。
//! 回放：`replay_stairs`（undo==0 插边集 StairsRemovedSupports，!=0 remove_entry；
//! 查附挂装饰 + PlayStairsAudioCmd）。

/// 楼梯边对 = 两 u32（O：add_decorator_diff 0x140a854a0 两 u32 边对循环；
/// StairsSnapshot edges 同为 Vec<(u32,u32)>）。
pub type StairsEdge = (u32, u32);

/// 楼梯历史编辑载荷（边对 before/after + hash；字段序按章节）。
#[derive(Clone, Debug, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct StairsHistoryEdit {
    pub before: Vec<StairsEdge>,
    pub after: Vec<StairsEdge>,
    pub hash: u64,
}

/// 楼梯边集差异（运行时 `StairsDiffTable` 96B = 两 Vec<u32>）。
#[derive(Clone, Debug, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct StairsDiff {
    pub before: Vec<StairsEdge>,
    pub after: Vec<StairsEdge>,
}

/// 楼梯回放命令（B `replay_stairs` 0x140af75b0 的 0x30B 命令，tag 10）。
/// 命令面：`{tag@0=10, flags byte@+0x08 (bit0=instant), duration f32@+0x0c,
/// payload@+0x10, elapsed f32@+0x18, undo@+0x1c}`；payload+0x10 = 边记录
/// `{edge1 u32, edge2 u32, 标志字节}`。
#[derive(Clone, Debug, PartialEq)]
pub struct StairsReplayCmd {
    pub edge1: u32,
    pub edge2: u32,
    /// 动画时长（原始命令 @+0x0c；默认值 U，沿用历史回放常用下限 0.1）。
    pub duration: f32,
    /// 已推进动画时间（@+0x18）。
    pub elapsed: f32,
    /// bit0=instant（@+0x08）。
    pub instant: bool,
    /// undo（@+0x1c）。
    pub undo: bool,
    /// 命令完成标志（本实现持有；原始以命令移除表达）。
    done: bool,
}

impl StairsReplayCmd {
    /// 默认时长（U：原值待取证；动画门控有 `_UNK_142930d70` 下限）。
    pub const DEFAULT_DURATION: f32 = 0.1;

    pub fn new(edge1: u32, edge2: u32, undo: bool, instant: bool) -> Self {
        Self {
            edge1,
            edge2,
            duration: Self::DEFAULT_DURATION,
            elapsed: 0.0,
            instant,
            undo,
            done: false,
        }
    }

    pub fn is_done(&self) -> bool {
        self.done
    }

    /// 边记录 = 差集（undo 语义在回放系统内按 undo 分支处理）。
    pub fn expanded(edit: &StairsHistoryEdit, undo: bool, instant: bool) -> Vec<Self> {
        // 原编辑移除的支撑边 = before − after；redo 再移除、undo 恢复移除。
        let mut before: hashbrown::HashSet<_> = edit.before.iter().copied().collect();
        for edge in &edit.after {
            before.remove(edge);
        }
        before
            .into_iter()
            .map(|(a, b)| Self::new(a, b, undo, instant))
            .collect()
    }

    /// 动画推进：`elapsed <= max(duration/speed, 下限)` 时**先判后加** `elapsed += dt`
    /// 等下一帧（O decomp-0x140af75b0：`if (elapsed <= required) { elapsed += dt; }`）。
    /// 返回是否应推进（未就绪 → 等下一帧）。
    pub fn advance(&mut self, dt: f32, speed: f32) -> bool {
        if self.instant {
            return true;
        }
        let required = (self.duration / speed).max(REPLAY_DURATION_FLOOR);
        if self.elapsed <= required {
            self.elapsed += dt;
            return false;
        }
        true
    }

    /// 完成处理（置 done）。
    pub fn finish(&mut self) {
        self.done = true;
    }
}

/// `replay_stairs` 动画门控下限常量（B `_UNK_142930d70`，U：精确值待取证）。
pub const REPLAY_DURATION_FLOOR: f32 = 0.1;
