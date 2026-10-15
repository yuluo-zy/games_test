//! B 0x141750c90 + 0x141751c90: JSON envelope and section/edit offsets.
//! Domain values are serialized by their owning modules; this layer does not
//! substitute fabricated wall/roof/snapshot payloads for those values.
pub use crate::resources::save_file::SaveFileSection;
use hashbrown::HashMap;
use serde::Serialize;
use std::ops::Range;

#[derive(Clone, Debug, Default)]
pub struct SaveFileStructure {
    pub section_ranges: HashMap<SaveFileSection, Range<u64>>,
    pub history_edit_offsets: Vec<u64>,
}

/// Compact serialized JSON, validated at construction; consumed unchanged by
/// section replacement and history append, matching their original &str inputs.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct SerializedJson(String);
impl SerializedJson {
    pub fn new<T: Serialize + ?Sized>(value: &T) -> serde_json::Result<Self> {
        serde_json::to_string(value).map(Self)
    }
    pub fn as_str(&self) -> &str {
        &self.0
    }
    pub fn as_bytes(&self) -> &[u8] {
        self.0.as_bytes()
    }
}

/// Each field is required. There are seven root sections and three History
/// sections; no absent-domain value is silently replaced with null or zero.
pub struct GameSaveSections {
    pub values: [SerializedJson; 10],
}

pub fn write_json_sections(
    output: &mut Vec<u8>,
    sections: impl IntoIterator<Item = (SaveFileSection, SerializedJson)>,
    structure: &mut SaveFileStructure,
) {
    for (section, value) in sections {
        output.push(b'"');
        output.extend_from_slice(section.json_key().as_bytes());
        output.extend_from_slice(b"\":");
        let start = output.len() as u64;
        output.extend_from_slice(value.as_bytes());
        output.push(b',');
        output.resize(output.len() + section.padding(), b' ');
        output.push(b'\n');
        structure
            .section_ranges
            .insert(section, start..output.len() as u64);
    }
}

pub fn save_game_to_memory(
    sections: GameSaveSections,
    edits: &[SerializedJson],
) -> (Vec<u8>, SaveFileStructure) {
    let mut output = vec![b'{'];
    let mut structure = SaveFileStructure::default();
    let mut sections = SaveFileSection::ALL.into_iter().zip(sections.values);
    write_json_sections(&mut output, sections.by_ref().take(7), &mut structure);
    output.extend_from_slice(b"\"History\":{\n");
    write_json_sections(&mut output, sections, &mut structure);
    output.extend_from_slice(b"\"edit_chain\":[\n");
    for (index, edit) in edits.iter().enumerate() {
        structure.history_edit_offsets.push(output.len() as u64);
        if index != 0 {
            output.extend_from_slice(b",\n");
        }
        output.extend_from_slice(edit.as_bytes());
    }
    output.extend_from_slice(b"\n]}}");
    (output, structure)
}
