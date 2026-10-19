//! B saved-history input, converted into the live replay owners.
//! Schema anchors: EditType visitor 140b3b630 (Wall/WallNewSystem aliases),
//! MultiEdit fields 1415d1150, WallHistoryEdit visitors 1415e6c50/1415ef890/
//! 1415dad80/1415d7430 and OperationType 1415d3270. Unrecovered variants fail
//! decoding; they never become an empty edit or a fabricated wall.
use super::{
    EditType,
    terrain_edit::TerrainEdit,
    transactions::HistoryEntry,
    wall_edit::{WallCompoundHistoryEdit, WallHistoryEdit},
};
use crate::resources::walls::{inner_wall_state::WallState, publication_fields::WallId};
use serde::{Deserialize, Deserializer, Serialize, Serializer, de::Error};
use std::sync::Arc;

#[derive(Deserialize, Serialize)]
enum SavedOperation {
    ExpandFrom { start_t: f32, end_t: f32 },
    ShrinkTo { start_t: f32, end_t: f32 },
    SwapInstant,
}

/// Original tag12's bool payload has distinct field names, despite sharing the
/// live scalar-edit owner. Serializer141400759 / strings142c5b69b,142c5b6b2.
#[derive(Deserialize, Serialize)]
struct SavedWallSupports {
    wall_index: u64,
    before_removed_supports: bool,
    after_removed_supports: bool,
}
impl SavedOperation {
    fn bytes(self) -> [u8; 12] {
        let (tag, start, end): (u32, f32, f32) = match self {
            Self::ExpandFrom { start_t, end_t } => (0, start_t, end_t),
            Self::ShrinkTo { start_t, end_t } => (1, start_t, end_t),
            // Unit variant: the other two words are not consumed for this tag.
            Self::SwapInstant => (2, 0.0, 0.0),
        };
        let mut out = [0; 12];
        out[..4].copy_from_slice(&tag.to_le_bytes());
        out[4..8].copy_from_slice(&start.to_le_bytes());
        out[8..].copy_from_slice(&end.to_le_bytes());
        out
    }
}

#[derive(Deserialize, Serialize)]
enum SavedWallEdit {
    WallChanged {
        wall_index: u64,
        operation_ty: SavedOperation,
        before: WallState,
        after: WallState,
    },
    CreateWall {
        wall_index: u64,
        wall_height: f32,
        elevation: f32,
        // Both spellings map to field3 in the original field visitor
        // 141402680. They are aliases, not separate owners.
        #[serde(alias = "redo_instant_state")]
        after: WallState,
        color_id: u32,
    },
    DeleteWall {
        wall_index: u64,
        wall_height: f32,
        #[serde(alias = "undo_instant_state")]
        before: WallState,
        color_id: u32,
    },
    AdjustHeight {
        wall_index: u64,
        before: f32,
        after: f32,
    },
    MorphPremadeShape(super::wall_edit::MorphPremadeShapeEdit),
    ChangeIsEnclosedState(
        super::wall_edit::WallValueEdit<crate::systems::wall::valid_enclosure::IsEnclosed>,
    ),
    ChangeElevation(super::wall_edit::WallValueEdit<f32>),
    ChangeArchHeight(super::wall_edit::WallValueEdit<f32>),
    ChangeFreehandHeightType(super::wall_edit::WallValueEdit<super::wall_edit::WallTerrainPolicy>),
    ChangeArchType(super::wall_edit::WallValueEdit<super::wall_edit::WallTerrainPolicy>),
    ChangeSupports(SavedWallSupports),
    ChangeStyle(super::wall_edit::WallValueEdit<super::wall_edit::SavedWallStyle>),
    ChangeSkirt(super::wall_edit::WallValueEdit<bool>),
    ChangePlasterPeelAmount(super::wall_edit::WallValueEdit<f32>),
    MorphFreehandWall(super::wall_edit::WallValueEdit<super::wall_edit::WallCurveHistory>),
}

