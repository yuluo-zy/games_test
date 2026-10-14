//! Replay raw and explicit resolved building inputs through the actual Bevy App.
use bevy_ecs::prelude::*;
use country_core::recovered_building::{
    building_roof_creation::RoofCreateRequestObserved,
    building_wall_state::{WallId, WallShapeSnapshot},
};
use serde_json::{Value, json};
use std::{error::Error, sync::Arc};
use tg_building_runtime::*;
fn decode<const N: usize>(v: &Value) -> Result<[u8; N], Box<dyn Error>> {
    let s = v.as_str().ok_or("Expected hex string")?;
    if s.len() != N * 2 || !s.is_ascii() {
        return Err("Wrong hex length".into());
    }
    let mut b = [0; N];
    for (i, x) in b.iter_mut().enumerate() {
        *x = u8::from_str_radix(&s[i * 2..i * 2 + 2], 16)?;
    }
    Ok(b)
}
fn id(v: &Value) -> Result<WallId, Box<dyn Error>> {
    Ok(WallId(
        v.as_str()
            .ok_or("Wall ID must be a decimal string")?
            .parse()?,
    ))
}
fn number(v: &Value) -> Result<f32, Box<dyn Error>> {
    Ok(v.as_f64().ok_or("Expected number")? as f32)
}
fn emit<T: Event>(app: &mut bevy_app::App, event: T) {
    app.world_mut().resource_mut::<Events<T>>().send(event);
}
fn authoring_shape(
    v: &Value,
) -> Result<country_core::recovered_building::building_wall_authoring::AuthoringShape, Box<dyn Error>>
{
    use country_core::recovered_building::{
        building_wall_authoring::AuthoringShape,
        building_wall_freeform_cache::{PreviewCircleObserved, PreviewRectangleObserved},
    };
    let bytes = decode::<28>(v)?;
    let f = |offset| f32::from_le_bytes(bytes[offset..offset + 4].try_into().unwrap());
    Ok(match u32::from_le_bytes(bytes[..4].try_into().unwrap()) {
        0 => AuthoringShape::Circle(PreviewCircleObserved {
            center: [f(4), f(8)],
            radius: f(12),
            angle: f(16),
        }),
        1 => AuthoringShape::Rectangle(PreviewRectangleObserved {
            axes: [f(4), f(8)],
            center: [f(12), f(16)],
            dimensions: [f(20), f(24)],
        }),
        _ => return Err("Creation requires original circle/rectangle shape".into()),
    })
}
fn opening_family(
    value: &Value,
) -> Result<country_core::recovered_building::building_opening_rules::OpeningFamily, Box<dyn Error>>
{
    use country_core::recovered_building::building_opening_rules::OpeningFamily;
    Ok(match value.as_str() {
        Some("cottage_window") => OpeningFamily::CottageWindow,
        Some("gothic_window") => OpeningFamily::GothicWindow,
        Some("cottage_door") => OpeningFamily::CottageBalconyDoor,
        Some("gothic_door") => OpeningFamily::GothicBalconyDoor,
        _ => return Err("Unknown opening family".into()),
    })
}

// Parallel implementation inputs: observed owners, not recovered UI defaults.
fn observed_decorator_address(
    v: &Value,
) -> Result<
    country_core::recovered_building::building_wall_decorator_archivist::DecoratorStorageAddress,
    Box<dyn Error>,
> {
    use country_core::recovered_building::building_wall_decorator_archivist::DecoratorStorageAddress;
    Ok(match v["kind"].as_str() {
        Some("wall") => DecoratorStorageAddress::Wall(id(&v["wall_id"])?),
        Some("special1") => DecoratorStorageAddress::ObservedSpecial1,
        Some("special2") => DecoratorStorageAddress::ObservedSpecial2,
        _ => return Err("Unknown observed decorator address".into()),
    })
}
fn decorator_id(v: &Value) -> Result<u32, Box<dyn Error>> {
    Ok(u32::try_from(
        v.as_u64().ok_or("Decorator ID must be an integer")?,
    )?)
}
fn observed_tracked_decorators(
    v: &Value,
) -> Result<
    country_core::recovered_building::building_wall_decorator_archivist::TrackedDecorators,
    Box<dyn Error>,
> {
    use country_core::recovered_building::building_wall_decorator_archivist::{
        ObservedDecoratorInfo, TrackedDecorators,
    };
    let removed = v["removed"]
        .as_array()
        .ok_or("Missing observed removed IDs")?
        .iter()
        .map(decorator_id)
        .collect::<Result<_, _>>()?;
    let added = v["added"]
        .as_array()
        .ok_or("Missing observed added IDs")?
        .iter()
        .map(decorator_id)
        .collect::<Result<_, _>>()?;
    let mut items = std::collections::HashMap::new();
    for row in v["items"]
        .as_array()
        .ok_or("Missing observed live decorators")?
    {
        let key = decorator_id(&row["decorator_id"])?;
        if items
            .insert(key, ObservedDecoratorInfo(decode(&row["info_hex"])?))
            .is_some()
        {
            return Err("Duplicate live decorator ID in observed input".into());
        }
    }
    Ok(TrackedDecorators {
        removed,
        items,
        added,
    })
}
fn observed_decorator_containers(
    v: &Value,
) -> Result<
    country_core::recovered_building::building_wall_decorator_archivist::DecoratorStorage,
    Box<dyn Error>,
> {
    use country_core::recovered_building::building_wall_decorator_archivist::DecoratorStorage;
    let mut storage = DecoratorStorage::default();
    for row in v["walls"]
        .as_array()
        .ok_or("Missing observed wall decorator containers")?
    {
        if storage
            .walls
            .insert(id(&row["wall_id"])?, observed_tracked_decorators(row)?)
            .is_some()
        {
            return Err("Duplicate wall decorator container".into());
        }
    }
    for (key, target) in [
        ("special1", &mut storage.special1),
        ("special2", &mut storage.special2),
    ] {
        let value = v
            .get(key)
            .ok_or("Special decorator container must be explicit, possibly null")?;
        if !value.is_null() {
            *target = Some(observed_tracked_decorators(value)?);
        }
    }
    Ok(storage)
}
fn observed_decorator_storage(
    v: &Value,
) -> Result<
    country_core::recovered_building::building_wall_decorator_archivist::DecoratorStorage,
    Box<dyn Error>,
> {
    let mut storage = observed_decorator_containers(v)?;
    for row in v["id_to_address"]
        .as_array()
        .ok_or("Missing observed decorator live index")?
    {
        if storage
            .id_to_address
            .insert(
                decorator_id(&row["decorator_id"])?,
                observed_decorator_address(&row["address"])?,
            )
            .is_some()
        {
            return Err("Duplicate ID in decorator live index".into());
        }
    }
    for row in v["removed_by_address"]
        .as_array()
        .ok_or("Missing observed saved removed IDs")?
    {
        let ids = row["removed"]
            .as_array()
            .ok_or("Missing observed removed IDs")?
            .iter()
            .map(decorator_id)
            .collect::<Result<_, _>>()?;
        if storage
            .removed_by_address
            .insert(observed_decorator_address(&row["address"])?, ids)
            .is_some()
        {
            return Err("Duplicate saved decorator address".into());
        }
    }
    Ok(storage)
}
fn observed_wall_snapshot(
    v: &Value,
) -> Result<
    country_core::recovered_building::building_wall_history_edit::WallAuthoritySnapshot,
    Box<dyn Error>,
