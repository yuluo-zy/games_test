"""Original complete per-entity setvelocity CFG141d86cb7..141d8691a.
Bevy query traversal/ticks outside admission; all normalizing arithmetic,
four original state handlers and writes execute unchanged.
"""
from pathlib import Path
import json,struct,math
from unicorn import Uc,UC_ARCH_X86,UC_MODE_64,UC_HOOK_CODE,UC_HOOK_MEM_UNMAPPED
from unicorn.x86_const import *
O=Path(__file__).resolve().parent
scope={'__file__':str(O/'native_butterfly_state.py')};exec((O/'native_butterfly_state.py').read_text().split('def execute(c):')[0],scope)
read,sha,bits=scope['read'],scope['sha'],scope['bits']
def execute(c):
 u=Uc(UC_ARCH_X86,UC_MODE_64);u.mem_map(0x60000000,0x20000);u.mem_map(0x70000000,0x100000)
 tr,st,goal,target,mult,tick=range(0x60001000,0x60007000,0x1000);sp=0x70080000
 def missing(uc,access,a,n,value,data):
  page=a&~4095
  if 0x140000000<=page<0x144000000:
   try:b=read(page,4096)
   except ValueError:return False
   uc.mem_map(page,4096);uc.mem_write(page,b);return True
  return False
 def code(uc,a,n,data):
  if a==0x141d8691a:uc.emu_stop()
 u.hook_add(UC_HOOK_MEM_UNMAPPED,missing);u.hook_add(UC_HOOK_CODE,code)
 u.mem_write(tr,struct.pack('<10f',0,0,0,1,*c['position'],1,1,1)+bytes(8));u.mem_write(st,struct.pack('<I5f',c['state'],7,2,.3,c['phase'],c['phase_ref']));u.mem_write(goal,struct.pack('<I3f',c['goal'],*c['target']));u.mem_write(target,struct.pack('<4f',11,12,13,.05));u.mem_write(mult,struct.pack('<f',99));u.mem_write(sp+0x6c,struct.pack('<I',77));u.mem_write(sp+0x4c,struct.pack('<I',88))
 for reg,value in [(UC_X86_REG_RSP,sp),(UC_X86_REG_R15,tr),(UC_X86_REG_R12,st),(UC_X86_REG_R13,goal),(UC_X86_REG_RBX,target),(UC_X86_REG_RDI,mult),(UC_X86_REG_RDX,tick),(UC_X86_REG_R9,tick+4)]:u.reg_write(reg,value)
 for reg,values in [(UC_X86_REG_XMM0,[1,0,0,0]),(UC_X86_REG_XMM1,[1.76,0,0,0]),(UC_X86_REG_XMM2,[1.76,1.76,0,0]),(UC_X86_REG_XMM3,[.5,0,0,0]),(UC_X86_REG_XMM4,[1.5,0,0,0]),(UC_X86_REG_XMM5,[struct.unpack('<f',read(0x142a83cb8,4))[0],0,0,0])]:u.reg_write(reg,int.from_bytes(struct.pack('<4f',*values),'little'))
 try:u.emu_start(0x141d86cb7,0,count=10000)
 except Exception as e:raise RuntimeError(c['name'],hex(u.reg_read(UC_X86_REG_RIP)),str(e))
 assert u.reg_read(UC_X86_REG_RIP)==0x141d8691a
 return {'target_bits':list(struct.unpack('<4I',u.mem_read(target,16))),'multiplier_bits':struct.unpack('<I',u.mem_read(mult,4))[0]}
cases=[]
for state in range(4):
 for goal in [0,1]:
  for p,t in [([1,2,3],[4,6,9]),([0,0,0],[0,0,0]),([1,1.4999999,1],[4,6,9]),([1,1.5,1],[4,6,9]),([1,math.nan,1],[4,6,9]),([0,0,0],[math.inf,0,0]),([0,0,0],[math.nan,0,0]),([0,0,0],[1e-20,0,0]),([0,0,0],[1e-10,0,0]),([0,0,0],[0,-1,0])]:
   for phase,phase_ref in [(0.,0.),(.5,2.),(1.,0.),(math.nan,0.)]:cases.append({'name':f'{state}-{goal}-{p}-{t}-{phase}-{phase_ref}','state':state,'goal':goal,'position':p,'target':t,'phase':phase,'phase_ref':phase_ref})
def encode(x):
 if isinstance(x,float):return {'f32':bits(x)}
 if isinstance(x,list):return [encode(v) for v in x]
 if isinstance(x,dict):return {k:encode(v) for k,v in x.items()}
 return x
rows=[{'input':c,'output':execute(c)} for c in cases]
(O/'native-butterfly-velocity-fixtures.json').write_text(json.dumps({'original_sha256':sha,'scope':__doc__,'cases':encode(rows)},indent=2),encoding='utf8');print('actual original velocity CFG cases',len(rows))
