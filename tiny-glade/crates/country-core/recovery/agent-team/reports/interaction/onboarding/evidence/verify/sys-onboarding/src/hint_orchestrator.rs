//! hint_orchestrator（O 0x14227fc30）：请求 → 提示实体 + BindingHintDisplay 组合子。
//!
//! 组合子（O 反编译，evidence/batch1-combinators.txt）：
//! - `alternate` 8 单态化 0x141c4e4f0/0x141c4e6d0/0x141c4e8b0/0x141c4ead0/0x141c4ecb0、
//!   `adjacent` 0x141c4ee80/0x141c4f060、`together` 0x141c4f230：self 已是同类列表
//!   （判别 0xca/0xcb/0xcc）则 push 到 Vec（容量满 grow_one）；否则新建 2 元素列表。
//! - `filter_to_mode_impl` 0x1422808d0：游戏手柄分支 0xc5→清空/0xc9 保留/列表递归/
//!   其余→Empty；非手柄 0xc4..0xc8 保留/列表递归/0xc9 与 0xcd→Empty。
//! - `cleanup_impl` 0x14227f4a0：列表递归清理→原地压缩非 Empty→剩 1 提升单元素→
//!   全空置 Empty；Graphic 释放载荷置 Empty；Steam 置 Empty；0xc8/0xcd 默认分支仅当
//!   +0/+0xc/+0x18==0xc3 才置 Empty（对真 0xc8 恒假，保留）。
//! - `resolve_steam_impl` 0x14227e5e0：**未实现**（SteamInput 未落库，分歧点）。

use bevy_ecs::prelude::*;
use glam::Vec2;

/// 提示图形参数（O `BindingHintDisplay::Graphic` 载荷：String + Vec2 + u32，
/// 48B 元素内的 40B 载荷）。
#[derive(Clone, Debug, PartialEq)]
pub struct HintGraphicParams {
    pub graphic: String,
    pub size: Vec2,
    pub color: u32,
}

/// 键码 → 显示（组合子元素类型映射，O 单态化元素 tag：0xc6/0xc7/0xc5/嵌套/0xc8）。
impl From<Option<crate::KeyCode>> for BindingHintDisplay {
    fn from(k: Option<crate::KeyCode>) -> Self {
        Self::Key(k)
    }
}
impl From<crate::MouseButton> for BindingHintDisplay {
    fn from(m: crate::MouseButton) -> Self {
        Self::Mouse(m)
    }
}
impl From<HintGraphicParams> for BindingHintDisplay {
    fn from(g: HintGraphicParams) -> Self {
        Self::Graphic(g)
    }
}
impl From<[Option<crate::KeyCode>; 4]> for BindingHintDisplay {
    fn from(k: [Option<crate::KeyCode>; 4]) -> Self {
        Self::Row(k)
    }
}

/// 按键绑定显示（O `system_onboarding::hint_orchestrator::BindingHintDisplay`，
/// 判别 tag 0xc4..0xcc；列表变体内联 `Vec<BindingHintDisplay>`（48B 元素，判别 i32@+0，
/// Vec{ptr,len,cap}@+8）。Rust 版不复制字节布局，判别/分支序与之等价）。
#[derive(Component, Clone, Debug, PartialEq)]
pub enum BindingHintDisplay {
    /// 0xc4
    Empty,
    /// 0xc5
    Graphic(HintGraphicParams),
    /// 0xc6（载荷 8B = Option<KeyCode>；None/未绑=0xc3 哨兵）
    Key(Option<crate::KeyCode>),
    /// 0xc7（2×u16）
    Mouse(crate::MouseButton),
    /// 0xc8 四键组合（4×Option<KeyCode>，render_hints 读 +4/+12/+24/+36）
    Row([Option<crate::KeyCode>; 4]),
    /// 0xc9（Steam 覆盖层字形）
    Steam(crate::SteamInput),
    /// 0xca 或
    Alternate(Vec<BindingHintDisplay>),
    /// 0xcb 相邻
    Adjacent(Vec<BindingHintDisplay>),
    /// 0xcc 同时
    Together(Vec<BindingHintDisplay>),
}

impl BindingHintDisplay {
    /// `alternate<T>`（O 0x141c4e4f0 等 5 单态化）：self 判别==0xca 则 push，否则
    /// 新建 2 元素列表 [self, item]。
    pub fn alternate<T: Into<BindingHintDisplay>>(self, item: T) -> Self {
        let item = item.into();
        match self {
            Self::Alternate(mut list) => {
                list.push(item);
                Self::Alternate(list)
            }
            other => Self::Alternate(vec![other, item]),
        }
    }

