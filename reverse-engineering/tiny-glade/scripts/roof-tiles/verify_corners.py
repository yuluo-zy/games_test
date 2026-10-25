from pathlib import Path
import importlib.util,json,random,subprocess,struct,math
from unicorn.x86_const import *
R=Path('D:/game/reverse-engineering/tiny-glade');O=R/'evidence/roof-tiles'
spec=importlib.util.spec_from_file_location('rect_oracle',R/'scripts/roof-ridge/capture_rectangular.py');m=importlib.util.module_from_spec(spec);spec.loader.exec_module(m)
runner=O/'corners_runner.rs';runner.write_text('''#[path="D:/game/reverse-engineering/tiny-glade/reconstruction/roof/surface.rs"] mod surface;
#[path="D:/game/reverse-engineering/tiny-glade/reconstruction/roof/tiles.rs"] mod tiles;
use std::io::{self,BufRead};fn main(){for l in io::stdin().lock().lines(){let l=l.unwrap();let raw:[u8;24]=(0..48).step_by(2).map(|i|u8::from_str_radix(&l[i..i+2],16).unwrap()).collect::<Vec<_>>().try_into().unwrap();for point in tiles::ObservedRectangle::from_bytes(&raw).corners(){for x in point{for b in x.to_le_bytes(){print!("{b:02x}");}}}println!();}}
''',encoding='utf-8');exe=R/'workspace/corners-reconstructed.exe'
import sys
sys.path.insert(0,str(R/'scripts'))
from dependency_build import build_runner
build_runner(runner,exe)
rand=random.Random(20261010);lines=[];expected=[]
for i in range(1200):
 angle=rand.uniform(-10,10);b=struct.pack('<6f',math.cos(angle),math.sin(angle),rand.uniform(-100,100),rand.uniform(-100,100),rand.uniform(.01,100),rand.uniform(.01,100))
 m.cpu.mem_write(m.DATA,b);m.cpu.reg_write(UC_X86_REG_RDX,m.DATA);m.cpu.reg_write(UC_X86_REG_RCX,m.DATA+0x100);m.cpu.reg_write(UC_X86_REG_RSP,m.STACK+0x10008);m.cpu.mem_write(m.STACK+0x10008,struct.pack('<Q',m.STOP));m.cpu.reg_write(UC_X86_REG_MXCSR,0x1f80)
 m.cpu.emu_start(0x140c98ab0,m.STOP,count=1000);expected.append(bytes(m.cpu.mem_read(m.DATA+0x100,32)).hex());lines.append(b.hex())
actual=subprocess.run([str(exe)],input='\n'.join(lines)+'\n',capture_output=True,text=True,check=True).stdout.splitlines();fail=[dict(case=i,input=lines[i],original=a,rust=b) for i,(a,b) in enumerate(zip(expected,actual)) if a!=b];report=dict(ok=not fail and len(actual)==len(expected),cases=len(lines),failures=fail,scope='Entire original Rectangle2d::as_points2 machine code; ordered four-corner f32 bits compared',source_sha256='f507a6cea25c5199447b88ba746a5293e64a279f73797ca74c1ce7f4ff2b7d03',full_game_executed=False,limitations=['Finite positive dimensions and finite unit-basis inputs; panic branches not exercised'])
(O/'corners-verification.json').write_text(json.dumps(report,indent=2),encoding='utf-8');print(json.dumps(report)[:1200])
if not report['ok']:raise SystemExit(1)
