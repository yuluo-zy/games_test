from pathlib import Path
import hashlib,json,struct,random
import pefile
from unicorn import Uc,UC_ARCH_X86,UC_MODE_64
from unicorn.x86_const import *
OUT=Path(__file__).resolve().parent
ROOT=OUT.parents[2]
raw=(ROOT/'B版本内容/tiny-glade.rne').read_bytes()
assert hashlib.sha256(raw).hexdigest()=='f507a6cea25c5199447b88ba746a5293e64a279f73797ca74c1ce7f4ff2b7d03'
p=pefile.PE(data=raw,fast_load=True);base=p.OPTIONAL_HEADER.ImageBase
u=Uc(UC_ARCH_X86,UC_MODE_64);u.mem_map(base,(p.OPTIONAL_HEADER.SizeOfImage+4095)&~4095);u.mem_write(base,p.get_memory_mapped_image());u.mem_map(0x20000000,0x1000)
u.reg_write(UC_X86_REG_R12,0x20000000);u.reg_write(UC_X86_REG_MXCSR,0x1f80)
values=[0,0x80000000,0x3f000000,0x3f000001,0x3effffff,0x40400000,0x403fffff,0x40400001,0x3f800000,0x40000000,0x7f800000,0xff800000,0x7fc12345,0xffc12345,0x7f812345,0xff812345,0x7f7fffff,0xff7fffff]
rng=random.Random(0x14097d280);values += [rng.getrandbits(32) for _ in range(128)]
rows=[]
for bits in values:
 u.mem_write(0x20000030,struct.pack('<I',bits));u.emu_start(0x14097d2c4,0x14097d326,count=100)
 rows.append([bits,u.reg_read(UC_X86_REG_XMM7)&0xffffffff])
out={'original_sha256':hashlib.sha256(raw).hexdigest(),'range':['0x14097d2c4','0x14097d326'],'cases':rows}
(OUT/'m01-move-gizmo-native.json').write_text(json.dumps(out,indent=2))
print(len(rows))
