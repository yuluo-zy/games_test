//! Constant-size low-cost candidate pool. Never scales the detailed house or
//! creates domain objects, generation jobs, history entries or pickable meshes.
use crate::editor::DesktopEditor;
use bevy::{
    asset::RenderAssetUsages,
    light::{NotShadowCaster, NotShadowReceiver},
    prelude::*,
    render::render_resource::PrimitiveTopology,
};
use garden_bevy::{GardenSet, PendingTargets};
use garden_domain::{Building, MAX_BLOCKS, Roof};
use garden_generation::mesh::GeometryProfile;

pub struct PreviewPlugin(pub GeometryProfile);
impl Plugin for PreviewPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(PreviewProfile(self.0))
            .add_systems(Startup, setup)
            .add_systems(Update, show.after(GardenSet::Commit));
    }
}
#[derive(Resource)]
struct PreviewProfile(GeometryProfile);
#[derive(Resource)]
struct PreviewAssets {
    body: Handle<Mesh>,
    gabled: Handle<Mesh>,
    hipped: Handle<Mesh>,
    valid: Handle<StandardMaterial>,
    invalid: Handle<StandardMaterial>,
}
#[derive(Component)]
struct PreviewPart {
    slot: usize,
    roof: bool,
}

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    let body = meshes.add(Cuboid::new(1., 1., 1.));
    let gabled = meshes.add(roof_mesh(false));
    let hipped = meshes.add(roof_mesh(true));
    let material = |color| StandardMaterial {
        base_color: color,
        alpha_mode: AlphaMode::Blend,
        unlit: true,
        cull_mode: None,
        ..default()
    };
    let valid = materials.add(material(Color::srgba_u8(58, 213, 219, 45)));
    let invalid = materials.add(material(Color::srgba_u8(240, 91, 73, 60)));
    for slot in 0..MAX_BLOCKS {
        for roof in [false, true] {
            commands.spawn((
                PreviewPart { slot, roof },
                Mesh3d(body.clone()),
                MeshMaterial3d(valid.clone()),
                Visibility::Hidden,
                Transform::default(),
                NotShadowCaster,
                NotShadowReceiver,
            ));
        }
    }
    commands.insert_resource(PreviewAssets {
        body,
        gabled,
        hipped,
        valid,
        invalid,
    });
}

// A gabled prism / hip pyramid, no holes, templates, tangents or per-frame mesh.
fn roof_mesh(hipped: bool) -> Mesh {
    let a = Vec3::new(-0.5, 0., -0.5);
    let b = Vec3::new(0.5, 0., -0.5);
    let c = Vec3::new(0.5, 0., 0.5);
    let d = Vec3::new(-0.5, 0., 0.5);
    let e = Vec3::new(0., 1., if hipped { 0. } else { -0.5 });
    let f = Vec3::new(0., 1., 0.5);
    let triangles = if hipped {
        vec![[a, e, b], [b, e, c], [c, e, d], [d, e, a]]
    } else {
        vec![
            [a, e, b],
            [d, c, f],
            [a, d, f],
            [a, f, e],
            [b, e, f],
            [b, f, c],
        ]
    };
    let mut positions = Vec::new();
    let mut normals = Vec::new();
    for [a, b, c] in triangles {
        let normal = (b - a).cross(c - a).normalize().to_array();
        positions.extend([a.to_array(), b.to_array(), c.to_array()]);
        normals.extend([normal; 3]);
    }
    let uvs = vec![[0., 0.]; positions.len()];
    Mesh::new(
        PrimitiveTopology::TriangleList,
        RenderAssetUsages::RENDER_WORLD,
    )
    .with_inserted_attribute(Mesh::ATTRIBUTE_POSITION, positions)
    .with_inserted_attribute(Mesh::ATTRIBUTE_NORMAL, normals)
    .with_inserted_attribute(Mesh::ATTRIBUTE_UV_0, uvs)
}

