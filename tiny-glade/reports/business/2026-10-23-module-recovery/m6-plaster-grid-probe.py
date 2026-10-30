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
def run(roof, ridge, tag, y, top, cutoff, ridge_y, side_top, edge):
 c.mem_write(0x20000000,bytes(0x1000)); rbp=0x20000800
 c.reg_write(UC_X86_REG_RBP,rbp); c.reg_write(UC_X86_REG_RDI,0x20000000); c.reg_write(UC_X86_REG_R14,0); c.reg_write(UC_X86_REG_R12,tag); c.reg_write(UC_X86_REG_RBX,ridge); c.reg_write(UC_X86_REG_RSP,0x30008000);c.reg_write(UC_X86_REG_MXCSR,0x1f80)
 def put(off,fmt,v):c.mem_write(rbp+off,struct.pack(fmt,v))
 put(0x2c0,'<Q',1);put(0x340,'<Q',1);put(0x3a0,'<f',1.0);put(0x308,'<B',roof);put(0x378,'<f',side_top);put(0x2d0,'<f',ridge_y)
 c.mem_write(rbp+0x284,struct.pack('<fff',1,2,3))
 for r,v in [(UC_X86_REG_XMM6,0),(UC_X86_REG_XMM8,2),(UC_X86_REG_XMM9,y),(UC_X86_REG_XMM11,ridge_y-0.46),(UC_X86_REG_XMM12,top),(UC_X86_REG_XMM13,cutoff),(UC_X86_REG_XMM14,edge)]:c.reg_write(r,xmm([v,0,0,0]))
 c.emu_start(0x141227952,0x14122794f,count=10000)
 assert c.reg_read(UC_X86_REG_RIP)==0x14122794f
 return bytes(c.mem_read(0x20000000,32)).hex()
cases=[(0,0,0,1.,4.,2.,3.,5.,1.),(1,0,0,1.,4.,2.,3.,5.,1.),(0,1,2,1.,4.,2.,3.,5.,1.),(1,0,0,4.,3.,2.,3.,4.,1.),(0,1,2,4.,3.,2.,3.,5.,1.),(0,0,0,float('nan'),4.,2.,3.,5.,1.)]
print(json.dumps([{'input':list(a),'bytes':run(*a)} for a in cases]))
