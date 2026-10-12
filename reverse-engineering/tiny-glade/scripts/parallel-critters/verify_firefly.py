"""B original-RNE per-entity firefly oracle. Actual game row instructions execute.
ECS iterator is resolved outside the row; CRT sinf/cosf/powf use host libc (explicit bridge).
RNG executes original perchance call instructions, not a Python replacement.
No target process or shared Ghidra database mutation.
"""
from pathlib import Path
import ctypes,hashlib,json,math,random,struct,subprocess,sys,platform
from unicorn import Uc,UC_ARCH_X86,UC_MODE_64,UC_HOOK_CODE
from unicorn.x86_const import *
ROOT=Path(__file__).resolve().parents[2];GAME=ROOT.parents[2];OUT=ROOT/'evidence/parallel-critters'
ORIGINAL=GAME/'ljxsj_92385/Tiny Glade/tiny-glade.rne'
raw=ORIGINAL.read_bytes();sha=hashlib.sha256(raw).hexdigest();assert sha=='f507a6cea25c5199447b88ba746a5293e64a279f73797ca74c1ce7f4ff2b7d03'
p=struct.unpack_from('<I',raw,60)[0];opt=p+24;base=struct.unpack_from('<Q',raw,opt+24)[0];size=struct.unpack_from('<I',raw,opt+56)[0]
cpu=Uc(UC_ARCH_X86,UC_MODE_64);cpu.mem_map(base,(size+4095)&~4095)
for i in range(struct.unpack_from('<H',raw,p+6)[0]):
 _,rva,n,off=struct.unpack_from('<IIII',raw,opt+struct.unpack_from('<H',raw,p+20)[0]+40*i+8)
 if n:cpu.mem_write(base+rva,raw[off:off+n])
DATA=0x20000000;STACK=0x30000000;cpu.mem_map(DATA,0x20000);cpu.mem_map(STACK,0x20000)
libc=ctypes.CDLL(None)
for n,argc in [('sinf',1),('cosf',1),('powf',2)]:
 fn=getattr(libc,n);fn.argtypes=[ctypes.c_float]*argc;fn.restype=ctypes.c_float

def bits(x):return struct.unpack('<I',struct.pack('<f',x))[0]
def fl(x):return struct.unpack('<f',struct.pack('<I',x))[0]
def f32(x):return fl(bits(x))
def read_f(a):return struct.unpack('<f',cpu.mem_read(a,4))[0]
def setx(reg,value):cpu.reg_write(reg,bits(value))
def ret():
 sp=cpu.reg_read(UC_X86_REG_RSP);pc=struct.unpack('<Q',cpu.mem_read(sp,8))[0];cpu.reg_write(UC_X86_REG_RSP,sp+8);cpu.reg_write(UC_X86_REG_RIP,pc)
mode='';rng_calls=0

def hook(c,pc,sz,user):
 global rng_calls
 if pc==0x141625ae0:rng_calls+=1
 if pc in [0x142923fd0,0x142923e80,0x142923f70]:
  x=fl(c.reg_read(UC_X86_REG_XMM0)&0xffffffff)
  if pc==0x142923fd0:value=libc.sinf(x)
  elif pc==0x142923e80:value=libc.cosf(x)
  else:value=libc.powf(x,fl(c.reg_read(UC_X86_REG_XMM1)&0xffffffff))
  setx(UC_X86_REG_XMM0,value);ret()
 if mode=='goal' and pc==0x142316a96:c.emu_stop()
 if mode=='velocity' and pc==0x14231715b:c.emu_stop()
 if mode=='smooth' and pc==0x142318ec7:c.emu_stop()