#[derive(Clone, Copy)]
struct Shape {
    body: Transform,
    roof: Transform,
    kind: Roof,
}
fn shapes(building: &Building, profile: GeometryProfile) -> Vec<Shape> {
    let p = building.placement();
    let root = Transform::from_xyz(p.x as f32, p.elevation as f32, p.z as f32)
        .with_rotation(Quat::from_rotation_y(p.yaw as f32));
    building
        .blocks()
        .iter()
        .map(|block| {
            let mut base = 0.;
            let mut parent = block.parent;
            while let Some(id) = parent {
                let support = building.block(id).unwrap();
                base += support.height;
                parent = support.parent;
            }
            let r = block.footprint;
            let kind = if building.blocks().iter().any(|b| b.parent == Some(block.id)) {
                Roof::Flat
            } else {
                block.roof_intent
            };
            let x = (r.x + r.width * 0.5) as f32;
            let z = (r.z + r.depth * 0.5) as f32;
            let height = block.height as f32;
            let body = root.mul_transform(
                Transform::from_xyz(x, base as f32 + height * 0.5, z).with_scale(Vec3::new(
                    r.width as f32,
                    height,
                    r.depth as f32,
                )),
            );
            let roof = if kind == Roof::Flat {
                Transform::from_xyz(x, (base + block.height) as f32 + 0.04, z)
                    .with_scale(Vec3::new(r.width as f32, 0.08, r.depth as f32))
            } else {
                let width = r.width + 2. * profile.eave;
                let depth = r.depth + 2. * profile.eave;
                let rise = (width.min(depth) * 0.5 * profile.roof_pitch_degrees.to_radians().tan())
                    .min(profile.max_roof_rise);
                let local = Transform::from_xyz(x, (base + block.height) as f32, z);
                if depth >= width {
                    local.with_scale(Vec3::new(width as f32, rise as f32, depth as f32))
                } else {
                    local
                        .with_rotation(Quat::from_rotation_y(std::f32::consts::FRAC_PI_2))
                        .with_scale(Vec3::new(depth as f32, rise as f32, width as f32))
                }
            };
            Shape {
                body,
                roof: root.mul_transform(roof),
                kind,
            }
        })
        .collect()
}

type PreviewQuery<'a> = (
    &'a PreviewPart,
    &'a mut Transform,
    &'a mut Mesh3d,
    &'a mut MeshMaterial3d<StandardMaterial>,
    &'a mut Visibility,
);

