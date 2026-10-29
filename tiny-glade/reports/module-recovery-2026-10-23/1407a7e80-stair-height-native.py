"""原楼梯候选MAX-number和高度clamp指令段；无算法桥接。"""
from pathlib import Path
import hashlib,json,random,struct,pefile
from unicorn import Uc,UC_ARCH_X86,UC_MODE_64
from unicorn.x86_const import *
root=Path(__file__).resolve().parents[3]
raw=(root/'B版本内容/tiny-glade.rne').read_bytes()
sha=hashlib.sha256(raw).hexdigest()
assert sha=='f507a6cea25c5199447b88ba746a5293e64a279f73797ca74c1ce7f4ff2b7d03'
pe=pefile.PE(data=raw,fast_load=True);base=pe.OPTIONAL_HEADER.ImageBase;image=pe.get_memory_mapped_image()
u=Uc(UC_ARCH_X86,UC_MODE_64);u.mem_map(base,(len(image)+4095)&~4095);u.mem_write(base,image)
edge=[0,0x80000000,0x3f800000,0xbf800000,0x42c80000,0x42c80001,0x42c7ffff,0x7f800000,0xff800000,0x7fc01234,0xffc01234,0x7f801234,0xff801234]
rng=random.Random(0x1407a7e80);rows=[]
for a,b in [(a,b)for a in edge for b in edge]+[(rng.getrandbits(32),rng.getrandbits(32))for _ in range(512)]:
    u.reg_write(UC_X86_REG_XMM0,a);u.reg_write(UC_X86_REG_XMM10,b)
    u.emu_start(0x1407a7f34,0x1407a7f53,count=100)
    rows.append((0,a,b,u.reg_read(UC_X86_REG_XMM10)&0xffffffff))
for a in edge+[rng.getrandbits(32)for _ in range(512)]:
    u.reg_write(UC_X86_REG_XMM10,a)
    u.emu_start(0x1407a8225,0x1407a8239,count=100)
    rows.append((1,a,0,u.reg_read(UC_X86_REG_XMM3)&0xffffffff))
out=Path(__file__).with_suffix('.txt');out.write_text('\n'.join(' '.join(f'{v:08x}'for v in row)for row in rows)+'\n')
Path(__file__).with_suffix('.json').write_text(json.dumps({'binary_sha256':sha,'cases':len(rows),'bridges':[],'ranges':['1407a7f34..1407a7f53','1407a8225..1407a8239'],'fixture_sha256':hashlib.sha256(out.read_bytes()).hexdigest(),'boundary':'two original numeric blocks only; not whole candidate or UI acceptance'},indent=2))
print(len(rows))
