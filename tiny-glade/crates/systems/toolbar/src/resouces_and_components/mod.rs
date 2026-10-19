//! 工具栏资源/组件（B 版恢复，restorer-wave2b-toolbar-types，2026-10-17；
//! wave3 restorer-wave3-toolbar-layout 2026-10-17 增补按钮族组件；
//! **unlock-mei-toolbar 2026-10-17：20 个 `register_required_components` 组件尺寸
//! 按 fresh `ComponentDescriptor` 立即数逐项校正 + UiNav/HistoryButton 布局落库**）。
//!
//! 已恢复：`LogSlider::param_to_value`（O 0x142383730）；`add_toolbar_item`
//! （O 0x141e4dc40，判等/重建见 RESTORE.md）。
//! 组件尺寸证据：`work/unlock-mei-toolbar/evidence/disasm-reg-0x*.txt`
//! （20 个 `register_required_components` 单态的 `[RBP+0x200]` size 立即数）；
//! core UI 组件尺寸：同目录 `disasm-coreui-0x*.txt`（UiInteractionState 3B /
//! UiMouseInteractable 4B / Ui2dExtent 0x10 / Ui2dImage 0x20）。
//!
//! 26 组件台账见 [`SHARED_UI_COMPONENTS`]；本地 24 项由
//! [`register_local_components`] 注册（derive 胶水，glue-legitimate）。
//! 未实现（分歧见 RESTORE.md）：taffy_layout/undo_redo_layout、TaffyLayout 组件族。

pub mod toolbar_layout;
pub mod taffy_layout;

use bevy_ecs::entity::Entity;
use bevy_ecs::prelude::Component;
use taffy::node::Taffy;

/// UI 布局树节点（O：`toolbar::resouces_and_components::TaffyNode`；add_toolbar_item
/// spawn 8 组件之一）。taffy `Node = slotmap::DefaultKey`（8B，align 4）。
///
/// **B 布局（20B = 0x14，fresh `register_required_components` impl$2
/// 0x142383770 的 `ComponentDescriptor` size 立即数 0x14、align 1 面校正）**：
/// `{node: Node@+0 (8B), _tail: [u8;12]@+8}`。原 16B `{node, _gap:u64}` 推断作废；
/// 尾 12B 的字段语义 U（add_toolbar_item spawn 写面观测到 +0x40 qword、
/// +0x48 qword、+0x50 dword 共 20B）。
#[derive(Component, Clone, Copy, Debug)]
#[repr(C)]
pub struct TaffyNode {
    /// taffy 树节点 id（I：B 布局 +0，8B）。
    pub node: taffy::node::Node,
    /// +8..+0x14 尾区（布局 O，字段语义 U）。
    pub _tail: [u8; 12],
}

impl Default for TaffyNode {
    /// Node id defaults to the slotmap default; the animation scale at +0x10 is
    /// initialized to 1.0 (I: native spawn stores the multiplier that
    /// ui_select_animation multiplies, so a zero would annihilate the widget).
    fn default() -> Self {
        let mut value = TaffyNode {
            node: taffy::node::Node::default(),
            _tail: [0; 12],
        };
        value.set_scale(1.0);
        value
    }
}

const _TAFFY_NODE_SIZE: () = assert!(std::mem::size_of::<TaffyNode>() == 0x14);

impl TaffyNode {
    /// 用给定 taffy 树建叶子节点（O add_toolbar_item 前段：`Taffy::new_leaf`
    /// + `add_child` 各 `.unwrap()`）。**重建**。
    pub fn spawn_leaf(taffy: &mut Taffy, parent: taffy::node::Node) -> Self {
        let node = taffy.new_leaf(taffy::style::Style::default()).unwrap();
        taffy.add_child(parent, node).unwrap();
        TaffyNode {
            node,
            ..TaffyNode::default()
        }
    }

    /// One-frame size multiplier at TaffyNode+0x10 (tail bytes 8..12).
    /// taffy_finalize_element_size reads it as `once` and resets it to 1.0;
    /// ui_select_animation/ui_locked_animation multiply it in place. O: the
    /// 0x14-stride write `*(f32*)(dense + idx*0x14 + 0x10)`. Field name U.
    pub fn scale(&self) -> f32 {
        f32::from_le_bytes(self._tail[8..12].try_into().unwrap())
    }
    pub fn set_scale(&mut self, value: f32) {
        self._tail[8..12].copy_from_slice(&value.to_le_bytes());
    }
}

/// UI 缩放（用户设置跟随；O：`toolbar::resouces_and_components::UiScaleWithUserSetting`）。
///
/// **B 布局（0B，ZST；fresh `register_required_components` impl$3 0x142383e60 的
/// `ComponentDescriptor` size 立即数 0x0、align 1）**。原 16B `{size: Vec2, tail: Vec2}`
/// 推断作废：add_toolbar_item spawn 区的 `82.5`/`(1.0,0.011)` 常量**不属于**本组件，
/// 归属待定（最可能为 `Ui2dExtent` 16B，core 类型 typed-gap 未承载——登记）。
#[derive(Component, Clone, Copy, Debug, Default)]
pub struct UiScaleWithUserSetting;

const _UI_SCALE_WITH_USER_SETTING_SIZE: () =
    assert!(std::mem::size_of::<UiScaleWithUserSetting>() == 0);

