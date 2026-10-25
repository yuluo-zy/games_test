//! B freehand input: raw Curve2, integer point weights and an incremental RDP
//! cache. Original source/PDB binds 0x140ad3070 / 0x140ad3f50 / 0x140ad41a0.
//! The public wall metadata below is the existing host adapter; this type does
//! not claim the original ABI or the enclosing InProgressWall layout.

use bevy_ecs::resource::Resource;
use glam::Vec2;
use utils::{curve::Curve2 as InputCurve, rdp::Rdp};

use crate::resources::{roofs::geometry::surface::Curve2, walls::WallId};

#[derive(Resource, Debug, Default)]
pub struct InProgressCurve {
    pub wall: Option<WallId>,
    pub raw_input_points: Vec<Vec2>,
    pub current_wall: WallId,
    pub flags: u8,
    pub final_curve: Option<Curve2>,
    /// Live draw metadata; shares this curve and wall ID with existing snapping.
    pub drawing: Option<crate::resources::walls::in_progress_wall::WallDrawingMetadata>,
    point_weights: Vec<usize>,
    raw_curve: Option<InputCurve>,
    simplified: Option<Rdp<Vec2>>,
}

impl InProgressCurve {
    /// Empty host adapter. The native constructor takes an existing valid curve;
    /// use `from_curve` when continuing or starting an original wall stroke.
    pub fn new() -> Self {
        Self::default()
    }

    /// B 0x140ad3070: freeze the old prefix of a continued wall while leaving
    /// its final at-most-two world units available for weighted smoothing.
    pub fn from_curve(curve: InputCurve) -> Self {
        let (raw_curve, point_weights, simplified) = if curve.length <= 2.1 {
            let weights = vec![1; curve.points.len()];
            let simplified = Rdp::new_original_2d(&curve.points);
            (curve, weights, simplified)
        } else {
            let count = curve.points.len();
            assert!(count >= 2, "original curve endpoint assertion");
            let delta = curve.points[count - 1] - curve.points[count - 2];
            let last_length = (delta.y * delta.y + delta.x * delta.x).sqrt();
            let tail_length = if last_length < 2.0 { last_length } else { 2.0 };
            let split = 1.0 - tail_length / curve.length;
            let prefix = curve.crashy_slice(0.0, split);
            let tail = curve
                .crashy_slice(split, 1.0)
                .crashy_resample_approx_pt_count(21);
            let mut weights = vec![1000; prefix.points.len()];
            weights.resize(prefix.points.len() + tail.points.len() - 1, 1);
            let mut points = prefix.points;
            points.extend_from_slice(&tail.points[1..]);
            let raw = InputCurve::crashy_new_from_points(points, false);
            let mut indices: Vec<_> = (0..curve.points.len() - 1).collect();
            indices.push(raw.points.len() - 1);
            let simplified = Rdp {
                indices,
                points: curve.points,
            };
            (raw, weights, simplified)
        };
        Self {
            raw_input_points: raw_curve.points.clone(),
            point_weights,
            raw_curve: Some(raw_curve),
            simplified: Some(simplified),
            ..Self::default()
        }
    }

    /// B 0x140ad3f50: append, rebuild raw arc-length metadata with duplicate
    /// rejection, then append weight 1 for each accepted new point. On failure
    /// the native raw point allocation keeps the appended points while its old
    /// arc-length metadata and weights remain. The caller must discard that
    /// failed stroke; a failed append is not an instruction to clear all input.
    pub fn try_add_raw_input_points(&mut self, points: &[Vec2]) -> bool {
        self.raw_input_points.extend_from_slice(points);
        let updated = InputCurve::try_new_from_points(self.raw_input_points.clone(), true);
        if let Some(raw) = &mut self.raw_curve {
            raw.points.clone_from(&self.raw_input_points);
        }
        let Ok(updated) = updated else {
            return false;
        };
        if self.raw_curve.is_none() {
            // Compatibility for existing callers building the empty host adapter
            // or assigning its public raw points before its first valid append.
            self.point_weights = vec![1; self.raw_input_points.len()];
        } else {
            self.point_weights
                .resize(self.point_weights.len() + points.len(), 1);
        }
        self.raw_curve = Some(updated);
        self.final_curve = None;
        true
    }

    /// B inline at draw_wall+0x9b9/+0xa5b: wrapping integer addition followed
    /// by the observed inclusive 1..=50 clamp. An empty weight vector is a no-op.
    pub fn add_weight_to_the_last_point(&mut self, additional_weight: usize) {
        if let Some(weight) = self.point_weights.last_mut() {
            *weight = weight.wrapping_add(additional_weight).clamp(1, 50);
        }
    }

    pub fn point_weights(&self) -> &[usize] {
        &self.point_weights
    }

    pub fn simplified_source_indices(&self) -> &[usize] {
        self.simplified.as_ref().map_or(&[], |rdp| &rdp.indices)
    }

    pub fn clear(&mut self) {
        *self = Self::default();
    }

    pub fn first_raw_input_point(&self) -> Vec2 {
        *self
            .raw_input_points
            .first()
            .expect("InProgressCurve has no raw input points")
    }

    pub fn last_raw_input_point(&self) -> Vec2 {
        *self
            .raw_input_points
            .last()
            .expect("InProgressCurve has no raw input points")
    }

    pub fn raw_input_curve_length_ws(&self) -> f32 {
        if let Some(curve) = &self.raw_curve {
            return curve.length;
        }
        self.raw_input_points.windows(2).fold(0.0, |length, pair| {
            let delta = pair[1] - pair[0];
            length + (delta.y * delta.y + delta.x * delta.x).sqrt()
        })
    }

    pub fn is_raw_input_curve_valid(&self) -> bool {
        self.raw_input_points.len() >= 2 && self.raw_input_curve_length_ws() > 0.0
    }

    /// Both observed draw_wall consumers request smoothing (R9B=1).
    pub fn compute_final_curve(&mut self) -> Option<Curve2> {
        self.compute_final_curve_with_smoothing(true)
    }

    /// B 0x140ad41a0: optionally apply 50 weighted smoothing passes, retaining
    /// old endpoints each pass. Recompute only the suffix after the last cached
    /// source index strictly less than raw_count-20; preserve the earlier RDP
    /// points, omit the duplicate seam, and offset new source indices.
    pub fn compute_final_curve_with_smoothing(&mut self, smooth: bool) -> Option<Curve2> {
        if self.raw_curve.is_none() {
            self.raw_curve =
                InputCurve::try_new_from_points(self.raw_input_points.clone(), false).ok();
            self.point_weights = vec![1; self.raw_input_points.len()];
        }
        let mut input = self.raw_curve.clone()?;
        input.points.clone_from(&self.raw_input_points);
        if smooth {
            input = input.smooth_weighted(50, &self.point_weights).ok()?;
        }
        let threshold = (self.raw_input_points.len() as i32).wrapping_sub(20);
        let seam = self.simplified.as_ref().and_then(|rdp| {
            rdp.indices
                .iter()
                .enumerate()
                .rev()
                .find(|(_, source)| (**source as i32) < threshold)
                .filter(|(_, source)| **source != 0)
                .map(|(index, &source)| (index, source))
        });
        let simplified = if let Some((index, source)) = seam {
            let suffix = Rdp::new_original_2d(&input.points[source..]);
            self.simplified
                .as_ref()
                .unwrap()
                .stitch_with(index, &suffix)
        } else {
            Rdp::new_original_2d(&input.points)
        };
        let curve = Curve2::try_new(
            simplified
                .points
                .iter()
                .map(|point| point.to_array())
                .collect(),
            false,
        )
        .ok();
        self.simplified = Some(simplified);
        self.final_curve = curve.clone();
        curve
    }
}

