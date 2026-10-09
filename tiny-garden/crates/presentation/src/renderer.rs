//! Budgeted chunk upload, coherent publication/transition and explicit residency.
//! Children represent material chunks, never one entity per brick/window.
#[cfg(test)]
mod tests;
mod transitions;
use crate::catalog::ArtCatalog;
use bevy::ecs::system::SystemParam;
use bevy::image::{ImageAddressMode, ImageLoaderSettings, ImageSampler, ImageSamplerDescriptor};
use bevy::pbr::SpecializedMaterialPipelineCache;
use bevy::render::{
    Render, RenderApp, RenderSystems, camera::ExtractedCamera, mesh::RenderMesh,
    render_asset::RenderAssets as GpuAssets, render_resource::PipelineCache,
    sync_world::MainEntity, view::RenderVisibleEntities,
};
use bevy::{
    asset::RenderAssetUsages, mesh::Indices, prelude::*, render::render_resource::PrimitiveTopology,
};
use garden_bevy::{BuildingProjection, EditorState, GardenSet, PendingTargets, ProjectionUpdates};
use garden_generation::BuildingLayout;
use garden_generation::incremental::{BatchId, PreparedBatch, PreparedBuilding};
use garden_generation::mesh::MaterialKey;
use std::{
    collections::{BTreeMap, BTreeSet, VecDeque},
    sync::{Arc, Mutex},
};
use transitions::{
    abort_waiting, advance_transitions, dispose_fade, fading_material, warm_old_part,
};

/// CPU pick data follows the uploaded picture, not a newer pending projection.
#[derive(Component, Clone)]
pub struct DisplayedBuilding {
    pub ticket: garden_application::JobTicket,
    pub target: Arc<BuildingLayout>,
    pub mesh: Arc<PreparedBuilding>,
    pub min: Vec3,
    pub max: Vec3,
}

