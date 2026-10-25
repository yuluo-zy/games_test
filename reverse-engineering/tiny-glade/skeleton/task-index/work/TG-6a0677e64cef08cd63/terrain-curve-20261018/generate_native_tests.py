import json
from pathlib import Path
w=Path(__file__).resolve().parent
x=json.loads((w/'native-fixtures.json').read_text())
s='''
#[cfg(test)]
mod native_tests {
    use super::*;
    fn points(bits: &[u32]) -> Vec<Vec2> {
        bits.chunks_exact(2).map(|p| Vec2::new(f32::from_bits(p[0]), f32::from_bits(p[1]))).collect()
    }
    fn point_bits(points: &[Vec2]) -> Vec<u32> {
        points.iter().flat_map(|p| p.to_array().map(f32::to_bits)).collect()
    }
    fn check_curve(curve: &Curve2, points: &[u32], u: &[u32], length: u32) {
        assert_eq!(curve.points.iter().flat_map(|p| p.map(f32::to_bits)).collect::<Vec<_>>(), points);
        assert_eq!(curve.points_u.iter().map(|v| v.to_bits()).collect::<Vec<_>>(), u);
        assert_eq!(curve.length.to_bits(), length);
    }
'''
for i,c in enumerate(x['cases']):
 flat=lambda p:[v for row in p for v in row]
 arr=lambda p:'&'+str(p).lower()
 s+=f'''    #[test]
    fn original_b_curve_chain_{i}() {{
        let input = InputCurve::crashy_new_from_points(points({arr(flat(c['points']))}), false);
        let mut stroke = InProgressCurve::from_curve(input);
        assert_eq!(point_bits(&stroke.raw_input_points), {arr(c['initial']['raw']['points'])});
        assert_eq!(stroke.point_weights(), {arr(c['initial']['weights'])});
        assert_eq!(stroke.simplified_source_indices(), {arr(c['initial']['indices'])});
        let first = stroke.compute_final_curve_with_smoothing({str(c['smooth']).lower()}).unwrap();
        check_curve(&first, {arr(c['first']['points'])}, {arr(c['first']['points_u'])}, {c['first']['length']});
        assert_eq!(stroke.simplified_source_indices(), {arr(c['after_first']['indices'])});
        assert!(stroke.try_add_raw_input_points(&points({arr(flat(c['append']))})));
        stroke.add_weight_to_the_last_point(4);
        let second = stroke.compute_final_curve_with_smoothing({str(c['smooth']).lower()}).unwrap();
        check_curve(&second, {arr(c['second']['points'])}, {arr(c['second']['points_u'])}, {c['second']['length']});
        assert_eq!(stroke.point_weights(), {arr(c['after_second']['weights'])});
        assert_eq!(stroke.simplified_source_indices(), {arr(c['after_second']['indices'])});
    }}
'''
s+='''    #[test]
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
'''
p=w/'in_progress_curve.rs';p.write_text(p.read_text()+s)
print({'generated_tests':18,'native_cases':len(x['cases'])})