    /// `adjacent<T>`（O 0x141c4ee80/0x141c4f060）：同 alternate，列表判别 0xcb。
    pub fn adjacent<T: Into<BindingHintDisplay>>(self, item: T) -> Self {
        let item = item.into();
        match self {
            Self::Adjacent(mut list) => {
                list.push(item);
                Self::Adjacent(list)
            }
            other => Self::Adjacent(vec![other, item]),
        }
    }

    /// `together<T>`（O 0x141c4f230）：同 alternate，列表判别 0xcc。
    pub fn together<T: Into<BindingHintDisplay>>(self, item: T) -> Self {
        let item = item.into();
        match self {
            Self::Together(mut list) => {
                list.push(item);
                Self::Together(list)
            }
            other => Self::Together(vec![other, item]),
        }
    }

    /// `steam()`（O 行 L353）：Steam 覆盖层键帽（经 `resolve_steam_impl`）。
    /// **未实现**：SteamInput 未落库；本处返回自封 Steam（原语义需
    /// `SteamInput::get_action_origin_glyphs` 逐字形映射）。
    pub fn steam(self) -> Self {
        Self::Steam(crate::SteamInput)
    }

    /// `is_some()`（inline，PDB 仅 inlinee）：内容非空（`!Empty`）。语义 I。
    pub fn is_some(&self) -> bool {
        !matches!(self, Self::Empty)
    }

    /// `is_row()`（inline，PDB 行 843）：是否列表/行变体。语义 I（0xca/0xcb/0xcc）。
    pub fn is_row(&self) -> bool {
        matches!(self, Self::Alternate(_) | Self::Adjacent(_) | Self::Together(_))
    }

    /// `filter_to_mode`（O `filter_to_mode_impl` 0x1422808d0）：按输入模式过滤键帽。
    pub fn filter_to_mode(mut self, is_gamepad: bool) -> Self {
        filter_to_mode_impl(&mut self, is_gamepad);
        self
    }

    /// `cleanup`（O `cleanup_impl` 0x14227f4a0）：递归规范化显示树。
    /// owned 顶层（closure$2 反汇编 `XOR EDX,EDX`，0x14228c555）：结果非列表
    /// （`*self-0xca>=3`）→ 把 self 移入新 1 元素 Alternate；递归传 RDX=1 不包裹。
    pub fn cleanup(mut self) -> Self {
        cleanup_impl(&mut self);
        if !self.is_row() {
            self = Self::Alternate(vec![self]);
        }
        self
    }
}

/// `filter_to_mode_impl`（O 0x1422808d0，125）：`uVar3=tag-0xc4` 截断 4。
/// 游戏手柄：0xc5→释放载荷置 Empty；0xc9 保留；0xca/0xcb/0xcc 逐元素递归；其余（含 0xcd）→Empty。
/// 非手柄：tag 0xc4..0xc8（uVar1<5）保留；0xca/0xcb/0xcc（uVar1-6<3）逐元素递归；
/// 0xc9（uVar1==5）→Empty；0xcd（tag-0xc4=9 截断→uVar1=4→uVar1<5）**保留**（Rust 枚举无
/// 0xcd 变体，代码不受影响）。
pub fn filter_to_mode_impl(display: &mut BindingHintDisplay, is_gamepad: bool) {
    if is_gamepad {
        match display {
            BindingHintDisplay::Graphic(_) => *display = BindingHintDisplay::Empty,
            BindingHintDisplay::Steam(_) => {}
            BindingHintDisplay::Alternate(list)
            | BindingHintDisplay::Adjacent(list)
            | BindingHintDisplay::Together(list) => {
                for element in list {
                    filter_to_mode_impl(element, true);
                }
            }
            _ => *display = BindingHintDisplay::Empty,
        }
    } else {
        match display {
            BindingHintDisplay::Empty
            | BindingHintDisplay::Graphic(_)
            | BindingHintDisplay::Key(_)
            | BindingHintDisplay::Mouse(_)
            | BindingHintDisplay::Row(_) => {}
            BindingHintDisplay::Alternate(list)
            | BindingHintDisplay::Adjacent(list)
            | BindingHintDisplay::Together(list) => {
                for element in list {
                    filter_to_mode_impl(element, false);
                }
            }
            BindingHintDisplay::Steam(_) => *display = BindingHintDisplay::Empty,
        }
    }
}

