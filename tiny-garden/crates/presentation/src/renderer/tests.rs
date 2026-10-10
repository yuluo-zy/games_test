use super::*;
use garden_application::EditCommand;
use garden_bevy::{CommandInbox, EditFeedback, GardenPlugin, Operation};
use garden_domain::{BuildingId, sample_building};
use std::time::{Duration, Instant};
fn app(animated: bool) -> App {
    let mut app = App::new();
    app.init_resource::<Assets<Mesh>>()
        .init_resource::<Assets<StandardMaterial>>();
    if animated {
        app.init_resource::<Time>();
    }
    app.add_plugins((
        GardenPlugin::default()
            .with_compiler(Arc::new(crate::building_kit::BuildingKit::warm_stone())),
        PresentationPlugin::new(ArtCatalog::warm_stone()),
    ));
    app
}
fn submit(app: &mut App, command: EditCommand) {
    app.world_mut()
        .resource_mut::<CommandInbox>()
        .submit(Operation::Edit(command))
        .unwrap();
}
fn tick(app: &mut App) {
    if let Some(mut time) = app.world_mut().get_resource_mut::<Time>() {
        time.advance_by(Duration::from_millis(16));
    }
    app.update();
    for outcome in app.world_mut().resource_mut::<EditFeedback>().drain() {
        outcome.result.unwrap();
    }
    std::thread::sleep(Duration::from_millis(1));
}
fn wait(app: &mut App, condition: impl Fn(&App) -> bool) {
    let deadline = Instant::now() + Duration::from_secs(8);
    while !condition(app) {
        assert!(Instant::now() < deadline);
        tick(app);
    }
}
fn create(app: &mut App) -> garden_domain::BuildingId {
    let id = BuildingId::new(1).unwrap();
    submit(app, EditCommand::Create(sample_building(id, 6., 6.)));
    wait(app, |a| {
        a.world().resource::<RenderStats>().buildings == 1
            && a.world().resource::<Fades>().0.is_empty()
    });
    id
}
fn height(app: &mut App, id: BuildingId, height: f64) {
    let footprint = app
        .world()
        .resource::<EditorState>()
        .editor()
        .get(id)
        .unwrap()
        .blocks()[0]
        .footprint;
    submit(
        app,
        EditCommand::Edit {
            building: id,
            edit: garden_domain::BuildingEdit::Resize {
                block: garden_domain::BlockId::new(1).unwrap(),
                footprint,
                height,
            },
        },
    );
}
#[test]
fn move_uploads_zero_bytes_and_roof_changes_keep_wall_gpu_handles() {
    use garden_domain::{BlockId, BuildingEdit, Placement, Roof};
    let mut app = app(false);
    let id = create(&mut app);
    let root = *app.world().resource::<Residency>().0.keys().next().unwrap();
    let before = app.world().resource::<Residency>().0[&root]
        .parts
        .iter()
        .map(|(id, p)| (*id, (p.child, p.handle.id())))
        .collect::<BTreeMap<_, _>>();
    let bytes = app.world().resource::<RenderStats>().uploaded_bytes;
    submit(
        &mut app,
        EditCommand::Edit {
            building: id,
            edit: BuildingEdit::Move(Placement {
                x: 10.,
                z: 5.,
                elevation: 0.,
                yaw: 0.5,
            }),
        },
    );
    wait(&mut app, |a| {
        a.world().resource::<RenderStats>().uploads == 2
    });
    assert_eq!(app.world().resource::<RenderStats>().uploaded_bytes, bytes);
    assert_eq!(
        app.world().get::<Transform>(root).unwrap().translation,
        Vec3::new(10., 0., 5.)
    );
    for (id, p) in &app.world().resource::<Residency>().0[&root].parts {
        assert_eq!(before[id], (p.child, p.handle.id()));
    }
    submit(
        &mut app,
        EditCommand::Edit {
            building: id,
            edit: BuildingEdit::SetRoof {
                block: BlockId::new(1).unwrap(),
                roof: Roof::Flat,
            },
        },
    );
    wait(&mut app, |a| {
        a.world().resource::<RenderStats>().uploads == 3
    });
    for (id, p) in &app.world().resource::<Residency>().0[&root].parts {
        if !matches!(id.part.kind, garden_generation::incremental::PartKind::Roof) {
            assert_eq!(before[id], (p.child, p.handle.id()));
        }
    }
}
#[test]
fn multi_frame_upload_keeps_old_complete_pose_and_delete_reclaims_staged_assets() {
    let mut app = app(false);
    let id = create(&mut app);
    let root = *app.world().resource::<Residency>().0.keys().next().unwrap();
    let ticket = app.world().get::<DisplayedBuilding>(root).unwrap().ticket;
    app.world_mut()
        .resource_mut::<UploadBudget>()
        .batches_per_frame = 1;
    height(&mut app, id, 9.);
    let mut staged_frames = 0;
    let deadline = Instant::now() + Duration::from_secs(8);
    while app.world().resource::<RenderStats>().uploads < 2 {
        assert!(Instant::now() < deadline);
        let bytes = app.world().resource::<RenderStats>().uploaded_bytes;
        tick(&mut app);
        assert!(app.world().resource::<RenderStats>().uploaded_bytes - bytes <= 512 * 1024);
        if !app.world().resource::<Staging>().0.is_empty() {
            staged_frames += 1;
            assert_eq!(
                app.world().get::<DisplayedBuilding>(root).unwrap().ticket,
                ticket
            );
        }
    }
    assert!(staged_frames >= 5);
    height(&mut app, id, 12.);
    wait(&mut app, |a| !a.world().resource::<Staging>().0.is_empty());
    submit(&mut app, EditCommand::Delete(id));
    tick(&mut app);
    assert!(app.world().resource::<Staging>().0.is_empty());
    assert_eq!(app.world().resource::<Assets<Mesh>>().len(), 0);
    assert!(
        app.world()
            .resource::<MeshReadiness>()
            .shared
            .lock()
            .unwrap()
            .waiting
            .is_empty()
    );
}
#[test]
fn render_readiness_ack_is_required_before_publishing_a_group() {
    let mut app = app(false);
    app.world_mut().resource_mut::<MeshReadiness>().enabled = true;
    let id = BuildingId::new(1).unwrap();
    submit(&mut app, EditCommand::Create(sample_building(id, 3., 3.)));
    wait(&mut app, |a| {
        a.world()
            .resource::<Staging>()
            .0
            .values()
            .any(|s| s.upload.pending.is_empty())
    });
    assert_eq!(app.world().resource::<RenderStats>().buildings, 0);
    let bridge = app.world().resource::<MeshReadiness>().clone();
    let mut state = bridge.shared.lock().unwrap();
    state.ready = std::mem::take(&mut state.waiting);
    drop(state);
    tick(&mut app);
    assert_eq!(app.world().resource::<RenderStats>().buildings, 1);
}
#[test]
fn coherent_crossfade_switches_pick_pose_at_midpoint_and_reclaims_on_delete() {
    let mut app = app(true);
    let id = create(&mut app);
    let root = *app.world().resource::<Residency>().0.keys().next().unwrap();
    let ticket = app.world().get::<DisplayedBuilding>(root).unwrap().ticket;
    assert_eq!(app.world().resource::<Assets<StandardMaterial>>().len(), 5);
    height(&mut app, id, 9.);
    wait(&mut app, |a| !a.world().resource::<Fades>().0.is_empty());
    assert_eq!(
        app.world().get::<DisplayedBuilding>(root).unwrap().ticket,
        ticket
    );
    assert!(!app.world().resource::<Fades>().0[&root].retired.is_empty());
    wait(&mut app, |a| {
        a.world()
            .resource::<Fades>()
            .0
            .get(&root)
            .is_some_and(|f| f.handoff.switched)
    });
    assert_ne!(
        app.world().get::<DisplayedBuilding>(root).unwrap().ticket,
        ticket
    );
    wait(&mut app, |a| a.world().resource::<Fades>().0.is_empty());
    assert_eq!(
        app.world().resource::<Assets<Mesh>>().len(),
        app.world().resource::<RenderStats>().batches
    );
    assert_eq!(app.world().resource::<Assets<StandardMaterial>>().len(), 5);
    height(&mut app, id, 12.);
    wait(&mut app, |a| !a.world().resource::<Fades>().0.is_empty());
    submit(&mut app, EditCommand::Delete(id));
    tick(&mut app);
    assert!(app.world().resource::<Fades>().0.is_empty());
    assert_eq!(app.world().resource::<Assets<Mesh>>().len(), 0);
    assert_eq!(app.world().resource::<Assets<StandardMaterial>>().len(), 5);
}
#[test]
fn rapid_targets_and_undo_settle_on_latest_pose_without_visual_resource_leaks() {
    let mut app = app(true);
    let id = create(&mut app);
    let root = *app.world().resource::<Residency>().0.keys().next().unwrap();
    height(&mut app, id, 9.);
    wait(&mut app, |a| !a.world().resource::<Fades>().0.is_empty());
    height(&mut app, id, 12.);
    tick(&mut app);
    height(&mut app, id, 15.);
    tick(&mut app);
    submit(&mut app, EditCommand::Undo);
    tick(&mut app);
    let latest = app
        .world()
        .resource::<EditorState>()
        .editor()
        .latest_ticket(id)
        .unwrap();
    wait(&mut app, |a| {
        a.world().resource::<Fades>().0.is_empty()
            && a.world().resource::<Staging>().0.is_empty()
            && a.world()
                .get::<DisplayedBuilding>(root)
                .is_some_and(|p| p.ticket == latest)
    });
    assert_eq!(
        app.world()
            .get::<DisplayedBuilding>(root)
            .unwrap()
            .target
            .blocks[0]
            .height,
        12.
    );
    assert_eq!(
        app.world().resource::<Assets<Mesh>>().len(),
        app.world().resource::<RenderStats>().batches
    );
    assert_eq!(app.world().resource::<Assets<StandardMaterial>>().len(), 5);
    assert!(app.world().resource::<PendingTargets>().get(id).is_none());
}
#[test]
fn moving_retargets_current_pose_and_scene_swap_releases_an_active_fade() {
    use garden_domain::{BuildingEdit, Placement};
    let mut app = app(true);
    let id = create(&mut app);
    let root = *app.world().resource::<Residency>().0.keys().next().unwrap();
    submit(
        &mut app,
        EditCommand::Edit {
            building: id,
            edit: BuildingEdit::Move(Placement {
                x: 10.,
                ..default()
            }),
        },
    );
    wait(&mut app, |a| {
        a.world().resource::<RenderStats>().uploads == 2
    });
    let x = app.world().get::<Transform>(root).unwrap().translation.x;
    assert!(x > 0. && x < 10.);
    submit(
        &mut app,
        EditCommand::Edit {
            building: id,
            edit: BuildingEdit::Move(Placement {
                x: -10.,
                ..default()
            }),
        },
    );
    wait(&mut app, |a| {
        a.world().resource::<RenderStats>().uploads == 3
    });
    let x = app.world().get::<Transform>(root).unwrap().translation.x;
    assert!(x > -10. && x < 10.);
    height(&mut app, id, 9.);
    wait(&mut app, |a| !a.world().resource::<Fades>().0.is_empty());
    let replacement = garden_domain::Building::try_new(sample_building(id, 3., 3.)).unwrap();
    app.world_mut()
        .resource_mut::<CommandInbox>()
        .submit(Operation::ReplaceScene(vec![replacement]))
        .unwrap();
    tick(&mut app);
    assert!(app.world().get_entity(root).is_err());
    assert!(app.world().resource::<Fades>().0.is_empty());
    wait(&mut app, |a| {
        a.world().resource::<RenderStats>().buildings == 1
    });
    assert_eq!(
        app.world().resource::<Assets<Mesh>>().len(),
        app.world().resource::<RenderStats>().batches
    );
    assert_eq!(app.world().resource::<Assets<StandardMaterial>>().len(), 5);
}
#[test]
fn visible_cold_fade_keeps_old_opaque_picture_until_pipeline_ack() {
    let mut app = app(true);
    let id = create(&mut app);
    let root = *app.world().resource::<Residency>().0.keys().next().unwrap();
    let ticket = app.world().get::<DisplayedBuilding>(root).unwrap().ticket;
    app.world_mut().resource_mut::<MeshReadiness>().enabled = true;
    height(&mut app, id, 9.);
    wait(&mut app, |a| {
        a.world()
            .resource::<Staging>()
            .0
            .values()
            .any(|s| s.upload.pending.is_empty())
    });
    let ready = app
        .world()
        .resource::<Assets<Mesh>>()
        .iter()
        .map(|(id, _)| id)
        .collect::<Vec<_>>();
    app.world()
        .resource::<MeshReadiness>()
        .shared
        .lock()
        .unwrap()
        .ready
        .extend(ready);
    tick(&mut app);
    let children = {
        let fade = &app.world().resource::<Fades>().0[&root];
        fade.restore
            .iter()
            .map(|(e, _)| *e)
            .chain(fade.warmups.iter().copied())
            .collect::<Vec<_>>()
    };
    app.world()
        .resource::<MeshReadiness>()
        .shared
        .lock()
        .unwrap()
        .visible_draws
        .extend(children.iter().copied());
    for _ in 0..5 {
        tick(&mut app);
    }
    assert_eq!(app.world().resource::<Fades>().0[&root].handoff.elapsed, 0.);
    let old_child = app.world().resource::<Fades>().0[&root].retired[0].child;
    let material = &app
        .world()
        .get::<MeshMaterial3d<StandardMaterial>>(old_child)
        .unwrap()
        .0;
    assert_eq!(
        app.world()
            .resource::<Assets<StandardMaterial>>()
            .get(material)
            .unwrap()
            .alpha_mode,
        AlphaMode::Opaque
    );
    assert_eq!(
        app.world().get::<DisplayedBuilding>(root).unwrap().ticket,
        ticket
    );
    let bridge = app.world().resource::<MeshReadiness>().clone();
    bridge.shared.lock().unwrap().ready_draws.extend(children);
    wait(&mut app, |a| a.world().resource::<Fades>().0.is_empty());
    assert!(bridge.shared.lock().unwrap().ready_draws.is_empty());
    assert_ne!(
        app.world().get::<DisplayedBuilding>(root).unwrap().ticket,
        ticket
    );
}
#[test]
fn newer_target_cancels_unstarted_shader_wait_without_deleting_old_picture() {
    let mut app = app(true);
    let id = create(&mut app);
    let root = *app.world().resource::<Residency>().0.keys().next().unwrap();
    let original = app.world().get::<DisplayedBuilding>(root).unwrap().ticket;
    let baseline = app.world().resource::<Residency>().0[&root].clone();
    app.world_mut().resource_mut::<MeshReadiness>().enabled = true;
    height(&mut app, id, 9.);
    wait(&mut app, |a| {
        a.world()
            .resource::<Staging>()
            .0
            .values()
            .any(|s| s.upload.pending.is_empty())
    });
    let ready = app
        .world()
        .resource::<Assets<Mesh>>()
        .iter()
        .map(|(id, _)| id)
        .collect::<Vec<_>>();
    app.world()
        .resource::<MeshReadiness>()
        .shared
        .lock()
        .unwrap()
        .ready
        .extend(ready);
    tick(&mut app);
    assert!(!app.world().resource::<Fades>().0[&root].handoff.started);
    height(&mut app, id, 12.);
    tick(&mut app);
    assert!(app.world().resource::<Fades>().0.is_empty());
    assert_eq!(
        app.world().get::<DisplayedBuilding>(root).unwrap().ticket,
        original
    );
    for (id, part) in &baseline.parts {
        assert_eq!(
            app.world().resource::<Residency>().0[&root].parts[id].child,
            part.child
        );
        assert!(app.world().get_entity(part.child).is_ok());
        assert!(
            app.world()
                .resource::<Assets<Mesh>>()
                .get(&part.handle)
                .is_some()
        );
    }
    // Finish the headless simulation using immediate resource readiness.
    app.world_mut().resource_mut::<MeshReadiness>().enabled = false;
    let latest = app
        .world()
        .resource::<EditorState>()
        .editor()
        .latest_ticket(id)
        .unwrap();
    wait(&mut app, |a| {
        a.world().resource::<Fades>().0.is_empty()
            && a.world()
                .get::<DisplayedBuilding>(root)
                .is_some_and(|d| d.ticket == latest)
    });
    assert_eq!(
        app.world().resource::<Assets<Mesh>>().len(),
        app.world().resource::<RenderStats>().batches
    );
    assert_eq!(app.world().resource::<Assets<StandardMaterial>>().len(), 5);
}
#[test]
fn worker_tangents_match_bevy_and_production_chunks_fit_upload_budget() {
    let mut draft = sample_building(BuildingId::new(1).unwrap(), 100., 12.);
    draft.blocks[0].footprint.depth = 100.;
    let b = garden_domain::Building::try_new(draft).unwrap();
    let cancel = garden_generation::incremental::Cancellation::default();
    let kit = crate::building_kit::BuildingKit::warm_stone();
    let mut cpu = kit
        .compile_incremental(
            &mut garden_generation::compile(&b),
            garden_generation::mesh::GeometryProfile::default(),
            None,
            &cancel,
        )
        .unwrap();
    garden_bevy::prepare::prepare(&mut cpu, &cancel).unwrap();
    for batch in &cpu.batches {
        assert!(
            batch.data.positions.len() * 48 + batch.data.indices.len() * 4
                <= UploadBudget::default().bytes_per_frame
        );
        let mut reference = to_bevy_mesh(batch);
        reference.generate_tangents().unwrap();
        assert_eq!(
            reference.attribute(Mesh::ATTRIBUTE_TANGENT),
            to_bevy_mesh(batch).attribute(Mesh::ATTRIBUTE_TANGENT)
        );
    }
}
#[test]
fn upload_batches_are_bounded_and_owned_meshes_are_released_on_delete() {
    let mut app = App::new();
    // Assets containers are sufficient for testing upload ownership without GPU.
    app.init_resource::<Assets<Mesh>>()
        .init_resource::<Assets<StandardMaterial>>();
    let catalog = ArtCatalog::warm_stone();
    app.add_plugins((
        GardenPlugin::default()
            .with_geometry(catalog.geometry())
            .unwrap()
            .with_compiler(Arc::new(crate::building_kit::BuildingKit::warm_stone())),
        PresentationPlugin::new(catalog),
    ));
    let id = BuildingId::new(1).unwrap();
    app.world_mut()
        .resource_mut::<CommandInbox>()
        .submit(Operation::Edit(EditCommand::Create(sample_building(
            id, 6.0, 6.0,
        ))))
        .unwrap();
    let deadline = Instant::now() + Duration::from_secs(5);
    loop {
        app.update();
        for outcome in app.world_mut().resource_mut::<EditFeedback>().drain() {
            outcome.result.unwrap();
        }
        if app.world().resource::<RenderStats>().buildings == 1 {
            break;
        }
        assert!(Instant::now() < deadline);
        std::thread::sleep(Duration::from_millis(1));
    }
    let stats = app.world().resource::<RenderStats>();
    assert!(stats.batches <= 48 && stats.triangles > 0);
    let old_children = app
        .world()
        .resource::<Residency>()
        .0
        .values()
        .next()
        .unwrap()
        .parts
        .values()
        .map(|p| p.child)
        .collect::<Vec<_>>();
    app.world_mut()
        .resource_mut::<CommandInbox>()
        .submit(Operation::Edit(EditCommand::Edit {
            building: id,
            edit: garden_domain::BuildingEdit::Resize {
                block: garden_domain::BlockId::new(1).unwrap(),
                footprint: garden_geometry::Rect {
                    x: 0.0,
                    z: 0.0,
                    width: 10.0,
                    depth: 6.0,
                },
                height: 12.0,
            },
        }))
        .unwrap();
    loop {
        app.update();
        for outcome in app.world_mut().resource_mut::<EditFeedback>().drain() {
            outcome.result.unwrap();
        }
        if app.world().resource::<RenderStats>().uploads == 2 {
            break;
        }
        assert!(Instant::now() < deadline);
        std::thread::sleep(Duration::from_millis(1));
    }
    assert!(
        old_children
            .into_iter()
            .any(|e| app.world().get_entity(e).is_ok())
    );
    assert_eq!(
        app.world().resource::<Assets<Mesh>>().len(),
        app.world().resource::<RenderStats>().batches
    );
    app.world_mut()
        .resource_mut::<CommandInbox>()
        .submit(Operation::Edit(EditCommand::Delete(id)))
        .unwrap();
    app.update();
    assert_eq!(app.world().resource::<RenderStats>().buildings, 0);
    assert_eq!(app.world().resource::<Assets<Mesh>>().len(), 0);
}