cpu.hook_add(UC_HOOK_CODE,hook)
def setup():
 cpu.mem_write(DATA,bytes(0x3000));cpu.mem_write(STACK,bytes(0x10000));cpu.reg_write(UC_X86_REG_RSP,STACK+0x8000);cpu.mem_write(STACK+0x8000,struct.pack('<Q',STACK));cpu.reg_write(UC_X86_REG_MXCSR,0x1f80)
 for reg in [UC_X86_REG_RAX,UC_X86_REG_RBX,UC_X86_REG_RCX,UC_X86_REG_RDX,UC_X86_REG_RSI,UC_X86_REG_RDI,UC_X86_REG_RBP,UC_X86_REG_R8,UC_X86_REG_R9,UC_X86_REG_R10,UC_X86_REG_R11,UC_X86_REG_R12,UC_X86_REG_R13,UC_X86_REG_R14,UC_X86_REG_R15]:cpu.reg_write(reg,0)
 for reg in [UC_X86_REG_XMM0,UC_X86_REG_XMM1,UC_X86_REG_XMM2,UC_X86_REG_XMM3,UC_X86_REG_XMM4,UC_X86_REG_XMM5,UC_X86_REG_XMM6,UC_X86_REG_XMM7,UC_X86_REG_XMM8,UC_X86_REG_XMM9,UC_X86_REG_XMM10,UC_X86_REG_XMM11,UC_X86_REG_XMM12,UC_X86_REG_XMM13,UC_X86_REG_XMM14,UC_X86_REG_XMM15]:cpu.reg_write(reg,0)
def uniform():
 global mode
 saved=mode;mode='rng';cpu.reg_write(UC_X86_REG_RCX,DATA+0x500);cpu.reg_write(UC_X86_REG_RSP,STACK+0x7000);cpu.mem_write(STACK+0x7000,struct.pack('<Q',STACK));cpu.emu_start(0x141625ae0,STACK,count=200);mode=saved
 return f'{cpu.reg_read(UC_X86_REG_XMM0)&0xffffffff:08x}'
def goal(case):
 global mode,rng_calls
 setup();seed,advance,timer,pos,vel,has,old,dt,enabled=case
 cpu.mem_write(DATA+0x500,((seed<<1|1)&((1<<128)-1)).to_bytes(16,'little'))
 for _ in range(advance):uniform()
 cpu.mem_write(DATA,struct.pack('<fff',*pos));cpu.mem_write(DATA+0x100,struct.pack('<fff',*vel));cpu.mem_write(DATA+0x200,struct.pack('<f',timer));cpu.mem_write(DATA+0x300,struct.pack('<Ifff',has,*old))
 before=bytes(cpu.mem_read(DATA+0x300,16));rng_calls=0;mode='goal'
 if enabled:
  sp=STACK+0x8000;cpu.reg_write(UC_X86_REG_RSP,sp);cpu.mem_write(sp+0x24,struct.pack('<I',123));cpu.mem_write(sp+0x90,struct.pack('<Q',DATA+0x500));
  cpu.reg_write(UC_X86_REG_RBP,DATA-0x10);cpu.reg_write(UC_X86_REG_RBX,DATA+0x200);cpu.reg_write(UC_X86_REG_RSI,DATA+0x300);cpu.reg_write(UC_X86_REG_R13,DATA+0x100);cpu.reg_write(UC_X86_REG_R14,DATA+0x400);cpu.reg_write(UC_X86_REG_RCX,DATA+0x410)
  for reg,val in [(UC_X86_REG_XMM7,dt),(UC_X86_REG_XMM8,0.0),(UC_X86_REG_XMM10,0.15),(UC_X86_REG_XMM11,0.05),(UC_X86_REG_XMM12,0.5),(UC_X86_REG_XMM13,1.0),(UC_X86_REG_XMM14,65.0)]:setx(reg,val)
  cpu.emu_start(0x142316d79,STACK,count=5000);assert cpu.reg_read(UC_X86_REG_RIP)==0x142316a96
 count=rng_calls;raw_state=bytes(cpu.mem_read(DATA+0x500,16)).hex();g=bytes(cpu.mem_read(DATA+0x300,16));t=struct.unpack('<I',cpu.mem_read(DATA+0x200,4))[0]
 # Undefined payload for None must remain untouched; semantic comparison excludes it but records preservation.
 preserve=(struct.unpack('<I',g[:4])[0]!=0 or g[4:]==before[4:])
 output=(f'{t:08x} '+' '.join(f'{x:08x}' for x in struct.unpack('<III',g[4:])) if struct.unpack('<I',g[:4])[0] else f'{t:08x} none')
 follow=' '.join(uniform() for _ in range(4));return output+' | '+follow,{'rng_calls':count,'raw_rng_state':raw_state,'none_payload_preserved':preserve,'goal_bytes':g.hex()}
