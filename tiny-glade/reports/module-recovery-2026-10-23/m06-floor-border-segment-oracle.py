import struct,hashlib,pefile
from unicorn import Uc,UC_ARCH_X86,UC_MODE_64,UC_HOOK_CODE
from unicorn.x86_const import *
from pathlib import Path
raw=Path('B版本内容/tiny-glade.rne').read_bytes();assert hashlib.sha256(raw).hexdigest()=='f507a6cea25c5199447b88ba746a5293e64a279f73797ca74c1ce7f4ff2b7d03'
p=pefile.PE(data=raw,fast_load=True);base=p.OPTIONAL_HEADER.ImageBase
for a,b in [((-2.,-1.),(-2.,1.)),((-2.,1.),(2.,1.)),((1.,-1.),(-2.,-1.))]:
 u=Uc(UC_ARCH_X86,UC_MODE_64);u.mem_map(base,(p.OPTIONAL_HEADER.SizeOfImage+4095)&~4095);u.mem_write(base,p.get_memory_mapped_image());u.mem_map(0x30000000,0x200000)
 rb=0x30100000;sp=0x300ff000;vertices=0x30010000
 u.mem_write(vertices,struct.pack('<8f',-2,-1,-2,1,2,1,2,-1));u.mem_write(rb+496,struct.pack('<Q',vertices));u.mem_write(rb+412,struct.pack('<3f',a[0],0,a[1]));u.mem_write(rb+424,struct.pack('<3f',b[0],0,b[1]))
 for r,v in [(UC_X86_REG_RBP,rb),(UC_X86_REG_RSP,sp),(UC_X86_REG_R14,32),(UC_X86_REG_XMM6,0x3f800000),(UC_X86_REG_XMM10,0x3d4ccccd),(UC_X86_REG_MXCSR,0x1f80)]:u.reg_write(r,v)
 result={}
 def hook(c,pc,sz,_):
  if pc==0x141244c7e:
   result['data']=bytes(c.mem_read(rb+304,64)).hex();c.emu_stop();return
  if pc in (0x140833250,0x14082bd60,0x14003bb80):
   r=c.reg_read(UC_X86_REG_RSP);ret=struct.unpack('<Q',c.mem_read(r,8))[0];c.reg_write(UC_X86_REG_RSP,r+8);c.reg_write(UC_X86_REG_RIP,ret);c.reg_write(UC_X86_REG_RAX,0)
 u.hook_add(UC_HOOK_CODE,hook);u.emu_start(0x141244ad5,0x141244c7f,count=10000)
 print(a,b,result)
