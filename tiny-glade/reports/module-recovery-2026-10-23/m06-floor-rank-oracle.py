import ctypes, hashlib, json, math, struct
from pathlib import Path
import pefile
from unicorn import Uc, UC_ARCH_X86, UC_MODE_64
from unicorn.x86_const import *
raw=Path('B版本内容/tiny-glade.rne').read_bytes()
sha=hashlib.sha256(raw).hexdigest()
assert sha=='f507a6cea25c5199447b88ba746a5293e64a279f73797ca74c1ce7f4ff2b7d03'
p=pefile.PE(data=raw,fast_load=True); base=p.OPTIONAL_HEADER.ImageBase
u=Uc(UC_ARCH_X86,UC_MODE_64);u.mem_map(base,(p.OPTIONAL_HEADER.SizeOfImage+4095)&~4095);u.mem_write(base,p.get_memory_mapped_image())
u.mem_map(0x30000000,0x10000);sp=0x30008000
lib=ctypes.CDLL('/usr/lib/libSystem.B.dylib');lib.log2f.argtypes=[ctypes.c_float];lib.log2f.restype=ctypes.c_float
def bits(f):return struct.unpack('<I',struct.pack('<f',f))[0]
vals=[0,0x80000000,0x7f800000,0xff800000,0x7fc12345,0x7f812345,1,0x3f800000,0x3eaaaaab,0x402aaaab,0x412aaaaa,0x422aaaab,0x7f7fffff]
for x in (1., 2., 4., 8., 16.,32.):
 b=bits(x/3);vals.extend([b-1,b,b+1])
rows=[]
for v in vals:
 u.reg_write(UC_X86_REG_RSP,sp);u.reg_write(UC_X86_REG_MXCSR,0x1f80);u.reg_write(UC_X86_REG_RFLAGS,0x202)
 u.mem_write(sp+0x6c,struct.pack('<I',v))
 for reg,val in [(UC_X86_REG_XMM7,0),(UC_X86_REG_XMM8,bits(1.)),(UC_X86_REG_XMM9,bits(3.)),(UC_X86_REG_XMM10,bits(5.)),(UC_X86_REG_XMM11,bits(4294967040.))]:u.reg_write(reg,val)
 u.emu_start(0x1407666b7,0x1407666c7,count=10)
 logarg=u.reg_read(UC_X86_REG_XMM0)&0xffffffff
 f=struct.unpack('<f',struct.pack('<I',logarg))[0]
 logresult=bits(lib.log2f(f))
 u.reg_write(UC_X86_REG_XMM0,logresult)
 u.emu_start(0x1407666cc,0x1407666f8,count=30)
 rows.append({'radius_bits':v,'log_arg_bits':logarg,'bridged_log2_bits':logresult,'rank':u.reg_read(UC_X86_REG_RCX)&0xff})
out={'sha256':sha,'instructions':['1407666b7..1407666c7','1407666cc..1407666f8'],'bridge':'macOS libSystem log2f; not Windows UCRT','rows':rows}
Path('tiny-glade/reports/module-recovery-2026-10-23/m06-floor-rank-oracle.json').write_text(json.dumps(out,indent=2)+'\n')
print(len(rows),[(hex(x['radius_bits']),x['rank']) for x in rows])
