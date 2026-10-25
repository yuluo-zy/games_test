//! Optional construction diagnostics; unavailable prerequisites remain explicit.
use super::input::encode_hex;
use bevy_ecs::prelude::World;
use country_core::resources::walls::publication_fields::WallId;
use serde_json::{Value, json};
use tg_building_runtime::{PublicWalls, Roof, WallCollectionState, WallRoofMap};
pub(super) fn wall_production_trace(world: &World, wall: WallId) -> Value {
    try_wall_production_trace(world, wall)
        .unwrap_or_else(|reason| json!({"pending_reason": reason}))
}

fn try_wall_production_trace(world: &World, wall: WallId) -> Result<Value, String> {
    use country_core::systems::wall::construction::parameters::ConstructionTopPolicy;
    use country_core::systems::wall::construction::parameters::construction_policy;
    use country_core::systems::wall::construction::parameters::prepared_construction_arrays;
    use country_core::systems::wall::wall_state::publication_policy::publication_metrics;
    let host = world
        .resource::<PublicWalls>()
        .walls
        .get(&wall)
        .ok_or_else(|| format!("Missing public wall {}", wall.0))?;
    let raw = world
        .resource::<WallCollectionState>()
        .raw_sources
        .get(&wall)
        .ok_or_else(|| format!("Missing raw source for wall {}", wall.0))?;
    let prepared = world
        .resource::<tg_building_runtime::wall_paths::WallPreparedPaths>()
        .current(wall, host.publication_revision)
        .ok_or_else(|| format!("Missing current prepared path for wall {}", wall.0))?;
    let relative: Vec<_> = host
        .source
        .outline
        .iter()
        .zip(&host.source.terrain_y)
        .map(|(point, y)| point[1] - y)
        .collect();
    let metrics = publication_metrics(&relative, &host.source.outline, raw.extra, raw.special);
    let roof = world
        .resource::<WallRoofMap>()
        .entity(wall)
        .and_then(|entity| world.get::<Roof>(entity));
    let policy = construction_policy(
        wall,
        host.source.outline.len() as u64,
        host.source.source_curve_length,
        raw.clutter_version,
        raw.style,
        &raw.shape,
        metrics.minimum_position_y,
        metrics.constrained_extra,
        host.state.max_y().ok(),
        roof.map(|roof| &roof.0),
        u8::from(raw.special),
    )
    .map_err(|error| format!("Construction policy: {error:?}"))?;
    let top = match policy.top {
        ConstructionTopPolicy::Plain => [0., 0., 0., 0., 0., 2.],
        ConstructionTopPolicy::Gable {
            height,
            intervals,
            tag,
        } => [
            height,
            intervals[0][0],
            intervals[0][1],
            intervals[1][0],
            intervals[1][1],
            tag as f32,
        ],
    };
    let construction_arches = policy.constructor_arch_heights(&host.arch_segment_heights);
    let arrays = prepared_construction_arrays(
        &prepared.path.positions,
        &construction_arches,
        &host.segment_heights,
    )
    .map_err(|error| format!("Construction arrays: {error:?}"))?;
    let holes = world
        .resource::<tg_building_runtime::openings::WallOpenings>()
        .storage
        .get(&wall)
        .map(|storage| {
            Ok::<_, String>(
                storage
                    .finalized_holes()
                    .map_err(|error| format!("Unfinished wall holes: {error:?}"))?
                    .iter()
                    .map(|hole| json!([hole.aabb, hole.kind, hole.origin]))
                    .collect::<Vec<_>>(),
            )
        })
        .transpose()?
        .unwrap_or_default();
    Ok(json!({
        "raw_curve_points": raw.curve.points,
        "structural_arch": host.structural_arch_segment_heights,
        "base_third": host.base_segment_heights,
        "public_hex": encode_hex(host.observed_projection().iter().copied()),
        "kind": policy.kind,
        "top_bounds": top,
        "fence_fit": false,
        "roof_hex": roof.map(|roof|encode_hex(roof.0.iter().copied())),
        "published_arch": host.arch_segment_heights,
        "base_arch_cache": host.base_arch_segment_heights,
        "base_maximum_y": host.state.max_y().map_err(|error| format!("Base height cache: {error:?}"))?,
        "maximum_y_gable": host.state.max_y_gable().map_err(|error| format!("Gable height cache: {error:?}"))?,
        "points": arrays.curve.points,
        "us": arrays.curve.points_u,
        "length": arrays.curve.length,
        "arch": arrays.arch_heights,
        "second": arrays.second_heights,
        "third": arrays.third_heights,
        "holes": holes,
        "seed": policy.seed.to_string(),
        "material": policy.brick_source_id,
        "closed": raw.shape.tag()!=2,
        "base_height": policy.base,
        "perturb": policy.row_end_perturbation,
        "width_padding": policy.options.width_padding,
        "thickness_scale": policy.options.thickness_scale,
        "wall_float": policy.wall_float,
        "holes_length": host.source.source_curve_length,
        "mapped_positions": host.source.outline.as_ref(),
        "relative_heights": relative,
        "observed_path_scalar": raw.observed_path_scalar,
        "prepared_positions": prepared.path.positions,
        "prepared_u": prepared.path.points_u,
        "prepared_length": prepared.path.length,
        "prepared_segment_values": prepared.path.segment_values,
        "clutter_version": raw.clutter_version,
        "shape_tag": raw.shape.tag(),
        "shape_hex": encode_hex(raw.shape.bytes().iter().copied()),
        "authored_height": raw.extra,
        "style": raw.style,
        "special": raw.special,
    }))
}

