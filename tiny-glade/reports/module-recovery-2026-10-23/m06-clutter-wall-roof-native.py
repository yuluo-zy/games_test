import pefile,hashlib,struct,random
from pathlib import Path
from unicorn import Uc,UC_ARCH_X86,UC_MODE_64
from unicorn.x86_const import *
raw=Path('B版本内容/tiny-glade.rne').read_bytes();assert hashlib.sha256(raw).hexdigest()=='f507a6cea25c5199447b88ba746a5293e64a279f73797ca74c1ce7f4ff2b7d03'
p=pefile.PE(data=raw,fast_load=True);base=p.OPTIONAL_HEADER.ImageBase;image=p.get_memory_mapped_image();u=Uc(UC_ARCH_X86,UC_MODE_64);u.mem_map(base,(len(image)+4095)&~4095);u.mem_write(base,image);D=0x20000000;u.mem_map(D,0x10000)
r=random.Random(422);rows=[]
for i in range(128):
 v=[r.uniform(-40,40) for _ in range(7)]+[r.uniform(.1,5)]
 ax,az,bx,bz,x,y,z,size=v;rbp=D+0x1000;seg=D+0x5000
 u.mem_write(seg,struct.pack('<4f',ax,az,bx,bz));u.mem_write(rbp+0x6d0,struct.pack('<Q',seg));u.mem_write(rbp+0x6d8,struct.pack('<Q',3));u.reg_write(UC_X86_REG_RBP,rbp);u.reg_write(UC_X86_REG_RSP,D+0xf008);u.reg_write(UC_X86_REG_RDX,seg)
 for reg,data in [(UC_X86_REG_XMM10,[y]),(UC_X86_REG_XMM11,[x,y]),(UC_X86_REG_XMM12,[z]),(UC_X86_REG_XMM14,[size])]:u.reg_write(reg,int.from_bytes(struct.pack('<'+'f'*len(data),*data)+bytes(16-4*len(data)),'little'))
 try: u.emu_start(0x1422d952b,0x1422d97d9,count=500)
 except Exception: print(hex(u.reg_read(UC_X86_REG_RIP))); raise
 print('',end='')
 inp=struct.unpack('<8I',struct.pack('<8f',*v)); out=tuple(struct.unpack('<I',u.reg_read(reg).to_bytes(16,'little')[:4])[0] for reg in (UC_X86_REG_XMM11,UC_X86_REG_XMM12,UC_X86_REG_XMM14));norm=struct.unpack('<3I',u.mem_read(rbp+0x4d0,12));rows.append((inp,out,norm))
Path('tiny-glade/reports/module-recovery-2026-10-23/m06-clutter-wall-roof-native-cases.rs').write_text('&[\n'+''.join('('+str(list(a)).replace(' ','')+','+str(list(b)).replace(' ','')+','+str(list(c)).replace(' ','')+'),\n' for a,b,c in rows)+']\n');print(len(rows))
