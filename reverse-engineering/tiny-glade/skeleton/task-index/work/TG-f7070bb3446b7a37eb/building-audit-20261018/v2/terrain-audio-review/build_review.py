from pathlib import Path
import json, hashlib, difflib
root=Path('/Users/yuluo/Desktop/game')
out=Path(__file__).resolve().parent
author=root/'games_test/reverse-engineering/tiny-glade/skeleton/task-index/work/TG-2e2cb1fa2e213f65da/terrain-curve-20261018'
frozen=json.loads((author/'frozen-inputs.json').read_text())
checks=[{'path':x['path'],'expected':x['sha256'],'actual':hashlib.sha256(Path(x['path']).read_bytes()).hexdigest()} for x in frozen['files']]
assert all(x['expected']==x['actual']for x in checks)
(out/'frozen19-check.json').write_text(json.dumps(checks,indent=2)+'\n')
records=[]
for name in ['B-dossier','B-second-dossier']:
 p=root/'games_test/reverse-engineering/tiny-glade/evidence/terrain-editing'/name/'dossier.json'
 d=json.loads(p.read_text())
 for fun in d['functions']:
  refs=[ref for ref in fun.get('entry_references',[]) if 'PlayTerrainSound' in ref.get('owner','')]
  if refs:records.append({'dossier':str(p),'dossier_sha256':hashlib.sha256(p.read_bytes()).hexdigest(),'database_executable_sha256':d.get('executable_sha256'),'address':fun['address'],'name':fun['name'],'entry_references':refs})
(out/'producer-consumer-type-records.json').write_text(json.dumps({'target':'B','evidence_kind':'retained original FunctionSystem call owners; body grow_one labels are not type identity','records':records},indent=2)+'\n')
rel='games_test/tiny-glade/crates/country-core/src/systems/audio/terrain_audio.rs'
p=root/rel
base=p.read_text()
old='fn clamp01(value: f32) -> f32 {\n    value.max(0.0).min(1.0)\n}'
new='''fn clamp01(value: f32) -> f32 {
    // Native MAXSS/MINSS retain NaN and the input signed zero. The f32
    // min/max methods ignore NaN and canonicalize opposite signed zeros.
    value.clamp(0.0, 1.0)
}'''
assert old in base
candidate=base.replace(old,new,1)
needle='    #[test]\n    fn original_one_shot_0()'
assert needle in candidate
newtest='''    #[test]
    fn low_point_speed_preserves_native_nan_and_signed_zero() {
        // Original B tag4 branch replay: one-shot-boundaries.json. NaN sign
        // and payload from arithmetic depend on the host CPU, so compare
        // NaN semantically; finite and signed-zero results compare by bits.
        for (magnitude, editing_speed, expected) in [
            (0.0, 0.0, None),
            (f32::NAN, 1.0, None),
            (0.1, 0.0, Some(1.0_f32.to_bits())),
            (-0.1, 0.0, Some(0.0_f32.to_bits())),
            (-0.0, 1.0, Some((-0.0_f32).to_bits())),
        ] {
            let calls = observe(
                PlayTerrainSound::Removed {
                    position: Vec3::new(3.0, -1.0, 4.0),
                    falloff: 0.4,
                    magnitude,
                },
                editing_speed,
                TERRAIN_LOW_POINT_EVENT,
            );
            let bits = calls
                .iter()
                .find_map(|call| match call {
                    Call::Parameter(name, parameter, bits)
                        if name == TERRAIN_LOW_POINT_EVENT
                            && parameter == TERRAIN_EDITING_SPEED_PARAM =>
                    {
                        Some(*bits)
                    }
                    _ => None,
                })
                .expect("LowPoint must forward its editing_speed parameter");
            match expected {
                Some(expected) => assert_eq!(bits, expected),
                None => assert!(f32::from_bits(bits).is_nan()),
            }
        }
    }

'''
candidate=candidate.replace(needle,newtest+needle,1)
(out/'terrain_audio.rs').write_text(candidate)
(out/'clamp-native-boundaries.patch').write_text(''.join(difflib.unified_diff(base.splitlines(True),candidate.splitlines(True),fromfile='a/'+rel,tofile='b/'+rel)))
(out/'clamp-manifest.json').write_text(json.dumps({'target':'B','production_edited':False,'cargo_run':False,'base':{'path':str(p),'sha256':hashlib.sha256(base.encode()).hexdigest()},'candidate':{'path':str(out/'terrain_audio.rs'),'sha256':hashlib.sha256(candidate.encode()).hexdigest()},'patch':{'path':str(out/'clamp-native-boundaries.patch'),'sha256':hashlib.sha256((out/'clamp-native-boundaries.patch').read_bytes()).hexdigest()},'frozen19_all_match':True,'type_identity_records':len(records)},indent=2)+'\n')
print({'frozen19':len(checks),'type_identity_records':len(records),'candidate':str(out/'terrain_audio.rs')})
