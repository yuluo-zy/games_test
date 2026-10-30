import struct,hashlib,json,ctypes,pefile
from unicorn import Uc,UC_ARCH_X86,UC_MODE_64,UC_HOOK_CODE
from unicorn.x86_const import *
raw=open('B版本内容/tiny-glade.rne','rb').read();assert hashlib.sha256(raw).hexdigest()=='f507a6cea25c5199447b88ba746a5293e64a279f73797ca74c1ce7f4ff2b7d03'
p=pefile.PE(data=raw,fast_load=True);base=p.OPTIONAL_HEADER.ImageBase
u=Uc(UC_ARCH_X86,UC_MODE_64);u.mem_map(base,(p.OPTIONAL_HEADER.SizeOfImage+4095)&~4095);u.mem_write(base,p.get_memory_mapped_image());u.mem_map(0x30000000,0x400000);u.mem_map(0x40000000,0x4000000)
sp=0x30100008;wall=0x30010000;src=0x30020000;heap=0x40000000;observed={"buffers":[],"entities":[]};last=[]
w=lambda a,b:u.mem_write(a,b);q=lambda a,v:w(a,struct.pack('<Q',v));f=lambda a,v:w(a,struct.pack('<f',v));bits=lambda v:struct.unpack('<I',struct.pack('<f',v))[0];num=lambda v:struct.unpack('<f',struct.pack('<I',v&0xffffffff))[0]
lib=ctypes.CDLL('/usr/lib/libSystem.B.dylib');bridge={0x142923e60:('ceilf',1),0x142923ef0:('fmaf',3),0x142923fa0:('roundf',1),0x142923f10:('fmodf',2),0x142923e80:('cosf',1),0x142923fd0:('sinf',1)}
for n,k in bridge.values():getattr(lib,n).argtypes=[ctypes.c_float]*k;getattr(lib,n).restype=ctypes.c_float
def ret(c,value=None):
 if value is not None:c.reg_write(UC_X86_REG_RAX,value)
 s=c.reg_read(UC_X86_REG_RSP);c.reg_write(UC_X86_REG_RIP,struct.unpack('<Q',c.mem_read(s,8))[0]);c.reg_write(UC_X86_REG_RSP,s+8)
def hook(c,pc,size,_):
 global heap
 last.append(pc);del last[:-30]
 if pc==0x14122850f:
  rsp=c.reg_read(UC_X86_REG_RSP);ptr,n=struct.unpack('<QQ',c.mem_read(rsp+0x28,16));observed['buffers'].append(bytes(c.mem_read(ptr,n)).hex());out=c.reg_read(UC_X86_REG_RCX);c.mem_write(out,struct.pack('<I',11)+bytes(44));c.reg_write(UC_X86_REG_RIP,0x141228514);return
 if pc==0x140bd4c00:
  ret(c,0x60000000+(len(observed['buffers'])-1)*0x100000);return
 if pc==0x141228bdd:
  rbp=c.reg_read(UC_X86_REG_RBP);ptr=struct.unpack('<Q',c.mem_read(rbp+0x330,8))[0];n=struct.unpack('<Q',c.mem_read(rbp+0x338,8))[0];observed.update(strips=bytes(c.mem_read(ptr,n*32)).hex(),strip_count=n);return
 if pc==0x14120f320:
  out=c.reg_read(UC_X86_REG_RCX);c.mem_write(out,bytes(40));c.mem_write(out+32,struct.pack('<Q',0x100000001));observed['entities'].append('owner');ret(c);return
 if pc==0x140d146a0:
  out=c.reg_read(UC_X86_REG_RCX);c.mem_write(out,bytes(40));c.mem_write(out+8,struct.pack('<Q',0x30038000));c.mem_write(out+32,struct.pack('<Q',0x100000002+len(observed['entities'])));observed['entities'].append('strip');ret(c);return
 if pc==0x1411fc890:
  out=c.reg_read(UC_X86_REG_RCX);v=c.reg_read(UC_X86_REG_R9);cap,ptr,n=struct.unpack('<QQQ',c.mem_read(v,24));observed['registry_entities']=list(struct.unpack('<'+'Q'*n,c.mem_read(ptr,n*8)));c.mem_write(out,struct.pack('<Q',0x8000000000000000)+bytes(16));ret(c);return
 if pc in bridge:
  n,k=bridge[pc];vals=[num(c.reg_read(r)) for r in [UC_X86_REG_XMM0,UC_X86_REG_XMM1,UC_X86_REG_XMM2][:k]];c.reg_write(UC_X86_REG_XMM0,bits(getattr(lib,n)(*vals)));ret(c);return
 if pc in (0x140987ca0,0x140a922b0):ret(c,0);return
 if pc==0x140613c10:
  n=c.reg_read(UC_X86_REG_RCX);align=c.reg_read(UC_X86_REG_RDX);heap=(heap+align-1)&-align;v=heap;heap+=max(n,1);ret(c,v);return
 if pc==0x140613c20:ret(c);return
 if pc==0x140613c30:
  old,n,new=c.reg_read(UC_X86_REG_RCX),c.reg_read(UC_X86_REG_RDX),c.reg_read(UC_X86_REG_R9);align=c.reg_read(UC_X86_REG_R8);heap=(heap+align-1)&-align;v=heap;heap+=max(new,1);c.mem_write(v,bytes(c.mem_read(old,min(n,new))));ret(c,v);return
 if pc in (0x142923c00,0x142923c10):
  a,b,n=[c.reg_read(r) for r in (UC_X86_REG_RCX,UC_X86_REG_RDX,UC_X86_REG_R8)];c.mem_write(a,bytes(c.mem_read(b,n)));ret(c,a);return
