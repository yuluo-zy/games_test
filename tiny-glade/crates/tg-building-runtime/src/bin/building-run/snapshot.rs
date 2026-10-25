//! Serializes a completed update without mutating runtime authority.
use super::{
    input::encode_hex,
    trace::{lintel_trace, wall_production_trace},
};
use bevy_ecs::prelude::*;
use serde_json::{Value, json};
use std::error::Error;
use tg_building_runtime::*;

pub(super) fn snapshot(world: &mut World, detailed: bool) -> Result<Value, Box<dyn Error>> {
    let roofs = roof_snapshots(world);
    let wall_bricks = brick_snapshots(world, detailed);
    let wall_lintels = lintel_snapshots(world, detailed);
    let wall_arches = arch_snapshots(world, detailed);
    let platform_stairs = stair_snapshots(world, detailed);
    let openings = opening_snapshots(world);
    let hole_state = world.resource::<openings::WallOpenings>();
    let mut hole_rows: Vec<_> = hole_state.storage.iter().collect();
    hole_rows.sort_unstable_by_key(|(wall, _)| wall.0);
    let hole_geometry: Vec<_> = hole_rows
        .into_iter()
        .map(|(wall, storage)| {
            json!({
                "wall_id": wall.0.to_string(),
                "source_records": storage.raw_holes().len(),
                "final_records": storage.finalized_holes().map(|holes| holes.len()).unwrap_or(0),
                "bounds": storage.finalized_holes().map(|holes| holes.iter().map(|hole| json!({
                    "aabb": hole.aabb, "origin": hole.origin, "kind": hole.kind,
                })).collect::<Vec<_>>()).unwrap_or_default(),
                "wall_length": storage.wall_length().unwrap_or(0.),
            })
        })
        .collect();
    let building_history = world.resource::<history::BuildingHistory>();
    let replay = world.resource::<history::RoofHistoryReplay>();
    let history = json!({
        "configured": building_history.transactions.is_some(),
        "cursor": building_history.transactions.as_ref().map(|transactions| transactions.ledger().cursor),
        "entries": building_history.transactions.as_ref().map(|transactions| transactions.ledger().entries.len()),
        "recording": building_history.transactions.as_ref().map(|transactions| transactions.is_recording()),
        "committed": building_history.committed_this_frame,
        "replayed_mutations": building_history.replayed_this_frame,
        "replay_buckets": replay.buckets.len(),
        "completed_buckets": replay.completed_this_frame,
    });
    let gate_commands: Vec<_> = world
        .resource::<Events<wall_arches::ConstructGateCmd>>()
        .iter_current_update_events()
        .map(|event| {
            json!({
                "wall_id": event.0.wall_id.0.to_string(),
                "ordinal": event.0.ordinal,
                "projection_hex": gate_projection_hex(&event.0),
            })
        })
        .collect();
    let solid_nani = json!({
        "staged_entities": world.resource::<nani::SolidNaniCpuStaging>().snapshots.len(),
        "changes": world.resource::<nani::SolidNaniCpuChanges>().0.len(),
        "atlas_pending": world.resource::<nani::SolidNaniCpuDiagnostics>().0.len(),
        "gpu_backend_accepted": false,
    });
    let rectangle_state = world.resource::<wall_rectangle_edit::WallRectangleEditState>();
    let rectangle_edit = json!({
        "observed_ui": rectangle_state.ui.is_some(),
        "finish_pending": rectangle_state.finish_pending,
        "pending_reason": rectangle_state.pending_reason,
        "completed": rectangle_state.completed_this_frame.iter().map(|wall| wall.0.to_string()).collect::<Vec<_>>(),
    });
    let native_openings = world.resource::<decorator_info_binding::NativeDecoratorOpeningState>();
    let native_decorators = json!({
        "resolved": native_openings.resolved_this_frame,
        "authoritative_count": native_openings.authoritative_inputs.len(),
        "pending": native_openings.pending.iter().map(|(id, reason)| json!({"decorator_id": id, "reason": reason})).collect::<Vec<_>>(),
        "uses_observed_info_fields": true,
    });
    let decorator_archive = world.resource::<wall_decorator_archivist::WallDecoratorArchiveState>();
    let decorator_cancel = json!({
        "storage_observed": decorator_archive.storage.is_some(),
        "archive_observed": decorator_archive.archive.is_some(),
        "restored": decorator_archive.restored_this_frame,
        "restored_decorators": decorator_archive.restored_decorators_this_frame,
        "pending_projection": decorator_archive.pending_projection.iter().map(|(id, reason)| json!({"decorator_id": id, "reason": reason})).collect::<Vec<_>>(),
    });
    let gate_state = world.resource::<gates::GateBuildState>();
    let gate_geometry = json!({
        "entity_count": world.resource::<gates::Gates>().0.values().map(Vec::len).sum::<usize>(),
        "cpu_mesh_count": world.resource::<gates::GateCpuMeshAssets>().meshes.len(),
        "pending": gate_state.pending.iter().map(|(wall, reason)| json!({"wall_id": wall.0.to_string(), "reason": format!("{reason:?}")})).collect::<Vec<_>>(),
        "rebuilt": gate_state.rebuilt.iter().map(|wall| wall.0.to_string()).collect::<Vec<_>>(),
        "support_records": gate_state.cpu_outputs.values().flatten().map(|output| output.bricks.len()).sum::<usize>(),
    });
    let collision = world.resource::<wall_platform_collision::WallPlatformCollision>();
    let platform_collision = {
        let raster = collision
            .raster
            .lock()
            .map_err(|_| "Platform dirty owner poisoned")?;
        json!({
            "collider_count": collision.world.colliders.len(),
            "dirty_authority": raster.tiles.is_some(),
            "dirty_revision": raster.revision,
            "queued_world_rects": raster.queued_world_rects,
            "rasterized_rects": raster.rasterized.len(),
            "dirty_rows_hex": raster.tiles.as_ref().map(|tiles| tiles.rows().iter().map(|row| format!("{row:016x}")).collect::<Vec<_>>()),
            "pending": collision.pending.iter().map(|(wall, reason)| json!({"wall_id": wall.0.to_string(), "reason": format!("{reason:?}")})).collect::<Vec<_>>(),
            "ray_results": world.resource::<Events<wall_platform_collision::PlatformRayResult>>().iter_current_update_events().map(|event| json!({"request_id": event.request_id, "outcome": format!("{:?}", event.outcome)})).collect::<Vec<_>>(),
            "query_scope": "platform_subworld7",
        })
    };
    let pending_wall_bricks: Vec<_> = world.resource::<wall_bricks::WallBrickBuildState>().pending.iter()
        .map(|(wall, reason)| json!({"wall_id": wall.0.to_string(), "reason": format!("{reason:?}")})).collect();
    let masks_registry = world.resource::<wall_segments::WallPathMaskRegistry>();
    let material_stats = world.resource::<wall_segments::WallPathMaterialStats>();
    let mut mask_rows: Vec<_> = masks_registry.records.iter().collect();
    mask_rows.sort_unstable_by_key(|(wall, _)| wall.0);
    let wall_path_masks: Vec<_> = mask_rows.into_iter().map(|(wall, record)| {
        let mut entry = json!({
            "wall_id": wall.0.to_string(), "generation": record.generation,
            "prepared_generation": record.prepared_generation, "mask_count": record.masks.len(),
        });
        if detailed {
            entry["masks"] = json!(record.masks.iter().map(|mask| json!({
                "flags_hex": encode_hex(mask.flags.iter().copied()),
                "coords_hex": encode_hex(mask.coords.iter().flat_map(|coord| coord.observed_bytes())),
                "range_hex": encode_hex(mask.range.observed_bytes()),
                "length_bits": format!("{:08x}", mask.length.to_bits()),
            })).collect::<Vec<_>>());
        }
        if let Some(stats) = material_stats.0.get(wall) {
            entry["material_stats"] = json!({
                "regular_path_length": stats.regular_path_length, "special_path_length": stats.special_path_length,
                "stairs_length": stats.stairs_length, "segmentation_count": stats.segmentation_count,
            });
        }
        entry
    }).collect();
    let pending_wall_path_masks: Vec<_> = masks_registry
        .pending
        .iter()
        .map(|wall| wall.0.to_string())
        .collect();
    let owner = world.resource::<wall_authoring::WallAuthoringState>();
    let mut authored_rows: Vec<_> = owner.appearance.iter().collect();
    authored_rows.sort_unstable_by_key(|(wall, _)| wall.0);
    let authored_walls: Vec<_> = authored_rows.into_iter().filter_map(|(wall, appearance)| {
        world.resource::<WallCollectionState>().raw_sources.get(wall).map(|source| json!({
            "wall_id": wall.0.to_string(), "shape_hex": encode_hex(source.shape.bytes().iter().copied()),
            "points": source.curve.points, "us": source.curve.points_u, "length": source.curve.length,
            "height": source.extra, "elevation": source.terrain_floor, "path_scalar": source.observed_path_scalar,
            "style": source.style, "clutter_version": source.clutter_version, "special": source.special,
            "wall_color_id": appearance.wall_color_id, "roof_color_id": appearance.roof_color_id,
            "timber_color_id": appearance.timber_color_id,
        }))
    }).collect();
    let authoring = json!({
        "id_counter": owner.id_counter.map(|counter| counter.to_string()),
        "created": owner.created_this_frame.iter().map(|wall| wall.0.to_string()).collect::<Vec<_>>(),
        "pending_creation_history": owner.pending_creation_history.len(),
        "creation_roof_requests": owner.pending_creation_history.iter().map(|building| json!({
            "wall_id": building.wall.wall_id.0.to_string(), "record_hex": encode_hex(building.roof_request.record().iter().copied()),
        })).collect::<Vec<_>>(),
        "walls": authored_walls,
    });
    let processed_registry = world.resource::<wall_segments::WallProcessedMaskRegistry>();
    let mut processed_rows: Vec<_> = processed_registry.records.iter().collect();
    processed_rows.sort_unstable_by_key(|(wall, _)| wall.0);
    let processed_wall_masks: Vec<_> = processed_rows.into_iter().map(|(wall, record)| json!({
        "wall_id": wall.0.to_string(), "generation": record.generation,
        "raw_mask_generation": record.raw_mask_generation, "mask_count": record.output.processed_masks.len(),
        "masks": record.output.processed_masks.iter().map(|mask| json!({
            "flags_hex": encode_hex(mask.flags.iter().copied()),
            "coords_hex": encode_hex(mask.coords.iter().flat_map(|coord| coord.observed_bytes())),
            "range_hex": encode_hex(mask.range.observed_bytes()), "length_bits": format!("{:08x}", mask.length.to_bits()),
        })).collect::<Vec<_>>(),
        "steps": record.output.steps.iter().map(|step| json!({
            "start": step.start, "end": step.end, "range_hex": encode_hex(step.source_range.observed_bytes()),
        })).collect::<Vec<_>>(),
    })).collect();
    let mut wall_rows: Vec<_> = world.resource::<PublicWalls>().walls.iter().collect();
    wall_rows.sort_unstable_by_key(|(wall, _)| wall.0);
    let walls: Vec<_> = wall_rows.into_iter().map(|(id, wall)| json!({
        "wall_id": id.0.to_string(), "source_point_count": wall.state.source_point_count(),
        "source_curve_length": wall.state.source_curve_length(), "published_point_count": wall.source.outline.len(),
        "publication_revision": wall.publication_revision,
        "arch_min": wall.arch_segment_heights.iter().copied().fold(f32::INFINITY, f32::min),
        "arch_max": wall.arch_segment_heights.iter().copied().fold(f32::NEG_INFINITY, f32::max),
        "gable_profile_pending": wall.gable_profile_pending,
    })).collect();
    let requests = &world.resource::<nani::DeferredNaniRequests>().0;
    let created = requests
        .iter()
        .filter(|request| matches!(request, nani::DeferredNaniRequest::Create { .. }))
        .count();
    let updated = requests
        .iter()
        .filter(|request| matches!(request, nani::DeferredNaniRequest::Update { .. }))
        .count();
    let removed = requests
        .iter()
        .filter(|request| matches!(request, nani::DeferredNaniRequest::Remove { .. }))
        .count();
    let report = world.resource::<BuildingReport>();
    Ok(json!({
        "tick": world.resource::<BuildingFrame>().tick,
        "wall_bricks": wall_bricks, "wall_lintels": wall_lintels, "wall_arches": wall_arches,
        "platform_stairs": platform_stairs, "gate_commands": gate_commands, "gate_geometry": gate_geometry,
        "platform_collision": platform_collision, "solid_nani": solid_nani, "rectangle_edit": rectangle_edit,
        "decorator_cancel": decorator_cancel, "native_decorators": native_decorators,
        "pending_wall_bricks": pending_wall_bricks, "wall_path_masks": wall_path_masks,
        "processed_wall_masks": processed_wall_masks, "authoring": authoring,
        "invalid_roof_history_pending": world.resource::<roof_invalid::RoofInvalidNotificationState>().history_pending.len(),
        "pending_wall_path_masks": pending_wall_path_masks, "openings": openings, "hole_geometry": hole_geometry,
        "hole_geometry_rebuilt": hole_state.geometry_rebuilt.iter().map(|wall| wall.0.to_string()).collect::<Vec<_>>(),
        "on_wall_changed": hole_state.forwarded_recalculations.iter().map(|wall| wall.0.to_string()).collect::<Vec<_>>(),
        "history": history, "roofs": roofs, "walls": walls,
        "nani_staged_entities": world.resource::<nani::NaniCpuStagingState>().len(),
        "nani_requests": {"create": created, "update": updated, "remove": removed},
        "created": report.created.len(), "marked": report.marked.len(), "despawned": report.despawned.len(),
        "presence_changes": report.presence_changes.iter().map(|wall| wall.0.to_string()).collect::<Vec<_>>(),
        "geometry_changes": report.geometry_changes.len(),
        "wall_holes_requests": report.wall_holes_requests.iter().map(|wall| wall.0.to_string()).collect::<Vec<_>>(),
        "rim_history_requested": report.rim_history_requests.len(),
        "rim_history_pending": world.resource::<RoofRimHistoryRequests>().0.len(),
        "sound_kinds": report.sounds.iter().map(|sound| sound.kind).collect::<Vec<_>>(), "faults": report.faults,
    }))
}

