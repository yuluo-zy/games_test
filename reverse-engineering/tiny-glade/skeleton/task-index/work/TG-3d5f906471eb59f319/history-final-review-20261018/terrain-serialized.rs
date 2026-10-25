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
            } => SavedWallEdit::DeleteWall {
                wall_index: id.0,
                wall_height: source.source.extra,
                before: wall(source)?,
                color_id: *color_id,
            },
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
            } => {
                // The current live Delete payload stores the source height.
                // Preserve that owner until the distinct legacy wall_height
                // consumer is recovered; never discard a different value.
                if before.height().to_bits() != wall_height.to_bits() {
                    return Err(D::Error::custom(
                        "B DeleteWall distinct wall_height consumer is not restored",
                    ));
                }
                Self::Delete {
                    wall: WallId(wall_index),
                    source: before.snapshot().clone(),
                    color_id,
                }
            }
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
    Terrain(TerrainEdit),
    #[serde(alias = "WallNewSystem")]
    Wall(WallCompoundHistoryEdit),
}
impl<'de> Deserialize<'de> for EditType {
    fn deserialize<D: Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        match SavedEdit::deserialize(d)? {
            SavedEdit::Terrain(terrain) => Ok(Self::Terrain(Arc::new(terrain))),
            SavedEdit::Wall(wall) => Ok(Self::Wall(Arc::new(wall))),
        }
    }
}

impl Serialize for EditType {
    fn serialize<S: Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        match self {
            Self::Terrain(terrain) => {
                s.serialize_newtype_variant("EditType", 0, "Terrain", &**terrain)
            }
            Self::Wall(wall) => s.serialize_newtype_variant("EditType", 7, "Wall", &**wall),
            _ => Err(serde::ser::Error::custom(
                "B History save encoder for this edit is not restored",
            )),
        }
    }
}

/// A decoded original MultiEdit, before it is installed in the History ledger.
/// The auxiliary owners are kept as input until their native schema is usable;
/// nonempty data is rejected instead of losing decorator or stairs changes.
#[derive(Clone, Debug)]
pub struct SavedMultiEdit(pub HistoryEntry<EditType>);
impl Serialize for SavedMultiEdit {
    fn serialize<S: Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        use serde::ser::SerializeStruct;
        if self.0.observed_auxiliary_counts != [0; 3] || self.0.observed_stairs_checksum.is_some() {
            return Err(serde::ser::Error::custom(
                "B MultiEdit auxiliary save encoder is not restored",
            ));
        }
        let mut value = s.serialize_struct("MultiEdit", 1)?;
        value.serialize_field("edits", &self.0.edits)?;
        value.end()
    }
}
impl<'de> Deserialize<'de> for SavedMultiEdit {
    fn deserialize<D: Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        #[derive(Deserialize)]
        struct Input {
            edits: Vec<EditType>,
            #[serde(default)]
            decorators: Option<serde_json::Value>,
            #[serde(default)]
            stairs: Option<serde_json::Value>,
        }
        let input = Input::deserialize(d)?;
        if input.decorators.is_some() || input.stairs.is_some() {
            return Err(D::Error::custom(
                "B MultiEdit decorator/stairs saved owner decoder is not restored",
            ));
        }
        Ok(Self(HistoryEntry::from_edits(input.edits)))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

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
}
