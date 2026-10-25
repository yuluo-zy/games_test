//! Owned toolbar item trees and native Taffy/Bevy construction.
//! Original B: add_toolbar_item 0x141e4dc40; add_contextual_menu 0x141e4d1e0.
//! Evidence: progress-20261020/phase2-toolbar-layout. Rust owns the original
//! strings, child Vecs and FnOnce payloads rather than emulating their old ABI.

use bevy_ecs::{entity::Entity, system::{Commands, EntityCommands}};
use glam::Vec2;
use country_core::systems::mode_manager::AppMode;

pub struct ToolbarItem {
    pub kind: ToolbarItemKind,
    pub label: String,
    pub size: Vec2,
    pub show_only_fpv: bool,
}

pub enum ToolbarItemKind {
    Custom(Box<dyn for<'a> FnOnce(&mut EntityCommands<'a>) + Send + Sync>),
    AppMode(AppMode),
    Slider(u8),
    SliderReset(u8),
    Row(Vec<ToolbarItem>),
    Column(Vec<ToolbarItem>),
    Spacer,
    Locked,
}

impl ToolbarItem {
    pub fn new(kind: ToolbarItemKind, label: impl Into<String>, size: Vec2) -> Self {
        Self { kind, label: label.into(), size, show_only_fpv: false }
    }
}

/// Native min/max/default tables at 142f8f6d8/142f8f70c/142f8f740.
pub fn initial_slider_value_01(kind: u8) -> f32 {
    const DEFAULTS: [f32; 13] = [0., 1., 1., 0., 0., 0.015, 0.5, 0.5, 0.5, 1., 0.8, 2., 3.];
    let value = DEFAULTS[kind as usize];
    crate::photomode::slider_math::normalized(kind, value).expect("native UiSliderType")
}

/// Original Style constants, captured before Taffy::new_leaf for all three
/// branches. Only containers receive the 10-point gap; every item is centered.
fn item_style(direction: Option<taffy::style::FlexDirection>) -> taffy::style::Style {
    use taffy::{geometry::Size, style::{AlignItems, JustifyContent, LengthPercentage, Style}};
    Style {
        align_items: Some(AlignItems::Center),
        justify_content: Some(JustifyContent::Center),
        flex_direction: direction.unwrap_or_default(),
        gap: Size { width: LengthPercentage::Points(if direction.is_some() { 10. } else { 0. }),
                    height: LengthPercentage::Points(if direction.is_some() { 10. } else { 0. }) },
        ..Style::default()
    }
}

/// Consume the complete original eight-variant item tree. Containers allocate
/// only a Taffy node; all descendants retain the caller's focus-parent Entity.
/// Safe ownership preserves FnOnce execution and drops unconsumed tails on panic.
pub fn add_toolbar_item(
    taffy: &mut taffy::node::Taffy,
    parent: taffy::node::Node,
    item: ToolbarItem,
    commands: &mut Commands,
    parent_entity: Entity,
) -> Option<Entity> {
    use super::*;
    use taffy::style::FlexDirection;
    let direction = match &item.kind {
        ToolbarItemKind::Row(_) => Some(FlexDirection::Row),
        ToolbarItemKind::Column(_) => Some(FlexDirection::Column),
        _ => None,
    };
    let node = taffy.new_leaf(item_style(direction)).expect("native toolbar item new_leaf");
    taffy.add_child(parent, node).expect("native toolbar item add_child");
    let kind = match item.kind {
        ToolbarItemKind::Row(children) | ToolbarItemKind::Column(children) => {
            for child in children {
                add_toolbar_item(taffy, node, child, commands, parent_entity);
            }
            return None;
        }
        kind => kind,
    };
    let mut leaf = TaffyNode { node, ..Default::default() };
    crate::taffy_update::set_taffy_node_base_size(&mut leaf, item.size * 82.5);
    let mut entity = commands.spawn((
        leaf,
        UiScaleWithUserSetting,
        UiInteractionState::default(),
        UiAnimationState::for_spawn(),
        UiMouseInteractable { bytes: 0.011f32.to_le_bytes() },
        country_core::components::Ui2dExtent::default(),
        country_core::components::Ui2dImage::new(item.label),
        country_core::components::UiDrawOrder::default(),
    ));
    entity.insert(UiShowIfParentIsInFocus { parent: parent_entity.to_bits() });
    if item.show_only_fpv {
        entity.insert(ShowOnlyInFpvAndFree);
    }
    match kind {
        ToolbarItemKind::Custom(callback) => {
            callback(&mut entity);
            entity.insert(UiNav::default());
            entity.insert(ToolbarExpand);
        }
        ToolbarItemKind::AppMode(mode) => {
            entity.insert(SwitchAppModeButton { mode });
            entity.insert(UiNav::default());
            entity.insert(ToolbarExpand);
        }
        ToolbarItemKind::Slider(kind) => {
            let mut slider = UiSlider::default();
            slider.set_previous_value_01(initial_slider_value_01(kind));
            slider.set_kind(kind);
            let mut nav = UiNav::default();
            nav.links[0].tag = 2;
            nav.links[1].tag = 2;
            entity.insert(slider);
            entity.insert(nav);
            entity.insert(ToolbarExpand);
        }
        ToolbarItemKind::SliderReset(flag) => {
            entity.insert(UiSliderReset { flag });
            entity.insert(UiNav::default());
            entity.insert(ToolbarExpand);
        }
        ToolbarItemKind::Spacer => {
            // Original 8B payload is u32 tag=1 followed by f32 value=1.
            entity.insert(UiCustomSize { kind: 1, value: 1.0 });
        }
        ToolbarItemKind::Locked => { entity.insert(UiLocked); entity.insert(ToolbarExpand); }
        ToolbarItemKind::Row(_) | ToolbarItemKind::Column(_) => unreachable!(),
    }
    Some(entity.id())
}

