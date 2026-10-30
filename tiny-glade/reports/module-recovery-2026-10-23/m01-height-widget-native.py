"""只执行原1409566d0；桥接allocator、format、UiState::add，捕获真实原WidgetParams。"""
from pathlib import Path
import struct,json,hashlib,itertools,pefile
from unicorn import Uc,UC_ARCH_X86,UC_MODE_64,UC_HOOK_CODE
from unicorn.x86_const import *
root=Path(__file__).resolve().parents[3]
raw=(root/'B版本内容/tiny-glade.rne').read_bytes();sha=hashlib.sha256(raw).hexdigest()
assert sha=='f507a6cea25c5199447b88ba746a5293e64a279f73797ca74c1ce7f4ff2b7d03'
p=pefile.PE(data=raw,fast_load=True);base=p.OPTIONAL_HEADER.ImageBase;image=p.get_memory_mapped_image()
u=Uc(UC_ARCH_X86,UC_MODE_64);u.mem_map(base,(len(image)+4095)&~4095);u.mem_write(base,image)
D,S,STOP=0x20000000,0x30000000,0x200ff000;u.mem_map(D,0x100000);u.mem_map(S,0x100000)
def q(a,v):u.mem_write(a,struct.pack('<Q',v))
def uq(a):return struct.unpack('<Q',u.mem_read(a,8))[0]
def words(a,n):return list(struct.unpack('<'+'I'*n,u.mem_read(a,4*n)))
def ret(v=None):
 rsp=u.reg_read(UC_X86_REG_RSP);u.reg_write(UC_X86_REG_RIP,uq(rsp));u.reg_write(UC_X86_REG_RSP,rsp+8)
 if v is not None:u.reg_write(UC_X86_REG_RAX,v)
def hook(uc,addr,size,ctx):
 global heap,captured
 if addr==0x140613c10:
  n=u.reg_read(UC_X86_REG_RCX);a=heap;heap=(heap+n+31)&~31;ret(a)
 elif addr==0x140d79f90:
  a=u.reg_read(UC_X86_REG_RCX);q(a,1);q(a+8,D+0x10000);q(a+16,1);ret(a)
 elif addr==0x1409231f0:
  a=u.reg_read(UC_X86_REG_R8);n=uq(a+0xa0);ptr=uq(a+0x98)
  captured={'points':words(ptr,n*3),'radius':words(a+0x8c,1)[0],
   'sound_tags':[words(a+0x18+i*24,1)[0]for i in range(4)],
   'sound_second_words':[words(a+0x1c+i*24,1)[0]for i in range(4)],
   'extra':list(struct.unpack('<2Q',u.mem_read(a+0x78,16))),
   'occlude':u.mem_read(a+0xa8,1)[0],'flags':u.mem_read(a+0xa9,1)[0]}
  u.emu_stop()
u.hook_add(UC_HOOK_CODE,hook)
rows=[]
for mode,flag,moved,rim,shape,n in itertools.product([0,1],[0,1],[0,1],[0,1],[0,2],[0,1,2,3,7]):
 u.mem_write(D,b'\0'*0x20000);heap=D+0x20000;captured=None
 obj,pub,points,heights=D+0x100,D+0x1000,D+0x3000,D+0x4000
 q(obj,42);u.mem_write(obj+8,bytes([flag,mode]));q(pub+0x90,points);q(pub+0x98,n);q(pub+0xe0,heights);q(pub+0xe8,n);u.mem_write(pub+0x258,struct.pack('<I',shape))
 ps=[v for i in range(n)for v in [0x80000000 if i==0 else 0x3f800000,0x41200000,0x40400000]]
 hs=([0x80000000,0,0x3f800000,0x7fc01234,0x7f800000,0xff800000,0xbe947ae2])[:n]
 if n:u.mem_write(points,struct.pack('<'+'I'*len(ps),*ps));u.mem_write(heights,struct.pack('<'+'I'*n,*hs))
 rsp=S+0xf0008;q(rsp,STOP);q(rsp+0x28,rim)
 for r,v in zip([UC_X86_REG_RCX,UC_X86_REG_RDX,UC_X86_REG_R8,UC_X86_REG_R9],[obj,D+0x5000,pub,moved]):u.reg_write(r,v)
 u.reg_write(UC_X86_REG_RSP,rsp);u.emu_start(0x1409566d0,STOP,count=20000)
 assert captured is not None
 rows.append(dict(mode=mode,flag=flag,moved=moved,rim=rim,shape=shape,input_points=ps,input_heights=hs,**captured))
result={'binary_sha256':sha,'original':'0x1409566d0','cases':rows}
Path(__file__).with_suffix('.json').write_text(json.dumps(result,indent=2))
print(json.dumps({'cases':len(rows),'sha256':sha,'sound_second_words':rows[0]['sound_second_words'],'extra':rows[0]['extra']}))
