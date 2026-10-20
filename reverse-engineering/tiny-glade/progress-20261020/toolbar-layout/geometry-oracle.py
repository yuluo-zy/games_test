import pefile,struct,json
from unicorn import Uc,UC_ARCH_X86,UC_MODE_64
from unicorn.x86_const import *
from pathlib import Path
base=Path('D:/game')
p=base/'reverse-engineering/tiny-glade/workspace/input/tiny-glade.exe'
pe=pefile.PE(str(p)); image=pe.get_memory_mapped_image(); ib=pe.OPTIONAL_HEADER.ImageBase
u=Uc(UC_ARCH_X86,UC_MODE_64);u.mem_map(ib,(len(image)+4095)&~4095);u.mem_write(ib,image)
u.mem_map(0x100000,0x30000)
def bits(xs): return int.from_bytes(struct.pack('<4f',*(xs+[0.]*(4-len(xs)))),'little')
def xmm(n,x):u.reg_write(UC_X86_REG_XMM0+n,bits(x))
def out(n):return struct.unpack('<4f',u.reg_read(UC_X86_REG_XMM0+n).to_bytes(16,'little'))
def read(va,n):return struct.unpack('<'+'f'*n,u.mem_read(va,n*4))
cs={6:0x14304e2fc,7:0x14295a0e4,8:0x1429345a8,9:0x1429258d8,12:0x14295a0e8,11:0x142925870,14:0x14304e310,15:0x14304e300}
print({k:read(v,2 if k in(14,15) else 1) for k,v in cs.items()})
for a in range(4):
 u.reg_write(UC_X86_REG_RSP,0x120000);u.reg_write(UC_X86_REG_R13,0x100000);u.mem_write(0x1000f0,bytes([a]));u.reg_write(UC_X86_REG_ESI,6)
 for k,v in cs.items():xmm(k,list(read(v,2 if k in(14,15) else 1)))
 xmm(13,[1.]);xmm(0,[1000.]);xmm(2,[600.])
 u.emu_start(0x142343f2f,0x142343fdc,count=100)
 print(a,'center',out(0)[0],out(2)[0],'dims-pre',out(3)[0],out(1)[0],'margin-half',out(4)[:2])
