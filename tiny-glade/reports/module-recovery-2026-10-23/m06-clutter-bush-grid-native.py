import pefile,hashlib,struct,random,math
from pathlib import Path
from unicorn import Uc,UC_ARCH_X86,UC_MODE_64,UC_HOOK_CODE
from unicorn.x86_const import *
raw=Path('B版本内容/tiny-glade.rne').read_bytes();assert hashlib.sha256(raw).hexdigest()=='f507a6cea25c5199447b88ba746a5293e64a279f73797ca74c1ce7f4ff2b7d03'
p=pefile.PE(data=raw,fast_load=True);base=p.OPTIONAL_HEADER.ImageBase;image=p.get_memory_mapped_image();u=Uc(UC_ARCH_X86,UC_MODE_64);u.mem_map(base,(len(image)+4095)&~4095);u.mem_write(base,image);D=0x20000000;u.mem_map(D,0x400000)
def xv(reg):return struct.unpack('<f',u.reg_read(reg).to_bytes(16,'little')[:4])[0]
def hook(u,a,size,data):
 if a not in (0x142923ef0,0x142923ee0,0x142923e60):return
 x=xv(UC_X86_REG_XMM0)
 if a==0x142923ef0: val=x*xv(UC_X86_REG_XMM1)+xv(UC_X86_REG_XMM2)
 elif a==0x142923ee0: val=float(math.floor(x))
 else: val=float(math.ceil(x))
 u.reg_write(UC_X86_REG_XMM0,int.from_bytes(struct.pack('<f',val)+bytes(12),'little'))
 rsp=u.reg_read(UC_X86_REG_RSP);ret=struct.unpack('<Q',u.mem_read(rsp,8))[0];u.reg_write(UC_X86_REG_RSP,rsp+8);u.reg_write(UC_X86_REG_RIP,ret)
u.hook_add(UC_HOOK_CODE,hook)
r=random.Random(301);cases=[];grid=D+0x10000;pool=D+0x40000;header=D+0x1000; rect=D+0x2000; point=D+0x3000; stack=D+0x3f0000; stop=D+0x3eff00
for n in range(128):
 x=r.uniform(-68,68);y=r.uniform(-68,68);rad=r.uniform(0,4)
 vals=[x-rad,y-rad,x+rad,y+rad]; vals=struct.unpack('<4f',struct.pack('<4f',*vals))
 if n==0: vals=(64.99,1.,65.01,1.5)
 if n==1: vals=(-10.125,9.875,-9.875,10.125)
 u.mem_write(rect,struct.pack('<4f',*vals));u.mem_write(point,struct.pack('<2f',x,y))
 u.mem_write(header,struct.pack('<QQQ',4096,grid,4096));u.mem_write(grid,bytes(4096*32))
 for i in range(4096):u.mem_write(grid+i*32,struct.pack('<QQQ',1,pool+i*8,0))
 rsp=stack+8;u.mem_write(rsp,struct.pack('<Q',stop));u.reg_write(UC_X86_REG_RSP,rsp);u.reg_write(UC_X86_REG_RCX,rect);u.reg_write(UC_X86_REG_RDX,header);u.reg_write(UC_X86_REG_R8,point)
 u.emu_start(0x141cf3ee0,stop,count=200000);assert u.reg_read(UC_X86_REG_RIP)==stop
 indices=[i for i in range(4096) if struct.unpack('<Q',u.mem_read(grid+i*32+16,8))[0]]
 cases.append((list(struct.unpack('<4I',struct.pack('<4f',*vals))),indices))
Path('tiny-glade/reports/module-recovery-2026-10-23/m06-clutter-bush-grid-native-cases.rs').write_text('&[\n'+''.join('('+str(a).replace(' ','')+',&'+str(b).replace(' ','')+'),\n'for a,b in cases)+']\n')
print(len(cases))
