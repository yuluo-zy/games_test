import hashlib,struct,json
from pathlib import Path
import pefile
from unicorn import Uc,UC_ARCH_X86,UC_MODE_64
from unicorn.x86_const import *
raw=Path('B版本内容/tiny-glade.rne').read_bytes();sha=hashlib.sha256(raw).hexdigest();assert sha=='f507a6cea25c5199447b88ba746a5293e64a279f73797ca74c1ce7f4ff2b7d03'
p=pefile.PE(data=raw,fast_load=True);b=p.OPTIONAL_HEADER.ImageBase;c=Uc(UC_ARCH_X86,UC_MODE_64);c.mem_map(b,(p.OPTIONAL_HEADER.SizeOfImage+4095)&~4095);c.mem_write(b,p.get_memory_mapped_image());c.mem_map(0x30000000,0x10000);sp=0x30008000
bits=lambda x:struct.unpack('<I',struct.pack('<f',x))[0]
rows=[]
for origin in [0x7fc12345,0x7f812345,0x7f800000,0xff800000,0x0,0x80000000]:
 c.mem_write(0x30001000,struct.pack('<4I',0,0,bits(2.),0));c.mem_write(0x30002000,bytes(16));c.mem_write(sp,struct.pack('<Q',0x30000000));c.mem_write(sp+0x28,struct.pack('<I',bits(1.)));c.mem_write(sp+0x30,struct.pack('<I',0));
 for reg,v in [(UC_X86_REG_RSP,sp),(UC_X86_REG_RCX,0x30002000),(UC_X86_REG_RDX,0x30001000),(UC_X86_REG_XMM2,origin),(UC_X86_REG_XMM3,0),(UC_X86_REG_MXCSR,0x1f80)]:c.reg_write(reg,v)
 c.emu_start(0x140c97980,0x30000000,count=1000); rows.append({'origin_x_bits':origin,'result_words':list(struct.unpack('<III',c.mem_read(0x30002000,12)))})
Path('tiny-glade/reports/module-recovery-2026-10-23/m06-circle-ray-oracle.json').write_text(json.dumps({'sha256':sha,'scope':'original 140c97980, no bridges','rows':rows},indent=2)+'\n');print(rows)
