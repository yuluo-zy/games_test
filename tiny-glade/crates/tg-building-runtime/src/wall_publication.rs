//! Validates wall authority input, publishes terrain-mapped walls, and rebuilds height caches.
//! Rejected inputs preserve the last valid authority; derived caches follow publication revisions.
use crate::{
    BuildingReport, PublicWalls, PublishWall, PublishedWall, RemoveWall, Roof, SetTerrainGrid,
    TerrainDirtyCandidates, TerrainDirtyTiles, UpsertRawWall, WallCollectionState, WallGableDirty,
    WallPublicationInput, WallRoofMap, WallRoofPresenceChanged, WallSpatialDirty, wall_authoring,
    wall_height, wall_paths,
};
use bevy_ecs::prelude::*;
use country_core::resources::walls::height_cache::regular_wall_height_cache;
use country_core::systems::wall::wall_state::publication_policy::WallPublicationCore;
use country_core::systems::wall::wall_state::publication_policy::publication_metrics;
use country_core::resources::walls::publication_fields::PublicWallBuildingState;
use country_core::resources::walls::publication_fields::WallShapeSnapshot;

/// Only regular shapes carry geometric scalar fields; freeform tails are opaque.
pub(super) fn shape_is_finite(shape: &WallShapeSnapshot) -> bool {
    let field_count = match shape.tag() {
        0 => 4,
        1 => 6,
        _ => return true,
    };
    shape.bytes()[4..4 + field_count * 4]
        .as_chunks::<4>()
        .0
        .iter()
        .all(|bytes| f32::from_le_bytes(*bytes).is_finite())
}

/// Curve lookup requires exact endpoints and sorted parameters. Interior values
/// allow rounding tolerance, including duplicate parameters from zero-length edges.
pub(super) fn raw_source_is_valid(
    source: &country_core::resources::walls::publication_source::RawWallPublicationSource,
) -> bool {
    let curve = &source.curve;
    if curve.points.len() < 2
        || curve.points.len() != curve.points_u.len()
        || curve.points_u.first() != Some(&0.0)
        || curve.points_u.last() != Some(&1.0)
        || curve.points_u.windows(2).any(|pair| pair[0] > pair[1])
        || !curve.length.is_finite()
        || curve.length <= 0.0
        || !curve.points.iter().flatten().all(|value| value.is_finite())
        || !source.terrain_floor.is_finite()
        || !source.extra.is_finite()
        || !source.observed_path_scalar.is_finite()
        || !shape_is_finite(&source.shape)
    {
        return false;
    }
    let mut length = 0.0;
    let mut previous = curve.points[0];
    for (point, u) in curve.points.iter().zip(&curve.points_u) {
        let dx = previous[0] - point[0];
        let dz = previous[1] - point[1];
        length += (dz * dz + dx * dx).sqrt();
        if !u.is_finite() || (*u - length / curve.length).abs() > 4.0 * f32::EPSILON {
            return false;
        }
        previous = *point;
    }
    length.is_finite() && (length - curve.length).abs() <= curve.length * 4.0 * f32::EPSILON
}

pub(super) fn publish(
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
        if input.outline.len() < 2
            || input.outline.len() != input.terrain_y.len()
            || input.source_point_count < 2
            || !input.source_curve_length.is_finite()
            || input.source_curve_length <= 0.0
            || !input.extra.is_finite()
            || !input
                .outline
                .iter()
                .flatten()
                .all(|value| value.is_finite())
            || !input.terrain_y.iter().all(|value| value.is_finite())
            || !input
                .outline
                .iter()
                .zip(&input.terrain_y)
                .all(|(point, terrain)| (point[1] - terrain).is_finite())
            || !shape_is_finite(&input.shape)
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
            // Missing source metadata is explicit. Consumers requiring these
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

pub(super) fn fill_base_cache(
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
            let cache = country_core::resources::walls::freeform_height_cache::freeform_wall_height_cache(
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
                wall.segment_heights = country_core::systems::wall::construction::parameters::initialize_segment_height_array(segment_count);
                wall.base_segment_heights = wall.segment_heights.clone();
            }
            Err(e) => report.faults.push(format!("Wall cache {}: {e:?}", id.0)),
        }
    }
}

pub(super) fn fill_gable_cache(
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
            country_core::resources::walls::gable_heights::apply_gable_arch_heights(
                positions,
                &wall.base_arch_segment_heights,
                base,
                source_length,
                wall.state.is_halftimbered(),
                roof,
            )
        } else {
            Ok(
                country_core::resources::walls::gable_heights::GableArchHeights {
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

pub(super) fn collect_raw_inputs(
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
    use country_core::resources::walls::publication_source::DirtyPublicationSubscription;
    use country_core::resources::walls::publication_source::collect_raw_wall_publication;
    use country_core::resources::walls::publication_source::dirty_tile_candidate;
    use country_core::resources::walls::publication_source::terrain_requires_republication;
    let mut terrain_candidates = std::collections::HashSet::new();
    for event in terrain_events.read() {
        let grid = &event.grid;
        if grid.encoded.len() != 512 * 512
            || !grid.minimum_y.is_finite()
            || !grid.maximum_y.is_finite()
            || grid.minimum_y > grid.maximum_y
            || !(grid.maximum_y - grid.minimum_y).is_finite()
        {
            report.faults.push(
                "Invalid terrain grid: expected 512 x 512 samples and finite ordered bounds".into(),
            );
            continue;
        }
        collection.terrain = Some(grid.clone());
        terrain_candidates.extend(event.affected_walls.iter().copied());
    }
    for event in source_events.read() {
        if !raw_source_is_valid(&event.source) {
            report
                .faults
                .push(format!("Invalid raw wall source {}", event.wall.0));
            continue;
        }
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
        if !event.height.is_finite() {
            report
                .faults
                .push(format!("Invalid wall source height {}", event.wall.0));
            continue;
        }
        if let Some(source) = collection.raw_sources.get_mut(&event.wall) {
            let outcome =
                country_core::systems::wall::wall_state::height_authority::assign_resolved_height(
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
        if !request.value.is_finite()
            || request
                .anchor
                .is_some_and(|anchor| !anchor.into_iter().all(|value| value.is_finite()))
        {
            report
                .faults
                .push(format!("Invalid wall height command {}", request.wall.0));
            continue;
        }
        if request.mode == 0 {
            continue;
        }
        if request.mode != 1 {
            report.faults.push(format!(
                "Unsupported wall height scalar mode {}",
                request.mode
            ));
            continue;
        }
        let Some(terrain) = collection.terrain.clone() else {
            report
                .faults
                .push("Wall height command requires terrain".into());
            continue;
        };
        let Some(source) = collection.raw_sources.get_mut(&request.wall) else {
            report.faults.push(format!(
                "Wall height command missing authority {}",
                request.wall.0
            ));
            continue;
        };
        let roof_height_01 = height_environment
            .associations
            .entity(request.wall)
            .and_then(|entity| height_environment.roofs.get(entity).ok())
            .map(|roof| f32::from_le_bytes(roof.0[0x30..0x34].try_into().unwrap()));
        let clamp = walls.walls.get(&request.wall).map(|wall| {
            country_core::systems::wall::height_command::ResolvedWallHeightClamp {
                roof_height_01,
                special_flag: wall.projection[0x296],
            }
        });
        match country_core::systems::wall::height_command::apply_resolved_height_command(
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
                continue;
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
                    DirtyPublicationSubscription::Primary => {
                        collection.dirty.insert(*id);
                    }
                    DirtyPublicationSubscription::Secondary => {
                        terrain_candidates.insert(*id);
                    }
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
