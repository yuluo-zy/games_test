//! Headless Bevy 0.16 adapter. Business leaves are recovered; this explicit
//! dependency order is reconstruction scheduling, not proof of the fork graph.
// Bevy system parameters remain explicit; grouping them changes the scheduling input API.
#![allow(clippy::too_many_arguments, clippy::type_complexity)]
pub mod decorator_assets;
pub mod decorator_info_binding;
pub mod decorator_placement;
pub mod fences;
pub mod gates;
pub mod history;
pub mod nani;
pub mod openings;
pub mod roof_edits;
pub mod roof_invalid;
pub mod wall_arches;
pub mod wall_authoring;
pub mod wall_bricks;
pub mod wall_bulldoze;
pub mod wall_decorator_archivist;
pub mod wall_height;
pub mod wall_paths;
pub mod wall_pending_changed;
pub mod wall_platform_collision;
pub mod wall_platform_stairs;
pub mod wall_rectangle_edit;
pub mod wall_segments;
pub mod wall_structures;

use bevy_app::{App, Plugin, Update};
use bevy_ecs::{prelude::*, schedule::ApplyDeferred};
use country_core::recovered_building::{
    building_roof_associations::{ResolvedRoofAssociation, RoofAssociations},
    building_roof_components::{
        RoofAnimation, RoofTileSimulation, RoofTriggerVolume, prepare_resolved_roof_components,
    },
    building_roof_creation::RoofCreateRequestObserved,
    building_roof_deletion::{match_deletion_request, should_despawn},
    building_roof_spatial::synchronize_resolved_roof,
    building_wall_cache::regular_wall_height_cache,
    building_wall_publication::{WallPublicationCore, publication_metrics},
    building_wall_state::{PublicWallBuildingState, WallId, WallShapeSnapshot},
};
use std::{collections::HashMap, sync::Arc};

/// Resolved compatibility input. UpsertRawWall collects this from raw Curve2
/// and explicit terrain; callers of PublishWall provide already resolved data.
#[derive(Clone, Debug)]
pub struct WallPublicationInput {
    pub outline: Arc<Vec<[f32; 3]>>,
    pub terrain_y: Vec<f32>,
    pub source_curve_length: f32,
    pub source_point_count: usize,
    pub shape: WallShapeSnapshot,
    pub style: u8,
    pub extra: f32,
    pub special: bool,
}
#[derive(Event, Clone)]
pub struct PublishWall {
    pub wall: WallId,
    pub input: WallPublicationInput,
}
#[derive(Event, Clone, Copy)]
pub struct RemoveWall(pub WallId);
#[derive(Event, Clone)]
pub struct CreateRoof(pub RoofCreateRequestObserved);
#[derive(Event, Clone, Copy)]
pub struct DeleteRoof(pub WallId);
#[derive(Event, Clone, Copy)]
pub struct WallSpatialDirty(pub WallId);
#[derive(Event, Clone, Copy)]
pub struct WallRoofPresenceChanged(pub WallId);
#[derive(Event, Clone, Copy)]
pub struct WallGableDirty(pub WallId);
#[derive(Event, Clone, Copy, Debug)]
pub struct RoofGeometryChanged {
    pub entity: Entity,
    pub wall: WallId,
    pub kind: u8,
}
#[derive(Event, Clone, Copy, Debug)]
pub struct RoofSound {
    pub entity: Entity,
    pub wall: WallId,
    pub pivot: [f32; 3],
    pub kind: u8,
}

#[derive(Clone)]
pub struct PublishedWall {
    projection: [u8; 672],
    pub state: PublicWallBuildingState,
    pub source: WallPublicationInput,
    pub arch_segment_heights: Vec<f32>,
    /// True only when the derived gable cache could not be computed.
    pub gable_profile_pending: bool,
    pub base_arch_segment_heights: Vec<f32>,
    pub structural_arch_segment_heights: Vec<f32>,
    pub observed_source_metadata_complete: bool,
    pub segment_heights: Vec<f32>,
    /// Adapter snapshot of the cache owner's initialization, before original
    /// arches/lintels accumulate into the single public third-height array.
    pub base_segment_heights: Vec<f32>,
    pub publication_revision: u64,
}
impl PublishedWall {
    /// Evidence projection for native differential diagnostics, not ABI storage.
    pub fn observed_projection(&self) -> &[u8; 672] {
        &self.projection
    }
}
#[derive(Resource, Default)]
pub struct PublicWalls {
    pub walls: HashMap<WallId, PublishedWall>,
    order: Vec<WallId>,
}
#[derive(Resource, Default)]
pub struct WallRoofMap(pub RoofAssociations);
impl WallRoofMap {
    pub fn entity(&self, wall: WallId) -> Option<Entity> {
        self.0.entity(wall).map(Entity::from_bits)
    }
}
#[derive(Resource)]
pub struct BuildingFrame {
    pub delta_time: f32,
    pub phase: u32,
    pub tick: u32,
}
impl Default for BuildingFrame {
    fn default() -> Self {
        Self {
            delta_time: 0.0,
            phase: 0,
            tick: 0,
        }
    }
}
/// Current update only; Bevy owns event retention and command queues.
#[derive(Resource, Default, Debug)]
pub struct BuildingReport {
    pub faults: Vec<String>,
    pub created: Vec<Entity>,
    pub marked: Vec<Entity>,
    pub despawned: Vec<Entity>,
    pub presence_changes: Vec<WallId>,
    pub geometry_changes: Vec<RoofGeometryChanged>,
    pub sounds: Vec<RoofSound>,
    pub wall_holes_requests: Vec<WallId>,
    pub rim_history_requests:
        Vec<country_core::recovered_building::building_roof_customization::RoofRimHistoryMutation>,
}
#[derive(Component, Clone, Debug)]
pub struct Roof(pub [u8; 88]);
#[derive(Component, Clone, Debug)]
pub struct TileSimulation(pub RoofTileSimulation);
#[derive(Component, Clone, Debug, Default)]
pub struct Animation(pub RoofAnimation);
#[derive(Component, Clone, Debug, Default)]
pub struct TriggerVolume(pub RoofTriggerVolume);
#[derive(Component, Clone, Copy, Debug)]
pub struct RoofMarkedForDeletion;
// Original RoofSpire/Support are zero-size markers with empty required graphs.
// The Nani descriptor/Transform/typed marker are inserted explicitly as a bundle.
#[derive(Component, Clone, Copy, Debug, Default)]
pub struct RoofSpire;
#[derive(Component, Clone, Copy, Debug, Default)]
pub struct RoofSupportBeams;
/// Empty original RoofTiles allocation. Active GPU allocation is not fabricated.
#[derive(Component, Clone, Debug, Default)]
pub struct RoofTiles {
    pub allocation: Option<DeferredRoofTileAllocation>,
}
#[derive(Clone, Debug)]
pub struct DeferredRoofTileAllocation {
    _private: (),
}
#[derive(Component, Clone, Debug)]
pub struct ObjectTransform(pub [u8; 48]);
impl Default for ObjectTransform {
    fn default() -> Self {
        let mut bytes = [0; 48];
        for offset in [12, 28, 32, 36] {
            bytes[offset..offset + 4].copy_from_slice(&1f32.to_le_bytes());
        }
        Self(bytes)
    }
}
#[derive(Bundle)]
pub struct RoofBundle {
    pub roof: Roof,
    pub tile: TileSimulation,
    pub animation: Animation,
    pub trigger: TriggerVolume,
    pub tiles: RoofTiles,
    pub spire: RoofSpire,
    pub supports: RoofSupportBeams,
    pub nani: nani::NaniStoneFloorBundle,
}
#[derive(SystemSet, Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum BuildingStage {
    Begin,
    HistoryPrepare,
    Authoring,
    CollectRaw,
    Publish,
    PathPreparation,
    Segmentation,
    PlatformSegmentation,
    PlatformStairs,
    PlatformCollision,
    WallStructures,
    WallArches,
    Gates,
    Fences,
    Cache,
    Replay,
    Create,
    Associate,
    Spatial,
    InvalidRoofs,
    Edit,
    GableCache,
    Placement,
    Openings,
    WallGeometry,
    Visual,
    Mark,
    Animate,
    NaniUpdate,
    Delete,
    Reassociate,
    FinalCache,
    NaniCleanup,
}

