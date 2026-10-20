from pathlib import Path
import pefile,struct,json,hashlib
from unicorn import Uc,UC_ARCH_X86,UC_MODE_64,UC_HOOK_CODE
from unicorn.x86_const import *
HERE=Path(__file__).resolve().parent;ROOT=HERE.parents[3]
data=(ROOT/'reverse-engineering/tiny-glade/workspace/input/tiny-glade.exe').read_bytes()
assert hashlib.sha256(data).hexdigest()=='f507a6cea25c5199447b88ba746a5293e64a279f73797ca74c1ce7f4ff2b7d03'
p=pefile.PE(data=data,fast_load=True);image=p.get_memory_mapped_image();base=p.OPTIONAL_HEADER.ImageBase
results=[]
for demo in (False,True):
 for tag in range(11,22):
  u=Uc(UC_ARCH_X86,UC_MODE_64);u.mem_map(base,(len(image)+4095)&~4095);u.mem_write(base,image);u.mem_map(0x100000,0x100000)
  def q(a):return struct.unpack('<Q',u.mem_read(a,8))[0]
  def hook(u,a,size,_):
   if not 0x142623ff4<=a<0x1426242ee:raise RuntimeError(f'unexpected external {a:x}')
  u.hook_add(UC_HOOK_CODE,hook)
  u.mem_write(0x14362c098,struct.pack('<Q',0x110000));u.mem_write(0x110000,bytes([demo]));u.mem_write(0x110010,bytes([tag]))
  u.mem_write(0x120000,struct.pack('<QQQ',4096,0x130000,0))
  u.mem_write(0x180038,struct.pack('<Q',0x120000));u.mem_write(0x180040,struct.pack('<Q',0x120000));u.mem_write(0x180050,struct.pack('<Q',0x200000001))
  u.reg_write(UC_X86_REG_RSP,0x180000);u.reg_write(UC_X86_REG_R13,0x110010)
  u.emu_start(0x142623ff4,0x1426242ee,count=500)
  n=q(0x120010);commands=[]
  for i in range(n//32):
   a=0x130000+i*32;commands.append({'fn':hex(q(a)),'entity':hex(q(a+16)),'payload':bytes(u.mem_read(a+24,8)).hex()})
  results.append({'demo':demo,'tag':tag,'commands':commands})
print(json.dumps(results,indent=2));(HERE/'photo-category-callbacks.json').write_text(json.dumps(results,indent=2))
