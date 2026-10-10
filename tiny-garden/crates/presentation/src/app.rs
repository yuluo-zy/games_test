//! Desktop app assembly. Normal launch opens the game; capture modes render offscreen.
use crate::{
    building_kit::BuildingKit,
    camera::{OrbitCamera, OrbitCameraPlugin},
    catalog::ArtCatalog,
    editor::{DesktopEditor, DesktopEditorPlugin},
    renderer::{ArtLoadState, PresentationPlugin, RenderStats},
    ui::{self, UiFont},
};
use bevy::ecs::system::SystemParam;
use bevy::{
    app::{AppExit, ScheduleRunnerPlugin},
    camera::RenderTarget,
    prelude::*,
    render::{
        render_resource::TextureFormat,
        view::screenshot::{Screenshot, ScreenshotCaptured, save_to_disk},
    },
    window::ExitCondition,
    winit::WinitPlugin,
};
use garden_application::tools::{GroundPoint, Tool};
use garden_bevy::{CommandInbox, EditorState, GardenPlugin, Operation};
use garden_domain::{
    BlockDraft, BlockId, Building, BuildingId, Facade, Roof, Stories, sample_building,
};
use garden_geometry::Rect;
use std::{
    path::PathBuf,
    time::{Duration, Instant},
};

#[derive(Resource)]
struct Capture {
    path: Option<PathBuf>,
    image: Option<Handle<Image>>,
    started: Instant,
    ready_frames: u32,
    requested: bool,
    review: bool,
    tools_review: bool,
    review_phase: u8,
    closeup: bool,
    wall_review: bool,
    context_review: bool,
}
pub fn run() {
    let mut args = std::env::args().skip(1);
    let (path, review, tools_review, closeup, wall_review, context_review) = match args
        .next()
        .as_deref()
    {
        None => (None, false, false, false, false, false),
        Some(
            flag @ ("--capture" | "--review-capture" | "--tools-review" | "--art-closeup"
            | "--wall-review" | "--context-review"),
        ) => (
            Some(PathBuf::from(
                args.next().expect("capture requires a PNG path"),
            )),
            flag == "--review-capture" || flag == "--tools-review",
            flag == "--tools-review",
            flag == "--art-closeup",
            flag == "--wall-review",
            flag == "--context-review",
        ),
        _ => panic!(
            "usage: tiny-garden [--capture PATH.png | --review-capture PATH.png | --tools-review PATH.png | --art-closeup PATH.png | --wall-review PATH.png]"
        ),
    };
    assert!(args.next().is_none(), "unexpected arguments");
    if let Some(path) = &path {
        assert!(
            !path.exists(),
            "capture path already exists; choose a new path"
        );
        if let Some(parent) = path.parent() {
            std::fs::create_dir_all(parent).expect("capture directory");
        }
    }
    let art = ArtCatalog::warm_stone();
    let mut app = App::new();
    let asset_plugin = AssetPlugin {
        file_path: PathBuf::from(env!("CARGO_MANIFEST_DIR"))
            .join("../../assets")
            .to_string_lossy()
            .into_owned(),
        ..default()
    };
    if path.is_some() {
        app.add_plugins(
            DefaultPlugins
                .set(asset_plugin)
                .set(WindowPlugin {
                    primary_window: None,
                    exit_condition: ExitCondition::DontExit,
                    ..default()
                })
                .disable::<WinitPlugin>(),
        )
        .add_plugins(ScheduleRunnerPlugin::run_loop(Duration::from_secs_f64(
            1.0 / 60.0,
        )));
    } else {
        app.add_plugins(DefaultPlugins.set(asset_plugin).set(WindowPlugin {
            primary_window: Some(Window {
                title: ui::TITLE.into(),
                resolution: (1440, 960).into(),
                ..default()
            }),
            ..default()
        }));
    }
    app.init_resource::<UiFont>()
        .insert_resource(ClearColor(Color::srgb_u8(224, 229, 215)))
        .insert_resource(Capture {
            path,
            image: None,
            started: Instant::now(),
            ready_frames: 0,
            requested: false,
            review,
            tools_review,
            review_phase: 0,
            closeup,
            wall_review,
            context_review,
        })
        .add_plugins((
            GardenPlugin::default()
                .with_geometry(art.geometry())
                .unwrap()
                .with_compiler(std::sync::Arc::new(BuildingKit::warm_stone())),
            PresentationPlugin::new(art).with_house_textures(),
            OrbitCameraPlugin,
            DesktopEditorPlugin,
            ui::ToolbarPlugin,
            crate::preview::PreviewPlugin(ArtCatalog::warm_stone().geometry()),
            crate::strokes::StrokePlugin,
        ))
        .add_systems(Startup, (setup_scene, setup_ui))
        .add_systems(
            Update,
            (
                prepare_review,
                prepare_wall_review,
                prepare_context_review,
                capture,
            )
                .chain()
                .after(garden_bevy::GardenSet::Collect),
        )
        .run();
}