pub struct BuildingPlugin;
impl Plugin for BuildingPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<PublicWalls>()
            .init_resource::<WallRoofMap>()
            .init_resource::<BuildingFrame>()
            .init_resource::<BuildingReport>()
            .init_resource::<WallCollectionState>()
            .init_resource::<RoofRimHistoryRequests>()
            .init_resource::<nani::DeferredNaniRequests>()
            .init_resource::<nani::NaniCpuStagingState>()
            .init_resource::<nani::NaniMeshAtlas>()
            .init_resource::<nani::NaniBackendConfirmations>()
            .init_resource::<nani::NaniCpuDiagnostics>()
            .init_resource::<nani::SolidNaniCpuStaging>()
            .init_resource::<nani::SolidNaniCpuChanges>()
            .init_resource::<nani::SolidNaniCpuDiagnostics>()
            .add_event::<UpsertRawWall>()
            .add_event::<SetTerrainGrid>()
            .add_event::<TerrainDirtyCandidates>()
            .add_event::<TerrainDirtyTiles>()
            .add_event::<PublishWall>()
            .add_event::<RemoveWall>()
            .add_event::<CreateRoof>()
            .add_event::<DeleteRoof>()
            .add_event::<SwitchRoofDirection>()
            .add_event::<SetFlatRoofStyle>()
            .add_event::<SetFlatRoofRimStyle>()
            .add_event::<ReplayRoofEvent>()
            .add_event::<RecalculateWallHoles>()
            .add_event::<WallSpatialDirty>()
            .add_event::<WallRoofPresenceChanged>()
            .add_event::<WallGableDirty>()
            .add_event::<RoofGeometryChanged>()
            .add_event::<RoofSound>();
        openings::register(app);
        decorator_placement::register(app);
        history::register(app);
        roof_edits::register(app);
        wall_height::register(app);
        wall_bricks::register(app);
        wall_paths::register(app);
        wall_segments::register(app);
        wall_platform_stairs::register(app);
        wall_platform_collision::register(app);
        wall_rectangle_edit::register(app);
        wall_pending_changed::register(app);
        wall_decorator_archivist::register(app);
        decorator_info_binding::register(app);
        wall_authoring::register(app);
        wall_bulldoze::register(app);
        roof_invalid::register(app);
        wall_structures::register(app);
        wall_arches::register(app);
        gates::register(app);
        fences::register(app);
        let world = app.world_mut();
        world.register_component::<Roof>();
        world.register_component::<TileSimulation>();
        world.register_component::<Animation>();
        world.register_component::<TriggerVolume>();
        world.register_component::<RoofTiles>();
        world.register_component::<RoofSpire>();
        world.register_component::<RoofSupportBeams>();
        world.register_component::<ObjectTransform>();
        nani::register_nani_components(world);
        world.register_component::<RoofMarkedForDeletion>();
        app.configure_sets(
            Update,
            (
                BuildingStage::Begin,
                BuildingStage::Authoring,
                BuildingStage::CollectRaw,
                BuildingStage::Publish,
                BuildingStage::Cache,
                BuildingStage::Replay,
                BuildingStage::Create,
                BuildingStage::Associate,
                BuildingStage::Spatial,
                BuildingStage::InvalidRoofs,
                BuildingStage::Edit,
                BuildingStage::GableCache,
                BuildingStage::Visual,
                BuildingStage::Mark,
                BuildingStage::Animate,
                BuildingStage::NaniUpdate,
                BuildingStage::Delete,
                BuildingStage::Reassociate,
                BuildingStage::FinalCache,
                BuildingStage::NaniCleanup,
            )
                .chain(),
        )
        .configure_sets(
            Update,
            BuildingStage::HistoryPrepare
                .after(BuildingStage::Begin)
                .before(BuildingStage::Authoring),
        )
        .configure_sets(
            Update,
            (BuildingStage::PathPreparation, BuildingStage::Segmentation)
                .chain()
                .after(BuildingStage::Publish)
                .before(BuildingStage::Cache),
        )
        .configure_sets(
            Update,
            (
                BuildingStage::PlatformSegmentation,
                BuildingStage::PlatformStairs,
                BuildingStage::PlatformCollision,
                BuildingStage::WallStructures,
                BuildingStage::WallArches,
                BuildingStage::Gates,
                BuildingStage::Fences,
                BuildingStage::Placement,
                BuildingStage::Openings,
                BuildingStage::WallGeometry,
            )
                .chain()
                .after(BuildingStage::FinalCache)
                .before(BuildingStage::NaniCleanup),
        )
        .add_systems(Update, begin.in_set(BuildingStage::Begin))
        .add_systems(Update, collect_raw_inputs.in_set(BuildingStage::CollectRaw))
        .add_systems(Update, publish.in_set(BuildingStage::Publish))
        .add_systems(Update, fill_base_cache.in_set(BuildingStage::Cache))
        .add_systems(
            Update,
            history::prepare.in_set(BuildingStage::HistoryPrepare),
        )
        .add_systems(Update, route_replay.in_set(BuildingStage::Replay))
        .add_systems(
            Update,
            (create_roofs, ApplyDeferred)
                .chain()
                .in_set(BuildingStage::Create),
        )
        .add_systems(Update, associations.in_set(BuildingStage::Associate))
        .add_systems(Update, spatial.in_set(BuildingStage::Spatial))
        .add_systems(
            Update,
            (
                switch_directions,
                roof_edits::consume,
                roof_edits::switch_type_cpu,
                customize_roofs,
                history::record_and_finish,
            )
                .chain()
                .in_set(BuildingStage::Edit),
        )
        .add_systems(Update, fill_gable_cache.in_set(BuildingStage::GableCache))
        .add_systems(
            Update,
            (build_cpu_tiles, ApplyDeferred)
                .chain()
                .in_set(BuildingStage::Visual),
        )
        .add_systems(
            Update,
            (mark_roofs, ApplyDeferred)
                .chain()
                .in_set(BuildingStage::Mark),
        )
        .add_systems(Update, animate.in_set(BuildingStage::Animate))
        .add_systems(
            Update,
            nani::stage_roof_nani_updates.in_set(BuildingStage::NaniUpdate),
        )
        .add_systems(
            Update,
            (delete_roofs, ApplyDeferred)
                .chain()
                .in_set(BuildingStage::Delete),
        )
        .add_systems(Update, associations.in_set(BuildingStage::Reassociate))
        .add_systems(Update, fill_gable_cache.in_set(BuildingStage::FinalCache))
        .add_systems(
            Update,
            (
                nani::stage_roof_nani_updates,
                nani::stage_solid_nani_updates,
            )
                .chain()
                .in_set(BuildingStage::NaniCleanup),
        );
    }
}
pub fn building_app() -> App {
    let mut app = App::new();
    app.add_plugins(BuildingPlugin);
    app
}
fn begin(
    mut frame: ResMut<BuildingFrame>,
    mut report: ResMut<BuildingReport>,
    mut nani_requests: ResMut<nani::DeferredNaniRequests>,
    mut solid_changes: ResMut<nani::SolidNaniCpuChanges>,
) {
    frame.tick = frame.tick.wrapping_add(1);
    *report = BuildingReport::default();
    nani_requests.0.clear();
    solid_changes.0.clear();
}
// Reject NaN as in the recovered publication contract.
#[allow(clippy::neg_cmp_op_on_partial_ord)]
fn publish(
    mut requests: EventReader<PublishWall>,
    mut removed: EventReader<RemoveWall>,
    mut walls: ResMut<PublicWalls>,
    collection: Res<WallCollectionState>,
    authoring: Res<wall_authoring::WallAuthoringState>,
    mut dirty: EventWriter<WallSpatialDirty>,
    mut report: ResMut<BuildingReport>,
) {
    for request in requests.read() {
        let input = &request.input;
        if input.outline.len() != input.terrain_y.len()
            || input.source_point_count < 2
            || !(input.source_curve_length > 0.0)
        {
            report
                .faults
                .push(format!("Invalid publication for wall {}", request.wall.0));
            continue;
        }
        let relative: Vec<_> = input
            .outline
            .iter()
            .zip(&input.terrain_y)
            .map(|(p, y)| p[1] - y)
            .collect();
        let metrics = publication_metrics(&relative, &input.outline, input.extra, input.special);
        let core = WallPublicationCore {
            source_point_count: input.source_point_count as u64,
            source_curve_length: input.source_curve_length,
            shape: &input.shape,
            style: input.style,
            metrics,
        };
        let mut projection = walls
            .walls
            .get(&request.wall)
            .map(|w| w.projection)
            .unwrap_or([0; 672]);
        core.write_projection(&mut projection);
        // Identified source metadata copies; no Vec/Arc headers are forged.
        projection[0x296] = u8::from(input.special);
        if let Some(raw) = collection.raw_sources.get(&request.wall) {
            projection[0x295] = raw.clutter_version;
            projection[0x294] = raw.observed_source_flag_64;
            projection[0x288..0x28c].copy_from_slice(&raw.observed_path_scalar.to_le_bytes());
        }
        let metadata = authoring.history_metadata.get(&request.wall);
        if let Some(metadata) = metadata {
            projection[0x290..0x294].copy_from_slice(&metadata.source_60_bits.to_le_bytes());
            projection[0x298] = metadata.source_67;
            projection[0x29a] = metadata.source_69;
        } else {
            // Evidence padding has no observed value. Consumers requiring these
            // fields must respect the complete flag and remain pending.
            projection[0x290..0x294].fill(0);
            projection[0x298] = 0;
            projection[0x29a] = 0;
        }
        let state = PublicWallBuildingState::from_observed(projection);
        let publication_revision = walls
            .walls
            .get(&request.wall)
            .map_or(1, |w| w.publication_revision.wrapping_add(1));
        if !walls.walls.contains_key(&request.wall) {
            walls.order.push(request.wall);
        }
        walls.walls.insert(
            request.wall,
            PublishedWall {
                projection,
                state,
                source: input.clone(),
                arch_segment_heights: Vec::new(),
                gable_profile_pending: false,
                base_arch_segment_heights: Vec::new(),
                structural_arch_segment_heights: Vec::new(),
                observed_source_metadata_complete: metadata.is_some(),
                segment_heights: Vec::new(),
                base_segment_heights: Vec::new(),
                publication_revision,
            },
        );
        dirty.write(WallSpatialDirty(request.wall));
    }
    for request in removed.read() {
        walls.walls.remove(&request.0);
        walls.order.retain(|id| *id != request.0);
    }
}
fn fill_base_cache(
    mut dirty: EventReader<WallSpatialDirty>,
    mut walls: ResMut<PublicWalls>,
    collection: Res<WallCollectionState>,
    prepared: Res<wall_paths::WallPreparedPaths>,
    mut report: ResMut<BuildingReport>,
) {
    let mut ids: Vec<_> = dirty.read().map(|event| event.0).collect();
    ids.sort_unstable_by_key(|wall| wall.0);
    ids.dedup();
    for id in ids {
        let Some(wall) = walls.walls.get_mut(&id) else {
            continue;
        };
        let input = &wall.source;
        let positions = if collection.raw_sources.contains_key(&id) {
            let Some(path) = prepared.current(id, wall.publication_revision) else {
                wall.projection[0x108..0x10c].fill(0);
                wall.projection[0x110..0x114].fill(0);
                wall.state = PublicWallBuildingState::from_observed(wall.projection);
                wall.base_arch_segment_heights.clear();
                wall.structural_arch_segment_heights.clear();
                wall.arch_segment_heights.clear();
                wall.segment_heights.clear();
                wall.base_segment_heights.clear();
                wall.gable_profile_pending = true;
                continue;
            };
            &path.path.positions
        } else {
            // PublishWall is a separate, explicitly resolved compatibility API.
            &input.outline
        };
        let extra = f32::from_le_bytes(wall.projection[0x284..0x288].try_into().unwrap());
        let segment_count = prepared
            .current(id, wall.publication_revision)
            .map_or(0, |record| record.path.segment_values.len());
        if input.shape.tag() >= 2 {
            if !collection.raw_sources.contains_key(&id) {
                report.faults.push(format!(
                    "Freeform cache requires raw source and prepared segments {}",
                    id.0
                ));
                continue;
            }
            let cache = country_core::recovered_building::building_wall_freeform_cache::freeform_wall_height_cache(
                positions, &input.shape, extra, wall.projection[0x295], segment_count,
            ).unwrap();
            wall.state = cache.write_projection(&mut wall.projection);
            wall.base_arch_segment_heights = cache.arch_segment_heights.clone();
            wall.structural_arch_segment_heights = cache.arch_segment_heights.clone();
            wall.arch_segment_heights = cache.arch_segment_heights;
            wall.segment_heights = cache.segment_heights;
            wall.base_segment_heights = wall.segment_heights.clone();
            wall.gable_profile_pending = false;
            continue;
        }
        match regular_wall_height_cache(positions, &input.shape, extra) {
            Ok(cache) => {
                wall.state = cache.write_projection(&mut wall.projection);
                wall.base_arch_segment_heights = cache.arch_segment_heights.clone();
                wall.structural_arch_segment_heights = cache.arch_segment_heights.clone();
                wall.arch_segment_heights = cache.arch_segment_heights;
                wall.segment_heights = country_core::recovered_building::building_wall_construction_sources::initialize_segment_height_array(segment_count);
                wall.base_segment_heights = wall.segment_heights.clone();
            }
            Err(e) => report.faults.push(format!("Wall cache {}: {e:?}", id.0)),
        }
    }
}
fn create_roofs(
    mut commands: Commands,
    mut requests: EventReader<CreateRoof>,
    walls: Res<PublicWalls>,
    mut gable: EventWriter<WallGableDirty>,
    mut report: ResMut<BuildingReport>,
) {
    for request in requests.read() {
        let Some(wall) = walls.walls.get(&request.0.wall_id()) else {
            continue;
        };
        match prepare_resolved_roof_components([0; 88], [0; 24], &request.0, &wall.state) {
            Ok(c) => {
                let entity = commands
                    .spawn(RoofBundle {
                        roof: Roof(c.roof),
                        tile: TileSimulation(c.tile_simulation),
                        animation: Animation(c.animation),
                        trigger: TriggerVolume::default(),
                        tiles: RoofTiles::default(),
                        spire: RoofSpire,
                        supports: RoofSupportBeams,
                        nani: nani::NaniStoneFloorBundle::default(),
                    })
                    .id();
                report.created.push(entity);
                if let Some(id) = c.creation.gable_notification {
                    gable.write(WallGableDirty(id));
                }
            }
            Err(e) => report
                .faults
                .push(format!("Roof create {}: {e:?}", request.0.wall_id().0)),
        }
    }
}
fn associations(
    roofs: Query<(Entity, &Roof, Has<RoofMarkedForDeletion>)>,
    walls: Res<PublicWalls>,
    mut map: ResMut<WallRoofMap>,
    frame: Res<BuildingFrame>,
    mut invalid: EventWriter<roof_invalid::RoofIsInvalid>,
    mut presence: EventWriter<WallRoofPresenceChanged>,
    mut report: ResMut<BuildingReport>,
) {
    let rows = roofs.iter().map(|(entity, roof, marked)| {
        let wall_id = WallId(u64::from_le_bytes(roof.0[..8].try_into().unwrap()));
        ResolvedRoofAssociation {
            wall_id,
            entity: entity.to_bits(),
            marked,
            public_wall: walls.walls.get(&wall_id).map(|w| &w.state),
        }
    });
    match map.0.rebuild(rows, &walls.order, frame.phase) {
        Ok(changes) => {
            for id in changes.invalid_roof_requests {
                invalid.write(roof_invalid::RoofIsInvalid(id));
            }
            for id in changes.wall_presence_changes {
                presence.write(WallRoofPresenceChanged(id));
                report.presence_changes.push(id);
            }
        }
        Err(e) => report.faults.push(format!("Roof association: {e:?}")),
    }
}
fn spatial(
    mut dirty: EventReader<WallSpatialDirty>,
    walls: Res<PublicWalls>,
    mut roofs: Query<(Entity, &mut Roof, &mut ObjectTransform), Without<RoofMarkedForDeletion>>,
    frame: Res<BuildingFrame>,
    mut geometry: EventWriter<RoofGeometryChanged>,
    mut report: ResMut<BuildingReport>,
) {
    let ids: Vec<_> = dirty.read().map(|e| e.0).collect();
    for (entity, mut roof, mut transform) in &mut roofs {
        let id = WallId(u64::from_le_bytes(roof.0[..8].try_into().unwrap()));
        if !ids.contains(&id) {
            continue;
        }
        let Some(wall) = walls.walls.get(&id) else {
            continue;
        };
        let mut rt = 0;
        let mut tt = 0;
        match synchronize_resolved_roof(
            &mut roof.0,
            &mut transform.0,
            &wall.state,
            id,
            entity.to_bits(),
            &mut rt,
            &mut tt,
            frame.tick,
        ) {
            Ok(result) => {
                for change in result.geometry_changes {
                    let event = RoofGeometryChanged {
                        entity,
                        wall: id,
                        kind: change.observed_kind,
                    };
                    geometry.write(event);
                    report.geometry_changes.push(event);
                }
            }
            Err(e) => report.faults.push(format!("Roof spatial {}: {e:?}", id.0)),
        }
    }
}
fn mark_roofs(
    mut commands: Commands,
    mut requests: EventReader<DeleteRoof>,
    roofs: Query<(Entity, &Roof), Without<RoofMarkedForDeletion>>,
    mut sound: EventWriter<RoofSound>,
    mut report: ResMut<BuildingReport>,
) {
    let ids: Vec<_> = requests.read().map(|e| e.0).collect();
    for (entity, roof) in &roofs {
        if let Some(mark) = match_deletion_request(&ids, entity.to_bits(), &roof.0) {
            commands.entity(entity).insert(RoofMarkedForDeletion);
            report.marked.push(entity);
            let event = RoofSound {
                entity,
                wall: mark.wall_id,
                pivot: mark.pivot,
                kind: mark.sound_event_kind,
            };
            sound.write(event);
            report.sounds.push(event);
        }
    }
}
fn animate(
    mut roofs: Query<(
        &Roof,
        &mut Animation,
        Has<RoofMarkedForDeletion>,
        &mut TileSimulation,
    )>,
    frame: Res<BuildingFrame>,
) {
    for (roof, mut animation, marked, mut tile) in &mut roofs {
        let mut tick = 0;
        // wall_height_dt writes delta at TileSimulation+0 and last max-y at +4.
        let mut bytes = *tile.0.bytes();
        let current = f32::from_le_bytes(roof.0[0x4c..0x50].try_into().unwrap());
        let previous = f32::from_le_bytes(bytes[4..8].try_into().unwrap());
        bytes[0..4].copy_from_slice(&(current - previous).to_le_bytes());
        bytes[4..8].copy_from_slice(&current.to_le_bytes());
        tile.0 = RoofTileSimulation::from_observed(bytes);
        animation.bypass_change_detection().0.advance(
            &roof.0,
            marked,
            frame.delta_time,
            &mut tick,
            frame.tick,
        );
        if tick == frame.tick {
            animation.set_changed();
        }
    }
}
fn delete_roofs(
    mut commands: Commands,
    roofs: Query<(Entity, &Animation), With<RoofMarkedForDeletion>>,
    mut report: ResMut<BuildingReport>,
) {
    for (entity, animation) in &roofs {
        if should_despawn(true, &animation.0) {
            commands.entity(entity).despawn();
            report.despawned.push(entity);
        }
    }
}

