//! Observed history data, not an implementation of undo/replay semantics.
//! Evidence: reconstruction/history/findings.md and three-version sample audit.
//! Keep the raw document so unknown commands, extensions and u64 IDs survive.
use serde::{Deserialize, Serialize};
use serde_json::{Map, Value};
use std::{collections::BTreeMap, error::Error, fmt};

#[derive(Clone, Debug, Deserialize, Serialize, PartialEq, Eq)]
pub enum RidgeDirection {
    Width,
    Length,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct RoofParameters {
    pub height_01: f32,
    pub profile_01: f32,
    pub eave_length_01: f32,
    pub ridge_length_01: f32,
    pub ridge_dir: RidgeDirection,
    pub tip_offset_01: [f32; 2],
    #[serde(flatten)]
    pub extensions: Map<String, Value>,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct RoofPresence {
    /// Serialized wall_index, not proof of the internal ECS/entity ID encoding.
    pub wall_index: u64,
    pub color_id: u64,
    pub params: RoofParameters,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub flat_roof_style: Option<Value>,
    #[serde(flatten)]
    pub extensions: Map<String, Value>,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct RoofVisualEdit {
    pub wall_index: u64,
    pub prev_height: f32,
    pub new_height: f32,
    pub prev_profile: f32,
    pub new_profile: f32,
    pub prev_eave: f32,
    pub new_eave: f32,
    pub prev_ridge: f32,
    pub new_ridge: f32,
    pub prev_tip_offset: [f32; 2],
    pub new_tip_offset: [f32; 2],
    #[serde(flatten)]
    pub extensions: Map<String, Value>,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct RoofDirectionEdit {
    pub wall_index: u64,
    pub from: RidgeDirection,
    pub to: RidgeDirection,
    #[serde(flatten)]
    pub extensions: Map<String, Value>,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub enum RoofCommand {
    CreateRoof(RoofPresence),
    DeleteRoof(RoofPresence),
    VisualEdit(RoofVisualEdit),
    SwitchRoofRidgeDir(RoofDirectionEdit),
}

#[derive(Clone, Debug)]
pub struct LocatedRoofCommand {
    pub entry_index: usize,
    pub edit_index: usize,
    pub command: RoofCommand,
}

/// An outer command tag observed in the shipped A/B/C history documents.
/// `Unknown` stays available because the observed sample set is not a closed schema.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum HistoryCommandKind {
    WallNewSystem,
    Wall,
    Roof,
    DecoratorV5,
    Terrain,
    Tree,
    Color,
    Stairs,
    Path,
    Water,
    Garden,
    Cobblestone1,
    Cobblestone2,
    Unknown(String),
}

impl HistoryCommandKind {
    fn from_tag(tag: &str) -> Self {
        match tag {
            "WallNewSystem" => Self::WallNewSystem,
            "Wall" => Self::Wall,
            "Roof" => Self::Roof,
            "DecoratorV5" => Self::DecoratorV5,
            "Terrain" => Self::Terrain,
            "Tree" => Self::Tree,
            "Color" => Self::Color,
            "Stairs" => Self::Stairs,
            "Path" => Self::Path,
            "Water" => Self::Water,
            "Garden" => Self::Garden,
            "Cobblestone1" => Self::Cobblestone1,
            "Cobblestone2" => Self::Cobblestone2,
            other => Self::Unknown(other.to_owned()),
        }
    }
}

/// One `{ command_tag: payload }` value in an edit-chain entry.
#[derive(Clone, Debug)]
pub struct LocatedHistoryCommand {
    pub entry_index: usize,
    pub edit_index: usize,
    pub tag: String,
    pub kind: HistoryCommandKind,
    /// Kept as JSON because most payloads contain version-specific fields and
    /// some observed command families are not yet semantically decoded.
    pub payload: Value,
}

/// The snapshot file itself is an external binary object. These fields only
/// identify the reference stored in the history document.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct HistorySnapshotReference {
    /// The original history cursor and snapshot indices are signed 32-bit
    /// values (`History::chain_index` at `History + 0x68`; snapshot record
    /// index at `HistoryChainSnapshot + 0x08`).
    pub chain_index: i32,
    pub snapshot_id: u64,
    pub raw: Value,
}

/// A borrowed view of an entry and its non-command sidecar records.
#[derive(Clone, Debug)]
pub struct HistoryEntryView<'a> {
    pub entry_index: usize,
    pub raw: &'a Value,
    pub decorators: Option<&'a Value>,
    pub stairs: Option<&'a Value>,
    pub commands: Vec<LocatedHistoryCommand>,
}

#[derive(Debug)]
pub struct DecodeError(pub String);
impl fmt::Display for DecodeError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(&self.0)
    }
}
impl Error for DecodeError {}

