from pathlib import Path
import hashlib,json,re,subprocess
H=Path(__file__).resolve().parent;R=H.parents[3];P=R/'tiny-glade/crates/country-core/src/resources/walls/decorator.rs';s=P.read_text(encoding='utf-8')
def block(token,start=0):
 a=s.index(token,start);b=s.index('{',a);depth=1;e=b+1
 while depth:
  if s[e]=='{':depth+=1
  elif s[e]=='}':depth-=1
  e+=1
 return s[a:e]
info=block('pub struct DecoratorInfo {');extra=block('pub enum DecoratorExtraInfo {');chim=block('pub struct ChimneyExtra {')
methods='\n'.join(block(t) for t in ['pub fn extra_disc(', 'fn extra_bytes(', 'pub fn extra(', 'pub fn set_extra(', 'pub fn extra_style(']);disc=block('pub fn disc(')
code='#![allow(dead_code)]\n#[derive(Clone,Copy,Debug)]\n'+extra+'\n#[derive(Clone,Copy,Debug)]\npub struct StairExtra(pub [u8;20]);\n#[derive(Clone,Copy,Debug)]\npub struct WindowExtra(pub u32);\n#[derive(Clone,Copy,Debug)]\npub struct FlagExtra(pub u8);\n#[derive(Clone,Copy,Debug)]\npub struct LanternExtra(pub f32);\n#[derive(Clone,Copy,Debug)]\n'+chim+'\n#[repr(C)]\n#[derive(Clone,Copy,Debug)]\n'+info+'\nimpl DecoratorInfo {\n'+methods+'\n}\nimpl DecoratorExtraInfo {\n'+disc+'\n}\n'
code+='''fn main() {
    assert_eq!(std::mem::size_of::<DecoratorInfo>(),72);
    let bytes=include_bytes!("native-from-fixtures.bin");
    let mut total=0;let mut errors=0;
    for case in bytes.chunks_exact(165) {
        let kind=case[0];let before:[u8;72]=case[1..73].try_into().unwrap();let payload:[u8;20]=case[73..93].try_into().unwrap();let expected:[u8;72]=case[93..165].try_into().unwrap();
        let mut info:DecoratorInfo=unsafe{std::mem::transmute(before)};
        let value=match kind {0|1=>DecoratorExtraInfo::Stair(StairExtra(payload)),4=>DecoratorExtraInfo::Chimney(ChimneyExtra{f:f32::from_le_bytes(payload[..4].try_into().unwrap()),smoke:payload[4]}),5=>DecoratorExtraInfo::Flag(FlagExtra(payload[0])),6=>DecoratorExtraInfo::Window(WindowExtra(u32::from_le_bytes(payload[..4].try_into().unwrap()))),7=>DecoratorExtraInfo::Lantern(LanternExtra(f32::from_le_bytes(payload[..4].try_into().unwrap()))),_=>unreachable!()};
        info.set_extra(value);
        let actual:[u8;72]=unsafe{std::mem::transmute(info)};
        if actual!=expected {errors+=1;eprintln!("setter mismatch case{total} kind{kind}");}
        assert_eq!(info.extra_disc(),kind);assert_eq!(info.extra_style(),kind);
        let read=info.extra();
        let matches=match read {DecoratorExtraInfo::Stair(x)=>x.0==payload,DecoratorExtraInfo::Chimney(x)=>x.f.to_bits()==u32::from_le_bytes(payload[..4].try_into().unwrap())&&x.smoke==payload[4],DecoratorExtraInfo::Flag(x)=>x.0==payload[0],DecoratorExtraInfo::Window(x)=>x.0==u32::from_le_bytes(payload[..4].try_into().unwrap()),DecoratorExtraInfo::Lantern(x)=>x.0.to_bits()==u32::from_le_bytes(payload[..4].try_into().unwrap())};
        if !matches {errors+=1;eprintln!("getter mismatch case{total} kind{kind}");}
        if read.disc()!=kind {errors+=1;if errors<5{eprintln!("enumdisc mismatch case{total} kind{kind}, got{}",read.disc());}}
        // Independent destination union and rank/type/state remain untouched.
        assert_eq!(&actual[..48],&before[..48]);assert_eq!(&actual[68..],&before[68..]);
        total+=1;
    }
    println!("actual production ExtraInfo: {total} cases, {errors} errors");assert_eq!(errors,0);
}
'''
(H/'actual_extra_source.rs').write_text(code,encoding='utf-8');(H/'actual-source-manifest.json').write_text(json.dumps({'source':str(P),'source_sha256':hashlib.sha256(P.read_bytes()).hexdigest(),'extracted_methods':['extra_disc','extra_bytes','extra','set_extra','extra_style','DecoratorExtraInfo::disc'],'body_replacements':0},indent=2))
rustc=Path('C:/Users/liyu/.rustup/toolchains/stable-x86_64-pc-windows-msvc/bin/rustc.exe');subprocess.run([str(rustc),'--edition','2021',str(H/'actual_extra_source.rs'),'-o',str(H/'actual_extra_source.exe')],check=True)
p=subprocess.run([str(H/'actual_extra_source.exe')],capture_output=True,text=True);(H/'actual-source-results.txt').write_text(p.stdout+p.stderr);print(p.stdout+p.stderr);raise SystemExit(p.returncode)
