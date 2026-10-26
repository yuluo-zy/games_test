//! B 14234a410：最后一个抬升受限事件的空间标尺；菜单打开时不消耗事件。
use super::demo_show_full_glade_size::project;
use crate::mei::{core::MeiContext, widgets::LabelContent};
use bevy_ecs::prelude::*;
use country_core::{
    resources::{
        glade_border::MaxDemoElevationReached, gui::EscMenuState,
        terrain_height::TerrainHeightsData, window::WindowSize,
    },
    systems::{
        backend::input::translate_input::DemoModeEnabled, camera_rig::MainCamera,
        ui::signifiers::UiSignifierStream,
    },
    utils::debug_viz::DebugLine,
};
use glam::{Vec2, Vec3};

/// B ui_raise_shape::MAX_ELEVATION：原只读 f32 142ab44c8 = 60，导入指针14362c030。
/// 不把演示文字门禁错误地当成消费者门禁：原满版亦能显示 Max。
const MAX_ELEVATION: f32 = 60.0;

fn rightmost(points: &[Vec2], mut projected_x: impl FnMut(Vec2) -> f32) -> Vec2 {
    let mut selected = points[0];
    let mut max_x = 0.0;
    for point in points {
        let x = projected_x(*point);
        if x > max_x {
            max_x = x;
            selected = *point;
        }
    }
    selected
}

/// 原箭头有一条竖标尺和两条 0.7 倍半宽的斜向短线。
fn ruler(point: Vec2, terrain: f32, right: Vec3) -> [Vec<Vec3>; 3] {
    let right = Vec3::new(right.x, 0.0, right.z);
    let inv = 1.0 / (right.z * right.z + (right.y * right.y + right.x * right.x)).sqrt();
    let half = right * inv * 0.5;
    let base = Vec3::new(point.x, terrain, point.y) + half;
    let top = Vec3::new(base.x, MAX_ELEVATION, base.z);
    let wing = half * 0.7;
    [
        vec![base, top],
        vec![top, top + wing + Vec3::new(0.0, -0.7, 0.0)],
        vec![top, top - wing + Vec3::new(0.0, -0.7, 0.0)],
    ]
}

