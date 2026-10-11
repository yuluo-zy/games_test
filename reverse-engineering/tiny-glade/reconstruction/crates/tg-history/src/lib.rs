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
    pub chain_index: u64,
    pub entries: usize,
    pub command_counts: BTreeMap<String, usize>,
    pub roof_command_counts: BTreeMap<String, usize>,
    pub unknown_roof_commands: Vec<String>,
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

    /// Visits all stored commands, including entries beyond the cursor.
    /// Current-state selection requires the original loader/undo consumers.
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
            .and_then(Value::as_u64)
            .ok_or_else(|| DecodeError("/History/chain_index: expected u64".into()))?;
        let mut result = HistorySummary {
            version,
            chain_index,
            entries: self.entries()?.len(),
            command_counts: BTreeMap::new(),
            roof_command_counts: BTreeMap::new(),
            unknown_roof_commands: Vec::new(),
            replay_verified: false,
        };
        for (i, entry) in self.entries()?.iter().enumerate() {
            for (j, edit) in self.edits(entry, i)?.iter().enumerate() {
                let pointer = format!("/History/edit_chain/{i}/edits/{j}");
                let (tag, payload) = singleton(edit, &pointer)?;
                *result.command_counts.entry(tag.into()).or_default() += 1;
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
        let bytes = br#"{"Version":27,"History":{"chain_index":0,"edit_chain":[{"edits":[]},{"edits":[{"Roof":{"Future":{"id":18446744073709551615}}},{"FutureWorld":{"id":11519500871291546481}}]}],"snapshots":[{"snapshot_id":11519500871291546481}]},"Extension":{"x":9}}"#;
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
        let bad = br#"{"Version":17,"History":{"chain_index":0,"edit_chain":[{"edits":[{"Roof":{},"Wall":[]}]}]}}"#;
        let error = HistoryDocument::parse(bad).unwrap_err();
        assert!(error.0.contains("/History/edit_chain/0/edits/0"));
    }
}