impl Serialize for WallHistoryEdit {
    fn serialize<S: Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        let wall =
            |snapshot: &super::wall_edit::WallAuthoritySnapshot| -> Result<WallState, S::Error> {
                snapshot
                    .metadata
                    .source_66
                    .map(|ty| WallState::from_snapshot(snapshot.clone(), ty))
                    .ok_or_else(|| serde::ser::Error::custom("wall source+66 missing"))
            };
        let saved = match self {
            Self::StateChange {
                wall: id,
                previous,
                next,
                observed_operation,
            } => {
                let start_t = f32::from_le_bytes(observed_operation[4..8].try_into().unwrap());
                let end_t = f32::from_le_bytes(observed_operation[8..12].try_into().unwrap());
                let operation_ty =
                    match u32::from_le_bytes(observed_operation[..4].try_into().unwrap()) {
                        0 => SavedOperation::ExpandFrom { start_t, end_t },
                        1 => SavedOperation::ShrinkTo { start_t, end_t },
                        2 => SavedOperation::SwapInstant,
                        _ => {
                            return Err(serde::ser::Error::custom(
                                "unrestored WallChanged operation",
                            ));
                        }
                    };
                SavedWallEdit::WallChanged {
                    wall_index: id.0,
                    operation_ty,
                    before: wall(previous)?,
                    after: wall(next)?,
                }
            }
            Self::Create {
                wall: id,
                source,
                color_id,
                restored_height,
                restored_elevation,
            } => SavedWallEdit::CreateWall {
                wall_index: id.0,
                wall_height: *restored_height,
                elevation: *restored_elevation,
                after: wall(source)?,
                color_id: *color_id,
            },
            Self::Delete {
                wall: id,
                source,
                color_id,
                restored_height,
            } => SavedWallEdit::DeleteWall {
                wall_index: id.0,
                wall_height: *restored_height,
                before: wall(source)?,
                color_id: *color_id,
            },
            Self::MorphPremadeShape(edit) => SavedWallEdit::MorphPremadeShape(*edit),
            Self::EnclosureChange {
                wall,
                previous,
                next,
            } => SavedWallEdit::ChangeIsEnclosedState(super::wall_edit::WallValueEdit {
                wall_index: wall.0,
                before: *previous,
                after: *next,
            }),
            Self::ChangeElevation(edit) => SavedWallEdit::ChangeElevation(edit.clone()),
            Self::ChangeArchHeight(edit) => SavedWallEdit::ChangeArchHeight(edit.clone()),
            Self::ChangeFreehandHeightType(edit) => {
                SavedWallEdit::ChangeFreehandHeightType(edit.clone())
            }
            Self::ChangeArchType(edit) => SavedWallEdit::ChangeArchType(edit.clone()),
            Self::ChangeSupports(edit) => SavedWallEdit::ChangeSupports(SavedWallSupports {
                wall_index: edit.wall_index,
                before_removed_supports: edit.before,
                after_removed_supports: edit.after,
            }),
            Self::ChangeStyle(edit) => SavedWallEdit::ChangeStyle(edit.clone()),
            Self::ChangeSkirt(edit) => SavedWallEdit::ChangeSkirt(edit.clone()),
            Self::ChangePlasterPeelAmount(edit) => {
                SavedWallEdit::ChangePlasterPeelAmount(edit.clone())
            }
            Self::MorphFreehandWall(edit) => SavedWallEdit::MorphFreehandWall(edit.clone()),
            Self::AdjustHeight { wall, from, to } => SavedWallEdit::AdjustHeight {
                wall_index: wall.0,
                before: *from,
                after: *to,
            },
            _ => {
                return Err(serde::ser::Error::custom(
                    "B wall edit save encoder is not restored for this operation",
                ));
            }
        };
        saved.serialize(s)
    }
}

