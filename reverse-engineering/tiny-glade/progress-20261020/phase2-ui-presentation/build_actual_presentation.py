from pathlib import Path
import json,hashlib,subprocess
H=Path(__file__).resolve().parent;R=H.parents[3];P=R/'tiny-glade/crates/systems/toolbar/src/ui_presentation.rs';s=P.read_text(encoding='utf-8');easing=R/'tiny-glade/crates/utils/src/easing_functions.rs'
def fn(name):
 a=s.index('pub fn '+name+'(');b=s.index('{',a);d=1;e=b+1
 while d:d+=(s[e]=='{')-(s[e]=='}');e+=1
 return s[a:e],s[b:e]
p='#![allow(dead_code)]\n#[path="'+easing.as_posix()+'"] mod easing_functions;\nuse easing_functions::EasingFunction;\n#[derive(Clone,Copy)]pub struct UiCustomSize{kind:u32,value:f32}\n'
for name in ['focus_scale','appear_scale','click_sound','drag_sound']:p+=fn(name)[0]+'\n'
p+='''fn n(b:&[u8],a:usize)->u32{u32::from_le_bytes(b[a..a+4].try_into().unwrap())}fn f(b:&[u8],a:usize)->f32{f32::from_bits(n(b,a))}
fn main(){let mut nan_payload_differences=0;
 let fixture=include_bytes!("native-focus.bin");for (i,b) in fixture.chunks_exact(20).enumerate(){let custom=UiCustomSize{kind:b[3] as u32,value:f(b,4)};let actual=focus_scale(f(b,8),b[0]!=0,b[1]!=0,(b[2]!=0).then_some(&custom),f(b,12));if actual.is_nan()&&f(b,16).is_nan(){nan_payload_differences+=usize::from(actual.to_bits()!=n(b,16));}else{assert_eq!(actual.to_bits(),n(b,16),"focus{i}");}}
 let fixture=include_bytes!("native-appear.bin");for (i,b) in fixture.chunks_exact(12).enumerate(){let actual=appear_scale(f(b,0),f(b,4));if actual.is_nan()&&f(b,8).is_nan(){nan_payload_differences+=usize::from(actual.to_bits()!=n(b,8));}else{assert_eq!(actual.to_bits(),n(b,8),"appear{i}");}}
 let mut sounds=Vec::new();
 for i in 0..578 {let mask=if i<576 {i/18}else{0};sounds.push(if mask&1!=0{10}else{15});}
 for i in 0..578 {let mask=if i<576 {i/18}else{0};if let Some(sound)=click_sound(mask&8!=0,mask&1!=0,(mask&16!=0).then_some((i%18) as u8),mask&2!=0,mask&4!=0){sounds.push(sound);}}
 for i in 0..578 {let mask=if i<576 {i/18}else{0};for state in 0..=255 {if let Some(sound)=drag_sound(mask&8!=0,state){sounds.push(sound);}}}
 let expected=include_bytes!("native-sounds-output.bin").chunks_exact(4).map(|b|n(b,0)).collect::<Vec<_>>();assert_eq!(sounds,expected);
 println!("Actual source1848focus+66appear nonNaNbits/NaNclassification and {} original sound emissions match",sounds.len());println!("NaN payload/sign differences permitted by Rust arithmetic: {nan_payload_differences}; all nonNaN output bits exact");
}
'''
(H/'actual_presentation_source.rs').write_text(p,encoding='utf-8');(H/'actual-source-manifest.json').write_text(json.dumps({'source_sha256':hashlib.sha256(P.read_bytes()).hexdigest(),'easing_sha256':hashlib.sha256(easing.read_bytes()).hexdigest(),'body_replacements':0,'extracted':['focus_scale','appear_scale','click_sound','drag_sound'],'sound_stream_boundary':'fixed original event order, 578componentcombinations and256dragbytes each'},indent=2));rustc='C:/Users/liyu/.rustup/toolchains/stable-x86_64-pc-windows-msvc/bin/rustc.exe';subprocess.run([rustc,'--edition','2021',str(H/'actual_presentation_source.rs'),'-o',str(H/'actual_presentation_source.exe')],check=True);r=subprocess.run([str(H/'actual_presentation_source.exe')],capture_output=True,text=True);print(r.stdout+r.stderr);(H/'actual-source-results.txt').write_text(r.stdout+r.stderr);raise SystemExit(r.returncode)