/// `cleanup_impl`（O 0x14227f4a0，73）：
/// - 0xc4/0xc5/0xc7 不变；0xc6 Key 的 +4 载荷==0xc3（未绑）→Empty；
/// - 0xc9 Steam →Empty；
/// - 0xca/0xcb/0xcc：逐元素递归 cleanup → 原地把非 Empty 前移压缩（首个 Empty 之后）
///   → 剩 1 个则提升为单元素（丢 Vec）→ 全空则丢 Vec 置 Empty；
/// - 0xc8/0xcd 默认分支：+0/+0xc/+0x18==0xc3 才 Empty（真 0xc8 判别 0xc8≠0xc3 恒不触发，
///   故 Row 保留——按机器码原样）。
/// owned 顶层包裹（`cleanup(self)`，对应末尾 in_RDX==0 + closure$2 XOR EDX,EDX）：结果
/// 非列表（`*self-0xca>=3`）→ 移入新 1 元素 Alternate；递归传 RDX=1 不包裹。
pub fn cleanup_impl(display: &mut BindingHintDisplay) {
    let is_together = matches!(display, BindingHintDisplay::Together(_));
    match display {
        BindingHintDisplay::Empty
        | BindingHintDisplay::Graphic(_)
        | BindingHintDisplay::Mouse(_) => {}
        BindingHintDisplay::Key(key) => {
            if key.is_none() {
                *display = BindingHintDisplay::Empty;
            }
        }
        BindingHintDisplay::Steam(_) => *display = BindingHintDisplay::Empty,
        BindingHintDisplay::Row(_) => {}
        BindingHintDisplay::Alternate(list) | BindingHintDisplay::Adjacent(list) => {
            if list.is_empty() {
                *display = BindingHintDisplay::Empty;
                return;
            }
            for element in list.iter_mut() {
                cleanup_impl(element);
            }
            // 原地压缩：非 Empty 前移，去掉 Empty（O 首个 Empty 之后移位）。
            list.retain(|e| !matches!(e, BindingHintDisplay::Empty));
            match list.len() {
                0 => *display = BindingHintDisplay::Empty,
                1 => *display = list.pop().expect("cleanup promoted single"),
                _ => {}
            }
        }
        BindingHintDisplay::Together(list) => {
            if list.is_empty() {
                *display = BindingHintDisplay::Empty;
                return;
            }
            for element in list.iter_mut() {
                cleanup_impl(element);
            }
            if list.iter().all(|e| matches!(e, BindingHintDisplay::Empty)) {
                *display = BindingHintDisplay::Empty;
            }
            // 非全空则保留列表（O：Together 不做压缩/提升）。
            let _ = is_together;
        }
    }
}

/// 提示类型（O：实体 HintType 组件；值 = tutorial_completion switch 的提示 id，
/// 0..17。0 DeselectTool 1 FocusedEdit 2 Undo 3 DisableSnapping 4 TerrainSteepness
/// 5 TerrainFocus 6 Terrain 7 RoofShape 8 UnlinkWindows 9 ClutterFocus
/// 10 CameraPan 11 CameraRotate 12 CameraZoom 13..17 Stairs 五阶段）。
#[derive(Component, Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum HintType {
    DeselectTool = 0,
    FocusedEdit = 1,
    Undo = 2,
    DisableSnapping = 3,
    TerrainSteepness = 4,
    TerrainFocus = 5,
    Terrain = 6,
    RoofShape = 7,
    UnlinkWindows = 8,
    ClutterFocus = 9,
    CameraPan = 10,
    CameraRotate = 11,
    CameraZoom = 12,
    StairsPlaced = 13,
    StairsFocus = 14,
    StairsContinue = 15,
    StairsDetach = 16,
    StairsCanDetach = 17,
}

/// 提示放置（O：HintPlacement 值集部分恢复；hint_position_update 跳转表 6/7/8/10，
/// 其余未恢复）。
#[derive(Component, Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum HintPlacement {
    Cursor = 6,
    WorldPoint = 7,
    Bounds = 8,
    ScreenCenter = 10,
}

/// 提示布局记忆（HintLayoutMemo；position/draw 消费）。
#[derive(Component, Clone, Debug, Default, PartialEq)]
pub struct HintLayoutMemo {
    pub anchor: Vec2,
    pub extent: Vec2,
}