impl<'de> Deserialize<'de> for WallHistoryEdit {
    fn deserialize<D: Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        Ok(match SavedWallEdit::deserialize(d)? {
            SavedWallEdit::WallChanged {
                wall_index,
                operation_ty,
                before,
                after,
            } => Self::StateChange {
                wall: WallId(wall_index),
                previous: before.snapshot().clone(),
                next: after.snapshot().clone(),
                observed_operation: operation_ty.bytes(),
            },
            SavedWallEdit::CreateWall {
                wall_index,
                wall_height,
                elevation,
                after,
                color_id,
            } => Self::Create {
                wall: WallId(wall_index),
                source: after.snapshot().clone(),
                color_id,
                restored_height: wall_height,
                restored_elevation: elevation,
            },
            SavedWallEdit::DeleteWall {
                wall_index,
                wall_height,
                before,
                color_id,
            } => Self::Delete {
                wall: WallId(wall_index),
                source: before.snapshot().clone(),
                color_id,
                restored_height: wall_height,
            },
            SavedWallEdit::MorphPremadeShape(edit) => Self::MorphPremadeShape(edit),
            SavedWallEdit::ChangeIsEnclosedState(edit) => Self::EnclosureChange {
                wall: WallId(edit.wall_index),
                previous: edit.before,
                next: edit.after,
            },
            SavedWallEdit::ChangeElevation(edit) => Self::ChangeElevation(edit),
            SavedWallEdit::ChangeArchHeight(edit) => Self::ChangeArchHeight(edit),
            SavedWallEdit::ChangeFreehandHeightType(edit) => Self::ChangeFreehandHeightType(edit),
            SavedWallEdit::ChangeArchType(edit) => Self::ChangeArchType(edit),
            SavedWallEdit::ChangeSupports(edit) => {
                Self::ChangeSupports(super::wall_edit::WallValueEdit {
                    wall_index: edit.wall_index,
                    before: edit.before_removed_supports,
                    after: edit.after_removed_supports,
                })
            }
            SavedWallEdit::ChangeStyle(edit) => Self::ChangeStyle(edit),
            SavedWallEdit::ChangeSkirt(edit) => Self::ChangeSkirt(edit),
            SavedWallEdit::ChangePlasterPeelAmount(edit) => Self::ChangePlasterPeelAmount(edit),
            SavedWallEdit::MorphFreehandWall(edit) => Self::MorphFreehandWall(edit),
            SavedWallEdit::AdjustHeight {
                wall_index,
                before,
                after,
            } => Self::AdjustHeight {
                wall: WallId(wall_index),
                from: before,
                to: after,
            },
        })
    }
}

impl<'de> Deserialize<'de> for WallCompoundHistoryEdit {
    fn deserialize<D: Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        // Vec<Vec<_>> is the original ConcurrentEdits sequence. In particular,
        // empty and multi-command groups survive decoding with their indices.
        Ok(Self {
            groups: Vec::<Vec<WallHistoryEdit>>::deserialize(d)?,
        })
    }
}

impl Serialize for WallCompoundHistoryEdit {
    fn serialize<S: Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        self.groups.serialize(s)
    }
}