/// CPU output of the previously recovered roof tile pipeline. Rendering and
/// GPU allocation stay at the deferred boundary; records are usable artifacts.
#[derive(Component, Debug)]
pub struct RoofCpuTiles(pub country_core::recovered_building::roof_pipeline::RoofTileBatch);
fn build_cpu_tiles(
    mut commands: Commands,
    roofs: Query<(Entity, &Roof), (Changed<Roof>, Without<RoofMarkedForDeletion>)>,
    mut report: ResMut<BuildingReport>,
) {
    for (entity, roof) in &roofs {
        let h = f32::from_le_bytes(roof.0[0x30..0x34].try_into().unwrap());
        if !country_core::recovered_building::roof_state::ty_observed(h) {
            commands.entity(entity).remove::<RoofCpuTiles>();
            continue;
        }
        let rectangle: [u8; 24] = roof.0[12..36].try_into().unwrap();
        match country_core::recovered_building::roof_pipeline::assemble_roof_tiles_observed(
            &roof.0,
            (roof.0[8] & 1 != 0).then_some(&rectangle),
            false,
        ) {
            Ok(batch) => {
                commands.entity(entity).insert(RoofCpuTiles(batch));
            }
            Err(e) => report
                .faults
                .push(format!("Roof CPU tiles {entity:?}: {e:?}")),
        }
    }
}

