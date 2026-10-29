import pefile,hashlib,struct,random,math
from pathlib import Path
from unicorn import Uc,UC_ARCH_X86,UC_MODE_64,UC_HOOK_CODE
from unicorn.x86_const import *
raw=Path('B版本内容/tiny-glade.rne').read_bytes();assert hashlib.sha256(raw).hexdigest()=='f507a6cea25c5199447b88ba746a5293e64a279f73797ca74c1ce7f4ff2b7d03'
p=pefile.PE(data=raw,fast_load=True);base=p.OPTIONAL_HEADER.ImageBase; image=p.get_memory_mapped_image()
u=Uc(UC_ARCH_X86,UC_MODE_64);u.mem_map(base,(len(image)+4095)&~4095);u.mem_write(base,image); D=0x20000000;u.mem_map(D,0x10000)
def hook(u,a,size,data):
 if a==0x142923ee0:
  x=struct.unpack('<f',u.reg_read(UC_X86_REG_XMM0).to_bytes(16,'little')[:4])[0]
  y=float(math.floor(x))
  u.reg_write(UC_X86_REG_XMM0,int.from_bytes(struct.pack('<f',y)+bytes(12),'little'))
  rsp=u.reg_read(UC_X86_REG_RSP);dest=struct.unpack('<Q',u.mem_read(rsp,8))[0];u.reg_write(UC_X86_REG_RSP,rsp+8);u.reg_write(UC_X86_REG_RIP,dest)
u.hook_add(UC_HOOK_CODE,hook)
r=random.Random(61);rows=[]
for _ in range(256):
 angle=r.uniform(-3.14,3.14); axis=(math.cos(angle),math.sin(angle)); size=(r.uniform(.1,9),r.uniform(.1,9));origin=(r.uniform(-60,60),r.uniform(-60,60));radius=r.uniform(0,2); samples=(r.random(),r.random())
 def w(at,v):u.mem_write(at,struct.pack('<'+'f'*len(v),*v))
 rbp=D+0x1000; mesh=D+0x3000; region=D+0x5000
 u.mem_write(rbp+0x6f0,struct.pack('<Q',mesh));u.mem_write(rbp+0x6c0,struct.pack('<Q',region))
 w(mesh+0x40,(radius,));w(region-0x28,axis);w(region-0x20,size);w(region-0x18,(origin[0],0.,origin[1]))
 for reg,value in [(UC_X86_REG_XMM6,samples[0]),(UC_X86_REG_XMM0,samples[1])]:u.reg_write(reg,int.from_bytes(struct.pack('<f',value)+bytes(12),'little'))
 u.reg_write(UC_X86_REG_RBP,rbp);u.reg_write(UC_X86_REG_RSP,D+0xf008)
 u.emu_start(0x1422d7f5f,0x1422d8048,count=1000)
 assert u.reg_read(UC_X86_REG_RIP)==0x1422d8048
 inp=struct.unpack('<9I',struct.pack('<9f',*(origin+axis+size+(radius,)+samples)))
 out=struct.unpack('<2I',u.mem_read(rbp+0x450,8));rows.append((inp,out))
Path('tiny-glade/reports/module-recovery-2026-10-23/m06-clutter-landing-native-cases.rs').write_text('&[\n'+''.join('('+str(list(a)).replace(' ','')+','+str(list(b)).replace(' ','')+'),\n' for a,b in rows)+']\n')
print(len(rows))