pub struct PresentationPlugin {
    catalog: ArtCatalog,
    textures: bool,
}
impl PresentationPlugin {
    pub fn new(catalog: ArtCatalog) -> Self {
        Self {
            catalog,
            textures: false,
        }
    }
    pub fn with_house_textures(mut self) -> Self {
        self.textures = true;
        self
    }
}
#[derive(Resource)]
struct Theme(ArtCatalog);
#[derive(Resource)]
struct TexturedPalette(bool);
/// Capture and diagnostics wait for texture dependencies, not only meshes.
#[derive(Resource, Default)]
pub struct ArtLoadState {
    pub textures: Vec<Handle<Image>>,
}
#[derive(Resource, Default)]
struct Palette(BTreeMap<MaterialKey, Handle<StandardMaterial>>);
#[derive(Resource, Default)]
struct Residency(BTreeMap<Entity, ResidentMesh>);
#[derive(Clone)]
struct ResidentMesh {
    parts: BTreeMap<BatchId, ResidentBatch>,
    triangles: usize,
}
#[derive(Clone)]
struct ResidentBatch {
    child: Entity,
    handle: Handle<Mesh>,
    data: Arc<garden_generation::mesh::MeshData>,
    material: MaterialKey,
    offset: [f32; 3],
}
#[derive(Resource, Default)]
struct Fades(BTreeMap<Entity, Fade>);
struct Fade {
    elapsed: f32,
    age: f32,
    started: bool,
    next: DisplayedBuilding,
    retired: Vec<ResidentBatch>,
    old_materials: BTreeMap<MaterialKey, Handle<StandardMaterial>>,
    next_materials: BTreeMap<MaterialKey, Handle<StandardMaterial>>,
    restore: Vec<(Entity, MaterialKey)>,
    switched: bool,
    warmups: Vec<Entity>,
    baseline: ResidentMesh,
}
#[derive(Component)]
struct RootMotion(Transform);
/// Fade coherent topology groups, never interpolate unrelated mesh vertices or
/// scale the whole house (which would distort doors and windows).
#[derive(Resource, Clone, Copy)]
pub struct TransitionSettings {
    pub duration: f32,
    pub motion_half_life: f32,
}
impl Default for TransitionSettings {
    fn default() -> Self {
        Self {
            duration: 0.16,
            motion_half_life: 0.055,
        }
    }
}
#[derive(Resource, Default)]
struct Staging(BTreeMap<Entity, StagedBuilding>);
#[derive(Resource, Default)]
struct UploadCursor(Option<Entity>);
struct StagedBuilding {
    ticket: garden_application::JobTicket,
    target: Arc<BuildingLayout>,
    mesh: Arc<PreparedBuilding>,
    pending: VecDeque<usize>,
    handles: BTreeMap<BatchId, Handle<Mesh>>,
    owned: Vec<Handle<Mesh>>,
}
/// Shared bridge reports actual RenderMesh preparation, not CPU installation.
/// It deliberately doesn't block/poll the GPU on the UI thread.
#[derive(Resource, Clone, Default)]
struct MeshReadiness {
    enabled: bool,
    shared: Arc<Mutex<ReadinessState>>,
}
#[derive(Default)]
struct ReadinessState {
    waiting: BTreeSet<AssetId<Mesh>>,
    ready: BTreeSet<AssetId<Mesh>>,
    waiting_draws: BTreeSet<Entity>,
    ready_draws: BTreeSet<Entity>,
    visible_draws: BTreeSet<Entity>,
}
fn acknowledge_meshes(
    bridge: Res<MeshReadiness>,
    meshes: Res<GpuAssets<RenderMesh>>,
    pipelines: Res<SpecializedMaterialPipelineCache>,
    cache: Res<PipelineCache>,
    views: Query<&RenderVisibleEntities, With<ExtractedCamera>>,
) {
    let mut state = bridge.shared.lock().unwrap();
    let ready = state
        .waiting
        .iter()
        .copied()
        .filter(|id| meshes.get(*id).is_some())
        .collect::<Vec<_>>();
    for id in ready {
        state.waiting.remove(&id);
        state.ready.insert(id);
    }
    let ready = state
        .waiting_draws
        .iter()
        .copied()
        .filter(|entity| {
            pipelines
                .values()
                .filter_map(|view| view.get(&MainEntity::from(*entity)))
                .any(|id| cache.get_render_pipeline(*id).is_some())
        })
        .collect::<Vec<_>>();
    for entity in ready {
        state.waiting_draws.remove(&entity);
        state.ready_draws.insert(entity);
    }
    // Camera visibility, not ViewVisibility (which also includes shadow
    // lights). Off-camera buildings cannot wait on unqueued material pipelines.
    state.visible_draws = state
        .waiting_draws
        .iter()
        .chain(&state.ready_draws)
        .copied()
        .filter(|entity| {
            let main = MainEntity::from(*entity);
            views.iter().any(|view| {
                view.classes
                    .get(&std::any::TypeId::of::<Mesh3d>())
                    .is_some_and(|class| {
                        class
                            .entities_cpu_culling
                            .binary_search_by_key(&main, |(_, main)| *main)
                            .is_ok()
                            || class.entities_gpu_culling.contains_key(&main)
                    })
            })
        })
        .collect();
}
impl MeshReadiness {
    fn track_draw(&self, entity: Entity) {
        if self.enabled {
            self.shared.lock().unwrap().waiting_draws.insert(entity);
        }
    }
    fn draw_ready(&self, entity: Entity) -> bool {
        !self.enabled || self.shared.lock().unwrap().ready_draws.contains(&entity)
    }
    fn draw_visible(&self, entity: Entity) -> bool {
        !self.enabled || self.shared.lock().unwrap().visible_draws.contains(&entity)
    }
    fn forget_draw(&self, entity: Entity) {
        let mut state = self.shared.lock().unwrap();
        state.waiting_draws.remove(&entity);
        state.ready_draws.remove(&entity);
        state.visible_draws.remove(&entity);
    }
    fn track(&self, handle: &Handle<Mesh>) {
        if self.enabled {
            self.shared.lock().unwrap().waiting.insert(handle.id());
        }
    }
    fn ready(&self, handle: &Handle<Mesh>) -> bool {
        !self.enabled || self.shared.lock().unwrap().ready.contains(&handle.id())
    }
    fn forget(&self, handle: &Handle<Mesh>) {
        let mut state = self.shared.lock().unwrap();
        state.waiting.remove(&handle.id());
        state.ready.remove(&handle.id());
    }
}
/// Main-world resource installation budget. All production chunks are bounded;
/// an oversized third-party chunk is allowed alone and explicitly counted.
#[derive(Resource, Clone, Copy)]
pub struct UploadBudget {
    pub bytes_per_frame: usize,
    pub batches_per_frame: usize,
}
impl Default for UploadBudget {
    fn default() -> Self {
        Self {
            bytes_per_frame: 512 * 1024,
            batches_per_frame: 8,
        }
    }
}

