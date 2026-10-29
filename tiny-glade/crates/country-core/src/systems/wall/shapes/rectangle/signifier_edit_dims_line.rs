//! B1408dda10：尺寸编辑线的真实动态网格、活动原点及淡入淡出。
use super::ui_edit_rectangle_dims::UiEditRectangleActiveWidgetResource;
use crate::{
    asset_libraries::{Handle, mesh_library::AssetMeshLibrary},
    render::mesh::Mesh,
    systems::ui::signifiers::{
        SignifierAnimation, SignifierEntityBuilder, SignifierFactory, UiSignifierStream,
    },
    utils::{components::transform::Transform, geo::update_curve_mesh},
};
use bevy_ecs::prelude::*;
use std::sync::Arc;
use utils::curve::Curve2;

/// 原查询stride56B的Curve2拥有者，不是存进name字段的两个浮点数。
#[derive(Component, Clone, Debug)]
pub struct EditRectangleLinePreview(pub Curve2);

/// 原四参数：发布拥有型工厂，并更新包括淡出实体在内的所有真实编辑线。
pub fn signifier_edit_dims(
    active: Res<UiEditRectangleActiveWidgetResource>,
    mut stream: ResMut<UiSignifierStream>,
    mut query: Query<(
        &EditRectangleLinePreview,
        &Handle<Mesh>,
        &SignifierAnimation,
        &mut Transform,
    )>,
    mut meshes: ResMut<AssetMeshLibrary>,
) {
    if let Some((curve, widget, origin)) = &active.0 {
        let curve = curve.clone();
        let origin = *origin;
        stream.write_factory(&SignifierFactory::from_closure(
            &format!("rectangle_edit_dims_line_WidgetId {{ hash: {} }}", widget.0),
            Arc::new(move |m, s, c| {
                // B1408de7e0先建空瞬态网格；下一次原query才填曲线。
                let entity = SignifierEntityBuilder::new(
                    crate::systems::ui::signifiers::signifier_factory::SignifierMesh::Empty,
                )
                .position(origin)
                .animation_duration(0.3)
                .disable_default_animation()
                .build(m, s, c);
                c.entity(entity).insert((
                    EditRectangleLinePreview(curve.clone()),
                    crate::render::pass_builder::draw_list::DrawTopology::Alt(3.0),
                ));
                vec![entity]
            }),
        ));
    }
    for (line, mesh, animation, mut transform) in &mut query {
        if let Some((_, _, origin)) = &active.0 {
            transform.translation = *origin;
        }
        // B先把eased映到[0.01,0.5]，再取中心对称切片，淡出时同样缩短。
        let half = animation.eased * 0.5 + (1.0 - animation.eased) * 0.01;
        let points: Vec<_> = line
            .0
            .crashy_slice(0.5 - half, half + 0.5)
            .points
            .into_iter()
            .map(|point| glam::Vec3::new(point.x, 0.0, point.y))
            .collect();
        let target = meshes.get_mut(*mesh).expect("矩形编辑线网格已被过早释放");
        update_curve_mesh(Some(target), points);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{
        asset_libraries::{
            shader_library::{AssetShaderLibrary, SharedAssetShaderLibrary},
            transient_mesh::{TransientMesh, delete_dropped_transient_meshes},
            vao_library::AssetVAOLibrary,
        },
        render::{shader::RasterShaderProgram, test_device::TestDevice},
        systems::ui::{
            resources::{
                ui_state::{UiInteraction, UiState, WidgetId},
                widget_params::WidgetParams,
            },
            signifiers::{
                animation::signifier_animation_t,
                signifier_create::{SignifierId, signifier_create},
                signifier_mark_for_delete::{despawn_marked, signfier_mark_for_delete},
            },
        },
    };
    use bevy_ecs::system::RunSystemOnce;

    fn active_ui(widget: WidgetId) -> UiState {
        let interaction = UiInteraction {
            native_tag: 2,
            native_phase: 4,
            widget,
        };
        let mut ui = UiState::new(interaction);
        ui.add_with_id(widget, WidgetParams::default());
        let params = ui.widget(widget).unwrap();
        ui.clear_widgets();
        ui.replace_interaction_full(interaction, Some(params), [0; 56]);
        ui
    }
    #[test]
    fn actual_line_factory_keeps_widget_identity_updates_mesh_and_unloads() {
        let mut world = World::new();
        world.insert_resource(AssetMeshLibrary::new());
        let shaders = AssetShaderLibrary::new();
        shaders.add_raster(
            RasterShaderProgram::new_draw(
                "vertex_color_shader",
                Default::default(),
                &TestDevice(1),
            ),
            false,
        );
        world.insert_resource(SharedAssetShaderLibrary(Arc::new(shaders)));
        let widget = WidgetId(42);
        world.insert_resource(active_ui(widget));
        world.init_resource::<UiSignifierStream>();
        let curve = Curve2::crashy_new_from_points(
            vec![glam::Vec2::new(-2., 0.), glam::Vec2::new(2., 0.)],
            false,
        );
        world.insert_resource(UiEditRectangleActiveWidgetResource(Some((
            curve.clone(),
            widget,
            glam::Vec3::new(2., 3., 4.),
        ))));
        let mut schedule = Schedule::default();
        schedule.add_systems((signifier_edit_dims, signifier_create).chain());
        schedule.run(&mut world);
        let entities: Vec<_> = world
            .query_filtered::<Entity, With<EditRectangleLinePreview>>()
            .iter(&world)
            .collect();
        assert_eq!(entities.len(), 1);
        let entity = entities[0];
        let mesh = world.get::<TransientMesh>(entity).unwrap().mesh_handle();
        assert!(
            world
                .resource::<AssetMeshLibrary>()
                .get(mesh)
                .unwrap()
                .attributes
                .is_empty(),
            "原工厂先创建空网格"
        );
        assert_eq!(
            world.get::<SignifierAnimation>(entity).unwrap().duration,
            0.3
        );
        assert_eq!(
            world.get::<SignifierAnimation>(entity).unwrap().flag_8,
            1,
            "由线网格长度控制动画，不额外缩放Transform"
        );
        world.insert_resource(active_ui(widget));
        world
            .resource_mut::<UiEditRectangleActiveWidgetResource>()
            .0 = Some((curve, widget, glam::Vec3::new(3., 4., 5.)));
        schedule.run(&mut world);
        assert_eq!(
            world
                .query_filtered::<Entity, With<SignifierId>>()
                .iter(&world)
                .count(),
            1,
            "鼠标移动保持同一widget的指示符身份"
        );
        assert_eq!(
            world.get::<Transform>(entity).unwrap().translation,
            glam::Vec3::new(3., 4., 5.)
        );
        assert_eq!(
            world.get::<Transform>(entity).unwrap().scale,
            glam::Vec3::ONE
        );
        assert_eq!(
            world.get::<TransientMesh>(entity).unwrap().mesh_handle(),
            mesh
        );
        assert_eq!(
            world
                .resource::<AssetMeshLibrary>()
                .get(mesh)
                .unwrap()
                .positions_without_indices()
                .len(),
            2
        );
        world.get_mut::<SignifierAnimation>(entity).unwrap().eased = 1.0;
        world.insert_resource(active_ui(widget));
        schedule.run(&mut world);
        let points = world
            .resource::<AssetMeshLibrary>()
            .get(mesh)
            .unwrap()
            .positions_without_indices();
        assert_eq!(
            points,
            &[glam::Vec3::new(-2., 0., 0.), glam::Vec3::new(2., 0., 0.)]
        );
        world
            .resource_mut::<UiEditRectangleActiveWidgetResource>()
            .0 = None;
        world.insert_resource(UiState::at_startup());
        world.insert_resource(crate::resources::time::GameTime {
            delta: 1.0,
            ..Default::default()
        });
        world.run_system_once(signfier_mark_for_delete).unwrap();
        world.run_system_once(signifier_animation_t).unwrap();
        world.run_system_once(despawn_marked).unwrap();
        assert!(world.get_entity(entity).is_err());
        delete_dropped_transient_meshes(
            &TestDevice(1),
            &mut world.resource_mut::<AssetMeshLibrary>(),
            &mut AssetVAOLibrary::new(),
        );
        assert!(world.resource::<AssetMeshLibrary>().get(mesh).is_none());
    }
}
