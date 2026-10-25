"""Run unchanged B geometry body after its ECS lookup in Unicorn.
ECS/container entry and allocator/libc bridges are excluded from the business
oracle. The entire 1421a3490..5a90 geometry/noise range executes original bytes.
"""
from pathlib import Path
import json,struct,hashlib,ctypes,random
import pefile
from unicorn import Uc,UC_ARCH_X86,UC_MODE_64,UC_HOOK_CODE
from unicorn.x86_const import *
ROOT=Path(__file__).resolve().parent
raw=Path('D:/game/ljxsj_92385/Tiny Glade/tiny-glade.rne').read_bytes()
SHA='f507a6cea25c5199447b88ba746a5293e64a279f73797ca74c1ce7f4ff2b7d03'
assert hashlib.sha256(raw).hexdigest()==SHA
pe=pefile.PE(data=raw);base=pe.OPTIONAL_HEADER.ImageBase
cpu=Uc(UC_ARCH_X86,UC_MODE_64);cpu.mem_map(base,(pe.OPTIONAL_HEADER.SizeOfImage+4095)&~4095);cpu.mem_write(base,pe.get_memory_mapped_image())
DATA,STACK,HEAP,STOP=0x20000000,0x30000000,0x40000000,0x50000000
cpu.mem_map(DATA,0x100000);cpu.mem_map(STACK,0x200000);cpu.mem_map(HEAP,0x8000000);cpu.mem_map(STOP,0x1000)
heap=HEAP
crt=ctypes.CDLL('ucrtbase.dll')
math={}
for entry in pe.DIRECTORY_ENTRY_IMPORT:
 for imp in entry.imports:
  name=(imp.name or b'').decode()
  if name in ('sinf','cosf','powf','fmodf','sqrtf','log2f','floorf'):
   f=getattr(crt,name);f.restype=ctypes.c_float;f.argtypes=[ctypes.c_float]*(2 if name in ('powf','fmodf') else 1);math[imp.address]=(name,f)

def u64(a):return struct.unpack('<Q',cpu.mem_read(a,8))[0]
def bits(x):return struct.unpack('<I',struct.pack('<f',x))[0]
def ffrom(x):return struct.unpack('<f',struct.pack('<I',x&0xffffffff))[0]
def put64(a,v):cpu.mem_write(a,struct.pack('<Q',v))
def ret(v=None):
 if v is not None:cpu.reg_write(UC_X86_REG_RAX,v)
 sp=cpu.reg_read(UC_X86_REG_RSP);pc=u64(sp);cpu.reg_write(UC_X86_REG_RSP,sp+8);cpu.reg_write(UC_X86_REG_RIP,pc)

from collections import deque
trace=deque(maxlen=12)
def hook(c,pc,size,user):
 trace.append(hex(pc))
 global heap
 if pc in (0x140613c10,0x140613c40):
  n=c.reg_read(UC_X86_REG_RCX);p=heap;heap+=max((n+15)&~15,0x1000);assert heap<HEAP+0x8000000
  if pc==0x140613c40 and n:c.mem_write(p,bytes(n))
  ret(p)
 elif pc in (0x140613c20,0x14289aa40):ret()
 elif pc==0x140613c30:
  old,n,new=[c.reg_read(r) for r in (UC_X86_REG_RCX,UC_X86_REG_RDX,UC_X86_REG_R9)];p=heap;heap+=max((new+15)&~15,0x1000);c.mem_write(p,bytes(c.mem_read(old,min(n,new))));ret(p)
 elif pc in (0x142923c00,0x142923c10):
  dst,src,n=[c.reg_read(r) for r in (UC_X86_REG_RCX,UC_X86_REG_RDX,UC_X86_REG_R8)]
  if n:c.mem_write(dst,bytes(c.mem_read(src,n)))
  ret(dst)
 elif pc==0x142923c20:
  dst,v,n=[c.reg_read(r) for r in (UC_X86_REG_RCX,UC_X86_REG_RDX,UC_X86_REG_R8)]
  if n:c.mem_write(dst,bytes([v&255])*n)
  ret(dst)
 elif pc in thunks:
  name,f=thunks[pc];args=[ffrom(c.reg_read(UC_X86_REG_XMM0+i)) for i in range(len(f.argtypes))];c.reg_write(UC_X86_REG_XMM0,bits(f(*args)));ret()
 elif 0x1428d8f00<=pc<=0x1428f1800:raise RuntimeError('original panic '+hex(pc))
