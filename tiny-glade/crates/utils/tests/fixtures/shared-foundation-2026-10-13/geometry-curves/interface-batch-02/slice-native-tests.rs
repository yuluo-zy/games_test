use super::*;use glam::Vec3;
fn f(b:u32)->f32{f32::from_bits(b)}
fn c(v:[u32;2])->CurveInternalCoord{CurveInternalCoord{segment:v[0]as usize,t:f(v[1])}}
fn eq_coord(a:CurveInternalCoord,b:[u32;2]){assert_eq!(a.segment,b[0]as usize);if f(b[1]).is_nan(){assert!(a.t.is_nan())}else{assert_eq!(a.t.to_bits(),b[1])}}
fn points(p:&[[u32;3]])->Vec<Vec3>{p.iter().map(|v|Vec3::new(f(v[0]),f(v[1]),f(v[2]))).collect()}
#[test]fn original_b_slice_extend_left(){
{let p=points(&[[0,0,0],[1077936128,1073741824,1082130432],[1086324736,1082130432,1082130432],[1086324736,1086324736,1091567616]]);
let mut slice=CurveSlice::<Vec3,_,super::super::CurveCoordSemantic2D>::from_coord_range(p.as_slice(),c([1,1058501480]),c([1,1062958333])).remap_2d();
slice.extend_left(f(0));eq_coord(slice.start(),[1,1058501480]);eq_coord(slice.end(),[1,1062958333]);
}
{let p=points(&[[0,0,0],[1077936128,1073741824,1082130432],[1086324736,1082130432,1082130432],[1086324736,1086324736,1091567616]]);
let mut slice=CurveSlice::<Vec3,_,super::super::CurveCoordSemantic2D>::from_coord_range(p.as_slice(),c([0,1058224654]),c([1,1052481871])).remap_2d();
slice.extend_left(f(1008981770));eq_coord(slice.start(),[0,1058191100]);eq_coord(slice.end(),[1,1052481871]);
}
{let p=points(&[[0,0,0],[1077936128,1073741824,1082130432],[1086324736,1082130432,1082130432],[1086324736,1086324736,1091567616]]);
let mut slice=CurveSlice::<Vec3,_,super::super::CurveCoordSemantic2D>::from_coord_range(p.as_slice(),c([0,1062532183]),c([1,1053312486])).remap_2d();
slice.extend_left(f(1077936128));eq_coord(slice.start(),[0,1047358195]);eq_coord(slice.end(),[1,1053312486]);
}
{let p=points(&[[0,0,0],[1077936128,1073741824,1082130432],[1086324736,1082130432,1082130432],[1086324736,1086324736,1091567616]]);
let mut slice=CurveSlice::<Vec3,_,super::super::CurveCoordSemantic2D>::from_coord_range(p.as_slice(),c([0,1060798721]),c([0,1060798721])).remap_2d();
slice.extend_left(f(1120403456));eq_coord(slice.start(),[0,0]);eq_coord(slice.end(),[0,1060798721]);
}
{let p=points(&[[0,0,0],[1077936128,1073741824,1082130432],[1086324736,1082130432,1082130432],[1086324736,1086324736,1091567616]]);
let mut slice=CurveSlice::<Vec3,_,super::super::CurveCoordSemantic2D>::from_coord_range(p.as_slice(),c([2,1054429360]),c([2,1058585751])).remap_2d();
slice.extend_left(f(2139095040));eq_coord(slice.start(),[0,0]);eq_coord(slice.end(),[2,1058585751]);
}
{let p=points(&[[0,0,0],[1077936128,1073741824,1082130432],[1086324736,1082130432,1082130432],[1086324736,1086324736,1091567616]]);
let mut slice=CurveSlice::<Vec3,_,super::super::CurveCoordSemantic2D>::from_coord_range(p.as_slice(),c([2,1062108793]),c([2,1062108793])).remap_2d();
assert!(std::panic::catch_unwind(std::panic::AssertUnwindSafe(||{slice.extend_left(f(3212836864))})).is_err());
}
{let p=points(&[[0,0,0],[1077936128,1073741824,1082130432],[1086324736,1082130432,1082130432],[1086324736,1086324736,1091567616]]);
let mut slice=CurveSlice::<Vec3,_,super::super::CurveCoordSemantic2D>::from_coord_range(p.as_slice(),c([2,1061072993]),c([2,1061072993])).remap_2d();
assert!(std::panic::catch_unwind(std::panic::AssertUnwindSafe(||{slice.extend_left(f(2143289344))})).is_err());
}
{let p=points(&[[0,0,0],[0,1065353216,0],[1065353216,1082130432,0],[1073741824,1088421888,0]]);
let mut slice=CurveSlice::<Vec3,_,super::super::CurveCoordSemantic2D>::from_coord_range(p.as_slice(),c([0,1060289205]),c([1,1060027134])).remap_2d();
slice.extend_left(f(0));eq_coord(slice.start(),[0,4290772992]);eq_coord(slice.end(),[1,1060027134]);
}
{let p=points(&[[0,0,0],[0,1065353216,0],[1065353216,1082130432,0],[1073741824,1088421888,0]]);
let mut slice=CurveSlice::<Vec3,_,super::super::CurveCoordSemantic2D>::from_coord_range(p.as_slice(),c([2,1047650643]),c([2,1061006381])).remap_2d();
slice.extend_left(f(1008981770));eq_coord(slice.start(),[2,1046979555]);eq_coord(slice.end(),[2,1061006381]);
}
{let p=points(&[[0,0,0],[0,1065353216,0],[1065353216,1082130432,0],[1073741824,1088421888,0]]);
let mut slice=CurveSlice::<Vec3,_,super::super::CurveCoordSemantic2D>::from_coord_range(p.as_slice(),c([0,1057737336]),c([1,1025236148])).remap_2d();
slice.extend_left(f(1077936128));eq_coord(slice.start(),[0,0]);eq_coord(slice.end(),[1,1025236148]);
}
{let p=points(&[[0,0,0],[0,1065353216,0],[1065353216,1082130432,0],[1073741824,1088421888,0]]);
let mut slice=CurveSlice::<Vec3,_,super::super::CurveCoordSemantic2D>::from_coord_range(p.as_slice(),c([2,1059535198]),c([2,1059535198])).remap_2d();
slice.extend_left(f(1120403456));eq_coord(slice.start(),[0,0]);eq_coord(slice.end(),[2,1059535198]);
}
{let p=points(&[[0,0,0],[0,1065353216,0],[1065353216,1082130432,0],[1073741824,1088421888,0]]);
let mut slice=CurveSlice::<Vec3,_,super::super::CurveCoordSemantic2D>::from_coord_range(p.as_slice(),c([1,1061354077]),c([2,1039054636])).remap_2d();
slice.extend_left(f(2139095040));eq_coord(slice.start(),[0,0]);eq_coord(slice.end(),[2,1039054636]);
}
{let p=points(&[[0,0,0],[0,1065353216,0],[1065353216,1082130432,0],[1073741824,1088421888,0]]);
let mut slice=CurveSlice::<Vec3,_,super::super::CurveCoordSemantic2D>::from_coord_range(p.as_slice(),c([0,1060094772]),c([0,1060705804])).remap_2d();
assert!(std::panic::catch_unwind(std::panic::AssertUnwindSafe(||{slice.extend_left(f(3212836864))})).is_err());
}
{let p=points(&[[0,0,0],[0,1065353216,0],[1065353216,1082130432,0],[1073741824,1088421888,0]]);
let mut slice=CurveSlice::<Vec3,_,super::super::CurveCoordSemantic2D>::from_coord_range(p.as_slice(),c([0,1047513997]),c([0,1056075668])).remap_2d();
assert!(std::panic::catch_unwind(std::panic::AssertUnwindSafe(||{slice.extend_left(f(2143289344))})).is_err());
}
{let p=points(&[[3242262499,3208823723,1095550516],[3247643230,1082972693,1065410640],[3225834784,3229971066,1086543384]]);
let mut slice=CurveSlice::<Vec3,_,super::super::CurveCoordSemantic2D>::from_coord_range(p.as_slice(),c([0,1058187937]),c([1,1052414139])).remap_2d();
slice.extend_left(f(0));eq_coord(slice.start(),[0,1058187937]);eq_coord(slice.end(),[1,1052414139]);
}
{let p=points(&[[3242262499,3208823723,1095550516],[3247643230,1082972693,1065410640],[3225834784,3229971066,1086543384]]);
let mut slice=CurveSlice::<Vec3,_,super::super::CurveCoordSemantic2D>::from_coord_range(p.as_slice(),c([1,1049452160]),c([1,1049452160])).remap_2d();
slice.extend_left(f(1008981770));eq_coord(slice.start(),[1,1049431339]);eq_coord(slice.end(),[1,1049452160]);
}
{let p=points(&[[3242262499,3208823723,1095550516],[3247643230,1082972693,1065410640],[3225834784,3229971066,1086543384]]);
let mut slice=CurveSlice::<Vec3,_,super::super::CurveCoordSemantic2D>::from_coord_range(p.as_slice(),c([0,1057748357]),c([0,1057748357])).remap_2d();
slice.extend_left(f(1077936128));eq_coord(slice.start(),[0,1051009254]);eq_coord(slice.end(),[0,1057748357]);
}
{let p=points(&[[3242262499,3208823723,1095550516],[3247643230,1082972693,1065410640],[3225834784,3229971066,1086543384]]);
let mut slice=CurveSlice::<Vec3,_,super::super::CurveCoordSemantic2D>::from_coord_range(p.as_slice(),c([0,1052903052]),c([1,1062190666])).remap_2d();
slice.extend_left(f(1120403456));eq_coord(slice.start(),[0,0]);eq_coord(slice.end(),[1,1062190666]);
}
{let p=points(&[[3242262499,3208823723,1095550516],[3247643230,1082972693,1065410640],[3225834784,3229971066,1086543384]]);
let mut slice=CurveSlice::<Vec3,_,super::super::CurveCoordSemantic2D>::from_coord_range(p.as_slice(),c([1,1052714098]),c([1,1059106902])).remap_2d();
slice.extend_left(f(2139095040));eq_coord(slice.start(),[0,0]);eq_coord(slice.end(),[1,1059106902]);
}
{let p=points(&[[3242262499,3208823723,1095550516],[3247643230,1082972693,1065410640],[3225834784,3229971066,1086543384]]);
let mut slice=CurveSlice::<Vec3,_,super::super::CurveCoordSemantic2D>::from_coord_range(p.as_slice(),c([1,1052006168]),c([1,1059911018])).remap_2d();
assert!(std::panic::catch_unwind(std::panic::AssertUnwindSafe(||{slice.extend_left(f(3212836864))})).is_err());
}
{let p=points(&[[3242262499,3208823723,1095550516],[3247643230,1082972693,1065410640],[3225834784,3229971066,1086543384]]);
let mut slice=CurveSlice::<Vec3,_,super::super::CurveCoordSemantic2D>::from_coord_range(p.as_slice(),c([0,1061771262]),c([0,1061771262])).remap_2d();
assert!(std::panic::catch_unwind(std::panic::AssertUnwindSafe(||{slice.extend_left(f(2143289344))})).is_err());
}
{let p=points(&[[1084747998,1086586339,1100903328],[1095998162,1094281809,3183456677],[1096916140,3247270388,3240764055],[3243037119,1100811594,3239211518],[3233700383,1099539686,3239259458],[3230381993,1096713839,3247535037],[1095330858,1099423118,1094774569],[1069596664,3225385354,1098326837]]);
let mut slice=CurveSlice::<Vec3,_,super::super::CurveCoordSemantic2D>::from_coord_range(p.as_slice(),c([2,1062399810]),c([2,1062399810])).remap_2d();
slice.extend_left(f(0));eq_coord(slice.start(),[2,1062399810]);eq_coord(slice.end(),[2,1062399810]);
}
{let p=points(&[[1084747998,1086586339,1100903328],[1095998162,1094281809,3183456677],[1096916140,3247270388,3240764055],[3243037119,1100811594,3239211518],[3233700383,1099539686,3239259458],[3230381993,1096713839,3247535037],[1095330858,1099423118,1094774569],[1069596664,3225385354,1098326837]]);
let mut slice=CurveSlice::<Vec3,_,super::super::CurveCoordSemantic2D>::from_coord_range(p.as_slice(),c([6,1043795402]),c([6,1050037719])).remap_2d();
slice.extend_left(f(1008981770));eq_coord(slice.start(),[6,1043737493]);eq_coord(slice.end(),[6,1050037719]);
}
{let p=points(&[[1084747998,1086586339,1100903328],[1095998162,1094281809,3183456677],[1096916140,3247270388,3240764055],[3243037119,1100811594,3239211518],[3233700383,1099539686,3239259458],[3230381993,1096713839,3247535037],[1095330858,1099423118,1094774569],[1069596664,3225385354,1098326837]]);
let mut slice=CurveSlice::<Vec3,_,super::super::CurveCoordSemantic2D>::from_coord_range(p.as_slice(),c([1,1043461871]),c([4,1058643371])).remap_2d();
slice.extend_left(f(1077936128));eq_coord(slice.start(),[0,1064443953]);eq_coord(slice.end(),[4,1058643371]);
}
{let p=points(&[[1084747998,1086586339,1100903328],[1095998162,1094281809,3183456677],[1096916140,3247270388,3240764055],[3243037119,1100811594,3239211518],[3233700383,1099539686,3239259458],[3230381993,1096713839,3247535037],[1095330858,1099423118,1094774569],[1069596664,3225385354,1098326837]]);
let mut slice=CurveSlice::<Vec3,_,super::super::CurveCoordSemantic2D>::from_coord_range(p.as_slice(),c([1,1051242243]),c([2,1050443097])).remap_2d();
slice.extend_left(f(1120403456));eq_coord(slice.start(),[0,0]);eq_coord(slice.end(),[2,1050443097]);
}
{let p=points(&[[1084747998,1086586339,1100903328],[1095998162,1094281809,3183456677],[1096916140,3247270388,3240764055],[3243037119,1100811594,3239211518],[3233700383,1099539686,3239259458],[3230381993,1096713839,3247535037],[1095330858,1099423118,1094774569],[1069596664,3225385354,1098326837]]);
let mut slice=CurveSlice::<Vec3,_,super::super::CurveCoordSemantic2D>::from_coord_range(p.as_slice(),c([0,1047305548]),c([0,1060307701])).remap_2d();
slice.extend_left(f(2139095040));eq_coord(slice.start(),[0,0]);eq_coord(slice.end(),[0,1060307701]);
}
{let p=points(&[[1084747998,1086586339,1100903328],[1095998162,1094281809,3183456677],[1096916140,3247270388,3240764055],[3243037119,1100811594,3239211518],[3233700383,1099539686,3239259458],[3230381993,1096713839,3247535037],[1095330858,1099423118,1094774569],[1069596664,3225385354,1098326837]]);
let mut slice=CurveSlice::<Vec3,_,super::super::CurveCoordSemantic2D>::from_coord_range(p.as_slice(),c([6,1050271392]),c([6,1051467437])).remap_2d();
assert!(std::panic::catch_unwind(std::panic::AssertUnwindSafe(||{slice.extend_left(f(3212836864))})).is_err());
}
{let p=points(&[[1084747998,1086586339,1100903328],[1095998162,1094281809,3183456677],[1096916140,3247270388,3240764055],[3243037119,1100811594,3239211518],[3233700383,1099539686,3239259458],[3230381993,1096713839,3247535037],[1095330858,1099423118,1094774569],[1069596664,3225385354,1098326837]]);
let mut slice=CurveSlice::<Vec3,_,super::super::CurveCoordSemantic2D>::from_coord_range(p.as_slice(),c([3,1007195167]),c([3,1052731629])).remap_2d();
assert!(std::panic::catch_unwind(std::panic::AssertUnwindSafe(||{slice.extend_left(f(2143289344))})).is_err());
}
{let p=points(&[[1082977368,3221932557,3247282740],[1091417814,1097176541,1068208583],[1100111636,1075139201,1093142526],[3213382931,3214971046,1099242834],[3242840851,3239170174,1095797098],[1095376411,3240195434,3244981559],[1099755233,1095767615,3246438629]]);
let mut slice=CurveSlice::<Vec3,_,super::super::CurveCoordSemantic2D>::from_coord_range(p.as_slice(),c([1,1058852471]),c([4,1064917952])).remap_2d();
slice.extend_left(f(0));eq_coord(slice.start(),[1,1058852470]);eq_coord(slice.end(),[4,1064917952]);
}
{let p=points(&[[1082977368,3221932557,3247282740],[1091417814,1097176541,1068208583],[1100111636,1075139201,1093142526],[3213382931,3214971046,1099242834],[3242840851,3239170174,1095797098],[1095376411,3240195434,3244981559],[1099755233,1095767615,3246438629]]);
let mut slice=CurveSlice::<Vec3,_,super::super::CurveCoordSemantic2D>::from_coord_range(p.as_slice(),c([4,1021581377]),c([4,1061981315])).remap_2d();
slice.extend_left(f(1008981770));eq_coord(slice.start(),[4,1021438100]);eq_coord(slice.end(),[4,1061981315]);
}
{let p=points(&[[1082977368,3221932557,3247282740],[1091417814,1097176541,1068208583],[1100111636,1075139201,1093142526],[3213382931,3214971046,1099242834],[3242840851,3239170174,1095797098],[1095376411,3240195434,3244981559],[1099755233,1095767615,3246438629]]);
let mut slice=CurveSlice::<Vec3,_,super::super::CurveCoordSemantic2D>::from_coord_range(p.as_slice(),c([3,1036028570]),c([5,1032170177])).remap_2d();
slice.extend_left(f(1077936128));eq_coord(slice.start(),[2,1063814722]);eq_coord(slice.end(),[5,1032170177]);
}
{let p=points(&[[1082977368,3221932557,3247282740],[1091417814,1097176541,1068208583],[1100111636,1075139201,1093142526],[3213382931,3214971046,1099242834],[3242840851,3239170174,1095797098],[1095376411,3240195434,3244981559],[1099755233,1095767615,3246438629]]);
let mut slice=CurveSlice::<Vec3,_,super::super::CurveCoordSemantic2D>::from_coord_range(p.as_slice(),c([3,1063636124]),c([5,1042123052])).remap_2d();
slice.extend_left(f(1120403456));eq_coord(slice.start(),[0,0]);eq_coord(slice.end(),[5,1042123052]);
}
{let p=points(&[[1082977368,3221932557,3247282740],[1091417814,1097176541,1068208583],[1100111636,1075139201,1093142526],[3213382931,3214971046,1099242834],[3242840851,3239170174,1095797098],[1095376411,3240195434,3244981559],[1099755233,1095767615,3246438629]]);
let mut slice=CurveSlice::<Vec3,_,super::super::CurveCoordSemantic2D>::from_coord_range(p.as_slice(),c([3,1062285856]),c([4,1064108825])).remap_2d();
slice.extend_left(f(2139095040));eq_coord(slice.start(),[0,0]);eq_coord(slice.end(),[4,1064108825]);
}
{let p=points(&[[1082977368,3221932557,3247282740],[1091417814,1097176541,1068208583],[1100111636,1075139201,1093142526],[3213382931,3214971046,1099242834],[3242840851,3239170174,1095797098],[1095376411,3240195434,3244981559],[1099755233,1095767615,3246438629]]);
let mut slice=CurveSlice::<Vec3,_,super::super::CurveCoordSemantic2D>::from_coord_range(p.as_slice(),c([1,1055140182]),c([5,1060418297])).remap_2d();
assert!(std::panic::catch_unwind(std::panic::AssertUnwindSafe(||{slice.extend_left(f(3212836864))})).is_err());
}
{let p=points(&[[1082977368,3221932557,3247282740],[1091417814,1097176541,1068208583],[1100111636,1075139201,1093142526],[3213382931,3214971046,1099242834],[3242840851,3239170174,1095797098],[1095376411,3240195434,3244981559],[1099755233,1095767615,3246438629]]);
let mut slice=CurveSlice::<Vec3,_,super::super::CurveCoordSemantic2D>::from_coord_range(p.as_slice(),c([4,1046562721]),c([5,1056332180])).remap_2d();
assert!(std::panic::catch_unwind(std::panic::AssertUnwindSafe(||{slice.extend_left(f(2143289344))})).is_err());
}
{let p=points(&[[3247798851,1058059480,3233609334],[3240528963,1083175319,3219678495]]);
let mut slice=CurveSlice::<Vec3,_,super::super::CurveCoordSemantic2D>::from_coord_range(p.as_slice(),c([0,1044231370]),c([0,1044231370])).remap_2d();
slice.extend_left(f(0));eq_coord(slice.start(),[0,1044231368]);eq_coord(slice.end(),[0,1044231370]);
}
{let p=points(&[[3247798851,1058059480,3233609334],[3240528963,1083175319,3219678495]]);
let mut slice=CurveSlice::<Vec3,_,super::super::CurveCoordSemantic2D>::from_coord_range(p.as_slice(),c([0,1063520312]),c([0,1063520312])).remap_2d();
slice.extend_left(f(1008981770));eq_coord(slice.start(),[0,1063502136]);eq_coord(slice.end(),[0,1063520312]);
}
{let p=points(&[[3247798851,1058059480,3233609334],[3240528963,1083175319,3219678495]]);
let mut slice=CurveSlice::<Vec3,_,super::super::CurveCoordSemantic2D>::from_coord_range(p.as_slice(),c([0,1061707332]),c([0,1064026850])).remap_2d();
slice.extend_left(f(1077936128));eq_coord(slice.start(),[0,1055544868]);eq_coord(slice.end(),[0,1064026850]);
}
{let p=points(&[[3247798851,1058059480,3233609334],[3240528963,1083175319,3219678495]]);
let mut slice=CurveSlice::<Vec3,_,super::super::CurveCoordSemantic2D>::from_coord_range(p.as_slice(),c([0,1059353040]),c([0,1059353040])).remap_2d();
slice.extend_left(f(1120403456));eq_coord(slice.start(),[0,0]);eq_coord(slice.end(),[0,1059353040]);
}
{let p=points(&[[3247798851,1058059480,3233609334],[3240528963,1083175319,3219678495]]);
let mut slice=CurveSlice::<Vec3,_,super::super::CurveCoordSemantic2D>::from_coord_range(p.as_slice(),c([0,1048172792]),c([0,1058512223])).remap_2d();
slice.extend_left(f(2139095040));eq_coord(slice.start(),[0,0]);eq_coord(slice.end(),[0,1058512223]);
}
{let p=points(&[[3247798851,1058059480,3233609334],[3240528963,1083175319,3219678495]]);
let mut slice=CurveSlice::<Vec3,_,super::super::CurveCoordSemantic2D>::from_coord_range(p.as_slice(),c([0,1063782068]),c([0,1063782068])).remap_2d();
assert!(std::panic::catch_unwind(std::panic::AssertUnwindSafe(||{slice.extend_left(f(3212836864))})).is_err());
}
{let p=points(&[[3247798851,1058059480,3233609334],[3240528963,1083175319,3219678495]]);
let mut slice=CurveSlice::<Vec3,_,super::super::CurveCoordSemantic2D>::from_coord_range(p.as_slice(),c([0,1060054381]),c([0,1061230180])).remap_2d();
assert!(std::panic::catch_unwind(std::panic::AssertUnwindSafe(||{slice.extend_left(f(2143289344))})).is_err());
}
{let p=points(&[[3248254451,3241936917,1099955010],[1079140333,3244934773,3213218942],[1090209441,1052470290,3230079560],[1095496662,3240322186,1095800513],[1092103392,3242769597,3226461344],[1087355470,1093338760,1090907651],[3225938952,1100032890,1099521180]]);
let mut slice=CurveSlice::<Vec3,_,super::super::CurveCoordSemantic2D>::from_coord_range(p.as_slice(),c([0,1061885973]),c([3,1056104883])).remap_2d();
slice.extend_left(f(0));eq_coord(slice.start(),[0,1061885973]);eq_coord(slice.end(),[3,1056104883]);
}
{let p=points(&[[3248254451,3241936917,1099955010],[1079140333,3244934773,3213218942],[1090209441,1052470290,3230079560],[1095496662,3240322186,1095800513],[1092103392,3242769597,3226461344],[1087355470,1093338760,1090907651],[3225938952,1100032890,1099521180]]);
let mut slice=CurveSlice::<Vec3,_,super::super::CurveCoordSemantic2D>::from_coord_range(p.as_slice(),c([3,1058762754]),c([5,1062068271])).remap_2d();
slice.extend_left(f(1008981770));eq_coord(slice.start(),[3,1058752649]);eq_coord(slice.end(),[5,1062068271]);
}
{let p=points(&[[3248254451,3241936917,1099955010],[1079140333,3244934773,3213218942],[1090209441,1052470290,3230079560],[1095496662,3240322186,1095800513],[1092103392,3242769597,3226461344],[1087355470,1093338760,1090907651],[3225938952,1100032890,1099521180]]);
let mut slice=CurveSlice::<Vec3,_,super::super::CurveCoordSemantic2D>::from_coord_range(p.as_slice(),c([2,1058095490]),c([5,1064351880])).remap_2d();
slice.extend_left(f(1077936128));eq_coord(slice.start(),[2,1053615079]);eq_coord(slice.end(),[5,1064351880]);
}
{let p=points(&[[3248254451,3241936917,1099955010],[1079140333,3244934773,3213218942],[1090209441,1052470290,3230079560],[1095496662,3240322186,1095800513],[1092103392,3242769597,3226461344],[1087355470,1093338760,1090907651],[3225938952,1100032890,1099521180]]);
let mut slice=CurveSlice::<Vec3,_,super::super::CurveCoordSemantic2D>::from_coord_range(p.as_slice(),c([4,1064403730]),c([5,1057345867])).remap_2d();
slice.extend_left(f(1120403456));eq_coord(slice.start(),[0,0]);eq_coord(slice.end(),[5,1057345867]);
}
{let p=points(&[[3248254451,3241936917,1099955010],[1079140333,3244934773,3213218942],[1090209441,1052470290,3230079560],[1095496662,3240322186,1095800513],[1092103392,3242769597,3226461344],[1087355470,1093338760,1090907651],[3225938952,1100032890,1099521180]]);
let mut slice=CurveSlice::<Vec3,_,super::super::CurveCoordSemantic2D>::from_coord_range(p.as_slice(),c([0,1052194530]),c([2,1059650752])).remap_2d();
slice.extend_left(f(2139095040));eq_coord(slice.start(),[0,0]);eq_coord(slice.end(),[2,1059650752]);
}
{let p=points(&[[3248254451,3241936917,1099955010],[1079140333,3244934773,3213218942],[1090209441,1052470290,3230079560],[1095496662,3240322186,1095800513],[1092103392,3242769597,3226461344],[1087355470,1093338760,1090907651],[3225938952,1100032890,1099521180]]);
let mut slice=CurveSlice::<Vec3,_,super::super::CurveCoordSemantic2D>::from_coord_range(p.as_slice(),c([2,1010610944]),c([5,1062908959])).remap_2d();
assert!(std::panic::catch_unwind(std::panic::AssertUnwindSafe(||{slice.extend_left(f(3212836864))})).is_err());
}
{let p=points(&[[3248254451,3241936917,1099955010],[1079140333,3244934773,3213218942],[1090209441,1052470290,3230079560],[1095496662,3240322186,1095800513],[1092103392,3242769597,3226461344],[1087355470,1093338760,1090907651],[3225938952,1100032890,1099521180]]);
let mut slice=CurveSlice::<Vec3,_,super::super::CurveCoordSemantic2D>::from_coord_range(p.as_slice(),c([3,1055929655]),c([5,1058230996])).remap_2d();
assert!(std::panic::catch_unwind(std::panic::AssertUnwindSafe(||{slice.extend_left(f(2143289344))})).is_err());
}
{let p=points(&[[3212759075,3245919388,3242556122],[3229372900,3233527430,3234242874],[3247490566,1099735996,3246681455],[1100591571,3246526217,1094965764],[1099566613,3242955625,999123697],[3238120409,3235777217,1097885840],[3244363478,1099288271,1088099009],[1080504870,1075892897,3239915365]]);
let mut slice=CurveSlice::<Vec3,_,super::super::CurveCoordSemantic2D>::from_coord_range(p.as_slice(),c([3,1058239219]),c([6,1064600529])).remap_2d();
slice.extend_left(f(0));eq_coord(slice.start(),[3,1058239219]);eq_coord(slice.end(),[6,1064600529]);
}
{let p=points(&[[3212759075,3245919388,3242556122],[3229372900,3233527430,3234242874],[3247490566,1099735996,3246681455],[1100591571,3246526217,1094965764],[1099566613,3242955625,999123697],[3238120409,3235777217,1097885840],[3244363478,1099288271,1088099009],[1080504870,1075892897,3239915365]]);
let mut slice=CurveSlice::<Vec3,_,super::super::CurveCoordSemantic2D>::from_coord_range(p.as_slice(),c([1,1062035609]),c([5,1023547938])).remap_2d();
slice.extend_left(f(1008981770));eq_coord(slice.start(),[1,1062026040]);eq_coord(slice.end(),[5,1023547938]);
}
{let p=points(&[[3212759075,3245919388,3242556122],[3229372900,3233527430,3234242874],[3247490566,1099735996,3246681455],[1100591571,3246526217,1094965764],[1099566613,3242955625,999123697],[3238120409,3235777217,1097885840],[3244363478,1099288271,1088099009],[1080504870,1075892897,3239915365]]);
let mut slice=CurveSlice::<Vec3,_,super::super::CurveCoordSemantic2D>::from_coord_range(p.as_slice(),c([0,1015543392]),c([5,1064036374])).remap_2d();
slice.extend_left(f(1077936128));eq_coord(slice.start(),[0,0]);eq_coord(slice.end(),[5,1064036374]);
}
{let p=points(&[[3212759075,3245919388,3242556122],[3229372900,3233527430,3234242874],[3247490566,1099735996,3246681455],[1100591571,3246526217,1094965764],[1099566613,3242955625,999123697],[3238120409,3235777217,1097885840],[3244363478,1099288271,1088099009],[1080504870,1075892897,3239915365]]);
let mut slice=CurveSlice::<Vec3,_,super::super::CurveCoordSemantic2D>::from_coord_range(p.as_slice(),c([6,1044205828]),c([6,1062189588])).remap_2d();
slice.extend_left(f(1120403456));eq_coord(slice.start(),[2,1033695143]);eq_coord(slice.end(),[6,1062189588]);
}
{let p=points(&[[3212759075,3245919388,3242556122],[3229372900,3233527430,3234242874],[3247490566,1099735996,3246681455],[1100591571,3246526217,1094965764],[1099566613,3242955625,999123697],[3238120409,3235777217,1097885840],[3244363478,1099288271,1088099009],[1080504870,1075892897,3239915365]]);
let mut slice=CurveSlice::<Vec3,_,super::super::CurveCoordSemantic2D>::from_coord_range(p.as_slice(),c([6,1023403005]),c([6,1057177821])).remap_2d();
slice.extend_left(f(2139095040));eq_coord(slice.start(),[0,0]);eq_coord(slice.end(),[6,1057177821]);
}
{let p=points(&[[3212759075,3245919388,3242556122],[3229372900,3233527430,3234242874],[3247490566,1099735996,3246681455],[1100591571,3246526217,1094965764],[1099566613,3242955625,999123697],[3238120409,3235777217,1097885840],[3244363478,1099288271,1088099009],[1080504870,1075892897,3239915365]]);
let mut slice=CurveSlice::<Vec3,_,super::super::CurveCoordSemantic2D>::from_coord_range(p.as_slice(),c([4,1060012810]),c([5,1059169182])).remap_2d();
assert!(std::panic::catch_unwind(std::panic::AssertUnwindSafe(||{slice.extend_left(f(3212836864))})).is_err());
}
{let p=points(&[[3212759075,3245919388,3242556122],[3229372900,3233527430,3234242874],[3247490566,1099735996,3246681455],[1100591571,3246526217,1094965764],[1099566613,3242955625,999123697],[3238120409,3235777217,1097885840],[3244363478,1099288271,1088099009],[1080504870,1075892897,3239915365]]);
let mut slice=CurveSlice::<Vec3,_,super::super::CurveCoordSemantic2D>::from_coord_range(p.as_slice(),c([0,1055736440]),c([1,1049825019])).remap_2d();
assert!(std::panic::catch_unwind(std::panic::AssertUnwindSafe(||{slice.extend_left(f(2143289344))})).is_err());
}
{let p=points(&[[1097867177,1082561744,3236069917],[1100696440,3214659718,3240360452],[1097567244,1098257180,1100860970],[3235053765,1087985259,3240223001]]);
let mut slice=CurveSlice::<Vec3,_,super::super::CurveCoordSemantic2D>::from_coord_range(p.as_slice(),c([2,1045142437]),c([2,1062940415])).remap_2d();
slice.extend_left(f(0));eq_coord(slice.start(),[2,1045142436]);eq_coord(slice.end(),[2,1062940415]);
}
{let p=points(&[[1097867177,1082561744,3236069917],[1100696440,3214659718,3240360452],[1097567244,1098257180,1100860970],[3235053765,1087985259,3240223001]]);
let mut slice=CurveSlice::<Vec3,_,super::super::CurveCoordSemantic2D>::from_coord_range(p.as_slice(),c([2,1065303941]),c([2,1065303941])).remap_2d();
slice.extend_left(f(1008981770));eq_coord(slice.start(),[2,1065299367]);eq_coord(slice.end(),[2,1065303941]);
}
{let p=points(&[[1097867177,1082561744,3236069917],[1100696440,3214659718,3240360452],[1097567244,1098257180,1100860970],[3235053765,1087985259,3240223001]]);
let mut slice=CurveSlice::<Vec3,_,super::super::CurveCoordSemantic2D>::from_coord_range(p.as_slice(),c([0,1044069075]),c([2,1041054868])).remap_2d();
slice.extend_left(f(1077936128));eq_coord(slice.start(),[0,0]);eq_coord(slice.end(),[2,1041054868]);
}
{let p=points(&[[1097867177,1082561744,3236069917],[1100696440,3214659718,3240360452],[1097567244,1098257180,1100860970],[3235053765,1087985259,3240223001]]);
let mut slice=CurveSlice::<Vec3,_,super::super::CurveCoordSemantic2D>::from_coord_range(p.as_slice(),c([0,1063512224]),c([1,1055971519])).remap_2d();
slice.extend_left(f(1120403456));eq_coord(slice.start(),[0,0]);eq_coord(slice.end(),[1,1055971519]);
}
{let p=points(&[[1097867177,1082561744,3236069917],[1100696440,3214659718,3240360452],[1097567244,1098257180,1100860970],[3235053765,1087985259,3240223001]]);
let mut slice=CurveSlice::<Vec3,_,super::super::CurveCoordSemantic2D>::from_coord_range(p.as_slice(),c([1,1034516477]),c([1,1058347024])).remap_2d();
slice.extend_left(f(2139095040));eq_coord(slice.start(),[0,0]);eq_coord(slice.end(),[1,1058347024]);
}
{let p=points(&[[1097867177,1082561744,3236069917],[1100696440,3214659718,3240360452],[1097567244,1098257180,1100860970],[3235053765,1087985259,3240223001]]);
let mut slice=CurveSlice::<Vec3,_,super::super::CurveCoordSemantic2D>::from_coord_range(p.as_slice(),c([0,1046742842]),c([0,1049367416])).remap_2d();
assert!(std::panic::catch_unwind(std::panic::AssertUnwindSafe(||{slice.extend_left(f(3212836864))})).is_err());
}
{let p=points(&[[1097867177,1082561744,3236069917],[1100696440,3214659718,3240360452],[1097567244,1098257180,1100860970],[3235053765,1087985259,3240223001]]);
let mut slice=CurveSlice::<Vec3,_,super::super::CurveCoordSemantic2D>::from_coord_range(p.as_slice(),c([0,1062486813]),c([1,1048912735])).remap_2d();
assert!(std::panic::catch_unwind(std::panic::AssertUnwindSafe(||{slice.extend_left(f(2143289344))})).is_err());
}
{let p=points(&[[1099010029,3244580215,1083184304],[1093727935,3231322885,3243514808],[3244515978,1099562993,1098985396],[3247736614,1094079983,1099238188],[1090771178,3233238451,3247286416],[1076830784,3243246862,1087956944],[3232418479,3231129765,3226472668]]);
let mut slice=CurveSlice::<Vec3,_,super::super::CurveCoordSemantic2D>::from_coord_range(p.as_slice(),c([3,1055437349]),c([5,1052678283])).remap_2d();
slice.extend_left(f(0));eq_coord(slice.start(),[3,1055437349]);eq_coord(slice.end(),[5,1052678283]);
}
{let p=points(&[[1099010029,3244580215,1083184304],[1093727935,3231322885,3243514808],[3244515978,1099562993,1098985396],[3247736614,1094079983,1099238188],[1090771178,3233238451,3247286416],[1076830784,3243246862,1087956944],[3232418479,3231129765,3226472668]]);
let mut slice=CurveSlice::<Vec3,_,super::super::CurveCoordSemantic2D>::from_coord_range(p.as_slice(),c([5,1063603436]),c([5,1063603436])).remap_2d();
slice.extend_left(f(1008981770));eq_coord(slice.start(),[5,1063590405]);eq_coord(slice.end(),[5,1063603436]);
}
{let p=points(&[[1099010029,3244580215,1083184304],[1093727935,3231322885,3243514808],[3244515978,1099562993,1098985396],[3247736614,1094079983,1099238188],[1090771178,3233238451,3247286416],[1076830784,3243246862,1087956944],[3232418479,3231129765,3226472668]]);
let mut slice=CurveSlice::<Vec3,_,super::super::CurveCoordSemantic2D>::from_coord_range(p.as_slice(),c([3,1047808550]),c([4,1065290385])).remap_2d();
slice.extend_left(f(1077936128));eq_coord(slice.start(),[3,1043186965]);eq_coord(slice.end(),[4,1065290385]);
}
{let p=points(&[[1099010029,3244580215,1083184304],[1093727935,3231322885,3243514808],[3244515978,1099562993,1098985396],[3247736614,1094079983,1099238188],[1090771178,3233238451,3247286416],[1076830784,3243246862,1087956944],[3232418479,3231129765,3226472668]]);
let mut slice=CurveSlice::<Vec3,_,super::super::CurveCoordSemantic2D>::from_coord_range(p.as_slice(),c([0,1056846523]),c([0,1056846523])).remap_2d();
slice.extend_left(f(1120403456));eq_coord(slice.start(),[0,0]);eq_coord(slice.end(),[0,1056846523]);
}
{let p=points(&[[1099010029,3244580215,1083184304],[1093727935,3231322885,3243514808],[3244515978,1099562993,1098985396],[3247736614,1094079983,1099238188],[1090771178,3233238451,3247286416],[1076830784,3243246862,1087956944],[3232418479,3231129765,3226472668]]);
let mut slice=CurveSlice::<Vec3,_,super::super::CurveCoordSemantic2D>::from_coord_range(p.as_slice(),c([0,1059195599]),c([1,1039882038])).remap_2d();
slice.extend_left(f(2139095040));eq_coord(slice.start(),[0,0]);eq_coord(slice.end(),[1,1039882038]);
}
{let p=points(&[[1099010029,3244580215,1083184304],[1093727935,3231322885,3243514808],[3244515978,1099562993,1098985396],[3247736614,1094079983,1099238188],[1090771178,3233238451,3247286416],[1076830784,3243246862,1087956944],[3232418479,3231129765,3226472668]]);
let mut slice=CurveSlice::<Vec3,_,super::super::CurveCoordSemantic2D>::from_coord_range(p.as_slice(),c([5,1058014107]),c([5,1059383153])).remap_2d();
assert!(std::panic::catch_unwind(std::panic::AssertUnwindSafe(||{slice.extend_left(f(3212836864))})).is_err());
}
{let p=points(&[[1099010029,3244580215,1083184304],[1093727935,3231322885,3243514808],[3244515978,1099562993,1098985396],[3247736614,1094079983,1099238188],[1090771178,3233238451,3247286416],[1076830784,3243246862,1087956944],[3232418479,3231129765,3226472668]]);
let mut slice=CurveSlice::<Vec3,_,super::super::CurveCoordSemantic2D>::from_coord_range(p.as_slice(),c([2,1065146151]),c([2,1065146151])).remap_2d();
assert!(std::panic::catch_unwind(std::panic::AssertUnwindSafe(||{slice.extend_left(f(2143289344))})).is_err());
}
{let p=points(&[[3230146028,3235895519,3247361702],[1100544358,3247787439,1088863577],[3247088989,1100658099,3239814798],[1058223971,3231476188,3242121657],[3247695502,1097243273,1086462731],[3242328033,1097305974,1088204397],[3245446996,1095672463,3238153751]]);
let mut slice=CurveSlice::<Vec3,_,super::super::CurveCoordSemantic2D>::from_coord_range(p.as_slice(),c([2,1061498121]),c([2,1061498121])).remap_2d();
slice.extend_left(f(0));eq_coord(slice.start(),[2,1061498121]);eq_coord(slice.end(),[2,1061498121]);
}
{let p=points(&[[3230146028,3235895519,3247361702],[1100544358,3247787439,1088863577],[3247088989,1100658099,3239814798],[1058223971,3231476188,3242121657],[3247695502,1097243273,1086462731],[3242328033,1097305974,1088204397],[3245446996,1095672463,3238153751]]);
let mut slice=CurveSlice::<Vec3,_,super::super::CurveCoordSemantic2D>::from_coord_range(p.as_slice(),c([3,1041920020]),c([4,1061857144])).remap_2d();
slice.extend_left(f(1008981770));eq_coord(slice.start(),[3,1041894420]);eq_coord(slice.end(),[4,1061857144]);
}
{let p=points(&[[3230146028,3235895519,3247361702],[1100544358,3247787439,1088863577],[3247088989,1100658099,3239814798],[1058223971,3231476188,3242121657],[3247695502,1097243273,1086462731],[3242328033,1097305974,1088204397],[3245446996,1095672463,3238153751]]);
let mut slice=CurveSlice::<Vec3,_,super::super::CurveCoordSemantic2D>::from_coord_range(p.as_slice(),c([2,1064883515]),c([5,1000843747])).remap_2d();
slice.extend_left(f(1077936128));eq_coord(slice.start(),[2,1062093582]);eq_coord(slice.end(),[5,1000843747]);
}
{let p=points(&[[3230146028,3235895519,3247361702],[1100544358,3247787439,1088863577],[3247088989,1100658099,3239814798],[1058223971,3231476188,3242121657],[3247695502,1097243273,1086462731],[3242328033,1097305974,1088204397],[3245446996,1095672463,3238153751]]);
let mut slice=CurveSlice::<Vec3,_,super::super::CurveCoordSemantic2D>::from_coord_range(p.as_slice(),c([0,1063786280]),c([4,1046535994])).remap_2d();
slice.extend_left(f(1120403456));eq_coord(slice.start(),[0,0]);eq_coord(slice.end(),[4,1046535994]);
}
{let p=points(&[[3230146028,3235895519,3247361702],[1100544358,3247787439,1088863577],[3247088989,1100658099,3239814798],[1058223971,3231476188,3242121657],[3247695502,1097243273,1086462731],[3242328033,1097305974,1088204397],[3245446996,1095672463,3238153751]]);
let mut slice=CurveSlice::<Vec3,_,super::super::CurveCoordSemantic2D>::from_coord_range(p.as_slice(),c([0,1047941199]),c([0,1047941199])).remap_2d();
slice.extend_left(f(2139095040));eq_coord(slice.start(),[0,0]);eq_coord(slice.end(),[0,1047941199]);
}
{let p=points(&[[3230146028,3235895519,3247361702],[1100544358,3247787439,1088863577],[3247088989,1100658099,3239814798],[1058223971,3231476188,3242121657],[3247695502,1097243273,1086462731],[3242328033,1097305974,1088204397],[3245446996,1095672463,3238153751]]);
let mut slice=CurveSlice::<Vec3,_,super::super::CurveCoordSemantic2D>::from_coord_range(p.as_slice(),c([4,1063276960]),c([4,1063276960])).remap_2d();
assert!(std::panic::catch_unwind(std::panic::AssertUnwindSafe(||{slice.extend_left(f(3212836864))})).is_err());
}
{let p=points(&[[3230146028,3235895519,3247361702],[1100544358,3247787439,1088863577],[3247088989,1100658099,3239814798],[1058223971,3231476188,3242121657],[3247695502,1097243273,1086462731],[3242328033,1097305974,1088204397],[3245446996,1095672463,3238153751]]);
let mut slice=CurveSlice::<Vec3,_,super::super::CurveCoordSemantic2D>::from_coord_range(p.as_slice(),c([5,1039019732]),c([5,1051311132])).remap_2d();
assert!(std::panic::catch_unwind(std::panic::AssertUnwindSafe(||{slice.extend_left(f(2143289344))})).is_err());
}
{let p=points(&[[3244590905,3197112644,3241275444],[1091470735,1078434697,3246801676]]);
let mut slice=CurveSlice::<Vec3,_,super::super::CurveCoordSemantic2D>::from_coord_range(p.as_slice(),c([0,1052975101]),c([0,1052975101])).remap_2d();
slice.extend_left(f(0));eq_coord(slice.start(),[0,1052975100]);eq_coord(slice.end(),[0,1052975101]);
}
{let p=points(&[[3244590905,3197112644,3241275444],[1091470735,1078434697,3246801676]]);
let mut slice=CurveSlice::<Vec3,_,super::super::CurveCoordSemantic2D>::from_coord_range(p.as_slice(),c([0,1062938425]),c([0,1062938425])).remap_2d();
slice.extend_left(f(1008981770));eq_coord(slice.start(),[0,1062931397]);eq_coord(slice.end(),[0,1062938425]);
}
{let p=points(&[[3244590905,3197112644,3241275444],[1091470735,1078434697,3246801676]]);
let mut slice=CurveSlice::<Vec3,_,super::super::CurveCoordSemantic2D>::from_coord_range(p.as_slice(),c([0,1048932144]),c([0,1055504331])).remap_2d();
slice.extend_left(f(1077936128));eq_coord(slice.start(),[0,1040854598]);eq_coord(slice.end(),[0,1055504331]);
}
{let p=points(&[[3244590905,3197112644,3241275444],[1091470735,1078434697,3246801676]]);
let mut slice=CurveSlice::<Vec3,_,super::super::CurveCoordSemantic2D>::from_coord_range(p.as_slice(),c([0,1054896535]),c([0,1054896535])).remap_2d();
slice.extend_left(f(1120403456));eq_coord(slice.start(),[0,0]);eq_coord(slice.end(),[0,1054896535]);
}
{let p=points(&[[3244590905,3197112644,3241275444],[1091470735,1078434697,3246801676]]);
let mut slice=CurveSlice::<Vec3,_,super::super::CurveCoordSemantic2D>::from_coord_range(p.as_slice(),c([0,1049879893]),c([0,1063027519])).remap_2d();
slice.extend_left(f(2139095040));eq_coord(slice.start(),[0,0]);eq_coord(slice.end(),[0,1063027519]);
}
{let p=points(&[[3244590905,3197112644,3241275444],[1091470735,1078434697,3246801676]]);
let mut slice=CurveSlice::<Vec3,_,super::super::CurveCoordSemantic2D>::from_coord_range(p.as_slice(),c([0,1060404180]),c([0,1060404180])).remap_2d();
assert!(std::panic::catch_unwind(std::panic::AssertUnwindSafe(||{slice.extend_left(f(3212836864))})).is_err());
}
{let p=points(&[[3244590905,3197112644,3241275444],[1091470735,1078434697,3246801676]]);
let mut slice=CurveSlice::<Vec3,_,super::super::CurveCoordSemantic2D>::from_coord_range(p.as_slice(),c([0,1060525236]),c([0,1060525236])).remap_2d();
assert!(std::panic::catch_unwind(std::panic::AssertUnwindSafe(||{slice.extend_left(f(2143289344))})).is_err());
}
{let p=points(&[[3240111851,3214152468,3248294313],[3227393663,1094220890,1099303283],[1096948280,1096577260,3244078806],[3247117193,3203861168,3242412098],[3214212390,1097191631,3247085682],[1090525399,3239617351,3238254111]]);
let mut slice=CurveSlice::<Vec3,_,super::super::CurveCoordSemantic2D>::from_coord_range(p.as_slice(),c([1,1064081358]),c([1,1064081358])).remap_2d();
slice.extend_left(f(0));eq_coord(slice.start(),[1,1064081357]);eq_coord(slice.end(),[1,1064081358]);
}
{let p=points(&[[3240111851,3214152468,3248294313],[3227393663,1094220890,1099303283],[1096948280,1096577260,3244078806],[3247117193,3203861168,3242412098],[3214212390,1097191631,3247085682],[1090525399,3239617351,3238254111]]);
let mut slice=CurveSlice::<Vec3,_,super::super::CurveCoordSemantic2D>::from_coord_range(p.as_slice(),c([0,1030973887]),c([4,1061836831])).remap_2d();
slice.extend_left(f(1008981770));eq_coord(slice.start(),[0,1030901275]);eq_coord(slice.end(),[4,1061836831]);
}
{let p=points(&[[3240111851,3214152468,3248294313],[3227393663,1094220890,1099303283],[1096948280,1096577260,3244078806],[3247117193,3203861168,3242412098],[3214212390,1097191631,3247085682],[1090525399,3239617351,3238254111]]);
let mut slice=CurveSlice::<Vec3,_,super::super::CurveCoordSemantic2D>::from_coord_range(p.as_slice(),c([1,1058719836]),c([3,1049547467])).remap_2d();
slice.extend_left(f(1077936128));eq_coord(slice.start(),[1,1057292290]);eq_coord(slice.end(),[3,1049547467]);
}
{let p=points(&[[3240111851,3214152468,3248294313],[3227393663,1094220890,1099303283],[1096948280,1096577260,3244078806],[3247117193,3203861168,3242412098],[3214212390,1097191631,3247085682],[1090525399,3239617351,3238254111]]);
let mut slice=CurveSlice::<Vec3,_,super::super::CurveCoordSemantic2D>::from_coord_range(p.as_slice(),c([2,1057815878]),c([2,1057815878])).remap_2d();
slice.extend_left(f(1120403456));eq_coord(slice.start(),[0,0]);eq_coord(slice.end(),[2,1057815878]);
}
{let p=points(&[[3240111851,3214152468,3248294313],[3227393663,1094220890,1099303283],[1096948280,1096577260,3244078806],[3247117193,3203861168,3242412098],[3214212390,1097191631,3247085682],[1090525399,3239617351,3238254111]]);
let mut slice=CurveSlice::<Vec3,_,super::super::CurveCoordSemantic2D>::from_coord_range(p.as_slice(),c([4,1056826771]),c([4,1056826771])).remap_2d();
slice.extend_left(f(2139095040));eq_coord(slice.start(),[0,0]);eq_coord(slice.end(),[4,1056826771]);
}
{let p=points(&[[3240111851,3214152468,3248294313],[3227393663,1094220890,1099303283],[1096948280,1096577260,3244078806],[3247117193,3203861168,3242412098],[3214212390,1097191631,3247085682],[1090525399,3239617351,3238254111]]);
let mut slice=CurveSlice::<Vec3,_,super::super::CurveCoordSemantic2D>::from_coord_range(p.as_slice(),c([1,1050801248]),c([2,1058637192])).remap_2d();
assert!(std::panic::catch_unwind(std::panic::AssertUnwindSafe(||{slice.extend_left(f(3212836864))})).is_err());
}
{let p=points(&[[3240111851,3214152468,3248294313],[3227393663,1094220890,1099303283],[1096948280,1096577260,3244078806],[3247117193,3203861168,3242412098],[3214212390,1097191631,3247085682],[1090525399,3239617351,3238254111]]);
let mut slice=CurveSlice::<Vec3,_,super::super::CurveCoordSemantic2D>::from_coord_range(p.as_slice(),c([1,1061210540]),c([3,1058745237])).remap_2d();
assert!(std::panic::catch_unwind(std::panic::AssertUnwindSafe(||{slice.extend_left(f(2143289344))})).is_err());
}
{let p=points(&[[3238526442,3225180439,1093488061],[1078041480,1093017581,1097536225]]);
let mut slice=CurveSlice::<Vec3,_,super::super::CurveCoordSemantic2D>::from_coord_range(p.as_slice(),c([0,1041148242]),c([0,1060274854])).remap_2d();
slice.extend_left(f(0));eq_coord(slice.start(),[0,1041148240]);eq_coord(slice.end(),[0,1060274854]);
}
{let p=points(&[[3238526442,3225180439,1093488061],[1078041480,1093017581,1097536225]]);
let mut slice=CurveSlice::<Vec3,_,super::super::CurveCoordSemantic2D>::from_coord_range(p.as_slice(),c([0,1061385566]),c([0,1061385566])).remap_2d();
slice.extend_left(f(1008981770));eq_coord(slice.start(),[0,1061371762]);eq_coord(slice.end(),[0,1061385566]);
}
{let p=points(&[[3238526442,3225180439,1093488061],[1078041480,1093017581,1097536225]]);
let mut slice=CurveSlice::<Vec3,_,super::super::CurveCoordSemantic2D>::from_coord_range(p.as_slice(),c([0,1029374445]),c([0,1049525237])).remap_2d();
slice.extend_left(f(1077936128));eq_coord(slice.start(),[0,0]);eq_coord(slice.end(),[0,1049525237]);
}
{let p=points(&[[3238526442,3225180439,1093488061],[1078041480,1093017581,1097536225]]);
let mut slice=CurveSlice::<Vec3,_,super::super::CurveCoordSemantic2D>::from_coord_range(p.as_slice(),c([0,1044199362]),c([0,1062649204])).remap_2d();
slice.extend_left(f(1120403456));eq_coord(slice.start(),[0,0]);eq_coord(slice.end(),[0,1062649204]);
}
{let p=points(&[[3238526442,3225180439,1093488061],[1078041480,1093017581,1097536225]]);
let mut slice=CurveSlice::<Vec3,_,super::super::CurveCoordSemantic2D>::from_coord_range(p.as_slice(),c([0,1053732958]),c([0,1059726833])).remap_2d();
slice.extend_left(f(2139095040));eq_coord(slice.start(),[0,0]);eq_coord(slice.end(),[0,1059726833]);
}
{let p=points(&[[3238526442,3225180439,1093488061],[1078041480,1093017581,1097536225]]);
let mut slice=CurveSlice::<Vec3,_,super::super::CurveCoordSemantic2D>::from_coord_range(p.as_slice(),c([0,1057483332]),c([0,1062626428])).remap_2d();
assert!(std::panic::catch_unwind(std::panic::AssertUnwindSafe(||{slice.extend_left(f(3212836864))})).is_err());
}
{let p=points(&[[3238526442,3225180439,1093488061],[1078041480,1093017581,1097536225]]);
let mut slice=CurveSlice::<Vec3,_,super::super::CurveCoordSemantic2D>::from_coord_range(p.as_slice(),c([0,1046039652]),c([0,1046039652])).remap_2d();
assert!(std::panic::catch_unwind(std::panic::AssertUnwindSafe(||{slice.extend_left(f(2143289344))})).is_err());
}
{let p=points(&[[3222271324,1078079688,3243825342],[1096982494,1079568647,1099405603],[3242841411,3240632831,3245247096],[3246731317,3233967264,1096598129],[3246450232,3240616278,3242819378],[3243045751,3244783215,3236906801],[1079076129,1087281926,3247552020],[3228913317,3246544670,1085545680]]);
let mut slice=CurveSlice::<Vec3,_,super::super::CurveCoordSemantic2D>::from_coord_range(p.as_slice(),c([2,1051791153]),c([5,1061800027])).remap_2d();
slice.extend_left(f(0));eq_coord(slice.start(),[2,1051791152]);eq_coord(slice.end(),[5,1061800027]);
}
{let p=points(&[[3222271324,1078079688,3243825342],[1096982494,1079568647,1099405603],[3242841411,3240632831,3245247096],[3246731317,3233967264,1096598129],[3246450232,3240616278,3242819378],[3243045751,3244783215,3236906801],[1079076129,1087281926,3247552020],[3228913317,3246544670,1085545680]]);
let mut slice=CurveSlice::<Vec3,_,super::super::CurveCoordSemantic2D>::from_coord_range(p.as_slice(),c([0,1054825345]),c([1,1061452534])).remap_2d();
slice.extend_left(f(1008981770));eq_coord(slice.start(),[0,1054815656]);eq_coord(slice.end(),[1,1061452534]);
}
{let p=points(&[[3222271324,1078079688,3243825342],[1096982494,1079568647,1099405603],[3242841411,3240632831,3245247096],[3246731317,3233967264,1096598129],[3246450232,3240616278,3242819378],[3243045751,3244783215,3236906801],[1079076129,1087281926,3247552020],[3228913317,3246544670,1085545680]]);
let mut slice=CurveSlice::<Vec3,_,super::super::CurveCoordSemantic2D>::from_coord_range(p.as_slice(),c([3,1058128134]),c([4,1043815609])).remap_2d();
slice.extend_left(f(1077936128));eq_coord(slice.start(),[3,1055478146]);eq_coord(slice.end(),[4,1043815609]);
}
{let p=points(&[[3222271324,1078079688,3243825342],[1096982494,1079568647,1099405603],[3242841411,3240632831,3245247096],[3246731317,3233967264,1096598129],[3246450232,3240616278,3242819378],[3243045751,3244783215,3236906801],[1079076129,1087281926,3247552020],[3228913317,3246544670,1085545680]]);
let mut slice=CurveSlice::<Vec3,_,super::super::CurveCoordSemantic2D>::from_coord_range(p.as_slice(),c([0,1049913637]),c([3,1059261143])).remap_2d();
slice.extend_left(f(1120403456));eq_coord(slice.start(),[0,0]);eq_coord(slice.end(),[3,1059261143]);
}
{let p=points(&[[3222271324,1078079688,3243825342],[1096982494,1079568647,1099405603],[3242841411,3240632831,3245247096],[3246731317,3233967264,1096598129],[3246450232,3240616278,3242819378],[3243045751,3244783215,3236906801],[1079076129,1087281926,3247552020],[3228913317,3246544670,1085545680]]);
let mut slice=CurveSlice::<Vec3,_,super::super::CurveCoordSemantic2D>::from_coord_range(p.as_slice(),c([0,1054802742]),c([6,1034746638])).remap_2d();
slice.extend_left(f(2139095040));eq_coord(slice.start(),[0,0]);eq_coord(slice.end(),[6,1034746638]);
}
{let p=points(&[[3222271324,1078079688,3243825342],[1096982494,1079568647,1099405603],[3242841411,3240632831,3245247096],[3246731317,3233967264,1096598129],[3246450232,3240616278,3242819378],[3243045751,3244783215,3236906801],[1079076129,1087281926,3247552020],[3228913317,3246544670,1085545680]]);
let mut slice=CurveSlice::<Vec3,_,super::super::CurveCoordSemantic2D>::from_coord_range(p.as_slice(),c([4,1062817263]),c([4,1062817263])).remap_2d();
assert!(std::panic::catch_unwind(std::panic::AssertUnwindSafe(||{slice.extend_left(f(3212836864))})).is_err());
}
{let p=points(&[[3222271324,1078079688,3243825342],[1096982494,1079568647,1099405603],[3242841411,3240632831,3245247096],[3246731317,3233967264,1096598129],[3246450232,3240616278,3242819378],[3243045751,3244783215,3236906801],[1079076129,1087281926,3247552020],[3228913317,3246544670,1085545680]]);
let mut slice=CurveSlice::<Vec3,_,super::super::CurveCoordSemantic2D>::from_coord_range(p.as_slice(),c([5,1057235623]),c([6,1040938344])).remap_2d();
assert!(std::panic::catch_unwind(std::panic::AssertUnwindSafe(||{slice.extend_left(f(2143289344))})).is_err());
}
{let p=points(&[[1082813449,1096827562,3230281234],[3242052789,1095650379,1094603447],[3243721762,1079838579,1088985230],[1099182513,3216536538,3240457667],[1093553918,1078473990,3237901303],[3241797748,1083693937,1092107875],[1073653720,3247189803,3242372558],[1088451804,1083656160,1098172592]]);
let mut slice=CurveSlice::<Vec3,_,super::super::CurveCoordSemantic2D>::from_coord_range(p.as_slice(),c([0,1057433256]),c([4,1064850405])).remap_2d();
slice.extend_left(f(0));eq_coord(slice.start(),[0,1057433256]);eq_coord(slice.end(),[4,1064850405]);
}
{let p=points(&[[1082813449,1096827562,3230281234],[3242052789,1095650379,1094603447],[3243721762,1079838579,1088985230],[1099182513,3216536538,3240457667],[1093553918,1078473990,3237901303],[3241797748,1083693937,1092107875],[1073653720,3247189803,3242372558],[1088451804,1083656160,1098172592]]);
let mut slice=CurveSlice::<Vec3,_,super::super::CurveCoordSemantic2D>::from_coord_range(p.as_slice(),c([6,1062649979]),c([6,1062649979])).remap_2d();
slice.extend_left(f(1008981770));eq_coord(slice.start(),[6,1062643970]);eq_coord(slice.end(),[6,1062649979]);
}
{let p=points(&[[1082813449,1096827562,3230281234],[3242052789,1095650379,1094603447],[3243721762,1079838579,1088985230],[1099182513,3216536538,3240457667],[1093553918,1078473990,3237901303],[3241797748,1083693937,1092107875],[1073653720,3247189803,3242372558],[1088451804,1083656160,1098172592]]);
let mut slice=CurveSlice::<Vec3,_,super::super::CurveCoordSemantic2D>::from_coord_range(p.as_slice(),c([2,1064290478]),c([4,1058044056])).remap_2d();
slice.extend_left(f(1077936128));eq_coord(slice.start(),[2,1062842834]);eq_coord(slice.end(),[4,1058044056]);
}
{let p=points(&[[1082813449,1096827562,3230281234],[3242052789,1095650379,1094603447],[3243721762,1079838579,1088985230],[1099182513,3216536538,3240457667],[1093553918,1078473990,3237901303],[3241797748,1083693937,1092107875],[1073653720,3247189803,3242372558],[1088451804,1083656160,1098172592]]);
let mut slice=CurveSlice::<Vec3,_,super::super::CurveCoordSemantic2D>::from_coord_range(p.as_slice(),c([1,1056998125]),c([2,1045055995])).remap_2d();
slice.extend_left(f(1120403456));eq_coord(slice.start(),[0,0]);eq_coord(slice.end(),[2,1045055995]);
}
{let p=points(&[[1082813449,1096827562,3230281234],[3242052789,1095650379,1094603447],[3243721762,1079838579,1088985230],[1099182513,3216536538,3240457667],[1093553918,1078473990,3237901303],[3241797748,1083693937,1092107875],[1073653720,3247189803,3242372558],[1088451804,1083656160,1098172592]]);
let mut slice=CurveSlice::<Vec3,_,super::super::CurveCoordSemantic2D>::from_coord_range(p.as_slice(),c([0,1036363951]),c([0,1036363951])).remap_2d();
slice.extend_left(f(2139095040));eq_coord(slice.start(),[0,0]);eq_coord(slice.end(),[0,1036363951]);
}
{let p=points(&[[1082813449,1096827562,3230281234],[3242052789,1095650379,1094603447],[3243721762,1079838579,1088985230],[1099182513,3216536538,3240457667],[1093553918,1078473990,3237901303],[3241797748,1083693937,1092107875],[1073653720,3247189803,3242372558],[1088451804,1083656160,1098172592]]);
let mut slice=CurveSlice::<Vec3,_,super::super::CurveCoordSemantic2D>::from_coord_range(p.as_slice(),c([5,1063145923]),c([6,1045828594])).remap_2d();
assert!(std::panic::catch_unwind(std::panic::AssertUnwindSafe(||{slice.extend_left(f(3212836864))})).is_err());
}
{let p=points(&[[1082813449,1096827562,3230281234],[3242052789,1095650379,1094603447],[3243721762,1079838579,1088985230],[1099182513,3216536538,3240457667],[1093553918,1078473990,3237901303],[3241797748,1083693937,1092107875],[1073653720,3247189803,3242372558],[1088451804,1083656160,1098172592]]);
let mut slice=CurveSlice::<Vec3,_,super::super::CurveCoordSemantic2D>::from_coord_range(p.as_slice(),c([4,1059436131]),c([6,1063450220])).remap_2d();
assert!(std::panic::catch_unwind(std::panic::AssertUnwindSafe(||{slice.extend_left(f(2143289344))})).is_err());
}
{let p=points(&[[3240380506,1094704937,1080930960],[1064909175,3246980524,3247618111],[1096525402,1100801308,1088209986],[3242764860,3191123991,3248147672],[1098848291,1066187263,1096178632],[3244772354,1071902063,1020797699]]);
let mut slice=CurveSlice::<Vec3,_,super::super::CurveCoordSemantic2D>::from_coord_range(p.as_slice(),c([0,1053374317]),c([4,1059239107])).remap_2d();
slice.extend_left(f(0));eq_coord(slice.start(),[0,1053374316]);eq_coord(slice.end(),[4,1059239107]);
}
{let p=points(&[[3240380506,1094704937,1080930960],[1064909175,3246980524,3247618111],[1096525402,1100801308,1088209986],[3242764860,3191123991,3248147672],[1098848291,1066187263,1096178632],[3244772354,1071902063,1020797699]]);
let mut slice=CurveSlice::<Vec3,_,super::super::CurveCoordSemantic2D>::from_coord_range(p.as_slice(),c([4,1064056043]),c([4,1064056043])).remap_2d();
slice.extend_left(f(1008981770));eq_coord(slice.start(),[4,1064050992]);eq_coord(slice.end(),[4,1064056043]);
}
{let p=points(&[[3240380506,1094704937,1080930960],[1064909175,3246980524,3247618111],[1096525402,1100801308,1088209986],[3242764860,3191123991,3248147672],[1098848291,1066187263,1096178632],[3244772354,1071902063,1020797699]]);
let mut slice=CurveSlice::<Vec3,_,super::super::CurveCoordSemantic2D>::from_coord_range(p.as_slice(),c([3,1051316924]),c([4,1053651243])).remap_2d();
slice.extend_left(f(1077936128));eq_coord(slice.start(),[3,1048997633]);eq_coord(slice.end(),[4,1053651243]);
}
{let p=points(&[[3240380506,1094704937,1080930960],[1064909175,3246980524,3247618111],[1096525402,1100801308,1088209986],[3242764860,3191123991,3248147672],[1098848291,1066187263,1096178632],[3244772354,1071902063,1020797699]]);
let mut slice=CurveSlice::<Vec3,_,super::super::CurveCoordSemantic2D>::from_coord_range(p.as_slice(),c([1,1045520709]),c([2,1059094335])).remap_2d();
slice.extend_left(f(1120403456));eq_coord(slice.start(),[0,0]);eq_coord(slice.end(),[2,1059094335]);
}
{let p=points(&[[3240380506,1094704937,1080930960],[1064909175,3246980524,3247618111],[1096525402,1100801308,1088209986],[3242764860,3191123991,3248147672],[1098848291,1066187263,1096178632],[3244772354,1071902063,1020797699]]);
let mut slice=CurveSlice::<Vec3,_,super::super::CurveCoordSemantic2D>::from_coord_range(p.as_slice(),c([4,1051405079]),c([4,1059799668])).remap_2d();
slice.extend_left(f(2139095040));eq_coord(slice.start(),[0,0]);eq_coord(slice.end(),[4,1059799668]);
}
{let p=points(&[[3240380506,1094704937,1080930960],[1064909175,3246980524,3247618111],[1096525402,1100801308,1088209986],[3242764860,3191123991,3248147672],[1098848291,1066187263,1096178632],[3244772354,1071902063,1020797699]]);
let mut slice=CurveSlice::<Vec3,_,super::super::CurveCoordSemantic2D>::from_coord_range(p.as_slice(),c([1,1031380847]),c([4,1049551163])).remap_2d();
assert!(std::panic::catch_unwind(std::panic::AssertUnwindSafe(||{slice.extend_left(f(3212836864))})).is_err());
}
{let p=points(&[[3240380506,1094704937,1080930960],[1064909175,3246980524,3247618111],[1096525402,1100801308,1088209986],[3242764860,3191123991,3248147672],[1098848291,1066187263,1096178632],[3244772354,1071902063,1020797699]]);
let mut slice=CurveSlice::<Vec3,_,super::super::CurveCoordSemantic2D>::from_coord_range(p.as_slice(),c([4,1064934163]),c([4,1064934163])).remap_2d();
assert!(std::panic::catch_unwind(std::panic::AssertUnwindSafe(||{slice.extend_left(f(2143289344))})).is_err());
}
{let p=points(&[[3240405232,1058186416,1097096531],[1099817832,1092237199,3244041696],[3198902661,3238610697,3228428760],[3242466192,3242088876,3232658630],[3225324183,3223441038,1100750355]]);
let mut slice=CurveSlice::<Vec3,_,super::super::CurveCoordSemantic2D>::from_coord_range(p.as_slice(),c([3,1054491959]),c([3,1063514323])).remap_2d();
slice.extend_left(f(0));eq_coord(slice.start(),[3,1054491960]);eq_coord(slice.end(),[3,1063514323]);
}
{let p=points(&[[3240405232,1058186416,1097096531],[1099817832,1092237199,3244041696],[3198902661,3238610697,3228428760],[3242466192,3242088876,3232658630],[3225324183,3223441038,1100750355]]);
let mut slice=CurveSlice::<Vec3,_,super::super::CurveCoordSemantic2D>::from_coord_range(p.as_slice(),c([1,1039420472]),c([1,1060796345])).remap_2d();
slice.extend_left(f(1008981770));eq_coord(slice.start(),[1,1039355549]);eq_coord(slice.end(),[1,1060796345]);
}
{let p=points(&[[3240405232,1058186416,1097096531],[1099817832,1092237199,3244041696],[3198902661,3238610697,3228428760],[3242466192,3242088876,3232658630],[3225324183,3223441038,1100750355]]);
let mut slice=CurveSlice::<Vec3,_,super::super::CurveCoordSemantic2D>::from_coord_range(p.as_slice(),c([2,1042227927]),c([2,1061523916])).remap_2d();
slice.extend_left(f(1077936128));eq_coord(slice.start(),[1,1064438014]);eq_coord(slice.end(),[2,1061523916]);
}
{let p=points(&[[3240405232,1058186416,1097096531],[1099817832,1092237199,3244041696],[3198902661,3238610697,3228428760],[3242466192,3242088876,3232658630],[3225324183,3223441038,1100750355]]);
let mut slice=CurveSlice::<Vec3,_,super::super::CurveCoordSemantic2D>::from_coord_range(p.as_slice(),c([2,1065347843]),c([3,1064958357])).remap_2d();
slice.extend_left(f(1120403456));eq_coord(slice.start(),[0,0]);eq_coord(slice.end(),[3,1064958357]);
}
{let p=points(&[[3240405232,1058186416,1097096531],[1099817832,1092237199,3244041696],[3198902661,3238610697,3228428760],[3242466192,3242088876,3232658630],[3225324183,3223441038,1100750355]]);
let mut slice=CurveSlice::<Vec3,_,super::super::CurveCoordSemantic2D>::from_coord_range(p.as_slice(),c([1,1060716052]),c([3,1045724206])).remap_2d();
slice.extend_left(f(2139095040));eq_coord(slice.start(),[0,0]);eq_coord(slice.end(),[3,1045724206]);
}
{let p=points(&[[3240405232,1058186416,1097096531],[1099817832,1092237199,3244041696],[3198902661,3238610697,3228428760],[3242466192,3242088876,3232658630],[3225324183,3223441038,1100750355]]);
let mut slice=CurveSlice::<Vec3,_,super::super::CurveCoordSemantic2D>::from_coord_range(p.as_slice(),c([0,1044882318]),c([3,1032113048])).remap_2d();
assert!(std::panic::catch_unwind(std::panic::AssertUnwindSafe(||{slice.extend_left(f(3212836864))})).is_err());
}
{let p=points(&[[3240405232,1058186416,1097096531],[1099817832,1092237199,3244041696],[3198902661,3238610697,3228428760],[3242466192,3242088876,3232658630],[3225324183,3223441038,1100750355]]);
let mut slice=CurveSlice::<Vec3,_,super::super::CurveCoordSemantic2D>::from_coord_range(p.as_slice(),c([3,1056357783]),c([3,1056357783])).remap_2d();
assert!(std::panic::catch_unwind(std::panic::AssertUnwindSafe(||{slice.extend_left(f(2143289344))})).is_err());
}
{let p=points(&[[1095869065,3247728450,3238144214],[3222142358,1069576697,1097745549],[3236692308,1082213236,1084051305],[3220909042,1095564094,1094016660],[1093158551,3217191098,1073815575],[1081707205,3248005126,3225940329],[1097165234,1095995953,1087159011],[3221340487,3240047769,3246657917]]);
let mut slice=CurveSlice::<Vec3,_,super::super::CurveCoordSemantic2D>::from_coord_range(p.as_slice(),c([4,1061350961]),c([4,1061350961])).remap_2d();
slice.extend_left(f(0));eq_coord(slice.start(),[4,1061350961]);eq_coord(slice.end(),[4,1061350961]);
}
{let p=points(&[[1095869065,3247728450,3238144214],[3222142358,1069576697,1097745549],[3236692308,1082213236,1084051305],[3220909042,1095564094,1094016660],[1093158551,3217191098,1073815575],[1081707205,3248005126,3225940329],[1097165234,1095995953,1087159011],[3221340487,3240047769,3246657917]]);
let mut slice=CurveSlice::<Vec3,_,super::super::CurveCoordSemantic2D>::from_coord_range(p.as_slice(),c([2,1036122161]),c([4,1058412339])).remap_2d();
slice.extend_left(f(1008981770));eq_coord(slice.start(),[2,1035962322]);eq_coord(slice.end(),[4,1058412339]);
}
{let p=points(&[[1095869065,3247728450,3238144214],[3222142358,1069576697,1097745549],[3236692308,1082213236,1084051305],[3220909042,1095564094,1094016660],[1093158551,3217191098,1073815575],[1081707205,3248005126,3225940329],[1097165234,1095995953,1087159011],[3221340487,3240047769,3246657917]]);
let mut slice=CurveSlice::<Vec3,_,super::super::CurveCoordSemantic2D>::from_coord_range(p.as_slice(),c([2,1061649645]),c([6,1065166496])).remap_2d();
slice.extend_left(f(1077936128));eq_coord(slice.start(),[2,1054346809]);eq_coord(slice.end(),[6,1065166496]);
}
{let p=points(&[[1095869065,3247728450,3238144214],[3222142358,1069576697,1097745549],[3236692308,1082213236,1084051305],[3220909042,1095564094,1094016660],[1093158551,3217191098,1073815575],[1081707205,3248005126,3225940329],[1097165234,1095995953,1087159011],[3221340487,3240047769,3246657917]]);
let mut slice=CurveSlice::<Vec3,_,super::super::CurveCoordSemantic2D>::from_coord_range(p.as_slice(),c([1,1049184518]),c([3,1059957207])).remap_2d();
slice.extend_left(f(1120403456));eq_coord(slice.start(),[0,0]);eq_coord(slice.end(),[3,1059957207]);
}
{let p=points(&[[1095869065,3247728450,3238144214],[3222142358,1069576697,1097745549],[3236692308,1082213236,1084051305],[3220909042,1095564094,1094016660],[1093158551,3217191098,1073815575],[1081707205,3248005126,3225940329],[1097165234,1095995953,1087159011],[3221340487,3240047769,3246657917]]);
let mut slice=CurveSlice::<Vec3,_,super::super::CurveCoordSemantic2D>::from_coord_range(p.as_slice(),c([1,1045235421]),c([1,1049431887])).remap_2d();
slice.extend_left(f(2139095040));eq_coord(slice.start(),[0,0]);eq_coord(slice.end(),[1,1049431887]);
}
{let p=points(&[[1095869065,3247728450,3238144214],[3222142358,1069576697,1097745549],[3236692308,1082213236,1084051305],[3220909042,1095564094,1094016660],[1093158551,3217191098,1073815575],[1081707205,3248005126,3225940329],[1097165234,1095995953,1087159011],[3221340487,3240047769,3246657917]]);
let mut slice=CurveSlice::<Vec3,_,super::super::CurveCoordSemantic2D>::from_coord_range(p.as_slice(),c([0,1032333751]),c([4,1033099165])).remap_2d();
assert!(std::panic::catch_unwind(std::panic::AssertUnwindSafe(||{slice.extend_left(f(3212836864))})).is_err());
}
{let p=points(&[[1095869065,3247728450,3238144214],[3222142358,1069576697,1097745549],[3236692308,1082213236,1084051305],[3220909042,1095564094,1094016660],[1093158551,3217191098,1073815575],[1081707205,3248005126,3225940329],[1097165234,1095995953,1087159011],[3221340487,3240047769,3246657917]]);
let mut slice=CurveSlice::<Vec3,_,super::super::CurveCoordSemantic2D>::from_coord_range(p.as_slice(),c([5,1061177844]),c([6,1059809059])).remap_2d();
assert!(std::panic::catch_unwind(std::panic::AssertUnwindSafe(||{slice.extend_left(f(2143289344))})).is_err());
}
{let p=points(&[[3230740587,3248458583,1089714619],[3245563810,1095406261,3225768936]]);
let mut slice=CurveSlice::<Vec3,_,super::super::CurveCoordSemantic2D>::from_coord_range(p.as_slice(),c([0,1059599970]),c([0,1059599970])).remap_2d();
slice.extend_left(f(0));eq_coord(slice.start(),[0,1059599970]);eq_coord(slice.end(),[0,1059599970]);
}
{let p=points(&[[3230740587,3248458583,1089714619],[3245563810,1095406261,3225768936]]);
let mut slice=CurveSlice::<Vec3,_,super::super::CurveCoordSemantic2D>::from_coord_range(p.as_slice(),c([0,1035280956]),c([0,1035280956])).remap_2d();
slice.extend_left(f(1008981770));eq_coord(slice.start(),[0,1035192152]);eq_coord(slice.end(),[0,1035280956]);
}
{let p=points(&[[3230740587,3248458583,1089714619],[3245563810,1095406261,3225768936]]);
let mut slice=CurveSlice::<Vec3,_,super::super::CurveCoordSemantic2D>::from_coord_range(p.as_slice(),c([0,1035504764]),c([0,1053868221])).remap_2d();
slice.extend_left(f(1077936128));eq_coord(slice.start(),[0,0]);eq_coord(slice.end(),[0,1053868221]);
}
{let p=points(&[[3230740587,3248458583,1089714619],[3245563810,1095406261,3225768936]]);
let mut slice=CurveSlice::<Vec3,_,super::super::CurveCoordSemantic2D>::from_coord_range(p.as_slice(),c([0,1060636397]),c([0,1060636397])).remap_2d();
slice.extend_left(f(1120403456));eq_coord(slice.start(),[0,0]);eq_coord(slice.end(),[0,1060636397]);
}
{let p=points(&[[3230740587,3248458583,1089714619],[3245563810,1095406261,3225768936]]);
let mut slice=CurveSlice::<Vec3,_,super::super::CurveCoordSemantic2D>::from_coord_range(p.as_slice(),c([0,1064243484]),c([0,1064243484])).remap_2d();
slice.extend_left(f(2139095040));eq_coord(slice.start(),[0,0]);eq_coord(slice.end(),[0,1064243484]);
}
{let p=points(&[[3230740587,3248458583,1089714619],[3245563810,1095406261,3225768936]]);
let mut slice=CurveSlice::<Vec3,_,super::super::CurveCoordSemantic2D>::from_coord_range(p.as_slice(),c([0,1051955764]),c([0,1052571764])).remap_2d();
assert!(std::panic::catch_unwind(std::panic::AssertUnwindSafe(||{slice.extend_left(f(3212836864))})).is_err());
}
{let p=points(&[[3230740587,3248458583,1089714619],[3245563810,1095406261,3225768936]]);
let mut slice=CurveSlice::<Vec3,_,super::super::CurveCoordSemantic2D>::from_coord_range(p.as_slice(),c([0,1038493279]),c([0,1059105558])).remap_2d();
assert!(std::panic::catch_unwind(std::panic::AssertUnwindSafe(||{slice.extend_left(f(2143289344))})).is_err());
}
{let p=points(&[[1100597656,3246161973,3229240955],[1100375588,1071835982,1065429120],[3247206345,3227164320,3209281283],[1072673282,1089449914,1074183584]]);
let mut slice=CurveSlice::<Vec3,_,super::super::CurveCoordSemantic2D>::from_coord_range(p.as_slice(),c([0,1050287263]),c([0,1060759990])).remap_2d();
slice.extend_left(f(0));eq_coord(slice.start(),[0,1050287264]);eq_coord(slice.end(),[0,1060759990]);
}
{let p=points(&[[1100597656,3246161973,3229240955],[1100375588,1071835982,1065429120],[3247206345,3227164320,3209281283],[1072673282,1089449914,1074183584]]);
let mut slice=CurveSlice::<Vec3,_,super::super::CurveCoordSemantic2D>::from_coord_range(p.as_slice(),c([1,1054270133]),c([1,1059380480])).remap_2d();
slice.extend_left(f(1008981770));eq_coord(slice.start(),[1,1054260913]);eq_coord(slice.end(),[1,1059380480]);
}
{let p=points(&[[1100597656,3246161973,3229240955],[1100375588,1071835982,1065429120],[3247206345,3227164320,3209281283],[1072673282,1089449914,1074183584]]);
let mut slice=CurveSlice::<Vec3,_,super::super::CurveCoordSemantic2D>::from_coord_range(p.as_slice(),c([0,1045232529]),c([1,1061892227])).remap_2d();
slice.extend_left(f(1077936128));eq_coord(slice.start(),[0,0]);eq_coord(slice.end(),[1,1061892227]);
}
{let p=points(&[[1100597656,3246161973,3229240955],[1100375588,1071835982,1065429120],[3247206345,3227164320,3209281283],[1072673282,1089449914,1074183584]]);
let mut slice=CurveSlice::<Vec3,_,super::super::CurveCoordSemantic2D>::from_coord_range(p.as_slice(),c([0,1062748774]),c([2,1051945663])).remap_2d();
slice.extend_left(f(1120403456));eq_coord(slice.start(),[0,0]);eq_coord(slice.end(),[2,1051945663]);
}
{let p=points(&[[1100597656,3246161973,3229240955],[1100375588,1071835982,1065429120],[3247206345,3227164320,3209281283],[1072673282,1089449914,1074183584]]);
let mut slice=CurveSlice::<Vec3,_,super::super::CurveCoordSemantic2D>::from_coord_range(p.as_slice(),c([1,1030665781]),c([1,1047297507])).remap_2d();
slice.extend_left(f(2139095040));eq_coord(slice.start(),[0,0]);eq_coord(slice.end(),[1,1047297507]);
}
{let p=points(&[[1100597656,3246161973,3229240955],[1100375588,1071835982,1065429120],[3247206345,3227164320,3209281283],[1072673282,1089449914,1074183584]]);
let mut slice=CurveSlice::<Vec3,_,super::super::CurveCoordSemantic2D>::from_coord_range(p.as_slice(),c([0,1064517742]),c([1,1059371101])).remap_2d();
assert!(std::panic::catch_unwind(std::panic::AssertUnwindSafe(||{slice.extend_left(f(3212836864))})).is_err());
}
{let p=points(&[[1100597656,3246161973,3229240955],[1100375588,1071835982,1065429120],[3247206345,3227164320,3209281283],[1072673282,1089449914,1074183584]]);
let mut slice=CurveSlice::<Vec3,_,super::super::CurveCoordSemantic2D>::from_coord_range(p.as_slice(),c([0,1054796700]),c([2,1032454111])).remap_2d();
assert!(std::panic::catch_unwind(std::panic::AssertUnwindSafe(||{slice.extend_left(f(2143289344))})).is_err());
}
{let p=points(&[[3246510138,3241659323,3237462629],[3221978396,3247458044,3246701192],[3244357164,1095015870,3244035361],[1099583006,3244088859,1097357154],[1084963187,3184481463,3245721065]]);
let mut slice=CurveSlice::<Vec3,_,super::super::CurveCoordSemantic2D>::from_coord_range(p.as_slice(),c([2,1057826807]),c([2,1058469297])).remap_2d();
slice.extend_left(f(0));eq_coord(slice.start(),[2,1057826807]);eq_coord(slice.end(),[2,1058469297]);
}
{let p=points(&[[3246510138,3241659323,3237462629],[3221978396,3247458044,3246701192],[3244357164,1095015870,3244035361],[1099583006,3244088859,1097357154],[1084963187,3184481463,3245721065]]);
let mut slice=CurveSlice::<Vec3,_,super::super::CurveCoordSemantic2D>::from_coord_range(p.as_slice(),c([1,1061991151]),c([2,1049547262])).remap_2d();
slice.extend_left(f(1008981770));eq_coord(slice.start(),[1,1061977415]);eq_coord(slice.end(),[2,1049547262]);
}
{let p=points(&[[3246510138,3241659323,3237462629],[3221978396,3247458044,3246701192],[3244357164,1095015870,3244035361],[1099583006,3244088859,1097357154],[1084963187,3184481463,3245721065]]);
let mut slice=CurveSlice::<Vec3,_,super::super::CurveCoordSemantic2D>::from_coord_range(p.as_slice(),c([0,1009832828]),c([1,1045845780])).remap_2d();
slice.extend_left(f(1077936128));eq_coord(slice.start(),[0,0]);eq_coord(slice.end(),[1,1045845780]);
}
{let p=points(&[[3246510138,3241659323,3237462629],[3221978396,3247458044,3246701192],[3244357164,1095015870,3244035361],[1099583006,3244088859,1097357154],[1084963187,3184481463,3245721065]]);
let mut slice=CurveSlice::<Vec3,_,super::super::CurveCoordSemantic2D>::from_coord_range(p.as_slice(),c([0,1058490899]),c([3,1050771189])).remap_2d();
slice.extend_left(f(1120403456));eq_coord(slice.start(),[0,0]);eq_coord(slice.end(),[3,1050771189]);
}
{let p=points(&[[3246510138,3241659323,3237462629],[3221978396,3247458044,3246701192],[3244357164,1095015870,3244035361],[1099583006,3244088859,1097357154],[1084963187,3184481463,3245721065]]);
let mut slice=CurveSlice::<Vec3,_,super::super::CurveCoordSemantic2D>::from_coord_range(p.as_slice(),c([1,1059193987]),c([2,1056752940])).remap_2d();
slice.extend_left(f(2139095040));eq_coord(slice.start(),[0,0]);eq_coord(slice.end(),[2,1056752940]);
}
{let p=points(&[[3246510138,3241659323,3237462629],[3221978396,3247458044,3246701192],[3244357164,1095015870,3244035361],[1099583006,3244088859,1097357154],[1084963187,3184481463,3245721065]]);
let mut slice=CurveSlice::<Vec3,_,super::super::CurveCoordSemantic2D>::from_coord_range(p.as_slice(),c([2,1050486173]),c([2,1050486173])).remap_2d();
assert!(std::panic::catch_unwind(std::panic::AssertUnwindSafe(||{slice.extend_left(f(3212836864))})).is_err());
}
{let p=points(&[[3246510138,3241659323,3237462629],[3221978396,3247458044,3246701192],[3244357164,1095015870,3244035361],[1099583006,3244088859,1097357154],[1084963187,3184481463,3245721065]]);
let mut slice=CurveSlice::<Vec3,_,super::super::CurveCoordSemantic2D>::from_coord_range(p.as_slice(),c([2,1052099016]),c([3,1055444048])).remap_2d();
assert!(std::panic::catch_unwind(std::panic::AssertUnwindSafe(||{slice.extend_left(f(2143289344))})).is_err());
}
{let p=points(&[[1084408924,3239877357,3242505151],[1099195345,3236083368,1047330589],[1086185979,3244311131,1094930848],[3241436293,1099889881,3232199892]]);
let mut slice=CurveSlice::<Vec3,_,super::super::CurveCoordSemantic2D>::from_coord_range(p.as_slice(),c([2,1063418695]),c([2,1063418695])).remap_2d();
slice.extend_left(f(0));eq_coord(slice.start(),[2,1063418695]);eq_coord(slice.end(),[2,1063418695]);
}
{let p=points(&[[1084408924,3239877357,3242505151],[1099195345,3236083368,1047330589],[1086185979,3244311131,1094930848],[3241436293,1099889881,3232199892]]);
let mut slice=CurveSlice::<Vec3,_,super::super::CurveCoordSemantic2D>::from_coord_range(p.as_slice(),c([0,1057788921]),c([1,1047027876])).remap_2d();
slice.extend_left(f(1008981770));eq_coord(slice.start(),[0,1057779040]);eq_coord(slice.end(),[1,1047027876]);
}
{let p=points(&[[1084408924,3239877357,3242505151],[1099195345,3236083368,1047330589],[1086185979,3244311131,1094930848],[3241436293,1099889881,3232199892]]);
let mut slice=CurveSlice::<Vec3,_,super::super::CurveCoordSemantic2D>::from_coord_range(p.as_slice(),c([0,1057485061]),c([0,1057485061])).remap_2d();
slice.extend_left(f(1077936128));eq_coord(slice.start(),[0,1052076624]);eq_coord(slice.end(),[0,1057485061]);
}
{let p=points(&[[1084408924,3239877357,3242505151],[1099195345,3236083368,1047330589],[1086185979,3244311131,1094930848],[3241436293,1099889881,3232199892]]);
let mut slice=CurveSlice::<Vec3,_,super::super::CurveCoordSemantic2D>::from_coord_range(p.as_slice(),c([0,1059976172]),c([1,1049290850])).remap_2d();
slice.extend_left(f(1120403456));eq_coord(slice.start(),[0,0]);eq_coord(slice.end(),[1,1049290850]);
}
{let p=points(&[[1084408924,3239877357,3242505151],[1099195345,3236083368,1047330589],[1086185979,3244311131,1094930848],[3241436293,1099889881,3232199892]]);
let mut slice=CurveSlice::<Vec3,_,super::super::CurveCoordSemantic2D>::from_coord_range(p.as_slice(),c([1,1054866578]),c([2,1050468423])).remap_2d();
slice.extend_left(f(2139095040));eq_coord(slice.start(),[0,0]);eq_coord(slice.end(),[2,1050468423]);
}
{let p=points(&[[1084408924,3239877357,3242505151],[1099195345,3236083368,1047330589],[1086185979,3244311131,1094930848],[3241436293,1099889881,3232199892]]);
let mut slice=CurveSlice::<Vec3,_,super::super::CurveCoordSemantic2D>::from_coord_range(p.as_slice(),c([2,1061175037]),c([2,1061175037])).remap_2d();
assert!(std::panic::catch_unwind(std::panic::AssertUnwindSafe(||{slice.extend_left(f(3212836864))})).is_err());
}
{let p=points(&[[1084408924,3239877357,3242505151],[1099195345,3236083368,1047330589],[1086185979,3244311131,1094930848],[3241436293,1099889881,3232199892]]);
let mut slice=CurveSlice::<Vec3,_,super::super::CurveCoordSemantic2D>::from_coord_range(p.as_slice(),c([0,1050565802]),c([1,1059664047])).remap_2d();
assert!(std::panic::catch_unwind(std::panic::AssertUnwindSafe(||{slice.extend_left(f(2143289344))})).is_err());
}
{let p=points(&[[3244253566,3238545491,1083992339],[3224608338,1093633606,1093754342]]);
let mut slice=CurveSlice::<Vec3,_,super::super::CurveCoordSemantic2D>::from_coord_range(p.as_slice(),c([0,1064922604]),c([0,1064922604])).remap_2d();
slice.extend_left(f(0));eq_coord(slice.start(),[0,1064922604]);eq_coord(slice.end(),[0,1064922604]);
}
{let p=points(&[[3244253566,3238545491,1083992339],[3224608338,1093633606,1093754342]]);
let mut slice=CurveSlice::<Vec3,_,super::super::CurveCoordSemantic2D>::from_coord_range(p.as_slice(),c([0,1055868240]),c([0,1055868240])).remap_2d();
slice.extend_left(f(1008981770));eq_coord(slice.start(),[0,1055841944]);eq_coord(slice.end(),[0,1055868240]);
}
{let p=points(&[[3244253566,3238545491,1083992339],[3224608338,1093633606,1093754342]]);
let mut slice=CurveSlice::<Vec3,_,super::super::CurveCoordSemantic2D>::from_coord_range(p.as_slice(),c([0,1040187294]),c([0,1058787079])).remap_2d();
slice.extend_left(f(1077936128));eq_coord(slice.start(),[0,0]);eq_coord(slice.end(),[0,1058787079]);
}
{let p=points(&[[3244253566,3238545491,1083992339],[3224608338,1093633606,1093754342]]);
let mut slice=CurveSlice::<Vec3,_,super::super::CurveCoordSemantic2D>::from_coord_range(p.as_slice(),c([0,1037221748]),c([0,1040490373])).remap_2d();
slice.extend_left(f(1120403456));eq_coord(slice.start(),[0,0]);eq_coord(slice.end(),[0,1040490373]);
}
{let p=points(&[[3244253566,3238545491,1083992339],[3224608338,1093633606,1093754342]]);
let mut slice=CurveSlice::<Vec3,_,super::super::CurveCoordSemantic2D>::from_coord_range(p.as_slice(),c([0,1050153298]),c([0,1061398614])).remap_2d();
slice.extend_left(f(2139095040));eq_coord(slice.start(),[0,0]);eq_coord(slice.end(),[0,1061398614]);
}
{let p=points(&[[3244253566,3238545491,1083992339],[3224608338,1093633606,1093754342]]);
let mut slice=CurveSlice::<Vec3,_,super::super::CurveCoordSemantic2D>::from_coord_range(p.as_slice(),c([0,1054931440]),c([0,1063984310])).remap_2d();
assert!(std::panic::catch_unwind(std::panic::AssertUnwindSafe(||{slice.extend_left(f(3212836864))})).is_err());
}
{let p=points(&[[3244253566,3238545491,1083992339],[3224608338,1093633606,1093754342]]);
let mut slice=CurveSlice::<Vec3,_,super::super::CurveCoordSemantic2D>::from_coord_range(p.as_slice(),c([0,1015259496]),c([0,1031915753])).remap_2d();
assert!(std::panic::catch_unwind(std::panic::AssertUnwindSafe(||{slice.extend_left(f(2143289344))})).is_err());
}
{let p=points(&[[3237489998,3248309204,1088059251],[1093283022,1096235402,3239656559],[3236412957,3247351658,1097842762],[3239470823,3231809706,1095304493],[1094902171,1065889429,3246283044],[3223199735,1095789934,1094959419]]);
let mut slice=CurveSlice::<Vec3,_,super::super::CurveCoordSemantic2D>::from_coord_range(p.as_slice(),c([1,1060046079]),c([2,1051277295])).remap_2d();
slice.extend_left(f(0));eq_coord(slice.start(),[1,1060046079]);eq_coord(slice.end(),[2,1051277295]);
}
{let p=points(&[[3237489998,3248309204,1088059251],[1093283022,1096235402,3239656559],[3236412957,3247351658,1097842762],[3239470823,3231809706,1095304493],[1094902171,1065889429,3246283044],[3223199735,1095789934,1094959419]]);
let mut slice=CurveSlice::<Vec3,_,super::super::CurveCoordSemantic2D>::from_coord_range(p.as_slice(),c([4,1050287610]),c([4,1056829695])).remap_2d();
slice.extend_left(f(1008981770));eq_coord(slice.start(),[4,1050277030]);eq_coord(slice.end(),[4,1056829695]);
}
{let p=points(&[[3237489998,3248309204,1088059251],[1093283022,1096235402,3239656559],[3236412957,3247351658,1097842762],[3239470823,3231809706,1095304493],[1094902171,1065889429,3246283044],[3223199735,1095789934,1094959419]]);
let mut slice=CurveSlice::<Vec3,_,super::super::CurveCoordSemantic2D>::from_coord_range(p.as_slice(),c([0,1052707623]),c([3,1061844320])).remap_2d();
slice.extend_left(f(1077936128));eq_coord(slice.start(),[0,1048622998]);eq_coord(slice.end(),[3,1061844320]);
}
{let p=points(&[[3237489998,3248309204,1088059251],[1093283022,1096235402,3239656559],[3236412957,3247351658,1097842762],[3239470823,3231809706,1095304493],[1094902171,1065889429,3246283044],[3223199735,1095789934,1094959419]]);
let mut slice=CurveSlice::<Vec3,_,super::super::CurveCoordSemantic2D>::from_coord_range(p.as_slice(),c([2,1063403561]),c([3,1052082102])).remap_2d();
slice.extend_left(f(1120403456));eq_coord(slice.start(),[0,0]);eq_coord(slice.end(),[3,1052082102]);
}
{let p=points(&[[3237489998,3248309204,1088059251],[1093283022,1096235402,3239656559],[3236412957,3247351658,1097842762],[3239470823,3231809706,1095304493],[1094902171,1065889429,3246283044],[3223199735,1095789934,1094959419]]);
let mut slice=CurveSlice::<Vec3,_,super::super::CurveCoordSemantic2D>::from_coord_range(p.as_slice(),c([3,1056613214]),c([3,1056613214])).remap_2d();
slice.extend_left(f(2139095040));eq_coord(slice.start(),[0,0]);eq_coord(slice.end(),[3,1056613214]);
}
{let p=points(&[[3237489998,3248309204,1088059251],[1093283022,1096235402,3239656559],[3236412957,3247351658,1097842762],[3239470823,3231809706,1095304493],[1094902171,1065889429,3246283044],[3223199735,1095789934,1094959419]]);
let mut slice=CurveSlice::<Vec3,_,super::super::CurveCoordSemantic2D>::from_coord_range(p.as_slice(),c([1,1037355347]),c([4,1064017200])).remap_2d();
assert!(std::panic::catch_unwind(std::panic::AssertUnwindSafe(||{slice.extend_left(f(3212836864))})).is_err());
}
{let p=points(&[[3237489998,3248309204,1088059251],[1093283022,1096235402,3239656559],[3236412957,3247351658,1097842762],[3239470823,3231809706,1095304493],[1094902171,1065889429,3246283044],[3223199735,1095789934,1094959419]]);
let mut slice=CurveSlice::<Vec3,_,super::super::CurveCoordSemantic2D>::from_coord_range(p.as_slice(),c([3,1058906985]),c([4,1059041885])).remap_2d();
assert!(std::panic::catch_unwind(std::panic::AssertUnwindSafe(||{slice.extend_left(f(2143289344))})).is_err());
}
{let p=points(&[[3236778374,3247171092,1100137156],[3229026171,3245992429,1098887181],[3247277947,3239205250,1070240050],[3230812912,3241022183,3237516106],[3238044015,1094711426,1097253160],[1090553702,1099196477,1093576738],[3243024163,1100515465,1098886831]]);
let mut slice=CurveSlice::<Vec3,_,super::super::CurveCoordSemantic2D>::from_coord_range(p.as_slice(),c([1,1059436846]),c([2,1040329189])).remap_2d();
slice.extend_left(f(0));eq_coord(slice.start(),[1,1059436846]);eq_coord(slice.end(),[2,1040329189]);
}
{let p=points(&[[3236778374,3247171092,1100137156],[3229026171,3245992429,1098887181],[3247277947,3239205250,1070240050],[3230812912,3241022183,3237516106],[3238044015,1094711426,1097253160],[1090553702,1099196477,1093576738],[3243024163,1100515465,1098886831]]);
let mut slice=CurveSlice::<Vec3,_,super::super::CurveCoordSemantic2D>::from_coord_range(p.as_slice(),c([1,1060428285]),c([5,1046290428])).remap_2d();
slice.extend_left(f(1008981770));eq_coord(slice.start(),[1,1060419882]);eq_coord(slice.end(),[5,1046290428]);
}
{let p=points(&[[3236778374,3247171092,1100137156],[3229026171,3245992429,1098887181],[3247277947,3239205250,1070240050],[3230812912,3241022183,3237516106],[3238044015,1094711426,1097253160],[1090553702,1099196477,1093576738],[3243024163,1100515465,1098886831]]);
let mut slice=CurveSlice::<Vec3,_,super::super::CurveCoordSemantic2D>::from_coord_range(p.as_slice(),c([1,1056129128]),c([3,1058865254])).remap_2d();
slice.extend_left(f(1077936128));eq_coord(slice.start(),[1,1051087168]);eq_coord(slice.end(),[3,1058865254]);
}
{let p=points(&[[3236778374,3247171092,1100137156],[3229026171,3245992429,1098887181],[3247277947,3239205250,1070240050],[3230812912,3241022183,3237516106],[3238044015,1094711426,1097253160],[1090553702,1099196477,1093576738],[3243024163,1100515465,1098886831]]);
let mut slice=CurveSlice::<Vec3,_,super::super::CurveCoordSemantic2D>::from_coord_range(p.as_slice(),c([3,1040615416]),c([5,1063590178])).remap_2d();
slice.extend_left(f(1120403456));eq_coord(slice.start(),[0,0]);eq_coord(slice.end(),[5,1063590178]);
}
{let p=points(&[[3236778374,3247171092,1100137156],[3229026171,3245992429,1098887181],[3247277947,3239205250,1070240050],[3230812912,3241022183,3237516106],[3238044015,1094711426,1097253160],[1090553702,1099196477,1093576738],[3243024163,1100515465,1098886831]]);
let mut slice=CurveSlice::<Vec3,_,super::super::CurveCoordSemantic2D>::from_coord_range(p.as_slice(),c([2,1061162577]),c([2,1061162577])).remap_2d();
slice.extend_left(f(2139095040));eq_coord(slice.start(),[0,0]);eq_coord(slice.end(),[2,1061162577]);
}
{let p=points(&[[3236778374,3247171092,1100137156],[3229026171,3245992429,1098887181],[3247277947,3239205250,1070240050],[3230812912,3241022183,3237516106],[3238044015,1094711426,1097253160],[1090553702,1099196477,1093576738],[3243024163,1100515465,1098886831]]);
let mut slice=CurveSlice::<Vec3,_,super::super::CurveCoordSemantic2D>::from_coord_range(p.as_slice(),c([3,1063908686]),c([5,1060165072])).remap_2d();
assert!(std::panic::catch_unwind(std::panic::AssertUnwindSafe(||{slice.extend_left(f(3212836864))})).is_err());
}
{let p=points(&[[3236778374,3247171092,1100137156],[3229026171,3245992429,1098887181],[3247277947,3239205250,1070240050],[3230812912,3241022183,3237516106],[3238044015,1094711426,1097253160],[1090553702,1099196477,1093576738],[3243024163,1100515465,1098886831]]);
let mut slice=CurveSlice::<Vec3,_,super::super::CurveCoordSemantic2D>::from_coord_range(p.as_slice(),c([1,1040614404]),c([4,1064793064])).remap_2d();
assert!(std::panic::catch_unwind(std::panic::AssertUnwindSafe(||{slice.extend_left(f(2143289344))})).is_err());
}
{let p=points(&[[3241172539,3247851269,1098748307],[3247563556,1090964172,1096258752],[3248392708,1089245329,1099693570],[3237199233,1064552993,1100652604],[3240910505,1094699492,3237742908]]);
let mut slice=CurveSlice::<Vec3,_,super::super::CurveCoordSemantic2D>::from_coord_range(p.as_slice(),c([1,1054049965]),c([2,1065339404])).remap_2d();
slice.extend_left(f(0));eq_coord(slice.start(),[1,1054049964]);eq_coord(slice.end(),[2,1065339404]);
}
{let p=points(&[[3241172539,3247851269,1098748307],[3247563556,1090964172,1096258752],[3248392708,1089245329,1099693570],[3237199233,1064552993,1100652604],[3240910505,1094699492,3237742908]]);
let mut slice=CurveSlice::<Vec3,_,super::super::CurveCoordSemantic2D>::from_coord_range(p.as_slice(),c([2,985585066]),c([3,1052233957])).remap_2d();
slice.extend_left(f(1008981770));eq_coord(slice.start(),[2,975777543]);eq_coord(slice.end(),[3,1052233957]);
}
{let p=points(&[[3241172539,3247851269,1098748307],[3247563556,1090964172,1096258752],[3248392708,1089245329,1099693570],[3237199233,1064552993,1100652604],[3240910505,1094699492,3237742908]]);
let mut slice=CurveSlice::<Vec3,_,super::super::CurveCoordSemantic2D>::from_coord_range(p.as_slice(),c([0,1062506734]),c([3,1061184005])).remap_2d();
slice.extend_left(f(1077936128));eq_coord(slice.start(),[0,1054792545]);eq_coord(slice.end(),[3,1061184005]);
}
{let p=points(&[[3241172539,3247851269,1098748307],[3247563556,1090964172,1096258752],[3248392708,1089245329,1099693570],[3237199233,1064552993,1100652604],[3240910505,1094699492,3237742908]]);
let mut slice=CurveSlice::<Vec3,_,super::super::CurveCoordSemantic2D>::from_coord_range(p.as_slice(),c([1,1046470788]),c([3,1039428847])).remap_2d();
slice.extend_left(f(1120403456));eq_coord(slice.start(),[0,0]);eq_coord(slice.end(),[3,1039428847]);
}
{let p=points(&[[3241172539,3247851269,1098748307],[3247563556,1090964172,1096258752],[3248392708,1089245329,1099693570],[3237199233,1064552993,1100652604],[3240910505,1094699492,3237742908]]);
let mut slice=CurveSlice::<Vec3,_,super::super::CurveCoordSemantic2D>::from_coord_range(p.as_slice(),c([2,1051467963]),c([2,1051467963])).remap_2d();
slice.extend_left(f(2139095040));eq_coord(slice.start(),[0,0]);eq_coord(slice.end(),[2,1051467963]);
}
{let p=points(&[[3241172539,3247851269,1098748307],[3247563556,1090964172,1096258752],[3248392708,1089245329,1099693570],[3237199233,1064552993,1100652604],[3240910505,1094699492,3237742908]]);
let mut slice=CurveSlice::<Vec3,_,super::super::CurveCoordSemantic2D>::from_coord_range(p.as_slice(),c([2,1056497158]),c([2,1062676617])).remap_2d();
assert!(std::panic::catch_unwind(std::panic::AssertUnwindSafe(||{slice.extend_left(f(3212836864))})).is_err());
}
{let p=points(&[[3241172539,3247851269,1098748307],[3247563556,1090964172,1096258752],[3248392708,1089245329,1099693570],[3237199233,1064552993,1100652604],[3240910505,1094699492,3237742908]]);
let mut slice=CurveSlice::<Vec3,_,super::super::CurveCoordSemantic2D>::from_coord_range(p.as_slice(),c([0,1057942222]),c([2,1057472868])).remap_2d();
assert!(std::panic::catch_unwind(std::panic::AssertUnwindSafe(||{slice.extend_left(f(2143289344))})).is_err());
}
{let p=points(&[[3235510957,3225421292,3243546556],[1075856735,1073987888,1100993602],[3246718811,3238565377,1076541785],[1095096324,3234524412,3248202371],[1083180013,1091966328,3245405560],[1095968577,1089057561,3245848733],[1098572274,3247422157,3243298987],[1072012514,1100993691,1065988040]]);
let mut slice=CurveSlice::<Vec3,_,super::super::CurveCoordSemantic2D>::from_coord_range(p.as_slice(),c([2,1042477620]),c([5,1055269418])).remap_2d();
slice.extend_left(f(0));eq_coord(slice.start(),[2,1042477620]);eq_coord(slice.end(),[5,1055269418]);
}
{let p=points(&[[3235510957,3225421292,3243546556],[1075856735,1073987888,1100993602],[3246718811,3238565377,1076541785],[1095096324,3234524412,3248202371],[1083180013,1091966328,3245405560],[1095968577,1089057561,3245848733],[1098572274,3247422157,3243298987],[1072012514,1100993691,1065988040]]);
let mut slice=CurveSlice::<Vec3,_,super::super::CurveCoordSemantic2D>::from_coord_range(p.as_slice(),c([2,1047820646]),c([5,1061991205])).remap_2d();
slice.extend_left(f(1008981770));eq_coord(slice.start(),[2,1047802244]);eq_coord(slice.end(),[5,1061991205]);
}
{let p=points(&[[3235510957,3225421292,3243546556],[1075856735,1073987888,1100993602],[3246718811,3238565377,1076541785],[1095096324,3234524412,3248202371],[1083180013,1091966328,3245405560],[1095968577,1089057561,3245848733],[1098572274,3247422157,3243298987],[1072012514,1100993691,1065988040]]);
let mut slice=CurveSlice::<Vec3,_,super::super::CurveCoordSemantic2D>::from_coord_range(p.as_slice(),c([0,1057107027]),c([4,1042546317])).remap_2d();
slice.extend_left(f(1077936128));eq_coord(slice.start(),[0,1054335517]);eq_coord(slice.end(),[4,1042546317]);
}
{let p=points(&[[3235510957,3225421292,3243546556],[1075856735,1073987888,1100993602],[3246718811,3238565377,1076541785],[1095096324,3234524412,3248202371],[1083180013,1091966328,3245405560],[1095968577,1089057561,3245848733],[1098572274,3247422157,3243298987],[1072012514,1100993691,1065988040]]);
let mut slice=CurveSlice::<Vec3,_,super::super::CurveCoordSemantic2D>::from_coord_range(p.as_slice(),c([4,1060572671]),c([4,1060572671])).remap_2d();
slice.extend_left(f(1120403456));eq_coord(slice.start(),[0,1051887797]);eq_coord(slice.end(),[4,1060572671]);
}
{let p=points(&[[3235510957,3225421292,3243546556],[1075856735,1073987888,1100993602],[3246718811,3238565377,1076541785],[1095096324,3234524412,3248202371],[1083180013,1091966328,3245405560],[1095968577,1089057561,3245848733],[1098572274,3247422157,3243298987],[1072012514,1100993691,1065988040]]);
let mut slice=CurveSlice::<Vec3,_,super::super::CurveCoordSemantic2D>::from_coord_range(p.as_slice(),c([2,1064806353]),c([5,1019261269])).remap_2d();
slice.extend_left(f(2139095040));eq_coord(slice.start(),[0,0]);eq_coord(slice.end(),[5,1019261269]);
}
{let p=points(&[[3235510957,3225421292,3243546556],[1075856735,1073987888,1100993602],[3246718811,3238565377,1076541785],[1095096324,3234524412,3248202371],[1083180013,1091966328,3245405560],[1095968577,1089057561,3245848733],[1098572274,3247422157,3243298987],[1072012514,1100993691,1065988040]]);
let mut slice=CurveSlice::<Vec3,_,super::super::CurveCoordSemantic2D>::from_coord_range(p.as_slice(),c([0,1055901053]),c([6,1053797497])).remap_2d();
assert!(std::panic::catch_unwind(std::panic::AssertUnwindSafe(||{slice.extend_left(f(3212836864))})).is_err());
}
{let p=points(&[[3235510957,3225421292,3243546556],[1075856735,1073987888,1100993602],[3246718811,3238565377,1076541785],[1095096324,3234524412,3248202371],[1083180013,1091966328,3245405560],[1095968577,1089057561,3245848733],[1098572274,3247422157,3243298987],[1072012514,1100993691,1065988040]]);
let mut slice=CurveSlice::<Vec3,_,super::super::CurveCoordSemantic2D>::from_coord_range(p.as_slice(),c([6,1059415688]),c([6,1062395518])).remap_2d();
assert!(std::panic::catch_unwind(std::panic::AssertUnwindSafe(||{slice.extend_left(f(2143289344))})).is_err());
}
{let p=points(&[[3248143559,3242855462,3217433213],[1095768306,3206348635,3239179310],[1089358524,1085122942,1089088795]]);
let mut slice=CurveSlice::<Vec3,_,super::super::CurveCoordSemantic2D>::from_coord_range(p.as_slice(),c([0,1059075362]),c([0,1059075362])).remap_2d();
slice.extend_left(f(0));eq_coord(slice.start(),[0,1059075362]);eq_coord(slice.end(),[0,1059075362]);
}
{let p=points(&[[3248143559,3242855462,3217433213],[1095768306,3206348635,3239179310],[1089358524,1085122942,1089088795]]);
let mut slice=CurveSlice::<Vec3,_,super::super::CurveCoordSemantic2D>::from_coord_range(p.as_slice(),c([1,1057653017]),c([1,1057653017])).remap_2d();
slice.extend_left(f(1008981770));eq_coord(slice.start(),[1,1057643349]);eq_coord(slice.end(),[1,1057653017]);
}
{let p=points(&[[3248143559,3242855462,3217433213],[1095768306,3206348635,3239179310],[1089358524,1085122942,1089088795]]);
let mut slice=CurveSlice::<Vec3,_,super::super::CurveCoordSemantic2D>::from_coord_range(p.as_slice(),c([0,1061955211]),c([0,1061955211])).remap_2d();
slice.extend_left(f(1077936128));eq_coord(slice.start(),[0,1060440254]);eq_coord(slice.end(),[0,1061955211]);
}
{let p=points(&[[3248143559,3242855462,3217433213],[1095768306,3206348635,3239179310],[1089358524,1085122942,1089088795]]);
let mut slice=CurveSlice::<Vec3,_,super::super::CurveCoordSemantic2D>::from_coord_range(p.as_slice(),c([0,1049278166]),c([1,1046177255])).remap_2d();
slice.extend_left(f(1120403456));eq_coord(slice.start(),[0,0]);eq_coord(slice.end(),[1,1046177255]);
}
{let p=points(&[[3248143559,3242855462,3217433213],[1095768306,3206348635,3239179310],[1089358524,1085122942,1089088795]]);
let mut slice=CurveSlice::<Vec3,_,super::super::CurveCoordSemantic2D>::from_coord_range(p.as_slice(),c([0,1060046221]),c([1,1054037164])).remap_2d();
slice.extend_left(f(2139095040));eq_coord(slice.start(),[0,0]);eq_coord(slice.end(),[1,1054037164]);
}
{let p=points(&[[3248143559,3242855462,3217433213],[1095768306,3206348635,3239179310],[1089358524,1085122942,1089088795]]);
let mut slice=CurveSlice::<Vec3,_,super::super::CurveCoordSemantic2D>::from_coord_range(p.as_slice(),c([1,1060596512]),c([1,1060596512])).remap_2d();
assert!(std::panic::catch_unwind(std::panic::AssertUnwindSafe(||{slice.extend_left(f(3212836864))})).is_err());
}
{let p=points(&[[3248143559,3242855462,3217433213],[1095768306,3206348635,3239179310],[1089358524,1085122942,1089088795]]);
let mut slice=CurveSlice::<Vec3,_,super::super::CurveCoordSemantic2D>::from_coord_range(p.as_slice(),c([1,1051627775]),c([1,1051627775])).remap_2d();
assert!(std::panic::catch_unwind(std::panic::AssertUnwindSafe(||{slice.extend_left(f(2143289344))})).is_err());
}
{let p=points(&[[3246861589,3236099130,1096230032],[1089722329,1100558226,1097214363],[3233710308,1093554716,1091714934]]);
let mut slice=CurveSlice::<Vec3,_,super::super::CurveCoordSemantic2D>::from_coord_range(p.as_slice(),c([0,1054889597]),c([1,1032910446])).remap_2d();
slice.extend_left(f(0));eq_coord(slice.start(),[0,1054889595]);eq_coord(slice.end(),[1,1032910446]);
}
{let p=points(&[[3246861589,3236099130,1096230032],[1089722329,1100558226,1097214363],[3233710308,1093554716,1091714934]]);
let mut slice=CurveSlice::<Vec3,_,super::super::CurveCoordSemantic2D>::from_coord_range(p.as_slice(),c([0,1056612366]),c([0,1056612366])).remap_2d();
slice.extend_left(f(1008981770));eq_coord(slice.start(),[0,1056598690]);eq_coord(slice.end(),[0,1056612366]);
}
{let p=points(&[[3246861589,3236099130,1096230032],[1089722329,1100558226,1097214363],[3233710308,1093554716,1091714934]]);
let mut slice=CurveSlice::<Vec3,_,super::super::CurveCoordSemantic2D>::from_coord_range(p.as_slice(),c([1,1061810046]),c([1,1061810046])).remap_2d();
slice.extend_left(f(1077936128));eq_coord(slice.start(),[1,1058351147]);eq_coord(slice.end(),[1,1061810046]);
}
{let p=points(&[[3246861589,3236099130,1096230032],[1089722329,1100558226,1097214363],[3233710308,1093554716,1091714934]]);
let mut slice=CurveSlice::<Vec3,_,super::super::CurveCoordSemantic2D>::from_coord_range(p.as_slice(),c([1,1030612615]),c([1,1065192767])).remap_2d();
slice.extend_left(f(1120403456));eq_coord(slice.start(),[0,0]);eq_coord(slice.end(),[1,1065192767]);
}
{let p=points(&[[3246861589,3236099130,1096230032],[1089722329,1100558226,1097214363],[3233710308,1093554716,1091714934]]);
let mut slice=CurveSlice::<Vec3,_,super::super::CurveCoordSemantic2D>::from_coord_range(p.as_slice(),c([1,1058784678]),c([1,1058784678])).remap_2d();
slice.extend_left(f(2139095040));eq_coord(slice.start(),[0,0]);eq_coord(slice.end(),[1,1058784678]);
}
{let p=points(&[[3246861589,3236099130,1096230032],[1089722329,1100558226,1097214363],[3233710308,1093554716,1091714934]]);
let mut slice=CurveSlice::<Vec3,_,super::super::CurveCoordSemantic2D>::from_coord_range(p.as_slice(),c([0,1026794548]),c([0,1059770760])).remap_2d();
assert!(std::panic::catch_unwind(std::panic::AssertUnwindSafe(||{slice.extend_left(f(3212836864))})).is_err());
}
{let p=points(&[[3246861589,3236099130,1096230032],[1089722329,1100558226,1097214363],[3233710308,1093554716,1091714934]]);
let mut slice=CurveSlice::<Vec3,_,super::super::CurveCoordSemantic2D>::from_coord_range(p.as_slice(),c([1,1062843457]),c([1,1062843457])).remap_2d();
assert!(std::panic::catch_unwind(std::panic::AssertUnwindSafe(||{slice.extend_left(f(2143289344))})).is_err());
}
{let p=points(&[[3233875629,1100439890,3221750416],[3244641832,3219102097,1099434485],[3239294463,1083348569,1091908989],[1066463965,3247003563,3243820551],[1086267014,1086798572,3247207196],[3229297017,3237025652,3224474684],[3242945823,1099496294,3226449646]]);
let mut slice=CurveSlice::<Vec3,_,super::super::CurveCoordSemantic2D>::from_coord_range(p.as_slice(),c([3,1019275023]),c([3,1062822242])).remap_2d();
slice.extend_left(f(0));eq_coord(slice.start(),[3,1019275008]);eq_coord(slice.end(),[3,1062822242]);
}
{let p=points(&[[3233875629,1100439890,3221750416],[3244641832,3219102097,1099434485],[3239294463,1083348569,1091908989],[1066463965,3247003563,3243820551],[1086267014,1086798572,3247207196],[3229297017,3237025652,3224474684],[3242945823,1099496294,3226449646]]);
let mut slice=CurveSlice::<Vec3,_,super::super::CurveCoordSemantic2D>::from_coord_range(p.as_slice(),c([0,1057646872]),c([1,1062034481])).remap_2d();
slice.extend_left(f(1008981770));eq_coord(slice.start(),[0,1057638826]);eq_coord(slice.end(),[1,1062034481]);
}
{let p=points(&[[3233875629,1100439890,3221750416],[3244641832,3219102097,1099434485],[3239294463,1083348569,1091908989],[1066463965,3247003563,3243820551],[1086267014,1086798572,3247207196],[3229297017,3237025652,3224474684],[3242945823,1099496294,3226449646]]);
let mut slice=CurveSlice::<Vec3,_,super::super::CurveCoordSemantic2D>::from_coord_range(p.as_slice(),c([1,1064138938]),c([1,1064138938])).remap_2d();
slice.extend_left(f(1077936128));eq_coord(slice.start(),[1,1058679001]);eq_coord(slice.end(),[1,1064138938]);
}
{let p=points(&[[3233875629,1100439890,3221750416],[3244641832,3219102097,1099434485],[3239294463,1083348569,1091908989],[1066463965,3247003563,3243820551],[1086267014,1086798572,3247207196],[3229297017,3237025652,3224474684],[3242945823,1099496294,3226449646]]);
let mut slice=CurveSlice::<Vec3,_,super::super::CurveCoordSemantic2D>::from_coord_range(p.as_slice(),c([4,1051424464]),c([5,1052128671])).remap_2d();
slice.extend_left(f(1120403456));eq_coord(slice.start(),[0,0]);eq_coord(slice.end(),[5,1052128671]);
}
{let p=points(&[[3233875629,1100439890,3221750416],[3244641832,3219102097,1099434485],[3239294463,1083348569,1091908989],[1066463965,3247003563,3243820551],[1086267014,1086798572,3247207196],[3229297017,3237025652,3224474684],[3242945823,1099496294,3226449646]]);
let mut slice=CurveSlice::<Vec3,_,super::super::CurveCoordSemantic2D>::from_coord_range(p.as_slice(),c([4,1060067820]),c([4,1060067820])).remap_2d();
slice.extend_left(f(2139095040));eq_coord(slice.start(),[0,0]);eq_coord(slice.end(),[4,1060067820]);
}
{let p=points(&[[3233875629,1100439890,3221750416],[3244641832,3219102097,1099434485],[3239294463,1083348569,1091908989],[1066463965,3247003563,3243820551],[1086267014,1086798572,3247207196],[3229297017,3237025652,3224474684],[3242945823,1099496294,3226449646]]);
let mut slice=CurveSlice::<Vec3,_,super::super::CurveCoordSemantic2D>::from_coord_range(p.as_slice(),c([4,1034061139]),c([5,1061705633])).remap_2d();
assert!(std::panic::catch_unwind(std::panic::AssertUnwindSafe(||{slice.extend_left(f(3212836864))})).is_err());
}
{let p=points(&[[3233875629,1100439890,3221750416],[3244641832,3219102097,1099434485],[3239294463,1083348569,1091908989],[1066463965,3247003563,3243820551],[1086267014,1086798572,3247207196],[3229297017,3237025652,3224474684],[3242945823,1099496294,3226449646]]);
let mut slice=CurveSlice::<Vec3,_,super::super::CurveCoordSemantic2D>::from_coord_range(p.as_slice(),c([0,1061192484]),c([0,1061192484])).remap_2d();
assert!(std::panic::catch_unwind(std::panic::AssertUnwindSafe(||{slice.extend_left(f(2143289344))})).is_err());
}
{let p=points(&[[1094803690,1097716500,1099340436],[3240304422,1095195330,1080068889]]);
let mut slice=CurveSlice::<Vec3,_,super::super::CurveCoordSemantic2D>::from_coord_range(p.as_slice(),c([0,1056972245]),c([0,1056972245])).remap_2d();
slice.extend_left(f(0));eq_coord(slice.start(),[0,1056972245]);eq_coord(slice.end(),[0,1056972245]);
}
{let p=points(&[[1094803690,1097716500,1099340436],[3240304422,1095195330,1080068889]]);
let mut slice=CurveSlice::<Vec3,_,super::super::CurveCoordSemantic2D>::from_coord_range(p.as_slice(),c([0,1048236464]),c([0,1048236464])).remap_2d();
slice.extend_left(f(1008981770));eq_coord(slice.start(),[0,1048210610]);eq_coord(slice.end(),[0,1048236464]);
}
{let p=points(&[[1094803690,1097716500,1099340436],[3240304422,1095195330,1080068889]]);
let mut slice=CurveSlice::<Vec3,_,super::super::CurveCoordSemantic2D>::from_coord_range(p.as_slice(),c([0,1049458301]),c([0,1049458301])).remap_2d();
slice.extend_left(f(1077936128));eq_coord(slice.start(),[0,1042584621]);eq_coord(slice.end(),[0,1049458301]);
}
{let p=points(&[[1094803690,1097716500,1099340436],[3240304422,1095195330,1080068889]]);
let mut slice=CurveSlice::<Vec3,_,super::super::CurveCoordSemantic2D>::from_coord_range(p.as_slice(),c([0,1060191918]),c([0,1060191918])).remap_2d();
slice.extend_left(f(1120403456));eq_coord(slice.start(),[0,0]);eq_coord(slice.end(),[0,1060191918]);
}
{let p=points(&[[1094803690,1097716500,1099340436],[3240304422,1095195330,1080068889]]);
let mut slice=CurveSlice::<Vec3,_,super::super::CurveCoordSemantic2D>::from_coord_range(p.as_slice(),c([0,1061948083]),c([0,1061948083])).remap_2d();
slice.extend_left(f(2139095040));eq_coord(slice.start(),[0,0]);eq_coord(slice.end(),[0,1061948083]);
}
{let p=points(&[[1094803690,1097716500,1099340436],[3240304422,1095195330,1080068889]]);
let mut slice=CurveSlice::<Vec3,_,super::super::CurveCoordSemantic2D>::from_coord_range(p.as_slice(),c([0,1057970121]),c([0,1062602239])).remap_2d();
assert!(std::panic::catch_unwind(std::panic::AssertUnwindSafe(||{slice.extend_left(f(3212836864))})).is_err());
}
{let p=points(&[[1094803690,1097716500,1099340436],[3240304422,1095195330,1080068889]]);
let mut slice=CurveSlice::<Vec3,_,super::super::CurveCoordSemantic2D>::from_coord_range(p.as_slice(),c([0,1051101250]),c([0,1057530129])).remap_2d();
assert!(std::panic::catch_unwind(std::panic::AssertUnwindSafe(||{slice.extend_left(f(2143289344))})).is_err());
}
{let p=points(&[[0,0,0],[1077936128,1073741824,1082130432],[1086324736,1082130432,1082130432],[1086324736,1086324736,1091567616]]);
let view=LoopCurvePoints{points:p.as_slice(),range:crate::loop_range::LoopRange::new_loop(-1,4,4)};
let mut slice=CurveSlice::<Vec3,_,super::super::CurveCoordSemantic2D>::from_coord_range(view,c([1,1050253722]),c([2,1061997773])).remap_2d();
slice.extend_left(f(0));eq_coord(slice.start(),[1,1050253722]);eq_coord(slice.end(),[2,1061997773]);
}
{let p=points(&[[0,0,0],[1077936128,1073741824,1082130432],[1086324736,1082130432,1082130432],[1086324736,1086324736,1091567616]]);
let view=LoopCurvePoints{points:p.as_slice(),range:crate::loop_range::LoopRange::new_loop(-1,4,4)};
let mut slice=CurveSlice::<Vec3,_,super::super::CurveCoordSemantic2D>::from_coord_range(view,c([1,1050253722]),c([2,1061997773])).remap_2d();
slice.extend_left(f(1077936128));eq_coord(slice.start(),[0,1063026635]);eq_coord(slice.end(),[2,1061997773]);
}
{let p=points(&[[0,0,0],[1077936128,1073741824,1082130432],[1086324736,1082130432,1082130432],[1086324736,1086324736,1091567616]]);
let view=LoopCurvePoints{points:p.as_slice(),range:crate::loop_range::LoopRange::new_loop(-1,4,4)};
let mut slice=CurveSlice::<Vec3,_,super::super::CurveCoordSemantic2D>::from_coord_range(view,c([1,1050253722]),c([2,1061997773])).remap_2d();
slice.extend_left(f(1120403456));eq_coord(slice.start(),[0,0]);eq_coord(slice.end(),[2,1061997773]);
}
{let p=points(&[[0,0,0],[0,1065353216,0],[1065353216,1082130432,0],[1073741824,1088421888,0]]);
let view=LoopCurvePoints{points:p.as_slice(),range:crate::loop_range::LoopRange::new_loop(-1,4,4)};
let mut slice=CurveSlice::<Vec3,_,super::super::CurveCoordSemantic2D>::from_coord_range(view,c([1,1050253722]),c([2,1061997773])).remap_2d();
slice.extend_left(f(0));eq_coord(slice.start(),[1,4290772992]);eq_coord(slice.end(),[2,1061997773]);
}
{let p=points(&[[0,0,0],[0,1065353216,0],[1065353216,1082130432,0],[1073741824,1088421888,0]]);
let view=LoopCurvePoints{points:p.as_slice(),range:crate::loop_range::LoopRange::new_loop(-1,4,4)};
let mut slice=CurveSlice::<Vec3,_,super::super::CurveCoordSemantic2D>::from_coord_range(view,c([1,1050253722]),c([2,1061997773])).remap_2d();
slice.extend_left(f(1077936128));eq_coord(slice.start(),[0,0]);eq_coord(slice.end(),[2,1061997773]);
}
{let p=points(&[[0,0,0],[0,1065353216,0],[1065353216,1082130432,0],[1073741824,1088421888,0]]);
let view=LoopCurvePoints{points:p.as_slice(),range:crate::loop_range::LoopRange::new_loop(-1,4,4)};
let mut slice=CurveSlice::<Vec3,_,super::super::CurveCoordSemantic2D>::from_coord_range(view,c([1,1050253722]),c([2,1061997773])).remap_2d();
slice.extend_left(f(1120403456));eq_coord(slice.start(),[0,0]);eq_coord(slice.end(),[2,1061997773]);
}
{let p=points(&[[3242262499,3208823723,1095550516],[3247643230,1082972693,1065410640],[3225834784,3229971066,1086543384]]);
let view=LoopCurvePoints{points:p.as_slice(),range:crate::loop_range::LoopRange::new_loop(-1,3,3)};
let mut slice=CurveSlice::<Vec3,_,super::super::CurveCoordSemantic2D>::from_coord_range(view,c([1,1050253722]),c([2,1061997773])).remap_2d();
slice.extend_left(f(0));eq_coord(slice.start(),[1,1050253722]);eq_coord(slice.end(),[2,1061997773]);
}
{let p=points(&[[3242262499,3208823723,1095550516],[3247643230,1082972693,1065410640],[3225834784,3229971066,1086543384]]);
let view=LoopCurvePoints{points:p.as_slice(),range:crate::loop_range::LoopRange::new_loop(-1,3,3)};
let mut slice=CurveSlice::<Vec3,_,super::super::CurveCoordSemantic2D>::from_coord_range(view,c([1,1050253722]),c([2,1061997773])).remap_2d();
slice.extend_left(f(1077936128));eq_coord(slice.start(),[1,1033584089]);eq_coord(slice.end(),[2,1061997773]);
}
{let p=points(&[[3242262499,3208823723,1095550516],[3247643230,1082972693,1065410640],[3225834784,3229971066,1086543384]]);
let view=LoopCurvePoints{points:p.as_slice(),range:crate::loop_range::LoopRange::new_loop(-1,3,3)};
let mut slice=CurveSlice::<Vec3,_,super::super::CurveCoordSemantic2D>::from_coord_range(view,c([1,1050253722]),c([2,1061997773])).remap_2d();
slice.extend_left(f(1120403456));eq_coord(slice.start(),[0,0]);eq_coord(slice.end(),[2,1061997773]);
}
{let p=points(&[[1084747998,1086586339,1100903328],[1095998162,1094281809,3183456677],[1096916140,3247270388,3240764055],[3243037119,1100811594,3239211518],[3233700383,1099539686,3239259458],[3230381993,1096713839,3247535037],[1095330858,1099423118,1094774569],[1069596664,3225385354,1098326837]]);
let view=LoopCurvePoints{points:p.as_slice(),range:crate::loop_range::LoopRange::new_loop(-1,8,8)};
let mut slice=CurveSlice::<Vec3,_,super::super::CurveCoordSemantic2D>::from_coord_range(view,c([1,1050253722]),c([2,1061997773])).remap_2d();
slice.extend_left(f(0));eq_coord(slice.start(),[1,1050253722]);eq_coord(slice.end(),[2,1061997773]);
}
{let p=points(&[[1084747998,1086586339,1100903328],[1095998162,1094281809,3183456677],[1096916140,3247270388,3240764055],[3243037119,1100811594,3239211518],[3233700383,1099539686,3239259458],[3230381993,1096713839,3247535037],[1095330858,1099423118,1094774569],[1069596664,3225385354,1098326837]]);
let view=LoopCurvePoints{points:p.as_slice(),range:crate::loop_range::LoopRange::new_loop(-1,8,8)};
let mut slice=CurveSlice::<Vec3,_,super::super::CurveCoordSemantic2D>::from_coord_range(view,c([1,1050253722]),c([2,1061997773])).remap_2d();
slice.extend_left(f(1077936128));eq_coord(slice.start(),[1,1042540942]);eq_coord(slice.end(),[2,1061997773]);
}
{let p=points(&[[1084747998,1086586339,1100903328],[1095998162,1094281809,3183456677],[1096916140,3247270388,3240764055],[3243037119,1100811594,3239211518],[3233700383,1099539686,3239259458],[3230381993,1096713839,3247535037],[1095330858,1099423118,1094774569],[1069596664,3225385354,1098326837]]);
let view=LoopCurvePoints{points:p.as_slice(),range:crate::loop_range::LoopRange::new_loop(-1,8,8)};
let mut slice=CurveSlice::<Vec3,_,super::super::CurveCoordSemantic2D>::from_coord_range(view,c([1,1050253722]),c([2,1061997773])).remap_2d();
slice.extend_left(f(1120403456));eq_coord(slice.start(),[0,0]);eq_coord(slice.end(),[2,1061997773]);
}
{let p=points(&[[1082977368,3221932557,3247282740],[1091417814,1097176541,1068208583],[1100111636,1075139201,1093142526],[3213382931,3214971046,1099242834],[3242840851,3239170174,1095797098],[1095376411,3240195434,3244981559],[1099755233,1095767615,3246438629]]);
let view=LoopCurvePoints{points:p.as_slice(),range:crate::loop_range::LoopRange::new_loop(-1,7,7)};
let mut slice=CurveSlice::<Vec3,_,super::super::CurveCoordSemantic2D>::from_coord_range(view,c([1,1050253722]),c([2,1061997773])).remap_2d();
slice.extend_left(f(0));eq_coord(slice.start(),[1,1050253722]);eq_coord(slice.end(),[2,1061997773]);
}
{let p=points(&[[1082977368,3221932557,3247282740],[1091417814,1097176541,1068208583],[1100111636,1075139201,1093142526],[3213382931,3214971046,1099242834],[3242840851,3239170174,1095797098],[1095376411,3240195434,3244981559],[1099755233,1095767615,3246438629]]);
let view=LoopCurvePoints{points:p.as_slice(),range:crate::loop_range::LoopRange::new_loop(-1,7,7)};
let mut slice=CurveSlice::<Vec3,_,super::super::CurveCoordSemantic2D>::from_coord_range(view,c([1,1050253722]),c([2,1061997773])).remap_2d();
slice.extend_left(f(1077936128));eq_coord(slice.start(),[1,1041635788]);eq_coord(slice.end(),[2,1061997773]);
}
{let p=points(&[[1082977368,3221932557,3247282740],[1091417814,1097176541,1068208583],[1100111636,1075139201,1093142526],[3213382931,3214971046,1099242834],[3242840851,3239170174,1095797098],[1095376411,3240195434,3244981559],[1099755233,1095767615,3246438629]]);
let view=LoopCurvePoints{points:p.as_slice(),range:crate::loop_range::LoopRange::new_loop(-1,7,7)};
let mut slice=CurveSlice::<Vec3,_,super::super::CurveCoordSemantic2D>::from_coord_range(view,c([1,1050253722]),c([2,1061997773])).remap_2d();
slice.extend_left(f(1120403456));eq_coord(slice.start(),[0,0]);eq_coord(slice.end(),[2,1061997773]);
}
{let p=points(&[[3247798851,1058059480,3233609334],[3240528963,1083175319,3219678495]]);
let view=LoopCurvePoints{points:p.as_slice(),range:crate::loop_range::LoopRange::new_loop(-1,2,2)};
let mut slice=CurveSlice::<Vec3,_,super::super::CurveCoordSemantic2D>::from_coord_range(view,c([1,1050253722]),c([1,1061997773])).remap_2d();
slice.extend_left(f(0));eq_coord(slice.start(),[1,1050253722]);eq_coord(slice.end(),[1,1061997773]);
}
{let p=points(&[[3247798851,1058059480,3233609334],[3240528963,1083175319,3219678495]]);
let view=LoopCurvePoints{points:p.as_slice(),range:crate::loop_range::LoopRange::new_loop(-1,2,2)};
let mut slice=CurveSlice::<Vec3,_,super::super::CurveCoordSemantic2D>::from_coord_range(view,c([1,1050253722]),c([1,1061997773])).remap_2d();
slice.extend_left(f(1077936128));eq_coord(slice.start(),[0,1064933788]);eq_coord(slice.end(),[1,1061997773]);
}
{let p=points(&[[3247798851,1058059480,3233609334],[3240528963,1083175319,3219678495]]);
let view=LoopCurvePoints{points:p.as_slice(),range:crate::loop_range::LoopRange::new_loop(-1,2,2)};
let mut slice=CurveSlice::<Vec3,_,super::super::CurveCoordSemantic2D>::from_coord_range(view,c([1,1050253722]),c([1,1061997773])).remap_2d();
slice.extend_left(f(1120403456));eq_coord(slice.start(),[0,0]);eq_coord(slice.end(),[1,1061997773]);
}
{let p=points(&[[3248254451,3241936917,1099955010],[1079140333,3244934773,3213218942],[1090209441,1052470290,3230079560],[1095496662,3240322186,1095800513],[1092103392,3242769597,3226461344],[1087355470,1093338760,1090907651],[3225938952,1100032890,1099521180]]);
let view=LoopCurvePoints{points:p.as_slice(),range:crate::loop_range::LoopRange::new_loop(-1,7,7)};
let mut slice=CurveSlice::<Vec3,_,super::super::CurveCoordSemantic2D>::from_coord_range(view,c([1,1050253722]),c([2,1061997773])).remap_2d();
slice.extend_left(f(0));eq_coord(slice.start(),[1,1050253722]);eq_coord(slice.end(),[2,1061997773]);
}
{let p=points(&[[3248254451,3241936917,1099955010],[1079140333,3244934773,3213218942],[1090209441,1052470290,3230079560],[1095496662,3240322186,1095800513],[1092103392,3242769597,3226461344],[1087355470,1093338760,1090907651],[3225938952,1100032890,1099521180]]);
let view=LoopCurvePoints{points:p.as_slice(),range:crate::loop_range::LoopRange::new_loop(-1,7,7)};
let mut slice=CurveSlice::<Vec3,_,super::super::CurveCoordSemantic2D>::from_coord_range(view,c([1,1050253722]),c([2,1061997773])).remap_2d();
slice.extend_left(f(1077936128));eq_coord(slice.start(),[1,1045161425]);eq_coord(slice.end(),[2,1061997773]);
}
{let p=points(&[[3248254451,3241936917,1099955010],[1079140333,3244934773,3213218942],[1090209441,1052470290,3230079560],[1095496662,3240322186,1095800513],[1092103392,3242769597,3226461344],[1087355470,1093338760,1090907651],[3225938952,1100032890,1099521180]]);
let view=LoopCurvePoints{points:p.as_slice(),range:crate::loop_range::LoopRange::new_loop(-1,7,7)};
let mut slice=CurveSlice::<Vec3,_,super::super::CurveCoordSemantic2D>::from_coord_range(view,c([1,1050253722]),c([2,1061997773])).remap_2d();
slice.extend_left(f(1120403456));eq_coord(slice.start(),[0,0]);eq_coord(slice.end(),[2,1061997773]);
}
{let p=points(&[[3212759075,3245919388,3242556122],[3229372900,3233527430,3234242874],[3247490566,1099735996,3246681455],[1100591571,3246526217,1094965764],[1099566613,3242955625,999123697],[3238120409,3235777217,1097885840],[3244363478,1099288271,1088099009],[1080504870,1075892897,3239915365]]);
let view=LoopCurvePoints{points:p.as_slice(),range:crate::loop_range::LoopRange::new_loop(-1,8,8)};
let mut slice=CurveSlice::<Vec3,_,super::super::CurveCoordSemantic2D>::from_coord_range(view,c([1,1050253722]),c([2,1061997773])).remap_2d();
slice.extend_left(f(0));eq_coord(slice.start(),[1,1050253722]);eq_coord(slice.end(),[2,1061997773]);
}
{let p=points(&[[3212759075,3245919388,3242556122],[3229372900,3233527430,3234242874],[3247490566,1099735996,3246681455],[1100591571,3246526217,1094965764],[1099566613,3242955625,999123697],[3238120409,3235777217,1097885840],[3244363478,1099288271,1088099009],[1080504870,1075892897,3239915365]]);
let view=LoopCurvePoints{points:p.as_slice(),range:crate::loop_range::LoopRange::new_loop(-1,8,8)};
let mut slice=CurveSlice::<Vec3,_,super::super::CurveCoordSemantic2D>::from_coord_range(view,c([1,1050253722]),c([2,1061997773])).remap_2d();
slice.extend_left(f(1077936128));eq_coord(slice.start(),[0,1062292211]);eq_coord(slice.end(),[2,1061997773]);
}
{let p=points(&[[3212759075,3245919388,3242556122],[3229372900,3233527430,3234242874],[3247490566,1099735996,3246681455],[1100591571,3246526217,1094965764],[1099566613,3242955625,999123697],[3238120409,3235777217,1097885840],[3244363478,1099288271,1088099009],[1080504870,1075892897,3239915365]]);
let view=LoopCurvePoints{points:p.as_slice(),range:crate::loop_range::LoopRange::new_loop(-1,8,8)};
let mut slice=CurveSlice::<Vec3,_,super::super::CurveCoordSemantic2D>::from_coord_range(view,c([1,1050253722]),c([2,1061997773])).remap_2d();
slice.extend_left(f(1120403456));eq_coord(slice.start(),[0,0]);eq_coord(slice.end(),[2,1061997773]);
}
{let p=points(&[[1097867177,1082561744,3236069917],[1100696440,3214659718,3240360452],[1097567244,1098257180,1100860970],[3235053765,1087985259,3240223001]]);
let view=LoopCurvePoints{points:p.as_slice(),range:crate::loop_range::LoopRange::new_loop(-1,4,4)};
let mut slice=CurveSlice::<Vec3,_,super::super::CurveCoordSemantic2D>::from_coord_range(view,c([1,1050253722]),c([2,1061997773])).remap_2d();
slice.extend_left(f(0));eq_coord(slice.start(),[1,1050253722]);eq_coord(slice.end(),[2,1061997773]);
}
{let p=points(&[[1097867177,1082561744,3236069917],[1100696440,3214659718,3240360452],[1097567244,1098257180,1100860970],[3235053765,1087985259,3240223001]]);
let view=LoopCurvePoints{points:p.as_slice(),range:crate::loop_range::LoopRange::new_loop(-1,4,4)};
let mut slice=CurveSlice::<Vec3,_,super::super::CurveCoordSemantic2D>::from_coord_range(view,c([1,1050253722]),c([2,1061997773])).remap_2d();
slice.extend_left(f(1077936128));eq_coord(slice.start(),[0,1064297967]);eq_coord(slice.end(),[2,1061997773]);
}
{let p=points(&[[1097867177,1082561744,3236069917],[1100696440,3214659718,3240360452],[1097567244,1098257180,1100860970],[3235053765,1087985259,3240223001]]);
let view=LoopCurvePoints{points:p.as_slice(),range:crate::loop_range::LoopRange::new_loop(-1,4,4)};
let mut slice=CurveSlice::<Vec3,_,super::super::CurveCoordSemantic2D>::from_coord_range(view,c([1,1050253722]),c([2,1061997773])).remap_2d();
slice.extend_left(f(1120403456));eq_coord(slice.start(),[0,0]);eq_coord(slice.end(),[2,1061997773]);
}
{let p=points(&[[1099010029,3244580215,1083184304],[1093727935,3231322885,3243514808],[3244515978,1099562993,1098985396],[3247736614,1094079983,1099238188],[1090771178,3233238451,3247286416],[1076830784,3243246862,1087956944],[3232418479,3231129765,3226472668]]);
let view=LoopCurvePoints{points:p.as_slice(),range:crate::loop_range::LoopRange::new_loop(-1,7,7)};
let mut slice=CurveSlice::<Vec3,_,super::super::CurveCoordSemantic2D>::from_coord_range(view,c([1,1050253722]),c([2,1061997773])).remap_2d();
slice.extend_left(f(0));eq_coord(slice.start(),[1,1050253722]);eq_coord(slice.end(),[2,1061997773]);
}
{let p=points(&[[1099010029,3244580215,1083184304],[1093727935,3231322885,3243514808],[3244515978,1099562993,1098985396],[3247736614,1094079983,1099238188],[1090771178,3233238451,3247286416],[1076830784,3243246862,1087956944],[3232418479,3231129765,3226472668]]);
let view=LoopCurvePoints{points:p.as_slice(),range:crate::loop_range::LoopRange::new_loop(-1,7,7)};
let mut slice=CurveSlice::<Vec3,_,super::super::CurveCoordSemantic2D>::from_coord_range(view,c([1,1050253722]),c([2,1061997773])).remap_2d();
slice.extend_left(f(1077936128));eq_coord(slice.start(),[1,1041041144]);eq_coord(slice.end(),[2,1061997773]);
}
{let p=points(&[[1099010029,3244580215,1083184304],[1093727935,3231322885,3243514808],[3244515978,1099562993,1098985396],[3247736614,1094079983,1099238188],[1090771178,3233238451,3247286416],[1076830784,3243246862,1087956944],[3232418479,3231129765,3226472668]]);
let view=LoopCurvePoints{points:p.as_slice(),range:crate::loop_range::LoopRange::new_loop(-1,7,7)};
let mut slice=CurveSlice::<Vec3,_,super::super::CurveCoordSemantic2D>::from_coord_range(view,c([1,1050253722]),c([2,1061997773])).remap_2d();
slice.extend_left(f(1120403456));eq_coord(slice.start(),[0,0]);eq_coord(slice.end(),[2,1061997773]);
}
{let p=points(&[[3230146028,3235895519,3247361702],[1100544358,3247787439,1088863577],[3247088989,1100658099,3239814798],[1058223971,3231476188,3242121657],[3247695502,1097243273,1086462731],[3242328033,1097305974,1088204397],[3245446996,1095672463,3238153751]]);
let view=LoopCurvePoints{points:p.as_slice(),range:crate::loop_range::LoopRange::new_loop(-1,7,7)};
let mut slice=CurveSlice::<Vec3,_,super::super::CurveCoordSemantic2D>::from_coord_range(view,c([1,1050253722]),c([2,1061997773])).remap_2d();
slice.extend_left(f(0));eq_coord(slice.start(),[1,1050253722]);eq_coord(slice.end(),[2,1061997773]);
}
{let p=points(&[[3230146028,3235895519,3247361702],[1100544358,3247787439,1088863577],[3247088989,1100658099,3239814798],[1058223971,3231476188,3242121657],[3247695502,1097243273,1086462731],[3242328033,1097305974,1088204397],[3245446996,1095672463,3238153751]]);
let view=LoopCurvePoints{points:p.as_slice(),range:crate::loop_range::LoopRange::new_loop(-1,7,7)};
let mut slice=CurveSlice::<Vec3,_,super::super::CurveCoordSemantic2D>::from_coord_range(view,c([1,1050253722]),c([2,1061997773])).remap_2d();
slice.extend_left(f(1077936128));eq_coord(slice.start(),[1,1046056869]);eq_coord(slice.end(),[2,1061997773]);
}
{let p=points(&[[3230146028,3235895519,3247361702],[1100544358,3247787439,1088863577],[3247088989,1100658099,3239814798],[1058223971,3231476188,3242121657],[3247695502,1097243273,1086462731],[3242328033,1097305974,1088204397],[3245446996,1095672463,3238153751]]);
let view=LoopCurvePoints{points:p.as_slice(),range:crate::loop_range::LoopRange::new_loop(-1,7,7)};
let mut slice=CurveSlice::<Vec3,_,super::super::CurveCoordSemantic2D>::from_coord_range(view,c([1,1050253722]),c([2,1061997773])).remap_2d();
slice.extend_left(f(1120403456));eq_coord(slice.start(),[0,0]);eq_coord(slice.end(),[2,1061997773]);
}
{let p=points(&[[3244590905,3197112644,3241275444],[1091470735,1078434697,3246801676]]);
let view=LoopCurvePoints{points:p.as_slice(),range:crate::loop_range::LoopRange::new_loop(-1,2,2)};
let mut slice=CurveSlice::<Vec3,_,super::super::CurveCoordSemantic2D>::from_coord_range(view,c([1,1050253722]),c([1,1061997773])).remap_2d();
slice.extend_left(f(0));eq_coord(slice.start(),[1,1050253722]);eq_coord(slice.end(),[1,1061997773]);
}
{let p=points(&[[3244590905,3197112644,3241275444],[1091470735,1078434697,3246801676]]);
let view=LoopCurvePoints{points:p.as_slice(),range:crate::loop_range::LoopRange::new_loop(-1,2,2)};
let mut slice=CurveSlice::<Vec3,_,super::super::CurveCoordSemantic2D>::from_coord_range(view,c([1,1050253722]),c([1,1061997773])).remap_2d();
slice.extend_left(f(1077936128));eq_coord(slice.start(),[1,1043497755]);eq_coord(slice.end(),[1,1061997773]);
}
{let p=points(&[[3244590905,3197112644,3241275444],[1091470735,1078434697,3246801676]]);
let view=LoopCurvePoints{points:p.as_slice(),range:crate::loop_range::LoopRange::new_loop(-1,2,2)};
let mut slice=CurveSlice::<Vec3,_,super::super::CurveCoordSemantic2D>::from_coord_range(view,c([1,1050253722]),c([1,1061997773])).remap_2d();
slice.extend_left(f(1120403456));eq_coord(slice.start(),[0,0]);eq_coord(slice.end(),[1,1061997773]);
}
}
#[test]fn original_b_slice_extend_right(){
{let p=points(&[[0,0,0],[1077936128,1073741824,1082130432],[1086324736,1082130432,1082130432],[1086324736,1086324736,1091567616]]);
let mut slice=CurveSlice::<Vec3,_,super::super::CurveCoordSemantic2D>::from_coord_range(p.as_slice(),c([1,1064097808]),c([1,1064097808])).remap_2d();
slice.extend_right(f(0));eq_coord(slice.start(),[1,1064097808]);eq_coord(slice.end(),[1,1064097808]);
}
{let p=points(&[[0,0,0],[1077936128,1073741824,1082130432],[1086324736,1082130432,1082130432],[1086324736,1086324736,1091567616]]);
let mut slice=CurveSlice::<Vec3,_,super::super::CurveCoordSemantic2D>::from_coord_range(p.as_slice(),c([2,1061496080]),c([2,1062572540])).remap_2d();
slice.extend_right(f(1008981770));eq_coord(slice.start(),[2,1061496080]);eq_coord(slice.end(),[2,1062606094]);
}
{let p=points(&[[0,0,0],[1077936128,1073741824,1082130432],[1086324736,1082130432,1082130432],[1086324736,1086324736,1091567616]]);
let mut slice=CurveSlice::<Vec3,_,super::super::CurveCoordSemantic2D>::from_coord_range(p.as_slice(),c([1,1058763572]),c([2,1051605229])).remap_2d();
slice.extend_right(f(1077936128));eq_coord(slice.start(),[1,1058763572]);eq_coord(slice.end(),[2,1064351248]);
}
{let p=points(&[[0,0,0],[1077936128,1073741824,1082130432],[1086324736,1082130432,1082130432],[1086324736,1086324736,1091567616]]);
let mut slice=CurveSlice::<Vec3,_,super::super::CurveCoordSemantic2D>::from_coord_range(p.as_slice(),c([0,1061581981]),c([0,1061581981])).remap_2d();
slice.extend_right(f(1120403456));eq_coord(slice.start(),[0,1061581981]);eq_coord(slice.end(),[2,1065353216]);
}
{let p=points(&[[0,0,0],[1077936128,1073741824,1082130432],[1086324736,1082130432,1082130432],[1086324736,1086324736,1091567616]]);
let mut slice=CurveSlice::<Vec3,_,super::super::CurveCoordSemantic2D>::from_coord_range(p.as_slice(),c([1,1042773571]),c([1,1059229868])).remap_2d();
slice.extend_right(f(2139095040));eq_coord(slice.start(),[1,1042773571]);eq_coord(slice.end(),[2,1065353216]);
}
{let p=points(&[[0,0,0],[1077936128,1073741824,1082130432],[1086324736,1082130432,1082130432],[1086324736,1086324736,1091567616]]);
let mut slice=CurveSlice::<Vec3,_,super::super::CurveCoordSemantic2D>::from_coord_range(p.as_slice(),c([1,1054477217]),c([1,1059526067])).remap_2d();
assert!(std::panic::catch_unwind(std::panic::AssertUnwindSafe(||{slice.extend_right(f(3212836864))})).is_err());
}
{let p=points(&[[0,0,0],[1077936128,1073741824,1082130432],[1086324736,1082130432,1082130432],[1086324736,1086324736,1091567616]]);
let mut slice=CurveSlice::<Vec3,_,super::super::CurveCoordSemantic2D>::from_coord_range(p.as_slice(),c([1,1059365910]),c([1,1059365910])).remap_2d();
assert!(std::panic::catch_unwind(std::panic::AssertUnwindSafe(||{slice.extend_right(f(2143289344))})).is_err());
}
{let p=points(&[[0,0,0],[0,1065353216,0],[1065353216,1082130432,0],[1073741824,1088421888,0]]);
let mut slice=CurveSlice::<Vec3,_,super::super::CurveCoordSemantic2D>::from_coord_range(p.as_slice(),c([0,1056585420]),c([1,1043330619])).remap_2d();
slice.extend_right(f(0));eq_coord(slice.start(),[0,1056585420]);eq_coord(slice.end(),[1,1043330620]);
}
{let p=points(&[[0,0,0],[0,1065353216,0],[1065353216,1082130432,0],[1073741824,1088421888,0]]);
let mut slice=CurveSlice::<Vec3,_,super::super::CurveCoordSemantic2D>::from_coord_range(p.as_slice(),c([2,1048632443]),c([2,1048632443])).remap_2d();
slice.extend_right(f(1008981770));eq_coord(slice.start(),[2,1048632443]);eq_coord(slice.end(),[2,1048967988]);
}
{let p=points(&[[0,0,0],[0,1065353216,0],[1065353216,1082130432,0],[1073741824,1088421888,0]]);
let mut slice=CurveSlice::<Vec3,_,super::super::CurveCoordSemantic2D>::from_coord_range(p.as_slice(),c([0,1024647406]),c([0,1063050521])).remap_2d();
slice.extend_right(f(1077936128));eq_coord(slice.start(),[0,1024647406]);eq_coord(slice.end(),[2,1065353216]);
}
{let p=points(&[[0,0,0],[0,1065353216,0],[1065353216,1082130432,0],[1073741824,1088421888,0]]);
let mut slice=CurveSlice::<Vec3,_,super::super::CurveCoordSemantic2D>::from_coord_range(p.as_slice(),c([1,1058705607]),c([2,1050832676])).remap_2d();
slice.extend_right(f(1120403456));eq_coord(slice.start(),[1,1058705607]);eq_coord(slice.end(),[2,1065353216]);
}
{let p=points(&[[0,0,0],[0,1065353216,0],[1065353216,1082130432,0],[1073741824,1088421888,0]]);
let mut slice=CurveSlice::<Vec3,_,super::super::CurveCoordSemantic2D>::from_coord_range(p.as_slice(),c([1,1058599616]),c([2,1025715344])).remap_2d();
slice.extend_right(f(2139095040));eq_coord(slice.start(),[1,1058599616]);eq_coord(slice.end(),[2,1065353216]);
}
{let p=points(&[[0,0,0],[0,1065353216,0],[1065353216,1082130432,0],[1073741824,1088421888,0]]);
let mut slice=CurveSlice::<Vec3,_,super::super::CurveCoordSemantic2D>::from_coord_range(p.as_slice(),c([1,1051498220]),c([2,1057430890])).remap_2d();
assert!(std::panic::catch_unwind(std::panic::AssertUnwindSafe(||{slice.extend_right(f(3212836864))})).is_err());
}
{let p=points(&[[0,0,0],[0,1065353216,0],[1065353216,1082130432,0],[1073741824,1088421888,0]]);
let mut slice=CurveSlice::<Vec3,_,super::super::CurveCoordSemantic2D>::from_coord_range(p.as_slice(),c([2,1057218097]),c([2,1060951039])).remap_2d();
assert!(std::panic::catch_unwind(std::panic::AssertUnwindSafe(||{slice.extend_right(f(2143289344))})).is_err());
}
{let p=points(&[[3242262499,3208823723,1095550516],[3247643230,1082972693,1065410640],[3225834784,3229971066,1086543384]]);
let mut slice=CurveSlice::<Vec3,_,super::super::CurveCoordSemantic2D>::from_coord_range(p.as_slice(),c([1,1062132974]),c([1,1062132974])).remap_2d();
slice.extend_right(f(0));eq_coord(slice.start(),[1,1062132974]);eq_coord(slice.end(),[1,1062132974]);
}
{let p=points(&[[3242262499,3208823723,1095550516],[3247643230,1082972693,1065410640],[3225834784,3229971066,1086543384]]);
let mut slice=CurveSlice::<Vec3,_,super::super::CurveCoordSemantic2D>::from_coord_range(p.as_slice(),c([1,1061713406]),c([1,1061713406])).remap_2d();
slice.extend_right(f(1008981770));eq_coord(slice.start(),[1,1061713406]);eq_coord(slice.end(),[1,1061723816]);
}
{let p=points(&[[3242262499,3208823723,1095550516],[3247643230,1082972693,1065410640],[3225834784,3229971066,1086543384]]);
let mut slice=CurveSlice::<Vec3,_,super::super::CurveCoordSemantic2D>::from_coord_range(p.as_slice(),c([0,1054567680]),c([1,1037972024])).remap_2d();
slice.extend_right(f(1077936128));eq_coord(slice.start(),[0,1054567680]);eq_coord(slice.end(),[1,1050073910]);
}
{let p=points(&[[3242262499,3208823723,1095550516],[3247643230,1082972693,1065410640],[3225834784,3229971066,1086543384]]);
let mut slice=CurveSlice::<Vec3,_,super::super::CurveCoordSemantic2D>::from_coord_range(p.as_slice(),c([0,1016437482]),c([1,1058105392])).remap_2d();
slice.extend_right(f(1120403456));eq_coord(slice.start(),[0,1016437482]);eq_coord(slice.end(),[1,1065353216]);
}
{let p=points(&[[3242262499,3208823723,1095550516],[3247643230,1082972693,1065410640],[3225834784,3229971066,1086543384]]);
let mut slice=CurveSlice::<Vec3,_,super::super::CurveCoordSemantic2D>::from_coord_range(p.as_slice(),c([0,1049488287]),c([0,1050382508])).remap_2d();
slice.extend_right(f(2139095040));eq_coord(slice.start(),[0,1049488287]);eq_coord(slice.end(),[1,1065353216]);
}
{let p=points(&[[3242262499,3208823723,1095550516],[3247643230,1082972693,1065410640],[3225834784,3229971066,1086543384]]);
let mut slice=CurveSlice::<Vec3,_,super::super::CurveCoordSemantic2D>::from_coord_range(p.as_slice(),c([0,1041915560]),c([1,1063951914])).remap_2d();
assert!(std::panic::catch_unwind(std::panic::AssertUnwindSafe(||{slice.extend_right(f(3212836864))})).is_err());
}
{let p=points(&[[3242262499,3208823723,1095550516],[3247643230,1082972693,1065410640],[3225834784,3229971066,1086543384]]);
let mut slice=CurveSlice::<Vec3,_,super::super::CurveCoordSemantic2D>::from_coord_range(p.as_slice(),c([1,1061171988]),c([1,1061171988])).remap_2d();
assert!(std::panic::catch_unwind(std::panic::AssertUnwindSafe(||{slice.extend_right(f(2143289344))})).is_err());
}
{let p=points(&[[1084747998,1086586339,1100903328],[1095998162,1094281809,3183456677],[1096916140,3247270388,3240764055],[3243037119,1100811594,3239211518],[3233700383,1099539686,3239259458],[3230381993,1096713839,3247535037],[1095330858,1099423118,1094774569],[1069596664,3225385354,1098326837]]);
let mut slice=CurveSlice::<Vec3,_,super::super::CurveCoordSemantic2D>::from_coord_range(p.as_slice(),c([5,1049368080]),c([5,1049368080])).remap_2d();
slice.extend_right(f(0));eq_coord(slice.start(),[5,1049368080]);eq_coord(slice.end(),[5,1049368080]);
}
{let p=points(&[[1084747998,1086586339,1100903328],[1095998162,1094281809,3183456677],[1096916140,3247270388,3240764055],[3243037119,1100811594,3239211518],[3233700383,1099539686,3239259458],[3230381993,1096713839,3247535037],[1095330858,1099423118,1094774569],[1069596664,3225385354,1098326837]]);
let mut slice=CurveSlice::<Vec3,_,super::super::CurveCoordSemantic2D>::from_coord_range(p.as_slice(),c([3,1058528140]),c([4,1058594588])).remap_2d();
slice.extend_right(f(1008981770));eq_coord(slice.start(),[3,1058528140]);eq_coord(slice.end(),[4,1058612982]);
}
{let p=points(&[[1084747998,1086586339,1100903328],[1095998162,1094281809,3183456677],[1096916140,3247270388,3240764055],[3243037119,1100811594,3239211518],[3233700383,1099539686,3239259458],[3230381993,1096713839,3247535037],[1095330858,1099423118,1094774569],[1069596664,3225385354,1098326837]]);
let mut slice=CurveSlice::<Vec3,_,super::super::CurveCoordSemantic2D>::from_coord_range(p.as_slice(),c([1,1026189021]),c([6,1056843087])).remap_2d();
slice.extend_right(f(1077936128));eq_coord(slice.start(),[1,1026189021]);eq_coord(slice.end(),[6,1061246811]);
}
{let p=points(&[[1084747998,1086586339,1100903328],[1095998162,1094281809,3183456677],[1096916140,3247270388,3240764055],[3243037119,1100811594,3239211518],[3233700383,1099539686,3239259458],[3230381993,1096713839,3247535037],[1095330858,1099423118,1094774569],[1069596664,3225385354,1098326837]]);
let mut slice=CurveSlice::<Vec3,_,super::super::CurveCoordSemantic2D>::from_coord_range(p.as_slice(),c([4,1053383808]),c([6,1054519398])).remap_2d();
slice.extend_right(f(1120403456));eq_coord(slice.start(),[4,1053383808]);eq_coord(slice.end(),[6,1065353216]);
}
{let p=points(&[[1084747998,1086586339,1100903328],[1095998162,1094281809,3183456677],[1096916140,3247270388,3240764055],[3243037119,1100811594,3239211518],[3233700383,1099539686,3239259458],[3230381993,1096713839,3247535037],[1095330858,1099423118,1094774569],[1069596664,3225385354,1098326837]]);
let mut slice=CurveSlice::<Vec3,_,super::super::CurveCoordSemantic2D>::from_coord_range(p.as_slice(),c([1,1058066821]),c([5,1052395799])).remap_2d();
slice.extend_right(f(2139095040));eq_coord(slice.start(),[1,1058066821]);eq_coord(slice.end(),[6,1065353216]);
}
{let p=points(&[[1084747998,1086586339,1100903328],[1095998162,1094281809,3183456677],[1096916140,3247270388,3240764055],[3243037119,1100811594,3239211518],[3233700383,1099539686,3239259458],[3230381993,1096713839,3247535037],[1095330858,1099423118,1094774569],[1069596664,3225385354,1098326837]]);
let mut slice=CurveSlice::<Vec3,_,super::super::CurveCoordSemantic2D>::from_coord_range(p.as_slice(),c([4,1063521843]),c([6,1041660662])).remap_2d();
assert!(std::panic::catch_unwind(std::panic::AssertUnwindSafe(||{slice.extend_right(f(3212836864))})).is_err());
}
{let p=points(&[[1084747998,1086586339,1100903328],[1095998162,1094281809,3183456677],[1096916140,3247270388,3240764055],[3243037119,1100811594,3239211518],[3233700383,1099539686,3239259458],[3230381993,1096713839,3247535037],[1095330858,1099423118,1094774569],[1069596664,3225385354,1098326837]]);
let mut slice=CurveSlice::<Vec3,_,super::super::CurveCoordSemantic2D>::from_coord_range(p.as_slice(),c([4,1056387541]),c([5,1034931310])).remap_2d();
assert!(std::panic::catch_unwind(std::panic::AssertUnwindSafe(||{slice.extend_right(f(2143289344))})).is_err());
}
{let p=points(&[[1082977368,3221932557,3247282740],[1091417814,1097176541,1068208583],[1100111636,1075139201,1093142526],[3213382931,3214971046,1099242834],[3242840851,3239170174,1095797098],[1095376411,3240195434,3244981559],[1099755233,1095767615,3246438629]]);
let mut slice=CurveSlice::<Vec3,_,super::super::CurveCoordSemantic2D>::from_coord_range(p.as_slice(),c([4,1046452280]),c([5,1048714024])).remap_2d();
slice.extend_right(f(0));eq_coord(slice.start(),[4,1046452280]);eq_coord(slice.end(),[5,1048714024]);
}
{let p=points(&[[1082977368,3221932557,3247282740],[1091417814,1097176541,1068208583],[1100111636,1075139201,1093142526],[3213382931,3214971046,1099242834],[3242840851,3239170174,1095797098],[1095376411,3240195434,3244981559],[1099755233,1095767615,3246438629]]);
let mut slice=CurveSlice::<Vec3,_,super::super::CurveCoordSemantic2D>::from_coord_range(p.as_slice(),c([0,1048096147]),c([1,1062554929])).remap_2d();
slice.extend_right(f(1008981770));eq_coord(slice.start(),[0,1048096147]);eq_coord(slice.end(),[1,1062567683]);
}
{let p=points(&[[1082977368,3221932557,3247282740],[1091417814,1097176541,1068208583],[1100111636,1075139201,1093142526],[3213382931,3214971046,1099242834],[3242840851,3239170174,1095797098],[1095376411,3240195434,3244981559],[1099755233,1095767615,3246438629]]);
let mut slice=CurveSlice::<Vec3,_,super::super::CurveCoordSemantic2D>::from_coord_range(p.as_slice(),c([4,1040614798]),c([5,1057210107])).remap_2d();
slice.extend_right(f(1077936128));eq_coord(slice.start(),[4,1040614798]);eq_coord(slice.end(),[5,1065353216]);
}
{let p=points(&[[1082977368,3221932557,3247282740],[1091417814,1097176541,1068208583],[1100111636,1075139201,1093142526],[3213382931,3214971046,1099242834],[3242840851,3239170174,1095797098],[1095376411,3240195434,3244981559],[1099755233,1095767615,3246438629]]);
let mut slice=CurveSlice::<Vec3,_,super::super::CurveCoordSemantic2D>::from_coord_range(p.as_slice(),c([0,1038831616]),c([4,1058221665])).remap_2d();
slice.extend_right(f(1120403456));eq_coord(slice.start(),[0,1038831616]);eq_coord(slice.end(),[5,1065353216]);
}
{let p=points(&[[1082977368,3221932557,3247282740],[1091417814,1097176541,1068208583],[1100111636,1075139201,1093142526],[3213382931,3214971046,1099242834],[3242840851,3239170174,1095797098],[1095376411,3240195434,3244981559],[1099755233,1095767615,3246438629]]);
let mut slice=CurveSlice::<Vec3,_,super::super::CurveCoordSemantic2D>::from_coord_range(p.as_slice(),c([2,1055646861]),c([2,1057022660])).remap_2d();
slice.extend_right(f(2139095040));eq_coord(slice.start(),[2,1055646861]);eq_coord(slice.end(),[5,1065353216]);
}
{let p=points(&[[1082977368,3221932557,3247282740],[1091417814,1097176541,1068208583],[1100111636,1075139201,1093142526],[3213382931,3214971046,1099242834],[3242840851,3239170174,1095797098],[1095376411,3240195434,3244981559],[1099755233,1095767615,3246438629]]);
let mut slice=CurveSlice::<Vec3,_,super::super::CurveCoordSemantic2D>::from_coord_range(p.as_slice(),c([1,1034505102]),c([2,1059550150])).remap_2d();
assert!(std::panic::catch_unwind(std::panic::AssertUnwindSafe(||{slice.extend_right(f(3212836864))})).is_err());
}
{let p=points(&[[1082977368,3221932557,3247282740],[1091417814,1097176541,1068208583],[1100111636,1075139201,1093142526],[3213382931,3214971046,1099242834],[3242840851,3239170174,1095797098],[1095376411,3240195434,3244981559],[1099755233,1095767615,3246438629]]);
let mut slice=CurveSlice::<Vec3,_,super::super::CurveCoordSemantic2D>::from_coord_range(p.as_slice(),c([5,1058156108]),c([5,1059049030])).remap_2d();
assert!(std::panic::catch_unwind(std::panic::AssertUnwindSafe(||{slice.extend_right(f(2143289344))})).is_err());
}
{let p=points(&[[3247798851,1058059480,3233609334],[3240528963,1083175319,3219678495]]);
let mut slice=CurveSlice::<Vec3,_,super::super::CurveCoordSemantic2D>::from_coord_range(p.as_slice(),c([0,1020527329]),c([0,1059815554])).remap_2d();
slice.extend_right(f(0));eq_coord(slice.start(),[0,1020527329]);eq_coord(slice.end(),[0,1059815554]);
}
{let p=points(&[[3247798851,1058059480,3233609334],[3240528963,1083175319,3219678495]]);
let mut slice=CurveSlice::<Vec3,_,super::super::CurveCoordSemantic2D>::from_coord_range(p.as_slice(),c([0,1060839683]),c([0,1060839683])).remap_2d();
slice.extend_right(f(1008981770));eq_coord(slice.start(),[0,1060839683]);eq_coord(slice.end(),[0,1060857858]);
}
{let p=points(&[[3247798851,1058059480,3233609334],[3240528963,1083175319,3219678495]]);
let mut slice=CurveSlice::<Vec3,_,super::super::CurveCoordSemantic2D>::from_coord_range(p.as_slice(),c([0,1063509257]),c([0,1063807650])).remap_2d();
slice.extend_right(f(1077936128));eq_coord(slice.start(),[0,1063509257]);eq_coord(slice.end(),[0,1065353216]);
}
{let p=points(&[[3247798851,1058059480,3233609334],[3240528963,1083175319,3219678495]]);
let mut slice=CurveSlice::<Vec3,_,super::super::CurveCoordSemantic2D>::from_coord_range(p.as_slice(),c([0,1062151069]),c([0,1062151069])).remap_2d();
slice.extend_right(f(1120403456));eq_coord(slice.start(),[0,1062151069]);eq_coord(slice.end(),[0,1065353216]);
}
{let p=points(&[[3247798851,1058059480,3233609334],[3240528963,1083175319,3219678495]]);
let mut slice=CurveSlice::<Vec3,_,super::super::CurveCoordSemantic2D>::from_coord_range(p.as_slice(),c([0,1036156616]),c([0,1062211417])).remap_2d();
slice.extend_right(f(2139095040));eq_coord(slice.start(),[0,1036156616]);eq_coord(slice.end(),[0,1065353216]);
}
{let p=points(&[[3247798851,1058059480,3233609334],[3240528963,1083175319,3219678495]]);
let mut slice=CurveSlice::<Vec3,_,super::super::CurveCoordSemantic2D>::from_coord_range(p.as_slice(),c([0,1051633972]),c([0,1064261212])).remap_2d();
assert!(std::panic::catch_unwind(std::panic::AssertUnwindSafe(||{slice.extend_right(f(3212836864))})).is_err());
}
{let p=points(&[[3247798851,1058059480,3233609334],[3240528963,1083175319,3219678495]]);
let mut slice=CurveSlice::<Vec3,_,super::super::CurveCoordSemantic2D>::from_coord_range(p.as_slice(),c([0,1049966505]),c([0,1059702277])).remap_2d();
assert!(std::panic::catch_unwind(std::panic::AssertUnwindSafe(||{slice.extend_right(f(2143289344))})).is_err());
}
{let p=points(&[[3248254451,3241936917,1099955010],[1079140333,3244934773,3213218942],[1090209441,1052470290,3230079560],[1095496662,3240322186,1095800513],[1092103392,3242769597,3226461344],[1087355470,1093338760,1090907651],[3225938952,1100032890,1099521180]]);
let mut slice=CurveSlice::<Vec3,_,super::super::CurveCoordSemantic2D>::from_coord_range(p.as_slice(),c([0,1063138650]),c([5,1057255416])).remap_2d();
slice.extend_right(f(0));eq_coord(slice.start(),[0,1063138650]);eq_coord(slice.end(),[5,1057255416]);
}
{let p=points(&[[3248254451,3241936917,1099955010],[1079140333,3244934773,3213218942],[1090209441,1052470290,3230079560],[1095496662,3240322186,1095800513],[1092103392,3242769597,3226461344],[1087355470,1093338760,1090907651],[3225938952,1100032890,1099521180]]);
let mut slice=CurveSlice::<Vec3,_,super::super::CurveCoordSemantic2D>::from_coord_range(p.as_slice(),c([0,1054400152]),c([0,1060591087])).remap_2d();
slice.extend_right(f(1008981770));eq_coord(slice.start(),[0,1054400152]);eq_coord(slice.end(),[0,1060596729]);
}
{let p=points(&[[3248254451,3241936917,1099955010],[1079140333,3244934773,3213218942],[1090209441,1052470290,3230079560],[1095496662,3240322186,1095800513],[1092103392,3242769597,3226461344],[1087355470,1093338760,1090907651],[3225938952,1100032890,1099521180]]);
let mut slice=CurveSlice::<Vec3,_,super::super::CurveCoordSemantic2D>::from_coord_range(p.as_slice(),c([1,1064160612]),c([2,1051366122])).remap_2d();
slice.extend_right(f(1077936128));eq_coord(slice.start(),[1,1064160612]);eq_coord(slice.end(),[2,1056971011]);
}
{let p=points(&[[3248254451,3241936917,1099955010],[1079140333,3244934773,3213218942],[1090209441,1052470290,3230079560],[1095496662,3240322186,1095800513],[1092103392,3242769597,3226461344],[1087355470,1093338760,1090907651],[3225938952,1100032890,1099521180]]);
let mut slice=CurveSlice::<Vec3,_,super::super::CurveCoordSemantic2D>::from_coord_range(p.as_slice(),c([1,1052601425]),c([5,1065311945])).remap_2d();
slice.extend_right(f(1120403456));eq_coord(slice.start(),[1,1052601425]);eq_coord(slice.end(),[5,1065353216]);
}
{let p=points(&[[3248254451,3241936917,1099955010],[1079140333,3244934773,3213218942],[1090209441,1052470290,3230079560],[1095496662,3240322186,1095800513],[1092103392,3242769597,3226461344],[1087355470,1093338760,1090907651],[3225938952,1100032890,1099521180]]);
let mut slice=CurveSlice::<Vec3,_,super::super::CurveCoordSemantic2D>::from_coord_range(p.as_slice(),c([3,1040871699]),c([4,1062381923])).remap_2d();
slice.extend_right(f(2139095040));eq_coord(slice.start(),[3,1040871699]);eq_coord(slice.end(),[5,1065353216]);
}
{let p=points(&[[3248254451,3241936917,1099955010],[1079140333,3244934773,3213218942],[1090209441,1052470290,3230079560],[1095496662,3240322186,1095800513],[1092103392,3242769597,3226461344],[1087355470,1093338760,1090907651],[3225938952,1100032890,1099521180]]);
let mut slice=CurveSlice::<Vec3,_,super::super::CurveCoordSemantic2D>::from_coord_range(p.as_slice(),c([4,1064771042]),c([4,1064771042])).remap_2d();
assert!(std::panic::catch_unwind(std::panic::AssertUnwindSafe(||{slice.extend_right(f(3212836864))})).is_err());
}
{let p=points(&[[3248254451,3241936917,1099955010],[1079140333,3244934773,3213218942],[1090209441,1052470290,3230079560],[1095496662,3240322186,1095800513],[1092103392,3242769597,3226461344],[1087355470,1093338760,1090907651],[3225938952,1100032890,1099521180]]);
let mut slice=CurveSlice::<Vec3,_,super::super::CurveCoordSemantic2D>::from_coord_range(p.as_slice(),c([3,1063528561]),c([3,1064554827])).remap_2d();
assert!(std::panic::catch_unwind(std::panic::AssertUnwindSafe(||{slice.extend_right(f(2143289344))})).is_err());
}
{let p=points(&[[3212759075,3245919388,3242556122],[3229372900,3233527430,3234242874],[3247490566,1099735996,3246681455],[1100591571,3246526217,1094965764],[1099566613,3242955625,999123697],[3238120409,3235777217,1097885840],[3244363478,1099288271,1088099009],[1080504870,1075892897,3239915365]]);
let mut slice=CurveSlice::<Vec3,_,super::super::CurveCoordSemantic2D>::from_coord_range(p.as_slice(),c([0,1063846742]),c([2,1059635371])).remap_2d();
slice.extend_right(f(0));eq_coord(slice.start(),[0,1063846742]);eq_coord(slice.end(),[2,1059635371]);
}
{let p=points(&[[3212759075,3245919388,3242556122],[3229372900,3233527430,3234242874],[3247490566,1099735996,3246681455],[1100591571,3246526217,1094965764],[1099566613,3242955625,999123697],[3238120409,3235777217,1097885840],[3244363478,1099288271,1088099009],[1080504870,1075892897,3239915365]]);
let mut slice=CurveSlice::<Vec3,_,super::super::CurveCoordSemantic2D>::from_coord_range(p.as_slice(),c([1,1051068030]),c([5,1044691072])).remap_2d();
slice.extend_right(f(1008981770));eq_coord(slice.start(),[1,1051068030]);eq_coord(slice.end(),[5,1044757404]);
}
{let p=points(&[[3212759075,3245919388,3242556122],[3229372900,3233527430,3234242874],[3247490566,1099735996,3246681455],[1100591571,3246526217,1094965764],[1099566613,3242955625,999123697],[3238120409,3235777217,1097885840],[3244363478,1099288271,1088099009],[1080504870,1075892897,3239915365]]);
let mut slice=CurveSlice::<Vec3,_,super::super::CurveCoordSemantic2D>::from_coord_range(p.as_slice(),c([3,1057487326]),c([4,1053266635])).remap_2d();
slice.extend_right(f(1077936128));eq_coord(slice.start(),[3,1057487326]);eq_coord(slice.end(),[4,1056680952]);
}
{let p=points(&[[3212759075,3245919388,3242556122],[3229372900,3233527430,3234242874],[3247490566,1099735996,3246681455],[1100591571,3246526217,1094965764],[1099566613,3242955625,999123697],[3238120409,3235777217,1097885840],[3244363478,1099288271,1088099009],[1080504870,1075892897,3239915365]]);
let mut slice=CurveSlice::<Vec3,_,super::super::CurveCoordSemantic2D>::from_coord_range(p.as_slice(),c([4,1049437728]),c([4,1064034951])).remap_2d();
slice.extend_right(f(1120403456));eq_coord(slice.start(),[4,1049437728]);eq_coord(slice.end(),[6,1065353216]);
}
{let p=points(&[[3212759075,3245919388,3242556122],[3229372900,3233527430,3234242874],[3247490566,1099735996,3246681455],[1100591571,3246526217,1094965764],[1099566613,3242955625,999123697],[3238120409,3235777217,1097885840],[3244363478,1099288271,1088099009],[1080504870,1075892897,3239915365]]);
let mut slice=CurveSlice::<Vec3,_,super::super::CurveCoordSemantic2D>::from_coord_range(p.as_slice(),c([6,1050560917]),c([6,1060373999])).remap_2d();
slice.extend_right(f(2139095040));eq_coord(slice.start(),[6,1050560917]);eq_coord(slice.end(),[6,1065353216]);
}
{let p=points(&[[3212759075,3245919388,3242556122],[3229372900,3233527430,3234242874],[3247490566,1099735996,3246681455],[1100591571,3246526217,1094965764],[1099566613,3242955625,999123697],[3238120409,3235777217,1097885840],[3244363478,1099288271,1088099009],[1080504870,1075892897,3239915365]]);
let mut slice=CurveSlice::<Vec3,_,super::super::CurveCoordSemantic2D>::from_coord_range(p.as_slice(),c([3,1052974462]),c([4,1063190352])).remap_2d();
assert!(std::panic::catch_unwind(std::panic::AssertUnwindSafe(||{slice.extend_right(f(3212836864))})).is_err());
}
{let p=points(&[[3212759075,3245919388,3242556122],[3229372900,3233527430,3234242874],[3247490566,1099735996,3246681455],[1100591571,3246526217,1094965764],[1099566613,3242955625,999123697],[3238120409,3235777217,1097885840],[3244363478,1099288271,1088099009],[1080504870,1075892897,3239915365]]);
let mut slice=CurveSlice::<Vec3,_,super::super::CurveCoordSemantic2D>::from_coord_range(p.as_slice(),c([3,1060295181]),c([4,1050869525])).remap_2d();
assert!(std::panic::catch_unwind(std::panic::AssertUnwindSafe(||{slice.extend_right(f(2143289344))})).is_err());
}
{let p=points(&[[1097867177,1082561744,3236069917],[1100696440,3214659718,3240360452],[1097567244,1098257180,1100860970],[3235053765,1087985259,3240223001]]);
let mut slice=CurveSlice::<Vec3,_,super::super::CurveCoordSemantic2D>::from_coord_range(p.as_slice(),c([1,1041485277]),c([2,1059661794])).remap_2d();
slice.extend_right(f(0));eq_coord(slice.start(),[1,1041485277]);eq_coord(slice.end(),[2,1059661794]);
}
{let p=points(&[[1097867177,1082561744,3236069917],[1100696440,3214659718,3240360452],[1097567244,1098257180,1100860970],[3235053765,1087985259,3240223001]]);
let mut slice=CurveSlice::<Vec3,_,super::super::CurveCoordSemantic2D>::from_coord_range(p.as_slice(),c([1,1035614137]),c([2,1059590989])).remap_2d();
slice.extend_right(f(1008981770));eq_coord(slice.start(),[1,1035614137]);eq_coord(slice.end(),[2,1059595564]);
}
{let p=points(&[[1097867177,1082561744,3236069917],[1100696440,3214659718,3240360452],[1097567244,1098257180,1100860970],[3235053765,1087985259,3240223001]]);
let mut slice=CurveSlice::<Vec3,_,super::super::CurveCoordSemantic2D>::from_coord_range(p.as_slice(),c([1,1058019125]),c([1,1058019125])).remap_2d();
slice.extend_right(f(1077936128));eq_coord(slice.start(),[1,1058019125]);eq_coord(slice.end(),[1,1059678104]);
}
{let p=points(&[[1097867177,1082561744,3236069917],[1100696440,3214659718,3240360452],[1097567244,1098257180,1100860970],[3235053765,1087985259,3240223001]]);
let mut slice=CurveSlice::<Vec3,_,super::super::CurveCoordSemantic2D>::from_coord_range(p.as_slice(),c([2,1057524557]),c([2,1058216899])).remap_2d();
slice.extend_right(f(1120403456));eq_coord(slice.start(),[2,1057524557]);eq_coord(slice.end(),[2,1065353216]);
}
{let p=points(&[[1097867177,1082561744,3236069917],[1100696440,3214659718,3240360452],[1097567244,1098257180,1100860970],[3235053765,1087985259,3240223001]]);
let mut slice=CurveSlice::<Vec3,_,super::super::CurveCoordSemantic2D>::from_coord_range(p.as_slice(),c([0,1040219985]),c([0,1063822792])).remap_2d();
slice.extend_right(f(2139095040));eq_coord(slice.start(),[0,1040219985]);eq_coord(slice.end(),[2,1065353216]);
}
{let p=points(&[[1097867177,1082561744,3236069917],[1100696440,3214659718,3240360452],[1097567244,1098257180,1100860970],[3235053765,1087985259,3240223001]]);
let mut slice=CurveSlice::<Vec3,_,super::super::CurveCoordSemantic2D>::from_coord_range(p.as_slice(),c([0,1058795674]),c([0,1058795674])).remap_2d();
assert!(std::panic::catch_unwind(std::panic::AssertUnwindSafe(||{slice.extend_right(f(3212836864))})).is_err());
}
{let p=points(&[[1097867177,1082561744,3236069917],[1100696440,3214659718,3240360452],[1097567244,1098257180,1100860970],[3235053765,1087985259,3240223001]]);
let mut slice=CurveSlice::<Vec3,_,super::super::CurveCoordSemantic2D>::from_coord_range(p.as_slice(),c([1,1058961068]),c([2,1048299781])).remap_2d();
assert!(std::panic::catch_unwind(std::panic::AssertUnwindSafe(||{slice.extend_right(f(2143289344))})).is_err());
}
{let p=points(&[[1099010029,3244580215,1083184304],[1093727935,3231322885,3243514808],[3244515978,1099562993,1098985396],[3247736614,1094079983,1099238188],[1090771178,3233238451,3247286416],[1076830784,3243246862,1087956944],[3232418479,3231129765,3226472668]]);
let mut slice=CurveSlice::<Vec3,_,super::super::CurveCoordSemantic2D>::from_coord_range(p.as_slice(),c([0,1061665031]),c([4,1056355810])).remap_2d();
slice.extend_right(f(0));eq_coord(slice.start(),[0,1061665031]);eq_coord(slice.end(),[4,1056355810]);
}
{let p=points(&[[1099010029,3244580215,1083184304],[1093727935,3231322885,3243514808],[3244515978,1099562993,1098985396],[3247736614,1094079983,1099238188],[1090771178,3233238451,3247286416],[1076830784,3243246862,1087956944],[3232418479,3231129765,3226472668]]);
let mut slice=CurveSlice::<Vec3,_,super::super::CurveCoordSemantic2D>::from_coord_range(p.as_slice(),c([1,1061039738]),c([1,1061039738])).remap_2d();
slice.extend_right(f(1008981770));eq_coord(slice.start(),[1,1061039738]);eq_coord(slice.end(),[1,1061044065]);
}
{let p=points(&[[1099010029,3244580215,1083184304],[1093727935,3231322885,3243514808],[3244515978,1099562993,1098985396],[3247736614,1094079983,1099238188],[1090771178,3233238451,3247286416],[1076830784,3243246862,1087956944],[3232418479,3231129765,3226472668]]);
let mut slice=CurveSlice::<Vec3,_,super::super::CurveCoordSemantic2D>::from_coord_range(p.as_slice(),c([4,1057828519]),c([4,1058612659])).remap_2d();
slice.extend_right(f(1077936128));eq_coord(slice.start(),[4,1057828519]);eq_coord(slice.end(),[4,1060618177]);
}
{let p=points(&[[1099010029,3244580215,1083184304],[1093727935,3231322885,3243514808],[3244515978,1099562993,1098985396],[3247736614,1094079983,1099238188],[1090771178,3233238451,3247286416],[1076830784,3243246862,1087956944],[3232418479,3231129765,3226472668]]);
let mut slice=CurveSlice::<Vec3,_,super::super::CurveCoordSemantic2D>::from_coord_range(p.as_slice(),c([2,1033436509]),c([3,1034360628])).remap_2d();
slice.extend_right(f(1120403456));eq_coord(slice.start(),[2,1033436509]);eq_coord(slice.end(),[5,1065353216]);
}
{let p=points(&[[1099010029,3244580215,1083184304],[1093727935,3231322885,3243514808],[3244515978,1099562993,1098985396],[3247736614,1094079983,1099238188],[1090771178,3233238451,3247286416],[1076830784,3243246862,1087956944],[3232418479,3231129765,3226472668]]);
let mut slice=CurveSlice::<Vec3,_,super::super::CurveCoordSemantic2D>::from_coord_range(p.as_slice(),c([5,1064867105]),c([5,1064867105])).remap_2d();
slice.extend_right(f(2139095040));eq_coord(slice.start(),[5,1064867105]);eq_coord(slice.end(),[5,1065353216]);
}
{let p=points(&[[1099010029,3244580215,1083184304],[1093727935,3231322885,3243514808],[3244515978,1099562993,1098985396],[3247736614,1094079983,1099238188],[1090771178,3233238451,3247286416],[1076830784,3243246862,1087956944],[3232418479,3231129765,3226472668]]);
let mut slice=CurveSlice::<Vec3,_,super::super::CurveCoordSemantic2D>::from_coord_range(p.as_slice(),c([4,1058382229]),c([5,1041300636])).remap_2d();
assert!(std::panic::catch_unwind(std::panic::AssertUnwindSafe(||{slice.extend_right(f(3212836864))})).is_err());
}
{let p=points(&[[1099010029,3244580215,1083184304],[1093727935,3231322885,3243514808],[3244515978,1099562993,1098985396],[3247736614,1094079983,1099238188],[1090771178,3233238451,3247286416],[1076830784,3243246862,1087956944],[3232418479,3231129765,3226472668]]);
let mut slice=CurveSlice::<Vec3,_,super::super::CurveCoordSemantic2D>::from_coord_range(p.as_slice(),c([3,1054687038]),c([5,1062368306])).remap_2d();
assert!(std::panic::catch_unwind(std::panic::AssertUnwindSafe(||{slice.extend_right(f(2143289344))})).is_err());
}
{let p=points(&[[3230146028,3235895519,3247361702],[1100544358,3247787439,1088863577],[3247088989,1100658099,3239814798],[1058223971,3231476188,3242121657],[3247695502,1097243273,1086462731],[3242328033,1097305974,1088204397],[3245446996,1095672463,3238153751]]);
let mut slice=CurveSlice::<Vec3,_,super::super::CurveCoordSemantic2D>::from_coord_range(p.as_slice(),c([0,1061309955]),c([5,1012723785])).remap_2d();
slice.extend_right(f(0));eq_coord(slice.start(),[0,1061309955]);eq_coord(slice.end(),[5,1012723776]);
}
{let p=points(&[[3230146028,3235895519,3247361702],[1100544358,3247787439,1088863577],[3247088989,1100658099,3239814798],[1058223971,3231476188,3242121657],[3247695502,1097243273,1086462731],[3242328033,1097305974,1088204397],[3245446996,1095672463,3238153751]]);
let mut slice=CurveSlice::<Vec3,_,super::super::CurveCoordSemantic2D>::from_coord_range(p.as_slice(),c([1,1060318040]),c([3,1024239022])).remap_2d();
slice.extend_right(f(1008981770));eq_coord(slice.start(),[1,1060318040]);eq_coord(slice.end(),[3,1024341424]);
}
{let p=points(&[[3230146028,3235895519,3247361702],[1100544358,3247787439,1088863577],[3247088989,1100658099,3239814798],[1058223971,3231476188,3242121657],[3247695502,1097243273,1086462731],[3242328033,1097305974,1088204397],[3245446996,1095672463,3238153751]]);
let mut slice=CurveSlice::<Vec3,_,super::super::CurveCoordSemantic2D>::from_coord_range(p.as_slice(),c([1,1052341600]),c([1,1064065762])).remap_2d();
slice.extend_right(f(1077936128));eq_coord(slice.start(),[1,1052341600]);eq_coord(slice.end(),[1,1065317921]);
}
{let p=points(&[[3230146028,3235895519,3247361702],[1100544358,3247787439,1088863577],[3247088989,1100658099,3239814798],[1058223971,3231476188,3242121657],[3247695502,1097243273,1086462731],[3242328033,1097305974,1088204397],[3245446996,1095672463,3238153751]]);
let mut slice=CurveSlice::<Vec3,_,super::super::CurveCoordSemantic2D>::from_coord_range(p.as_slice(),c([1,1008051464]),c([4,1029956003])).remap_2d();
slice.extend_right(f(1120403456));eq_coord(slice.start(),[1,1008051464]);eq_coord(slice.end(),[5,1065353216]);
}
{let p=points(&[[3230146028,3235895519,3247361702],[1100544358,3247787439,1088863577],[3247088989,1100658099,3239814798],[1058223971,3231476188,3242121657],[3247695502,1097243273,1086462731],[3242328033,1097305974,1088204397],[3245446996,1095672463,3238153751]]);
let mut slice=CurveSlice::<Vec3,_,super::super::CurveCoordSemantic2D>::from_coord_range(p.as_slice(),c([0,1063770211]),c([4,1053205383])).remap_2d();
slice.extend_right(f(2139095040));eq_coord(slice.start(),[0,1063770211]);eq_coord(slice.end(),[5,1065353216]);
}
{let p=points(&[[3230146028,3235895519,3247361702],[1100544358,3247787439,1088863577],[3247088989,1100658099,3239814798],[1058223971,3231476188,3242121657],[3247695502,1097243273,1086462731],[3242328033,1097305974,1088204397],[3245446996,1095672463,3238153751]]);
let mut slice=CurveSlice::<Vec3,_,super::super::CurveCoordSemantic2D>::from_coord_range(p.as_slice(),c([0,996646851]),c([2,1050878178])).remap_2d();
assert!(std::panic::catch_unwind(std::panic::AssertUnwindSafe(||{slice.extend_right(f(3212836864))})).is_err());
}
{let p=points(&[[3230146028,3235895519,3247361702],[1100544358,3247787439,1088863577],[3247088989,1100658099,3239814798],[1058223971,3231476188,3242121657],[3247695502,1097243273,1086462731],[3242328033,1097305974,1088204397],[3245446996,1095672463,3238153751]]);
let mut slice=CurveSlice::<Vec3,_,super::super::CurveCoordSemantic2D>::from_coord_range(p.as_slice(),c([2,1063827992]),c([5,1053068261])).remap_2d();
assert!(std::panic::catch_unwind(std::panic::AssertUnwindSafe(||{slice.extend_right(f(2143289344))})).is_err());
}
{let p=points(&[[3244590905,3197112644,3241275444],[1091470735,1078434697,3246801676]]);
let mut slice=CurveSlice::<Vec3,_,super::super::CurveCoordSemantic2D>::from_coord_range(p.as_slice(),c([0,1056365121]),c([0,1056365121])).remap_2d();
slice.extend_right(f(0));eq_coord(slice.start(),[0,1056365121]);eq_coord(slice.end(),[0,1056365120]);
}
{let p=points(&[[3244590905,3197112644,3241275444],[1091470735,1078434697,3246801676]]);
let mut slice=CurveSlice::<Vec3,_,super::super::CurveCoordSemantic2D>::from_coord_range(p.as_slice(),c([0,1051301248]),c([0,1060242232])).remap_2d();
slice.extend_right(f(1008981770));eq_coord(slice.start(),[0,1051301248]);eq_coord(slice.end(),[0,1060249260]);
}
{let p=points(&[[3244590905,3197112644,3241275444],[1091470735,1078434697,3246801676]]);
let mut slice=CurveSlice::<Vec3,_,super::super::CurveCoordSemantic2D>::from_coord_range(p.as_slice(),c([0,1064703142]),c([0,1064703142])).remap_2d();
slice.extend_right(f(1077936128));eq_coord(slice.start(),[0,1064703142]);eq_coord(slice.end(),[0,1065353216]);
}
{let p=points(&[[3244590905,3197112644,3241275444],[1091470735,1078434697,3246801676]]);
let mut slice=CurveSlice::<Vec3,_,super::super::CurveCoordSemantic2D>::from_coord_range(p.as_slice(),c([0,1065192840]),c([0,1065192840])).remap_2d();
slice.extend_right(f(1120403456));eq_coord(slice.start(),[0,1065192840]);eq_coord(slice.end(),[0,1065353216]);
}
{let p=points(&[[3244590905,3197112644,3241275444],[1091470735,1078434697,3246801676]]);
let mut slice=CurveSlice::<Vec3,_,super::super::CurveCoordSemantic2D>::from_coord_range(p.as_slice(),c([0,1050157432]),c([0,1050157432])).remap_2d();
slice.extend_right(f(2139095040));eq_coord(slice.start(),[0,1050157432]);eq_coord(slice.end(),[0,1065353216]);
}
{let p=points(&[[3244590905,3197112644,3241275444],[1091470735,1078434697,3246801676]]);
let mut slice=CurveSlice::<Vec3,_,super::super::CurveCoordSemantic2D>::from_coord_range(p.as_slice(),c([0,1044469200]),c([0,1056629552])).remap_2d();
assert!(std::panic::catch_unwind(std::panic::AssertUnwindSafe(||{slice.extend_right(f(3212836864))})).is_err());
}
{let p=points(&[[3244590905,3197112644,3241275444],[1091470735,1078434697,3246801676]]);
let mut slice=CurveSlice::<Vec3,_,super::super::CurveCoordSemantic2D>::from_coord_range(p.as_slice(),c([0,1064489189]),c([0,1064489189])).remap_2d();
assert!(std::panic::catch_unwind(std::panic::AssertUnwindSafe(||{slice.extend_right(f(2143289344))})).is_err());
}
{let p=points(&[[3240111851,3214152468,3248294313],[3227393663,1094220890,1099303283],[1096948280,1096577260,3244078806],[3247117193,3203861168,3242412098],[3214212390,1097191631,3247085682],[1090525399,3239617351,3238254111]]);
let mut slice=CurveSlice::<Vec3,_,super::super::CurveCoordSemantic2D>::from_coord_range(p.as_slice(),c([4,1062215927]),c([4,1062215927])).remap_2d();
slice.extend_right(f(0));eq_coord(slice.start(),[4,1062215927]);eq_coord(slice.end(),[4,1062215927]);
}
{let p=points(&[[3240111851,3214152468,3248294313],[3227393663,1094220890,1099303283],[1096948280,1096577260,3244078806],[3247117193,3203861168,3242412098],[3214212390,1097191631,3247085682],[1090525399,3239617351,3238254111]]);
let mut slice=CurveSlice::<Vec3,_,super::super::CurveCoordSemantic2D>::from_coord_range(p.as_slice(),c([2,1056469925]),c([2,1056479218])).remap_2d();
slice.extend_right(f(1008981770));eq_coord(slice.start(),[2,1056469925]);eq_coord(slice.end(),[2,1056489852]);
}
{let p=points(&[[3240111851,3214152468,3248294313],[3227393663,1094220890,1099303283],[1096948280,1096577260,3244078806],[3247117193,3203861168,3242412098],[3214212390,1097191631,3247085682],[1090525399,3239617351,3238254111]]);
let mut slice=CurveSlice::<Vec3,_,super::super::CurveCoordSemantic2D>::from_coord_range(p.as_slice(),c([2,1041387709]),c([4,1051397712])).remap_2d();
slice.extend_right(f(1077936128));eq_coord(slice.start(),[2,1041387709]);eq_coord(slice.end(),[4,1058080370]);
}
{let p=points(&[[3240111851,3214152468,3248294313],[3227393663,1094220890,1099303283],[1096948280,1096577260,3244078806],[3247117193,3203861168,3242412098],[3214212390,1097191631,3247085682],[1090525399,3239617351,3238254111]]);
let mut slice=CurveSlice::<Vec3,_,super::super::CurveCoordSemantic2D>::from_coord_range(p.as_slice(),c([2,1048738911]),c([2,1056116918])).remap_2d();
slice.extend_right(f(1120403456));eq_coord(slice.start(),[2,1048738911]);eq_coord(slice.end(),[4,1065353216]);
}
{let p=points(&[[3240111851,3214152468,3248294313],[3227393663,1094220890,1099303283],[1096948280,1096577260,3244078806],[3247117193,3203861168,3242412098],[3214212390,1097191631,3247085682],[1090525399,3239617351,3238254111]]);
let mut slice=CurveSlice::<Vec3,_,super::super::CurveCoordSemantic2D>::from_coord_range(p.as_slice(),c([2,1060810865]),c([3,1052029836])).remap_2d();
slice.extend_right(f(2139095040));eq_coord(slice.start(),[2,1060810865]);eq_coord(slice.end(),[4,1065353216]);
}
{let p=points(&[[3240111851,3214152468,3248294313],[3227393663,1094220890,1099303283],[1096948280,1096577260,3244078806],[3247117193,3203861168,3242412098],[3214212390,1097191631,3247085682],[1090525399,3239617351,3238254111]]);
let mut slice=CurveSlice::<Vec3,_,super::super::CurveCoordSemantic2D>::from_coord_range(p.as_slice(),c([1,1056457855]),c([2,1041642427])).remap_2d();
assert!(std::panic::catch_unwind(std::panic::AssertUnwindSafe(||{slice.extend_right(f(3212836864))})).is_err());
}
{let p=points(&[[3240111851,3214152468,3248294313],[3227393663,1094220890,1099303283],[1096948280,1096577260,3244078806],[3247117193,3203861168,3242412098],[3214212390,1097191631,3247085682],[1090525399,3239617351,3238254111]]);
let mut slice=CurveSlice::<Vec3,_,super::super::CurveCoordSemantic2D>::from_coord_range(p.as_slice(),c([0,1040341394]),c([2,1056490237])).remap_2d();
assert!(std::panic::catch_unwind(std::panic::AssertUnwindSafe(||{slice.extend_right(f(2143289344))})).is_err());
}
{let p=points(&[[3238526442,3225180439,1093488061],[1078041480,1093017581,1097536225]]);
let mut slice=CurveSlice::<Vec3,_,super::super::CurveCoordSemantic2D>::from_coord_range(p.as_slice(),c([0,1065143911]),c([0,1065143911])).remap_2d();
slice.extend_right(f(0));eq_coord(slice.start(),[0,1065143911]);eq_coord(slice.end(),[0,1065143911]);
}
{let p=points(&[[3238526442,3225180439,1093488061],[1078041480,1093017581,1097536225]]);
let mut slice=CurveSlice::<Vec3,_,super::super::CurveCoordSemantic2D>::from_coord_range(p.as_slice(),c([0,1046481387]),c([0,1063138160])).remap_2d();
slice.extend_right(f(1008981770));eq_coord(slice.start(),[0,1046481387]);eq_coord(slice.end(),[0,1063151964]);
}
{let p=points(&[[3238526442,3225180439,1093488061],[1078041480,1093017581,1097536225]]);
let mut slice=CurveSlice::<Vec3,_,super::super::CurveCoordSemantic2D>::from_coord_range(p.as_slice(),c([0,1037907222]),c([0,1055742766])).remap_2d();
slice.extend_right(f(1077936128));eq_coord(slice.start(),[0,1037907222]);eq_coord(slice.end(),[0,1060494827]);
}
{let p=points(&[[3238526442,3225180439,1093488061],[1078041480,1093017581,1097536225]]);
let mut slice=CurveSlice::<Vec3,_,super::super::CurveCoordSemantic2D>::from_coord_range(p.as_slice(),c([0,1058977572]),c([0,1059556433])).remap_2d();
slice.extend_right(f(1120403456));eq_coord(slice.start(),[0,1058977572]);eq_coord(slice.end(),[0,1065353216]);
}
{let p=points(&[[3238526442,3225180439,1093488061],[1078041480,1093017581,1097536225]]);
let mut slice=CurveSlice::<Vec3,_,super::super::CurveCoordSemantic2D>::from_coord_range(p.as_slice(),c([0,1027853176]),c([0,1051639517])).remap_2d();
slice.extend_right(f(2139095040));eq_coord(slice.start(),[0,1027853176]);eq_coord(slice.end(),[0,1065353216]);
}
{let p=points(&[[3238526442,3225180439,1093488061],[1078041480,1093017581,1097536225]]);
let mut slice=CurveSlice::<Vec3,_,super::super::CurveCoordSemantic2D>::from_coord_range(p.as_slice(),c([0,1025137815]),c([0,1039599959])).remap_2d();
assert!(std::panic::catch_unwind(std::panic::AssertUnwindSafe(||{slice.extend_right(f(3212836864))})).is_err());
}
{let p=points(&[[3238526442,3225180439,1093488061],[1078041480,1093017581,1097536225]]);
let mut slice=CurveSlice::<Vec3,_,super::super::CurveCoordSemantic2D>::from_coord_range(p.as_slice(),c([0,1051074340]),c([0,1060357837])).remap_2d();
assert!(std::panic::catch_unwind(std::panic::AssertUnwindSafe(||{slice.extend_right(f(2143289344))})).is_err());
}
{let p=points(&[[3222271324,1078079688,3243825342],[1096982494,1079568647,1099405603],[3242841411,3240632831,3245247096],[3246731317,3233967264,1096598129],[3246450232,3240616278,3242819378],[3243045751,3244783215,3236906801],[1079076129,1087281926,3247552020],[3228913317,3246544670,1085545680]]);
let mut slice=CurveSlice::<Vec3,_,super::super::CurveCoordSemantic2D>::from_coord_range(p.as_slice(),c([5,1043533705]),c([6,1010463205])).remap_2d();
slice.extend_right(f(0));eq_coord(slice.start(),[5,1043533705]);eq_coord(slice.end(),[6,1010463232]);
}
{let p=points(&[[3222271324,1078079688,3243825342],[1096982494,1079568647,1099405603],[3242841411,3240632831,3245247096],[3246731317,3233967264,1096598129],[3246450232,3240616278,3242819378],[3243045751,3244783215,3236906801],[1079076129,1087281926,3247552020],[3228913317,3246544670,1085545680]]);
let mut slice=CurveSlice::<Vec3,_,super::super::CurveCoordSemantic2D>::from_coord_range(p.as_slice(),c([4,1062155934]),c([4,1062155934])).remap_2d();
slice.extend_right(f(1008981770));eq_coord(slice.start(),[4,1062155934]);eq_coord(slice.end(),[4,1062183484]);
}
{let p=points(&[[3222271324,1078079688,3243825342],[1096982494,1079568647,1099405603],[3242841411,3240632831,3245247096],[3246731317,3233967264,1096598129],[3246450232,3240616278,3242819378],[3243045751,3244783215,3236906801],[1079076129,1087281926,3247552020],[3228913317,3246544670,1085545680]]);
let mut slice=CurveSlice::<Vec3,_,super::super::CurveCoordSemantic2D>::from_coord_range(p.as_slice(),c([4,1037739154]),c([4,1064157569])).remap_2d();
slice.extend_right(f(1077936128));eq_coord(slice.start(),[4,1037739154]);eq_coord(slice.end(),[5,1040704596]);
}
{let p=points(&[[3222271324,1078079688,3243825342],[1096982494,1079568647,1099405603],[3242841411,3240632831,3245247096],[3246731317,3233967264,1096598129],[3246450232,3240616278,3242819378],[3243045751,3244783215,3236906801],[1079076129,1087281926,3247552020],[3228913317,3246544670,1085545680]]);
let mut slice=CurveSlice::<Vec3,_,super::super::CurveCoordSemantic2D>::from_coord_range(p.as_slice(),c([1,1037557194]),c([6,1023417371])).remap_2d();
slice.extend_right(f(1120403456));eq_coord(slice.start(),[1,1037557194]);eq_coord(slice.end(),[6,1065353216]);
}
{let p=points(&[[3222271324,1078079688,3243825342],[1096982494,1079568647,1099405603],[3242841411,3240632831,3245247096],[3246731317,3233967264,1096598129],[3246450232,3240616278,3242819378],[3243045751,3244783215,3236906801],[1079076129,1087281926,3247552020],[3228913317,3246544670,1085545680]]);
let mut slice=CurveSlice::<Vec3,_,super::super::CurveCoordSemantic2D>::from_coord_range(p.as_slice(),c([5,1042751890]),c([6,1061049600])).remap_2d();
slice.extend_right(f(2139095040));eq_coord(slice.start(),[5,1042751890]);eq_coord(slice.end(),[6,1065353216]);
}
{let p=points(&[[3222271324,1078079688,3243825342],[1096982494,1079568647,1099405603],[3242841411,3240632831,3245247096],[3246731317,3233967264,1096598129],[3246450232,3240616278,3242819378],[3243045751,3244783215,3236906801],[1079076129,1087281926,3247552020],[3228913317,3246544670,1085545680]]);
let mut slice=CurveSlice::<Vec3,_,super::super::CurveCoordSemantic2D>::from_coord_range(p.as_slice(),c([5,1013366251]),c([5,1056752077])).remap_2d();
assert!(std::panic::catch_unwind(std::panic::AssertUnwindSafe(||{slice.extend_right(f(3212836864))})).is_err());
}
{let p=points(&[[3222271324,1078079688,3243825342],[1096982494,1079568647,1099405603],[3242841411,3240632831,3245247096],[3246731317,3233967264,1096598129],[3246450232,3240616278,3242819378],[3243045751,3244783215,3236906801],[1079076129,1087281926,3247552020],[3228913317,3246544670,1085545680]]);
let mut slice=CurveSlice::<Vec3,_,super::super::CurveCoordSemantic2D>::from_coord_range(p.as_slice(),c([3,1054433861]),c([4,1024448460])).remap_2d();
assert!(std::panic::catch_unwind(std::panic::AssertUnwindSafe(||{slice.extend_right(f(2143289344))})).is_err());
}
{let p=points(&[[1082813449,1096827562,3230281234],[3242052789,1095650379,1094603447],[3243721762,1079838579,1088985230],[1099182513,3216536538,3240457667],[1093553918,1078473990,3237901303],[3241797748,1083693937,1092107875],[1073653720,3247189803,3242372558],[1088451804,1083656160,1098172592]]);
let mut slice=CurveSlice::<Vec3,_,super::super::CurveCoordSemantic2D>::from_coord_range(p.as_slice(),c([3,1042673795]),c([3,1058755465])).remap_2d();
slice.extend_right(f(0));eq_coord(slice.start(),[3,1042673795]);eq_coord(slice.end(),[3,1058755465]);
}
{let p=points(&[[1082813449,1096827562,3230281234],[3242052789,1095650379,1094603447],[3243721762,1079838579,1088985230],[1099182513,3216536538,3240457667],[1093553918,1078473990,3237901303],[3241797748,1083693937,1092107875],[1073653720,3247189803,3242372558],[1088451804,1083656160,1098172592]]);
let mut slice=CurveSlice::<Vec3,_,super::super::CurveCoordSemantic2D>::from_coord_range(p.as_slice(),c([0,1051789778]),c([2,1046523671])).remap_2d();
slice.extend_right(f(1008981770));eq_coord(slice.start(),[0,1051789778]);eq_coord(slice.end(),[2,1046542972]);
}
{let p=points(&[[1082813449,1096827562,3230281234],[3242052789,1095650379,1094603447],[3243721762,1079838579,1088985230],[1099182513,3216536538,3240457667],[1093553918,1078473990,3237901303],[3241797748,1083693937,1092107875],[1073653720,3247189803,3242372558],[1088451804,1083656160,1098172592]]);
let mut slice=CurveSlice::<Vec3,_,super::super::CurveCoordSemantic2D>::from_coord_range(p.as_slice(),c([2,1063576189]),c([5,1060290725])).remap_2d();
slice.extend_right(f(1077936128));eq_coord(slice.start(),[2,1063576189]);eq_coord(slice.end(),[5,1062256841]);
}
{let p=points(&[[1082813449,1096827562,3230281234],[3242052789,1095650379,1094603447],[3243721762,1079838579,1088985230],[1099182513,3216536538,3240457667],[1093553918,1078473990,3237901303],[3241797748,1083693937,1092107875],[1073653720,3247189803,3242372558],[1088451804,1083656160,1098172592]]);
let mut slice=CurveSlice::<Vec3,_,super::super::CurveCoordSemantic2D>::from_coord_range(p.as_slice(),c([3,1060581980]),c([6,1061520476])).remap_2d();
slice.extend_right(f(1120403456));eq_coord(slice.start(),[3,1060581980]);eq_coord(slice.end(),[6,1065353216]);
}
{let p=points(&[[1082813449,1096827562,3230281234],[3242052789,1095650379,1094603447],[3243721762,1079838579,1088985230],[1099182513,3216536538,3240457667],[1093553918,1078473990,3237901303],[3241797748,1083693937,1092107875],[1073653720,3247189803,3242372558],[1088451804,1083656160,1098172592]]);
let mut slice=CurveSlice::<Vec3,_,super::super::CurveCoordSemantic2D>::from_coord_range(p.as_slice(),c([1,1059712985]),c([6,1043759859])).remap_2d();
slice.extend_right(f(2139095040));eq_coord(slice.start(),[1,1059712985]);eq_coord(slice.end(),[6,1065353216]);
}
{let p=points(&[[1082813449,1096827562,3230281234],[3242052789,1095650379,1094603447],[3243721762,1079838579,1088985230],[1099182513,3216536538,3240457667],[1093553918,1078473990,3237901303],[3241797748,1083693937,1092107875],[1073653720,3247189803,3242372558],[1088451804,1083656160,1098172592]]);
let mut slice=CurveSlice::<Vec3,_,super::super::CurveCoordSemantic2D>::from_coord_range(p.as_slice(),c([4,1047959163]),c([5,1012297820])).remap_2d();
assert!(std::panic::catch_unwind(std::panic::AssertUnwindSafe(||{slice.extend_right(f(3212836864))})).is_err());
}
{let p=points(&[[1082813449,1096827562,3230281234],[3242052789,1095650379,1094603447],[3243721762,1079838579,1088985230],[1099182513,3216536538,3240457667],[1093553918,1078473990,3237901303],[3241797748,1083693937,1092107875],[1073653720,3247189803,3242372558],[1088451804,1083656160,1098172592]]);
let mut slice=CurveSlice::<Vec3,_,super::super::CurveCoordSemantic2D>::from_coord_range(p.as_slice(),c([1,1050698303]),c([2,1065329495])).remap_2d();
assert!(std::panic::catch_unwind(std::panic::AssertUnwindSafe(||{slice.extend_right(f(2143289344))})).is_err());
}
{let p=points(&[[3240380506,1094704937,1080930960],[1064909175,3246980524,3247618111],[1096525402,1100801308,1088209986],[3242764860,3191123991,3248147672],[1098848291,1066187263,1096178632],[3244772354,1071902063,1020797699]]);
let mut slice=CurveSlice::<Vec3,_,super::super::CurveCoordSemantic2D>::from_coord_range(p.as_slice(),c([3,1050559449]),c([3,1060044147])).remap_2d();
slice.extend_right(f(0));eq_coord(slice.start(),[3,1050559449]);eq_coord(slice.end(),[3,1060044147]);
}
{let p=points(&[[3240380506,1094704937,1080930960],[1064909175,3246980524,3247618111],[1096525402,1100801308,1088209986],[3242764860,3191123991,3248147672],[1098848291,1066187263,1096178632],[3244772354,1071902063,1020797699]]);
let mut slice=CurveSlice::<Vec3,_,super::super::CurveCoordSemantic2D>::from_coord_range(p.as_slice(),c([2,1025450623]),c([3,1063246187])).remap_2d();
slice.extend_right(f(1008981770));eq_coord(slice.start(),[2,1025450623]);eq_coord(slice.end(),[3,1063250052]);
}
{let p=points(&[[3240380506,1094704937,1080930960],[1064909175,3246980524,3247618111],[1096525402,1100801308,1088209986],[3242764860,3191123991,3248147672],[1098848291,1066187263,1096178632],[3244772354,1071902063,1020797699]]);
let mut slice=CurveSlice::<Vec3,_,super::super::CurveCoordSemantic2D>::from_coord_range(p.as_slice(),c([4,1060476544]),c([4,1060476544])).remap_2d();
slice.extend_right(f(1077936128));eq_coord(slice.start(),[4,1060476544]);eq_coord(slice.end(),[4,1061992096]);
}
{let p=points(&[[3240380506,1094704937,1080930960],[1064909175,3246980524,3247618111],[1096525402,1100801308,1088209986],[3242764860,3191123991,3248147672],[1098848291,1066187263,1096178632],[3244772354,1071902063,1020797699]]);
let mut slice=CurveSlice::<Vec3,_,super::super::CurveCoordSemantic2D>::from_coord_range(p.as_slice(),c([4,1049228909]),c([4,1050895699])).remap_2d();
slice.extend_right(f(1120403456));eq_coord(slice.start(),[4,1049228909]);eq_coord(slice.end(),[4,1065353216]);
}
{let p=points(&[[3240380506,1094704937,1080930960],[1064909175,3246980524,3247618111],[1096525402,1100801308,1088209986],[3242764860,3191123991,3248147672],[1098848291,1066187263,1096178632],[3244772354,1071902063,1020797699]]);
let mut slice=CurveSlice::<Vec3,_,super::super::CurveCoordSemantic2D>::from_coord_range(p.as_slice(),c([1,1040563885]),c([1,1040563885])).remap_2d();
slice.extend_right(f(2139095040));eq_coord(slice.start(),[1,1040563885]);eq_coord(slice.end(),[4,1065353216]);
}
{let p=points(&[[3240380506,1094704937,1080930960],[1064909175,3246980524,3247618111],[1096525402,1100801308,1088209986],[3242764860,3191123991,3248147672],[1098848291,1066187263,1096178632],[3244772354,1071902063,1020797699]]);
let mut slice=CurveSlice::<Vec3,_,super::super::CurveCoordSemantic2D>::from_coord_range(p.as_slice(),c([1,1053322598]),c([2,1060396639])).remap_2d();
assert!(std::panic::catch_unwind(std::panic::AssertUnwindSafe(||{slice.extend_right(f(3212836864))})).is_err());
}
{let p=points(&[[3240380506,1094704937,1080930960],[1064909175,3246980524,3247618111],[1096525402,1100801308,1088209986],[3242764860,3191123991,3248147672],[1098848291,1066187263,1096178632],[3244772354,1071902063,1020797699]]);
let mut slice=CurveSlice::<Vec3,_,super::super::CurveCoordSemantic2D>::from_coord_range(p.as_slice(),c([3,1060102833]),c([4,1050942007])).remap_2d();
assert!(std::panic::catch_unwind(std::panic::AssertUnwindSafe(||{slice.extend_right(f(2143289344))})).is_err());
}
{let p=points(&[[3240405232,1058186416,1097096531],[1099817832,1092237199,3244041696],[3198902661,3238610697,3228428760],[3242466192,3242088876,3232658630],[3225324183,3223441038,1100750355]]);
let mut slice=CurveSlice::<Vec3,_,super::super::CurveCoordSemantic2D>::from_coord_range(p.as_slice(),c([3,1058056681]),c([3,1058056681])).remap_2d();
slice.extend_right(f(0));eq_coord(slice.start(),[3,1058056681]);eq_coord(slice.end(),[3,1058056681]);
}
{let p=points(&[[3240405232,1058186416,1097096531],[1099817832,1092237199,3244041696],[3198902661,3238610697,3228428760],[3242466192,3242088876,3232658630],[3225324183,3223441038,1100750355]]);
let mut slice=CurveSlice::<Vec3,_,super::super::CurveCoordSemantic2D>::from_coord_range(p.as_slice(),c([2,1064914953]),c([2,1064914953])).remap_2d();
slice.extend_right(f(1008981770));eq_coord(slice.start(),[2,1064914953]);eq_coord(slice.end(),[2,1064928879]);
}
{let p=points(&[[3240405232,1058186416,1097096531],[1099817832,1092237199,3244041696],[3198902661,3238610697,3228428760],[3242466192,3242088876,3232658630],[3225324183,3223441038,1100750355]]);
let mut slice=CurveSlice::<Vec3,_,super::super::CurveCoordSemantic2D>::from_coord_range(p.as_slice(),c([1,1054615255]),c([3,1026630085])).remap_2d();
slice.extend_right(f(1077936128));eq_coord(slice.start(),[1,1054615255]);eq_coord(slice.end(),[3,1042259584]);
}
{let p=points(&[[3240405232,1058186416,1097096531],[1099817832,1092237199,3244041696],[3198902661,3238610697,3228428760],[3242466192,3242088876,3232658630],[3225324183,3223441038,1100750355]]);
let mut slice=CurveSlice::<Vec3,_,super::super::CurveCoordSemantic2D>::from_coord_range(p.as_slice(),c([3,1064964831]),c([3,1064964831])).remap_2d();
slice.extend_right(f(1120403456));eq_coord(slice.start(),[3,1064964831]);eq_coord(slice.end(),[3,1065353216]);
}
{let p=points(&[[3240405232,1058186416,1097096531],[1099817832,1092237199,3244041696],[3198902661,3238610697,3228428760],[3242466192,3242088876,3232658630],[3225324183,3223441038,1100750355]]);
let mut slice=CurveSlice::<Vec3,_,super::super::CurveCoordSemantic2D>::from_coord_range(p.as_slice(),c([2,1058302147]),c([2,1058513787])).remap_2d();
slice.extend_right(f(2139095040));eq_coord(slice.start(),[2,1058302147]);eq_coord(slice.end(),[3,1065353216]);
}
{let p=points(&[[3240405232,1058186416,1097096531],[1099817832,1092237199,3244041696],[3198902661,3238610697,3228428760],[3242466192,3242088876,3232658630],[3225324183,3223441038,1100750355]]);
let mut slice=CurveSlice::<Vec3,_,super::super::CurveCoordSemantic2D>::from_coord_range(p.as_slice(),c([0,1051824776]),c([0,1051824776])).remap_2d();
assert!(std::panic::catch_unwind(std::panic::AssertUnwindSafe(||{slice.extend_right(f(3212836864))})).is_err());
}
{let p=points(&[[3240405232,1058186416,1097096531],[1099817832,1092237199,3244041696],[3198902661,3238610697,3228428760],[3242466192,3242088876,3232658630],[3225324183,3223441038,1100750355]]);
let mut slice=CurveSlice::<Vec3,_,super::super::CurveCoordSemantic2D>::from_coord_range(p.as_slice(),c([2,1060144510]),c([2,1060144510])).remap_2d();
assert!(std::panic::catch_unwind(std::panic::AssertUnwindSafe(||{slice.extend_right(f(2143289344))})).is_err());
}
{let p=points(&[[1095869065,3247728450,3238144214],[3222142358,1069576697,1097745549],[3236692308,1082213236,1084051305],[3220909042,1095564094,1094016660],[1093158551,3217191098,1073815575],[1081707205,3248005126,3225940329],[1097165234,1095995953,1087159011],[3221340487,3240047769,3246657917]]);
let mut slice=CurveSlice::<Vec3,_,super::super::CurveCoordSemantic2D>::from_coord_range(p.as_slice(),c([0,1059298973]),c([0,1059298973])).remap_2d();
slice.extend_right(f(0));eq_coord(slice.start(),[0,1059298973]);eq_coord(slice.end(),[0,1059298973]);
}
{let p=points(&[[1095869065,3247728450,3238144214],[3222142358,1069576697,1097745549],[3236692308,1082213236,1084051305],[3220909042,1095564094,1094016660],[1093158551,3217191098,1073815575],[1081707205,3248005126,3225940329],[1097165234,1095995953,1087159011],[3221340487,3240047769,3246657917]]);
let mut slice=CurveSlice::<Vec3,_,super::super::CurveCoordSemantic2D>::from_coord_range(p.as_slice(),c([3,1054609521]),c([6,1050265552])).remap_2d();
slice.extend_right(f(1008981770));eq_coord(slice.start(),[3,1054609521]);eq_coord(slice.end(),[6,1050277472]);
}
{let p=points(&[[1095869065,3247728450,3238144214],[3222142358,1069576697,1097745549],[3236692308,1082213236,1084051305],[3220909042,1095564094,1094016660],[1093158551,3217191098,1073815575],[1081707205,3248005126,3225940329],[1097165234,1095995953,1087159011],[3221340487,3240047769,3246657917]]);
let mut slice=CurveSlice::<Vec3,_,super::super::CurveCoordSemantic2D>::from_coord_range(p.as_slice(),c([6,1057338058]),c([6,1064408560])).remap_2d();
slice.extend_right(f(1077936128));eq_coord(slice.start(),[6,1057338058]);eq_coord(slice.end(),[6,1065353216]);
}
{let p=points(&[[1095869065,3247728450,3238144214],[3222142358,1069576697,1097745549],[3236692308,1082213236,1084051305],[3220909042,1095564094,1094016660],[1093158551,3217191098,1073815575],[1081707205,3248005126,3225940329],[1097165234,1095995953,1087159011],[3221340487,3240047769,3246657917]]);
let mut slice=CurveSlice::<Vec3,_,super::super::CurveCoordSemantic2D>::from_coord_range(p.as_slice(),c([6,1052207241]),c([6,1060336360])).remap_2d();
slice.extend_right(f(1120403456));eq_coord(slice.start(),[6,1052207241]);eq_coord(slice.end(),[6,1065353216]);
}
{let p=points(&[[1095869065,3247728450,3238144214],[3222142358,1069576697,1097745549],[3236692308,1082213236,1084051305],[3220909042,1095564094,1094016660],[1093158551,3217191098,1073815575],[1081707205,3248005126,3225940329],[1097165234,1095995953,1087159011],[3221340487,3240047769,3246657917]]);
let mut slice=CurveSlice::<Vec3,_,super::super::CurveCoordSemantic2D>::from_coord_range(p.as_slice(),c([3,1064343817]),c([3,1064343817])).remap_2d();
slice.extend_right(f(2139095040));eq_coord(slice.start(),[3,1064343817]);eq_coord(slice.end(),[6,1065353216]);
}
{let p=points(&[[1095869065,3247728450,3238144214],[3222142358,1069576697,1097745549],[3236692308,1082213236,1084051305],[3220909042,1095564094,1094016660],[1093158551,3217191098,1073815575],[1081707205,3248005126,3225940329],[1097165234,1095995953,1087159011],[3221340487,3240047769,3246657917]]);
let mut slice=CurveSlice::<Vec3,_,super::super::CurveCoordSemantic2D>::from_coord_range(p.as_slice(),c([6,1060562084]),c([6,1060562084])).remap_2d();
assert!(std::panic::catch_unwind(std::panic::AssertUnwindSafe(||{slice.extend_right(f(3212836864))})).is_err());
}
{let p=points(&[[1095869065,3247728450,3238144214],[3222142358,1069576697,1097745549],[3236692308,1082213236,1084051305],[3220909042,1095564094,1094016660],[1093158551,3217191098,1073815575],[1081707205,3248005126,3225940329],[1097165234,1095995953,1087159011],[3221340487,3240047769,3246657917]]);
let mut slice=CurveSlice::<Vec3,_,super::super::CurveCoordSemantic2D>::from_coord_range(p.as_slice(),c([0,1057051347]),c([2,1044189977])).remap_2d();
assert!(std::panic::catch_unwind(std::panic::AssertUnwindSafe(||{slice.extend_right(f(2143289344))})).is_err());
}
{let p=points(&[[3230740587,3248458583,1089714619],[3245563810,1095406261,3225768936]]);
let mut slice=CurveSlice::<Vec3,_,super::super::CurveCoordSemantic2D>::from_coord_range(p.as_slice(),c([0,1055363115]),c([0,1058293096])).remap_2d();
slice.extend_right(f(0));eq_coord(slice.start(),[0,1055363115]);eq_coord(slice.end(),[0,1058293096]);
}
{let p=points(&[[3230740587,3248458583,1089714619],[3245563810,1095406261,3225768936]]);
let mut slice=CurveSlice::<Vec3,_,super::super::CurveCoordSemantic2D>::from_coord_range(p.as_slice(),c([0,1038919116]),c([0,1057429073])).remap_2d();
slice.extend_right(f(1008981770));eq_coord(slice.start(),[0,1038919116]);eq_coord(slice.end(),[0,1057440174]);
}
{let p=points(&[[3230740587,3248458583,1089714619],[3245563810,1095406261,3225768936]]);
let mut slice=CurveSlice::<Vec3,_,super::super::CurveCoordSemantic2D>::from_coord_range(p.as_slice(),c([0,1032057694]),c([0,1043341357])).remap_2d();
slice.extend_right(f(1077936128));eq_coord(slice.start(),[0,1032057694]);eq_coord(slice.end(),[0,1052619274]);
}
{let p=points(&[[3230740587,3248458583,1089714619],[3245563810,1095406261,3225768936]]);
let mut slice=CurveSlice::<Vec3,_,super::super::CurveCoordSemantic2D>::from_coord_range(p.as_slice(),c([0,1029962018]),c([0,1059994990])).remap_2d();
slice.extend_right(f(1120403456));eq_coord(slice.start(),[0,1029962018]);eq_coord(slice.end(),[0,1065353216]);
}
{let p=points(&[[3230740587,3248458583,1089714619],[3245563810,1095406261,3225768936]]);
let mut slice=CurveSlice::<Vec3,_,super::super::CurveCoordSemantic2D>::from_coord_range(p.as_slice(),c([0,1045469257]),c([0,1057947612])).remap_2d();
slice.extend_right(f(2139095040));eq_coord(slice.start(),[0,1045469257]);eq_coord(slice.end(),[0,1065353216]);
}
{let p=points(&[[3230740587,3248458583,1089714619],[3245563810,1095406261,3225768936]]);
let mut slice=CurveSlice::<Vec3,_,super::super::CurveCoordSemantic2D>::from_coord_range(p.as_slice(),c([0,1036377799]),c([0,1064732436])).remap_2d();
assert!(std::panic::catch_unwind(std::panic::AssertUnwindSafe(||{slice.extend_right(f(3212836864))})).is_err());
}
{let p=points(&[[3230740587,3248458583,1089714619],[3245563810,1095406261,3225768936]]);
let mut slice=CurveSlice::<Vec3,_,super::super::CurveCoordSemantic2D>::from_coord_range(p.as_slice(),c([0,1053335636]),c([0,1059153790])).remap_2d();
assert!(std::panic::catch_unwind(std::panic::AssertUnwindSafe(||{slice.extend_right(f(2143289344))})).is_err());
}
{let p=points(&[[1100597656,3246161973,3229240955],[1100375588,1071835982,1065429120],[3247206345,3227164320,3209281283],[1072673282,1089449914,1074183584]]);
let mut slice=CurveSlice::<Vec3,_,super::super::CurveCoordSemantic2D>::from_coord_range(p.as_slice(),c([1,1046910147]),c([2,1058692233])).remap_2d();
slice.extend_right(f(0));eq_coord(slice.start(),[1,1046910147]);eq_coord(slice.end(),[2,1058692233]);
}
{let p=points(&[[1100597656,3246161973,3229240955],[1100375588,1071835982,1065429120],[3247206345,3227164320,3209281283],[1072673282,1089449914,1074183584]]);
let mut slice=CurveSlice::<Vec3,_,super::super::CurveCoordSemantic2D>::from_coord_range(p.as_slice(),c([0,1045347942]),c([1,1057126035])).remap_2d();
slice.extend_right(f(1008981770));eq_coord(slice.start(),[0,1045347942]);eq_coord(slice.end(),[1,1057130644]);
}
{let p=points(&[[1100597656,3246161973,3229240955],[1100375588,1071835982,1065429120],[3247206345,3227164320,3209281283],[1072673282,1089449914,1074183584]]);
let mut slice=CurveSlice::<Vec3,_,super::super::CurveCoordSemantic2D>::from_coord_range(p.as_slice(),c([1,1044280986]),c([2,1050688933])).remap_2d();
slice.extend_right(f(1077936128));eq_coord(slice.start(),[1,1044280986]);eq_coord(slice.end(),[2,1055813968]);
}
{let p=points(&[[1100597656,3246161973,3229240955],[1100375588,1071835982,1065429120],[3247206345,3227164320,3209281283],[1072673282,1089449914,1074183584]]);
let mut slice=CurveSlice::<Vec3,_,super::super::CurveCoordSemantic2D>::from_coord_range(p.as_slice(),c([1,1041317994]),c([2,1056555077])).remap_2d();
slice.extend_right(f(1120403456));eq_coord(slice.start(),[1,1041317994]);eq_coord(slice.end(),[2,1065353216]);
}
{let p=points(&[[1100597656,3246161973,3229240955],[1100375588,1071835982,1065429120],[3247206345,3227164320,3209281283],[1072673282,1089449914,1074183584]]);
let mut slice=CurveSlice::<Vec3,_,super::super::CurveCoordSemantic2D>::from_coord_range(p.as_slice(),c([0,1032519419]),c([2,1052272758])).remap_2d();
slice.extend_right(f(2139095040));eq_coord(slice.start(),[0,1032519419]);eq_coord(slice.end(),[2,1065353216]);
}
{let p=points(&[[1100597656,3246161973,3229240955],[1100375588,1071835982,1065429120],[3247206345,3227164320,3209281283],[1072673282,1089449914,1074183584]]);
let mut slice=CurveSlice::<Vec3,_,super::super::CurveCoordSemantic2D>::from_coord_range(p.as_slice(),c([1,1058395812]),c([2,1057321203])).remap_2d();
assert!(std::panic::catch_unwind(std::panic::AssertUnwindSafe(||{slice.extend_right(f(3212836864))})).is_err());
}
{let p=points(&[[1100597656,3246161973,3229240955],[1100375588,1071835982,1065429120],[3247206345,3227164320,3209281283],[1072673282,1089449914,1074183584]]);
let mut slice=CurveSlice::<Vec3,_,super::super::CurveCoordSemantic2D>::from_coord_range(p.as_slice(),c([1,1059233245]),c([2,1037202271])).remap_2d();
assert!(std::panic::catch_unwind(std::panic::AssertUnwindSafe(||{slice.extend_right(f(2143289344))})).is_err());
}
{let p=points(&[[3246510138,3241659323,3237462629],[3221978396,3247458044,3246701192],[3244357164,1095015870,3244035361],[1099583006,3244088859,1097357154],[1084963187,3184481463,3245721065]]);
let mut slice=CurveSlice::<Vec3,_,super::super::CurveCoordSemantic2D>::from_coord_range(p.as_slice(),c([2,1054703181]),c([3,1059674912])).remap_2d();
slice.extend_right(f(0));eq_coord(slice.start(),[2,1054703181]);eq_coord(slice.end(),[3,1059674912]);
}
{let p=points(&[[3246510138,3241659323,3237462629],[3221978396,3247458044,3246701192],[3244357164,1095015870,3244035361],[1099583006,3244088859,1097357154],[1084963187,3184481463,3245721065]]);
let mut slice=CurveSlice::<Vec3,_,super::super::CurveCoordSemantic2D>::from_coord_range(p.as_slice(),c([3,1064270038]),c([3,1064270038])).remap_2d();
slice.extend_right(f(1008981770));eq_coord(slice.start(),[3,1064270038]);eq_coord(slice.end(),[3,1064275252]);
}
{let p=points(&[[3246510138,3241659323,3237462629],[3221978396,3247458044,3246701192],[3244357164,1095015870,3244035361],[1099583006,3244088859,1097357154],[1084963187,3184481463,3245721065]]);
let mut slice=CurveSlice::<Vec3,_,super::super::CurveCoordSemantic2D>::from_coord_range(p.as_slice(),c([0,1063711762]),c([1,1051527474])).remap_2d();
slice.extend_right(f(1077936128));eq_coord(slice.start(),[0,1063711762]);eq_coord(slice.end(),[1,1058366579]);
}
{let p=points(&[[3246510138,3241659323,3237462629],[3221978396,3247458044,3246701192],[3244357164,1095015870,3244035361],[1099583006,3244088859,1097357154],[1084963187,3184481463,3245721065]]);
let mut slice=CurveSlice::<Vec3,_,super::super::CurveCoordSemantic2D>::from_coord_range(p.as_slice(),c([0,1059762829]),c([1,1051840398])).remap_2d();
slice.extend_right(f(1120403456));eq_coord(slice.start(),[0,1059762829]);eq_coord(slice.end(),[3,1065353216]);
}
{let p=points(&[[3246510138,3241659323,3237462629],[3221978396,3247458044,3246701192],[3244357164,1095015870,3244035361],[1099583006,3244088859,1097357154],[1084963187,3184481463,3245721065]]);
let mut slice=CurveSlice::<Vec3,_,super::super::CurveCoordSemantic2D>::from_coord_range(p.as_slice(),c([0,1063766016]),c([1,1059337649])).remap_2d();
slice.extend_right(f(2139095040));eq_coord(slice.start(),[0,1063766016]);eq_coord(slice.end(),[3,1065353216]);
}
{let p=points(&[[3246510138,3241659323,3237462629],[3221978396,3247458044,3246701192],[3244357164,1095015870,3244035361],[1099583006,3244088859,1097357154],[1084963187,3184481463,3245721065]]);
let mut slice=CurveSlice::<Vec3,_,super::super::CurveCoordSemantic2D>::from_coord_range(p.as_slice(),c([0,1017305020]),c([1,1049143192])).remap_2d();
assert!(std::panic::catch_unwind(std::panic::AssertUnwindSafe(||{slice.extend_right(f(3212836864))})).is_err());
}
{let p=points(&[[3246510138,3241659323,3237462629],[3221978396,3247458044,3246701192],[3244357164,1095015870,3244035361],[1099583006,3244088859,1097357154],[1084963187,3184481463,3245721065]]);
let mut slice=CurveSlice::<Vec3,_,super::super::CurveCoordSemantic2D>::from_coord_range(p.as_slice(),c([0,1048684585]),c([0,1059422665])).remap_2d();
assert!(std::panic::catch_unwind(std::panic::AssertUnwindSafe(||{slice.extend_right(f(2143289344))})).is_err());
}
{let p=points(&[[1084408924,3239877357,3242505151],[1099195345,3236083368,1047330589],[1086185979,3244311131,1094930848],[3241436293,1099889881,3232199892]]);
let mut slice=CurveSlice::<Vec3,_,super::super::CurveCoordSemantic2D>::from_coord_range(p.as_slice(),c([1,1063927249]),c([1,1063927249])).remap_2d();
slice.extend_right(f(0));eq_coord(slice.start(),[1,1063927249]);eq_coord(slice.end(),[1,1063927249]);
}
{let p=points(&[[1084408924,3239877357,3242505151],[1099195345,3236083368,1047330589],[1086185979,3244311131,1094930848],[3241436293,1099889881,3232199892]]);
let mut slice=CurveSlice::<Vec3,_,super::super::CurveCoordSemantic2D>::from_coord_range(p.as_slice(),c([0,1056923978]),c([2,1056181750])).remap_2d();
slice.extend_right(f(1008981770));eq_coord(slice.start(),[0,1056923978]);eq_coord(slice.end(),[2,1056195446]);
}
{let p=points(&[[1084408924,3239877357,3242505151],[1099195345,3236083368,1047330589],[1086185979,3244311131,1094930848],[3241436293,1099889881,3232199892]]);
let mut slice=CurveSlice::<Vec3,_,super::super::CurveCoordSemantic2D>::from_coord_range(p.as_slice(),c([2,1059294916]),c([2,1059294916])).remap_2d();
slice.extend_right(f(1077936128));eq_coord(slice.start(),[2,1059294916]);eq_coord(slice.end(),[2,1061349188]);
}
{let p=points(&[[1084408924,3239877357,3242505151],[1099195345,3236083368,1047330589],[1086185979,3244311131,1094930848],[3241436293,1099889881,3232199892]]);
let mut slice=CurveSlice::<Vec3,_,super::super::CurveCoordSemantic2D>::from_coord_range(p.as_slice(),c([1,1052113608]),c([2,1026338050])).remap_2d();
slice.extend_right(f(1120403456));eq_coord(slice.start(),[1,1052113608]);eq_coord(slice.end(),[2,1065353216]);
}
{let p=points(&[[1084408924,3239877357,3242505151],[1099195345,3236083368,1047330589],[1086185979,3244311131,1094930848],[3241436293,1099889881,3232199892]]);
let mut slice=CurveSlice::<Vec3,_,super::super::CurveCoordSemantic2D>::from_coord_range(p.as_slice(),c([1,1062598295]),c([1,1062598295])).remap_2d();
slice.extend_right(f(2139095040));eq_coord(slice.start(),[1,1062598295]);eq_coord(slice.end(),[2,1065353216]);
}
{let p=points(&[[1084408924,3239877357,3242505151],[1099195345,3236083368,1047330589],[1086185979,3244311131,1094930848],[3241436293,1099889881,3232199892]]);
let mut slice=CurveSlice::<Vec3,_,super::super::CurveCoordSemantic2D>::from_coord_range(p.as_slice(),c([2,1060797611]),c([2,1060797611])).remap_2d();
assert!(std::panic::catch_unwind(std::panic::AssertUnwindSafe(||{slice.extend_right(f(3212836864))})).is_err());
}
{let p=points(&[[1084408924,3239877357,3242505151],[1099195345,3236083368,1047330589],[1086185979,3244311131,1094930848],[3241436293,1099889881,3232199892]]);
let mut slice=CurveSlice::<Vec3,_,super::super::CurveCoordSemantic2D>::from_coord_range(p.as_slice(),c([2,1060765379]),c([2,1060765379])).remap_2d();
assert!(std::panic::catch_unwind(std::panic::AssertUnwindSafe(||{slice.extend_right(f(2143289344))})).is_err());
}
{let p=points(&[[3244253566,3238545491,1083992339],[3224608338,1093633606,1093754342]]);
let mut slice=CurveSlice::<Vec3,_,super::super::CurveCoordSemantic2D>::from_coord_range(p.as_slice(),c([0,1063455130]),c([0,1064680544])).remap_2d();
slice.extend_right(f(0));eq_coord(slice.start(),[0,1063455130]);eq_coord(slice.end(),[0,1064680544]);
}
{let p=points(&[[3244253566,3238545491,1083992339],[3224608338,1093633606,1093754342]]);
let mut slice=CurveSlice::<Vec3,_,super::super::CurveCoordSemantic2D>::from_coord_range(p.as_slice(),c([0,1057890591]),c([0,1065003614])).remap_2d();
slice.extend_right(f(1008981770));eq_coord(slice.start(),[0,1057890591]);eq_coord(slice.end(),[0,1065016761]);
}
{let p=points(&[[3244253566,3238545491,1083992339],[3224608338,1093633606,1093754342]]);
let mut slice=CurveSlice::<Vec3,_,super::super::CurveCoordSemantic2D>::from_coord_range(p.as_slice(),c([0,1053032666]),c([0,1057033689])).remap_2d();
slice.extend_right(f(1077936128));eq_coord(slice.start(),[0,1053032666]);eq_coord(slice.end(),[0,1060977916]);
}
{let p=points(&[[3244253566,3238545491,1083992339],[3224608338,1093633606,1093754342]]);
let mut slice=CurveSlice::<Vec3,_,super::super::CurveCoordSemantic2D>::from_coord_range(p.as_slice(),c([0,1032727024]),c([0,1048970996])).remap_2d();
slice.extend_right(f(1120403456));eq_coord(slice.start(),[0,1032727024]);eq_coord(slice.end(),[0,1065353216]);
}
{let p=points(&[[3244253566,3238545491,1083992339],[3224608338,1093633606,1093754342]]);
let mut slice=CurveSlice::<Vec3,_,super::super::CurveCoordSemantic2D>::from_coord_range(p.as_slice(),c([0,1023416997]),c([0,1064847795])).remap_2d();
slice.extend_right(f(2139095040));eq_coord(slice.start(),[0,1023416997]);eq_coord(slice.end(),[0,1065353216]);
}
{let p=points(&[[3244253566,3238545491,1083992339],[3224608338,1093633606,1093754342]]);
let mut slice=CurveSlice::<Vec3,_,super::super::CurveCoordSemantic2D>::from_coord_range(p.as_slice(),c([0,1055576798]),c([0,1055576798])).remap_2d();
assert!(std::panic::catch_unwind(std::panic::AssertUnwindSafe(||{slice.extend_right(f(3212836864))})).is_err());
}
{let p=points(&[[3244253566,3238545491,1083992339],[3224608338,1093633606,1093754342]]);
let mut slice=CurveSlice::<Vec3,_,super::super::CurveCoordSemantic2D>::from_coord_range(p.as_slice(),c([0,1043480798]),c([0,1061357799])).remap_2d();
assert!(std::panic::catch_unwind(std::panic::AssertUnwindSafe(||{slice.extend_right(f(2143289344))})).is_err());
}
{let p=points(&[[3237489998,3248309204,1088059251],[1093283022,1096235402,3239656559],[3236412957,3247351658,1097842762],[3239470823,3231809706,1095304493],[1094902171,1065889429,3246283044],[3223199735,1095789934,1094959419]]);
let mut slice=CurveSlice::<Vec3,_,super::super::CurveCoordSemantic2D>::from_coord_range(p.as_slice(),c([0,1058981954]),c([3,1052559268])).remap_2d();
slice.extend_right(f(0));eq_coord(slice.start(),[0,1058981954]);eq_coord(slice.end(),[3,1052559268]);
}
{let p=points(&[[3237489998,3248309204,1088059251],[1093283022,1096235402,3239656559],[3236412957,3247351658,1097842762],[3239470823,3231809706,1095304493],[1094902171,1065889429,3246283044],[3223199735,1095789934,1094959419]]);
let mut slice=CurveSlice::<Vec3,_,super::super::CurveCoordSemantic2D>::from_coord_range(p.as_slice(),c([4,1061923095]),c([4,1061923095])).remap_2d();
slice.extend_right(f(1008981770));eq_coord(slice.start(),[4,1061923095]);eq_coord(slice.end(),[4,1061928385]);
}
{let p=points(&[[3237489998,3248309204,1088059251],[1093283022,1096235402,3239656559],[3236412957,3247351658,1097842762],[3239470823,3231809706,1095304493],[1094902171,1065889429,3246283044],[3223199735,1095789934,1094959419]]);
let mut slice=CurveSlice::<Vec3,_,super::super::CurveCoordSemantic2D>::from_coord_range(p.as_slice(),c([3,1051673379]),c([4,1051532443])).remap_2d();
slice.extend_right(f(1077936128));eq_coord(slice.start(),[3,1051673379]);eq_coord(slice.end(),[4,1054706154]);
}
{let p=points(&[[3237489998,3248309204,1088059251],[1093283022,1096235402,3239656559],[3236412957,3247351658,1097842762],[3239470823,3231809706,1095304493],[1094902171,1065889429,3246283044],[3223199735,1095789934,1094959419]]);
let mut slice=CurveSlice::<Vec3,_,super::super::CurveCoordSemantic2D>::from_coord_range(p.as_slice(),c([1,1051920327]),c([3,1064283654])).remap_2d();
slice.extend_right(f(1120403456));eq_coord(slice.start(),[1,1051920327]);eq_coord(slice.end(),[4,1065353216]);
}
{let p=points(&[[3237489998,3248309204,1088059251],[1093283022,1096235402,3239656559],[3236412957,3247351658,1097842762],[3239470823,3231809706,1095304493],[1094902171,1065889429,3246283044],[3223199735,1095789934,1094959419]]);
let mut slice=CurveSlice::<Vec3,_,super::super::CurveCoordSemantic2D>::from_coord_range(p.as_slice(),c([0,1058199890]),c([4,1053823534])).remap_2d();
slice.extend_right(f(2139095040));eq_coord(slice.start(),[0,1058199890]);eq_coord(slice.end(),[4,1065353216]);
}
{let p=points(&[[3237489998,3248309204,1088059251],[1093283022,1096235402,3239656559],[3236412957,3247351658,1097842762],[3239470823,3231809706,1095304493],[1094902171,1065889429,3246283044],[3223199735,1095789934,1094959419]]);
let mut slice=CurveSlice::<Vec3,_,super::super::CurveCoordSemantic2D>::from_coord_range(p.as_slice(),c([4,1064900214]),c([4,1064900214])).remap_2d();
assert!(std::panic::catch_unwind(std::panic::AssertUnwindSafe(||{slice.extend_right(f(3212836864))})).is_err());
}
{let p=points(&[[3237489998,3248309204,1088059251],[1093283022,1096235402,3239656559],[3236412957,3247351658,1097842762],[3239470823,3231809706,1095304493],[1094902171,1065889429,3246283044],[3223199735,1095789934,1094959419]]);
let mut slice=CurveSlice::<Vec3,_,super::super::CurveCoordSemantic2D>::from_coord_range(p.as_slice(),c([3,1060232682]),c([3,1060352049])).remap_2d();
assert!(std::panic::catch_unwind(std::panic::AssertUnwindSafe(||{slice.extend_right(f(2143289344))})).is_err());
}
{let p=points(&[[3236778374,3247171092,1100137156],[3229026171,3245992429,1098887181],[3247277947,3239205250,1070240050],[3230812912,3241022183,3237516106],[3238044015,1094711426,1097253160],[1090553702,1099196477,1093576738],[3243024163,1100515465,1098886831]]);
let mut slice=CurveSlice::<Vec3,_,super::super::CurveCoordSemantic2D>::from_coord_range(p.as_slice(),c([0,1033448234]),c([4,1061437015])).remap_2d();
slice.extend_right(f(0));eq_coord(slice.start(),[0,1033448234]);eq_coord(slice.end(),[4,1061437015]);
}
{let p=points(&[[3236778374,3247171092,1100137156],[3229026171,3245992429,1098887181],[3247277947,3239205250,1070240050],[3230812912,3241022183,3237516106],[3238044015,1094711426,1097253160],[1090553702,1099196477,1093576738],[3243024163,1100515465,1098886831]]);
let mut slice=CurveSlice::<Vec3,_,super::super::CurveCoordSemantic2D>::from_coord_range(p.as_slice(),c([4,1065169851]),c([5,1049037867])).remap_2d();
slice.extend_right(f(1008981770));eq_coord(slice.start(),[4,1065169851]);eq_coord(slice.end(),[5,1049053528]);
}
{let p=points(&[[3236778374,3247171092,1100137156],[3229026171,3245992429,1098887181],[3247277947,3239205250,1070240050],[3230812912,3241022183,3237516106],[3238044015,1094711426,1097253160],[1090553702,1099196477,1093576738],[3243024163,1100515465,1098886831]]);
let mut slice=CurveSlice::<Vec3,_,super::super::CurveCoordSemantic2D>::from_coord_range(p.as_slice(),c([2,1038411689]),c([5,1033793163])).remap_2d();
slice.extend_right(f(1077936128));eq_coord(slice.start(),[2,1038411689]);eq_coord(slice.end(),[5,1046385380]);
}
{let p=points(&[[3236778374,3247171092,1100137156],[3229026171,3245992429,1098887181],[3247277947,3239205250,1070240050],[3230812912,3241022183,3237516106],[3238044015,1094711426,1097253160],[1090553702,1099196477,1093576738],[3243024163,1100515465,1098886831]]);
let mut slice=CurveSlice::<Vec3,_,super::super::CurveCoordSemantic2D>::from_coord_range(p.as_slice(),c([1,1056412658]),c([2,1054797738])).remap_2d();
slice.extend_right(f(1120403456));eq_coord(slice.start(),[1,1056412658]);eq_coord(slice.end(),[5,1065353216]);
}
{let p=points(&[[3236778374,3247171092,1100137156],[3229026171,3245992429,1098887181],[3247277947,3239205250,1070240050],[3230812912,3241022183,3237516106],[3238044015,1094711426,1097253160],[1090553702,1099196477,1093576738],[3243024163,1100515465,1098886831]]);
let mut slice=CurveSlice::<Vec3,_,super::super::CurveCoordSemantic2D>::from_coord_range(p.as_slice(),c([3,1055276956]),c([5,1050498816])).remap_2d();
slice.extend_right(f(2139095040));eq_coord(slice.start(),[3,1055276956]);eq_coord(slice.end(),[5,1065353216]);
}
{let p=points(&[[3236778374,3247171092,1100137156],[3229026171,3245992429,1098887181],[3247277947,3239205250,1070240050],[3230812912,3241022183,3237516106],[3238044015,1094711426,1097253160],[1090553702,1099196477,1093576738],[3243024163,1100515465,1098886831]]);
let mut slice=CurveSlice::<Vec3,_,super::super::CurveCoordSemantic2D>::from_coord_range(p.as_slice(),c([1,1058201424]),c([2,1050926963])).remap_2d();
assert!(std::panic::catch_unwind(std::panic::AssertUnwindSafe(||{slice.extend_right(f(3212836864))})).is_err());
}
{let p=points(&[[3236778374,3247171092,1100137156],[3229026171,3245992429,1098887181],[3247277947,3239205250,1070240050],[3230812912,3241022183,3237516106],[3238044015,1094711426,1097253160],[1090553702,1099196477,1093576738],[3243024163,1100515465,1098886831]]);
let mut slice=CurveSlice::<Vec3,_,super::super::CurveCoordSemantic2D>::from_coord_range(p.as_slice(),c([0,1054265537]),c([3,1059519736])).remap_2d();
assert!(std::panic::catch_unwind(std::panic::AssertUnwindSafe(||{slice.extend_right(f(2143289344))})).is_err());
}
{let p=points(&[[3241172539,3247851269,1098748307],[3247563556,1090964172,1096258752],[3248392708,1089245329,1099693570],[3237199233,1064552993,1100652604],[3240910505,1094699492,3237742908]]);
let mut slice=CurveSlice::<Vec3,_,super::super::CurveCoordSemantic2D>::from_coord_range(p.as_slice(),c([2,1028652345]),c([3,1061745055])).remap_2d();
slice.extend_right(f(0));eq_coord(slice.start(),[2,1028652345]);eq_coord(slice.end(),[3,1061745055]);
}
{let p=points(&[[3241172539,3247851269,1098748307],[3247563556,1090964172,1096258752],[3248392708,1089245329,1099693570],[3237199233,1064552993,1100652604],[3240910505,1094699492,3237742908]]);
let mut slice=CurveSlice::<Vec3,_,super::super::CurveCoordSemantic2D>::from_coord_range(p.as_slice(),c([1,1058676214]),c([1,1058676214])).remap_2d();
slice.extend_right(f(1008981770));eq_coord(slice.start(),[1,1058676214]);eq_coord(slice.end(),[1,1058715008]);
}
{let p=points(&[[3241172539,3247851269,1098748307],[3247563556,1090964172,1096258752],[3248392708,1089245329,1099693570],[3237199233,1064552993,1100652604],[3240910505,1094699492,3237742908]]);
let mut slice=CurveSlice::<Vec3,_,super::super::CurveCoordSemantic2D>::from_coord_range(p.as_slice(),c([0,1049137840]),c([1,1053733548])).remap_2d();
slice.extend_right(f(1077936128));eq_coord(slice.start(),[0,1049137840]);eq_coord(slice.end(),[2,1024186032]);
}
{let p=points(&[[3241172539,3247851269,1098748307],[3247563556,1090964172,1096258752],[3248392708,1089245329,1099693570],[3237199233,1064552993,1100652604],[3240910505,1094699492,3237742908]]);
let mut slice=CurveSlice::<Vec3,_,super::super::CurveCoordSemantic2D>::from_coord_range(p.as_slice(),c([1,1062421216]),c([3,1052557928])).remap_2d();
slice.extend_right(f(1120403456));eq_coord(slice.start(),[1,1062421216]);eq_coord(slice.end(),[3,1065353216]);
}
{let p=points(&[[3241172539,3247851269,1098748307],[3247563556,1090964172,1096258752],[3248392708,1089245329,1099693570],[3237199233,1064552993,1100652604],[3240910505,1094699492,3237742908]]);
let mut slice=CurveSlice::<Vec3,_,super::super::CurveCoordSemantic2D>::from_coord_range(p.as_slice(),c([0,1059479740]),c([0,1059479740])).remap_2d();
slice.extend_right(f(2139095040));eq_coord(slice.start(),[0,1059479740]);eq_coord(slice.end(),[3,1065353216]);
}
{let p=points(&[[3241172539,3247851269,1098748307],[3247563556,1090964172,1096258752],[3248392708,1089245329,1099693570],[3237199233,1064552993,1100652604],[3240910505,1094699492,3237742908]]);
let mut slice=CurveSlice::<Vec3,_,super::super::CurveCoordSemantic2D>::from_coord_range(p.as_slice(),c([3,1057863031]),c([3,1057863031])).remap_2d();
assert!(std::panic::catch_unwind(std::panic::AssertUnwindSafe(||{slice.extend_right(f(3212836864))})).is_err());
}
{let p=points(&[[3241172539,3247851269,1098748307],[3247563556,1090964172,1096258752],[3248392708,1089245329,1099693570],[3237199233,1064552993,1100652604],[3240910505,1094699492,3237742908]]);
let mut slice=CurveSlice::<Vec3,_,super::super::CurveCoordSemantic2D>::from_coord_range(p.as_slice(),c([2,1042128931]),c([2,1061248470])).remap_2d();
assert!(std::panic::catch_unwind(std::panic::AssertUnwindSafe(||{slice.extend_right(f(2143289344))})).is_err());
}
{let p=points(&[[3235510957,3225421292,3243546556],[1075856735,1073987888,1100993602],[3246718811,3238565377,1076541785],[1095096324,3234524412,3248202371],[1083180013,1091966328,3245405560],[1095968577,1089057561,3245848733],[1098572274,3247422157,3243298987],[1072012514,1100993691,1065988040]]);
let mut slice=CurveSlice::<Vec3,_,super::super::CurveCoordSemantic2D>::from_coord_range(p.as_slice(),c([1,1061576351]),c([3,1045588969])).remap_2d();
slice.extend_right(f(0));eq_coord(slice.start(),[1,1061576351]);eq_coord(slice.end(),[3,1045588968]);
}
{let p=points(&[[3235510957,3225421292,3243546556],[1075856735,1073987888,1100993602],[3246718811,3238565377,1076541785],[1095096324,3234524412,3248202371],[1083180013,1091966328,3245405560],[1095968577,1089057561,3245848733],[1098572274,3247422157,3243298987],[1072012514,1100993691,1065988040]]);
let mut slice=CurveSlice::<Vec3,_,super::super::CurveCoordSemantic2D>::from_coord_range(p.as_slice(),c([5,1023972502]),c([6,1063250736])).remap_2d();
slice.extend_right(f(1008981770));eq_coord(slice.start(),[5,1023972502]);eq_coord(slice.end(),[6,1063259206]);
}
{let p=points(&[[3235510957,3225421292,3243546556],[1075856735,1073987888,1100993602],[3246718811,3238565377,1076541785],[1095096324,3234524412,3248202371],[1083180013,1091966328,3245405560],[1095968577,1089057561,3245848733],[1098572274,3247422157,3243298987],[1072012514,1100993691,1065988040]]);
let mut slice=CurveSlice::<Vec3,_,super::super::CurveCoordSemantic2D>::from_coord_range(p.as_slice(),c([3,1052572605]),c([4,1042393914])).remap_2d();
slice.extend_right(f(1077936128));eq_coord(slice.start(),[3,1052572605]);eq_coord(slice.end(),[4,1057005465]);
}
{let p=points(&[[3235510957,3225421292,3243546556],[1075856735,1073987888,1100993602],[3246718811,3238565377,1076541785],[1095096324,3234524412,3248202371],[1083180013,1091966328,3245405560],[1095968577,1089057561,3245848733],[1098572274,3247422157,3243298987],[1072012514,1100993691,1065988040]]);
let mut slice=CurveSlice::<Vec3,_,super::super::CurveCoordSemantic2D>::from_coord_range(p.as_slice(),c([4,1060980505]),c([5,1060759433])).remap_2d();
slice.extend_right(f(1120403456));eq_coord(slice.start(),[4,1060980505]);eq_coord(slice.end(),[6,1065353216]);
}
{let p=points(&[[3235510957,3225421292,3243546556],[1075856735,1073987888,1100993602],[3246718811,3238565377,1076541785],[1095096324,3234524412,3248202371],[1083180013,1091966328,3245405560],[1095968577,1089057561,3245848733],[1098572274,3247422157,3243298987],[1072012514,1100993691,1065988040]]);
let mut slice=CurveSlice::<Vec3,_,super::super::CurveCoordSemantic2D>::from_coord_range(p.as_slice(),c([0,1036339293]),c([3,1061619012])).remap_2d();
slice.extend_right(f(2139095040));eq_coord(slice.start(),[0,1036339293]);eq_coord(slice.end(),[6,1065353216]);
}
{let p=points(&[[3235510957,3225421292,3243546556],[1075856735,1073987888,1100993602],[3246718811,3238565377,1076541785],[1095096324,3234524412,3248202371],[1083180013,1091966328,3245405560],[1095968577,1089057561,3245848733],[1098572274,3247422157,3243298987],[1072012514,1100993691,1065988040]]);
let mut slice=CurveSlice::<Vec3,_,super::super::CurveCoordSemantic2D>::from_coord_range(p.as_slice(),c([5,1043169697]),c([5,1043169697])).remap_2d();
assert!(std::panic::catch_unwind(std::panic::AssertUnwindSafe(||{slice.extend_right(f(3212836864))})).is_err());
}
{let p=points(&[[3235510957,3225421292,3243546556],[1075856735,1073987888,1100993602],[3246718811,3238565377,1076541785],[1095096324,3234524412,3248202371],[1083180013,1091966328,3245405560],[1095968577,1089057561,3245848733],[1098572274,3247422157,3243298987],[1072012514,1100993691,1065988040]]);
let mut slice=CurveSlice::<Vec3,_,super::super::CurveCoordSemantic2D>::from_coord_range(p.as_slice(),c([1,1054017348]),c([3,1065053106])).remap_2d();
assert!(std::panic::catch_unwind(std::panic::AssertUnwindSafe(||{slice.extend_right(f(2143289344))})).is_err());
}
{let p=points(&[[3248143559,3242855462,3217433213],[1095768306,3206348635,3239179310],[1089358524,1085122942,1089088795]]);
let mut slice=CurveSlice::<Vec3,_,super::super::CurveCoordSemantic2D>::from_coord_range(p.as_slice(),c([1,1041183863]),c([1,1062252266])).remap_2d();
slice.extend_right(f(0));eq_coord(slice.start(),[1,1041183863]);eq_coord(slice.end(),[1,1062252266]);
}
{let p=points(&[[3248143559,3242855462,3217433213],[1095768306,3206348635,3239179310],[1089358524,1085122942,1089088795]]);
let mut slice=CurveSlice::<Vec3,_,super::super::CurveCoordSemantic2D>::from_coord_range(p.as_slice(),c([1,1060829105]),c([1,1060829105])).remap_2d();
slice.extend_right(f(1008981770));eq_coord(slice.start(),[1,1060829105]);eq_coord(slice.end(),[1,1060838772]);
}
{let p=points(&[[3248143559,3242855462,3217433213],[1095768306,3206348635,3239179310],[1089358524,1085122942,1089088795]]);
let mut slice=CurveSlice::<Vec3,_,super::super::CurveCoordSemantic2D>::from_coord_range(p.as_slice(),c([0,1058775197]),c([0,1058775197])).remap_2d();
slice.extend_right(f(1077936128));eq_coord(slice.start(),[0,1058775197]);eq_coord(slice.end(),[0,1060290154]);
}
{let p=points(&[[3248143559,3242855462,3217433213],[1095768306,3206348635,3239179310],[1089358524,1085122942,1089088795]]);
let mut slice=CurveSlice::<Vec3,_,super::super::CurveCoordSemantic2D>::from_coord_range(p.as_slice(),c([0,1061436990]),c([0,1061436990])).remap_2d();
slice.extend_right(f(1120403456));eq_coord(slice.start(),[0,1061436990]);eq_coord(slice.end(),[1,1065353216]);
}
{let p=points(&[[3248143559,3242855462,3217433213],[1095768306,3206348635,3239179310],[1089358524,1085122942,1089088795]]);
let mut slice=CurveSlice::<Vec3,_,super::super::CurveCoordSemantic2D>::from_coord_range(p.as_slice(),c([0,1060407176]),c([0,1060407176])).remap_2d();
slice.extend_right(f(2139095040));eq_coord(slice.start(),[0,1060407176]);eq_coord(slice.end(),[1,1065353216]);
}
{let p=points(&[[3248143559,3242855462,3217433213],[1095768306,3206348635,3239179310],[1089358524,1085122942,1089088795]]);
let mut slice=CurveSlice::<Vec3,_,super::super::CurveCoordSemantic2D>::from_coord_range(p.as_slice(),c([1,1064008289]),c([1,1064008289])).remap_2d();
assert!(std::panic::catch_unwind(std::panic::AssertUnwindSafe(||{slice.extend_right(f(3212836864))})).is_err());
}
{let p=points(&[[3248143559,3242855462,3217433213],[1095768306,3206348635,3239179310],[1089358524,1085122942,1089088795]]);
let mut slice=CurveSlice::<Vec3,_,super::super::CurveCoordSemantic2D>::from_coord_range(p.as_slice(),c([0,1059852195]),c([0,1059852195])).remap_2d();
assert!(std::panic::catch_unwind(std::panic::AssertUnwindSafe(||{slice.extend_right(f(2143289344))})).is_err());
}
{let p=points(&[[3246861589,3236099130,1096230032],[1089722329,1100558226,1097214363],[3233710308,1093554716,1091714934]]);
let mut slice=CurveSlice::<Vec3,_,super::super::CurveCoordSemantic2D>::from_coord_range(p.as_slice(),c([0,1057632207]),c([0,1057632207])).remap_2d();
slice.extend_right(f(0));eq_coord(slice.start(),[0,1057632207]);eq_coord(slice.end(),[0,1057632208]);
}
{let p=points(&[[3246861589,3236099130,1096230032],[1089722329,1100558226,1097214363],[3233710308,1093554716,1091714934]]);
let mut slice=CurveSlice::<Vec3,_,super::super::CurveCoordSemantic2D>::from_coord_range(p.as_slice(),c([1,1047279830]),c([1,1058161902])).remap_2d();
slice.extend_right(f(1008981770));eq_coord(slice.start(),[1,1047279830]);eq_coord(slice.end(),[1,1058173432]);
}
{let p=points(&[[3246861589,3236099130,1096230032],[1089722329,1100558226,1097214363],[3233710308,1093554716,1091714934]]);
let mut slice=CurveSlice::<Vec3,_,super::super::CurveCoordSemantic2D>::from_coord_range(p.as_slice(),c([0,1051437168]),c([1,1055990931])).remap_2d();
slice.extend_right(f(1077936128));eq_coord(slice.start(),[0,1051437168]);eq_coord(slice.end(),[1,1059936668]);
}
{let p=points(&[[3246861589,3236099130,1096230032],[1089722329,1100558226,1097214363],[3233710308,1093554716,1091714934]]);
let mut slice=CurveSlice::<Vec3,_,super::super::CurveCoordSemantic2D>::from_coord_range(p.as_slice(),c([0,1062744138]),c([1,1065189033])).remap_2d();
slice.extend_right(f(1120403456));eq_coord(slice.start(),[0,1062744138]);eq_coord(slice.end(),[1,1065353216]);
}
{let p=points(&[[3246861589,3236099130,1096230032],[1089722329,1100558226,1097214363],[3233710308,1093554716,1091714934]]);
let mut slice=CurveSlice::<Vec3,_,super::super::CurveCoordSemantic2D>::from_coord_range(p.as_slice(),c([0,1048927972]),c([1,1052325263])).remap_2d();
slice.extend_right(f(2139095040));eq_coord(slice.start(),[0,1048927972]);eq_coord(slice.end(),[1,1065353216]);
}
{let p=points(&[[3246861589,3236099130,1096230032],[1089722329,1100558226,1097214363],[3233710308,1093554716,1091714934]]);
let mut slice=CurveSlice::<Vec3,_,super::super::CurveCoordSemantic2D>::from_coord_range(p.as_slice(),c([0,1045895224]),c([1,1061155549])).remap_2d();
assert!(std::panic::catch_unwind(std::panic::AssertUnwindSafe(||{slice.extend_right(f(3212836864))})).is_err());
}
{let p=points(&[[3246861589,3236099130,1096230032],[1089722329,1100558226,1097214363],[3233710308,1093554716,1091714934]]);
let mut slice=CurveSlice::<Vec3,_,super::super::CurveCoordSemantic2D>::from_coord_range(p.as_slice(),c([1,1047576457]),c([1,1059836522])).remap_2d();
assert!(std::panic::catch_unwind(std::panic::AssertUnwindSafe(||{slice.extend_right(f(2143289344))})).is_err());
}
{let p=points(&[[3233875629,1100439890,3221750416],[3244641832,3219102097,1099434485],[3239294463,1083348569,1091908989],[1066463965,3247003563,3243820551],[1086267014,1086798572,3247207196],[3229297017,3237025652,3224474684],[3242945823,1099496294,3226449646]]);
let mut slice=CurveSlice::<Vec3,_,super::super::CurveCoordSemantic2D>::from_coord_range(p.as_slice(),c([1,1051240018]),c([2,1024272582])).remap_2d();
slice.extend_right(f(0));eq_coord(slice.start(),[1,1051240018]);eq_coord(slice.end(),[2,1024272576]);
}
{let p=points(&[[3233875629,1100439890,3221750416],[3244641832,3219102097,1099434485],[3239294463,1083348569,1091908989],[1066463965,3247003563,3243820551],[1086267014,1086798572,3247207196],[3229297017,3237025652,3224474684],[3242945823,1099496294,3226449646]]);
let mut slice=CurveSlice::<Vec3,_,super::super::CurveCoordSemantic2D>::from_coord_range(p.as_slice(),c([4,1049632768]),c([5,1064530003])).remap_2d();
slice.extend_right(f(1008981770));eq_coord(slice.start(),[4,1049632768]);eq_coord(slice.end(),[5,1064549063]);
}
{let p=points(&[[3233875629,1100439890,3221750416],[3244641832,3219102097,1099434485],[3239294463,1083348569,1091908989],[1066463965,3247003563,3243820551],[1086267014,1086798572,3247207196],[3229297017,3237025652,3224474684],[3242945823,1099496294,3226449646]]);
let mut slice=CurveSlice::<Vec3,_,super::super::CurveCoordSemantic2D>::from_coord_range(p.as_slice(),c([3,1055933341]),c([5,1061734541])).remap_2d();
slice.extend_right(f(1077936128));eq_coord(slice.start(),[3,1055933341]);eq_coord(slice.end(),[5,1065353216]);
}
{let p=points(&[[3233875629,1100439890,3221750416],[3244641832,3219102097,1099434485],[3239294463,1083348569,1091908989],[1066463965,3247003563,3243820551],[1086267014,1086798572,3247207196],[3229297017,3237025652,3224474684],[3242945823,1099496294,3226449646]]);
let mut slice=CurveSlice::<Vec3,_,super::super::CurveCoordSemantic2D>::from_coord_range(p.as_slice(),c([5,1065078282]),c([5,1065078282])).remap_2d();
slice.extend_right(f(1120403456));eq_coord(slice.start(),[5,1065078282]);eq_coord(slice.end(),[5,1065353216]);
}
{let p=points(&[[3233875629,1100439890,3221750416],[3244641832,3219102097,1099434485],[3239294463,1083348569,1091908989],[1066463965,3247003563,3243820551],[1086267014,1086798572,3247207196],[3229297017,3237025652,3224474684],[3242945823,1099496294,3226449646]]);
let mut slice=CurveSlice::<Vec3,_,super::super::CurveCoordSemantic2D>::from_coord_range(p.as_slice(),c([1,1035297173]),c([2,1037965466])).remap_2d();
slice.extend_right(f(2139095040));eq_coord(slice.start(),[1,1035297173]);eq_coord(slice.end(),[5,1065353216]);
}
{let p=points(&[[3233875629,1100439890,3221750416],[3244641832,3219102097,1099434485],[3239294463,1083348569,1091908989],[1066463965,3247003563,3243820551],[1086267014,1086798572,3247207196],[3229297017,3237025652,3224474684],[3242945823,1099496294,3226449646]]);
let mut slice=CurveSlice::<Vec3,_,super::super::CurveCoordSemantic2D>::from_coord_range(p.as_slice(),c([5,1035308155]),c([5,1056509170])).remap_2d();
assert!(std::panic::catch_unwind(std::panic::AssertUnwindSafe(||{slice.extend_right(f(3212836864))})).is_err());
}
{let p=points(&[[3233875629,1100439890,3221750416],[3244641832,3219102097,1099434485],[3239294463,1083348569,1091908989],[1066463965,3247003563,3243820551],[1086267014,1086798572,3247207196],[3229297017,3237025652,3224474684],[3242945823,1099496294,3226449646]]);
let mut slice=CurveSlice::<Vec3,_,super::super::CurveCoordSemantic2D>::from_coord_range(p.as_slice(),c([2,1052013776]),c([2,1064953102])).remap_2d();
assert!(std::panic::catch_unwind(std::panic::AssertUnwindSafe(||{slice.extend_right(f(2143289344))})).is_err());
}
{let p=points(&[[1094803690,1097716500,1099340436],[3240304422,1095195330,1080068889]]);
let mut slice=CurveSlice::<Vec3,_,super::super::CurveCoordSemantic2D>::from_coord_range(p.as_slice(),c([0,1062859265]),c([0,1062859265])).remap_2d();
slice.extend_right(f(0));eq_coord(slice.start(),[0,1062859265]);eq_coord(slice.end(),[0,1062859265]);
}
{let p=points(&[[1094803690,1097716500,1099340436],[3240304422,1095195330,1080068889]]);
let mut slice=CurveSlice::<Vec3,_,super::super::CurveCoordSemantic2D>::from_coord_range(p.as_slice(),c([0,1050055544]),c([0,1050055544])).remap_2d();
slice.extend_right(f(1008981770));eq_coord(slice.start(),[0,1050055544]);eq_coord(slice.end(),[0,1050068470]);
}
{let p=points(&[[1094803690,1097716500,1099340436],[3240304422,1095195330,1080068889]]);
let mut slice=CurveSlice::<Vec3,_,super::super::CurveCoordSemantic2D>::from_coord_range(p.as_slice(),c([0,1025049292]),c([0,1062420085])).remap_2d();
slice.extend_right(f(1077936128));eq_coord(slice.start(),[0,1025049292]);eq_coord(slice.end(),[0,1064359080]);
}
{let p=points(&[[1094803690,1097716500,1099340436],[3240304422,1095195330,1080068889]]);
let mut slice=CurveSlice::<Vec3,_,super::super::CurveCoordSemantic2D>::from_coord_range(p.as_slice(),c([0,1058005404]),c([0,1058005404])).remap_2d();
slice.extend_right(f(1120403456));eq_coord(slice.start(),[0,1058005404]);eq_coord(slice.end(),[0,1065353216]);
}
{let p=points(&[[1094803690,1097716500,1099340436],[3240304422,1095195330,1080068889]]);
let mut slice=CurveSlice::<Vec3,_,super::super::CurveCoordSemantic2D>::from_coord_range(p.as_slice(),c([0,1059188867]),c([0,1059188867])).remap_2d();
slice.extend_right(f(2139095040));eq_coord(slice.start(),[0,1059188867]);eq_coord(slice.end(),[0,1065353216]);
}
{let p=points(&[[1094803690,1097716500,1099340436],[3240304422,1095195330,1080068889]]);
let mut slice=CurveSlice::<Vec3,_,super::super::CurveCoordSemantic2D>::from_coord_range(p.as_slice(),c([0,1064411382]),c([0,1064411382])).remap_2d();
assert!(std::panic::catch_unwind(std::panic::AssertUnwindSafe(||{slice.extend_right(f(3212836864))})).is_err());
}
{let p=points(&[[1094803690,1097716500,1099340436],[3240304422,1095195330,1080068889]]);
let mut slice=CurveSlice::<Vec3,_,super::super::CurveCoordSemantic2D>::from_coord_range(p.as_slice(),c([0,1046743971]),c([0,1055141998])).remap_2d();
assert!(std::panic::catch_unwind(std::panic::AssertUnwindSafe(||{slice.extend_right(f(2143289344))})).is_err());
}
{let p=points(&[[0,0,0],[1077936128,1073741824,1082130432],[1086324736,1082130432,1082130432],[1086324736,1086324736,1091567616]]);
let view=LoopCurvePoints{points:p.as_slice(),range:crate::loop_range::LoopRange::new_loop(-1,4,4)};
let mut slice=CurveSlice::<Vec3,_,super::super::CurveCoordSemantic2D>::from_coord_range(view,c([1,1050253722]),c([2,1061997773])).remap_2d();
slice.extend_right(f(0));eq_coord(slice.start(),[1,1050253722]);eq_coord(slice.end(),[2,1061997773]);
}
{let p=points(&[[0,0,0],[1077936128,1073741824,1082130432],[1086324736,1082130432,1082130432],[1086324736,1086324736,1091567616]]);
let view=LoopCurvePoints{points:p.as_slice(),range:crate::loop_range::LoopRange::new_loop(-1,4,4)};
let mut slice=CurveSlice::<Vec3,_,super::super::CurveCoordSemantic2D>::from_coord_range(view,c([1,1050253722]),c([2,1061997773])).remap_2d();
slice.extend_right(f(1077936128));eq_coord(slice.start(),[1,1050253722]);eq_coord(slice.end(),[3,1056293520]);
}
{let p=points(&[[0,0,0],[1077936128,1073741824,1082130432],[1086324736,1082130432,1082130432],[1086324736,1086324736,1091567616]]);
let view=LoopCurvePoints{points:p.as_slice(),range:crate::loop_range::LoopRange::new_loop(-1,4,4)};
let mut slice=CurveSlice::<Vec3,_,super::super::CurveCoordSemantic2D>::from_coord_range(view,c([1,1050253722]),c([2,1061997773])).remap_2d();
slice.extend_right(f(1120403456));eq_coord(slice.start(),[1,1050253722]);eq_coord(slice.end(),[3,1065353216]);
}
{let p=points(&[[0,0,0],[0,1065353216,0],[1065353216,1082130432,0],[1073741824,1088421888,0]]);
let view=LoopCurvePoints{points:p.as_slice(),range:crate::loop_range::LoopRange::new_loop(-1,4,4)};
let mut slice=CurveSlice::<Vec3,_,super::super::CurveCoordSemantic2D>::from_coord_range(view,c([1,1050253722]),c([2,1061997773])).remap_2d();
slice.extend_right(f(0));eq_coord(slice.start(),[1,1050253722]);eq_coord(slice.end(),[2,1061997773]);
}
{let p=points(&[[0,0,0],[0,1065353216,0],[1065353216,1082130432,0],[1073741824,1088421888,0]]);
let view=LoopCurvePoints{points:p.as_slice(),range:crate::loop_range::LoopRange::new_loop(-1,4,4)};
let mut slice=CurveSlice::<Vec3,_,super::super::CurveCoordSemantic2D>::from_coord_range(view,c([1,1050253722]),c([2,1061997773])).remap_2d();
slice.extend_right(f(1077936128));eq_coord(slice.start(),[1,1050253722]);eq_coord(slice.end(),[3,1065353216]);
}
{let p=points(&[[0,0,0],[0,1065353216,0],[1065353216,1082130432,0],[1073741824,1088421888,0]]);
let view=LoopCurvePoints{points:p.as_slice(),range:crate::loop_range::LoopRange::new_loop(-1,4,4)};
let mut slice=CurveSlice::<Vec3,_,super::super::CurveCoordSemantic2D>::from_coord_range(view,c([1,1050253722]),c([2,1061997773])).remap_2d();
slice.extend_right(f(1120403456));eq_coord(slice.start(),[1,1050253722]);eq_coord(slice.end(),[3,1065353216]);
}
{let p=points(&[[3242262499,3208823723,1095550516],[3247643230,1082972693,1065410640],[3225834784,3229971066,1086543384]]);
let view=LoopCurvePoints{points:p.as_slice(),range:crate::loop_range::LoopRange::new_loop(-1,3,3)};
let mut slice=CurveSlice::<Vec3,_,super::super::CurveCoordSemantic2D>::from_coord_range(view,c([1,1050253722]),c([2,1061997773])).remap_2d();
slice.extend_right(f(0));eq_coord(slice.start(),[1,1050253722]);eq_coord(slice.end(),[2,1061997773]);
}
{let p=points(&[[3242262499,3208823723,1095550516],[3247643230,1082972693,1065410640],[3225834784,3229971066,1086543384]]);
let view=LoopCurvePoints{points:p.as_slice(),range:crate::loop_range::LoopRange::new_loop(-1,3,3)};
let mut slice=CurveSlice::<Vec3,_,super::super::CurveCoordSemantic2D>::from_coord_range(view,c([1,1050253722]),c([2,1061997773])).remap_2d();
slice.extend_right(f(1077936128));eq_coord(slice.start(),[1,1050253722]);eq_coord(slice.end(),[2,1065120802]);
}
{let p=points(&[[3242262499,3208823723,1095550516],[3247643230,1082972693,1065410640],[3225834784,3229971066,1086543384]]);
let view=LoopCurvePoints{points:p.as_slice(),range:crate::loop_range::LoopRange::new_loop(-1,3,3)};
let mut slice=CurveSlice::<Vec3,_,super::super::CurveCoordSemantic2D>::from_coord_range(view,c([1,1050253722]),c([2,1061997773])).remap_2d();
slice.extend_right(f(1120403456));eq_coord(slice.start(),[1,1050253722]);eq_coord(slice.end(),[2,1065353216]);
}
{let p=points(&[[1084747998,1086586339,1100903328],[1095998162,1094281809,3183456677],[1096916140,3247270388,3240764055],[3243037119,1100811594,3239211518],[3233700383,1099539686,3239259458],[3230381993,1096713839,3247535037],[1095330858,1099423118,1094774569],[1069596664,3225385354,1098326837]]);
let view=LoopCurvePoints{points:p.as_slice(),range:crate::loop_range::LoopRange::new_loop(-1,8,8)};
let mut slice=CurveSlice::<Vec3,_,super::super::CurveCoordSemantic2D>::from_coord_range(view,c([1,1050253722]),c([2,1061997773])).remap_2d();
slice.extend_right(f(0));eq_coord(slice.start(),[1,1050253722]);eq_coord(slice.end(),[2,1061997773]);
}
{let p=points(&[[1084747998,1086586339,1100903328],[1095998162,1094281809,3183456677],[1096916140,3247270388,3240764055],[3243037119,1100811594,3239211518],[3233700383,1099539686,3239259458],[3230381993,1096713839,3247535037],[1095330858,1099423118,1094774569],[1069596664,3225385354,1098326837]]);
let view=LoopCurvePoints{points:p.as_slice(),range:crate::loop_range::LoopRange::new_loop(-1,8,8)};
let mut slice=CurveSlice::<Vec3,_,super::super::CurveCoordSemantic2D>::from_coord_range(view,c([1,1050253722]),c([2,1061997773])).remap_2d();
slice.extend_right(f(1077936128));eq_coord(slice.start(),[1,1050253722]);eq_coord(slice.end(),[3,1023836736]);
}
{let p=points(&[[1084747998,1086586339,1100903328],[1095998162,1094281809,3183456677],[1096916140,3247270388,3240764055],[3243037119,1100811594,3239211518],[3233700383,1099539686,3239259458],[3230381993,1096713839,3247535037],[1095330858,1099423118,1094774569],[1069596664,3225385354,1098326837]]);
let view=LoopCurvePoints{points:p.as_slice(),range:crate::loop_range::LoopRange::new_loop(-1,8,8)};
let mut slice=CurveSlice::<Vec3,_,super::super::CurveCoordSemantic2D>::from_coord_range(view,c([1,1050253722]),c([2,1061997773])).remap_2d();
slice.extend_right(f(1120403456));eq_coord(slice.start(),[1,1050253722]);eq_coord(slice.end(),[7,1065353216]);
}
{let p=points(&[[1082977368,3221932557,3247282740],[1091417814,1097176541,1068208583],[1100111636,1075139201,1093142526],[3213382931,3214971046,1099242834],[3242840851,3239170174,1095797098],[1095376411,3240195434,3244981559],[1099755233,1095767615,3246438629]]);
let view=LoopCurvePoints{points:p.as_slice(),range:crate::loop_range::LoopRange::new_loop(-1,7,7)};
let mut slice=CurveSlice::<Vec3,_,super::super::CurveCoordSemantic2D>::from_coord_range(view,c([1,1050253722]),c([2,1061997773])).remap_2d();
slice.extend_right(f(0));eq_coord(slice.start(),[1,1050253722]);eq_coord(slice.end(),[2,1061997773]);
}
{let p=points(&[[1082977368,3221932557,3247282740],[1091417814,1097176541,1068208583],[1100111636,1075139201,1093142526],[3213382931,3214971046,1099242834],[3242840851,3239170174,1095797098],[1095376411,3240195434,3244981559],[1099755233,1095767615,3246438629]]);
let view=LoopCurvePoints{points:p.as_slice(),range:crate::loop_range::LoopRange::new_loop(-1,7,7)};
let mut slice=CurveSlice::<Vec3,_,super::super::CurveCoordSemantic2D>::from_coord_range(view,c([1,1050253722]),c([2,1061997773])).remap_2d();
slice.extend_right(f(1077936128));eq_coord(slice.start(),[1,1050253722]);eq_coord(slice.end(),[3,1016390528]);
}
{let p=points(&[[1082977368,3221932557,3247282740],[1091417814,1097176541,1068208583],[1100111636,1075139201,1093142526],[3213382931,3214971046,1099242834],[3242840851,3239170174,1095797098],[1095376411,3240195434,3244981559],[1099755233,1095767615,3246438629]]);
let view=LoopCurvePoints{points:p.as_slice(),range:crate::loop_range::LoopRange::new_loop(-1,7,7)};
let mut slice=CurveSlice::<Vec3,_,super::super::CurveCoordSemantic2D>::from_coord_range(view,c([1,1050253722]),c([2,1061997773])).remap_2d();
slice.extend_right(f(1120403456));eq_coord(slice.start(),[1,1050253722]);eq_coord(slice.end(),[6,1065353216]);
}
{let p=points(&[[3247798851,1058059480,3233609334],[3240528963,1083175319,3219678495]]);
let view=LoopCurvePoints{points:p.as_slice(),range:crate::loop_range::LoopRange::new_loop(-1,2,2)};
let mut slice=CurveSlice::<Vec3,_,super::super::CurveCoordSemantic2D>::from_coord_range(view,c([1,1050253722]),c([1,1061997773])).remap_2d();
slice.extend_right(f(0));eq_coord(slice.start(),[1,1050253722]);eq_coord(slice.end(),[1,1061997773]);
}
{let p=points(&[[3247798851,1058059480,3233609334],[3240528963,1083175319,3219678495]]);
let view=LoopCurvePoints{points:p.as_slice(),range:crate::loop_range::LoopRange::new_loop(-1,2,2)};
let mut slice=CurveSlice::<Vec3,_,super::super::CurveCoordSemantic2D>::from_coord_range(view,c([1,1050253722]),c([1,1061997773])).remap_2d();
slice.extend_right(f(1077936128));eq_coord(slice.start(),[1,1050253722]);eq_coord(slice.end(),[1,1065353216]);
}
{let p=points(&[[3247798851,1058059480,3233609334],[3240528963,1083175319,3219678495]]);
let view=LoopCurvePoints{points:p.as_slice(),range:crate::loop_range::LoopRange::new_loop(-1,2,2)};
let mut slice=CurveSlice::<Vec3,_,super::super::CurveCoordSemantic2D>::from_coord_range(view,c([1,1050253722]),c([1,1061997773])).remap_2d();
slice.extend_right(f(1120403456));eq_coord(slice.start(),[1,1050253722]);eq_coord(slice.end(),[1,1065353216]);
}
{let p=points(&[[3248254451,3241936917,1099955010],[1079140333,3244934773,3213218942],[1090209441,1052470290,3230079560],[1095496662,3240322186,1095800513],[1092103392,3242769597,3226461344],[1087355470,1093338760,1090907651],[3225938952,1100032890,1099521180]]);
let view=LoopCurvePoints{points:p.as_slice(),range:crate::loop_range::LoopRange::new_loop(-1,7,7)};
let mut slice=CurveSlice::<Vec3,_,super::super::CurveCoordSemantic2D>::from_coord_range(view,c([1,1050253722]),c([2,1061997773])).remap_2d();
slice.extend_right(f(0));eq_coord(slice.start(),[1,1050253722]);eq_coord(slice.end(),[2,1061997773]);
}
{let p=points(&[[3248254451,3241936917,1099955010],[1079140333,3244934773,3213218942],[1090209441,1052470290,3230079560],[1095496662,3240322186,1095800513],[1092103392,3242769597,3226461344],[1087355470,1093338760,1090907651],[3225938952,1100032890,1099521180]]);
let view=LoopCurvePoints{points:p.as_slice(),range:crate::loop_range::LoopRange::new_loop(-1,7,7)};
let mut slice=CurveSlice::<Vec3,_,super::super::CurveCoordSemantic2D>::from_coord_range(view,c([1,1050253722]),c([2,1061997773])).remap_2d();
slice.extend_right(f(1077936128));eq_coord(slice.start(),[1,1050253722]);eq_coord(slice.end(),[3,1037533288]);
}
{let p=points(&[[3248254451,3241936917,1099955010],[1079140333,3244934773,3213218942],[1090209441,1052470290,3230079560],[1095496662,3240322186,1095800513],[1092103392,3242769597,3226461344],[1087355470,1093338760,1090907651],[3225938952,1100032890,1099521180]]);
let view=LoopCurvePoints{points:p.as_slice(),range:crate::loop_range::LoopRange::new_loop(-1,7,7)};
let mut slice=CurveSlice::<Vec3,_,super::super::CurveCoordSemantic2D>::from_coord_range(view,c([1,1050253722]),c([2,1061997773])).remap_2d();
slice.extend_right(f(1120403456));eq_coord(slice.start(),[1,1050253722]);eq_coord(slice.end(),[6,1065353216]);
}
{let p=points(&[[3212759075,3245919388,3242556122],[3229372900,3233527430,3234242874],[3247490566,1099735996,3246681455],[1100591571,3246526217,1094965764],[1099566613,3242955625,999123697],[3238120409,3235777217,1097885840],[3244363478,1099288271,1088099009],[1080504870,1075892897,3239915365]]);
let view=LoopCurvePoints{points:p.as_slice(),range:crate::loop_range::LoopRange::new_loop(-1,8,8)};
let mut slice=CurveSlice::<Vec3,_,super::super::CurveCoordSemantic2D>::from_coord_range(view,c([1,1050253722]),c([2,1061997773])).remap_2d();
slice.extend_right(f(0));eq_coord(slice.start(),[1,1050253722]);eq_coord(slice.end(),[2,1061997773]);
}
{let p=points(&[[3212759075,3245919388,3242556122],[3229372900,3233527430,3234242874],[3247490566,1099735996,3246681455],[1100591571,3246526217,1094965764],[1099566613,3242955625,999123697],[3238120409,3235777217,1097885840],[3244363478,1099288271,1088099009],[1080504870,1075892897,3239915365]]);
let view=LoopCurvePoints{points:p.as_slice(),range:crate::loop_range::LoopRange::new_loop(-1,8,8)};
let mut slice=CurveSlice::<Vec3,_,super::super::CurveCoordSemantic2D>::from_coord_range(view,c([1,1050253722]),c([2,1061997773])).remap_2d();
slice.extend_right(f(1077936128));eq_coord(slice.start(),[1,1050253722]);eq_coord(slice.end(),[2,1064868547]);
}
{let p=points(&[[3212759075,3245919388,3242556122],[3229372900,3233527430,3234242874],[3247490566,1099735996,3246681455],[1100591571,3246526217,1094965764],[1099566613,3242955625,999123697],[3238120409,3235777217,1097885840],[3244363478,1099288271,1088099009],[1080504870,1075892897,3239915365]]);
let view=LoopCurvePoints{points:p.as_slice(),range:crate::loop_range::LoopRange::new_loop(-1,8,8)};
let mut slice=CurveSlice::<Vec3,_,super::super::CurveCoordSemantic2D>::from_coord_range(view,c([1,1050253722]),c([2,1061997773])).remap_2d();
slice.extend_right(f(1120403456));eq_coord(slice.start(),[1,1050253722]);eq_coord(slice.end(),[6,1060999232]);
}
{let p=points(&[[1097867177,1082561744,3236069917],[1100696440,3214659718,3240360452],[1097567244,1098257180,1100860970],[3235053765,1087985259,3240223001]]);
let view=LoopCurvePoints{points:p.as_slice(),range:crate::loop_range::LoopRange::new_loop(-1,4,4)};
let mut slice=CurveSlice::<Vec3,_,super::super::CurveCoordSemantic2D>::from_coord_range(view,c([1,1050253722]),c([2,1061997773])).remap_2d();
slice.extend_right(f(0));eq_coord(slice.start(),[1,1050253722]);eq_coord(slice.end(),[2,1061997773]);
}
{let p=points(&[[1097867177,1082561744,3236069917],[1100696440,3214659718,3240360452],[1097567244,1098257180,1100860970],[3235053765,1087985259,3240223001]]);
let view=LoopCurvePoints{points:p.as_slice(),range:crate::loop_range::LoopRange::new_loop(-1,4,4)};
let mut slice=CurveSlice::<Vec3,_,super::super::CurveCoordSemantic2D>::from_coord_range(view,c([1,1050253722]),c([2,1061997773])).remap_2d();
slice.extend_right(f(1077936128));eq_coord(slice.start(),[1,1050253722]);eq_coord(slice.end(),[2,1063656752]);
}
{let p=points(&[[1097867177,1082561744,3236069917],[1100696440,3214659718,3240360452],[1097567244,1098257180,1100860970],[3235053765,1087985259,3240223001]]);
let view=LoopCurvePoints{points:p.as_slice(),range:crate::loop_range::LoopRange::new_loop(-1,4,4)};
let mut slice=CurveSlice::<Vec3,_,super::super::CurveCoordSemantic2D>::from_coord_range(view,c([1,1050253722]),c([2,1061997773])).remap_2d();
slice.extend_right(f(1120403456));eq_coord(slice.start(),[1,1050253722]);eq_coord(slice.end(),[3,1065353216]);
}
{let p=points(&[[1099010029,3244580215,1083184304],[1093727935,3231322885,3243514808],[3244515978,1099562993,1098985396],[3247736614,1094079983,1099238188],[1090771178,3233238451,3247286416],[1076830784,3243246862,1087956944],[3232418479,3231129765,3226472668]]);
let view=LoopCurvePoints{points:p.as_slice(),range:crate::loop_range::LoopRange::new_loop(-1,7,7)};
let mut slice=CurveSlice::<Vec3,_,super::super::CurveCoordSemantic2D>::from_coord_range(view,c([1,1050253722]),c([2,1061997773])).remap_2d();
slice.extend_right(f(0));eq_coord(slice.start(),[1,1050253722]);eq_coord(slice.end(),[2,1061997773]);
}
{let p=points(&[[1099010029,3244580215,1083184304],[1093727935,3231322885,3243514808],[3244515978,1099562993,1098985396],[3247736614,1094079983,1099238188],[1090771178,3233238451,3247286416],[1076830784,3243246862,1087956944],[3232418479,3231129765,3226472668]]);
let view=LoopCurvePoints{points:p.as_slice(),range:crate::loop_range::LoopRange::new_loop(-1,7,7)};
let mut slice=CurveSlice::<Vec3,_,super::super::CurveCoordSemantic2D>::from_coord_range(view,c([1,1050253722]),c([2,1061997773])).remap_2d();
slice.extend_right(f(1077936128));eq_coord(slice.start(),[1,1050253722]);eq_coord(slice.end(),[2,1063295894]);
}
{let p=points(&[[1099010029,3244580215,1083184304],[1093727935,3231322885,3243514808],[3244515978,1099562993,1098985396],[3247736614,1094079983,1099238188],[1090771178,3233238451,3247286416],[1076830784,3243246862,1087956944],[3232418479,3231129765,3226472668]]);
let view=LoopCurvePoints{points:p.as_slice(),range:crate::loop_range::LoopRange::new_loop(-1,7,7)};
let mut slice=CurveSlice::<Vec3,_,super::super::CurveCoordSemantic2D>::from_coord_range(view,c([1,1050253722]),c([2,1061997773])).remap_2d();
slice.extend_right(f(1120403456));eq_coord(slice.start(),[1,1050253722]);eq_coord(slice.end(),[6,1065353216]);
}
{let p=points(&[[3230146028,3235895519,3247361702],[1100544358,3247787439,1088863577],[3247088989,1100658099,3239814798],[1058223971,3231476188,3242121657],[3247695502,1097243273,1086462731],[3242328033,1097305974,1088204397],[3245446996,1095672463,3238153751]]);
let view=LoopCurvePoints{points:p.as_slice(),range:crate::loop_range::LoopRange::new_loop(-1,7,7)};
let mut slice=CurveSlice::<Vec3,_,super::super::CurveCoordSemantic2D>::from_coord_range(view,c([1,1050253722]),c([2,1061997773])).remap_2d();
slice.extend_right(f(0));eq_coord(slice.start(),[1,1050253722]);eq_coord(slice.end(),[2,1061997773]);
}
{let p=points(&[[3230146028,3235895519,3247361702],[1100544358,3247787439,1088863577],[3247088989,1100658099,3239814798],[1058223971,3231476188,3242121657],[3247695502,1097243273,1086462731],[3242328033,1097305974,1088204397],[3245446996,1095672463,3238153751]]);
let view=LoopCurvePoints{points:p.as_slice(),range:crate::loop_range::LoopRange::new_loop(-1,7,7)};
let mut slice=CurveSlice::<Vec3,_,super::super::CurveCoordSemantic2D>::from_coord_range(view,c([1,1050253722]),c([2,1061997773])).remap_2d();
slice.extend_right(f(1077936128));eq_coord(slice.start(),[1,1050253722]);eq_coord(slice.end(),[2,1063249932]);
}
{let p=points(&[[3230146028,3235895519,3247361702],[1100544358,3247787439,1088863577],[3247088989,1100658099,3239814798],[1058223971,3231476188,3242121657],[3247695502,1097243273,1086462731],[3242328033,1097305974,1088204397],[3245446996,1095672463,3238153751]]);
let view=LoopCurvePoints{points:p.as_slice(),range:crate::loop_range::LoopRange::new_loop(-1,7,7)};
let mut slice=CurveSlice::<Vec3,_,super::super::CurveCoordSemantic2D>::from_coord_range(view,c([1,1050253722]),c([2,1061997773])).remap_2d();
slice.extend_right(f(1120403456));eq_coord(slice.start(),[1,1050253722]);eq_coord(slice.end(),[6,1065353216]);
}
{let p=points(&[[3244590905,3197112644,3241275444],[1091470735,1078434697,3246801676]]);
let view=LoopCurvePoints{points:p.as_slice(),range:crate::loop_range::LoopRange::new_loop(-1,2,2)};
let mut slice=CurveSlice::<Vec3,_,super::super::CurveCoordSemantic2D>::from_coord_range(view,c([1,1050253722]),c([1,1061997773])).remap_2d();
slice.extend_right(f(0));eq_coord(slice.start(),[1,1050253722]);eq_coord(slice.end(),[1,1061997773]);
}
{let p=points(&[[3244590905,3197112644,3241275444],[1091470735,1078434697,3246801676]]);
let view=LoopCurvePoints{points:p.as_slice(),range:crate::loop_range::LoopRange::new_loop(-1,2,2)};
let mut slice=CurveSlice::<Vec3,_,super::super::CurveCoordSemantic2D>::from_coord_range(view,c([1,1050253722]),c([1,1061997773])).remap_2d();
slice.extend_right(f(1077936128));eq_coord(slice.start(),[1,1050253722]);eq_coord(slice.end(),[1,1064106196]);
}
{let p=points(&[[3244590905,3197112644,3241275444],[1091470735,1078434697,3246801676]]);
let view=LoopCurvePoints{points:p.as_slice(),range:crate::loop_range::LoopRange::new_loop(-1,2,2)};
let mut slice=CurveSlice::<Vec3,_,super::super::CurveCoordSemantic2D>::from_coord_range(view,c([1,1050253722]),c([1,1061997773])).remap_2d();
slice.extend_right(f(1120403456));eq_coord(slice.start(),[1,1050253722]);eq_coord(slice.end(),[1,1065353216]);
}
}
#[test]fn original_b_slice_coord_ranges(){
{let p=[Vec3::ZERO;8];let s=CurveSlice::<Vec3,_,super::super::CurveCoordSemantic2D>::from_coord_range(p.as_slice(),c([0,0]),c([0,0]));let got=s.iter_coords().map(|p|[p.segment as u32,p.t.to_bits()]).collect::<Vec<_>>();assert_eq!(got,vec![[0,0],[0,0]]);}
{let p=[Vec3::ZERO;8];let s=CurveSlice::<Vec3,_,super::super::CurveCoordSemantic2D>::from_coord_range(p.as_slice(),c([0,0]),c([0,1065353216]));let got=s.iter_coords().map(|p|[p.segment as u32,p.t.to_bits()]).collect::<Vec<_>>();assert_eq!(got,vec![[0,0],[0,1065353216]]);}
assert!(std::panic::catch_unwind(||{CurveSlice::<Vec3,_,super::super::CurveCoordSemantic2D>::from_coord_range([Vec3::ZERO;8].as_slice(),c([0,1065353216]),c([0,0]))}).is_err());
{let p=[Vec3::ZERO;8];let s=CurveSlice::<Vec3,_,super::super::CurveCoordSemantic2D>::from_coord_range(p.as_slice(),c([0,1065353216]),c([0,1065353216]));let got=s.iter_coords().map(|p|[p.segment as u32,p.t.to_bits()]).collect::<Vec<_>>();assert_eq!(got,vec![[0,1065353216],[0,1065353216]]);}
{let p=[Vec3::ZERO;8];let s=CurveSlice::<Vec3,_,super::super::CurveCoordSemantic2D>::from_coord_range(p.as_slice(),c([0,1045220557]),c([0,1061997773]));let got=s.iter_coords().map(|p|[p.segment as u32,p.t.to_bits()]).collect::<Vec<_>>();assert_eq!(got,vec![[0,1045220557],[0,1061997773]]);}
assert!(std::panic::catch_unwind(||{CurveSlice::<Vec3,_,super::super::CurveCoordSemantic2D>::from_coord_range([Vec3::ZERO;8].as_slice(),c([0,1061997773]),c([0,1045220557]))}).is_err());
assert!(std::panic::catch_unwind(||{CurveSlice::<Vec3,_,super::super::CurveCoordSemantic2D>::from_coord_range([Vec3::ZERO;8].as_slice(),c([0,2143289344]),c([0,0]))}).is_err());
{let p=[Vec3::ZERO;8];let s=CurveSlice::<Vec3,_,super::super::CurveCoordSemantic2D>::from_coord_range(p.as_slice(),c([0,0]),c([1,0]));let got=s.iter_coords().map(|p|[p.segment as u32,p.t.to_bits()]).collect::<Vec<_>>();assert_eq!(got,vec![[0,0],[1,0]]);}
{let p=[Vec3::ZERO;8];let s=CurveSlice::<Vec3,_,super::super::CurveCoordSemantic2D>::from_coord_range(p.as_slice(),c([0,0]),c([1,1065353216]));let got=s.iter_coords().map(|p|[p.segment as u32,p.t.to_bits()]).collect::<Vec<_>>();assert_eq!(got,vec![[0,0],[1,0],[1,1065353216]]);}
{let p=[Vec3::ZERO;8];let s=CurveSlice::<Vec3,_,super::super::CurveCoordSemantic2D>::from_coord_range(p.as_slice(),c([0,1065353216]),c([1,0]));let got=s.iter_coords().map(|p|[p.segment as u32,p.t.to_bits()]).collect::<Vec<_>>();assert_eq!(got,vec![[0,1065353216],[1,0]]);}
{let p=[Vec3::ZERO;8];let s=CurveSlice::<Vec3,_,super::super::CurveCoordSemantic2D>::from_coord_range(p.as_slice(),c([0,1065353216]),c([1,1065353216]));let got=s.iter_coords().map(|p|[p.segment as u32,p.t.to_bits()]).collect::<Vec<_>>();assert_eq!(got,vec![[0,1065353216],[1,1065353216]]);}
{let p=[Vec3::ZERO;8];let s=CurveSlice::<Vec3,_,super::super::CurveCoordSemantic2D>::from_coord_range(p.as_slice(),c([0,1045220557]),c([1,1061997773]));let got=s.iter_coords().map(|p|[p.segment as u32,p.t.to_bits()]).collect::<Vec<_>>();assert_eq!(got,vec![[0,1045220557],[1,0],[1,1061997773]]);}
{let p=[Vec3::ZERO;8];let s=CurveSlice::<Vec3,_,super::super::CurveCoordSemantic2D>::from_coord_range(p.as_slice(),c([0,1061997773]),c([1,1045220557]));let got=s.iter_coords().map(|p|[p.segment as u32,p.t.to_bits()]).collect::<Vec<_>>();assert_eq!(got,vec![[0,1061997773],[1,0],[1,1045220557]]);}
assert!(std::panic::catch_unwind(||{CurveSlice::<Vec3,_,super::super::CurveCoordSemantic2D>::from_coord_range([Vec3::ZERO;8].as_slice(),c([0,2143289344]),c([1,0]))}).is_err());
{let p=[Vec3::ZERO;8];let s=CurveSlice::<Vec3,_,super::super::CurveCoordSemantic2D>::from_coord_range(p.as_slice(),c([2,0]),c([5,0]));let got=s.iter_coords().map(|p|[p.segment as u32,p.t.to_bits()]).collect::<Vec<_>>();assert_eq!(got,vec![[2,0],[3,0],[4,0],[5,0]]);}
{let p=[Vec3::ZERO;8];let s=CurveSlice::<Vec3,_,super::super::CurveCoordSemantic2D>::from_coord_range(p.as_slice(),c([2,0]),c([5,1065353216]));let got=s.iter_coords().map(|p|[p.segment as u32,p.t.to_bits()]).collect::<Vec<_>>();assert_eq!(got,vec![[2,0],[3,0],[4,0],[5,0],[5,1065353216]]);}
{let p=[Vec3::ZERO;8];let s=CurveSlice::<Vec3,_,super::super::CurveCoordSemantic2D>::from_coord_range(p.as_slice(),c([2,1065353216]),c([5,0]));let got=s.iter_coords().map(|p|[p.segment as u32,p.t.to_bits()]).collect::<Vec<_>>();assert_eq!(got,vec![[2,1065353216],[4,0],[5,0]]);}
{let p=[Vec3::ZERO;8];let s=CurveSlice::<Vec3,_,super::super::CurveCoordSemantic2D>::from_coord_range(p.as_slice(),c([2,1065353216]),c([5,1065353216]));let got=s.iter_coords().map(|p|[p.segment as u32,p.t.to_bits()]).collect::<Vec<_>>();assert_eq!(got,vec![[2,1065353216],[4,0],[5,0],[5,1065353216]]);}
{let p=[Vec3::ZERO;8];let s=CurveSlice::<Vec3,_,super::super::CurveCoordSemantic2D>::from_coord_range(p.as_slice(),c([2,1045220557]),c([5,1061997773]));let got=s.iter_coords().map(|p|[p.segment as u32,p.t.to_bits()]).collect::<Vec<_>>();assert_eq!(got,vec![[2,1045220557],[3,0],[4,0],[5,0],[5,1061997773]]);}
{let p=[Vec3::ZERO;8];let s=CurveSlice::<Vec3,_,super::super::CurveCoordSemantic2D>::from_coord_range(p.as_slice(),c([2,1061997773]),c([5,1045220557]));let got=s.iter_coords().map(|p|[p.segment as u32,p.t.to_bits()]).collect::<Vec<_>>();assert_eq!(got,vec![[2,1061997773],[3,0],[4,0],[5,0],[5,1045220557]]);}
assert!(std::panic::catch_unwind(||{CurveSlice::<Vec3,_,super::super::CurveCoordSemantic2D>::from_coord_range([Vec3::ZERO;8].as_slice(),c([2,2143289344]),c([5,0]))}).is_err());
assert!(std::panic::catch_unwind(||{CurveSlice::<Vec3,_,super::super::CurveCoordSemantic2D>::from_coord_range([Vec3::ZERO;8].as_slice(),c([3,0]),c([1,0]))}).is_err());
assert!(std::panic::catch_unwind(||{CurveSlice::<Vec3,_,super::super::CurveCoordSemantic2D>::from_coord_range([Vec3::ZERO;8].as_slice(),c([3,0]),c([1,1065353216]))}).is_err());
assert!(std::panic::catch_unwind(||{CurveSlice::<Vec3,_,super::super::CurveCoordSemantic2D>::from_coord_range([Vec3::ZERO;8].as_slice(),c([3,1065353216]),c([1,0]))}).is_err());
assert!(std::panic::catch_unwind(||{CurveSlice::<Vec3,_,super::super::CurveCoordSemantic2D>::from_coord_range([Vec3::ZERO;8].as_slice(),c([3,1065353216]),c([1,1065353216]))}).is_err());
assert!(std::panic::catch_unwind(||{CurveSlice::<Vec3,_,super::super::CurveCoordSemantic2D>::from_coord_range([Vec3::ZERO;8].as_slice(),c([3,1045220557]),c([1,1061997773]))}).is_err());
assert!(std::panic::catch_unwind(||{CurveSlice::<Vec3,_,super::super::CurveCoordSemantic2D>::from_coord_range([Vec3::ZERO;8].as_slice(),c([3,1061997773]),c([1,1045220557]))}).is_err());
assert!(std::panic::catch_unwind(||{CurveSlice::<Vec3,_,super::super::CurveCoordSemantic2D>::from_coord_range([Vec3::ZERO;8].as_slice(),c([3,2143289344]),c([1,0]))}).is_err());
}
#[test]fn original_b_slice_point_centers(){
{let p=vec![Vec3::ZERO;4];
let s=CurveSlice::<Vec3,_,super::super::CurveCoordSemantic2D>::from_point_range(p.as_slice(),0..=0);eq_coord(s.start(),[0,0]);eq_coord(s.end(),[0,0]);
}
{let p=vec![Vec3::ZERO;4];
let s=CurveSlice::<Vec3,_,super::super::CurveCoordSemantic2D>::from_point_range(p.as_slice(),1..=1);eq_coord(s.start(),[1,0]);eq_coord(s.end(),[1,0]);
}
{let p=vec![Vec3::ZERO;4];
let s=CurveSlice::<Vec3,_,super::super::CurveCoordSemantic2D>::from_point_range(p.as_slice(),2..=2);eq_coord(s.start(),[2,0]);eq_coord(s.end(),[2,0]);
}
{let p=vec![Vec3::ZERO;4];
let s=CurveSlice::<Vec3,_,super::super::CurveCoordSemantic2D>::from_point_range(p.as_slice(),3..=3);eq_coord(s.start(),[2,1065353216]);eq_coord(s.end(),[2,1065353216]);
}
{let p=vec![Vec3::ZERO;4];
assert!(std::panic::catch_unwind(||{CurveSlice::<Vec3,_,super::super::CurveCoordSemantic2D>::from_point_range(p.as_slice(),4..=4)}).is_err());
}
{let p=vec![Vec3::ZERO;4];
let s=CurveSlice::<Vec3,_,super::super::CurveCoordSemantic2D>::from_point_range(p.as_slice(),0..=0);eq_coord(s.start(),[0,0]);eq_coord(s.end(),[0,0]);
}
{let p=vec![Vec3::ZERO;4];
let s=CurveSlice::<Vec3,_,super::super::CurveCoordSemantic2D>::from_point_range(p.as_slice(),1..=1);eq_coord(s.start(),[1,0]);eq_coord(s.end(),[1,0]);
}
{let p=vec![Vec3::ZERO;4];
let s=CurveSlice::<Vec3,_,super::super::CurveCoordSemantic2D>::from_point_range(p.as_slice(),2..=2);eq_coord(s.start(),[2,0]);eq_coord(s.end(),[2,0]);
}
{let p=vec![Vec3::ZERO;4];
let s=CurveSlice::<Vec3,_,super::super::CurveCoordSemantic2D>::from_point_range(p.as_slice(),3..=3);eq_coord(s.start(),[2,1065353216]);eq_coord(s.end(),[2,1065353216]);
}
{let p=vec![Vec3::ZERO;4];
assert!(std::panic::catch_unwind(||{CurveSlice::<Vec3,_,super::super::CurveCoordSemantic2D>::from_point_range(p.as_slice(),4..=4)}).is_err());
}
{let p=vec![Vec3::ZERO;3];
let s=CurveSlice::<Vec3,_,super::super::CurveCoordSemantic2D>::from_point_range(p.as_slice(),0..=0);eq_coord(s.start(),[0,0]);eq_coord(s.end(),[0,0]);
}
{let p=vec![Vec3::ZERO;3];
let s=CurveSlice::<Vec3,_,super::super::CurveCoordSemantic2D>::from_point_range(p.as_slice(),1..=1);eq_coord(s.start(),[1,0]);eq_coord(s.end(),[1,0]);
}
{let p=vec![Vec3::ZERO;3];
let s=CurveSlice::<Vec3,_,super::super::CurveCoordSemantic2D>::from_point_range(p.as_slice(),2..=2);eq_coord(s.start(),[1,1065353216]);eq_coord(s.end(),[1,1065353216]);
}
{let p=vec![Vec3::ZERO;3];
assert!(std::panic::catch_unwind(||{CurveSlice::<Vec3,_,super::super::CurveCoordSemantic2D>::from_point_range(p.as_slice(),3..=3)}).is_err());
}
{let p=vec![Vec3::ZERO;8];
let s=CurveSlice::<Vec3,_,super::super::CurveCoordSemantic2D>::from_point_range(p.as_slice(),0..=0);eq_coord(s.start(),[0,0]);eq_coord(s.end(),[0,0]);
}
{let p=vec![Vec3::ZERO;8];
let s=CurveSlice::<Vec3,_,super::super::CurveCoordSemantic2D>::from_point_range(p.as_slice(),1..=1);eq_coord(s.start(),[1,0]);eq_coord(s.end(),[1,0]);
}
{let p=vec![Vec3::ZERO;8];
let s=CurveSlice::<Vec3,_,super::super::CurveCoordSemantic2D>::from_point_range(p.as_slice(),2..=2);eq_coord(s.start(),[2,0]);eq_coord(s.end(),[2,0]);
}
{let p=vec![Vec3::ZERO;8];
let s=CurveSlice::<Vec3,_,super::super::CurveCoordSemantic2D>::from_point_range(p.as_slice(),3..=3);eq_coord(s.start(),[3,0]);eq_coord(s.end(),[3,0]);
}
{let p=vec![Vec3::ZERO;8];
let s=CurveSlice::<Vec3,_,super::super::CurveCoordSemantic2D>::from_point_range(p.as_slice(),4..=4);eq_coord(s.start(),[4,0]);eq_coord(s.end(),[4,0]);
}
{let p=vec![Vec3::ZERO;8];
let s=CurveSlice::<Vec3,_,super::super::CurveCoordSemantic2D>::from_point_range(p.as_slice(),5..=5);eq_coord(s.start(),[5,0]);eq_coord(s.end(),[5,0]);
}
{let p=vec![Vec3::ZERO;8];
let s=CurveSlice::<Vec3,_,super::super::CurveCoordSemantic2D>::from_point_range(p.as_slice(),6..=6);eq_coord(s.start(),[6,0]);eq_coord(s.end(),[6,0]);
}
{let p=vec![Vec3::ZERO;8];
let s=CurveSlice::<Vec3,_,super::super::CurveCoordSemantic2D>::from_point_range(p.as_slice(),7..=7);eq_coord(s.start(),[6,1065353216]);eq_coord(s.end(),[6,1065353216]);
}
{let p=vec![Vec3::ZERO;8];
assert!(std::panic::catch_unwind(||{CurveSlice::<Vec3,_,super::super::CurveCoordSemantic2D>::from_point_range(p.as_slice(),8..=8)}).is_err());
}
}
