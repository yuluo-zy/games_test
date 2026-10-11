"""Untouched B RNE instruction oracle. Owned heap/stack; no shared output."""
from pathlib import Path
import json,struct,ctypes,ctypes.util,math,hashlib
import pefile
from unicorn import *
from unicorn.x86_const import *
ROOT=Path(__file__).resolve().parents[2];GAME=ROOT.parents[2]
b=json.loads((ROOT/'tracking/baselines.json').read_text())['baselines']['B'];raw=(GAME/'ljxsj_92385/Tiny Glade/tiny-glade.rne').read_bytes();assert hashlib.sha256(raw).hexdigest()==b['binary']['sha256'];pe=pefile.PE(data=raw);base=pe.OPTIONAL_HEADER.ImageBase;image=pe.get_memory_mapped_image()
cpu=Uc(UC_ARCH_X86,UC_MODE_64);cpu.mem_map(base,(pe.OPTIONAL_HEADER.SizeOfImage+4095)&~4095);cpu.mem_write(base,image)
data,stack,heap,stop=0x20000000,0x30000000,0x40000000,0x50000000
cpu.mem_map(data,0x100000);cpu.mem_map(stack,0x200000);cpu.mem_map(heap,0x1000000);cpu.mem_map(stop,0x1000);allocation=[heap];last=[];queries=[]
def u64(p):return struct.unpack('<Q',cpu.mem_read(p,8))[0]
def f32(x):return struct.unpack('<f',struct.pack('<f',x))[0]
def wf(p,x):cpu.mem_write(p,struct.pack('<f',x))
def fl(x):return struct.unpack('<f',struct.pack('<I',x&0xffffffff))[0]
def bits(x):return struct.unpack('<I',struct.pack('<f',x))[0]
def ret(v=None):
 if v is not None:cpu.reg_write(UC_X86_REG_RAX,v)
 sp=cpu.reg_read(UC_X86_REG_RSP);ra=u64(sp);cpu.reg_write(UC_X86_REG_RSP,sp+8);cpu.reg_write(UC_X86_REG_RIP,ra)
def alloc(n):
 p=allocation[0];allocation[0]+=max((n+15)&~15,0x1000);assert allocation[0]<heap+0x1000000;return p
lib=ctypes.CDLL(ctypes.util.find_library('m') or ctypes.util.find_library('c'));iat={x.address:x.name.decode() if x.name else '' for d in pe.DIRECTORY_ENTRY_IMPORT for x in d.imports};maths={}
for s in pe.sections:
 if s.Name.startswith(b'.text'):
  code=s.get_data();rva=s.VirtualAddress
  for i in range(len(code)-6):
   if code[i:i+2]==b'\xff\x25':
    pc=base+rva+i;ia=pc+6+struct.unpack_from('<i',code,i+2)[0];name=iat.get(ia)
    if name and name in ['sinf','cosf','expf','exp2f','fmodf','acosf','atan2f','sqrtf','tanf','asinf']:
     fn=getattr(lib,name);count=2 if name in ['fmodf','atan2f'] else 1;fn.argtypes=[ctypes.c_float]*count;fn.restype=ctypes.c_float;maths[pc]=(name,fn,count)
extra_hook=None

