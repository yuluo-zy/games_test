#![doc = include_str!("../README.md")]
// Bevy system parameters remain explicit; grouping them changes the scheduling input API.
#![allow(clippy::too_many_arguments, clippy::type_complexity)]
pub mod decorator_assets;
pub mod decorator_info_binding;
pub mod decorator_placement;
pub mod fences;
pub mod gates;
pub mod history;
mod history_replay;
pub mod nani;
pub mod openings;
pub mod roof_edits;
pub mod roof_invalid;
mod roof_record;
mod roof_runtime;
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
mod wall_projection;
mod wall_publication;
pub mod wall_rectangle_edit;
pub mod wall_segments;
pub mod wall_structures;

use bevy_app::{App, Plugin, Update};
use bevy_ecs::{prelude::*, schedule::ApplyDeferred};
use country_core::resources::roofs::associations::RoofAssociations;
use country_core::resources::roofs::components::RoofAnimation;
use country_core::resources::roofs::components::RoofTileSimulation;
use country_core::resources::roofs::components::RoofTriggerVolume;
use country_core::resources::roofs::creation::RoofCreateRequestObserved;
use country_core::resources::walls::publication_fields::PublicWallBuildingState;
use country_core::resources::walls::publication_fields::WallId;
use country_core::resources::walls::publication_fields::WallShapeSnapshot;
use roof_runtime::{
    animate, associations, build_cpu_tiles, create_roofs, customize_roofs, delete_roofs,
    mark_roofs, roof_geometry_is_finite, route_replay, spatial, switch_directions,
};
use std::{collections::HashMap, sync::Arc};
use wall_publication::{
    collect_raw_inputs, fill_base_cache, fill_gable_cache, publish, raw_source_is_valid,
};

/// Terrain-mapped wall input. `UpsertRawWall` derives it from a raw curve and
/// explicit terrain; `PublishWall` accepts an already resolved representation.
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
    /// Initial segment heights before arches and lintels apply their contributions.
    pub base_segment_heights: Vec<f32>,
    pub publication_revision: u64,
}
impl PublishedWall {
    /// Serialized publication fields for diagnostics; this is not a Rust ABI layout.
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
        Vec<country_core::resources::roofs::customization::RoofRimHistoryMutation>,
}
impl BuildingReport {
    fn clear(&mut self) {
        self.faults.clear();
        self.created.clear();
        self.marked.clear();
        self.despawned.clear();
        self.presence_changes.clear();
        self.geometry_changes.clear();
        self.sounds.clear();
        self.wall_holes_requests.clear();
        self.rim_history_requests.clear();
    }
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
// These markers do not create geometry. Nani descriptors and transforms are
// installed explicitly by RoofBundle.
#[derive(Component, Clone, Copy, Debug, Default)]
pub struct RoofSpire;
#[derive(Component, Clone, Copy, Debug, Default)]
pub struct RoofSupportBeams;
/// Deferred GPU allocation slot. CPU generation leaves this empty.
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
    report.clear();
    if !frame.delta_time.is_finite() || frame.delta_time < 0.0 {
        report
            .faults
            .push("Building frame requires a finite nonnegative delta time".into());
        frame.delta_time = 0.0;
    }
    nani_requests.0.clear();
    solid_changes.0.clear();
}

/// Generated roof tile records. Rendering and GPU allocation are external.
#[derive(Component, Debug)]
pub struct RoofCpuTiles(pub country_core::resources::roofs::geometry::pipeline::RoofTileBatch);

/// Authoritative source mutations enter here; no caller-supplied resampled
/// arrays or terrain samples are required by this path.
#[derive(Event, Clone)]
pub struct UpsertRawWall {
    pub wall: WallId,
    pub source:
        country_core::resources::walls::publication_source::RawWallPublicationSource,
}
#[derive(Event, Clone)]
pub struct SetTerrainGrid {
    pub grid: country_core::resources::walls::publication_source::TerrainHeightGrid512,
    pub affected_walls: Vec<WallId>,
}
/// Explicit spatial-index candidates, filtered by the selected dirty subscription.
/// Primary and secondary subscriptions retain their distinct filtering policies.
#[derive(Event, Clone)]
pub struct TerrainDirtyCandidates {
    pub subscription:
        country_core::resources::walls::publication_source::DirtyPublicationSubscription,
    pub walls: Vec<WallId>,
    pub already_lazy_dirty: std::collections::HashSet<WallId>,
}
/// Dirty mask of 4,096 spatial cells represented by 64 words. The collector
/// resolves raw-curve memberships before applying subscription filtering.
#[derive(Event, Clone)]
pub struct TerrainDirtyTiles {
    pub subscription:
        country_core::resources::walls::publication_source::DirtyPublicationSubscription,
    pub mask: [u64; 64],
    pub already_lazy_dirty: std::collections::HashSet<WallId>,
}
#[derive(Resource, Default)]
pub struct WallCollectionState {
    pub raw_sources: HashMap<
        WallId,
        country_core::resources::walls::publication_source::RawWallPublicationSource,
    >,
    pub terrain:
        Option<country_core::resources::walls::publication_source::TerrainHeightGrid512>,
    pub spatial_index:
        country_core::resources::walls::spatial_index::WallSpatialIndex,
    dirty: std::collections::HashSet<WallId>,
}

#[derive(Event, Clone, Copy, Debug)]
pub struct SwitchRoofDirection(
    pub country_core::resources::roofs::direction::RoofDirectionRequest,
);

#[derive(Event, Clone, Copy, Debug)]
pub struct SetFlatRoofStyle(
    pub country_core::resources::roofs::customization::FlatRoofStyleRequest,
);
#[derive(Event, Clone, Copy, Debug)]
pub struct SetFlatRoofRimStyle(
    pub country_core::resources::roofs::customization::FlatRoofRimRequest,
);
#[derive(Event, Clone, Copy, Debug)]
pub struct RecalculateWallHoles(pub WallId);
/// Adapter requests awaiting the history owner, not committed transactions.
#[derive(Resource, Default)]
pub struct RoofRimHistoryRequests(
    pub Vec<country_core::resources::roofs::customization::RoofRimHistoryMutation>,
);
/// Decoded replay output; cursor selection and
/// complete history transaction semantics are outside this dispatch adapter.
#[derive(Event, Clone, Debug)]
pub struct ReplayRoofEvent(pub country_core::resources::roofs::geometry::replay::ObservedReplayEvent);