def velocity(case):
 global mode
 setup();pos,has,old,target,retention,m=case;cpu.mem_write(DATA+0x10,struct.pack('<fff',*pos));cpu.mem_write(DATA+0x100,struct.pack('<Ifff',has,*old));cpu.mem_write(DATA+0x200,struct.pack('<ffff',*target,retention));cpu.mem_write(DATA+0x300,struct.pack('<f',m));sp=STACK+0x8000;cpu.mem_write(sp+0x54,struct.pack('<I',111));cpu.mem_write(sp+0x74,struct.pack('<I',222));cpu.reg_write(UC_X86_REG_R8,DATA);cpu.reg_write(UC_X86_REG_R14,DATA+0x100);cpu.reg_write(UC_X86_REG_RDX,DATA+0x200);cpu.reg_write(UC_X86_REG_R9,DATA+0x400);cpu.reg_write(UC_X86_REG_RSI,DATA+0x410);cpu.reg_write(UC_X86_REG_R11,DATA+0x300);setx(UC_X86_REG_XMM0,1.0);setx(UC_X86_REG_XMM1,0.25);mode='velocity';cpu.emu_start(0x142317430,STACK,count=1000);assert cpu.reg_read(UC_X86_REG_RIP)==0x14231715b
 return ' '.join(f'{x:08x}' for x in struct.unpack('<IIII',cpu.mem_read(DATA+0x200,16))+struct.unpack('<I',cpu.mem_read(DATA+0x300,4)))
def smooth(case):
 global mode
 setup();vel,t,retention,dt=case;cpu.mem_write(DATA,struct.pack('<ffff',*t,retention));cpu.mem_write(DATA+0x100,struct.pack('<fff',*vel));cpu.mem_write(STACK+0x8000+0x30,struct.pack('<Q',DATA+0x100));cpu.mem_write(STACK+0x8000+0x58,struct.pack('<Q',DATA+0x400));cpu.reg_write(UC_X86_REG_RBP,DATA);cpu.reg_write(UC_X86_REG_RBX,0);cpu.reg_write(UC_X86_REG_R13,0);cpu.reg_write(UC_X86_REG_RDI,0);setx(UC_X86_REG_XMM6,dt);setx(UC_X86_REG_XMM7,1.0);mode='smooth';cpu.emu_start(0x142318e67,STACK,count=1000);assert cpu.reg_read(UC_X86_REG_RIP)==0x142318ec7
 return ' '.join(f'{x:08x}' for x in struct.unpack('<III',cpu.mem_read(DATA+0x100,12)))
