from pathlib import Path
import pefile,struct,json,hashlib
from unicorn import Uc,UC_ARCH_X86,UC_MODE_64,UC_HOOK_CODE
from unicorn.x86_const import *
ROOT=Path(__file__).resolve().parent;raw=Path('D:/game/ljxsj_92385/Tiny Glade/tiny-glade.rne').read_bytes();pe=pefile.PE(data=raw,fast_load=True);base=pe.OPTIONAL_HEADER.ImageBase
cpu=Uc(UC_ARCH_X86,UC_MODE_64);cpu.mem_map(base,(pe.OPTIONAL_HEADER.SizeOfImage+4095)&~4095);cpu.mem_write(base,pe.get_memory_mapped_image());DATA,STACK,STOP=0x20000000,0x30000000,0x50000000
for p,n in [(DATA,0x100000),(STACK,0x100000),(STOP,0x1000)]:cpu.mem_map(p,n)
segments=[]
def u64(a):return struct.unpack('<Q',cpu.mem_read(a,8))[0]
def put(a,fmt,*v):cpu.mem_write(a,struct.pack(fmt,*v))
def hook(c,pc,size,user):
 if pc==STOP:c.emu_stop()
 elif pc==0x140106160:
  p=c.reg_read(UC_X86_REG_RDX);n=c.reg_read(UC_X86_REG_R8);segments.append(bytes(c.mem_read(p,n)).hex())
 elif pc in (0x142923c00,0x142923c10):
  dst,src,n=[c.reg_read(x)for x in (UC_X86_REG_RCX,UC_X86_REG_RDX,UC_X86_REG_R8)]
  if n:c.mem_write(dst,bytes(c.mem_read(src,n)))
  sp=c.reg_read(UC_X86_REG_RSP);ret=u64(sp);c.reg_write(UC_X86_REG_RSP,sp+8);c.reg_write(UC_X86_REG_RAX,dst);c.reg_write(UC_X86_REG_RIP,ret)
cpu.hook_add(UC_HOOK_CODE,hook)
def run(widget,name):
 segments.clear();sp=STACK+0xf0008;put(sp,'<Q',STOP);b=b''if name is None else name.encode();ptr=0 if name is None else DATA+0x100;put(DATA,'<QQQ',widget,ptr,len(b));cpu.mem_write(DATA+0x100,b or b'\0');cpu.reg_write(UC_X86_REG_RCX,DATA);cpu.reg_write(UC_X86_REG_RSP,sp);cpu.emu_start(0x14145a710,STOP,count=100000)
 assert cpu.reg_read(UC_X86_REG_RIP)==STOP
 return dict(widget=widget,name=name,segments=list(segments),hash=cpu.reg_read(UC_X86_REG_RAX))
if __name__=='__main__':
 rows=[run(w,n)for w in [0,1,11,91,0x0123456789abcdef,0xffffffffffffffff]for n in [None,'','water','wall','fences','roof height','a\0b','草地','x'*7,'x'*8,'x'*9,'x'*16,'x'*33]]
 for r in rows:
  want=[struct.pack('<Q',r['widget']).hex(),struct.pack('<Q',r['name']is not None).hex()]
  if r['name']is not None:want +=[r['name'].encode().hex(),'ff']
  assert want==r['segments'],r
 (ROOT/'native-hash-fixtures.json').write_text(json.dumps(rows,indent=2));print(len(rows),'native hashes + original Hash input sequences passed')
