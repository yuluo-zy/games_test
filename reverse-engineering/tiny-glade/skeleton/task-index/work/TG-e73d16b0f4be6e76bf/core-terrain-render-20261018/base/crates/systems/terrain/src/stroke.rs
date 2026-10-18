use crate::schema::StrokeId;
use glam::Vec3;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

pub const MAX_POINTS: usize = 1024;
pub const ID_COUNTER_LIMIT: u32 = 0x3fffff;

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct TerrainStroke {
    pub points: Vec<[f32; 3]>,
    pub falloff: f32,
    pub index: StrokeId,
    pub height_ws: f32,
}
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PointChange {
    Added,
    Morphed,
    Unchanged,
}

// Game's explicit y²+x² then +z² order, not an alternate SIMD dot sum.
fn distance(a: [f32; 3], b: [f32; 3]) -> f32 {
    let d = Vec3::from_array(a) - Vec3::from_array(b);
    (d.z * d.z + (d.y * d.y + d.x * d.x)).sqrt()
}
impl TerrainStroke {
    // Original unordered comparison selects append; a >= rewrite changes NaN.
    #[allow(clippy::neg_cmp_op_on_partial_ord)]
    pub fn insert_or_morph_last_point(&mut self, point: [f32; 3], spacing: f32) -> PointChange {
        match self.points.len() {
            0 => {
                self.points.push(point);
                PointChange::Added
            }
            1 => {
                if distance(point, self.points[0]) > f32::from_bits(0x3a83126f) {
                    self.points.push(point);
                    PointChange::Added
                } else {
                    PointChange::Unchanged
                }
            }
            n => {
                // UCOMISS/JBE also chooses append for unordered inputs.
                if !(spacing > distance(self.points[n - 2], self.points[n - 1])) {
                    if n < MAX_POINTS {
                        self.points.push(point);
                        PointChange::Added
                    } else {
                        PointChange::Unchanged
                    }
                } else {
                    self.points[n - 1] = point;
                    PointChange::Morphed
                }
            }
        }
    }
}
#[derive(Clone, Debug, Default)]
pub struct TerrainEditingData {
    pub strokes: HashMap<StrokeId, TerrainStroke>,
    pub next_index: u32,
}
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum StrokeError {
    IdCollision(StrokeId),
    MissingLastStroke,
}
impl TerrainEditingData {
    pub fn new_stroke(&mut self, point: [f32; 3]) -> Result<Option<StrokeId>, StrokeError> {
        if self.next_index >= ID_COUNTER_LIMIT {
            return Ok(None);
        }
        let id = StrokeId(self.next_index);
        self.next_index += 1;
        let old = self.strokes.insert(
            id,
            TerrainStroke {
                points: vec![point],
                falloff: 0.4,
                index: id,
                height_ws: 2.5,
            },
        );
        if old.is_some() {
            return Err(StrokeError::IdCollision(id));
        }
        Ok(Some(id))
    }
    pub fn get_last_mut_stroke(&mut self) -> Result<&mut TerrainStroke, StrokeError> {
        let id = self
            .next_index
            .checked_sub(1)
            .ok_or(StrokeError::MissingLastStroke)?;
        self.strokes
            .get_mut(&StrokeId(id))
            .ok_or(StrokeError::MissingLastStroke)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn returned_id_is_counter_before_increment() {
        let mut data = TerrainEditingData::default();
        assert_eq!(data.new_stroke([0.0; 3]), Ok(Some(StrokeId(0))));
        assert_eq!(data.next_index, 1);
        assert_eq!(data.get_last_mut_stroke().unwrap().index, StrokeId(0));
    }
    #[test]
    fn morph_decision_uses_existing_segment() {
        let mut stroke = TerrainStroke {
            points: vec![[0.0; 3], [0.1, 0.0, 0.0]],
            falloff: 0.4,
            index: StrokeId(0),
            height_ws: 2.5,
        };
        assert_eq!(
            stroke.insert_or_morph_last_point([50.0, 0.0, 0.0], 1.0),
            PointChange::Morphed
        );
        assert_eq!(stroke.points.len(), 2);
    }
}