fn fill_gable_cache(
    mut gable: EventReader<WallGableDirty>,
    mut spatial: EventReader<WallSpatialDirty>,
    mut presence: EventReader<WallRoofPresenceChanged>,
    mut walls: ResMut<PublicWalls>,
    collection: Res<WallCollectionState>,
    prepared: Res<wall_paths::WallPreparedPaths>,
    map: Res<WallRoofMap>,
    roofs: Query<&Roof>,
    mut report: ResMut<BuildingReport>,
) {
    let mut ids: Vec<_> = gable
        .read()
        .map(|e| e.0)
        .chain(spatial.read().map(|e| e.0))
        .chain(presence.read().map(|e| e.0))
        .collect();
    // Reassociation runs after deletion; the event also reaches this consumer
    // next update through Bevy's independent reader cursor.
    ids.sort_unstable_by_key(|id| id.0);
    ids.dedup();
    for id in ids {
        let Some(wall) = walls.walls.get_mut(&id) else {
            continue;
        };
        if wall.source.shape.tag() >= 2 {
            continue;
        }
        let Ok(base) = wall.state.max_y() else {
            continue;
        };
        let roof = map
            .entity(id)
            .and_then(|entity| roofs.get(entity).ok())
            .map(|r| &r.0);
        let source_length = f32::from_le_bytes(wall.projection[0x28c..0x290].try_into().unwrap());
        let positions = if collection.raw_sources.contains_key(&id) {
            let Some(path) = prepared.current(id, wall.publication_revision) else {
                continue;
            };
            &path.path.positions
        } else {
            &wall.source.outline
        };
        let result = if let Some(roof) = roof {
            country_core::recovered_building::building_wall_gable::apply_gable_arch_heights(
                positions,
                &wall.base_arch_segment_heights,
                base,
                source_length,
                wall.state.is_halftimbered(),
                roof,
            )
        } else {
            Ok(
                country_core::recovered_building::building_wall_gable::GableArchHeights {
                    heights: wall.base_arch_segment_heights.clone(),
                    maximum_y: base,
                    curve_u: Vec::new(),
                    closing_endpoint_offset: None,
                },
            )
        };
        match result {
            Ok(result) => {
                wall.structural_arch_segment_heights = result.heights.clone();
                wall.arch_segment_heights = result.heights;
                wall.projection[0x110..0x114].copy_from_slice(&1u32.to_le_bytes());
                wall.projection[0x114..0x118].copy_from_slice(&result.maximum_y.to_le_bytes());
                wall.state = PublicWallBuildingState::from_observed(wall.projection);
                wall.gable_profile_pending = false;
            }
            Err(error) => {
                wall.gable_profile_pending = true;
                report
                    .faults
                    .push(format!("Wall gable {}: {error:?}", id.0));
            }
        }
    }
}

