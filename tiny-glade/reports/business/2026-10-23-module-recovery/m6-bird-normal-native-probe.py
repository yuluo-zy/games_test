import struct,hashlib,json,pefile,sys
from unicorn import Uc,UC_ARCH_X86,UC_MODE_64,UC_HOOK_CODE
from unicorn.x86_const import *
raw=open('B版本内容/tiny-glade.rne','rb').read();assert hashlib.sha256(raw).hexdigest()=='f507a6cea25c5199447b88ba746a5293e64a279f73797ca74c1ce7f4ff2b7d03';p=pefile.PE(data=raw,fast_load=True);base=p.OPTIONAL_HEADER.ImageBase
u=Uc(UC_ARCH_X86,UC_MODE_64);u.mem_map(base,(p.OPTIONAL_HEADER.SizeOfImage+4095)&~4095);u.mem_write(base,p.get_memory_mapped_image());u.mem_map(0x30000000,0x400000);u.mem_map(0x40000000,0x1000000)
sp=0x30100008;heap=0x40000000;last=[];hits={}
w=lambda a,b:u.mem_write(a,b);q=lambda a,v:w(a,struct.pack('<Q',v))
def ret(c,v=None):
 if v is not None:c.reg_write(UC_X86_REG_RAX,v)
 s=c.reg_read(UC_X86_REG_RSP);c.reg_write(UC_X86_REG_RIP,struct.unpack('<Q',c.mem_read(s,8))[0]);c.reg_write(UC_X86_REG_RSP,s+8)
def hook(c,pc,size,_):
 global heap
 last.append(pc);del last[:-25];hits[hex(pc)]=hits.get(hex(pc),0)+1
 if pc==0x140c16a00:ret(c,0);return
 if pc==0x1405b0d70:ret(c,0);return
 if pc==0x1412269b0:hits["generate"]=hits.get("generate",0)+1;ret(c);return
 if pc==0x14075da71:hits["missing_wall_panic"]=1;c.emu_stop();return
 if pc==0x140613c10:
  n=c.reg_read(UC_X86_REG_RCX);a=c.reg_read(UC_X86_REG_RDX);heap=(heap+a-1)&-a;v=heap;heap+=max(n,1);ret(c,v);return
 if pc==0x140613c20:ret(c);return
u.hook_add(UC_HOOK_CODE,hook)
q(base+0x362c420,0x30001000);q(0x30001000,0);w(base+0x362bfc0,bytes(32))
# Vec<Polyline> and contiguous native VecDeque<PolylinePoint>.
q(0x30002000,1);q(0x30002008,0x30002100);q(0x30002010,1)
q(0x30002100,3);q(0x30002108,0x30002200);q(0x30002110,0);q(0x30002118,3)
for i,(x,y,z) in enumerate([(2.,0.,0.),(1.,2.,0.),(0.,2.,0.)]):
 w(0x30002200+i*24,struct.pack('<6f',float(i),0.,0.,x,y,z))
q(sp,0x30000000)
for r,v in [(UC_X86_REG_RSP,sp),(UC_X86_REG_RCX,0x30002000),(UC_X86_REG_MXCSR,0x1f80)]:u.reg_write(r,v)
try:u.emu_start(0x1412f3850,0x30000000,count=100000);result={'returned':u.reg_read(UC_X86_REG_RIP)==0x30000000}
except Exception as e:result={'error':str(e),'pc':hex(u.reg_read(UC_X86_REG_RIP))}
result['points_hex']=bytes(u.mem_read(0x30002200,72)).hex();print(json.dumps(result))
