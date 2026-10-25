from pathlib import Path
import json,hashlib,difflib,subprocess,re
ROOT=Path('D:/game');OUT=Path(__file__).resolve().parent
changes=[]
def change(path,transform):
    source=ROOT/path;raw=source.read_bytes() if source.exists() else None
    before=source.read_text(encoding='utf8') if source.exists() else ''
    base=OUT/'base'/path;base.parent.mkdir(parents=True,exist_ok=True)
    if base.exists():assert base.read_text(encoding='utf8')==before,'shared source changed: '+path
    elif raw is not None:base.write_bytes(raw)
    after=transform(before);target=OUT/'candidate'/path;target.parent.mkdir(parents=True,exist_ok=True);target.write_text(after,encoding='utf8',newline='\n')
    changes.append((path,before,raw,target,after))
core='tiny-glade/crates/country-core/src/'
def gcc(_):
    return (OUT/'glade_color_customization.rs').read_text(encoding='utf8')+'\n#[cfg(test)]\n#[path="glade_color_customization_native_oracle.rs"]\nmod native_oracle;\n'
change(core+'resources/glade_color_customization.rs',gcc)
change(core+'resources/walls/inner_wall_state.rs',lambda s:s+(OUT/'types_arch.rs').read_text(encoding='utf8'))
def rim(s):
    old='#[derive(Deserialize)]\nenum SavedFlatRimStyle {\n    Rim,\n    NoRim,\n}'
    assert old in s
    return s.replace(old,'type SavedFlatRimStyle = FlatRoofRimStyle;')+(OUT/'types_rim.rs').read_text(encoding='utf8')
change(core+'resources/roofs/roof_shape.rs',rim)
change(core+'resources/walls/decorator.rs',lambda s:s+(OUT/'types_railing.rs').read_text(encoding='utf8'))
def selection(s):
    s=s.replace('wall_style: settings.building_wall_style,','wall_style: settings.building_wall_style.0,')
    s=s.replace('building_wall_style: 14,','building_wall_style: crate::resources::walls::inner_wall_state::WallStyle(14),')
    return s
change(core+'resources/walls/color_selection.rs',selection)
def drawing(s):
    s=s.replace('if kind == 0 { settings.freehand_ty } else { 0 }','if kind == 0 { settings.freehand_ty.native_tag() } else { 0 }')
    s=s.replace('settings.fence.0','settings.fences.0')
    s=s.replace('settings.freehand_ty = 1;','settings.freehand_ty = crate::resources::walls::inner_wall_state::ArchType::ConstantMaxHeight;')
    return s
change(core+'systems/wall/draw_wall.rs',drawing)
def circle(s):
    s=s.replace('colors.flat_rim_style,','colors.flat_rim_style.native_tag(),')
    s=s.replace('colors.building_wall_style = 4;','colors.building_wall_style = crate::resources::walls::inner_wall_state::WallStyle(4);')
    s=s.replace('colors.flat_rim_style = 6;','colors.flat_rim_style = crate::resources::roofs::roof_shape::FlatRoofRimStyle::NoRim;')
    s=s.replace('assert_eq!(record[0x39], 6);','assert_eq!(record[0x39], 1);')
    return s
change(core+'systems/wall/shapes/circle/create_circle.rs',circle)
def rectangle(s):
    s=s.replace('let mut roof_style = colors.flat_rim_style;','let mut roof_style = colors.flat_rim_style.native_tag();')
    s=s.replace('colors.building_wall_style = 3;','colors.building_wall_style = crate::resources::walls::inner_wall_state::WallStyle(3);')
    s=s.replace('colors.flat_rim_style = 5;','colors.flat_rim_style = crate::resources::roofs::roof_shape::FlatRoofRimStyle::NoRim;')
    s=s.replace('assert_eq!(record[0x39], 5);','assert_eq!(record[0x39], 1);')
    return s
change(core+'systems/wall/shapes/rectangle/create_rectangle.rs',rectangle)
def load(s):
    s=s.replace('stairs: crate::resources::color_id::ColorId(100003),','stairs: crate::resources::glade_color_customization::StairCustomization::default(),')
    s=s.replace('building_wall_style: 1,','building_wall_style: crate::resources::walls::inner_wall_state::WallStyle::STONE_WITH_SKIRTS,')
    return s
