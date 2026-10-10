//! 使用真实屏幕投影、命令、后台语义与上传链路验证上下文工具。
use super::*;
use crate::{
    catalog::ArtCatalog,
    renderer::{DisplayedBuilding, PresentationPlugin},
    strokes::{StrokeDisplay, StrokePlugin},
};
use garden_bevy::{EditFeedback, GardenPlugin, Operation};
use garden_domain::{BuildingId, sample_building};
use std::time::{Duration, Instant};
fn app() -> App {
    let mut app = App::new();
    app.init_resource::<Assets<Mesh>>()
        .init_resource::<Assets<StandardMaterial>>()
        .init_resource::<Assets<bevy::gizmos::GizmoAsset>>()
        .add_plugins((
            GardenPlugin::default(),
            PresentationPlugin::new(ArtCatalog::warm_stone()),
            StrokePlugin,
        ));
    crate::editor::setup_context_test_input(&mut app);
    let orbit = OrbitCamera::from_view(Vec3::new(10., 15., 20.), Vec3::ZERO).unwrap();
    let mut camera = Camera::default();
    camera.computed.clip_from_view =
        Mat4::perspective_infinite_reverse_rh(std::f32::consts::PI / 3., 1.5, 0.1);
    camera.computed.target_info = Some(bevy::camera::RenderTargetInfo {
        physical_size: UVec2::new(1440, 960),
        scale_factor: 1.,
    });
    app.world_mut().spawn((camera, orbit.transform(), orbit));
    app.update();
    wait(&mut app, settled);
    app
}
fn wait(app: &mut App, condition: impl Fn(&App) -> bool) {
    let deadline = Instant::now() + Duration::from_secs(12);
    while !condition(app) {
        app.update();
        for outcome in app.world_mut().resource_mut::<EditFeedback>().drain() {
            outcome.result.unwrap();
        }
        assert!(
            Instant::now() < deadline,
            "上下文链路未收敛：{}，历史={}，窗数={}，模式={}",
            app.world().resource::<DesktopEditor>().status,
            app.world().resource::<EditorState>().editor().undo_count(),
            app.world()
                .resource::<EditorState>()
                .editor()
                .context()
                .openings
                .len(),
            app.world().resource::<ContextTools>().mode as u8
        );
        std::thread::sleep(Duration::from_millis(1));
    }
}
fn settled(app: &App) -> bool {
    !app.world().resource::<ContextProjection>().busy
        && app.world().resource::<StrokeDisplay>().revision
            == Some(
                app.world()
                    .resource::<EditorState>()
                    .editor()
                    .world_revision(),
            )
        && app
            .world()
            .resource::<garden_bevy::PendingTargets>()
            .is_empty()
}
fn action(app: &mut App, action: impl Component) {
    let e = app.world_mut().spawn((Interaction::Pressed, action)).id();
    app.update();
    app.world_mut().entity_mut(e).despawn();
}
fn frame(app: &mut App, p: Vec3, begin: bool, finish: bool) {
    let mut query = app.world_mut().query::<(&Camera, &Transform)>();
    let (c, t) = query.iter(app.world()).next().unwrap();
    let cursor = c.world_to_viewport(&GlobalTransform::from(*t), p).unwrap();
    *app.world_mut().resource_mut::<PointerFrame>() = PointerFrame {
        cursor: Some(cursor),
        world_begin: begin,
        world_held: !finish,
        world_finish: finish,
        ..default()
    };
    app.update();
    *app.world_mut().resource_mut::<PointerFrame>() = PointerFrame::default();
}
fn building(app: &mut App) {
    app.world_mut()
        .resource_mut::<CommandInbox>()
        .submit(Operation::Edit(EditCommand::Create(sample_building(
            BuildingId::new(1).unwrap(),
            6.,
            3.,
        ))))
        .unwrap();
    wait(app, |a| {
        let mut q = a.world().try_query::<&DisplayedBuilding>().unwrap();
        q.iter(a.world()).next().is_some() && settled(a)
    });
}
#[test]
fn auto_stroke_changes_type_and_undo_restores_the_saved_decision() {
    let mut app = app();
    action(&mut app, ContextAction::AutoDraw);
    frame(&mut app, Vec3::new(-4., 0., 0.), true, false);
    frame(&mut app, Vec3::new(4., 0., 0.), false, true);
    assert_eq!(
        app.world().resource::<EditorState>().editor().undo_count(),
        0
    );
    wait(&mut app, |a| {
        a.world()
            .resource::<EditorState>()
            .editor()
            .strokes()
            .count()
            == 1
            && settled(a)
    });
    let id = garden_domain::strokes::StrokeId(1);
    assert_eq!(
        app.world().resource::<ContextProjection>().semantic.linear[&id].kind,
        StrokeKind::Path
    );
    for _ in 0..15 {
        let revision = app
            .world()
            .resource::<EditorState>()
            .editor()
            .world_revision();
        action(&mut app, StrokeAction::HeightUp);
        wait(&mut app, |a| {
            a.world()
                .resource::<EditorState>()
                .editor()
                .world_revision()
                > revision
                && settled(a)
        });
    }
    assert_eq!(
        app.world().resource::<ContextProjection>().semantic.linear[&id].kind,
        StrokeKind::Wall
    );
    let count = app.world().resource::<EditorState>().editor().undo_count();
    action(&mut app, EditorAction::Undo);
    wait(&mut app, settled);
    assert_eq!(
        app.world().resource::<EditorState>().editor().undo_count(),
        count - 1
    );
    assert_eq!(
        app.world().resource::<ContextProjection>().semantic.linear[&id].kind,
        StrokeKind::Fence
    );
    assert_eq!(
        app.world()
            .resource::<EditorState>()
            .editor()
            .stroke(id)
            .unwrap()
            .points
            .len(),
        2
    );
}
#[test]
fn terrain_preview_cancel_commit_and_undo_do_not_modify_building_intent() {
    let mut app = app();
    building(&mut app);
    let original = app
        .world()
        .resource::<EditorState>()
        .editor()
        .get(BuildingId::new(1).unwrap())
        .unwrap()
        .clone();
    action(&mut app, ContextAction::Raise);
    for i in 0..12 {
        frame(&mut app, Vec3::new(3., 0., 1.), i == 0, false);
    }
    assert!(
        app.world()
            .resource::<EditorState>()
            .editor()
            .context()
            .terrain
            .tiles
            .is_empty()
    );
    *app.world_mut().resource_mut::<PointerFrame>() = PointerFrame {
        cancel_world: true,
        ..default()
    };
    app.update();
    assert_eq!(
        app.world().resource::<EditorState>().editor().undo_count(),
        1
    );
    *app.world_mut().resource_mut::<PointerFrame>() = PointerFrame::default();
    for i in 0..12 {
        frame(&mut app, Vec3::new(3., 0., 1.), i == 0, i == 11);
    }
    wait(&mut app, |a| {
        a.world().resource::<EditorState>().editor().undo_count() == 2 && settled(a)
    });
    assert!(
        app.world()
            .resource::<ContextProjection>()
            .semantic
            .foundations[&original.id()]
            > 0.
    );
    assert_eq!(
        original,
        app.world()
            .resource::<EditorState>()
            .editor()
            .get(original.id())
            .unwrap()
            .clone()
    );
    action(&mut app, EditorAction::Undo);
    wait(&mut app, settled);
    assert!(
        app.world()
            .resource::<EditorState>()
            .editor()
            .context()
            .terrain
            .tiles
            .is_empty()
    );
    assert_eq!(
        app.world()
            .resource::<ContextProjection>()
            .semantic
            .foundations[&original.id()],
        0.
    );
}
#[test]
fn two_windows_combine_and_deletion_undo_preserves_sources() {
    let mut app = app();
    building(&mut app);
    let b = app
        .world()
        .resource::<EditorState>()
        .editor()
        .get(BuildingId::new(1).unwrap())
        .unwrap()
        .clone();
    let depth = b.blocks()[0].footprint.depth as f32;
    action(&mut app, ContextAction::PlaceWindow);
    for x in [1.0, 2.0] {
        frame(&mut app, Vec3::new(x, 1.4, depth), true, false);
        frame(&mut app, Vec3::new(x, 1.4, depth), false, true);
        let count = if x < 2. { 1 } else { 2 };
        wait(&mut app, |a| {
            a.world()
                .resource::<EditorState>()
                .editor()
                .context()
                .openings
                .len()
                == count
                && settled(a)
        });
    }
    assert!(
        app.world()
            .resource::<ContextProjection>()
            .semantic
            .facades
            .values()
            .any(|f| f.assemblies.iter().any(|g| g.members.len() == 2))
    );
    let id = app.world().resource::<ContextTools>().selected.unwrap();
    action(&mut app, ContextAction::DeleteWindow);
    wait(&mut app, |a| {
        a.world()
            .resource::<EditorState>()
            .editor()
            .context()
            .openings
            .len()
            == 1
            && settled(a)
    });
    action(&mut app, EditorAction::Undo);
    wait(&mut app, |a| {
        a.world()
            .resource::<EditorState>()
            .editor()
            .context()
            .openings
            .len()
            == 2
            && settled(a)
    });
    assert!(
        app.world()
            .resource::<EditorState>()
            .editor()
            .context()
            .openings
            .contains_key(&id)
    );
}

