use serde::Deserialize;
use serde_json::Value;

use crate::{DecodeError, LocatedRoofCommand, RoofCommand};

/// Classification of an outer command's serialized tag.
/// Variant names follow the JSON format without implying a migration between
/// families. An unknown tag borrows its name from the document without allocation.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum HistoryCommandKind<'a> {
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
    /// A tag outside the supported classification, retaining its original name.
    Unknown(&'a str),
}

impl<'a> HistoryCommandKind<'a> {
    fn from_tag(tag: &'a str) -> Self {
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
            other => Self::Unknown(other),
        }
    }
}

/// One borrowed `{ command_tag: payload }` value and its stored location.
/// Every reference remains valid for the source document's lifetime `'a`.
#[derive(Clone, Copy, Debug)]
pub struct LocatedHistoryCommand<'a> {
    /// Zero-based position in `History.edit_chain`.
    pub entry_index: usize,
    /// Zero-based position in the entry's `edits` list.
    pub edit_index: usize,
    pub tag: &'a str,
    pub kind: HistoryCommandKind<'a>,
    /// The unmodified payload, including fields outside the typed schema.
    pub payload: &'a Value,
}

/// A serialized reference to an external snapshot.
/// `raw` borrows the document for `'a`; this view neither opens a snapshot file
/// nor establishes whether the caller can resolve its contents.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct HistorySnapshotReference<'a> {
    /// The stored history index; any value representable by `i32` is preserved.
    pub chain_index: i32,
    /// The stored snapshot identifier, interpreted by the caller.
    pub snapshot_id: u64,
    /// The complete reference object, including unknown properties.
    pub raw: &'a Value,
}

/// An entry and its optional sidecars, borrowing the immutable document.
/// References borrow the document for `'a`. An absent sidecar is `None`; an
/// explicitly stored JSON `null` remains `Some(Value::Null)`.
#[derive(Clone, Copy, Debug)]
pub struct HistoryEntryView<'a> {
    /// Zero-based position in `History.edit_chain`.
    pub entry_index: usize,
    /// The complete entry object, including unknown properties.
    pub raw: &'a Value,
    /// The entry's `decorators` sidecar, outside the command list.
    pub decorators: Option<&'a Value>,
    /// The entry's `stairs` sidecar, outside the command list.
    pub stairs: Option<&'a Value>,
    edits: &'a [Value],
}

impl<'a> HistoryEntryView<'a> {
    /// Iterates this entry's commands in stored order without copying payloads.
    /// The iterator borrows the document for `'a`, so it can outlive this view.
    pub fn commands(
        &self,
    ) -> impl DoubleEndedIterator<Item = LocatedHistoryCommand<'a>>
    + ExactSizeIterator
    + Clone
    + 'a
    + use<'a> {
        let entry_index = self.entry_index;
        self.edits
            .iter()
            .enumerate()
            .map(move |(edit_index, edit)| {
                let (tag, payload) = validated_singleton(edit);
                LocatedHistoryCommand {
                    entry_index,
                    edit_index,
                    tag,
                    kind: HistoryCommandKind::from_tag(tag),
                    payload,
                }
            })
    }
}

/// An owned JSON document with a validated, immutable history structure.
///
/// Only `parse` constructs this type, and `raw` permits shared access only.
/// Accessors can therefore use the structural invariants without revalidation.
#[derive(Clone, Debug)]
pub struct HistoryDocument {
    raw: Value,
    version: u64,
    chain_index: i32,
}

impl HistoryDocument {
    /// Parses UTF-8 JSON into an owned document and validates its structure once.
    ///
    /// Requires `Version: u64`, `History.chain_index: i32`, entry `edits` arrays,
    /// single-tag outer commands, single-tag roof commands, and snapshot objects
    /// with `chain_index: i32` and `snapshot_id: u64`. Unknown tags and properties
    /// are retained. No cursor-to-entry bounds or roof numeric ranges are imposed.
    /// Known roof fields remain unchecked until [`Self::roof_commands`] is used.
    /// The returned document does not borrow `bytes`.
    ///
    /// # Errors
    /// Returns [`DecodeError`] for invalid JSON or missing/malformed structural
    /// fields, retaining the JSON Pointer and any underlying JSON codec error.
    pub fn parse(bytes: &[u8]) -> Result<Self, DecodeError> {
        let raw = serde_json::from_slice(bytes).map_err(|source| DecodeError::json("", source))?;
        let (version, chain_index) = validate_structure(&raw)?;
        Ok(Self {
            raw,
            version,
            chain_index,
        })
    }

    /// Borrows the complete original JSON values for this document's lifetime.
    /// Typed `f32` projections never replace these numbers. Shared access keeps
    /// the structural invariants established by parsing intact.
    pub fn raw(&self) -> &Value {
        &self.raw
    }

