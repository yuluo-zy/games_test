//! Shipped collision asset + raw Curve2 input feeding the real opening chain.
//! Keeping a world request across host changes is an explicit CPU adapter;
//! original anchor dragging, roof morph and visual bundles are separate owners.
use crate::decorator_assets::DecoratorAssetLibrary;
use crate::openings::{OpeningAuthority, OpeningId, RemoveOpening, UpsertOpening, WithdrawOpening};
use crate::{BuildingReport, PublicWalls, Roof, WallCollectionState, WallRoofMap};
use bevy_ecs::prelude::*;
use std::collections::HashMap;
use tiny_glade_reconstruction::{
    building_decorator_placement::WallDecoratorRequest,
    building_decorator_subtype::WallDecoratorSubtypeContext, building_opening_rules::OpeningFamily,
    building_wall_state::WallId,
};

#[derive(Event, Clone, Copy, Debug)]
pub struct PlaceWallOpening {
    pub id: OpeningId,
    pub wall: WallId,
    pub request: WallDecoratorRequest,
}
#[derive(Clone, Debug)]
pub struct WallOpeningIntent {
    pub source: PlaceWallOpening,
    pub projected_world_position: Option<[f32; 3]>,
    host_revision: Option<u64>,
    asset_revision: u64,
    dirty: bool,
    roof_state: Option<[u8; 88]>,
    arch_height_bits: Vec<u32>,
    prepared_generation: Option<u64>,
}
#[derive(Resource, Default, Debug)]
pub struct WallOpeningIntents(pub HashMap<OpeningId, WallOpeningIntent>);

pub fn register(app: &mut bevy_app::App) {
    app.init_resource::<DecoratorAssetLibrary>()
        .init_resource::<WallOpeningIntents>()
        .add_event::<PlaceWallOpening>()
        .add_systems(
            bevy_app::Update,
            resolve.in_set(crate::BuildingStage::Placement),
        );
}
fn resolve(
    mut input: EventReader<PlaceWallOpening>,
    mut removed: EventReader<RemoveOpening>,
    mut intents: ResMut<WallOpeningIntents>,
    collection: Res<WallCollectionState>,
    prepared: Res<crate::wall_paths::WallPreparedPaths>,
    walls: Res<PublicWalls>,
    assets: Res<DecoratorAssetLibrary>,
    associations: Res<WallRoofMap>,
    roofs: Query<&Roof>,
    mut outputs: EventWriter<UpsertOpening>,
    mut withdraw: EventWriter<WithdrawOpening>,
    mut report: ResMut<BuildingReport>,
) {
    for event in input.read() {
        intents.0.insert(
            event.id,
            WallOpeningIntent {
                source: *event,
                projected_world_position: None,
                host_revision: None,
                asset_revision: assets.revision,
                dirty: true,
                roof_state: None,
                arch_height_bits: Vec::new(),
                prepared_generation: None,
            },
        );
    }
    for event in removed.read() {
        intents.0.remove(&event.0);
    }
    for intent in intents.0.values_mut() {
        let Some(host) = walls.walls.get(&intent.source.wall) else {
            // OpeningState clears the old contribution in the same frame.
            intent.host_revision = None;
            intent.projected_world_position = None;
            intent.dirty = true;
            intent.prepared_generation = None;
            continue;
        };
        let Some(path) = prepared.current(intent.source.wall, host.publication_revision) else {
            withdraw.write(WithdrawOpening(intent.source.id));
            intent.projected_world_position = None;
            intent.prepared_generation = None;
            intent.dirty = true;
            continue;
        };
        let roof_state = associations
            .entity(intent.source.wall)
            .and_then(|entity| roofs.get(entity).ok())
            .map(|roof| roof.0);
        let arch_height_bits: Vec<_> = host
            .arch_segment_heights
            .iter()
            .map(|height| height.to_bits())
            .collect();
        if !intent.dirty
            && intent.host_revision == Some(host.publication_revision)
            && intent.asset_revision == assets.revision
            && intent.roof_state == roof_state
            && intent.arch_height_bits == arch_height_bits
            && intent.prepared_generation == Some(path.generation)
        {
            continue;
        }
        let Some(raw) = collection.raw_sources.get(&intent.source.wall) else {
            withdraw.write(WithdrawOpening(intent.source.id));
            intent.projected_world_position = None;
            report.faults.push(format!(
                "World opening {} requires the original raw Curve2",
                intent.source.id.0
            ));
            continue;
        };
        let door = matches!(
            intent.source.request.family,
            OpeningFamily::CottageBalconyDoor | OpeningFamily::GothicBalconyDoor
        );
        let context = WallDecoratorSubtypeContext {
            source_curve: &raw.curve,
            mapped_outline: &path.path.positions,
            arch_heights: &host.arch_segment_heights,
            wall_shape: &host.source.shape,
            roof: roof_state.as_ref(),
        };
        match assets.derive_and_place_wall(intent.source.request, door, false, context) {
            Ok(placement) => {
                outputs.write(UpsertOpening(OpeningAuthority {
                    id: intent.source.id,
                    wall: intent.source.wall,
                    input: placement.opening,
                }));
                intent.projected_world_position = Some(placement.projected_world_position);
                intent.host_revision = Some(host.publication_revision);
                intent.asset_revision = assets.revision;
                intent.dirty = false;
                intent.roof_state = roof_state;
                intent.arch_height_bits = arch_height_bits;
                intent.prepared_generation = Some(path.generation);
            }
            Err(error) => {
                withdraw.write(WithdrawOpening(intent.source.id));
                intent.projected_world_position = None;
                report
                    .faults
                    .push(format!("World opening {}: {error:?}", intent.source.id.0));
            }
        }
    }
}
