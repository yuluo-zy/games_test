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
//! - `resolve_steam_impl` 0x14227e5e0：`SteamInput::get_action_origin_glyphs` 逐字形
//!   映射（0→Empty/1→取元素/≥2→Alternate）；**已重建**（2026-10-17，SteamInput 落库）。

use bevy_ecs::prelude::*;
use glam::Vec2;

/// 提示图形参数（O `BindingHintDisplay::Graphic` 载荷）。
///
/// fresh 反编译 0x141c3de00（make_focus_or_camera_binding_hint）证明真实载荷为
/// **32B** = `{Vec<&str>, f32@+0x18, f32@+0x1c}`（Bevy `Vec` 布局 {cap@0, ptr@+8,
/// len@+0x10}）：`alloc(0x20)` 写两条 `&str` `[("hint/mouse",10),("hint/rmb"|"hint/mmb",8)]`
/// → `{cap=2, ptr, len=2, 0x3f333333(0.7), 0x42c80000(100.0)}`，再走一次
/// `alternate<HintGraphicParams>`。**两个图标在同一 Graphic 元素内**，不是两个元素。
/// +0x18/+0x1c 的字段语义 U（按原 f32 值保留）。
#[derive(Clone, Debug, PartialEq)]
pub struct HintGraphicParams {
    /// 图标名列表（原生 `Vec<&'static str>`；make_focus 分支 len=2，
    /// camera/stairs 等单图标写面 len=1）。
    pub graphics: Vec<String>,
    /// native +0x18 f32（make_focus = 0.7f；其余写面未取证，置 0）。
    pub value_0x18: f32,
    /// native +0x1c f32（make_focus = 100.0f；其余写面未取证，置 0）。
    pub value_0x1c: f32,
}

/// 单图标 Graphic 参数（O：camera_*/make_*_binding_hint 构造；字符串 len 与
/// DAT/anon 表逐项核对）。+0x18/+0x1c 对单图标写面未取证，置 0（登记 U）。
pub(crate) fn graphic(name: &str) -> HintGraphicParams {
    HintGraphicParams {
        graphics: vec![name.to_string()],
        value_0x18: 0.0,
        value_0x1c: 0.0,
    }
}

/// Steam 动作源字形查询（O：`country_core::resources::steam::ActionOriginGlyphQuery`
/// (u16)，打包 `(index<<8)|kind`；`BindingHintDisplay::Steam` 载荷）。
pub type SteamQuery = country_core::resources::steam::ActionOriginGlyphQuery;