/// Authoritative source mutations enter here; no caller-supplied resampled
/// arrays or terrain samples are required by this path.
#[derive(Event, Clone)]
pub struct UpsertRawWall {
    pub wall: WallId,
    pub source:
        country_core::recovered_building::building_wall_collection::RawWallPublicationSource,
}
#[derive(Event, Clone)]
pub struct SetTerrainGrid {
    pub grid: country_core::recovered_building::building_wall_collection::TerrainHeightGrid512,
    pub affected_walls: Vec<WallId>,
}
/// Candidate IDs are resolved by the spatial-index owner. The original two
/// dirty subscriptions have distinct policies; their source names remain unknown.
#[derive(Event, Clone)]
pub struct TerrainDirtyCandidates {
    pub subscription:
        country_core::recovered_building::building_wall_collection::DirtyPublicationSubscription,
    pub walls: Vec<WallId>,
    pub already_lazy_dirty: std::collections::HashSet<WallId>,
}
/// Original 4096 spatial cells represented by 64 words. The collector resolves
/// wall IDs from the original raw-curve memberships before subscriber filtering.
#[derive(Event, Clone)]
pub struct TerrainDirtyTiles {
    pub subscription:
        country_core::recovered_building::building_wall_collection::DirtyPublicationSubscription,
    pub mask: [u64; 64],
    pub already_lazy_dirty: std::collections::HashSet<WallId>,
}
#[derive(Resource, Default)]
pub struct WallCollectionState {
    pub raw_sources: HashMap<
        WallId,
        country_core::recovered_building::building_wall_collection::RawWallPublicationSource,
    >,
    pub terrain:
        Option<country_core::recovered_building::building_wall_collection::TerrainHeightGrid512>,
    pub spatial_index:
        country_core::recovered_building::building_wall_spatial_index::WallSpatialIndex,
    dirty: std::collections::HashSet<WallId>,
}
fn collect_raw_inputs(
    mut source_events: EventReader<UpsertRawWall>,
    mut terrain_events: EventReader<SetTerrainGrid>,
    mut dirty_events: EventReader<TerrainDirtyCandidates>,
    mut tile_events: EventReader<TerrainDirtyTiles>,
    mut removed: EventReader<RemoveWall>,
    mut heights: EventReader<wall_height::SetWallSourceHeight>,
    mut height_updates: ResMut<wall_height::WallHeightUpdates>,
    mut height_commands: EventReader<wall_height::AdjustWallHeight>,
    mut height_command_updates: ResMut<wall_height::WallHeightCommandUpdates>,
    mut height_audio: EventWriter<wall_height::WallHeightAudio>,
    height_environment: wall_height::WallHeightEnvironment,
    mut collection: ResMut<WallCollectionState>,
    walls: Res<PublicWalls>,
    mut publish: EventWriter<PublishWall>,
    mut report: ResMut<BuildingReport>,
) {
    use country_core::recovered_building::building_wall_collection::{
        collect_raw_wall_publication, dirty_tile_candidate, terrain_requires_republication,
    };
    let mut terrain_candidates = std::collections::HashSet::new();
    for event in terrain_events.read() {
        collection.terrain = Some(event.grid.clone());
        terrain_candidates.extend(event.affected_walls.iter().copied());
    }
    for event in source_events.read() {
        let mut source = event.source.clone();
        // Original publisher takes the segmentation tag from this same shape,
        // preventing impossible independent shape/curve tag combinations.
        source.curve_kind_tag = source.shape.tag();
        collection
            .spatial_index
            .replace_curve(event.wall, &source.curve);
        collection.raw_sources.insert(event.wall, source);
        collection.dirty.insert(event.wall);
    }
    for event in removed.read() {
        collection.raw_sources.remove(&event.0);
        collection.spatial_index.remove(event.0);
        collection.dirty.remove(&event.0);
    }
    height_updates.0.clear();
    for event in heights.read() {
        if let Some(source) = collection.raw_sources.get_mut(&event.wall) {
            let outcome =
                country_core::recovered_building::building_wall_height_authority::assign_resolved_height(
                    &mut source.extra,
                    event.wall,
                    event.height,
                );
            collection.dirty.insert(event.wall);
            height_updates.0.push(outcome);
        }
    }
    height_command_updates.0.clear();
    for event in height_commands.read() {
        let request = &event.request;
        if request.mode == 0 {
            continue;
        }
        if request.mode != 1 {
            report.faults.push(format!(
                "Unsupported wall height scalar mode {}",
                request.mode
            ));
            return;
        }
        let Some(terrain) = collection.terrain.clone() else {
            report
                .faults
                .push("Wall height command requires terrain".into());
            return;
        };
        let Some(source) = collection.raw_sources.get_mut(&request.wall) else {
            report.faults.push(format!(
                "Wall height command missing authority {}",
                request.wall.0
            ));
            return;
        };
        let roof_height_01 = height_environment
            .associations
            .entity(request.wall)
            .and_then(|entity| height_environment.roofs.get(entity).ok())
            .map(|roof| f32::from_le_bytes(roof.0[0x30..0x34].try_into().unwrap()));
        let clamp = walls.walls.get(&request.wall).map(|wall| {
            country_core::recovered_building::building_wall_height_command::ResolvedWallHeightClamp {
                roof_height_01,
                special_flag: wall.projection[0x296],
            }
        });
        match country_core::recovered_building::building_wall_height_command::apply_resolved_height_command(
            request,
            &mut source.extra,
            source.curve_kind_tag,
            &source.curve,
            &terrain,
            clamp,
            height_environment.frame.delta_time,
        ) {
            Ok(Some(outcome)) => {
                height_audio.write(wall_height::WallHeightAudio(outcome.feedback));
                height_updates
                    .0
                    .extend(outcome.authority_updates.iter().copied());
                collection.dirty.insert(request.wall);
                height_command_updates.0.push(outcome);
            }
            Ok(None) => {}
            Err(error) => {
                report
                    .faults
                    .push(format!("Wall height command {}: {error:?}", request.wall.0));
                return;
            }
        }
    }
    let candidates = dirty_events
        .read()
        .cloned()
        .chain(tile_events.read().map(|event| TerrainDirtyCandidates {
            subscription: event.subscription,
            walls: collection.spatial_index.candidates(&event.mask),
            already_lazy_dirty: event.already_lazy_dirty.clone(),
        }))
        .collect::<Vec<_>>();
    for event in &candidates {
        for id in &event.walls {
            let Some(wall) = walls.walls.get(id) else {
                continue;
            };
            if wall.state.validate_read().is_err() {
                continue;
            }
            let relative = f32::from_le_bytes(wall.projection[0x27c..0x280].try_into().unwrap());
            if dirty_tile_candidate(
                event.subscription,
                relative,
                event.already_lazy_dirty.contains(id),
            ) {
                match event.subscription {
                    country_core::recovered_building::building_wall_collection::DirtyPublicationSubscription::Primary=>{collection.dirty.insert(*id);}
                    country_core::recovered_building::building_wall_collection::DirtyPublicationSubscription::Secondary=>{terrain_candidates.insert(*id);}
                }
            }
        }
    }
    if let Some(terrain) = &collection.terrain {
        let filtered: Vec<_> = terrain_candidates
            .into_iter()
            .filter(
                |id| match (collection.raw_sources.get(id), walls.walls.get(id)) {
                    (Some(source), Some(wall)) => terrain_requires_republication(
                        &wall.source.outline,
                        source.terrain_floor,
                        terrain,
                    ),
                    _ => false,
                },
            )
            .collect();
        collection.dirty.extend(filtered);
    }
    if collection.terrain.is_none() {
        if !collection.dirty.is_empty() {
            report
                .faults
                .push("Raw wall collection requires an explicit terrain grid".into());
        }
        return;
    }
    let mut ids: Vec<_> = collection.dirty.drain().collect();
    ids.sort_unstable_by_key(|id| id.0);
    let terrain = collection.terrain.as_ref().unwrap();
    for id in ids {
        let Some(source) = collection.raw_sources.get(&id) else {
            continue;
        };
        match collect_raw_wall_publication(source, terrain) {
            Ok(collected) => {
                publish.write(PublishWall {
                    wall: id,
                    input: WallPublicationInput {
                        outline: collected.outline,
                        terrain_y: collected.terrain_y,
                        source_curve_length: collected.original_curve_length,
                        source_point_count: source.curve.points.len(),
                        shape: source.shape.clone(),
                        style: source.style,
                        extra: source.extra,
                        special: source.special,
                    },
                });
            }
            Err(error) => report.faults.push(format!("Raw wall {}: {error:?}", id.0)),
        }
    }
}

