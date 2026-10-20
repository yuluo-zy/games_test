from pathlib import Path
import pefile,struct,json
from unicorn import Uc,UC_ARCH_X86,UC_MODE_64,UC_HOOK_CODE
from unicorn.x86_const import *
H=Path(__file__).resolve().parent;data=(H.parents[3]/'reverse-engineering/tiny-glade/workspace/input/tiny-glade.exe').read_bytes();p=pefile.PE(data=data,fast_load=True);image=p.get_memory_mapped_image();base=p.OPTIONAL_HEADER.ImageBase
out=[]
for alignment in range(4):
 u=Uc(UC_ARCH_X86,UC_MODE_64);u.mem_map(base,(len(image)+4095)&~4095);u.mem_write(base,image);u.mem_map(0x100000,0x100000);calls=[]
 def q(a,v):u.mem_write(a,struct.pack('<Q',v))
 def hook(u,a,size,_):
  if a in [0x142376050,0x142376300,0x142375b40,0x142375c10,0x142375f80,0x142375eb0,0x142375ce0,0x142376230]:
   calls.append({'setter':hex(a),'node':[u.reg_read(UC_X86_REG_EDX),u.reg_read(UC_X86_REG_R8D)]})
   sp=u.reg_read(UC_X86_REG_RSP);u.reg_write(UC_X86_REG_RIP,struct.unpack('<Q',u.mem_read(sp,8))[0]);u.reg_write(UC_X86_REG_RSP,sp+8)
 u.hook_add(UC_HOOK_CODE,hook)
 q(0x110000,0x120000);u.mem_write(0x1200f0,bytes([alignment]));q(0x110100,0x120100);q(0x110110,0x120200);q(0x110200,0x120300)
 q(0x120300,0x2200000001);q(0x120308,0x3300000002);q(0x180008,0x1f0000)
 for reg,value in [(UC_X86_REG_RSP,0x180008),(UC_X86_REG_RCX,0x110000),(UC_X86_REG_RDX,0x110100),(UC_X86_REG_R8,0x110200)]:u.reg_write(reg,value)
 u.emu_start(0x1423448a0,0x1f0000,count=500);assert len(calls)==3
 out.append({'alignment':alignment,'calls':calls})
(H/'native-alignment-calls.json').write_text(json.dumps(out,indent=2));print(json.dumps(out,indent=2))