fn show(
    state: Res<DesktopEditor>,
    pending: Res<PendingTargets>,
    assets: Res<PreviewAssets>,
    profile: Res<PreviewProfile>,
    mut parts: Query<PreviewQuery<'_>>,
) {
    let target = state.tools.selected().and_then(|id| pending.get(id));
    let building = if state.tools.dragging() {
        state.tools.candidate()
    } else {
        target.map(|t| t.building.as_ref())
    };
    let valid = if state.tools.dragging() {
        state.tools.valid()
    } else {
        target.is_some_and(|t| !t.failed)
    };
    let shapes = building.map(|b| shapes(b, profile.0)).unwrap_or_default();
    for (part, mut transform, mut mesh, mut material, mut visibility) in &mut parts {
        let Some(shape) = shapes.get(part.slot) else {
            if *visibility != Visibility::Hidden {
                *visibility = Visibility::Hidden;
            }
            continue;
        };
        if *visibility != Visibility::Visible {
            *visibility = Visibility::Visible;
        }
        let pose = if part.roof { shape.roof } else { shape.body };
        if *transform != pose {
            *transform = pose;
        }
        let handle = if !part.roof || shape.kind == Roof::Flat {
            &assets.body
        } else if shape.kind == Roof::Gabled {
            &assets.gabled
        } else {
            &assets.hipped
        };
        if mesh.0 != *handle {
            mesh.0 = handle.clone();
        }
        let handle = if valid {
            &assets.valid
        } else {
            &assets.invalid
        };
        if material.0 != *handle {
            material.0 = handle.clone();
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use garden_application::tools::{GroundPoint, Tool};
    use garden_bevy::{CommandInbox, EditorState, GardenPlugin, Operation};
    use garden_domain::{BuildingId, sample_building};
    #[derive(Resource, Default)]
    struct Changes(usize);
    type TrackedPreview<'a> = (
        Ref<'a, Mesh3d>,
        Ref<'a, MeshMaterial3d<StandardMaterial>>,
        Ref<'a, Transform>,
        Ref<'a, Visibility>,
    );
    #[test]
    fn candidate_pool_is_constant_and_hands_off_without_domain_or_history_writes() {
        let mut app = App::new();
        app.init_resource::<Assets<Mesh>>()
            .init_resource::<Assets<StandardMaterial>>()
            .init_resource::<DesktopEditor>()
            .add_plugins((
                GardenPlugin::default(),
                PreviewPlugin(GeometryProfile::default()),
            ))
            .init_resource::<Changes>()
            .add_systems(
                PostUpdate,
                |parts: Query<TrackedPreview<'_>, With<PreviewPart>>,
                 mut changes: ResMut<Changes>| {
                    changes.0 = parts
                        .iter()
                        .filter(|(mesh, material, pose, visibility)| {
                            mesh.is_changed()
                                || material.is_changed()
                                || pose.is_changed()
                                || visibility.is_changed()
                        })
                        .count();
                },
            );
        app.update();
        let mesh_count = app.world().resource::<Assets<Mesh>>().len();
        assert_eq!(mesh_count, 3);
        app.world_mut()
            .resource_scope(|world, mut state: Mut<DesktopEditor>| {
                state.tools.set_tool(Tool::Build);
                state
                    .tools
                    .begin(
                        world.resource::<EditorState>().editor(),
                        GroundPoint { x: 0., z: 0. },
                    )
                    .unwrap();
                state.tools.update(GroundPoint { x: 4., z: 5. }).unwrap();
            });
        app.update();
        let visible = |world: &mut World| {
            world
                .query_filtered::<&Visibility, With<PreviewPart>>()
                .iter(world)
                .filter(|v| **v != Visibility::Hidden)
                .count()
        };
        assert_eq!(visible(app.world_mut()), 2);
        assert_eq!(
            app.world().resource::<EditorState>().editor().undo_count(),
            0
        );
        app.update();
        assert_eq!(
            app.world().resource::<Changes>().0,
            0,
            "a stable preview must not dirty render components every frame"
        );
        app.world_mut()
            .resource_scope(|world, mut state: Mut<DesktopEditor>| {
                let command = state.tools.finish().unwrap().unwrap();
                state.tools.select(Some(BuildingId::new(1).unwrap()));
                state.submit(command, &mut world.resource_mut::<CommandInbox>());
            });
        app.update();
        assert_eq!(visible(app.world_mut()), 2);
        let ticket = app
            .world()
            .resource::<EditorState>()
            .editor()
            .latest_ticket(BuildingId::new(1).unwrap())
            .unwrap();
        app.world_mut()
            .resource_mut::<PendingTargets>()
            .displayed(ticket);
        app.update();
        assert_eq!(visible(app.world_mut()), 0);
        app.world_mut()
            .resource_mut::<CommandInbox>()
            .submit(Operation::ReplaceScene(vec![
                Building::try_new(sample_building(BuildingId::new(2).unwrap(), 6., 3.)).unwrap(),
            ]))
            .unwrap();
        app.update();
        assert_eq!(visible(app.world_mut()), 0);
        assert_eq!(app.world().resource::<Assets<Mesh>>().len(), mesh_count);
        let world = app.world_mut();
        assert_eq!(
            world
                .query_filtered::<Entity, With<PreviewPart>>()
                .iter(world)
                .count(),
            MAX_BLOCKS * 2
        );
    }
    #[test]
    fn simple_preview_roofs_obey_support_and_position_without_scaling_attachments() {
        use garden_application::{EditCommand, Editor, tools::ToolController};
        let mut editor = Editor::new(32);
        let id = BuildingId::new(1).unwrap();
        editor
            .execute(EditCommand::Create(sample_building(id, 10., 3.)))
            .unwrap();
        let mut tools = ToolController::default();
        tools.select(Some(id));
        editor.execute(tools.add_upper(&editor).unwrap()).unwrap();
        let shapes = shapes(editor.get(id).unwrap(), GeometryProfile::default());
        assert_eq!(shapes.len(), 2);
        assert_eq!(shapes[0].kind, Roof::Flat);
        assert_eq!(shapes[1].body.translation.y, 4.5);
        assert_eq!(shapes[1].kind, Roof::Gabled);
    }
}
