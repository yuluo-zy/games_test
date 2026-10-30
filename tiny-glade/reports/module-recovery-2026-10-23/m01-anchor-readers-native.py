"""原B14081c790前段四reader隔离真执行；只桥接存储与tracker容器边界。"""
from pathlib import Path
import struct, json, hashlib, itertools, pefile
from unicorn import Uc, UC_ARCH_X86, UC_MODE_64, UC_HOOK_CODE
from unicorn.x86_const import *
root=Path(__file__).resolve().parents[3]
raw=(root/'B版本内容/tiny-glade.rne').read_bytes();sha=hashlib.sha256(raw).hexdigest()
assert sha=='f507a6cea25c5199447b88ba746a5293e64a279f73797ca74c1ce7f4ff2b7d03'
p=pefile.PE(data=raw,fast_load=True);base=p.OPTIONAL_HEADER.ImageBase;image=p.get_memory_mapped_image()
u=Uc(UC_ARCH_X86,UC_MODE_64);u.mem_map(base,(len(image)+4095)&~4095);u.mem_write(base,image)
D,S,STOP=0x20000000,0x30000000,0x200ff000;u.mem_map(D,0x100000);u.mem_map(S,0x100000)
def q(a,v):u.mem_write(a,struct.pack('<Q',v))
def uq(a):return struct.unpack('<Q',u.mem_read(a,8))[0]
def ret(value=0):
 rsp=u.reg_read(UC_X86_REG_RSP);u.reg_write(UC_X86_REG_RAX,value);u.reg_write(UC_X86_REG_RIP,uq(rsp));u.reg_write(UC_X86_REG_RSP,rsp+8)
def hook(uc,a,n,c):
 if a==0x140c16a00:ret(0)
 elif a==0x140b26a70:ret(0)
 elif a==0x140a1f550:
  calls.append(['deleted',uc.reg_read(UC_X86_REG_RDX)]);ret()
 elif a==0x140a1e420:
  caller=uq(uc.reg_read(UC_X86_REG_RSP));wall=uc.reg_read(UC_X86_REG_RDX)
  kind={0x14081cc6d:'changed',0x14081cdb2:'roof',0x14081cf52:'height'}[caller]
  calls.append([kind,wall]);ret(D+0x25000 if kind=='changed' else 0)
 elif a==0x140a1b140:calls.append(['mark_all',0]);ret()
 elif a==0x1402ca2f0:
  owner=uc.reg_read(UC_X86_REG_RCX);calls.append(['chimney' if owner==D+0xe030 else 'stairs',uc.reg_read(UC_X86_REG_RDX)]);ret()
 elif a==0x140a1e540:uc.emu_stop()
u.hook_add(UC_HOOK_CODE,hook)
def reader(i,walls,stride,offset,split):
 a=D+0x100+i*0x100;cursor=D+0x1000+i*0x100;event=D+0x2000+i*0x100;arr=D+0x4000+i*0x1000
 q(a,cursor);q(a+8,event);old=min(split,len(walls));q(event,32);q(event+8,arr);q(event+16,old);q(event+24,0);q(event+32,32);q(event+40,arr+0x800);q(event+48,len(walls)-old);q(event+56,old);q(event+64,len(walls))
 for j,w in enumerate(walls):q((arr+j*stride if j<old else arr+0x800+(j-old)*stride)+offset,w)
 return a,cursor
rows=[]
for mask,split in itertools.product(range(16),[0,1]):
 u.mem_write(D,b'\0'*0x30000)
 streams=[[i*2+1,i*2+2] if mask&(1<<i) else [] for i in range(4)]
 readers=[reader(i,w,[16,16,32,48][i],[8,8,16,24][i],split)for i,w in enumerate(streams)]
 # 原ResMut的ptr/tick/tick_value；原两个set位于各自真实tracker+30。
 for header,payload in [(D+0xb000,D+0xe000),(D+0xb100,D+0xe100),(D+0xb200,D+0xe200)]:
  q(header,payload);q(header+16,D+0xf000);q(header+24,0)
 frames=[]
 for frame in range(2):
  calls=[];rsp=S+0xf0008;q(rsp,STOP)
  for off,value in [(0x28,readers[3][0]),(0x30,D+0xb000),(0x38,D+0xb100),(0x40,D+0xb200),(0x48,D+0xb300)]:q(rsp+off,value)
  for r,v in zip([UC_X86_REG_RCX,UC_X86_REG_RDX,UC_X86_REG_R8,UC_X86_REG_R9],[D+0xa000,*[a for a,c in readers[:3]]]):u.reg_write(r,v)
  u.reg_write(UC_X86_REG_RSP,rsp);u.emu_start(0x14081c790,STOP,count=50000)
  frames.append({'calls':calls,'cursors':[uq(c)for a,c in readers]})
 rows.append(dict(mask=mask,split=split,streams=streams,frames=frames))
Path(__file__).with_suffix('.json').write_text(json.dumps({'binary_sha256':sha,'original':'0x14081c790','cases':rows},indent=2))
Path(__file__).with_suffix('.txt').write_text('\n'.join(f"{r['mask']} {r['split']}|"+','.join(f'{k}:{w}' for k,w in r['frames'][0]['calls'])+'|'+','.join(f'{k}:{w}' for k,w in r['frames'][1]['calls']) for r in rows)+'\n')
print(json.dumps({'cases':len(rows),'frames':len(rows)*2,'sha':sha}))