fn roof_snapshots(world: &mut World) -> Vec<Value> {
    let mut query = world.query::<(
        Entity,
        &Roof,
        Option<&RoofCpuTiles>,
        Has<RoofMarkedForDeletion>,
    )>();
    let mut roofs: Vec<_> = query.iter(world).collect();
    roofs.sort_unstable_by_key(|(entity, roof, ..)| {
        (
            u64::from_le_bytes(roof.0[..8].try_into().expect("fixed roof record")),
            entity.to_bits(),
        )
    });
    roofs.into_iter().map(|(entity, roof, tiles, marked)| json!({
        "entity": entity.to_bits().to_string(),
        "wall_id": u64::from_le_bytes(roof.0[..8].try_into().expect("fixed roof record")).to_string(),
        "direction": roof.0[0x3c], "height": f32::from_le_bytes(roof.0[0x30..0x34].try_into().expect("fixed roof record")),
        "record_hex": encode_hex(roof.0), "rim_style": roof.0[0x3d], "flat_style_hex": encode_hex(roof.0[0x40..0x4c].iter().copied()),
        "marked": marked, "cpu_tile_records": tiles.map(|tiles| tiles.0.records.len()).unwrap_or(0),
    })).collect()
}

fn opening_snapshots(world: &mut World) -> Vec<Value> {
    let mut query = world.query::<(&openings::OpeningAuthority, &openings::OpeningState)>();
    let mut openings: Vec<_> = query.iter(world).collect();
    openings.sort_unstable_by_key(|(input, _)| input.id.0);
    openings
        .into_iter()
        .map(|(input, state)| {
            json!({
                "id": input.id.0.to_string(), "wall_id": input.wall.0.to_string(),
                "resolved_host": state.resolved_host.map(|wall| wall.0.to_string()),
                "generation": state.generation, "raw_holes": state.raw_holes.len(),
            })
        })
        .collect()
}

