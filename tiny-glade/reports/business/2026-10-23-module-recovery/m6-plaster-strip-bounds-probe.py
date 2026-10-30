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
def run(face):
 c.mem_write(0x20000000,bytes(0x10000));rbp=0x20008000;verts=0x20001000;desc=0x20002000
 for i in range(8):c.mem_write(verts+i*48,struct.pack('<fff',float(i%4),float(i//4)*2.,0.))
 c.mem_write(desc+16,struct.pack('<QIi',0x60000000+(192 if face else 0),3,-4 if face else 4))
 for off,v in [(0x330,desc),(0x2d0,0x60000000)]:c.mem_write(rbp+off,struct.pack('<Q',v))
 for r,v in [(UC_X86_REG_RBP,rbp),(UC_X86_REG_RSP,0x30008000),(UC_X86_REG_RBX,verts),(UC_X86_REG_R12,8),(UC_X86_REG_R9,1),(UC_X86_REG_RSI,0)]:c.reg_write(r,v)
 for r,v in [(UC_X86_REG_XMM10,2.),(UC_X86_REG_XMM12,1.)]:c.reg_write(r,xmm([v,0,0,0]))
 c.emu_start(0x141228815,0x141228847,count=10000);assert c.reg_read(UC_X86_REG_RIP)==0x141228847
 return list(struct.unpack('<IIII',c.mem_read(desc,16)))
print(json.dumps([run(0),run(1)]))
