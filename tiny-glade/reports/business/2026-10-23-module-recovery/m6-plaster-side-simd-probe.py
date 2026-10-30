import struct,hashlib,json,pefile,ctypes
from unicorn import Uc,UC_ARCH_X86,UC_MODE_64,UC_HOOK_CODE
from unicorn.x86_const import *
raw=open('/Users/yuluo/Desktop/game/B版本内容/tiny-glade.rne','rb').read()
assert hashlib.sha256(raw).hexdigest()=='f507a6cea25c5199447b88ba746a5293e64a279f73797ca74c1ce7f4ff2b7d03'
p=pefile.PE(data=raw); base=p.OPTIONAL_HEADER.ImageBase
c=Uc(UC_ARCH_X86,UC_MODE_64);c.mem_map(base,(p.OPTIONAL_HEADER.SizeOfImage+4095)&~4095);c.mem_write(base,p.get_memory_mapped_image());c.mem_map(0x20000000,0x10000);c.mem_map(0x30000000,0x10000)
lib=ctypes.CDLL('/usr/lib/libSystem.B.dylib'); lib.fmaf.argtypes=[ctypes.c_float]*3; lib.fmaf.restype=ctypes.c_float;lib.ceilf.argtypes=[ctypes.c_float];lib.ceilf.restype=ctypes.c_float
visited=set()
def hook(cpu,pc,size,data):
 visited.add(pc)
 if pc==0x142923e60:
  value=struct.unpack('<f',cpu.reg_read(UC_X86_REG_XMM0).to_bytes(16,'little')[:4])[0];result=lib.ceilf(value);cpu.reg_write(UC_X86_REG_XMM0,int.from_bytes(struct.pack('<f',result),'little'));sp=cpu.reg_read(UC_X86_REG_RSP);target=struct.unpack('<Q',cpu.mem_read(sp,8))[0];cpu.reg_write(UC_X86_REG_RSP,sp+8);cpu.reg_write(UC_X86_REG_RIP,target)
 if pc==0x142923ef0:
  vals=[struct.unpack('<f',cpu.reg_read(r).to_bytes(16,'little')[:4])[0] for r in [UC_X86_REG_XMM0,UC_X86_REG_XMM1,UC_X86_REG_XMM2]]
  result=lib.fmaf(*vals);cpu.reg_write(UC_X86_REG_XMM0,int.from_bytes(struct.pack('<f',result),'little'))
  sp=cpu.reg_read(UC_X86_REG_RSP);target=struct.unpack('<Q',cpu.mem_read(sp,8))[0];cpu.reg_write(UC_X86_REG_RSP,sp+8);cpu.reg_write(UC_X86_REG_RIP,target)
c.hook_add(UC_HOOK_CODE,hook)
def xmm(v):return int.from_bytes(struct.pack('<4f',*v),'little')
def run():
 c.mem_write(0x20000000,bytes(0x10000)); rbp=0x20008000; wall=0x20001000; points=0x20002000; seg=0x20003000; heights=0x20004000; grid=0x20005000
 c.reg_write(UC_X86_REG_RBP,rbp); c.reg_write(UC_X86_REG_R15,wall); c.reg_write(UC_X86_REG_R12,2); c.reg_write(UC_X86_REG_RBX,1); c.reg_write(UC_X86_REG_RDI,0);c.reg_write(UC_X86_REG_R13,1); c.reg_write(UC_X86_REG_RSP,0x30008000);c.reg_write(UC_X86_REG_MXCSR,0x1f80)
 def put(addr,fmt,*v):c.mem_write(addr,struct.pack(fmt,*v))
 put(wall+0x90,'<QQ',points,2);put(wall+0x30,'<f',4.);put(wall+0x280,'<f',1.);put(wall+0x258,'<I',2)
 put(0x20006000,'<ffff',0.,1.,2.,2.);put(rbp+0x348,'<Q',0x20006000);put(rbp+0xd0,'<Q',0x20006010);put(rbp+0x500,'<Q',wall);put(rbp+0x2ac,'<I',6 | (6<<16));put(points,'<ffffff',0.,0.,0.,4.,0.,0.);put(seg,'<ff',2.,2.);put(heights,'<ff',3.,3.)
 for off,val in [(0x340,6),(0x2c0,6),(0xd8,2),(0xe0,2),(0xe8,2),(0xf8,points),(0xf0,seg),(0x100,grid+8),(0x108,grid-0x34),(0x110,heights),(0x3b8,grid)]:put(rbp+off,'<Q',val)
 for off,val in [(0x380,1.),(0x39c,4.),(0x388,0.),(0x378,3.),(0x3a0,5.)]:put(rbp+off,'<f',val)
 c.mem_write(rbp-0x20,struct.pack('<IIII',6,6,0,0));c.mem_write(rbp+0x290,struct.pack('<ffff',5.,5.,0.,0.));put(rbp+0x308,'<B',0);put(rbp+0x1f0,'<f',0.5)
 for r,v in [(UC_X86_REG_XMM7,2.),(UC_X86_REG_XMM8,0.),(UC_X86_REG_XMM9,5.),(UC_X86_REG_XMM11,4.),(UC_X86_REG_XMM12,1.),(UC_X86_REG_XMM15,0.)]:c.reg_write(r,xmm([v,0,0,0]))
 c.reg_write(UC_X86_REG_XMM13,0);c.reg_write(UC_X86_REG_XMM14,0)
 try:c.emu_start(0x14122752d,0x141228062,count=500000)
 except Exception:print('failed pc',hex(c.reg_read(UC_X86_REG_RIP)));raise
 assert c.reg_read(UC_X86_REG_RIP)==0x141228062
 return bytes(c.mem_read(grid,1152)).hex()
print(run());print('simd_entry', 0x141227e20 in visited)
