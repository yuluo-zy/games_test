"""Original RNE complete ridge numeric chain, no core substitutions."""
from pathlib import Path
import hashlib,json,struct,subprocess,random,math
from collections import Counter
from unicorn import Uc,UC_ARCH_X86,UC_MODE_64,UC_HOOK_CODE
from unicorn.x86_const import *
ROOT=Path('D:/game/reverse-engineering/tiny-glade')
raw=Path('D:/game/ljxsj_92385/Tiny Glade/tiny-glade.rne').read_bytes()
assert hashlib.sha256(raw).hexdigest()=='f507a6cea25c5199447b88ba746a5293e64a279f73797ca74c1ce7f4ff2b7d03'
pe=struct.unpack_from('<I',raw,60)[0]; ns=struct.unpack_from('<H',raw,pe+6)[0]; os=struct.unpack_from('<H',raw,pe+20)[0]
base=struct.unpack_from('<Q',raw,pe+48)[0]; image_size=struct.unpack_from('<I',raw,pe+80)[0]
cpu=Uc(UC_ARCH_X86,UC_MODE_64);cpu.mem_map(base,(image_size+4095)&~4095)
for i in range(ns):
 z,a,s,o=struct.unpack_from('<IIII',raw,pe+24+os+40*i+8);cpu.mem_write(base+a,raw[o:o+s])
DATA=0x20000000;STACK=0x30000000;STOP=0x40000000
cpu.mem_map(DATA,0x10000);cpu.mem_map(STACK,0x10000);cpu.mem_map(STOP,0x1000)
panics={0x140d8b480,0x1428d9390,0x1428d9430,0x1428d9310}
calls=Counter();state={}
entries=[0x1408e2690,0x1408e2700,0x1408e2750,0x1408e2eb0,0x1408e30b0,0x1408e30f0,0x1408e3780,0x140c99540,0x140c9cdc0]
def hook(c,va,size,user):
 if va in entries:calls[hex(va)]+=1
 if va in panics:state['panic']=hex(va);c.emu_stop()
cpu.hook_add(UC_HOOK_CODE,hook)
def invoke(va,regs={},xmm={}):
 state.clear();cpu.reg_write(UC_X86_REG_MXCSR,0x1f80)
 cpu.reg_write(UC_X86_REG_RSP,STACK+0x8008);cpu.mem_write(STACK+0x8008,struct.pack('<Q',STOP))
 for i in range(16):cpu.reg_write(UC_X86_REG_XMM0+i,0)
 for k,v in regs.items():cpu.reg_write(k,v)
 for k,v in xmm.items():cpu.reg_write(UC_X86_REG_XMM0+k,v)
 cpu.emu_start(va,STOP,count=8000)
 if not state and cpu.reg_read(UC_X86_REG_RIP)!=STOP:raise RuntimeError('instruction bound')
 return 'panic' if state else None
def fb(v):return struct.unpack('<I',struct.pack('<f',v))[0]
def put(b,o,x):struct.pack_into('<f',b,o,x)
def words(v):return ' '.join(f'{x:08x}' for x in v)
def xmmwords():return [cpu.reg_read(UC_X86_REG_XMM0)&0xffffffff,cpu.reg_read(UC_X86_REG_XMM1)&0xffffffff]
lines=[];expected=[];meta=[];counts=Counter()
def record(line,out,info=None):lines.append(line);expected.append(out);meta.append(info);counts[line.split()[0]]+=1
rng=random.Random(20261011)
edge=[0,0x80000000,1,0x80000001,fb(.1)-1,fb(.1),fb(.1)+1,fb(1),fb(-1),0x7f800000,0xff800000]
for i in range(1500):
 a=rng.choice(edge) if i<200 else fb(rng.uniform(-2,2));d=rng.choice([0,1,2,255]);x=fb(rng.uniform(.01,25));z=fb(rng.uniform(.01,25))
 invoke(0x1408e2700,{UC_X86_REG_RDX:d},{0:a,2:x,3:z});v=cpu.reg_read(UC_X86_REG_RAX)
 record(f'calc {a:08x} {d} {x:08x} {z:08x}',words([v&0xffffffff,v>>32]))
 inv_a=fb(rng.uniform(-30,30)) if i>200 else rng.choice(edge);invx=fb(.1) if i%40==0 else x
 err=invoke(0x1408e30f0,{UC_X86_REG_RDX:d},{0:inv_a,2:invx,3:z})
 record(f'inverse {inv_a:08x} {d} {invx:08x} {z:08x}',err or words([cpu.reg_read(UC_X86_REG_XMM0)&0xffffffff]))

fixtures=json.loads((ROOT/'evidence/roof-fixtures.json').read_text(encoding='utf8'))
print('fixture keys',fixtures.keys(),flush=True)
# Fixture mappings retain the recovered parameters; shape and transforms are
# generated independently because history roof commands do not carry walls.
fixture_values=fixtures.get('fixtures',fixtures.get('cases',[]))
if not fixture_values:fixture_values=next((v for k,v in fixtures.items() if isinstance(v,list) and v and isinstance(v[0],dict) and 'params' in str(v[0])),[])
roof_cases=[]
for i in range(2200):
 b=bytearray(0x58);rect=(i%4!=0);struct.pack_into('<I',b,8,int(rect))
 angle=rng.uniform(-math.pi,math.pi)
 if rect:
  put(b,0xc,math.cos(angle));put(b,0x10,math.sin(angle));put(b,0x14,rng.uniform(-50,50));put(b,0x18,rng.uniform(-50,50));put(b,0x1c,rng.uniform(.1,30));put(b,0x20,rng.uniform(.1,30))
 else:put(b,0xc,rng.uniform(-50,50));put(b,0x10,rng.uniform(-50,50));put(b,0x14,rng.uniform(.1,15))
 put(b,0x24,rng.uniform(-1,1));put(b,0x28,rng.uniform(-1,1));put(b,0x30,rng.choice([0,.1,.10000001,1]) if i%10==0 else rng.uniform(0,1.2));put(b,0x34,rng.choice([0,1]) if i%10==0 else rng.uniform(0,1));b[0x3c]=i%2;put(b,0x4c,rng.uniform(-3,12))
 roof_cases.append((bytes(b),None))
