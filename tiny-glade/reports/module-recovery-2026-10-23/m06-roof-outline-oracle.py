import hashlib, json, struct, ctypes, sys
from pathlib import Path
import pefile
from unicorn import Uc, UC_ARCH_X86, UC_MODE_64, UC_HOOK_CODE
from unicorn.x86_const import *
raw=Path('B版本内容/tiny-glade.rne').read_bytes()
assert hashlib.sha256(raw).hexdigest()=='f507a6cea25c5199447b88ba746a5293e64a279f73797ca74c1ce7f4ff2b7d03'
p=pefile.PE(data=raw,fast_load=True); base=p.OPTIONAL_HEADER.ImageBase
u=Uc(UC_ARCH_X86,UC_MODE_64); u.mem_map(base,(p.OPTIONAL_HEADER.SizeOfImage+4095)&~4095); u.mem_write(base,p.get_memory_mapped_image())
u.mem_map(0x30000000,0x40000); u.mem_map(0x40000000,0x1000000)
rbp=0x30020000; sp=rbp-0x80; shape=0x30003000; roof=0x30004000; heap=0x40000000; rows=[]; recent=[]; status='unset'
case=int(sys.argv[1]) if len(sys.argv)>1 else 0
lib=ctypes.CDLL('/usr/lib/libSystem.B.dylib')
for n in ('powf','roundf'):
 f=getattr(lib,n); f.argtypes=[ctypes.c_float]*(2 if n=='powf' else 1); f.restype=ctypes.c_float
bits=lambda x:struct.unpack('<I',struct.pack('<f',x))[0]
num=lambda x:struct.unpack('<f',struct.pack('<I',x&0xffffffff))[0]
def q(addr,v): u.mem_write(addr,struct.pack('<Q',v))
def ret(c):
 r=c.reg_read(UC_X86_REG_RSP);c.reg_write(UC_X86_REG_RIP,struct.unpack('<Q',c.mem_read(r,8))[0]);c.reg_write(UC_X86_REG_RSP,r+8)
def hook(c,pc,size,_):
 global heap,status
 recent.append(pc); del recent[:-40]
 if pc==0x1412188e0: status='edge';c.emu_stop()
 elif pc in (0x140613c20,0x140d65810,0x140c81c70,0x140c81e10):ret(c)
 elif pc in (0x140613c10,0x140d657b0):
  n=c.reg_read(UC_X86_REG_RCX); a=c.reg_read(UC_X86_REG_RDX);heap=(heap+a-1)&-a;c.reg_write(UC_X86_REG_RAX,heap);heap+=n;ret(c)
 elif pc==0x140d65850:
  old=c.reg_read(UC_X86_REG_RCX);oldsize=c.reg_read(UC_X86_REG_RDX);a=c.reg_read(UC_X86_REG_R8);n=c.reg_read(UC_X86_REG_R9);heap=(heap+a-1)&-a;c.mem_write(heap,bytes(c.mem_read(old,min(n,oldsize))));c.reg_write(UC_X86_REG_RAX,heap);heap+=n;ret(c)
 elif pc==0x142923c00:
  d=c.reg_read(UC_X86_REG_RCX);s=c.reg_read(UC_X86_REG_RDX);n=c.reg_read(UC_X86_REG_R8);c.mem_write(d,bytes(c.mem_read(s,n)));c.reg_write(UC_X86_REG_RAX,d);ret(c)
 elif pc==0x142923f70:c.reg_write(UC_X86_REG_XMM0,bits(lib.powf(num(c.reg_read(UC_X86_REG_XMM0)),num(c.reg_read(UC_X86_REG_XMM1)))));ret(c)
 elif pc==0x142923fa0:c.reg_write(UC_X86_REG_XMM0,bits(lib.roundf(num(c.reg_read(UC_X86_REG_XMM0)))));ret(c)
 elif pc==0x14120c430:
  rows.append(bytes(c.mem_read(c.reg_read(UC_X86_REG_RDX),64)).hex());c.reg_write(UC_X86_REG_RAX,0);ret(c)
u.hook_add(UC_HOOK_CODE,hook)
# Valid rectangular wall: axis=(1,0), center=(0,0), dimensions=(6,4).
axis=(1,0) if case%4<2 else (0,1)
u.mem_write(shape,struct.pack('<I6f',1,*axis,0,0,6,4))
b=bytearray(88); struct.pack_into('<I6f',b,8,1,*axis,0,0,6,4);struct.pack_into('<fff',b,0x2c,0.2 if case>=4 else 0.5,0.8 if case>=4 else 0.5,1.0);
if case>=4: struct.pack_into('<ff',b,0x24,0.2,-0.3);struct.pack_into('<f',b,0x38,0.7)
b[0x3c]=case%2;struct.pack_into('<f',b,0x4c,2.0);u.mem_write(roof,bytes(b))
q(rbp+0x320,roof);q(rbp+0xe8,123)
u.mem_write(0x30005000+0x258,bytes(u.mem_read(shape,28)))
for reg,val in [(UC_X86_REG_RSP,sp),(UC_X86_REG_RBP,rbp),(UC_X86_REG_R12,shape),(UC_X86_REG_RSI,0x30005000),(UC_X86_REG_RDX,roof),(UC_X86_REG_MXCSR,0x1f80)]:u.reg_write(reg,val)
try:u.emu_start(0x141217d88,0x1412188e1,count=10000000)
except Exception as e:status='fault:'+str(e)+'@'+hex(u.reg_read(UC_X86_REG_RIP))
print(status,len(rows),'last',list(map(hex,recent[-12:])))
if status=='edge':Path(f'tiny-glade/reports/module-recovery-2026-10-23/m06-roof-outline-native-output-{case}.json').write_text(json.dumps(rows,indent=2)+'\n')