/// UI 动画状态（O：`toolbar::resouces_and_components::UiAnimationState`；读侧
/// **ui_animation_state_update 0x14233b3f0**，wave3 fresh 反编译）。
///
/// **B 布局（16B，wave3 fresh 逐字段核）**：`{appear: f32@+0, select: f32@+4,
/// _gap4: [u8;4]@+8, state: u8@+0xc, _pad3: [u8;3]@+0xd}`——
/// - +0：外观进度（ui_animation_state_update 按 `±4·dt` 推进，clamp01）；
/// - +4：选择进度（按 `±dt/0.2` 推进，clamp01）；
/// - +0xc：动画状态字节（0/1/2，进入/离开判别）。
///
/// **判等（布局/公式）**：逐字段对 ui_animation_state_update fresh 反编译
/// `{f32@0, f32@4, byte@0xc}` 读面。**分歧（未实现）**：状态字节 0/1/2 语义名
/// （进入/停留/离开）U。
#[derive(Component, Clone, Copy, Debug, Default)]
#[repr(C)]
pub struct UiAnimationState {
    /// +0 外观进度（`appear`）。
    pub appear: f32,
    /// +4 选择进度（`select`）。
    pub select: f32,
    /// +8..+0xc typed gap。
    pub _gap4: [u8; 4],
    /// +0xc 动画状态字节（0/1/2）。
    pub state: u8,
    /// +0xd..+0x10 typed gap。
    pub _pad3: [u8; 3],
}

const _UI_ANIMATION_STATE_SIZE: () = assert!(std::mem::size_of::<UiAnimationState>() == 0x10);

impl UiAnimationState {
    /// add_toolbar_item spawn 面的初始值（O：0x141e4e495..0x141e4e4e2 的载荷写入：
    /// `+0x20..+0x28 = 0`、`+0x28 dword = 0`、`+0x2c byte = 2`——即
    /// `{appear: 0.0, select: 0.0, _gap4: [0;4], state: 2, _pad3: [0;3]}`）。
    ///
    /// 与 `Default`（state = 0）不同：生成态字节从 **2** 开始。
    pub fn for_spawn() -> Self {
        Self {
            appear: 0.0,
            select: 0.0,
            _gap4: [0; 4],
            state: 2,
            _pad3: [0; 3],
        }
    }

    /// 外观/选择进度 + 状态字节更新（O ui_animation_state_update 公式，wave3 fresh
    /// 逐语句 + reviewer-wave3 FIX-F 补 state 写面）：
    /// - 离开（`entering=false`）：state==0 → state=2；否则 `select -= dt/0.2`、
    ///   state=1；
    /// - 进入（`entering=true`）：state!=1 → `select += dt/0.2`、state=0；
    ///   否则 state=2。
    /// `select` 最后 clamp01（O `pfVar18[1]`）。**判等（公式+state 写面）**；
    /// 分歧：进入/离开判别源（per-widget 字节记录，U）由调用侧传入。
    pub fn update_progress(&mut self, dt: f32, entering: bool) {
        if !entering {
            if self.state == 0 {
                self.state = 2;
            } else {
                self.select = (self.select - dt / 0.2).clamp(0.0, 1.0);
                self.state = 1;
            }
        } else {
            if self.state != 1 {
                self.select = (self.select + dt / 0.2).clamp(0.0, 1.0);
                self.state = 0;
            } else {
                self.state = 2;
            }
        }
    }
}

/// UI 鼠标可交互（O：`country_core::components::UiMouseInteractable`，本批在 toolbar
/// 本地落库暂置；B spawn 8 组件之一）。
///
/// **B 布局（4B，fresh `register_component<UiMouseInteractable>` 0x141e35eb0 的
/// `ComponentDescriptor` size 立即数 0x4、align 4）**。原 16B `{flag:u8,_gap:u64}`
/// 推断作废；字段语义 U（ui_2d_pass 读 +2 面）。
pub use country_core::components::UiMouseInteractable;

const _UI_MOUSE_INTERACTABLE_SIZE: () =
    assert!(std::mem::size_of::<UiMouseInteractable>() == 0x4);

/// UI 交互状态（O：`country_core::components::UiInteractionState`，本批在 toolbar
/// 本地落库暂置；ui_2d_pass 读 `+2` 字节）。
///
/// **B 布局（3B，fresh `register_component<UiInteractionState>` 0x141e34530 /
/// 0x14084fcc0 的 `ComponentDescriptor` size 立即数 0x3、align 1）**：
/// `{state: u8@0, keep: u8@1, selected: u8@2}`——**reviewer-A-toolbar FIX-T1**：
/// 原生 per-widget 三字节记录（`+0` appear 符号/交互层、`+1` keep、`+2` selected）就是
/// 本组件自身（非独立 `WidgetByteState`）。原 4B/16B 推断作废；wave3 “stride-4 u32
/// 写面”是 bevy `&mut` changed-tick 记账（`TicksMut::set_changed`），非业务写面
/// （Rust `&mut` 自动等价）。
pub use country_core::components::UiInteractionState;

const _UI_INTERACTION_STATE_SIZE: () =
    assert!(std::mem::size_of::<UiInteractionState>() == 0x3);