#[derive(Clone, Debug)]
pub struct HistoryDocument {
    raw: Value,
}

#[derive(Debug, Serialize)]
pub struct HistorySummary {
    pub version: u64,
    pub chain_index: i32,
    pub entries: usize,
    pub command_counts: BTreeMap<String, usize>,
    pub unknown_commands: Vec<String>,
    pub roof_command_counts: BTreeMap<String, usize>,
    pub unknown_roof_commands: Vec<String>,
    pub entries_after_chain_index: usize,
    pub snapshot_references: usize,
    pub replay_verified: bool,
}

fn object<'a>(v: &'a Value, pointer: &str) -> Result<&'a Map<String, Value>, DecodeError> {
    v.as_object()
        .ok_or_else(|| DecodeError(format!("{pointer}: expected object")))
}
fn singleton<'a>(v: &'a Value, pointer: &str) -> Result<(&'a str, &'a Value), DecodeError> {
    let obj = object(v, pointer)?;
    if obj.len() != 1 {
        return Err(DecodeError(format!("{pointer}: expected one enum tag")));
    }
    let (tag, payload) = obj.iter().next().unwrap();
    Ok((tag, payload))
}

impl HistoryDocument {
    pub fn parse(bytes: &[u8]) -> Result<Self, DecodeError> {
        let raw: Value = serde_json::from_slice(bytes).map_err(|e| DecodeError(e.to_string()))?;
        let document = Self { raw };
        document.summary()?;
        Ok(document)
    }

    /// Original JSON data; typed f32 projections never replace these numbers.
    pub fn raw(&self) -> &Value {
        &self.raw
    }

    pub fn to_json(&self) -> Result<Vec<u8>, serde_json::Error> {
        serde_json::to_vec(&self.raw)
    }

    fn entries(&self) -> Result<&Vec<Value>, DecodeError> {
        self.raw
            .pointer("/History/edit_chain")
            .and_then(Value::as_array)
            .ok_or_else(|| DecodeError("/History/edit_chain: expected array".into()))
    }