// GPU fixture uses the same command/history and projection pipeline as players.
// CPU gesture tests separately exercise screen rays and pointer cancellation.
fn prepare_wall_review(world: &mut World) {
    use garden_application::{EditCommand, StrokeEdit};
    use garden_domain::strokes::{Point, Stroke, StrokeId, StrokeKind};
    let capture = world.resource::<Capture>();
    if !capture.wall_review || capture.review_phase >= 4 {
        return;
    }
    let phase = capture.review_phase;
    if world.resource::<RenderStats>().buildings != 6 {
        return;
    }
    let editor = world.resource::<EditorState>().editor();
    if phase > 0
        && world.resource::<crate::strokes::StrokeDisplay>().revision
            != Some(editor.world_revision())
    {
        return;
    }
    let stroke = |id, kind, points, width| Stroke {
        id: StrokeId(id),
        kind,
        points,
        width,
        height: 2.8,
    };
    let op = match phase {
        0 => Operation::Edit(EditCommand::Stroke(StrokeEdit::Create(stroke(
            1,
            StrokeKind::Wall,
            vec![Point { x: -14., z: -13. }, Point { x: 0., z: -13. }],
            0.45,
        )))),
        1 => {
            if editor.stroke(StrokeId(1)).is_none() {
                return;
            }
            Operation::Edit(EditCommand::Stroke(StrokeEdit::Create(stroke(
                2,
                StrokeKind::Path,
                vec![Point { x: -7., z: -14.5 }, Point { x: -7., z: -9. }],
                1.8,
            ))))
        }
        2 => {
            if editor.stroke(StrokeId(2)).is_none() {
                return;
            }
            Operation::Edit(EditCommand::Stroke(StrokeEdit::Delete {
                id: StrokeId(2),
                expected_revision: editor.stroke_revision(),
            }))
        }
        _ => {
            if editor.stroke(StrokeId(2)).is_some() {
                return;
            }
            Operation::Edit(EditCommand::Undo)
        }
    };
    if world.resource_mut::<CommandInbox>().submit(op).is_ok() {
        world.resource_mut::<Capture>().review_phase += 1;
        world.resource_mut::<DesktopEditor>().status =
            "墙路验收：画路自动开拱，删除恢复，撤销重新开拱。".into();
    }
}

