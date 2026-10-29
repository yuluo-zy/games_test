import hashlib, json, struct
from pathlib import Path
import pefile
from unicorn import Uc, UC_ARCH_X86, UC_MODE_64, UC_HOOK_CODE
from unicorn.x86_const import *
raw=Path('B版本内容/tiny-glade.rne').read_bytes();sha=hashlib.sha256(raw).hexdigest();assert sha=='f507a6cea25c5199447b88ba746a5293e64a279f73797ca74c1ce7f4ff2b7d03'
p=pefile.PE(data=raw,fast_load=True);base=p.OPTIONAL_HEADER.ImageBase
u=Uc(UC_ARCH_X86,UC_MODE_64);u.mem_map(base,(p.OPTIONAL_HEADER.SizeOfImage+4095)&~4095);u.mem_write(base,p.get_memory_mapped_image());u.mem_map(0x30000000,0x20000);sp=0x30010008
bits=lambda x:struct.unpack('<I',struct.pack('<f',x))[0]
record=None;status='unknown';calls=[]
def ret(c):
 r=c.reg_read(UC_X86_REG_RSP);c.reg_write(UC_X86_REG_RIP,struct.unpack('<Q',c.mem_read(r,8))[0]);c.reg_write(UC_X86_REG_RSP,r+8)
def hook(c,pc,size,_):
 global record,status
 if pc==0x140caf380:
  calls.append('rng');c.reg_write(UC_X86_REG_XMM0,bits(0.25));ret(c)
 elif pc==0x14120c120:
  calls.append('push');record=bytes(c.mem_read(c.reg_read(UC_X86_REG_RDX),96));c.reg_write(UC_X86_REG_RAX,0);ret(c)
 elif pc==0x30000000:status='returned';c.emu_stop()
u.hook_add(UC_HOOK_CODE,hook)
u.mem_write(0x30001000,struct.pack('<QQ',0x30001200,0x30001100));u.mem_write(sp,struct.pack('<Q',0x30000000))
for off,value in [(0x28,bits(6.)),(0x30,bits(3.)),(0x38,0x30001000),(0x40,77),(0x48,bits(4.)),(0x50,9),(0x58,struct.unpack('<Q',struct.pack('<2f',0.25,0.75))[0])]:u.mem_write(sp+off,struct.pack('<Q',value))
for reg,val in [(UC_X86_REG_RSP,sp),(UC_X86_REG_RCX,0x30001400),(UC_X86_REG_XMM1,bits(1.)),(UC_X86_REG_XMM2,bits(2.)),(UC_X86_REG_XMM3,bits(4.)),(UC_X86_REG_MXCSR,0x1f80)]:u.reg_write(reg,val)
try:u.emu_start(0x141fe1de0,0x30000001,count=50000)
except Exception as e:status='fault:'+str(e)+'@'+hex(u.reg_read(UC_X86_REG_RIP))
assert status=='returned' and record is not None,(status,calls)
Path('tiny-glade/reports/module-recovery-2026-10-23/m06-bottom-brick-oracle.json').write_text(json.dumps({'sha256':sha,'scope':'original add_bottom_brick; rng=0.25 and push success bridged; all geometry/packing original instructions','inputs':{'a':[1.,2.],'b':[4.,6.],'base':3.,'height':4.,'color':77,'seed':9,'distance':[0.25,0.75]},'calls':calls,'words':list(struct.unpack('<24I',record))},indent=2)+'\n');print(status,calls)