#[allow(clippy::too_many_arguments)]
pub fn demo_show_max_elevation(
    mut events: EventReader<MaxDemoElevationReached>,
    heights: Res<TerrainHeightsData>,
    window: Res<WindowSize>,
    camera: Res<MainCamera>,
    mut stream: ResMut<UiSignifierStream>,
    mut mei: ResMut<MeiContext>,
    menu: Res<EscMenuState>,
    demo: Res<DemoModeEnabled>,
) {
    if menu.is_open() {
        return;
    }
    let Some(event) = events.read().last() else {
        return;
    };
    let curve = event.0.as_curve2();
    let size = window.as_vec2();
    let selected = rightmost(&curve.points, |p| {
        project(Vec3::new(p.x, 0.0, p.y), size, &camera).x
    });
    let lines = ruler(
        selected,
        heights.sample_at_world_xz(selected.x, selected.y),
        camera.camera_matrices().eye_right(),
    );
    let top = lines[0][1];
    for line in lines {
        DebugLine::new(line).line_width(4.0).show(&mut stream);
    }
    let position = project(top, size, &camera) / (size.y / 1000.0) + Vec2::new(0.0, 20.0);
    let node = mei.new_at_pos(
        "display max elevation demo text",
        position,
        Some(Vec2::new(220.0, 30.0)),
        Vec2::splat(-1.0),
        false,
    );
    node.template("other/hint_text").with_slot("text", |slot| {
        let label = slot.label(if demo.0 { "Max in DEMO" } else { "Max" });
        let content = label.content_mut::<LabelContent>();
        content.text.set_metrics(30.0).expect("原高度提示字号");
        content.text.set_weight(tg_text::Weight(800));
    });
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn ruler_keeps_terrain_start_and_absolute_limit() {
        let lines = ruler(Vec2::new(3., 4.), 7., Vec3::X);
        assert_eq!(lines[0], [Vec3::new(3.5, 7., 4.), Vec3::new(3.5, 60., 4.)]);
        assert_eq!(lines[1][0], lines[0][1]);
        assert_eq!(lines[2][0], lines[0][1]);
        assert_eq!(lines[1][1].y, 59.3);
        assert_eq!(
            rightmost(&[Vec2::new(-1., 2.), Vec2::new(-3., 4.)], |p| p.x),
            Vec2::new(-1., 2.)
        );
    }
    #[test]
    #[ignore = "需TG_INSTALL_ROOT原模板/字体，不发送OS输入"]
    fn real_shape_producer_to_last_event_ruler_and_menu_consumption() {
        use super::super::demo_show_full_glade_size::tests::{pending_text, real_mei};
        use country_core::{
            resources::{
                events::ConstructTimberframeCmd,
                glade_border::{
                    BorderShape, DisplayOutOfBorder, FeedbackInputOutOfBorder, GladeBorder,
                },
                mouse_capture::OsCursorCaptureCmd,
                mouse_terrain_raycast::{CameraCursorRay, CursorPositionSS},
                terrain_height::TerrainHeightSsbo,
                time::GameTime,
                tree_storage::HideTreesinAreaCmd,
                window::AudioCursorMoved,
            },
            systems::wall::shapes::{
                self, PreviewCircle, ShapePlacement, circle::ui_create_circle::ui_create_circle,
            },
        };
        let mut world = World::new();
        shapes::register_shape_resources(&mut world);
        world.insert_resource(country_core::systems::mode_manager::AppMode::Wall(
            country_core::systems::mode_manager::WallShape::Circle,
        ));
        world.init_resource::<country_core::input::action_set::ActionSetStates>();
        world.insert_resource(
            country_core::systems::ui::resources::ui_state::UiState::new(
                country_core::systems::ui::resources::ui_state::UiInteraction {
                    native_tag: 0,
                    native_phase: 2,
                    widget: country_core::systems::ui::resources::ui_state::WidgetId(0),
                },
            ),
        );
        world.insert_resource(CursorPositionSS(Vec2::ZERO));
        world.insert_resource(CameraCursorRay {
            origin: Vec3::new(0., 10., 0.),
            dir: -Vec3::Y,
        });
        world.init_resource::<GameTime>();
        world.init_resource::<Events<AudioCursorMoved>>();
        world.init_resource::<Events<ConstructTimberframeCmd>>();
        world.init_resource::<Events<HideTreesinAreaCmd>>();
        world.init_resource::<Events<country_core::systems::audio::PlayTimberframeSound>>();
        world.init_resource::<Events<OsCursorCaptureCmd>>();
        world.init_resource::<Events<shapes::DontRecalculateShapePlacement>>();
        world.init_resource::<Events<DisplayOutOfBorder>>();
        world.init_resource::<Events<FeedbackInputOutOfBorder>>();
        world.insert_resource(GladeBorder(BorderShape::Circle(
            utils::geometry::Circle2d::new(Vec2::ZERO, 15.0),
        )));
        fn set_height(world: &mut World, height: f32) {
            let mut observed = *world.resource::<ShapePlacement>().observed();
            observed[0x3c..0x40].copy_from_slice(&height.to_le_bytes());
            world.insert_resource(ShapePlacement::from_observed(observed));
        }
        set_height(&mut world, 60.0);
        world.insert_resource(PreviewCircle {
            center: Vec2::new(3., 4.),
            radius: 2.,
            rotation: 0.25,
            ..Default::default()
        });
        // 原Max在本帧画布分支更新预览之前读取权威ShapePlacement高度。
        world.insert_resource(TerrainHeightsData::<TerrainHeightSsbo>::new());
        world.insert_resource(MainCamera::new(1.0));
        world.insert_resource(WindowSize::new(1000, 700));
        world.insert_resource(UiSignifierStream::default());
        world.insert_resource(real_mei());
        world.insert_resource(DemoModeEnabled(true));
        world.insert_resource(EscMenuState::Main);
        let mut produce = Schedule::default();
        produce.add_systems(ui_create_circle);
        let mut consume = Schedule::default();
        consume.add_systems(demo_show_max_elevation);
        produce.run(&mut world);
        let events = world.resource::<Events<MaxDemoElevationReached>>();
        assert_eq!(events.len(), 1);
        let event = events.iter_current_update_events().last().unwrap();
        let circle = event.0.as_circle();
        assert_eq!(
            (circle.center, circle.radius, circle.rotation),
            (Vec2::new(3., 4.), 2., 0.25)
        );
        // 画布本身已提交真实预览工厂，先取走再单独检查菜单中的Max消费者。
        let preview = world.resource_mut::<UiSignifierStream>().take_entries();
        assert_eq!(preview.len(), 1);
        consume.run(&mut world);
        assert_eq!(world.resource::<UiSignifierStream>().committed_len(), 0);
        world.insert_resource(EscMenuState::Closed);
        consume.run(&mut world);
        assert_eq!(world.resource::<UiSignifierStream>().committed_len(), 3);
        assert!(
            pending_text(world.resource::<MeiContext>())
                .iter()
                .any(|text| text == "Max in DEMO")
        );
        world.resource_mut::<UiSignifierStream>().take_entries();
        consume.run(&mut world);
        assert_eq!(world.resource::<UiSignifierStream>().committed_len(), 0);
        set_height(&mut world, f32::from_bits(60.0f32.to_bits() - 1));
        produce.run(&mut world);
        assert_eq!(world.resource::<Events<MaxDemoElevationReached>>().len(), 1);
        set_height(&mut world, f32::NAN);
        produce.run(&mut world);
        assert_eq!(world.resource::<Events<MaxDemoElevationReached>>().len(), 1);
        set_height(&mut world, f32::INFINITY);
        produce.run(&mut world);
        assert_eq!(world.resource::<Events<MaxDemoElevationReached>>().len(), 2);
        world.insert_resource(country_core::systems::mode_manager::AppMode::Wall(
            country_core::systems::mode_manager::WallShape::Rectangle,
        ));
        set_height(&mut world, 60.0);
        *world.resource_mut::<shapes::PreviewRectangle>() = shapes::PreviewRectangle {
            center: Vec2::new(-4., 7.),
            dimensions: Vec2::new(3., 8.),
            orientation: Vec2::new(0., 1.),
            ..Default::default()
        };
        let mut rectangle = Schedule::default();
        rectangle.add_systems(shapes::rectangle::ui_create_rectangle::ui_create_rectangle);
        rectangle.run(&mut world);
        let events = world.resource::<Events<MaxDemoElevationReached>>();
        assert_eq!(events.len(), 3);
        let rect = events
            .iter_current_update_events()
            .last()
            .unwrap()
            .0
            .as_rectangle();
        assert_eq!(rect.center, Vec2::new(-4., 7.));
        assert_eq!(rect.size, Vec2::new(3., 8.));
        assert_eq!(rect.axis, Vec2::new(0., 1.));
    }
}
