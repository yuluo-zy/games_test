//! 审查反例：断言当前缺陷可观察；通过不表示修复完成。
pub use country_core::{asset_libraries, render, rhapsody_contract};

// 复用项目现有 CPU 测试设备，仅替代 shader 编译边界。
#[path = "../src/render/test_device.rs"]
mod existing_test_device;

use bevy_ecs::{prelude::*, system::RunSystemOnce};
use country_core::{
    asset_libraries::{mesh_library::AssetMeshLibrary, shader_library::AssetShaderLibrary},
    render::{mesh::Mesh, shader::RasterShaderProgram},
    systems::ui::{resources::{slider3d::{Slider3dMatrices, Slider3dState}, ui_state::{UiInteraction, UiState, WidgetId}},
        signifiers::{signifier_create::{signifier_create, SignifierId, UiSignifierStream},
            signifier_mark_for_delete::{despawn_marked, signfier_mark_for_delete}}},
    utils::debug_viz::DebugLine,
};
use glam::{Vec2, Vec3};

fn create_debug_line(dashed: bool) -> (World, Entity) {
    let mut world = World::new();
    world.insert_resource(AssetMeshLibrary::new());
    let shaders = AssetShaderLibrary::new();
    let shader_name = if dashed { "dashed_line" } else { "vertex_color_shader" };
    shaders.add_raster(RasterShaderProgram::new_draw(shader_name, Default::default(), &existing_test_device::TestDevice(1)), false);
    world.insert_resource(shaders);
    world.insert_resource(UiState::new(UiInteraction { native_tag: 1, native_phase: 0, widget: WidgetId::from_name("rereview") }));
    let mut stream = UiSignifierStream::default();
    let mut line = DebugLine::new(vec![Vec3::ZERO, Vec3::X]);
    line.dashed = dashed;
    line.try_show(&mut stream).unwrap();
    world.insert_resource(stream);
    world.run_system_once(signifier_create).unwrap();
    let entity = world.query_filtered::<Entity, With<SignifierId>>().single(&world).unwrap();
    (world, entity)
}

#[test]
fn actual_dashed_draw_callback_cannot_read_the_spawned_component() {
    let (world, entity) = create_debug_line(true);
    assert_eq!(world.get::<country_core::components::DashedLineLength>(entity).unwrap().0, 4.0);
    let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        country_core::systems::render_loop::draw_passes::draw_callbacks_a::read_env57(&world, entity)
    }));
    assert!(result.is_err(), "真实绘制回调请求了另一个 ECS 组件身份");
}

#[test]
fn visible_cursor_can_extrapolate_a_slider_point_behind_the_camera() {
    let mut matrices = Slider3dMatrices::default();
    matrices.second = glam::Mat4::perspective_rh_gl(std::f32::consts::FRAC_PI_2, 1.0, 0.1, 100.0);
    let slider = Slider3dState { origin: Vec3::new(0.0, 0.0, -1.0), direction: Vec3::new(0.2, 0.0, -1.0) };
    // 屏幕内部点；两个采样端点的 w 分别为 1、2，均在前方。
    let position = slider.project_cursor_onto_line(Vec2::new(62.5, 50.0), [100,100], &matrices);
    let clip = matrices.second * position.extend(1.0);
    assert!(position.is_finite());
    assert!(clip.w < 0.0, "当前返回了相机背后的编辑位置：{position:?}");
}

#[test]
fn current_signifier_cleanup_immediately_despawns_an_active_debug_line() {
    let (mut world, entity) = create_debug_line(false);
    let mesh = *world.get::<asset_libraries::Handle<Mesh>>(entity).unwrap();
    assert_eq!(world.resource::<UiState>().interaction().native_tag, 1);
    world.run_system_once(signfier_mark_for_delete).unwrap();
    world.run_system_once(despawn_marked).unwrap();
    assert!(world.get_entity(entity).is_err());
    assert_eq!(*world.resource::<AssetMeshLibrary>().transient_meshes().lock().unwrap(), vec![mesh]);
}
