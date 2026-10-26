//! B 0x141eac540：演示范围、完整游戏范围边框和投影提示。
//! 原 TextAnchors 是 Local<Option<Vec3>>；当前宿主用资源保有相同状态，
//! 以便同 App 的 Reload 精确清除旧会话。近景清空，菜单/摄影隐藏时保留。
use crate::mei::{core::MeiContext, widgets::LabelContent};
use bevy_ecs::prelude::*;
use country_core::{
    resources::{
        glade_border::GladeBorder, gui::EscMenuState, mouse_terrain_raycast::CameraCursorRay,
        terrain_height::TerrainHeightsData, time::GameTime, window::WindowSize,
    },
    systems::{
        backend::input::translate_input::DemoModeEnabled,
        camera_rig::MainCamera,
        mode_manager::AppMode,
        ui::signifiers::{
            SignifierEntityBuilder, SignifierFactory, UiSignifierStream,
            signifier_factory::SignifierMesh,
        },
    },
};
use glam::{Vec2, Vec3};
use std::sync::Arc;
use utils::geometry::Rectangle2d;

#[derive(Resource, Default)]
pub struct TextAnchors(pub Option<Vec3>);

/// 复用已恢复的B原174B权威投影；窗口/相机包装不另造第二套数学。
pub(super) fn project(point: Vec3, window: Vec2, camera: &MainCamera) -> Vec2 {
    country_core::utils::world_space_to_screen_space(
        point,
        window.x,
        window.y,
        &camera.read_matrix(0x80),
        &camera.read_matrix(0),
    )
}

/// 原循环只保留窗内、距离屏幕顶部中央最近的采样点，平距离取先出现者。
fn nearest_visible(
    points: impl Iterator<Item = Vec3>,
    window: Vec2,
    mut project_point: impl FnMut(Vec3) -> Vec2,
) -> Option<Vec3> {
    let mut result = None;
    let mut nearest = 10000.0;
    for point in points {
        let screen = project_point(point);
        if !(0.0 <= screen.x && screen.x <= window.x && 0.0 <= screen.y && screen.y <= window.y) {
            continue;
        }
        // 原 y 偏置为 -0.0，不能改为窗口中点。
        let dx = screen.x + window.x * -0.5;
        let dy = screen.y + window.y * -0.0;
        let distance = (dy * dy + dx * dx).sqrt();
        if distance < nearest {
            nearest = distance;
            result = Some(point);
        }
    }
    result
}

fn interpolate(previous: Vec3, target: Vec3, delta: f32) -> Vec3 {
    let t = 1.0 - (-delta * 5.0).exp();
    previous + (target - previous) * t
}

fn border_factory(frame: u32) -> SignifierFactory {
    let rectangle = Rectangle2d::new_centered_axis_aligned(Vec2::splat(130.0));
    let points = rectangle.as_curve3(0.0).points;
    let dashed_length = rectangle.perimeter() * 0.3;
    // 原名字随 frame_count 变化，避免复用旧边框的实体身份。
    SignifierFactory::from_closure(
        &format!("full_glade_size_{frame}"),
        Arc::new(move |meshes, shaders, commands| {
            let entity = SignifierEntityBuilder::new(SignifierMesh::Points(points.clone()))
                .animation_duration(0.0)
                .custom_shader("dashed_line")
                .build(meshes, shaders, commands);
            commands.entity(entity).insert((
                country_core::render::pass_builder::draw_list::DrawTopology::Alt(5.0),
                country_core::components::DashedLineLength(dashed_length),
            ));
            vec![entity]
        }),
    )
}

fn text(
    mei: &mut MeiContext,
    name: &str,
    world: Vec3,
    text: &str,
    offset_y: f32,
    camera: &MainCamera,
    size: Vec2,
) {
    let position = project(world, size, camera) / (size.y / 1000.0) + Vec2::new(0.0, offset_y);
    let node = mei.new_at_pos(
        name,
        position,
        Some(Vec2::new(200.0, 30.0)),
        Vec2::splat(-1.0),
        false,
    );
    node.template("other/hint_text").with_slot("text", |slot| {
        let label = slot.label(text);
        let content = label.content_mut::<LabelContent>();
        content.text.set_metrics(30.0).expect("原演示提示字号");
        content.text.set_weight(tg_text::Weight(800));
    });
}