#[derive(Event, Clone, Copy, Debug)]
pub struct SwitchRoofDirection(
    pub country_core::recovered_building::building_roof_direction::RoofDirectionRequest,
);
fn switch_directions(
    mut events: EventReader<SwitchRoofDirection>,
    map: Res<WallRoofMap>,
    mut roofs: Query<&mut Roof>,
    frame: Res<BuildingFrame>,
    mut geometry: EventWriter<RoofGeometryChanged>,
    mut gable: EventWriter<WallGableDirty>,
    mut sound: EventWriter<RoofSound>,
    mut report: ResMut<BuildingReport>,
) {
    for event in events.read() {
        let Some(entity) = map.entity(event.0.wall) else {
            report.faults.push(format!(
                "Roof direction missing association {}",
                event.0.wall.0
            ));
            return;
        };
        let Ok(mut roof) = roofs.get_mut(entity) else {
            report
                .faults
                .push(format!("Roof direction invalid entity {entity:?}"));
            return;
        };
        let mut tick = 0;
        let change =
            country_core::recovered_building::building_roof_direction::apply_resolved_direction(
                &mut roof.0,
                event.0,
                entity.to_bits(),
                &mut tick,
                frame.tick,
            );
        let update = RoofGeometryChanged {
            entity,
            wall: change.wall,
            kind: change.geometry_kind,
        };
        geometry.write(update);
        report.geometry_changes.push(update);
        gable.write(WallGableDirty(change.wall));
        let feedback = RoofSound {
            entity,
            wall: change.wall,
            pivot: change.sound_pivot,
            kind: change.sound_kind,
        };
        sound.write(feedback);
        report.sounds.push(feedback);
    }
}

