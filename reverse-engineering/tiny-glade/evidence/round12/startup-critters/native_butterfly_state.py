"""Original complete per-entity CFG141d85655..141d85317, including all4
state handlers and continuation. Bevy traversal and owner admission precede
the admitted span. Camera/rock/water/dependency RNG results are explicit
inputs; all game branches, arithmetic, original jump tables and fmodf run.
"""
from pathlib import Path
import struct,json,math,ctypes,hashlib
from unicorn import Uc,UC_ARCH_X86,UC_MODE_64,UC_HOOK_CODE,UC_HOOK_MEM_UNMAPPED
from unicorn.x86_const import *
O=Path(__file__).resolve().parent;blob=Path('D:/game/ljxsj_92385/Tiny Glade/tiny-glade.rne').read_bytes();sha=hashlib.sha256(blob).hexdigest();assert sha=='f507a6cea25c5199447b88ba746a5293e64a279f73797ca74c1ce7f4ff2b7d03'
pe=struct.unpack_from('<I',blob,60)[0];sec=pe+24+struct.unpack_from('<H',blob,pe+20)[0];sections=[struct.unpack_from('<4I',blob,sec+i*40+8) for i in range(struct.unpack_from('<H',blob,pe+6)[0])]
def read(a,n):
 for vs,rv,rs,off in sections:
  if rv<=a-0x140000000 and a-0x140000000+n<=rv+max(vs,rs):return blob[off+a-0x140000000-rv:off+a-0x140000000-rv+n]
 raise ValueError(hex(a))
bits=lambda x:struct.unpack('<I',struct.pack('<f',x))[0]
crt=ctypes.CDLL('ucrtbase');fmod=crt.fmodf;fmod.argtypes=[ctypes.c_float]*2;fmod.restype=ctypes.c_float
def execute(c):
 u=Uc(UC_ARCH_X86,UC_MODE_64);u.mem_map(0x60000000,0x40000);u.mem_map(0x70000000,0x100000)
 bp=0x70080000;tr,vel,st,goal,tick,sheep=range(0x60001000,0x60007000,0x1000);calls=[];draws=0
 def sf(reg):return struct.unpack('<f',struct.pack('<I',u.reg_read(reg)&0xffffffff))[0]
 def ret():
  sp=u.reg_read(UC_X86_REG_RSP);u.reg_write(UC_X86_REG_RIP,struct.unpack('<Q',u.mem_read(sp,8))[0]);u.reg_write(UC_X86_REG_RSP,sp+8)
 def missing(uc,access,a,n,value,data):
  page=a&~4095
  if 0x140000000<=page<0x144000000:
   try:b=read(page,4096)
   except ValueError:return False
   uc.mem_map(page,4096);uc.mem_write(page,b);return True
  return False
 def code(uc,a,n,data):
  nonlocal draws
  if a==0x141d85317:uc.emu_stop();return
  if a==0x140911230:uc.reg_write(UC_X86_REG_RAX,c['camera_mode']);ret()
  elif a==0x140911dd0:uc.mem_write(uc.reg_read(UC_X86_REG_RCX),struct.pack('<3f',*c['eye']));ret()
  elif a==0x141625ae0:
   value=c['random'][draws%len(c['random'])];draws+=1;uc.reg_write(UC_X86_REG_XMM0,bits(value));ret()
  elif a==0x1409ea800:
   calls.append({'kind':'rock','xz':[bits(sf(UC_X86_REG_XMM2)),bits(sf(UC_X86_REG_XMM3))]});uc.reg_write(UC_X86_REG_RAX,c['rock']);ret()
  elif a==0x140a14df0:
   calls.append({'kind':'water','xz':[bits(sf(UC_X86_REG_XMM1)),bits(sf(UC_X86_REG_XMM2))]});uc.reg_write(UC_X86_REG_XMM0,bits(c['water']));ret()
  elif a==0x142923f10:uc.reg_write(UC_X86_REG_XMM0,bits(fmod(sf(UC_X86_REG_XMM0),sf(UC_X86_REG_XMM1))));ret()
 u.hook_add(UC_HOOK_MEM_UNMAPPED,missing);u.hook_add(UC_HOOK_CODE,code)
 u.mem_write(tr,struct.pack('<10f',0,0,0,1,*c['position'],1,1,1)+bytes(8));u.mem_write(vel,struct.pack('<3f',*c['velocity']));u.mem_write(st,struct.pack('<I5f',c['state'],c['state_t'],c['goal_timer'],c['dwell'],c['phase'],c['phase_ref']));u.mem_write(goal,struct.pack('<I3f',c['goal'],1,1,1))
 for i,p in enumerate(c['sheep']):u.mem_write(sheep+i*8,struct.pack('<2f',*p))
 for off,value in [(0x130,tr),(0x138,vel),(0x100,st),(0x140,goal),(0x108,tick),(0x118,sheep),(0x110,tick+128)]:u.mem_write(bp+off,struct.pack('<Q',value))
 u.mem_write(bp+0x154,struct.pack('<I',77))
 for reg,value in [(UC_X86_REG_RBP,bp),(UC_X86_REG_RSP,bp-0x80),(UC_X86_REG_RBX,0),(UC_X86_REG_RSI,len(c['sheep'])*8)]:u.reg_write(reg,value)
 for reg,values in [(UC_X86_REG_XMM6,[1,0,0,0]),(UC_X86_REG_XMM7,[.5,0,0,0]),(UC_X86_REG_XMM9,[c['dt'],0,0,0]),(UC_X86_REG_XMM10,[c['cursor'][0],c['cursor'][2],0,0]),(UC_X86_REG_XMM11,[.5*c['dt'],0,0,0]),(UC_X86_REG_XMM12,[3,0,0,0]),(UC_X86_REG_XMM13,[4,0,0,0]),(UC_X86_REG_XMM14,[0]*4),(UC_X86_REG_XMM15,[4,0,0,0])]:u.reg_write(reg,int.from_bytes(struct.pack('<4f',*values),'little'))
 try:u.emu_start(0x141d85655,0,count=30000)
 except Exception as e:raise RuntimeError(c['name'],hex(u.reg_read(UC_X86_REG_RIP)),str(e))
 assert u.reg_read(UC_X86_REG_RIP)==0x141d85317,(c['name'],hex(u.reg_read(UC_X86_REG_RIP)))
 return {'state_bits':list(struct.unpack('<6I',u.mem_read(st,24))),'draws':draws,'calls':calls}