/// SteamQuery 构造（O：打包动作码）。
pub fn steam_query(code: u16) -> SteamQuery {
    country_core::resources::steam::ActionOriginGlyphQuery(code)
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
impl From<SteamQuery> for BindingHintDisplay {
    fn from(q: SteamQuery) -> Self {
        Self::Steam(q)
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
    /// 0xc9（Steam 覆盖层字形；载荷 = `ActionOriginGlyphQuery` 打包动作码，经
    /// `resolve_steam` → `SteamInput::get_action_origin_glyphs` 逐字形映射）。
    Steam(SteamQuery),
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
    /// 载荷 = 打包动作码查询（O：`impl$30::from`/`impl$31::from` =
    /// `packed_set`/`packed_set_plus_one`）。
    pub fn steam(self, query: SteamQuery) -> Self {
        Self::Steam(query)
    }

    /// `resolve_steam`（O `resolve_steam_impl` 0x14227e5e0）：把树中每个
    /// `Steam(query)` 经 `SteamInput::get_action_origin_glyphs` 解析为字形串；
    /// None → `Empty`；Some(1) → 单元素；Some(n≥2) → `Alternate`。列表递归。
    pub fn resolve_steam(self, steam: &country_core::resources::steam::SteamInput) -> Self {
        match self {
            Self::Steam(query) => {
                match steam.get_action_origin_glyphs(query) {
                    None => Self::Empty,
                    Some(glyphs) => {
                        let items: Vec<Self> = glyphs
                            .into_iter()
                            .map(|glyph| Self::Graphic(HintGraphicParams {
                                graphics: vec![glyph],
                                value_0x18: 0.0,
                                value_0x1c: 0.0,
                            }))
                            .collect();
                        match items.len() {
                            0 => Self::Empty,
                            1 => items.into_iter().next().expect("resolve_steam single"),
                            _ => Self::Alternate(items),
                        }
                    }
                }
            }
            Self::Alternate(list) => {
                Self::Alternate(list.into_iter().map(|e| e.resolve_steam(steam)).collect())
            }
            Self::Adjacent(list) => {
                Self::Adjacent(list.into_iter().map(|e| e.resolve_steam(steam)).collect())
            }
            Self::Together(list) => {
                Self::Together(list.into_iter().map(|e| e.resolve_steam(steam)).collect())
            }
            other => other,
        }
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

/// 提示类型（O：实体 HintType 组件；值 = DisplayHintRequest id@+0x49，
/// 0..17 + 0x12。0 DeselectTool 1 FocusedEdit 2 Undo 3 DisableSnapping 4 TerrainSteepness
/// 5 TerrainFocus 6 Terrain 7 RoofShape 8 UnlinkWindows 9 ClutterFocus
/// 10 CameraPan 11 CameraRotate 12 CameraZoom 13..17 Stairs 五阶段
/// **18 Rollercoaster**（hint_orchestrator 对 id==0x12 有独立文本模板分支，
/// prompt_rollercoaster 写面 u16@+0x48 = 0x1200 → id 0x12）。
///
/// 注意：request id **不等于** TutorialHintComplete/TutorialOp id——多个教学操作
/// 复用同一展示提示文本。已判等证据：
/// `hint_terrain_focus` 0x142283980 / `hint_stairs` 阶段 3 0x142285a20 写
/// `0x0100`（id 1 = FocusedEdit 通用 focus-or-camera 文本），而非 5/15；
/// `hint_deselect_tool` 0x1422853c0 写 `MOV word ptr [RBP+0x38],0x0`（id 0），
/// `hint_roof_shape` 0x142284ed0 写 `MOV word ptr [RSP+0xf0],0x700`（id 7）。
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
    /// 0x12：过山车提示（prompt_rollercoaster；orchestrator 独立模板分支）。
    Rollercoaster = 18,
}

impl HintType {
    /// id → 枚举（O：hint id 0..17；越界返回 None）。
    pub fn from_id(id: u8) -> Option<Self> {
        Some(match id {
            0 => Self::DeselectTool,
            1 => Self::FocusedEdit,
            2 => Self::Undo,
            3 => Self::DisableSnapping,
            4 => Self::TerrainSteepness,
            5 => Self::TerrainFocus,
            6 => Self::Terrain,
            7 => Self::RoofShape,
            8 => Self::UnlinkWindows,
            9 => Self::ClutterFocus,
            10 => Self::CameraPan,
            11 => Self::CameraRotate,
            12 => Self::CameraZoom,
            13 => Self::StairsPlaced,
            14 => Self::StairsFocus,
            15 => Self::StairsContinue,
            16 => Self::StairsDetach,
            17 => Self::StairsCanDetach,
            18 => Self::Rollercoaster,
            _ => return None,
        })
    }
}

/// HintType 的 native Display 名称表（O `impl$14::fmt` 0x1422810c0：
/// `Formatter::write_str`，字符串长度表 0x143038b38、相对偏移表 0x143038bd0，
/// 19 项按 id 索引）。`hint_orchestrator` 默认文本分支
/// `format!("Hint.{}", HintType)` 使用该 Display（反编译 pieces = ["Hint."]，
/// std `fmt::write` 在 piece 0 之后追加 arg 0 → 实际输出 `Hint.<name>`）。
///
/// id > 18 时 native 越界读表；本实现返回 `""`（合法写面只产生 0..=18）。
pub fn hint_type_display_name(id: u8) -> &'static str {
    const NAMES: [&str; 19] = [
        "Deselect",                 // 0  (8)
        "FocusEdit",                // 1  (9)
        "Undo",                     // 2  (4)
        "DisableSnapping",          // 3  (15)
        "TerrainSteepness",         // 4  (16)
        "TerrainFocus",             // 5  (12)
        "Terrain",                  // 6  (7)
        "RoofShape",                // 7  (9)
        "UnlinkWindows",            // 8  (13)
        "Clutter",                  // 9  (7)
        "CameraPan",                // 10 (9)
        "CameraRotate",             // 11 (12)
        "CameraZoom",               // 12 (10)
        "StairsCreateStart",        // 13 (17)
        "StairsCreateFinish",       // 14 (18)
        "StairsFocus",              // 15 (11)
        "StairsContinue",           // 16 (14)
        "StairsDetachFromWall",     // 17 (20)
        "RollercoasterPrompt",      // 18 (19)
    ];
    NAMES.get(id as usize).copied().unwrap_or("")
}

impl std::fmt::Display for HintType {
    /// O `impl$14::fmt` 0x1422810c0 = 按 id 取名称串后 `Formatter::write_str`。
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(hint_type_display_name(*self as u8))
    }
}

/// 提示放置（O `HintPlacement`，B native 枚举 0x18B）。
///
/// fresh 证据（2026-10-17）：
/// * `hint_position_update` 0x141c50510 以 `idx*0x18` 步长读组件、`switch(dword@+0)`
///   分派 case **6 / 7 / 8 / 10**（其它值走 0x142f14844 跳转表）：
///   - 6：光标处 + 双倍边距（Cursor）；
///   - 7：`CameraMatrices::eye_right` + `world_space_to_screen_space`（世界点）；
///   - 8：`Rectangle2d::as_points3` + 四角投影 min/max（世界矩形）；
///   - 10：窗口中心 `0.5*w, 0.5*h`（ScreenCenter）。
/// * 写面判别来自 `6 + AppMode::Wall` 的形状字节：
///   `hint_deselect_tool` 0x1422853c0 `MOV EAX,0x6` / `MOVZX EAX,[RDI+4]; ADD EAX,0x6`
///   后写 `[RBP-0x10]`；`hint_disable_snapping` 0x14227e140 写
///   `CONCAT24(..., bVar2 + 6)`。WallShape = 0 Freeform / 1 Circle / 2 Rectangle
///   → placement = 6 / 7 / 8。
/// * `prompt_rollercoaster` 0x14227bb70 写 `0x0a`（ScreenCenter）。
///
/// native 布局：判别 u32@+0，payload 区 +4..+0x18（20B；写面对 7/8 也写满该区）。
/// **payload 来源 U**：反编译中 7/8 的 payload 来自调用后栈临时字节，未取到
/// 世界点/矩形的真实来源；本重建保留负载变体，占位 ZERO 并登记 U（不伪造几何）。
#[derive(Component, Clone, Copy, Debug, PartialEq)]
pub enum HintPlacement {
    /// 6：光标位置。
    Cursor,
    /// 7：世界点（`WorldPoint(Vec3)`；payload 来源 U）。
    WorldPoint(glam::Vec3),
    /// 8：世界矩形（`Rectangle2d`；payload 来源 U）。
    Bounds(utils::geometry::rectangle::Rectangle2d),
    /// 10：屏幕中心。
    ScreenCenter,
    /// 其它 native 判别（hint_position_update 0x141c50510 default 走跳转表
    /// 0x142f14844；未结构化建模的写面判别）。保留原判别值，不伪造变体名。
    Other(u32),
    /// focused-edit 写面（`hint_focused_edit_hint` 0x14227e0e1 的
    /// `movups xmmword [rsp+0x70], xmm0` + `mov word [rsp+0x80], 0x101`；
    /// `hint_clutter_focused_edit_hint` 0x141c3dcdc/0x141c3dce6）。
    ///
    /// native 判别 = **EditSubject 判别**（1 Wall / 2 Roof / 3 DecoClutter），
    /// 载荷 = EditSubject 16B（Wall/Roof 的 id @+8；DecoClutter 的 decorator_id
    /// @+4）后接 u16 @+0x10：Wall/Roof 写 **0x0101**，DecoClutter 写 **0x0100**。
    /// 后续 `hint_position_update` default 跳转表 0x142f14844 消费（未实现）。
    Subject {
        /// 原 EditSubject（逐字节写面）。
        subject: country_core::systems::mode_manager::EditSubject,
        /// native +0x10 的 u16（0x0101 Wall/Roof / 0x0100 DecoClutter）。
        word_0x10: u16,
    },
}

impl HintPlacement {
    /// native 判别值（hint_position_update switch / 各写面 +0 字节）。
    pub const fn native_tag(self) -> u32 {
        match self {
            Self::Cursor => 6,
            Self::WorldPoint(_) => 7,
            Self::Bounds(_) => 8,
            Self::ScreenCenter => 10,
            Self::Other(tag) => tag,
            // EditSubject 判别 0..=5（mode_manager::AppMode::native_tag 同表）。
            Self::Subject { subject, .. } => match subject {
                country_core::systems::mode_manager::EditSubject::None => 0,
                country_core::systems::mode_manager::EditSubject::Wall(_) => 1,
                country_core::systems::mode_manager::EditSubject::Roof(_) => 2,
                country_core::systems::mode_manager::EditSubject::DecoClutter { .. } => 3,
                country_core::systems::mode_manager::EditSubject::StairPoint { .. } => 4,
                country_core::systems::mode_manager::EditSubject::Terrain(_) => 5,
            },
        }
    }
}

/// `AppMode::Wall` 形状字节 → placement（O deselect_tool / disable_snapping：
/// `placement = 6 + shape`）。7/8 的 payload 来源 U → 以 ZERO 占位，判别值判等。
pub fn wall_shape_placement(shape: u8) -> HintPlacement {
    match shape {
        1 => HintPlacement::WorldPoint(glam::Vec3::ZERO),
        2 => HintPlacement::Bounds(utils::geometry::rectangle::Rectangle2d {
            axis: Vec2::ZERO,
            center: Vec2::ZERO,
            size: Vec2::ZERO,
        }),
        _ => HintPlacement::Cursor,
    }
}

/// 提示布局记忆（HintLayoutMemo；position/draw 消费）。
///
/// native 布局（O）：组件步长 **0x18**，三个 `Vec2`：
/// * `+0x00`：`hint_draw` 写布局位置（`*puStack_200 = uVar28`）；
/// * `+0x08`：`hint_draw` 写文本尺寸（`puVar6[1] = 文本度量 × scale`）；
///   `hint_position_update` 以 `size - position` 作为目标位移；
/// * `+0x10`：`hint_draw` 写与 `+0x00` 相同的值（`puVar6[2] = uVar28`）；
///   `hint_position_update` 以上界 `(window - field_c) - margin30` 使用它。
#[derive(Component, Clone, Copy, Debug, Default, PartialEq)]
pub struct HintLayoutMemo {
    /// +0x00
    pub position: Vec2,
    /// +0x08
    pub size: Vec2,
    /// +0x10
    pub position_2: Vec2,
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

/// `DisplayHintRequest` 完整 native 布局（B patched，fresh 反编译判等）。
///
/// 证据：
/// * `hint_orchestrator` 0x14227fc30 事件迭代步长 **0x58**（`IMUL r, n, 0x58`），
///   读 `+0x49` 作 id（`bVar2 = *(byte*)(req+0x49)`，`<0x12` / `==0x12` 分支）、
///   读 `+0x48` 作进度条门（`*(char*)(req+0x48) != 0` → insert HintProgressBar）、
///   读 `+0x18` 作 BindingHintDisplay 判别（`!= 0xc4` → insert display）、
///   读 `+0x00` 作 placement 判别（`*(int*)req == 10` → insert InteractionPrompt），
///   并把 `+0x00..+0x18` 24B 整体作为 HintPlacement 组件、`+0x49` 字节作为
///   HintType 组件。
/// * 写面（inlined, stride 0x58）：
///   - `prompt_rollercoaster` 0x14227bb70：`+0` = 10、`+0x48` u16 = 0x1200；
///   - `hint_camera_display_condition` 0x14227c910：`+0` = 6、
///     `+0x48` = 1、`+0x49` = cVar9（0xa/0xb/0xc）；
///   - `hint_disable_snapping` 0x14227e140：`+0` = 6+shape、`+0x48` u16 = 0x0300；
///   - `hint_undo` 0x142284220：`+0` = 6、`+0x48` u16 = 0x0200；
///   - `hint_roof_shape` 0x142284ed0：`MOV word [RSP+0xf0],0x700`、`+0` = 6；
///   - `hint_deselect_tool` 0x1422853c0：`MOV word [RBP+0x38],0x0`、`+0` = 6+shape。
///
/// 字段偏移（`T` 为事件载荷，`EventInstance<T>` 另有 +0x50 EventId）：
/// ```text
/// +0x00 u32  placement 判别（HintPlacement，native 共 0x18B）
/// +0x04 .. placement payload（0x14B；7=世界点 8=矩形，来源 U）
/// +0x18 u32  display 判别（BindingHintDisplay，native 共 0x30B）
/// +0x1c .. display payload
/// +0x48 u8   flags（bit0 = 显示 HintProgressBar；观测值 0/1）
/// +0x49 u8   id（HintType 值，0..0x12）
/// +0x4a .. 填充（6B，未初始化/零）
/// +0x50      EventInstance event_id（不在 T 内）
/// ```
/// T size = **0x50B**；EventInstance stride = **0x58B**。
#[derive(Event, Clone, Debug, PartialEq)]
pub struct DisplayHintRequest {
    /// +0x00：放置方式（native 0x18B 枚举；判别 6/7/8/10）。
    pub placement: HintPlacement,
    /// +0x18：键帽/图形显示树（native 0x30B 枚举，判别 0xc4..0xcc）。
    pub display: BindingHintDisplay,
    /// +0x48：flags 字节。bit0（值 1）= 生成 HintProgressBar（相机 pan/rotate/zoom
    /// 三个写面唯一置位；orchestrator 以 `!=0` 判定，未观测其它位）。
    pub flags: u8,
    /// +0x49：展示提示 id（HintType 值 0..0x12；见 `HintType` 的复用说明）。
    pub id: u8,
}

/// `DisplayHintRequest` native 载荷大小（0x50B；事件步长 0x58 = +EventId 8B）。
pub const DISPLAY_HINT_REQUEST_SIZE: usize = 0x50;
/// `EventInstance<DisplayHintRequest>` native 步长（0x58B）。
pub const DISPLAY_HINT_REQUEST_EVENT_INSTANCE_STRIDE: usize = 0x58;
/// placement 字段偏移 / 大小。
pub const DISPLAY_HINT_REQUEST_PLACEMENT_OFFSET: usize = 0x00;
pub const DISPLAY_HINT_REQUEST_PLACEMENT_SIZE: usize = 0x18;
/// display 字段偏移 / 大小。
pub const DISPLAY_HINT_REQUEST_DISPLAY_OFFSET: usize = 0x18;
pub const DISPLAY_HINT_REQUEST_DISPLAY_SIZE: usize = 0x30;
/// flags @+0x48；bit0 = HintProgressBar。
pub const DISPLAY_HINT_REQUEST_FLAGS_OFFSET: usize = 0x48;
pub const DISPLAY_HINT_FLAG_PROGRESS_BAR: u8 = 0x01;
/// id @+0x49（HintType 值）。
pub const DISPLAY_HINT_REQUEST_ID_OFFSET: usize = 0x49;

impl DisplayHintRequest {
    /// 便捷构造（O 各 hint_* 写面；placement 默认 Cursor(6)——除
    /// prompt_rollercoaster 用 ScreenCenter(10)，见 `with_placement`）。
    /// `hint_type` 写入 id@+0x49；`flags` 为 +0x48 字节（不是旧的 `id<<8|flag`
    /// 混合 16 位值）。
    pub fn new(display: BindingHintDisplay, hint_type: HintType, flags: u8) -> Self {
        Self {
            placement: HintPlacement::Cursor,
            display,
            flags,
            id: hint_type as u8,
        }
    }

    /// 以 native id 字节构造（id 不在 `HintType` 已知值集时使用）。
    pub fn with_id(display: BindingHintDisplay, id: u8, flags: u8) -> Self {
        Self {
            placement: HintPlacement::Cursor,
            display,
            flags,
            id,
        }
    }

    /// 覆写 placement（O：DisplayHintRequest +0 字节 = 6 Cursor / 7 WorldPoint /
    /// 8 Bounds / 10 ScreenCenter）。
    pub fn with_placement(mut self, placement: HintPlacement) -> Self {
        self.placement = placement;
        self
    }

    /// id 字节 → HintType（越界 None）。
    pub fn hint_type(&self) -> Option<HintType> {
        HintType::from_id(self.id)
    }

    /// +0x48 flags != 0 → orchestrator 插入 HintProgressBar。
    pub fn has_progress_bar(&self) -> bool {
        self.flags != 0
    }
}

/// 停留提示请求（LingerHintRequest；与 DisplayHintRequest 同族）。
#[derive(Event, Clone, Debug, PartialEq)]
pub struct LingerHintRequest {
    pub display: BindingHintDisplay,
    pub hint_type: HintType,
    pub duration: f32,
}

/// closure$0 排序键（O `hint_orchestrator` 0x14227fc30 排序实例化）：
/// `insertion_sort_shift_left<ref<DisplayHintRequest>, closure_env_0>` 0x1425e5b70
/// 逐语句为 `*(byte*)(a + 0x49) < *(byte*)(b + 0x49)`；`sort8_stable` 0x1425e57e0
/// 同样逐对比较 +0x49 字节。即**按 id 字节升序稳定排序**（len≤0x15 用 insertion，
/// 否则 driftsort；两者都是 `slice::sort_by` 的稳定实现）。
pub fn sort_display_hint_requests(requests: &mut [&DisplayHintRequest]) {
    requests.sort_by(|a, b| a.id.cmp(&b.id));
}

/// 工具栏 UI 焦点事件（O 原生类型 =
/// `country_core::resources::toolbar::ToolbarUiIsInFocus`，由 toolbar plugin 注册；
/// country-core 尚未落该类型）。本模块以同名本地 unit 事件承接 `hint_orchestrator`
/// 的第 5 个 `EventReader` SystemParam；生产者/注册归属接线登记 U。
#[derive(Event, Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct ToolbarUiIsInFocus;

/// 提示文本构造（O 0x14227fc30 文本分支，disasm 0x1422800ea/0x142280408/0x142280490）：
/// * id == 0x12：`lang_lib::lookup("Rollercoaster.Ride")` → `format!("{} 🎢", …)`
///   （pieces `["", " 🎢"]` @0x1430388f0，emoji U+1F3A2）；
/// * id == 2：`format!("{}/{}", lookup("KeyBindings.Undo"), lookup("KeyBindings.Redo"))`
///   （pieces `["", "/"]` @0x1430388b8，键 16B @0x143038890/0x1430388a0）；
/// * 其它：`format!("Hint.{}", HintType)`（pieces `["Hint."]` @0x143038918 +
///   `impl$14::fmt` 名称表）→ `lang_lib::lookup(..)` → `to_uppercase()`。
fn hint_text(id: u8) -> String {
    if id == 0x12 {
        format!("{} 🎢", lang_lib::lookup("Rollercoaster.Ride"))
    } else if id == 2 {
        format!(
            "{}/{}",
            lang_lib::lookup("KeyBindings.Undo"),
            lang_lib::lookup("KeyBindings.Redo")
        )
    } else {
        lang_lib::lookup(&format!("Hint.{}", hint_type_display_name(id))).to_uppercase()
    }
}

/// hint_orchestrator（O 0x14227fc30，3035）。SystemParam 顺序取自 PDB 单态化名
/// （`FunctionSystemState` drop glue）：`Res<AppMode>`, `EventReader<DisplayHintRequest>`,
/// `EventReader<LingerHintRequest>`, `Query<(Entity, &HintType,
/// Option<&HintDeleteAfterCooldown>, Option<&HintMarkedForDeletion>)>`,
/// `EventReader<ToolbarUiIsInFocus>`, `Res<EscMenuState>`, `Commands`。
///
/// 逐语句重建（2026-10-17 fresh 反编译 + B 机器码）：
/// 1. `EventReader<DisplayHintRequest>.read()` collect 为 `Vec<&T>` → `sort_by(id)`（0x1425e5b70）；
/// 2. 消费 Linger reader：**有事件** → `suppressed = true`；
///    否则 `suppressed = AppMode == Replay(0xd) ? true : (AppMode == Photomode(0x13) || esc.is_open())`；
/// 3. 消费 ToolbarUiIsInFocus reader：`toolbar_has_events && !suppressed` → 直接返回
///    （工具栏聚焦时不显示提示，菜单/重放/光拍模式除外）；
/// 4. `query.iter().next()` 取当前提示实体（无唯一性检查，native 取首个匹配）；
/// 5. 无实体：无请求 → 返回；`suppressed && id < 0x12` → 返回；
///    否则 spawn `(HintLayoutMemo, HintSpring, HintAnimation, HintText, HintType, HintPlacement)`，
///    placement tag == 10 追加 `InteractionPrompt`，display clone 插入，`flags != 0`
///    追加 `HintProgressBar`；
/// 6. 有实体：`(suppressed && existing_id < 0x12)` → `insert(HintMarkedForDeletion)`；
///    否则有请求时——同 id → insert 新 placement + 非 Empty display + remove 两个删除标记；
///    不同 id → `insert(HintMarkedForDeletion)`；无请求时——已有任一删除标记则返回，
///    否则 `insert(HintDeleteAfterCooldown { timer: DAT_143038ae8[id] })`。
pub fn hint_orchestrator(
    app_mode: Res<country_core::systems::mode_manager::AppMode>,
    mut requests: EventReader<DisplayHintRequest>,
    mut linger: EventReader<LingerHintRequest>,
    query: Query<(
        Entity,
        &HintType,
        Option<&crate::hint_display_system::HintDeleteAfterCooldown>,
        Option<&crate::hint_display_system::HintMarkedForDeletion>,
    )>,
    mut toolbar_focus: EventReader<ToolbarUiIsInFocus>,
    esc_menu: Res<country_core::resources::gui::EscMenuState>,
    mut commands: Commands,
) {
    // O：from_iter<ref<DisplayHintRequest>, EventIterator<..>> 0x141c58da0 →
    // sort_by closure$0（id@+0x49 升序、稳定）。
    let mut sorted: Vec<&DisplayHintRequest> = requests.read().collect();
    sort_display_hint_requests(&mut sorted);

    // O：两个 reader 的“本帧是否有事件”检查（EventCursor 双缓冲游标差；
    // native 在存在未读事件时把游标推到 event_count，等价于 `read().count()`）。
    let linger_present = linger.read().count() > 0;
    let toolbar_present = toolbar_focus.read().count() > 0;

    // O 0x1422800xx：`AppMode == 0xd`（Replay）→ 抑制；否则
    // `(AppMode == 0x13 (Photomode)) | EscMenuState::is_open()`；Linger 有事件恒抑制。
    let suppressed = if linger_present {
        true
    } else if matches!(*app_mode, country_core::systems::mode_manager::AppMode::Replay) {
        true
    } else {
        matches!(
            *app_mode,
            country_core::systems::mode_manager::AppMode::Photomode
        ) || esc_menu.is_open()
    };

    if toolbar_present && !suppressed {
        return;
    }

    match query.iter().next() {
        None => {
            let Some(first) = sorted.first().copied() else {
                return;
            };
            if suppressed && first.id < 0x12 {
                return;
            }
            // O spawn<tuple<HintLayoutMemo, HintSpring, HintAnimation, HintText,
            //   HintType, HintPlacement>> 0x142289260；文本为上面三条 lang 分支结果。
            let mut entity = commands.spawn((
                HintLayoutMemo::default(),
                crate::hint_display_system::HintSpring::default(),
                crate::hint_display_system::HintAnimation::default(),
                crate::hint_display_system::HintText {
                    text: hint_text(first.id),
                },
                first.placement,
            ));
            // native 直接写 id 字节；合法域 0..=0x12 全部可表示。
            if let Some(hint_type) = HintType::from_id(first.id) {
                entity.insert(hint_type);
            }
            // O `*(int*)req == 10` → insert InteractionPrompt。
            if first.placement.native_tag() == 10 {
                entity.insert(InteractionPrompt::default());
            }
            // O：Impl$30::clone + insert<BindingHintDisplay>（无条件，含 Empty）。
            entity.insert(first.display.clone());
            // O `*(char*)(req+0x48) != 0` → insert HintProgressBar。
            if first.flags != 0 {
                entity.insert(HintProgressBar::default());
            }
        }
        Some((entity, hint_type, cooldown, marked)) => {
            let existing_id = *hint_type as u8;
            if suppressed && existing_id < 0x12 {
                // 菜单/重放/光拍/linger：把当前提示标记删除。
                commands
                    .entity(entity)
                    .insert(crate::hint_display_system::HintMarkedForDeletion);
                return;
            }
            match sorted.first().copied() {
                Some(first) if existing_id == first.id => {
                    // 同 id：刷新 placement；非 Empty display 覆盖；清两个删除标记
                    // （native remove 的错误处理器 = warn）。
                    let mut entity_commands = commands.entity(entity);
                    entity_commands.insert(first.placement);
                    if !matches!(first.display, BindingHintDisplay::Empty) {
                        entity_commands.insert(first.display.clone());
                    }
                    entity_commands
                        .remove::<crate::hint_display_system::HintMarkedForDeletion>();
                    entity_commands
                        .remove::<crate::hint_display_system::HintDeleteAfterCooldown>();
                }
                Some(_) => {
                    // id 变化：标记删除，等 entity 消失后再 spawn 新提示。
                    commands
                        .entity(entity)
                        .insert(crate::hint_display_system::HintMarkedForDeletion);
                }
                None => {
                    // 无新请求：已有删除标记/冷却则返回；否则启动冷却计时。
                    if marked.is_some() || cooldown.is_some() {
                        return;
                    }
                    let timer =
                        crate::hint_display_system::DELETION_COOLDOWN_TABLE[existing_id as usize];
                    commands
                        .entity(entity)
                        .insert(crate::hint_display_system::HintDeleteAfterCooldown {
                            timer,
                        });
                }
            }
        }
    }
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
            .alternate(BindingHintDisplay::Steam(steam_query(0x101)))
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
            steam_query(0x101),
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
            BindingHintDisplay::Steam(steam_query(0x101)).cleanup(),
            BindingHintDisplay::Alternate(vec![BindingHintDisplay::Empty])
        );
    }

    #[test]
    fn hint_type_native_values() {
        // O：HintType 组件值 = 提示 id 0..17 + 0x12。
        assert_eq!(HintType::CameraPan as u8, 10);
        assert_eq!(HintType::CameraRotate as u8, 11);
        assert_eq!(HintType::CameraZoom as u8, 12);
        assert_eq!(HintType::DeselectTool as u8, 0);
        assert_eq!(HintType::Rollercoaster as u8, 18);
        assert_eq!(HintType::from_id(18), Some(HintType::Rollercoaster));
    }

    #[test]
    fn display_hint_request_layout_and_flags() {
        // O 0x14227fc30：T=0x50B、EventInstance stride=0x58B；placement@+0、
        // display@+0x18、flags@+0x48、id@+0x49。
        assert_eq!(DISPLAY_HINT_REQUEST_SIZE, 0x50);
        assert_eq!(DISPLAY_HINT_REQUEST_EVENT_INSTANCE_STRIDE, 0x58);
        assert_eq!(DISPLAY_HINT_REQUEST_PLACEMENT_OFFSET, 0x00);
        assert_eq!(DISPLAY_HINT_REQUEST_PLACEMENT_SIZE, 0x18);
        assert_eq!(DISPLAY_HINT_REQUEST_DISPLAY_OFFSET, 0x18);
        assert_eq!(DISPLAY_HINT_REQUEST_DISPLAY_SIZE, 0x30);
        assert_eq!(DISPLAY_HINT_REQUEST_FLAGS_OFFSET, 0x48);
        assert_eq!(DISPLAY_HINT_REQUEST_ID_OFFSET, 0x49);
        assert_eq!(DISPLAY_HINT_FLAG_PROGRESS_BAR, 0x01);

        let req = DisplayHintRequest::new(
            BindingHintDisplay::Key(Some(1)),
            HintType::Terrain,
            0,
        );
        assert_eq!(req.id, 6); // 0x0600 高字节
        assert_eq!(req.flags, 0);
        assert_eq!(req.placement, HintPlacement::Cursor);
        assert_eq!(req.hint_type(), Some(HintType::Terrain));
        assert!(!req.has_progress_bar());

        // 相机写面 +0x48=1（0x0a01/0x0b01/0x0c01 的低字节）。
        let camera = DisplayHintRequest::new(
            BindingHintDisplay::Key(Some(1)),
            HintType::CameraPan,
            DISPLAY_HINT_FLAG_PROGRESS_BAR,
        );
        assert_eq!(camera.id, 0x0a);
        assert!(camera.has_progress_bar());
    }

    #[test]
    fn hint_placement_native_tags() {
        // O hint_position_update 0x141c50510 switch 6/7/8/10；
        // deselect/disable_snapping 写面 placement = 6 + WallShape。
        assert_eq!(HintPlacement::Cursor.native_tag(), 6);
        assert_eq!(
            HintPlacement::WorldPoint(glam::Vec3::ZERO).native_tag(),
            7
        );
        assert_eq!(
            HintPlacement::Bounds(utils::geometry::rectangle::Rectangle2d {
                axis: Vec2::ZERO,
                center: Vec2::ZERO,
                size: Vec2::ZERO,
            })
            .native_tag(),
            8
        );
        assert_eq!(HintPlacement::ScreenCenter.native_tag(), 10);
        // hint_clutter_focused_edit_hint 0x141c3db80 观测的 placement=3（语义 U）。
        assert_eq!(HintPlacement::Other(3).native_tag(), 3);
        assert_eq!(wall_shape_placement(0), HintPlacement::Cursor);
        assert_eq!(
            wall_shape_placement(1),
            HintPlacement::WorldPoint(glam::Vec3::ZERO)
        );
        assert!(matches!(wall_shape_placement(2), HintPlacement::Bounds(_)));
    }

    #[test]
    fn sort_display_hint_requests_by_id_is_stable() {
        // O 0x1425e5b70 / 0x1425e57e0：逐对比较 +0x49 字节（id 升序，稳定）。
        let a = DisplayHintRequest::new(BindingHintDisplay::Key(None), HintType::RoofShape, 0);
        let b = DisplayHintRequest::new(BindingHintDisplay::Key(None), HintType::Undo, 0);
        let c = DisplayHintRequest::new(BindingHintDisplay::Key(None), HintType::Undo, 1);
        let mut refs = vec![&a, &b, &c];
        sort_display_hint_requests(&mut refs);
        // id: a=7, b=2, c=2 → [b, c, a]，且两个 id=2 保持输入顺序。
        assert!(std::ptr::eq(refs[0], &b));
        assert!(std::ptr::eq(refs[1], &c));
        assert!(std::ptr::eq(refs[2], &a));
    }

    #[test]
    fn hint_type_display_names_match_native_table() {
        // O 0x1422810c0 + 表 0x143038b38/0x143038bd0（19 项）。
        assert_eq!(hint_type_display_name(0), "Deselect");
        assert_eq!(hint_type_display_name(6), "Terrain");
        assert_eq!(hint_type_display_name(12), "CameraZoom");
        assert_eq!(hint_type_display_name(17), "StairsDetachFromWall");
        assert_eq!(hint_type_display_name(18), "RollercoasterPrompt");
        assert_eq!(format!("{}", HintType::CameraRotate), "CameraRotate");
    }

    fn orchestrator_world() -> World {
        let mut world = World::new();
        world.init_resource::<Events<DisplayHintRequest>>();
        world.init_resource::<Events<LingerHintRequest>>();
        world.init_resource::<Events<ToolbarUiIsInFocus>>();
        world.init_resource::<country_core::systems::mode_manager::AppMode>();
        world.init_resource::<country_core::resources::gui::EscMenuState>();
        world
    }

    fn run_orchestrator(world: &mut World) {
        let mut schedule = Schedule::default();
        schedule.add_systems(hint_orchestrator);
        schedule.run(world);
    }

    fn write_request(world: &mut World, request: DisplayHintRequest) {
        world
            .resource_mut::<Events<DisplayHintRequest>>()
            .send(request);
    }

    fn hint_entities(world: &mut World) -> Vec<Entity> {
        let mut query = world.query::<(Entity, &HintType)>();
        query.iter(world).map(|(entity, _)| entity).collect()
    }

    #[test]
    fn orchestrator_spawns_first_sorted_request() {
        // O：事件 collect → id 升序排序 → 只取首个请求 spawn。
        let mut world = orchestrator_world();
        write_request(
            &mut world,
            DisplayHintRequest::new(BindingHintDisplay::Key(Some(1)), HintType::RoofShape, 0),
        );
        write_request(
            &mut world,
            DisplayHintRequest::new(BindingHintDisplay::Key(Some(1)), HintType::Undo, 0),
        );
        run_orchestrator(&mut world);

        let entities = hint_entities(&mut world);
        assert_eq!(entities.len(), 1);
        let entity = entities[0];
        assert_eq!(
            world.get::<HintType>(entity),
            Some(&HintType::Undo),
            "id=2 应排在 id=7 前"
        );
        assert_eq!(
            world.get::<crate::hint_display_system::HintText>(entity).map(|t| t.text.as_str()),
            Some("KeyBindings.Undo/KeyBindings.Redo")
        );
        assert!(world.get::<BindingHintDisplay>(entity).is_some());
        assert!(world.get::<InteractionPrompt>(entity).is_none());
        assert!(world.get::<HintProgressBar>(entity).is_none());
    }

    #[test]
    fn orchestrator_rollercoaster_and_default_text() {
        // O：id 0x12 → lookup("Rollercoaster.Ride") + " 🎢"；默认 → "HINT.<NAME>"。
        assert_eq!(hint_text(0x12), "Rollercoaster.Ride 🎢");
        assert_eq!(hint_text(6), "HINT.TERRAIN");
        assert_eq!(hint_text(2), "KeyBindings.Undo/KeyBindings.Redo");
    }

    #[test]
    fn orchestrator_screen_center_adds_prompt_and_progress_bar() {
        // O：placement tag==10 → InteractionPrompt；flags!=0 → HintProgressBar。
        let mut world = orchestrator_world();
        write_request(
            &mut world,
            DisplayHintRequest::new(
                BindingHintDisplay::Key(Some(1)),
                HintType::CameraPan,
                DISPLAY_HINT_FLAG_PROGRESS_BAR,
            )
            .with_placement(HintPlacement::ScreenCenter),
        );
        run_orchestrator(&mut world);
        let entity = hint_entities(&mut world)[0];
        assert!(world.get::<InteractionPrompt>(entity).is_some());
        assert!(world.get::<HintProgressBar>(entity).is_some());
        assert_eq!(
            world.get::<HintPlacement>(entity),
            Some(&HintPlacement::ScreenCenter)
        );
    }

    #[test]
    fn orchestrator_same_id_refreshes_and_clears_markers() {
        // O：同 id → insert placement/display + remove 两个删除标记。
        let mut world = orchestrator_world();
        let entity = world
            .spawn((
                HintType::Terrain,
                crate::hint_display_system::HintMarkedForDeletion,
                crate::hint_display_system::HintDeleteAfterCooldown { timer: 1.0 },
            ))
            .id();
        write_request(
            &mut world,
            DisplayHintRequest::new(BindingHintDisplay::Key(Some(3)), HintType::Terrain, 0)
                .with_placement(HintPlacement::ScreenCenter),
        );
        run_orchestrator(&mut world);

        assert_eq!(hint_entities(&mut world).len(), 1);
        assert!(world
            .get::<crate::hint_display_system::HintMarkedForDeletion>(entity)
            .is_none());
        assert!(world
            .get::<crate::hint_display_system::HintDeleteAfterCooldown>(entity)
            .is_none());
        assert_eq!(
            world.get::<HintPlacement>(entity),
            Some(&HintPlacement::ScreenCenter)
        );
        assert_eq!(
            world.get::<BindingHintDisplay>(entity),
            Some(&BindingHintDisplay::Key(Some(3)))
        );
    }

    #[test]
    fn orchestrator_different_id_marks_for_deletion() {
        // O：id 变化 → insert HintMarkedForDeletion（等实体消失后再 spawn）。
        let mut world = orchestrator_world();
        let entity = world.spawn(HintType::Terrain).id();
        write_request(
            &mut world,
            DisplayHintRequest::new(BindingHintDisplay::Key(None), HintType::Undo, 0),
        );
        run_orchestrator(&mut world);
        assert_eq!(hint_entities(&mut world).len(), 1);
        assert!(world
            .get::<crate::hint_display_system::HintMarkedForDeletion>(entity)
            .is_some());
    }

    #[test]
    fn orchestrator_starts_cooldown_when_no_requests() {
        // O：无请求且无删除标记/冷却 → insert HintDeleteAfterCooldown(DAT[id])。
        let mut world = orchestrator_world();
        let entity = world.spawn(HintType::Terrain).id();
        run_orchestrator(&mut world);
        let cooldown = world
            .get::<crate::hint_display_system::HintDeleteAfterCooldown>(entity)
            .expect("cooldown inserted");
        assert_eq!(cooldown.timer, 0.3);
    }

    #[test]
    fn orchestrator_esc_open_suppresses_or_marks_deletion() {
        // O：suppressed && existing_id<0x12 → insert HintMarkedForDeletion；
        //    无实体时 suppressed && new id<0x12 → 不 spawn。
        let mut world = orchestrator_world();
        *world.resource_mut::<country_core::resources::gui::EscMenuState>() =
            country_core::resources::gui::EscMenuState::Main;
        run_orchestrator(&mut world);
        write_request(
            &mut world,
            DisplayHintRequest::new(BindingHintDisplay::Key(None), HintType::Terrain, 0),
        );
        run_orchestrator(&mut world);
        assert!(hint_entities(&mut world).is_empty(), "菜单打开时不得 spawn");

        let entity = world.spawn(HintType::Terrain).id();
        run_orchestrator(&mut world);
        assert!(world
            .get::<crate::hint_display_system::HintMarkedForDeletion>(entity)
            .is_some());
    }

    #[test]
    fn orchestrator_toolbar_focus_gate_matches_linger_flag() {
        // O：toolbar 有事件 && !bVar12 → return；bVar12 = linger 有事件 ||
        //     (AppMode==13) || (AppMode==19) || esc.is_open()。
        // bVar12 同时是 normal-id spawn 抑制位（id < 0x12），id 0x12 不受抑制。
        let mut world = orchestrator_world();
        world
            .resource_mut::<Events<ToolbarUiIsInFocus>>()
            .send(ToolbarUiIsInFocus);
        write_request(
            &mut world,
            DisplayHintRequest::new(
                BindingHintDisplay::Steam(steam_query(0x101)),
                HintType::Rollercoaster,
                0,
            )
            .with_placement(HintPlacement::ScreenCenter),
        );
        run_orchestrator(&mut world);
        assert!(hint_entities(&mut world).is_empty(), "工具栏聚焦不显示提示");

        // linger 事件置 bVar12=1 → 工具栏门控不再拦截（id 0x12 通过 spawn 门）。
        world
            .resource_mut::<Events<LingerHintRequest>>()
            .send(LingerHintRequest {
                display: BindingHintDisplay::Empty,
                hint_type: HintType::Rollercoaster,
                duration: 1.0,
            });
        world
            .resource_mut::<Events<ToolbarUiIsInFocus>>()
            .send(ToolbarUiIsInFocus);
        run_orchestrator(&mut world);
        assert_eq!(
            hint_entities(&mut world).len(),
            1,
            "bVar12=1 时 toolbar 门控被绕过"
        );
    }

    #[test]
    fn orchestrator_ignores_rollercoaster_in_menu() {
        // O：spawn 门 `suppressed && id < 0x12`；id 0x12 不受菜单抑制。
        let mut world = orchestrator_world();
        *world.resource_mut::<country_core::resources::gui::EscMenuState>() =
            country_core::resources::gui::EscMenuState::Main;
        write_request(
            &mut world,
            DisplayHintRequest::new(
                BindingHintDisplay::Steam(steam_query(0x101)),
                HintType::Rollercoaster,
                0,
            )
            .with_placement(HintPlacement::ScreenCenter),
        );
        run_orchestrator(&mut world);
        assert_eq!(hint_entities(&mut world).len(), 1);
    }
}
