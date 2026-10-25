from pathlib import Path
import sys,struct,json,hashlib
sys.path.insert(0,str(Path('reverse-engineering/tiny-glade/evidence/round12/ui-picker').resolve()))
from native_picker_oracle import cpu,DATA,STACK,HEAP,STOP,put,u64,alloc,ret
from unicorn import UC_HOOK_CODE
from unicorn.x86_const import *
ROOT=Path('reverse-engineering/tiny-glade/evidence/round12/ui-update');ROOT.mkdir(exist_ok=True)
# Remove inherited code hook. A fresh CPU keeps original .text/rdata and uses only
# standard-library bridge functions; no game state-machine branch is intercepted.
from unicorn import Uc,UC_ARCH_X86,UC_MODE_64
import pefile
raw=Path('D:/game/ljxsj_92385/Tiny Glade/tiny-glade.rne').read_bytes();pe=pefile.PE(data=raw,fast_load=True);base=pe.OPTIONAL_HEADER.ImageBase
cpu=Uc(UC_ARCH_X86,UC_MODE_64);cpu.mem_map(base,(pe.OPTIONAL_HEADER.SizeOfImage+4095)&~4095);cpu.mem_write(base,pe.get_memory_mapped_image())
for p,n in [(DATA,0x100000),(STACK,0x200000),(STOP,0x1000)]:cpu.mem_map(p,n)
def put(a,fmt,*v):cpu.mem_write(a,struct.pack(fmt,*v))
def u64(a):return struct.unpack('<Q',cpu.mem_read(a,8))[0]
def ret(v=None):
 if v is not None:cpu.reg_write(UC_X86_REG_RAX,v)
 sp=cpu.reg_read(UC_X86_REG_RSP);pc=u64(sp);cpu.reg_write(UC_X86_REG_RSP,sp+8);cpu.reg_write(UC_X86_REG_RIP,pc)
trace=[];lookup=[]
def hook(c,pc,size,user):
 trace.append(pc)
 if pc==STOP:c.emu_stop();return
 if pc==0x140c16a00:ret(0) # disabled tracing spans
 elif pc==0x140b2dc60:
  key=c.reg_read(UC_X86_REG_RDX);lookup.append(key);arc=DATA+0x4000 if key==11 else DATA+0x4200;put(arc,'<Q',u64(arc)+1);ret(arc)
 elif pc==0x140b79e90:put(DATA+0x1000+0x80,'<Q',0);ret() # std HashMap.clear
 elif pc==0x1405b0d70:ret(0) # std hash probe fixed bucket0, real key comparison executes
 elif pc in (0x140c81c70,0x140c81e10):ret() # parking_lot lock debug hooks
 elif pc in (0x140562c30,0x140562630):raise RuntimeError('fixture event capacity exhausted')