for j,f in enumerate(fixture_values):
 p=f.get('params',{})
 if not p:continue
 b=bytearray(roof_cases[j%len(roof_cases)][0]);struct.pack_into('<I',b,8,1)
 for k,o in [('height_01',0x30),('ridge_length_01',0x34)]:
  if k in p:put(b,o,p[k])
 if 'tip_offset_01' in p:
  t=p['tip_offset_01'];t=list(t.values()) if isinstance(t,dict) else t
  if isinstance(t,(list,tuple)):put(b,0x24,t[0]);put(b,0x28,t[1])
 # Serialized Width/Length discriminant mapping is not asserted here.
 # Every shipped parameter snapshot is checked along both original directions.
 for direction in [0,1]:
  b[0x3c]=direction
  roof_cases.append((bytes(b),dict(f,observed_machine_direction=direction)))
geometry_edge_count=0
for rect in [0,1]:
 for o in [0x30,0x34,0x24,0x28]:
  for edge_bits in [0,0x80000000,1,0x80000001,fb(.1)-1,fb(.1),fb(.1)+1,fb(1),0x7f800000,0xff800000,0x7fc00000]:
   b=bytearray(roof_cases[rect][0]);struct.pack_into('<I',b,8,rect)
   for center_offset in [0xc,0x10,0x14,0x18]:put(b,center_offset,0.0)
   if rect:put(b,0xc,1.0);put(b,0x1c,5.0);put(b,0x20,8.0)
   else:put(b,0x14,3.0)
   put(b,0x30,.5);put(b,0x34,.5);put(b,0x24,0.0);put(b,0x28,0.0);put(b,0x4c,0.0)
   struct.pack_into('<I',b,o,edge_bits);roof_cases.append((bytes(b),dict(edge=True,shape=rect,field=hex(o),bits=hex(edge_bits))))
   geometry_edge_count+=1
for b,source in roof_cases:
 cpu.mem_write(DATA,b)
 invoke(0x1408e2690,{UC_X86_REG_RCX:DATA});v=cpu.reg_read(UC_X86_REG_RAX);dims=[v&0xffffffff,v>>32]
 record('from '+b.hex(),words(dims),source)
 invoke(0x1408e3780,{UC_X86_REG_RCX:DATA});record('tip '+b.hex(),words(xmmwords()),source)
 scale=[fb(rng.uniform(-2,2)),fb(rng.uniform(-2,2))]
 invoke(0x1408e30b0,{UC_X86_REG_RCX:DATA},{1:scale[0],2:scale[1]});record('center '+b.hex()+' '+words(scale),words(xmmwords()),source)
 cpu.mem_write(DATA+0x100,struct.pack('<II',*dims));cpu.mem_write(DATA+0x200,b'\xaa'*28)
 err=invoke(0x1408e2750,{UC_X86_REG_RCX:DATA+0x200,UC_X86_REG_RDX:DATA+0x100,UC_X86_REG_R8:DATA})
 if err:out=err
 else:
  ob=bytes(cpu.mem_read(DATA+0x200,28));discr=struct.unpack_from('<I',ob)[0];n=3 if discr==0 else 6
  out=str(discr)+' '+words(struct.unpack_from('<'+'I'*n,ob,4))
 record('world '+b.hex()+' '+words(dims),out,source)
bin=ROOT/'workspace/ridge-reconstructed.exe'
import sys
sys.path.insert(0,str(ROOT/'scripts'))
from dependency_build import runner_command
command=runner_command(ROOT/'reconstruction/roof/ridge.rs',bin)
actual=subprocess.run([str(bin)],input='\n'.join(lines)+'\n',text=True,capture_output=True,check=True).stdout.splitlines()
fails=[dict(case=i,input=lines[i],original=a,candidate=b,fixture=meta[i]) for i,(a,b) in enumerate(zip(expected,actual)) if a!=b]
report={'ok':len(actual)==len(expected) and not fails,'source_sha256':hashlib.sha256(raw).hexdigest(),'oracle':'Unicorn executes original complete numeric routines and original orientation/x0y/point closures; no numeric call stubs','scope':'ridge length/direction/point-vs-segment, tipoffset, world coordinates; complete original RoofRidgeDims chain with supplied Roof bytes','counts':dict(counts),'total_cases':len(lines),'fixture_snapshots':len(roof_cases)-2200-geometry_edge_count,'geometry_edge_cases':geometry_edge_count,'machine_entry_calls':dict(calls),'failures':fails,'mxcsr':'0x1f80','limitations':['Shape transforms/dimensions synthesized; original params from history do not include linked Wall shape','Only valid shape tags 0/1; original panic entry detected before logging/unwind','Full mesh/tile orchestration and SIMD exception flags are not validated','Geometry tests include 88 boundary injections; dimensions/inverse include infinities, subnormals, signed zero']}
(ROOT/'evidence/roof-ridge/verification.json').write_text(json.dumps(report,indent=2,ensure_ascii=False),encoding='utf8')
print(json.dumps({k:report[k] for k in ['ok','total_cases','counts','fixture_snapshots']}));print('failures',len(fails));print(json.dumps(fails[:8],indent=2))
if not report['ok']:raise SystemExit(1)
