//! Resolves serialized decorator storage and asset context into wall openings.
//! Native decorator IDs are explicitly bound to unique adapter opening IDs.
use crate::{
    BuildingReport, PublicWalls, Roof, WallCollectionState, WallRoofMap,
    decorator_assets::DecoratorAssetLibrary,
    openings::{
        OpeningAuthority, OpeningEntities, OpeningId, RemoveOpening, UpsertOpening, WithdrawOpening,
    },
    wall_decorator_archivist::WallDecoratorArchiveState,
};
use bevy_ecs::prelude::*;
use country_core::resources::walls::decorator::anchors::DecoratorInfoError;
use country_core::resources::walls::decorator::anchors::DecoratorInfoObserved;
use country_core::resources::walls::decorator::anchors::DecoratorPlaceInsertError;
use country_core::resources::walls::decorator::anchors::DecoratorPlaceRequestObserved;
use country_core::resources::walls::decorator::anchors::decorator_source_from_place;
use country_core::resources::walls::decorator::anchors::insert_decorator_source;
use country_core::resources::walls::decorator::placement::DecoratorLibKey;
use country_core::resources::walls::decorator::placement::wall_decorator_key;
use country_core::resources::walls::decorator_subtype::placement::WallDecoratorSubtypeContext;
use country_core::resources::walls::decorator_subtype::placement::WallWindowKind;
use country_core::resources::walls::decorator_subtype::placement::derive_wall_subtype;
use country_core::resources::walls::decorator_subtype::placement::wall_attachment_subtype;
use country_core::resources::walls::decorator_subtype::placement::wall_opening_family;
use country_core::resources::walls::wall_holes::opening_rules::DecoratorOpeningInput;
use country_core::resources::walls::wall_holes::opening_rules::OpeningFamily;
use std::collections::{HashMap, HashSet};
#[derive(Resource, Default)]
pub struct NativeDecoratorOpeningIds(pub HashMap<u32, OpeningId>);
impl NativeDecoratorOpeningIds {
    pub fn owns(&self, id: OpeningId) -> bool {
        self.0.values().any(|v| *v == id)
    }
    pub fn owns_native(&self, id: u32) -> bool {
        self.0.contains_key(&id)
    }
}
#[derive(Event, Clone, Copy)]
pub struct ObserveNativeDecoratorOpeningId {
    pub decorator_id: u32,
    pub opening_id: OpeningId,
}
#[derive(Event, Clone, Copy)]
pub struct ForgetNativeDecoratorOpeningId(pub u32);
#[derive(Event, Clone, Copy)]
pub struct ObserveDecoratorStorageReadonly(pub bool);
#[derive(Event, Clone)]
pub struct ApplyObservedDecoratorPlace {
    pub request: DecoratorPlaceRequestObserved,
    pub observed_caller_byte: u8,
}
#[derive(Resource, Default)]
pub struct NativeDecoratorPlaceState {
    pub observed_storage_readonly: Option<bool>,
    pub queued: std::collections::VecDeque<ApplyObservedDecoratorPlace>,
    pub applied_this_frame: Vec<u32>,
    pub previous_records:
        Vec<country_core::resources::walls::decorator::anchors::StoredDecoratorPlace>,
    pub pending_reason: Option<String>,
    pub rejected_this_frame: usize,
}
#[derive(Resource, Default)]
pub struct NativeDecoratorOpeningState {
    pub pending: HashMap<u32, String>,
    pub resolved_this_frame: Vec<u32>,
    pub authoritative_inputs: HashMap<u32, OpeningAuthority>,
    owned_openings: HashSet<OpeningId>,
}
pub fn register(app: &mut bevy_app::App) {
    app.init_resource::<NativeDecoratorOpeningIds>()
        .init_resource::<NativeDecoratorOpeningState>()
        .init_resource::<NativeDecoratorPlaceState>()
        .add_event::<ObserveDecoratorStorageReadonly>()
        .add_event::<ApplyObservedDecoratorPlace>()
        .add_event::<ObserveNativeDecoratorOpeningId>()
        .add_event::<ForgetNativeDecoratorOpeningId>()
        .add_systems(
            bevy_app::Update,
            consume_ids
                .after(crate::BuildingStage::Begin)
                .before(crate::BuildingStage::Authoring),
        )
        .add_systems(
            bevy_app::Update,
            (
                consume_places,
                crate::wall_decorator_archivist::publish_projection,
            )
                .chain()
                .after(crate::BuildingStage::Authoring)
                .before(crate::BuildingStage::CollectRaw),
        )
        .add_systems(
            bevy_app::Update,
            resolve
                .after(crate::BuildingStage::Placement)
                .before(crate::BuildingStage::Openings),
        );
}
fn consume_ids(
    mut additions: EventReader<ObserveNativeDecoratorOpeningId>,
    mut removals: EventReader<ForgetNativeDecoratorOpeningId>,
    mut ids: ResMut<NativeDecoratorOpeningIds>,
    mut report: ResMut<BuildingReport>,
) {
    let removed: HashSet<_> = removals.read().map(|event| event.0).collect();
    if !removed.is_empty() {
        ids.0.retain(|native, _| !removed.contains(native));
    }
    // Removing a native ID dominates a simultaneous observation of that same
    // native ID. Its opening identity is available to other IDs in this batch.
    let additions: Vec<_> = additions
        .read()
        .copied()
        .filter(|event| !removed.contains(&event.decorator_id))
        .collect();
    if additions.is_empty() {
        return;
    }
    let mut desired = ids.0.clone();
    for event in &additions {
        desired.insert(event.decorator_id, event.opening_id);
    }
    let mut seen = HashSet::new();
    if desired.values().all(|opening| seen.insert(*opening)) {
        // Validate the final map so two bindings may exchange identities.
        ids.0 = desired;
        return;
    }
    // A malformed observation does not discard unrelated valid observations.
    for event in additions {
        if ids
            .0
            .iter()
            .any(|(native, opening)| *native != event.decorator_id && *opening == event.opening_id)
        {
            report.faults.push(format!(
                "Opening {} is already bound to another native decorator",
                event.opening_id.0
            ));
            continue;
        }
        ids.0.insert(event.decorator_id, event.opening_id);
    }
}
fn consume_places(
    mut inputs: EventReader<ApplyObservedDecoratorPlace>,
    mut readonly: EventReader<ObserveDecoratorStorageReadonly>,
    mut state: ResMut<NativeDecoratorPlaceState>,
    mut archive: ResMut<WallDecoratorArchiveState>,
    authoring: Res<crate::wall_authoring::WallAuthoringState>,
    mut report: ResMut<BuildingReport>,
) {
    state.applied_this_frame.clear();
    state.pending_reason = None;
    state.rejected_this_frame = 0;
    for event in readonly.read() {
        state.observed_storage_readonly = Some(event.0);
    }
    state.queued.extend(inputs.read().cloned());
    if state.queued.is_empty() {
        return;
    }
    let Some(mode) = authoring.dev_override_is_demo_mode else {
        state.pending_reason = Some("Actual DEV_OVERRIDE_IS_DEMO_MODE owner is required".into());
        return;
    };
    let Some(readonly) = state.observed_storage_readonly else {
        state.pending_reason = Some("Actual DecoratorStorage readonly flag is required".into());
        return;
    };
    let Some(storage) = archive.storage.as_mut() else {
        state.pending_reason = Some("Actual DecoratorStorage is required".into());
        return;
    };
    while let Some(event) = state.queued.front().cloned() {
        let source = match decorator_source_from_place(
            &event.request,
            mode,
            event.observed_caller_byte,
            readonly,
        ) {
            Ok(value) => value,
            Err(DecoratorInfoError::ReadonlyStorage) => {
                let fault = "Decorator placement pending: ReadonlyStorage".to_string();
                state.pending_reason = Some(fault.clone());
                report.faults.push(fault);
                break;
            }
            Err(error) => {
                report
                    .faults
                    .push(format!("Decorator placement rejected: {error:?}"));
                state.rejected_this_frame += 1;
                state.queued.pop_front();
                continue;
            }
        };
        match insert_decorator_source(storage, &source) {
            Ok(out) => {
                state.applied_this_frame.push(out.decorator_id.0);
                state.previous_records.push(out);
                state.queued.pop_front();
            }
            Err(error @ DecoratorPlaceInsertError::InvalidLiveNiche(_)) => {
                report
                    .faults
                    .push(format!("Decorator placement rejected: {error:?}"));
                state.rejected_this_frame += 1;
                state.queued.pop_front();
            }
            Err(error) => {
                let fault = format!("Decorator storage placement pending: {error:?}");
                state.pending_reason = Some(fault.clone());
                report.faults.push(fault);
                break;
            }
        }
    }
    if !state.applied_this_frame.is_empty() {
        archive.projection_dirty = true;
    }
}
fn derive_input(
    info: &DecoratorInfoObserved,
    context: WallDecoratorSubtypeContext<'_>,
    assets: &DecoratorAssetLibrary,
) -> Result<DecoratorOpeningInput, String> {
    let policy = info.wall_opening_policy().map_err(|e| format!("{e:?}"))?;
    let subtype = if let Some(attached_subtype) = wall_attachment_subtype(
        policy.kind,
        policy.platform_is_valid,
        policy.corner_attached,
    ) {
        attached_subtype
    } else {
        let plain_family = match policy.kind {
            WallWindowKind::Cottage => OpeningFamily::CottageWindow,
            WallWindowKind::Gothic => OpeningFamily::GothicWindow,
        };
        let plain_key =
            wall_decorator_key(plain_family, policy.rank).map_err(|e| format!("{e:?}"))?;
        let collision = assets
            .collisions
            .get(&plain_key)
            .ok_or_else(|| "Missing actual plain-window collision asset".to_string())?;
        derive_wall_subtype(
            policy.kind,
            policy.coordinate,
            false,
            false,
            context,
            collision.bounding_box,
        )
        .map_err(|e| format!("{e:?}"))?
    };
    let family = wall_opening_family(subtype).map_err(|e| format!("{e:?}"))?;
    let key = DecoratorLibKey {
        subtype,
        rank: policy.rank,
    };
    let collision = assets
        .collisions
        .get(&key)
        .ok_or_else(|| "Missing actual selected subtype/rank asset".to_string())?;
    Ok(DecoratorOpeningInput {
        family,
        size_variant: policy.rank,
        center: policy.coordinate,
        collision: collision.bounding_box.metrics(),
    })
}
fn same_opening_authority(previous: &OpeningAuthority, next: &OpeningAuthority) -> bool {
    previous.id == next.id
        && previous.wall == next.wall
        && previous.input.family == next.input.family
        && previous.input.size_variant == next.input.size_variant
        && previous.input.center.map(f32::to_bits) == next.input.center.map(f32::to_bits)
        && previous.input.collision.extent_xz.map(f32::to_bits)
            == next.input.collision.extent_xz.map(f32::to_bits)
        && previous.input.collision.vertical_bounds.map(f32::to_bits)
            == next.input.collision.vertical_bounds.map(f32::to_bits)
}

