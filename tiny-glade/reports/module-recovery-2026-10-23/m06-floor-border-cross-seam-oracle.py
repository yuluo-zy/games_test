import struct,hashlib,pefile,ctypes,sys
from unicorn import Uc,UC_ARCH_X86,UC_MODE_64,UC_HOOK_CODE
from unicorn.x86_const import *
from pathlib import Path
raw=Path('B版本内容/tiny-glade.rne').read_bytes();assert hashlib.sha256(raw).hexdigest()=='f507a6cea25c5199447b88ba746a5293e64a279f73797ca74c1ce7f4ff2b7d03'
p=pefile.PE(data=raw,fast_load=True);base=p.OPTIONAL_HEADER.ImageBase
u=Uc(UC_ARCH_X86,UC_MODE_64);u.mem_map(base,(p.OPTIONAL_HEADER.SizeOfImage+4095)&~4095);u.mem_write(base,p.get_memory_mapped_image());u.mem_map(0x30000000,0x200000);u.mem_map(0x40000000,0x1000000)
rb=0x30100000;sp=0x300ff000;wall=0x30010000;holes=0x30012000;heap=0x40000000;rows=[];recent=[];status='running'
u.mem_write(wall+0x258,struct.pack('<I6f',1,1,0,0,0,4,2));u.mem_write(wall+0x28c,struct.pack('<f',12.));u.mem_write(holes,struct.pack('<4fBB2x',-1.,0.,1.,2.,int(sys.argv[1]) if len(sys.argv)>1 else 4,0))
u.mem_write(rb+464,struct.pack('<Q',holes+20));u.mem_write(rb+480,struct.pack('<Q',holes+20))
lib=ctypes.CDLL('/usr/lib/libSystem.B.dylib');lib.fmodf.argtypes=[ctypes.c_float,ctypes.c_float];lib.fmodf.restype=ctypes.c_float
bits=lambda x:struct.unpack('<I',struct.pack('<f',x))[0];num=lambda x:struct.unpack('<f',struct.pack('<I',x&0xffffffff))[0]
def ret(c):
 r=c.reg_read(UC_X86_REG_RSP);v=struct.unpack('<Q',c.mem_read(r,8))[0];c.reg_write(UC_X86_REG_RSP,r+8);c.reg_write(UC_X86_REG_RIP,v)
def hook(c,pc,size,_):
 global heap,status
 recent.append(pc);del recent[:-20]
 if pc==0x141244c7e:rows.append(bytes(c.mem_read(rb+304,64)).hex());c.reg_write(UC_X86_REG_RIP,0x1412449d6)
 elif pc==0x141244ed7:status='returned';c.emu_stop()
 elif pc in (0x140833250,0x14082bd60,0x14003bb80,0x140613c20,0x140d65810,0x140d66740):c.reg_write(UC_X86_REG_RAX,0);ret(c)
 elif pc in (0x140613c10,0x140d657b0):
  n=c.reg_read(UC_X86_REG_RCX);a=c.reg_read(UC_X86_REG_RDX);heap=(heap+a-1)&-a;c.reg_write(UC_X86_REG_RAX,heap);heap+=n;ret(c)
 elif pc==0x140d65850:
  old=c.reg_read(UC_X86_REG_RCX);oldsize=c.reg_read(UC_X86_REG_RDX);a=c.reg_read(UC_X86_REG_R8);n=c.reg_read(UC_X86_REG_R9);heap=(heap+a-1)&-a;c.mem_write(heap,bytes(c.mem_read(old,min(n,oldsize))));c.reg_write(UC_X86_REG_RAX,heap);heap+=n;ret(c)
 elif pc==0x142923f10:c.reg_write(UC_X86_REG_XMM0,bits(lib.fmodf(num(c.reg_read(UC_X86_REG_XMM0)),num(c.reg_read(UC_X86_REG_XMM1)))));ret(c)
 elif pc in (0x142923c00,0x142923c10):
  dst=c.reg_read(UC_X86_REG_RCX);src=c.reg_read(UC_X86_REG_RDX);n=c.reg_read(UC_X86_REG_R8);c.mem_write(dst,bytes(c.mem_read(src,n)));c.reg_write(UC_X86_REG_RAX,dst);ret(c)
u.hook_add(UC_HOOK_CODE,hook)
for r,v in [(UC_X86_REG_RBP,rb),(UC_X86_REG_RSP,sp),(UC_X86_REG_RSI,wall),(UC_X86_REG_R14,holes),(UC_X86_REG_RBX,holes+20),(UC_X86_REG_XMM6,0x3f800000),(UC_X86_REG_XMM10,0x3d4ccccd),(UC_X86_REG_MXCSR,0x1f80)]:u.reg_write(r,v)
try:u.emu_start(0x141244573,0x141244ed8,count=10000000)
except Exception as e:status='fault:'+str(e)+'@'+hex(u.reg_read(UC_X86_REG_RIP))
print(status,len(rows),rows,'recent',list(map(hex,recent[-8:])))
