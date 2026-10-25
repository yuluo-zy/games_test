from pathlib import Path
import hashlib,json,difflib,datetime
root=Path('D:/game/tiny-glade');here=Path(__file__).parent;out=here/'tod-sequence-candidate';rows={};patch=[]
def save(rel,new):
 p=root/rel;old=p.read_bytes() if p.exists() else b'';dst=out/rel;dst.parent.mkdir(parents=True,exist_ok=True);dst.write_bytes(new)
 rows[rel]={'base_sha256':hashlib.sha256(old).hexdigest() if old else None,'candidate_sha256':hashlib.sha256(new).hexdigest()}
 patch.append(''.join(line if line.endswith('\n') else line+'\n\\ No newline at end of file\n' for line in difflib.unified_diff(old.decode().splitlines(True),new.decode().splitlines(True),'a/'+rel if old else '/dev/null','b/'+rel)))
rel='crates/country-core/src/resources/time_of_day.rs';raw=(root/rel).read_bytes();start=raw.index(b'    /// `TodSequence::snapshot_at_tod`');end=raw.index(b'\n}\r\n',start) if b'\n}\r\n' in raw[start:] else raw.index(b'\n}\n',start);nl=b'\r\n' if b'\r\n' in raw[start:end] else b'\n'
new='''    /// B140b067d0/824B: adjacent keys are periodic across midnight.
    /// Before first, interpolate last.time-1 -> first; after the final pair,
    /// interpolate last -> first.time+1. A NaN weight copies the left snapshot
    /// without arithmetic; infinities and finite weights clamp to [0,1].
    pub fn snapshot_at_tod(&self, tod: f32) -> TodLightingSnapshot {
        let entries = &self.entries;
        if entries.len() == 1 {
            return entries[0].snapshot;
        }
        let t = tod.rem_euclid(1.0);
        // Native performs rem_euclid before the empty-vector bounds panic.
        let first = &entries[0];
        let last = entries.last().unwrap();
        let (a, b, lower, upper) = if t < first.time_of_day {
            (last, first, last.time_of_day - 1.0, first.time_of_day)
        } else if let Some(pair) = entries.windows(2).find(|pair| {
            pair[0].time_of_day <= t && t <= pair[1].time_of_day
        }) {
            (&pair[0], &pair[1], pair[0].time_of_day, pair[1].time_of_day)
        } else {
            (last, first, last.time_of_day, first.time_of_day + 1.0)
        };
        let raw = inverse_lerp(lower, upper, t);
        // Native bit-classifies the raw weight. Only NaN bypasses the lerp:
        // finite negatives and -inf reach weight0, +inf reaches weight1.
        if raw.is_nan() {
            return a.snapshot;
        }
        a.snapshot.lerp(&b.snapshot, raw.clamp(0.0, 1.0))
    }'''.encode().replace(b'\n',nl)
save(rel,raw[:start]+new+raw[end:])
fixture=(here/'tod-sequence-native-fixture.json').read_bytes();save('crates/country-core/tests/fixtures/original-tod-sequence-b.json',fixture)
test='''//! Full original824B snapshot function outputs, not hand-written expected
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
'''.encode()
save('crates/country-core/tests/original_tod_sequence.rs',test)
(here/'tod-sequence-candidate.patch').write_bytes(''.join(patch).encode())
(here/'tod-sequence-manifest.json').write_text(json.dumps({'created':datetime.datetime.now().isoformat(),'scope':'evidence-only full periodic algorithm; original189 cases executed, Rust tests pending central execution','files':rows},indent=2),encoding='utf-8')
print(json.dumps(rows,indent=2))
