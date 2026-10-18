//! Projects retained world-space placement requests into wall opening authority.
//! Host, roof, asset, and height changes invalidate the cached projection.
use crate::decorator_assets::DecoratorAssetLibrary;
use crate::openings::{OpeningAuthority, OpeningId, RemoveOpening, UpsertOpening, WithdrawOpening};
use crate::{BuildingReport, PublicWalls, Roof, WallCollectionState, WallRoofMap};
use bevy_ecs::prelude::*;
use country_core::resources::walls::decorator::placement::WallDecoratorRequest;
use country_core::resources::walls::decorator_subtype::placement::WallDecoratorSubtypeContext;
use country_core::resources::walls::wall_holes::opening_rules::OpeningFamily;
use country_core::resources::walls::publication_fields::WallId;
use std::collections::{HashMap, HashSet};

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
    native_ids: Res<crate::decorator_info_binding::NativeDecoratorOpeningIds>,
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
    // Native storage owns bound identities. Retire the old adapter intent
    // without removing the actual replacement opening or its contribution.
    let native_openings: HashSet<_> = native_ids.0.values().copied().collect();
    intents.0.retain(|id, _| !native_openings.contains(id));
    for event in input.read() {
        if native_openings.contains(&event.id) {
            continue;
        }
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
        let Some(raw) = collection.raw_sources.get(&intent.source.wall) else {
            withdraw.write(WithdrawOpening(intent.source.id));
            intent.projected_world_position = None;
            intent.dirty = true;
            report.faults.push(format!(
                "World opening {} requires the original raw Curve2",
                intent.source.id.0
            ));
            continue;
        };
        let roof_state = associations
            .entity(intent.source.wall)
            .and_then(|entity| roofs.get(entity).ok())
            .map(|roof| roof.0);
        if !intent.dirty
            && intent.host_revision == Some(host.publication_revision)
            && intent.asset_revision == assets.revision
            && intent.roof_state == roof_state
            && intent.arch_height_bits.iter().copied().eq(host
                .arch_segment_heights
                .iter()
                .map(|height| height.to_bits()))
            && intent.prepared_generation == Some(path.generation)
        {
            continue;
        }
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
                intent.arch_height_bits.clear();
                intent.arch_height_bits.extend(
                    host.arch_segment_heights
                        .iter()
                        .map(|height| height.to_bits()),
                );
                intent.prepared_generation = Some(path.generation);
            }
            Err(error) => {
                // Withdrawing destroys the projected entity. Even if inputs
                // return to the last successful values, it must be recreated.
                intent.dirty = true;
                withdraw.write(WithdrawOpening(intent.source.id));
                intent.projected_world_position = None;
                report
                    .faults
                    .push(format!("World opening {}: {error:?}", intent.source.id.0));
            }
        }
    }
}