def base(name,state):return {'name':name,'state':state,'state_t':7.,'goal_timer':2.,'dwell':.3,'phase':0.,'phase_ref':.25,'velocity':[0,0,0],'dt':.1,'position':[5,.5,7],'camera_mode':0,'eye':[100,0,100],'cursor':[100,0,100],'sheep':[],'goal':1,'water':0.,'rock':0,'random':[.125,.75,.5,.25]}
cases=[]
for state in range(4):
 for dwell in [-1.,-.1,0.,.1,.3,math.nan,math.inf]:
  for state_t in [-1.,0.,.05,7.,math.nan]:
   c=base(f's{state}-d{dwell}-t{state_t}',state);c.update(dwell=dwell,state_t=state_t);cases.append(c)
for state in range(4):
 for key,values in [('phase',[0.,-.0,.01,1.,math.nan]),('phase_ref',[.05,.25,.3,.5,.7,.95,math.nan]),('dt',[0.,-.1,1.,math.nan]),('goal',[0,1]),('water',[.009999,.01,.010001,math.nan]),('rock',[0,1]),('position',[[5,.5999999,7],[5,.6,7],[5,.6000001,7],[5,1.4999999,7],[5,1.5,7],[5,math.nan,7]]),('velocity',[[0,-1,0],[0,0,0],[0,1,0],[0,math.nan,0]])]:
  for value in values:
   c=base(f's{state}-{key}-{value}',state);c[key]=value;cases.append(c)
 for mode in [0,3]:
  for dist in [0.,1.9999999,2.,2.0000002,2.9999998,3.,3.0000002,math.nan]:
   c=base(f's{state}-mode{mode}-distance{dist}',state);c.update(camera_mode=mode,eye=[5+dist,0,7],cursor=[5+dist,0,7]);cases.append(c)
 for ps in [[],[[5,7]],[[7,7]],[[7.000001,7]],[[math.nan,7]],[[100,100],[5,7]]]:
  c=base(f's{state}-sheep{ps}',state);c['sheep']=ps;cases.append(c)
def encode(x):
 if isinstance(x,float):return {'f32':bits(x)}
 if isinstance(x,list):return [encode(v) for v in x]
 if isinstance(x,dict):return {k:encode(v) for k,v in x.items()}
 return x
rows=[{'input':c,'output':execute(c)} for c in cases]
(O/'native-butterfly-state-fixtures.json').write_text(json.dumps({'original_sha256':sha,'scope':__doc__,'cases':encode(rows)},indent=2),encoding='utf8');print('actual original state CFG cases',len(rows))