/// 提示进度条（HintProgressBar）。
#[derive(Component, Clone, Copy, Debug, Default, PartialEq)]
pub struct HintProgressBar {
    pub level: u8,
}

/// 交互提示（InteractionPrompt；orchestrator 在 `*(int*)req==10` 时附加）。
#[derive(Component, Clone, Copy, Debug, Default, PartialEq)]
pub struct InteractionPrompt {
    pub active: bool,
}

/// 显示提示请求（O：DisplayHintRequest 0x58B；id@+0x49、进度条@+0x48、flags 位义 U）。
/// 载荷 = BindingHintDisplay + HintType id + placement。
#[derive(Event, Clone, Debug, PartialEq)]
pub struct DisplayHintRequest {
    pub display: BindingHintDisplay,
    pub hint_type: HintType,
    pub placement: HintPlacement,
}

/// 停留提示请求（LingerHintRequest；与 DisplayHintRequest 同族）。
#[derive(Event, Clone, Debug, PartialEq)]
pub struct LingerHintRequest {
    pub display: BindingHintDisplay,
    pub hint_type: HintType,
    pub duration: f32,
}

/// hint_orchestrator（O 0x14227fc30，3035）。
/// 真逻辑（O）：收集本帧 DisplayHintRequest 到 Vec → 按 closure$0（行 152）**稳定排序**
/// （len≤0x15 insertion_sort_shift_left，否则 driftsort_main）→ 读当前提示实体游标 →
/// 对排序后首请求：同 id → insert 新 HintPlacement + remove HintMarkedForDeletion /
/// HintDeleteAfterCooldown（warn）；类型变 → insert HintMarkedForDeletion；否则按
/// DAT_143038ae8[id] 冷却 → insert HintDeleteAfterCooldown；无同 id 且未完成 →
/// Commands::spawn(HintLayoutMemo,HintSpring,HintAnimation,HintText,HintType,
/// HintPlacement)，`*(int*)req==10` 再插 InteractionPrompt，`*(char*)(req+0x48)!=0`
/// 再插 HintProgressBar；文本 lang_lib::lookup（bVar2==0x12/==2 走格式化模板
/// DAT_1430388f0/0x1430388b8）。
/// **未实现**：closure$0 排序键 U；DisplayHintRequest 完整布局 U；EscMenuState 门控
/// `(iVar3==0xd)|is_open` 已确认；依赖 GameProfile 完成标志 + 事件注册面。仅登记骨架，
/// 不伪造文本/实体增删逻辑。
pub fn hint_orchestrator(
    _commands: Commands,
    _esc: Res<country_core::resources::gui::EscMenuState>,
    _requests: EventReader<DisplayHintRequest>,
    _linger: EventReader<LingerHintRequest>,
    _query: Query<(Entity, &HintType, Option<&BindingHintDisplay>)>,
) {
    // 未实现：见模块注释分歧点（closure$0 排序键、请求布局、lang_lib 模板）。
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn alternate_pushes_into_existing_list() {
        // O 0x141c4e4f0：self 判别==0xca 则 push，否则新建 2 元素列表。
        let seed = BindingHintDisplay::Key(Some(1));
        let list = seed.alternate(Some(2)).alternate(Some(3));
        match &list {
            BindingHintDisplay::Alternate(v) => {
                assert_eq!(v.len(), 3);
                assert_eq!(v[0], BindingHintDisplay::Key(Some(1)));
                assert_eq!(v[1], BindingHintDisplay::Key(Some(2)));
                assert_eq!(v[2], BindingHintDisplay::Key(Some(3)));
            }
            other => panic!("expected Alternate, got {other:?}"),
        }
    }

    #[test]
    fn adjacent_and_together_push_into_own_lists() {
        // O 0x141c4ee80/0x141c4f230：self 判别==0xcb/0xcc 则 push。
        let a = BindingHintDisplay::Empty
            .adjacent(crate::MouseButton::Left)
            .adjacent(crate::MouseButton::Right);
        assert!(matches!(a, BindingHintDisplay::Adjacent(v) if v.len() == 3));
        let t = BindingHintDisplay::Empty
            .together(crate::MouseButton::Left)
            .together(crate::MouseButton::Right);
        assert!(matches!(t, BindingHintDisplay::Together(v) if v.len() == 3));
        // 不跨列表合并：adjacent 不 push 进 Alternate（新建 2 元素列表）。
        let x = BindingHintDisplay::Empty
            .alternate(Some(1u32))
            .adjacent(Some(2u32));
        assert!(matches!(x, BindingHintDisplay::Adjacent(v) if v.len() == 2));
    }

    #[test]
    fn filter_to_mode_gamepad_removes_key_mouse_keeps_steam() {
        // O 0x1422808d0：手柄 0xc5→Empty、0xc9 保留、列表递归、其余→Empty。
        let d = BindingHintDisplay::Key(Some(1))
            .alternate(BindingHintDisplay::Steam(crate::SteamInput))
            .alternate(BindingHintDisplay::Mouse(crate::MouseButton::Left));
        let filtered = d.filter_to_mode(true);
        match &filtered {
            BindingHintDisplay::Alternate(v) => {
                assert!(matches!(v[0], BindingHintDisplay::Empty));
                assert!(matches!(v[1], BindingHintDisplay::Steam(_)));
                assert!(matches!(v[2], BindingHintDisplay::Empty));
            }
            other => panic!("expected Alternate, got {other:?}"),
        }
    }

    #[test]
    fn filter_to_mode_non_gamepad_keeps_keys_empties_steam() {
        let d = BindingHintDisplay::Key(Some(1)).alternate(BindingHintDisplay::Steam(
            crate::SteamInput,
        ));
        let filtered = d.filter_to_mode(false);
        match &filtered {
            BindingHintDisplay::Alternate(v) => {
                assert!(matches!(v[0], BindingHintDisplay::Key(_)));
                assert!(matches!(v[1], BindingHintDisplay::Empty));
            }
            other => panic!("expected Alternate, got {other:?}"),
        }
    }

    #[test]
    fn cleanup_promotes_single_and_empties_all() {
        // O 0x14227f4a0：列表递归→压缩→剩 1 提升；全空置 Empty；owned 顶层把非列表
        // 结果包成 1 元素 Alternate。
        let single = BindingHintDisplay::Key(Some(1))
            .alternate(BindingHintDisplay::Empty)
            .alternate(BindingHintDisplay::Empty);
        assert_eq!(
            single.cleanup(),
            BindingHintDisplay::Alternate(vec![BindingHintDisplay::Key(Some(1))])
        );

        let all_empty = BindingHintDisplay::Empty
            .alternate(BindingHintDisplay::Empty)
            .alternate(BindingHintDisplay::Empty);
        assert_eq!(
            all_empty.cleanup(),
            BindingHintDisplay::Alternate(vec![BindingHintDisplay::Empty])
        );

        // Together 非全空则保留列表（O：不做压缩/提升）；结果已是列表 → 不包裹。
        let together = BindingHintDisplay::Key(Some(1)).together(BindingHintDisplay::Empty);
        assert!(matches!(together.cleanup(), BindingHintDisplay::Together(v) if v.len() == 2));
    }

    #[test]
    fn cleanup_owned_wraps_non_list_into_single_alternate() {
        // O closure$2 反汇编 XOR EDX,EDX（owned 顶层）：非列表结果包成 1 元素 Alternate；
        // 递归（RDX=1）不包裹。
        let key = BindingHintDisplay::Key(Some(1));
        assert_eq!(
            key.clone().cleanup(),
            BindingHintDisplay::Alternate(vec![key.clone()])
        );
        // 结果已是列表 → 返回原列表。
        let list = BindingHintDisplay::Key(Some(1)).alternate(BindingHintDisplay::Key(Some(2)));
        assert!(matches!(list.clone().cleanup(), BindingHintDisplay::Alternate(v) if v.len() == 2));
    }

    #[test]
    fn cleanup_empties_unbound_key() {
        // O：Key +4 载荷==0xc3（未绑）→ Empty；Steam → Empty；owned 顶层再包成 1 元素
        // Alternate。
        assert_eq!(
            BindingHintDisplay::Key(None).cleanup(),
            BindingHintDisplay::Alternate(vec![BindingHintDisplay::Empty])
        );
        assert_eq!(
            BindingHintDisplay::Steam(crate::SteamInput).cleanup(),
            BindingHintDisplay::Alternate(vec![BindingHintDisplay::Empty])
        );
    }

    #[test]
    fn hint_type_native_values() {
        // O：HintType 组件值 = 提示 id 0..17。
        assert_eq!(HintType::CameraPan as u8, 10);
        assert_eq!(HintType::CameraRotate as u8, 11);
        assert_eq!(HintType::CameraZoom as u8, 12);
        assert_eq!(HintType::DeselectTool as u8, 0);
    }
}
