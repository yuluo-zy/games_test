//! B `resources/history/stairs_history_edit.rs` — 楼梯编辑负载（章节 O；12 函数）。
//!
//! `StairsHistoryEdit` 原生（fresh decomp-0x140259af0 serialize / 0x140a18ee0
//! deserialize）为**单边枚举**：判别字节 bit0（`*in_RCX & 1`），每变体持单个
//! `StairGraphEdge`（@+0x4）；serde 形状 `{"<变体>": {"edge": <edge>}}`：
//! `{"SupportsDelete": {"edge": ...}}`（bit0==0）/ `{"SupportsRecreate": {"edge": ...}}`
//! （bit0==1）。变体/字段名字符串常量 @0x1429a4c82（"SupportsDelete"，len 0xe）、
//! @0x1429a4c90（"edge"，len 4）、@0x1429a4c94（"SupportsRecreate"，len 0x10）——
//! disasm 0x140259b95/0x140259b2b 直接装载。旧 src `{before, after, hash}` diff-Vec
//! 模型为近似（两形状不同构），已按 fresh 反编译重载为原生枚举形状
//! （REBUILD-REMAINDER §1.1）。运行时差异 `StairsHistoryDiff`（removed/added 边集）
//! 在 `resources::stairs::stairs_state`；回放命令 `StairsReplayCmd`（tag 10）见模块尾。

use crate::resources::stairs::stairs_state::StairGraphEdge;
use serde::ser::SerializeMap;
use serde::{Deserializer, Serializer};

/// 楼梯边对 = 两 u32（O：add_decorator_diff 0x140a854a0 两 u32 边对循环；
/// StairsSnapshot edges 同为 Vec<(u32,u32)>）。
pub type StairsEdge = (u32, u32);

/// 楼梯历史编辑（原生单边枚举；每变体一个 `StairGraphEdge`）。
#[derive(Clone, Debug, PartialEq)]
pub enum StairsHistoryEdit {
    /// bit0==0：记录**删除**的支撑边。
    SupportsDelete(StairGraphEdge),
    /// bit0==1：记录**重建**的支撑边。
    SupportsRecreate(StairGraphEdge),
}

impl StairsHistoryEdit {
    /// 编辑承载边。
    pub fn edge(&self) -> StairGraphEdge {
        match self {
            Self::SupportsDelete(edge) | Self::SupportsRecreate(edge) => *edge,
        }
    }
}

/// `{"edge": <StairGraphEdge>}` 中间载荷（原生 `serialize_entry<str, StairGraphEdge>`
/// 的形状封装；derive 覆盖，判别/键面由 `StairsHistoryEdit` 手写 serde 持有）。
#[derive(Clone, Copy, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
struct EdgeEntry {
    edge: StairGraphEdge,
}

/// 手写 `Serialize`（0x140259af0）：单 entry map
/// `{"<SupportsDelete|SupportsRecreate>": {"edge": <edge>}}`（机器码双分支写
/// `{` + format_escaped_str(variant) + `:` + `{` + serialize_entry("edge") + `}` + `}`）。
impl serde::Serialize for StairsHistoryEdit {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let (key, edge) = match self {
            Self::SupportsDelete(edge) => ("SupportsDelete", *edge),
            Self::SupportsRecreate(edge) => ("SupportsRecreate", *edge),
        };
        let mut map = serializer.serialize_map(Some(1))?;
        map.serialize_entry(key, &EdgeEntry { edge })?;
        map.end()
    }
}

/// 手写 `Deserialize`（0x140a18ee0 手写 visitor：variant_seed + struct_variant
/// 状态机；单 entry map → 变体）。
impl<'de> serde::Deserialize<'de> for StairsHistoryEdit {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = StairsHistoryEdit;
            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("a single-edge stairs history edit")
            }
            fn visit_map<A>(self, mut map: A) -> Result<Self::Value, A::Error>
            where
                A: serde::de::MapAccess<'de>,
            {
                let Some(key) = map.next_key::<String>()? else {
                    return Err(serde::de::Error::custom(
                        "expected one variant key in StairsHistoryEdit",
                    ));
                };
                let entry = map.next_value::<EdgeEntry>()?;
                Ok(match key.as_str() {
                    "SupportsDelete" => StairsHistoryEdit::SupportsDelete(entry.edge),
                    "SupportsRecreate" => StairsHistoryEdit::SupportsRecreate(entry.edge),
                    _ => {
                        return Err(serde::de::Error::unknown_variant(
                            key.as_str(),
                            &["SupportsDelete", "SupportsRecreate"],
                        ))
                    }
                })
            }
        }
        deserializer.deserialize_map(Visitor)
    }
}