#[derive(Event, Clone, Copy, Debug)]
pub struct SetFlatRoofStyle(
    pub country_core::recovered_building::building_roof_customization::FlatRoofStyleRequest,
);
#[derive(Event, Clone, Copy, Debug)]
pub struct SetFlatRoofRimStyle(
    pub country_core::recovered_building::building_roof_customization::FlatRoofRimRequest,
);
#[derive(Event, Clone, Copy, Debug)]
pub struct RecalculateWallHoles(pub WallId);
/// Adapter requests awaiting the history owner, not committed transactions.
#[derive(Resource, Default)]
pub struct RoofRimHistoryRequests(
    pub Vec<country_core::recovered_building::building_roof_customization::RoofRimHistoryMutation>,
);
/// Already decoded original replay producer output; cursor selection and
/// complete history transaction semantics are outside this dispatch adapter.
#[derive(Event, Clone, Debug)]
pub struct ReplayRoofEvent(pub country_core::recovered_building::roof_replay::ObservedReplayEvent);
fn route_replay(
    mut events: EventReader<ReplayRoofEvent>,
    mut create: EventWriter<CreateRoof>,
    mut delete: EventWriter<DeleteRoof>,
    mut direction: EventWriter<SwitchRoofDirection>,
    mut rim: EventWriter<SetFlatRoofRimStyle>,
) {
    use country_core::recovered_building::roof_replay::ObservedReplayEvent;
    for event in events.read() {
        match &event.0 {
            ObservedReplayEvent::Create { .. } => {
                // All authoritative payload bytes come from the explicit
                // presence44. Queue message ID/padding are not game input.
                let mut record = [0; 64];
                event.0.write_observed_fields(&mut record);
                create.write(CreateRoof(RoofCreateRequestObserved::from_observed(record)));
            }
            ObservedReplayEvent::Delete { wall_index } => {
                delete.write(DeleteRoof(WallId(*wall_index)));
            }
            ObservedReplayEvent::Direction {
                wall_index,
                direction: value,
            } => {
                direction.write(SwitchRoofDirection(
                    country_core::recovered_building::building_roof_direction::RoofDirectionRequest {
                        wall: WallId(*wall_index),
                        direction: *value,
                    },
                ));
            }
            ObservedReplayEvent::FlatRimStyle { wall_index, style } => {
                rim.write(SetFlatRoofRimStyle(
                    country_core::recovered_building::building_roof_customization::FlatRoofRimRequest {
                        wall_id: WallId(*wall_index),
                        from: *style,
                        to: *style,
                        observed_record_history: 0,
                    },
                ));
            }
        }
    }
}
fn customize_roofs(
    mut styles: EventReader<SetFlatRoofStyle>,
    mut rims: EventReader<SetFlatRoofRimStyle>,
    map: Res<WallRoofMap>,
    mut roofs: Query<&mut Roof>,
    frame: Res<BuildingFrame>,
    mut holes: EventWriter<RecalculateWallHoles>,
    mut sounds: EventWriter<RoofSound>,
    mut history: ResMut<RoofRimHistoryRequests>,
    mut report: ResMut<BuildingReport>,
) {
    use country_core::recovered_building::building_roof_customization::*;
    // Original processes the full-style queue before the separate rim queue.
    let requests = styles
        .read()
        .map(|e| (e.0.wall_id, Some(e.0), None))
        .chain(rims.read().map(|e| (e.0.wall_id, None, Some(e.0))));
    for (wall, style, rim) in requests {
        // Unlike direction's unwrap, these original consumers skip absent
        // associations or query rows. No deletion-marked filter is present.
        let Some(entity) = map.entity(wall) else {
            continue;
        };
        let Ok(mut roof) = roofs.get_mut(entity) else {
            continue;
        };
        let mut tick = 0;
        let outcome = if let Some(request) = style {
            apply_flat_style_resolved(&mut roof.0, request, &mut tick, frame.tick)
        } else {
            apply_flat_rim_resolved(&mut roof.0, rim.unwrap(), &mut tick, frame.tick)
        };
        if let Some(wall) = outcome.recalculate_wall_holes {
            holes.write(RecalculateWallHoles(wall));
            report.wall_holes_requests.push(wall);
        }
        if let Some(sound) = outcome.sound {
            let sound = RoofSound {
                entity,
                wall,
                pivot: sound.pivot,
                kind: sound.observed_kind,
            };
            sounds.write(sound);
            report.sounds.push(sound);
        }
        if let Some(mutation) = outcome.history_mutation {
            history.0.push(mutation);
            report.rim_history_requests.push(mutation);
        }
    }
}