#[allow(clippy::too_many_arguments)]
pub fn demo_show_full_glade_size(
    mode: Res<AppMode>,
    camera: Res<MainCamera>,
    window: Res<WindowSize>,
    border: Res<GladeBorder>,
    mut stream: ResMut<UiSignifierStream>,
    time: Res<GameTime>,
    mut mei: ResMut<MeiContext>,
    heights: Res<TerrainHeightsData>,
    mut anchors: ResMut<TextAnchors>,
    menu: Res<EscMenuState>,
    demo: Res<DemoModeEnabled>,
) {
    if !demo.0 || *mode == AppMode::Photomode || menu.is_open() {
        return;
    }
    let eye = camera.eye_position();
    let mut direction = camera.eye_forward();
    direction.y += -0.2;
    let intersection = CameraCursorRay {
        origin: eye,
        dir: direction,
    }
    .intersect_with_xz_plane(0.0);
    let delta = eye - intersection;
    let distance = (delta.z * delta.z + (delta.y * delta.y + delta.x * delta.x)).sqrt();
    if distance <= 70.0 {
        anchors.0 = None;
    }
    if !(distance > 70.0) {
        return;
    }
    stream.write_factory(&border_factory(time.frame_count));
    let curve = match border.0 {
        country_core::resources::glade_border::BorderShape::Circle(circle) => circle.as_curve2(1.0),
        _ => panic!("原演示范围必须是圆形"),
    };
    let size = window.as_vec2();
    let candidate = nearest_visible(
        curve
            .points
            .iter()
            .map(|p| Vec3::new(p.x, heights.sample_at_world_xz(p.x, p.y), p.y)),
        size,
        |p| project(p, size, &camera),
    );
    if let Some(target) = candidate {
        let previous = anchors.0.unwrap_or(target);
        let anchor = interpolate(previous, target, time.delta);
        anchors.0 = Some(anchor);
        text(
            &mut mei,
            "display demo text",
            anchor,
            "DEMO",
            -30.0,
            &camera,
            size,
        );
    }
    if let Some(anchor) = anchors.0 {
        let scale = 1.0 / (anchor.z * anchor.z + anchor.x * anchor.x).sqrt();
        let full = Vec3::new(
            scale * anchor.x * 137.88582 * 0.5,
            0.0,
            scale * anchor.z * 137.88582 * 0.5,
        );
        text(
            &mut mei,
            "display full game text",
            full,
            "FULL GAME",
            -50.0,
            &camera,
            size,
        );
    }
}

