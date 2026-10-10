//! 中文工具栏和错误翻译；按对象展开参数，避免全部控件遮挡工作区。
use crate::{
    camera::{BlocksWorldInput, CameraAction},
    editor::{EditorAction, EditorStatus},
};
use bevy::prelude::*;
use garden_application::{
    EditError,
    tools::{Tool, ToolError},
};

/// 按编辑对象展开参数；所有控件继续调用已有命令入口。
#[derive(Component, Default, Clone, Copy, PartialEq, Eq)]
pub enum ToolPanel {
    #[default]
    House,
    Linear,
    Context,
}
#[derive(Resource, Default)]
struct ToolbarState(ToolPanel);
#[derive(Component)]
struct PanelNode(ToolPanel);
pub struct ToolbarPlugin;
impl Plugin for ToolbarPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<ToolbarState>()
            .add_systems(Update, panels);
    }
}
type PanelButtons<'a> = (
    &'a Interaction,
    Option<&'a ToolPanel>,
    Option<&'a EditorAction>,
    Option<&'a crate::context_tools::ContextAction>,
);
fn panels(
    buttons: Query<PanelButtons<'_>, Changed<Interaction>>,
    mut state: ResMut<ToolbarState>,
    mut nodes: Query<(&PanelNode, &mut Node)>,
) {
    let selected = &mut state.0;
    for (interaction, tab, legacy, context) in &buttons {
        if *interaction != Interaction::Pressed {
            continue;
        }
        if let Some(tab) = tab {
            *selected = *tab;
        }
        if let Some(action) = legacy {
            *selected = match action {
                EditorAction::Wall | EditorAction::Path | EditorAction::StrokeSelect => {
                    ToolPanel::Linear
                }
                EditorAction::Build
                | EditorAction::Move
                | EditorAction::Resize
                | EditorAction::Height
                | EditorAction::Rotate
                | EditorAction::Select => ToolPanel::House,
                _ => *selected,
            };
        }
        if let Some(action) = context {
            *selected = match action {
                crate::context_tools::ContextAction::AutoDraw
                | crate::context_tools::ContextAction::AutoType
                | crate::context_tools::ContextAction::LockPath
                | crate::context_tools::ContextAction::LockFence
                | crate::context_tools::ContextAction::LockWall
                | crate::context_tools::ContextAction::Style => ToolPanel::Linear,
                _ => ToolPanel::Context,
            };
        }
    }
    for (panel, mut node) in &mut nodes {
        let wanted = if panel.0 == *selected {
            Display::Flex
        } else {
            Display::None
        };
        if node.display != wanted {
            node.display = wanted;
        }
    }
}
fn panel(
    parent: &mut ChildSpawnerCommands,
    kind: ToolPanel,
    spawn: impl FnOnce(&mut ChildSpawnerCommands),
) {
    parent
        .spawn((
            PanelNode(kind),
            Node {
                display: if kind == ToolPanel::House {
                    Display::Flex
                } else {
                    Display::None
                },
                flex_direction: FlexDirection::Column,
                row_gap: px(6),
                ..default()
            },
        ))
        .with_children(spawn);
}

pub const FONT_PATH: &str = "fonts/NotoSansSC-Regular.otf";
pub const TITLE: &str = "暖石庭院 · 自由建造";
pub const HELP: &str = "画墙 / 画路：左键拖动；路穿墙自动开拱；墙路编辑可移动\n房屋：橙点调宽、蓝点调深、绿点调高；右键转视角；滚轮缩放";

#[derive(Resource, Default)]
pub struct UiFont(pub Option<Handle<Font>>);

pub fn tool_name(tool: Tool) -> &'static str {
    match tool {
        Tool::Select => "选择",
        Tool::Build => "建造",
        Tool::Move => "移动",
        Tool::MoveBlock => "移动体块",
        Tool::Resize => "尺寸",
        Tool::Height => "高度",
        Tool::Rotate => "旋转房屋",
        Tool::Pan => "平移",
        Tool::Wall => "画墙",
        Tool::Path => "画路",
        Tool::StrokeSelect => "墙路编辑",
    }
}

