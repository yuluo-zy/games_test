"""原LazyTile From<UVec2>140aac630、from_index140833250及消费端1412946a2..6b2。"""
from pathlib import Path
import hashlib,json,struct,pefile
from unicorn import Uc,UC_ARCH_X86,UC_MODE_64
from unicorn.x86_const import *
p=Path(__file__).resolve();root=p.parents[3];raw=(root/'B版本内容/tiny-glade.rne').read_bytes()
assert hashlib.sha256(raw).hexdigest()=='f507a6cea25c5199447b88ba746a5293e64a279f73797ca74c1ce7f4ff2b7d03'
pe=pefile.PE(data=raw,fast_load=True);base=pe.OPTIONAL_HEADER.ImageBase
u=Uc(UC_ARCH_X86,UC_MODE_64);u.mem_map(base,(pe.OPTIONAL_HEADER.SizeOfImage+4095)&~4095);u.mem_write(base,pe.get_memory_mapped_image());u.mem_map(0x200000,0x10000)
rows=[]
for x in [0,1,63,64,255,256,4095,4096,16383,16384,0xffffffff,0x80000000]:
 for y in [0,1,63,64,255,256,4095,4096,16383,16384,0xffffffff,0x80000000]:
  stack=0x208008;stop=0x209000;u.mem_write(stack,struct.pack('<Q',stop));u.reg_write(UC_X86_REG_RSP,stack);u.reg_write(UC_X86_REG_RCX,x|(y<<32));u.emu_start(0x140aac630,stop,count=20);index=u.reg_read(UC_X86_REG_RAX)&0xffffffff
  u.mem_write(stack,struct.pack('<Q',stop));u.reg_write(UC_X86_REG_RSP,stack);u.reg_write(UC_X86_REG_RCX,index);u.emu_start(0x140833250,stop,count=20);forward=u.reg_read(UC_X86_REG_RAX)&0xffffffff
  u.reg_write(UC_X86_REG_RSP,stack);u.emu_start(0x1412946a2,0x1412946b2,count=10);key=list(u.mem_read(stack+0x26,2));rows.append(dict(x=x,y=y,index=index,forward=forward,key=key))
out=p.with_suffix('.json')
with out.open('x')as f:json.dump(dict(scope=__doc__,rne_sha256=hashlib.sha256(raw).hexdigest(),rows=rows),f,indent=2)
fixture=p.with_name('m01-stair-lazy-tile-native-cases.rs')
with fixture.open('x')as f:
 f.write('// 原指令144项，包含32位溢出与消费端u8截断。\nconst ORIGINAL_LAZY_TILES:&[(u32,u32,u32,[u8;2])]=&[\n')
 for o in rows:f.write('(%d,%d,%d,[%d,%d]),\n'%(o['x'],o['y'],o['index'],*o['key']))
 f.write('];\n')
print(json.dumps(dict(cases=len(rows))))
