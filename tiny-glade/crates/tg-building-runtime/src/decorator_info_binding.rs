//! UNVERIFIED B native Info72 → actual asset/context → authoritative openings.
//! Native ID→adapter OpeningId identity is explicit; no position/family sidecar.
use crate::{
    BuildingReport, PublicWalls, Roof, WallCollectionState, WallRoofMap,
    decorator_assets::DecoratorAssetLibrary,
    openings::{OpeningAuthority, OpeningId, RemoveOpening, UpsertOpening, WithdrawOpening},
    wall_decorator_archivist::WallDecoratorArchiveState,
};
use bevy_ecs::prelude::*;
use country_core::recovered_building::{
    building_decorator_info::{
        DecoratorInfoObserved, DecoratorPlaceRequestObserved, decorator_source_from_place,
        insert_decorator_source,
    },
    building_decorator_placement::{DecoratorLibKey, wall_decorator_key},
    building_decorator_subtype::{
        WallDecoratorSubtypeContext, WallWindowKind, derive_wall_subtype, wall_attachment_subtype,
        wall_opening_family,
    },
    building_opening_rules::{DecoratorOpeningInput, OpeningFamily},
};
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
        Vec<country_core::recovered_building::building_decorator_info::StoredDecoratorPlace>,
    pub pending_reason: Option<String>,
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
            consume_ids.before(crate::BuildingStage::Authoring),
        )
        .add_systems(
            bevy_app::Update,
            consume_places
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
) {
    for event in additions.read() {
        ids.0.insert(event.decorator_id, event.opening_id);
    }
    for event in removals.read() {
        ids.0.remove(&event.0);
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
            Err(error) => {
                let fault = format!("Decorator placement pending: {error:?}");
                state.pending_reason = Some(fault.clone());
                report.faults.push(fault);
                break;
            }
        };
        match insert_decorator_source(storage, &source) {
            Ok(out) => {
                state.applied_this_frame.push(out.decorator_id.0);
                state.previous_records.push(out);
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
}
fn derive_input(
    info: &DecoratorInfoObserved,
    context: WallDecoratorSubtypeContext<'_>,
    assets: &DecoratorAssetLibrary,
) -> Result<DecoratorOpeningInput, String> {
    let policy = info.wall_opening_policy().map_err(|e| format!("{e:?}"))?;
    let subtype = if let Some(s) = wall_attachment_subtype(
        policy.kind,
        policy.platform_is_valid,
        policy.corner_attached,
    ) {
        s
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
fn resolve(
    ids: Res<NativeDecoratorOpeningIds>,
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
    let mut live_openings = HashSet::new();
    for (id, opening_id) in &ids.0 {
        let wall=match storage.id_to_address.get(id){Some(country_core::recovered_building::building_wall_decorator_archivist::DecoratorStorageAddress::Wall(w))=>*w,Some(_)=>{state.pending.insert(*id,"Non-wall destination renderer/attachment owner remains unresolved".into());withdraw.write(WithdrawOpening(*opening_id));live_openings.insert(*opening_id);continue;},None=>continue};
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
                upsert.write(UpsertOpening(input));
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
