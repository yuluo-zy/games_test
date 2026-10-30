import struct,hashlib,json,pefile,ctypes
from unicorn import Uc,UC_ARCH_X86,UC_MODE_64,UC_HOOK_CODE
from unicorn.x86_const import *
raw=open('B版本内容/tiny-glade.rne','rb').read();assert hashlib.sha256(raw).hexdigest()=='f507a6cea25c5199447b88ba746a5293e64a279f73797ca74c1ce7f4ff2b7d03';p=pefile.PE(data=raw,fast_load=True);base=p.OPTIONAL_HEADER.ImageBase
u=Uc(UC_ARCH_X86,UC_MODE_64);u.mem_map(base,(p.OPTIONAL_HEADER.SizeOfImage+4095)&~4095);u.mem_write(base,p.get_memory_mapped_image());u.mem_map(0x30000000,0x400000);u.mem_map(0x40000000,0x1000000)
sp=0x30100008;heap=0x40000000;last=[];hits={};inserts=[];generate=[];lib=ctypes.CDLL('/usr/lib/libSystem.B.dylib');lib.expf.argtypes=[ctypes.c_float];lib.expf.restype=ctypes.c_float
w=lambda a,b:u.mem_write(a,b);q=lambda a,v:w(a,struct.pack('<Q',v))
def ret(c,v=None):
 if v is not None:c.reg_write(UC_X86_REG_RAX,v)
 s=c.reg_read(UC_X86_REG_RSP);c.reg_write(UC_X86_REG_RIP,struct.unpack('<Q',c.mem_read(s,8))[0]);c.reg_write(UC_X86_REG_RSP,s+8)
def hook(c,pc,size,_):
 global heap
 last.append(pc);del last[:-25];hits[hex(pc)]=hits.get(hex(pc),0)+1
 if pc==0x140c16a00:ret(c,0);return
 if pc==0x142923ec0:
  x=struct.unpack('<f',struct.pack('<I',c.reg_read(UC_X86_REG_XMM0)&0xffffffff))[0];c.reg_write(UC_X86_REG_XMM0,struct.unpack('<I',struct.pack('<f',lib.expf(x)))[0]);ret(c);return
 if pc==0x1405b0d70:ret(c,0);return
 if pc==0x140b777f0:
  out=c.reg_read(UC_X86_REG_RCX);c.mem_write(out,struct.pack('<QQQQ',42,0,8,0));ret(c);return
 if pc==0x1412269b0:
  generate.append('generate');ret(c);return
 if pc==0x1411fcad0:
  inserts.append([c.reg_read(UC_X86_REG_RDX),c.reg_read(UC_X86_REG_XMM2)&0xffffffff]);ret(c,0);return
 if pc==0x140613c10:
  n=c.reg_read(UC_X86_REG_RCX);a=c.reg_read(UC_X86_REG_RDX);heap=(heap+a-1)&-a;v=heap;heap+=max(n,1);ret(c,v);return
 if pc==0x140613c20:ret(c);return
u.hook_add(UC_HOOK_CODE,hook)
q(base+0x362c420,0x30001000);q(0x30001000,0);w(base+0x362bfc0,bytes(32))
# Empty event readers: local cursor + two empty native Events buffers.
for local,events,reader in [(0x30002000,0x30002100,0x30002200),(0x30002300,0x30002400,0x30002500)]:
 q(local,0);q(events+8,8);q(events+0x10,0);q(events+0x18,0);q(events+0x28,8);q(events+0x30,0);q(events+0x38,0);q(events+0x40,0);q(reader,local);q(reader+8,events)
# PublicWalls one raw bucket; first observation inserts wall peel without animation.
w(0x30003000,bytes([0])+bytes([0x80])*15);q(0x30003118,1);q(0x30003000-0x2a8,42);w(0x30003000-0x10,struct.pack('<f',0.5));q(0x30004600,0x30004100)
# Empty PublicWalls and animated peel native hashbrown tables.
w(0x30003000,bytes([0x80])*16);q(0x30003100,0x30003000);q(0x30003118,0);q(0x30003200,0x30003100)
w(0x30003000,bytes([0])+bytes([0x80])*15);q(0x30003118,1);q(0x30003000-0x2a8,42);w(0x30003000-0x10,struct.pack('<f',0.5));w(0x30004000,bytes([0x80])*16);q(0x30004100,0x30004000);q(0x30004118,0);q(0x30004200,0x30004100)
q(0x30004300,0x30004400);q(0x30004310,0x30004500);q(0x30004600,0x30004700);q(0x30004610,0x30004800)
q(0x30007000,0x30007200);# One valid simplified wall Curve2 header and mapped positions.
wb=0x30003000-0x2a8+8
for off,val in [(0,2),(8,0x3000b000),(0x10,2),(0x18,2),(0x20,0x3000b100),(0x28,2)]:q(wb+off,val)
w(0x3000b000,struct.pack('<4f',0.,0.,4.,0.));w(0x3000b100,struct.pack('<2f',0.,1.));w(wb+0x30,struct.pack('<f',4.))
# Existing peel bucket and an audio EventWriter buffer.
w(0x30004000,bytes([0])+bytes([0x80])*15);q(0x30004118,1);q(0x30004000-16,42);w(0x30004000-8,struct.pack('<f',0.0));q(0x30004600,0x30004100)
w(0x30003000-0x2a8+8+0x258,struct.pack('<I',2));w(0x30003000-0x2a8+8+0x108,struct.pack('<If',1,3.));w(0x30003000-0x2a8+8+0x280,struct.pack('<f',1.));w(0x30007200+0x28,struct.pack('<f',0.1))
w(wb+0x296,bytes([1]));q(0x30006200,0x30006300);q(0x30006010,0x30009100);q(0x30006100,0x30009200);q(0x30004420,8);q(0x30004428,0x3000a000);q(0x30004430,0);q(0x30004440,0);q(sp,0x30000000)
for off,v in [(0x28,0x30005000),(0x30,0x30005100),(0x38,0x30002500),(0x40,0x30002200),(0x48,0x30004300),(0x50,0x30004600),(0x58,0x30007000),(0x60,0x30007100)]:q(sp+off,v)
for r,v in [(UC_X86_REG_RSP,sp),(UC_X86_REG_RCX,0x30006000),(UC_X86_REG_RDX,0x30006100),(UC_X86_REG_R8,0x30003200),(UC_X86_REG_R9,0x30006200),(UC_X86_REG_MXCSR,0x1f80)]:u.reg_write(r,v)
try:u.emu_start(0x14075b810,0x30000000,count=3000000);result={'returned':u.reg_read(UC_X86_REG_RIP)==0x30000000}
except Exception as e:result={'error':str(e),'pc':hex(u.reg_read(UC_X86_REG_RIP))}
result['generate']=generate;result['audio_count']=struct.unpack('<Q',u.mem_read(0x30004430,8))[0];result['audio_hex']=bytes(u.mem_read(0x3000a000,40)).hex();result['inserts']=inserts;result['recent']=[hex(x) for x in last];print(json.dumps(result))
