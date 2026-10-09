//! Chinese presentation only: domain errors and identifiers stay language-independent.
use crate::{
    camera::{BlocksWorldInput, CameraAction},
    editor::{EditorAction, EditorStatus},
};
use bevy::prelude::*;
use garden_application::{
    EditError,
    tools::{Tool, ToolError},
};

pub const FONT_PATH: &str = "fonts/NotoSansSC-Regular.otf";
pub const TITLE: &str = "暖石庭院 · 自由建造";
pub const HELP: &str = "点击选择体块；橙点调宽、蓝点调深、绿点调高、紫点转房屋\n左键拖动工具；右键单击取消 / 拖动转视角；滚轮缩放";

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

fn button_row<A: Component + Copy>(
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
                        padding: UiRect::axes(px(12), px(10)),
                        min_height: px(44),
                        justify_content: JustifyContent::Center,
                        align_items: AlignItems::Center,
                        ..default()
                    },
                    BackgroundColor(Color::srgb_u8(108, 132, 99)),
                ))
                .with_children(|button| {
                    button.spawn((
                        Text::new(label),
                        text_font(font, 17.0),
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
                    (EditorAction::Select, "选择"),
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
            button_row(
                parent,
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
        assert_eq!(labels.len(), 30);
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
            27
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
