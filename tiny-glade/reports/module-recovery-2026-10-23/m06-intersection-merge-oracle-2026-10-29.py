import struct,hashlib,json,pefile,math
from unicorn import *
from unicorn.x86_const import *
raw=open('B版本内容/tiny-glade.rne','rb').read(); assert hashlib.sha256(raw).hexdigest()=='f507a6cea25c5199447b88ba746a5293e64a279f73797ca74c1ce7f4ff2b7d03'
p=pefile.PE(data=raw,fast_load=True); base=p.OPTIONAL_HEADER.ImageBase

def run(segs):
 u=Uc(UC_ARCH_X86,UC_MODE_64);u.mem_map(base,(p.OPTIONAL_HEADER.SizeOfImage+4095)&~4095);u.mem_write(base,p.get_memory_mapped_image());u.mem_map(0x30000000,0x500000);u.mem_map(0x40000000,0x1000000)
 w=lambda a,b:u.mem_write(a,b); q=lambda a,v:w(a,struct.pack('<Q',v)); read=lambda a,n:bytes(u.mem_read(a,n)); get=lambda a:struct.unpack('<Q',read(a,8))[0]
 sp=0x30100008; q(sp,0x30000000); ids={}; heap=0x40000000; last=[]
 def ret(v=None):
  if v is not None:u.reg_write(UC_X86_REG_RAX,v)
  s=u.reg_read(UC_X86_REG_RSP);u.reg_write(UC_X86_REG_RIP,get(s));u.reg_write(UC_X86_REG_RSP,s+8)
 def hook(u,pc,size,_):
  nonlocal heap
  last.append(pc);del last[:-20]
  if pc==0x140c16a00:ret(0)
  elif pc==0x1408f4670:
   ptr=u.reg_read(UC_X86_REG_RDX); vals=struct.unpack('<3f',read(ptr,12)); key=tuple(int(math.copysign(math.floor(abs(x)+.5),x)) for x in vals)
   if key not in ids:ids[key]=len(ids)
   ret(ids[key])
  elif pc==0x1412fb420:
   dst=u.reg_read(UC_X86_REG_RCX);src=u.reg_read(UC_X86_REG_RDX); ptr=heap;heap+=0x1000
   w(ptr,read(src,48));q(dst,64);q(dst+8,ptr);q(dst+16,2);ret(dst)
  elif pc==0x140613c20:ret()
  elif pc in (0x1428f87d0,0x142923c20):
   dst=u.reg_read(UC_X86_REG_RCX); val=u.reg_read(UC_X86_REG_RDX)&255;n=u.reg_read(UC_X86_REG_R8);w(dst,bytes([val])*n);ret(dst)
 u.hook_add(UC_HOOK_CODE,hook)
 q(0x30001000,0);w(base+0x362bfc0,bytes(32));q(base+0x362c420,0x30001000)
 for i,s in enumerate(segs):w(0x30010000+i*36,struct.pack('<9f',*s))
 q(0x30020000,64);q(0x30020008,0x30021000);q(0x30020010,0)
 q(0x30030000,128);q(0x30030008,0x30031000);q(0x30030010,0);q(0x30030018,0x30032000);q(0x30030020,127);q(0x30030028,112);q(0x30030030,0)
 for r,v in [(UC_X86_REG_RSP,sp),(UC_X86_REG_RCX,0x30010000),(UC_X86_REG_RDX,len(segs)),(UC_X86_REG_R8,0x30020000),(UC_X86_REG_R9,0x30030000),(UC_X86_REG_MXCSR,0x1f80)]:u.reg_write(r,v)
 try:u.emu_start(0x1408f2850,0x30000000,count=1000000)
 except Exception as e:raise Exception((str(e),hex(u.reg_read(UC_X86_REG_RIP)),list(map(hex,last))))
 assert u.reg_read(UC_X86_REG_RIP)==0x30000000, list(map(hex,last))
 lines=[]
 for i in range(get(0x30020010)):
  a=0x30021000+i*40;cap,ptr,head,n=struct.unpack('<4Q',read(a,32));first,lastid=struct.unpack('<2I',read(a+32,8)); points=[list(struct.unpack('<6f',read(ptr+((head+k)%cap)*24,24))) for k in range(n)];lines.append(dict(first=first,last=lastid,points=points))
 return lines

def s(a,b,n):return [a,0.,0.,b,0.,0.,n,1.,0.]
cases=[]
for ra in (0,1):
 for rb in (0,1):cases.append([s(*( (1.,0.) if ra else (0.,1.)),.5),s(*((3.,2.) if rb else (2.,3.)),2.5),s(1.,2.,1.5)])
cases += [[s(0.,1.,1.),s(1.,2.,2.),s(2.,0.,3.),s(0.,4.,4.)],[s(0.,0.,1.),s(0.,1.,2.),s(1.,2.,3.),s(1.,3.,4.)]]
import random
rng=random.Random(0x6a2850)
for _ in range(96):
 cases.append([s(float(rng.randrange(-3,6)),float(rng.randrange(-3,6)),float(rng.randrange(-8,9))/4) for _ in range(rng.randrange(1,13))])
for i,c in enumerate(cases):print(json.dumps({'case':i,'input':c,'original':run(c)}))