def hx(v):return f'{bits(v):08x}'
def main():
 rng=random.Random(20261011);inputs=[];expected=[];metadata=[]
 goals=[]
 for i in range(768):
  pos=tuple(f32(rng.uniform(-70,70)) for _ in range(3));vel=tuple(f32(rng.uniform(-2,2)) for _ in range(3));old=tuple(f32(p+rng.uniform(-1,1)) for p in pos)
  goals.append((rng.getrandbits(120),i%13,f32(rng.uniform(-0.1,0.5)),pos,vel,i%2,old,f32(rng.uniform(0,0.2)),i%7!=0))
 for t in [0.0,-0.0,0.05,math.nan,math.inf,-math.inf]:
  for has in [0,1]:goals.append((0x606f,0,t,(65.0,1.0,0.0),(0.0,0.0,0.0),has,(65.1,1.0,0.0),0.0,True))
 for c in goals:
  seed,advance,t,pos,vel,has,old,dt,en=c;inputs.append('goal '+f'{seed:x} {advance} '+ ' '.join([hx(t),*(hx(x) for x in pos),*(hx(x) for x in vel),str(has),*(hx(x) for x in old),hx(dt),str(int(en))]));a,m=goal(c);expected.append(a);metadata.append({'kind':'goal',**m})
 edges=[0.0,-0.0,0.25,1.0,math.nan,math.inf,-math.inf,fl(1),fl(0x7f7fffff)]
 vc=[]
 for y in edges:
  for has in [0,1]:vc.append(((0.0,y,0.0),has,(0.1,y,0.0),(1.0,2.0,3.0),0.05,0.0))
 for _ in range(768):
  pos=tuple(f32(rng.uniform(-3,3)) for _ in range(3));old=tuple(f32(rng.uniform(-3,3)) for _ in range(3));vc.append((pos,rng.randrange(2),old,(1.0,2.0,3.0),0.05,2.0))
 for c in vc:
  pos,has,old,t,ret,m=c;inputs.append('velocity '+' '.join([*(hx(x) for x in pos),str(has),*(hx(x) for x in old),*(hx(x) for x in t),hx(ret),hx(m)]));expected.append(velocity(c));metadata.append({'kind':'velocity'})
 sc=[]
 for dt in [0.0,0.016,1.0,-0.016,math.inf,math.nan]:
  for ret in [0.0,0.05,1.0,-0.05]:sc.append(((1.0,-2.0,0.25),(0.5,0.0,-1.0),ret,dt))
 for _ in range(512):sc.append((tuple(f32(rng.uniform(-2,2)) for _ in range(3)),tuple(f32(rng.uniform(-2,2)) for _ in range(3)),f32(rng.uniform(0,1)),f32(rng.uniform(0,0.1))))
 for c in sc:
  vel,t,ret,dt=c;inputs.append('smooth '+' '.join(hx(x) for x in (*vel,*t,ret,dt)));expected.append(smooth(c));metadata.append({'kind':'smooth'})
 binary=ROOT/'reconstruction/critters/harness/target/release/critters-probe';actual=subprocess.run([str(binary)],input='\n'.join(inputs)+'\n',capture_output=True,text=True,check=True).stdout.splitlines();assert len(actual)==len(inputs)
 failures=[]
 for i,(a,e) in enumerate(zip(actual,expected)):
  if a!=e:failures.append({'case':i,'input':inputs[i],'original':e,'rust':a,'kind':metadata[i]['kind']})
 counts={k:sum(m['kind']==k for m in metadata) for k in ['goal','velocity','smooth']};report={'ok':not failures,'baseline':'B','source_sha256':sha,'oracle_binary':str(ORIGINAL),'candidate_binary':str(binary),'candidate_sha256':hashlib.sha256(binary.read_bytes()).hexdigest(),'counts':counts,'failures':failures,'none_payload_preserved':all(m.get('none_payload_preserved',True) for m in metadata),'rng_call_histogram':{str(n):sum(m.get('rng_calls')==n for m in metadata) for n in [0,3]},'samples':metadata,'comparison':'Exact semantic f32 bits/Option tag plus four subsequent uniform_f32 bit values; raw original RNG state recorded. None payload separately preservation-checked.','ranges':{'goal':['0x142316d79','0x142316a96'],'velocity':['0x142317430','0x14231715b'],'smooth':['0x142318e67','0x142318ec7']},'bridges':['Selected entity pointers instead of executing Bevy query iteration','CRT sinf 0x142923fd0, cosf 0x142923e80, powf 0x142923f70 to current host libc','Control disabled skips entry (equivalent caller branch); paused full caller not executed'],'limits':['No startup/terrain/building/scheduler/delete/full lifecycle acceptance','No C verification','Four future RNG values are a state fingerprint; candidate private state not read','Host libc is not Windows CRT; all mismatches preserved','Finite/nonfinite edge coverage is sampled; SSE exception flags excluded'],'host':platform.platform(),'source_hashes':{str(p.relative_to(ROOT)):hashlib.sha256(p.read_bytes()).hexdigest() for p in [ROOT/'reconstruction/critters/firefly.rs',ROOT/'reconstruction/critters/mod.rs',ROOT/'reconstruction/critters/harness/probe.rs',ROOT/'reconstruction/critters/harness/Cargo.toml',ROOT/'reconstruction/critters/harness/Cargo.lock',Path(__file__)]}}
 (OUT/'B-firefly-verification.json').write_text(json.dumps(report,indent=2)+'\n');print(json.dumps({'ok':report['ok'],'counts':counts,'failures':len(failures),'first_failures':failures[:4]}));
 if failures:raise SystemExit(1)
if __name__=='__main__':main()