> {
    use country_core::recovered_building::{
        building_wall_collection::RawWallPublicationSource,
        building_wall_history_edit::{WallAuthoritySnapshot, WallHistoryMetadata},
        surface::Curve2,
    };
    let points = v["points_xz"]
        .as_array()
        .ok_or("Missing next raw Curve2")?
        .iter()
        .map(|p| Ok([number(&p[0])?, number(&p[1])?]))
        .collect::<Result<Vec<_>, Box<dyn Error>>>()?;
    let curve = Curve2::try_new(
        points,
        v["closed"]
            .as_bool()
            .ok_or("Missing observed closed flag")?,
    )
    .map_err(|e| format!("Invalid next raw curve: {e:?}"))?;
    let byte = |key: &str| -> Result<u8, Box<dyn Error>> {
        Ok(u8::try_from(
            v[key].as_u64().ok_or("Missing observed byte")?,
        )?)
    };
    Ok(WallAuthoritySnapshot {
        source: RawWallPublicationSource {
            curve,
            curve_kind_tag: u32::try_from(
                v["curve_kind_tag"]
                    .as_u64()
                    .ok_or("Missing observed raw curve kind")?,
            )?,
            terrain_floor: number(&v["terrain_floor"])?,
            shape: WallShapeSnapshot::from_observed(decode(&v["shape_hex"])?),
            style: byte("style")?,
            extra: number(&v["extra"])?,
            observed_path_scalar: number(&v["observed_path_scalar"])?,
            clutter_version: byte("clutter_version")?,
            observed_source_flag_64: byte("observed_source_flag_64")?,
            special: v["special"]
                .as_bool()
                .ok_or("Missing observed special flag")?,
        },
        metadata: WallHistoryMetadata {
            source_60_bits: u32::from_str_radix(
                v["source_60_bits"]
                    .as_str()
                    .ok_or("Missing source60 bits")?,
                16,
            )?,
            source_67: byte("source_67")?,
            source_69: byte("source_69")?,
        },
    })
}
fn wall_production_trace(world: &World, wall: WallId) -> Value {
    use country_core::recovered_building::{
        building_wall_construction_sources::{
            ConstructionTopPolicy, construction_policy, prepared_construction_arrays,
        },
        building_wall_publication::publication_metrics,
    };
    let host = &world.resource::<PublicWalls>().walls[&wall];
    let raw = &world.resource::<WallCollectionState>().raw_sources[&wall];
    let prepared = world
        .resource::<tg_building_runtime::wall_paths::WallPreparedPaths>()
        .current(wall, host.publication_revision)
        .unwrap();
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
    .unwrap();
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
    .unwrap();
    let holes = world
        .resource::<tg_building_runtime::openings::WallOpenings>()
        .storage
        .get(&wall)
        .map(|storage| {
            storage
                .finalized_holes()
                .unwrap()
                .iter()
                .map(|hole| json!([hole.aabb, hole.kind, hole.origin]))
                .collect::<Vec<_>>()
        })
        .unwrap_or_default();
    json!({"raw_curve_points":raw.curve.points,"structural_arch":host.structural_arch_segment_heights,"base_third":host.base_segment_heights,"public_hex":host.observed_projection().iter().map(|byte|format!("{byte:02x}")).collect::<String>(),"kind":policy.kind,"top_bounds":top,"fence_fit":false,"roof_hex":roof.map(|roof|roof.0.iter().map(|byte|format!("{byte:02x}")).collect::<String>()),"published_arch":host.arch_segment_heights,"base_arch_cache":host.base_arch_segment_heights,"base_maximum_y":host.state.max_y().unwrap(),"maximum_y_gable":host.state.max_y_gable().unwrap(),"points":arrays.curve.points,"us":arrays.curve.points_u,"length":arrays.curve.length,
        "arch":arrays.arch_heights,"second":arrays.second_heights,"third":arrays.third_heights,
        "holes":holes,"seed":policy.seed.to_string(),"material":policy.brick_source_id,"closed":raw.shape.tag()!=2,
        "base_height":policy.base,"perturb":policy.row_end_perturbation,"width_padding":policy.options.width_padding,
        "thickness_scale":policy.options.thickness_scale,"wall_float":policy.wall_float,"holes_length":host.source.source_curve_length,
        "mapped_positions":host.source.outline.as_ref(),"relative_heights":relative,"observed_path_scalar":raw.observed_path_scalar,
        "prepared_positions":prepared.path.positions,"prepared_u":prepared.path.points_u,"prepared_length":prepared.path.length,
        "prepared_segment_values":prepared.path.segment_values,"clutter_version":raw.clutter_version,"shape_tag":raw.shape.tag(),"shape_hex":raw.shape.bytes().iter().map(|byte|format!("{byte:02x}")).collect::<String>(),"authored_height":raw.extra,"style":raw.style,"special":raw.special})
}
fn lintel_trace(world: &World, wall: WallId) -> Value {
    let host = &world.resource::<PublicWalls>().walls[&wall];
    let raw = &world.resource::<WallCollectionState>().raw_sources[&wall];
    let prepared = world
        .resource::<tg_building_runtime::wall_paths::WallPreparedPaths>()
        .current(wall, host.publication_revision)
        .unwrap();
    let relative: Vec<_> = host
        .source
        .outline
        .iter()
        .zip(&host.source.terrain_y)
        .map(|(point, y)| point[1] - y)
        .collect();
    let metrics = country_core::recovered_building::building_wall_publication::publication_metrics(
        &relative,
        &host.source.outline,
        raw.extra,
        raw.special,
    );
    let roof = world
        .resource::<WallRoofMap>()
        .entity(wall)
        .and_then(|entity| world.get::<Roof>(entity));
    json!({"wall_id":wall.0.to_string(),"mapped_positions":host.source.outline.as_ref(),"relative_heights":relative,"observed_path_scalar":raw.observed_path_scalar,"prepared_positions":prepared.path.positions,"prepared_u":prepared.path.points_u,"prepared_length":prepared.path.length,"prepared_segment_values":prepared.path.segment_values,"public_284":metrics.constrained_extra,"public_288":raw.observed_path_scalar,"published_arch":host.arch_segment_heights,"base_arch_cache":host.base_arch_segment_heights,"base_maximum_y":host.state.max_y().unwrap(),"maximum_y_gable":host.state.max_y_gable().unwrap(),"third":host.base_segment_heights,"final_third":host.segment_heights,"roof_hex":roof.map(|roof|roof.0.iter().map(|byte|format!("{byte:02x}")).collect::<String>()),"arch":host.arch_segment_heights,"shape_tag":raw.shape.tag(),"shape_hex":raw.shape.bytes().iter().map(|byte|format!("{byte:02x}")).collect::<String>(),"authored_height":raw.extra,"clutter_version":raw.clutter_version,"special":raw.special})
}
fn run() -> Result<(), Box<dyn Error>> {
    let path = std::env::args()
        .nth(1)
        .ok_or("Usage: building-run scenario.json")?;
    let operations: Vec<Value> = serde_json::from_slice(&std::fs::read(path)?)?;
    let mut app = building_app();
    for operation in operations {
        match operation["kind"].as_str().ok_or("Missing kind")? {
            "configure_wall_authoring"=>{
                use country_core::recovered_building::building_wall_authoring::CreationSettingsObserved;
                let mut state=app.world_mut().resource_mut::<tg_building_runtime::wall_authoring::WallAuthoringState>();
                state.dev_override_is_demo_mode=operation["dev_override_is_demo_mode"].as_bool();
                state.id_counter=Some(operation["id_counter"].as_str().ok_or("ID counter must be a decimal string")?.parse()?);
                let color=|key:&str|->Result<u32,Box<dyn Error>>{Ok(u32::try_from(operation[key].as_u64().ok_or("Missing color ID")?)?)};
                state.settings=Some(CreationSettingsObserved{wall_color_id:color("wall_color_id")?,roof_color_id:color("roof_color_id")?,timber_color_id:color("timber_color_id")?,wall_style:u8::try_from(operation["wall_style"].as_u64().ok_or("Missing wall style")?)?,rim_style:u8::try_from(operation["rim_style"].as_u64().ok_or("Missing rim style")?)?});
            }
            "set_wall_source_metadata"=>{
                let wall=id(&operation["wall_id"])?;
                let bits=u32::from_str_radix(operation["source_60_bits"].as_str().ok_or("Missing observed source60 bits")?,16)?;
                let metadata=country_core::recovered_building::building_wall_history_edit::WallHistoryMetadata{source_60_bits:bits,source_67:u8::try_from(operation["source_67"].as_u64().ok_or("Missing source67")?)?,source_69:u8::try_from(operation["source_69"].as_u64().ok_or("Missing source69")?)?};
                app.world_mut().resource_mut::<tg_building_runtime::wall_authoring::WallAuthoringState>().history_metadata.insert(wall,metadata);
                if let Some(source)=app.world().resource::<WallCollectionState>().raw_sources.get(&wall).cloned() {emit(&mut app,UpsertRawWall{wall,source});}
            }
            "bulldoze_wall"=>emit(&mut app,tg_building_runtime::wall_bulldoze::BulldozeWall{wall:id(&operation["wall_id"])?,low:number(&operation["low"])?,high:number(&operation["high"])?,anchor:[number(&operation["anchor"][0])?,number(&operation["anchor"][1])?,number(&operation["anchor"][2])?]}),
            "set_platform_decorator_rows"=>{
                let rows=operation["records_hex"].as_array().ok_or("Missing original 80B decorator rows")?.iter().map(|row|decode(row).map(country_core::recovered_building::building_wall_platform_stairs::WallPlatformDecoratorObserved)).collect::<Result<Vec<_>,_>>()?;
                emit(&mut app,tg_building_runtime::wall_platform_stairs::SetPlatformDecoratorRows{wall:id(&operation["wall_id"])?,rows});
            }
            "set_platform_collision_dirty_tiles" => {
                let rows = operation["rows_hex"].as_array().ok_or("Missing observed 64 DirtyTiles rows")?.iter()
                    .map(|row| Ok(u64::from_str_radix(row.as_str().ok_or("DirtyTiles row must be hex string")?, 16)?))
                    .collect::<Result<Vec<_>, Box<dyn Error>>>()?;
                let rows: [u64; 64] = rows.try_into().map_err(|_| "DirtyTiles requires exactly 64 rows")?;
                emit(&mut app, wall_platform_collision::SetPlatformCollisionDirtyTiles { rows });
            }
            "clear_platform_collision_dirty_tiles" => emit(&mut app, wall_platform_collision::ClearPlatformCollisionDirtyTiles),
            "query_platform_ray" => emit(&mut app, wall_platform_collision::QueryPlatformRay {
                request_id: operation["request_id"].as_u64().ok_or("Missing ray request ID")?,
                origin: [number(&operation["origin"][0])?, number(&operation["origin"][1])?, number(&operation["origin"][2])?],
                direction: [number(&operation["direction"][0])?, number(&operation["direction"][1])?, number(&operation["direction"][2])?],
                maximum_time_of_impact: number(&operation["maximum_time_of_impact"])?,
                solid: operation["solid"].as_bool().ok_or("Missing solid query policy")?,
            }),
            "create_building"=>emit(&mut app,tg_building_runtime::wall_authoring::CreateBuildingFromPlacement{shape:authoring_shape(&operation["shape_hex"])?,placement:decode(&operation["placement_hex"])?}),
            "edit_authored_rectangle"=>emit(&mut app,tg_building_runtime::wall_authoring::EditAuthoredRectangle(decode(&operation["record_hex"])?)),
            "observe_rectangle_edit_ui" => emit(&mut app, wall_rectangle_edit::ObserveRectangleEditUi(decode(&operation["state_hex"])?)),
            "finish_rectangle_edit" => emit(&mut app, wall_rectangle_edit::FinishRectangleEdit),
            "observe_native_decorator_opening_id" => emit(&mut app, decorator_info_binding::ObserveNativeDecoratorOpeningId {
                decorator_id: decorator_id(&operation["decorator_id"])?,
                opening_id: openings::OpeningId(operation["opening_id"].as_str().ok_or("Missing actual opening ID binding")?.parse()?),
            }),
            "forget_native_decorator_opening_id" => emit(&mut app, decorator_info_binding::ForgetNativeDecoratorOpeningId(decorator_id(&operation["decorator_id"])?)),
            "observe_decorator_storage_readonly" => emit(&mut app, decorator_info_binding::ObserveDecoratorStorageReadonly(
                operation["readonly"].as_bool().ok_or("Missing actual readonly state")?,
            )),
            "apply_observed_decorator_place" => emit(&mut app, decorator_info_binding::ApplyObservedDecoratorPlace {
                request: country_core::recovered_building::building_decorator_info::DecoratorPlaceRequestObserved(decode(&operation["request_hex"])?),
                observed_caller_byte: u8::try_from(operation["observed_caller_byte"].as_u64().ok_or("Missing observed placement caller byte")?)?,
            }),
            "observe_decorator_storage" => emit(&mut app, wall_decorator_archivist::ObserveDecoratorStorage(Arc::new(observed_decorator_storage(&operation["storage"])?))),
            "observe_decorator_archive" => {
                if operation["collision_handles_observed_empty"].as_bool() != Some(true) {
                    return Err("Nonempty real collision handles require the typed runtime API".into());
                }
                let containers = observed_decorator_containers(&operation["archive"])?;
                emit(&mut app, wall_decorator_archivist::ObserveDecoratorArchive(Arc::new(wall_decorator_archivist::ObservedDecoratorArchive {
                    walls: containers.walls, special1: containers.special1, special2: containers.special2,
                    collision_a: std::collections::HashMap::new(), collision_b: std::collections::HashMap::new(),
                })));
            }
            "observe_decorator_opening_binding" => emit(&mut app, wall_decorator_archivist::ObserveDecoratorOpeningBinding {
                decorator_id: decorator_id(&operation["decorator_id"])?, observed_info: decode(&operation["info_hex"])?,
                opening: decorator_placement::PlaceWallOpening {
                    id: openings::OpeningId(operation["opening_id"].as_str().ok_or("Missing actual opening ID")?.parse()?),
                    wall: id(&operation["wall_id"])?,
                    request: country_core::recovered_building::building_decorator_placement::WallDecoratorRequest {
                        family: opening_family(&operation["family"])?, rank: u8::try_from(operation["rank"].as_u64().ok_or("Missing observed rank")?)?,
                        world_position: [number(&operation["position"][0])?, number(&operation["position"][1])?, number(&operation["position"][2])?],
                    },
                },
            }),
            "apply_observed_free_wall_change" => emit(&mut app, wall_pending_changed::ApplyObservedFreeWallChange {
                wall: id(&operation["wall_id"])?, next: observed_wall_snapshot(&operation["next"])? ,
                extend_start: operation["extend_start"].as_bool().ok_or("Missing actual extension selection")?,
            }),
            "remove_authored_wall"=>emit(&mut app,tg_building_runtime::wall_authoring::RemoveAuthoredWall(id(&operation["wall_id"])?)),
            "adjust_wall_height" => emit(&mut app, tg_building_runtime::wall_height::AdjustWallHeight {
                request: country_core::recovered_building::building_wall_height_command::AdjustWallHeightRequest::from_observed(&decode(&operation["record_hex"])?),
            }),
            "load_b_wall_opening_assets" => {
                let root = operation["assets_root"].as_str().ok_or("Missing explicit assets root")?;
                app.world_mut().resource_mut::<tg_building_runtime::decorator_assets::DecoratorAssetLibrary>()
                    .load_b_wall_opening_assets(std::path::Path::new(root))
                    .map_err(|error| format!("Decorator assets: {error:?}"))?;
            }
            "set_wall_source_height" => emit(&mut app, tg_building_runtime::wall_height::SetWallSourceHeight {
                wall: id(&operation["wall_id"])?, height: number(&operation["height"])?,
            }),
            "load_b_plain_wall_assets" => {
                let root = operation["assets_root"].as_str().ok_or("Missing explicit assets root")?;
                app.world_mut().resource_mut::<tg_building_runtime::decorator_assets::DecoratorAssetLibrary>()
                    .load_b_plain_wall_assets(std::path::Path::new(root))
                    .map_err(|error| format!("Decorator assets: {error:?}"))?;
            }
            "place_wall_opening" => {
                emit(&mut app, tg_building_runtime::decorator_placement::PlaceWallOpening {
                    id: tg_building_runtime::openings::OpeningId(operation["opening_id"].as_str().ok_or("Opening ID must be a string")?.parse()?),
                    wall: id(&operation["wall_id"])?,
                    request: country_core::recovered_building::building_decorator_placement::WallDecoratorRequest {
                        family: opening_family(&operation["family"])?,
                        rank: u8::try_from(operation["rank"].as_u64().ok_or("Missing rank")?)?,
                        world_position: [number(&operation["position"][0])?, number(&operation["position"][1])?, number(&operation["position"][2])?],
                    },
                });
            }
            "edit_roof" => emit(&mut app, tg_building_runtime::roof_edits::EditRoof(
                country_core::recovered_building::roof_edit_rules::RoofEditEvent::from_observed(&decode(&operation["record_hex"])?))),
            "record_roof_visual" => emit(&mut app, tg_building_runtime::history::RecordRoofVisualChange(
                country_core::recovered_building::building_history_transactions::RoofVisualHistoryRecord::from_observed(decode(&operation["record_hex"])?))),
            "record_roof_direction" => emit(&mut app, tg_building_runtime::history::RecordRoofDirectionChange {
                wall: id(&operation["wall_id"])?.0,
                from: u8::try_from(operation["from"].as_u64().ok_or("Missing from direction")?)?,
                to: u8::try_from(operation["to"].as_u64().ok_or("Missing to direction")?)?,
            }),
            "record_roof_owner" => emit(&mut app, tg_building_runtime::history::RecordRoofOwner(
                country_core::recovered_building::building_history_transactions::RoofHistoryEdit::from_observed(decode(&operation["record_hex"])?).map_err(|error| format!("Roof history owner: {error:?}"))?)),
            "set_terrain_grid" => {
                use country_core::recovered_building::building_wall_collection::TerrainHeightGrid512;
                let encoded = if let Some(value) = operation["encoded_u16_constant"].as_u64() {
                    vec![u16::try_from(value)?; 512 * 512]
                } else {
                    let filename = operation["encoded_u16_file"]
                        .as_str()
                        .ok_or("Missing u16 grid file or constant")?;
                    let bytes = std::fs::read(filename)?;
                    if bytes.len() != 512 * 512 * 2 {
                        return Err(
                            "Grid file must contain 512x512 little-endian u16 values".into()
                        );
                    }
                    bytes
                        .chunks_exact(2)
                        .map(|b| u16::from_le_bytes(b.try_into().unwrap()))
                        .collect()
                };
                let affected_walls = operation["affected_wall_ids"]
                    .as_array()
                    .ok_or("Missing affected wall ID array")?
                    .iter()
                    .map(id)
                    .collect::<Result<_, _>>()?;
                emit(
                    &mut app,
                    SetTerrainGrid {
                        grid: TerrainHeightGrid512 {
                            encoded: Arc::from(encoded),
                            minimum_y: number(&operation["minimum_y"])?,
                            maximum_y: number(&operation["maximum_y"])?,
                            sampling_fix: operation["sampling_fix"]
                                .as_bool()
                                .ok_or("Missing sampling flag")?,
                        },
                        affected_walls,
                    },
                );
            }
            "set_path_rasters" => {
                use country_core::recovered_building::building_wall_path_preparation::PathRasters512;
                let grids = operation["grids_u8"].as_array().ok_or("Missing four path grids")?;
                if grids.len() != 4 { return Err("Expected four path grids".into()); }
                let encoded: Vec<Arc<[u8]>> = grids.iter().map(|grid| {
                    if let Some(value) = grid["constant"].as_u64() {
                        Ok(Arc::from(vec![u8::try_from(value)?; 512 * 512]))
                    } else {
                        Ok(Arc::from(std::fs::read(grid["file"].as_str().ok_or("Missing path grid file or constant")?)?))
                    }
                }).collect::<Result<_, Box<dyn Error>>>()?;
                let paths = PathRasters512::from_encoded(encoded.try_into().map_err(|_| "Expected four path grids")?,
                    operation["sampling_fix"].as_bool().ok_or("Missing path sampling flag")?)
                    .map_err(|error| format!("Path raster resource: {error:?}"))?;
                emit(&mut app, tg_building_runtime::wall_paths::SetWallPathRasters(paths));
            }
            "terrain_dirty_tiles" => {
                use country_core::recovered_building::building_wall_collection::DirtyPublicationSubscription;
                let subscription = match operation["subscription"].as_str() {
                    Some("primary") => DirtyPublicationSubscription::Primary,
                    Some("secondary") => DirtyPublicationSubscription::Secondary,
                    _ => return Err("Expected primary/secondary subscription".into()),
                };
                let mut mask = [0u64; 64];
                for cell in operation["cells"]
                    .as_array()
                    .ok_or("Missing dirty cell array")?
                {
                    let cell = cell.as_u64().ok_or("Expected cell index")?;
                    if cell >= 4096 {
                        return Err("Dirty cell index exceeds 4095".into());
                    }
                    mask[cell as usize / 64] |= 1 << (cell as usize % 64);
                }
                let already_lazy_dirty = operation["already_lazy_dirty"]
                    .as_array()
                    .ok_or("Missing already lazy dirty ID array")?
                    .iter()
                    .map(id)
                    .collect::<Result<_, _>>()?;
                emit(
                    &mut app,
                    TerrainDirtyTiles {
                        subscription,
                        mask,
                        already_lazy_dirty,
                    },
                );
            }
            "upsert_raw_wall" => {
                use country_core::recovered_building::{
                    building_wall_collection::RawWallPublicationSource, surface::Curve2,
                };
                let points = operation["points_xz"]
                    .as_array()
                    .ok_or("Missing raw Curve2 points")?
                    .iter()
                    .map(|p| Ok([number(&p[0])?, number(&p[1])?]))
                    .collect::<Result<Vec<_>, Box<dyn Error>>>()?;
                let curve = Curve2::try_new(points, false)
                    .map_err(|e| format!("Invalid raw Curve2: {e:?}"))?;
                let shape = WallShapeSnapshot::from_observed(decode(&operation["shape_hex"])?);
                emit(
                    &mut app,
                    UpsertRawWall {
                        wall: id(&operation["wall_id"])?,
                        source: RawWallPublicationSource {
                            curve,
                            curve_kind_tag: shape.tag(),
                            terrain_floor: number(&operation["terrain_floor"])?,
                            shape,
                            style: u8::try_from(
                                operation["style"].as_u64().ok_or("Missing style")?,
                            )?,
                            extra: number(&operation["extra"])?,
                            observed_path_scalar: number(&operation["observed_path_scalar"])?,
                            clutter_version: u8::try_from(operation["clutter_version"].as_u64().ok_or("Missing ClutterVersion")?)?,
                            observed_source_flag_64:u8::try_from(operation["observed_source_flag_64"].as_u64().ok_or("Missing observed source flag")?)?,
                            special: operation["special"]
                                .as_bool()
                                .ok_or("Missing special flag")?,
                        },
                    },
                );
            }
            "publish_wall" => {
                let points = operation["outline"].as_array().ok_or("Missing outline")?;
                let outline: Vec<_> = points
                    .iter()
                    .map(|p| Ok([number(&p[0])?, number(&p[1])?, number(&p[2])?]))
                    .collect::<Result<_, Box<dyn Error>>>()?;
                let terrain_y = operation["terrain_y"]
                    .as_array()
                    .ok_or("Missing terrain samples")?
                    .iter()
                    .map(number)
                    .collect::<Result<_, _>>()?;
                emit(
                    &mut app,
                    PublishWall {
                        wall: id(&operation["wall_id"])?,
                        input: WallPublicationInput {
                            outline: Arc::new(outline),
                            terrain_y,
                            source_curve_length: number(&operation["source_curve_length"])?,
                            source_point_count: operation["source_point_count"]
                                .as_u64()
                                .ok_or("Missing raw source point count")?
                                as usize,
                            shape: WallShapeSnapshot::from_observed(decode(
                                &operation["shape_hex"],
                            )?),
                            style: u8::try_from(
                                operation["style"].as_u64().ok_or("Missing style")?,
                            )?,
                            extra: number(&operation["extra"])?,
                            special: operation["special"]
                                .as_bool()
                                .ok_or("Missing special flag")?,
                        },
                    },
                );
            }
            "create_roof" => emit(
                &mut app,
                CreateRoof(RoofCreateRequestObserved::from_observed(decode(
                    &operation["record_hex"],
                )?)),
            ),
            "upsert_opening" => {
                use tg_building_runtime::openings::*;
                use country_core::recovered_building::building_opening_rules::*;
                let family=opening_family(&operation["family"])?;
                emit(&mut app,UpsertOpening(OpeningAuthority {
                    id:OpeningId(operation["opening_id"].as_str().ok_or("Opening ID must be a string")?.parse()?),
                    wall:id(&operation["wall_id"])?,
                    input:DecoratorOpeningInput{family,size_variant:u8::try_from(operation["size_variant"].as_u64().ok_or("Missing size variant")?)?,center:[number(&operation["wall_center"][0])?,number(&operation["wall_center"][1])?],collision:DecoratorCollisionMetrics{extent_xz:[number(&operation["collision_extent_xz"][0])?,number(&operation["collision_extent_xz"][1])?],vertical_bounds:[number(&operation["collision_vertical_bounds"][0])?,number(&operation["collision_vertical_bounds"][1])?]}}
                }));
            }
            "remove_opening"=>emit(&mut app,tg_building_runtime::openings::RemoveOpening(tg_building_runtime::openings::OpeningId(operation["opening_id"].as_str().ok_or("Opening ID must be a string")?.parse()?))),
            "recalculate_wall_holes"=>emit(&mut app,RecalculateWallHoles(id(&operation["wall_id"])?)),
            "history_reset_after_future_clear"=>emit(&mut app,tg_building_runtime::history::HistoryAction::ResetAfterFutureClear),
            "history_begin"=>emit(&mut app,tg_building_runtime::history::HistoryAction::Start),
            "history_commit"=>emit(&mut app,tg_building_runtime::history::HistoryAction::Finish),
            "history_undo"=>emit(&mut app,tg_building_runtime::history::HistoryAction::Undo{instant:operation["instant"].as_bool().ok_or("Missing instant flag")?}),
            "history_redo"=>emit(&mut app,tg_building_runtime::history::HistoryAction::Redo{instant:operation["instant"].as_bool().ok_or("Missing instant flag")?}),
            "set_flat_roof_style" => emit(
                &mut app,
                SetFlatRoofStyle(country_core::recovered_building::building_roof_customization::FlatRoofStyleRequest {
                    wall_id: id(&operation["wall_id"])?,
                    style: country_core::recovered_building::building_roof_customization::FlatRoofStyleObserved::from_observed(decode(&operation["style_hex"])?),
                    observed_play_sound: u8::from(operation["play_sound"].as_bool().ok_or("Missing play_sound flag")?),
                }),
            ),
            "set_flat_roof_rim_style" => emit(
                &mut app,
                SetFlatRoofRimStyle(country_core::recovered_building::building_roof_customization::FlatRoofRimRequest {
                    wall_id: id(&operation["wall_id"])?,
                    from: u8::try_from(operation["from"].as_u64().ok_or("Missing from style")?)?,
                    to: u8::try_from(operation["to"].as_u64().ok_or("Missing to style")?)?,
                    observed_record_history: u8::from(operation["record_history"].as_bool().ok_or("Missing record_history flag")?),
                }),
            ),
            "replay_roof_direction" => emit(
                &mut app,
                ReplayRoofEvent(country_core::recovered_building::roof_replay::ObservedReplayEvent::Direction {
                    wall_index: id(&operation["wall_id"])?.0,
                    direction: match operation["direction"].as_str() {
                        Some("Width") => 0, Some("Length") => 1,
                        _ => return Err("Expected Width/Length direction".into()),
                    },
                }),
            ),
            "replay_flat_roof_rim_style" => emit(
                &mut app,
                ReplayRoofEvent(country_core::recovered_building::roof_replay::ObservedReplayEvent::FlatRimStyle {
                    wall_index: id(&operation["wall_id"])?.0,
                    style: u8::try_from(operation["style"].as_u64().ok_or("Missing style")?)?,
                }),
            ),
            "switch_roof_direction" => emit(
                &mut app,
                SwitchRoofDirection(country_core::recovered_building::building_roof_direction::RoofDirectionRequest {
                    wall: id(&operation["wall_id"])?,
                    direction: match operation["direction"].as_str() {
                        Some("Width") => 0,
                        Some("Length") => 1,
                        _ => return Err("Expected Width/Length direction".into()),
                    },
                }),
            ),
            "delete_roof" => emit(&mut app, DeleteRoof(id(&operation["wall_id"])?)),
            "remove_wall" => emit(&mut app, RemoveWall(id(&operation["wall_id"])?)),
            "step" => {
                app.world_mut().resource_mut::<tg_building_runtime::wall_bulldoze::WallBulldozeTiming>().observed_game_time_58=operation["bulldoze_delta_time"].as_f64().map(|v|v as f32);
                app.world_mut().resource_mut::<BuildingFrame>().delta_time =
                    number(&operation["dt"])?;
                app.update();
                let world = app.world_mut();
                let mut arches_query=world.query::<&tg_building_runtime::wall_arches::WallCpuArches>();
                let mut stairs_query=world.query::<&tg_building_runtime::wall_platform_stairs::WallCpuPlatformStairs>();
                let mut bricks_query = world.query::<&tg_building_runtime::wall_bricks::WallCpuBricks>();
                let mut lintels_query=world.query::<&tg_building_runtime::wall_structures::WallCpuLintels>();
                let mut query = world.query::<(
                    Entity,
                    &Roof,
                    Option<&RoofCpuTiles>,
                    Has<RoofMarkedForDeletion>,
                )>();
                let roofs:Vec<_>=query.iter(world).map(|(entity,roof,tiles,marked)|json!({"entity":entity.to_bits().to_string(),"wall_id":u64::from_le_bytes(roof.0[..8].try_into().unwrap()).to_string(),"direction":roof.0[0x3c],"height":f32::from_le_bytes(roof.0[0x30..0x34].try_into().unwrap()),"record_hex":roof.0.iter().map(|b|format!("{b:02x}")).collect::<String>(),"rim_style":roof.0[0x3d],"flat_style_hex":roof.0[0x40..0x4c].iter().map(|b|format!("{b:02x}")).collect::<String>(),"marked":marked,"cpu_tile_records":tiles.map(|t|t.0.records.len()).unwrap_or(0)})).collect();
                let mut openings_query=world.query::<(&tg_building_runtime::openings::OpeningAuthority,&tg_building_runtime::openings::OpeningState)>();
                let mut openings:Vec<_>=openings_query.iter(world).map(|(input,state)|json!({"id":input.id.0.to_string(),"wall_id":input.wall.0.to_string(),"resolved_host":state.resolved_host.map(|w|w.0.to_string()),"generation":state.generation,"raw_holes":state.raw_holes.len()})).collect();
                openings.sort_by_key(|o|o["id"].as_str().unwrap().parse::<u64>().unwrap());
                let hole_state=world.resource::<tg_building_runtime::openings::WallOpenings>();
                let mut hole_geometry:Vec<_>=hole_state.storage.iter().map(|(wall,storage)|json!({"wall_id":wall.0.to_string(),"source_records":storage.raw_holes().len(),"final_records":storage.finalized_holes().map(|h|h.len()).unwrap_or(0),"bounds":storage.finalized_holes().map(|h|h.iter().map(|h|json!({"aabb":h.aabb,"origin":h.origin,"kind":h.kind})).collect::<Vec<_>>()).unwrap_or_default(),"wall_length":storage.wall_length().unwrap_or(0.)})).collect();
                hole_geometry.sort_by_key(|o|o["wall_id"].as_str().unwrap().parse::<u64>().unwrap());
                let building_history=world.resource::<tg_building_runtime::history::BuildingHistory>();
                let replay=world.resource::<tg_building_runtime::history::RoofHistoryReplay>();
                let history=json!({"configured":building_history.transactions.is_some(),"cursor":building_history.transactions.as_ref().map(|t|t.ledger().cursor),"entries":building_history.transactions.as_ref().map(|t|t.ledger().entries.len()),"recording":building_history.transactions.as_ref().map(|t|t.is_recording()),"committed":building_history.committed_this_frame,"replayed_mutations":building_history.replayed_this_frame,"replay_buckets":replay.buckets.len(),"completed_buckets":replay.completed_this_frame});
                let detailed = operation["include_wall_production_trace"].as_bool().unwrap_or(false);
                let mut wall_bricks: Vec<_> = bricks_query.iter(world).map(|bricks| {
                    let mut entry = json!({"wall_id":bricks.wall.0.to_string(),"generation":bricks.generation,"instance_count":bricks.instances.len()});
                    if detailed {
                        entry["records_hex"] = json!(bricks.instances.iter().map(|record| record.observed_bytes().iter().map(|byte| format!("{byte:02x}")).collect::<String>()).collect::<Vec<_>>());
                        entry["source"] = wall_production_trace(world, bricks.wall);
                    }
                    entry
                }).collect();
                wall_bricks.sort_by_key(|entry| entry["wall_id"].as_str().unwrap().parse::<u64>().unwrap());
                let mut wall_lintels:Vec<_>=lintels_query.iter(world).map(|lintels| {
                    let mut entry=json!({"wall_id":lintels.wall.0.to_string(),"generation":lintels.generation,"instance_count":lintels.instances.len()});
                    if detailed {entry["records_hex"]=json!(lintels.instances.iter().map(|record|record.observed_bytes().iter().map(|byte|format!("{byte:02x}")).collect::<String>()).collect::<Vec<_>>());entry["source"]=lintel_trace(world,lintels.wall);}
                    entry
                }).collect();
                wall_lintels.sort_by_key(|entry|entry["wall_id"].as_str().unwrap().parse::<u64>().unwrap());
                let mut wall_arches:Vec<_>=arches_query.iter(world).map(|arches| {
                    let mut entry=json!({"wall_id":arches.wall.0.to_string(),"generation":arches.generation,"instance_count":arches.instances.len()});
                    if detailed {
                        entry["records_hex"]=json!(arches.instances.iter().map(|r|r.observed_bytes().iter().map(|b|format!("{b:02x}")).collect::<String>()).collect::<Vec<_>>());
                        entry["source"]=wall_production_trace(world,arches.wall);
                        let host=&world.resource::<PublicWalls>().walls[&arches.wall];
                        entry["public_hex"]=json!(host.observed_projection().iter().map(|b|format!("{b:02x}")).collect::<String>());
                        entry["structural_arch"]=json!(host.structural_arch_segment_heights);
                        entry["base_third"]=json!(host.base_segment_heights);
                    }
                    entry
                }).collect();wall_arches.sort_by_key(|e|e["wall_id"].as_str().unwrap().parse::<u64>().unwrap());
                let mut platform_stairs:Vec<_>=stairs_query.iter(world).map(|stairs| {
                    let mut entry=json!({"wall_id":stairs.wall.0.to_string(),"generation":stairs.generation,"cells":stairs.geometry.cells.len(),"render_record_count":stairs.render_records.len(),"stairs_length":stairs.geometry.stairs_length,"holes":stairs.holes.iter().map(|h|json!({"center":h.center,"dimensions":h.dimensions,"kind":h.kind})).collect::<Vec<_>>()});
                    if detailed {
                        entry["render_records_hex"]=json!(stairs.render_records.iter().map(|r|r.iter().map(|b|format!("{b:02x}")).collect::<String>()).collect::<Vec<_>>());
                        entry["cell_geometry"]=json!(stairs.geometry.cells.iter().map(|c|json!({"top":c.top,"render_bottom":c.render_bottom,"rise_count":c.rise_count,"depth":c.depth,"vertices":c.vertices,"indices":c.indices.as_slice()})).collect::<Vec<_>>());
                        entry["source"]=wall_production_trace(world,stairs.wall);
                        let host=&world.resource::<PublicWalls>().walls[&stairs.wall];
                        entry["public_hex"]=json!(host.observed_projection().iter().map(|b|format!("{b:02x}")).collect::<String>());
                        entry["structural_arch"]=json!(host.structural_arch_segment_heights);
                        entry["decorator_rows_hex"]=json!(world.resource::<tg_building_runtime::wall_platform_stairs::PlatformDecoratorRows>().rows[&stairs.wall].iter().map(|r|r.0.iter().map(|b|format!("{b:02x}")).collect::<String>()).collect::<Vec<_>>());
                    }entry
                }).collect();platform_stairs.sort_by_key(|e|e["wall_id"].as_str().unwrap().parse::<u64>().unwrap());
                let gate_commands:Vec<_>=world.resource::<Events<tg_building_runtime::wall_arches::ConstructGateCmd>>().iter_current_update_events().map(|event|json!({"wall_id":event.0.wall_id.0.to_string(),"ordinal":event.0.ordinal,"projection_hex":gate_projection_hex(&event.0)})).collect();
                let solid_staging = world.resource::<nani::SolidNaniCpuStaging>();
                let solid_nani = json!({
                    "staged_entities": solid_staging.snapshots.len(),
                    "changes": world.resource::<nani::SolidNaniCpuChanges>().0.len(),
                    "atlas_pending": world.resource::<nani::SolidNaniCpuDiagnostics>().0.len(),
                    "gpu_backend_accepted": false,
                });
                let rectangle_state = world.resource::<wall_rectangle_edit::WallRectangleEditState>();
                let rectangle_edit = json!({
                    "observed_ui": rectangle_state.ui.is_some(),
                    "finish_pending": rectangle_state.finish_pending,
                    "pending_reason": rectangle_state.pending_reason,
                    "completed": rectangle_state.completed_this_frame.iter().map(|w|w.0.to_string()).collect::<Vec<_>>(),
                });
                let native_openings = world.resource::<decorator_info_binding::NativeDecoratorOpeningState>();
                let native_decorators = json!({
                    "resolved": native_openings.resolved_this_frame,
                    "authoritative_count": native_openings.authoritative_inputs.len(),
                    "pending": native_openings.pending.iter().map(|(id, reason)|json!({"decorator_id":id,"reason":reason})).collect::<Vec<_>>(),
                    "uses_observed_info_fields": true,
                });
                let decorator_archive = world.resource::<wall_decorator_archivist::WallDecoratorArchiveState>();
                let decorator_cancel = json!({
                    "storage_observed": decorator_archive.storage.is_some(),
                    "archive_observed": decorator_archive.archive.is_some(),
                    "restored": decorator_archive.restored_this_frame,
                    "restored_decorators": decorator_archive.restored_decorators_this_frame,
                    "pending_projection": decorator_archive.pending_projection.iter().map(|(id, reason)|json!({"decorator_id":id,"reason":reason})).collect::<Vec<_>>(),
                });
                let gate_state = world.resource::<gates::GateBuildState>();
                let gate_entities = world.resource::<gates::Gates>();
                let gate_geometry = json!({
                    "entity_count": gate_entities.0.values().map(Vec::len).sum::<usize>(),
                    "cpu_mesh_count": world.resource::<gates::GateCpuMeshAssets>().meshes.len(),
                    "pending": gate_state.pending.iter().map(|(wall, reason)|json!({"wall_id":wall.0.to_string(),"reason":format!("{reason:?}")})).collect::<Vec<_>>(),
                    "rebuilt": gate_state.rebuilt.iter().map(|wall|wall.0.to_string()).collect::<Vec<_>>(),
                    "support_records": gate_state.cpu_outputs.values().flatten().map(|output|output.bricks.len()).sum::<usize>(),
                });
                let collision = world.resource::<wall_platform_collision::WallPlatformCollision>();
                let raster = collision.raster.lock().map_err(|_| "Platform dirty owner poisoned")?;
                let platform_collision = json!({
                    "collider_count": collision.world.colliders.len(),
                    "dirty_authority": raster.tiles.is_some(),
                    "dirty_revision": raster.revision,
                    "queued_world_rects": raster.queued_world_rects,
                    "rasterized_rects": raster.rasterized.len(),
                    "dirty_rows_hex": raster.tiles.as_ref().map(|tiles|tiles.rows().iter().map(|row|format!("{row:016x}")).collect::<Vec<_>>()),
                    "pending": collision.pending.iter().map(|(wall, reason)|json!({"wall_id":wall.0.to_string(),"reason":format!("{reason:?}")})).collect::<Vec<_>>(),
                    "ray_results": world.resource::<Events<wall_platform_collision::PlatformRayResult>>().iter_current_update_events().map(|event|json!({"request_id":event.request_id,"outcome":format!("{:?}",event.outcome)})).collect::<Vec<_>>(),
                    "query_scope": "platform_subworld7",
                });
                drop(raster);
                let pending_wall_bricks: Vec<_> = world.resource::<tg_building_runtime::wall_bricks::WallBrickBuildState>().pending.iter().map(|(wall,reason)| json!({"wall_id":wall.0.to_string(),"reason":format!("{reason:?}")})).collect();
                let masks_registry = world.resource::<tg_building_runtime::wall_segments::WallPathMaskRegistry>();
                let material_stats = world.resource::<tg_building_runtime::wall_segments::WallPathMaterialStats>();
                let mut wall_path_masks: Vec<_> = masks_registry.records.iter().map(|(wall, record)| {
                    let mut entry = json!({"wall_id":wall.0.to_string(),"generation":record.generation,"prepared_generation":record.prepared_generation,"mask_count":record.masks.len()});
                    if detailed {
                        entry["masks"] = json!(record.masks.iter().map(|mask|json!({"flags_hex":mask.flags.iter().map(|b|format!("{b:02x}")).collect::<String>(),"coords_hex":mask.coords.iter().flat_map(|coord|coord.observed_bytes()).map(|b|format!("{b:02x}")).collect::<String>(),"range_hex":mask.range.observed_bytes().iter().map(|b|format!("{b:02x}")).collect::<String>(),"length_bits":format!("{:08x}",mask.length.to_bits())})).collect::<Vec<_>>());
                    }
                    if let Some(stats) = material_stats.0.get(wall) {
                        entry["material_stats"] = json!({"regular_path_length":stats.regular_path_length,"special_path_length":stats.special_path_length,"stairs_length":stats.stairs_length,"segmentation_count":stats.segmentation_count});
                    }
                    entry
                }).collect();
                wall_path_masks.sort_by_key(|entry|entry["wall_id"].as_str().unwrap().parse::<u64>().unwrap());
                let pending_wall_path_masks: Vec<_> = masks_registry.pending.iter().map(|wall|wall.0.to_string()).collect();
                let owner=world.resource::<tg_building_runtime::wall_authoring::WallAuthoringState>();
                let mut authored_walls:Vec<_>=owner.appearance.iter().filter_map(|(wall,appearance)|world.resource::<WallCollectionState>().raw_sources.get(wall).map(|source|json!({"wall_id":wall.0.to_string(),"shape_hex":source.shape.bytes().iter().map(|byte|format!("{byte:02x}")).collect::<String>(),"points":source.curve.points,"us":source.curve.points_u,"length":source.curve.length,"height":source.extra,"elevation":source.terrain_floor,"path_scalar":source.observed_path_scalar,"style":source.style,"clutter_version":source.clutter_version,"special":source.special,"wall_color_id":appearance.wall_color_id,"roof_color_id":appearance.roof_color_id,"timber_color_id":appearance.timber_color_id}))).collect();
                authored_walls.sort_by_key(|entry|entry["wall_id"].as_str().unwrap().parse::<u64>().unwrap());
                let authoring=json!({"id_counter":owner.id_counter.map(|counter|counter.to_string()),"created":owner.created_this_frame.iter().map(|wall|wall.0.to_string()).collect::<Vec<_>>(),"pending_creation_history":owner.pending_creation_history.len(),"creation_roof_requests":owner.pending_creation_history.iter().map(|building|json!({"wall_id":building.wall.wall_id.0.to_string(),"record_hex":building.roof_request.record().iter().map(|byte|format!("{byte:02x}")).collect::<String>()})).collect::<Vec<_>>(),"walls":authored_walls});
                let processed_registry=world.resource::<tg_building_runtime::wall_segments::WallProcessedMaskRegistry>();
                let mut processed_wall_masks:Vec<_>=processed_registry.records.iter().map(|(wall,record)|json!({"wall_id":wall.0.to_string(),"generation":record.generation,"raw_mask_generation":record.raw_mask_generation,"mask_count":record.output.processed_masks.len(),"masks":record.output.processed_masks.iter().map(|mask|json!({"flags_hex":mask.flags.iter().map(|b|format!("{b:02x}")).collect::<String>(),"coords_hex":mask.coords.iter().flat_map(|coord|coord.observed_bytes()).map(|b|format!("{b:02x}")).collect::<String>(),"range_hex":mask.range.observed_bytes().iter().map(|b|format!("{b:02x}")).collect::<String>(),"length_bits":format!("{:08x}",mask.length.to_bits())})).collect::<Vec<_>>(),"steps":record.output.steps.iter().map(|step|json!({"start":step.start,"end":step.end,"range_hex":step.source_range.observed_bytes().iter().map(|byte|format!("{byte:02x}")).collect::<String>()})).collect::<Vec<_>>()})).collect();
                processed_wall_masks.sort_by_key(|entry|entry["wall_id"].as_str().unwrap().parse::<u64>().unwrap());
                let report = world.resource::<BuildingReport>();
                let walls:Vec<_>=world.resource::<PublicWalls>().walls.iter().map(|(id,wall)|json!({"wall_id":id.0.to_string(),"source_point_count":wall.state.source_point_count(),"source_curve_length":wall.state.source_curve_length(),"published_point_count":wall.source.outline.len(),"publication_revision":wall.publication_revision,"arch_min":wall.arch_segment_heights.iter().copied().fold(f32::INFINITY,f32::min),"arch_max":wall.arch_segment_heights.iter().copied().fold(f32::NEG_INFINITY,f32::max),"gable_profile_pending":wall.gable_profile_pending})).collect();
                let requests = &world.resource::<nani::DeferredNaniRequests>().0;
                let created = requests
                    .iter()
                    .filter(|r| matches!(r, nani::DeferredNaniRequest::Create { .. }))
                    .count();
                let updated = requests
                    .iter()
                    .filter(|r| matches!(r, nani::DeferredNaniRequest::Update { .. }))
                    .count();
                let removed = requests
                    .iter()
                    .filter(|r| matches!(r, nani::DeferredNaniRequest::Remove { .. }))
                    .count();
                println!(
                    "{}",
                    json!({"tick":world.resource::<BuildingFrame>().tick,"wall_bricks":wall_bricks,"wall_lintels":wall_lintels,"wall_arches":wall_arches,"platform_stairs":platform_stairs,"gate_commands":gate_commands,"gate_geometry":gate_geometry,"platform_collision":platform_collision,"solid_nani":solid_nani,"rectangle_edit":rectangle_edit,"decorator_cancel":decorator_cancel,"native_decorators":native_decorators,"pending_wall_bricks":pending_wall_bricks,"wall_path_masks":wall_path_masks,"processed_wall_masks":processed_wall_masks,"authoring":authoring,"invalid_roof_history_pending":world.resource::<tg_building_runtime::roof_invalid::RoofInvalidNotificationState>().history_pending.len(),"pending_wall_path_masks":pending_wall_path_masks,"openings":openings,"hole_geometry":hole_geometry,"hole_geometry_rebuilt":hole_state.geometry_rebuilt.iter().map(|w|w.0.to_string()).collect::<Vec<_>>(),"on_wall_changed":hole_state.forwarded_recalculations.iter().map(|w|w.0.to_string()).collect::<Vec<_>>(),"history":history,"roofs":roofs,"walls":walls,"nani_staged_entities":world.resource::<nani::NaniCpuStagingState>().len(),"nani_requests":{"create":created,"update":updated,"remove":removed},"created":report.created.len(),"marked":report.marked.len(),"despawned":report.despawned.len(),"presence_changes":report.presence_changes.iter().map(|id|id.0.to_string()).collect::<Vec<_>>(),"geometry_changes":report.geometry_changes.len(),"wall_holes_requests":report.wall_holes_requests.iter().map(|id|id.0.to_string()).collect::<Vec<_>>(),"rim_history_requested":report.rim_history_requests.len(),"rim_history_pending":world.resource::<RoofRimHistoryRequests>().0.len(),"sound_kinds":report.sounds.iter().map(|s|s.kind).collect::<Vec<_>>(),"faults":report.faults})
                );
                if !report.faults.is_empty() {
                    return Err("Building update reported a fault".into());
                }
            }
            kind => return Err(format!("Unknown operation {kind}").into()),
        }
    }
    Ok(())
}
fn gate_projection_hex(
    r: &country_core::recovered_building::building_wall_arches::ConstructGateCmd120Projection,
) -> String {
    let segment = &r.segment;
    let mut bytes = Vec::new();
    bytes.extend((segment.heights.len() as u64).to_le_bytes());
    for f in &segment.heights {
        bytes.extend(f.to_le_bytes());
    }
    bytes.extend((segment.positions.len() as u64).to_le_bytes());
    for p in &segment.positions {
        for f in p {
            bytes.extend(f.to_le_bytes());
        }
    }
    bytes.extend((segment.points_u.len() as u64).to_le_bytes());
    for f in &segment.points_u {
        bytes.extend(f.to_le_bytes());
    }
    for i in segment.source_indices {
        bytes.extend((i as u64).to_le_bytes());
    }
    bytes.extend(segment.range.observed_bytes());
    bytes.extend(r.wall_id.0.to_le_bytes());
    bytes.extend(r.ordinal.to_le_bytes());
    bytes.iter().map(|b| format!("{b:02x}")).collect()
}
fn main() {
    if let Err(error) = run() {
        eprintln!("{error}");
        std::process::exit(1);
    }
}
