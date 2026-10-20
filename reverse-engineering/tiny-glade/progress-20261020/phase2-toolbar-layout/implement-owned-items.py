from pathlib import Path
p=Path('D:/game/tiny-glade/crates/systems/toolbar/src/resouces_and_components/toolbar_layout.rs');s=p.read_text(encoding='utf-8');start=s.index('/// 一条 contextual menu');s='''//! Owned toolbar item trees and native Taffy/Bevy construction.
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
            entity.insert((UiNav::default(), ToolbarExpand));
        }
        ToolbarItemKind::AppMode(mode) => {
            entity.insert((SwitchAppModeButton { mode }, UiNav::default(), ToolbarExpand));
        }
        ToolbarItemKind::Slider(kind) => {
            let mut slider = UiSlider::default();
            slider.set_previous_value_01(initial_slider_value_01(kind));
            slider.set_kind(kind);
            let mut nav = UiNav::default();
            nav.links[0].tag = 2;
            nav.links[1].tag = 2;
            entity.insert((slider, nav, ToolbarExpand));
        }
        ToolbarItemKind::SliderReset(flag) => {
            entity.insert((UiSliderReset { flag }, UiNav::default(), ToolbarExpand));
        }
        ToolbarItemKind::Spacer => {
            // Original 8B payload is u32 tag=1 followed by f32 value=1.
            entity.insert(UiCustomSize { kind: 1, value: 1.0 });
        }
        ToolbarItemKind::Locked => { entity.insert((UiLocked, ToolbarExpand)); }
        ToolbarItemKind::Row(_) | ToolbarItemKind::Column(_) => unreachable!(),
    }
    Some(entity.id())
}

'''+s[start:];s=s.replace('#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]\npub struct ContextualMenu', '#[derive(Clone, Copy, Debug, PartialEq, Eq)]\npub struct ContextualMenu').replace('    /// 定义表 +0x10：菜单根节点', '    /// Definition row +0x08: focus parent Entity, passed to every descendant.\n    pub parent_entity: Entity,\n    /// 定义表 +0x10：菜单根节点');needle='/// 工具栏布局资源';idx=s.index(needle);s=s[:idx]+'''impl Default for ContextualMenu {
    fn default() -> Self {
        Self { relative_to: Default::default(), parent_entity: Entity::PLACEHOLDER,
               node: Default::default(), tag: 0 }
    }
}

'''+s[idx:];s=s.replace('items: &mut [ToolbarItemKind]', 'items: Vec<ToolbarItem>').replace(') -> taffy::node::Node {\n        use crate::resouces_and_components::taffy_layout::{AnchorSide, AnchoredNode};\n        let row', ') -> Option<taffy::node::Node> {\n        use crate::resouces_and_components::taffy_layout::{AnchorSide, AnchoredNode};\n        if items.is_empty() { return None; }\n        let row');s=s.replace('''        for item in items.iter_mut() {
            if item.show_only_fpv == 2 {
                break;
            }
            add_toolbar_item(&mut taffy.taffy, menu, item, commands);
        }
        row.node = menu;
        menu''','''        for item in items {
            add_toolbar_item(&mut taffy.taffy, menu, item, commands, row.parent_entity);
        }
        row.node = menu;
        Some(menu)''');s=s[:s.index('#[cfg(test)]')];p.write_text(s,encoding='utf-8')