    /// Encodes the complete document as owned JSON bytes, preserving values and
    /// unknown fields rather than whitespace, key order, or number text.
    /// Returns the original [`serde_json::Error`] if encoding fails.
    pub fn to_json(&self) -> Result<Vec<u8>, serde_json::Error> {
        serde_json::to_vec(&self.raw)
    }

    /// Returns the serialized `Version` value, not a product release number.
    pub fn version(&self) -> u64 {
        self.version
    }

    /// Returns the stored signed cursor without clamping it to the entry count.
    pub fn chain_index(&self) -> i32 {
        self.chain_index
    }

    /// Borrows all commands in stored order, including entries beyond the cursor.
    /// The iterator and payloads borrow `self`; no command list is allocated.
    pub fn commands(&self) -> impl Iterator<Item = LocatedHistoryCommand<'_>> + Clone {
        self.history_entries().flat_map(|entry| entry.commands())
    }

    /// Borrows every entry and its sidecars in stored order, including empty
    /// entries and the cursor tail. The iterator's views borrow `self`.
    pub fn history_entries(
        &self,
    ) -> impl DoubleEndedIterator<Item = HistoryEntryView<'_>> + ExactSizeIterator + Clone {
        self.entries().iter().enumerate().map(|(entry_index, raw)| {
            let edits = raw["edits"]
                .as_array()
                .expect("parse validated entry.edits as an array");
            HistoryEntryView {
                entry_index,
                raw,
                decorators: raw.get("decorators"),
                stairs: raw.get("stairs"),
                edits,
            }
        })
    }

    /// Borrows all snapshot references in stored order, retaining duplicates.
    /// The iterator's views borrow `self`; the caller owns external asset loading
    /// and validation. No snapshot contents or candidate lists are allocated.
    pub fn snapshot_references(
        &self,
    ) -> impl DoubleEndedIterator<Item = HistorySnapshotReference<'_>> + ExactSizeIterator + Clone
    {
        self.snapshots().iter().map(|raw| HistorySnapshotReference {
            chain_index: validated_i32(&raw["chain_index"]),
            snapshot_id: raw["snapshot_id"]
                .as_u64()
                .expect("parse validated snapshot_id as u64"),
            raw,
        })
    }

    /// Selects the nearest snapshot accepted by the caller's resolution predicate.
    /// Any `i32` cursor is accepted. The predicate is called once per reference
    /// in stored order, including duplicate IDs; JSON cannot reveal resolution.
    /// Equal distances choose the greater chain index, retaining the first stored
    /// reference at duplicate indices. Returns `None` when no reference is
    /// accepted, otherwise the result borrows `self` without copying its JSON.
    pub fn closest_resolved_snapshot(
        &self,
        cursor: i32,
        mut is_resolved: impl FnMut(u64) -> bool,
    ) -> Option<HistorySnapshotReference<'_>> {
        let mut closest: Option<(i64, HistorySnapshotReference<'_>)> = None;
        for candidate in self.snapshot_references() {
            if !is_resolved(candidate.snapshot_id) {
                continue;
            }
            // Widen before subtraction so the full signed cursor range is safe.
            let distance = (i64::from(candidate.chain_index) - i64::from(cursor)).abs();
            if closest.as_ref().is_none_or(|(best_distance, best)| {
                distance < *best_distance
                    || (distance == *best_distance && candidate.chain_index > best.chain_index)
            }) {
                closest = Some((distance, candidate));
            }
        }
        closest.map(|(_, snapshot)| snapshot)
    }

    /// Borrows the first stored reference exactly at the serialized cursor.
    /// Returns `None` if no reference has that index, including an empty list.
    pub fn current_snapshot_reference(&self) -> Option<HistorySnapshotReference<'_>> {
        self.snapshot_references()
            .find(|snapshot| snapshot.chain_index == self.chain_index)
    }

    /// Borrows optional `history_start` metadata without interpreting its value.
    /// A missing field is `None`; an explicit JSON `null` remains `Some`.
    pub fn history_start(&self) -> Option<&Value> {
        self.raw["History"].get("history_start")
    }

    /// Lazily decodes supported roof commands in stored order, including the tail.
    /// The iterator borrows `self`, but each returned command and error is owned.
    /// Unknown variants are skipped and remain visible through [`Self::commands`],
    /// [`Self::raw`], and [`Self::summary`]. Numeric ranges are not clamped.
    ///
    /// A malformed known variant yields a [`DecodeError`] at its `/Roof` payload
    /// only when consumed. Iteration can continue after that error; collecting
    /// into `Result<Vec<_>, _>` instead stops at the first error.
    pub fn roof_commands(
        &self,
    ) -> impl Iterator<Item = Result<LocatedRoofCommand, DecodeError>> + '_ {
        self.commands().filter_map(|located| {
            if located.kind != HistoryCommandKind::Roof {
                return None;
            }
            let (tag, _) = validated_singleton(located.payload);
            if !RoofCommand::supports_tag(tag) {
                return None;
            }
            Some(
                RoofCommand::deserialize(located.payload)
                    .map(|command| LocatedRoofCommand {
                        entry_index: located.entry_index,
                        edit_index: located.edit_index,
                        command,
                    })
                    .map_err(|source| {
                        DecodeError::json(
                            format!(
                                "/History/edit_chain/{}/edits/{}/Roof",
                                located.entry_index, located.edit_index
                            ),
                            source,
                        )
                    }),
            )
        })
    }

    fn entries(&self) -> &[Value] {
        self.raw["History"]["edit_chain"]
            .as_array()
            .expect("parse validated immutable edit_chain as an array")
    }

    fn snapshots(&self) -> &[Value] {
        self.raw["History"]["snapshots"]
            .as_array()
            .expect("parse validated immutable snapshots as an array")
    }
}

