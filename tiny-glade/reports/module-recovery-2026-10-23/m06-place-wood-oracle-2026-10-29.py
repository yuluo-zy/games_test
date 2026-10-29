import hashlib,json,struct,pefile
from unicorn import *
from unicorn.x86_const import *
raw=open('B版本内容/tiny-glade.rne','rb').read();assert hashlib.sha256(raw).hexdigest()=='f507a6cea25c5199447b88ba746a5293e64a279f73797ca74c1ce7f4ff2b7d03';p=pefile.PE(data=raw,fast_load=True);base=p.OPTIONAL_HEADER.ImageBase

def run(direction,pos,height):
 u=Uc(UC_ARCH_X86,UC_MODE_64);u.mem_map(base,(p.OPTIONAL_HEADER.SizeOfImage+4095)&~4095);u.mem_write(base,p.get_memory_mapped_image());u.mem_map(0x30000000,0x500000);rbp=0x30100000;last=[]
 w=lambda a,b:u.mem_write(a,b);q=lambda a,v:w(a,struct.pack('<Q',v));r=lambda a,n:bytes(u.mem_read(a,n));v=lambda a:int.from_bytes(a,'little')
 w(0x30002000,struct.pack('<7f',*direction,1.,2.,3.,height,.25));w(rbp+0x124,struct.pack('<3f',*pos));q(rbp+0x1b8,123);q(rbp+0xc8,2);q(rbp+0x1c0,7)
 for reg,val in [(UC_X86_REG_RSP,rbp-0x1000),(UC_X86_REG_RBP,rbp),(UC_X86_REG_R12,0x30002000),(UC_X86_REG_XMM13,0x3f800000),(UC_X86_REG_XMM14,v(p.get_data(0x142a7c1e0-base,16))),(UC_X86_REG_XMM8,v(p.get_data(0x142a7c1f0-base,16))),(UC_X86_REG_XMM9,v(p.get_data(0x1429345b0-base,8))),(UC_X86_REG_XMM6,v(p.get_data(0x142a7c200-base,16))),(UC_X86_REG_XMM15,0),(UC_X86_REG_MXCSR,0x1f80)]:u.reg_write(reg,val)
 def hook(c,pc,size,_):
  last.append(pc);del last[:-20]
  if pc==0x1407546ad:c.emu_stop()
 u.hook_add(UC_HOOK_CODE,hook)
 try:u.emu_start(0x14075456d,0x1407546ae,count=100000)
 except Exception as e:raise Exception((str(e),hex(u.reg_read(UC_X86_REG_RIP)),list(map(hex,last))))
 assert u.reg_read(UC_X86_REG_RIP)==0x1407546ad,list(map(hex,last));return r(rbp+0xe0,64).hex()
cases=[([1.,0.],[4.,5.,6.],2.),([0.,1.],[4.,5.,6.],2.),([.6,.8],[4.,5.,6.],2.),([-0.,1.],[-0.,0.,-0.],0.),([1.,-0.],[0.,-0.,0.],-.2),([-1.,0.],[-1.,1.,-1.],1.)]
for i,c in enumerate(cases):print(json.dumps({'case':i,'input':c,'original':run(*c)}))
