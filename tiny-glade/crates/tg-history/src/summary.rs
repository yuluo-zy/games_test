use std::collections::BTreeMap;

use serde::Serialize;

use crate::document::validated_singleton;
use crate::error::escape_pointer_segment;
use crate::{DecodeError, HistoryCommandKind, HistoryDocument, RoofCommand};

/// Owned metadata and tag counts for every stored entry, including the cursor tail.
/// Counts describe serialized commands and make no claim about applied world state.
#[derive(Debug, Serialize)]
pub struct HistorySummary {
    /// The document's `Version` value, not a product release number.
    pub version: u64,
    /// The serialized signed history cursor.
    pub chain_index: i32,
    /// Total entries, including entries beyond the cursor.
    pub entries: usize,
    /// Outer-tag occurrence counts, including unknown command families.
    pub command_counts: BTreeMap<String, usize>,
    /// RFC 6901 pointers to unknown outer command payloads.
    pub unknown_commands: Vec<String>,
    /// Nested roof-tag occurrence counts, without typed payload validation.
    pub roof_command_counts: BTreeMap<String, usize>,
    /// RFC 6901 pointers to unknown roof command payloads.
    pub unknown_roof_commands: Vec<String>,
    /// Entries with a stored index greater than `chain_index`.
    pub entries_after_chain_index: usize,
    /// Number of serialized references, regardless of external file availability.
    pub snapshot_references: usize,
}

impl HistoryDocument {
    /// Counts every stored tag without decoding typed roof payloads.
    /// The result owns its maps and paths and can outlive the document; parsing
    /// already guarantees the required structure, so this operation is infallible.
    pub fn summary(&self) -> HistorySummary {
        let chain_index = self.chain_index();
        let entries = self.history_entries().len();
        // Compute in i64: the exclusive cursor position can be i32::MAX + 1.
        // The resulting nonnegative value fits usize on supported 32/64-bit hosts.
        let cursor_position = usize::try_from((i64::from(chain_index) + 1).max(0))
            .expect("nonnegative i32 plus one fits usize");
        let mut result = HistorySummary {
            version: self.version(),
            chain_index,
            entries,
            command_counts: BTreeMap::new(),
            unknown_commands: Vec::new(),
            roof_command_counts: BTreeMap::new(),
            unknown_roof_commands: Vec::new(),
            entries_after_chain_index: entries.saturating_sub(cursor_position),
            snapshot_references: self.snapshot_references().len(),
        };
        for command in self.commands() {
            increment(&mut result.command_counts, command.tag);
            if matches!(command.kind, HistoryCommandKind::Unknown(_)) {
                result.unknown_commands.push(format!(
                    "/History/edit_chain/{}/edits/{}/{}",
                    command.entry_index,
                    command.edit_index,
                    escape_pointer_segment(command.tag),
                ));
            }
            if command.kind == HistoryCommandKind::Roof {
                let (tag, _) = validated_singleton(command.payload);
                increment(&mut result.roof_command_counts, tag);
                if !RoofCommand::supports_tag(tag) {
                    result.unknown_roof_commands.push(format!(
                        "/History/edit_chain/{}/edits/{}/Roof/{}",
                        command.entry_index,
                        command.edit_index,
                        escape_pointer_segment(tag),
                    ));
                }
            }
        }
        result
    }
}

fn increment(counts: &mut BTreeMap<String, usize>, tag: &str) {
    if let Some(count) = counts.get_mut(tag) {
        *count += 1;
    } else {
        counts.insert(tag.to_owned(), 1);
    }
}

/// Parses UTF-8 JSON and returns the serialized [`HistorySummary`].
///
/// Invalid JSON or document structure returns [`DecodeError`], as does a JSON
/// serialization failure. Known roof payload fields stay unchecked, matching
/// [`HistoryDocument::parse`] followed by [`HistoryDocument::summary`].
pub fn summarize_json(bytes: &[u8]) -> Result<String, DecodeError> {
    let document = HistoryDocument::parse(bytes)?;
    serde_json::to_string(&document.summary()).map_err(|source| DecodeError::json("", source))
}