change(core+'systems/save/load_game.rs',load)
fixture=json.loads((OUT/'original-gcc-full-fixtures.json').read_text(encoding='utf8'))
def oracle(_):
    s='''//! Independent original RNE outputs, not candidate-generated roundtrips.
use super::*;
#[test]
fn original_default_82_bytes_and_all_fifteen_fields() {
 let original=r#"'''+fixture['default']['json']+'''"#;
 assert_eq!(serde_json::to_string(&GladeColorCustomization::default()).unwrap(),original);
}
#[test]
fn all_192_original_serializer_cases_decode_to_the_proven_typed_fields() {
 let originals=&[
'''
    for row in fixture['cases']:
        s+=' r#"'+row['json']+'"#,\n'
    s+=''' ];
 for &original in originals {
  let value:GladeColorCustomization=serde_json::from_str(original).unwrap();
  assert_eq!(value.freeform_wall,ColorId(101));assert_eq!(value.building,ColorId(102));
  assert_eq!(value.roof,ColorId(103));assert_eq!(value.halftimber,ColorId(104));
  assert_eq!(value.lantern_light,ColorId(105));assert_eq!(value.fences,ColorId(106));
  assert_eq!(value.window_light,ColorId(107));assert_eq!(value.flag,ColorId(108));
  assert_eq!(value.gates,ColorId(109));assert_eq!(value.lantern_brightness,1.25);
  assert_eq!(value.stairs.stairs_color,ColorId(11));assert_eq!(value.stairs.ladder_color,ColorId(22));
  assert_eq!(value.stairs.magic_color,ColorId(33));
  assert_eq!(serde_json::to_string(&value).unwrap(),original);
 }
}
#[test]
fn obsolete_simplified_wire_is_rejected_and_present_values_are_never_defaulted() {
 let valid=serde_json::to_value(GladeColorCustomization::default()).unwrap();
 for (field,bad) in [("flag",serde_json::json!(true)),("stairs",serde_json::json!(100003)),
  ("building_wall_style",serde_json::json!(1)),("freehand_ty",serde_json::json!(0)),
  ("flat_rim_style",serde_json::json!(0)),("windows",serde_json::json!(true))] {
  let mut invalid=valid.clone();invalid[field]=bad;
  assert!(serde_json::from_value::<GladeColorCustomization>(invalid).is_err(),"{field}");
 }
 let mut invalid=valid.clone();invalid["stairs"]["railing_ty"]=serde_json::json!("Unknown");
 assert!(serde_json::from_value::<GladeColorCustomization>(invalid).is_err());
 let mut missing_fences=valid;let fences=missing_fences.as_object_mut().unwrap().remove("fences").unwrap();
 missing_fences["fence"]=fences;
 assert!(serde_json::from_value::<GladeColorCustomization>(missing_fences).is_err());
}
'''
    s+='''
#[test]
fn original_missing_field_policy_is_preserved_individually() {
 let original=serde_json::to_value(GladeColorCustomization::default()).unwrap();
'''
    for row in fixture['native_missing_fields']:
        field=row['missing_field'];nested=field in ['stairs_color','ladder_color','railing_ty','magic_color']
        s+=' { let mut value=original.clone();\n'
        expr='value["stairs"]' if nested else 'value'
        s+=f' {expr}.as_object_mut().unwrap().remove("{field}");\n'
        if row['original_result_tag']==1:s+=' assert!(serde_json::from_value::<GladeColorCustomization>(value).is_err());\n'
        else:
            s+=' let parsed=serde_json::from_value::<GladeColorCustomization>(value).unwrap();\n'
            s+=' assert_eq!(serde_json::to_string(&parsed).unwrap(),r#"'+row['native_result_json']+'"#);\n'
        s+=' }\n'
    s+='}\n'
    return s
change(core+'resources/glade_color_customization_native_oracle.rs',oracle)
subprocess.run(['C:/Users/liyu/.rustup/toolchains/stable-x86_64-pc-windows-msvc/bin/rustfmt.exe','--edition','2024','--config','skip_children=true',*[str(t) for _,_,_,t,_ in changes]],check=True)
rows=[];patch=''
for path,before,raw,target,unformatted in changes:
    # Retain untouched existing source formatting around narrow caller/type
    # hunks. rustfmt has parsed these exact inputs, so this loses no validation.
    if not path.endswith(('glade_color_customization.rs','glade_color_customization_native_oracle.rs')):
        target.write_text(unformatted,encoding='utf8',newline='\n')
    proposed=target.read_bytes();after=target.read_text(encoding='utf8')
    patch+=''.join(difflib.unified_diff(before.splitlines(True),after.splitlines(True),fromfile='a/'+path if raw is not None else '/dev/null',tofile='b/'+path))
    rows.append({'path':path,'base_sha256':hashlib.sha256(raw).hexdigest() if raw is not None else None,'proposed_sha256':hashlib.sha256(proposed).hexdigest()})
(OUT/'production-review.patch').write_text(patch,encoding='utf8',newline='\n')
(OUT/'candidate-checkpoint.json').write_text(json.dumps({'status':'candidate only; no production/Cargo; full original native serializer/default/visitor fixtures executed','files':rows},indent=2)+'\n')
print(json.dumps({'files':len(rows)}))
