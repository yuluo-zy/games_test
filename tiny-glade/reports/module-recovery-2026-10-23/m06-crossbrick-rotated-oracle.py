import hashlib, json, struct, ctypes
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
 if pc==0x14124b3f4:c.reg_write(UC_X86_REG_RAX,0x30005000);c.reg_write(UC_X86_REG_RIP,0x14124b45e)
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
 elif pc==0x14120c120:
  rows.append(list(struct.unpack('<24I',c.mem_read(c.reg_read(UC_X86_REG_RDX),96))));c.reg_write(UC_X86_REG_RAX,0);ret(c)
 elif pc==0x30000000:status='returned';c.emu_stop()
u.hook_add(UC_HOOK_CODE,hook)
u.mem_write(0x30002000,struct.pack('<Q',123));u.mem_write(0x30001000,struct.pack('<QQ',0x30001100,0x30001200));u.mem_write(sp,struct.pack('<Q',0x30000000))
for off,val in [(0x28,bits(1.)),(0x30,0x30002000),(0x38,bits(0.6)),(0x40,bits(0.8)),(0x48,0x30001000),(0x50,77),(0x58,1)]:u.mem_write(sp+off,struct.pack('<Q',val))
for reg,val in [(UC_X86_REG_RSP,sp),(UC_X86_REG_XMM0,bits(1.)),(UC_X86_REG_XMM1,bits(2.)),(UC_X86_REG_XMM2,bits(-3.)),(UC_X86_REG_XMM3,bits(5.)),(UC_X86_REG_MXCSR,0x1f80)]:u.reg_write(reg,val)
try:u.emu_start(0x142418f50,0x30000001,count=3000000)
except Exception as e:status='fault:'+str(e)+'@'+hex(u.reg_read(UC_X86_REG_RIP))
print(status,'rows',len(rows),'last',[hex(x) for x in recent])
assert status=='returned' and len(rows)==36, (status,len(rows))
Path('tiny-glade/reports/module-recovery-2026-10-23/m06-crossbrick-rotated-native-output.json').write_text(json.dumps(rows,indent=2)+'\n')
