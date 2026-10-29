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
    /// Active cursor plane height in the canonical editing owner. B ui_draw
    /// writes the original drag distance after successful creation, before
    /// reading SSBOs/deriving stroke height; release clears it, even if empty.
    /// This transient state is not part of the saved stroke map.
    pub cursor_height_override: Option<f32>,
    pub strokes: HashMap<StrokeId, TerrainStroke>,
    /// 原TerrainEditingData+38是usize/u64；超过发号上限的存档值仍原样保留。
    pub next_index: u64,
    /// Runtime rasterization state of this stroke owner. B startup +0x40 is
    /// false; W3 marks it true only after all four passes are constructed.
    /// RecalculateHeightMapCmd events trigger subsequent updates, so edits do
    /// not independently reset this flag or allocate a mirror stroke map.
    pub rasterized: bool,
}
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum StrokeError {
    IdCollision(StrokeId),
    MissingLastStroke,
}
impl TerrainEditingData {
    pub fn new_stroke(&mut self, point: [f32; 3]) -> Result<Option<StrokeId>, StrokeError> {
        if self.next_index >= u64::from(ID_COUNTER_LIMIT) {
            return Ok(None);
        }
        // 原924a20先限制到0x3ffffe以内，随后ESI写32位ID；此处窄化已有原范围保证。
        let id = StrokeId(self.next_index as u32);
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
        if let Some(old) = old {
            // 原924ad1先drop被替换旧条目，再panic；计数和新条目不能回滚或被UI吞掉。
            drop(old);
            panic!("terrain stroke ID collision after counter increment: {}", id.0);
        }
        Ok(Some(id))
    }
    /// 原924906先检查完整64位计数非零，随后DECL ESI仅对低32位减一。
    /// 这是原查询合同，不能对高位存档计数新增try_from拒绝或先做64位减法后断言范围。
    pub fn last_stroke_id(&self) -> StrokeId {
        assert_ne!(self.next_index, 0, "original last terrain stroke counter assertion");
        StrokeId((self.next_index as u32).wrapping_sub(1))
    }
    pub fn get_last_mut_stroke(&mut self) -> Result<&mut TerrainStroke, StrokeError> {
        let id = self.last_stroke_id();
        self.strokes
            .get_mut(&id)
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
    fn native_u64_counter_boundary_and_last_id_projection() {
        for (counter, expected) in [(0x3ffffeu64, Some(StrokeId(0x3ffffe))),
            (0x3fffff, None), (u64::from(u32::MAX) + 1, None), (u64::MAX, None)] {
            let mut data = TerrainEditingData { next_index: counter, ..Default::default() };
            assert_eq!(data.new_stroke([1.0, 2.0, 3.0]).unwrap(), expected);
            assert_eq!(data.next_index, counter + u64::from(expected.is_some()));
        }
        let mut data = TerrainEditingData::default();
        data.new_stroke([0.0; 3]).unwrap();
        data.next_index = u64::from(u32::MAX) + 2;
        assert_eq!(data.last_stroke_id(), StrokeId(0));
        assert_eq!(data.get_last_mut_stroke().unwrap().index, StrokeId(0));
        data.next_index = 0;
        assert!(std::panic::catch_unwind(|| data.last_stroke_id()).is_err());
    }
    #[test]
    fn native_collision_panics_after_replacement_and_counter_increment() {
        let mut data = TerrainEditingData::default();
        data.new_stroke([0.0; 3]).unwrap();
        data.next_index = 0;
        let failure = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
            data.new_stroke([4.0, 5.0, 6.0])
        }));
        assert!(failure.is_err());
        assert_eq!(data.next_index, 1);
        assert_eq!(data.strokes[&StrokeId(0)].points, [[4.0, 5.0, 6.0]]);
        assert_eq!(data.strokes[&StrokeId(0)].height_ws, 2.5);
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