/// 仅当父级获得焦点时显示（O：`toolbar::resouces_and_components::UiShowIfParentIsInFocus`；
/// add_toolbar_item 对所有叶子无条件 insert）。
///
/// **B 布局（8B，fresh `register_required_components` impl$14 0x142386110 的
/// `ComponentDescriptor` size 立即数 0x8、align 1）**：`{parent: u64}`（字段语义 U——
/// 名称暗示父 id；不排除 8×u8 标志面）。
#[derive(Component, Clone, Copy, Debug, Default)]
#[repr(C)]
pub struct UiShowIfParentIsInFocus {
    /// +0 父/标志载荷（I）。
    pub parent: u64,
}

const _UI_SHOW_IF_PARENT_SIZE: () =
    assert!(std::mem::size_of::<UiShowIfParentIsInFocus>() == 0x8);

/// 仅第一人称与自由相机显示（O：`toolbar::resouces_and_components::ShowOnlyInFpvAndFree`；
/// add_toolbar_item 按 `item+0x38` 字节条件 insert）。
///
/// **B 布局（0B，ZST；fresh impl$15 0x142386800 size 0x0 / align 1）**。
#[derive(Component, Clone, Copy, Debug, Default)]
pub struct ShowOnlyInFpvAndFree;

/// 调试工具栏激活区（O：`toolbar::resouces_and_components::DebugToolbarActivationZone`）。
///
/// **B 布局（0B，ZST；fresh impl$4 0x142384550 size 0x0 / align 1）**。
#[derive(Component, Clone, Copy, Debug, Default)]
pub struct DebugToolbarActivationZone;

/// 自定义尺寸（O：`toolbar::resouces_and_components::UiCustomSize`）。
///
/// **B 布局（8B，fresh impl$16 0x142386ef0 size 0x8 / align 1）**：`{size: Vec2}`
/// （名称与 8B 尺寸推定；字段名 I）。
#[derive(Component, Clone, Copy, Debug, Default)]
#[repr(C)]
pub struct UiCustomSize {
    /// +0 两 f32（I）。
    pub size: glam::Vec2,
}

const _UI_CUSTOM_SIZE_SIZE: () = assert!(std::mem::size_of::<UiCustomSize>() == 0x8);

/// 工具栏按钮冷却（O：`toolbar::resouces_and_components::ToolbarButtonCooldown`；
/// **toolbar_button_keep_focus 0x1423414b0** 的 f32 冷却——逐帧 `-= dt`，>0 时
/// 保持焦点字节置位）。**重建**（I：单 f32；B 布局 4B@+0）。
#[derive(Component, Clone, Copy, Debug, Default, PartialEq)]
#[repr(C)]
pub struct ToolbarButtonCooldown {
    /// 冷却剩余（秒；`f32@+0`）。
    pub remaining: f32,
}

/// 锁定 UI 项（O：`toolbar::resouces_and_components::UiLocked`；ui_sounds
/// 0x14233a7f0 的 Query 过滤目标）。
///
/// **B 布局（0B，ZST；fresh impl$19 0x1423883c0 size 0x0 / align 1）**。
#[derive(Component, Clone, Copy, Debug, Default)]
pub struct UiLocked;

/// 切换应用模式按钮（O：`toolbar::resouces_and_components::SwitchAppModeButton`；
/// **switch_app_mode_button 0x142341fe0** 的 Query 目标，持有目标 AppMode）。
///
/// **B 布局（16B，fresh impl$5 0x142384c40 size 0x10 / align 8）**：`{mode: AppMode}`
/// （AppMode 本机尺寸 16B；原 `_gap:u64` 推断作废）。
#[derive(Component, Clone, Copy, Debug, Default)]
#[repr(C)]
pub struct SwitchAppModeButton {
    /// 目标应用模式（I）。
    pub mode: country_core::systems::mode_manager::AppMode,
}

const _SWITCH_APP_MODE_SIZE: () = assert!(std::mem::size_of::<SwitchAppModeButton>() == 0x10);

/// 切换工具栏类别按钮（O：`toolbar::resouces_and_components::SwitchToolbarCategoryButton`；
/// **switch_toolbar_category_button 0x1423427f0**，持有目标 ToolbarCategory 字节）。
///
/// **B 布局（1B，fresh impl$6 0x142385330 size 0x1 / align 1）**：
/// `{category: ToolbarCategory}`（类别枚举 1B；原 16B 推断作废）。
#[derive(Component, Clone, Copy, Debug, Default)]
#[repr(C)]
pub struct SwitchToolbarCategoryButton {
    /// 目标类别（I）。
    pub category: country_core::resources::toolbar::ToolbarCategory,
}

const _SWITCH_CATEGORY_SIZE: () =
    assert!(std::mem::size_of::<SwitchToolbarCategoryButton>() == 0x1);

/// 滑块组件（O：`toolbar::resouces_and_components::UiSlider`；ui_sounds /
/// photomode::ui_slider 的 Query 目标）。
///
/// **B 布局（24B = 0x18，fresh impl$26 0x14238b450 size 0x18 / align 1）**：
/// 字段面 U（`switch(entry+0x10)` 的 widget kind 判别在 dense 记录，不在本组件列）；
/// 本实现以 typed-gap 承载 24B，不造字段名。
#[derive(Component, Clone, Copy, Debug, Default)]
#[repr(C)]
pub struct UiSlider {
    /// +0..0x18 typed gap（布局 O，字段 U）。
    pub _gap: [u8; 0x18],
}

const _UI_SLIDER_SIZE: () = assert!(std::mem::size_of::<UiSlider>() == 0x18);