#[cfg(test)]
pub(super) mod tests {
    use super::*;
    use glam::Mat4;
    #[test]
    fn original_projection_instruction_fixture() {
        type Row = ([u32; 16], [u32; 16], [u32; 3], [u32; 2], [u32; 2]);
        let rows: Vec<Row> = ron::from_str(include_str!(
            "../../../../../tests/fixtures/native/toolbar-demo-projection.ron"
        ))
        .unwrap();
        assert_eq!(rows.len(), 36);
        for (projection, view, point, size, expected) in rows {
            let mut camera = MainCamera::new(1.0);
            camera.write_matrix(0, &Mat4::from_cols_array(&projection.map(f32::from_bits)));
            camera.write_matrix(0x80, &Mat4::from_cols_array(&view.map(f32::from_bits)));
            let actual = project(
                Vec3::from_array(point.map(f32::from_bits)),
                Vec2::from_array(size.map(f32::from_bits)),
                &camera,
            );
            for (actual, bits) in actual.to_array().into_iter().zip(expected) {
                if f32::from_bits(bits).is_nan() {
                    assert!(actual.is_nan());
                } else {
                    assert_eq!(actual.to_bits(), bits);
                }
            }
        }
    }
    #[test]
    fn closest_top_center_preserves_window_edges_ties_and_no_candidate() {
        let points = [
            Vec3::new(20., 0., 0.),
            Vec3::new(50., 0., 0.),
            Vec3::new(50., 0., 1.),
        ];
        assert_eq!(
            nearest_visible(points.into_iter(), Vec2::new(100., 100.), |p| p.truncate()),
            Some(points[1])
        );
        assert_eq!(
            nearest_visible(
                [Vec3::new(-1., 0., 0.)].into_iter(),
                Vec2::splat(100.),
                |p| p.truncate()
            ),
            None
        );
        assert_eq!(
            nearest_visible(
                [Vec3::new(50., 100., 0.)].into_iter(),
                Vec2::splat(100.),
                |p| p.truncate()
            ),
            Some(Vec3::new(50., 100., 0.))
        );
        assert_eq!(
            nearest_visible(
                [Vec3::new(0., 10000., 0.)].into_iter(),
                Vec2::splat(10000.),
                |p| p.truncate()
            ),
            None
        );
    }
    pub(crate) fn real_mei() -> MeiContext {
        let install = std::path::PathBuf::from(std::env::var_os("TG_INSTALL_ROOT").unwrap());
        let mut mei = MeiContext::default();
        mei.load_font_table(&install.join("assets/fonts/table.ron"))
            .unwrap();
        mei.configs.insert(
            "other/hint_text".into(),
            Arc::new(
                crate::mei::config::read_node_config_from_file(
                    &install.join("assets/ui/other/hint_text.ron"),
                )
                .unwrap(),
            ),
        );
        mei
    }
    pub(crate) fn pending_text(mei: &MeiContext) -> Vec<String> {
        mei.nodes
            .values()
            .filter_map(|node| {
                let any: &dyn std::any::Any = node.content.as_ref();
                any.downcast_ref::<LabelContent>()
                    .and_then(|c| c.pending_text.clone())
            })
            .collect()
    }
    #[test]
    #[ignore = "需TG_INSTALL_ROOT原模板/字体，不发送OS输入"]
    fn actual_system_emits_factories_and_both_labels_and_menu_gates() {
        use country_core::resources::glade_border::BorderShape;
        let mut world = World::new();
        let mut camera = MainCamera::new(1.0);
        camera.set_camera_matrices(Mat4::from_translation(Vec3::new(0., 100., 0.)));
        camera.write_matrix(0, &Mat4::from_scale(Vec3::splat(0.02)));
        camera.write_matrix(
            0x80,
            &Mat4::from_cols_array(&[
                1., 0., 0., 0., 0., 0., 1., 0., 0., 1., 0., 0., 0., 0., 0., 1.,
            ]),
        );
        world.insert_resource(camera);
        world.insert_resource(WindowSize::new(1000, 700));
        world.insert_resource(AppMode::default());
        world.insert_resource(TextAnchors::default());
        world.insert_resource(DemoModeEnabled(true));
        world.insert_resource(GladeBorder(BorderShape::Circle(
            utils::geometry::Circle2d::new(Vec2::ZERO, 15.),
        )));
        world.insert_resource(GameTime::default());
        world.insert_resource(TerrainHeightsData::<
            country_core::resources::terrain_height::TerrainHeightSsbo,
        >::new());
        world.insert_resource(UiSignifierStream::default());
        world.insert_resource(real_mei());
        world.insert_resource(EscMenuState::Main);
        let mut schedule = Schedule::default();
        schedule.add_systems(demo_show_full_glade_size);
        schedule.run(&mut world);
        assert_eq!(world.resource::<UiSignifierStream>().committed_len(), 0);
        world.insert_resource(EscMenuState::Closed);
        schedule.run(&mut world);
        let labels = pending_text(world.resource::<MeiContext>());
        assert!(labels.iter().any(|t| t == "DEMO"), "{labels:?}");
        assert!(labels.iter().any(|t| t == "FULL GAME"), "{labels:?}");
        let entries = world.resource_mut::<UiSignifierStream>().take_entries();
        assert_eq!(entries.len(), 1);
        let country_core::systems::ui::signifiers::UiSignifierEntry::Factory(factory) = &entries[0]
        else {
            panic!("边框必须是可消费工厂")
        };
        assert_eq!(factory.name(), Some("full_glade_size_0"));
        assert!(factory.has_closure());
        world.insert_resource(AppMode::Photomode);
        schedule.run(&mut world);
        assert_eq!(world.resource::<UiSignifierStream>().committed_len(), 0);
    }
}
