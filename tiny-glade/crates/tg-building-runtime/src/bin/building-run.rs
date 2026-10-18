//! Executes JSON scenarios and emits one diagnostic snapshot per update.
use bevy_ecs::prelude::*;
use country_core::resources::roofs::creation::RoofCreateRequestObserved;
use country_core::resources::walls::publication_fields::WallShapeSnapshot;

use serde_json::Value;
use std::{error::Error, sync::Arc};
use tg_building_runtime::*;
#[path = "building-run/input.rs"]
mod input;
#[path = "building-run/snapshot.rs"]
mod snapshot;
#[path = "building-run/trace.rs"]
mod trace;
use input::{
    authoring_shape, decode_hex, decorator_id, finite_f32, observed_decorator_containers,
    observed_decorator_storage, observed_wall_snapshot, opening_family, wall_id,
};
fn emit<T: Event>(app: &mut bevy_app::App, event: T) {
    app.world_mut().resource_mut::<Events<T>>().send(event);
}

fn run() -> Result<(), Box<dyn Error>> {
    let mut arguments = std::env::args_os().skip(1);
    let path = arguments
        .next()
        .ok_or("Usage: building-run scenario.json")?;
    if arguments.next().is_some() {
        return Err("Usage: building-run scenario.json".into());
    }
    let operations: Vec<Value> = serde_json::from_slice(&std::fs::read(path)?)?;
    let mut app = building_app();
    for (index, operation) in operations.iter().enumerate() {
        apply_operation(&mut app, operation)
            .map_err(|error| format!("Operation {}: {error}", index + 1))?;
    }
    Ok(())
}

