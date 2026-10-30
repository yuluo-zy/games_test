import hashlib,struct,json,itertools
from pathlib import Path
import pefile
from unicorn import Uc,UC_ARCH_X86,UC_MODE_64
from unicorn.x86_const import *
raw=Path('B版本内容/tiny-glade.rne').read_bytes();sha=hashlib.sha256(raw).hexdigest();assert sha=='f507a6cea25c5199447b88ba746a5293e64a279f73797ca74c1ce7f4ff2b7d03'
p=pefile.PE(data=raw,fast_load=True);b=p.OPTIONAL_HEADER.ImageBase;c=Uc(UC_ARCH_X86,UC_MODE_64);c.mem_map(b,(p.OPTIONAL_HEADER.SizeOfImage+4095)&~4095);c.mem_write(b,p.get_memory_mapped_image());c.mem_map(0x30000000,0x10000);sp=0x30008000
nan=[0x7fc12345,0xffc54321,0x7f812345,0xff854321];base=[0,0,0x40000000,0,0,0x3f800000,0]; cases=[]
for i in (0,1,2,3,4,5,6):
 for v in nan:
  row=base.copy();row[i]=v;cases.append(row)
for i,j in itertools.combinations(range(7),2):
 row=base.copy();row[i]=nan[0];row[j]=nan[1];cases.append(row)
rows=[]
for cx,cy,r,ox,oy,dx,dy in cases:
 c.mem_write(0x30001000,struct.pack('<4I',cx,cy,r,0));c.mem_write(0x30002000,bytes(16));c.mem_write(sp,struct.pack('<Q',0x30000000));c.mem_write(sp+0x28,struct.pack('<I',dx));c.mem_write(sp+0x30,struct.pack('<I',dy));
 for reg,v in [(UC_X86_REG_RSP,sp),(UC_X86_REG_RCX,0x30002000),(UC_X86_REG_RDX,0x30001000),(UC_X86_REG_XMM2,ox),(UC_X86_REG_XMM3,oy),(UC_X86_REG_MXCSR,0x1f80)]:c.reg_write(reg,v)
 c.emu_start(0x140c97980,0x30000000,count=1000);rows.append({'input_bits':[cx,cy,r,ox,oy,dx,dy],'result_words':list(struct.unpack('<III',c.mem_read(0x30002000,12)))})
Path('tiny-glade/reports/module-recovery-2026-10-23/m06-circle-ray-nan-oracle.json').write_text(json.dumps({'sha256':sha,'scope':'original 140c97980 no bridges','rows':rows},indent=2)+'\n');print(len(rows))
