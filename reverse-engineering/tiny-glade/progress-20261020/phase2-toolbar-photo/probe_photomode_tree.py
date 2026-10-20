from pathlib import Path
import pefile,struct,json,hashlib
from unicorn import Uc,UC_ARCH_X86,UC_MODE_64,UC_HOOK_CODE
from unicorn.x86_const import *
ROOT=Path(__file__).resolve().parents[4];HERE=Path(__file__).resolve().parent
data=(ROOT/'reverse-engineering/tiny-glade/workspace/input/tiny-glade.exe').read_bytes()
assert hashlib.sha256(data).hexdigest()=='f507a6cea25c5199447b88ba746a5293e64a279f73797ca74c1ce7f4ff2b7d03'
p=pefile.PE(data=data,fast_load=True);image=p.get_memory_mapped_image();base=p.OPTIONAL_HEADER.ImageBase
results=[]
for demo in (False,True):
 u=Uc(UC_ARCH_X86,UC_MODE_64);u.mem_map(base,(len(image)+4095)&~4095);u.mem_write(base,image);u.mem_map(0x100000,0x2000000);heap=[0x200000]
 def q(a):return struct.unpack('<Q',u.mem_read(a,8))[0]
 def ret(v=0):
  rsp=u.reg_read(UC_X86_REG_RSP);u.reg_write(UC_X86_REG_RAX,v);u.reg_write(UC_X86_REG_RIP,q(rsp));u.reg_write(UC_X86_REG_RSP,rsp+8)
 def hook(u,a,size,_):
  if a==0x140613c10:
   n=u.reg_read(UC_X86_REG_RCX);where=heap[0];heap[0]+=(n+31)&~31;ret(where)
  elif a==0x142923c00:
   dst=u.reg_read(UC_X86_REG_RCX);src=u.reg_read(UC_X86_REG_RDX);n=u.reg_read(UC_X86_REG_R8);u.mem_write(dst,bytes(u.mem_read(src,n)));ret(dst)
  elif a==0x140faa6e0:ret() # storage-only temporary drop; final tree copied beforehand
  elif not 0x14262472d<=a<0x142625ad8:raise RuntimeError(f'unexpected external {a:x}')
 u.hook_add(UC_HOOK_CODE,hook)
 u.mem_write(0x14362c098,struct.pack('<Q',0x110000));u.mem_write(0x110000,bytes([demo]))
 for reg,val in [(UC_X86_REG_RBP,0x180000),(UC_X86_REG_RSP,0x17ff80),(UC_X86_REG_RBX,0x130000),(UC_X86_REG_R14,0x140000),(UC_X86_REG_RDI,0x150000)]:u.reg_write(reg,val)
 u.emu_start(0x14262472d,0x142625ad8,count=20000)
 def item(address):
  b=bytes(u.mem_read(address,64));kind=b[0];label=q(address+0x20);length=q(address+0x28)
  out={'kind':kind,'label':bytes(u.mem_read(label,length)).decode() if length else '', 'size':struct.unpack_from('<ff',b,0x30),'show_only_fpv':b[0x38]}
  if kind in (4,5):out['children']=[item(q(address+0x10)+64*i) for i in range(q(address+0x18))]
  elif kind==0:
   payload=q(address+8);vtable=q(address+16)
   out.update(custom_payload=bytes(u.mem_read(payload,1)).hex() if payload>8 else '',vtable=hex(vtable),vtable_words=[hex(q(vtable+i*8)) for i in range(4)])
  elif kind in (2,3):out['sub']=b[1]
  elif kind==1:out['mode_raw']=b[8:24].hex()
  return out
 categories=[];array=u.reg_read(UC_X86_REG_R13)
 for i in range(11):
  a=array+48*i;label=q(a);length=q(a+8);count=q(a+40)
  categories.append({'label':bytes(u.mem_read(label,length)).decode(),'tag':u.mem_read(a+16,1)[0],'items':[item(q(a+32)+64*j) for j in range(count)]})
 results.append({'demo':demo,'categories':categories,'binary_sha256':hashlib.sha256(data).hexdigest(),'range':['14262472d','142625ad8']})
 print('demo',demo,[(c['tag'],c['label'],len(c['items'])) for c in categories])
(HERE/'photomode-original-trees.json').write_text(json.dumps(results,indent=2))