/// 一条 contextual menu 定义/实例记录。
///
/// Native stride 0x20（`add_contextual_menu` 0x141e4d1e0 / `toolbar_alignment`
/// 0x142344560）：`relative_to Node@+0x00`、`menu Node@+0x10`（0 = 尚未创建）、
/// `tag@+0x18`。
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct ContextualMenu {
    /// 定义表 +0x00：菜单锚定目标节点。
    pub relative_to: taffy::node::Node,
    /// Definition row +0x08: focus parent Entity, passed to every descendant.
    pub parent_entity: Entity,
    /// 定义表 +0x10：菜单根节点（创建前为 `Node::default()`）。
    pub node: taffy::node::Node,
    /// 定义表 +0x18：菜单 tag。
    pub tag: u8,
}

impl Default for ContextualMenu {
    fn default() -> Self {
        Self { relative_to: Default::default(), parent_entity: Entity::PLACEHOLDER,
               node: Default::default(), tag: 0 }
    }
}

/// 工具栏布局资源（native `ToolbarLayout`）。
///
/// **B 字段读面（O，0x141e4ccb0 / 0x141e4d1e0 / 0x142344560）**：
/// - `+0x08/+0x10`：contextual menu 定义表 `Vec`（ptr/len，stride 0x20）；
/// - `+0x18`：Node A（对齐批改目标，U 名称）；
/// - `+0x20`：Node B（对齐/间距批改目标，U 名称）；
/// - `+0x28`：`f32`（`toolbar_button_size_and_padding_update` 的 BackOut 输入，
///   U 名称）；
/// - `+0x2c`：`f32` 自动隐藏进度（`toolbar_auto_hide` clamp01 写面，O）。
#[derive(bevy_ecs::prelude::Resource, Default)]
pub struct ToolbarLayout {
    /// native +0x08/+0x10 定义表。
    pub menus: Vec<ContextualMenu>,
    /// native +0x18。
    pub node_a: taffy::node::Node,
    /// native +0x20。
    pub node_b: taffy::node::Node,
    /// native +0x28（U 语义；BackOut easing 输入）。
    pub scale_progress: f32,
    /// native +0x2c（自动隐藏进度，clamp01）。
    pub hide_progress: f32,
}

/// 拍照模式工具栏布局资源（native `PhotomodeToolbarLayout`）。
///
/// PDB 为独立类型（`toolbar_alignment` / `toolbar_button_size_and_padding_update`
/// 依次读取二者），字段与 [`ToolbarLayout`] 同构。
#[derive(bevy_ecs::prelude::Resource, Default)]
pub struct PhotomodeToolbarLayout {
    /// native +0x08/+0x10 定义表。
    pub menus: Vec<ContextualMenu>,
    /// native +0x18。
    pub node_a: taffy::node::Node,
    /// native +0x20。
    pub node_b: taffy::node::Node,
    /// native +0x28（U 语义）。
    pub scale_progress: f32,
    /// native +0x2c（自动隐藏进度）。
    pub hide_progress: f32,
}

/// Shared field view of the two context resources. Native `toolbar_alignment`
/// and `toolbar_button_size_and_padding_update` run the same body once per
/// resource (two-iteration loop in 0x142344560 / 0x141eaec50); this trait keeps
/// that single body without duplicating it.
pub trait ToolbarContext {
    fn node_a(&self) -> taffy::node::Node;
    fn node_b(&self) -> taffy::node::Node;
    fn contextual_menus(&self) -> &[ContextualMenu];
    fn scale_progress(&self) -> f32;
    fn hide_progress(&self) -> f32;
    fn set_hide_progress(&mut self, value: f32);
}

