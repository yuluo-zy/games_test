use serde::{Deserialize, Serialize};

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(transparent)]
pub struct StrokeId(pub u32);

/// Persisted B game schema, not its native niche/Vec memory layout.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub enum TerrainEdit {
    NewTerrainStroke {
        index: StrokeId,
        height_ws: f32,
        stroke_pts: Vec<[f32; 3]>,
    },
    EditTerrainStroke {
        index: StrokeId,
        prev_height_ws: f32,
        new_height_ws: f32,
        prev_falloff: f32,
        new_falloff: f32,
    },
    MoveTerrainStroke {
        id: StrokeId,
        original_stroke_pts: Vec<[f32; 3]>,
        move_vec: [f32; 2],
    },
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn preserves_external_variant_and_field_names() {
        let text = r#"{"EditTerrainStroke":{"index":4,"prev_height_ws":2.5,"new_height_ws":4.0,"prev_falloff":0.4,"new_falloff":0.8}}"#;
        let edit: TerrainEdit = serde_json::from_str(text).unwrap();
        let encoded = serde_json::to_value(&edit).unwrap();
        let fields = encoded["EditTerrainStroke"].as_object().unwrap();
        assert_eq!(
            fields.keys().cloned().collect::<Vec<_>>(),
            [
                "index",
                "new_falloff",
                "new_height_ws",
                "prev_falloff",
                "prev_height_ws"
            ]
        );
        assert_eq!(fields["index"], 4);
        assert_eq!(
            serde_json::from_value::<TerrainEdit>(encoded).unwrap(),
            edit
        );
    }
}
