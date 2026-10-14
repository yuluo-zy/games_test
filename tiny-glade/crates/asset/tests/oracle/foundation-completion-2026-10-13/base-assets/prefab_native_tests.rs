use super::*;
fn value(tag:u32,values:[u32;3])->Inheritable<[f32;3]> {
    match tag {0=>Inheritable::Inherit,1=>Inheritable::MissingDefault,2=>Inheritable::Value(values.map(f32::from_bits)),_=>unreachable!()}
}
fn mesh(tag:u8)->Inheritable<RtMesh> {match tag {0=>Inheritable::Inherit,1=>Inheritable::MissingDefault,2=>Inheritable::Value(RtMesh::Aabb),_=>unreachable!()}}
fn semantic(tag:u8)->Inheritable<RtMeshSemantic> {match tag {0=>Inheritable::Value(RtMeshSemantic::None),1=>Inheritable::Value(RtMeshSemantic::Canopy),2=>Inheritable::Inherit,3=>Inheritable::MissingDefault,_=>unreachable!()}}
fn make_rt(tags:[u32;4],bits:[[u32;3];4],m:u8,s:u8)->PrefabRtConfig {PrefabRtConfig {mesh:mesh(m),mesh_scale:value(tags[0],bits[0]),summer_albedo:value(tags[1],bits[1]),autumn_albedo:value(tags[2],bits[2]),winter_albedo:value(tags[3],bits[3]),semantic:semantic(s)}}
fn check_value(value:&Inheritable<[f32;3]>,tag:u32,bits:[u32;3]) {match tag {0=>assert!(matches!(value,Inheritable::Inherit)),1=>assert!(matches!(value,Inheritable::MissingDefault)),2=>assert_eq!(value.as_concrete().map(f32::to_bits),bits),_=>unreachable!()}}
#[test]
fn original_b_rt_option_defaults_and_parent_priority() {
{ let mut child=Some(make_rt([0, 0, 1, 2],[[1073741824, 1077936128, 1082130432], [1077936128, 1082130432, 1084227584], [1082130432, 1084227584, 1086324736], [1084227584, 1086324736, 1088421888]],1,3));child.inherit_from(&Some(make_rt([0, 1, 0, 1],[[1101004800, 1101529088, 1102053376], [1101529088, 1102053376, 1102577664], [1102053376, 1102577664, 1103101952], [1102577664, 1103101952, 1103626240]],0,1)));let child=child.unwrap();
check_value(&child.mesh_scale,2,[1065353216, 1065353216, 1065353216]);
check_value(&child.summer_albedo,1,[0, 0, 0]);
check_value(&child.autumn_albedo,1,[1082130432, 1084227584, 1086324736]);
check_value(&child.winter_albedo,2,[1084227584, 1086324736, 1088421888]);
assert_eq!(std::mem::discriminant(&child.mesh),std::mem::discriminant(&mesh(1)));assert_eq!(std::mem::discriminant(&child.semantic),std::mem::discriminant(&semantic(3)));}
{ let mut child=Some(make_rt([0, 0, 1, 2],[[1073741824, 1077936128, 1082130432], [1077936128, 1082130432, 1084227584], [1082130432, 1084227584, 1086324736], [1084227584, 1086324736, 1088421888]],1,3));child.inherit_from(&None);let child=child.unwrap();
check_value(&child.mesh_scale,2,[1065353216, 1065353216, 1065353216]);
check_value(&child.summer_albedo,2,[1065353216, 1065353216, 1065353216]);
check_value(&child.autumn_albedo,1,[1082130432, 1084227584, 1086324736]);
check_value(&child.winter_albedo,2,[1084227584, 1086324736, 1088421888]);
assert_eq!(std::mem::discriminant(&child.mesh),std::mem::discriminant(&mesh(1)));assert_eq!(std::mem::discriminant(&child.semantic),std::mem::discriminant(&semantic(3)));}
{ let mut child=Some(make_rt([2, 0, 1, 0],[[1073741824, 1077936128, 1082130432], [1077936128, 1082130432, 1084227584], [1082130432, 1084227584, 1086324736], [1084227584, 1086324736, 1088421888]],0,1));child.inherit_from(&Some(make_rt([2, 1, 1, 0],[[1101004800, 1101529088, 1102053376], [1101529088, 1102053376, 1102577664], [1102053376, 1102577664, 1103101952], [1102577664, 1103101952, 1103626240]],0,3)));let child=child.unwrap();
check_value(&child.mesh_scale,2,[1073741824, 1077936128, 1082130432]);
check_value(&child.summer_albedo,1,[0, 0, 0]);
check_value(&child.autumn_albedo,1,[1082130432, 1084227584, 1086324736]);
check_value(&child.winter_albedo,2,[1065353216, 1065353216, 1065353216]);
assert_eq!(std::mem::discriminant(&child.mesh),std::mem::discriminant(&mesh(1)));assert_eq!(std::mem::discriminant(&child.semantic),std::mem::discriminant(&semantic(1)));assert_eq!(child.semantic.as_concrete(),semantic(1).as_concrete());}
{ let mut child=Some(make_rt([2, 0, 1, 0],[[1073741824, 1077936128, 1082130432], [1077936128, 1082130432, 1084227584], [1082130432, 1084227584, 1086324736], [1084227584, 1086324736, 1088421888]],0,1));child.inherit_from(&None);let child=child.unwrap();
check_value(&child.mesh_scale,2,[1073741824, 1077936128, 1082130432]);
check_value(&child.summer_albedo,2,[1065353216, 1065353216, 1065353216]);
check_value(&child.autumn_albedo,1,[1082130432, 1084227584, 1086324736]);
check_value(&child.winter_albedo,2,[1065353216, 1065353216, 1065353216]);
assert_eq!(std::mem::discriminant(&child.mesh),std::mem::discriminant(&mesh(1)));assert_eq!(std::mem::discriminant(&child.semantic),std::mem::discriminant(&semantic(1)));assert_eq!(child.semantic.as_concrete(),semantic(1).as_concrete());}
{ let mut child=Some(make_rt([0, 0, 2, 0],[[1073741824, 1077936128, 1082130432], [1077936128, 1082130432, 1084227584], [1082130432, 1084227584, 1086324736], [1084227584, 1086324736, 1088421888]],2,0));child.inherit_from(&Some(make_rt([0, 1, 0, 2],[[1101004800, 1101529088, 1102053376], [1101529088, 1102053376, 1102577664], [1102053376, 1102577664, 1103101952], [1102577664, 1103101952, 1103626240]],2,0)));let child=child.unwrap();
check_value(&child.mesh_scale,2,[1065353216, 1065353216, 1065353216]);
check_value(&child.summer_albedo,1,[0, 0, 0]);
check_value(&child.autumn_albedo,2,[1082130432, 1084227584, 1086324736]);
check_value(&child.winter_albedo,2,[1102577664, 1103101952, 1103626240]);
assert_eq!(std::mem::discriminant(&child.mesh),std::mem::discriminant(&mesh(2)));assert_eq!(std::mem::discriminant(&child.semantic),std::mem::discriminant(&semantic(0)));assert_eq!(child.semantic.as_concrete(),semantic(0).as_concrete());}
{ let mut child=Some(make_rt([0, 0, 2, 0],[[1073741824, 1077936128, 1082130432], [1077936128, 1082130432, 1084227584], [1082130432, 1084227584, 1086324736], [1084227584, 1086324736, 1088421888]],2,0));child.inherit_from(&None);let child=child.unwrap();
check_value(&child.mesh_scale,2,[1065353216, 1065353216, 1065353216]);
check_value(&child.summer_albedo,2,[1065353216, 1065353216, 1065353216]);
check_value(&child.autumn_albedo,2,[1082130432, 1084227584, 1086324736]);
check_value(&child.winter_albedo,2,[1065353216, 1065353216, 1065353216]);
assert_eq!(std::mem::discriminant(&child.mesh),std::mem::discriminant(&mesh(2)));assert_eq!(std::mem::discriminant(&child.semantic),std::mem::discriminant(&semantic(0)));assert_eq!(child.semantic.as_concrete(),semantic(0).as_concrete());}
{ let mut child=Some(make_rt([2, 2, 1, 1],[[1073741824, 1077936128, 1082130432], [1077936128, 1082130432, 1084227584], [1082130432, 1084227584, 1086324736], [1084227584, 1086324736, 1088421888]],1,0));child.inherit_from(&Some(make_rt([0, 1, 0, 1],[[1101004800, 1101529088, 1102053376], [1101529088, 1102053376, 1102577664], [1102053376, 1102577664, 1103101952], [1102577664, 1103101952, 1103626240]],0,3)));let child=child.unwrap();
check_value(&child.mesh_scale,2,[1073741824, 1077936128, 1082130432]);
check_value(&child.summer_albedo,2,[1077936128, 1082130432, 1084227584]);
check_value(&child.autumn_albedo,1,[1082130432, 1084227584, 1086324736]);
check_value(&child.winter_albedo,1,[1084227584, 1086324736, 1088421888]);
assert_eq!(std::mem::discriminant(&child.mesh),std::mem::discriminant(&mesh(1)));assert_eq!(std::mem::discriminant(&child.semantic),std::mem::discriminant(&semantic(0)));assert_eq!(child.semantic.as_concrete(),semantic(0).as_concrete());}
{ let mut child=Some(make_rt([2, 2, 1, 1],[[1073741824, 1077936128, 1082130432], [1077936128, 1082130432, 1084227584], [1082130432, 1084227584, 1086324736], [1084227584, 1086324736, 1088421888]],1,0));child.inherit_from(&None);let child=child.unwrap();
check_value(&child.mesh_scale,2,[1073741824, 1077936128, 1082130432]);
check_value(&child.summer_albedo,2,[1077936128, 1082130432, 1084227584]);
check_value(&child.autumn_albedo,1,[1082130432, 1084227584, 1086324736]);
check_value(&child.winter_albedo,1,[1084227584, 1086324736, 1088421888]);
assert_eq!(std::mem::discriminant(&child.mesh),std::mem::discriminant(&mesh(1)));assert_eq!(std::mem::discriminant(&child.semantic),std::mem::discriminant(&semantic(0)));assert_eq!(child.semantic.as_concrete(),semantic(0).as_concrete());}
{ let mut child=Some(make_rt([0, 1, 1, 2],[[1073741824, 1077936128, 1082130432], [1077936128, 1082130432, 1084227584], [1082130432, 1084227584, 1086324736], [1084227584, 1086324736, 1088421888]],1,0));child.inherit_from(&Some(make_rt([2, 2, 1, 0],[[1101004800, 1101529088, 1102053376], [1101529088, 1102053376, 1102577664], [1102053376, 1102577664, 1103101952], [1102577664, 1103101952, 1103626240]],0,2)));let child=child.unwrap();
check_value(&child.mesh_scale,2,[1101004800, 1101529088, 1102053376]);
check_value(&child.summer_albedo,1,[1077936128, 1082130432, 1084227584]);
check_value(&child.autumn_albedo,1,[1082130432, 1084227584, 1086324736]);
check_value(&child.winter_albedo,2,[1084227584, 1086324736, 1088421888]);
assert_eq!(std::mem::discriminant(&child.mesh),std::mem::discriminant(&mesh(1)));assert_eq!(std::mem::discriminant(&child.semantic),std::mem::discriminant(&semantic(0)));assert_eq!(child.semantic.as_concrete(),semantic(0).as_concrete());}
{ let mut child=Some(make_rt([0, 1, 1, 2],[[1073741824, 1077936128, 1082130432], [1077936128, 1082130432, 1084227584], [1082130432, 1084227584, 1086324736], [1084227584, 1086324736, 1088421888]],1,0));child.inherit_from(&None);let child=child.unwrap();
check_value(&child.mesh_scale,2,[1065353216, 1065353216, 1065353216]);
check_value(&child.summer_albedo,1,[1077936128, 1082130432, 1084227584]);
check_value(&child.autumn_albedo,1,[1082130432, 1084227584, 1086324736]);
check_value(&child.winter_albedo,2,[1084227584, 1086324736, 1088421888]);
assert_eq!(std::mem::discriminant(&child.mesh),std::mem::discriminant(&mesh(1)));assert_eq!(std::mem::discriminant(&child.semantic),std::mem::discriminant(&semantic(0)));assert_eq!(child.semantic.as_concrete(),semantic(0).as_concrete());}
{ let mut child=Some(make_rt([1, 2, 0, 1],[[1073741824, 1077936128, 1082130432], [1077936128, 1082130432, 1084227584], [1082130432, 1084227584, 1086324736], [1084227584, 1086324736, 1088421888]],2,0));child.inherit_from(&Some(make_rt([2, 2, 0, 1],[[1101004800, 1101529088, 1102053376], [1101529088, 1102053376, 1102577664], [1102053376, 1102577664, 1103101952], [1102577664, 1103101952, 1103626240]],0,1)));let child=child.unwrap();
check_value(&child.mesh_scale,1,[1073741824, 1077936128, 1082130432]);
check_value(&child.summer_albedo,2,[1077936128, 1082130432, 1084227584]);
check_value(&child.autumn_albedo,2,[1065353216, 1065353216, 1065353216]);
check_value(&child.winter_albedo,1,[1084227584, 1086324736, 1088421888]);
assert_eq!(std::mem::discriminant(&child.mesh),std::mem::discriminant(&mesh(2)));assert_eq!(std::mem::discriminant(&child.semantic),std::mem::discriminant(&semantic(0)));assert_eq!(child.semantic.as_concrete(),semantic(0).as_concrete());}
{ let mut child=Some(make_rt([1, 2, 0, 1],[[1073741824, 1077936128, 1082130432], [1077936128, 1082130432, 1084227584], [1082130432, 1084227584, 1086324736], [1084227584, 1086324736, 1088421888]],2,0));child.inherit_from(&None);let child=child.unwrap();
check_value(&child.mesh_scale,1,[1073741824, 1077936128, 1082130432]);
check_value(&child.summer_albedo,2,[1077936128, 1082130432, 1084227584]);
check_value(&child.autumn_albedo,2,[1065353216, 1065353216, 1065353216]);
check_value(&child.winter_albedo,1,[1084227584, 1086324736, 1088421888]);
assert_eq!(std::mem::discriminant(&child.mesh),std::mem::discriminant(&mesh(2)));assert_eq!(std::mem::discriminant(&child.semantic),std::mem::discriminant(&semantic(0)));assert_eq!(child.semantic.as_concrete(),semantic(0).as_concrete());}
{ let mut child=Some(make_rt([2, 0, 2, 1],[[1073741824, 1077936128, 1082130432], [1077936128, 1082130432, 1084227584], [1082130432, 1084227584, 1086324736], [1084227584, 1086324736, 1088421888]],1,3));child.inherit_from(&Some(make_rt([2, 1, 2, 1],[[1101004800, 1101529088, 1102053376], [1101529088, 1102053376, 1102577664], [1102053376, 1102577664, 1103101952], [1102577664, 1103101952, 1103626240]],0,2)));let child=child.unwrap();
check_value(&child.mesh_scale,2,[1073741824, 1077936128, 1082130432]);
check_value(&child.summer_albedo,1,[1101004800, 1101529088, 1102053376]);
check_value(&child.autumn_albedo,2,[1082130432, 1084227584, 1086324736]);
check_value(&child.winter_albedo,1,[1084227584, 1086324736, 1088421888]);
assert_eq!(std::mem::discriminant(&child.mesh),std::mem::discriminant(&mesh(1)));assert_eq!(std::mem::discriminant(&child.semantic),std::mem::discriminant(&semantic(3)));}
{ let mut child=Some(make_rt([2, 0, 2, 1],[[1073741824, 1077936128, 1082130432], [1077936128, 1082130432, 1084227584], [1082130432, 1084227584, 1086324736], [1084227584, 1086324736, 1088421888]],1,3));child.inherit_from(&None);let child=child.unwrap();
check_value(&child.mesh_scale,2,[1073741824, 1077936128, 1082130432]);
check_value(&child.summer_albedo,2,[1065353216, 1065353216, 1065353216]);
check_value(&child.autumn_albedo,2,[1082130432, 1084227584, 1086324736]);
check_value(&child.winter_albedo,1,[1084227584, 1086324736, 1088421888]);
assert_eq!(std::mem::discriminant(&child.mesh),std::mem::discriminant(&mesh(1)));assert_eq!(std::mem::discriminant(&child.semantic),std::mem::discriminant(&semantic(3)));}
{ let mut child=Some(make_rt([2, 0, 0, 2],[[1073741824, 1077936128, 1082130432], [1077936128, 1082130432, 1084227584], [1082130432, 1084227584, 1086324736], [1084227584, 1086324736, 1088421888]],1,0));child.inherit_from(&Some(make_rt([1, 2, 0, 2],[[1101004800, 1101529088, 1102053376], [1101529088, 1102053376, 1102577664], [1102053376, 1102577664, 1103101952], [1102577664, 1103101952, 1103626240]],1,1)));let child=child.unwrap();
check_value(&child.mesh_scale,2,[1073741824, 1077936128, 1082130432]);
check_value(&child.summer_albedo,2,[1101529088, 1102053376, 1102577664]);
check_value(&child.autumn_albedo,2,[1065353216, 1065353216, 1065353216]);
check_value(&child.winter_albedo,2,[1084227584, 1086324736, 1088421888]);
assert_eq!(std::mem::discriminant(&child.mesh),std::mem::discriminant(&mesh(1)));assert_eq!(std::mem::discriminant(&child.semantic),std::mem::discriminant(&semantic(0)));assert_eq!(child.semantic.as_concrete(),semantic(0).as_concrete());}
{ let mut child=Some(make_rt([2, 0, 0, 2],[[1073741824, 1077936128, 1082130432], [1077936128, 1082130432, 1084227584], [1082130432, 1084227584, 1086324736], [1084227584, 1086324736, 1088421888]],1,0));child.inherit_from(&None);let child=child.unwrap();
check_value(&child.mesh_scale,2,[1073741824, 1077936128, 1082130432]);
check_value(&child.summer_albedo,2,[1065353216, 1065353216, 1065353216]);
check_value(&child.autumn_albedo,2,[1065353216, 1065353216, 1065353216]);
check_value(&child.winter_albedo,2,[1084227584, 1086324736, 1088421888]);
assert_eq!(std::mem::discriminant(&child.mesh),std::mem::discriminant(&mesh(1)));assert_eq!(std::mem::discriminant(&child.semantic),std::mem::discriminant(&semantic(0)));assert_eq!(child.semantic.as_concrete(),semantic(0).as_concrete());}
{ let mut child=Some(make_rt([1, 0, 2, 0],[[1073741824, 1077936128, 1082130432], [1077936128, 1082130432, 1084227584], [1082130432, 1084227584, 1086324736], [1084227584, 1086324736, 1088421888]],0,0));child.inherit_from(&Some(make_rt([2, 2, 0, 2],[[1101004800, 1101529088, 1102053376], [1101529088, 1102053376, 1102577664], [1102053376, 1102577664, 1103101952], [1102577664, 1103101952, 1103626240]],1,3)));let child=child.unwrap();
check_value(&child.mesh_scale,1,[1073741824, 1077936128, 1082130432]);
check_value(&child.summer_albedo,2,[1101529088, 1102053376, 1102577664]);
check_value(&child.autumn_albedo,2,[1082130432, 1084227584, 1086324736]);
check_value(&child.winter_albedo,2,[1102577664, 1103101952, 1103626240]);
assert_eq!(std::mem::discriminant(&child.mesh),std::mem::discriminant(&mesh(1)));assert_eq!(std::mem::discriminant(&child.semantic),std::mem::discriminant(&semantic(0)));assert_eq!(child.semantic.as_concrete(),semantic(0).as_concrete());}
{ let mut child=Some(make_rt([1, 0, 2, 0],[[1073741824, 1077936128, 1082130432], [1077936128, 1082130432, 1084227584], [1082130432, 1084227584, 1086324736], [1084227584, 1086324736, 1088421888]],0,0));child.inherit_from(&None);let child=child.unwrap();
check_value(&child.mesh_scale,1,[1073741824, 1077936128, 1082130432]);
check_value(&child.summer_albedo,2,[1065353216, 1065353216, 1065353216]);
check_value(&child.autumn_albedo,2,[1082130432, 1084227584, 1086324736]);
check_value(&child.winter_albedo,2,[1065353216, 1065353216, 1065353216]);
assert_eq!(std::mem::discriminant(&child.mesh),std::mem::discriminant(&mesh(1)));assert_eq!(std::mem::discriminant(&child.semantic),std::mem::discriminant(&semantic(0)));assert_eq!(child.semantic.as_concrete(),semantic(0).as_concrete());}
{ let mut child=Some(make_rt([1, 0, 1, 2],[[1073741824, 1077936128, 1082130432], [1077936128, 1082130432, 1084227584], [1082130432, 1084227584, 1086324736], [1084227584, 1086324736, 1088421888]],1,3));child.inherit_from(&Some(make_rt([2, 2, 0, 0],[[1101004800, 1101529088, 1102053376], [1101529088, 1102053376, 1102577664], [1102053376, 1102577664, 1103101952], [1102577664, 1103101952, 1103626240]],0,3)));let child=child.unwrap();
check_value(&child.mesh_scale,1,[1073741824, 1077936128, 1082130432]);
check_value(&child.summer_albedo,2,[1101529088, 1102053376, 1102577664]);
check_value(&child.autumn_albedo,1,[1082130432, 1084227584, 1086324736]);
check_value(&child.winter_albedo,2,[1084227584, 1086324736, 1088421888]);
assert_eq!(std::mem::discriminant(&child.mesh),std::mem::discriminant(&mesh(1)));assert_eq!(std::mem::discriminant(&child.semantic),std::mem::discriminant(&semantic(3)));}
{ let mut child=Some(make_rt([1, 0, 1, 2],[[1073741824, 1077936128, 1082130432], [1077936128, 1082130432, 1084227584], [1082130432, 1084227584, 1086324736], [1084227584, 1086324736, 1088421888]],1,3));child.inherit_from(&None);let child=child.unwrap();
check_value(&child.mesh_scale,1,[1073741824, 1077936128, 1082130432]);
check_value(&child.summer_albedo,2,[1065353216, 1065353216, 1065353216]);
check_value(&child.autumn_albedo,1,[1082130432, 1084227584, 1086324736]);
check_value(&child.winter_albedo,2,[1084227584, 1086324736, 1088421888]);
assert_eq!(std::mem::discriminant(&child.mesh),std::mem::discriminant(&mesh(1)));assert_eq!(std::mem::discriminant(&child.semantic),std::mem::discriminant(&semantic(3)));}
{ let mut child=Some(make_rt([0, 1, 2, 0],[[1073741824, 1077936128, 1082130432], [1077936128, 1082130432, 1084227584], [1082130432, 1084227584, 1086324736], [1084227584, 1086324736, 1088421888]],2,1));child.inherit_from(&Some(make_rt([1, 2, 0, 0],[[1101004800, 1101529088, 1102053376], [1101529088, 1102053376, 1102577664], [1102053376, 1102577664, 1103101952], [1102577664, 1103101952, 1103626240]],2,1)));let child=child.unwrap();
check_value(&child.mesh_scale,1,[1101529088, 1102053376, 1102577664]);
check_value(&child.summer_albedo,1,[1077936128, 1082130432, 1084227584]);
check_value(&child.autumn_albedo,2,[1082130432, 1084227584, 1086324736]);
check_value(&child.winter_albedo,2,[1065353216, 1065353216, 1065353216]);
assert_eq!(std::mem::discriminant(&child.mesh),std::mem::discriminant(&mesh(2)));assert_eq!(std::mem::discriminant(&child.semantic),std::mem::discriminant(&semantic(1)));assert_eq!(child.semantic.as_concrete(),semantic(1).as_concrete());}
{ let mut child=Some(make_rt([0, 1, 2, 0],[[1073741824, 1077936128, 1082130432], [1077936128, 1082130432, 1084227584], [1082130432, 1084227584, 1086324736], [1084227584, 1086324736, 1088421888]],2,1));child.inherit_from(&None);let child=child.unwrap();
check_value(&child.mesh_scale,2,[1065353216, 1065353216, 1065353216]);
check_value(&child.summer_albedo,1,[1077936128, 1082130432, 1084227584]);
check_value(&child.autumn_albedo,2,[1082130432, 1084227584, 1086324736]);
check_value(&child.winter_albedo,2,[1065353216, 1065353216, 1065353216]);
assert_eq!(std::mem::discriminant(&child.mesh),std::mem::discriminant(&mesh(2)));assert_eq!(std::mem::discriminant(&child.semantic),std::mem::discriminant(&semantic(1)));assert_eq!(child.semantic.as_concrete(),semantic(1).as_concrete());}
{ let mut child=Some(make_rt([0, 0, 0, 1],[[1073741824, 1077936128, 1082130432], [1077936128, 1082130432, 1084227584], [1082130432, 1084227584, 1086324736], [1084227584, 1086324736, 1088421888]],1,3));child.inherit_from(&Some(make_rt([0, 2, 2, 2],[[1101004800, 1101529088, 1102053376], [1101529088, 1102053376, 1102577664], [1102053376, 1102577664, 1103101952], [1102577664, 1103101952, 1103626240]],2,3)));let child=child.unwrap();
check_value(&child.mesh_scale,2,[1065353216, 1065353216, 1065353216]);
check_value(&child.summer_albedo,2,[1101529088, 1102053376, 1102577664]);
check_value(&child.autumn_albedo,2,[1102053376, 1102577664, 1103101952]);
check_value(&child.winter_albedo,1,[1084227584, 1086324736, 1088421888]);
assert_eq!(std::mem::discriminant(&child.mesh),std::mem::discriminant(&mesh(1)));assert_eq!(std::mem::discriminant(&child.semantic),std::mem::discriminant(&semantic(3)));}
{ let mut child=Some(make_rt([0, 0, 0, 1],[[1073741824, 1077936128, 1082130432], [1077936128, 1082130432, 1084227584], [1082130432, 1084227584, 1086324736], [1084227584, 1086324736, 1088421888]],1,3));child.inherit_from(&None);let child=child.unwrap();
check_value(&child.mesh_scale,2,[1065353216, 1065353216, 1065353216]);
check_value(&child.summer_albedo,2,[1065353216, 1065353216, 1065353216]);
check_value(&child.autumn_albedo,2,[1065353216, 1065353216, 1065353216]);
check_value(&child.winter_albedo,1,[1084227584, 1086324736, 1088421888]);
assert_eq!(std::mem::discriminant(&child.mesh),std::mem::discriminant(&mesh(1)));assert_eq!(std::mem::discriminant(&child.semantic),std::mem::discriminant(&semantic(3)));}
{ let mut child=Some(make_rt([0, 2, 1, 0],[[1073741824, 1077936128, 1082130432], [1077936128, 1082130432, 1084227584], [1082130432, 1084227584, 1086324736], [1084227584, 1086324736, 1088421888]],0,0));child.inherit_from(&Some(make_rt([0, 0, 2, 0],[[1101004800, 1101529088, 1102053376], [1101529088, 1102053376, 1102577664], [1102053376, 1102577664, 1103101952], [1102577664, 1103101952, 1103626240]],0,3)));let child=child.unwrap();
check_value(&child.mesh_scale,2,[1065353216, 1065353216, 1065353216]);
check_value(&child.summer_albedo,2,[1077936128, 1082130432, 1084227584]);
check_value(&child.autumn_albedo,1,[1082130432, 1084227584, 1086324736]);
check_value(&child.winter_albedo,2,[1065353216, 1065353216, 1065353216]);
assert_eq!(std::mem::discriminant(&child.mesh),std::mem::discriminant(&mesh(1)));assert_eq!(std::mem::discriminant(&child.semantic),std::mem::discriminant(&semantic(0)));assert_eq!(child.semantic.as_concrete(),semantic(0).as_concrete());}
{ let mut child=Some(make_rt([0, 2, 1, 0],[[1073741824, 1077936128, 1082130432], [1077936128, 1082130432, 1084227584], [1082130432, 1084227584, 1086324736], [1084227584, 1086324736, 1088421888]],0,0));child.inherit_from(&None);let child=child.unwrap();
check_value(&child.mesh_scale,2,[1065353216, 1065353216, 1065353216]);
check_value(&child.summer_albedo,2,[1077936128, 1082130432, 1084227584]);
check_value(&child.autumn_albedo,1,[1082130432, 1084227584, 1086324736]);
check_value(&child.winter_albedo,2,[1065353216, 1065353216, 1065353216]);
assert_eq!(std::mem::discriminant(&child.mesh),std::mem::discriminant(&mesh(1)));assert_eq!(std::mem::discriminant(&child.semantic),std::mem::discriminant(&semantic(0)));assert_eq!(child.semantic.as_concrete(),semantic(0).as_concrete());}
{ let mut child=Some(make_rt([2, 2, 2, 1],[[1073741824, 1077936128, 1082130432], [1077936128, 1082130432, 1084227584], [1082130432, 1084227584, 1086324736], [1084227584, 1086324736, 1088421888]],2,2));child.inherit_from(&Some(make_rt([2, 0, 0, 0],[[1101004800, 1101529088, 1102053376], [1101529088, 1102053376, 1102577664], [1102053376, 1102577664, 1103101952], [1102577664, 1103101952, 1103626240]],2,2)));let child=child.unwrap();
check_value(&child.mesh_scale,2,[1073741824, 1077936128, 1082130432]);
check_value(&child.summer_albedo,2,[1077936128, 1082130432, 1084227584]);
check_value(&child.autumn_albedo,2,[1082130432, 1084227584, 1086324736]);
check_value(&child.winter_albedo,1,[1084227584, 1086324736, 1088421888]);
assert_eq!(std::mem::discriminant(&child.mesh),std::mem::discriminant(&mesh(2)));assert_eq!(std::mem::discriminant(&child.semantic),std::mem::discriminant(&semantic(0)));assert_eq!(child.semantic.as_concrete(),semantic(0).as_concrete());}
{ let mut child=Some(make_rt([2, 2, 2, 1],[[1073741824, 1077936128, 1082130432], [1077936128, 1082130432, 1084227584], [1082130432, 1084227584, 1086324736], [1084227584, 1086324736, 1088421888]],2,2));child.inherit_from(&None);let child=child.unwrap();
check_value(&child.mesh_scale,2,[1073741824, 1077936128, 1082130432]);
check_value(&child.summer_albedo,2,[1077936128, 1082130432, 1084227584]);
check_value(&child.autumn_albedo,2,[1082130432, 1084227584, 1086324736]);
check_value(&child.winter_albedo,1,[1084227584, 1086324736, 1088421888]);
assert_eq!(std::mem::discriminant(&child.mesh),std::mem::discriminant(&mesh(2)));assert_eq!(std::mem::discriminant(&child.semantic),std::mem::discriminant(&semantic(0)));assert_eq!(child.semantic.as_concrete(),semantic(0).as_concrete());}
{ let mut child=Some(make_rt([1, 0, 2, 2],[[1073741824, 1077936128, 1082130432], [1077936128, 1082130432, 1084227584], [1082130432, 1084227584, 1086324736], [1084227584, 1086324736, 1088421888]],2,0));child.inherit_from(&Some(make_rt([2, 1, 0, 2],[[1101004800, 1101529088, 1102053376], [1101529088, 1102053376, 1102577664], [1102053376, 1102577664, 1103101952], [1102577664, 1103101952, 1103626240]],0,1)));let child=child.unwrap();
check_value(&child.mesh_scale,1,[1073741824, 1077936128, 1082130432]);
check_value(&child.summer_albedo,1,[1102053376, 1102577664, 1103101952]);
check_value(&child.autumn_albedo,2,[1082130432, 1084227584, 1086324736]);
check_value(&child.winter_albedo,2,[1084227584, 1086324736, 1088421888]);
assert_eq!(std::mem::discriminant(&child.mesh),std::mem::discriminant(&mesh(2)));assert_eq!(std::mem::discriminant(&child.semantic),std::mem::discriminant(&semantic(0)));assert_eq!(child.semantic.as_concrete(),semantic(0).as_concrete());}
{ let mut child=Some(make_rt([1, 0, 2, 2],[[1073741824, 1077936128, 1082130432], [1077936128, 1082130432, 1084227584], [1082130432, 1084227584, 1086324736], [1084227584, 1086324736, 1088421888]],2,0));child.inherit_from(&None);let child=child.unwrap();
check_value(&child.mesh_scale,1,[1073741824, 1077936128, 1082130432]);
check_value(&child.summer_albedo,2,[1065353216, 1065353216, 1065353216]);
check_value(&child.autumn_albedo,2,[1082130432, 1084227584, 1086324736]);
check_value(&child.winter_albedo,2,[1084227584, 1086324736, 1088421888]);
assert_eq!(std::mem::discriminant(&child.mesh),std::mem::discriminant(&mesh(2)));assert_eq!(std::mem::discriminant(&child.semantic),std::mem::discriminant(&semantic(0)));assert_eq!(child.semantic.as_concrete(),semantic(0).as_concrete());}
{ let mut child=Some(make_rt([2, 2, 0, 1],[[1073741824, 1077936128, 1082130432], [1077936128, 1082130432, 1084227584], [1082130432, 1084227584, 1086324736], [1084227584, 1086324736, 1088421888]],0,2));child.inherit_from(&Some(make_rt([2, 0, 1, 0],[[1101004800, 1101529088, 1102053376], [1101529088, 1102053376, 1102577664], [1102053376, 1102577664, 1103101952], [1102577664, 1103101952, 1103626240]],0,1)));let child=child.unwrap();
check_value(&child.mesh_scale,2,[1073741824, 1077936128, 1082130432]);
check_value(&child.summer_albedo,2,[1077936128, 1082130432, 1084227584]);
check_value(&child.autumn_albedo,1,[1102053376, 1102577664, 1103101952]);
check_value(&child.winter_albedo,1,[1084227584, 1086324736, 1088421888]);
assert_eq!(std::mem::discriminant(&child.mesh),std::mem::discriminant(&mesh(1)));assert_eq!(std::mem::discriminant(&child.semantic),std::mem::discriminant(&semantic(1)));assert_eq!(child.semantic.as_concrete(),semantic(1).as_concrete());}
{ let mut child=Some(make_rt([2, 2, 0, 1],[[1073741824, 1077936128, 1082130432], [1077936128, 1082130432, 1084227584], [1082130432, 1084227584, 1086324736], [1084227584, 1086324736, 1088421888]],0,2));child.inherit_from(&None);let child=child.unwrap();
check_value(&child.mesh_scale,2,[1073741824, 1077936128, 1082130432]);
check_value(&child.summer_albedo,2,[1077936128, 1082130432, 1084227584]);
check_value(&child.autumn_albedo,2,[1065353216, 1065353216, 1065353216]);
check_value(&child.winter_albedo,1,[1084227584, 1086324736, 1088421888]);
assert_eq!(std::mem::discriminant(&child.mesh),std::mem::discriminant(&mesh(1)));assert_eq!(std::mem::discriminant(&child.semantic),std::mem::discriminant(&semantic(0)));assert_eq!(child.semantic.as_concrete(),semantic(0).as_concrete());}
{ let mut child=Some(make_rt([0, 0, 0, 2],[[1073741824, 1077936128, 1082130432], [1077936128, 1082130432, 1084227584], [1082130432, 1084227584, 1086324736], [1084227584, 1086324736, 1088421888]],2,0));child.inherit_from(&Some(make_rt([1, 0, 0, 1],[[1101004800, 1101529088, 1102053376], [1101529088, 1102053376, 1102577664], [1102053376, 1102577664, 1103101952], [1102577664, 1103101952, 1103626240]],1,0)));let child=child.unwrap();
check_value(&child.mesh_scale,1,[1102053376, 1102577664, 1103101952]);
check_value(&child.summer_albedo,2,[1065353216, 1065353216, 1065353216]);
check_value(&child.autumn_albedo,2,[1065353216, 1065353216, 1065353216]);
check_value(&child.winter_albedo,2,[1084227584, 1086324736, 1088421888]);
assert_eq!(std::mem::discriminant(&child.mesh),std::mem::discriminant(&mesh(2)));assert_eq!(std::mem::discriminant(&child.semantic),std::mem::discriminant(&semantic(0)));assert_eq!(child.semantic.as_concrete(),semantic(0).as_concrete());}
{ let mut child=Some(make_rt([0, 0, 0, 2],[[1073741824, 1077936128, 1082130432], [1077936128, 1082130432, 1084227584], [1082130432, 1084227584, 1086324736], [1084227584, 1086324736, 1088421888]],2,0));child.inherit_from(&None);let child=child.unwrap();
check_value(&child.mesh_scale,2,[1065353216, 1065353216, 1065353216]);
check_value(&child.summer_albedo,2,[1065353216, 1065353216, 1065353216]);
check_value(&child.autumn_albedo,2,[1065353216, 1065353216, 1065353216]);
check_value(&child.winter_albedo,2,[1084227584, 1086324736, 1088421888]);
assert_eq!(std::mem::discriminant(&child.mesh),std::mem::discriminant(&mesh(2)));assert_eq!(std::mem::discriminant(&child.semantic),std::mem::discriminant(&semantic(0)));assert_eq!(child.semantic.as_concrete(),semantic(0).as_concrete());}
{ let mut child=Some(make_rt([1, 2, 2, 0],[[1073741824, 1077936128, 1082130432], [1077936128, 1082130432, 1084227584], [1082130432, 1084227584, 1086324736], [1084227584, 1086324736, 1088421888]],0,2));child.inherit_from(&Some(make_rt([0, 1, 2, 0],[[1101004800, 1101529088, 1102053376], [1101529088, 1102053376, 1102577664], [1102053376, 1102577664, 1103101952], [1102577664, 1103101952, 1103626240]],0,0)));let child=child.unwrap();
check_value(&child.mesh_scale,1,[1073741824, 1077936128, 1082130432]);
check_value(&child.summer_albedo,2,[1077936128, 1082130432, 1084227584]);
check_value(&child.autumn_albedo,2,[1082130432, 1084227584, 1086324736]);
check_value(&child.winter_albedo,2,[1065353216, 1065353216, 1065353216]);
assert_eq!(std::mem::discriminant(&child.mesh),std::mem::discriminant(&mesh(1)));assert_eq!(std::mem::discriminant(&child.semantic),std::mem::discriminant(&semantic(0)));assert_eq!(child.semantic.as_concrete(),semantic(0).as_concrete());}
{ let mut child=Some(make_rt([1, 2, 2, 0],[[1073741824, 1077936128, 1082130432], [1077936128, 1082130432, 1084227584], [1082130432, 1084227584, 1086324736], [1084227584, 1086324736, 1088421888]],0,2));child.inherit_from(&None);let child=child.unwrap();
check_value(&child.mesh_scale,1,[1073741824, 1077936128, 1082130432]);
check_value(&child.summer_albedo,2,[1077936128, 1082130432, 1084227584]);
check_value(&child.autumn_albedo,2,[1082130432, 1084227584, 1086324736]);
check_value(&child.winter_albedo,2,[1065353216, 1065353216, 1065353216]);
assert_eq!(std::mem::discriminant(&child.mesh),std::mem::discriminant(&mesh(1)));assert_eq!(std::mem::discriminant(&child.semantic),std::mem::discriminant(&semantic(0)));assert_eq!(child.semantic.as_concrete(),semantic(0).as_concrete());}
{ let mut child=Some(make_rt([0, 0, 2, 0],[[1073741824, 1077936128, 1082130432], [1077936128, 1082130432, 1084227584], [1082130432, 1084227584, 1086324736], [1084227584, 1086324736, 1088421888]],2,1));child.inherit_from(&Some(make_rt([0, 1, 1, 1],[[1101004800, 1101529088, 1102053376], [1101529088, 1102053376, 1102577664], [1102053376, 1102577664, 1103101952], [1102577664, 1103101952, 1103626240]],1,0)));let child=child.unwrap();
check_value(&child.mesh_scale,2,[1065353216, 1065353216, 1065353216]);
check_value(&child.summer_albedo,1,[1102053376, 1102577664, 1103101952]);
check_value(&child.autumn_albedo,2,[1082130432, 1084227584, 1086324736]);
check_value(&child.winter_albedo,1,[1102053376, 1102577664, 1103101952]);
assert_eq!(std::mem::discriminant(&child.mesh),std::mem::discriminant(&mesh(2)));assert_eq!(std::mem::discriminant(&child.semantic),std::mem::discriminant(&semantic(1)));assert_eq!(child.semantic.as_concrete(),semantic(1).as_concrete());}
{ let mut child=Some(make_rt([0, 0, 2, 0],[[1073741824, 1077936128, 1082130432], [1077936128, 1082130432, 1084227584], [1082130432, 1084227584, 1086324736], [1084227584, 1086324736, 1088421888]],2,1));child.inherit_from(&None);let child=child.unwrap();
check_value(&child.mesh_scale,2,[1065353216, 1065353216, 1065353216]);
check_value(&child.summer_albedo,2,[1065353216, 1065353216, 1065353216]);
check_value(&child.autumn_albedo,2,[1082130432, 1084227584, 1086324736]);
check_value(&child.winter_albedo,2,[1065353216, 1065353216, 1065353216]);
assert_eq!(std::mem::discriminant(&child.mesh),std::mem::discriminant(&mesh(2)));assert_eq!(std::mem::discriminant(&child.semantic),std::mem::discriminant(&semantic(1)));assert_eq!(child.semantic.as_concrete(),semantic(1).as_concrete());}
{ let mut child=Some(make_rt([1, 1, 2, 1],[[1073741824, 1077936128, 1082130432], [1077936128, 1082130432, 1084227584], [1082130432, 1084227584, 1086324736], [1084227584, 1086324736, 1088421888]],2,3));child.inherit_from(&Some(make_rt([2, 0, 2, 1],[[1101004800, 1101529088, 1102053376], [1101529088, 1102053376, 1102577664], [1102053376, 1102577664, 1103101952], [1102577664, 1103101952, 1103626240]],2,3)));let child=child.unwrap();
check_value(&child.mesh_scale,1,[1073741824, 1077936128, 1082130432]);
check_value(&child.summer_albedo,1,[1077936128, 1082130432, 1084227584]);
check_value(&child.autumn_albedo,2,[1082130432, 1084227584, 1086324736]);
check_value(&child.winter_albedo,1,[1084227584, 1086324736, 1088421888]);
assert_eq!(std::mem::discriminant(&child.mesh),std::mem::discriminant(&mesh(2)));assert_eq!(std::mem::discriminant(&child.semantic),std::mem::discriminant(&semantic(3)));}
{ let mut child=Some(make_rt([1, 1, 2, 1],[[1073741824, 1077936128, 1082130432], [1077936128, 1082130432, 1084227584], [1082130432, 1084227584, 1086324736], [1084227584, 1086324736, 1088421888]],2,3));child.inherit_from(&None);let child=child.unwrap();
check_value(&child.mesh_scale,1,[1073741824, 1077936128, 1082130432]);
check_value(&child.summer_albedo,1,[1077936128, 1082130432, 1084227584]);
check_value(&child.autumn_albedo,2,[1082130432, 1084227584, 1086324736]);
check_value(&child.winter_albedo,1,[1084227584, 1086324736, 1088421888]);
assert_eq!(std::mem::discriminant(&child.mesh),std::mem::discriminant(&mesh(2)));assert_eq!(std::mem::discriminant(&child.semantic),std::mem::discriminant(&semantic(3)));}
{ let mut child=Some(make_rt([0, 0, 2, 0],[[1073741824, 1077936128, 1082130432], [1077936128, 1082130432, 1084227584], [1082130432, 1084227584, 1086324736], [1084227584, 1086324736, 1088421888]],1,3));child.inherit_from(&Some(make_rt([2, 1, 2, 2],[[1101004800, 1101529088, 1102053376], [1101529088, 1102053376, 1102577664], [1102053376, 1102577664, 1103101952], [1102577664, 1103101952, 1103626240]],0,3)));let child=child.unwrap();
check_value(&child.mesh_scale,2,[1101004800, 1101529088, 1102053376]);
check_value(&child.summer_albedo,1,[1101004800, 1101529088, 1102053376]);
check_value(&child.autumn_albedo,2,[1082130432, 1084227584, 1086324736]);
check_value(&child.winter_albedo,2,[1102577664, 1103101952, 1103626240]);
assert_eq!(std::mem::discriminant(&child.mesh),std::mem::discriminant(&mesh(1)));assert_eq!(std::mem::discriminant(&child.semantic),std::mem::discriminant(&semantic(3)));}
{ let mut child=Some(make_rt([0, 0, 2, 0],[[1073741824, 1077936128, 1082130432], [1077936128, 1082130432, 1084227584], [1082130432, 1084227584, 1086324736], [1084227584, 1086324736, 1088421888]],1,3));child.inherit_from(&None);let child=child.unwrap();
check_value(&child.mesh_scale,2,[1065353216, 1065353216, 1065353216]);
check_value(&child.summer_albedo,2,[1065353216, 1065353216, 1065353216]);
check_value(&child.autumn_albedo,2,[1082130432, 1084227584, 1086324736]);
check_value(&child.winter_albedo,2,[1065353216, 1065353216, 1065353216]);
assert_eq!(std::mem::discriminant(&child.mesh),std::mem::discriminant(&mesh(1)));assert_eq!(std::mem::discriminant(&child.semantic),std::mem::discriminant(&semantic(3)));}
{ let mut child=Some(make_rt([2, 2, 0, 2],[[1073741824, 1077936128, 1082130432], [1077936128, 1082130432, 1084227584], [1082130432, 1084227584, 1086324736], [1084227584, 1086324736, 1088421888]],2,1));child.inherit_from(&Some(make_rt([0, 1, 2, 2],[[1101004800, 1101529088, 1102053376], [1101529088, 1102053376, 1102577664], [1102053376, 1102577664, 1103101952], [1102577664, 1103101952, 1103626240]],0,3)));let child=child.unwrap();
check_value(&child.mesh_scale,2,[1073741824, 1077936128, 1082130432]);
check_value(&child.summer_albedo,2,[1077936128, 1082130432, 1084227584]);
check_value(&child.autumn_albedo,2,[1102053376, 1102577664, 1103101952]);
check_value(&child.winter_albedo,2,[1084227584, 1086324736, 1088421888]);
assert_eq!(std::mem::discriminant(&child.mesh),std::mem::discriminant(&mesh(2)));assert_eq!(std::mem::discriminant(&child.semantic),std::mem::discriminant(&semantic(1)));assert_eq!(child.semantic.as_concrete(),semantic(1).as_concrete());}
{ let mut child=Some(make_rt([2, 2, 0, 2],[[1073741824, 1077936128, 1082130432], [1077936128, 1082130432, 1084227584], [1082130432, 1084227584, 1086324736], [1084227584, 1086324736, 1088421888]],2,1));child.inherit_from(&None);let child=child.unwrap();
check_value(&child.mesh_scale,2,[1073741824, 1077936128, 1082130432]);
check_value(&child.summer_albedo,2,[1077936128, 1082130432, 1084227584]);
check_value(&child.autumn_albedo,2,[1065353216, 1065353216, 1065353216]);
check_value(&child.winter_albedo,2,[1084227584, 1086324736, 1088421888]);
assert_eq!(std::mem::discriminant(&child.mesh),std::mem::discriminant(&mesh(2)));assert_eq!(std::mem::discriminant(&child.semantic),std::mem::discriminant(&semantic(1)));assert_eq!(child.semantic.as_concrete(),semantic(1).as_concrete());}
{ let mut child=Some(make_rt([1, 1, 0, 0],[[1073741824, 1077936128, 1082130432], [1077936128, 1082130432, 1084227584], [1082130432, 1084227584, 1086324736], [1084227584, 1086324736, 1088421888]],2,3));child.inherit_from(&Some(make_rt([0, 1, 0, 2],[[1101004800, 1101529088, 1102053376], [1101529088, 1102053376, 1102577664], [1102053376, 1102577664, 1103101952], [1102577664, 1103101952, 1103626240]],1,0)));let child=child.unwrap();
check_value(&child.mesh_scale,1,[1073741824, 1077936128, 1082130432]);
check_value(&child.summer_albedo,1,[1077936128, 1082130432, 1084227584]);
check_value(&child.autumn_albedo,2,[1065353216, 1065353216, 1065353216]);
check_value(&child.winter_albedo,2,[1102577664, 1103101952, 1103626240]);
assert_eq!(std::mem::discriminant(&child.mesh),std::mem::discriminant(&mesh(2)));assert_eq!(std::mem::discriminant(&child.semantic),std::mem::discriminant(&semantic(3)));}
{ let mut child=Some(make_rt([1, 1, 0, 0],[[1073741824, 1077936128, 1082130432], [1077936128, 1082130432, 1084227584], [1082130432, 1084227584, 1086324736], [1084227584, 1086324736, 1088421888]],2,3));child.inherit_from(&None);let child=child.unwrap();
check_value(&child.mesh_scale,1,[1073741824, 1077936128, 1082130432]);
check_value(&child.summer_albedo,1,[1077936128, 1082130432, 1084227584]);
check_value(&child.autumn_albedo,2,[1065353216, 1065353216, 1065353216]);
check_value(&child.winter_albedo,2,[1065353216, 1065353216, 1065353216]);
assert_eq!(std::mem::discriminant(&child.mesh),std::mem::discriminant(&mesh(2)));assert_eq!(std::mem::discriminant(&child.semantic),std::mem::discriminant(&semantic(3)));}
{ let mut child=Some(make_rt([1, 0, 0, 0],[[1073741824, 1077936128, 1082130432], [1077936128, 1082130432, 1084227584], [1082130432, 1084227584, 1086324736], [1084227584, 1086324736, 1088421888]],0,0));child.inherit_from(&Some(make_rt([1, 1, 1, 2],[[1101004800, 1101529088, 1102053376], [1101529088, 1102053376, 1102577664], [1102053376, 1102577664, 1103101952], [1102577664, 1103101952, 1103626240]],2,1)));let child=child.unwrap();
check_value(&child.mesh_scale,1,[1073741824, 1077936128, 1082130432]);
check_value(&child.summer_albedo,1,[1102577664, 1103101952, 1103626240]);
check_value(&child.autumn_albedo,1,[1102577664, 1103101952, 1103626240]);
check_value(&child.winter_albedo,2,[1102577664, 1103101952, 1103626240]);
assert_eq!(std::mem::discriminant(&child.mesh),std::mem::discriminant(&mesh(2)));assert_eq!(std::mem::discriminant(&child.semantic),std::mem::discriminant(&semantic(0)));assert_eq!(child.semantic.as_concrete(),semantic(0).as_concrete());}
{ let mut child=Some(make_rt([1, 0, 0, 0],[[1073741824, 1077936128, 1082130432], [1077936128, 1082130432, 1084227584], [1082130432, 1084227584, 1086324736], [1084227584, 1086324736, 1088421888]],0,0));child.inherit_from(&None);let child=child.unwrap();
check_value(&child.mesh_scale,1,[1073741824, 1077936128, 1082130432]);
check_value(&child.summer_albedo,2,[1065353216, 1065353216, 1065353216]);
check_value(&child.autumn_albedo,2,[1065353216, 1065353216, 1065353216]);
check_value(&child.winter_albedo,2,[1065353216, 1065353216, 1065353216]);
assert_eq!(std::mem::discriminant(&child.mesh),std::mem::discriminant(&mesh(1)));assert_eq!(std::mem::discriminant(&child.semantic),std::mem::discriminant(&semantic(0)));assert_eq!(child.semantic.as_concrete(),semantic(0).as_concrete());}
{ let mut child=Some(make_rt([0, 2, 0, 0],[[1073741824, 1077936128, 1082130432], [1077936128, 1082130432, 1084227584], [1082130432, 1084227584, 1086324736], [1084227584, 1086324736, 1088421888]],0,1));child.inherit_from(&Some(make_rt([0, 2, 2, 2],[[1101004800, 1101529088, 1102053376], [1101529088, 1102053376, 1102577664], [1102053376, 1102577664, 1103101952], [1102577664, 1103101952, 1103626240]],2,1)));let child=child.unwrap();
check_value(&child.mesh_scale,2,[1065353216, 1065353216, 1065353216]);
check_value(&child.summer_albedo,2,[1077936128, 1082130432, 1084227584]);
check_value(&child.autumn_albedo,2,[1102053376, 1102577664, 1103101952]);
check_value(&child.winter_albedo,2,[1102577664, 1103101952, 1103626240]);
assert_eq!(std::mem::discriminant(&child.mesh),std::mem::discriminant(&mesh(2)));assert_eq!(std::mem::discriminant(&child.semantic),std::mem::discriminant(&semantic(1)));assert_eq!(child.semantic.as_concrete(),semantic(1).as_concrete());}
{ let mut child=Some(make_rt([0, 2, 0, 0],[[1073741824, 1077936128, 1082130432], [1077936128, 1082130432, 1084227584], [1082130432, 1084227584, 1086324736], [1084227584, 1086324736, 1088421888]],0,1));child.inherit_from(&None);let child=child.unwrap();
check_value(&child.mesh_scale,2,[1065353216, 1065353216, 1065353216]);
check_value(&child.summer_albedo,2,[1077936128, 1082130432, 1084227584]);
check_value(&child.autumn_albedo,2,[1065353216, 1065353216, 1065353216]);
check_value(&child.winter_albedo,2,[1065353216, 1065353216, 1065353216]);
assert_eq!(std::mem::discriminant(&child.mesh),std::mem::discriminant(&mesh(1)));assert_eq!(std::mem::discriminant(&child.semantic),std::mem::discriminant(&semantic(1)));assert_eq!(child.semantic.as_concrete(),semantic(1).as_concrete());}
{ let mut child=Some(make_rt([1, 2, 0, 0],[[1073741824, 1077936128, 1082130432], [1077936128, 1082130432, 1084227584], [1082130432, 1084227584, 1086324736], [1084227584, 1086324736, 1088421888]],2,3));child.inherit_from(&Some(make_rt([1, 2, 1, 2],[[1101004800, 1101529088, 1102053376], [1101529088, 1102053376, 1102577664], [1102053376, 1102577664, 1103101952], [1102577664, 1103101952, 1103626240]],2,1)));let child=child.unwrap();
check_value(&child.mesh_scale,1,[1073741824, 1077936128, 1082130432]);
check_value(&child.summer_albedo,2,[1077936128, 1082130432, 1084227584]);
check_value(&child.autumn_albedo,1,[1102577664, 1103101952, 1103626240]);
check_value(&child.winter_albedo,2,[1102577664, 1103101952, 1103626240]);
assert_eq!(std::mem::discriminant(&child.mesh),std::mem::discriminant(&mesh(2)));assert_eq!(std::mem::discriminant(&child.semantic),std::mem::discriminant(&semantic(3)));}
{ let mut child=Some(make_rt([1, 2, 0, 0],[[1073741824, 1077936128, 1082130432], [1077936128, 1082130432, 1084227584], [1082130432, 1084227584, 1086324736], [1084227584, 1086324736, 1088421888]],2,3));child.inherit_from(&None);let child=child.unwrap();
check_value(&child.mesh_scale,1,[1073741824, 1077936128, 1082130432]);
check_value(&child.summer_albedo,2,[1077936128, 1082130432, 1084227584]);
check_value(&child.autumn_albedo,2,[1065353216, 1065353216, 1065353216]);
check_value(&child.winter_albedo,2,[1065353216, 1065353216, 1065353216]);
assert_eq!(std::mem::discriminant(&child.mesh),std::mem::discriminant(&mesh(2)));assert_eq!(std::mem::discriminant(&child.semantic),std::mem::discriminant(&semantic(3)));}
{ let mut child=Some(make_rt([2, 0, 2, 2],[[1073741824, 1077936128, 1082130432], [1077936128, 1082130432, 1084227584], [1082130432, 1084227584, 1086324736], [1084227584, 1086324736, 1088421888]],2,1));child.inherit_from(&Some(make_rt([1, 2, 1, 1],[[1101004800, 1101529088, 1102053376], [1101529088, 1102053376, 1102577664], [1102053376, 1102577664, 1103101952], [1102577664, 1103101952, 1103626240]],2,3)));let child=child.unwrap();
check_value(&child.mesh_scale,2,[1073741824, 1077936128, 1082130432]);
check_value(&child.summer_albedo,2,[1101529088, 1102053376, 1102577664]);
check_value(&child.autumn_albedo,2,[1082130432, 1084227584, 1086324736]);
check_value(&child.winter_albedo,2,[1084227584, 1086324736, 1088421888]);
assert_eq!(std::mem::discriminant(&child.mesh),std::mem::discriminant(&mesh(2)));assert_eq!(std::mem::discriminant(&child.semantic),std::mem::discriminant(&semantic(1)));assert_eq!(child.semantic.as_concrete(),semantic(1).as_concrete());}
{ let mut child=Some(make_rt([2, 0, 2, 2],[[1073741824, 1077936128, 1082130432], [1077936128, 1082130432, 1084227584], [1082130432, 1084227584, 1086324736], [1084227584, 1086324736, 1088421888]],2,1));child.inherit_from(&None);let child=child.unwrap();
check_value(&child.mesh_scale,2,[1073741824, 1077936128, 1082130432]);
check_value(&child.summer_albedo,2,[1065353216, 1065353216, 1065353216]);
check_value(&child.autumn_albedo,2,[1082130432, 1084227584, 1086324736]);
check_value(&child.winter_albedo,2,[1084227584, 1086324736, 1088421888]);
assert_eq!(std::mem::discriminant(&child.mesh),std::mem::discriminant(&mesh(2)));assert_eq!(std::mem::discriminant(&child.semantic),std::mem::discriminant(&semantic(1)));assert_eq!(child.semantic.as_concrete(),semantic(1).as_concrete());}
{ let mut child=Some(make_rt([2, 2, 1, 1],[[1073741824, 1077936128, 1082130432], [1077936128, 1082130432, 1084227584], [1082130432, 1084227584, 1086324736], [1084227584, 1086324736, 1088421888]],0,3));child.inherit_from(&Some(make_rt([2, 0, 1, 1],[[1101004800, 1101529088, 1102053376], [1101529088, 1102053376, 1102577664], [1102053376, 1102577664, 1103101952], [1102577664, 1103101952, 1103626240]],2,3)));let child=child.unwrap();
check_value(&child.mesh_scale,2,[1073741824, 1077936128, 1082130432]);
check_value(&child.summer_albedo,2,[1077936128, 1082130432, 1084227584]);
check_value(&child.autumn_albedo,1,[1082130432, 1084227584, 1086324736]);
check_value(&child.winter_albedo,1,[1084227584, 1086324736, 1088421888]);
assert_eq!(std::mem::discriminant(&child.mesh),std::mem::discriminant(&mesh(2)));assert_eq!(std::mem::discriminant(&child.semantic),std::mem::discriminant(&semantic(3)));}
{ let mut child=Some(make_rt([2, 2, 1, 1],[[1073741824, 1077936128, 1082130432], [1077936128, 1082130432, 1084227584], [1082130432, 1084227584, 1086324736], [1084227584, 1086324736, 1088421888]],0,3));child.inherit_from(&None);let child=child.unwrap();
check_value(&child.mesh_scale,2,[1073741824, 1077936128, 1082130432]);
check_value(&child.summer_albedo,2,[1077936128, 1082130432, 1084227584]);
check_value(&child.autumn_albedo,1,[1082130432, 1084227584, 1086324736]);
check_value(&child.winter_albedo,1,[1084227584, 1086324736, 1088421888]);
assert_eq!(std::mem::discriminant(&child.mesh),std::mem::discriminant(&mesh(1)));assert_eq!(std::mem::discriminant(&child.semantic),std::mem::discriminant(&semantic(3)));}
{ let mut child=Some(make_rt([1, 1, 2, 2],[[1073741824, 1077936128, 1082130432], [1077936128, 1082130432, 1084227584], [1082130432, 1084227584, 1086324736], [1084227584, 1086324736, 1088421888]],1,1));child.inherit_from(&Some(make_rt([1, 1, 2, 2],[[1101004800, 1101529088, 1102053376], [1101529088, 1102053376, 1102577664], [1102053376, 1102577664, 1103101952], [1102577664, 1103101952, 1103626240]],2,1)));let child=child.unwrap();
check_value(&child.mesh_scale,1,[1073741824, 1077936128, 1082130432]);
check_value(&child.summer_albedo,1,[1077936128, 1082130432, 1084227584]);
check_value(&child.autumn_albedo,2,[1082130432, 1084227584, 1086324736]);
check_value(&child.winter_albedo,2,[1084227584, 1086324736, 1088421888]);
assert_eq!(std::mem::discriminant(&child.mesh),std::mem::discriminant(&mesh(1)));assert_eq!(std::mem::discriminant(&child.semantic),std::mem::discriminant(&semantic(1)));assert_eq!(child.semantic.as_concrete(),semantic(1).as_concrete());}
{ let mut child=Some(make_rt([1, 1, 2, 2],[[1073741824, 1077936128, 1082130432], [1077936128, 1082130432, 1084227584], [1082130432, 1084227584, 1086324736], [1084227584, 1086324736, 1088421888]],1,1));child.inherit_from(&None);let child=child.unwrap();
check_value(&child.mesh_scale,1,[1073741824, 1077936128, 1082130432]);
check_value(&child.summer_albedo,1,[1077936128, 1082130432, 1084227584]);
check_value(&child.autumn_albedo,2,[1082130432, 1084227584, 1086324736]);
check_value(&child.winter_albedo,2,[1084227584, 1086324736, 1088421888]);
assert_eq!(std::mem::discriminant(&child.mesh),std::mem::discriminant(&mesh(1)));assert_eq!(std::mem::discriminant(&child.semantic),std::mem::discriminant(&semantic(1)));assert_eq!(child.semantic.as_concrete(),semantic(1).as_concrete());}
{ let mut child=Some(make_rt([2, 2, 0, 1],[[1073741824, 1077936128, 1082130432], [1077936128, 1082130432, 1084227584], [1082130432, 1084227584, 1086324736], [1084227584, 1086324736, 1088421888]],2,0));child.inherit_from(&Some(make_rt([2, 2, 2, 1],[[1101004800, 1101529088, 1102053376], [1101529088, 1102053376, 1102577664], [1102053376, 1102577664, 1103101952], [1102577664, 1103101952, 1103626240]],2,3)));let child=child.unwrap();
check_value(&child.mesh_scale,2,[1073741824, 1077936128, 1082130432]);
check_value(&child.summer_albedo,2,[1077936128, 1082130432, 1084227584]);
check_value(&child.autumn_albedo,2,[1102053376, 1102577664, 1103101952]);
check_value(&child.winter_albedo,1,[1084227584, 1086324736, 1088421888]);
assert_eq!(std::mem::discriminant(&child.mesh),std::mem::discriminant(&mesh(2)));assert_eq!(std::mem::discriminant(&child.semantic),std::mem::discriminant(&semantic(0)));assert_eq!(child.semantic.as_concrete(),semantic(0).as_concrete());}
{ let mut child=Some(make_rt([2, 2, 0, 1],[[1073741824, 1077936128, 1082130432], [1077936128, 1082130432, 1084227584], [1082130432, 1084227584, 1086324736], [1084227584, 1086324736, 1088421888]],2,0));child.inherit_from(&None);let child=child.unwrap();
check_value(&child.mesh_scale,2,[1073741824, 1077936128, 1082130432]);
check_value(&child.summer_albedo,2,[1077936128, 1082130432, 1084227584]);
check_value(&child.autumn_albedo,2,[1065353216, 1065353216, 1065353216]);
check_value(&child.winter_albedo,1,[1084227584, 1086324736, 1088421888]);
assert_eq!(std::mem::discriminant(&child.mesh),std::mem::discriminant(&mesh(2)));assert_eq!(std::mem::discriminant(&child.semantic),std::mem::discriminant(&semantic(0)));assert_eq!(child.semantic.as_concrete(),semantic(0).as_concrete());}
{ let mut child=Some(make_rt([1, 1, 1, 2],[[1073741824, 1077936128, 1082130432], [1077936128, 1082130432, 1084227584], [1082130432, 1084227584, 1086324736], [1084227584, 1086324736, 1088421888]],1,2));child.inherit_from(&Some(make_rt([1, 2, 2, 2],[[1101004800, 1101529088, 1102053376], [1101529088, 1102053376, 1102577664], [1102053376, 1102577664, 1103101952], [1102577664, 1103101952, 1103626240]],1,1)));let child=child.unwrap();
check_value(&child.mesh_scale,1,[1073741824, 1077936128, 1082130432]);
check_value(&child.summer_albedo,1,[1077936128, 1082130432, 1084227584]);
check_value(&child.autumn_albedo,1,[1082130432, 1084227584, 1086324736]);
check_value(&child.winter_albedo,2,[1084227584, 1086324736, 1088421888]);
assert_eq!(std::mem::discriminant(&child.mesh),std::mem::discriminant(&mesh(1)));assert_eq!(std::mem::discriminant(&child.semantic),std::mem::discriminant(&semantic(1)));assert_eq!(child.semantic.as_concrete(),semantic(1).as_concrete());}
{ let mut child=Some(make_rt([1, 1, 1, 2],[[1073741824, 1077936128, 1082130432], [1077936128, 1082130432, 1084227584], [1082130432, 1084227584, 1086324736], [1084227584, 1086324736, 1088421888]],1,2));child.inherit_from(&None);let child=child.unwrap();
check_value(&child.mesh_scale,1,[1073741824, 1077936128, 1082130432]);
check_value(&child.summer_albedo,1,[1077936128, 1082130432, 1084227584]);
check_value(&child.autumn_albedo,1,[1082130432, 1084227584, 1086324736]);
check_value(&child.winter_albedo,2,[1084227584, 1086324736, 1088421888]);
assert_eq!(std::mem::discriminant(&child.mesh),std::mem::discriminant(&mesh(1)));assert_eq!(std::mem::discriminant(&child.semantic),std::mem::discriminant(&semantic(0)));assert_eq!(child.semantic.as_concrete(),semantic(0).as_concrete());}
{ let mut child=Some(make_rt([2, 1, 1, 2],[[1073741824, 1077936128, 1082130432], [1077936128, 1082130432, 1084227584], [1082130432, 1084227584, 1086324736], [1084227584, 1086324736, 1088421888]],1,1));child.inherit_from(&Some(make_rt([1, 0, 1, 1],[[1101004800, 1101529088, 1102053376], [1101529088, 1102053376, 1102577664], [1102053376, 1102577664, 1103101952], [1102577664, 1103101952, 1103626240]],0,1)));let child=child.unwrap();
check_value(&child.mesh_scale,2,[1073741824, 1077936128, 1082130432]);
check_value(&child.summer_albedo,1,[1077936128, 1082130432, 1084227584]);
check_value(&child.autumn_albedo,1,[1082130432, 1084227584, 1086324736]);
check_value(&child.winter_albedo,2,[1084227584, 1086324736, 1088421888]);
assert_eq!(std::mem::discriminant(&child.mesh),std::mem::discriminant(&mesh(1)));assert_eq!(std::mem::discriminant(&child.semantic),std::mem::discriminant(&semantic(1)));assert_eq!(child.semantic.as_concrete(),semantic(1).as_concrete());}
{ let mut child=Some(make_rt([2, 1, 1, 2],[[1073741824, 1077936128, 1082130432], [1077936128, 1082130432, 1084227584], [1082130432, 1084227584, 1086324736], [1084227584, 1086324736, 1088421888]],1,1));child.inherit_from(&None);let child=child.unwrap();
check_value(&child.mesh_scale,2,[1073741824, 1077936128, 1082130432]);
check_value(&child.summer_albedo,1,[1077936128, 1082130432, 1084227584]);
check_value(&child.autumn_albedo,1,[1082130432, 1084227584, 1086324736]);
check_value(&child.winter_albedo,2,[1084227584, 1086324736, 1088421888]);
assert_eq!(std::mem::discriminant(&child.mesh),std::mem::discriminant(&mesh(1)));assert_eq!(std::mem::discriminant(&child.semantic),std::mem::discriminant(&semantic(1)));assert_eq!(child.semantic.as_concrete(),semantic(1).as_concrete());}
{ let mut child=Some(make_rt([0, 2, 0, 1],[[1073741824, 1077936128, 1082130432], [1077936128, 1082130432, 1084227584], [1082130432, 1084227584, 1086324736], [1084227584, 1086324736, 1088421888]],1,1));child.inherit_from(&Some(make_rt([2, 2, 0, 0],[[1101004800, 1101529088, 1102053376], [1101529088, 1102053376, 1102577664], [1102053376, 1102577664, 1103101952], [1102577664, 1103101952, 1103626240]],2,3)));let child=child.unwrap();
check_value(&child.mesh_scale,2,[1101004800, 1101529088, 1102053376]);
check_value(&child.summer_albedo,2,[1077936128, 1082130432, 1084227584]);
check_value(&child.autumn_albedo,2,[1065353216, 1065353216, 1065353216]);
check_value(&child.winter_albedo,1,[1084227584, 1086324736, 1088421888]);
assert_eq!(std::mem::discriminant(&child.mesh),std::mem::discriminant(&mesh(1)));assert_eq!(std::mem::discriminant(&child.semantic),std::mem::discriminant(&semantic(1)));assert_eq!(child.semantic.as_concrete(),semantic(1).as_concrete());}
{ let mut child=Some(make_rt([0, 2, 0, 1],[[1073741824, 1077936128, 1082130432], [1077936128, 1082130432, 1084227584], [1082130432, 1084227584, 1086324736], [1084227584, 1086324736, 1088421888]],1,1));child.inherit_from(&None);let child=child.unwrap();
check_value(&child.mesh_scale,2,[1065353216, 1065353216, 1065353216]);
check_value(&child.summer_albedo,2,[1077936128, 1082130432, 1084227584]);
check_value(&child.autumn_albedo,2,[1065353216, 1065353216, 1065353216]);
check_value(&child.winter_albedo,1,[1084227584, 1086324736, 1088421888]);
assert_eq!(std::mem::discriminant(&child.mesh),std::mem::discriminant(&mesh(1)));assert_eq!(std::mem::discriminant(&child.semantic),std::mem::discriminant(&semantic(1)));assert_eq!(child.semantic.as_concrete(),semantic(1).as_concrete());}
{ let mut child=Some(make_rt([1, 2, 1, 2],[[1073741824, 1077936128, 1082130432], [1077936128, 1082130432, 1084227584], [1082130432, 1084227584, 1086324736], [1084227584, 1086324736, 1088421888]],1,3));child.inherit_from(&Some(make_rt([2, 2, 2, 1],[[1101004800, 1101529088, 1102053376], [1101529088, 1102053376, 1102577664], [1102053376, 1102577664, 1103101952], [1102577664, 1103101952, 1103626240]],1,1)));let child=child.unwrap();
check_value(&child.mesh_scale,1,[1073741824, 1077936128, 1082130432]);
check_value(&child.summer_albedo,2,[1077936128, 1082130432, 1084227584]);
check_value(&child.autumn_albedo,1,[1082130432, 1084227584, 1086324736]);
check_value(&child.winter_albedo,2,[1084227584, 1086324736, 1088421888]);
assert_eq!(std::mem::discriminant(&child.mesh),std::mem::discriminant(&mesh(1)));assert_eq!(std::mem::discriminant(&child.semantic),std::mem::discriminant(&semantic(3)));}
{ let mut child=Some(make_rt([1, 2, 1, 2],[[1073741824, 1077936128, 1082130432], [1077936128, 1082130432, 1084227584], [1082130432, 1084227584, 1086324736], [1084227584, 1086324736, 1088421888]],1,3));child.inherit_from(&None);let child=child.unwrap();
check_value(&child.mesh_scale,1,[1073741824, 1077936128, 1082130432]);
check_value(&child.summer_albedo,2,[1077936128, 1082130432, 1084227584]);
check_value(&child.autumn_albedo,1,[1082130432, 1084227584, 1086324736]);
check_value(&child.winter_albedo,2,[1084227584, 1086324736, 1088421888]);
assert_eq!(std::mem::discriminant(&child.mesh),std::mem::discriminant(&mesh(1)));assert_eq!(std::mem::discriminant(&child.semantic),std::mem::discriminant(&semantic(3)));}
{ let mut child=Some(make_rt([1, 0, 1, 2],[[1073741824, 1077936128, 1082130432], [1077936128, 1082130432, 1084227584], [1082130432, 1084227584, 1086324736], [1084227584, 1086324736, 1088421888]],0,0));child.inherit_from(&Some(make_rt([0, 2, 2, 0],[[1101004800, 1101529088, 1102053376], [1101529088, 1102053376, 1102577664], [1102053376, 1102577664, 1103101952], [1102577664, 1103101952, 1103626240]],1,3)));let child=child.unwrap();
check_value(&child.mesh_scale,1,[1073741824, 1077936128, 1082130432]);
check_value(&child.summer_albedo,2,[1101529088, 1102053376, 1102577664]);
check_value(&child.autumn_albedo,1,[1082130432, 1084227584, 1086324736]);
check_value(&child.winter_albedo,2,[1084227584, 1086324736, 1088421888]);
assert_eq!(std::mem::discriminant(&child.mesh),std::mem::discriminant(&mesh(1)));assert_eq!(std::mem::discriminant(&child.semantic),std::mem::discriminant(&semantic(0)));assert_eq!(child.semantic.as_concrete(),semantic(0).as_concrete());}
{ let mut child=Some(make_rt([1, 0, 1, 2],[[1073741824, 1077936128, 1082130432], [1077936128, 1082130432, 1084227584], [1082130432, 1084227584, 1086324736], [1084227584, 1086324736, 1088421888]],0,0));child.inherit_from(&None);let child=child.unwrap();
check_value(&child.mesh_scale,1,[1073741824, 1077936128, 1082130432]);
check_value(&child.summer_albedo,2,[1065353216, 1065353216, 1065353216]);
check_value(&child.autumn_albedo,1,[1082130432, 1084227584, 1086324736]);
check_value(&child.winter_albedo,2,[1084227584, 1086324736, 1088421888]);
assert_eq!(std::mem::discriminant(&child.mesh),std::mem::discriminant(&mesh(1)));assert_eq!(std::mem::discriminant(&child.semantic),std::mem::discriminant(&semantic(0)));assert_eq!(child.semantic.as_concrete(),semantic(0).as_concrete());}
{ let mut child=Some(make_rt([2, 1, 0, 1],[[1073741824, 1077936128, 1082130432], [1077936128, 1082130432, 1084227584], [1082130432, 1084227584, 1086324736], [1084227584, 1086324736, 1088421888]],0,1));child.inherit_from(&Some(make_rt([0, 0, 1, 2],[[1101004800, 1101529088, 1102053376], [1101529088, 1102053376, 1102577664], [1102053376, 1102577664, 1103101952], [1102577664, 1103101952, 1103626240]],2,0)));let child=child.unwrap();
check_value(&child.mesh_scale,2,[1073741824, 1077936128, 1082130432]);
check_value(&child.summer_albedo,1,[1077936128, 1082130432, 1084227584]);
check_value(&child.autumn_albedo,1,[1101529088, 1102053376, 1102577664]);
check_value(&child.winter_albedo,1,[1084227584, 1086324736, 1088421888]);
assert_eq!(std::mem::discriminant(&child.mesh),std::mem::discriminant(&mesh(2)));assert_eq!(std::mem::discriminant(&child.semantic),std::mem::discriminant(&semantic(1)));assert_eq!(child.semantic.as_concrete(),semantic(1).as_concrete());}
{ let mut child=Some(make_rt([2, 1, 0, 1],[[1073741824, 1077936128, 1082130432], [1077936128, 1082130432, 1084227584], [1082130432, 1084227584, 1086324736], [1084227584, 1086324736, 1088421888]],0,1));child.inherit_from(&None);let child=child.unwrap();
check_value(&child.mesh_scale,2,[1073741824, 1077936128, 1082130432]);
check_value(&child.summer_albedo,1,[1077936128, 1082130432, 1084227584]);
check_value(&child.autumn_albedo,2,[1065353216, 1065353216, 1065353216]);
check_value(&child.winter_albedo,1,[1084227584, 1086324736, 1088421888]);
assert_eq!(std::mem::discriminant(&child.mesh),std::mem::discriminant(&mesh(1)));assert_eq!(std::mem::discriminant(&child.semantic),std::mem::discriminant(&semantic(1)));assert_eq!(child.semantic.as_concrete(),semantic(1).as_concrete());}
{ let mut child=Some(make_rt([2, 0, 1, 1],[[1073741824, 1077936128, 1082130432], [1077936128, 1082130432, 1084227584], [1082130432, 1084227584, 1086324736], [1084227584, 1086324736, 1088421888]],1,3));child.inherit_from(&Some(make_rt([0, 1, 1, 1],[[1101004800, 1101529088, 1102053376], [1101529088, 1102053376, 1102577664], [1102053376, 1102577664, 1103101952], [1102577664, 1103101952, 1103626240]],0,1)));let child=child.unwrap();
check_value(&child.mesh_scale,2,[1073741824, 1077936128, 1082130432]);
check_value(&child.summer_albedo,1,[1101529088, 1102053376, 1102577664]);
check_value(&child.autumn_albedo,1,[1082130432, 1084227584, 1086324736]);
check_value(&child.winter_albedo,1,[1084227584, 1086324736, 1088421888]);
assert_eq!(std::mem::discriminant(&child.mesh),std::mem::discriminant(&mesh(1)));assert_eq!(std::mem::discriminant(&child.semantic),std::mem::discriminant(&semantic(3)));}
{ let mut child=Some(make_rt([2, 0, 1, 1],[[1073741824, 1077936128, 1082130432], [1077936128, 1082130432, 1084227584], [1082130432, 1084227584, 1086324736], [1084227584, 1086324736, 1088421888]],1,3));child.inherit_from(&None);let child=child.unwrap();
check_value(&child.mesh_scale,2,[1073741824, 1077936128, 1082130432]);
check_value(&child.summer_albedo,2,[1065353216, 1065353216, 1065353216]);
check_value(&child.autumn_albedo,1,[1082130432, 1084227584, 1086324736]);
check_value(&child.winter_albedo,1,[1084227584, 1086324736, 1088421888]);
assert_eq!(std::mem::discriminant(&child.mesh),std::mem::discriminant(&mesh(1)));assert_eq!(std::mem::discriminant(&child.semantic),std::mem::discriminant(&semantic(3)));}
{ let mut child=Some(make_rt([2, 2, 0, 2],[[1073741824, 1077936128, 1082130432], [1077936128, 1082130432, 1084227584], [1082130432, 1084227584, 1086324736], [1084227584, 1086324736, 1088421888]],2,3));child.inherit_from(&Some(make_rt([0, 0, 0, 2],[[1101004800, 1101529088, 1102053376], [1101529088, 1102053376, 1102577664], [1102053376, 1102577664, 1103101952], [1102577664, 1103101952, 1103626240]],2,2)));let child=child.unwrap();
check_value(&child.mesh_scale,2,[1073741824, 1077936128, 1082130432]);
check_value(&child.summer_albedo,2,[1077936128, 1082130432, 1084227584]);
check_value(&child.autumn_albedo,2,[1065353216, 1065353216, 1065353216]);
check_value(&child.winter_albedo,2,[1084227584, 1086324736, 1088421888]);
assert_eq!(std::mem::discriminant(&child.mesh),std::mem::discriminant(&mesh(2)));assert_eq!(std::mem::discriminant(&child.semantic),std::mem::discriminant(&semantic(3)));}
{ let mut child=Some(make_rt([2, 2, 0, 2],[[1073741824, 1077936128, 1082130432], [1077936128, 1082130432, 1084227584], [1082130432, 1084227584, 1086324736], [1084227584, 1086324736, 1088421888]],2,3));child.inherit_from(&None);let child=child.unwrap();
check_value(&child.mesh_scale,2,[1073741824, 1077936128, 1082130432]);
check_value(&child.summer_albedo,2,[1077936128, 1082130432, 1084227584]);
check_value(&child.autumn_albedo,2,[1065353216, 1065353216, 1065353216]);
check_value(&child.winter_albedo,2,[1084227584, 1086324736, 1088421888]);
assert_eq!(std::mem::discriminant(&child.mesh),std::mem::discriminant(&mesh(2)));assert_eq!(std::mem::discriminant(&child.semantic),std::mem::discriminant(&semantic(3)));}
{ let mut child=Some(make_rt([0, 1, 1, 2],[[1073741824, 1077936128, 1082130432], [1077936128, 1082130432, 1084227584], [1082130432, 1084227584, 1086324736], [1084227584, 1086324736, 1088421888]],2,0));child.inherit_from(&Some(make_rt([2, 0, 1, 2],[[1101004800, 1101529088, 1102053376], [1101529088, 1102053376, 1102577664], [1102053376, 1102577664, 1103101952], [1102577664, 1103101952, 1103626240]],0,0)));let child=child.unwrap();
check_value(&child.mesh_scale,2,[1101004800, 1101529088, 1102053376]);
check_value(&child.summer_albedo,1,[1077936128, 1082130432, 1084227584]);
check_value(&child.autumn_albedo,1,[1082130432, 1084227584, 1086324736]);
check_value(&child.winter_albedo,2,[1084227584, 1086324736, 1088421888]);
assert_eq!(std::mem::discriminant(&child.mesh),std::mem::discriminant(&mesh(2)));assert_eq!(std::mem::discriminant(&child.semantic),std::mem::discriminant(&semantic(0)));assert_eq!(child.semantic.as_concrete(),semantic(0).as_concrete());}
{ let mut child=Some(make_rt([0, 1, 1, 2],[[1073741824, 1077936128, 1082130432], [1077936128, 1082130432, 1084227584], [1082130432, 1084227584, 1086324736], [1084227584, 1086324736, 1088421888]],2,0));child.inherit_from(&None);let child=child.unwrap();
check_value(&child.mesh_scale,2,[1065353216, 1065353216, 1065353216]);
check_value(&child.summer_albedo,1,[1077936128, 1082130432, 1084227584]);
check_value(&child.autumn_albedo,1,[1082130432, 1084227584, 1086324736]);
check_value(&child.winter_albedo,2,[1084227584, 1086324736, 1088421888]);
assert_eq!(std::mem::discriminant(&child.mesh),std::mem::discriminant(&mesh(2)));assert_eq!(std::mem::discriminant(&child.semantic),std::mem::discriminant(&semantic(0)));assert_eq!(child.semantic.as_concrete(),semantic(0).as_concrete());}
{ let mut child=Some(make_rt([2, 1, 1, 0],[[1073741824, 1077936128, 1082130432], [1077936128, 1082130432, 1084227584], [1082130432, 1084227584, 1086324736], [1084227584, 1086324736, 1088421888]],2,0));child.inherit_from(&Some(make_rt([1, 0, 0, 2],[[1101004800, 1101529088, 1102053376], [1101529088, 1102053376, 1102577664], [1102053376, 1102577664, 1103101952], [1102577664, 1103101952, 1103626240]],1,0)));let child=child.unwrap();
check_value(&child.mesh_scale,2,[1073741824, 1077936128, 1082130432]);
check_value(&child.summer_albedo,1,[1077936128, 1082130432, 1084227584]);
check_value(&child.autumn_albedo,1,[1082130432, 1084227584, 1086324736]);
check_value(&child.winter_albedo,2,[1102577664, 1103101952, 1103626240]);
assert_eq!(std::mem::discriminant(&child.mesh),std::mem::discriminant(&mesh(2)));assert_eq!(std::mem::discriminant(&child.semantic),std::mem::discriminant(&semantic(0)));assert_eq!(child.semantic.as_concrete(),semantic(0).as_concrete());}
{ let mut child=Some(make_rt([2, 1, 1, 0],[[1073741824, 1077936128, 1082130432], [1077936128, 1082130432, 1084227584], [1082130432, 1084227584, 1086324736], [1084227584, 1086324736, 1088421888]],2,0));child.inherit_from(&None);let child=child.unwrap();
check_value(&child.mesh_scale,2,[1073741824, 1077936128, 1082130432]);
check_value(&child.summer_albedo,1,[1077936128, 1082130432, 1084227584]);
check_value(&child.autumn_albedo,1,[1082130432, 1084227584, 1086324736]);
check_value(&child.winter_albedo,2,[1065353216, 1065353216, 1065353216]);
assert_eq!(std::mem::discriminant(&child.mesh),std::mem::discriminant(&mesh(2)));assert_eq!(std::mem::discriminant(&child.semantic),std::mem::discriminant(&semantic(0)));assert_eq!(child.semantic.as_concrete(),semantic(0).as_concrete());}
{ let mut child=Some(make_rt([0, 1, 1, 0],[[1073741824, 1077936128, 1082130432], [1077936128, 1082130432, 1084227584], [1082130432, 1084227584, 1086324736], [1084227584, 1086324736, 1088421888]],1,3));child.inherit_from(&Some(make_rt([2, 2, 0, 1],[[1101004800, 1101529088, 1102053376], [1101529088, 1102053376, 1102577664], [1102053376, 1102577664, 1103101952], [1102577664, 1103101952, 1103626240]],0,2)));let child=child.unwrap();
check_value(&child.mesh_scale,2,[1101004800, 1101529088, 1102053376]);
check_value(&child.summer_albedo,1,[1077936128, 1082130432, 1084227584]);
check_value(&child.autumn_albedo,1,[1082130432, 1084227584, 1086324736]);
check_value(&child.winter_albedo,1,[1101004800, 1101529088, 1102053376]);
assert_eq!(std::mem::discriminant(&child.mesh),std::mem::discriminant(&mesh(1)));assert_eq!(std::mem::discriminant(&child.semantic),std::mem::discriminant(&semantic(3)));}
{ let mut child=Some(make_rt([0, 1, 1, 0],[[1073741824, 1077936128, 1082130432], [1077936128, 1082130432, 1084227584], [1082130432, 1084227584, 1086324736], [1084227584, 1086324736, 1088421888]],1,3));child.inherit_from(&None);let child=child.unwrap();
check_value(&child.mesh_scale,2,[1065353216, 1065353216, 1065353216]);
check_value(&child.summer_albedo,1,[1077936128, 1082130432, 1084227584]);
check_value(&child.autumn_albedo,1,[1082130432, 1084227584, 1086324736]);
check_value(&child.winter_albedo,2,[1065353216, 1065353216, 1065353216]);
assert_eq!(std::mem::discriminant(&child.mesh),std::mem::discriminant(&mesh(1)));assert_eq!(std::mem::discriminant(&child.semantic),std::mem::discriminant(&semantic(3)));}
{ let mut child=Some(make_rt([1, 1, 2, 1],[[1073741824, 1077936128, 1082130432], [1077936128, 1082130432, 1084227584], [1082130432, 1084227584, 1086324736], [1084227584, 1086324736, 1088421888]],2,3));child.inherit_from(&Some(make_rt([0, 2, 0, 0],[[1101004800, 1101529088, 1102053376], [1101529088, 1102053376, 1102577664], [1102053376, 1102577664, 1103101952], [1102577664, 1103101952, 1103626240]],2,2)));let child=child.unwrap();
check_value(&child.mesh_scale,1,[1073741824, 1077936128, 1082130432]);
check_value(&child.summer_albedo,1,[1077936128, 1082130432, 1084227584]);
check_value(&child.autumn_albedo,2,[1082130432, 1084227584, 1086324736]);
check_value(&child.winter_albedo,1,[1084227584, 1086324736, 1088421888]);
assert_eq!(std::mem::discriminant(&child.mesh),std::mem::discriminant(&mesh(2)));assert_eq!(std::mem::discriminant(&child.semantic),std::mem::discriminant(&semantic(3)));}
{ let mut child=Some(make_rt([1, 1, 2, 1],[[1073741824, 1077936128, 1082130432], [1077936128, 1082130432, 1084227584], [1082130432, 1084227584, 1086324736], [1084227584, 1086324736, 1088421888]],2,3));child.inherit_from(&None);let child=child.unwrap();
check_value(&child.mesh_scale,1,[1073741824, 1077936128, 1082130432]);
check_value(&child.summer_albedo,1,[1077936128, 1082130432, 1084227584]);
check_value(&child.autumn_albedo,2,[1082130432, 1084227584, 1086324736]);
check_value(&child.winter_albedo,1,[1084227584, 1086324736, 1088421888]);
assert_eq!(std::mem::discriminant(&child.mesh),std::mem::discriminant(&mesh(2)));assert_eq!(std::mem::discriminant(&child.semantic),std::mem::discriminant(&semantic(3)));}
{ let mut child=Some(make_rt([1, 0, 2, 1],[[1073741824, 1077936128, 1082130432], [1077936128, 1082130432, 1084227584], [1082130432, 1084227584, 1086324736], [1084227584, 1086324736, 1088421888]],0,3));child.inherit_from(&Some(make_rt([0, 2, 0, 1],[[1101004800, 1101529088, 1102053376], [1101529088, 1102053376, 1102577664], [1102053376, 1102577664, 1103101952], [1102577664, 1103101952, 1103626240]],0,2)));let child=child.unwrap();
check_value(&child.mesh_scale,1,[1073741824, 1077936128, 1082130432]);
check_value(&child.summer_albedo,2,[1101529088, 1102053376, 1102577664]);
check_value(&child.autumn_albedo,2,[1082130432, 1084227584, 1086324736]);
check_value(&child.winter_albedo,1,[1084227584, 1086324736, 1088421888]);
assert_eq!(std::mem::discriminant(&child.mesh),std::mem::discriminant(&mesh(1)));assert_eq!(std::mem::discriminant(&child.semantic),std::mem::discriminant(&semantic(3)));}
{ let mut child=Some(make_rt([1, 0, 2, 1],[[1073741824, 1077936128, 1082130432], [1077936128, 1082130432, 1084227584], [1082130432, 1084227584, 1086324736], [1084227584, 1086324736, 1088421888]],0,3));child.inherit_from(&None);let child=child.unwrap();
check_value(&child.mesh_scale,1,[1073741824, 1077936128, 1082130432]);
check_value(&child.summer_albedo,2,[1065353216, 1065353216, 1065353216]);
check_value(&child.autumn_albedo,2,[1082130432, 1084227584, 1086324736]);
check_value(&child.winter_albedo,1,[1084227584, 1086324736, 1088421888]);
assert_eq!(std::mem::discriminant(&child.mesh),std::mem::discriminant(&mesh(1)));assert_eq!(std::mem::discriminant(&child.semantic),std::mem::discriminant(&semantic(3)));}
{ let mut child=Some(make_rt([2, 2, 2, 2],[[1073741824, 1077936128, 1082130432], [1077936128, 1082130432, 1084227584], [1082130432, 1084227584, 1086324736], [1084227584, 1086324736, 1088421888]],2,1));child.inherit_from(&Some(make_rt([2, 1, 1, 2],[[1101004800, 1101529088, 1102053376], [1101529088, 1102053376, 1102577664], [1102053376, 1102577664, 1103101952], [1102577664, 1103101952, 1103626240]],1,0)));let child=child.unwrap();
check_value(&child.mesh_scale,2,[1073741824, 1077936128, 1082130432]);
check_value(&child.summer_albedo,2,[1077936128, 1082130432, 1084227584]);
check_value(&child.autumn_albedo,2,[1082130432, 1084227584, 1086324736]);
check_value(&child.winter_albedo,2,[1084227584, 1086324736, 1088421888]);
assert_eq!(std::mem::discriminant(&child.mesh),std::mem::discriminant(&mesh(2)));assert_eq!(std::mem::discriminant(&child.semantic),std::mem::discriminant(&semantic(1)));assert_eq!(child.semantic.as_concrete(),semantic(1).as_concrete());}
{ let mut child=Some(make_rt([2, 2, 2, 2],[[1073741824, 1077936128, 1082130432], [1077936128, 1082130432, 1084227584], [1082130432, 1084227584, 1086324736], [1084227584, 1086324736, 1088421888]],2,1));child.inherit_from(&None);let child=child.unwrap();
check_value(&child.mesh_scale,2,[1073741824, 1077936128, 1082130432]);
check_value(&child.summer_albedo,2,[1077936128, 1082130432, 1084227584]);
check_value(&child.autumn_albedo,2,[1082130432, 1084227584, 1086324736]);
check_value(&child.winter_albedo,2,[1084227584, 1086324736, 1088421888]);
assert_eq!(std::mem::discriminant(&child.mesh),std::mem::discriminant(&mesh(2)));assert_eq!(std::mem::discriminant(&child.semantic),std::mem::discriminant(&semantic(1)));assert_eq!(child.semantic.as_concrete(),semantic(1).as_concrete());}
{ let mut child=Some(make_rt([0, 0, 1, 0],[[1073741824, 1077936128, 1082130432], [1077936128, 1082130432, 1084227584], [1082130432, 1084227584, 1086324736], [1084227584, 1086324736, 1088421888]],2,3));child.inherit_from(&Some(make_rt([2, 2, 0, 1],[[1101004800, 1101529088, 1102053376], [1101529088, 1102053376, 1102577664], [1102053376, 1102577664, 1103101952], [1102577664, 1103101952, 1103626240]],2,0)));let child=child.unwrap();
check_value(&child.mesh_scale,2,[1101004800, 1101529088, 1102053376]);
check_value(&child.summer_albedo,2,[1101529088, 1102053376, 1102577664]);
check_value(&child.autumn_albedo,1,[1082130432, 1084227584, 1086324736]);
check_value(&child.winter_albedo,1,[1101529088, 1102053376, 1102577664]);
assert_eq!(std::mem::discriminant(&child.mesh),std::mem::discriminant(&mesh(2)));assert_eq!(std::mem::discriminant(&child.semantic),std::mem::discriminant(&semantic(3)));}
{ let mut child=Some(make_rt([0, 0, 1, 0],[[1073741824, 1077936128, 1082130432], [1077936128, 1082130432, 1084227584], [1082130432, 1084227584, 1086324736], [1084227584, 1086324736, 1088421888]],2,3));child.inherit_from(&None);let child=child.unwrap();
check_value(&child.mesh_scale,2,[1065353216, 1065353216, 1065353216]);
check_value(&child.summer_albedo,2,[1065353216, 1065353216, 1065353216]);
check_value(&child.autumn_albedo,1,[1082130432, 1084227584, 1086324736]);
check_value(&child.winter_albedo,2,[1065353216, 1065353216, 1065353216]);
assert_eq!(std::mem::discriminant(&child.mesh),std::mem::discriminant(&mesh(2)));assert_eq!(std::mem::discriminant(&child.semantic),std::mem::discriminant(&semantic(3)));}
{ let mut child=Some(make_rt([1, 1, 1, 0],[[1073741824, 1077936128, 1082130432], [1077936128, 1082130432, 1084227584], [1082130432, 1084227584, 1086324736], [1084227584, 1086324736, 1088421888]],0,1));child.inherit_from(&Some(make_rt([1, 2, 0, 0],[[1101004800, 1101529088, 1102053376], [1101529088, 1102053376, 1102577664], [1102053376, 1102577664, 1103101952], [1102577664, 1103101952, 1103626240]],0,3)));let child=child.unwrap();
check_value(&child.mesh_scale,1,[1073741824, 1077936128, 1082130432]);
check_value(&child.summer_albedo,1,[1077936128, 1082130432, 1084227584]);
check_value(&child.autumn_albedo,1,[1082130432, 1084227584, 1086324736]);
check_value(&child.winter_albedo,2,[1065353216, 1065353216, 1065353216]);
assert_eq!(std::mem::discriminant(&child.mesh),std::mem::discriminant(&mesh(1)));assert_eq!(std::mem::discriminant(&child.semantic),std::mem::discriminant(&semantic(1)));assert_eq!(child.semantic.as_concrete(),semantic(1).as_concrete());}
{ let mut child=Some(make_rt([1, 1, 1, 0],[[1073741824, 1077936128, 1082130432], [1077936128, 1082130432, 1084227584], [1082130432, 1084227584, 1086324736], [1084227584, 1086324736, 1088421888]],0,1));child.inherit_from(&None);let child=child.unwrap();
check_value(&child.mesh_scale,1,[1073741824, 1077936128, 1082130432]);
check_value(&child.summer_albedo,1,[1077936128, 1082130432, 1084227584]);
check_value(&child.autumn_albedo,1,[1082130432, 1084227584, 1086324736]);
check_value(&child.winter_albedo,2,[1065353216, 1065353216, 1065353216]);
assert_eq!(std::mem::discriminant(&child.mesh),std::mem::discriminant(&mesh(1)));assert_eq!(std::mem::discriminant(&child.semantic),std::mem::discriminant(&semantic(1)));assert_eq!(child.semantic.as_concrete(),semantic(1).as_concrete());}
{ let mut child=Some(make_rt([0, 2, 1, 2],[[1073741824, 1077936128, 1082130432], [1077936128, 1082130432, 1084227584], [1082130432, 1084227584, 1086324736], [1084227584, 1086324736, 1088421888]],2,1));child.inherit_from(&Some(make_rt([2, 0, 2, 2],[[1101004800, 1101529088, 1102053376], [1101529088, 1102053376, 1102577664], [1102053376, 1102577664, 1103101952], [1102577664, 1103101952, 1103626240]],2,1)));let child=child.unwrap();
check_value(&child.mesh_scale,2,[1101004800, 1101529088, 1102053376]);
check_value(&child.summer_albedo,2,[1077936128, 1082130432, 1084227584]);
check_value(&child.autumn_albedo,1,[1082130432, 1084227584, 1086324736]);
check_value(&child.winter_albedo,2,[1084227584, 1086324736, 1088421888]);
assert_eq!(std::mem::discriminant(&child.mesh),std::mem::discriminant(&mesh(2)));assert_eq!(std::mem::discriminant(&child.semantic),std::mem::discriminant(&semantic(1)));assert_eq!(child.semantic.as_concrete(),semantic(1).as_concrete());}
{ let mut child=Some(make_rt([0, 2, 1, 2],[[1073741824, 1077936128, 1082130432], [1077936128, 1082130432, 1084227584], [1082130432, 1084227584, 1086324736], [1084227584, 1086324736, 1088421888]],2,1));child.inherit_from(&None);let child=child.unwrap();
check_value(&child.mesh_scale,2,[1065353216, 1065353216, 1065353216]);
check_value(&child.summer_albedo,2,[1077936128, 1082130432, 1084227584]);
check_value(&child.autumn_albedo,1,[1082130432, 1084227584, 1086324736]);
check_value(&child.winter_albedo,2,[1084227584, 1086324736, 1088421888]);
assert_eq!(std::mem::discriminant(&child.mesh),std::mem::discriminant(&mesh(2)));assert_eq!(std::mem::discriminant(&child.semantic),std::mem::discriminant(&semantic(1)));assert_eq!(child.semantic.as_concrete(),semantic(1).as_concrete());}
{ let mut child=Some(make_rt([0, 0, 0, 0],[[1073741824, 1077936128, 1082130432], [1077936128, 1082130432, 1084227584], [1082130432, 1084227584, 1086324736], [1084227584, 1086324736, 1088421888]],1,1));child.inherit_from(&Some(make_rt([0, 1, 1, 0],[[1101004800, 1101529088, 1102053376], [1101529088, 1102053376, 1102577664], [1102053376, 1102577664, 1103101952], [1102577664, 1103101952, 1103626240]],2,1)));let child=child.unwrap();
check_value(&child.mesh_scale,2,[1065353216, 1065353216, 1065353216]);
check_value(&child.summer_albedo,1,[1101004800, 1101529088, 1102053376]);
check_value(&child.autumn_albedo,1,[1101004800, 1101529088, 1102053376]);
check_value(&child.winter_albedo,2,[1065353216, 1065353216, 1065353216]);
assert_eq!(std::mem::discriminant(&child.mesh),std::mem::discriminant(&mesh(1)));assert_eq!(std::mem::discriminant(&child.semantic),std::mem::discriminant(&semantic(1)));assert_eq!(child.semantic.as_concrete(),semantic(1).as_concrete());}
{ let mut child=Some(make_rt([0, 0, 0, 0],[[1073741824, 1077936128, 1082130432], [1077936128, 1082130432, 1084227584], [1082130432, 1084227584, 1086324736], [1084227584, 1086324736, 1088421888]],1,1));child.inherit_from(&None);let child=child.unwrap();
check_value(&child.mesh_scale,2,[1065353216, 1065353216, 1065353216]);
check_value(&child.summer_albedo,2,[1065353216, 1065353216, 1065353216]);
check_value(&child.autumn_albedo,2,[1065353216, 1065353216, 1065353216]);
check_value(&child.winter_albedo,2,[1065353216, 1065353216, 1065353216]);
assert_eq!(std::mem::discriminant(&child.mesh),std::mem::discriminant(&mesh(1)));assert_eq!(std::mem::discriminant(&child.semantic),std::mem::discriminant(&semantic(1)));assert_eq!(child.semantic.as_concrete(),semantic(1).as_concrete());}
{ let mut child=Some(make_rt([0, 2, 2, 0],[[1073741824, 1077936128, 1082130432], [1077936128, 1082130432, 1084227584], [1082130432, 1084227584, 1086324736], [1084227584, 1086324736, 1088421888]],1,0));child.inherit_from(&Some(make_rt([2, 0, 0, 2],[[1101004800, 1101529088, 1102053376], [1101529088, 1102053376, 1102577664], [1102053376, 1102577664, 1103101952], [1102577664, 1103101952, 1103626240]],0,1)));let child=child.unwrap();
check_value(&child.mesh_scale,2,[1101004800, 1101529088, 1102053376]);
check_value(&child.summer_albedo,2,[1077936128, 1082130432, 1084227584]);
check_value(&child.autumn_albedo,2,[1082130432, 1084227584, 1086324736]);
check_value(&child.winter_albedo,2,[1102577664, 1103101952, 1103626240]);
assert_eq!(std::mem::discriminant(&child.mesh),std::mem::discriminant(&mesh(1)));assert_eq!(std::mem::discriminant(&child.semantic),std::mem::discriminant(&semantic(0)));assert_eq!(child.semantic.as_concrete(),semantic(0).as_concrete());}
{ let mut child=Some(make_rt([0, 2, 2, 0],[[1073741824, 1077936128, 1082130432], [1077936128, 1082130432, 1084227584], [1082130432, 1084227584, 1086324736], [1084227584, 1086324736, 1088421888]],1,0));child.inherit_from(&None);let child=child.unwrap();
check_value(&child.mesh_scale,2,[1065353216, 1065353216, 1065353216]);
check_value(&child.summer_albedo,2,[1077936128, 1082130432, 1084227584]);
check_value(&child.autumn_albedo,2,[1082130432, 1084227584, 1086324736]);
check_value(&child.winter_albedo,2,[1065353216, 1065353216, 1065353216]);
assert_eq!(std::mem::discriminant(&child.mesh),std::mem::discriminant(&mesh(1)));assert_eq!(std::mem::discriminant(&child.semantic),std::mem::discriminant(&semantic(0)));assert_eq!(child.semantic.as_concrete(),semantic(0).as_concrete());}
{ let mut child=Some(make_rt([2, 1, 0, 0],[[1073741824, 1077936128, 1082130432], [1077936128, 1082130432, 1084227584], [1082130432, 1084227584, 1086324736], [1084227584, 1086324736, 1088421888]],2,0));child.inherit_from(&Some(make_rt([0, 1, 0, 1],[[1101004800, 1101529088, 1102053376], [1101529088, 1102053376, 1102577664], [1102053376, 1102577664, 1103101952], [1102577664, 1103101952, 1103626240]],1,1)));let child=child.unwrap();
check_value(&child.mesh_scale,2,[1073741824, 1077936128, 1082130432]);
check_value(&child.summer_albedo,1,[1077936128, 1082130432, 1084227584]);
check_value(&child.autumn_albedo,2,[1065353216, 1065353216, 1065353216]);
check_value(&child.winter_albedo,1,[1102577664, 1103101952, 1103626240]);
assert_eq!(std::mem::discriminant(&child.mesh),std::mem::discriminant(&mesh(2)));assert_eq!(std::mem::discriminant(&child.semantic),std::mem::discriminant(&semantic(0)));assert_eq!(child.semantic.as_concrete(),semantic(0).as_concrete());}
{ let mut child=Some(make_rt([2, 1, 0, 0],[[1073741824, 1077936128, 1082130432], [1077936128, 1082130432, 1084227584], [1082130432, 1084227584, 1086324736], [1084227584, 1086324736, 1088421888]],2,0));child.inherit_from(&None);let child=child.unwrap();
check_value(&child.mesh_scale,2,[1073741824, 1077936128, 1082130432]);
check_value(&child.summer_albedo,1,[1077936128, 1082130432, 1084227584]);
check_value(&child.autumn_albedo,2,[1065353216, 1065353216, 1065353216]);
check_value(&child.winter_albedo,2,[1065353216, 1065353216, 1065353216]);
assert_eq!(std::mem::discriminant(&child.mesh),std::mem::discriminant(&mesh(2)));assert_eq!(std::mem::discriminant(&child.semantic),std::mem::discriminant(&semantic(0)));assert_eq!(child.semantic.as_concrete(),semantic(0).as_concrete());}
{ let mut child=Some(make_rt([2, 0, 0, 0],[[1073741824, 1077936128, 1082130432], [1077936128, 1082130432, 1084227584], [1082130432, 1084227584, 1086324736], [1084227584, 1086324736, 1088421888]],2,1));child.inherit_from(&Some(make_rt([1, 0, 1, 0],[[1101004800, 1101529088, 1102053376], [1101529088, 1102053376, 1102577664], [1102053376, 1102577664, 1103101952], [1102577664, 1103101952, 1103626240]],1,0)));let child=child.unwrap();
check_value(&child.mesh_scale,2,[1073741824, 1077936128, 1082130432]);
check_value(&child.summer_albedo,2,[1065353216, 1065353216, 1065353216]);
check_value(&child.autumn_albedo,1,[1102577664, 1103101952, 1103626240]);
check_value(&child.winter_albedo,2,[1065353216, 1065353216, 1065353216]);
assert_eq!(std::mem::discriminant(&child.mesh),std::mem::discriminant(&mesh(2)));assert_eq!(std::mem::discriminant(&child.semantic),std::mem::discriminant(&semantic(1)));assert_eq!(child.semantic.as_concrete(),semantic(1).as_concrete());}
{ let mut child=Some(make_rt([2, 0, 0, 0],[[1073741824, 1077936128, 1082130432], [1077936128, 1082130432, 1084227584], [1082130432, 1084227584, 1086324736], [1084227584, 1086324736, 1088421888]],2,1));child.inherit_from(&None);let child=child.unwrap();
check_value(&child.mesh_scale,2,[1073741824, 1077936128, 1082130432]);
check_value(&child.summer_albedo,2,[1065353216, 1065353216, 1065353216]);
check_value(&child.autumn_albedo,2,[1065353216, 1065353216, 1065353216]);
check_value(&child.winter_albedo,2,[1065353216, 1065353216, 1065353216]);
assert_eq!(std::mem::discriminant(&child.mesh),std::mem::discriminant(&mesh(2)));assert_eq!(std::mem::discriminant(&child.semantic),std::mem::discriminant(&semantic(1)));assert_eq!(child.semantic.as_concrete(),semantic(1).as_concrete());}
{ let mut child=Some(make_rt([2, 1, 0, 0],[[1073741824, 1077936128, 1082130432], [1077936128, 1082130432, 1084227584], [1082130432, 1084227584, 1086324736], [1084227584, 1086324736, 1088421888]],0,1));child.inherit_from(&Some(make_rt([2, 0, 0, 0],[[1101004800, 1101529088, 1102053376], [1101529088, 1102053376, 1102577664], [1102053376, 1102577664, 1103101952], [1102577664, 1103101952, 1103626240]],2,2)));let child=child.unwrap();
check_value(&child.mesh_scale,2,[1073741824, 1077936128, 1082130432]);
check_value(&child.summer_albedo,1,[1077936128, 1082130432, 1084227584]);
check_value(&child.autumn_albedo,2,[1065353216, 1065353216, 1065353216]);
check_value(&child.winter_albedo,2,[1065353216, 1065353216, 1065353216]);
assert_eq!(std::mem::discriminant(&child.mesh),std::mem::discriminant(&mesh(2)));assert_eq!(std::mem::discriminant(&child.semantic),std::mem::discriminant(&semantic(1)));assert_eq!(child.semantic.as_concrete(),semantic(1).as_concrete());}
{ let mut child=Some(make_rt([2, 1, 0, 0],[[1073741824, 1077936128, 1082130432], [1077936128, 1082130432, 1084227584], [1082130432, 1084227584, 1086324736], [1084227584, 1086324736, 1088421888]],0,1));child.inherit_from(&None);let child=child.unwrap();
check_value(&child.mesh_scale,2,[1073741824, 1077936128, 1082130432]);
check_value(&child.summer_albedo,1,[1077936128, 1082130432, 1084227584]);
check_value(&child.autumn_albedo,2,[1065353216, 1065353216, 1065353216]);
check_value(&child.winter_albedo,2,[1065353216, 1065353216, 1065353216]);
assert_eq!(std::mem::discriminant(&child.mesh),std::mem::discriminant(&mesh(1)));assert_eq!(std::mem::discriminant(&child.semantic),std::mem::discriminant(&semantic(1)));assert_eq!(child.semantic.as_concrete(),semantic(1).as_concrete());}
{ let mut child=Some(make_rt([2, 1, 1, 0],[[1073741824, 1077936128, 1082130432], [1077936128, 1082130432, 1084227584], [1082130432, 1084227584, 1086324736], [1084227584, 1086324736, 1088421888]],2,1));child.inherit_from(&Some(make_rt([1, 1, 2, 1],[[1101004800, 1101529088, 1102053376], [1101529088, 1102053376, 1102577664], [1102053376, 1102577664, 1103101952], [1102577664, 1103101952, 1103626240]],1,3)));let child=child.unwrap();
check_value(&child.mesh_scale,2,[1073741824, 1077936128, 1082130432]);
check_value(&child.summer_albedo,1,[1077936128, 1082130432, 1084227584]);
check_value(&child.autumn_albedo,1,[1082130432, 1084227584, 1086324736]);
check_value(&child.winter_albedo,1,[1102577664, 1103101952, 1103626240]);
assert_eq!(std::mem::discriminant(&child.mesh),std::mem::discriminant(&mesh(2)));assert_eq!(std::mem::discriminant(&child.semantic),std::mem::discriminant(&semantic(1)));assert_eq!(child.semantic.as_concrete(),semantic(1).as_concrete());}
{ let mut child=Some(make_rt([2, 1, 1, 0],[[1073741824, 1077936128, 1082130432], [1077936128, 1082130432, 1084227584], [1082130432, 1084227584, 1086324736], [1084227584, 1086324736, 1088421888]],2,1));child.inherit_from(&None);let child=child.unwrap();
check_value(&child.mesh_scale,2,[1073741824, 1077936128, 1082130432]);
check_value(&child.summer_albedo,1,[1077936128, 1082130432, 1084227584]);
check_value(&child.autumn_albedo,1,[1082130432, 1084227584, 1086324736]);
check_value(&child.winter_albedo,2,[1065353216, 1065353216, 1065353216]);
assert_eq!(std::mem::discriminant(&child.mesh),std::mem::discriminant(&mesh(2)));assert_eq!(std::mem::discriminant(&child.semantic),std::mem::discriminant(&semantic(1)));assert_eq!(child.semantic.as_concrete(),semantic(1).as_concrete());}
{ let mut child=Some(make_rt([0, 0, 2, 0],[[1073741824, 1077936128, 1082130432], [1077936128, 1082130432, 1084227584], [1082130432, 1084227584, 1086324736], [1084227584, 1086324736, 1088421888]],1,3));child.inherit_from(&Some(make_rt([2, 2, 0, 2],[[1101004800, 1101529088, 1102053376], [1101529088, 1102053376, 1102577664], [1102053376, 1102577664, 1103101952], [1102577664, 1103101952, 1103626240]],2,1)));let child=child.unwrap();
check_value(&child.mesh_scale,2,[1101004800, 1101529088, 1102053376]);
check_value(&child.summer_albedo,2,[1101529088, 1102053376, 1102577664]);
check_value(&child.autumn_albedo,2,[1082130432, 1084227584, 1086324736]);
check_value(&child.winter_albedo,2,[1102577664, 1103101952, 1103626240]);
assert_eq!(std::mem::discriminant(&child.mesh),std::mem::discriminant(&mesh(1)));assert_eq!(std::mem::discriminant(&child.semantic),std::mem::discriminant(&semantic(3)));}
{ let mut child=Some(make_rt([0, 0, 2, 0],[[1073741824, 1077936128, 1082130432], [1077936128, 1082130432, 1084227584], [1082130432, 1084227584, 1086324736], [1084227584, 1086324736, 1088421888]],1,3));child.inherit_from(&None);let child=child.unwrap();
check_value(&child.mesh_scale,2,[1065353216, 1065353216, 1065353216]);
check_value(&child.summer_albedo,2,[1065353216, 1065353216, 1065353216]);
check_value(&child.autumn_albedo,2,[1082130432, 1084227584, 1086324736]);
check_value(&child.winter_albedo,2,[1065353216, 1065353216, 1065353216]);
assert_eq!(std::mem::discriminant(&child.mesh),std::mem::discriminant(&mesh(1)));assert_eq!(std::mem::discriminant(&child.semantic),std::mem::discriminant(&semantic(3)));}
{ let mut child=Some(make_rt([0, 1, 2, 0],[[1073741824, 1077936128, 1082130432], [1077936128, 1082130432, 1084227584], [1082130432, 1084227584, 1086324736], [1084227584, 1086324736, 1088421888]],1,3));child.inherit_from(&Some(make_rt([2, 0, 0, 2],[[1101004800, 1101529088, 1102053376], [1101529088, 1102053376, 1102577664], [1102053376, 1102577664, 1103101952], [1102577664, 1103101952, 1103626240]],2,2)));let child=child.unwrap();
check_value(&child.mesh_scale,2,[1101004800, 1101529088, 1102053376]);
check_value(&child.summer_albedo,1,[1077936128, 1082130432, 1084227584]);
check_value(&child.autumn_albedo,2,[1082130432, 1084227584, 1086324736]);
check_value(&child.winter_albedo,2,[1102577664, 1103101952, 1103626240]);
assert_eq!(std::mem::discriminant(&child.mesh),std::mem::discriminant(&mesh(1)));assert_eq!(std::mem::discriminant(&child.semantic),std::mem::discriminant(&semantic(3)));}
{ let mut child=Some(make_rt([0, 1, 2, 0],[[1073741824, 1077936128, 1082130432], [1077936128, 1082130432, 1084227584], [1082130432, 1084227584, 1086324736], [1084227584, 1086324736, 1088421888]],1,3));child.inherit_from(&None);let child=child.unwrap();
check_value(&child.mesh_scale,2,[1065353216, 1065353216, 1065353216]);
check_value(&child.summer_albedo,1,[1077936128, 1082130432, 1084227584]);
check_value(&child.autumn_albedo,2,[1082130432, 1084227584, 1086324736]);
check_value(&child.winter_albedo,2,[1065353216, 1065353216, 1065353216]);
assert_eq!(std::mem::discriminant(&child.mesh),std::mem::discriminant(&mesh(1)));assert_eq!(std::mem::discriminant(&child.semantic),std::mem::discriminant(&semantic(3)));}
{ let mut child=Some(make_rt([0, 1, 1, 1],[[1073741824, 1077936128, 1082130432], [1077936128, 1082130432, 1084227584], [1082130432, 1084227584, 1086324736], [1084227584, 1086324736, 1088421888]],2,2));child.inherit_from(&Some(make_rt([0, 0, 1, 2],[[1101004800, 1101529088, 1102053376], [1101529088, 1102053376, 1102577664], [1102053376, 1102577664, 1103101952], [1102577664, 1103101952, 1103626240]],0,1)));let child=child.unwrap();
check_value(&child.mesh_scale,2,[1065353216, 1065353216, 1065353216]);
check_value(&child.summer_albedo,1,[1077936128, 1082130432, 1084227584]);
check_value(&child.autumn_albedo,1,[1082130432, 1084227584, 1086324736]);
check_value(&child.winter_albedo,1,[1084227584, 1086324736, 1088421888]);
assert_eq!(std::mem::discriminant(&child.mesh),std::mem::discriminant(&mesh(2)));assert_eq!(std::mem::discriminant(&child.semantic),std::mem::discriminant(&semantic(1)));assert_eq!(child.semantic.as_concrete(),semantic(1).as_concrete());}
{ let mut child=Some(make_rt([0, 1, 1, 1],[[1073741824, 1077936128, 1082130432], [1077936128, 1082130432, 1084227584], [1082130432, 1084227584, 1086324736], [1084227584, 1086324736, 1088421888]],2,2));child.inherit_from(&None);let child=child.unwrap();
check_value(&child.mesh_scale,2,[1065353216, 1065353216, 1065353216]);
check_value(&child.summer_albedo,1,[1077936128, 1082130432, 1084227584]);
check_value(&child.autumn_albedo,1,[1082130432, 1084227584, 1086324736]);
check_value(&child.winter_albedo,1,[1084227584, 1086324736, 1088421888]);
assert_eq!(std::mem::discriminant(&child.mesh),std::mem::discriminant(&mesh(2)));assert_eq!(std::mem::discriminant(&child.semantic),std::mem::discriminant(&semantic(0)));assert_eq!(child.semantic.as_concrete(),semantic(0).as_concrete());}
{ let mut child=Some(make_rt([0, 2, 2, 0],[[1073741824, 1077936128, 1082130432], [1077936128, 1082130432, 1084227584], [1082130432, 1084227584, 1086324736], [1084227584, 1086324736, 1088421888]],1,1));child.inherit_from(&Some(make_rt([1, 1, 0, 2],[[1101004800, 1101529088, 1102053376], [1101529088, 1102053376, 1102577664], [1102053376, 1102577664, 1103101952], [1102577664, 1103101952, 1103626240]],2,3)));let child=child.unwrap();
check_value(&child.mesh_scale,1,[1102577664, 1103101952, 1103626240]);
check_value(&child.summer_albedo,2,[1077936128, 1082130432, 1084227584]);
check_value(&child.autumn_albedo,2,[1082130432, 1084227584, 1086324736]);
check_value(&child.winter_albedo,2,[1102577664, 1103101952, 1103626240]);
assert_eq!(std::mem::discriminant(&child.mesh),std::mem::discriminant(&mesh(1)));assert_eq!(std::mem::discriminant(&child.semantic),std::mem::discriminant(&semantic(1)));assert_eq!(child.semantic.as_concrete(),semantic(1).as_concrete());}
{ let mut child=Some(make_rt([0, 2, 2, 0],[[1073741824, 1077936128, 1082130432], [1077936128, 1082130432, 1084227584], [1082130432, 1084227584, 1086324736], [1084227584, 1086324736, 1088421888]],1,1));child.inherit_from(&None);let child=child.unwrap();
check_value(&child.mesh_scale,2,[1065353216, 1065353216, 1065353216]);
check_value(&child.summer_albedo,2,[1077936128, 1082130432, 1084227584]);
check_value(&child.autumn_albedo,2,[1082130432, 1084227584, 1086324736]);
check_value(&child.winter_albedo,2,[1065353216, 1065353216, 1065353216]);
assert_eq!(std::mem::discriminant(&child.mesh),std::mem::discriminant(&mesh(1)));assert_eq!(std::mem::discriminant(&child.semantic),std::mem::discriminant(&semantic(1)));assert_eq!(child.semantic.as_concrete(),semantic(1).as_concrete());}
{ let mut child=Some(make_rt([0, 1, 2, 0],[[1073741824, 1077936128, 1082130432], [1077936128, 1082130432, 1084227584], [1082130432, 1084227584, 1086324736], [1084227584, 1086324736, 1088421888]],0,3));child.inherit_from(&Some(make_rt([1, 0, 1, 0],[[1101004800, 1101529088, 1102053376], [1101529088, 1102053376, 1102577664], [1102053376, 1102577664, 1103101952], [1102577664, 1103101952, 1103626240]],0,3)));let child=child.unwrap();
check_value(&child.mesh_scale,1,[1102577664, 1103101952, 1103626240]);
check_value(&child.summer_albedo,1,[1077936128, 1082130432, 1084227584]);
check_value(&child.autumn_albedo,2,[1082130432, 1084227584, 1086324736]);
check_value(&child.winter_albedo,2,[1065353216, 1065353216, 1065353216]);
assert_eq!(std::mem::discriminant(&child.mesh),std::mem::discriminant(&mesh(1)));assert_eq!(std::mem::discriminant(&child.semantic),std::mem::discriminant(&semantic(3)));}
{ let mut child=Some(make_rt([0, 1, 2, 0],[[1073741824, 1077936128, 1082130432], [1077936128, 1082130432, 1084227584], [1082130432, 1084227584, 1086324736], [1084227584, 1086324736, 1088421888]],0,3));child.inherit_from(&None);let child=child.unwrap();
check_value(&child.mesh_scale,2,[1065353216, 1065353216, 1065353216]);
check_value(&child.summer_albedo,1,[1077936128, 1082130432, 1084227584]);
check_value(&child.autumn_albedo,2,[1082130432, 1084227584, 1086324736]);
check_value(&child.winter_albedo,2,[1065353216, 1065353216, 1065353216]);
assert_eq!(std::mem::discriminant(&child.mesh),std::mem::discriminant(&mesh(1)));assert_eq!(std::mem::discriminant(&child.semantic),std::mem::discriminant(&semantic(3)));}
{ let mut child=Some(make_rt([1, 1, 1, 2],[[1073741824, 1077936128, 1082130432], [1077936128, 1082130432, 1084227584], [1082130432, 1084227584, 1086324736], [1084227584, 1086324736, 1088421888]],1,0));child.inherit_from(&Some(make_rt([2, 1, 0, 2],[[1101004800, 1101529088, 1102053376], [1101529088, 1102053376, 1102577664], [1102053376, 1102577664, 1103101952], [1102577664, 1103101952, 1103626240]],1,1)));let child=child.unwrap();
check_value(&child.mesh_scale,1,[1073741824, 1077936128, 1082130432]);
check_value(&child.summer_albedo,1,[1077936128, 1082130432, 1084227584]);
check_value(&child.autumn_albedo,1,[1082130432, 1084227584, 1086324736]);
check_value(&child.winter_albedo,2,[1084227584, 1086324736, 1088421888]);
assert_eq!(std::mem::discriminant(&child.mesh),std::mem::discriminant(&mesh(1)));assert_eq!(std::mem::discriminant(&child.semantic),std::mem::discriminant(&semantic(0)));assert_eq!(child.semantic.as_concrete(),semantic(0).as_concrete());}
{ let mut child=Some(make_rt([1, 1, 1, 2],[[1073741824, 1077936128, 1082130432], [1077936128, 1082130432, 1084227584], [1082130432, 1084227584, 1086324736], [1084227584, 1086324736, 1088421888]],1,0));child.inherit_from(&None);let child=child.unwrap();
check_value(&child.mesh_scale,1,[1073741824, 1077936128, 1082130432]);
check_value(&child.summer_albedo,1,[1077936128, 1082130432, 1084227584]);
check_value(&child.autumn_albedo,1,[1082130432, 1084227584, 1086324736]);
check_value(&child.winter_albedo,2,[1084227584, 1086324736, 1088421888]);
assert_eq!(std::mem::discriminant(&child.mesh),std::mem::discriminant(&mesh(1)));assert_eq!(std::mem::discriminant(&child.semantic),std::mem::discriminant(&semantic(0)));assert_eq!(child.semantic.as_concrete(),semantic(0).as_concrete());}
{ let mut child=Some(make_rt([2, 0, 0, 2],[[1073741824, 1077936128, 1082130432], [1077936128, 1082130432, 1084227584], [1082130432, 1084227584, 1086324736], [1084227584, 1086324736, 1088421888]],2,2));child.inherit_from(&Some(make_rt([2, 1, 1, 0],[[1101004800, 1101529088, 1102053376], [1101529088, 1102053376, 1102577664], [1102053376, 1102577664, 1103101952], [1102577664, 1103101952, 1103626240]],0,0)));let child=child.unwrap();
check_value(&child.mesh_scale,2,[1073741824, 1077936128, 1082130432]);
check_value(&child.summer_albedo,1,[1102577664, 1103101952, 1103626240]);
check_value(&child.autumn_albedo,1,[1102577664, 1103101952, 1103626240]);
check_value(&child.winter_albedo,2,[1084227584, 1086324736, 1088421888]);
assert_eq!(std::mem::discriminant(&child.mesh),std::mem::discriminant(&mesh(2)));assert_eq!(std::mem::discriminant(&child.semantic),std::mem::discriminant(&semantic(0)));assert_eq!(child.semantic.as_concrete(),semantic(0).as_concrete());}
{ let mut child=Some(make_rt([2, 0, 0, 2],[[1073741824, 1077936128, 1082130432], [1077936128, 1082130432, 1084227584], [1082130432, 1084227584, 1086324736], [1084227584, 1086324736, 1088421888]],2,2));child.inherit_from(&None);let child=child.unwrap();
check_value(&child.mesh_scale,2,[1073741824, 1077936128, 1082130432]);
check_value(&child.summer_albedo,2,[1065353216, 1065353216, 1065353216]);
check_value(&child.autumn_albedo,2,[1065353216, 1065353216, 1065353216]);
check_value(&child.winter_albedo,2,[1084227584, 1086324736, 1088421888]);
assert_eq!(std::mem::discriminant(&child.mesh),std::mem::discriminant(&mesh(2)));assert_eq!(std::mem::discriminant(&child.semantic),std::mem::discriminant(&semantic(0)));assert_eq!(child.semantic.as_concrete(),semantic(0).as_concrete());}
{ let mut child=Some(make_rt([1, 2, 1, 2],[[1073741824, 1077936128, 1082130432], [1077936128, 1082130432, 1084227584], [1082130432, 1084227584, 1086324736], [1084227584, 1086324736, 1088421888]],2,0));child.inherit_from(&Some(make_rt([1, 2, 0, 0],[[1101004800, 1101529088, 1102053376], [1101529088, 1102053376, 1102577664], [1102053376, 1102577664, 1103101952], [1102577664, 1103101952, 1103626240]],0,2)));let child=child.unwrap();
check_value(&child.mesh_scale,1,[1073741824, 1077936128, 1082130432]);
check_value(&child.summer_albedo,2,[1077936128, 1082130432, 1084227584]);
check_value(&child.autumn_albedo,1,[1082130432, 1084227584, 1086324736]);
check_value(&child.winter_albedo,2,[1084227584, 1086324736, 1088421888]);
assert_eq!(std::mem::discriminant(&child.mesh),std::mem::discriminant(&mesh(2)));assert_eq!(std::mem::discriminant(&child.semantic),std::mem::discriminant(&semantic(0)));assert_eq!(child.semantic.as_concrete(),semantic(0).as_concrete());}
{ let mut child=Some(make_rt([1, 2, 1, 2],[[1073741824, 1077936128, 1082130432], [1077936128, 1082130432, 1084227584], [1082130432, 1084227584, 1086324736], [1084227584, 1086324736, 1088421888]],2,0));child.inherit_from(&None);let child=child.unwrap();
check_value(&child.mesh_scale,1,[1073741824, 1077936128, 1082130432]);
check_value(&child.summer_albedo,2,[1077936128, 1082130432, 1084227584]);
check_value(&child.autumn_albedo,1,[1082130432, 1084227584, 1086324736]);
check_value(&child.winter_albedo,2,[1084227584, 1086324736, 1088421888]);
assert_eq!(std::mem::discriminant(&child.mesh),std::mem::discriminant(&mesh(2)));assert_eq!(std::mem::discriminant(&child.semantic),std::mem::discriminant(&semantic(0)));assert_eq!(child.semantic.as_concrete(),semantic(0).as_concrete());}
{ let mut child=Some(make_rt([1, 0, 2, 2],[[1073741824, 1077936128, 1082130432], [1077936128, 1082130432, 1084227584], [1082130432, 1084227584, 1086324736], [1084227584, 1086324736, 1088421888]],0,3));child.inherit_from(&Some(make_rt([0, 2, 2, 2],[[1101004800, 1101529088, 1102053376], [1101529088, 1102053376, 1102577664], [1102053376, 1102577664, 1103101952], [1102577664, 1103101952, 1103626240]],2,2)));let child=child.unwrap();
check_value(&child.mesh_scale,1,[1073741824, 1077936128, 1082130432]);
check_value(&child.summer_albedo,2,[1101529088, 1102053376, 1102577664]);
check_value(&child.autumn_albedo,2,[1082130432, 1084227584, 1086324736]);
check_value(&child.winter_albedo,2,[1084227584, 1086324736, 1088421888]);
assert_eq!(std::mem::discriminant(&child.mesh),std::mem::discriminant(&mesh(2)));assert_eq!(std::mem::discriminant(&child.semantic),std::mem::discriminant(&semantic(3)));}
{ let mut child=Some(make_rt([1, 0, 2, 2],[[1073741824, 1077936128, 1082130432], [1077936128, 1082130432, 1084227584], [1082130432, 1084227584, 1086324736], [1084227584, 1086324736, 1088421888]],0,3));child.inherit_from(&None);let child=child.unwrap();
check_value(&child.mesh_scale,1,[1073741824, 1077936128, 1082130432]);
check_value(&child.summer_albedo,2,[1065353216, 1065353216, 1065353216]);
check_value(&child.autumn_albedo,2,[1082130432, 1084227584, 1086324736]);
check_value(&child.winter_albedo,2,[1084227584, 1086324736, 1088421888]);
assert_eq!(std::mem::discriminant(&child.mesh),std::mem::discriminant(&mesh(1)));assert_eq!(std::mem::discriminant(&child.semantic),std::mem::discriminant(&semantic(3)));}
{ let mut child=Some(make_rt([2, 1, 0, 2],[[1073741824, 1077936128, 1082130432], [1077936128, 1082130432, 1084227584], [1082130432, 1084227584, 1086324736], [1084227584, 1086324736, 1088421888]],2,0));child.inherit_from(&Some(make_rt([1, 0, 2, 0],[[1101004800, 1101529088, 1102053376], [1101529088, 1102053376, 1102577664], [1102053376, 1102577664, 1103101952], [1102577664, 1103101952, 1103626240]],1,3)));let child=child.unwrap();
check_value(&child.mesh_scale,2,[1073741824, 1077936128, 1082130432]);
check_value(&child.summer_albedo,1,[1077936128, 1082130432, 1084227584]);
check_value(&child.autumn_albedo,2,[1102053376, 1102577664, 1103101952]);
check_value(&child.winter_albedo,2,[1084227584, 1086324736, 1088421888]);
assert_eq!(std::mem::discriminant(&child.mesh),std::mem::discriminant(&mesh(2)));assert_eq!(std::mem::discriminant(&child.semantic),std::mem::discriminant(&semantic(0)));assert_eq!(child.semantic.as_concrete(),semantic(0).as_concrete());}
{ let mut child=Some(make_rt([2, 1, 0, 2],[[1073741824, 1077936128, 1082130432], [1077936128, 1082130432, 1084227584], [1082130432, 1084227584, 1086324736], [1084227584, 1086324736, 1088421888]],2,0));child.inherit_from(&None);let child=child.unwrap();
check_value(&child.mesh_scale,2,[1073741824, 1077936128, 1082130432]);
check_value(&child.summer_albedo,1,[1077936128, 1082130432, 1084227584]);
check_value(&child.autumn_albedo,2,[1065353216, 1065353216, 1065353216]);
check_value(&child.winter_albedo,2,[1084227584, 1086324736, 1088421888]);
assert_eq!(std::mem::discriminant(&child.mesh),std::mem::discriminant(&mesh(2)));assert_eq!(std::mem::discriminant(&child.semantic),std::mem::discriminant(&semantic(0)));assert_eq!(child.semantic.as_concrete(),semantic(0).as_concrete());}
{ let mut child=Some(make_rt([0, 1, 0, 2],[[1073741824, 1077936128, 1082130432], [1077936128, 1082130432, 1084227584], [1082130432, 1084227584, 1086324736], [1084227584, 1086324736, 1088421888]],1,3));child.inherit_from(&Some(make_rt([1, 0, 0, 1],[[1101004800, 1101529088, 1102053376], [1101529088, 1102053376, 1102577664], [1102053376, 1102577664, 1103101952], [1102577664, 1103101952, 1103626240]],1,1)));let child=child.unwrap();
check_value(&child.mesh_scale,1,[1102053376, 1102577664, 1103101952]);
check_value(&child.summer_albedo,1,[1077936128, 1082130432, 1084227584]);
check_value(&child.autumn_albedo,2,[1065353216, 1065353216, 1065353216]);
check_value(&child.winter_albedo,2,[1084227584, 1086324736, 1088421888]);
assert_eq!(std::mem::discriminant(&child.mesh),std::mem::discriminant(&mesh(1)));assert_eq!(std::mem::discriminant(&child.semantic),std::mem::discriminant(&semantic(3)));}
{ let mut child=Some(make_rt([0, 1, 0, 2],[[1073741824, 1077936128, 1082130432], [1077936128, 1082130432, 1084227584], [1082130432, 1084227584, 1086324736], [1084227584, 1086324736, 1088421888]],1,3));child.inherit_from(&None);let child=child.unwrap();
check_value(&child.mesh_scale,2,[1065353216, 1065353216, 1065353216]);
check_value(&child.summer_albedo,1,[1077936128, 1082130432, 1084227584]);
check_value(&child.autumn_albedo,2,[1065353216, 1065353216, 1065353216]);
check_value(&child.winter_albedo,2,[1084227584, 1086324736, 1088421888]);
assert_eq!(std::mem::discriminant(&child.mesh),std::mem::discriminant(&mesh(1)));assert_eq!(std::mem::discriminant(&child.semantic),std::mem::discriminant(&semantic(3)));}
{ let mut child=Some(make_rt([1, 1, 2, 1],[[1073741824, 1077936128, 1082130432], [1077936128, 1082130432, 1084227584], [1082130432, 1084227584, 1086324736], [1084227584, 1086324736, 1088421888]],0,0));child.inherit_from(&Some(make_rt([1, 2, 2, 2],[[1101004800, 1101529088, 1102053376], [1101529088, 1102053376, 1102577664], [1102053376, 1102577664, 1103101952], [1102577664, 1103101952, 1103626240]],2,1)));let child=child.unwrap();
check_value(&child.mesh_scale,1,[1073741824, 1077936128, 1082130432]);
check_value(&child.summer_albedo,1,[1077936128, 1082130432, 1084227584]);
check_value(&child.autumn_albedo,2,[1082130432, 1084227584, 1086324736]);
check_value(&child.winter_albedo,1,[1084227584, 1086324736, 1088421888]);
assert_eq!(std::mem::discriminant(&child.mesh),std::mem::discriminant(&mesh(2)));assert_eq!(std::mem::discriminant(&child.semantic),std::mem::discriminant(&semantic(0)));assert_eq!(child.semantic.as_concrete(),semantic(0).as_concrete());}
{ let mut child=Some(make_rt([1, 1, 2, 1],[[1073741824, 1077936128, 1082130432], [1077936128, 1082130432, 1084227584], [1082130432, 1084227584, 1086324736], [1084227584, 1086324736, 1088421888]],0,0));child.inherit_from(&None);let child=child.unwrap();
check_value(&child.mesh_scale,1,[1073741824, 1077936128, 1082130432]);
check_value(&child.summer_albedo,1,[1077936128, 1082130432, 1084227584]);
check_value(&child.autumn_albedo,2,[1082130432, 1084227584, 1086324736]);
check_value(&child.winter_albedo,1,[1084227584, 1086324736, 1088421888]);
assert_eq!(std::mem::discriminant(&child.mesh),std::mem::discriminant(&mesh(1)));assert_eq!(std::mem::discriminant(&child.semantic),std::mem::discriminant(&semantic(0)));assert_eq!(child.semantic.as_concrete(),semantic(0).as_concrete());}
{ let mut child=Some(make_rt([0, 2, 0, 2],[[1073741824, 1077936128, 1082130432], [1077936128, 1082130432, 1084227584], [1082130432, 1084227584, 1086324736], [1084227584, 1086324736, 1088421888]],0,1));child.inherit_from(&Some(make_rt([2, 0, 0, 0],[[1101004800, 1101529088, 1102053376], [1101529088, 1102053376, 1102577664], [1102053376, 1102577664, 1103101952], [1102577664, 1103101952, 1103626240]],2,1)));let child=child.unwrap();
check_value(&child.mesh_scale,2,[1101004800, 1101529088, 1102053376]);
check_value(&child.summer_albedo,2,[1077936128, 1082130432, 1084227584]);
check_value(&child.autumn_albedo,2,[1065353216, 1065353216, 1065353216]);
check_value(&child.winter_albedo,2,[1084227584, 1086324736, 1088421888]);
assert_eq!(std::mem::discriminant(&child.mesh),std::mem::discriminant(&mesh(2)));assert_eq!(std::mem::discriminant(&child.semantic),std::mem::discriminant(&semantic(1)));assert_eq!(child.semantic.as_concrete(),semantic(1).as_concrete());}
{ let mut child=Some(make_rt([0, 2, 0, 2],[[1073741824, 1077936128, 1082130432], [1077936128, 1082130432, 1084227584], [1082130432, 1084227584, 1086324736], [1084227584, 1086324736, 1088421888]],0,1));child.inherit_from(&None);let child=child.unwrap();
check_value(&child.mesh_scale,2,[1065353216, 1065353216, 1065353216]);
check_value(&child.summer_albedo,2,[1077936128, 1082130432, 1084227584]);
check_value(&child.autumn_albedo,2,[1065353216, 1065353216, 1065353216]);
check_value(&child.winter_albedo,2,[1084227584, 1086324736, 1088421888]);
assert_eq!(std::mem::discriminant(&child.mesh),std::mem::discriminant(&mesh(1)));assert_eq!(std::mem::discriminant(&child.semantic),std::mem::discriminant(&semantic(1)));assert_eq!(child.semantic.as_concrete(),semantic(1).as_concrete());}
{ let mut child=Some(make_rt([0, 1, 0, 2],[[1073741824, 1077936128, 1082130432], [1077936128, 1082130432, 1084227584], [1082130432, 1084227584, 1086324736], [1084227584, 1086324736, 1088421888]],2,3));child.inherit_from(&Some(make_rt([1, 1, 0, 1],[[1101004800, 1101529088, 1102053376], [1101529088, 1102053376, 1102577664], [1102053376, 1102577664, 1103101952], [1102577664, 1103101952, 1103626240]],1,2)));let child=child.unwrap();
check_value(&child.mesh_scale,1,[1101004800, 1101529088, 1102053376]);
check_value(&child.summer_albedo,1,[1077936128, 1082130432, 1084227584]);
check_value(&child.autumn_albedo,2,[1065353216, 1065353216, 1065353216]);
check_value(&child.winter_albedo,2,[1084227584, 1086324736, 1088421888]);
assert_eq!(std::mem::discriminant(&child.mesh),std::mem::discriminant(&mesh(2)));assert_eq!(std::mem::discriminant(&child.semantic),std::mem::discriminant(&semantic(3)));}
{ let mut child=Some(make_rt([0, 1, 0, 2],[[1073741824, 1077936128, 1082130432], [1077936128, 1082130432, 1084227584], [1082130432, 1084227584, 1086324736], [1084227584, 1086324736, 1088421888]],2,3));child.inherit_from(&None);let child=child.unwrap();
check_value(&child.mesh_scale,2,[1065353216, 1065353216, 1065353216]);
check_value(&child.summer_albedo,1,[1077936128, 1082130432, 1084227584]);
check_value(&child.autumn_albedo,2,[1065353216, 1065353216, 1065353216]);
check_value(&child.winter_albedo,2,[1084227584, 1086324736, 1088421888]);
assert_eq!(std::mem::discriminant(&child.mesh),std::mem::discriminant(&mesh(2)));assert_eq!(std::mem::discriminant(&child.semantic),std::mem::discriminant(&semantic(3)));}
{ let mut child=Some(make_rt([1, 1, 1, 0],[[1073741824, 1077936128, 1082130432], [1077936128, 1082130432, 1084227584], [1082130432, 1084227584, 1086324736], [1084227584, 1086324736, 1088421888]],0,0));child.inherit_from(&Some(make_rt([2, 1, 1, 0],[[1101004800, 1101529088, 1102053376], [1101529088, 1102053376, 1102577664], [1102053376, 1102577664, 1103101952], [1102577664, 1103101952, 1103626240]],2,1)));let child=child.unwrap();
check_value(&child.mesh_scale,1,[1073741824, 1077936128, 1082130432]);
check_value(&child.summer_albedo,1,[1077936128, 1082130432, 1084227584]);
check_value(&child.autumn_albedo,1,[1082130432, 1084227584, 1086324736]);
check_value(&child.winter_albedo,2,[1065353216, 1065353216, 1065353216]);
assert_eq!(std::mem::discriminant(&child.mesh),std::mem::discriminant(&mesh(2)));assert_eq!(std::mem::discriminant(&child.semantic),std::mem::discriminant(&semantic(0)));assert_eq!(child.semantic.as_concrete(),semantic(0).as_concrete());}
{ let mut child=Some(make_rt([1, 1, 1, 0],[[1073741824, 1077936128, 1082130432], [1077936128, 1082130432, 1084227584], [1082130432, 1084227584, 1086324736], [1084227584, 1086324736, 1088421888]],0,0));child.inherit_from(&None);let child=child.unwrap();
check_value(&child.mesh_scale,1,[1073741824, 1077936128, 1082130432]);
check_value(&child.summer_albedo,1,[1077936128, 1082130432, 1084227584]);
check_value(&child.autumn_albedo,1,[1082130432, 1084227584, 1086324736]);
check_value(&child.winter_albedo,2,[1065353216, 1065353216, 1065353216]);
assert_eq!(std::mem::discriminant(&child.mesh),std::mem::discriminant(&mesh(1)));assert_eq!(std::mem::discriminant(&child.semantic),std::mem::discriminant(&semantic(0)));assert_eq!(child.semantic.as_concrete(),semantic(0).as_concrete());}
{ let mut child=Some(make_rt([0, 1, 1, 2],[[1073741824, 1077936128, 1082130432], [1077936128, 1082130432, 1084227584], [1082130432, 1084227584, 1086324736], [1084227584, 1086324736, 1088421888]],2,1));child.inherit_from(&Some(make_rt([1, 0, 0, 0],[[1101004800, 1101529088, 1102053376], [1101529088, 1102053376, 1102577664], [1102053376, 1102577664, 1103101952], [1102577664, 1103101952, 1103626240]],1,3)));let child=child.unwrap();
check_value(&child.mesh_scale,1,[1101004800, 1101529088, 1102053376]);
check_value(&child.summer_albedo,1,[1077936128, 1082130432, 1084227584]);
check_value(&child.autumn_albedo,1,[1082130432, 1084227584, 1086324736]);
check_value(&child.winter_albedo,2,[1084227584, 1086324736, 1088421888]);
assert_eq!(std::mem::discriminant(&child.mesh),std::mem::discriminant(&mesh(2)));assert_eq!(std::mem::discriminant(&child.semantic),std::mem::discriminant(&semantic(1)));assert_eq!(child.semantic.as_concrete(),semantic(1).as_concrete());}
{ let mut child=Some(make_rt([0, 1, 1, 2],[[1073741824, 1077936128, 1082130432], [1077936128, 1082130432, 1084227584], [1082130432, 1084227584, 1086324736], [1084227584, 1086324736, 1088421888]],2,1));child.inherit_from(&None);let child=child.unwrap();
check_value(&child.mesh_scale,2,[1065353216, 1065353216, 1065353216]);
check_value(&child.summer_albedo,1,[1077936128, 1082130432, 1084227584]);
check_value(&child.autumn_albedo,1,[1082130432, 1084227584, 1086324736]);
check_value(&child.winter_albedo,2,[1084227584, 1086324736, 1088421888]);
assert_eq!(std::mem::discriminant(&child.mesh),std::mem::discriminant(&mesh(2)));assert_eq!(std::mem::discriminant(&child.semantic),std::mem::discriminant(&semantic(1)));assert_eq!(child.semantic.as_concrete(),semantic(1).as_concrete());}
{ let mut child=Some(make_rt([1, 1, 2, 1],[[1073741824, 1077936128, 1082130432], [1077936128, 1082130432, 1084227584], [1082130432, 1084227584, 1086324736], [1084227584, 1086324736, 1088421888]],1,3));child.inherit_from(&Some(make_rt([2, 1, 2, 0],[[1101004800, 1101529088, 1102053376], [1101529088, 1102053376, 1102577664], [1102053376, 1102577664, 1103101952], [1102577664, 1103101952, 1103626240]],1,1)));let child=child.unwrap();
check_value(&child.mesh_scale,1,[1073741824, 1077936128, 1082130432]);
check_value(&child.summer_albedo,1,[1077936128, 1082130432, 1084227584]);
check_value(&child.autumn_albedo,2,[1082130432, 1084227584, 1086324736]);
check_value(&child.winter_albedo,1,[1084227584, 1086324736, 1088421888]);
assert_eq!(std::mem::discriminant(&child.mesh),std::mem::discriminant(&mesh(1)));assert_eq!(std::mem::discriminant(&child.semantic),std::mem::discriminant(&semantic(3)));}
{ let mut child=Some(make_rt([1, 1, 2, 1],[[1073741824, 1077936128, 1082130432], [1077936128, 1082130432, 1084227584], [1082130432, 1084227584, 1086324736], [1084227584, 1086324736, 1088421888]],1,3));child.inherit_from(&None);let child=child.unwrap();
check_value(&child.mesh_scale,1,[1073741824, 1077936128, 1082130432]);
check_value(&child.summer_albedo,1,[1077936128, 1082130432, 1084227584]);
check_value(&child.autumn_albedo,2,[1082130432, 1084227584, 1086324736]);
check_value(&child.winter_albedo,1,[1084227584, 1086324736, 1088421888]);
assert_eq!(std::mem::discriminant(&child.mesh),std::mem::discriminant(&mesh(1)));assert_eq!(std::mem::discriminant(&child.semantic),std::mem::discriminant(&semantic(3)));}
{ let mut child=Some(make_rt([2, 1, 0, 1],[[1073741824, 1077936128, 1082130432], [1077936128, 1082130432, 1084227584], [1082130432, 1084227584, 1086324736], [1084227584, 1086324736, 1088421888]],1,2));child.inherit_from(&Some(make_rt([2, 2, 2, 1],[[1101004800, 1101529088, 1102053376], [1101529088, 1102053376, 1102577664], [1102053376, 1102577664, 1103101952], [1102577664, 1103101952, 1103626240]],2,1)));let child=child.unwrap();
check_value(&child.mesh_scale,2,[1073741824, 1077936128, 1082130432]);
check_value(&child.summer_albedo,1,[1077936128, 1082130432, 1084227584]);
check_value(&child.autumn_albedo,2,[1102053376, 1102577664, 1103101952]);
check_value(&child.winter_albedo,1,[1084227584, 1086324736, 1088421888]);
assert_eq!(std::mem::discriminant(&child.mesh),std::mem::discriminant(&mesh(1)));assert_eq!(std::mem::discriminant(&child.semantic),std::mem::discriminant(&semantic(1)));assert_eq!(child.semantic.as_concrete(),semantic(1).as_concrete());}
{ let mut child=Some(make_rt([2, 1, 0, 1],[[1073741824, 1077936128, 1082130432], [1077936128, 1082130432, 1084227584], [1082130432, 1084227584, 1086324736], [1084227584, 1086324736, 1088421888]],1,2));child.inherit_from(&None);let child=child.unwrap();
check_value(&child.mesh_scale,2,[1073741824, 1077936128, 1082130432]);
check_value(&child.summer_albedo,1,[1077936128, 1082130432, 1084227584]);
check_value(&child.autumn_albedo,2,[1065353216, 1065353216, 1065353216]);
check_value(&child.winter_albedo,1,[1084227584, 1086324736, 1088421888]);
assert_eq!(std::mem::discriminant(&child.mesh),std::mem::discriminant(&mesh(1)));assert_eq!(std::mem::discriminant(&child.semantic),std::mem::discriminant(&semantic(0)));assert_eq!(child.semantic.as_concrete(),semantic(0).as_concrete());}
{ let mut child=Some(make_rt([2, 2, 2, 2],[[1073741824, 1077936128, 1082130432], [1077936128, 1082130432, 1084227584], [1082130432, 1084227584, 1086324736], [1084227584, 1086324736, 1088421888]],1,3));child.inherit_from(&Some(make_rt([0, 0, 2, 0],[[1101004800, 1101529088, 1102053376], [1101529088, 1102053376, 1102577664], [1102053376, 1102577664, 1103101952], [1102577664, 1103101952, 1103626240]],2,2)));let child=child.unwrap();
check_value(&child.mesh_scale,2,[1073741824, 1077936128, 1082130432]);
check_value(&child.summer_albedo,2,[1077936128, 1082130432, 1084227584]);
check_value(&child.autumn_albedo,2,[1082130432, 1084227584, 1086324736]);
check_value(&child.winter_albedo,2,[1084227584, 1086324736, 1088421888]);
assert_eq!(std::mem::discriminant(&child.mesh),std::mem::discriminant(&mesh(1)));assert_eq!(std::mem::discriminant(&child.semantic),std::mem::discriminant(&semantic(3)));}
{ let mut child=Some(make_rt([2, 2, 2, 2],[[1073741824, 1077936128, 1082130432], [1077936128, 1082130432, 1084227584], [1082130432, 1084227584, 1086324736], [1084227584, 1086324736, 1088421888]],1,3));child.inherit_from(&None);let child=child.unwrap();
check_value(&child.mesh_scale,2,[1073741824, 1077936128, 1082130432]);
check_value(&child.summer_albedo,2,[1077936128, 1082130432, 1084227584]);
check_value(&child.autumn_albedo,2,[1082130432, 1084227584, 1086324736]);
check_value(&child.winter_albedo,2,[1084227584, 1086324736, 1088421888]);
assert_eq!(std::mem::discriminant(&child.mesh),std::mem::discriminant(&mesh(1)));assert_eq!(std::mem::discriminant(&child.semantic),std::mem::discriminant(&semantic(3)));}
{ let mut child=Some(make_rt([2, 1, 0, 2],[[1073741824, 1077936128, 1082130432], [1077936128, 1082130432, 1084227584], [1082130432, 1084227584, 1086324736], [1084227584, 1086324736, 1088421888]],2,2));child.inherit_from(&Some(make_rt([2, 1, 0, 0],[[1101004800, 1101529088, 1102053376], [1101529088, 1102053376, 1102577664], [1102053376, 1102577664, 1103101952], [1102577664, 1103101952, 1103626240]],1,0)));let child=child.unwrap();
check_value(&child.mesh_scale,2,[1073741824, 1077936128, 1082130432]);
check_value(&child.summer_albedo,1,[1077936128, 1082130432, 1084227584]);
check_value(&child.autumn_albedo,2,[1065353216, 1065353216, 1065353216]);
check_value(&child.winter_albedo,2,[1084227584, 1086324736, 1088421888]);
assert_eq!(std::mem::discriminant(&child.mesh),std::mem::discriminant(&mesh(2)));assert_eq!(std::mem::discriminant(&child.semantic),std::mem::discriminant(&semantic(0)));assert_eq!(child.semantic.as_concrete(),semantic(0).as_concrete());}
{ let mut child=Some(make_rt([2, 1, 0, 2],[[1073741824, 1077936128, 1082130432], [1077936128, 1082130432, 1084227584], [1082130432, 1084227584, 1086324736], [1084227584, 1086324736, 1088421888]],2,2));child.inherit_from(&None);let child=child.unwrap();
check_value(&child.mesh_scale,2,[1073741824, 1077936128, 1082130432]);
check_value(&child.summer_albedo,1,[1077936128, 1082130432, 1084227584]);
check_value(&child.autumn_albedo,2,[1065353216, 1065353216, 1065353216]);
check_value(&child.winter_albedo,2,[1084227584, 1086324736, 1088421888]);
assert_eq!(std::mem::discriminant(&child.mesh),std::mem::discriminant(&mesh(2)));assert_eq!(std::mem::discriminant(&child.semantic),std::mem::discriminant(&semantic(0)));assert_eq!(child.semantic.as_concrete(),semantic(0).as_concrete());}
{ let mut child=Some(make_rt([0, 0, 2, 2],[[1073741824, 1077936128, 1082130432], [1077936128, 1082130432, 1084227584], [1082130432, 1084227584, 1086324736], [1084227584, 1086324736, 1088421888]],1,1));child.inherit_from(&Some(make_rt([0, 2, 2, 2],[[1101004800, 1101529088, 1102053376], [1101529088, 1102053376, 1102577664], [1102053376, 1102577664, 1103101952], [1102577664, 1103101952, 1103626240]],0,2)));let child=child.unwrap();
check_value(&child.mesh_scale,2,[1065353216, 1065353216, 1065353216]);
check_value(&child.summer_albedo,2,[1101529088, 1102053376, 1102577664]);
check_value(&child.autumn_albedo,2,[1082130432, 1084227584, 1086324736]);
check_value(&child.winter_albedo,2,[1084227584, 1086324736, 1088421888]);
assert_eq!(std::mem::discriminant(&child.mesh),std::mem::discriminant(&mesh(1)));assert_eq!(std::mem::discriminant(&child.semantic),std::mem::discriminant(&semantic(1)));assert_eq!(child.semantic.as_concrete(),semantic(1).as_concrete());}
{ let mut child=Some(make_rt([0, 0, 2, 2],[[1073741824, 1077936128, 1082130432], [1077936128, 1082130432, 1084227584], [1082130432, 1084227584, 1086324736], [1084227584, 1086324736, 1088421888]],1,1));child.inherit_from(&None);let child=child.unwrap();
check_value(&child.mesh_scale,2,[1065353216, 1065353216, 1065353216]);
check_value(&child.summer_albedo,2,[1065353216, 1065353216, 1065353216]);
check_value(&child.autumn_albedo,2,[1082130432, 1084227584, 1086324736]);
check_value(&child.winter_albedo,2,[1084227584, 1086324736, 1088421888]);
assert_eq!(std::mem::discriminant(&child.mesh),std::mem::discriminant(&mesh(1)));assert_eq!(std::mem::discriminant(&child.semantic),std::mem::discriminant(&semantic(1)));assert_eq!(child.semantic.as_concrete(),semantic(1).as_concrete());}
{ let mut child=Some(make_rt([2, 0, 2, 0],[[1073741824, 1077936128, 1082130432], [1077936128, 1082130432, 1084227584], [1082130432, 1084227584, 1086324736], [1084227584, 1086324736, 1088421888]],1,0));child.inherit_from(&Some(make_rt([2, 2, 1, 1],[[1101004800, 1101529088, 1102053376], [1101529088, 1102053376, 1102577664], [1102053376, 1102577664, 1103101952], [1102577664, 1103101952, 1103626240]],2,2)));let child=child.unwrap();
check_value(&child.mesh_scale,2,[1073741824, 1077936128, 1082130432]);
check_value(&child.summer_albedo,2,[1101529088, 1102053376, 1102577664]);
check_value(&child.autumn_albedo,2,[1082130432, 1084227584, 1086324736]);
check_value(&child.winter_albedo,1,[1101529088, 1102053376, 1102577664]);
assert_eq!(std::mem::discriminant(&child.mesh),std::mem::discriminant(&mesh(1)));assert_eq!(std::mem::discriminant(&child.semantic),std::mem::discriminant(&semantic(0)));assert_eq!(child.semantic.as_concrete(),semantic(0).as_concrete());}
{ let mut child=Some(make_rt([2, 0, 2, 0],[[1073741824, 1077936128, 1082130432], [1077936128, 1082130432, 1084227584], [1082130432, 1084227584, 1086324736], [1084227584, 1086324736, 1088421888]],1,0));child.inherit_from(&None);let child=child.unwrap();
check_value(&child.mesh_scale,2,[1073741824, 1077936128, 1082130432]);
check_value(&child.summer_albedo,2,[1065353216, 1065353216, 1065353216]);
check_value(&child.autumn_albedo,2,[1082130432, 1084227584, 1086324736]);
check_value(&child.winter_albedo,2,[1065353216, 1065353216, 1065353216]);
assert_eq!(std::mem::discriminant(&child.mesh),std::mem::discriminant(&mesh(1)));assert_eq!(std::mem::discriminant(&child.semantic),std::mem::discriminant(&semantic(0)));assert_eq!(child.semantic.as_concrete(),semantic(0).as_concrete());}
{ let mut child=Some(make_rt([2, 2, 2, 2],[[1073741824, 1077936128, 1082130432], [1077936128, 1082130432, 1084227584], [1082130432, 1084227584, 1086324736], [1084227584, 1086324736, 1088421888]],2,1));child.inherit_from(&Some(make_rt([1, 0, 2, 1],[[1101004800, 1101529088, 1102053376], [1101529088, 1102053376, 1102577664], [1102053376, 1102577664, 1103101952], [1102577664, 1103101952, 1103626240]],1,2)));let child=child.unwrap();
check_value(&child.mesh_scale,2,[1073741824, 1077936128, 1082130432]);
check_value(&child.summer_albedo,2,[1077936128, 1082130432, 1084227584]);
check_value(&child.autumn_albedo,2,[1082130432, 1084227584, 1086324736]);
check_value(&child.winter_albedo,2,[1084227584, 1086324736, 1088421888]);
assert_eq!(std::mem::discriminant(&child.mesh),std::mem::discriminant(&mesh(2)));assert_eq!(std::mem::discriminant(&child.semantic),std::mem::discriminant(&semantic(1)));assert_eq!(child.semantic.as_concrete(),semantic(1).as_concrete());}
{ let mut child=Some(make_rt([2, 2, 2, 2],[[1073741824, 1077936128, 1082130432], [1077936128, 1082130432, 1084227584], [1082130432, 1084227584, 1086324736], [1084227584, 1086324736, 1088421888]],2,1));child.inherit_from(&None);let child=child.unwrap();
check_value(&child.mesh_scale,2,[1073741824, 1077936128, 1082130432]);
check_value(&child.summer_albedo,2,[1077936128, 1082130432, 1084227584]);
check_value(&child.autumn_albedo,2,[1082130432, 1084227584, 1086324736]);
check_value(&child.winter_albedo,2,[1084227584, 1086324736, 1088421888]);
assert_eq!(std::mem::discriminant(&child.mesh),std::mem::discriminant(&mesh(2)));assert_eq!(std::mem::discriminant(&child.semantic),std::mem::discriminant(&semantic(1)));assert_eq!(child.semantic.as_concrete(),semantic(1).as_concrete());}
{ let mut child=Some(make_rt([0, 0, 2, 2],[[1073741824, 1077936128, 1082130432], [1077936128, 1082130432, 1084227584], [1082130432, 1084227584, 1086324736], [1084227584, 1086324736, 1088421888]],2,1));child.inherit_from(&Some(make_rt([0, 0, 0, 1],[[1101004800, 1101529088, 1102053376], [1101529088, 1102053376, 1102577664], [1102053376, 1102577664, 1103101952], [1102577664, 1103101952, 1103626240]],0,0)));let child=child.unwrap();
check_value(&child.mesh_scale,2,[1065353216, 1065353216, 1065353216]);
check_value(&child.summer_albedo,2,[1065353216, 1065353216, 1065353216]);
check_value(&child.autumn_albedo,2,[1082130432, 1084227584, 1086324736]);
check_value(&child.winter_albedo,2,[1084227584, 1086324736, 1088421888]);
assert_eq!(std::mem::discriminant(&child.mesh),std::mem::discriminant(&mesh(2)));assert_eq!(std::mem::discriminant(&child.semantic),std::mem::discriminant(&semantic(1)));assert_eq!(child.semantic.as_concrete(),semantic(1).as_concrete());}
{ let mut child=Some(make_rt([0, 0, 2, 2],[[1073741824, 1077936128, 1082130432], [1077936128, 1082130432, 1084227584], [1082130432, 1084227584, 1086324736], [1084227584, 1086324736, 1088421888]],2,1));child.inherit_from(&None);let child=child.unwrap();
check_value(&child.mesh_scale,2,[1065353216, 1065353216, 1065353216]);
check_value(&child.summer_albedo,2,[1065353216, 1065353216, 1065353216]);
check_value(&child.autumn_albedo,2,[1082130432, 1084227584, 1086324736]);
check_value(&child.winter_albedo,2,[1084227584, 1086324736, 1088421888]);
assert_eq!(std::mem::discriminant(&child.mesh),std::mem::discriminant(&mesh(2)));assert_eq!(std::mem::discriminant(&child.semantic),std::mem::discriminant(&semantic(1)));assert_eq!(child.semantic.as_concrete(),semantic(1).as_concrete());}
{ let mut child=Some(make_rt([1, 2, 0, 0],[[1073741824, 1077936128, 1082130432], [1077936128, 1082130432, 1084227584], [1082130432, 1084227584, 1086324736], [1084227584, 1086324736, 1088421888]],1,0));child.inherit_from(&Some(make_rt([2, 1, 0, 2],[[1101004800, 1101529088, 1102053376], [1101529088, 1102053376, 1102577664], [1102053376, 1102577664, 1103101952], [1102577664, 1103101952, 1103626240]],0,3)));let child=child.unwrap();
check_value(&child.mesh_scale,1,[1073741824, 1077936128, 1082130432]);
check_value(&child.summer_albedo,2,[1077936128, 1082130432, 1084227584]);
check_value(&child.autumn_albedo,2,[1065353216, 1065353216, 1065353216]);
check_value(&child.winter_albedo,2,[1102577664, 1103101952, 1103626240]);
assert_eq!(std::mem::discriminant(&child.mesh),std::mem::discriminant(&mesh(1)));assert_eq!(std::mem::discriminant(&child.semantic),std::mem::discriminant(&semantic(0)));assert_eq!(child.semantic.as_concrete(),semantic(0).as_concrete());}
{ let mut child=Some(make_rt([1, 2, 0, 0],[[1073741824, 1077936128, 1082130432], [1077936128, 1082130432, 1084227584], [1082130432, 1084227584, 1086324736], [1084227584, 1086324736, 1088421888]],1,0));child.inherit_from(&None);let child=child.unwrap();
check_value(&child.mesh_scale,1,[1073741824, 1077936128, 1082130432]);
check_value(&child.summer_albedo,2,[1077936128, 1082130432, 1084227584]);
check_value(&child.autumn_albedo,2,[1065353216, 1065353216, 1065353216]);
check_value(&child.winter_albedo,2,[1065353216, 1065353216, 1065353216]);
assert_eq!(std::mem::discriminant(&child.mesh),std::mem::discriminant(&mesh(1)));assert_eq!(std::mem::discriminant(&child.semantic),std::mem::discriminant(&semantic(0)));assert_eq!(child.semantic.as_concrete(),semantic(0).as_concrete());}
{ let mut child=Some(make_rt([2, 2, 1, 2],[[1073741824, 1077936128, 1082130432], [1077936128, 1082130432, 1084227584], [1082130432, 1084227584, 1086324736], [1084227584, 1086324736, 1088421888]],0,2));child.inherit_from(&Some(make_rt([1, 1, 1, 2],[[1101004800, 1101529088, 1102053376], [1101529088, 1102053376, 1102577664], [1102053376, 1102577664, 1103101952], [1102577664, 1103101952, 1103626240]],1,2)));let child=child.unwrap();
check_value(&child.mesh_scale,2,[1073741824, 1077936128, 1082130432]);
check_value(&child.summer_albedo,2,[1077936128, 1082130432, 1084227584]);
check_value(&child.autumn_albedo,1,[1082130432, 1084227584, 1086324736]);
check_value(&child.winter_albedo,2,[1084227584, 1086324736, 1088421888]);
assert_eq!(std::mem::discriminant(&child.mesh),std::mem::discriminant(&mesh(1)));assert_eq!(std::mem::discriminant(&child.semantic),std::mem::discriminant(&semantic(0)));assert_eq!(child.semantic.as_concrete(),semantic(0).as_concrete());}
{ let mut child=Some(make_rt([2, 2, 1, 2],[[1073741824, 1077936128, 1082130432], [1077936128, 1082130432, 1084227584], [1082130432, 1084227584, 1086324736], [1084227584, 1086324736, 1088421888]],0,2));child.inherit_from(&None);let child=child.unwrap();
check_value(&child.mesh_scale,2,[1073741824, 1077936128, 1082130432]);
check_value(&child.summer_albedo,2,[1077936128, 1082130432, 1084227584]);
check_value(&child.autumn_albedo,1,[1082130432, 1084227584, 1086324736]);
check_value(&child.winter_albedo,2,[1084227584, 1086324736, 1088421888]);
assert_eq!(std::mem::discriminant(&child.mesh),std::mem::discriminant(&mesh(1)));assert_eq!(std::mem::discriminant(&child.semantic),std::mem::discriminant(&semantic(0)));assert_eq!(child.semantic.as_concrete(),semantic(0).as_concrete());}
{ let mut child=Some(make_rt([1, 1, 2, 1],[[1073741824, 1077936128, 1082130432], [1077936128, 1082130432, 1084227584], [1082130432, 1084227584, 1086324736], [1084227584, 1086324736, 1088421888]],0,0));child.inherit_from(&Some(make_rt([0, 2, 0, 1],[[1101004800, 1101529088, 1102053376], [1101529088, 1102053376, 1102577664], [1102053376, 1102577664, 1103101952], [1102577664, 1103101952, 1103626240]],2,2)));let child=child.unwrap();
check_value(&child.mesh_scale,1,[1073741824, 1077936128, 1082130432]);
check_value(&child.summer_albedo,1,[1077936128, 1082130432, 1084227584]);
check_value(&child.autumn_albedo,2,[1082130432, 1084227584, 1086324736]);
check_value(&child.winter_albedo,1,[1084227584, 1086324736, 1088421888]);
assert_eq!(std::mem::discriminant(&child.mesh),std::mem::discriminant(&mesh(2)));assert_eq!(std::mem::discriminant(&child.semantic),std::mem::discriminant(&semantic(0)));assert_eq!(child.semantic.as_concrete(),semantic(0).as_concrete());}
{ let mut child=Some(make_rt([1, 1, 2, 1],[[1073741824, 1077936128, 1082130432], [1077936128, 1082130432, 1084227584], [1082130432, 1084227584, 1086324736], [1084227584, 1086324736, 1088421888]],0,0));child.inherit_from(&None);let child=child.unwrap();
check_value(&child.mesh_scale,1,[1073741824, 1077936128, 1082130432]);
check_value(&child.summer_albedo,1,[1077936128, 1082130432, 1084227584]);
check_value(&child.autumn_albedo,2,[1082130432, 1084227584, 1086324736]);
check_value(&child.winter_albedo,1,[1084227584, 1086324736, 1088421888]);
assert_eq!(std::mem::discriminant(&child.mesh),std::mem::discriminant(&mesh(1)));assert_eq!(std::mem::discriminant(&child.semantic),std::mem::discriminant(&semantic(0)));assert_eq!(child.semantic.as_concrete(),semantic(0).as_concrete());}
{ let mut child=Some(make_rt([0, 2, 0, 1],[[1073741824, 1077936128, 1082130432], [1077936128, 1082130432, 1084227584], [1082130432, 1084227584, 1086324736], [1084227584, 1086324736, 1088421888]],1,0));child.inherit_from(&Some(make_rt([1, 0, 1, 0],[[1101004800, 1101529088, 1102053376], [1101529088, 1102053376, 1102577664], [1102053376, 1102577664, 1103101952], [1102577664, 1103101952, 1103626240]],1,2)));let child=child.unwrap();
check_value(&child.mesh_scale,1,[1102577664, 1103101952, 1103626240]);
check_value(&child.summer_albedo,2,[1077936128, 1082130432, 1084227584]);
check_value(&child.autumn_albedo,1,[1102577664, 1103101952, 1103626240]);
check_value(&child.winter_albedo,1,[1084227584, 1086324736, 1088421888]);
assert_eq!(std::mem::discriminant(&child.mesh),std::mem::discriminant(&mesh(1)));assert_eq!(std::mem::discriminant(&child.semantic),std::mem::discriminant(&semantic(0)));assert_eq!(child.semantic.as_concrete(),semantic(0).as_concrete());}
{ let mut child=Some(make_rt([0, 2, 0, 1],[[1073741824, 1077936128, 1082130432], [1077936128, 1082130432, 1084227584], [1082130432, 1084227584, 1086324736], [1084227584, 1086324736, 1088421888]],1,0));child.inherit_from(&None);let child=child.unwrap();
check_value(&child.mesh_scale,2,[1065353216, 1065353216, 1065353216]);
check_value(&child.summer_albedo,2,[1077936128, 1082130432, 1084227584]);
check_value(&child.autumn_albedo,2,[1065353216, 1065353216, 1065353216]);
check_value(&child.winter_albedo,1,[1084227584, 1086324736, 1088421888]);
assert_eq!(std::mem::discriminant(&child.mesh),std::mem::discriminant(&mesh(1)));assert_eq!(std::mem::discriminant(&child.semantic),std::mem::discriminant(&semantic(0)));assert_eq!(child.semantic.as_concrete(),semantic(0).as_concrete());}
{ let mut child=Some(make_rt([0, 2, 1, 0],[[1073741824, 1077936128, 1082130432], [1077936128, 1082130432, 1084227584], [1082130432, 1084227584, 1086324736], [1084227584, 1086324736, 1088421888]],0,1));child.inherit_from(&Some(make_rt([0, 2, 1, 1],[[1101004800, 1101529088, 1102053376], [1101529088, 1102053376, 1102577664], [1102053376, 1102577664, 1103101952], [1102577664, 1103101952, 1103626240]],0,3)));let child=child.unwrap();
check_value(&child.mesh_scale,2,[1065353216, 1065353216, 1065353216]);
check_value(&child.summer_albedo,2,[1077936128, 1082130432, 1084227584]);
check_value(&child.autumn_albedo,1,[1082130432, 1084227584, 1086324736]);
check_value(&child.winter_albedo,1,[1102577664, 1103101952, 1103626240]);
assert_eq!(std::mem::discriminant(&child.mesh),std::mem::discriminant(&mesh(1)));assert_eq!(std::mem::discriminant(&child.semantic),std::mem::discriminant(&semantic(1)));assert_eq!(child.semantic.as_concrete(),semantic(1).as_concrete());}
{ let mut child=Some(make_rt([0, 2, 1, 0],[[1073741824, 1077936128, 1082130432], [1077936128, 1082130432, 1084227584], [1082130432, 1084227584, 1086324736], [1084227584, 1086324736, 1088421888]],0,1));child.inherit_from(&None);let child=child.unwrap();
check_value(&child.mesh_scale,2,[1065353216, 1065353216, 1065353216]);
check_value(&child.summer_albedo,2,[1077936128, 1082130432, 1084227584]);
check_value(&child.autumn_albedo,1,[1082130432, 1084227584, 1086324736]);
check_value(&child.winter_albedo,2,[1065353216, 1065353216, 1065353216]);
assert_eq!(std::mem::discriminant(&child.mesh),std::mem::discriminant(&mesh(1)));assert_eq!(std::mem::discriminant(&child.semantic),std::mem::discriminant(&semantic(1)));assert_eq!(child.semantic.as_concrete(),semantic(1).as_concrete());}
{ let mut child=Some(make_rt([0, 0, 2, 2],[[1073741824, 1077936128, 1082130432], [1077936128, 1082130432, 1084227584], [1082130432, 1084227584, 1086324736], [1084227584, 1086324736, 1088421888]],2,2));child.inherit_from(&Some(make_rt([0, 1, 1, 0],[[1101004800, 1101529088, 1102053376], [1101529088, 1102053376, 1102577664], [1102053376, 1102577664, 1103101952], [1102577664, 1103101952, 1103626240]],2,0)));let child=child.unwrap();
check_value(&child.mesh_scale,2,[1065353216, 1065353216, 1065353216]);
check_value(&child.summer_albedo,1,[1102577664, 1103101952, 1103626240]);
check_value(&child.autumn_albedo,2,[1082130432, 1084227584, 1086324736]);
check_value(&child.winter_albedo,2,[1084227584, 1086324736, 1088421888]);
assert_eq!(std::mem::discriminant(&child.mesh),std::mem::discriminant(&mesh(2)));assert_eq!(std::mem::discriminant(&child.semantic),std::mem::discriminant(&semantic(0)));assert_eq!(child.semantic.as_concrete(),semantic(0).as_concrete());}
{ let mut child=Some(make_rt([0, 0, 2, 2],[[1073741824, 1077936128, 1082130432], [1077936128, 1082130432, 1084227584], [1082130432, 1084227584, 1086324736], [1084227584, 1086324736, 1088421888]],2,2));child.inherit_from(&None);let child=child.unwrap();
check_value(&child.mesh_scale,2,[1065353216, 1065353216, 1065353216]);
check_value(&child.summer_albedo,2,[1065353216, 1065353216, 1065353216]);
check_value(&child.autumn_albedo,2,[1082130432, 1084227584, 1086324736]);
check_value(&child.winter_albedo,2,[1084227584, 1086324736, 1088421888]);
assert_eq!(std::mem::discriminant(&child.mesh),std::mem::discriminant(&mesh(2)));assert_eq!(std::mem::discriminant(&child.semantic),std::mem::discriminant(&semantic(0)));assert_eq!(child.semantic.as_concrete(),semantic(0).as_concrete());}
{ let mut child=Some(make_rt([0, 1, 2, 2],[[1073741824, 1077936128, 1082130432], [1077936128, 1082130432, 1084227584], [1082130432, 1084227584, 1086324736], [1084227584, 1086324736, 1088421888]],1,1));child.inherit_from(&Some(make_rt([2, 2, 1, 0],[[1101004800, 1101529088, 1102053376], [1101529088, 1102053376, 1102577664], [1102053376, 1102577664, 1103101952], [1102577664, 1103101952, 1103626240]],1,0)));let child=child.unwrap();
check_value(&child.mesh_scale,2,[1101004800, 1101529088, 1102053376]);
check_value(&child.summer_albedo,1,[1077936128, 1082130432, 1084227584]);
check_value(&child.autumn_albedo,2,[1082130432, 1084227584, 1086324736]);
check_value(&child.winter_albedo,2,[1084227584, 1086324736, 1088421888]);
assert_eq!(std::mem::discriminant(&child.mesh),std::mem::discriminant(&mesh(1)));assert_eq!(std::mem::discriminant(&child.semantic),std::mem::discriminant(&semantic(1)));assert_eq!(child.semantic.as_concrete(),semantic(1).as_concrete());}
{ let mut child=Some(make_rt([0, 1, 2, 2],[[1073741824, 1077936128, 1082130432], [1077936128, 1082130432, 1084227584], [1082130432, 1084227584, 1086324736], [1084227584, 1086324736, 1088421888]],1,1));child.inherit_from(&None);let child=child.unwrap();
check_value(&child.mesh_scale,2,[1065353216, 1065353216, 1065353216]);
check_value(&child.summer_albedo,1,[1077936128, 1082130432, 1084227584]);
check_value(&child.autumn_albedo,2,[1082130432, 1084227584, 1086324736]);
check_value(&child.winter_albedo,2,[1084227584, 1086324736, 1088421888]);
assert_eq!(std::mem::discriminant(&child.mesh),std::mem::discriminant(&mesh(1)));assert_eq!(std::mem::discriminant(&child.semantic),std::mem::discriminant(&semantic(1)));assert_eq!(child.semantic.as_concrete(),semantic(1).as_concrete());}
{ let mut child=Some(make_rt([0, 1, 2, 0],[[1073741824, 1077936128, 1082130432], [1077936128, 1082130432, 1084227584], [1082130432, 1084227584, 1086324736], [1084227584, 1086324736, 1088421888]],2,1));child.inherit_from(&Some(make_rt([1, 2, 1, 0],[[1101004800, 1101529088, 1102053376], [1101529088, 1102053376, 1102577664], [1102053376, 1102577664, 1103101952], [1102577664, 1103101952, 1103626240]],0,1)));let child=child.unwrap();
check_value(&child.mesh_scale,1,[1101004800, 1101529088, 1102053376]);
check_value(&child.summer_albedo,1,[1077936128, 1082130432, 1084227584]);
check_value(&child.autumn_albedo,2,[1082130432, 1084227584, 1086324736]);
check_value(&child.winter_albedo,2,[1065353216, 1065353216, 1065353216]);
assert_eq!(std::mem::discriminant(&child.mesh),std::mem::discriminant(&mesh(2)));assert_eq!(std::mem::discriminant(&child.semantic),std::mem::discriminant(&semantic(1)));assert_eq!(child.semantic.as_concrete(),semantic(1).as_concrete());}
{ let mut child=Some(make_rt([0, 1, 2, 0],[[1073741824, 1077936128, 1082130432], [1077936128, 1082130432, 1084227584], [1082130432, 1084227584, 1086324736], [1084227584, 1086324736, 1088421888]],2,1));child.inherit_from(&None);let child=child.unwrap();
check_value(&child.mesh_scale,2,[1065353216, 1065353216, 1065353216]);
check_value(&child.summer_albedo,1,[1077936128, 1082130432, 1084227584]);
check_value(&child.autumn_albedo,2,[1082130432, 1084227584, 1086324736]);
check_value(&child.winter_albedo,2,[1065353216, 1065353216, 1065353216]);
assert_eq!(std::mem::discriminant(&child.mesh),std::mem::discriminant(&mesh(2)));assert_eq!(std::mem::discriminant(&child.semantic),std::mem::discriminant(&semantic(1)));assert_eq!(child.semantic.as_concrete(),semantic(1).as_concrete());}
{ let mut child=Some(make_rt([1, 0, 2, 1],[[1073741824, 1077936128, 1082130432], [1077936128, 1082130432, 1084227584], [1082130432, 1084227584, 1086324736], [1084227584, 1086324736, 1088421888]],2,1));child.inherit_from(&Some(make_rt([0, 2, 1, 0],[[1101004800, 1101529088, 1102053376], [1101529088, 1102053376, 1102577664], [1102053376, 1102577664, 1103101952], [1102577664, 1103101952, 1103626240]],1,0)));let child=child.unwrap();
check_value(&child.mesh_scale,1,[1073741824, 1077936128, 1082130432]);
check_value(&child.summer_albedo,2,[1101529088, 1102053376, 1102577664]);
check_value(&child.autumn_albedo,2,[1082130432, 1084227584, 1086324736]);
check_value(&child.winter_albedo,1,[1084227584, 1086324736, 1088421888]);
assert_eq!(std::mem::discriminant(&child.mesh),std::mem::discriminant(&mesh(2)));assert_eq!(std::mem::discriminant(&child.semantic),std::mem::discriminant(&semantic(1)));assert_eq!(child.semantic.as_concrete(),semantic(1).as_concrete());}
{ let mut child=Some(make_rt([1, 0, 2, 1],[[1073741824, 1077936128, 1082130432], [1077936128, 1082130432, 1084227584], [1082130432, 1084227584, 1086324736], [1084227584, 1086324736, 1088421888]],2,1));child.inherit_from(&None);let child=child.unwrap();
check_value(&child.mesh_scale,1,[1073741824, 1077936128, 1082130432]);
check_value(&child.summer_albedo,2,[1065353216, 1065353216, 1065353216]);
check_value(&child.autumn_albedo,2,[1082130432, 1084227584, 1086324736]);
check_value(&child.winter_albedo,1,[1084227584, 1086324736, 1088421888]);
assert_eq!(std::mem::discriminant(&child.mesh),std::mem::discriminant(&mesh(2)));assert_eq!(std::mem::discriminant(&child.semantic),std::mem::discriminant(&semantic(1)));assert_eq!(child.semantic.as_concrete(),semantic(1).as_concrete());}
{ let mut child=Some(make_rt([0, 0, 2, 2],[[1073741824, 1077936128, 1082130432], [1077936128, 1082130432, 1084227584], [1082130432, 1084227584, 1086324736], [1084227584, 1086324736, 1088421888]],0,1));child.inherit_from(&Some(make_rt([2, 2, 2, 2],[[1101004800, 1101529088, 1102053376], [1101529088, 1102053376, 1102577664], [1102053376, 1102577664, 1103101952], [1102577664, 1103101952, 1103626240]],1,1)));let child=child.unwrap();
check_value(&child.mesh_scale,2,[1101004800, 1101529088, 1102053376]);
check_value(&child.summer_albedo,2,[1101529088, 1102053376, 1102577664]);
check_value(&child.autumn_albedo,2,[1082130432, 1084227584, 1086324736]);
check_value(&child.winter_albedo,2,[1084227584, 1086324736, 1088421888]);
assert_eq!(std::mem::discriminant(&child.mesh),std::mem::discriminant(&mesh(1)));assert_eq!(std::mem::discriminant(&child.semantic),std::mem::discriminant(&semantic(1)));assert_eq!(child.semantic.as_concrete(),semantic(1).as_concrete());}
{ let mut child=Some(make_rt([0, 0, 2, 2],[[1073741824, 1077936128, 1082130432], [1077936128, 1082130432, 1084227584], [1082130432, 1084227584, 1086324736], [1084227584, 1086324736, 1088421888]],0,1));child.inherit_from(&None);let child=child.unwrap();
check_value(&child.mesh_scale,2,[1065353216, 1065353216, 1065353216]);
check_value(&child.summer_albedo,2,[1065353216, 1065353216, 1065353216]);
check_value(&child.autumn_albedo,2,[1082130432, 1084227584, 1086324736]);
check_value(&child.winter_albedo,2,[1084227584, 1086324736, 1088421888]);
assert_eq!(std::mem::discriminant(&child.mesh),std::mem::discriminant(&mesh(1)));assert_eq!(std::mem::discriminant(&child.semantic),std::mem::discriminant(&semantic(1)));assert_eq!(child.semantic.as_concrete(),semantic(1).as_concrete());}
{ let mut child=Some(make_rt([1, 2, 2, 1],[[1073741824, 1077936128, 1082130432], [1077936128, 1082130432, 1084227584], [1082130432, 1084227584, 1086324736], [1084227584, 1086324736, 1088421888]],1,3));child.inherit_from(&Some(make_rt([0, 0, 2, 1],[[1101004800, 1101529088, 1102053376], [1101529088, 1102053376, 1102577664], [1102053376, 1102577664, 1103101952], [1102577664, 1103101952, 1103626240]],2,1)));let child=child.unwrap();
check_value(&child.mesh_scale,1,[1073741824, 1077936128, 1082130432]);
check_value(&child.summer_albedo,2,[1077936128, 1082130432, 1084227584]);
check_value(&child.autumn_albedo,2,[1082130432, 1084227584, 1086324736]);
check_value(&child.winter_albedo,1,[1084227584, 1086324736, 1088421888]);
assert_eq!(std::mem::discriminant(&child.mesh),std::mem::discriminant(&mesh(1)));assert_eq!(std::mem::discriminant(&child.semantic),std::mem::discriminant(&semantic(3)));}
{ let mut child=Some(make_rt([1, 2, 2, 1],[[1073741824, 1077936128, 1082130432], [1077936128, 1082130432, 1084227584], [1082130432, 1084227584, 1086324736], [1084227584, 1086324736, 1088421888]],1,3));child.inherit_from(&None);let child=child.unwrap();
check_value(&child.mesh_scale,1,[1073741824, 1077936128, 1082130432]);
check_value(&child.summer_albedo,2,[1077936128, 1082130432, 1084227584]);
check_value(&child.autumn_albedo,2,[1082130432, 1084227584, 1086324736]);
check_value(&child.winter_albedo,1,[1084227584, 1086324736, 1088421888]);
assert_eq!(std::mem::discriminant(&child.mesh),std::mem::discriminant(&mesh(1)));assert_eq!(std::mem::discriminant(&child.semantic),std::mem::discriminant(&semantic(3)));}
{ let mut child=Some(make_rt([0, 0, 1, 1],[[1073741824, 1077936128, 1082130432], [1077936128, 1082130432, 1084227584], [1082130432, 1084227584, 1086324736], [1084227584, 1086324736, 1088421888]],2,0));child.inherit_from(&Some(make_rt([2, 0, 1, 1],[[1101004800, 1101529088, 1102053376], [1101529088, 1102053376, 1102577664], [1102053376, 1102577664, 1103101952], [1102577664, 1103101952, 1103626240]],0,1)));let child=child.unwrap();
check_value(&child.mesh_scale,2,[1101004800, 1101529088, 1102053376]);
check_value(&child.summer_albedo,2,[1065353216, 1065353216, 1065353216]);
check_value(&child.autumn_albedo,1,[1082130432, 1084227584, 1086324736]);
check_value(&child.winter_albedo,1,[1084227584, 1086324736, 1088421888]);
assert_eq!(std::mem::discriminant(&child.mesh),std::mem::discriminant(&mesh(2)));assert_eq!(std::mem::discriminant(&child.semantic),std::mem::discriminant(&semantic(0)));assert_eq!(child.semantic.as_concrete(),semantic(0).as_concrete());}
{ let mut child=Some(make_rt([0, 0, 1, 1],[[1073741824, 1077936128, 1082130432], [1077936128, 1082130432, 1084227584], [1082130432, 1084227584, 1086324736], [1084227584, 1086324736, 1088421888]],2,0));child.inherit_from(&None);let child=child.unwrap();
check_value(&child.mesh_scale,2,[1065353216, 1065353216, 1065353216]);
check_value(&child.summer_albedo,2,[1065353216, 1065353216, 1065353216]);
check_value(&child.autumn_albedo,1,[1082130432, 1084227584, 1086324736]);
check_value(&child.winter_albedo,1,[1084227584, 1086324736, 1088421888]);
assert_eq!(std::mem::discriminant(&child.mesh),std::mem::discriminant(&mesh(2)));assert_eq!(std::mem::discriminant(&child.semantic),std::mem::discriminant(&semantic(0)));assert_eq!(child.semantic.as_concrete(),semantic(0).as_concrete());}
{ let mut child=Some(make_rt([2, 0, 0, 0],[[1073741824, 1077936128, 1082130432], [1077936128, 1082130432, 1084227584], [1082130432, 1084227584, 1086324736], [1084227584, 1086324736, 1088421888]],0,0));child.inherit_from(&Some(make_rt([1, 0, 1, 0],[[1101004800, 1101529088, 1102053376], [1101529088, 1102053376, 1102577664], [1102053376, 1102577664, 1103101952], [1102577664, 1103101952, 1103626240]],1,3)));let child=child.unwrap();
check_value(&child.mesh_scale,2,[1073741824, 1077936128, 1082130432]);
check_value(&child.summer_albedo,2,[1065353216, 1065353216, 1065353216]);
check_value(&child.autumn_albedo,1,[1101004800, 1101529088, 1102053376]);
check_value(&child.winter_albedo,2,[1065353216, 1065353216, 1065353216]);
assert_eq!(std::mem::discriminant(&child.mesh),std::mem::discriminant(&mesh(1)));assert_eq!(std::mem::discriminant(&child.semantic),std::mem::discriminant(&semantic(0)));assert_eq!(child.semantic.as_concrete(),semantic(0).as_concrete());}
{ let mut child=Some(make_rt([2, 0, 0, 0],[[1073741824, 1077936128, 1082130432], [1077936128, 1082130432, 1084227584], [1082130432, 1084227584, 1086324736], [1084227584, 1086324736, 1088421888]],0,0));child.inherit_from(&None);let child=child.unwrap();
check_value(&child.mesh_scale,2,[1073741824, 1077936128, 1082130432]);
check_value(&child.summer_albedo,2,[1065353216, 1065353216, 1065353216]);
check_value(&child.autumn_albedo,2,[1065353216, 1065353216, 1065353216]);
check_value(&child.winter_albedo,2,[1065353216, 1065353216, 1065353216]);
assert_eq!(std::mem::discriminant(&child.mesh),std::mem::discriminant(&mesh(1)));assert_eq!(std::mem::discriminant(&child.semantic),std::mem::discriminant(&semantic(0)));assert_eq!(child.semantic.as_concrete(),semantic(0).as_concrete());}
{ let mut child=Some(make_rt([1, 2, 1, 0],[[1073741824, 1077936128, 1082130432], [1077936128, 1082130432, 1084227584], [1082130432, 1084227584, 1086324736], [1084227584, 1086324736, 1088421888]],2,0));child.inherit_from(&Some(make_rt([1, 1, 1, 1],[[1101004800, 1101529088, 1102053376], [1101529088, 1102053376, 1102577664], [1102053376, 1102577664, 1103101952], [1102577664, 1103101952, 1103626240]],1,2)));let child=child.unwrap();
check_value(&child.mesh_scale,1,[1073741824, 1077936128, 1082130432]);
check_value(&child.summer_albedo,2,[1077936128, 1082130432, 1084227584]);
check_value(&child.autumn_albedo,1,[1082130432, 1084227584, 1086324736]);
check_value(&child.winter_albedo,1,[1101004800, 1101529088, 1102053376]);
assert_eq!(std::mem::discriminant(&child.mesh),std::mem::discriminant(&mesh(2)));assert_eq!(std::mem::discriminant(&child.semantic),std::mem::discriminant(&semantic(0)));assert_eq!(child.semantic.as_concrete(),semantic(0).as_concrete());}
{ let mut child=Some(make_rt([1, 2, 1, 0],[[1073741824, 1077936128, 1082130432], [1077936128, 1082130432, 1084227584], [1082130432, 1084227584, 1086324736], [1084227584, 1086324736, 1088421888]],2,0));child.inherit_from(&None);let child=child.unwrap();
check_value(&child.mesh_scale,1,[1073741824, 1077936128, 1082130432]);
check_value(&child.summer_albedo,2,[1077936128, 1082130432, 1084227584]);
check_value(&child.autumn_albedo,1,[1082130432, 1084227584, 1086324736]);
check_value(&child.winter_albedo,2,[1065353216, 1065353216, 1065353216]);
assert_eq!(std::mem::discriminant(&child.mesh),std::mem::discriminant(&mesh(2)));assert_eq!(std::mem::discriminant(&child.semantic),std::mem::discriminant(&semantic(0)));assert_eq!(child.semantic.as_concrete(),semantic(0).as_concrete());}
{ let mut child=Some(make_rt([1, 2, 2, 2],[[1073741824, 1077936128, 1082130432], [1077936128, 1082130432, 1084227584], [1082130432, 1084227584, 1086324736], [1084227584, 1086324736, 1088421888]],1,1));child.inherit_from(&Some(make_rt([1, 1, 0, 0],[[1101004800, 1101529088, 1102053376], [1101529088, 1102053376, 1102577664], [1102053376, 1102577664, 1103101952], [1102577664, 1103101952, 1103626240]],1,2)));let child=child.unwrap();
check_value(&child.mesh_scale,1,[1073741824, 1077936128, 1082130432]);
check_value(&child.summer_albedo,2,[1077936128, 1082130432, 1084227584]);
check_value(&child.autumn_albedo,2,[1082130432, 1084227584, 1086324736]);
check_value(&child.winter_albedo,2,[1084227584, 1086324736, 1088421888]);
assert_eq!(std::mem::discriminant(&child.mesh),std::mem::discriminant(&mesh(1)));assert_eq!(std::mem::discriminant(&child.semantic),std::mem::discriminant(&semantic(1)));assert_eq!(child.semantic.as_concrete(),semantic(1).as_concrete());}
{ let mut child=Some(make_rt([1, 2, 2, 2],[[1073741824, 1077936128, 1082130432], [1077936128, 1082130432, 1084227584], [1082130432, 1084227584, 1086324736], [1084227584, 1086324736, 1088421888]],1,1));child.inherit_from(&None);let child=child.unwrap();
check_value(&child.mesh_scale,1,[1073741824, 1077936128, 1082130432]);
check_value(&child.summer_albedo,2,[1077936128, 1082130432, 1084227584]);
check_value(&child.autumn_albedo,2,[1082130432, 1084227584, 1086324736]);
check_value(&child.winter_albedo,2,[1084227584, 1086324736, 1088421888]);
assert_eq!(std::mem::discriminant(&child.mesh),std::mem::discriminant(&mesh(1)));assert_eq!(std::mem::discriminant(&child.semantic),std::mem::discriminant(&semantic(1)));assert_eq!(child.semantic.as_concrete(),semantic(1).as_concrete());}
{ let mut child=Some(make_rt([2, 0, 2, 1],[[1073741824, 1077936128, 1082130432], [1077936128, 1082130432, 1084227584], [1082130432, 1084227584, 1086324736], [1084227584, 1086324736, 1088421888]],0,3));child.inherit_from(&Some(make_rt([2, 2, 0, 0],[[1101004800, 1101529088, 1102053376], [1101529088, 1102053376, 1102577664], [1102053376, 1102577664, 1103101952], [1102577664, 1103101952, 1103626240]],1,1)));let child=child.unwrap();
check_value(&child.mesh_scale,2,[1073741824, 1077936128, 1082130432]);
check_value(&child.summer_albedo,2,[1101529088, 1102053376, 1102577664]);
check_value(&child.autumn_albedo,2,[1082130432, 1084227584, 1086324736]);
check_value(&child.winter_albedo,1,[1084227584, 1086324736, 1088421888]);
assert_eq!(std::mem::discriminant(&child.mesh),std::mem::discriminant(&mesh(1)));assert_eq!(std::mem::discriminant(&child.semantic),std::mem::discriminant(&semantic(3)));}
{ let mut child=Some(make_rt([2, 0, 2, 1],[[1073741824, 1077936128, 1082130432], [1077936128, 1082130432, 1084227584], [1082130432, 1084227584, 1086324736], [1084227584, 1086324736, 1088421888]],0,3));child.inherit_from(&None);let child=child.unwrap();
check_value(&child.mesh_scale,2,[1073741824, 1077936128, 1082130432]);
check_value(&child.summer_albedo,2,[1065353216, 1065353216, 1065353216]);
check_value(&child.autumn_albedo,2,[1082130432, 1084227584, 1086324736]);
check_value(&child.winter_albedo,1,[1084227584, 1086324736, 1088421888]);
assert_eq!(std::mem::discriminant(&child.mesh),std::mem::discriminant(&mesh(1)));assert_eq!(std::mem::discriminant(&child.semantic),std::mem::discriminant(&semantic(3)));}
{ let mut child=Some(make_rt([2, 1, 0, 2],[[1073741824, 1077936128, 1082130432], [1077936128, 1082130432, 1084227584], [1082130432, 1084227584, 1086324736], [1084227584, 1086324736, 1088421888]],1,2));child.inherit_from(&Some(make_rt([2, 1, 2, 0],[[1101004800, 1101529088, 1102053376], [1101529088, 1102053376, 1102577664], [1102053376, 1102577664, 1103101952], [1102577664, 1103101952, 1103626240]],2,2)));let child=child.unwrap();
check_value(&child.mesh_scale,2,[1073741824, 1077936128, 1082130432]);
check_value(&child.summer_albedo,1,[1077936128, 1082130432, 1084227584]);
check_value(&child.autumn_albedo,2,[1102053376, 1102577664, 1103101952]);
check_value(&child.winter_albedo,2,[1084227584, 1086324736, 1088421888]);
assert_eq!(std::mem::discriminant(&child.mesh),std::mem::discriminant(&mesh(1)));assert_eq!(std::mem::discriminant(&child.semantic),std::mem::discriminant(&semantic(0)));assert_eq!(child.semantic.as_concrete(),semantic(0).as_concrete());}
{ let mut child=Some(make_rt([2, 1, 0, 2],[[1073741824, 1077936128, 1082130432], [1077936128, 1082130432, 1084227584], [1082130432, 1084227584, 1086324736], [1084227584, 1086324736, 1088421888]],1,2));child.inherit_from(&None);let child=child.unwrap();
check_value(&child.mesh_scale,2,[1073741824, 1077936128, 1082130432]);
check_value(&child.summer_albedo,1,[1077936128, 1082130432, 1084227584]);
check_value(&child.autumn_albedo,2,[1065353216, 1065353216, 1065353216]);
check_value(&child.winter_albedo,2,[1084227584, 1086324736, 1088421888]);
assert_eq!(std::mem::discriminant(&child.mesh),std::mem::discriminant(&mesh(1)));assert_eq!(std::mem::discriminant(&child.semantic),std::mem::discriminant(&semantic(0)));assert_eq!(child.semantic.as_concrete(),semantic(0).as_concrete());}
{ let mut child=Some(make_rt([0, 2, 2, 2],[[1073741824, 1077936128, 1082130432], [1077936128, 1082130432, 1084227584], [1082130432, 1084227584, 1086324736], [1084227584, 1086324736, 1088421888]],2,3));child.inherit_from(&Some(make_rt([1, 1, 1, 2],[[1101004800, 1101529088, 1102053376], [1101529088, 1102053376, 1102577664], [1102053376, 1102577664, 1103101952], [1102577664, 1103101952, 1103626240]],2,1)));let child=child.unwrap();
check_value(&child.mesh_scale,1,[1102053376, 1102577664, 1103101952]);
check_value(&child.summer_albedo,2,[1077936128, 1082130432, 1084227584]);
check_value(&child.autumn_albedo,2,[1082130432, 1084227584, 1086324736]);
check_value(&child.winter_albedo,2,[1084227584, 1086324736, 1088421888]);
assert_eq!(std::mem::discriminant(&child.mesh),std::mem::discriminant(&mesh(2)));assert_eq!(std::mem::discriminant(&child.semantic),std::mem::discriminant(&semantic(3)));}
{ let mut child=Some(make_rt([0, 2, 2, 2],[[1073741824, 1077936128, 1082130432], [1077936128, 1082130432, 1084227584], [1082130432, 1084227584, 1086324736], [1084227584, 1086324736, 1088421888]],2,3));child.inherit_from(&None);let child=child.unwrap();
check_value(&child.mesh_scale,2,[1065353216, 1065353216, 1065353216]);
check_value(&child.summer_albedo,2,[1077936128, 1082130432, 1084227584]);
check_value(&child.autumn_albedo,2,[1082130432, 1084227584, 1086324736]);
check_value(&child.winter_albedo,2,[1084227584, 1086324736, 1088421888]);
assert_eq!(std::mem::discriminant(&child.mesh),std::mem::discriminant(&mesh(2)));assert_eq!(std::mem::discriminant(&child.semantic),std::mem::discriminant(&semantic(3)));}
{ let mut child=Some(make_rt([1, 1, 2, 1],[[1073741824, 1077936128, 1082130432], [1077936128, 1082130432, 1084227584], [1082130432, 1084227584, 1086324736], [1084227584, 1086324736, 1088421888]],0,0));child.inherit_from(&Some(make_rt([2, 1, 1, 2],[[1101004800, 1101529088, 1102053376], [1101529088, 1102053376, 1102577664], [1102053376, 1102577664, 1103101952], [1102577664, 1103101952, 1103626240]],1,0)));let child=child.unwrap();
check_value(&child.mesh_scale,1,[1073741824, 1077936128, 1082130432]);
check_value(&child.summer_albedo,1,[1077936128, 1082130432, 1084227584]);
check_value(&child.autumn_albedo,2,[1082130432, 1084227584, 1086324736]);
check_value(&child.winter_albedo,1,[1084227584, 1086324736, 1088421888]);
assert_eq!(std::mem::discriminant(&child.mesh),std::mem::discriminant(&mesh(1)));assert_eq!(std::mem::discriminant(&child.semantic),std::mem::discriminant(&semantic(0)));assert_eq!(child.semantic.as_concrete(),semantic(0).as_concrete());}
{ let mut child=Some(make_rt([1, 1, 2, 1],[[1073741824, 1077936128, 1082130432], [1077936128, 1082130432, 1084227584], [1082130432, 1084227584, 1086324736], [1084227584, 1086324736, 1088421888]],0,0));child.inherit_from(&None);let child=child.unwrap();
check_value(&child.mesh_scale,1,[1073741824, 1077936128, 1082130432]);
check_value(&child.summer_albedo,1,[1077936128, 1082130432, 1084227584]);
check_value(&child.autumn_albedo,2,[1082130432, 1084227584, 1086324736]);
check_value(&child.winter_albedo,1,[1084227584, 1086324736, 1088421888]);
assert_eq!(std::mem::discriminant(&child.mesh),std::mem::discriminant(&mesh(1)));assert_eq!(std::mem::discriminant(&child.semantic),std::mem::discriminant(&semantic(0)));assert_eq!(child.semantic.as_concrete(),semantic(0).as_concrete());}
{ let mut child=Some(make_rt([0, 2, 2, 1],[[1073741824, 1077936128, 1082130432], [1077936128, 1082130432, 1084227584], [1082130432, 1084227584, 1086324736], [1084227584, 1086324736, 1088421888]],2,1));child.inherit_from(&Some(make_rt([1, 1, 2, 1],[[1101004800, 1101529088, 1102053376], [1101529088, 1102053376, 1102577664], [1102053376, 1102577664, 1103101952], [1102577664, 1103101952, 1103626240]],0,2)));let child=child.unwrap();
check_value(&child.mesh_scale,1,[1102053376, 1102577664, 1103101952]);
check_value(&child.summer_albedo,2,[1077936128, 1082130432, 1084227584]);
check_value(&child.autumn_albedo,2,[1082130432, 1084227584, 1086324736]);
check_value(&child.winter_albedo,1,[1084227584, 1086324736, 1088421888]);
assert_eq!(std::mem::discriminant(&child.mesh),std::mem::discriminant(&mesh(2)));assert_eq!(std::mem::discriminant(&child.semantic),std::mem::discriminant(&semantic(1)));assert_eq!(child.semantic.as_concrete(),semantic(1).as_concrete());}
{ let mut child=Some(make_rt([0, 2, 2, 1],[[1073741824, 1077936128, 1082130432], [1077936128, 1082130432, 1084227584], [1082130432, 1084227584, 1086324736], [1084227584, 1086324736, 1088421888]],2,1));child.inherit_from(&None);let child=child.unwrap();
check_value(&child.mesh_scale,2,[1065353216, 1065353216, 1065353216]);
check_value(&child.summer_albedo,2,[1077936128, 1082130432, 1084227584]);
check_value(&child.autumn_albedo,2,[1082130432, 1084227584, 1086324736]);
check_value(&child.winter_albedo,1,[1084227584, 1086324736, 1088421888]);
assert_eq!(std::mem::discriminant(&child.mesh),std::mem::discriminant(&mesh(2)));assert_eq!(std::mem::discriminant(&child.semantic),std::mem::discriminant(&semantic(1)));assert_eq!(child.semantic.as_concrete(),semantic(1).as_concrete());}
{ let mut child=Some(make_rt([2, 2, 1, 2],[[1073741824, 1077936128, 1082130432], [1077936128, 1082130432, 1084227584], [1082130432, 1084227584, 1086324736], [1084227584, 1086324736, 1088421888]],2,3));child.inherit_from(&Some(make_rt([1, 2, 1, 1],[[1101004800, 1101529088, 1102053376], [1101529088, 1102053376, 1102577664], [1102053376, 1102577664, 1103101952], [1102577664, 1103101952, 1103626240]],2,1)));let child=child.unwrap();
check_value(&child.mesh_scale,2,[1073741824, 1077936128, 1082130432]);
check_value(&child.summer_albedo,2,[1077936128, 1082130432, 1084227584]);
check_value(&child.autumn_albedo,1,[1082130432, 1084227584, 1086324736]);
check_value(&child.winter_albedo,2,[1084227584, 1086324736, 1088421888]);
assert_eq!(std::mem::discriminant(&child.mesh),std::mem::discriminant(&mesh(2)));assert_eq!(std::mem::discriminant(&child.semantic),std::mem::discriminant(&semantic(3)));}
{ let mut child=Some(make_rt([2, 2, 1, 2],[[1073741824, 1077936128, 1082130432], [1077936128, 1082130432, 1084227584], [1082130432, 1084227584, 1086324736], [1084227584, 1086324736, 1088421888]],2,3));child.inherit_from(&None);let child=child.unwrap();
check_value(&child.mesh_scale,2,[1073741824, 1077936128, 1082130432]);
check_value(&child.summer_albedo,2,[1077936128, 1082130432, 1084227584]);
check_value(&child.autumn_albedo,1,[1082130432, 1084227584, 1086324736]);
check_value(&child.winter_albedo,2,[1084227584, 1086324736, 1088421888]);
assert_eq!(std::mem::discriminant(&child.mesh),std::mem::discriminant(&mesh(2)));assert_eq!(std::mem::discriminant(&child.semantic),std::mem::discriminant(&semantic(3)));}
{ let mut child=Some(make_rt([1, 2, 2, 1],[[1073741824, 1077936128, 1082130432], [1077936128, 1082130432, 1084227584], [1082130432, 1084227584, 1086324736], [1084227584, 1086324736, 1088421888]],2,1));child.inherit_from(&Some(make_rt([2, 1, 1, 0],[[1101004800, 1101529088, 1102053376], [1101529088, 1102053376, 1102577664], [1102053376, 1102577664, 1103101952], [1102577664, 1103101952, 1103626240]],2,3)));let child=child.unwrap();
check_value(&child.mesh_scale,1,[1073741824, 1077936128, 1082130432]);
check_value(&child.summer_albedo,2,[1077936128, 1082130432, 1084227584]);
check_value(&child.autumn_albedo,2,[1082130432, 1084227584, 1086324736]);
check_value(&child.winter_albedo,1,[1084227584, 1086324736, 1088421888]);
assert_eq!(std::mem::discriminant(&child.mesh),std::mem::discriminant(&mesh(2)));assert_eq!(std::mem::discriminant(&child.semantic),std::mem::discriminant(&semantic(1)));assert_eq!(child.semantic.as_concrete(),semantic(1).as_concrete());}
{ let mut child=Some(make_rt([1, 2, 2, 1],[[1073741824, 1077936128, 1082130432], [1077936128, 1082130432, 1084227584], [1082130432, 1084227584, 1086324736], [1084227584, 1086324736, 1088421888]],2,1));child.inherit_from(&None);let child=child.unwrap();
check_value(&child.mesh_scale,1,[1073741824, 1077936128, 1082130432]);
check_value(&child.summer_albedo,2,[1077936128, 1082130432, 1084227584]);
check_value(&child.autumn_albedo,2,[1082130432, 1084227584, 1086324736]);
check_value(&child.winter_albedo,1,[1084227584, 1086324736, 1088421888]);
assert_eq!(std::mem::discriminant(&child.mesh),std::mem::discriminant(&mesh(2)));assert_eq!(std::mem::discriminant(&child.semantic),std::mem::discriminant(&semantic(1)));assert_eq!(child.semantic.as_concrete(),semantic(1).as_concrete());}
{ let mut child=Some(make_rt([0, 2, 1, 2],[[1073741824, 1077936128, 1082130432], [1077936128, 1082130432, 1084227584], [1082130432, 1084227584, 1086324736], [1084227584, 1086324736, 1088421888]],2,1));child.inherit_from(&Some(make_rt([1, 2, 0, 1],[[1101004800, 1101529088, 1102053376], [1101529088, 1102053376, 1102577664], [1102053376, 1102577664, 1103101952], [1102577664, 1103101952, 1103626240]],0,2)));let child=child.unwrap();
check_value(&child.mesh_scale,1,[1102053376, 1102577664, 1103101952]);
check_value(&child.summer_albedo,2,[1077936128, 1082130432, 1084227584]);
check_value(&child.autumn_albedo,1,[1082130432, 1084227584, 1086324736]);
check_value(&child.winter_albedo,2,[1084227584, 1086324736, 1088421888]);
assert_eq!(std::mem::discriminant(&child.mesh),std::mem::discriminant(&mesh(2)));assert_eq!(std::mem::discriminant(&child.semantic),std::mem::discriminant(&semantic(1)));assert_eq!(child.semantic.as_concrete(),semantic(1).as_concrete());}
{ let mut child=Some(make_rt([0, 2, 1, 2],[[1073741824, 1077936128, 1082130432], [1077936128, 1082130432, 1084227584], [1082130432, 1084227584, 1086324736], [1084227584, 1086324736, 1088421888]],2,1));child.inherit_from(&None);let child=child.unwrap();
check_value(&child.mesh_scale,2,[1065353216, 1065353216, 1065353216]);
check_value(&child.summer_albedo,2,[1077936128, 1082130432, 1084227584]);
check_value(&child.autumn_albedo,1,[1082130432, 1084227584, 1086324736]);
check_value(&child.winter_albedo,2,[1084227584, 1086324736, 1088421888]);
assert_eq!(std::mem::discriminant(&child.mesh),std::mem::discriminant(&mesh(2)));assert_eq!(std::mem::discriminant(&child.semantic),std::mem::discriminant(&semantic(1)));assert_eq!(child.semantic.as_concrete(),semantic(1).as_concrete());}
}
#[test]
fn original_b_audio_option_defaults_and_required_markers() {
{let mut audio=Some(PrefabAudioConfig {trigger:Inheritable::Value(AudioTriggerConfig {kind:Inheritable::Inherit,shape_scale:value(0,[1073741824,1077936128,1082130432]),shape:Inheritable::Inherit})});audio.inherit_from(&None);let audio=audio.unwrap();let trigger=audio.trigger.as_concrete();assert!(matches!(trigger.kind,Inheritable::MissingDefault));check_value(&trigger.shape_scale,2,[1065353216, 1065353216, 1065353216]);assert!(matches!(trigger.shape,Inheritable::MissingDefault));}
{let mut audio=Some(PrefabAudioConfig {trigger:Inheritable::Value(AudioTriggerConfig {kind:Inheritable::Inherit,shape_scale:value(0,[1073741824,1077936128,1082130432]),shape:Inheritable::MissingDefault})});audio.inherit_from(&None);let audio=audio.unwrap();let trigger=audio.trigger.as_concrete();assert!(matches!(trigger.kind,Inheritable::MissingDefault));check_value(&trigger.shape_scale,2,[1065353216, 1065353216, 1065353216]);assert!(matches!(trigger.shape,Inheritable::MissingDefault));}
{let mut audio=Some(PrefabAudioConfig {trigger:Inheritable::Value(AudioTriggerConfig {kind:Inheritable::Inherit,shape_scale:value(0,[1073741824,1077936128,1082130432]),shape:Inheritable::Value(AudioTriggerShape::Aabb)})});audio.inherit_from(&None);let audio=audio.unwrap();let trigger=audio.trigger.as_concrete();assert!(matches!(trigger.kind,Inheritable::MissingDefault));check_value(&trigger.shape_scale,2,[1065353216, 1065353216, 1065353216]);assert!(matches!(trigger.shape,Inheritable::Value(AudioTriggerShape::Aabb)));}
{let mut audio=Some(PrefabAudioConfig {trigger:Inheritable::Value(AudioTriggerConfig {kind:Inheritable::Inherit,shape_scale:value(1,[1073741824,1077936128,1082130432]),shape:Inheritable::Inherit})});audio.inherit_from(&None);let audio=audio.unwrap();let trigger=audio.trigger.as_concrete();assert!(matches!(trigger.kind,Inheritable::MissingDefault));check_value(&trigger.shape_scale,1,[1073741824, 1077936128, 1082130432]);assert!(matches!(trigger.shape,Inheritable::MissingDefault));}
{let mut audio=Some(PrefabAudioConfig {trigger:Inheritable::Value(AudioTriggerConfig {kind:Inheritable::Inherit,shape_scale:value(1,[1073741824,1077936128,1082130432]),shape:Inheritable::MissingDefault})});audio.inherit_from(&None);let audio=audio.unwrap();let trigger=audio.trigger.as_concrete();assert!(matches!(trigger.kind,Inheritable::MissingDefault));check_value(&trigger.shape_scale,1,[1073741824, 1077936128, 1082130432]);assert!(matches!(trigger.shape,Inheritable::MissingDefault));}
{let mut audio=Some(PrefabAudioConfig {trigger:Inheritable::Value(AudioTriggerConfig {kind:Inheritable::Inherit,shape_scale:value(1,[1073741824,1077936128,1082130432]),shape:Inheritable::Value(AudioTriggerShape::Aabb)})});audio.inherit_from(&None);let audio=audio.unwrap();let trigger=audio.trigger.as_concrete();assert!(matches!(trigger.kind,Inheritable::MissingDefault));check_value(&trigger.shape_scale,1,[1073741824, 1077936128, 1082130432]);assert!(matches!(trigger.shape,Inheritable::Value(AudioTriggerShape::Aabb)));}
{let mut audio=Some(PrefabAudioConfig {trigger:Inheritable::Value(AudioTriggerConfig {kind:Inheritable::Inherit,shape_scale:value(2,[1073741824,1077936128,1082130432]),shape:Inheritable::Inherit})});audio.inherit_from(&None);let audio=audio.unwrap();let trigger=audio.trigger.as_concrete();assert!(matches!(trigger.kind,Inheritable::MissingDefault));check_value(&trigger.shape_scale,2,[1073741824, 1077936128, 1082130432]);assert!(matches!(trigger.shape,Inheritable::MissingDefault));}
{let mut audio=Some(PrefabAudioConfig {trigger:Inheritable::Value(AudioTriggerConfig {kind:Inheritable::Inherit,shape_scale:value(2,[1073741824,1077936128,1082130432]),shape:Inheritable::MissingDefault})});audio.inherit_from(&None);let audio=audio.unwrap();let trigger=audio.trigger.as_concrete();assert!(matches!(trigger.kind,Inheritable::MissingDefault));check_value(&trigger.shape_scale,2,[1073741824, 1077936128, 1082130432]);assert!(matches!(trigger.shape,Inheritable::MissingDefault));}
{let mut audio=Some(PrefabAudioConfig {trigger:Inheritable::Value(AudioTriggerConfig {kind:Inheritable::Inherit,shape_scale:value(2,[1073741824,1077936128,1082130432]),shape:Inheritable::Value(AudioTriggerShape::Aabb)})});audio.inherit_from(&None);let audio=audio.unwrap();let trigger=audio.trigger.as_concrete();assert!(matches!(trigger.kind,Inheritable::MissingDefault));check_value(&trigger.shape_scale,2,[1073741824, 1077936128, 1082130432]);assert!(matches!(trigger.shape,Inheritable::Value(AudioTriggerShape::Aabb)));}
{let mut audio=Some(PrefabAudioConfig {trigger:Inheritable::Value(AudioTriggerConfig {kind:Inheritable::MissingDefault,shape_scale:value(0,[1073741824,1077936128,1082130432]),shape:Inheritable::Inherit})});audio.inherit_from(&None);let audio=audio.unwrap();let trigger=audio.trigger.as_concrete();assert!(matches!(trigger.kind,Inheritable::MissingDefault));check_value(&trigger.shape_scale,2,[1065353216, 1065353216, 1065353216]);assert!(matches!(trigger.shape,Inheritable::MissingDefault));}
{let mut audio=Some(PrefabAudioConfig {trigger:Inheritable::Value(AudioTriggerConfig {kind:Inheritable::MissingDefault,shape_scale:value(0,[1073741824,1077936128,1082130432]),shape:Inheritable::MissingDefault})});audio.inherit_from(&None);let audio=audio.unwrap();let trigger=audio.trigger.as_concrete();assert!(matches!(trigger.kind,Inheritable::MissingDefault));check_value(&trigger.shape_scale,2,[1065353216, 1065353216, 1065353216]);assert!(matches!(trigger.shape,Inheritable::MissingDefault));}
{let mut audio=Some(PrefabAudioConfig {trigger:Inheritable::Value(AudioTriggerConfig {kind:Inheritable::MissingDefault,shape_scale:value(0,[1073741824,1077936128,1082130432]),shape:Inheritable::Value(AudioTriggerShape::Aabb)})});audio.inherit_from(&None);let audio=audio.unwrap();let trigger=audio.trigger.as_concrete();assert!(matches!(trigger.kind,Inheritable::MissingDefault));check_value(&trigger.shape_scale,2,[1065353216, 1065353216, 1065353216]);assert!(matches!(trigger.shape,Inheritable::Value(AudioTriggerShape::Aabb)));}
{let mut audio=Some(PrefabAudioConfig {trigger:Inheritable::Value(AudioTriggerConfig {kind:Inheritable::MissingDefault,shape_scale:value(1,[1073741824,1077936128,1082130432]),shape:Inheritable::Inherit})});audio.inherit_from(&None);let audio=audio.unwrap();let trigger=audio.trigger.as_concrete();assert!(matches!(trigger.kind,Inheritable::MissingDefault));check_value(&trigger.shape_scale,1,[1073741824, 1077936128, 1082130432]);assert!(matches!(trigger.shape,Inheritable::MissingDefault));}
{let mut audio=Some(PrefabAudioConfig {trigger:Inheritable::Value(AudioTriggerConfig {kind:Inheritable::MissingDefault,shape_scale:value(1,[1073741824,1077936128,1082130432]),shape:Inheritable::MissingDefault})});audio.inherit_from(&None);let audio=audio.unwrap();let trigger=audio.trigger.as_concrete();assert!(matches!(trigger.kind,Inheritable::MissingDefault));check_value(&trigger.shape_scale,1,[1073741824, 1077936128, 1082130432]);assert!(matches!(trigger.shape,Inheritable::MissingDefault));}
{let mut audio=Some(PrefabAudioConfig {trigger:Inheritable::Value(AudioTriggerConfig {kind:Inheritable::MissingDefault,shape_scale:value(1,[1073741824,1077936128,1082130432]),shape:Inheritable::Value(AudioTriggerShape::Aabb)})});audio.inherit_from(&None);let audio=audio.unwrap();let trigger=audio.trigger.as_concrete();assert!(matches!(trigger.kind,Inheritable::MissingDefault));check_value(&trigger.shape_scale,1,[1073741824, 1077936128, 1082130432]);assert!(matches!(trigger.shape,Inheritable::Value(AudioTriggerShape::Aabb)));}
{let mut audio=Some(PrefabAudioConfig {trigger:Inheritable::Value(AudioTriggerConfig {kind:Inheritable::MissingDefault,shape_scale:value(2,[1073741824,1077936128,1082130432]),shape:Inheritable::Inherit})});audio.inherit_from(&None);let audio=audio.unwrap();let trigger=audio.trigger.as_concrete();assert!(matches!(trigger.kind,Inheritable::MissingDefault));check_value(&trigger.shape_scale,2,[1073741824, 1077936128, 1082130432]);assert!(matches!(trigger.shape,Inheritable::MissingDefault));}
{let mut audio=Some(PrefabAudioConfig {trigger:Inheritable::Value(AudioTriggerConfig {kind:Inheritable::MissingDefault,shape_scale:value(2,[1073741824,1077936128,1082130432]),shape:Inheritable::MissingDefault})});audio.inherit_from(&None);let audio=audio.unwrap();let trigger=audio.trigger.as_concrete();assert!(matches!(trigger.kind,Inheritable::MissingDefault));check_value(&trigger.shape_scale,2,[1073741824, 1077936128, 1082130432]);assert!(matches!(trigger.shape,Inheritable::MissingDefault));}
{let mut audio=Some(PrefabAudioConfig {trigger:Inheritable::Value(AudioTriggerConfig {kind:Inheritable::MissingDefault,shape_scale:value(2,[1073741824,1077936128,1082130432]),shape:Inheritable::Value(AudioTriggerShape::Aabb)})});audio.inherit_from(&None);let audio=audio.unwrap();let trigger=audio.trigger.as_concrete();assert!(matches!(trigger.kind,Inheritable::MissingDefault));check_value(&trigger.shape_scale,2,[1073741824, 1077936128, 1082130432]);assert!(matches!(trigger.shape,Inheritable::Value(AudioTriggerShape::Aabb)));}
{let mut audio=Some(PrefabAudioConfig {trigger:Inheritable::Inherit});audio.inherit_from(&None);let audio=audio.unwrap();let trigger=audio.trigger.as_concrete();assert!(matches!(trigger.kind,Inheritable::MissingDefault));check_value(&trigger.shape_scale,2,[1065353216, 1065353216, 1065353216]);assert!(matches!(trigger.shape,Inheritable::MissingDefault));}
{let mut audio=Some(PrefabAudioConfig {trigger:Inheritable::Inherit});audio.inherit_from(&None);let audio=audio.unwrap();let trigger=audio.trigger.as_concrete();assert!(matches!(trigger.kind,Inheritable::MissingDefault));check_value(&trigger.shape_scale,2,[1065353216, 1065353216, 1065353216]);assert!(matches!(trigger.shape,Inheritable::MissingDefault));}
{let mut audio=Some(PrefabAudioConfig {trigger:Inheritable::Inherit});audio.inherit_from(&None);let audio=audio.unwrap();let trigger=audio.trigger.as_concrete();assert!(matches!(trigger.kind,Inheritable::MissingDefault));check_value(&trigger.shape_scale,2,[1065353216, 1065353216, 1065353216]);assert!(matches!(trigger.shape,Inheritable::MissingDefault));}
{let mut audio=Some(PrefabAudioConfig {trigger:Inheritable::Inherit});audio.inherit_from(&None);let audio=audio.unwrap();let trigger=audio.trigger.as_concrete();assert!(matches!(trigger.kind,Inheritable::MissingDefault));check_value(&trigger.shape_scale,2,[1065353216, 1065353216, 1065353216]);assert!(matches!(trigger.shape,Inheritable::MissingDefault));}
{let mut audio=Some(PrefabAudioConfig {trigger:Inheritable::Inherit});audio.inherit_from(&None);let audio=audio.unwrap();let trigger=audio.trigger.as_concrete();assert!(matches!(trigger.kind,Inheritable::MissingDefault));check_value(&trigger.shape_scale,2,[1065353216, 1065353216, 1065353216]);assert!(matches!(trigger.shape,Inheritable::MissingDefault));}
{let mut audio=Some(PrefabAudioConfig {trigger:Inheritable::Inherit});audio.inherit_from(&None);let audio=audio.unwrap();let trigger=audio.trigger.as_concrete();assert!(matches!(trigger.kind,Inheritable::MissingDefault));check_value(&trigger.shape_scale,2,[1065353216, 1065353216, 1065353216]);assert!(matches!(trigger.shape,Inheritable::MissingDefault));}
{let mut audio=Some(PrefabAudioConfig {trigger:Inheritable::Inherit});audio.inherit_from(&None);let audio=audio.unwrap();let trigger=audio.trigger.as_concrete();assert!(matches!(trigger.kind,Inheritable::MissingDefault));check_value(&trigger.shape_scale,2,[1065353216, 1065353216, 1065353216]);assert!(matches!(trigger.shape,Inheritable::MissingDefault));}
{let mut audio=Some(PrefabAudioConfig {trigger:Inheritable::Inherit});audio.inherit_from(&None);let audio=audio.unwrap();let trigger=audio.trigger.as_concrete();assert!(matches!(trigger.kind,Inheritable::MissingDefault));check_value(&trigger.shape_scale,2,[1065353216, 1065353216, 1065353216]);assert!(matches!(trigger.shape,Inheritable::MissingDefault));}
{let mut audio=Some(PrefabAudioConfig {trigger:Inheritable::Inherit});audio.inherit_from(&None);let audio=audio.unwrap();let trigger=audio.trigger.as_concrete();assert!(matches!(trigger.kind,Inheritable::MissingDefault));check_value(&trigger.shape_scale,2,[1065353216, 1065353216, 1065353216]);assert!(matches!(trigger.shape,Inheritable::MissingDefault));}
{let mut audio=Some(PrefabAudioConfig {trigger:Inheritable::MissingDefault});assert!(std::panic::catch_unwind(std::panic::AssertUnwindSafe(||audio.inherit_from(&None))).is_err());}
{let mut audio=Some(PrefabAudioConfig {trigger:Inheritable::MissingDefault});assert!(std::panic::catch_unwind(std::panic::AssertUnwindSafe(||audio.inherit_from(&None))).is_err());}
{let mut audio=Some(PrefabAudioConfig {trigger:Inheritable::MissingDefault});assert!(std::panic::catch_unwind(std::panic::AssertUnwindSafe(||audio.inherit_from(&None))).is_err());}
{let mut audio=Some(PrefabAudioConfig {trigger:Inheritable::MissingDefault});assert!(std::panic::catch_unwind(std::panic::AssertUnwindSafe(||audio.inherit_from(&None))).is_err());}
{let mut audio=Some(PrefabAudioConfig {trigger:Inheritable::MissingDefault});assert!(std::panic::catch_unwind(std::panic::AssertUnwindSafe(||audio.inherit_from(&None))).is_err());}
{let mut audio=Some(PrefabAudioConfig {trigger:Inheritable::MissingDefault});assert!(std::panic::catch_unwind(std::panic::AssertUnwindSafe(||audio.inherit_from(&None))).is_err());}
{let mut audio=Some(PrefabAudioConfig {trigger:Inheritable::MissingDefault});assert!(std::panic::catch_unwind(std::panic::AssertUnwindSafe(||audio.inherit_from(&None))).is_err());}
{let mut audio=Some(PrefabAudioConfig {trigger:Inheritable::MissingDefault});assert!(std::panic::catch_unwind(std::panic::AssertUnwindSafe(||audio.inherit_from(&None))).is_err());}
{let mut audio=Some(PrefabAudioConfig {trigger:Inheritable::MissingDefault});assert!(std::panic::catch_unwind(std::panic::AssertUnwindSafe(||audio.inherit_from(&None))).is_err());}
}