impl ToolbarContext for ToolbarLayout {
    fn node_a(&self) -> taffy::node::Node {
        self.node_a
    }
    fn node_b(&self) -> taffy::node::Node {
        self.node_b
    }
    fn contextual_menus(&self) -> &[ContextualMenu] {
        &self.menus
    }
    fn scale_progress(&self) -> f32 {
        self.scale_progress
    }
    fn hide_progress(&self) -> f32 {
        self.hide_progress
    }
    fn set_hide_progress(&mut self, value: f32) {
        self.hide_progress = value;
    }
}

impl ToolbarContext for PhotomodeToolbarLayout {
    fn node_a(&self) -> taffy::node::Node {
        self.node_a
    }
    fn node_b(&self) -> taffy::node::Node {
        self.node_b
    }
    fn contextual_menus(&self) -> &[ContextualMenu] {
        &self.menus
    }
    fn scale_progress(&self) -> f32 {
        self.scale_progress
    }
    fn hide_progress(&self) -> f32 {
        self.hide_progress
    }
    fn set_hide_progress(&mut self, value: f32) {
        self.hide_progress = value;
    }
}

impl ToolbarLayout {
    /// O 0x141e4ccb0 `ToolbarLayout::setup_screen_and_toolbar_nodes`.
    ///
    /// Native takes only the `TaffyLayout` owner (param_2) and returns two
    /// builders; Rust cannot return two builders borrowing the same `&mut`, so
    /// this port returns the two nodes. `root()`/further anchoring is the
    /// caller's (`toolbar_setup`) responsibility.
    pub fn setup_screen_and_toolbar_nodes(
        taffy: &mut crate::resouces_and_components::taffy_layout::TaffyLayout,
    ) -> (taffy::node::Node, taffy::node::Node) {
        use crate::resouces_and_components::taffy_layout::{AnchorSide, AnchoredNode};
        let screen = taffy
            .taffy
            .new_leaf(taffy::style::Style {
                padding: taffy::geometry::Rect {
                    left: taffy::style::LengthPercentage::Points(30.),
                    right: taffy::style::LengthPercentage::Points(30.),
                    top: taffy::style::LengthPercentage::Points(30.),
                    bottom: taffy::style::LengthPercentage::Points(30.),
                },
                align_items: Some(taffy::style::AlignItems::FlexEnd),
                justify_content: Some(taffy::style::JustifyContent::Center),
                ..Default::default()
            })
            .expect("native screen root new_leaf");
        // Native writes order=0, offset=(0,0), node=screen, side=4; the
        // Window branch never reads relative_to (filled with screen itself, I).
        taffy.anchored.push(AnchoredNode {
            order: 0,
            offset: glam::Vec2::ZERO,
            node: screen,
            relative_to: screen,
            gap: 0.,
            side: AnchorSide::Window,
        });
        let toolbar = taffy
            .taffy
            .new_leaf(item_style(Some(taffy::style::FlexDirection::Row)))
            .expect("native toolbar new_leaf");
        taffy
            .taffy
            .add_child(screen, toolbar)
            .expect("native toolbar add_child");
        (screen, toolbar)
    }

    /// Native generic category constructor (1426235d0, 1756B). Its callback
    /// inserts the real category-specific payload, before the common markers.
    pub fn add_category_button(
        &mut self,
        taffy: &mut crate::resouces_and_components::taffy_layout::TaffyLayout,
        category: country_core::resources::toolbar::ToolbarCategory,
        label: &str,
        commands: &mut Commands,
        custom: impl FnOnce(&mut EntityCommands),
        size_mult: Option<f32>,
    ) -> Entity {
        self.add_category_button_ordered(taffy, category, label, commands, custom, size_mult, false)
    }