#[cfg(test)]
mod native_tests {
    use super::*;
    fn points(bits: &[u32]) -> Vec<Vec2> {
        bits.chunks_exact(2)
            .map(|p| Vec2::new(f32::from_bits(p[0]), f32::from_bits(p[1])))
            .collect()
    }
    fn point_bits(points: &[Vec2]) -> Vec<u32> {
        points
            .iter()
            .flat_map(|p| p.to_array().map(f32::to_bits))
            .collect()
    }
    fn check_curve(curve: &Curve2, points: &[u32], u: &[u32], length: u32) {
        assert_eq!(
            curve
                .points
                .iter()
                .flat_map(|p| p.map(f32::to_bits))
                .collect::<Vec<_>>(),
            points
        );
        assert_eq!(
            curve
                .points_u
                .iter()
                .map(|v| v.to_bits())
                .collect::<Vec<_>>(),
            u
        );
        assert_eq!(curve.length.to_bits(), length);
    }
    #[test]
    fn original_b_curve_chain_0() {
        let input = InputCurve::crashy_new_from_points(points(&[0, 0, 1065353216, 0]), false);
        let mut stroke = InProgressCurve::from_curve(input);
        assert_eq!(point_bits(&stroke.raw_input_points), &[0, 0, 1065353216, 0]);
        assert_eq!(stroke.point_weights(), &[1, 1]);
        assert_eq!(stroke.simplified_source_indices(), &[0, 1]);
        let first = stroke.compute_final_curve_with_smoothing(false).unwrap();
        check_curve(&first, &[0, 0, 1065353216, 0], &[0, 1065353216], 1065353216);
        assert_eq!(stroke.simplified_source_indices(), &[0, 1]);
        assert!(
            stroke.try_add_raw_input_points(&points(&[
                1067030938, 1041865114, 1068708659, 3175926989
            ]))
        );
        stroke.add_weight_to_the_last_point(4);
        let second = stroke.compute_final_curve_with_smoothing(false).unwrap();
        check_curve(
            &second,
            &[0, 0, 1067030938, 1041865114, 1068708659, 3175926989],
            &[0, 1062173098, 1065353216],
            1069481932,
        );
        assert_eq!(stroke.point_weights(), &[1, 1, 1, 5]);
        assert_eq!(stroke.simplified_source_indices(), &[0, 2, 3]);
    }
    #[test]
    fn original_b_curve_chain_1() {
        let input = InputCurve::crashy_new_from_points(points(&[0, 0, 1065353216, 0]), false);
        let mut stroke = InProgressCurve::from_curve(input);
        assert_eq!(point_bits(&stroke.raw_input_points), &[0, 0, 1065353216, 0]);
        assert_eq!(stroke.point_weights(), &[1, 1]);
        assert_eq!(stroke.simplified_source_indices(), &[0, 1]);
        let first = stroke.compute_final_curve_with_smoothing(true).unwrap();
        check_curve(&first, &[0, 0, 1065353216, 0], &[0, 1065353216], 1065353216);
        assert_eq!(stroke.simplified_source_indices(), &[0, 1]);
        assert!(
            stroke.try_add_raw_input_points(&points(&[
                1067030938, 1041865114, 1068708659, 3175926989
            ]))
        );
        stroke.add_weight_to_the_last_point(4);
        let second = stroke.compute_final_curve_with_smoothing(true).unwrap();
        check_curve(
            &second,
            &[0, 0, 1068708659, 3175926989],
            &[0, 1065353216],
            1068716147,
        );
        assert_eq!(stroke.point_weights(), &[1, 1, 1, 5]);
        assert_eq!(stroke.simplified_source_indices(), &[0, 3]);
    }
    #[test]
    fn original_b_curve_chain_2() {
        let input = InputCurve::crashy_new_from_points(
            points(&[0, 0, 1065353216, 1065353216, 1073741824, 0]),
            false,
        );
        let mut stroke = InProgressCurve::from_curve(input);
        assert_eq!(
            point_bits(&stroke.raw_input_points),
            &[
                0, 0, 1065353216, 1065353216, 1065772646, 1064514355, 1066192077, 1063675494,
                1066611508, 1062836634, 1067030938, 1061997773, 1067450368, 1061158912, 1067869798,
                1060320051, 1068289229, 1059481190, 1068708660, 1058642329, 1069128090, 1057803468,
                1069547520, 1056964606, 1069966951, 1055286884, 1070386382, 1053609162, 1070805812,
                1051931440, 1071225242, 1050253718, 1071644673, 1048575992, 1072064104, 1045220548,
                1072483534, 1041865104, 1072902964, 1036831928, 1073322395, 1028443296, 1073741824,
                0
            ]
        );
        assert_eq!(
            stroke.point_weights(),
            &[
                1000, 1000, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1
            ]
        );
        assert_eq!(stroke.simplified_source_indices(), &[0, 1, 21]);
        let first = stroke.compute_final_curve_with_smoothing(false).unwrap();
        check_curve(
            &first,
            &[0, 0, 1065353216, 1065353216, 1073741824, 0],
            &[0, 1056964608, 1065353216],
            1077216499,
        );
        assert_eq!(stroke.simplified_source_indices(), &[0, 1, 21]);
        assert!(
            stroke.try_add_raw_input_points(&points(&[
                1074580685, 1041865114, 1075419546, 3175926989
            ]))
        );
        stroke.add_weight_to_the_last_point(4);
        let second = stroke.compute_final_curve_with_smoothing(false).unwrap();
        check_curve(
            &second,
            &[
                0, 0, 1065353216, 1065353216, 1073741824, 0, 1074580685, 1041865114, 1075419546,
                3175926989,
            ],
            &[0, 1054305012, 1062693620, 1063941453, 1065353216],
            1079451404,
        );
        assert_eq!(
            stroke.point_weights(),
            &[
                1000, 1000, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 5
            ]
        );
        assert_eq!(stroke.simplified_source_indices(), &[0, 1, 21, 22, 23]);
    }
    #[test]
    fn original_b_curve_chain_3() {
        let input = InputCurve::crashy_new_from_points(
            points(&[0, 0, 1065353216, 1065353216, 1073741824, 0]),
            false,
        );
        let mut stroke = InProgressCurve::from_curve(input);
        assert_eq!(
            point_bits(&stroke.raw_input_points),
            &[
                0, 0, 1065353216, 1065353216, 1065772646, 1064514355, 1066192077, 1063675494,
                1066611508, 1062836634, 1067030938, 1061997773, 1067450368, 1061158912, 1067869798,
                1060320051, 1068289229, 1059481190, 1068708660, 1058642329, 1069128090, 1057803468,
                1069547520, 1056964606, 1069966951, 1055286884, 1070386382, 1053609162, 1070805812,
                1051931440, 1071225242, 1050253718, 1071644673, 1048575992, 1072064104, 1045220548,
                1072483534, 1041865104, 1072902964, 1036831928, 1073322395, 1028443296, 1073741824,
                0
            ]
        );
        assert_eq!(
            stroke.point_weights(),
            &[
                1000, 1000, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1
            ]
        );
        assert_eq!(stroke.simplified_source_indices(), &[0, 1, 21]);
        let first = stroke.compute_final_curve_with_smoothing(true).unwrap();
        check_curve(
            &first,
            &[0, 0, 1065353216, 1065353216, 1073741824, 0],
            &[0, 1056964608, 1065353216],
            1077216499,
        );
        assert_eq!(stroke.simplified_source_indices(), &[0, 1, 21]);
        assert!(
            stroke.try_add_raw_input_points(&points(&[
                1074580685, 1041865114, 1075419546, 3175926989
            ]))
        );
        stroke.add_weight_to_the_last_point(4);
        let second = stroke.compute_final_curve_with_smoothing(true).unwrap();
        check_curve(
            &second,
            &[0, 0, 1065353216, 1065353216, 1075419546, 3175926989],
            &[0, 1055184209, 1065353216],
            1078624890,
        );
        assert_eq!(
            stroke.point_weights(),
            &[
                1000, 1000, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 5
            ]
        );
        assert_eq!(stroke.simplified_source_indices(), &[0, 1, 23]);
    }
    #[test]
    fn original_b_curve_chain_4() {
        let input = InputCurve::crashy_new_from_points(
            points(&[
                0, 0, 1065353216, 1045220557, 1073741824, 3192704205, 1077936128, 0,
            ]),
            false,
        );
        let mut stroke = InProgressCurve::from_curve(input);
        assert_eq!(
            point_bits(&stroke.raw_input_points),
            &[
                0, 0, 1065353216, 1045220557, 1073741822, 3192704201, 1073951538, 3192033119,
                1074161254, 3191362031, 1074370969, 3190690942, 1074580684, 3190019853, 1074790399,
                3189348764, 1075000114, 3188677675, 1075209830, 3188006586, 1075419546, 3186999955,
                1075629260, 3185657776, 1075838976, 3184315599, 1076048691, 3182973421, 1076258406,
                3181631243, 1076468122, 3180289064, 1076677837, 3178611341, 1076887552, 3175926986,
                1077097267, 3173242631, 1077306982, 3170222727, 1077516698, 3164854010, 1077726413,
                3156465383, 1077936128, 0
            ]
        );
        assert_eq!(
            stroke.point_weights(),
            &[
                1000, 1000, 1000, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1
            ]
        );
        assert_eq!(stroke.simplified_source_indices(), &[0, 1, 2, 22]);
        let first = stroke.compute_final_curve_with_smoothing(false).unwrap();
        check_curve(
            &first,
            &[
                0, 0, 1065353216, 1045220557, 1073741824, 3192704205, 1077936128, 0,
            ],
            &[0, 1051166823, 1059863501, 1065353216],
            1078425354,
        );
        assert_eq!(stroke.simplified_source_indices(), &[0, 1, 2, 22]);
        assert!(
            stroke.try_add_raw_input_points(&points(&[
                1078774989, 1041865114, 1079613850, 3175926989
            ]))
        );
        stroke.add_weight_to_the_last_point(4);
        let second = stroke.compute_final_curve_with_smoothing(false).unwrap();
        check_curve(
            &second,
            &[
                0, 0, 1065353216, 1045220557, 1073741824, 3192704205, 1078774989, 1041865114,
                1079613850, 3175926989,
            ],
            &[0, 1049614929, 1058268063, 1064045851, 1065353216],
            1080577196,
        );
        assert_eq!(
            stroke.point_weights(),
            &[
                1000, 1000, 1000, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 5
            ]
        );
        assert_eq!(stroke.simplified_source_indices(), &[0, 1, 2, 23, 24]);
    }
    #[test]
    fn original_b_curve_chain_5() {
        let input = InputCurve::crashy_new_from_points(
            points(&[
                0, 0, 1065353216, 1045220557, 1073741824, 3192704205, 1077936128, 0,
            ]),
            false,
        );
        let mut stroke = InProgressCurve::from_curve(input);
        assert_eq!(
            point_bits(&stroke.raw_input_points),
            &[
                0, 0, 1065353216, 1045220557, 1073741822, 3192704201, 1073951538, 3192033119,
                1074161254, 3191362031, 1074370969, 3190690942, 1074580684, 3190019853, 1074790399,
                3189348764, 1075000114, 3188677675, 1075209830, 3188006586, 1075419546, 3186999955,
                1075629260, 3185657776, 1075838976, 3184315599, 1076048691, 3182973421, 1076258406,
                3181631243, 1076468122, 3180289064, 1076677837, 3178611341, 1076887552, 3175926986,
                1077097267, 3173242631, 1077306982, 3170222727, 1077516698, 3164854010, 1077726413,
                3156465383, 1077936128, 0
            ]
        );
        assert_eq!(
            stroke.point_weights(),
            &[
                1000, 1000, 1000, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1
            ]
        );
        assert_eq!(stroke.simplified_source_indices(), &[0, 1, 2, 22]);
        let first = stroke.compute_final_curve_with_smoothing(true).unwrap();
        check_curve(
            &first,
            &[
                0, 0, 1065353216, 1045220557, 1073741824, 3192704205, 1077936128, 0,
            ],
            &[0, 1051166823, 1059863501, 1065353216],
            1078425354,
        );
        assert_eq!(stroke.simplified_source_indices(), &[0, 1, 2, 22]);
        assert!(
            stroke.try_add_raw_input_points(&points(&[
                1078774989, 1041865114, 1079613850, 3175926989
            ]))
        );
        stroke.add_weight_to_the_last_point(4);
        let second = stroke.compute_final_curve_with_smoothing(true).unwrap();
        check_curve(
            &second,
            &[
                0, 0, 1065353216, 1045220557, 1073741824, 3192704205, 1079613850, 3175926989,
            ],
            &[0, 1049950704, 1058613259, 1065353216],
            1080053621,
        );
        assert_eq!(
            stroke.point_weights(),
            &[
                1000, 1000, 1000, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 5
            ]
        );
        assert_eq!(stroke.simplified_source_indices(), &[0, 1, 2, 24]);
    }
    #[test]
    fn original_b_curve_chain_6() {
        let input = InputCurve::crashy_new_from_points(
            points(&[
                0, 0, 1036831949, 1053609165, 1045220557, 3201092813, 1050253722, 1045220557,
                1053609165, 0,
            ]),
            false,
        );
        let mut stroke = InProgressCurve::from_curve(input);
        assert_eq!(
            point_bits(&stroke.raw_input_points),
            &[
                0, 0, 1036831949, 1053609165, 1045220557, 3201092813, 1050253722, 1045220557,
                1053609165, 0
            ]
        );
        assert_eq!(stroke.point_weights(), &[1, 1, 1, 1, 1]);
        assert_eq!(stroke.simplified_source_indices(), &[0, 1, 2, 3, 4]);
        let first = stroke.compute_final_curve_with_smoothing(false).unwrap();
        check_curve(
            &first,
            &[
                0, 0, 1036831949, 1053609165, 1045220557, 3201092813, 1050253722, 1045220557,
                1053609165, 0,
            ],
            &[0, 1045293435, 1058546472, 1063523591, 1065353216],
            1073953298,
        );
        assert_eq!(stroke.simplified_source_indices(), &[0, 1, 2, 3, 4]);
        assert!(
            stroke.try_add_raw_input_points(&points(&[
                1058642330, 1041865114, 1061997773, 3175926989
            ]))
        );
        stroke.add_weight_to_the_last_point(4);
        let second = stroke.compute_final_curve_with_smoothing(false).unwrap();
        check_curve(
            &second,
            &[
                0, 0, 1036831949, 1053609165, 1045220557, 3201092813, 1050253722, 1045220557,
                1053609165, 0, 1058642330, 1041865114, 1061997773, 3175926989,
            ],
            &[
                0, 1042509929, 1056015170, 1060440391, 1061892624, 1063516270, 1065353216,
            ],
            1076188202,
        );
        assert_eq!(stroke.point_weights(), &[1, 1, 1, 1, 1, 1, 5]);
        assert_eq!(stroke.simplified_source_indices(), &[0, 1, 2, 3, 4, 5, 6]);
    }
    #[test]
    fn original_b_curve_chain_7() {
        let input = InputCurve::crashy_new_from_points(
            points(&[
                0, 0, 1036831949, 1053609165, 1045220557, 3201092813, 1050253722, 1045220557,
                1053609165, 0,
            ]),
            false,
        );
        let mut stroke = InProgressCurve::from_curve(input);
        assert_eq!(
            point_bits(&stroke.raw_input_points),
            &[
                0, 0, 1036831949, 1053609165, 1045220557, 3201092813, 1050253722, 1045220557,
                1053609165, 0
            ]
        );
        assert_eq!(stroke.point_weights(), &[1, 1, 1, 1, 1]);
        assert_eq!(stroke.simplified_source_indices(), &[0, 1, 2, 3, 4]);
        let first = stroke.compute_final_curve_with_smoothing(true).unwrap();
        check_curve(&first, &[0, 0, 1053609165, 0], &[0, 1065353216], 1053609165);
        assert_eq!(stroke.simplified_source_indices(), &[0, 4]);
        assert!(
            stroke.try_add_raw_input_points(&points(&[
                1058642330, 1041865114, 1061997773, 3175926989
            ]))
        );
        stroke.add_weight_to_the_last_point(4);
        let second = stroke.compute_final_curve_with_smoothing(true).unwrap();
        check_curve(
            &second,
            &[0, 0, 1061997773, 3175926989],
            &[0, 1065353216],
            1062023962,
        );
        assert_eq!(stroke.point_weights(), &[1, 1, 1, 1, 1, 1, 5]);
        assert_eq!(stroke.simplified_source_indices(), &[0, 6]);
    }
    #[test]
    fn original_b_curve_chain_8() {
        let input = InputCurve::crashy_new_from_points(
            points(&[
                0, 0, 1036831949, 1044768599, 1045220557, 1050107254, 1050253722, 1048876742,
                1053609165, 1036898581, 1056964608, 3185018222, 1058642330, 3196444609,
            ]),
            false,
        );
        let mut stroke = InProgressCurve::from_curve(input);
        assert_eq!(
            point_bits(&stroke.raw_input_points),
            &[
                0, 0, 1036831949, 1044768599, 1045220557, 1050107254, 1050253722, 1048876742,
                1053609165, 1036898581, 1056964608, 3185018222, 1058642330, 3196444609
            ]
        );
        assert_eq!(stroke.point_weights(), &[1, 1, 1, 1, 1, 1, 1]);
        assert_eq!(stroke.simplified_source_indices(), &[0, 3, 6]);
        let first = stroke.compute_final_curve_with_smoothing(false).unwrap();
        check_curve(
            &first,
            &[0, 0, 1050253722, 1048876742, 1058642330, 3196444609],
            &[0, 1053525091, 1065353216],
            1065303232,
        );
        assert_eq!(stroke.simplified_source_indices(), &[0, 3, 6]);
        assert!(
            stroke.try_add_raw_input_points(&points(&[
                1061997773, 3185855441, 1065353216, 3198122331
            ]))
        );
        stroke.add_weight_to_the_last_point(4);
        let second = stroke.compute_final_curve_with_smoothing(false).unwrap();
        check_curve(
            &second,
            &[
                0, 0, 1045220557, 1050107254, 1058642330, 3196444609, 1061997773, 3185855441,
                1065353216, 3198122331,
            ],
            &[0, 1047001330, 1059679461, 1062341483, 1065353216],
            1070181770,
        );
        assert_eq!(stroke.point_weights(), &[1, 1, 1, 1, 1, 1, 1, 1, 5]);
        assert_eq!(stroke.simplified_source_indices(), &[0, 2, 6, 7, 8]);
    }
    #[test]
    fn original_b_curve_chain_9() {
        let input = InputCurve::crashy_new_from_points(
            points(&[
                0, 0, 1036831949, 1044768599, 1045220557, 1050107254, 1050253722, 1048876742,
                1053609165, 1036898581, 1056964608, 3185018222, 1058642330, 3196444609,
            ]),
            false,
        );
        let mut stroke = InProgressCurve::from_curve(input);
        assert_eq!(
            point_bits(&stroke.raw_input_points),
            &[
                0, 0, 1036831949, 1044768599, 1045220557, 1050107254, 1050253722, 1048876742,
                1053609165, 1036898581, 1056964608, 3185018222, 1058642330, 3196444609
            ]
        );
        assert_eq!(stroke.point_weights(), &[1, 1, 1, 1, 1, 1, 1]);
        assert_eq!(stroke.simplified_source_indices(), &[0, 3, 6]);
        let first = stroke.compute_final_curve_with_smoothing(true).unwrap();
        check_curve(
            &first,
            &[0, 0, 1058642330, 3196444609],
            &[0, 1065353216],
            1059556659,
        );
        assert_eq!(stroke.simplified_source_indices(), &[0, 6]);
        assert!(
            stroke.try_add_raw_input_points(&points(&[
                1061997773, 3185855441, 1065353216, 3198122331
            ]))
        );
        stroke.add_weight_to_the_last_point(4);
        let second = stroke.compute_final_curve_with_smoothing(true).unwrap();
        check_curve(
            &second,
            &[0, 0, 1065353216, 3198122331],
            &[0, 1065353216],
            1065750710,
        );
        assert_eq!(stroke.point_weights(), &[1, 1, 1, 1, 1, 1, 1, 1, 5]);
        assert_eq!(stroke.simplified_source_indices(), &[0, 8]);
    }
    #[test]
    fn original_b_curve_chain_10() {
        let input = InputCurve::crashy_new_from_points(
            points(&[
                0, 0, 1036831949, 1044768599, 1045220557, 1050107254, 1050253722, 1048876742,
                1053609165, 1036898581, 1056964608, 3185018222, 1058642330, 3196444609, 1060320051,
                3197560732, 1061997773, 3191991508, 1063675494, 1000688017, 1065353216, 1045025671,
                1066192077, 1050134619, 1067030938, 1048790066, 1067869798, 1036258773, 1068708659,
                3185650223, 1069547520, 3196526347, 1070386381, 3197527766, 1071225242, 3191727176,
                1072064102, 1009075094, 1072902963, 1045279004, 1073741824, 1050159172,
            ]),
            false,
        );
        let mut stroke = InProgressCurve::from_curve(input);
        assert_eq!(
            point_bits(&stroke.raw_input_points),
            &[
                0, 0, 1036831949, 1044768599, 1045220557, 1050107254, 1050253722, 1048876742,
                1053609165, 1036898581, 1056964608, 3185018222, 1058642330, 3196444609, 1060320051,
                3197560732, 1061997773, 3191991508, 1063675494, 1000688017, 1065353216, 1045025671,
                1066192077, 1050134619, 1067030938, 1048790066, 1067869798, 1036258773, 1068708659,
                3185650223, 1069547520, 3196526347, 1070386381, 3197527766, 1071225242, 3191727176,
                1072064102, 1009075094, 1072902963, 1045279004, 1072944906, 1045602171, 1072986849,
                1045925338, 1073028792, 1046248505, 1073070735, 1046571672, 1073112678, 1046894839,
                1073154621, 1047218006, 1073196564, 1047541174, 1073238507, 1047864340, 1073280450,
                1048187508, 1073322394, 1048510674, 1073364336, 1048704920, 1073406280, 1048866504,
                1073448222, 1049028088, 1073490166, 1049189671, 1073532109, 1049351255, 1073574052,
                1049512839, 1073615995, 1049674422, 1073657938, 1049836006, 1073699881, 1049997589,
                1073741824, 1050159172
            ]
        );
        assert_eq!(
            stroke.point_weights(),
            &[
                1000, 1000, 1000, 1000, 1000, 1000, 1000, 1000, 1000, 1000, 1000, 1000, 1000, 1000,
                1000, 1000, 1000, 1000, 1000, 1000, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1,
                1, 1, 1, 1
            ]
        );
        assert_eq!(
            stroke.simplified_source_indices(),
            &[
                0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 39
            ]
        );
        let first = stroke.compute_final_curve_with_smoothing(false).unwrap();
        check_curve(
            &first,
            &[
                0, 0, 1036831949, 1044768599, 1045220557, 1050107254, 1050253722, 1048876742,
                1053609165, 1036898581, 1056964608, 3185018222, 1058642330, 3196444609, 1060320051,
                3197560732, 1061997773, 3191991508, 1063675494, 1000688017, 1065353216, 1045025671,
                1066192077, 1050134619, 1067030938, 1048790066, 1067869798, 1036258773, 1068708659,
                3185650223, 1069547520, 3196526347, 1070386381, 3197527766, 1071225242, 3191727176,
                1072064102, 1009075094, 1072902963, 1045279004, 1073741824, 1050159172,
            ],
            &[
                0, 1031923895, 1037522923, 1040938793, 1044604416, 1048827635, 1050642050,
                1051672862, 1053093659, 1055232162, 1057157332, 1057846740, 1058373602, 1059299057,
                1060417515, 1061315379, 1061825755, 1062546709, 1063620764, 1064674216, 1065353216,
            ],
            1079741773,
        );
        assert_eq!(
            stroke.simplified_source_indices(),
            &[
                0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 39
            ]
        );
        assert!(
            stroke.try_add_raw_input_points(&points(&[
                1074580685, 1055192337, 1075419546, 1048386901
            ]))
        );
        stroke.add_weight_to_the_last_point(4);
        let second = stroke.compute_final_curve_with_smoothing(false).unwrap();
        check_curve(
            &second,
            &[
                0, 0, 1036831949, 1044768599, 1045220557, 1050107254, 1050253722, 1048876742,
                1053609165, 1036898581, 1056964608, 3185018222, 1058642330, 3196444609, 1060320051,
                3197560732, 1061997773, 3191991508, 1063675494, 1000688017, 1065353216, 1045025671,
                1066192077, 1050134619, 1067030938, 1048790066, 1067869798, 1036258773, 1068708659,
                3185650223, 1069547520, 3196526347, 1070386381, 3197527766, 1071225242, 3191727176,
                1072064102, 1009075094, 1072902963, 1045279004, 1074580685, 1055192337, 1075419546,
                1048386901,
            ],
            &[
                0, 1029762305, 1035627650, 1039235277, 1042884682, 1046758585, 1049238040,
                1050130419, 1051360410, 1053211722, 1055045194, 1056238841, 1057057832, 1057859003,
                1058827257, 1059604543, 1060046378, 1060670511, 1061600325, 1062512304, 1064155710,
                1065353216,
            ],
            1081973842,
        );
        assert_eq!(
            stroke.point_weights(),
            &[
                1000, 1000, 1000, 1000, 1000, 1000, 1000, 1000, 1000, 1000, 1000, 1000, 1000, 1000,
                1000, 1000, 1000, 1000, 1000, 1000, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1,
                1, 1, 1, 1, 1, 5
            ]
        );
        assert_eq!(
            stroke.simplified_source_indices(),
            &[
                0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 40, 41
            ]
        );
    }
    #[test]
    fn original_b_curve_chain_11() {
        let input = InputCurve::crashy_new_from_points(
            points(&[
                0, 0, 1036831949, 1044768599, 1045220557, 1050107254, 1050253722, 1048876742,
                1053609165, 1036898581, 1056964608, 3185018222, 1058642330, 3196444609, 1060320051,
                3197560732, 1061997773, 3191991508, 1063675494, 1000688017, 1065353216, 1045025671,
                1066192077, 1050134619, 1067030938, 1048790066, 1067869798, 1036258773, 1068708659,
                3185650223, 1069547520, 3196526347, 1070386381, 3197527766, 1071225242, 3191727176,
                1072064102, 1009075094, 1072902963, 1045279004, 1073741824, 1050159172,
            ]),
            false,
        );
        let mut stroke = InProgressCurve::from_curve(input);
        assert_eq!(
            point_bits(&stroke.raw_input_points),
            &[
                0, 0, 1036831949, 1044768599, 1045220557, 1050107254, 1050253722, 1048876742,
                1053609165, 1036898581, 1056964608, 3185018222, 1058642330, 3196444609, 1060320051,
                3197560732, 1061997773, 3191991508, 1063675494, 1000688017, 1065353216, 1045025671,
                1066192077, 1050134619, 1067030938, 1048790066, 1067869798, 1036258773, 1068708659,
                3185650223, 1069547520, 3196526347, 1070386381, 3197527766, 1071225242, 3191727176,
                1072064102, 1009075094, 1072902963, 1045279004, 1072944906, 1045602171, 1072986849,
                1045925338, 1073028792, 1046248505, 1073070735, 1046571672, 1073112678, 1046894839,
                1073154621, 1047218006, 1073196564, 1047541174, 1073238507, 1047864340, 1073280450,
                1048187508, 1073322394, 1048510674, 1073364336, 1048704920, 1073406280, 1048866504,
                1073448222, 1049028088, 1073490166, 1049189671, 1073532109, 1049351255, 1073574052,
                1049512839, 1073615995, 1049674422, 1073657938, 1049836006, 1073699881, 1049997589,
                1073741824, 1050159172
            ]
        );
        assert_eq!(
            stroke.point_weights(),
            &[
                1000, 1000, 1000, 1000, 1000, 1000, 1000, 1000, 1000, 1000, 1000, 1000, 1000, 1000,
                1000, 1000, 1000, 1000, 1000, 1000, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1,
                1, 1, 1, 1
            ]
        );
        assert_eq!(
            stroke.simplified_source_indices(),
            &[
                0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 39
            ]
        );
        let first = stroke.compute_final_curve_with_smoothing(true).unwrap();
        check_curve(
            &first,
            &[
                0, 0, 1036831949, 1044768599, 1045220557, 1050107254, 1050253722, 1048876742,
                1053609165, 1036898581, 1056964608, 3185018222, 1058642330, 3196444609, 1060320051,
                3197560732, 1061997773, 3191991508, 1063675494, 1000688017, 1065353216, 1045025671,
                1066192077, 1050134619, 1067030938, 1048790066, 1067869798, 1036258773, 1068708659,
                3185650223, 1069547520, 3196526347, 1070386381, 3197527766, 1071225242, 3191727176,
                1072064102, 1009075094, 1072902963, 1045279004, 1073741824, 1050159172,
            ],
            &[
                0, 1031923895, 1037522923, 1040938793, 1044604416, 1048827635, 1050642050,
                1051672862, 1053093659, 1055232162, 1057157332, 1057846740, 1058373602, 1059299057,
                1060417515, 1061315379, 1061825755, 1062546709, 1063620764, 1064674216, 1065353216,
            ],
            1079741773,
        );
        assert_eq!(
            stroke.simplified_source_indices(),
            &[
                0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 39
            ]
        );
        assert!(
            stroke.try_add_raw_input_points(&points(&[
                1074580685, 1055192337, 1075419546, 1048386901
            ]))
        );
        stroke.add_weight_to_the_last_point(4);
        let second = stroke.compute_final_curve_with_smoothing(true).unwrap();
        check_curve(
            &second,
            &[
                0, 0, 1036831949, 1044768599, 1045220557, 1050107254, 1050253722, 1048876742,
                1053609165, 1036898581, 1056964608, 3185018222, 1058642330, 3196444609, 1060320051,
                3197560732, 1061997773, 3191991508, 1063675494, 1000688017, 1065353216, 1045025671,
                1066192077, 1050134619, 1067030938, 1048790066, 1067869798, 1036258773, 1068708659,
                3185650223, 1069547520, 3196526347, 1070386381, 3197527766, 1071225242, 3191727176,
                1072064102, 1009075094, 1072902963, 1045279004, 1075419546, 1048386901,
            ],
            &[
                0, 1030418422, 1036171456, 1039939661, 1043378121, 1047424453, 1049640889,
                1050572988, 1051857727, 1053791442, 1055706522, 1056953299, 1057435363, 1058272194,
                1059283546, 1060095429, 1060556930, 1061208844, 1062180045, 1063132616, 1065353216,
            ],
            1081265575,
        );
        assert_eq!(
            stroke.point_weights(),
            &[
                1000, 1000, 1000, 1000, 1000, 1000, 1000, 1000, 1000, 1000, 1000, 1000, 1000, 1000,
                1000, 1000, 1000, 1000, 1000, 1000, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1,
                1, 1, 1, 1, 1, 5
            ]
        );
        assert_eq!(
            stroke.simplified_source_indices(),
            &[
                0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 41
            ]
        );
    }
    #[test]
    fn original_b_curve_chain_12() {
        let input = InputCurve::crashy_new_from_points(
            points(&[
                0, 0, 1036831949, 1044768599, 1045220557, 1050107254, 1050253722, 1048876742,
                1053609165, 1036898581, 1056964608, 3185018222, 1058642330, 3196444609, 1060320051,
                3197560732, 1061997773, 3191991508, 1063675494, 1000688017, 1065353216, 1045025671,
                1066192077, 1050134619, 1067030938, 1048790066, 1067869798, 1036258773, 1068708659,
                3185650223, 1069547520, 3196526347, 1070386381, 3197527766, 1071225242, 3191727176,
                1072064102, 1009075094, 1072902963, 1045279004, 1073741824, 1050159172, 1074161254,
                1048700958, 1074580685, 1035615333, 1075000115, 3186278051, 1075419546, 3196605582,
                1075838976, 3197492013, 1076258406, 3191459325, 1076677837, 1014486637, 1077097267,
                1045528525, 1077516698, 1050180906, 1077936128, 1048609443, 1078355558, 1034968442,
                1078774989, 3186901530, 1079194419, 3196682291, 1079613850, 3197453483, 1080033280,
                3191188032, 1080452710, 1017457578, 1080872141, 1045774165, 1081291571, 1050199814,
                1081711002, 1048455095,
            ]),
            false,
        );
        let mut stroke = InProgressCurve::from_curve(input);
        assert_eq!(
            point_bits(&stroke.raw_input_points),
            &[
                0, 0, 1036831949, 1044768599, 1045220557, 1050107254, 1050253722, 1048876742,
                1053609165, 1036898581, 1056964608, 3185018222, 1058642330, 3196444609, 1060320051,
                3197560732, 1061997773, 3191991508, 1063675494, 1000688017, 1065353216, 1045025671,
                1066192077, 1050134619, 1067030938, 1048790066, 1067869798, 1036258773, 1068708659,
                3185650223, 1069547520, 3196526347, 1070386381, 3197527766, 1071225242, 3191727176,
                1072064102, 1009075094, 1072902963, 1045279004, 1073741824, 1050159172, 1074161254,
                1048700958, 1074580685, 1035615333, 1075000115, 3186278051, 1075419546, 3196605582,
                1075838976, 3197492013, 1076258406, 3191459325, 1076677837, 1014486637, 1077097267,
                1045528525, 1077516698, 1050180906, 1077936128, 1048609443, 1078355558, 1034968442,
                1078774989, 3186901530, 1079194419, 3196682291, 1079613850, 3197453483, 1080033280,
                3191188032, 1080452710, 1017457578, 1080872141, 1045774165, 1081291571, 1050199814,
                1081312542, 1050115601, 1081333514, 1050031388, 1081354486, 1049947174, 1081375457,
                1049862961, 1081396428, 1049778747, 1081417400, 1049694534, 1081438372, 1049610320,
                1081459343, 1049526108, 1081480314, 1049441894, 1081501286, 1049357681, 1081522258,
                1049273467, 1081543230, 1049189254, 1081564200, 1049105041, 1081585173, 1049020827,
                1081606144, 1048936614, 1081627116, 1048852401, 1081648087, 1048768187, 1081669059,
                1048683974, 1081690030, 1048599760, 1081711002, 1048455095
            ]
        );
        assert_eq!(
            stroke.point_weights(),
            &[
                1000, 1000, 1000, 1000, 1000, 1000, 1000, 1000, 1000, 1000, 1000, 1000, 1000, 1000,
                1000, 1000, 1000, 1000, 1000, 1000, 1000, 1000, 1000, 1000, 1000, 1000, 1000, 1000,
                1000, 1000, 1000, 1000, 1000, 1000, 1000, 1000, 1000, 1000, 1000, 1, 1, 1, 1, 1, 1,
                1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1
            ]
        );
        assert_eq!(
            stroke.simplified_source_indices(),
            &[
                0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21, 22,
                23, 24, 25, 26, 27, 28, 29, 30, 31, 32, 33, 34, 35, 36, 37, 38, 58
            ]
        );
        let first = stroke.compute_final_curve_with_smoothing(false).unwrap();
        check_curve(
            &first,
            &[
                0, 0, 1036831949, 1044768599, 1045220557, 1050107254, 1050253722, 1048876742,
                1053609165, 1036898581, 1056964608, 3185018222, 1058642330, 3196444609, 1060320051,
                3197560732, 1061997773, 3191991508, 1063675494, 1000688017, 1065353216, 1045025671,
                1066192077, 1050134619, 1067030938, 1048790066, 1067869798, 1036258773, 1068708659,
                3185650223, 1069547520, 3196526347, 1070386381, 3197527766, 1071225242, 3191727176,
                1072064102, 1009075094, 1072902963, 1045279004, 1073741824, 1050159172, 1074161254,
                1048700958, 1074580685, 1035615333, 1075000115, 3186278051, 1075419546, 3196605582,
                1075838976, 3197492013, 1076258406, 3191459325, 1076677837, 1014486637, 1077097267,
                1045528525, 1077516698, 1050180906, 1077936128, 1048609443, 1078355558, 1034968442,
                1078774989, 3186901530, 1079194419, 3196682291, 1079613850, 3197453483, 1080033280,
                3191188032, 1080452710, 1017457578, 1080872141, 1045774165, 1081291571, 1050199814,
                1081711002, 1048455095,
            ],
            &[
                0, 1023855832, 1029665665, 1032894310, 1036697945, 1040764335, 1042647063,
                1043716686, 1045190977, 1047409995, 1049091814, 1049807178, 1050353878, 1051314176,
                1052474744, 1053406414, 1053936005, 1054684103, 1055798597, 1056891712, 1057280442,
                1057557104, 1058041868, 1058621886, 1059082804, 1059345246, 1059724783, 1060284400,
                1060827989, 1061174912, 1061455097, 1061944392, 1062524013, 1062979944, 1063240296,
                1063625326, 1064187190, 1064727695, 1065069308, 1065353216,
            ],
            1087608303,
        );
        assert_eq!(
            stroke.simplified_source_indices(),
            &[
                0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21, 22,
                23, 24, 25, 26, 27, 28, 29, 30, 31, 32, 33, 34, 35, 36, 37, 38, 58
            ]
        );
        assert!(
            stroke.try_add_raw_input_points(&points(&[
                1082340147, 1053548712, 1082759578, 1045099652
            ]))
        );
        stroke.add_weight_to_the_last_point(4);
        let second = stroke.compute_final_curve_with_smoothing(false).unwrap();
        check_curve(
            &second,
            &[
                0, 0, 1036831949, 1044768599, 1045220557, 1050107254, 1050253722, 1048876742,
                1053609165, 1036898581, 1056964608, 3185018222, 1058642330, 3196444609, 1060320051,
                3197560732, 1061997773, 3191991508, 1063675494, 1000688017, 1065353216, 1045025671,
                1066192077, 1050134619, 1067030938, 1048790066, 1067869798, 1036258773, 1068708659,
                3185650223, 1069547520, 3196526347, 1070386381, 3197527766, 1071225242, 3191727176,
                1072064102, 1009075094, 1072902963, 1045279004, 1073741824, 1050159172, 1074161254,
                1048700958, 1074580685, 1035615333, 1075000115, 3186278051, 1075419546, 3196605582,
                1075838976, 3197492013, 1076258406, 3191459325, 1076677837, 1014486637, 1077097267,
                1045528525, 1077516698, 1050180906, 1077936128, 1048609443, 1078355558, 1034968442,
                1078774989, 3186901530, 1079194419, 3196682291, 1079613850, 3197453483, 1080033280,
                3191188032, 1080452710, 1017457578, 1080872141, 1045774165, 1081291571, 1050199814,
                1082340147, 1053548712, 1082759578, 1045099652,
            ],
            &[
                0, 1023089140, 1028660842, 1032243545, 1035786188, 1040110914, 1041902695,
                1042898924, 1044272055, 1046338812, 1048385653, 1049147105, 1049656292, 1050550698,
                1051631632, 1052499374, 1052992627, 1053689394, 1054727415, 1055745525, 1056401745,
                1056917100, 1057392356, 1057932576, 1058361866, 1058606301, 1058959795, 1059481013,
                1059987304, 1060310422, 1060571382, 1061027104, 1061566953, 1061991600, 1062234088,
                1062592699, 1063116009, 1063619427, 1063937599, 1064684783, 1065353216,
            ],
            1088629864,
        );
        assert_eq!(
            stroke.point_weights(),
            &[
                1000, 1000, 1000, 1000, 1000, 1000, 1000, 1000, 1000, 1000, 1000, 1000, 1000, 1000,
                1000, 1000, 1000, 1000, 1000, 1000, 1000, 1000, 1000, 1000, 1000, 1000, 1000, 1000,
                1000, 1000, 1000, 1000, 1000, 1000, 1000, 1000, 1000, 1000, 1000, 1, 1, 1, 1, 1, 1,
                1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 5
            ]
        );
        assert_eq!(
            stroke.simplified_source_indices(),
            &[
                0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21, 22,
                23, 24, 25, 26, 27, 28, 29, 30, 31, 32, 33, 34, 35, 36, 37, 38, 59, 60
            ]
        );
    }
    #[test]
    fn original_b_curve_chain_13() {
        let input = InputCurve::crashy_new_from_points(
            points(&[
                0, 0, 1036831949, 1044768599, 1045220557, 1050107254, 1050253722, 1048876742,
                1053609165, 1036898581, 1056964608, 3185018222, 1058642330, 3196444609, 1060320051,
                3197560732, 1061997773, 3191991508, 1063675494, 1000688017, 1065353216, 1045025671,
                1066192077, 1050134619, 1067030938, 1048790066, 1067869798, 1036258773, 1068708659,
                3185650223, 1069547520, 3196526347, 1070386381, 3197527766, 1071225242, 3191727176,
                1072064102, 1009075094, 1072902963, 1045279004, 1073741824, 1050159172, 1074161254,
                1048700958, 1074580685, 1035615333, 1075000115, 3186278051, 1075419546, 3196605582,
                1075838976, 3197492013, 1076258406, 3191459325, 1076677837, 1014486637, 1077097267,
                1045528525, 1077516698, 1050180906, 1077936128, 1048609443, 1078355558, 1034968442,
                1078774989, 3186901530, 1079194419, 3196682291, 1079613850, 3197453483, 1080033280,
                3191188032, 1080452710, 1017457578, 1080872141, 1045774165, 1081291571, 1050199814,
                1081711002, 1048455095,
            ]),
            false,
        );
        let mut stroke = InProgressCurve::from_curve(input);
        assert_eq!(
            point_bits(&stroke.raw_input_points),
            &[
                0, 0, 1036831949, 1044768599, 1045220557, 1050107254, 1050253722, 1048876742,
                1053609165, 1036898581, 1056964608, 3185018222, 1058642330, 3196444609, 1060320051,
                3197560732, 1061997773, 3191991508, 1063675494, 1000688017, 1065353216, 1045025671,
                1066192077, 1050134619, 1067030938, 1048790066, 1067869798, 1036258773, 1068708659,
                3185650223, 1069547520, 3196526347, 1070386381, 3197527766, 1071225242, 3191727176,
                1072064102, 1009075094, 1072902963, 1045279004, 1073741824, 1050159172, 1074161254,
                1048700958, 1074580685, 1035615333, 1075000115, 3186278051, 1075419546, 3196605582,
                1075838976, 3197492013, 1076258406, 3191459325, 1076677837, 1014486637, 1077097267,
                1045528525, 1077516698, 1050180906, 1077936128, 1048609443, 1078355558, 1034968442,
                1078774989, 3186901530, 1079194419, 3196682291, 1079613850, 3197453483, 1080033280,
                3191188032, 1080452710, 1017457578, 1080872141, 1045774165, 1081291571, 1050199814,
                1081312542, 1050115601, 1081333514, 1050031388, 1081354486, 1049947174, 1081375457,
                1049862961, 1081396428, 1049778747, 1081417400, 1049694534, 1081438372, 1049610320,
                1081459343, 1049526108, 1081480314, 1049441894, 1081501286, 1049357681, 1081522258,
                1049273467, 1081543230, 1049189254, 1081564200, 1049105041, 1081585173, 1049020827,
                1081606144, 1048936614, 1081627116, 1048852401, 1081648087, 1048768187, 1081669059,
                1048683974, 1081690030, 1048599760, 1081711002, 1048455095
            ]
        );
        assert_eq!(
            stroke.point_weights(),
            &[
                1000, 1000, 1000, 1000, 1000, 1000, 1000, 1000, 1000, 1000, 1000, 1000, 1000, 1000,
                1000, 1000, 1000, 1000, 1000, 1000, 1000, 1000, 1000, 1000, 1000, 1000, 1000, 1000,
                1000, 1000, 1000, 1000, 1000, 1000, 1000, 1000, 1000, 1000, 1000, 1, 1, 1, 1, 1, 1,
                1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1
            ]
        );
        assert_eq!(
            stroke.simplified_source_indices(),
            &[
                0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21, 22,
                23, 24, 25, 26, 27, 28, 29, 30, 31, 32, 33, 34, 35, 36, 37, 38, 58
            ]
        );
        let first = stroke.compute_final_curve_with_smoothing(true).unwrap();
        check_curve(
            &first,
            &[
                0, 0, 1036831949, 1044768599, 1045220557, 1050107254, 1050253722, 1048876742,
                1053609165, 1036898581, 1056964608, 3185018222, 1058642330, 3196444609, 1060320051,
                3197560732, 1061997773, 3191991508, 1063675494, 1000688017, 1065353216, 1045025671,
                1066192077, 1050134619, 1067030938, 1048790066, 1067869798, 1036258773, 1068708659,
                3185650223, 1069547520, 3196526347, 1070386381, 3197527766, 1071225242, 3191727176,
                1072064102, 1009075094, 1072902963, 1045279004, 1073741824, 1050159172, 1074161254,
                1048700958, 1074580685, 1035615333, 1075000115, 3186278051, 1075419546, 3196605582,
                1075838976, 3197492013, 1076258406, 3191459325, 1076677837, 1014486637, 1077097267,
                1045528525, 1077516698, 1050180906, 1077936128, 1048609443, 1078355558, 1034968442,
                1078774989, 3186901530, 1079194419, 3196682291, 1079613850, 3197453483, 1080033280,
                3191188032, 1080452710, 1017457578, 1080872141, 1045774165, 1081291571, 1050199814,
                1081711002, 1048455095,
            ],
            &[
                0, 1023855832, 1029665665, 1032894310, 1036697945, 1040764335, 1042647063,
                1043716686, 1045190977, 1047409995, 1049091814, 1049807178, 1050353878, 1051314176,
                1052474744, 1053406414, 1053936005, 1054684103, 1055798597, 1056891712, 1057280442,
                1057557104, 1058041868, 1058621886, 1059082804, 1059345246, 1059724783, 1060284400,
                1060827989, 1061174912, 1061455097, 1061944392, 1062524013, 1062979944, 1063240296,
                1063625326, 1064187190, 1064727695, 1065069308, 1065353216,
            ],
            1087608303,
        );
        assert_eq!(
            stroke.simplified_source_indices(),
            &[
                0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21, 22,
                23, 24, 25, 26, 27, 28, 29, 30, 31, 32, 33, 34, 35, 36, 37, 38, 58
            ]
        );
        assert!(
            stroke.try_add_raw_input_points(&points(&[
                1082340147, 1053548712, 1082759578, 1045099652
            ]))
        );
        stroke.add_weight_to_the_last_point(4);
        let second = stroke.compute_final_curve_with_smoothing(true).unwrap();
        check_curve(
            &second,
            &[
                0, 0, 1036831949, 1044768599, 1045220557, 1050107254, 1050253722, 1048876742,
                1053609165, 1036898581, 1056964608, 3185018222, 1058642330, 3196444609, 1060320051,
                3197560732, 1061997773, 3191991508, 1063675494, 1000688017, 1065353216, 1045025671,
                1066192077, 1050134619, 1067030938, 1048790066, 1067869798, 1036258773, 1068708659,
                3185650223, 1069547520, 3196526347, 1070386381, 3197527766, 1071225242, 3191727176,
                1072064102, 1009075094, 1072902963, 1045279004, 1073741824, 1050159172, 1074161254,
                1048700958, 1074580685, 1035615333, 1075000115, 3186278051, 1075419546, 3196605582,
                1075838976, 3197492013, 1076258406, 3191459325, 1076677837, 1014486637, 1077097267,
                1045528525, 1077516698, 1050180906, 1077936128, 1048609443, 1078355558, 1034968442,
                1078774989, 3186901530, 1079194419, 3196682291, 1079613850, 3197453483, 1080033280,
                3191188032, 1080452710, 1017457578, 1080872141, 1045774165, 1081291571, 1050199814,
                1082759578, 1045099652,
            ],
            &[
                0, 1023298228, 1028834139, 1032355779, 1035943435, 1040255251, 1042031073,
                1043039960, 1044430537, 1046523554, 1048586201, 1049260945, 1049776601, 1050682372,
                1051777040, 1052655807, 1053155327, 1053860947, 1054912156, 1055943202, 1056607760,
                1057047136, 1057504374, 1058051458, 1058486203, 1058733743, 1059091729, 1059619570,
                1060132292, 1060459516, 1060723792, 1061185304, 1061732012, 1062162055, 1062407624,
                1062770791, 1063300751, 1063810565, 1064132780, 1065353216,
            ],
            1088443074,
        );
        assert_eq!(
            stroke.point_weights(),
            &[
                1000, 1000, 1000, 1000, 1000, 1000, 1000, 1000, 1000, 1000, 1000, 1000, 1000, 1000,
                1000, 1000, 1000, 1000, 1000, 1000, 1000, 1000, 1000, 1000, 1000, 1000, 1000, 1000,
                1000, 1000, 1000, 1000, 1000, 1000, 1000, 1000, 1000, 1000, 1000, 1, 1, 1, 1, 1, 1,
                1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 5
            ]
        );
        assert_eq!(
            stroke.simplified_source_indices(),
            &[
                0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21, 22,
                23, 24, 25, 26, 27, 28, 29, 30, 31, 32, 33, 34, 35, 36, 37, 38, 60
            ]
        );
    }
    #[test]
    fn original_b_curve_chain_14() {
        let input = InputCurve::crashy_new_from_points(
            points(&[
                0, 0, 1073741824, 1065353216, 1077936128, 0, 1065353216, 3212836864, 0, 0,
            ]),
            false,
        );
        let mut stroke = InProgressCurve::from_curve(input);
        assert_eq!(
            point_bits(&stroke.raw_input_points),
            &[
                0, 0, 1073741824, 1065353216, 1077936128, 0, 1065353216, 3212836864, 1064514355,
                3211998003, 1063675494, 3211159142, 1062836634, 3210320282, 1061997773, 3209481421,
                1061158912, 3208642560, 1060320051, 3207803699, 1059481190, 3206964838, 1058642329,
                3206125977, 1057803468, 3205287116, 1056964606, 3204448254, 1055286884, 3202770532,
                1053609162, 3201092810, 1051931440, 3199415088, 1050253718, 3197737366, 1048575992,
                3196059640, 1045220548, 3192704196, 1041865104, 3189348752, 1036831928, 3184315576,
                1028443296, 3175926944, 0, 0
            ]
        );
        assert_eq!(
            stroke.point_weights(),
            &[
                1000, 1000, 1000, 1000, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1
            ]
        );
        assert_eq!(stroke.simplified_source_indices(), &[0, 1, 2, 3, 23]);
        let first = stroke.compute_final_curve_with_smoothing(false).unwrap();
        check_curve(
            &first,
            &[
                0, 0, 1073741824, 1065353216, 1077936128, 0, 1065353216, 3212836864, 0, 0,
            ],
            &[0, 1050464680, 1056964607, 1062103252, 1065353216],
            1089052215,
        );
        assert_eq!(stroke.simplified_source_indices(), &[0, 1, 2, 3, 23]);
        assert!(
            stroke.try_add_raw_input_points(&points(&[
                1045220557, 1041865114, 1053609165, 3175926989
            ]))
        );
        stroke.add_weight_to_the_last_point(4);
        let second = stroke.compute_final_curve_with_smoothing(false).unwrap();
        check_curve(
            &second,
            &[
                0, 0, 1073741824, 1065353216, 1077936128, 0, 1065353216, 3212836864, 0, 0,
                1045220557, 1041865114, 1053609165, 3175926989,
            ],
            &[
                0, 1049765600, 1055823390, 1061183104, 1064211999, 1064747437, 1065353216,
            ],
            1090169667,
        );
        assert_eq!(
            stroke.point_weights(),
            &[
                1000, 1000, 1000, 1000, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1,
                1, 5
            ]
        );
        assert_eq!(
            stroke.simplified_source_indices(),
            &[0, 1, 2, 3, 23, 24, 25]
        );
    }
    #[test]
    fn original_b_curve_chain_15() {
        let input = InputCurve::crashy_new_from_points(
            points(&[
                0, 0, 1073741824, 1065353216, 1077936128, 0, 1065353216, 3212836864, 0, 0,
            ]),
            false,
        );
        let mut stroke = InProgressCurve::from_curve(input);
        assert_eq!(
            point_bits(&stroke.raw_input_points),
            &[
                0, 0, 1073741824, 1065353216, 1077936128, 0, 1065353216, 3212836864, 1064514355,
                3211998003, 1063675494, 3211159142, 1062836634, 3210320282, 1061997773, 3209481421,
                1061158912, 3208642560, 1060320051, 3207803699, 1059481190, 3206964838, 1058642329,
                3206125977, 1057803468, 3205287116, 1056964606, 3204448254, 1055286884, 3202770532,
                1053609162, 3201092810, 1051931440, 3199415088, 1050253718, 3197737366, 1048575992,
                3196059640, 1045220548, 3192704196, 1041865104, 3189348752, 1036831928, 3184315576,
                1028443296, 3175926944, 0, 0
            ]
        );
        assert_eq!(
            stroke.point_weights(),
            &[
                1000, 1000, 1000, 1000, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1
            ]
        );
        assert_eq!(stroke.simplified_source_indices(), &[0, 1, 2, 3, 23]);
        let first = stroke.compute_final_curve_with_smoothing(true).unwrap();
        check_curve(
            &first,
            &[
                0, 0, 1073741824, 1065353216, 1077936128, 0, 1065353216, 3212836864, 0, 0,
            ],
            &[0, 1050464680, 1056964607, 1062103252, 1065353216],
            1089052215,
        );
        assert_eq!(stroke.simplified_source_indices(), &[0, 1, 2, 3, 23]);
        assert!(
            stroke.try_add_raw_input_points(&points(&[
                1045220557, 1041865114, 1053609165, 3175926989
            ]))
        );
        stroke.add_weight_to_the_last_point(4);
        let second = stroke.compute_final_curve_with_smoothing(true).unwrap();
        check_curve(
            &second,
            &[
                0, 0, 1073741824, 1065353216, 1077936128, 0, 1065353216, 3212836864, 1053250714,
                3199741478, 1053609165, 3175926989,
            ],
            &[
                0, 1050783251, 1057224634, 1062522564, 1064618941, 1065353216,
            ],
            1088591899,
        );
        assert_eq!(
            stroke.point_weights(),
            &[
                1000, 1000, 1000, 1000, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1,
                1, 5
            ]
        );
        assert_eq!(stroke.simplified_source_indices(), &[0, 1, 2, 3, 16, 25]);
    }
    #[test]
    fn failed_duplicate_append_keeps_points_and_old_metadata() {
        let input = InputCurve::crashy_new_from_points(vec![Vec2::ZERO, Vec2::X], false);
        let mut stroke = InProgressCurve::from_curve(input);
        assert!(!stroke.try_add_raw_input_points(&[Vec2::X]));
        assert_eq!(stroke.raw_input_points, [Vec2::ZERO, Vec2::X, Vec2::X]);
        assert_eq!(stroke.point_weights(), [1, 1]);
        assert_eq!(stroke.raw_input_curve_length_ws(), 1.0);
        assert_eq!(stroke.simplified_source_indices(), [0, 1]);
    }
    #[test]
    fn last_point_weight_clamps_after_wrapping_addition() {
        let input = InputCurve::crashy_new_from_points(vec![Vec2::ZERO, Vec2::X], false);
        let mut stroke = InProgressCurve::from_curve(input);
        stroke.add_weight_to_the_last_point(100);
        assert_eq!(stroke.point_weights(), [1, 50]);
        stroke.add_weight_to_the_last_point(usize::MAX - 49);
        assert_eq!(stroke.point_weights(), [1, 1]);
    }
}