pub fn edit_error(error: &EditError) -> &'static str {
    match error {
        EditError::Domain(error) => match error.0 {
            "invalid footprint" => "房屋宽度和进深须在 1.2 至 100 米之间。",
            "invalid story height/count" => {
                "层数须为 1 至 4 层，每层至少 2.2 米，单体高度不超过 16 米。"
            }
            "upper block outside support margin" => "上层超出了下层的支撑范围，请先扩大下层。",
            "support chain exceeds four stories" => "叠加后的总层数不能超过 4 层。",
            "siblings overlap" => "同一层的建筑体块不能相互重叠。",
            "building requires 1..=3 blocks" => "每栋房屋须包含 1 至 3 个建筑体块。",
            "unknown block" | "missing parent" | "missing support parent" => {
                "建筑体块或其支撑层已不存在。"
            }
            "duplicate block id" => "建筑体块编号重复。",
            "exactly one root required" => "房屋须有且仅有一个底层体块。",
            "support cycle" => "建筑层之间不能循环支撑。",
            "non-finite placement" => "房屋位置无效，请重新选择位置。",
            "invalid stroke" | "invalid stroke length" => {
                "轨迹需至少两个有效点，不能超出庭院；长度最多60米，宽0.25–3米，墙高0.8–5米。"
            }
            "too many strokes" => "此原型最多保留32条墙路轨迹。",
            "invalid terrain" | "invalid terrain brush" => "地形或笔刷参数无效，操作未提交。",
            "invalid opening" => "窗宽须为0.3–2米，窗高须为0.4–2.5米。",
            "too many openings" => "此庭院最多保留256个手工窗。",
            "invalid linear intent" => "笔画控制点或离地偏移无效，操作未提交。",
            _ => "房屋结构不符合建造规则，请调整后重试。",
        },
        EditError::Missing => "房屋已不存在，请重新选择。",
        EditError::IdAlreadyUsed | EditError::BlockIdAlreadyUsed | EditError::DuplicateId => {
            "建筑编号已被使用，请重新建造。"
        }
        EditError::EmptyHistory => "没有可撤销或重做的操作。",
        EditError::CounterExhausted => "建筑编号或版本数量已达到上限。",
        EditError::StalePreview => "房屋已发生变化，请重新拖动后提交。",
    }
}

pub fn tool_error(error: &ToolError) -> &'static str {
    match error {
        ToolError::InvalidDrag => "拖动范围无效，房屋宽度和进深至少为 1.2 米。",
        ToolError::OutsideGarden => "房屋超出了庭院范围，请移回草地内。",
        ToolError::NoSelection => "请先选择一栋房屋。",
        ToolError::RootBlock => "此操作只适用于上层；底层请使用「移动」或「删除房屋」。",
        ToolError::Edit(error) => edit_error(error),
    }
}

fn text_font(font: &Handle<Font>, size: f32) -> TextFont {
    TextFont {
        font: font.clone().into(),
        font_size: bevy::text::FontSize::Px(size),
        ..default()
    }
}

pub(crate) fn button_row<A: Component + Copy>(
    parent: &mut ChildSpawnerCommands,
    font: &Handle<Font>,
    buttons: &[(A, &str)],
) {
    parent
        .spawn(Node {
            column_gap: px(8),
            row_gap: px(8),
            flex_wrap: FlexWrap::Wrap,
            ..default()
        })
        .with_children(|row| {
            for &(action, label) in buttons {
                row.spawn((
                    Button,
                    action,
                    Node {
                        padding: UiRect::axes(px(8), px(6)),
                        min_height: px(36),
                        justify_content: JustifyContent::Center,
                        align_items: AlignItems::Center,
                        ..default()
                    },
                    BackgroundColor(Color::srgb_u8(108, 132, 99)),
                ))
                .with_children(|button| {
                    button.spawn((
                        Text::new(label),
                        text_font(font, 15.0),
                        TextColor(Color::WHITE),
                    ));
                });
            }
        });
}

