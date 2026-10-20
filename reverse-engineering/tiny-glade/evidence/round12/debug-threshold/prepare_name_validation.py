from pathlib import Path
import json
here=Path(__file__).resolve().parent
source=(here/'proposed/tiny-glade/crates/country-core/src/utils/debug_viz.rs').read_text(encoding='utf8')
start=source.index('    fn signifier_name(&self) -> String {')
end=source.index('\n    }',start)+6
method=source[start:end]
fixtures=json.loads((here/'native-sphere-show-fixtures.json').read_text())['cases']
test='''use glam::Vec3;
#[derive(Clone,Copy)] enum DebugColor {White,Red,Blue,Green,Grey,Custom(Vec3)}
struct DebugSphere {color:DebugColor,radius:f32,center:Vec3,alpha:f32,solid:bool}
impl DebugSphere {
'''+method+'\n}\nfn main(){\n'
colors=['DebugColor::White','DebugColor::Red','DebugColor::Blue','DebugColor::Green','DebugColor::Grey','DebugColor::Custom(Vec3::new(0.2,0.4,0.6))']
for color,case in zip(colors,fixtures):
    test+='assert_eq!(DebugSphere{color:'+color+',radius:0.1,center:Vec3::new(1.,2.,3.),alpha:1.,solid:false}.signifier_name(),'+json.dumps(case['name'])+');\n'
test+='println!("PASS: exact Rust production Sphere identity method; all6 original complete-show Debug names match");}\n'
(here/'name_validation.rs').write_text(test,encoding='utf8')