cpu.hook_add(UC_HOOK_CODE,hook)
def run(tag=0,phase=2,hit=11,old=11,action=4,timer=0.,focus=None,escape=0,shutdown=0,registry=0,flags=0,preview_tag=0,geometry_tag=7):
 trace.clear();lookup.clear();cpu.mem_write(DATA,bytes(0x10000));cpu.mem_write(STACK,bytes(0x200000))
 state=DATA+0x1000;ac=DATA+0x1100
 put(ac,'<If',action,timer)
 # native Res and ResMut wrappers
 refs=[]
 for i,p in enumerate([state,ac,0,0,0,0,0,0]):
  ref=DATA+0x100+i*0x30;refs.append(ref)
  if i<2:put(ref,'<Q',p);put(ref+0x10,'<Q',DATA+0x900+i*4)
 def event(i,vals,stride):
  ev=DATA+0x2000+i*0x100;buf=DATA+0x6000+i*0x500;counter=DATA+0xa00+i*8
  n=len(vals);put(ev,'<9Q',64,buf,n,0,64,buf+0x200,0,n,n)
  for j,v in enumerate(vals):put(buf+j*stride,'<Q',j);put(buf+j*stride+8,'<B',v)
  if i<5:put(refs[i],'<QQ',counter,ev)
  else:put(refs[i],'<Q',ev);put(refs[i]+0x10,'<Q',DATA+0x900+i*4)
  return ev
 event(2,[] if focus is None else focus,16);event(3,[0]*escape,8);event(4,[0]*shutdown,8)
 writers=[event(i,[],32 if i!=6 else 16) for i in range(5,8)]
 def intersection(id):
  blob=bytearray(56);struct.pack_into('<I3fQ',blob,0,1,3.,4.,5.,id);struct.pack_into('<Q',blob,24,2);struct.pack_into('<B',blob,49,flags);return blob
 if hit is not None:put(state,'<3Q',8,DATA+0x3500,1);cpu.mem_write(DATA+0x3500,bytes(intersection(hit)))
 else:put(state,'<3Q',8,DATA+0x3500,0)
 put(state+0x18,'<BB',tag,phase);put(state+0x20,'<Q',DATA+0x4000 if old==11 else DATA+0x4200);cpu.mem_write(state+0x28,bytes(intersection(old)))
 if registry:
  ctrl=DATA+0x5000;put(state+0x68,'<4Q',ctrl,3,0,1);cpu.mem_write(ctrl,bytes([0]+[255]*31));put(ctrl-16,'<QQ',old,DATA+0x4400)
 for j,arc in enumerate([DATA+0x4000,DATA+0x4200,DATA+0x4400]):
  put(arc,'<QQ',100,1)
  for k in range(4):put(arc+0x28+24*k,'<6I',100*j+k+1,55,66,77,88,99)
  # No brush preview (native extra tag0).
  put(arc+0x88,'<BBH3f',preview_tag,7,0,1.,2.,3.)
  put(arc+0x98,'<B',geometry_tag)
  if geometry_tag==0:put(arc+0x9c,'<4f',1.,10.,20.,30.)
  elif geometry_tag==1:
   put(arc+0x9c,'<f',1.);put(arc+0xa0,'<3Q',3,DATA+0x4800,3);put(DATA+0x4800,'<9f',0.,0.,0.,10.,20.,30.,40.,50.,60.)
  elif geometry_tag==6:put(arc+0x9c,'<7f',10.,20.,30.,0.,0.,1.,2.)
 sp=STACK+0x1f0008;put(sp,'<Q',STOP)
 for i in range(4,8):put(sp+0x28+(i-4)*8,'<Q',refs[i])
 for reg,v in zip([UC_X86_REG_RCX,UC_X86_REG_RDX,UC_X86_REG_R8,UC_X86_REG_R9],refs):cpu.reg_write(reg,v)
 cpu.reg_write(UC_X86_REG_RSP,sp)
 cpu.emu_start(0x140b2de60,STOP,count=300000)
 if cpu.reg_read(UC_X86_REG_RIP)!=STOP:raise RuntimeError('not completed '+hex(cpu.reg_read(UC_X86_REG_RIP)))
 outs=[]
 for i,ev in enumerate(writers):
  n=u64(ev+0x30);stride=16 if i==1 else 32;ptr=u64(ev+0x28);outs.append([bytes(cpu.mem_read(ptr+j*stride,stride)).hex() for j in range(n)])
 return dict(tag=cpu.mem_read(state+0x18,1)[0],phase=cpu.mem_read(state+0x19,1)[0],widget=u64(state+0x38),arc=u64(state+0x20)-DATA,lookups=list(lookup),brush=outs[0],cursor=outs[1],sounds=outs[2])
if __name__=='__main__':
 cases=[]
 for tag,phase in [(0,2),(1,0),(1,1),(2,0),(2,1),(2,2),(2,3),(2,4)]:
  for hit in [None,11,22]:
   for action in range(5):
    for evt in ['none','escape','shutdown','focusfalse','focustrue']:
     kw=dict(tag=tag,phase=phase,hit=hit,action=action)
     if evt=='escape':kw['escape']=1
     elif evt=='shutdown':kw['shutdown']=1
     elif evt=='focusfalse':kw['focus']=[0]
     elif evt=='focustrue':kw['focus']=[1]
     result=run(**kw);cases.append(dict(inputs=kw,result=result))
 (ROOT/'native-state-fixtures.json').write_text(json.dumps(cases,indent=2));print(len(cases),'completed')
 for kw in [dict(tag=1,phase=1,action=0),dict(tag=2,phase=3,action=1),dict(tag=2,phase=4,action=1),dict(tag=2,phase=4,action=3),dict(tag=2,phase=4,action=1,escape=1),dict(tag=2,phase=4,action=1,focus=[0])]:print(kw,run(**kw))