/// 真实 GPU 管线的可重复场景：原子创建、道路删除、撤销恢复及一致显示交接。
fn prepare_context_review(world: &mut World) {
    use garden_application::{EditCommand, StrokeEdit, context::ContextEdit};
    use garden_domain::{context::*, strokes::*, terrain::*};
    use std::sync::Arc;
    let capture = world.resource::<Capture>();
    if !capture.context_review || capture.review_phase >= 3 {
        return;
    }
    let phase = capture.review_phase;
    if world.resource::<RenderStats>().buildings != 6
        || !world.resource::<garden_bevy::PendingTargets>().is_empty()
        || world
            .resource::<garden_bevy::context::ContextProjection>()
            .busy
    {
        return;
    }
    let editor = world.resource::<EditorState>().editor();
    if world.resource::<crate::strokes::StrokeDisplay>().revision != Some(editor.world_revision()) {
        return;
    }
    let point = |x, z| Point { x, z };
    let op = if phase == 0 {
        let mut terrain = TerrainDocument::default();
        for _ in 0..4 {
            terrain = terrain
                .brushed(BrushSample {
                    center: point(-1., 6.),
                    radius: 2.,
                    amount: 0.2,
                    kind: BrushKind::Raise,
                })
                .unwrap();
        }
        // 另一个露出地面的起伏用于视觉验收；主丘仍位于建筑根轮廓下测试地基。
        for _ in 0..4 {
            terrain = terrain
                .brushed(BrushSample {
                    center: point(3., 3.),
                    radius: 2.,
                    amount: 0.2,
                    kind: BrushKind::Raise,
                })
                .unwrap();
        }
        let mut edits = vec![ContextEdit::Terrain(Arc::new(terrain))];
        for (id, points, height, width) in [
            (1, vec![point(-10., -13.), point(0., -13.)], 2.8, 0.4),
            (2, vec![point(-15., -1.), point(-15., 6.)], 0., 1.4),
            (3, vec![point(-15., -1.), point(-9., -1.)], 0., 0.8),
            (4, vec![point(-18., -13.), point(-10.1, -13.)], 1.0, 0.4),
            (5, vec![point(-7., -14.5), point(-7., -9.5)], 0., 1.4),
        ] {
            let stroke = Stroke {
                id: StrokeId(id),
                kind: StrokeKind::Path,
                points,
                width,
                height,
            };
            let mut intent = LinearIntent::legacy(&stroke);
            intent.mode = StructureMode::Auto;
            edits.push(ContextEdit::Stroke { stroke, intent });
        }
        for (id, building, along) in [(1, 2, 1.3), (2, 2, 2.3), (3, 4, 3.)] {
            let b = editor.get(BuildingId::new(building).unwrap()).unwrap();
            edits.push(ContextEdit::PutOpening(OpeningIntent {
                id: OpeningId(id),
                host: WallAnchor {
                    building: b.id(),
                    block: b.blocks()[0].id,
                    face: WallFace::Front,
                },
                along,
                elevation: 1.4,
                width: 0.9,
                height: 1.2,
                style: 0,
            }));
        }
        Operation::Edit(EditCommand::Context(ContextEdit::Batch(edits)))
    } else if phase == 1 {
        assert!(
            world
                .resource::<garden_bevy::context::ContextProjection>()
                .semantic
                .diagnostics
                .contains(&garden_generation::context::Diagnostic::Sleeping(
                    OpeningId(3)
                ))
        );
        Operation::Edit(EditCommand::Stroke(StrokeEdit::Delete {
            id: StrokeId(2),
            expected_revision: editor.stroke_revision(),
        }))
    } else {
        assert!(
            !world
                .resource::<garden_bevy::context::ContextProjection>()
                .semantic
                .diagnostics
                .contains(&garden_generation::context::Diagnostic::Sleeping(
                    OpeningId(3)
                ))
        );
        Operation::Edit(EditCommand::Undo)
    };
    if world.resource_mut::<CommandInbox>().submit(op).is_ok() {
        world.resource_mut::<Capture>().review_phase += 1;
        world.resource_mut::<DesktopEditor>().status =
            "上下文验收：三态、路口、拱门、连窗、休眠恢复与起伏地基。".into();
    }
}
fn setup_scene(
    mut commands: Commands,
    mut inbox: ResMut<CommandInbox>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut images: ResMut<Assets<Image>>,
    mut capture: ResMut<Capture>,
) {
    let cases = [
        (3.0, 3.0, Roof::Gabled, Facade::Plaster),
        (6.0, 6.0, Roof::Gabled, Facade::Timber),
        (10.0, 12.0, Roof::Gabled, Facade::Stone),
        (6.0, 3.0, Roof::Hipped, Facade::Plaster),
        (6.0, 6.0, Roof::Flat, Facade::Stone),
        (10.0, 3.0, Roof::Gabled, Facade::Plaster),
    ];
    let mut buildings = Vec::new();
    for (i, (width, height, roof, facade)) in cases.into_iter().enumerate() {
        let id = BuildingId::new(i as u64 + 1).unwrap();
        let mut d = sample_building(id, width, height);
        d.placement.x = (i % 3) as f64 * 14.0 - 18.0;
        d.placement.z = (i / 3) as f64 * 13.0 - 10.0;
        d.blocks[0].roof_intent = roof;
        d.blocks[0].facade = facade;
        if i == 5 {
            d.blocks.push(BlockDraft {
                id: BlockId::new(2).unwrap(),
                parent: Some(BlockId::new(1).unwrap()),
                footprint: Rect {
                    x: 2.0,
                    z: 1.0,
                    width: 5.0,
                    depth: 4.0,
                },
                height: 6.0,
                stories: Stories::Locked(2),
                roof_intent: Roof::Hipped,
                facade: Facade::Timber,
            });
        }
        buildings.push(Building::try_new(d).unwrap());
    }
    // Fixtures are a loaded scene, not six player edits in undo history.
    inbox.submit(Operation::ReplaceScene(buildings)).unwrap();
    commands.spawn((
        crate::context_tools::GroundBase,
        Mesh3d(meshes.add(Cuboid::new(53.0, 0.35, 34.0))),
        MeshMaterial3d(materials.add(StandardMaterial {
            base_color: Color::srgb_u8(135, 157, 115),
            perceptual_roughness: 0.95,
            ..default()
        })),
        Transform::from_xyz(0.0, -0.2, 0.0),
    ));
    commands.spawn((
        DirectionalLight {
            illuminance: 8500.0,
            color: Color::srgb_u8(255, 244, 226),
            shadow_maps_enabled: true,
            ..default()
        },
        Transform::from_xyz(-20.0, 35.0, -25.0).looking_at(Vec3::ZERO, Vec3::Y),
    ));
    // Low-intensity sky fill preserves form on the shaded facade without a
    // second shadow map. These are desktop look-dev values, not mobile budgets.
    commands.spawn((
        DirectionalLight {
            illuminance: 1800.0,
            color: Color::srgb_u8(205, 220, 239),
            shadow_maps_enabled: false,
            ..default()
        },
        Transform::from_xyz(25.0, 18.0, 20.0).looking_at(Vec3::ZERO, Vec3::Y),
    ));
    let (eye, target) = if capture.context_review {
        (Vec3::new(10., 26., -30.), Vec3::new(-9., 1., -1.))
    } else if capture.closeup {
        (Vec3::new(-9.0, 7.0, -17.0), Vec3::new(-16.5, 2.0, -7.0))
    } else {
        (Vec3::new(34.0, 32.0, -46.0), Vec3::new(0.0, 3.0, 0.0))
    };
    let orbit = OrbitCamera::from_view(eye, target).unwrap();
    let mut camera = commands.spawn((
        Camera3d::default(),
        IsDefaultUiCamera,
        orbit.transform(),
        orbit,
        AmbientLight {
            color: Color::srgb_u8(207, 220, 234),
            brightness: 650.0,
            ..default()
        },
    ));
    if capture.path.is_some() {
        let image = images.add(Image::new_target_texture(
            1440,
            960,
            TextureFormat::Rgba8UnormSrgb,
            None,
        ));
        camera.insert(RenderTarget::Image(image.clone().into()));
        capture.image = Some(image);
    }
}
fn setup_ui(
    mut commands: Commands,
    capture: Res<Capture>,
    server: Res<AssetServer>,
    mut font: ResMut<UiFont>,
) {
    if capture.closeup || capture.context_review {
        return;
    }
    let handle = server.load(ui::FONT_PATH);
    ui::spawn_toolbar(&mut commands, &handle);
    font.0 = Some(handle);
}
fn prepare_review(world: &mut World) {
    let (review, phase) = {
        let capture = world.resource::<Capture>();
        (capture.review, capture.review_phase)
    };
    if world.resource::<Capture>().tools_review && phase > 0 {
        prepare_tools_review(world, phase);
        return;
    }
    if !review || phase >= 3 {
        return;
    }
    let count = world.resource::<RenderStats>().buildings;
    if phase == 0 && count == 6 {
        world.resource_scope(|world, mut state: Mut<DesktopEditor>| {
            state.tools.set_tool(Tool::Build);
            state
                .tools
                .begin(
                    world.resource::<EditorState>().editor(),
                    GroundPoint { x: -24.0, z: 9.0 },
                )
                .unwrap();
            state
                .tools
                .update(GroundPoint { x: -20.0, z: 14.0 })
                .unwrap();
            let id = state.tools.candidate().unwrap().id();
            let command = state.tools.finish().unwrap().unwrap();
            state.tools.select(Some(id));
            state.submit(command, &mut world.resource_mut::<CommandInbox>());
        });
        world.resource_mut::<Capture>().review_phase = 1;
    } else if phase == 1 && count == 7 {
        assert_eq!(world.resource::<EditorState>().editor().undo_count(), 1);
        // Exercise the real generation/upload/pipeline-guard/crossfade path,
        // not only a static asset screenshot. A separate command keeps Undo.
        let id = BuildingId::new(7).unwrap();
        let footprint = world
            .resource::<EditorState>()
            .editor()
            .get(id)
            .unwrap()
            .blocks()[0]
            .footprint;
        world.resource_scope(|world, mut state: Mut<DesktopEditor>| {
            state.tools.select(Some(id));
            state.submit(
                garden_application::EditCommand::Edit {
                    building: id,
                    edit: garden_domain::BuildingEdit::Resize {
                        block: BlockId::new(1).unwrap(),
                        footprint,
                        height: 6.,
                    },
                },
                &mut world.resource_mut::<CommandInbox>(),
            );
        });
        world.resource_mut::<Capture>().review_phase = 2;
    } else if phase == 2
        && world
            .resource::<garden_bevy::PendingTargets>()
            .get(BuildingId::new(7).unwrap())
            .is_none()
    {
        assert_eq!(world.resource::<EditorState>().editor().undo_count(), 2);
        world.resource_scope(|world, mut state: Mut<DesktopEditor>| {
            state.tools.set_tool(Tool::Resize);
            state
                .tools
                .begin(
                    world.resource::<EditorState>().editor(),
                    GroundPoint { x: -24.0, z: 9.0 },
                )
                .unwrap();
            state
                .tools
                .update(GroundPoint { x: -23.0, z: 10.0 })
                .unwrap();
            state.status = "房屋 #7 已完成增高与过渡；青色尺寸预览尚未提交。".into();
        });
        let mut capture = world.resource_mut::<Capture>();
        capture.review_phase = 3;
        capture.ready_frames = 0;
    }
}

