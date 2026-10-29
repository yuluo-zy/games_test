import hashlib, json, struct, ctypes, os
from pathlib import Path
import pefile
from unicorn import Uc,UC_ARCH_X86,UC_MODE_64,UC_HOOK_CODE
from unicorn.x86_const import *
raw=Path('B版本内容/tiny-glade.rne').read_bytes();assert hashlib.sha256(raw).hexdigest()=='f507a6cea25c5199447b88ba746a5293e64a279f73797ca74c1ce7f4ff2b7d03'
p=pefile.PE(data=raw,fast_load=True);base=p.OPTIONAL_HEADER.ImageBase
u=Uc(UC_ARCH_X86,UC_MODE_64);u.mem_map(base,(p.OPTIONAL_HEADER.SizeOfImage+4095)&~4095);u.mem_write(base,p.get_memory_mapped_image());u.mem_map(0x30000000,0x20000);u.mem_map(0x40000000,0x100000);sp=0x30010008;heap=0x40000000
lib=ctypes.CDLL('/usr/lib/libSystem.B.dylib')
for n in ('ceilf','cosf','sinf','fmodf'):
 f=getattr(lib,n);f.argtypes=[ctypes.c_float]*(2 if n=='fmodf' else 1);f.restype=ctypes.c_float
bits=lambda x:struct.unpack('<I',struct.pack('<f',x))[0]
num=lambda x:struct.unpack('<f',struct.pack('<I',x&0xffffffff))[0]
rows=[];recent=[];status='unknown'
def ret(c):
 r=c.reg_read(UC_X86_REG_RSP);c.reg_write(UC_X86_REG_RIP,struct.unpack('<Q',c.mem_read(r,8))[0]);c.reg_write(UC_X86_REG_RSP,r+8)
def hook(c,pc,size,_):
 global heap,status
 recent.append(pc)
 if len(recent)>30:recent.pop(0)
 if pc==(0x14124ec4c if os.getenv('M06_WOOD') else 0x14124ed24):c.reg_write(UC_X86_REG_RAX,0x30005000);c.reg_write(UC_X86_REG_RIP,0x14124ecb2 if os.getenv('M06_WOOD') else 0x14124ed8a)
 elif pc in (0x140c81c70,0x140c81e10,0x140d65810):ret(c)
 elif pc==0x140d657b0:
  size=c.reg_read(UC_X86_REG_RCX);align=c.reg_read(UC_X86_REG_RDX);heap=(heap+align-1)&-align;c.reg_write(UC_X86_REG_RAX,heap);heap+=size;ret(c)
 elif pc==0x140d65850:
  old=c.reg_read(UC_X86_REG_RCX);oldsize=c.reg_read(UC_X86_REG_RDX);align=c.reg_read(UC_X86_REG_R8);size=c.reg_read(UC_X86_REG_R9);heap=(heap+align-1)&-align;c.mem_write(heap,bytes(c.mem_read(old,min(size,oldsize))));c.reg_write(UC_X86_REG_RAX,heap);heap+=size;ret(c)
 elif pc==0x142923c00:
  dst=c.reg_read(UC_X86_REG_RCX);src=c.reg_read(UC_X86_REG_RDX);size=c.reg_read(UC_X86_REG_R8);c.mem_write(dst,bytes(c.mem_read(src,size)));c.reg_write(UC_X86_REG_RAX,dst);ret(c)
 elif pc in (0x142923e60,0x142923e80,0x142923fd0):
  func={0x142923e60:lib.ceilf,0x142923e80:lib.cosf,0x142923fd0:lib.sinf}[pc];c.reg_write(UC_X86_REG_XMM0,bits(func(num(c.reg_read(UC_X86_REG_XMM0)))));ret(c)
 elif pc==0x142923f10:c.reg_write(UC_X86_REG_XMM0,bits(lib.fmodf(num(c.reg_read(UC_X86_REG_XMM0)),num(c.reg_read(UC_X86_REG_XMM1)))));ret(c)
 elif pc==(0x14120c430 if os.getenv('M06_WOOD') else 0x14120c120):
  rows.append(list(struct.unpack('<16I' if os.getenv('M06_WOOD') else '<24I',c.mem_read(c.reg_read(UC_X86_REG_RDX),64 if os.getenv('M06_WOOD') else 96))));c.reg_write(UC_X86_REG_RAX,0);ret(c)
 elif pc==0x30000000:status='returned';c.emu_stop()
u.hook_add(UC_HOOK_CODE,hook)
u.mem_write(0x30002000+0x299,bytes([2 if os.getenv('M06_WOOD') else 0]));u.mem_write(0x30002000+0x140,struct.pack('<Q',0x30004000));u.mem_write(0x30001000,struct.pack('<Q',77));u.mem_write(0x30002000+0x150,struct.pack('<Q',0x30004000));u.mem_write(0x30002000+0x280,struct.pack('<2f',3.,14.));u.mem_write(0x30003000,struct.pack('<6f',*( (1.,0.,0.,0.,4.,4.) if os.getenv('M06_SQUARE') else (0.6,0.8,3.,-2.,4.,6.) )));u.mem_write(0x30005000+0x28,struct.pack('<Q',16));u.mem_write(sp,struct.pack('<Q',0x30000000))
for reg,val in [(UC_X86_REG_RSP,sp),(UC_X86_REG_RCX,77),(UC_X86_REG_RDX,0x30002000),(UC_X86_REG_R8,0x30003000),(UC_X86_REG_R9,0x30006000),(UC_X86_REG_MXCSR,0x1f80)]:u.reg_write(reg,val)
try:u.emu_start(0x14124eb80,0x30000001,count=3000000)
except Exception as e:status='fault:'+str(e)+'@'+hex(u.reg_read(UC_X86_REG_RIP))
print(status,'rows',len(rows),'last',[hex(x) for x in recent])
assert status=='returned' and rows, (status,len(rows))
Path('tiny-glade/reports/module-recovery-2026-10-23/' + ('m06-rectangle-wood-bottom-native-output.json' if os.getenv('M06_WOOD') else ('m06-rectangle-square-bottom-native-output.json' if os.getenv('M06_SQUARE') else 'm06-rectangle-bottom-native-output.json'))).write_text(json.dumps(rows,indent=2)+'\n')