fn resolve(
    ids: Res<NativeDecoratorOpeningIds>,
    opening_entities: Res<OpeningEntities>,
    existing_openings: Query<&OpeningAuthority>,
    mut state: ResMut<NativeDecoratorOpeningState>,
    archive: Res<WallDecoratorArchiveState>,
    collection: Res<WallCollectionState>,
    prepared: Res<crate::wall_paths::WallPreparedPaths>,
    walls: Res<PublicWalls>,
    assets: Res<DecoratorAssetLibrary>,
    associations: Res<WallRoofMap>,
    roofs: Query<&Roof>,
    mut upsert: EventWriter<UpsertOpening>,
    mut remove: EventWriter<RemoveOpening>,
    mut withdraw: EventWriter<WithdrawOpening>,
    mut report: ResMut<BuildingReport>,
) {
    state.resolved_this_frame.clear();
    state.pending.clear();
    state.authoritative_inputs.clear();
    let Some(storage) = archive.storage.as_ref() else {
        let mapped: HashSet<_> = ids.0.values().copied().collect();
        for old in state.owned_openings.difference(&mapped) {
            remove.write(RemoveOpening(*old));
        }
        state.owned_openings.retain(|id| mapped.contains(id));
        for (id, opening) in &ids.0 {
            state
                .pending
                .insert(*id, "Actual DecoratorStorage is unavailable".into());
            withdraw.write(WithdrawOpening(*opening));
        }
        return;
    };
    let mut seen = HashSet::new();
    let conflicts: HashSet<_> = ids
        .0
        .values()
        .filter(|opening| !seen.insert(**opening))
        .copied()
        .collect();
    let mut live_openings = HashSet::new();
    for (id, opening_id) in &ids.0 {
        if conflicts.contains(opening_id) {
            let reason = format!(
                "Opening {} is bound to multiple native decorators",
                opening_id.0
            );
            state.pending.insert(*id, reason.clone());
            report.faults.push(reason);
            withdraw.write(WithdrawOpening(*opening_id));
            live_openings.insert(*opening_id);
            continue;
        }
        let wall = match storage.id_to_address.get(id) {
            Some(country_core::resources::walls::decorator_archivist::changes::DecoratorStorageAddress::Wall(wall)) => *wall,
            Some(_) => {
                state.pending.insert(*id, "Non-wall destination renderer/attachment owner remains unresolved".into());
                withdraw.write(WithdrawOpening(*opening_id));
                live_openings.insert(*opening_id);
                continue;
            }
            None => continue,
        };
        live_openings.insert(*opening_id);
        let result = (|| {
            let stored = storage
                .walls
                .get(&wall)
                .and_then(|d| d.items.get(id))
                .ok_or_else(|| "Global ID index has no actual matching live Info72".to_string())?;
            let info = DecoratorInfoObserved::from_observed(stored.0);
            if info.decorator_id().0 != *id {
                return Err("Info72 embedded producer ID differs from actual storage key".into());
            }
            let host = walls
                .walls
                .get(&wall)
                .ok_or_else(|| "Missing published host wall".to_string())?;
            let path = prepared
                .current(wall, host.publication_revision)
                .ok_or_else(|| "Missing current prepared path".to_string())?;
            let raw = collection
                .raw_sources
                .get(&wall)
                .ok_or_else(|| "Missing current original Curve2 source".to_string())?;
            let roof = associations
                .entity(wall)
                .and_then(|entity| roofs.get(entity).ok());
            let context = WallDecoratorSubtypeContext {
                source_curve: &raw.curve,
                mapped_outline: &path.path.positions,
                arch_heights: &host.arch_segment_heights,
                wall_shape: &host.source.shape,
                roof: roof.map(|r| &r.0),
            };
            let input = derive_input(&info, context, &assets)?;
            Ok::<_, String>(OpeningAuthority {
                id: *opening_id,
                wall,
                input,
            })
        })();
        match result {
            Ok(input) => {
                // Host publication and explicit gable invalidation already
                // reach the opening producer. Only changed authority needs an
                // upsert; equal values must not mark every opening dirty.
                let unchanged = opening_entities
                    .0
                    .get(&input.id)
                    .and_then(|entity| existing_openings.get(*entity).ok())
                    .is_some_and(|previous| same_opening_authority(previous, &input));
                if !unchanged {
                    upsert.write(UpsertOpening(input));
                }
                state.authoritative_inputs.insert(*id, input);
                state.resolved_this_frame.push(*id);
            }
            Err(reason) => {
                withdraw.write(WithdrawOpening(*opening_id));
                report
                    .faults
                    .push(format!("Native decorator {id} opening pending: {reason}"));
                state.pending.insert(*id, reason);
            }
        }
    }
    for old in state.owned_openings.difference(&live_openings) {
        remove.write(RemoveOpening(*old));
    }
    state.owned_openings = live_openings;
}
