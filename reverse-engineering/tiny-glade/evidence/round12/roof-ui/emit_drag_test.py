from pathlib import Path
import json,struct,hashlib
out=Path(__file__).parent
r=json.loads((out/'native-drag-fixtures.json').read_text())
source=out/'proposed/tiny-glade/crates/systems/roof/src/ui_edit_roof.rs'
s=source.read_text(encoding='utf8')
prefix=s.split('\n#[cfg(test)]\nmod original_drag_oracle_tests')[0]
rows=[]
for case in r['cases']:
    if case['event_hex']:
        b=bytes.fromhex(case['event_hex']);w=struct.unpack('<16I',b)
        event=[w[0],w[1] if w[0] else 0,w[2],w[3] if w[2] else 0]
    else:event=[0xffffffff,0,0,0]
    heights=case['height_requests'];assert len(heights)<=1
    rows.append('        ('+str(case['input_bits'])+','+str(event)+','+str(case['amount_bits'])+','+('Some('+str(heights[0]['height_bits'])+')' if heights else 'None')+','+str(case['hint']==[1]).lower()+'),')
body='''
#[cfg(test)]
mod original_drag_oracle_tests {
    use super::*;
    // Frozen unchanged-RNE oracle input/output. No implementation-generated
    // expected values: native_drag_oracle.py supplies these event stores.
    const CASES: &[([u32;5],[u32;4],u32,Option<u32>,bool)] = &[
'''+ '\n'.join(rows)+'''
    ];
    #[test]
    fn ordinary_roof_drag_matches_original_instructions() {
        for (index,(input,event,amount_bits,height,hint)) in CASES.iter().enumerate() {
            let [roof_height,dx,dy,start_amount,original]=input.map(f32::from_bits);
            let mut bytes=[0;88];bytes[..8].copy_from_slice(&91u64.to_le_bytes());
            bytes[0x30..0x34].copy_from_slice(&roof_height.to_le_bytes());
            let roof=Roof::from_observed(bytes);
            let mut amount=start_amount;
            let got=drag_edit(&roof,Vec2::new(dx,dy),&mut amount);
            assert_eq!(amount.to_bits(),*amount_bits,"amount case {index}");
            if event[0]==u32::MAX {
                assert!(got.is_none(),"no edit case {index}");
                assert!(height.is_none());
            } else {
                let (edit,got_hint,delta)=got.expect("native emitted edit");
                assert_eq!([edit.profile.mode,edit.height.mode],[event[0],event[2]],"modes case {index}");
                if edit.profile.mode!=0 {assert_eq!(edit.profile.value.to_bits(),event[1],"profile case {index}");}
                if edit.height.mode!=0 {assert_eq!(edit.height.value.to_bits(),event[3],"height case {index}");}
                assert_eq!(got_hint,*hint,"hint case {index}");
                assert_eq!(crossing_wall_height(original,roof_height,delta+roof_height).map(f32::to_bits),*height,"wall target case {index}");
                assert_eq!(edit.wall_id,91);
                assert_eq!([edit.ridge.mode,edit.eave.mode,edit.tip.mode],[0;3]);
                assert_eq!(edit.source_flag,0);
            }
        }
    }
    #[test]
    fn parameter_release_comparison_retains_nan_and_signed_zero_rules() {
        let mut bytes=[0;88];bytes[..8].copy_from_slice(&7u64.to_le_bytes());
        let roof=Roof::from_observed(bytes);
        let mut cache=RoofParamsCacheResource::default();cache.remember(&roof,None);
        let mut after=roof.clone();after.observed_mut()[0x24..0x28].copy_from_slice(&(-0.0f32).to_le_bytes());
        assert!(!cache.before.as_ref().unwrap().differs(&after));
        after.observed_mut()[0x24..0x28].copy_from_slice(&f32::NAN.to_le_bytes());
        assert!(cache.before.as_ref().unwrap().differs(&after));
        cache.remember(&after,None);
        assert!(cache.before.as_ref().unwrap().differs(&after));
    }
}
'''
source.write_text(prefix+body,encoding='utf8',newline='\n')
print('generated original cases',len(rows),'fixture sha',hashlib.sha256((out/'native-drag-fixtures.json').read_bytes()).hexdigest())