impl UiSlider {
    /// O: native `ui_slider` reads the widget kind as `*(u8*)(entry+0x10)`.
    /// The recovered 24-byte component keeps that slot in the typed gap; this
    /// accessor is the only reader and does not name the remaining fields.
    pub fn kind(&self) -> u8 {
        self._gap[0x10]
    }

    /// Writes the native kind slot (`+0x10`); used by the producer that spawns
    /// the slider entity.
    pub fn set_kind(&mut self, kind: u8) {
        self._gap[0x10] = kind;
    }
}

/// 拍照模式按钮（O：`toolbar::resouces_and_components::PhotomodeButton`，**枚举**
/// `enum2$`；ui_sounds 的 Query 目标 `Query<&PhotomodeButton>`）。
///
/// **B 布局（1B，fresh impl$25 0x14238ad60 size 0x1 / align 1）**：判别子@+0
/// （0.. 变体名 U）。
#[derive(Component, Clone, Copy, Debug, Default, PartialEq, Eq)]
#[repr(C)]
pub struct PhotomodeButton {
    /// 判别子（I）。
    pub tag: u8,
}

const _PHOTOMODE_BUTTON_SIZE: () = assert!(std::mem::size_of::<PhotomodeButton>() == 0x1);

/// 时间/昼夜按钮 V2（O：`toolbar::resouces_and_components::TimeOfDayButtonV2`；
/// **tod_v2_button 0x142342f30** 的 Query 目标）。
///
/// **B 布局（0B，ZST；fresh impl$22 0x142389890 size 0x0 / align 1）**。
#[derive(Component, Clone, Copy, Debug, Default)]
pub struct TimeOfDayButtonV2;

/// 拍照模式按钮显示门（O：`toolbar::resouces_and_components::ShowOnlyInPhotomode`；
/// ui_interaction 查询面 `Option<&ShowOnlyInPhotomode>`）。
///
/// **B 布局（0B，ZST；fresh impl$24 0x14238a670 size 0x0 / align 1）**。
#[derive(Component, Clone, Copy, Debug, Default)]
pub struct ShowOnlyInPhotomode;

/// 菜单打开时隐藏（O：`toolbar::resouces_and_components::HideIfMenuIsOpen`；
/// ui_interaction 查询面 + UndoRedoLayout::setup 条件 insert）。
///
/// **B 布局（0B，ZST；fresh impl$23 0x142389f80 size 0x0 / align 1）**。
#[derive(Component, Clone, Copy, Debug, Default)]
pub struct HideIfMenuIsOpen;

/// 工具栏展开（O：`toolbar::resouces_and_components::ToolbarExpand`；
/// toolbar_button_size_and_padding_update 的 `With<ToolbarExpand>` 过滤器）。
///
/// **B 布局（0B，ZST；fresh impl$21 0x1423891a0 size 0x0 / align 1）**。
#[derive(Component, Clone, Copy, Debug, Default)]
pub struct ToolbarExpand;

/// 滑块重置（O：`toolbar::resouces_and_components::UiSliderReset`）。
///
/// **B 布局（1B，fresh impl$27 0x14238bb40 size 0x1 / align 1）**：标志字节。
#[derive(Component, Clone, Copy, Debug, Default)]
#[repr(C)]
pub struct UiSliderReset {
    /// 标志（I）。
    pub flag: u8,
}

const _UI_SLIDER_RESET_SIZE: () = assert!(std::mem::size_of::<UiSliderReset>() == 0x1);

/// 播放选择动画（O：`toolbar::resouces_and_components::UiPlaySelectAnimation`；
/// switch_*_button / tod_v2_button / render_taffy 经 `Commands::entity(e).insert(...)`
/// 入队；0x28B 命令记录 payload 12B = `{f32@0, f32@4, u32@8}`）。
///
/// **B 布局（12B = 0xC，fresh impl$18 0x142387cd0 size 0xc / align 4）**：
/// 两 f32 观测值 `{0.1,0.2}`（tod_v2/switch_category）、`{0.2,0.2}`（switch_app_mode）、
/// render_taffy drag==3 也为 `{0.2,0.2}`；u32 载荷观测 0（U）。原 16B 含 byte/pad
/// 推断作废。
#[derive(Component, Clone, Copy, Debug, Default, PartialEq)]
#[repr(C)]
pub struct UiPlaySelectAnimation {
    /// +0 参数 A（观测 0.1/0.2）。
    pub a: f32,
    /// +4 参数 B（duration；观测 0.2）。
    pub b: f32,
    /// +8 progress（O 0x14233c300：`p += dt / b`, clamp01, stored back as f32）。
    /// The old `_pad_u32` name was wrong: native reads/writes this word as f32.
    pub progress: f32,
}

const _UI_PLAY_SELECT_SIZE: () = assert!(std::mem::size_of::<UiPlaySelectAnimation>() == 0xC);

/// 播放锁定动画（O：`toolbar::resouces_and_components::UiPlayLockedAnimation`）。
///
/// **B 布局（12B = 0xC，fresh impl$20 0x142388ab0 size 0xc / align 4）**：与
/// `UiPlaySelectAnimation` 同形 `{f32, f32, u32}`（字段语义 U）。
#[derive(Component, Clone, Copy, Debug, Default, PartialEq)]
#[repr(C)]
pub struct UiPlayLockedAnimation {
    /// +0 参数 A（U）。
    pub a: f32,
    /// +4 参数 B（U）。
    pub b: f32,
    /// +8 progress（O 0x14233c8b0 公式的 p；原 `_pad_u32` 名称作废）。
    pub progress: f32,
}

