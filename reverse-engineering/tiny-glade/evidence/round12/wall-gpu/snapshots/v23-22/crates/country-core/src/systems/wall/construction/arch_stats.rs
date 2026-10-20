//! Original760480..761142 /7629b1..2a49 material classification and counters.
//! This observes pre-profile materialized segments and raw mask length.
use super::structures::MaterializedWallSegment;
#[derive(Clone, Debug, Default)]
pub struct WallArchMaterialStats {
    pub total_segments: u64,
    pub arches_length: f32,
    pub gates_length: f32,
    pub sewer_length: f32,
    pub gates_count: u64,
    pub sewer_count: u64,
}
impl WallArchMaterialStats {
    pub fn observe_mask(
        &mut self,
        length: f32,
        segments: &[MaterializedWallSegment],
        has_roof: bool,
    ) {
        self.total_segments += segments.len() as u64;
        if !has_roof || segments.len() != 1 {
            self.arches_length += length;
            return;
        }
        let points = &segments[0].positions;
        let mut negative = 0.0f32;
        for p in points {
            if p[1] < 0.0 {
                negative += 1.0;
            }
        }
        // Original UCOMISS/JBE: unordered ratio takes the gate side.
        if negative / points.len() as f32 > f32::from_bits(0x3f000000) {
            self.sewer_length += length;
            self.sewer_count += 1;
        } else {
            self.gates_length += length;
            self.gates_count += 1;
        }
    }
    pub fn arches_segments_count(&self) -> u32 {
        // The native subtracts gate count only; sewer remains included.
        self.total_segments.wrapping_sub(self.gates_count) as u32
    }
}
