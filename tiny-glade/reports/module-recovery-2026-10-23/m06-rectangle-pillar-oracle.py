import hashlib,json,struct,os
from pathlib import Path
import pefile
from unicorn import Uc,UC_ARCH_X86,UC_MODE_64,UC_HOOK_CODE
from unicorn.x86_const import *
raw=Path('B版本内容/tiny-glade.rne').read_bytes();assert hashlib.sha256(raw).hexdigest()=='f507a6cea25c5199447b88ba746a5293e64a279f73797ca74c1ce7f4ff2b7d03'
p=pefile.PE(data=raw,fast_load=True);base=p.OPTIONAL_HEADER.ImageBase;u=Uc(UC_ARCH_X86,UC_MODE_64);u.mem_map(base,(p.OPTIONAL_HEADER.SizeOfImage+4095)&~4095);u.mem_write(base,p.get_memory_mapped_image());u.mem_map(0x30000000,0x20000);u.mem_map(0x40000000,0x100000);sp=0x30010008;heap=0x40000000
bits=lambda x:struct.unpack('<I',struct.pack('<f',x))[0]
proposals=[];recent=[];status='unknown'
def ret(c):
 r=c.reg_read(UC_X86_REG_RSP);c.reg_write(UC_X86_REG_RIP,struct.unpack('<Q',c.mem_read(r,8))[0]);c.reg_write(UC_X86_REG_RSP,r+8)
def hook(c,pc,size,_):
 global heap,status
 recent.append(pc)
 if len(recent)>30:recent.pop(0)
 if pc in (0x141246fb0,):c.reg_write(UC_X86_REG_XMM0,bits(0.));ret(c)
 elif pc==0x141250720:c.reg_write(UC_X86_REG_RAX,1);c.reg_write(UC_X86_REG_XMM0,bits(0.));ret(c)
 elif pc==0x141fe3240:
  vec=c.reg_read(UC_X86_REG_R8);cap,ptr,n=struct.unpack('<QQQ',c.mem_read(vec,24));proposals.extend([list(struct.unpack('<16I',c.mem_read(ptr+i*64,64))) for i in range(n)]);ret(c)
 elif pc==0x140d65810:ret(c)
 elif pc==0x140d657b0:
  size=c.reg_read(UC_X86_REG_RCX);align=c.reg_read(UC_X86_REG_RDX);heap=(heap+align-1)&-align;c.reg_write(UC_X86_REG_RAX,heap);heap+=size;ret(c)
 elif pc==0x140d65850:
  old=c.reg_read(UC_X86_REG_RCX);oldsize=c.reg_read(UC_X86_REG_RDX);align=c.reg_read(UC_X86_REG_R8);size=c.reg_read(UC_X86_REG_R9);heap=(heap+align-1)&-align;c.mem_write(heap,bytes(c.mem_read(old,min(size,oldsize))));c.reg_write(UC_X86_REG_RAX,heap);heap+=size;ret(c)
 elif pc==0x142923c00:
  dst=c.reg_read(UC_X86_REG_RCX);src=c.reg_read(UC_X86_REG_RDX);size=c.reg_read(UC_X86_REG_R8);c.mem_write(dst,bytes(c.mem_read(src,size)));c.reg_write(UC_X86_REG_RAX,dst);ret(c)
 elif pc==0x30000000:status='returned';c.emu_stop()
u.hook_add(UC_HOOK_CODE,hook)
u.mem_write(0x30002000+0x280,struct.pack('<f',4.));rectangle = (0.6,0.8,3.,-2.,4.,6.) if os.getenv('M06_ROTATED') else (1.,0.,0.,0.,4.,4.);u.mem_write(0x30003000,struct.pack('<6f',*rectangle));u.mem_write(0x30004000,struct.pack('<QQQ',0,4,0));u.mem_write(sp,struct.pack('<Q',0x30000000))
for off,val in [(0x28,0x30005000),(0x30,0),(0x38,0x30006000),(0x40,0x30007000),(0x48,0x30008000),(0x50,0x30009000),(0x58,0x3000a000),(0x60,0x30004000)]:u.mem_write(sp+off,struct.pack('<Q',val))
for reg,val in [(UC_X86_REG_RSP,sp),(UC_X86_REG_RCX,77),(UC_X86_REG_RDX,0x30002000),(UC_X86_REG_R8,0x30003000),(UC_X86_REG_R9,0x3000b000),(UC_X86_REG_MXCSR,0x1f80)]:u.reg_write(reg,val)
try:u.emu_start(0x14124fcb0,0x30000001,count=3000000)
except Exception as e:status='fault:'+str(e)+'@'+hex(u.reg_read(UC_X86_REG_RIP))
print(status,'proposals',len(proposals),'last',[hex(x) for x in recent]);print('vec',u.mem_read(0x30004000,24).hex())
assert status=='returned' and len(proposals)==4, (status,len(proposals))
output = 'm06-rectangle-pillar-rotated-native-output.json' if os.getenv('M06_ROTATED') else 'm06-rectangle-pillar-native-output.json'
Path('tiny-glade/reports/module-recovery-2026-10-23',output).write_text(json.dumps(proposals,indent=2)+'\n')

cap,ptr,n=struct.unpack('<QQQ',u.mem_read(0x30004000,24))
bounds=[list(struct.unpack('<8I',u.mem_read(ptr+i*32,32))) for i in range(n)]
Path('tiny-glade/reports/module-recovery-2026-10-23', output.replace('native-output','bounds-native-output')).write_text(json.dumps(bounds,indent=2)+'\n')