# Resolve exact ff25 RIP import thunks from the image; no game helper is bridged.
thunks={}
text=next(s for s in pe.sections if s.Name.rstrip(b'\0')==b'.text')
b=text.get_data();va=base+text.VirtualAddress
for i in range(len(b)-6):
 if b[i:i+2]==b'\xff\x25':
  a=va+i+6+struct.unpack_from('<i',b,i+2)[0]
  if a in math:thunks[va+i]=math[a]
cpu.hook_add(UC_HOOK_CODE,hook)
def run(roof):
 global heap
 heap=HEAP;cpu.mem_write(DATA,bytes(0x10000));cpu.mem_write(STACK,bytes(0x200000));cpu.mem_write(DATA,roof)
 for r in (UC_X86_REG_RAX,UC_X86_REG_RBX,UC_X86_REG_RCX,UC_X86_REG_RDX,UC_X86_REG_RSI,UC_X86_REG_RDI,UC_X86_REG_RBP,UC_X86_REG_R8,UC_X86_REG_R9,UC_X86_REG_R10,UC_X86_REG_R11,UC_X86_REG_R12,UC_X86_REG_R13,UC_X86_REG_R14,UC_X86_REG_R15):cpu.reg_write(r,0)
 for r in range(UC_X86_REG_XMM0,UC_X86_REG_XMM15+1):cpu.reg_write(r,0)
 sp=STACK+0x100000;bp=sp+0x80
 for r,v in [(UC_X86_REG_RSP,sp),(UC_X86_REG_RBP,bp),(UC_X86_REG_RSI,DATA),(UC_X86_REG_R13,DATA),(UC_X86_REG_R15,bp-0x2c),(UC_X86_REG_RBX,bp+0x3ac),(UC_X86_REG_R14,bp+0x8c),(UC_X86_REG_RDI,bp+0x3a0),(UC_X86_REG_MXCSR,0x1f80),(UC_X86_REG_RFLAGS,0x202),(UC_X86_REG_XMM9,bits(1.0))]:cpu.reg_write(r,v)
 try:cpu.emu_start(0x1421a3490,0x1421a5a90,count=15000000)
 except Exception as e:raise RuntimeError(str(e)+' @ '+hex(cpu.reg_read(UC_X86_REG_RIP))+' trace '+str(list(trace)))
 assert cpu.reg_read(UC_X86_REG_RIP)==0x1421a5a90,hex(cpu.reg_read(UC_X86_REG_RIP))
 def vec(offset,stride):
  n=u64(bp+offset+16);p=u64(bp+offset+8);return list(struct.unpack('<'+'I'*(n*stride//4),cpu.mem_read(p,n*stride)))
 return {'triangles':vec(0x440,36),'uvs':vec(0x420,24),'normals':vec(0x408,36)}
def make(rect,h=.5,ridge=.4,profile=.5,dir=0,floor=0,tip=(.3,-.2),rotation=.3):
 b=bytearray(88);struct.pack_into('<Q',b,0,7);struct.pack_into('<I',b,8,int(rect))
 if rect:struct.pack_into('<6f',b,12,__import__('math').cos(rotation),__import__('math').sin(rotation),3,4,4,6)
 else:struct.pack_into('<4f',b,12,3,4,2,rotation)
 struct.pack_into('<6fI',b,0x24,*tip,profile,h,ridge,.3,dir);struct.pack_into('<f',b,0x4c,floor);return bytes(b)
if __name__=='__main__':
 cases=[]
 for rect in [False,True]:
  for h in [.05,.5]:
   for ridge in [.4,1.0]:
    for d in [0,1]:
     b=make(rect,h,ridge,dir=d);v=run(b);cases.append({'roof_hex':b.hex(),**v});print(rect,h,ridge,d,len(v['triangles'])//9,flush=True)
 (ROOT/'native-trigger-fixtures.json').write_text(json.dumps({'source_sha256':SHA,'entry':'1421a3490 (after ECS lookup)','stop':'1421a5a90 (before ownership transfer)','bridges':['allocator/free/realloc','memcpy/memmove/memset','original-import ucrt f32 math'],'cases':cases},separators=(',',':'))+'\n')
