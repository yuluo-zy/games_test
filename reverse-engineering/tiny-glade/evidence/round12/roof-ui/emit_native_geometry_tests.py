from pathlib import Path
import json
out=Path(__file__).parent
proposed=out/'proposed/tiny-glade/crates'
helpers=proposed/'systems/roof/src/ui_focused/native_helpers.rs'
txt=helpers.read_text(encoding='utf8').split('\n#[cfg(test)]')[0]
data=json.loads((out/'native-focused-helper-fixtures.json').read_text())
tests='''
#[cfg(test)]
mod original_native_geometry_tests {
    use super::*;
    fn same_bits(actual: f32, expected: u32) {
        let value=f32::from_bits(expected);
        if value.is_nan() { assert!(actual.is_nan()); } else { assert_eq!(actual.to_bits(),expected); }
    }
    #[test]
    fn all_twenty_four_box_records_match_original() {
'''
for case in data['boxes']:
 tests+='        { let [x,y,z]='+str(case['input_bits'])+'.map(f32::from_bits);\n'
 tests+='          let actual=box_positions(x,y,z); let expected='+str(case['positions_bits'])+';\n'
 tests+='          for(i,p)in actual.into_iter().enumerate(){for(j,v)in p.to_array().into_iter().enumerate(){same_bits(v,expected[i*3+j]);}} }\n'
tests+='''    }
    #[test]
    fn rotation_matches_complete_original_private_function() {
'''
for case in data['xz']:
 tests+='        { let [x,z]='+str(case['input_bits'])+'.map(f32::from_bits); let expected='+str(case['quaternion_bits'])+';\n'
 tests+='          for(actual,expected)in xz_rotation_bad(x,z).to_array().into_iter().zip(expected){same_bits(actual,expected);} }\n'
tests+='    }\n}\n'
helpers.write_text(txt+tests,encoding='utf8')
projection=proposed/'country-core/src/utils/project_cursor_onto_ray.rs'
txt=projection.read_text(encoding='utf8').split('\n#[cfg(test)]')[0]
data=json.loads((out/'native-projection-fixtures.json').read_text())
tests='''
#[cfg(test)]
mod original_projection_tests {
    use super::*;
    fn same(actual: f32, expected: u32) {
        let expected=f32::from_bits(expected);
        if expected.is_nan() { assert!(actual.is_nan()); }
        else { assert_eq!(actual.to_bits(),expected.to_bits()); }
    }
    #[test]
    fn original_full_projection_including_near_zero_and_behind_camera() {
        const CASES:&[([u32;46],[u32;3])]=&[
'''
for case in data['cases']:
 assert len(case['input_bits'])==46
 tests+='            ('+str(case['input_bits'])+','+str(case['output_bits'])+'),\n'
tests+='''        ];
        for (index,(bits,expected)) in CASES.iter().enumerate() {
            let a=bits.map(f32::from_bits);
            let screen=ProportionalScreenSpace{size:Vec2::new(a[0],a[1]),aspect:a[2],unit:a[3]};
            let view=Mat4::from_cols_array(a[6..22].try_into().unwrap());
            let projection=Mat4::from_cols_array(a[22..38].try_into().unwrap());
            let ray=Ray{origin:Vec3::from_slice(&a[38..41]),direction:Vec3::from_slice(&a[41..44])};
            let actual=ProportionalScreenSpaceCoord(Vec2::new(a[4],a[5]))
                .project_onto_ray(&screen,&view,&projection,ray,[a[44],a[45]]);
            for (actual,expected) in actual.to_array().into_iter().zip(*expected) {
                if f32::from_bits(expected).is_nan(){assert!(actual.is_nan(),"native case{index}");}
                else {assert_eq!(actual.to_bits(),expected,"native case{index}");}
            }
        }
    }
    #[test]
    fn original_constructor_and_pixel_transform_including_zero_window() {
'''
for case in data['transform_screen']:
 tests+='        { let screen=ProportionalScreenSpace::from_window_size('+str(case['size'])+');\n'
 tests+='          let values=[screen.size.x,screen.size.y,screen.aspect,screen.unit];for(a,b)in values.into_iter().zip('+str(case['screen_bits'])+'){same(a,b);}\n'
 tests+='          let [x,y]='+str(case['xy_bits'])+'.map(f32::from_bits); let p=screen.transform_screen_space(x,y);\n'
 tests+='          for(a,b)in p.to_array().into_iter().zip('+str(case['output_bits'])+'){same(a,b);} }\n'
tests+='''    }
    #[test]
    #[should_panic]
    fn reversed_projection_range_keeps_original_panic() {
        let screen=ProportionalScreenSpace::from_window_size([100,100]);
        ProportionalScreenSpaceCoord(Vec2::ZERO).project_onto_ray(&screen,&Mat4::IDENTITY,&Mat4::IDENTITY,
            Ray{origin:Vec3::ZERO,direction:Vec3::Y},[1.0,0.0]);
    }
}
'''
projection.write_text(txt+tests,encoding='utf8')
print('source fixtures: xz13, box5, projection146, constructor+transform12')
