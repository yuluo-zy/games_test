import hashlib,json,struct,ctypes,pefile
from unicorn import *
from unicorn.x86_const import *
raw=open('B版本内容/tiny-glade.rne','rb').read();assert hashlib.sha256(raw).hexdigest()=='f507a6cea25c5199447b88ba746a5293e64a279f73797ca74c1ce7f4ff2b7d03';p=pefile.PE(data=raw,fast_load=True);base=p.OPTIONAL_HEADER.ImageBase
lib=ctypes.CDLL('/usr/lib/libSystem.B.dylib');lib.ceilf.argtypes=[ctypes.c_float];lib.ceilf.restype=ctypes.c_float
bits=lambda x:struct.unpack('<I',struct.pack('<f',x))[0];flt=lambda x:struct.unpack('<f',struct.pack('<I',x&0xffffffff))[0]
def run(bottom,top,seed,direction):
 u=Uc(UC_ARCH_X86,UC_MODE_64);u.mem_map(base,(p.OPTIONAL_HEADER.SizeOfImage+4095)&~4095);u.mem_write(base,p.get_memory_mapped_image());u.mem_map(0x30000000,0x400000);u.mem_map(0x40000000,0x1000000);sp=0x30100008;heap=0x40000000;bricks=[];last=[]
 w=lambda a,b:u.mem_write(a,b);q=lambda a,v:w(a,struct.pack('<Q',v));read=lambda a,n:bytes(u.mem_read(a,n))
 def ret(v=None):
  if v is not None:u.reg_write(UC_X86_REG_RAX,v)
  r=u.reg_read(UC_X86_REG_RSP);u.reg_write(UC_X86_REG_RIP,struct.unpack('<Q',read(r,8))[0]);u.reg_write(UC_X86_REG_RSP,r+8)
 def hook(c,pc,size,_):
  nonlocal heap
  last.append(pc);del last[:-30]
  if pc==0x14120c120: bricks.append(read(c.reg_read(UC_X86_REG_RDX),96).hex());ret(0)
  elif pc in (0x140613c10,0x140d657b0):
   size=c.reg_read(UC_X86_REG_RCX);align=c.reg_read(UC_X86_REG_RDX);heap=(heap+align-1)&-align;out=heap;heap+=max(size,1);ret(out)
  elif pc in (0x140613c20,0x140d65810):ret()
  elif pc==0x140d65850:
   old=c.reg_read(UC_X86_REG_RCX);size=c.reg_read(UC_X86_REG_R9);align=c.reg_read(UC_X86_REG_R8);heap=(heap+align-1)&-align;out=heap;heap+=size;w(out,read(old,min(size,c.reg_read(UC_X86_REG_RDX))));ret(out)
  elif pc==0x142923c00:
   dst=c.reg_read(UC_X86_REG_RCX);src=c.reg_read(UC_X86_REG_RDX);n=c.reg_read(UC_X86_REG_R8);w(dst,read(src,n));ret(dst)
  elif pc==0x142923e60:c.reg_write(UC_X86_REG_XMM0,bits(lib.ceilf(flt(c.reg_read(UC_X86_REG_XMM0)))));ret()
 u.hook_add(UC_HOOK_CODE,hook)
 for i,v in enumerate([0x30002000,0x30003000,0x30004000,0x30005000,0x30006000,0x30007000]):q(0x30001000+i*8,v)
 w(0x30002000,struct.pack('<7f',*direction,1.,2.,3.,bottom,.25));w(0x30003000+0x28c,struct.pack('<f',10.));q(0x30004000,123);q(0x30008000,seed);w(0x30009000,struct.pack('<3f',4.,5.,6.));q(sp,0x30000000);q(sp+0x28,0x30009000)
 for r,v in [(UC_X86_REG_RSP,sp),(UC_X86_REG_RCX,0x30001000),(UC_X86_REG_RDX,0x30008000),(UC_X86_REG_XMM2,bits(bottom)),(UC_X86_REG_XMM3,bits(top)),(UC_X86_REG_MXCSR,0x1f80)]:u.reg_write(r,v)
 try:u.emu_start(0x14120d670,0x30000000,count=3000000)
 except Exception as e:raise Exception((str(e),hex(u.reg_read(UC_X86_REG_RIP)),list(map(hex,last))))
 assert u.reg_read(UC_X86_REG_RIP)==0x30000000,list(map(hex,last));return {'bricks':bricks,'seed_after':struct.unpack('<Q',read(0x30008000,8))[0],'counter':struct.unpack('<I',read(0x30005000,4))[0]}
cases=[(1.,2.,123,[1.,0.]),(3.,4.,123,[0.,1.]),(.1,.1,42,[1.,0.]),(-.3,0.,42,[1.,0.])]
for x in [1.0599998,1.06,1.0600002,1.8999997,1.9,1.9000002,0.,-.2]:
 for seed in [0,123,0xffffffffffffffff]:cases.append((x,x+1.4,seed,[0.6,0.8]))
for i,args in enumerate(cases): print(json.dumps({'case':i,'input':args,'original':run(*args)}))