#[derive(Deserialize)]
enum SavedEdit {
    Tree(super::tree_edit::TreeEdit),
    Terrain(TerrainEdit),
    AutoClutter(super::auto_clutter_edit::AutoClutterEdit),
    DecoratorV5(super::decorator_edit::DecoratorV5Edit),
    #[serde(
        rename = "DecoratorOld",
        alias = "Decorator",
        alias = "Decorator4",
        alias = "Decorator5"
    )]
    Decorator(super::decorator_edit::DecoratorHistoryEdit),
    Color(super::color_edit::ColorEdit),
    Roof(super::building_transactions::RoofHistoryEdit),
    Path(crate::resources::raster_stroke::RasterStroke),
    Cobblestone1(crate::resources::raster_stroke::RasterStroke),
    Cobblestone2(crate::resources::raster_stroke::RasterStroke),
    Cobblestone3(crate::resources::raster_stroke::RasterStroke),
    Water(crate::resources::raster_stroke::RasterStroke),
    Garden(crate::resources::raster_stroke::RasterStroke),
    Stairs(super::stairs_history_edit::StairsHistoryEdit),
    #[serde(alias = "WallNewSystem")]
    Wall(WallCompoundHistoryEdit),
}
impl<'de> Deserialize<'de> for EditType {
    fn deserialize<D: Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        match SavedEdit::deserialize(d)? {
            SavedEdit::AutoClutter(edit) => Ok(Self::AutoClutter(Arc::new(edit))),
            SavedEdit::DecoratorV5(edit) => Ok(Self::DecoratorV5(Arc::new(edit))),
            SavedEdit::Decorator(edit) => Ok(Self::Decorator(Arc::new(edit))),
            SavedEdit::Color(edit) => Ok(Self::Color(Arc::new(edit))),
            SavedEdit::Stairs(edit) => Ok(Self::Stairs(Arc::new(edit))),
            SavedEdit::Path(edit) => Ok(Self::Path(Arc::new(edit))),
            SavedEdit::Cobblestone1(edit) => Ok(Self::Cobblestone1(Arc::new(edit))),
            SavedEdit::Cobblestone2(edit) => Ok(Self::Cobblestone2(Arc::new(edit))),
            SavedEdit::Cobblestone3(edit) => Ok(Self::Cobblestone3(Arc::new(edit))),
            SavedEdit::Water(edit) => Ok(Self::Water(Arc::new(edit))),
            SavedEdit::Garden(edit) => Ok(Self::Garden(Arc::new(edit))),
            SavedEdit::Roof(edit) => Ok(Self::Roof(Arc::new(edit))),
            SavedEdit::Tree(tree) => Ok(Self::Tree(Arc::new(tree))),
            SavedEdit::Terrain(terrain) => Ok(Self::Terrain(Arc::new(terrain))),
            SavedEdit::Wall(wall) => Ok(Self::Wall(Arc::new(wall))),
        }
    }
}

impl Serialize for EditType {
    fn serialize<S: Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        match self {
            Self::AutoClutter(edit) => {
                s.serialize_newtype_variant("EditType", 13, "AutoClutter", &**edit)
            }
            Self::DecoratorV5(edit) => {
                s.serialize_newtype_variant("EditType", 10, "DecoratorV5", &**edit)
            }
            Self::Decorator(edit) => {
                s.serialize_newtype_variant("EditType", 11, "DecoratorOld", &**edit)
            }
            Self::Color(edit) => s.serialize_newtype_variant("EditType", 9, "Color", &**edit),
            Self::Stairs(edit) => s.serialize_newtype_variant("EditType", 14, "Stairs", &**edit),
            Self::Path(edit) => s.serialize_newtype_variant("EditType", 1, "Path", &**edit),
            Self::Cobblestone1(edit) => {
                s.serialize_newtype_variant("EditType", 2, "Cobblestone1", &**edit)
            }
            Self::Cobblestone2(edit) => {
                s.serialize_newtype_variant("EditType", 3, "Cobblestone2", &**edit)
            }
            Self::Cobblestone3(edit) => {
                s.serialize_newtype_variant("EditType", 4, "Cobblestone3", &**edit)
            }
            Self::Water(edit) => s.serialize_newtype_variant("EditType", 5, "Water", &**edit),
            Self::Garden(edit) => s.serialize_newtype_variant("EditType", 6, "Garden", &**edit),
            Self::Roof(edit) => s.serialize_newtype_variant("EditType", 8, "Roof", &**edit),
            Self::Tree(tree) => s.serialize_newtype_variant("EditType", 12, "Tree", &**tree),
            Self::Terrain(terrain) => {
                s.serialize_newtype_variant("EditType", 0, "Terrain", &**terrain)
            }
            Self::Wall(wall) => s.serialize_newtype_variant("EditType", 7, "Wall", &**wall),
        }
    }
}

