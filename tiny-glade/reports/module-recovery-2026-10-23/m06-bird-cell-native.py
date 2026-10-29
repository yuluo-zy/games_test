import ctypes,pefile,hashlib,struct,random,math
from pathlib import Path
from unicorn import Uc,UC_ARCH_X86,UC_MODE_64,UC_HOOK_CODE
from unicorn.x86_const import *
raw=Path('B版本内容/tiny-glade.rne').read_bytes();assert hashlib.sha256(raw).hexdigest()=='f507a6cea25c5199447b88ba746a5293e64a279f73797ca74c1ce7f4ff2b7d03'
p=pefile.PE(data=raw,fast_load=True);base=p.OPTIONAL_HEADER.ImageBase;img=p.get_memory_mapped_image();u=Uc(UC_ARCH_X86,UC_MODE_64);u.mem_map(base,(len(img)+4095)&~4095);u.mem_write(base,img);D=0x20000000;u.mem_map(D,0x10000)
lib=ctypes.CDLL(None);fmaf=lib.fmaf;fmaf.argtypes=[ctypes.c_float]*3;fmaf.restype=ctypes.c_float
def hook(u,a,size,data):
 if a==0x142923ef0:
  def fv(reg):return struct.unpack('<f',u.reg_read(reg).to_bytes(16,'little')[:4])[0]
  value=fmaf(fv(UC_X86_REG_XMM0),fv(UC_X86_REG_XMM1),fv(UC_X86_REG_XMM2));u.reg_write(UC_X86_REG_XMM0,int.from_bytes(struct.pack('<f',value)+bytes(12),'little'))
  rsp=u.reg_read(UC_X86_REG_RSP);dest=struct.unpack('<Q',u.mem_read(rsp,8))[0];u.reg_write(UC_X86_REG_RSP,rsp+8);u.reg_write(UC_X86_REG_RIP,dest)
u.hook_add(UC_HOOK_CODE,hook);r=random.Random(14);rows=[]
for i in range(256):
 vals=[r.uniform(-70,70),r.uniform(-70,70)]
 if i<16: vals=[r.choice([-67.1,-65.01,-65.,-64.999,0.,64.999,65.,65.01,float('nan'),float('inf')]) for _ in range(2)]
 u.reg_write(UC_X86_REG_RSP,D+0xf008);u.mem_write(D+0xf008,struct.pack('<Q',D+0x10));u.reg_write(UC_X86_REG_RCX,D+0x100)
 for reg,v in zip([UC_X86_REG_XMM1,UC_X86_REG_XMM2],vals):u.reg_write(reg,int.from_bytes(struct.pack('<f',v)+bytes(12),'little'))
 u.emu_start(0x140960b20,D+0x10,count=500);option=struct.unpack('<I',u.mem_read(D+0x100,4))[0];out=struct.unpack('<2I',u.mem_read(D+0x104,8)) if option else (0,0)
 rows.append((list(struct.unpack('<2I',struct.pack('<2f',*vals))),option,list(out)))
Path('tiny-glade/reports/module-recovery-2026-10-23/m06-bird-cell-native-cases.rs').write_text('&[\n'+''.join('('+str(a).replace(' ','')+','+str(b)+','+str(c).replace(' ','')+'),\n'for a,b,c in rows)+']\n');print(len(rows))
