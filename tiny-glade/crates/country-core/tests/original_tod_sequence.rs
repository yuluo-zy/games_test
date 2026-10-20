//! Full original824B snapshot function outputs, not hand-written expected
//! interpolation results. See independent-flow-wiring/tod-sequence-native-policy.json.
use country_core::{resources::time_of_day::{TodSequence,TodSequenceEntry,TodLightingSnapshot},render::atmosphere::SunSkyColors};
use serde::Deserialize;
#[derive(Deserialize)]struct Entry { tod_bits:u32,snapshot_bits:[u32;17] }
#[derive(Deserialize)]struct Case { sequence:usize,tod_bits:String,native_status:String,native_bits:Option<[u32;17]> }
#[derive(Deserialize)]struct Fixture { original_sha256:String,sequences:Vec<Vec<Entry>>,results:Vec<Case> }
fn snapshot(bits:[u32;17])->TodLightingSnapshot {
 let v=bits.map(f32::from_bits);
 TodLightingSnapshot {sun_sky:SunSkyColors {a:glam::Vec4::from_array(v[0..4].try_into().unwrap()),b:glam::Vec4::from_array(v[4..8].try_into().unwrap()),c:glam::Vec4::from_array(v[8..12].try_into().unwrap())},base_fog_thickness:v[12],fog_tint:glam::Vec3::from_array(v[13..16].try_into().unwrap()),speed_bias:v[16]}
}
fn snapshot_bits(s:TodLightingSnapshot)->[u32;17] {
 let mut values=[0.0;17];values[0..4].copy_from_slice(&s.sun_sky.a.to_array());values[4..8].copy_from_slice(&s.sun_sky.b.to_array());values[8..12].copy_from_slice(&s.sun_sky.c.to_array());values[12]=s.base_fog_thickness;values[13..16].copy_from_slice(&s.fog_tint.to_array());values[16]=s.speed_bias;values.map(f32::to_bits)
}
#[test]fn entire_original_snapshot_matches_periodic_and_degenerate_policy() {
 let fixture:Fixture=serde_json::from_str(include_str!("fixtures/original-tod-sequence-b.json")).unwrap();
 assert_eq!(fixture.original_sha256,"f507a6cea25c5199447b88ba746a5293e64a279f73797ca74c1ce7f4ff2b7d03");
 assert_eq!(fixture.results.len(),189);
 for (i,case) in fixture.results.iter().enumerate() {
  let sequence=TodSequence { entries:fixture.sequences[case.sequence].iter().enumerate().map(|(i,e)|TodSequenceEntry {temp_ui_id:i as u64,time_of_day:f32::from_bits(e.tod_bits),snapshot:snapshot(e.snapshot_bits)}).collect() };
  let time=f32::from_bits(u32::from_str_radix(case.tod_bits.trim_start_matches("0x"),16).unwrap());
  let actual=std::panic::catch_unwind(||sequence.snapshot_at_tod(time));
  if case.native_status=="panic" { assert!(actual.is_err(),"native panic case {i}"); }
  else { assert_eq!(snapshot_bits(actual.unwrap()),case.native_bits.unwrap(),"native full wrapper case {i}"); }
 }
}