const _UI_PLAY_LOCKED_SIZE: () = assert!(std::mem::size_of::<UiPlayLockedAnimation>() == 0xC);

/// per-widget stride-3 字节记录容器（B 模型：dense `records[widget_id]`，stride 3）。
///
/// **reviewer-A-toolbar FIX-T1**：原生 dense 三字节记录（`+0` appear 符号 / `+1` keep /
/// `+2` selected）就是 `UiInteractionState` 组件本身（descriptor size=3），本容器以
/// `Vec<UiInteractionState>` 表达同一 dense 模型（widget id 索引）。四系统已改为直接
/// 读/写 per-entity `UiInteractionState`；本容器供 subtree 迭代 / render_taffy_ui_to_widgets
/// 写链使用（不再有独立 `WidgetByteState`）。
#[derive(Component, Clone, Debug, Default)]
pub struct PerWidgetButtonState {
    /// `records[widget_id]`（O：dense `widget_id * 3` 字节记录 = UiInteractionState）。
    pub records: Vec<UiInteractionState>,
}

impl PerWidgetButtonState {
    /// 越界返回默认记录（O：dense 表越界为 UB 面，bevy 归一化以默认承载，I 级）。
    pub fn get(&self, widget: u32) -> UiInteractionState {
        self.records.get(widget as usize).copied().unwrap_or_default()
    }
    fn get_mut_or_extend(&mut self, widget: u32) -> &mut UiInteractionState {
        let idx = widget as usize;
        if idx >= self.records.len() {
            self.records.resize(idx + 1, UiInteractionState::default());
        }
        &mut self.records[idx]
    }
    /// +1 keep 字节（toolbar_button_keep_focus 读/写）。
    pub fn keep(&self, widget: u32) -> u8 {
        self.get(widget).keep
    }
    pub fn set_keep(&mut self, widget: u32, value: u8) {
        self.get_mut_or_extend(widget).keep = value;
    }
    /// +0 appear 符号字节（ui_animation_state_update 读）。
    pub fn appear_sign(&self, widget: u32) -> u8 {
        self.get(widget).state
    }
    /// +2 选中标志（switch_*_button 读/写）。
    pub fn selected(&self, widget: u32) -> u8 {
        self.get(widget).selected
    }
    pub fn set_selected(&mut self, widget: u32, value: u8) {
        self.get_mut_or_extend(widget).selected = value;
    }
}

/// focused-window 子树表（O：`toolbar_button_keep_focus`/`switch_*_button`/
/// `ui_animation_state_update` 的查询迭代面——窗口实体 +0x50 子列表，stride 0x10）。
///
/// B 记录 `{entity u64@+0, widget_id u32@+8}`（fresh：`uVar9=*(u64*)(list+idx*0x10)` 实体、
/// `uVar14=*(u32*)(list+8+idx*0x10)` widget id）。子树表承载窗口→widget 列表，
/// 每 widget 的 stride-3 字节记录经 `PerWidgetButtonState` 索引。
#[derive(Component, Clone, Debug, Default)]
pub struct FocusedWidgetSubtree {
    /// 子树 widget 列表（stride 0x10 `{entity, widget_id}`）。
    pub widgets: Vec<FocusedWidget>,
}

/// focused-window 子树表项（O：stride 0x10 `{entity u64@+0, widget_id u32@+8}`）。
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct FocusedWidget {
    /// +0 widget 实体（UiPlaySelectAnimation 插入目标）。
    pub entity: Entity,
    /// +8 widget id（per-widget 容器索引）。
    pub widget_id: u32,
}

/// 单条导航链接记录（O：`UiNav` 每 16B 槽；fresh `link_up_nav` 0x141e4d880
/// 写面：`{tag: u64@+0, value: u64@+8}`）。
///
/// 观测 tag 值：`0`（未连接）与 `3`（已连接，邻居 id 在 value）。tag 3 的枚举
/// 语义（3 变体枚举的 niche None / 具体变体名）**U**；本实现保留原字节值。
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct UiNavLink {
    /// +0 判别/标签（观测 0 / 3）。
    pub tag: u64,
    /// +8 邻居 GlobalId（tag==3 时有效）。
    pub value: u64,
}

const _UI_NAV_LINK_SIZE: () = assert!(std::mem::size_of::<UiNavLink>() == 0x10);

/// UiNav 组件（O：**`country_core::components::UiNav`**——PDB 路径见
/// `render_taffy_ui_to_widgets` Query 面与 `link_up_nav` insert 命令泛型；
/// 本 crate 暂置本地，core 落库后应移回）。
///
/// **B 布局（0x40 = 64B，fresh `register_component<UiNav>` 0x141e36510
/// `ComponentDescriptor` size 立即数 0x40）**：四槽 `UiNavLink`（各 16B）。
///
/// 写面（`ToolbarLayout::link_up_nav` 0x141e4d880，O）：
/// - 横向（`in_DL==0`）：slot0/slot1 tag=0；slot2={3,next}、slot3={3,prev}；
/// - 纵向（`in_DL!=0`）：slot0={3,prev}、slot1={3,next}；slot2/slot3 tag=0。
///
/// 槽语义名（up/down/left/right 的排序）**U**——按索引写入，不猜名称。
///
/// **I 级残差**：`link_up_nav` 对“未连接”槽只显式写 tag qword = 0（横向 slot0/1、
/// 纵向 slot2/3），其 value qword 在原栈记录中未显式写（栈初值 U）。本实现取 0
/// （`Default`），为可复现的结构化选择，登记为 I。
#[derive(Component, Clone, Copy, Debug, Default, PartialEq, Eq)]
#[repr(C)]
pub struct UiNav {
    /// 四条链接（布局 O；语义名 U）。
    pub links: [UiNavLink; 4],
}