fn brick_snapshots(world: &mut World, detailed: bool) -> Vec<Value> {
    let mut query = world.query::<&wall_bricks::WallCpuBricks>();
    let mut rows: Vec<_> = query.iter(world).collect();
    rows.sort_unstable_by_key(|bricks| bricks.wall.0);
    rows.into_iter().map(|bricks| {
        let mut entry = json!({"wall_id": bricks.wall.0.to_string(), "generation": bricks.generation, "instance_count": bricks.instances.len()});
        if detailed {
            entry["records_hex"] = json!(bricks.instances.iter().map(|record| encode_hex(record.observed_bytes())).collect::<Vec<_>>());
            entry["source"] = wall_production_trace(world, bricks.wall);
        }
        entry
    }).collect()
}

fn lintel_snapshots(world: &mut World, detailed: bool) -> Vec<Value> {
    let mut query = world.query::<&wall_structures::WallCpuLintels>();
    let mut rows: Vec<_> = query.iter(world).collect();
    rows.sort_unstable_by_key(|lintels| lintels.wall.0);
    rows.into_iter().map(|lintels| {
        let mut entry = json!({"wall_id": lintels.wall.0.to_string(), "generation": lintels.generation, "instance_count": lintels.instances.len()});
        if detailed {
            entry["records_hex"] = json!(lintels.instances.iter().map(|record| encode_hex(record.observed_bytes())).collect::<Vec<_>>());
            entry["source"] = lintel_trace(world, lintels.wall);
        }
        entry
    }).collect()
}

