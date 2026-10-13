from pathlib import Path
import json,struct,random,math
import native_oracle as n
from native_oracle import *
OUT=Path(__file__).resolve().parent
calls=[]
def bridge(c,pc,size,_):
 if pc==0x142923fa0:
  x=struct.unpack('<f',struct.pack('<I',c.reg_read(UC_X86_REG_XMM0)&0xffffffff))[0]
  c.reg_write(UC_X86_REG_XMM0,bits(math.copysign(math.floor(abs(x)+.5),x)));ret()
 elif pc==0x1409e9640:
  x,z=[struct.unpack('<f',struct.pack('<I',c.reg_read(r)&0xffffffff))[0] for r in (UC_X86_REG_XMM1,UC_X86_REG_XMM2)]
  calls.append([bits(x),bits(z)])
  c.reg_write(UC_X86_REG_XMM0,bits(height));ret()
cpu.hook_add(UC_HOOK_CODE,bridge)
def decode(dim):
 a=DATA+0x800
 if u64(a)==1<<63:return {'error':cpu.mem_read(a+8,1)[0]}
 count=u64(a+16)
 return {'points':[floats(u64(a+8)+i*dim*4,dim) for i in range(count)],'points_u':floats(u64(a+32),count),'length':floats(a+48,1)[0]}
def prepare(points):
 assert 'error' not in n.curve(points,2)
 cpu.mem_write(DATA+0x400,bytes(cpu.mem_read(DATA,56)))
 arguments(DATA+0x800,DATA+0x400)
 sp=STACK+0x100008
 return sp
rows=[]
rng=random.Random(202610131)
for count in [2,3,4,6,8,11]:
 points=[[rng.uniform(-8,8),rng.uniform(-8,8)] for _ in range(count)]
 weights=[rng.choice([0,1,4,17,2**63,2**64-1]) for _ in points]
 for iterations in [0,1,3]:
  sp=prepare(points);cpu.mem_write(DATA+0x2000,struct.pack('<'+'Q'*count,*weights));cpu.reg_write(UC_X86_REG_R8,iterations);cpu.reg_write(UC_X86_REG_R9,DATA+0x2000);cpu.mem_write(sp+0x28,struct.pack('<Q',count));call(0x141468880)
  rows.append({'method':'weighted','points':[[bits(v) for v in p] for p in points],'iterations':iterations,'weights':weights,'expected':decode(2)})
for points in [[[0,0],[3,4],[6,4]],[[0,0],[0,0],[1,0]],[[0,0],[1,0],[0,0]]]:
 for spacing in [.5,2,10]:
  for midpoint in [False,True]:
   for parameter,height in [(-1,-4),(0,2),(3,1),(3,4),(float('nan'),float('nan'))]:
    sp=prepare(points);cpu.reg_write(UC_X86_REG_R8,int(midpoint));cpu.reg_write(UC_X86_REG_XMM3,bits(spacing));cpu.mem_write(sp+0x28,struct.pack('<Q',DATA+0x3000));cpu.mem_write(sp+0x30,struct.pack('<Q',DATA+0x4000));cpu.mem_write(DATA+0x4058,struct.pack('<f',parameter));calls.clear();call(0x14146bb70)
    rows.append({'method':'terrain','points':[[bits(v) for v in p] for p in points],'spacing':bits(spacing),'midpoint':midpoint,'parameter':bits(parameter),'height':bits(height),'calls':calls.copy(),'expected':decode(3)})
(OUT/'fixtures.json').write_text(json.dumps({'original_sha256':SHA,'bridges':['allocator/free/realloc','memcpy/memmove/memset','CRT roundf','terrain sampler constant response (only consumer callback, not terrain algorithm)'],'cases':rows},indent=2)+'\n')
print(json.dumps({'weighted':18,'terrain':len(rows)-18}))
