//! Bounded upload and explicit mesh ownership. One child per material, not window.
use crate::catalog::ArtCatalog;
use bevy::ecs::system::SystemParam;
use bevy::{
    asset::RenderAssetUsages, mesh::Indices, prelude::*, render::render_resource::PrimitiveTopology,
};
use garden_bevy::{BuildingProjection, EditorState, GardenSet, ProjectionUpdates};
use garden_generation::BuildingLayout;
use garden_generation::mesh::{BuildingMesh, MaterialKey, MeshData};
use std::{collections::BTreeMap, sync::Arc};

/// CPU pick data follows the uploaded picture, not a newer pending projection.
#[derive(Component)]
pub struct DisplayedBuilding {
    pub ticket: garden_application::JobTicket,
    pub target: Arc<BuildingLayout>,
    pub mesh: Arc<BuildingMesh>,
    pub min: Vec3,
    pub max: Vec3,
}

pub struct PresentationPlugin {
    catalog: ArtCatalog,
}
impl PresentationPlugin {
    pub fn new(catalog: ArtCatalog) -> Self {
        Self { catalog }
    }
}
#[derive(Resource)]
struct Theme(ArtCatalog);
#[derive(Resource, Default)]
struct Palette(BTreeMap<MaterialKey, Handle<StandardMaterial>>);
#[derive(Resource, Default)]
struct Residency(BTreeMap<Entity, ResidentMesh>);
struct ResidentMesh {
    children: Vec<Entity>,
    handles: Vec<Handle<Mesh>>,
    triangles: usize,
}

#[derive(Resource, Default, Debug)]
pub struct RenderStats {
    pub buildings: usize,
    pub batches: usize,
    pub triangles: usize,
    pub uploads: u64,
}

impl Plugin for PresentationPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(Theme(self.catalog.clone()))
            .init_resource::<Palette>()
            .init_resource::<Residency>()
            .init_resource::<RenderStats>()
            .add_systems(Startup, setup_palette)
            .add_systems(
                Update,
                (release_removed, upload).chain().after(GardenSet::Collect),
            );
    }
}
fn setup_palette(
    theme: Res<Theme>,
    mut palette: ResMut<Palette>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    for key in MaterialKey::ALL {
        let definition = theme.0.material(key);
        let [r, g, b] = definition.base_color_srgb;
        palette.0.insert(
            key,
            materials.add(StandardMaterial {
                base_color: Color::srgb_u8(r, g, b),
                perceptual_roughness: definition.roughness,
                metallic: 0.0,
                ..default()
            }),
        );
    }
}
pub fn to_bevy_mesh(data: &MeshData) -> Mesh {
    Mesh::new(
        PrimitiveTopology::TriangleList,
        RenderAssetUsages::RENDER_WORLD,
    )
    .with_inserted_attribute(Mesh::ATTRIBUTE_POSITION, data.positions.clone())
    .with_inserted_attribute(Mesh::ATTRIBUTE_NORMAL, data.normals.clone())
    .with_inserted_attribute(Mesh::ATTRIBUTE_UV_0, data.uvs.clone())
    .with_inserted_indices(Indices::U32(data.indices.clone()))
}
fn release_removed(
    mut removed: RemovedComponents<BuildingProjection>,
    mut residency: ResMut<Residency>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut stats: ResMut<RenderStats>,
) {
    for entity in removed.read() {
        if let Some(old) = residency.0.remove(&entity) {
            stats.batches -= old.handles.len();
            stats.triangles -= old.triangles;
            for handle in old.handles {
                meshes.remove(handle.id());
            }
        }
    }
    stats.buildings = residency.0.len();
}
#[derive(SystemParam)]
struct RenderAssets<'w> {
    palette: Res<'w, Palette>,
    residency: ResMut<'w, Residency>,
    meshes: ResMut<'w, Assets<Mesh>>,
    stats: ResMut<'w, RenderStats>,
}
fn upload(
    mut commands: Commands,
    mut dirty: ResMut<ProjectionUpdates>,
    projections: Query<&BuildingProjection>,
    editor: Res<EditorState>,
    mut assets: RenderAssets,
) {
    // At most two accepted building uploads per frame, <=5 batches/building.
    for entity in dirty.take(2) {
        let Ok(projection) = projections.get(entity) else {
            continue;
        };
        if !editor.editor().accepts(projection.ticket) {
            continue;
        }
        let mut next = ResidentMesh {
            children: Vec::new(),
            handles: Vec::new(),
            triangles: projection.mesh.triangles(),
        };
        for batch in &projection.mesh.batches {
            let mesh = assets.meshes.add(to_bevy_mesh(&batch.data));
            next.children.push(
                commands
                    .spawn((
                        Mesh3d(mesh.clone()),
                        MeshMaterial3d(assets.palette.0[&batch.material].clone()),
                        Transform::default(),
                        ChildOf(entity),
                    ))
                    .id(),
            );
            next.handles.push(mesh);
        }
        let p = projection.target.placement;
        let mut min = Vec3::splat(f32::INFINITY);
        let mut max = Vec3::splat(f32::NEG_INFINITY);
        for batch in &projection.mesh.batches {
            for &position in &batch.data.positions {
                let point = Vec3::from_array(position);
                min = min.min(point);
                max = max.max(point);
            }
        }
        commands.entity(entity).insert((
            DisplayedBuilding {
                ticket: projection.ticket,
                target: projection.target.clone(),
                mesh: projection.mesh.clone(),
                min,
                max,
            },
            Transform::from_xyz(p.x as f32, p.elevation as f32, p.z as f32)
                .with_rotation(Quat::from_rotation_y(p.yaw as f32)),
            Visibility::default(),
        ));
        assets.stats.batches += next.handles.len();
        assets.stats.triangles += next.triangles;
        if let Some(old) = assets.residency.0.insert(entity, next) {
            assets.stats.batches -= old.handles.len();
            assets.stats.triangles -= old.triangles;
            for child in old.children {
                commands.entity(child).despawn();
            }
            for handle in old.handles {
                assets.meshes.remove(handle.id());
            }
        }
        assets.stats.uploads += 1;
    }
    assets.stats.buildings = assets.residency.0.len();
}

#[cfg(test)]
mod tests {
    use super::*;
    use garden_application::EditCommand;
    use garden_bevy::{CommandInbox, EditFeedback, GardenPlugin, Operation};
    use garden_domain::{BuildingId, sample_building};
    use std::time::{Duration, Instant};
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
                .unwrap(),
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
        assert!(stats.batches <= 5 && stats.triangles > 0);
        let old_children = app
            .world()
            .resource::<Residency>()
            .0
            .values()
            .next()
            .unwrap()
            .children
            .clone();
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
                .all(|e| app.world().get_entity(e).is_err())
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
}