// A reproducible GPU integration fixture, not an OS mouse replay. Pointer/UI
// transactions have separate headless camera-ray tests in editor.rs.
fn prepare_tools_review(world: &mut World, phase: u8) {
    let id = BuildingId::new(7).unwrap();
    if world.resource::<EditorState>().editor().get(id).is_none()
        || phase >= 5
        || world
            .resource::<garden_bevy::PendingTargets>()
            .get(id)
            .is_some()
    {
        return;
    }
    world.resource_scope(|world, mut state: Mut<DesktopEditor>| {
        let editor = world.resource::<EditorState>().editor();
        let command = match phase {
            1 => {
                state.tools.select(Some(id));
                state.tools.add_upper(editor).unwrap()
            }
            2 => {
                state.tools.set_tool(Tool::Height);
                state
                    .tools
                    .begin(editor, GroundPoint { x: -22., z: 11.5 })
                    .unwrap();
                state.tools.update_height(1.5).unwrap();
                state.tools.finish().unwrap().unwrap()
            }
            3 => {
                state.tools.set_tool(Tool::Rotate);
                state
                    .tools
                    .begin(editor, GroundPoint { x: -18., z: 11.5 })
                    .unwrap();
                state
                    .tools
                    .update(GroundPoint {
                        x: -22. + 4. / 2f64.sqrt(),
                        z: 11.5 - 4. / 2f64.sqrt(),
                    })
                    .unwrap();
                state.tools.finish().unwrap().unwrap()
            }
            4 => {
                assert_eq!(editor.undo_count(), 4);
                assert_eq!(editor.get(id).unwrap().blocks().len(), 2);
                state.tools.set_tool(Tool::Select);
                state.status = "已建造退台双体块房屋、调整上层高度并旋转；彩色点可拖动。".into();
                return;
            }
            _ => return,
        };
        state.submit(command, &mut world.resource_mut::<CommandInbox>());
    });
    let mut capture = world.resource_mut::<Capture>();
    capture.review_phase = phase + 1;
    capture.ready_frames = 0;
}
fn scene_pending_ready(
    editor: &EditorState,
    strokes: &crate::strokes::StrokeDisplay,
    stats: &RenderStats,
) -> bool {
    strokes.revision == Some(editor.editor().world_revision())
        && stats.buildings == editor.editor().objects().count()
}
#[derive(SystemParam)]
struct CaptureScene<'w> {
    stats: Res<'w, RenderStats>,
    editor: Res<'w, EditorState>,
    art: Res<'w, ArtLoadState>,
    server: Res<'w, AssetServer>,
    font: Res<'w, UiFont>,
    strokes: Res<'w, crate::strokes::StrokeDisplay>,
}
fn capture(
    mut commands: Commands,
    mut capture: ResMut<Capture>,
    mut exit: MessageWriter<AppExit>,
    meshes: Query<(&GlobalTransform, &ViewVisibility), With<Mesh3d>>,
    cameras: Query<(&Camera, &GlobalTransform)>,
    scene: CaptureScene,
) {
    let CaptureScene {
        stats,
        editor,
        art,
        server,
        font,
        strokes,
    } = scene;
    let Some(path) = capture.path.clone() else {
        return;
    };
    if capture.started.elapsed() > Duration::from_secs(120) {
        error!("capture timeout");
        exit.write(AppExit::error());
        return;
    }
    if capture.requested {
        return;
    }
    if capture.review && capture.review_phase < if capture.tools_review { 5 } else { 3 } {
        return;
    }
    if capture.wall_review
        && (capture.review_phase < 4
            || editor
                .editor()
                .stroke(garden_domain::strokes::StrokeId(2))
                .is_none()
            || strokes.revision != Some(editor.editor().world_revision()))
    {
        return;
    }
    if capture.context_review
        && (capture.review_phase < 3 || !scene_pending_ready(&editor, &strokes, &stats))
    {
        return;
    }
    if stats.buildings != editor.editor().objects().count() || stats.buildings == 0 {
        return;
    }
    if art
        .textures
        .iter()
        .any(|h| !server.is_loaded_with_dependencies(h.id()))
    {
        return;
    }
    if font
        .0
        .as_ref()
        .is_some_and(|h| !server.is_loaded_with_dependencies(h.id()))
    {
        return;
    }
    capture.ready_frames += 1;
    // Give shader pipelines and shadow resources time to finish compiling.
    if capture.ready_frames < 900 {
        return;
    }
    capture.requested = true;
    println!(
        "Capture {stats:?}; visible meshes={}/{}; cameras={:?}",
        meshes.iter().filter(|(_, v)| v.get()).count(),
        meshes.iter().count(),
        cameras
            .iter()
            .map(|(c, t)| (c.is_active, t.translation()))
            .collect::<Vec<_>>()
    );
    let image = capture.image.clone().expect("capture render target");
    commands
        .spawn(Screenshot::image(image))
        .observe(save_to_disk(path))
        .observe(
            |_: On<ScreenshotCaptured>, mut exit: MessageWriter<AppExit>| {
                exit.write(AppExit::Success);
            },
        );
}
