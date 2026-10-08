//! Desktop proportion lab. --capture PATH renders offscreen and exits after saving.
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
use garden_application::EditCommand;
use garden_bevy::{CommandInbox, EditorState, GardenPlugin, Operation};
use garden_domain::{BlockDraft, BlockId, BuildingId, Facade, Roof, Stories, sample_building};
use garden_geometry::Rect;
use garden_presentation::{
    camera::{BlocksWorldInput, CameraAction, OrbitCamera, OrbitCameraPlugin},
    catalog::ArtCatalog,
    editor::{DesktopEditorPlugin, EditorAction, EditorStatus},
    renderer::{PresentationPlugin, RenderStats},
};
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
}
fn main() {
    let mut args = std::env::args().skip(1);
    let path = match args.next().as_deref() {
        None => None,
        Some("--capture") => Some(PathBuf::from(
            args.next().expect("--capture requires a PNG path"),
        )),
        _ => panic!("usage: showcase [--capture PATH.png]"),
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
    if path.is_some() {
        app.add_plugins(
            DefaultPlugins
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
        app.add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: "Tiny Garden | Desktop Build Greybox".into(),
                resolution: (1440, 960).into(),
                ..default()
            }),
            ..default()
        }));
    }
    app.insert_resource(ClearColor(Color::srgb_u8(224, 229, 215)))
        .insert_resource(Capture {
            path,
            image: None,
            started: Instant::now(),
            ready_frames: 0,
            requested: false,
        })
        .add_plugins((
            GardenPlugin::default()
                .with_geometry(art.geometry())
                .unwrap(),
            PresentationPlugin::new(art),
            OrbitCameraPlugin,
            DesktopEditorPlugin,
        ))
        .add_systems(Startup, (setup_scene, setup_ui))
        .add_systems(Update, capture)
        .run();
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
        inbox
            .submit(Operation::Edit(EditCommand::Create(d)))
            .unwrap();
    }
    commands.spawn((
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
            shadow_maps_enabled: true,
            ..default()
        },
        Transform::from_xyz(-15.0, 30.0, 15.0).looking_at(Vec3::ZERO, Vec3::Y),
    ));
    let orbit =
        OrbitCamera::from_view(Vec3::new(40.0, 38.0, 52.0), Vec3::new(0.0, 3.0, 0.0)).unwrap();
    let mut camera = commands.spawn((
        Camera3d::default(),
        IsDefaultUiCamera,
        orbit.transform(),
        orbit,
        AmbientLight {
            color: Color::srgb_u8(207, 220, 234),
            brightness: 220.0,
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
fn setup_ui(mut commands: Commands) {
    commands.spawn((BlocksWorldInput, bevy::ui::RelativeCursorPosition::default(), Node {position_type:PositionType::Absolute,top:px(20),left:px(24),flex_direction:FlexDirection::Column,row_gap:px(8),..default()},
        BackgroundColor(Color::srgba_u8(244,238,223,235))))
        .with_children(|parent| {
            parent.spawn((Text::new("TINY GARDEN | DESKTOP GREYBOX"),TextFont {font_size:bevy::text::FontSize::Px(24.0),..default()},TextColor(Color::srgb_u8(75,81,72))));
            parent.spawn((Text::new("Left-drag: build / move | Right-click: cancel drag\nRight-drag: orbit | Wheel: zoom | Prototype artwork"),TextFont {font_size:bevy::text::FontSize::Px(17.0),..default()},TextColor(Color::srgb_u8(75,81,72))));
            parent.spawn((Node {column_gap:px(8),..default()},)).with_children(|row| {
                for (action,label) in [(EditorAction::Select,"Select"),(EditorAction::Build,"Build"),(EditorAction::Move,"Move"),(EditorAction::Cancel,"Cancel"),(EditorAction::Retry,"Retry"),(EditorAction::Undo,"Undo"),(EditorAction::Redo,"Redo")] {
                    row.spawn((Button,action,Node {padding:UiRect::all(px(12)),min_height:px(48),..default()},BackgroundColor(Color::srgb_u8(108,132,99))))
                        .with_children(|button| {button.spawn((Text::new(label),TextFont {font_size:bevy::text::FontSize::Px(16.0),..default()},TextColor(Color::WHITE)));});
                }
            });
            parent.spawn(Node {column_gap:px(8),..default()}).with_children(|row| {
                for (action,label) in [(EditorAction::Roof,"Roof"),(EditorAction::HeightUp,"Height +"),(EditorAction::HeightDown,"Height -"),(EditorAction::Delete,"Delete selected")] {
                    row.spawn((Button,action,Node {padding:UiRect::all(px(10)),min_height:px(44),..default()},BackgroundColor(Color::srgb_u8(108,132,99))))
                        .with_children(|button| {button.spawn((Text::new(label),TextFont {font_size:bevy::text::FontSize::Px(16.0),..default()},TextColor(Color::WHITE)));});
                }
            });
            parent.spawn(Node {column_gap:px(8),..default()}).with_children(|row| {
                for (action,label) in [(CameraAction::Left,"View <"),(CameraAction::Right,"View >"),(CameraAction::Up,"Tilt +"),(CameraAction::Down,"Tilt -"),(CameraAction::Near,"Zoom +"),(CameraAction::Far,"Zoom -"),(CameraAction::Home,"Reset view")] {
                    row.spawn((Button,action,Node {padding:UiRect::all(px(10)),min_height:px(44),..default()},BackgroundColor(Color::srgb_u8(108,132,99))))
                        .with_children(|button| {button.spawn((Text::new(label),TextFont {font_size:bevy::text::FontSize::Px(16.0),..default()},TextColor(Color::WHITE)));});
                }
            });
            parent.spawn((EditorStatus,Text::new("Tool: Select | Selected: none"),TextFont {font_size:bevy::text::FontSize::Px(15.0),..default()},TextColor(Color::srgb_u8(75,81,72)),Node {max_width:px(632),..default()}));
        });
}
fn capture(
    mut commands: Commands,
    mut capture: ResMut<Capture>,
    stats: Res<RenderStats>,
    mut exit: MessageWriter<AppExit>,
    meshes: Query<(&GlobalTransform, &ViewVisibility), With<Mesh3d>>,
    cameras: Query<(&Camera, &GlobalTransform)>,
    editor: Res<EditorState>,
) {
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
    if stats.buildings != editor.editor().objects().count() || stats.buildings == 0 {
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
