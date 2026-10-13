import json
from pathlib import Path
OUT=Path(__file__).parent
data=json.loads((OUT/'native-prefab-fixtures.json').read_text(encoding='utf-8'))
code='''use super::*;
fn value(tag:u32,values:[u32;3])->Inheritable<[f32;3]> {
    match tag {0=>Inheritable::Inherit,1=>Inheritable::MissingDefault,2=>Inheritable::Value(values.map(f32::from_bits)),_=>unreachable!()}
}
fn mesh(tag:u8)->Inheritable<RtMesh> {match tag {0=>Inheritable::Inherit,1=>Inheritable::MissingDefault,2=>Inheritable::Value(RtMesh::Aabb),_=>unreachable!()}}
fn semantic(tag:u8)->Inheritable<RtMeshSemantic> {match tag {0=>Inheritable::Value(RtMeshSemantic::None),1=>Inheritable::Value(RtMeshSemantic::Canopy),2=>Inheritable::Inherit,3=>Inheritable::MissingDefault,_=>unreachable!()}}
fn make_rt(tags:[u32;4],bits:[[u32;3];4],m:u8,s:u8)->PrefabRtConfig {PrefabRtConfig {mesh:mesh(m),mesh_scale:value(tags[0],bits[0]),summer_albedo:value(tags[1],bits[1]),autumn_albedo:value(tags[2],bits[2]),winter_albedo:value(tags[3],bits[3]),semantic:semantic(s)}}
fn check_value(value:&Inheritable<[f32;3]>,tag:u32,bits:[u32;3]) {match tag {0=>assert!(matches!(value,Inheritable::Inherit)),1=>assert!(matches!(value,Inheritable::MissingDefault)),2=>assert_eq!(value.as_concrete().map(f32::to_bits),bits),_=>unreachable!()}}
#[test]
fn original_b_rt_option_defaults_and_parent_priority() {
'''
for row in data['rt']:
    def config(v):
        return f'make_rt({[f["tag"] for f in v["fields"]]},{[f["value_bits"] for f in v["fields"]]},{v["mesh"]},{v["semantic"]})'
    result=row['result'];parent='None' if row['parent'] is None else 'Some('+config(row['parent'])+')'
    code+='{ let mut child=Some('+config(row['child'])+');child.inherit_from(&'+parent+');let child=child.unwrap();\n'
    for name,field in zip(['mesh_scale','summer_albedo','autumn_albedo','winter_albedo'],result['fields']):
        code+=f'check_value(&child.{name},{field["tag"]},{field["value_bits"]});\n'
    code+=f'assert_eq!(std::mem::discriminant(&child.mesh),std::mem::discriminant(&mesh({result["mesh"]})));assert_eq!(std::mem::discriminant(&child.semantic),std::mem::discriminant(&semantic({result["semantic"]})));'
    if result['semantic']<=1:code+=f'assert_eq!(child.semantic.as_concrete(),semantic({result["semantic"]}).as_concrete());'
    code+='}\n'
code+='}\n#[test]\nfn original_b_audio_option_defaults_and_required_markers() {\n'
for row in data['audio']:
    tag=row['child_kind_tag'];scale=row['child_scale_tag'];shape=row['child_shape']
    trigger='Inheritable::Inherit' if tag==2 else 'Inheritable::MissingDefault' if tag==3 else f'Inheritable::Value(AudioTriggerConfig {{kind:Inheritable::{"Inherit" if tag==0 else "MissingDefault"},shape_scale:value({scale},[1073741824,1077936128,1082130432]),shape:'+(['Inheritable::Inherit','Inheritable::MissingDefault','Inheritable::Value(AudioTriggerShape::Aabb)'][shape])+'})'
    code+='{let mut audio=Some(PrefabAudioConfig {trigger:'+trigger+'});'
    if row['status']=='original_panic':code+='assert!(std::panic::catch_unwind(std::panic::AssertUnwindSafe(||audio.inherit_from(&None))).is_err());}\n';continue
    code+='audio.inherit_from(&None);let audio=audio.unwrap();let trigger=audio.trigger.as_concrete();'
    cap=row['result_kind_capacity']
    assert cap in [0x8000000000000000,0x8000000000000001]
    code+='assert!(matches!(trigger.kind,Inheritable::'+('Inherit' if cap==0x8000000000000000 else 'MissingDefault')+'));'
    code+=f'check_value(&trigger.shape_scale,{row["result_scale_tag"]},{row["result_scale_bits"]});'
    code+='assert!(matches!(trigger.shape,'+(['Inheritable::Inherit','Inheritable::MissingDefault','Inheritable::Value(AudioTriggerShape::Aabb)'][row['result_shape']])+'));}\n'
code+='}\n'
(OUT/'prefab_native_tests.rs').write_text(code,encoding='utf-8')
p=OUT/'prefab_default_tests.rs';base=p.read_text(encoding='utf-8').split('\nmod native_completion')[0]
p.write_text(base+'\nmod native_completion {include!("prefab_native_tests.rs");}\n',encoding='utf-8')
print('native RT cases',len(data['rt']),'native audio cases',len(data['audio']))
