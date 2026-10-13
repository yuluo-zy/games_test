use super::*;
use glam::Vec3;
fn points(value:&serde_json::Value) -> Vec<Vec3> {
 value.as_array().unwrap().iter().map(|p| {let a=p.as_array().unwrap();Vec3::new(f32::from_bits(a[0].as_u64().unwrap() as u32),f32::from_bits(a[1].as_u64().unwrap() as u32),f32::from_bits(a[2].as_u64().unwrap() as u32))}).collect()
}
#[test] fn original_completion_sampling() {
 let data:serde_json::Value=serde_json::from_str(include_str!("completion-fixtures.json")).unwrap();
 for (case,row) in data["cases"].as_array().unwrap().iter().enumerate() {
  let input=points(&row["points"]); let expected=&row["expected"];
  if expected.get("input_error").is_some() {continue;}
  let step=f32::from_bits(row["argument"].as_u64().unwrap() as u32);
  let got=match row["method"].as_str().unwrap() {
   "resample"=>Curve3::try_new(input,false).unwrap().try_resample(step,false).unwrap().points,
   "per_segment"=>Curve3XZ::try_new(input,false).unwrap().try_resample_per_segment(step).unwrap().points,
   "smooth"=>Curve3::try_new(input,false).unwrap().smooth(row["argument"].as_u64().unwrap() as usize).unwrap().points,
   "smooth_points"=>{let mut p=input;smooth_in_place(&mut p,row["argument"].as_u64().unwrap() as usize,row["closed"].as_bool().unwrap());p},
   _=>panic!("unknown fixture"),
  };
  let expected=points(&expected["points"]); assert_eq!(got.len(),expected.len(),"point count case {case}");
  for (a,b) in got.iter().zip(expected) {assert!((*a-b).length()<=0.00003,"case {case}: {a:?} != {b:?}");}
 }
}