pub fn spawn_toolbar(commands: &mut Commands, font: &Handle<Font>) {
    commands
        .spawn((
            BlocksWorldInput,
            bevy::ui::RelativeCursorPosition::default(),
            Node {
                position_type: PositionType::Absolute,
                top: px(20),
                left: px(24),
                right: px(24),
                max_width: px(760),
                padding: UiRect::all(px(12)),
                flex_direction: FlexDirection::Column,
                row_gap: px(8),
                ..default()
            },
            BackgroundColor(Color::srgba_u8(244, 238, 223, 235)),
        ))
        .with_children(|parent| {
            let ink = TextColor(Color::srgb_u8(75, 81, 72));
            parent.spawn((Text::new(TITLE), text_font(font, 24.0), ink));
            parent.spawn((Text::new(HELP), text_font(font, 17.0), ink));
            button_row(
                parent,
                font,
                &[
                    (ToolPanel::House, "房屋参数"),
                    (ToolPanel::Linear, "墙路参数"),
                    (ToolPanel::Context, "地形与窗"),
                ],
            );
            button_row(
                parent,
                font,
                &[
                    (EditorAction::Select, "选择"),
                    (EditorAction::Wall, "画墙"),
                    (EditorAction::Path, "画路"),
                    (EditorAction::StrokeSelect, "墙路编辑"),
                    (EditorAction::Build, "建造"),
                    (EditorAction::Move, "移动"),
                    (EditorAction::Resize, "尺寸"),
                    (EditorAction::Height, "高度"),
                    (EditorAction::Rotate, "旋转房屋"),
                    (EditorAction::Pan, "平移"),
                    (EditorAction::Cancel, "取消"),
                    (EditorAction::Retry, "重试"),
                    (EditorAction::Undo, "撤销"),
                    (EditorAction::Redo, "重做"),
                ],
            );
            panel(parent, ToolPanel::House, |section| {
                button_row(
                    section,
                    font,
                    &[
                        (EditorAction::Roof, "切换屋顶"),
                        (EditorAction::HeightUp, "增高"),
                        (EditorAction::HeightDown, "降低"),
                        (EditorAction::Facade, "切换立面"),
                        (EditorAction::NextBlock, "切换体块"),
                        (EditorAction::MoveBlock, "移动体块"),
                        (EditorAction::AddUpper, "添加上层"),
                        (EditorAction::RemoveUpper, "删除上层"),
                        (EditorAction::Delete, "删除房屋"),
                    ],
                );
            });
            button_row(
                parent,
                font,
                &[
                    (CameraAction::Left, "左转"),
                    (CameraAction::Right, "右转"),
                    (CameraAction::Up, "抬高视角"),
                    (CameraAction::Down, "降低视角"),
                    (CameraAction::Near, "放大"),
                    (CameraAction::Far, "缩小"),
                    (CameraAction::Home, "复位视角"),
                ],
            );
            panel(parent, ToolPanel::Linear, |section| {
                button_row(
                    section,
                    font,
                    &[
                        (crate::strokes::StrokeAction::HeightUp, "墙增高"),
                        (crate::strokes::StrokeAction::HeightDown, "墙降低"),
                        (crate::strokes::StrokeAction::Wider, "墙路加宽"),
                        (crate::strokes::StrokeAction::Narrower, "墙路收窄"),
                        (crate::strokes::StrokeAction::Delete, "删除墙路"),
                    ],
                );
                button_row(
                    section,
                    font,
                    &[
                        (crate::context_tools::ContextAction::AutoDraw, "自动画笔"),
                        (crate::context_tools::ContextAction::AutoType, "自动三态"),
                        (crate::context_tools::ContextAction::LockPath, "锁定道路"),
                        (crate::context_tools::ContextAction::LockFence, "锁定篱笆"),
                        (crate::context_tools::ContextAction::LockWall, "锁定围墙"),
                        (crate::context_tools::ContextAction::Style, "边界样式"),
                    ],
                );
            });
            panel(parent, ToolPanel::Context, |section| {
                button_row(
                    section,
                    font,
                    &[
                        (crate::context_tools::ContextAction::Raise, "抬高地形"),
                        (crate::context_tools::ContextAction::Lower, "降低地形"),
                        (crate::context_tools::ContextAction::Smooth, "平滑地形"),
                        (crate::context_tools::ContextAction::PlaceWindow, "放置窗"),
                        (crate::context_tools::ContextAction::EditWindow, "编辑窗"),
                        (crate::context_tools::ContextAction::WindowWider, "窗加宽"),
                        (
                            crate::context_tools::ContextAction::WindowNarrower,
                            "窗收窄",
                        ),
                        (crate::context_tools::ContextAction::DeleteWindow, "删除窗"),
                    ],
                );
            });
            parent.spawn((
                EditorStatus,
                Text::new("工具：选择 ｜ 已选房屋：无"),
                text_font(font, 16.0),
                ink,
                Node {
                    max_width: percent(100),
                    ..default()
                },
            ));
        });
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn toolbar_uses_one_explicit_font_for_every_chinese_label() {
        let mut app = App::new();
        app.init_resource::<Assets<Font>>()
            .init_resource::<UiFont>()
            .add_systems(
                Startup,
                |mut commands: Commands,
                 mut fonts: ResMut<Assets<Font>>,
                 mut ui: ResMut<UiFont>| {
                    let bytes = std::fs::read(concat!(
                        env!("CARGO_MANIFEST_DIR"),
                        "/../../assets/",
                        "fonts/NotoSansSC-Regular.otf"
                    ))
                    .unwrap();
                    assert_eq!(&bytes[..4], b"OTTO");
                    let font = fonts.add(Font::from_bytes(bytes));
                    spawn_toolbar(&mut commands, &font);
                    ui.0 = Some(font);
                },
            );
        app.update();
        let world = app.world_mut();
        let expected = world.resource::<UiFont>().0.as_ref().unwrap().id();
        let mut query = world.query::<(&Text, &TextFont)>();
        let labels: Vec<_> = query.iter(world).collect();
        assert_eq!(labels.len(), 55);
        for (text, font) in labels {
            assert!(
                text.0
                    .chars()
                    .any(|c| ('\u{4e00}'..='\u{9fff}').contains(&c))
            );
            assert!(matches!(&font.font, bevy::text::FontSource::Handle(h) if h.id() == expected));
        }
        assert_eq!(
            world
                .query_filtered::<Entity, With<Button>>()
                .iter(world)
                .count(),
            52
        );
    }
    #[test]
    fn tool_names_and_error_fallbacks_never_expose_debug_english() {
        for tool in [
            Tool::Select,
            Tool::Build,
            Tool::Move,
            Tool::MoveBlock,
            Tool::Resize,
            Tool::Height,
            Tool::Rotate,
            Tool::Pan,
        ] {
            assert!(!tool_name(tool).is_ascii());
        }
        assert_eq!(
            edit_error(&EditError::EmptyHistory),
            "没有可撤销或重做的操作。"
        );
        assert!(tool_error(&ToolError::OutsideGarden).contains("庭院"));
        assert!(
            !edit_error(&EditError::Domain(garden_domain::DomainError(
                "new internal diagnostic"
            )))
            .is_ascii()
        );
    }
}
