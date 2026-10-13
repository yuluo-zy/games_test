"""Generate actual Rust comparisons against captured B native results."""
import json
from pathlib import Path
OUT=Path(__file__).parent;ROOT=OUT.parents[2]
fixtures=json.loads((OUT/'spline-rdp-native-fixtures.json').read_text(encoding='utf8'))['fixtures']
src=ROOT/'reconstruction/shared-foundation/crates/utils/src'
rdp=(src/'rdp.rs').read_text(encoding='utf8')
rdp=rdp[:rdp.rfind('\n#[cfg(test)]')] if '\n#[cfg(test)]' in rdp else rdp
test='''
#[cfg(test)]
#[allow(clippy::type_complexity)]
mod native_oracle {
    use super::*;
    #[test] fn original_b_indices() {
        let fixtures: &[(&[[u32;3]],u32,u32,&[usize])] = &[
'''
for row in fixtures['rdp']:
    test+=f"            (&{row['points']!r}, {row['epsilon']}, {row['spacing']}, &{row['indices']!r}),\n"
test+='''        ];
        for (points, epsilon, spacing, indices) in fixtures {
            let points:Vec<_> = points.iter().map(|p|Vec3::from_array(p.map(f32::from_bits))).collect();
            let result=Rdp::new_with_opts(&points,f32::from_bits(*epsilon),f32::from_bits(*spacing));
            assert_eq!(result.indices,*indices,"points={points:?}, epsilon={}, spacing={}",f32::from_bits(*epsilon),f32::from_bits(*spacing));
        }
    }
}
'''
(src/'rdp.rs').write_text(rdp+test,encoding='utf8')
if not (src/'natural_cubic.rs').read_text(encoding='utf8').find('from_control_points_with_tangents')>=0:
    print('RDP embedded; spline constructors not yet written');raise SystemExit()
spline=(src/'natural_cubic.rs').read_text(encoding='utf8')
spline=spline[:spline.rfind('\n#[cfg(test)]')] if '\n#[cfg(test)]' in spline else spline
test='''
#[cfg(test)]
#[allow(clippy::type_complexity)]
mod native_oracle {
    use super::*;
    use std::sync::atomic::{AtomicU32,Ordering};
    static MAX_COEFFICIENT:AtomicU32=AtomicU32::new(0);
    static MAX_POSITION:AtomicU32=AtomicU32::new(0);
    static MAX_TANGENT:AtomicU32=AtomicU32::new(0);
    fn compare(actual:f32,bits:u32,maximum:&AtomicU32) {
        let expected=f32::from_bits(bits);let error=(actual-expected).abs();
        if error.is_finite() {maximum.fetch_max(error.to_bits(),Ordering::Relaxed);}
        assert!(error<=0.00001*expected.abs().max(1.0) || (actual.is_nan()&&expected.is_nan()),"actual={actual}, original={expected}, error={error}");
    }
    fn close(actual:f32,bits:u32) {
        compare(actual,bits,&MAX_COEFFICIENT);
    }
'''
for dims in [2,3]:
    test+=f'''    #[test] fn original_{dims}d_coefficients() {{
        let fixtures:&[(&[[u32;{dims}]],Option<[u32;{dims}]>,Option<[u32;{dims}]>,&[[u32;{1+dims*5}]])] = &[
'''
    for row in fixtures[f'spline{dims}d']:
        start='None' if row['start'] is None else f"Some({row['start']!r})"
        end='None' if row['end'] is None else f"Some({row['end']!r})"
        test+=f"            (&{row['points']!r}, {start}, {end}, &{row['segments']!r}),\n"
    test+=f'''        ];
        for (points,start,end,expected) in fixtures {{
            let points:Vec<_>=points.iter().map(|p|Vec{dims}::from_array(p.map(f32::from_bits))).collect();
            let start=start.map(|v|Vec{dims}::from_array(v.map(f32::from_bits)));
            let end=end.map(|v|Vec{dims}::from_array(v.map(f32::from_bits)));
            let result=Spline{dims}D::from_control_points_with_tangents(&points,start,end);
            assert_eq!(result.segments.len(),expected.len());
            for (segment,expected) in result.segments.iter().zip(*expected) {{
                close(segment.start_t,expected[0]);
                for coefficient in 0..4 {{
                    let actual=segment.coefficients[coefficient].to_array();
                    for axis in 0..{dims} {{close(actual[axis],expected[1+axis*5+coefficient]);}}
                }}
            }}
        }}
        println!("spline{dims}d_max_coefficient_error={{}}",f32::from_bits(MAX_COEFFICIENT.load(Ordering::Relaxed)));
    }}
'''
if 'from_periodic_loop' in spline:
    test+='''    #[test] fn original_periodic_coefficients() {
        let fixtures:&[(&[[u32;2]],&[[u32;11]])] = &[
'''
    for row in fixtures['spline_periodic']:
        test+=f"            (&{row['points']!r}, &{row['segments']!r}),\n"
    test+='''        ];
        for (points,expected) in fixtures {
            let points:Vec<_>=points.iter().map(|p|Vec2::from_array(p.map(f32::from_bits))).collect();
            let result=Spline2D::from_periodic_loop(&points);
            assert_eq!(result.segments.len(),expected.len());
            for (segment,expected) in result.segments.iter().zip(*expected) {
                close(segment.start_t,expected[0]);
                for coefficient in 0..4 {for axis in 0..2 {close(segment.coefficients[coefficient].to_array()[axis],expected[1+axis*5+coefficient]);}}
            }
        }
        println!("periodic_max_coefficient_error={}",f32::from_bits(MAX_COEFFICIENT.load(Ordering::Relaxed)));
    }
'''
test+='''    #[test] fn original_3d_point_tangent_outputs() {
        let fixtures:&[(&[[u32;3]],Option<[u32;3]>,Option<[u32;3]>,&[(u32,[u32;3],[u32;3])])] = &[
'''
for row in fixtures['spline3d']:
    start='None' if row['start'] is None else f"Some({row['start']!r})"
    end='None' if row['end'] is None else f"Some({row['end']!r})"
    test+=f"            (&{row['points']!r}, {start}, {end}, &["
    test+=','.join(f"({r['t']},{r['position']!r},{r['tangent']!r})" for r in row['samples'])+']),\n'
test+='''        ];
        for (points,start,end,samples) in fixtures {
            let points:Vec<_>=points.iter().map(|p|Vec3::from_array(p.map(f32::from_bits))).collect();
            let spline=Spline3D::from_control_points_with_tangents(&points,start.map(|v|Vec3::from_array(v.map(f32::from_bits))),end.map(|v|Vec3::from_array(v.map(f32::from_bits))));
            for (t,position,tangent) in *samples {
                for (actual,expected) in spline.position_at_t(f32::from_bits(*t)).to_array().into_iter().zip(*position) {compare(actual,expected,&MAX_POSITION);}
                for (actual,expected) in spline.tangent_at_t(f32::from_bits(*t)).to_array().into_iter().zip(*tangent) {compare(actual,expected,&MAX_TANGENT);}
            }
        }
        println!("spline_max_position_error={}",f32::from_bits(MAX_POSITION.load(Ordering::Relaxed)));
        println!("spline_max_tangent_error={}",f32::from_bits(MAX_TANGENT.load(Ordering::Relaxed)));
    }
}
'''
(src/'natural_cubic.rs').write_text(spline+test,encoding='utf8')
print('RDP and spline fixtures embedded')
