"""Validate pinned public dependency APIs against already identified callables.
No dependency implementation is decompiled or reproduced by this verification.
"""
from pathlib import Path
import importlib.util,json,random,subprocess,struct,hashlib,sys,tomllib
from unicorn.x86_const import *
R=Path('D:/game/reverse-engineering/tiny-glade');O=R/'evidence/roof-tiles';sys.path.insert(0,str(R/'scripts'))
from dependency_build import build_runner
spec=importlib.util.spec_from_file_location('rect_oracle',R/'scripts/roof-ridge/capture_rectangular.py');m=importlib.util.module_from_spec(spec);spec.loader.exec_module(m)
runner=O/'dependency_runner.rs';runner.write_text('''use std::io::{self,BufRead};fn main(){for l in io::stdin().lock().lines(){let l=l.unwrap();let w:Vec<_>=l.split_whitespace().collect();match w[0]{"half"=>{let v=f32::from_bits(u32::from_str_radix(w[1],16).unwrap());println!("{:04x}",half::f16::from_f32(v).to_bits());},"rng"=>{let seed=u64::from_str_radix(w[1],16).unwrap();let r=fastrand::Rng::with_seed(seed);let v=r.f32();println!("{:016x} {:08x}",r.get_seed(),v.to_bits());},_=>panic!()}}}
''',encoding='utf-8');exe=R/'workspace/pinned-dependencies.exe';build_runner(runner,exe)
rand=random.Random(20261010);edges=[0,0x80000000,1,0x80000001,0x3f800000,0xbf800000,0x7f800000,0xff800000,0x7fc00000,0xffc00000,0x7f800001,0x7fffffff,0x477fe000,0x477fefff,0x477ff000,0x38800000,0x387fffff,0x33000000,0x32ffffff,0x3f801000]
floatcases=edges+[rand.getrandbits(32) for _ in range(2048)];seedcases=[0,1,(1<<64)-1]+[rand.getrandbits(64) for _ in range(1024)];lines=[];expected=[]
def prep():
 m.cpu.reg_write(UC_X86_REG_RSP,m.STACK+0x10008);m.cpu.mem_write(m.STACK+0x10008,struct.pack('<Q',m.STOP));m.cpu.reg_write(UC_X86_REG_MXCSR,0x1f80)
for bits in floatcases:
 prep();m.cpu.reg_write(UC_X86_REG_XMM0,bits);m.cpu.emu_start(0x1408b9f20,m.STOP,count=5000);assert m.cpu.reg_read(UC_X86_REG_RIP)==m.STOP
 expected.append(f'{m.cpu.reg_read(UC_X86_REG_RAX)&65535:04x}');lines.append(f'half {bits:08x}')
for seed in seedcases:
 prep();m.cpu.reg_write(UC_X86_REG_RCX,m.DATA);m.cpu.mem_write(m.DATA,struct.pack('<Q',seed));m.cpu.emu_start(0x140caf380,m.STOP,count=100);assert m.cpu.reg_read(UC_X86_REG_RIP)==m.STOP
 value=m.cpu.reg_read(UC_X86_REG_XMM0)&0xffffffff;state=struct.unpack('<Q',m.cpu.mem_read(m.DATA,8))[0];expected.append(f'{state:016x} {value:08x}');lines.append(f'rng {seed:016x}')
actual=subprocess.run([str(exe)],input='\n'.join(lines)+'\n',capture_output=True,text=True,check=True).stdout.splitlines();fail=[dict(case=i,input=lines[i],original=a,published_api=b) for i,(a,b) in enumerate(zip(expected,actual)) if a!=b]
old=tomllib.loads(Path('D:/game/ljxsj_92385/Tiny Glade/build-info/Cargo.lock').read_text(encoding='utf-8'));new=tomllib.loads((R/'reconstruction/Cargo.lock').read_text(encoding='utf-8'));names={'fastrand':'1.8.0','glam':'0.29.3','half':'2.4.1'};deps=[]
for name,version in names.items():
 a=next(p for p in old['package'] if p['name']==name and p['version']==version);b=next(p for p in new['package'] if p['name']==name);assert b['version']==version and a.get('checksum')==b.get('checksum');deps.append(dict(name=name,version=version,checksum=b.get('checksum'),original_users=[p['name'] for p in old['package'] if p['name'] in ['system-roof','country-core','utils'] and any(d==name or d==name+' '+version for d in p.get('dependencies',[]))]))
report=dict(ok=not fail and len(actual)==len(expected),half_cases=len(floatcases),rng_cases=len(seedcases),failures=fail,dependencies=deps,source_sha256=hashlib.sha256(m.raw).hexdigest(),scope='Call existing original f32-to-f16 and RNG entrypoints as numerical oracles; compare exact published pinned API results/state; no dependency source recovery',full_game_executed=False,limitations=['MXCSR 0x1f80; current host CPU/public dependency dispatch','Matrix quaternion API covered by whole circular and rectangular 64-byte-record regressions rather than new dependency disassembly'])
(O/'dependency-verification.json').write_text(json.dumps(report,indent=2),encoding='utf-8');print(json.dumps(report)[:4000])
if not report['ok']:raise SystemExit(1)