fn arch_snapshots(world: &mut World, detailed: bool) -> Vec<Value> {
    let mut query = world.query::<&wall_arches::WallCpuArches>();
    let mut rows: Vec<_> = query.iter(world).collect();
    rows.sort_unstable_by_key(|arches| arches.wall.0);
    rows.into_iter().map(|arches| {
        let mut entry = json!({"wall_id": arches.wall.0.to_string(), "generation": arches.generation, "instance_count": arches.instances.len()});
        if detailed {
            entry["records_hex"] = json!(arches.instances.iter().map(|record| encode_hex(record.observed_bytes())).collect::<Vec<_>>());
            entry["source"] = wall_production_trace(world, arches.wall);
            if let Some(host) = world.resource::<PublicWalls>().walls.get(&arches.wall) {
                entry["public_hex"] = json!(encode_hex(host.observed_projection().iter().copied()));
                entry["structural_arch"] = json!(host.structural_arch_segment_heights);
                entry["base_third"] = json!(host.base_segment_heights);
            }
        }
        entry
    }).collect()
}

fn stair_snapshots(world: &mut World, detailed: bool) -> Vec<Value> {
    let mut query = world.query::<&wall_platform_stairs::WallCpuPlatformStairs>();
    let mut rows: Vec<_> = query.iter(world).collect();
    rows.sort_unstable_by_key(|stairs| stairs.wall.0);
    rows.into_iter().map(|stairs| {
        let mut entry = json!({
            "wall_id": stairs.wall.0.to_string(), "generation": stairs.generation,
            "cells": stairs.geometry.cells.len(), "render_record_count": stairs.render_records.len(),
            "stairs_length": stairs.geometry.stairs_length,
            "holes": stairs.holes.iter().map(|hole| json!({"center": hole.center, "dimensions": hole.dimensions, "kind": hole.kind})).collect::<Vec<_>>(),
        });
        if detailed {
            entry["render_records_hex"] = json!(stairs.render_records.iter().map(|record| encode_hex(*record)).collect::<Vec<_>>());
            entry["cell_geometry"] = json!(stairs.geometry.cells.iter().map(|cell| json!({
                "top": cell.top, "render_bottom": cell.render_bottom, "rise_count": cell.rise_count,
                "depth": cell.depth, "vertices": cell.vertices, "indices": cell.indices.as_slice(),
            })).collect::<Vec<_>>());
            entry["source"] = wall_production_trace(world, stairs.wall);
            if let Some(host) = world.resource::<PublicWalls>().walls.get(&stairs.wall) {
                entry["public_hex"] = json!(encode_hex(host.observed_projection().iter().copied()));
                entry["structural_arch"] = json!(host.structural_arch_segment_heights);
            }
            if let Some(rows) = world.resource::<wall_platform_stairs::PlatformDecoratorRows>().rows.get(&stairs.wall) {
                entry["decorator_rows_hex"] = json!(rows.iter().map(|row| encode_hex(row.0)).collect::<Vec<_>>());
            }
        }
        entry
    }).collect()
}

fn gate_projection_hex(
    command: &country_core::systems::wall::construction::arches::ConstructGateCmd120Projection,
) -> String {
    let segment = &command.segment;
    let mut bytes = Vec::new();
    bytes.extend((segment.heights.len() as u64).to_le_bytes());
    for height in &segment.heights {
        bytes.extend(height.to_le_bytes());
    }
    bytes.extend((segment.positions.len() as u64).to_le_bytes());
    for point in &segment.positions {
        for coordinate in point {
            bytes.extend(coordinate.to_le_bytes());
        }
    }
    bytes.extend((segment.points_u.len() as u64).to_le_bytes());
    for parameter in &segment.points_u {
        bytes.extend(parameter.to_le_bytes());
    }
    for index in segment.source_indices {
        bytes.extend((index as u64).to_le_bytes());
    }
    bytes.extend(segment.range.observed_bytes());
    bytes.extend(command.wall_id.0.to_le_bytes());
    bytes.extend(command.ordinal.to_le_bytes());
    encode_hex(bytes)
}
