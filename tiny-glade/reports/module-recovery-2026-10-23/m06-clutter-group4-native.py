import pefile,hashlib,struct,random
from pathlib import Path
from unicorn import Uc,UC_ARCH_X86,UC_MODE_64
from unicorn.x86_const import *
raw=Path('B版本内容/tiny-glade.rne').read_bytes();assert hashlib.sha256(raw).hexdigest()=='f507a6cea25c5199447b88ba746a5293e64a279f73797ca74c1ce7f4ff2b7d03'
p=pefile.PE(data=raw,fast_load=True);base=p.OPTIONAL_HEADER.ImageBase;img=p.get_memory_mapped_image();u=Uc(UC_ARCH_X86,UC_MODE_64);u.mem_map(base,(len(img)+4095)&~4095);u.mem_write(base,img);D=0x20000000;u.mem_map(D,0x20000)


slots=[45,56,57,39,40,50,48,49,51,52,46,25,24,23,21,20,19,17,16,22,13,14,15,18,28,29,26,27,9,8,3,4,5,6,7,10,11]
r=random.Random(421);rows=[]
for j in range(128):
 rbp=D+0x1000;table=D+0x4000;objects=D+0x8000;out=D+0xe000;mask=r.getrandbits(37);selector=j%2
 for n in range(60):
  u.mem_write(table+n*8,struct.pack('<Q',objects+n*64));u.mem_write(objects+n*64+0x30,struct.pack('<Q',0xffffffffffffffff if n in slots and not(mask & (1<<slots.index(n))) else 7))
 u.mem_write(rbp+0x6f8,struct.pack('<Q',table));u.mem_write(rbp+0x670,struct.pack('<Q',out));u.mem_write(rbp+0x668,struct.pack('<Q',64));u.mem_write(rbp+0x6f0,struct.pack('<f',float(selector!=1)));u.reg_write(UC_X86_REG_RBP,rbp);u.reg_write(UC_X86_REG_RSP,D+0x1f008)
 u.emu_start(0x1422db824,0x1422d7d2a,count=5000);assert u.reg_read(UC_X86_REG_RIP)==0x1422d7d2a
 actual=[struct.unpack('<QI',u.mem_read(out+i*16,12)) for i in range(37)];assert [(ptr-objects)//64 for ptr,_ in actual]==slots
 rows.append((mask,selector,[w for _,w in actual]))
Path('tiny-glade/reports/module-recovery-2026-10-23/m06-clutter-group4-native-cases.rs').write_text('&[\n'+''.join('('+str(m)+','+str(s)+','+str(w).replace(' ','')+'),\n'for m,s,w in rows)+']\n');print(len(rows))