    fn edits<'a>(&self, entry: &'a Value, index: usize) -> Result<&'a Vec<Value>, DecodeError> {
        entry.get("edits").and_then(Value::as_array).ok_or_else(|| {
            DecodeError(format!("/History/edit_chain/{index}/edits: expected array"))
        })
    }

    fn history_value(&self, key: &str) -> Result<&Value, DecodeError> {
        self.raw
            .pointer(&format!("/History/{key}"))
            .ok_or_else(|| DecodeError(format!("/History/{key}: missing field")))
    }

    /// Visits every stored command, including commands after the serialized
    /// cursor. Payloads remain raw so old and future command schemas survive.
    pub fn commands(&self) -> Result<Vec<LocatedHistoryCommand>, DecodeError> {
        let mut commands = Vec::new();
        for (entry_index, entry) in self.entries()?.iter().enumerate() {
            for (edit_index, edit) in self.edits(entry, entry_index)?.iter().enumerate() {
                let pointer = format!("/History/edit_chain/{entry_index}/edits/{edit_index}");
                let (tag, payload) = singleton(edit, &pointer)?;
                commands.push(LocatedHistoryCommand {
                    entry_index,
                    edit_index,
                    tag: tag.to_owned(),
                    kind: HistoryCommandKind::from_tag(tag),
                    payload: payload.clone(),
                });
            }
        }
        Ok(commands)
    }

    /// Returns commands and the decorator/stairs sidecars attached to each
    /// edit-chain entry. Sidecars are distinct from the `edits` tag list.
    pub fn history_entries(&self) -> Result<Vec<HistoryEntryView<'_>>, DecodeError> {
        let mut result = Vec::new();
        for (entry_index, raw) in self.entries()?.iter().enumerate() {
            let commands = self
                .edits(raw, entry_index)?
                .iter()
                .enumerate()
                .map(|(edit_index, edit)| {
                    let pointer = format!("/History/edit_chain/{entry_index}/edits/{edit_index}");
                    let (tag, payload) = singleton(edit, &pointer)?;
                    Ok(LocatedHistoryCommand {
                        entry_index,
                        edit_index,
                        tag: tag.to_owned(),
                        kind: HistoryCommandKind::from_tag(tag),
                        payload: payload.clone(),
                    })
                })
                .collect::<Result<Vec<_>, DecodeError>>()?;
            result.push(HistoryEntryView {
                entry_index,
                raw,
                decorators: raw.get("decorators"),
                stairs: raw.get("stairs"),
                commands,
            });
        }
        Ok(result)
    }

    /// Returns the serialized snapshot references without opening snapshot
    /// files or assuming that every referenced file is present.
    pub fn snapshot_references(&self) -> Result<Vec<HistorySnapshotReference>, DecodeError> {
        let values = self
            .history_value("snapshots")?
            .as_array()
            .ok_or_else(|| DecodeError("/History/snapshots: expected array".into()))?;
        values
            .iter()
            .enumerate()
            .map(|(index, value)| {
                let pointer = format!("/History/snapshots/{index}");
                let obj = object(value, &pointer)?;
                let chain_index = obj
                    .get("chain_index")
                    .and_then(Value::as_i64)
                    .and_then(|value| i32::try_from(value).ok())
                    .ok_or_else(|| DecodeError(format!("{pointer}/chain_index: expected i32")))?;
                let snapshot_id = obj
                    .get("snapshot_id")
                    .and_then(Value::as_u64)
                    .ok_or_else(|| DecodeError(format!("{pointer}/snapshot_id: expected u64")))?;
                Ok(HistorySnapshotReference {
                    chain_index,
                    snapshot_id,
                    raw: value.clone(),
                })
            })
            .collect()
    }

    /// Selects the nearest externally resolved snapshot to `cursor`.
    ///
    /// Original B `History::closest_resolved_snapshot` (0x140a84a50) first
    /// converts each runtime snapshot through `ResolvedHistoryChainSnapshot::try_from`
    /// and then chooses the nearest chain index. The conversion succeeds only
    /// when the snapshot Arc strong count is at least two (0x140a84260), which
    /// is runtime ownership state and cannot be inferred from a JSON file.
    /// `is_resolved` supplies that caller-side result for each serialized ID.
    /// On equal distance, the original chooses the snapshot after the cursor.
    pub fn closest_resolved_snapshot(
        &self,
        cursor: i32,
        mut is_resolved: impl FnMut(u64) -> bool,
    ) -> Result<Option<HistorySnapshotReference>, DecodeError> {
        let mut closest: Option<(i64, HistorySnapshotReference)> = None;
        for candidate in self.snapshot_references()? {
            if !is_resolved(candidate.snapshot_id) {
                continue;
            }
            let distance = (i64::from(candidate.chain_index) - i64::from(cursor)).abs();
            let replace = closest.as_ref().map_or(true, |(best_distance, best)| {
                distance < *best_distance
                    || (distance == *best_distance && candidate.chain_index > best.chain_index)
            });
            if replace {
                closest = Some((distance, candidate));
            }
        }
        Ok(closest.map(|(_, snapshot)| snapshot))
    }

    /// Returns a snapshot only when a stored snapshot is exactly at the
    /// serialized cursor. Original B `History::current_snapshot` (0x140a84650)
    /// performs this equality check after locating the nearest table entry.
    pub fn current_snapshot_reference(
        &self,
    ) -> Result<Option<HistorySnapshotReference>, DecodeError> {
        let cursor = self
            .raw
            .pointer("/History/chain_index")
            .and_then(Value::as_i64)
            .and_then(|value| i32::try_from(value).ok())
            .ok_or_else(|| DecodeError("/History/chain_index: expected i32".into()))?;
        Ok(self
            .snapshot_references()?
            .into_iter()
            .find(|snapshot| snapshot.chain_index == cursor))
    }

    /// The serialized `history_start` value is optional and kept raw because
    /// its version-specific loading semantics are not part of this schema API.
    pub fn history_start(&self) -> Option<&Value> {
        self.raw.pointer("/History/history_start")
    }

    /// Visits all stored commands, including entries beyond the cursor.
    /// Applying commands still requires the original world/replay consumers.
    pub fn roof_commands(&self) -> Result<Vec<LocatedRoofCommand>, DecodeError> {
        let mut commands = Vec::new();
        for (entry_index, entry) in self.entries()?.iter().enumerate() {
            for (edit_index, edit) in self.edits(entry, entry_index)?.iter().enumerate() {
                let pointer = format!("/History/edit_chain/{entry_index}/edits/{edit_index}");
                let (tag, payload) = singleton(edit, &pointer)?;
                if tag != "Roof" {
                    continue;
                }
                let (roof_tag, _) = singleton(payload, &format!("{pointer}/Roof"))?;
                if !matches!(
                    roof_tag,
                    "CreateRoof" | "DeleteRoof" | "VisualEdit" | "SwitchRoofRidgeDir"
                ) {
                    continue; // Preserved in raw(), reported by summary().
                }
                let command = serde_json::from_value(payload.clone())
                    .map_err(|e| DecodeError(format!("{pointer}/Roof: {e}")))?;
                commands.push(LocatedRoofCommand {
                    entry_index,
                    edit_index,
                    command,
                });
            }
        }
        Ok(commands)
    }

    pub fn summary(&self) -> Result<HistorySummary, DecodeError> {
        let version = self
            .raw
            .get("Version")
            .and_then(Value::as_u64)
            .ok_or_else(|| DecodeError("/Version: expected u64".into()))?;
        let chain_index = self
            .raw
            .pointer("/History/chain_index")
            .and_then(Value::as_i64)
            .and_then(|value| i32::try_from(value).ok())
            .ok_or_else(|| DecodeError("/History/chain_index: expected i32".into()))?;
        let mut result = HistorySummary {
            version,
            chain_index,
            entries: self.entries()?.len(),
            command_counts: BTreeMap::new(),
            unknown_commands: Vec::new(),
            roof_command_counts: BTreeMap::new(),
            unknown_roof_commands: Vec::new(),
            entries_after_chain_index: 0,
            snapshot_references: self.snapshot_references()?.len(),
            replay_verified: false,
        };
        let cursor_position = usize::try_from(chain_index.saturating_add(1).max(0))
            .expect("nonnegative i32 always fits usize");
        result.entries_after_chain_index = result.entries.saturating_sub(cursor_position);
        for (i, entry) in self.entries()?.iter().enumerate() {
            for (j, edit) in self.edits(entry, i)?.iter().enumerate() {
                let pointer = format!("/History/edit_chain/{i}/edits/{j}");
                let (tag, payload) = singleton(edit, &pointer)?;
                *result.command_counts.entry(tag.into()).or_default() += 1;
                if matches!(
                    HistoryCommandKind::from_tag(tag),
                    HistoryCommandKind::Unknown(_)
                ) {
                    result.unknown_commands.push(format!("{pointer}/{tag}"));
                }
                if tag == "Roof" {
                    let (roof_tag, _) = singleton(payload, &format!("{pointer}/Roof"))?;
                    *result
                        .roof_command_counts
                        .entry(roof_tag.into())
                        .or_default() += 1;
                    if !matches!(
                        roof_tag,
                        "CreateRoof" | "DeleteRoof" | "VisualEdit" | "SwitchRoofRidgeDir"
                    ) {
                        result
                            .unknown_roof_commands
                            .push(format!("{pointer}/Roof/{roof_tag}"));
                    }
                }
            }
        }
        Ok(result)
    }
}