fn apply_operation(app: &mut bevy_app::App, operation: &Value) -> Result<(), Box<dyn Error>> {
    match operation["kind"].as_str().ok_or("Missing kind")? {
        "configure_wall_authoring" => {
            use country_core::systems::wall::shapes::authoring::CreationSettingsObserved;
            let mut state = app
                .world_mut()
                .resource_mut::<wall_authoring::WallAuthoringState>();
            state.dev_override_is_demo_mode = operation["dev_override_is_demo_mode"].as_bool();
            state.id_counter = Some(
                operation["id_counter"]
                    .as_str()
                    .ok_or("ID counter must be a decimal string")?
                    .parse()?,
            );
            let color = |key: &str| -> Result<u32, Box<dyn Error>> {
                Ok(u32::try_from(
                    operation[key].as_u64().ok_or("Missing color ID")?,
                )?)
            };
            state.settings = Some(CreationSettingsObserved {
                wall_color_id: color("wall_color_id")?,
                roof_color_id: color("roof_color_id")?,
                timber_color_id: color("timber_color_id")?,
                wall_style: u8::try_from(
                    operation["wall_style"]
                        .as_u64()
                        .ok_or("Missing wall style")?,
                )?,
                rim_style: u8::try_from(
                    operation["rim_style"].as_u64().ok_or("Missing rim style")?,
                )?,
            });
        }
        "set_wall_source_metadata" => {
            let wall = wall_id(&operation["wall_id"])?;
            let bits = u32::from_str_radix(
                operation["source_60_bits"]
                    .as_str()
                    .ok_or("Missing observed source60 bits")?,
                16,
            )?;
            let metadata = country_core::resources::history::wall_edit::WallHistoryMetadata {
                source_60_bits: bits,
                source_66: operation
                    .get("source_66")
                    .map(|v| {
                        v.as_u64()
                            .ok_or("Invalid source66")
                            .and_then(|v| u8::try_from(v).map_err(|_| "Invalid source66"))
                    })
                    .transpose()?,
                source_67: u8::try_from(
                    operation["source_67"].as_u64().ok_or("Missing source67")?,
                )?,
                source_69: u8::try_from(
                    operation["source_69"].as_u64().ok_or("Missing source69")?,
                )?,
            };
            app.world_mut()
                .resource_mut::<wall_authoring::WallAuthoringState>()
                .history_metadata
                .insert(wall, metadata);
            if let Some(source) = app
                .world()
                .resource::<WallCollectionState>()
                .raw_sources
                .get(&wall)
                .cloned()
            {
                emit(app, UpsertRawWall { wall, source });
            }
        }
        "bulldoze_wall" => emit(
            app,
            wall_bulldoze::BulldozeWall {
                wall: wall_id(&operation["wall_id"])?,
                low: finite_f32(&operation["low"])?,
                high: finite_f32(&operation["high"])?,
                anchor: [
                    finite_f32(&operation["anchor"][0])?,
                    finite_f32(&operation["anchor"][1])?,
                    finite_f32(&operation["anchor"][2])?,
                ],
            },
        ),
        "set_platform_decorator_rows" => {
            let rows = operation["records_hex"]
                .as_array()
                .ok_or("Missing original 80B decorator rows")?
                .iter()
                .map(|row| {
                    decode_hex(row)
                        .map(country_core::systems::wall::construction::platform_stairs::WallPlatformDecoratorObserved)
                })
                .collect::<Result<Vec<_>, _>>()?;
            emit(
                app,
                wall_platform_stairs::SetPlatformDecoratorRows {
                    wall: wall_id(&operation["wall_id"])?,
                    rows,
                },
            );
        }
        "set_platform_collision_dirty_tiles" => {
            let rows = operation["rows_hex"]
                .as_array()
                .ok_or("Missing observed 64 DirtyTiles rows")?
                .iter()
                .map(|row| {
                    Ok(u64::from_str_radix(
                        row.as_str().ok_or("DirtyTiles row must be hex string")?,
                        16,
                    )?)
                })
                .collect::<Result<Vec<_>, Box<dyn Error>>>()?;
            let rows: [u64; 64] = rows
                .try_into()
                .map_err(|_| "DirtyTiles requires exactly 64 rows")?;
            emit(
                app,
                wall_platform_collision::SetPlatformCollisionDirtyTiles { rows },
            );
        }
        "clear_platform_collision_dirty_tiles" => emit(
            app,
            wall_platform_collision::ClearPlatformCollisionDirtyTiles,
        ),
        "query_platform_ray" => emit(
            app,
            wall_platform_collision::QueryPlatformRay {
                request_id: operation["request_id"]
                    .as_u64()
                    .ok_or("Missing ray request ID")?,
                origin: [
                    finite_f32(&operation["origin"][0])?,
                    finite_f32(&operation["origin"][1])?,
                    finite_f32(&operation["origin"][2])?,
                ],
                direction: [
                    finite_f32(&operation["direction"][0])?,
                    finite_f32(&operation["direction"][1])?,
                    finite_f32(&operation["direction"][2])?,
                ],
                maximum_time_of_impact: finite_f32(&operation["maximum_time_of_impact"])?,
                solid: operation["solid"]
                    .as_bool()
                    .ok_or("Missing solid query policy")?,
            },
        ),
        "create_building" => emit(
            app,
            wall_authoring::CreateBuildingFromPlacement {
                shape: authoring_shape(&operation["shape_hex"])?,
                placement: decode_hex(&operation["placement_hex"])?,
            },
        ),
        "edit_authored_rectangle" => emit(
            app,
            wall_authoring::EditAuthoredRectangle(decode_hex(&operation["record_hex"])?),
        ),
        "observe_rectangle_edit_ui" => emit(
            app,
            wall_rectangle_edit::ObserveRectangleEditUi(decode_hex(&operation["state_hex"])?),
        ),
        "finish_rectangle_edit" => emit(app, wall_rectangle_edit::FinishRectangleEdit),
        "observe_native_decorator_opening_id" => emit(
            app,
            decorator_info_binding::ObserveNativeDecoratorOpeningId {
                decorator_id: decorator_id(&operation["decorator_id"])?,
                opening_id: openings::OpeningId(
                    operation["opening_id"]
                        .as_str()
                        .ok_or("Missing actual opening ID binding")?
                        .parse()?,
                ),
            },
        ),
        "forget_native_decorator_opening_id" => emit(
            app,
            decorator_info_binding::ForgetNativeDecoratorOpeningId(decorator_id(
                &operation["decorator_id"],
            )?),
        ),
        "observe_decorator_storage_readonly" => emit(
            app,
            decorator_info_binding::ObserveDecoratorStorageReadonly(
                operation["readonly"]
                    .as_bool()
                    .ok_or("Missing actual readonly state")?,
            ),
        ),
        "apply_observed_decorator_place" => emit(
            app,
            decorator_info_binding::ApplyObservedDecoratorPlace {
                request: country_core::resources::walls::decorator::anchors::DecoratorPlaceRequestObserved(
                    decode_hex(&operation["request_hex"])?,
                ),
                observed_caller_byte: u8::try_from(
                    operation["observed_caller_byte"]
                        .as_u64()
                        .ok_or("Missing observed placement caller byte")?,
                )?,
            },
        ),
        "observe_decorator_storage" => emit(
            app,
            wall_decorator_archivist::ObserveDecoratorStorage(Arc::new(
                observed_decorator_storage(&operation["storage"])?,
            )),
        ),
        "observe_decorator_archive" => {
            if operation["collision_handles_observed_empty"].as_bool() != Some(true) {
                return Err("Nonempty real collision handles require the typed runtime API".into());
            }
            let containers = observed_decorator_containers(&operation["archive"])?;
            emit(app,
 wall_decorator_archivist::ObserveDecoratorArchive(Arc::new(wall_decorator_archivist::ObservedDecoratorArchive {
                    walls: containers.walls,
 special1: containers.special1,
 special2: containers.special2,
                    collision_a: std::collections::HashMap::new(),
 collision_b: std::collections::HashMap::new(),
                    culling: country_core::resources::walls::decorator_archivist::DecoratorCullPolicy {
                        allowed_ids: Default::default(),
                        allow_any: true,
                    }
,
                }
)));
        }
        "observe_decorator_opening_binding" => emit(
            app,
            wall_decorator_archivist::ObserveDecoratorOpeningBinding {
                decorator_id: decorator_id(&operation["decorator_id"])?,
                observed_info: decode_hex(&operation["info_hex"])?,
                opening: decorator_placement::PlaceWallOpening {
                    id: openings::OpeningId(
                        operation["opening_id"]
                            .as_str()
                            .ok_or("Missing actual opening ID")?
                            .parse()?,
                    ),
                    wall: wall_id(&operation["wall_id"])?,
                    request: country_core::resources::walls::decorator::placement::WallDecoratorRequest {
                        family: opening_family(&operation["family"])?,
                        rank: u8::try_from(
                            operation["rank"].as_u64().ok_or("Missing observed rank")?,
                        )?,
                        world_position: [
                            finite_f32(&operation["position"][0])?,
                            finite_f32(&operation["position"][1])?,
                            finite_f32(&operation["position"][2])?,
                        ],
                    },
                },
            },
        ),
        "apply_observed_free_wall_change" => emit(
            app,
            wall_pending_changed::ApplyObservedFreeWallChange {
                wall: wall_id(&operation["wall_id"])?,
                next: observed_wall_snapshot(&operation["next"])?,
                extend_start: operation["extend_start"]
                    .as_bool()
                    .ok_or("Missing actual extension selection")?,
            },
        ),
        "remove_authored_wall" => emit(
            app,
            wall_authoring::RemoveAuthoredWall(wall_id(&operation["wall_id"])?),
        ),
        "adjust_wall_height" => emit(
            app,
            wall_height::AdjustWallHeight {
                request:
                    country_core::systems::wall::height_command::AdjustWallHeightRequest::from_observed(
                        &decode_hex(&operation["record_hex"])?,
                    ),
            },
        ),
        "load_b_wall_opening_assets" => {
            let root = operation["assets_root"]
                .as_str()
                .ok_or("Missing explicit assets root")?;
            app.world_mut()
                .resource_mut::<decorator_assets::DecoratorAssetLibrary>()
                .load_b_wall_opening_assets(std::path::Path::new(root))
                .map_err(|error| format!("Decorator assets: {error:?}"))?;
        }
        "set_wall_source_height" => emit(
            app,
            wall_height::SetWallSourceHeight {
                wall: wall_id(&operation["wall_id"])?,
                height: finite_f32(&operation["height"])?,
            },
        ),
        "load_b_plain_wall_assets" => {
            let root = operation["assets_root"]
                .as_str()
                .ok_or("Missing explicit assets root")?;
            app.world_mut()
                .resource_mut::<decorator_assets::DecoratorAssetLibrary>()
                .load_b_plain_wall_assets(std::path::Path::new(root))
                .map_err(|error| format!("Decorator assets: {error:?}"))?;
        }
        "place_wall_opening" => {
            emit(
                app,
                decorator_placement::PlaceWallOpening {
                    id: openings::OpeningId(
                        operation["opening_id"]
                            .as_str()
                            .ok_or("Opening ID must be a string")?
                            .parse()?,
                    ),
                    wall: wall_id(&operation["wall_id"])?,
                    request: country_core::resources::walls::decorator::placement::WallDecoratorRequest {
                        family: opening_family(&operation["family"])?,
                        rank: u8::try_from(operation["rank"].as_u64().ok_or("Missing rank")?)?,
                        world_position: [
                            finite_f32(&operation["position"][0])?,
                            finite_f32(&operation["position"][1])?,
                            finite_f32(&operation["position"][2])?,
                        ],
                    },
                },
            );
        }
        "edit_roof" => emit(
            app,
            roof_edits::EditRoof(country_core::resources::roofs::geometry::edit_rules::RoofEditEvent::from_observed(
                &decode_hex(&operation["record_hex"])?,
            )),
        ),
        "record_roof_visual" => emit(
            app,
            history::RecordRoofVisualChange(
                country_core::resources::history::building_transactions::RoofVisualHistoryRecord::from_observed(
                    decode_hex(&operation["record_hex"])?,
                ),
            ),
        ),
        "record_roof_direction" => emit(
            app,
            history::RecordRoofDirectionChange {
                wall: wall_id(&operation["wall_id"])?.0,
                from: u8::try_from(operation["from"].as_u64().ok_or("Missing from direction")?)?,
                to: u8::try_from(operation["to"].as_u64().ok_or("Missing to direction")?)?,
            },
        ),
        "record_roof_owner" => emit(
            app,
            history::RecordRoofOwner(
                country_core::resources::history::building_transactions::RoofHistoryEdit::from_observed(
                    decode_hex(&operation["record_hex"])?,
                )
                .map_err(|error| format!("Roof history owner: {error:?}"))?,
            ),
        ),
        "set_terrain_grid" => {
            use country_core::resources::walls::publication_source::TerrainHeightGrid512;
            let encoded = if let Some(value) = operation["encoded_u16_constant"].as_u64() {
                vec![u16::try_from(value)?; 512 * 512]
            } else {
                let filename = operation["encoded_u16_file"]
                    .as_str()
                    .ok_or("Missing u16 grid file or constant")?;
                let bytes = std::fs::read(filename)?;
                if bytes.len() != 512 * 512 * 2 {
                    return Err("Grid file must contain 512x512 little-endian u16 values".into());
                }
                bytes
                    .as_chunks::<2>()
                    .0
                    .iter()
                    .map(|bytes| u16::from_le_bytes(*bytes))
                    .collect()
            };
            let affected_walls = operation["affected_wall_ids"]
                .as_array()
                .ok_or("Missing affected wall ID array")?
                .iter()
                .map(wall_id)
                .collect::<Result<_, _>>()?;
            emit(
                app,
                SetTerrainGrid {
                    grid: TerrainHeightGrid512 {
                        encoded: Arc::from(encoded),
                        minimum_y: finite_f32(&operation["minimum_y"])?,
                        maximum_y: finite_f32(&operation["maximum_y"])?,
                        sampling_fix: operation["sampling_fix"]
                            .as_bool()
                            .ok_or("Missing sampling flag")?,
                    },
                    affected_walls,
                },
            );
        }
        "set_path_rasters" => {
            use country_core::systems::wall::wall_state::path_preparation::PathRasters512;
            let grids = operation["grids_u8"]
                .as_array()
                .ok_or("Missing four path grids")?;
            if grids.len() != 4 {
                return Err("Expected four path grids".into());
            }
            let encoded: Vec<Arc<[u8]>> = grids
                .iter()
                .map(|grid| {
                    if let Some(value) = grid["constant"].as_u64() {
                        Ok(Arc::from(vec![u8::try_from(value)?; 512 * 512]))
                    } else {
                        Ok(Arc::from(std::fs::read(
                            grid["file"]
                                .as_str()
                                .ok_or("Missing path grid file or constant")?,
                        )?))
                    }
                })
                .collect::<Result<_, Box<dyn Error>>>()?;
            let paths = PathRasters512::from_encoded(
                encoded.try_into().map_err(|_| "Expected four path grids")?,
                operation["sampling_fix"]
                    .as_bool()
                    .ok_or("Missing path sampling flag")?,
            )
            .map_err(|error| format!("Path raster resource: {error:?}"))?;
            emit(app, wall_paths::SetWallPathRasters(paths));
        }
        "terrain_dirty_tiles" => {
            use country_core::resources::walls::publication_source::DirtyPublicationSubscription;
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
                .map(wall_id)
                .collect::<Result<_, _>>()?;
            emit(
                app,
                TerrainDirtyTiles {
                    subscription,
                    mask,
                    already_lazy_dirty,
                },
            );
        }
        "upsert_raw_wall" => {
            use country_core::resources::walls::publication_source::RawWallPublicationSource;
            use country_core::resources::roofs::geometry::surface::Curve2;
            let points = operation["points_xz"]
                .as_array()
                .ok_or("Missing raw Curve2 points")?
                .iter()
                .map(|p| Ok([finite_f32(&p[0])?, finite_f32(&p[1])?]))
                .collect::<Result<Vec<_>, Box<dyn Error>>>()?;
            let curve =
                Curve2::try_new(points, false).map_err(|e| format!("Invalid raw Curve2: {e:?}"))?;
            let shape = WallShapeSnapshot::from_observed(decode_hex(&operation["shape_hex"])?);
            emit(
                app,
                UpsertRawWall {
                    wall: wall_id(&operation["wall_id"])?,
                    source: RawWallPublicationSource {
                        curve,
                        curve_kind_tag: shape.tag(),
                        terrain_floor: finite_f32(&operation["terrain_floor"])?,
                        shape,
                        style: u8::try_from(operation["style"].as_u64().ok_or("Missing style")?)?,
                        extra: finite_f32(&operation["extra"])?,
                        observed_path_scalar: finite_f32(&operation["observed_path_scalar"])?,
                        clutter_version: u8::try_from(
                            operation["clutter_version"]
                                .as_u64()
                                .ok_or("Missing ClutterVersion")?,
                        )?,
                        observed_source_flag_64: u8::try_from(
                            operation["observed_source_flag_64"]
                                .as_u64()
                                .ok_or("Missing observed source flag")?,
                        )?,
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
                .map(|p| Ok([finite_f32(&p[0])?, finite_f32(&p[1])?, finite_f32(&p[2])?]))
                .collect::<Result<_, Box<dyn Error>>>()?;
            let terrain_y = operation["terrain_y"]
                .as_array()
                .ok_or("Missing terrain samples")?
                .iter()
                .map(finite_f32)
                .collect::<Result<_, _>>()?;
            emit(
                app,
                PublishWall {
                    wall: wall_id(&operation["wall_id"])?,
                    input: WallPublicationInput {
                        outline: Arc::new(outline),
                        terrain_y,
                        source_curve_length: finite_f32(&operation["source_curve_length"])?,
                        source_point_count: usize::try_from(
                            operation["source_point_count"]
                                .as_u64()
                                .ok_or("Missing raw source point count")?,
                        )?,
                        shape: WallShapeSnapshot::from_observed(decode_hex(
                            &operation["shape_hex"],
                        )?),
                        style: u8::try_from(operation["style"].as_u64().ok_or("Missing style")?)?,
                        extra: finite_f32(&operation["extra"])?,
                        special: operation["special"]
                            .as_bool()
                            .ok_or("Missing special flag")?,
                    },
                },
            );
        }
        "create_roof" => emit(
            app,
            CreateRoof(RoofCreateRequestObserved::from_observed(decode_hex(
                &operation["record_hex"],
            )?)),
        ),
        "upsert_opening" => {
            use country_core::resources::walls::wall_holes::opening_rules::*;
            use openings::*;
            let family = opening_family(&operation["family"])?;
            emit(
                app,
                UpsertOpening(OpeningAuthority {
                    id: OpeningId(
                        operation["opening_id"]
                            .as_str()
                            .ok_or("Opening ID must be a string")?
                            .parse()?,
                    ),
                    wall: wall_id(&operation["wall_id"])?,
                    input: DecoratorOpeningInput {
                        family,
                        size_variant: u8::try_from(
                            operation["size_variant"]
                                .as_u64()
                                .ok_or("Missing size variant")?,
                        )?,
                        center: [
                            finite_f32(&operation["wall_center"][0])?,
                            finite_f32(&operation["wall_center"][1])?,
                        ],
                        collision: DecoratorCollisionMetrics {
                            extent_xz: [
                                finite_f32(&operation["collision_extent_xz"][0])?,
                                finite_f32(&operation["collision_extent_xz"][1])?,
                            ],
                            vertical_bounds: [
                                finite_f32(&operation["collision_vertical_bounds"][0])?,
                                finite_f32(&operation["collision_vertical_bounds"][1])?,
                            ],
                        },
                    },
                }),
            );
        }
        "remove_opening" => emit(
            app,
            openings::RemoveOpening(openings::OpeningId(
                operation["opening_id"]
                    .as_str()
                    .ok_or("Opening ID must be a string")?
                    .parse()?,
            )),
        ),
        "recalculate_wall_holes" => {
            emit(app, RecalculateWallHoles(wall_id(&operation["wall_id"])?))
        }
        "history_reset_after_future_clear" => {
            emit(app, history::HistoryAction::ResetAfterFutureClear)
        }
        "history_begin" => emit(app, history::HistoryAction::Start),
        "history_commit" => emit(app, history::HistoryAction::Finish),
        "history_undo" => emit(
            app,
            history::HistoryAction::Undo {
                instant: operation["instant"]
                    .as_bool()
                    .ok_or("Missing instant flag")?,
            },
        ),
        "history_redo" => emit(
            app,
            history::HistoryAction::Redo {
                instant: operation["instant"]
                    .as_bool()
                    .ok_or("Missing instant flag")?,
            },
        ),
        "set_flat_roof_style" => emit(
            app,
            SetFlatRoofStyle(
                country_core::resources::roofs::customization::FlatRoofStyleRequest {
                    wall_id: wall_id(&operation["wall_id"])?,
                    style:
                        country_core::resources::roofs::customization::FlatRoofStyleObserved::from_observed(
                            decode_hex(&operation["style_hex"])?,
                        ),
                    observed_play_sound: u8::from(
                        operation["play_sound"]
                            .as_bool()
                            .ok_or("Missing play_sound flag")?,
                    ),
                },
            ),
        ),
        "set_flat_roof_rim_style" => emit(
            app,
            SetFlatRoofRimStyle(country_core::resources::roofs::customization::FlatRoofRimRequest {
                wall_id: wall_id(&operation["wall_id"])?,
                from: u8::try_from(operation["from"].as_u64().ok_or("Missing from style")?)?,
                to: u8::try_from(operation["to"].as_u64().ok_or("Missing to style")?)?,
                observed_record_history: u8::from(
                    operation["record_history"]
                        .as_bool()
                        .ok_or("Missing record_history flag")?,
                ),
            }),
        ),
        "replay_roof_direction" => emit(
            app,
            ReplayRoofEvent(country_core::resources::roofs::geometry::replay::ObservedReplayEvent::Direction {
                wall_index: wall_id(&operation["wall_id"])?.0,
                direction: match operation["direction"].as_str() {
                    Some("Width") => 0,
                    Some("Length") => 1,
                    _ => return Err("Expected Width/Length direction".into()),
                },
            }),
        ),
        "replay_flat_roof_rim_style" => emit(
            app,
            ReplayRoofEvent(country_core::resources::roofs::geometry::replay::ObservedReplayEvent::FlatRimStyle {
                wall_index: wall_id(&operation["wall_id"])?.0,
                style: u8::try_from(operation["style"].as_u64().ok_or("Missing style")?)?,
            }),
        ),
        "switch_roof_direction" => emit(
            app,
            SwitchRoofDirection(country_core::resources::roofs::direction::RoofDirectionRequest {
                wall: wall_id(&operation["wall_id"])?,
                direction: match operation["direction"].as_str() {
                    Some("Width") => 0,
                    Some("Length") => 1,
                    _ => return Err("Expected Width/Length direction".into()),
                },
            }),
        ),
        "delete_roof" => emit(app, DeleteRoof(wall_id(&operation["wall_id"])?)),
        "remove_wall" => emit(app, RemoveWall(wall_id(&operation["wall_id"])?)),
        "step" => {
            app.world_mut()
                .resource_mut::<wall_bulldoze::WallBulldozeTiming>()
                .observed_game_time_58 = operation
                .get("bulldoze_delta_time")
                .map(finite_f32)
                .transpose()?;
            app.world_mut().resource_mut::<BuildingFrame>().delta_time =
                finite_f32(&operation["dt"])?;
            app.update();
            let detailed = operation["include_wall_production_trace"]
                .as_bool()
                .unwrap_or(false);
            println!("{}", snapshot::snapshot(app.world_mut(), detailed)?);
            let report = app.world().resource::<BuildingReport>();
            if !report.faults.is_empty() {
                return Err("Building update reported a fault".into());
            }
        }
        kind => return Err(format!("Unknown operation {kind}").into()),
    }
    Ok(())
}

fn main() {
    if let Err(error) = run() {
        eprintln!("{error}");
        std::process::exit(1);
    }
}

#[cfg(test)]
mod tests {
    use super::input::{decode_hex, finite_f32};
    use super::trace::{lintel_trace, wall_production_trace};
    use serde_json::json;
    #[test]
    fn detailed_trace_reports_missing_production_inputs() {
        let app = tg_building_runtime::building_app();
        let trace = wall_production_trace(
            app.world(),
            country_core::resources::walls::publication_fields::WallId(7),
        );
        assert_eq!(trace["pending_reason"], "Missing public wall 7");
        let lintel = lintel_trace(
            app.world(),
            country_core::resources::walls::publication_fields::WallId(7),
        );
        assert_eq!(lintel["pending_reason"], "Missing public wall 7");
    }
    #[test]
    fn lintel_trace_reports_an_unfinished_gable_cache_with_a_current_path() {
        use country_core::resources::walls::publication_source::RawWallPublicationSource;
        use country_core::resources::walls::publication_source::TerrainHeightGrid512;
        use country_core::systems::wall::wall_state::path_preparation::PathRasters512;
        use country_core::resources::walls::publication_fields::PublicWallBuildingState;
        use country_core::resources::walls::publication_fields::WallId;
        use country_core::resources::walls::publication_fields::WallShapeSnapshot;
        use country_core::resources::roofs::geometry::surface::Curve2;
        use std::sync::Arc;
        use tg_building_runtime::{
            PublicWalls, SetTerrainGrid, UpsertRawWall, wall_paths::SetWallPathRasters,
        };
        let mut app = tg_building_runtime::building_app();
        let wall = WallId(7);
        super::emit(
            &mut app,
            SetTerrainGrid {
                grid: TerrainHeightGrid512 {
                    encoded: Arc::from(vec![0; 512 * 512]),
                    minimum_y: 0.,
                    maximum_y: 0.,
                    sampling_fix: true,
                },
                affected_walls: vec![],
            },
        );
        super::emit(
            &mut app,
            SetWallPathRasters(
                PathRasters512::from_encoded(
                    std::array::from_fn(|_| Arc::from(vec![0; 512 * 512])),
                    true,
                )
                .unwrap(),
            ),
        );
        let mut shape = [0; 28];
        shape[..4].copy_from_slice(&2u32.to_le_bytes());
        super::emit(
            &mut app,
            UpsertRawWall {
                wall,
                source: RawWallPublicationSource {
                    curve: Curve2::try_new(vec![[0., 0.], [2., 0.]], false).unwrap(),
                    curve_kind_tag: 2,
                    terrain_floor: 0.,
                    shape: WallShapeSnapshot::from_observed(shape),
                    style: 0,
                    extra: 2.,
                    special: false,
                    observed_path_scalar: 0.,
                    clutter_version: 0,
                    observed_source_flag_64: 0,
                },
            },
        );
        app.update();
        {
            let mut walls = app.world_mut().resource_mut::<PublicWalls>();
            let host = walls.walls.get_mut(&wall).unwrap();
            let mut projection = *host.observed_projection();
            projection[0x110..0x114].fill(0);
            host.state = PublicWallBuildingState::from_observed(projection);
        }
        let trace = lintel_trace(app.world(), wall);
        assert!(
            trace["pending_reason"]
                .as_str()
                .unwrap()
                .starts_with("Gable height cache:")
        );
    }
    #[test]
    fn detailed_snapshots_sort_typed_ids_and_tolerate_missing_authority() {
        use country_core::resources::walls::publication_fields::WallId;
        use tg_building_runtime::wall_arches::WallCpuArches;

        let mut app = tg_building_runtime::building_app();
        for wall in [WallId(40), WallId(9)] {
            app.world_mut().spawn(WallCpuArches {
                wall,
                generation: 1,
                instances: Vec::new(),
            });
        }
        let snapshot = super::snapshot::snapshot(app.world_mut(), true).unwrap();
        let arches = snapshot["wall_arches"].as_array().unwrap();
        assert_eq!(arches[0]["wall_id"], "9");
        assert_eq!(arches[1]["wall_id"], "40");
        for (entry, id) in arches.iter().zip([9, 40]) {
            assert_eq!(
                entry["source"]["pending_reason"],
                format!("Missing public wall {id}")
            );
            assert!(entry.get("public_hex").is_none());
        }
    }

    #[test]
    fn external_numbers_and_hex_are_checked_before_use() {
        assert_eq!(finite_f32(&json!(1.5)).unwrap(), 1.5);
        assert!(finite_f32(&json!(1e100)).is_err());
        assert!(decode_hex::<2>(&json!("éé")).is_err());
        assert_eq!(decode_hex::<2>(&json!("00ff")).unwrap(), [0, 255]);
        assert_eq!(super::input::encode_hex([0, 255]), "00ff");
    }
}