fn validate_structure(raw: &Value) -> Result<(u64, i32), DecodeError> {
    raw.as_object()
        .ok_or_else(|| DecodeError::expected("", "object"))?;
    let version = raw
        .get("Version")
        .and_then(Value::as_u64)
        .ok_or_else(|| DecodeError::expected("/Version", "u64"))?;
    let history = raw
        .get("History")
        .and_then(Value::as_object)
        .ok_or_else(|| DecodeError::expected("/History", "object"))?;
    let chain_index = history
        .get("chain_index")
        .and_then(read_i32)
        .ok_or_else(|| DecodeError::expected("/History/chain_index", "i32"))?;
    let entries = history
        .get("edit_chain")
        .and_then(Value::as_array)
        .ok_or_else(|| DecodeError::expected("/History/edit_chain", "array"))?;
    for (entry_index, entry) in entries.iter().enumerate() {
        let edits = entry
            .get("edits")
            .and_then(Value::as_array)
            .ok_or_else(|| {
                DecodeError::expected(format!("/History/edit_chain/{entry_index}/edits"), "array")
            })?;
        for (edit_index, edit) in edits.iter().enumerate() {
            let (tag, payload) = check_singleton(edit).map_err(|expected| {
                DecodeError::expected(
                    format!("/History/edit_chain/{entry_index}/edits/{edit_index}"),
                    expected,
                )
            })?;
            if tag == "Roof" {
                check_singleton(payload).map_err(|expected| {
                    DecodeError::expected(
                        format!("/History/edit_chain/{entry_index}/edits/{edit_index}/Roof"),
                        expected,
                    )
                })?;
            }
        }
    }
    let snapshots = history
        .get("snapshots")
        .and_then(Value::as_array)
        .ok_or_else(|| DecodeError::expected("/History/snapshots", "array"))?;
    for (index, snapshot) in snapshots.iter().enumerate() {
        snapshot.as_object().ok_or_else(|| {
            DecodeError::expected(format!("/History/snapshots/{index}"), "object")
        })?;
        snapshot
            .get("chain_index")
            .and_then(read_i32)
            .ok_or_else(|| {
                DecodeError::expected(format!("/History/snapshots/{index}/chain_index"), "i32")
            })?;
        snapshot
            .get("snapshot_id")
            .and_then(Value::as_u64)
            .ok_or_else(|| {
                DecodeError::expected(format!("/History/snapshots/{index}/snapshot_id"), "u64")
            })?;
    }
    Ok((version, chain_index))
}

fn read_i32(value: &Value) -> Option<i32> {
    value.as_i64().and_then(|number| i32::try_from(number).ok())
}

fn validated_i32(value: &Value) -> i32 {
    read_i32(value).expect("parse validated the snapshot chain_index as i32")
}

fn check_singleton(value: &Value) -> Result<(&str, &Value), &'static str> {
    let object = value.as_object().ok_or("object")?;
    if object.len() != 1 {
        return Err("one enum tag");
    }
    let (tag, payload) = object
        .iter()
        .next()
        .expect("object length was checked as one");
    Ok((tag, payload))
}

/// Reads an outer or roof command already checked by `validate_structure`.
/// These values come only from the document's private, immutable JSON tree.
pub(crate) fn validated_singleton(value: &Value) -> (&str, &Value) {
    let (tag, payload) = value
        .as_object()
        .expect("parse validated the command as an object")
        .iter()
        .next()
        .expect("parse validated exactly one command tag");
    (tag, payload)
}
