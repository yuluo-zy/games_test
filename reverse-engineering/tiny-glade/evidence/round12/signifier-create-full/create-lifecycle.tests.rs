// Append to candidate signifier_create.rs. Tests use original hash oracles and
// an actual picker/update/world/builder lifecycle; they never seed interaction.
#[cfg(test)]
mod original_creation_tests {
    use super::super::{
        signifier_factory::SignifierEntityBuilder,
        signifier_mark_for_delete::{despawn_marked, signfier_mark_for_delete},
    };
    use super::*;
    use crate::{
        asset_libraries::mesh_library::MeshAsset,
        input::action_set::ActionSetStates,
        render::{mesh::Mesh, shader::RasterShaderProgram, test_device::TestDevice},
        resources::{
            cursor_terrain_raycast::CursorTerrainRaycast,
            events::ShutdownGameCmd,
            gui::EscapeMenuJustOpened,
            mouse_capture::OsCursorVisibilityCmd,
            mouse_terrain_raycast::CursorPositionSS,
            read_mouse_depth_value::MouseDepthValue,
            window::{WindowFocusEvent, WindowSize},
        },
        systems::brush_preview::OverwriteBrushPreview,
        systems::camera_rig::MainCamera,
        systems::ui::{
            hover_intersection::hover_intersection,
            resources::widget_params::{PlayUiSound, WidgetGeometry, WidgetParams},
            ui_update::ui_update,
        },
    };
    use glam::{Mat4, Vec2};
    use std::sync::{
        Arc,
        atomic::{AtomicUsize, Ordering},
    };
    #[test]
    fn original_78_hashes_preserve_widget_option_and_utf8_boundaries() {
        let v:serde_json::Value=serde_json::from_str(include_str!(concat!(env!("CARGO_MANIFEST_DIR"),"/../../../reverse-engineering/tiny-glade/evidence/round12/signifier-create-full/native-hash-fixtures.json"))).unwrap();
        for x in v.as_array().unwrap() {
            assert_eq!(
                record_id(WidgetId(x["widget"].as_u64().unwrap()), x["name"].as_str()),
                x["hash"].as_u64().unwrap(),
                "{x}"
            );
        }
        assert_ne!(
            record_id(WidgetId(11), None),
            record_id(WidgetId(11), Some(""))
        );
        assert_eq!(std::mem::size_of::<SignifierId>(), 16);
    }
    fn factory(name: &str, calls: Arc<AtomicUsize>) -> SignifierFactory {
        SignifierFactory::from_closure(
            name,
            Arc::new(move |meshes, shaders, commands| {
                calls.fetch_add(1, Ordering::Relaxed);
                vec![
                    SignifierEntityBuilder::from_mesh_name("brush")
                        .disable_default_animation()
                        .build(meshes, shaders, commands),
                ]
            }),
        )
    }
    fn register_widget(world: &World) {
        world.resource::<UiState>().add_with_id(
            WidgetId(91),
            WidgetParams::default()
                .geometry(WidgetGeometry::ScreenAabb {
                    min: Vec2::splat(40.),
                    max: Vec2::splat(60.),
                    distance: 0.5,
                })
                .occlude_by_world(false),
        );
    }
    fn setup() -> (World, Schedule) {
        bevy_tasks::ComputeTaskPool::get_or_init(|| {
            bevy_tasks::TaskPoolBuilder::new().num_threads(2).build()
        });
        let mut w = World::new();
        w.insert_resource(UiState::at_startup());
        w.init_resource::<UiSignifierStream>();
        let meshes = AssetMeshLibrary::new();
        meshes.add(MeshAsset::new(Mesh::new("named"), Some("brush".into())));
        w.insert_resource(meshes);
        let shaders = AssetShaderLibrary::new();
        shaders.add_raster(
            RasterShaderProgram::new_draw(
                "vertex_color_shader",
                Default::default(),
                &TestDevice(1),
            ),
            false,
        );
        w.insert_resource(shaders);
        let mut camera = MainCamera::new(1.);
        for offset in [0x40, 0x80, 0xc0] {
            camera.write_matrix(offset, &Mat4::IDENTITY);
        }
        w.insert_resource(camera);
        w.insert_resource(CursorTerrainRaycast::default());
        w.insert_resource(CursorPositionSS(Vec2::splat(50.)));
        w.insert_resource(MouseDepthValue::default());
        w.insert_resource(WindowSize::new(100, 100));
        w.insert_resource(ActionSetStates::default());
        w.init_resource::<Events<WindowFocusEvent>>();
        w.init_resource::<Events<EscapeMenuJustOpened>>();
        w.init_resource::<Events<ShutdownGameCmd>>();
        w.init_resource::<Events<OverwriteBrushPreview>>();
        w.init_resource::<Events<OsCursorVisibilityCmd>>();
        w.init_resource::<Events<PlayUiSound>>();
        let mut s = Schedule::default();
        s.add_systems(
            (
                hover_intersection,
                ui_update,
                signifier_create,
                signfier_mark_for_delete,
                despawn_marked,
            )
                .chain_ignore_deferred(),
        );
        (w, s)
    }
    #[test]
    fn inactive_requests_wait_then_duplicate_factories_wake_and_actual_owner_marks() {
        use bevy_ecs::system::RunSystemOnce;
        let (mut w, mut s) = setup();
        let calls = Arc::new(AtomicUsize::new(0));
        let f = factory("a", calls.clone());
        w.resource_mut::<UiSignifierStream>().write_factory(&f);
        w.resource_mut::<UiSignifierStream>().write_factory(&f);
        w.run_system_once(signifier_create).unwrap();
        assert_eq!(w.resource::<UiSignifierStream>().committed_len(), 2);
        assert_eq!(calls.load(Ordering::Relaxed), 0);
        register_widget(&w);
        s.run(&mut w);
        let ids = w
            .query::<&SignifierId>()
            .iter(&w)
            .copied()
            .collect::<Vec<_>>();
        assert_eq!(ids.len(), 2);
        assert!(
            ids.iter()
                .all(|id| id.0 == record_id(WidgetId(91), Some("a")) && id.1 == WidgetId(91))
        );
        assert_eq!(calls.load(Ordering::Relaxed), 2);
        assert_eq!(
            w.resource::<UiState>()
                .active_params()
                .unwrap()
                .signifiers
                .len(),
            2
        );
        // Beginning-of-frame entity snapshot now owns both hashes; repeat request
        // reuses them, while active identity comes from UiState instead of stream.
        w.resource_mut::<UiSignifierStream>().write_factory(&f);
        register_widget(&w);
        s.run(&mut w);
        assert_eq!(calls.load(Ordering::Relaxed), 2);
        assert_eq!(w.query::<&SignifierId>().iter(&w).count(), 2);
        register_widget(&w);
        s.run(&mut w);
        assert_eq!(w.query::<&SignifierMarkedForDeletion>().iter(&w).count(), 2);
        // Deferred native chain exposes the marker on the following frame.
        register_widget(&w);
        s.run(&mut w);
        assert_eq!(w.query::<&SignifierId>().iter(&w).count(), 0);
    }
    #[test]
    fn mark_keeps_minimum_lifespan_but_nan_matches_original_ucomiss() {
        use bevy_ecs::system::RunSystemOnce;
        let (mut w, _) = setup();
        let keep = w
            .spawn((
                SignifierId(1, WidgetId(91)),
                super::super::signifier_mark_for_delete::SignifierMinimumLifespan {
                    t: 0.1,
                    min_lifespan: 0.2,
                },
            ))
            .id();
        let nan = w
            .spawn((
                SignifierId(2, WidgetId(91)),
                super::super::signifier_mark_for_delete::SignifierMinimumLifespan {
                    t: f32::NAN,
                    min_lifespan: 0.2,
                },
            ))
            .id();
        w.run_system_once(signfier_mark_for_delete).unwrap();
        assert!(w.get::<SignifierMarkedForDeletion>(keep).is_none());
        assert!(w.get::<SignifierMarkedForDeletion>(nan).is_some());
    }
}