const _UI_NAV_SIZE: () = assert!(std::mem::size_of::<UiNav>() == 0x40);

impl UiNav {
    /// `link_up_nav(in_DL==0)` 的槽写面（O）：slot2=next / slot3=prev，slot0/1 清零。
    pub fn horizontal(prev: u64, next: u64) -> Self {
        Self {
            links: [
                UiNavLink::default(),
                UiNavLink::default(),
                UiNavLink { tag: 3, value: next },
                UiNavLink { tag: 3, value: prev },
            ],
        }
    }

    /// `link_up_nav(in_DL!=0)` 的槽写面（O）：slot0=prev / slot1=next，slot2/3 清零。
    pub fn vertical(prev: u64, next: u64) -> Self {
        Self {
            links: [
                UiNavLink { tag: 3, value: prev },
                UiNavLink { tag: 3, value: next },
                UiNavLink::default(),
                UiNavLink::default(),
            ],
        }
    }
}

/// 重建 `link_up_nav` 0x141e4d880 的 payload 写面（命令队列插入由调用侧负责）。
///
/// 逐语句对应：取 id 列表 → 收集每个 id 的 `+0x8` qword → 对第 i 项
/// `prev=ids[(i+n-1)%n]`、`next=ids[(i+1)%n]` → `horizontal/vertical` 写面。
/// **重建**（I：tag=3 语义 U；命令队列/`UiNav` 组件 insert 由消费接线完成）。
pub fn link_up_nav(ids: &[u64], horizontal: bool) -> Vec<(u64, UiNav)> {
    let n = ids.len();
    if n == 0 {
        return Vec::new();
    }
    let mut out = Vec::with_capacity(n);
    for (i, &id) in ids.iter().enumerate() {
        let prev = ids[(i + n - 1) % n];
        let next = ids[(i + 1) % n];
        let nav = if horizontal {
            UiNav::horizontal(prev, next)
        } else {
            UiNav::vertical(prev, next)
        };
        out.push((id, nav));
    }
    out
}

/// HistoryButton 组件（O：**`country_core::components::HistoryButton`**——PDB 路径见
/// `render_taffy_ui_to_widgets` Query 面；本 crate 暂置本地）。
///
/// **B 布局（1B，fresh `register_component<HistoryButton>` 0x141e34b90
/// `ComponentDescriptor` size 立即数 0x1）**：`{kind: u8}`。
///
/// `UndoRedoLayout::setup` 0x140fad870（O）：两按钮静态表
/// `("icon/undoicon/redo", flag=0)` / `("icon/redo", flag=1)` → insert 写
/// `byte payload = flag & 1`；故 kind 观测值 **0 / 1**（0=undo 组、1=redo 组，
/// 语义映射 I）。
#[derive(Component, Clone, Copy, Debug, Default, PartialEq, Eq)]
#[repr(C)]
pub struct HistoryButton {
    /// +0 按钮种类（观测 0/1；语义 I）。
    pub kind: u8,
}

const _HISTORY_BUTTON_SIZE: () = assert!(std::mem::size_of::<HistoryButton>() == 0x1);

impl HistoryButton {
    /// `UndoRedoLayout::setup` 首项（flag=0）。
    pub const UNDO: HistoryButton = HistoryButton { kind: 0 };
    /// `UndoRedoLayout::setup` 次项（flag=1）。
    pub const REDO: HistoryButton = HistoryButton { kind: 1 };
}

/// 对数滑块（O：`toolbar::resouces_and_components::LogSlider`；`{min, max, _, log2_base}`
/// 四 f32；[3] 语义 I——加到 exp2 指数）。
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct LogSlider {
    /// [0] 最小值。
    pub min: f32,
    /// [1] 最大值。
    pub max: f32,
    /// [2] 未用/步进候选（U）。
    pub _pad: f32,
    /// [3] exp2 指数基数（O 读面：`exp2f(clamp+[3])`；语义 I）。
    pub log2_base: f32,
}

impl Default for LogSlider {
    fn default() -> Self {
        Self {
            min: 0.0,
            max: 1.0,
            _pad: 0.0,
            log2_base: 0.0,
        }
    }
}

impl LogSlider {
    /// O 0x142383730 `param_to_value(param)`：
    /// `p = param.clamp(min, max)`（min>max 触发 `f32::clamp` 的 do_panic::runtime）；
    /// 返回 `exp2f(p + log2_base)`。**判等**。
    pub fn param_to_value(&self, param: f32) -> f32 {
        let p = param.clamp(self.min, self.max);
        (p + self.log2_base).exp2()
    }
    /// O `LogSlider::value_to_param` (inline, 2 instances; host
    /// ui_slider_reset case 7): inverse of `param_to_value`, i.e.
    /// `log2(value)` clamped to `[min, max]`.
    pub fn value_to_param(&self, value: f32) -> f32 {
        value.log2().clamp(self.min, self.max)
    }
}

