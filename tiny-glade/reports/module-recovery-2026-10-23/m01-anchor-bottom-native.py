"""原B14081d3de..421门底profile/碰撞高度块隔离真执行，输出逐位oracle。"""
from pathlib import Path
import struct, json, hashlib, itertools, pefile
from unicorn import Uc, UC_ARCH_X86, UC_MODE_64
from unicorn.x86_const import *
root = Path(__file__).resolve().parents[3]
raw = (root / 'B版本内容/tiny-glade.rne').read_bytes()
sha = hashlib.sha256(raw).hexdigest()
assert sha == 'f507a6cea25c5199447b88ba746a5293e64a279f73797ca74c1ce7f4ff2b7d03'
pe = pefile.PE(data=raw, fast_load=True)
base = pe.OPTIONAL_HEADER.ImageBase
image = pe.get_memory_mapped_image()
u = Uc(UC_ARCH_X86, UC_MODE_64)
u.mem_map(base, (len(image) + 4095) & ~4095)
u.mem_write(base, image)
data = 0x20000000
u.mem_map(data, 4096)
values = [0, 0x80000000, 0x3f000000, 0x3f800000, 0xbf800000,
          0x7f800000, 0xff800000, 0x7fc12345, 0xffc54321,
          1, 0x80000001, 0x7f7fffff, 0xff7fffff, 0x3d4ccccd]
cases = list(itertools.product([0,0x3f000000,0x3f800000], values, values, [0], [0x40000000]))
state = 0x8127c790
for _ in range(512):
    row=[]
    for _ in range(5):
        state = (1664525 * state + 1013904223) & 0xffffffff
        row.append(state)
    cases.append(tuple(row))
rows=[]
for t,a,b,bottom,top in cases:
    u.mem_write(data + 0x38, struct.pack('<II',bottom,top))
    for reg,bits in [(UC_X86_REG_XMM6,t),(UC_X86_REG_XMM7,a),(UC_X86_REG_XMM14,b),
                     (UC_X86_REG_XMM9,0),(UC_X86_REG_XMM12,0x3f000000),(UC_X86_REG_XMM13,0x3d4ccccd)]:
        u.reg_write(reg,bits)
    u.reg_write(UC_X86_REG_RAX,data)
    u.emu_start(0x14081d3de,0x14081d421,count=100)
    rows.append([t,a,b,bottom,top,u.reg_read(UC_X86_REG_XMM6)&0xffffffff])
out=Path(__file__).with_suffix('.txt')
out.write_text('\n'.join(' '.join(f'{x:08x}' for x in row)for row in rows)+'\n')
Path(__file__).with_suffix('.json').write_text(json.dumps({'binary_sha256':sha,'block':'0x14081d3de..0x14081d421','cases':len(rows),'fixture_sha256':hashlib.sha256(out.read_bytes()).hexdigest()},indent=2))
print(json.dumps({'cases':len(rows),'sha':sha}))
