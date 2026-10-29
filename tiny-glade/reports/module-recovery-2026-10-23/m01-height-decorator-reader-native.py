"""原14081b5e0四reader真执行，closure边界仅记录wall，保留原双缓冲与未读游标。"""
from pathlib import Path
import struct,json,hashlib,itertools,pefile
from unicorn import Uc,UC_ARCH_X86,UC_MODE_64,UC_HOOK_CODE
from unicorn.x86_const import *
root=Path(__file__).resolve().parents[3];raw=(root/'B版本内容/tiny-glade.rne').read_bytes();sha=hashlib.sha256(raw).hexdigest()
assert sha=='f507a6cea25c5199447b88ba746a5293e64a279f73797ca74c1ce7f4ff2b7d03'
p=pefile.PE(data=raw,fast_load=True);base=p.OPTIONAL_HEADER.ImageBase;image=p.get_memory_mapped_image()
u=Uc(UC_ARCH_X86,UC_MODE_64);u.mem_map(base,(len(image)+4095)&~4095);u.mem_write(base,image)
D,S,STOP=0x20000000,0x30000000,0x200ff000;u.mem_map(D,0x100000);u.mem_map(S,0x100000)
def q(a,v):u.mem_write(a,struct.pack('<Q',v))
def uq(a):return struct.unpack('<Q',u.mem_read(a,8))[0]
def hook(uc,a,n,c):
 if a==0x14081b920:
  calls.append(uc.reg_read(UC_X86_REG_R8));rsp=uc.reg_read(UC_X86_REG_RSP);uc.reg_write(UC_X86_REG_RIP,uq(rsp));uc.reg_write(UC_X86_REG_RSP,rsp+8)
u.hook_add(UC_HOOK_CODE,hook)
def reader(i,walls,stride,offset,split):
 a=D+0x100+i*0x100;cursor=D+0x1000+i*0x100;event=D+0x2000+i*0x100;arr=D+0x4000+i*0x1000
 q(a,cursor);q(a+8,event);old=min(split,len(walls));q(event,32);q(event+8,arr);q(event+16,old);q(event+24,0);q(event+32,32);q(event+40,arr+0x800);q(event+48,len(walls)-old);q(event+56,old);q(event+64,len(walls))
 for j,w in enumerate(walls):q((arr+j*stride if j<old else arr+0x800+(j-old)*stride)+offset,w)
 return a,cursor
rows=[]
for cmd,ui,finish,reanchor,split in itertools.product([[],[1,2]],[[],[3,4]],[[],[5,6]],[[],[7,8]],[0,1]):
 u.mem_write(D,b'\0'*0x10000);readers=[reader(i,w,48 if i==0 else 16,24 if i==0 else 8,split)for i,w in enumerate([cmd,ui,finish,reanchor])]
 frames=[]
 for frame in range(2):
  calls=[];rsp=S+0xf0008;q(rsp,STOP);q(rsp+0x28,readers[3][0]);q(rsp+0x30,D+0xf000)
  for r,v in zip([UC_X86_REG_RCX,UC_X86_REG_RDX,UC_X86_REG_R8,UC_X86_REG_R9],[D+0xe000,*[a for a,c in readers[:3]]]):u.reg_write(r,v)
  u.reg_write(UC_X86_REG_RSP,rsp);u.emu_start(0x14081b5e0,STOP,count=50000)
  frames.append({'calls':calls,'cursors':[uq(c)for a,c in readers]})
 rows.append(dict(cmd=cmd,ui=ui,finish=finish,reanchor=reanchor,split=split,frames=frames))
Path(__file__).with_suffix('.json').write_text(json.dumps({'binary_sha256':sha,'original':'0x14081b5e0','cases':rows},indent=2))
lines=[]
for row in rows:
 mask=sum((bool(row[key])<<i)for i,key in enumerate(['cmd','ui','finish','reanchor']))
 lines.append(f"{mask} {row['split']}|"+','.join(map(str,row['frames'][0]['calls']))+'|'+','.join(map(str,row['frames'][1]['calls'])))
Path(__file__).with_suffix('.txt').write_text('\n'.join(lines)+'\n')
print(json.dumps({'cases':len(rows),'frames':len(rows)*2,'sha':sha}))
