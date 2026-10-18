//! Candidate addendum to utils::camera. Original B only, not production.
//! TaaJitterSet::default @140c97500; map/fold @14217f990.
use bevy_ecs::resource::Resource;
use glam::Vec2;

#[derive(Resource, Clone, Debug)]
pub struct TaaJitterSet(pub Vec<Vec2>);

impl Default for TaaJitterSet {
    fn default() -> Self {
        // Original range bytes are {start:1,end:128,exhausted:false}.
        Self(
            (1..=128)
                .map(|n| {
                    fn radical_inverse(mut index: u32, base: u32) -> f32 {
                        let reciprocal = 1.0 / base as f32;
                        let mut fraction = reciprocal;
                        let mut value = 0.0;
                        loop {
                            value += (index % base) as f32 * fraction;
                            index = (index as f32 * reciprocal) as u32;
                            fraction *= reciprocal;
                            if index == 0 {
                                break;
                            }
                        }
                        value - 0.5
                    }
                    Vec2::new(radical_inverse(n, 2), radical_inverse(n, 3))
                })
                .collect(),
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn original_range_is_inclusive_one_through_128() {
        let jitter = TaaJitterSet::default();
        assert_eq!(jitter.0.len(), 128);
        assert_eq!(jitter.0[0].x, 0.0);
        assert_eq!(jitter.0[1].x, -0.25);
        assert_eq!(jitter.0[2].x, 0.25);
        assert_eq!(jitter.0[127].x, -0.49609375);
    }
}
