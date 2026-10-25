//! B 0x141fe0750 classification uses height > threshold, equality is below.
//! The receiver carries continuous regions as well as crossing events.
/// Library-side output protocol. `above_sample` starts a region when none exists
/// and otherwise advances its last endpoint. The original concrete receiver
/// stores curve parameters; callers may supply their own region payload.
pub trait SplitReceiver {
    fn above_sample(&mut self, index: usize);
    fn below_to_above(&mut self, segment: usize, t: f32);
    fn above_to_below(&mut self, segment: usize, t: f32);
}

#[allow(clippy::neg_cmp_op_on_partial_ord)] // Native unordered comparisons follow the above path.
pub fn split_heightmap_into(heights: &[f32], threshold: f32, receiver: &mut impl SplitReceiver) {
    assert!(!heights.is_empty(), "original accesses first height");
    // Unordered native comparisons follow the above path, including NaN.
    if !(heights[0] <= threshold) {
        receiver.above_sample(0);
    }
    for (segment, pair) in heights.windows(2).enumerate() {
        let [a, b] = [pair[0], pair[1]];
        if a <= threshold {
            if !(b <= threshold) {
                receiver.below_to_above(segment, (threshold - a) / (b - a));
                receiver.above_sample(segment + 1);
            }
        } else if b <= threshold {
            receiver.above_to_below(segment, (threshold - a) / (b - a));
        } else {
            receiver.above_sample(segment + 1);
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct ThresholdRegion {
    pub start_u: f32,
    pub end_u: f32,
}

/// The B RowTrimmerSink parameter policy before row-specific quantization:
/// a new region initially ends at 1; above samples then advance that endpoint.
pub struct RegionReceiver<'a> {
    pub point_u: &'a [f32],
    pub regions: Vec<ThresholdRegion>,
}
impl<'a> RegionReceiver<'a> {
    pub fn new(point_u: &'a [f32]) -> Self {
        Self {
            point_u,
            regions: Vec::new(),
        }
    }
    fn crossing_u(&self, segment: usize, t: f32) -> f32 {
        assert!(
            (0.0..=1.0).contains(&t),
            "original receiver rejects invalid crossing"
        );
        // B 0x140c904d0 subtracts, multiplies, then adds to the first endpoint.
        let a = self.point_u[segment];
        a + (self.point_u[segment + 1] - a) * t
    }
}
impl SplitReceiver for RegionReceiver<'_> {
    fn above_sample(&mut self, index: usize) {
        let u = self.point_u[index];
        if let Some(last) = self.regions.last_mut() {
            last.end_u = u;
        } else {
            self.regions.push(ThresholdRegion {
                start_u: u,
                end_u: 1.0,
            });
        }
    }
    fn below_to_above(&mut self, segment: usize, t: f32) {
        let start_u = self.crossing_u(segment, t);
        self.regions.push(ThresholdRegion {
            start_u,
            end_u: 1.0,
        });
    }
    fn above_to_below(&mut self, segment: usize, t: f32) {
        let end_u = self.crossing_u(segment, t);
        self.regions
            .last_mut()
            .expect("original receiver requires an open region")
            .end_u = end_u;
    }
}

pub fn split_heightmap_regions(
    heights: &[f32],
    point_u: &[f32],
    threshold: f32,
) -> Vec<ThresholdRegion> {
    let mut receiver = RegionReceiver::new(point_u);
    split_heightmap_into(heights, threshold, &mut receiver);
    receiver.regions
}
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ThresholdCrossing {
    BelowToAbove { segment: usize, t: f32 },
    AboveToBelow { segment: usize, t: f32 },
}
pub fn split_heightmap_by_threshold(heights: &[f32], threshold: f32) -> Vec<ThresholdCrossing> {
    struct Events(Vec<ThresholdCrossing>);
    impl SplitReceiver for Events {
        fn above_sample(&mut self, _: usize) {}
        fn below_to_above(&mut self, segment: usize, t: f32) {
            self.0.push(ThresholdCrossing::BelowToAbove { segment, t });
        }
        fn above_to_below(&mut self, segment: usize, t: f32) {
            self.0.push(ThresholdCrossing::AboveToBelow { segment, t });
        }
    }
    let mut receiver = Events(Vec::new());
    split_heightmap_into(heights, threshold, &mut receiver);
    receiver.0
}

#[cfg(test)]
mod completion_oracle {
    include!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../../../../evidence/foundation-completion-2026-10-13/wall-heightmap/heightmap_tests.rs"
    ));
}