u.hook_add(UC_HOOK_CODE,hook)
points=[(0.,0.),(2.,0.),(2.,2.)];us=[0.,0.5,1.]
w(src,b''.join(struct.pack('<2f',*p) for p in points));w(src+0x100,struct.pack('<3f',*us));w(src+0x200,b''.join(struct.pack('<3f',x,0.,z) for x,z in points));w(src+0x300,struct.pack('<3f',*us));w(src+0x400,struct.pack('<3f',*[3.]*3));w(src+0x500,struct.pack('<3f',*[2.]*3))
for off,v in [(0,3),(8,src),(0x10,3),(0x18,3),(0x20,src+0x100),(0x28,3),(0x88,3),(0x90,src+0x200),(0x98,3),(0xa0,3),(0xa8,src+0x300),(0xb0,3),(0xd8,3),(0xe0,src+0x400),(0xe8,3),(0xf0,3),(0xf8,src+0x500),(0x100,3)]:q(wall+off,v)
for off,v in [(0x30,4.),(0xb8,4.),(0x10c,3.),(0x114,3.),(0x280,1.),(0x28c,4.)]:f(wall+off,v)
w(wall+0x108,struct.pack('<I',1));w(wall+0x110,struct.pack('<I',1));w(wall+0x258,struct.pack('<I',2));q(0x30030000,0);q(0x30031000,0);q(0x30032000,42);q(0x30034000,0x3003c000);q(0x30034010,0x3003e000);q(0x30038000,0x10000);q(0x30038008,0x30039000);q(0x30038010,0);q(sp,0x30000000)
for off,v in [(0x28,0x30033000),(0x30,0x30032000),(0x38,wall),(0x40,0),(0x48,0)]:q(sp+off,v)
for r,v in [(UC_X86_REG_RSP,sp),(UC_X86_REG_RCX,0x30034000),(UC_X86_REG_RDX,0x30030000),(UC_X86_REG_R8,0x30031000),(UC_X86_REG_R9,0x30035000),(UC_X86_REG_MXCSR,0x1f80)]:u.reg_write(r,v)
try:u.emu_start(0x1412269b0,0x30000000,count=20000000)
except Exception as e:observed.update(error=str(e),pc=hex(u.reg_read(UC_X86_REG_RIP)))
observed['returned']=u.reg_read(UC_X86_REG_RIP)==0x30000000;observed['recent']=[hex(x) for x in last];print(json.dumps(observed))