/// Original MultiEdit owns its decorator/stairs diffs alongside edit markers.
#[derive(Clone, Debug)]
pub struct SavedMultiEdit(pub HistoryEntry<EditType>);
impl Serialize for SavedMultiEdit {
    fn serialize<S: Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        use serde::ser::SerializeStruct;
        let mut value = s.serialize_struct("MultiEdit", 3)?;
        value.serialize_field("edits", &self.0.edits)?;
        if let Some(diff) = &self.0.auxiliary.decorator_diff {
            value.serialize_field(
                "decorators",
                &super::decorator_edit::DecoratorHistoryEdit {
                    diffs: (**diff).clone(),
                },
            )?;
        }
        if let Some(diff) = &self.0.auxiliary.stairs_diff {
            value.serialize_field("stairs", &**diff)?;
        }
        value.end()
    }
}
impl<'de> Deserialize<'de> for SavedMultiEdit {
    fn deserialize<D: Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        #[derive(Deserialize)]
        struct Input {
            edits: Vec<EditType>,
            #[serde(default)]
            decorators: Option<super::decorator_edit::DecoratorHistoryEdit>,
            #[serde(default)]
            stairs: Option<crate::resources::stairs::stairs_state::StairsHistoryDiff>,
        }
        let input = Input::deserialize(d)?;
        let mut entry = HistoryEntry::from_edits(input.edits);
        if let Some(diff) = input.decorators {
            entry.observed_auxiliary_counts[0] = diff.diffs.len();
            entry.auxiliary.decorator_diff = Some(Arc::new(diff.diffs));
        }
        if let Some(diff) = input.stairs {
            entry.observed_auxiliary_counts[1] = diff.removed.len() + diff.added.len();
            entry.observed_stairs_checksum = Some(diff.after_checksum);
            entry.auxiliary.stairs_diff = Some(Arc::new(diff));
        }
        Ok(Self(entry))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn original_mixed_multi_edit_aux_survives_save_reload_and_replay_expansion() {
        let fixture: serde_json::Value = serde_json::from_str(include_str!(
            "../../../tests/fixtures/original-mixed-history-b.json"
        ))
        .unwrap();
        let entries: Vec<SavedMultiEdit> = serde_json::from_value(fixture).unwrap();
        let mut decorators = 0;
        let mut stairs = 0;
        for entry in entries {
            if entry.0.auxiliary.decorator_diff.is_some() {
                decorators += 1;
            }
            if entry.0.auxiliary.stairs_diff.is_some() {
                stairs += 1;
            }
            let saved = serde_json::to_value(&entry).unwrap();
            let loaded: SavedMultiEdit = serde_json::from_value(saved.clone()).unwrap();
            assert_eq!(serde_json::to_value(&loaded).unwrap(), saved);
            assert_eq!(entry.0.auxiliary, loaded.0.auxiliary);
            for undo in [false, true] {
                let bucket = super::super::replay_cmd_buffer::ReplayBucket::expand_with_aux(
                    0,
                    &loaded.0,
                    undo,
                    true,
                    loaded.0.auxiliary.for_replay(undo),
                );
                assert!(
                    bucket.is_ok(),
                    "original entry replay expansion: {bucket:?}"
                );
            }
        }
        assert!(decorators >= 3 && stairs >= 3);
    }

    #[test]
    #[ignore = "external original asset diagnostic; root runs explicitly with TG_INSTALL_ROOT"]
    fn all_original_starting_build_histories_decode_save_reload() {
        let root = std::path::PathBuf::from(
            std::env::var_os("TG_INSTALL_ROOT")
                .expect("TG_INSTALL_ROOT must name the original B release root"),
        )
        .join("assets/starting-builds");
        let mut files = 0;
        let mut count = 0;
        for edition in std::fs::read_dir(root).unwrap() {
            for build in std::fs::read_dir(edition.unwrap().path()).unwrap() {
                let p = build.unwrap().path().join("history.json");
                if !p.is_file() {
                    continue;
                }
                let asset: serde_json::Value =
                    serde_json::from_str(&std::fs::read_to_string(&p).unwrap()).unwrap();
                let entries: Vec<SavedMultiEdit> =
                    serde_json::from_value(asset["History"]["edit_chain"].clone())
                        .unwrap_or_else(|e| panic!("{}: {e}", p.display()));
                count += entries.len();
                files += 1;
                for entry in entries {
                    let saved = serde_json::to_value(&entry).unwrap();
                    let reloaded: SavedMultiEdit = serde_json::from_value(saved.clone()).unwrap();
                    assert_eq!(
                        serde_json::to_value(reloaded).unwrap(),
                        saved,
                        "{}",
                        p.display()
                    );
                }
            }
        }
        assert_eq!((files, count), (33, 2120));
    }