pub(super) fn lintel_trace(world: &World, wall: WallId) -> Value {
    try_lintel_trace(world, wall).unwrap_or_else(|reason| json!({"pending_reason": reason}))
}

fn try_lintel_trace(world: &World, wall: WallId) -> Result<Value, String> {
    let host = world
        .resource::<PublicWalls>()
        .walls
        .get(&wall)
        .ok_or_else(|| format!("Missing public wall {}", wall.0))?;
    let raw = world
        .resource::<WallCollectionState>()
        .raw_sources
        .get(&wall)
        .ok_or_else(|| format!("Missing raw source for wall {}", wall.0))?;
    let prepared = world
        .resource::<tg_building_runtime::wall_paths::WallPreparedPaths>()
        .current(wall, host.publication_revision)
        .ok_or_else(|| format!("Missing current prepared path for wall {}", wall.0))?;
    let relative: Vec<_> = host
        .source
        .outline
        .iter()
        .zip(&host.source.terrain_y)
        .map(|(point, y)| point[1] - y)
        .collect();
    let metrics = country_core::systems::wall::wall_state::publication_policy::publication_metrics(
        &relative,
        &host.source.outline,
        raw.extra,
        raw.special,
    );
    let roof = world
        .resource::<WallRoofMap>()
        .entity(wall)
        .and_then(|entity| world.get::<Roof>(entity));
    Ok(json!({
        "wall_id": wall.0.to_string(),
        "mapped_positions": host.source.outline.as_ref(),
        "relative_heights": relative,
        "observed_path_scalar": raw.observed_path_scalar,
        "prepared_positions": prepared.path.positions,
        "prepared_u": prepared.path.points_u,
        "prepared_length": prepared.path.length,
        "prepared_segment_values": prepared.path.segment_values,
        "public_284": metrics.constrained_extra,
        "public_288": raw.observed_path_scalar,
        "published_arch": host.arch_segment_heights,
        "base_arch_cache": host.base_arch_segment_heights,
        "base_maximum_y": host.state.max_y().map_err(|error| format!("Base height cache: {error:?}"))?,
        "maximum_y_gable": host.state.max_y_gable().map_err(|error| format!("Gable height cache: {error:?}"))?,
        "third": host.base_segment_heights,
        "final_third": host.segment_heights,
        "roof_hex": roof.map(|roof|encode_hex(roof.0.iter().copied())),
        "arch": host.arch_segment_heights,
        "shape_tag": raw.shape.tag(),
        "shape_hex": encode_hex(raw.shape.bytes().iter().copied()),
        "authored_height": raw.extra,
        "clutter_version": raw.clutter_version,
        "special": raw.special,
    }))
}