pub fn summarize_json(bytes: &[u8]) -> Result<String, DecodeError> {
    let document = HistoryDocument::parse(bytes)?;
    document.roof_commands()?;
    serde_json::to_string(&document.summary()?).map_err(|e| DecodeError(e.to_string()))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn preserves_large_ids_unknown_commands_and_cursor_tail() {
        let bytes = br#"{"Version":27,"History":{"chain_index":0,"edit_chain":[{"edits":[]},{"edits":[{"Roof":{"Future":{"id":18446744073709551615}}},{"FutureWorld":{"id":11519500871291546481}}]}],"snapshots":[{"chain_index":0,"snapshot_id":11519500871291546481}]},"Extension":{"x":9}}"#;
        let document = HistoryDocument::parse(bytes).unwrap();
        let serialized = document.to_json().unwrap();
        assert_eq!(
            document.raw(),
            &serde_json::from_slice::<Value>(&serialized).unwrap()
        );
        assert_eq!(
            document
                .raw()
                .pointer("/History/snapshots/0/snapshot_id")
                .unwrap()
                .as_u64(),
            Some(11519500871291546481)
        );
        assert_eq!(
            document
                .raw()
                .pointer("/History/edit_chain/1/edits/0/Roof/Future/id")
                .unwrap()
                .as_u64(),
            Some(u64::MAX)
        );
        let summary = document.summary().unwrap();
        assert_eq!(summary.entries, 2);
        assert_eq!(summary.chain_index, 0);
        assert_eq!(summary.unknown_roof_commands.len(), 1);
        assert!(document.roof_commands().unwrap().is_empty());
    }

    #[test]
    fn invalid_enum_shape_returns_pointer() {
        let bad = br#"{"Version":17,"History":{"chain_index":0,"edit_chain":[{"edits":[{"Roof":{},"Wall":[]}]}],"snapshots":[]}}"#;
        let error = HistoryDocument::parse(bad).unwrap_err();
        assert!(error.0.contains("/History/edit_chain/0/edits/0"));
    }

    #[test]
    fn recognizes_observed_command_families_and_keeps_future_tags() {
        let tags = [
            "WallNewSystem",
            "Wall",
            "Roof",
            "DecoratorV5",
            "Terrain",
            "Tree",
            "Color",
            "Stairs",
            "Path",
            "Water",
            "Garden",
            "Cobblestone1",
            "Cobblestone2",
            "FutureCommand",
        ];
        let edits = tags
            .iter()
            .map(|tag| format!("{{\"{tag}\":{{\"opaque\":true}}}}"))
            .collect::<Vec<_>>()
            .join(",");
        let bytes = format!(
            "{{\"Version\":27,\"History\":{{\"chain_index\":0,\"edit_chain\":[{{\"edits\":[{edits}]}}],\"snapshots\":[]}}}}"
        );
        let document = HistoryDocument::parse(bytes.as_bytes()).unwrap();
        let commands = document.commands().unwrap();

        assert_eq!(commands.len(), tags.len());
        assert!(matches!(
            commands[0].kind,
            HistoryCommandKind::WallNewSystem
        ));
        assert!(matches!(commands[1].kind, HistoryCommandKind::Wall));
        assert!(matches!(
            commands[12].kind,
            HistoryCommandKind::Cobblestone2
        ));
        assert_eq!(
            commands[13].kind,
            HistoryCommandKind::Unknown("FutureCommand".into())
        );
        assert_eq!(commands[13].payload["opaque"], true);
        assert_eq!(
            document.summary().unwrap().unknown_commands,
            vec!["/History/edit_chain/0/edits/13/FutureCommand"]
        );
    }

    #[test]
    fn exposes_entry_sidecars_and_snapshot_refs_without_replay_assumptions() {
        let bytes = br#"{"Version":27,"History":{"chain_index":0,"history_start":-1,"edit_chain":[{"edits":[{"Wall":{"opaque":1}}],"decorators":{"diffs":[]},"stairs":{"hash_before":7,"hash_after":8}},{"edits":[]}],"snapshots":[{"chain_index":0,"snapshot_id":11519500871291546481}]}}"#;
        let document = HistoryDocument::parse(bytes).unwrap();
        let entries = document.history_entries().unwrap();
        let snapshots = document.snapshot_references().unwrap();
        let summary = document.summary().unwrap();

        assert_eq!(entries.len(), 2);
        assert!(entries[0].decorators.is_some());
        assert!(entries[0].stairs.is_some());
        assert_eq!(entries[0].commands[0].kind, HistoryCommandKind::Wall);
        assert_eq!(document.history_start().unwrap(), -1);
        assert_eq!(snapshots[0].snapshot_id, 11519500871291546481);
        assert_eq!(summary.entries_after_chain_index, 1);
        assert_eq!(summary.snapshot_references, 1);
    }

    #[test]
    fn decodes_the_four_observed_roof_variants() {
        let bytes = br#"{"Version":27,"History":{"chain_index":3,"edit_chain":[{"edits":[{"Roof":{"CreateRoof":{"wall_index":7,"color_id":9,"params":{"height_01":0.1,"profile_01":0.2,"eave_length_01":0.3,"ridge_length_01":0.4,"ridge_dir":"Width","tip_offset_01":[0.5,0.6]}}}},{"Roof":{"DeleteRoof":{"wall_index":7,"color_id":9,"params":{"height_01":0.1,"profile_01":0.2,"eave_length_01":0.3,"ridge_length_01":0.4,"ridge_dir":"Length","tip_offset_01":[0.5,0.6]}}}},{"Roof":{"VisualEdit":{"wall_index":7,"prev_height":0.1,"new_height":0.2,"prev_profile":0.3,"new_profile":0.4,"prev_eave":0.5,"new_eave":0.6,"prev_ridge":0.7,"new_ridge":0.8,"prev_tip_offset":[0.0,0.1],"new_tip_offset":[0.2,0.3]}}},{"Roof":{"SwitchRoofRidgeDir":{"wall_index":7,"from":"Width","to":"Length"}}}]}],"snapshots":[]}}"#;
        let document = HistoryDocument::parse(bytes).unwrap();
        let commands = document.roof_commands().unwrap();

        assert_eq!(commands.len(), 4);
        assert!(matches!(commands[0].command, RoofCommand::CreateRoof(_)));
        assert!(matches!(commands[1].command, RoofCommand::DeleteRoof(_)));
        assert!(matches!(commands[2].command, RoofCommand::VisualEdit(_)));
        assert!(matches!(
            commands[3].command,
            RoofCommand::SwitchRoofRidgeDir(_)
        ));
        assert_eq!(commands[3].entry_index, 0);
        assert_eq!(commands[3].edit_index, 3);
    }
}
