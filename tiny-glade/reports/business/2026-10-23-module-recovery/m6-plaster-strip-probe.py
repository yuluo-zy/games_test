import struct,hashlib,json,pefile,ctypes
from unicorn import Uc,UC_ARCH_X86,UC_MODE_64,UC_HOOK_CODE
from unicorn.x86_const import *
raw=open('/Users/yuluo/Desktop/game/B版本内容/tiny-glade.rne','rb').read()
assert hashlib.sha256(raw).hexdigest()=='f507a6cea25c5199447b88ba746a5293e64a279f73797ca74c1ce7f4ff2b7d03'
p=pefile.PE(data=raw); base=p.OPTIONAL_HEADER.ImageBase
c=Uc(UC_ARCH_X86,UC_MODE_64);c.mem_map(base,(p.OPTIONAL_HEADER.SizeOfImage+4095)&~4095);c.mem_write(base,p.get_memory_mapped_image());c.mem_map(0x20000000,0x10000);c.mem_map(0x30000000,0x10000)
lib=ctypes.CDLL('/usr/lib/libSystem.B.dylib'); lib.fmaf.argtypes=[ctypes.c_float]*3; lib.fmaf.restype=ctypes.c_float
def hook(cpu,pc,size,data):
 if pc==0x142923ef0:
  vals=[struct.unpack('<f',cpu.reg_read(r).to_bytes(16,'little')[:4])[0] for r in [UC_X86_REG_XMM0,UC_X86_REG_XMM1,UC_X86_REG_XMM2]]
  result=lib.fmaf(*vals);cpu.reg_write(UC_X86_REG_XMM0,int.from_bytes(struct.pack('<f',result),'little'))
  sp=cpu.reg_read(UC_X86_REG_RSP);target=struct.unpack('<Q',cpu.mem_read(sp,8))[0];cpu.reg_write(UC_X86_REG_RSP,sp+8);cpu.reg_write(UC_X86_REG_RIP,target)
c.hook_add(UC_HOOK_CODE,hook)
def xmm(v):return int.from_bytes(struct.pack('<4f',*v),'little')
def run(face,flags,nx,ny):
 c.mem_write(0x20000000,bytes(0x10000));rbp=0x20008000;grid=0x20001000;desc=0x20002000
 for i,f in enumerate(flags):c.mem_write(grid+32*i+28,bytes([f]))
 for off,v in [(0x340,nx),(0x2c0,ny-1),(0x3a0,nx-1),(0x3c8,nx*ny),(0x3b8,grid),(0x328,128),(0x330,desc),(0x338,0),(0x378,face),(0x2d0,0x60000000),(0x2e0,0)]:c.mem_write(rbp+off,struct.pack('<Q',v))
 c.reg_write(UC_X86_REG_RBP,rbp);c.reg_write(UC_X86_REG_RSP,0x30008000);c.reg_write(UC_X86_REG_XMM13,0)
 c.emu_start(0x141228603,0x1412287d0,count=100000);assert c.reg_read(UC_X86_REG_RIP)==0x1412287d0
 n=struct.unpack('<Q',c.mem_read(rbp+0x338,8))[0];raw=bytes(c.mem_read(desc,n*32));return [struct.unpack_from('<QIi',raw,i*32+16) for i in range(n)]
cases=[(0,[0]*12,4,3),(1,[0]*12,4,3),(0,[1]*12,4,3),(0,[0,0,1,1,0,0,1,1,1,1,1,1],4,3),(0,[0]*70,35,2)]
print(json.dumps([{'face':x[0],'nx':x[2],'ny':x[3],'descriptors':run(*x)} for x in cases]))
