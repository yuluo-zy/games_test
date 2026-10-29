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
def run():
 c.mem_write(0x20000000,bytes(0x10000));rbp=0x20008000;arr=0x20004000
 c.mem_write(arr,struct.pack('<ffffffff',0.5,2.,0.5,1.,9.5,2.,0.5,1.));c.mem_write(rbp+0x170,struct.pack('<QQ',8,arr));c.reg_write(UC_X86_REG_RBP,rbp);c.reg_write(UC_X86_REG_RBX,2);c.reg_write(UC_X86_REG_RSP,0x30008000);c.reg_write(UC_X86_REG_MXCSR,0x1f80);c.reg_write(UC_X86_REG_XMM11,xmm([10.,0.,0.,0.]))
 c.emu_start(0x141226d80,0x141226e48,count=10000)
 assert c.reg_read(UC_X86_REG_RIP)==0x141226e48
 return {'count':c.reg_read(UC_X86_REG_R13),'boxes':list(struct.unpack('<16f',c.mem_read(arr,64)))}
print(json.dumps(run()))