// ------------------------------------------------------------------
// 26 个共享 UI 组件（B `register_required_components` 面）
// ------------------------------------------------------------------
//
// 口径：B 的 `toolbar::resouces_and_components::{impl$2..impl$27}` 共 20 个
// `register_required_components` 单态（`evidence/toolbar-origin-2026-10-16/procs.tsv`
// 逐地址核对，impl 号 2/3/4/5/6/13..27），加 6 个共享 UI 类型（UiNav/HistoryButton/
// UiInteractionState/UiMouseInteractable/Ui2dExtent/Ui2dImage）= **26**。
// 各组件尺寸来自 fresh `ComponentDescriptor` size 立即数（`register_component<T>`
// 反汇编；20 个本地项地址见下），非猜测。
//
// `register_required_components` 本体是 bevy `#[derive(Component)]` 生成胶水
// （AGENTS.md glue-legitimate），不逐函数重建；组件**类型/布局**按上表落库。
// 本地 24 项由 `register_local_components` 统一 `World::register_component` 注册；
// `Ui2dExtent`/`Ui2dImage` 归 country-core（`components/trees.rs`），由 core 注册。

/// 26 个共享 UI 组件台账：`(类型名, size_of, 归属模块)`。
pub const SHARED_UI_COMPONENTS: [(&str, usize, &str); 26] = [
    // --- toolbar::resouces_and_components（20 个 register_required_components）---
    ("TaffyNode", 0x14, "toolbar::resouces_and_components"),
    ("UiScaleWithUserSetting", 0x0, "toolbar::resouces_and_components"),
    ("DebugToolbarActivationZone", 0x0, "toolbar::resouces_and_components"),
    ("SwitchAppModeButton", 0x10, "toolbar::resouces_and_components"),
    ("SwitchToolbarCategoryButton", 0x1, "toolbar::resouces_and_components"),
    ("UiAnimationState", 0x10, "toolbar::resouces_and_components"),
    ("UiShowIfParentIsInFocus", 0x8, "toolbar::resouces_and_components"),
    ("ShowOnlyInFpvAndFree", 0x0, "toolbar::resouces_and_components"),
    ("UiCustomSize", 0x8, "toolbar::resouces_and_components"),
    ("ToolbarButtonCooldown", 0x4, "toolbar::resouces_and_components"),
    ("UiPlaySelectAnimation", 0xC, "toolbar::resouces_and_components"),
    ("UiLocked", 0x0, "toolbar::resouces_and_components"),
    ("UiPlayLockedAnimation", 0xC, "toolbar::resouces_and_components"),
    ("ToolbarExpand", 0x0, "toolbar::resouces_and_components"),
    ("TimeOfDayButtonV2", 0x0, "toolbar::resouces_and_components"),
    ("HideIfMenuIsOpen", 0x0, "toolbar::resouces_and_components"),
    ("ShowOnlyInPhotomode", 0x0, "toolbar::resouces_and_components"),
    ("PhotomodeButton", 0x1, "toolbar::resouces_and_components"),
    ("UiSlider", 0x18, "toolbar::resouces_and_components"),
    ("UiSliderReset", 0x1, "toolbar::resouces_and_components"),
    // --- 共享 UI 类型（PDB 路径 country_core::components）---
    ("UiNav", 0x40, "country_core::components（toolbar 暂置）"),
    ("HistoryButton", 0x1, "country_core::components（toolbar 暂置）"),
    ("UiInteractionState", 0x3, "country_core::components（toolbar 暂置）"),
    ("UiMouseInteractable", 0x4, "country_core::components（toolbar 暂置）"),
    ("Ui2dExtent", 0x10, "country_core::components"),
    ("Ui2dImage", 0x20, "country_core::components"),
];

