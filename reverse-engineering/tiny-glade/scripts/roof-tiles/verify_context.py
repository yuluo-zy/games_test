from pathlib import Path
import importlib.util,json,random,subprocess,struct,hashlib
from unicorn import UC_HOOK_CODE
from unicorn.x86_const import *
R=Path('D:/game/reverse-engineering/tiny-glade');O=R/'evidence/roof-tiles'
spec=importlib.util.spec_from_file_location('rect_oracle',R/'scripts/roof-ridge/capture_rectangular.py');m=importlib.util.module_from_spec(spec);spec.loader.exec_module(m)
captured=[]
def obs(c,pc,n,u):
 if pc==0x1421af1ac:
  bp=c.reg_read(UC_X86_REG_RBP);b=bytes(c.mem_read(bp+0xa8,24));t=bytes(c.mem_read(bp+0x148,24));basis=bytes(c.mem_read(bp+0x3f0,16));captured.append((b+t+basis).hex());c.emu_stop()
m.cpu.hook_add(UC_HOOK_CODE,obs)
runner=O/'context_runner.rs';runner.write_text('''#[path="D:/game/reverse-engineering/tiny-glade/reconstruction/roof/surface.rs"] mod surface;
#[path="D:/game/reverse-engineering/tiny-glade/reconstruction/roof/tiles.rs"] mod tiles;
use std::io::{self,BufRead};fn main(){for l in io::stdin().lock().lines(){let l=l.unwrap();let x:Vec<_>=l.split_whitespace().collect();let d=|s:&str|{(0..s.len()).step_by(2).map(|i|u8::from_str_radix(&s[i..i+2],16).unwrap()).collect::<Vec<_>>()};let rect:[u8;24]=d(x[0]).try_into().unwrap();let roof:[u8;88]=d(x[1]).try_into().unwrap();let ctx=tiles::rectangular_context_from_observed(&rect,&roof);let mut out=Vec::new();out.extend(ctx.bottom_rect.bytes());out.extend(ctx.top_rect.bytes());for x in ctx.basis{out.extend(x.to_le_bytes());}for b in out{print!("{b:02x}");}println!();}}
''',encoding='utf-8');exe=R/'workspace/context-reconstructed.exe'
import sys
sys.path.insert(0,str(R/'scripts'))
from dependency_build import build_runner
build_runner(runner,exe)
fixtures=json.loads((R/'evidence/roof-fixtures.json').read_text(encoding='utf-8'))['samples'];rand=random.Random(20261010);lines=[];sources=[]
for sample in fixtures:
 for switch in [0,1]:
  p=sample['params'];roof=bytearray(88);struct.pack_into('<I',roof,8,1)
  for o,v in [(0xc,1.),(0x10,0.),(0x14,rand.uniform(-3,3)),(0x18,rand.uniform(-3,3)),(0x1c,rand.uniform(2,8)),(0x20,rand.uniform(2,8)),(0x24,p['tip_offset_01'][0]),(0x28,p['tip_offset_01'][1]),(0x2c,p['profile_01']),(0x30,p['height_01']),(0x34,p['ridge_length_01']),(0x38,p['eave_length_01'])]:struct.pack_into('<f',roof,o,v)
  roof[0x3c]=switch;roof[0x54]=rand.randrange(2);shape=bytes(roof[0xc:0x24]);captured.clear();m.case(shape,roof,0)
  if not captured:raise RuntimeError('No context '+str(m.panic))
  lines.append(f'{shape.hex()} {roof.hex()}');sources.append(dict(path=sample['path'],pointer=sample['pointer']))
  sources[-1]['expected']=captured[0]
# Explicit gable + flag cases, with finite rotated frames and both ridge dirs.
import math
for i in range(160):
 roof=bytearray(88);struct.pack_into('<I',roof,8,1);rotation=rand.uniform(-8,8);c=math.cos(rotation);s=math.sin(rotation)
 for o,v in [(0xc,c),(0x10,s),(0x14,0.),(0x18,0.),(0x1c,rand.uniform(2,10)),(0x20,rand.uniform(2,10)),(0x24,rand.uniform(-1,1)),(0x28,rand.uniform(-1,1)),(0x2c,rand.random()),(0x30,.5),(0x34,1.),(0x38,rand.random())]:struct.pack_into('<f',roof,o,v)
 roof[0x3c]=i%2;roof[0x54]=(i//2)%2;shape=bytes(roof[0xc:0x24]);captured.clear();m.case(shape,roof,0);assert captured
 lines.append(f'{shape.hex()} {roof.hex()}');sources.append(dict(expected=captured[0],synthetic_gable=True))
actual=subprocess.run([str(exe)],input='\n'.join(lines)+'\n',capture_output=True,text=True,check=True).stdout.splitlines();expected=[s['expected'] for s in sources]
fail=[dict(case=i,input=lines[i],original=a,rust=b) for i,(a,b) in enumerate(zip(expected,actual)) if a!=b]
report=dict(ok=not fail and len(expected)==len(actual),source_sha256=hashlib.sha256(m.raw).hexdigest(),cases=len(lines),failures=fail,scope='Entire original rectangular setup from function entry to 0x1421af1ac; compare bottom rect24,top rect24,orientation basis16. Original numerical helper code executes; allocator/CRT bridges only.',full_game_executed=False,limitations=['Inputs use original history roof parameters and synthetic rectangle dimensions/poses','Local corner producer separate testing forthcoming; context raw rectangles verified bitwise','Full rectangular tile stream is not this test target'])
(O/'context-verification.json').write_text(json.dumps(report,indent=2),encoding='utf-8');print(json.dumps(report)[:3000]);
if not report['ok']:raise SystemExit(1)