    #[test]
    fn original_intro_retains_all_groups_and_decodes_into_live_commands() {
        let asset: serde_json::Value = serde_json::from_str(include_str!(
            "../../../tests/fixtures/original-intro-history-b.json"
        ))
        .unwrap();
        let entries: Vec<SavedMultiEdit> =
            serde_json::from_value(asset["History"]["edit_chain"].clone()).unwrap();
        assert_eq!(entries.len(), 13);
        let mut lengths = Vec::new();
        for entry in &entries {
            for edit in &entry.0.edits {
                let EditType::Wall(wall) = edit else {
                    panic!("original intro wall edit");
                };
                lengths.extend(wall.groups.iter().map(Vec::len));
                let expanded =
                    crate::systems::history::wall_replay_interpreter::expand_wall_compound(
                        wall, false, true,
                    )
                    .unwrap();
                assert_eq!(
                    expanded.iter().map(Vec::len).collect::<Vec<_>>(),
                    wall.groups.iter().map(Vec::len).collect::<Vec<_>>()
                );
                assert!(expanded.iter().flatten().all(|command| !command.timed));
                let backwards =
                    crate::systems::history::wall_replay_interpreter::expand_wall_compound(
                        wall, true, true,
                    )
                    .unwrap();
                assert_eq!(
                    backwards.iter().map(Vec::len).collect::<Vec<_>>(),
                    wall.groups.iter().rev().map(Vec::len).collect::<Vec<_>>()
                );
            }
        }
        assert_eq!(lengths.iter().filter(|&&n| n == 0).count(), 8);
        assert_eq!(lengths.iter().filter(|&&n| n == 2).count(), 4);
        assert_eq!(lengths.iter().sum::<usize>(), 22);
    }

    #[test]
    fn all_observed_original_wall_variants_preserve_saved_payloads() {
        let rows: serde_json::Value = serde_json::from_str(include_str!(
            "../../../tests/fixtures/original-wall-variants-b.json"
        ))
        .unwrap();
        assert_eq!(rows.as_object().unwrap().len(), 12);
        for (name, row) in rows.as_object().unwrap() {
            let edit: WallHistoryEdit =
                serde_json::from_value(row.clone()).unwrap_or_else(|e| panic!("{name}: {e}"));
            let saved = serde_json::to_value(&edit).unwrap();
            let back: WallHistoryEdit = serde_json::from_value(saved.clone()).unwrap();
            assert_eq!(serde_json::to_value(back).unwrap(), saved, "{name}");
            if let WallHistoryEdit::ChangeArchType(_) = edit {
                assert_eq!(edit.operation_tag() as u64, 8);
            }
            if !matches!(name.as_str(), "WallChanged" | "CreateWall" | "DeleteWall") {
                assert_eq!(&saved, row, "original named payload {name}");
            }
        }
        let rim = serde_json::json!({"ChangeFlatRoofRimStyle":{"wall_index":7,"before":"Rim","after":"NoRim"}});
        let edit: crate::resources::history::building_transactions::RoofHistoryEdit =
            serde_json::from_value(rim.clone()).unwrap();
        assert_eq!(edit.tag(), 4);
        assert_eq!(serde_json::to_value(edit).unwrap(), rim);
    }