    /// Both native monomorphizations share all construction logic; photo
    /// inserts its callback before the cooldown, normal toolbar after it.
    pub(crate) fn add_category_button_ordered(
        &mut self,
        taffy: &mut crate::resouces_and_components::taffy_layout::TaffyLayout,
        category: country_core::resources::toolbar::ToolbarCategory,
        label: &str,
        commands: &mut Commands,
        custom: impl FnOnce(&mut EntityCommands),
        size_mult: Option<f32>,
        cooldown_after_custom: bool,
    ) -> Entity {
        use super::*;
        let node = taffy.taffy.new_leaf(item_style(None)).expect("native category new_leaf");
        taffy.taffy.add_child(self.node_b, node).expect("native category add_child");
        let mut leaf = TaffyNode { node, ..Default::default() };
        let size = if let Some(mult) = size_mult { 110. * mult } else { 110. };
        crate::taffy_update::set_taffy_node_base_size(&mut leaf, Vec2::splat(size));
        let mut animation = UiAnimationState::for_spawn();
        animation.appear = 1.;
        let mut entity = commands.spawn((leaf, UiScaleWithUserSetting,
            UiInteractionState { state: 1, keep: 0, selected: 0 }, animation,
            UiMouseInteractable { bytes: 0.011f32.to_le_bytes() },
            country_core::components::Ui2dExtent::default(),
            country_core::components::Ui2dImage::new(label),
            country_core::components::UiDrawOrder::default()));
        if !cooldown_after_custom { entity.insert(ToolbarButtonCooldown { remaining: 0. }); }
        custom(&mut entity);
        if cooldown_after_custom { entity.insert(ToolbarButtonCooldown { remaining: 0. }); }
        entity.insert(ToolbarExpand);
        entity.insert(HideIfMenuIsOpen);
        let parent_entity = entity.id();
        assert!(!self.menus.iter().any(|row| row.tag == category as u8),
                "duplicate toolbar category (native assert)");
        self.menus.push(ContextualMenu { relative_to: node, parent_entity,
            node: Default::default(), tag: category as u8 });
        parent_entity
    }

    /// O 0x141e4d1e0 `ToolbarLayout::add_contextual_menu`.
    ///
    /// Looks up the definition row by `tag` (native unwrap on failure), creates
    /// the centered menu leaf, records the native anchor, consumes all items,
    /// and writes the menu node back into the definition row. Empty lists return
    /// before the native definition lookup and preserve the existing menu.
    pub fn add_contextual_menu(
        &mut self,
        taffy: &mut crate::resouces_and_components::taffy_layout::TaffyLayout,
        tag: u8,
        items: Vec<ToolbarItem>,
        commands: &mut bevy_ecs::system::Commands,
    ) -> Option<taffy::node::Node> {
        use crate::resouces_and_components::taffy_layout::{AnchorSide, AnchoredNode};
        if items.is_empty() { return None; }
        let row = self
            .menus
            .iter_mut()
            .find(|row| row.tag == tag)
            .expect("contextual menu definition tag missing (native unwrap_failed)");
        let menu = taffy
            .taffy
            .new_leaf(item_style(None))
            .expect("contextual menu new_leaf");
        taffy.anchored.push(AnchoredNode {
            order: 1,
            offset: glam::Vec2::ZERO,
            node: menu,
            relative_to: row.relative_to,
            // `MOV dword [entry+0x20],0x41200000` = 10.0.
            gap: 10.0,
            // `MOV byte [entry+0x24],0` = Above.
            side: AnchorSide::Above,
        });
        for item in items {
            add_toolbar_item(&mut taffy.taffy, menu, item, commands, row.parent_entity);
        }
        row.node = menu;
        Some(menu)
    }

    /// O `contextual_menus` inline (5 instances): the registered menu slice
    /// consumed by alignment and the size/padding update.
    pub fn contextual_menus(&self) -> &[ContextualMenu] {
        &self.menus
    }

    /// O 0x141e4d880 `link_up_nav`: for each button write
    /// `UiNav{prev=ids[(i+n-1)%n], next=ids[(i+1)%n]}` into the real entity.
    /// The boolean selects the horizontal/vertical slot layout.
    pub fn link_up_cyclic_list_nav(
        &self,
        ids: &[(bevy_ecs::entity::Entity, u64)],
        horizontal: bool,
        commands: &mut bevy_ecs::system::Commands,
    ) {
        if ids.is_empty() {
            return;
        }
        for (i, (entity, _)) in ids.iter().enumerate() {
            let prev = ids[(i + ids.len() - 1) % ids.len()].1;
            let next = ids[(i + 1) % ids.len()].1;
            let nav = if horizontal {
                crate::resouces_and_components::UiNav::horizontal(prev, next)
            } else {
                crate::resouces_and_components::UiNav::vertical(prev, next)
            };
            commands.entity(*entity).insert(nav);
        }
    }
}

impl PhotomodeToolbarLayout {
    /// Same inline `contextual_menus` (native reads both layouts).
    pub fn contextual_menus(&self) -> &[ContextualMenu] {
        &self.menus
    }
}


#[cfg(test)]
#[path = "toolbar_layout_tests.rs"]
mod layout_owner_tests;