#[test]
fn held_candidate_resolves_before_release_and_replacement_cancels_it() {
    let mut app = app();
    action(&mut app, ContextAction::AutoDraw);
    frame(&mut app, Vec3::new(-4., 0., 0.), true, false);
    frame(&mut app, Vec3::new(4., 0., 0.), false, false);
    wait(&mut app, |a| {
        a.world().resource::<ContextTools>().preview.is_some()
    });
    assert_eq!(
        app.world().resource::<EditorState>().editor().undo_count(),
        0
    );
    frame(&mut app, Vec3::new(4., 0., 0.), false, true);
    wait(&mut app, |a| {
        a.world().resource::<EditorState>().editor().undo_count() == 1 && settled(a)
    });
    action(&mut app, ContextAction::AutoDraw);
    frame(&mut app, Vec3::new(-4., 0., 2.), true, false);
    frame(&mut app, Vec3::new(4., 0., 2.), false, false);
    wait(&mut app, |a| {
        a.world().resource::<ContextTools>().preview.is_some()
    });
    app.world_mut()
        .resource_mut::<CommandInbox>()
        .submit(Operation::ReplaceScene(Vec::new()))
        .unwrap();
    app.update();
    wait(&mut app, settled);
    assert!(app.world().resource::<ContextTools>().preview.is_none());
    assert_eq!(
        app.world().resource::<EditorState>().editor().undo_count(),
        0
    );
}