    #[test]
    fn instant_state_aliases_are_not_independent_snapshots() {
        let asset: serde_json::Value = serde_json::from_str(include_str!(
            "../../../tests/fixtures/original-intro-history-b.json"
        ))
        .unwrap();
        let mut first =
            asset["History"]["edit_chain"][0]["edits"][0]["WallNewSystem"][0][0].clone();
        let object = first["CreateWall"].as_object_mut().unwrap();
        let after = object.remove("after").unwrap();
        object.insert("redo_instant_state".into(), after.clone());
        serde_json::from_value::<WallHistoryEdit>(first.clone()).unwrap();
        first["CreateWall"]["after"] = after;
        assert!(
            serde_json::from_value::<WallHistoryEdit>(first)
                .unwrap_err()
                .to_string()
                .contains("duplicate field")
        );
        assert!(serde_json::from_value::<EditType>(serde_json::json!({"unknown": []})).is_err());
    }
    #[test]
    fn original_b_terrain_variants_survive_saved_multi_edit_round_trip() {
        // Original EditType serializer 0x140316660 calls TerrainEdit 0x1404f41b0.
        // Values are retained native outputs, covering all three typed payloads.
        let edits: serde_json::Value = serde_json::from_str(r#"[
            {"Terrain":{"NewTerrainStroke":{"index":9,"stroke_pts":[[1.0,2.0,3.0],[4.0,5.0,6.0]],"height_ws":2.5}}},
            {"Terrain":{"EditTerrainStroke":{"index":7,"prev_height_ws":1.0,"new_height_ws":2.0,"prev_falloff":0.25,"new_falloff":0.5}}},
            {"Terrain":{"MoveTerrainStroke":{"id":3,"original_stroke_pts":[],"move_vec":[1.25,-2.0]}}}
        ]"#).unwrap();
        let saved = serde_json::json!({"edits": edits});
        let loaded: SavedMultiEdit = serde_json::from_value(saved.clone()).unwrap();
        assert_eq!(loaded.0.edits.len(), 3);
        assert!(
            loaded
                .0
                .edits
                .iter()
                .all(|edit| matches!(edit, EditType::Terrain(_)))
        );
        assert_eq!(serde_json::to_value(&loaded).unwrap(), saved);
        let encoded = serde_json::to_string(&loaded).unwrap();
        let reloaded: SavedMultiEdit = serde_json::from_str(&encoded).unwrap();
        assert_eq!(serde_json::to_value(&reloaded).unwrap(), saved);
    }
    #[test]
    fn delete_wall_distinct_height_round_trips_without_overwriting_before() {
        let curve = crate::resources::roofs::geometry::surface::Curve2::try_new(
            vec![[0.0, 0.0], [4.0, 0.0]],
            false,
        )
        .unwrap();
        // Use the existing saved schema's valid HalfTimbered style. Tag3 is
        // intentionally rejected; this test isolates independent Delete height.
        let state = WallState::new(curve, 3.9, 2, 0, false);
        let edit = WallHistoryEdit::Delete {
            wall: WallId(9),
            source: state.snapshot().clone(),
            color_id: 17,
            restored_height: 7.0,
        };
        let saved = serde_json::to_value(&edit).unwrap();
        assert_eq!(saved["DeleteWall"]["wall_height"], 7.0);
        assert_eq!(
            saved["DeleteWall"]["before"]["height"],
            serde_json::json!(3.9_f32)
        );
        let back: WallHistoryEdit = serde_json::from_value(saved.clone()).unwrap();
        assert_eq!(serde_json::to_value(&back).unwrap(), saved);
        let WallHistoryEdit::Delete {
            source,
            restored_height,
            ..
        } = back
        else {
            panic!("delete record");
        };
        assert_eq!(source.source.extra.to_bits(), 3.9_f32.to_bits());
        assert_eq!(restored_height.to_bits(), 7.0_f32.to_bits());
        let mut alias = saved.clone();
        let fields = alias["DeleteWall"].as_object_mut().unwrap();
        let before = fields.remove("before").unwrap();
        fields.insert("undo_instant_state".into(), before);
        let back: WallHistoryEdit = serde_json::from_value(alias).unwrap();
        assert_eq!(serde_json::to_value(back).unwrap(), saved);
    }
}
