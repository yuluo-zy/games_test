//! Owned Serde projections of roof command payloads.
//!
//! Field and variant names are part of the history JSON format. Required fields
//! use Serde's normal type checks; these types do not clamp numbers, supply
//! migration defaults, or modify the source document. Extensions retain fields
//! outside the typed projection, and owned values can outlive the document.
use serde::{Deserialize, Serialize};
use serde_json::{Map, Value};

/// The serialized axis choice for a roof ridge, using the roof's width or length.
#[derive(Clone, Debug, Deserialize, Serialize, PartialEq, Eq)]
pub enum RidgeDirection {
    Width,
    Length,
}

/// Roof-shape parameters carried by creation and deletion records.
///
/// The `_01` suffix is retained from the JSON field names; it does not cause
/// deserialization to enforce a numeric range.
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct RoofParameters {
    pub height_01: f32,
    pub profile_01: f32,
    pub eave_length_01: f32,
    pub ridge_length_01: f32,
    pub ridge_dir: RidgeDirection,
    pub tip_offset_01: [f32; 2],
    /// Additional parameter properties retained by the typed projection.
    #[serde(flatten)]
    pub extensions: Map<String, Value>,
}

/// Roof state carried by a creation or deletion command.
/// The optional style remains JSON because this API does not interpret it.
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct RoofPresence {
    /// Serialized wall reference; the caller interprets this value.
    pub wall_index: u64,
    pub color_id: u64,
    pub params: RoofParameters,
    /// Omitted or JSON `null` styles decode as `None` and serialize as omitted.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub flat_roof_style: Option<Value>,
    /// Additional roof-state properties retained by the typed projection.
    #[serde(flatten)]
    pub extensions: Map<String, Value>,
}

/// Before/after appearance values for a roof edit.
/// The paired values describe a recorded change; this type does not apply it.
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct RoofVisualEdit {
    /// Serialized wall reference; the caller interprets this value.
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
    /// Additional edit properties retained by the typed projection.
    #[serde(flatten)]
    pub extensions: Map<String, Value>,
}

/// The ridge-direction values before and after an edit.
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct RoofDirectionEdit {
    /// Serialized wall reference; the caller interprets this value.
    pub wall_index: u64,
    pub from: RidgeDirection,
    pub to: RidgeDirection,
    /// Additional direction-edit properties retained by the typed projection.
    #[serde(flatten)]
    pub extensions: Map<String, Value>,
}

/// An owned projection of a supported, externally tagged roof command.
///
/// Direct Serde decoding rejects unknown variants or malformed required fields.
/// [`crate::HistoryDocument::roof_commands`] skips unknown roof tags and returns
/// decoding failures per item instead. Neither path executes the command.
#[derive(Clone, Debug, Deserialize, Serialize)]
pub enum RoofCommand {
    /// A creation record carrying the roof state.
    CreateRoof(RoofPresence),
    /// A deletion record retaining the removed roof state.
    DeleteRoof(RoofPresence),
    /// A recorded change to appearance parameters.
    VisualEdit(RoofVisualEdit),
    /// A recorded change between width and length ridge directions.
    SwitchRoofRidgeDir(RoofDirectionEdit),
}

/// A decoded roof command and its zero-based location in the edit chain.
/// The command owns its fields and does not borrow the source document.
#[derive(Clone, Debug)]
pub struct LocatedRoofCommand {
    /// Index within `History.edit_chain`.
    pub entry_index: usize,
    /// Index within that entry's `edits` list.
    pub edit_index: usize,
    pub command: RoofCommand,
}

impl RoofCommand {
    pub(crate) fn supports_tag(tag: &str) -> bool {
        matches!(
            tag,
            "CreateRoof" | "DeleteRoof" | "VisualEdit" | "SwitchRoofRidgeDir"
        )
    }
}