/// 楼梯回放命令（B `replay_stairs` 0x140af75b0 的 0x30B 命令，tag 10）。
/// 命令面：`{tag@0=10, flags byte@+0x08 (bit0: 0=instant/立即、1=动画), duration f32@+0x0c,
/// payload@+0x10, elapsed f32@+0x18, undo@+0x1c}`；payload+0x10 = 边记录
/// `{标志字节@+0x10 bit0, edge1 u32@+0x14, edge2 u32@+0x18}`。
///
/// **undo 语义来源（fresh 机器码 0x140af77a5/0x140af7829）**：两条路径都读
/// `payload+0x10 bit0` 选择 insert/remove 分支——bit0==0（`SupportsDelete`）：
/// undo==0→insert、undo!=0→remove_entry；bit0==1（`SupportsRecreate`）反之。
/// 命令 flags bit0 只选即时/动画门控，**不**决定 undo 语义（旧实现按 instant 反向，已纠）。
///
/// **instant 位语义（0x1409f1ab0 make_animations_instant 机器码：默认 case 清 flags byte
/// 置 0 → instant）**：本字段 `instant == true` 对应机器码 flags bit0==0（立即路径）；
/// `false` 对应 bit0==1（动画路径）。
#[derive(Clone, Debug, PartialEq)]
pub struct StairsReplayCmd {
    pub edge1: u32,
    pub edge2: u32,
    /// 动画时长（原始命令 @+0x0c；默认值 U，沿用历史回放常用下限 0.1）。
    pub duration: f32,
    /// 已推进动画时间（@+0x18）。
    pub elapsed: f32,
    /// `true` = 机器码 flags bit0==0（立即路径）；`false` = bit0==1（动画路径）。
    pub instant: bool,
    /// undo（@+0x1c）。
    pub undo: bool,
    /// payload+0x10 bit0：false = `SupportsDelete`（已删除支撑边）、
    /// true = `SupportsRecreate`（已重建支撑边）。决定 undo 语义方向。
    pub recreate: bool,
    /// 命令完成标志（本实现持有；原始以命令移除表达）。
    done: bool,
}

impl StairsReplayCmd {
    /// 默认时长（fresh `generate_replay_cmds_from_edit` 0x14143fbc7（tag 14 Stairs）
    /// 写命令 +0x0c = `0x3e4ccccd` = 0.2；旧登记 0.1 被证伪）。
    pub const DEFAULT_DURATION: f32 = 0.2;

    pub fn new(edge1: u32, edge2: u32, undo: bool, instant: bool) -> Self {
        Self {
            edge1,
            edge2,
            duration: Self::DEFAULT_DURATION,
            elapsed: 0.0,
            instant,
            undo,
            recreate: false,
            done: false,
        }
    }

    /// 带 payload 判别位构造：`recreate = payload+0x10 bit0`
    /// （false = `SupportsDelete`、true = `SupportsRecreate`）。
    pub fn new_with_kind(
        edge1: u32,
        edge2: u32,
        undo: bool,
        instant: bool,
        recreate: bool,
    ) -> Self {
        Self {
            recreate,
            ..Self::new(edge1, edge2, undo, instant)
        }
    }

    pub fn is_done(&self) -> bool {
        self.done
    }

    /// 边记录 = 编辑承载边（原生单边枚举：每编辑一个边，展开为单条命令；
    /// payload 判别位 = 枚举变体：`SupportsDelete` → 0、`SupportsRecreate` → 1）。
    pub fn expanded(edit: &StairsHistoryEdit, undo: bool, instant: bool) -> Vec<Self> {
        let (a, b) = edit.edge().nodes();
        let recreate = matches!(edit, StairsHistoryEdit::SupportsRecreate(_));
        vec![Self::new_with_kind(a, b, undo, instant, recreate)]
    }

    /// 动画推进：`elapsed <= max(duration/speed, 下限)` 时**先判后加** `elapsed += dt`
    /// 等下一帧（fresh 0x140af75b0：`elapsed <= required` → 0x140af7862 加 dt 后继续；
    /// 就绪分支 0x140af77a5 也执行 `elapsed += dt` 再处理）。返回是否应处理。
    pub fn advance(&mut self, dt: f32, speed: f32) -> bool {
        if self.instant {
            return true;
        }
        let required = (self.duration / speed).max(REPLAY_DURATION_FLOOR);
        let ready = self.elapsed > required;
        // 两分支都推进 elapsed（机器码 0x140af77a5 / 0x140af7866）。
        self.elapsed += dt;
        ready
    }

    /// 完成处理（置 done）。
    pub fn finish(&mut self) {
        self.done = true;
    }
}

/// `replay_stairs` 动画门控下限常量（B decomp-0x140af75b0：
/// `fVar44 = duration/speed; if (fVar44 <= 0.05) fVar44 = 0.05;`——
/// **下限 0.05**，非旧登记 0.1）。
pub const REPLAY_DURATION_FLOOR: f32 = 0.05;