#[derive(Resource, Default, Debug)]
pub struct RenderStats {
    pub buildings: usize,
    pub batches: usize,
    pub triangles: usize,
    pub uploads: u64,
    pub uploaded_bytes: u64,
    pub reused_batches: u64,
    pub oversized_batches: u64,
    pub pending_buildings: usize,
    pub transition_batches: usize,
}

impl Plugin for PresentationPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(Theme(self.catalog.clone()))
            .insert_resource(TexturedPalette(self.textures))
            .init_resource::<ArtLoadState>()
            .init_resource::<Palette>()
            .init_resource::<Residency>()
            .init_resource::<Staging>()
            .init_resource::<UploadCursor>()
            .init_resource::<Fades>()
            .init_resource::<TransitionSettings>()
            .init_resource::<UploadBudget>()
            .init_resource::<RenderStats>()
            .add_systems(Startup, setup_palette)
            .add_systems(
                Update,
                (release_removed, upload, advance_transitions)
                    .chain()
                    .after(GardenSet::Collect),
            );
        let bridge = MeshReadiness {
            enabled: app.get_sub_app(RenderApp).is_some(),
            ..default()
        };
        app.insert_resource(bridge.clone());
        if let Some(render_app) = app.get_sub_app_mut(RenderApp) {
            render_app
                .insert_resource(bridge)
                .add_systems(Render, acknowledge_meshes.after(RenderSystems::Cleanup));
        }
    }
}
fn setup_palette(
    theme: Res<Theme>,
    mut palette: ResMut<Palette>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    textured: Res<TexturedPalette>,
    server: Option<Res<AssetServer>>,
    mut load: ResMut<ArtLoadState>,
) {
    for key in MaterialKey::ALL {
        let definition = theme.0.material(key);
        let [r, g, b] = definition.base_color_srgb;
        let mut material = StandardMaterial {
            base_color: Color::srgb_u8(r, g, b),
            perceptual_roughness: definition.roughness,
            metallic: 0.0,
            ..default()
        };
        if textured.0 && key != MaterialKey::Window {
            let server = server
                .as_ref()
                .expect("textured presentation requires AssetPlugin");
            let role = match key {
                MaterialKey::Stone => "Stone",
                MaterialKey::Plaster => "Plaster",
                MaterialKey::Timber => "Timber",
                MaterialKey::Roof => "Roof",
                MaterialKey::Window => unreachable!(),
            };
            let mut texture = |suffix: &str, srgb: bool| {
                let path =
                    format!("themes/warm-stone/house-kit-v4/T_WarmStone_{role}_{suffix}.png");
                let image = server
                    .load_builder()
                    .with_settings(move |settings: &mut ImageLoaderSettings| {
                        settings.is_srgb = srgb;
                        settings.sampler = ImageSampler::Descriptor(ImageSamplerDescriptor {
                            address_mode_u: ImageAddressMode::Repeat,
                            address_mode_v: ImageAddressMode::Repeat,
                            ..default()
                        });
                    })
                    .load(path);
                load.textures.push(image.clone());
                image
            };
            material.base_color = Color::WHITE;
            material.base_color_texture = Some(texture("BC", true));
            material.normal_map_texture = Some(texture("N", false));
            material.metallic_roughness_texture = Some(texture("ORM", false));
            material.perceptual_roughness = 1.0;
        }
        palette.0.insert(key, materials.add(material));
    }
}
pub fn to_bevy_mesh(batch: &PreparedBatch) -> Mesh {
    let data = &batch.data;
    debug_assert_eq!(batch.tangents.len(), data.positions.len());
    Mesh::new(
        PrimitiveTopology::TriangleList,
        RenderAssetUsages::RENDER_WORLD,
    )
    .with_inserted_attribute(Mesh::ATTRIBUTE_POSITION, data.positions.clone())
    .with_inserted_attribute(Mesh::ATTRIBUTE_NORMAL, data.normals.clone())
    .with_inserted_attribute(Mesh::ATTRIBUTE_UV_0, data.uvs.clone())
    .with_inserted_attribute(Mesh::ATTRIBUTE_TANGENT, batch.tangents.as_ref().clone())
    .with_inserted_indices(Indices::U32(data.indices.clone()))
}
fn release_removed(mut removed: RemovedComponents<BuildingProjection>, mut assets: RenderAssets) {
    for entity in removed.read() {
        if let Some(fade) = assets.fades.0.remove(&entity) {
            dispose_fade(entity, fade, &mut assets);
        }
        if let Some(old) = assets.residency.0.remove(&entity) {
            assets.stats.batches -= old.parts.len();
            assets.stats.triangles -= old.triangles;
            for part in old.parts.into_values() {
                assets.bridge.forget(&part.handle);
                assets.meshes.remove(part.handle.id());
            }
        }
        if let Some(stage) = assets.staging.0.remove(&entity) {
            for handle in stage.owned {
                assets.bridge.forget(&handle);
                assets.meshes.remove(handle.id());
            }
        }
    }
    assets.stats.buildings = assets.residency.0.len();
}
#[derive(SystemParam)]
struct RenderAssets<'w> {
    palette: Res<'w, Palette>,
    residency: ResMut<'w, Residency>,
    meshes: ResMut<'w, Assets<Mesh>>,
    stats: ResMut<'w, RenderStats>,
    staging: ResMut<'w, Staging>,
    bridge: Res<'w, MeshReadiness>,
    budget: Res<'w, UploadBudget>,
    fades: ResMut<'w, Fades>,
    materials: ResMut<'w, Assets<StandardMaterial>>,
    pending: ResMut<'w, PendingTargets>,
    cursor: ResMut<'w, UploadCursor>,
    transitions: Res<'w, TransitionSettings>,
}
fn upload(
    mut commands: Commands,
    mut dirty: ResMut<ProjectionUpdates>,
    projections: Query<&BuildingProjection>,
    editor: Res<EditorState>,
    mut assets: RenderAssets,
    displayed: Query<(&DisplayedBuilding, &Transform)>,
    time: Option<Res<Time>>,
) {
    // An accepted-but-unpublished target can become obsolete while waiting for
    // cold shader pipelines. Do not make the newest target wait behind it.
    let obsolete = assets
        .fades
        .0
        .iter()
        .filter(|(_, fade)| !fade.started && !editor.editor().accepts(fade.next.ticket))
        .map(|(entity, _)| *entity)
        .collect::<Vec<_>>();
    for entity in obsolete {
        let fade = assets.fades.0.remove(&entity).unwrap();
        abort_waiting(entity, fade, &mut commands, &mut assets);
        dirty.schedule(entity);
    }
    // Coalesce prepared targets; keep the last valid picture while resources
    // are staged over multiple frames. No full-scene query or full mesh merge.
    for entity in dirty.take(2) {
        let Ok(projection) = projections.get(entity) else {
            continue;
        };
        if !editor.editor().accepts(projection.ticket) {
            continue;
        }
        if let Some(old) = assets.staging.0.remove(&entity) {
            for handle in old.owned {
                assets.bridge.forget(&handle);
                assets.meshes.remove(handle.id());
            }
        }
        let mut stage = StagedBuilding {
            ticket: projection.ticket,
            target: projection.target.clone(),
            mesh: projection.mesh.clone(),
            pending: VecDeque::new(),
            handles: BTreeMap::new(),
            owned: Vec::new(),
        };
        for (i, batch) in projection.mesh.batches.iter().enumerate() {
            if let Some(old) = assets
                .residency
                .0
                .get(&entity)
                .and_then(|r| r.parts.get(&batch.id))
                .filter(|old| Arc::ptr_eq(&old.data, &batch.data))
            {
                stage.handles.insert(batch.id, old.handle.clone());
                assets.stats.reused_batches += 1;
            } else {
                stage.pending.push_back(i);
            }
        }
        assets.staging.0.insert(entity, stage);
    }
    let mut bytes = 0;
    let mut batches = 0;
    let mut entities = assets.staging.0.keys().copied().collect::<Vec<_>>();
    if let Some(cursor) = assets.cursor.0 {
        let first = entities.partition_point(|e| *e <= cursor);
        if first < entities.len() {
            entities.rotate_left(first);
        }
    }
    for entity in entities {
        let mut stage = assets.staging.0.remove(&entity).unwrap();
        if !editor.editor().accepts(stage.ticket) {
            for handle in stage.owned {
                assets.bridge.forget(&handle);
                assets.meshes.remove(handle.id());
            }
            continue;
        }
        while let Some(&i) = stage.pending.front() {
            let batch = &stage.mesh.batches[i];
            let size = batch.data.positions.len() * 48 + batch.data.indices.len() * 4;
            if batches >= assets.budget.batches_per_frame.max(1)
                || (batches > 0 && bytes + size > assets.budget.bytes_per_frame)
            {
                break;
            }
            if size > assets.budget.bytes_per_frame {
                assets.stats.oversized_batches += 1;
            }
            let handle = assets.meshes.add(to_bevy_mesh(batch));
            assets.bridge.track(&handle);
            stage.handles.insert(batch.id, handle.clone());
            stage.owned.push(handle);
            stage.pending.pop_front();
            bytes += size;
            batches += 1;
            assets.cursor.0 = Some(entity);
            assets.stats.uploaded_bytes += size as u64;
        }
        // One building is a conservative consistency group. This can later be
        // split by independent dependency islands without changing chunk IDs.
        if !stage.pending.is_empty() || stage.handles.values().any(|h| !assets.bridge.ready(h)) {
            assets.staging.0.insert(entity, stage);
            continue;
        }
        // Keep a single short coherent transition per building. A newer target
        // can prepare meanwhile, but starts from the completed visible pose,
        // not from a partly faded old/new mix. Memory remains bounded to 2 poses.
        if assets.fades.0.contains_key(&entity) {
            assets.staging.0.insert(entity, stage);
            continue;
        }
        let previous_display = displayed.get(entity).ok().map(|(d, _)| d.clone());
        let fade_enabled = previous_display.is_some()
            && time.is_some()
            && assets.transitions.duration.is_finite()
            && assets.transitions.duration > 0.;
        let mut old_materials = BTreeMap::new();
        let mut next_materials = BTreeMap::new();
        let mut retired = Vec::new();
        let mut restore = Vec::new();
        let mut warmups = Vec::new();
        let mut old = assets.residency.0.remove(&entity);
        let baseline = if fade_enabled { old.clone() } else { None };
        let mut next = ResidentMesh {
            parts: BTreeMap::new(),
            triangles: stage.mesh.triangles(),
        };
        for batch in &stage.mesh.batches {
            let handle = stage.handles.remove(&batch.id).unwrap();
            let previous = old.as_mut().and_then(|old| old.parts.remove(&batch.id));
            let same = previous.as_ref().is_some_and(|old| old.handle == handle)
                && previous_display
                    .as_ref()
                    .and_then(|d| d.mesh.batches.iter().find(|b| b.id == batch.id))
                    .is_some_and(|b| b.offset == batch.offset);
            let child = if same {
                previous.unwrap().child
            } else if fade_enabled {
                if let Some(previous) = previous {
                    warm_old_part(
                        &previous,
                        entity,
                        &mut commands,
                        &mut old_materials,
                        &mut warmups,
                        &mut assets,
                    );
                    retired.push(previous);
                }
                let material =
                    fading_material(batch.material, 0., &mut next_materials, &mut assets);
                let child = commands
                    .spawn((
                        Mesh3d(handle.clone()),
                        MeshMaterial3d(material),
                        Transform::from_translation(Vec3::from_array(batch.offset)),
                        ChildOf(entity),
                    ))
                    .id();
                restore.push((child, batch.material));
                assets.bridge.track_draw(child);
                child
            } else if let Some(previous) = previous {
                if previous.handle != handle {
                    assets.bridge.forget(&previous.handle);
                    assets.meshes.remove(previous.handle.id());
                }
                commands.entity(previous.child).insert((
                    Mesh3d(handle.clone()),
                    Transform::from_translation(Vec3::from_array(batch.offset)),
                ));
                previous.child
            } else {
                commands
                    .spawn((
                        Mesh3d(handle.clone()),
                        MeshMaterial3d(assets.palette.0[&batch.material].clone()),
                        Transform::from_translation(Vec3::from_array(batch.offset)),
                        ChildOf(entity),
                    ))
                    .id()
            };
            next.parts.insert(
                batch.id,
                ResidentBatch {
                    child,
                    handle,
                    data: batch.data.clone(),
                    material: batch.material,
                    offset: batch.offset,
                },
            );
        }
        if let Some(old) = old {
            // Stats are recomputed from live residency below, not speculative
            // handles waiting for upload.
            for part in old.parts.into_values() {
                if fade_enabled {
                    warm_old_part(
                        &part,
                        entity,
                        &mut commands,
                        &mut old_materials,
                        &mut warmups,
                        &mut assets,
                    );
                    retired.push(part);
                } else {
                    commands.entity(part.child).despawn();
                    assets.bridge.forget(&part.handle);
                    assets.meshes.remove(part.handle.id());
                }
            }
        }
        let (min, max) = bounds(&stage.mesh);
        let p = stage.target.placement;
        let next_display = DisplayedBuilding {
            ticket: stage.ticket,
            target: stage.target,
            mesh: stage.mesh,
            min,
            max,
        };
        let goal = Transform::from_xyz(p.x as f32, p.elevation as f32, p.z as f32)
            .with_rotation(Quat::from_rotation_y(p.yaw as f32));
        commands.entity(entity).insert(Visibility::default());
        if time.is_some() && displayed.get(entity).is_ok() {
            commands.entity(entity).insert(RootMotion(goal));
        } else {
            commands.entity(entity).insert(goal);
        }
        if fade_enabled && (!restore.is_empty() || !retired.is_empty()) {
            assets.fades.0.insert(
                entity,
                Fade {
                    elapsed: 0.,
                    age: 0.,
                    started: false,
                    next: next_display,
                    retired,
                    old_materials,
                    next_materials,
                    restore,
                    switched: false,
                    warmups,
                    baseline: baseline.expect("only existing buildings fade"),
                },
            );
        } else {
            assets.pending.displayed(next_display.ticket);
            commands.entity(entity).insert(next_display);
        }
        assets.residency.0.insert(entity, next);
        assets.stats.uploads += 1;
    }
    assets.stats.buildings = assets.residency.0.len();
    assets.stats.batches = assets.residency.0.values().map(|r| r.parts.len()).sum();
    assets.stats.triangles = assets.residency.0.values().map(|r| r.triangles).sum();
    assets.stats.pending_buildings = assets.staging.0.len();
}
fn bounds(mesh: &PreparedBuilding) -> (Vec3, Vec3) {
    let mut min = Vec3::splat(f32::INFINITY);
    let mut max = Vec3::splat(f32::NEG_INFINITY);
    for b in &mesh.batches {
        let offset = Vec3::from_array(b.offset);
        min = min.min(Vec3::from_array(b.min) + offset);
        max = max.max(Vec3::from_array(b.max) + offset);
    }
    (min, max)
}
