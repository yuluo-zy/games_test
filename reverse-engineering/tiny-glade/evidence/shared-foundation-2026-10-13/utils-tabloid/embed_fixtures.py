from pathlib import Path
import json
root=Path('D:/game/reverse-engineering/tiny-glade')
fixtures=json.loads((Path(__file__).parent/'native-fixtures.json').read_text())['fixtures']
modules={
 'rng':('rng','for &(seed,expected) in cases { assert_eq!(init_rng_u64(seed),expected); }','u64,u64'),
 'loop_index':('loop_range','for &(idx,period,expected) in cases { assert_eq!(LoopIdx{idx,period}.index() as u64,expected); }','i32,i32,u64'),
 'circular_lerp':('circular_f32','for &(a,b,t,expected) in cases { let out=CircularF32(f32::from_bits(a)).lerp(f32::from_bits(b),f32::from_bits(t)); assert_eq!(out.0.to_bits(),expected,"a={a:x},b={b:x},t={t:x}"); }','u32,u32,u32,u32'),
 'angle_clamp':('angle_range','for &(min,len,angle,expected) in cases { let out=AngleRange{min:f32::from_bits(min),len:f32::from_bits(len)}.clamp(f32::from_bits(angle)); assert_eq!(out.to_bits(),expected,"min={min:x},len={len:x},angle={angle:x}"); }','u32,u32,u32,u32'),
 'easing':('easing_functions','for &(tag,t,bias,expected) in cases { let method=match tag {0=>EasingFunction::CircularInOut,1=>EasingFunction::CubicInOut,2=>EasingFunction::SineInOut,3=>EasingFunction::Bias(f32::from_bits(bias)),4=>EasingFunction::ElasticInOut,5=>EasingFunction::BackOut,6=>EasingFunction::CubicOut,_=>unreachable!()}; let out=method.apply(f32::from_bits(t));if tag == 4 && !cfg!(target_os = "windows") { assert!(out.to_bits().abs_diff(expected) <= 1, "tag={tag},t={t:x}"); } else { assert_eq!(out.to_bits(),expected,"tag={tag},t={t:x}"); } }','u32,u32,u32,u32'),
 'rgb9e5':('rgb9e5','for &(r,g,b,expected) in cases { assert_eq!(pack_rgb9e5(Vec3::new(f32::from_bits(r),f32::from_bits(g),f32::from_bits(b))),expected); }','u32,u32,u32,u32'),
}
for family,(module,body,types) in modules.items():
 file=root/f'reconstruction/shared-foundation/crates/utils/src/{module}.rs'
 source=file.read_text(encoding='utf8'); marker='\n#[cfg(test)]\nmod native_oracle'
 if marker in source:source=source.split(marker)[0]
 rows=',\n'.join('('+','.join(str(v) for v in row)+')'for row in fixtures[family])
 source+=marker+' {\nuse super::*;\n#[test]fn original_b_instructions() {\nlet cases:&[('+types+')]=&[\n'+rows+'\n];\n'+body+'\n}\n}\n'
 file.write_text(source,encoding='utf8')
print('embedded fixture families',len(modules))