/// 注册本地 24 个共享 UI 组件（`register_required_components` 的运行时等价入口；
/// 对应 ToolbarPlugin build / 依赖线接线）。**胶水-legitimate**。
pub fn register_local_components(world: &mut bevy_ecs::world::World) {
    world.register_component::<TaffyNode>();
    world.register_component::<UiScaleWithUserSetting>();
    world.register_component::<DebugToolbarActivationZone>();
    world.register_component::<SwitchAppModeButton>();
    world.register_component::<SwitchToolbarCategoryButton>();
    world.register_component::<UiAnimationState>();
    world.register_component::<UiShowIfParentIsInFocus>();
    world.register_component::<ShowOnlyInFpvAndFree>();
    world.register_component::<UiCustomSize>();
    world.register_component::<ToolbarButtonCooldown>();
    world.register_component::<UiPlaySelectAnimation>();
    world.register_component::<UiLocked>();
    world.register_component::<UiPlayLockedAnimation>();
    world.register_component::<ToolbarExpand>();
    world.register_component::<TimeOfDayButtonV2>();
    world.register_component::<HideIfMenuIsOpen>();
    world.register_component::<ShowOnlyInPhotomode>();
    world.register_component::<PhotomodeButton>();
    world.register_component::<UiSlider>();
    world.register_component::<UiSliderReset>();
    world.register_component::<UiNav>();
    world.register_component::<HistoryButton>();
    world.register_component::<UiInteractionState>();
    world.register_component::<UiMouseInteractable>();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn param_to_value_log_mapping() {
        // O 0x142383730：clamp 后 exp2(clamp+base)。
        let s = LogSlider {
            min: 0.0,
            max: 4.0,
            _pad: 0.0,
            log2_base: 0.0,
        };
        assert!((s.param_to_value(2.0) - 4.0).abs() < 1e-6);
        // 越界夹取
        assert!((s.param_to_value(-5.0) - 1.0).abs() < 1e-6);
        assert!((s.param_to_value(10.0) - 16.0).abs() < 1e-6);
        // 指数基数偏移
        let s2 = LogSlider {
            log2_base: 1.0,
            ..s
        };
        assert!((s2.param_to_value(2.0) - 8.0).abs() < 1e-6);
    }

    /// O: value_to_param is log2(value) clamped to the slider range and is the
    /// inverse of param_to_value where the mapping is representable.
    #[test]
    fn value_to_param_inverts_and_clamps() {
        let s = LogSlider {
            min: 0.0,
            max: 4.0,
            _pad: 0.0,
            log2_base: 0.0,
        };
        assert!((s.value_to_param(4.0) - 2.0).abs() < 1e-6);
        assert_eq!(s.value_to_param(0.25), 0.0);
        assert_eq!(s.value_to_param(64.0), 4.0);
        let param = 2.0;
        assert!((s.value_to_param(s.param_to_value(param)) - param).abs() < 1e-6);
    }

    #[test]
    fn ui_interaction_state_layout_and_accessors() {
        // FIX-T1：3B 记录 = UiInteractionState 本身：+0 state / +1 keep / +2 selected。
        assert_eq!(std::mem::size_of::<UiInteractionState>(), 3);
        assert_eq!(std::mem::offset_of!(UiInteractionState, state), 0);
        assert_eq!(std::mem::offset_of!(UiInteractionState, keep), 1);
        assert_eq!(std::mem::offset_of!(UiInteractionState, selected), 2);

        // dense 容器（widget_id 索引）读写。
        let mut container = PerWidgetButtonState::default();
        assert_eq!(container.keep(7), 0);
        container.set_keep(7, 1);
        assert_eq!(container.keep(7), 1);
        container.set_selected(3, 1);
        assert_eq!(container.selected(3), 1);
        assert_eq!(container.appear_sign(3), 0);
    }

    /// 26 组件尺寸台账与 fresh `ComponentDescriptor` 立即数一致。
    #[test]
    fn shared_ui_component_sizes_match_fresh_descriptors() {
        let sizes = [
            std::mem::size_of::<TaffyNode>(),
            std::mem::size_of::<UiScaleWithUserSetting>(),
            std::mem::size_of::<DebugToolbarActivationZone>(),
            std::mem::size_of::<SwitchAppModeButton>(),
            std::mem::size_of::<SwitchToolbarCategoryButton>(),
            std::mem::size_of::<UiAnimationState>(),
            std::mem::size_of::<UiShowIfParentIsInFocus>(),
            std::mem::size_of::<ShowOnlyInFpvAndFree>(),
            std::mem::size_of::<UiCustomSize>(),
            std::mem::size_of::<ToolbarButtonCooldown>(),
            std::mem::size_of::<UiPlaySelectAnimation>(),
            std::mem::size_of::<UiLocked>(),
            std::mem::size_of::<UiPlayLockedAnimation>(),
            std::mem::size_of::<ToolbarExpand>(),
            std::mem::size_of::<TimeOfDayButtonV2>(),
            std::mem::size_of::<HideIfMenuIsOpen>(),
            std::mem::size_of::<ShowOnlyInPhotomode>(),
            std::mem::size_of::<PhotomodeButton>(),
            std::mem::size_of::<UiSlider>(),
            std::mem::size_of::<UiSliderReset>(),
            std::mem::size_of::<UiNav>(),
            std::mem::size_of::<HistoryButton>(),
            std::mem::size_of::<UiInteractionState>(),
            std::mem::size_of::<UiMouseInteractable>(),
        ];
        for (i, (name, expected, _)) in SHARED_UI_COMPONENTS.iter().take(24).enumerate() {
            assert_eq!(sizes[i], *expected, "component {name} size mismatch");
        }
        // core 类型尺寸（同批 fresh register_component 证据，不在本 crate 定义）。
        assert_eq!(SHARED_UI_COMPONENTS[24], ("Ui2dExtent", 0x10, "country_core::components"));
        assert_eq!(SHARED_UI_COMPONENTS[25], ("Ui2dImage", 0x20, "country_core::components"));
    }

    /// `link_up_nav` 0x141e4d880 的槽写面：横向 slot2=next/slot3=prev，
    /// slot0/1 清零；纵向 slot0=prev/slot1=next，slot2/3 清零（O，tag=3）。
    #[test]
    fn link_up_nav_writes_expected_slots() {
        let ids = [10u64, 20, 30];
        let h = link_up_nav(&ids, true);
        assert_eq!(h.len(), 3);
        assert_eq!(h[0].0, 10);
        assert_eq!(h[0].1.links[2], UiNavLink { tag: 3, value: 20 });
        assert_eq!(h[0].1.links[3], UiNavLink { tag: 3, value: 30 });
        assert_eq!(h[0].1.links[0], UiNavLink::default());
        assert_eq!(h[0].1.links[1], UiNavLink::default());

        let v = link_up_nav(&ids, false);
        assert_eq!(v[1].1.links[0], UiNavLink { tag: 3, value: 10 });
        assert_eq!(v[1].1.links[1], UiNavLink { tag: 3, value: 30 });
        assert_eq!(v[1].1.links[2], UiNavLink::default());
        assert_eq!(v[1].1.links[3], UiNavLink::default());
    }
}
