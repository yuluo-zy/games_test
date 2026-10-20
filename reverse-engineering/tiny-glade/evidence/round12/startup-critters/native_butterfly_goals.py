"""Original complete goal per-entity CFG141d864af..141d861bf. Original
game sphere sampling and x0z helpers execute unchanged; only dependency RNG
draws and CRT cosf/sinf are boundaries. No Bevy traversal is emulated.
"""
from pathlib import Path
import json,struct,math,ctypes
from unicorn import Uc,UC_ARCH_X86,UC_MODE_64,UC_HOOK_CODE,UC_HOOK_MEM_UNMAPPED
from unicorn.x86_const import *
O=Path(__file__).resolve().parent;scope={'__file__':str(O/'native_butterfly_state.py')};exec((O/'native_butterfly_state.py').read_text().split('def execute(c):')[0],scope)
read,sha,bits=scope['read'],scope['sha'],scope['bits'];crt=ctypes.CDLL('ucrtbase');cos,sin=crt.cosf,crt.sinf
for f in [cos,sin]:f.argtypes=[ctypes.c_float];f.restype=ctypes.c_float
def execute(c):
 u=Uc(UC_ARCH_X86,UC_MODE_64);u.mem_map(0x60000000,0x20000);u.mem_map(0x70000000,0x100000)
 tr,st,goal,velocity,tick=range(0x60001000,0x60006000,0x1000);sp=0x70080000;draws=0
 def sf(reg):return struct.unpack('<f',struct.pack('<I',u.reg_read(reg)&0xffffffff))[0]
 def ret():
  p=u.reg_read(UC_X86_REG_RSP);u.reg_write(UC_X86_REG_RIP,struct.unpack('<Q',u.mem_read(p,8))[0]);u.reg_write(UC_X86_REG_RSP,p+8)
 def missing(uc,access,a,n,value,data):
  page=a&~4095
  if 0x140000000<=page<0x144000000:
   try:b=read(page,4096)
   except ValueError:return False
   uc.mem_map(page,4096);uc.mem_write(page,b);return True
  return False
 def code(uc,a,n,data):
  nonlocal draws
  if a==0x141d861bf:uc.emu_stop();return
  if a==0x141625ae0:
   v=c['random'][draws%len(c['random'])];draws+=1;uc.reg_write(UC_X86_REG_XMM0,bits(v));ret()
  elif a in [0x142923e80,0x142923fd0]:uc.reg_write(UC_X86_REG_XMM0,bits((cos if a==0x142923e80 else sin)(sf(UC_X86_REG_XMM0))));ret()
 u.hook_add(UC_HOOK_MEM_UNMAPPED,missing);u.hook_add(UC_HOOK_CODE,code)
 u.mem_write(tr,struct.pack('<10f',0,0,0,1,*c['position'],1,1,1)+bytes(8));u.mem_write(st,struct.pack('<I5f',c['state'],7,c['timer'],.3,0,.25));u.mem_write(goal,struct.pack('<I3f',c['goal'],*c['target']));u.mem_write(velocity,struct.pack('<3f',*c['velocity']));u.mem_write(sp+0x2c,struct.pack('<I',77))
 for reg,value in [(UC_X86_REG_RSP,sp),(UC_X86_REG_RBP,tr),(UC_X86_REG_RBX,st),(UC_X86_REG_RSI,goal),(UC_X86_REG_R15,velocity),(UC_X86_REG_R14,tick),(UC_X86_REG_RCX,tick+4)]:u.reg_write(reg,value)
 for reg,values in [(UC_X86_REG_XMM7,[c['dt'],0,0,0]),(UC_X86_REG_XMM8,[0]*4),(UC_X86_REG_XMM10,[.5,0,0,0]),(UC_X86_REG_XMM15,[1.76,0,0,0]),(UC_X86_REG_XMM9,[1,0,0,0]),(UC_X86_REG_XMM11,[8,0,0,0]),(UC_X86_REG_XMM12,[65,0,0,0])]:u.reg_write(reg,int.from_bytes(struct.pack('<4f',*values),'little'))
 try:u.emu_start(0x141d864af,0,count=30000)
 except Exception as e:raise RuntimeError(c['name'],hex(u.reg_read(UC_X86_REG_RIP)),str(e))
 assert u.reg_read(UC_X86_REG_RIP)==0x141d861bf
 return {'timer_bits':struct.unpack('<I',u.mem_read(st+8,4))[0],'goal_bits':list(struct.unpack('<4I',u.mem_read(goal,16))),'draws':draws}
cases=[]
for state in range(4):
 for goal in [0,1]:
  for timer in [-1.,0.,.05,.1,.10000001,1.,math.nan]:
   for pos,target,vel in [([0,1,0],[1,2,3],[1,2,3]),([0,1,0],[0,1,0],[0,0,0]),([0,7.99,0],[0,7.99,0],[0,4,0]),([65,0,0],[66,0,0],[10,0,0]),([66,0,0],[65,0,0],[-10,0,0]),([math.nan,0,0],[0,0,0],[0,0,0])]:
    c={'name':f'{state}-{goal}-{timer}-{pos}-{target}-{vel}','state':state,'goal':goal,'timer':timer,'position':pos,'target':target,'velocity':vel,'dt':.1,'random':[.125,.75,.5]};cases.append(c)
def encode(x):
 if isinstance(x,float):return {'f32':bits(x)}
 if isinstance(x,list):return [encode(v) for v in x]
 if isinstance(x,dict):return {k:encode(v) for k,v in x.items()}
 return x
rows=[{'input':c,'output':execute(c)} for c in cases]
(O/'native-butterfly-goals-fixtures.json').write_text(json.dumps({'original_sha256':sha,'scope':__doc__,'cases':encode(rows)},indent=2),encoding='utf8');print('actual original goal CFG cases',len(rows))