def hook(c,pc,size,user):
 if len(last)>60:last.pop(0)
 last.append(hex(pc))
 if extra_hook is not None and extra_hook(c,pc):return
 if pc in [0x140613c10,0x140613c40]:
  n=c.reg_read(UC_X86_REG_RCX);p=alloc(n)
  if pc==0x140613c40 and n:c.mem_write(p,bytes(n))
  ret(p)
 elif pc in [0x140613c20,0x14289aa40]:ret()
 elif pc==0x140613c30:
  old=c.reg_read(UC_X86_REG_RCX);oldn=c.reg_read(UC_X86_REG_RDX);newn=c.reg_read(UC_X86_REG_R9);p=alloc(newn);c.mem_write(p,bytes(c.mem_read(old,min(oldn,newn))));ret(p)
 elif pc in [0x142923c00,0x142923c10]:
  dst,src,n=(c.reg_read(r)for r in [UC_X86_REG_RCX,UC_X86_REG_RDX,UC_X86_REG_R8]);
  if n:c.mem_write(dst,bytes(c.mem_read(src,n)))
  ret(dst)
 elif pc==0x142923c20:
  dst,v,n=(c.reg_read(r)for r in [UC_X86_REG_RCX,UC_X86_REG_RDX,UC_X86_REG_R8]);
  if n:c.mem_write(dst,bytes([v&255])*n)
  ret(dst)
 elif pc==0x142923bf0:
  a,z,n=(c.reg_read(r)for r in [UC_X86_REG_RCX,UC_X86_REG_RDX,UC_X86_REG_R8]);aa=bytes(c.mem_read(a,n))if n else b'';zz=bytes(c.mem_read(z,n))if n else b'';ret((aa>zz)-(aa<zz))
 elif pc in maths:
  name,fn,count=maths[pc];args=[fl(c.reg_read(r))for r in [UC_X86_REG_XMM0,UC_X86_REG_XMM1][:count]];c.reg_write(UC_X86_REG_XMM0,bits(fn(*args)));ret()
 elif 0x1428d8f00<=pc<=0x1428f1800:raise RuntimeError('panic '+hex(pc))
cpu.hook_add(UC_HOOK_CODE,hook)
def setup(reset=True):
 if reset:
  allocation[0]=heap;cpu.mem_write(data,bytes(0x100000));cpu.mem_write(heap,bytes(0x100000));queries.clear()
 cpu.mem_write(stack,bytes(0x200000));last.clear()
 for r in [UC_X86_REG_RAX,UC_X86_REG_RBX,UC_X86_REG_RCX,UC_X86_REG_RDX,UC_X86_REG_RSI,UC_X86_REG_RDI,UC_X86_REG_RBP,UC_X86_REG_R8,UC_X86_REG_R9,UC_X86_REG_R10,UC_X86_REG_R11,UC_X86_REG_R12,UC_X86_REG_R13,UC_X86_REG_R14,UC_X86_REG_R15]:cpu.reg_write(r,0)
 for r in range(UC_X86_REG_XMM0,UC_X86_REG_XMM15+1):cpu.reg_write(r,0)
 sp=stack+0x100008;cpu.mem_write(sp,struct.pack('<Q',stop));cpu.reg_write(UC_X86_REG_RSP,sp);cpu.reg_write(UC_X86_REG_MXCSR,0x1f80);cpu.reg_write(UC_X86_REG_RFLAGS,0x202);return sp
def call(a,count=1000000):
 try:cpu.emu_start(a,stop,count=count)
 except Exception as e:raise RuntimeError(f'{e}; RIP={cpu.reg_read(UC_X86_REG_RIP):x}; last={last[-15:]}')from e
 assert cpu.reg_read(UC_X86_REG_RIP)==stop,hex(cpu.reg_read(UC_X86_REG_RIP))
def constructor(aspect=16/9):
 setup();cpu.reg_write(UC_X86_REG_RCX,data);cpu.reg_write(UC_X86_REG_XMM1,bits(aspect));call(0x14090ec70);return bytes(cpu.mem_read(data,0x4e0))
if __name__=='__main__':
 blob=constructor();report={'binary_sha256':b['binary']['sha256'],'entry':'0x14090ec70','native_returned':True,'main_camera_bytes':blob.hex(),'bridges':['allocation/free/reallocate','memcpy/memmove/memset','hostlibm import thunks'],'math_imports':{hex(k):v[0]for k,v in maths.items()}}
 (ROOT/'evidence/parallel-camera/B-constructor-native.json').write_text(json.dumps(report,indent=2)+'\n')
 for start in [0x100,0x1b8,0x270,0x2f0,0x390,0x448]:print(hex(start),blob[start:start+0x70].hex())
 for a in [0x138,0x150,0x18c,0x22c,0x2a8,0x400,0x480]:print(hex(a),struct.unpack_from('<8f',blob,a))
