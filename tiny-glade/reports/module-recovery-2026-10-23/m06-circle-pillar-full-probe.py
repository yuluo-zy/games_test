import hashlib,json,struct,ctypes,sys
from pathlib import Path
import pefile
from unicorn import Uc,UC_ARCH_X86,UC_MODE_64,UC_HOOK_CODE
from unicorn.x86_const import *
raw=Path('B版本内容/tiny-glade.rne').read_bytes();assert hashlib.sha256(raw).hexdigest()=='f507a6cea25c5199447b88ba746a5293e64a279f73797ca74c1ce7f4ff2b7d03'
p=pefile.PE(data=raw,fast_load=True);base=p.OPTIONAL_HEADER.ImageBase;u=Uc(UC_ARCH_X86,UC_MODE_64);u.mem_map(base,(p.OPTIONAL_HEADER.SizeOfImage+4095)&~4095);u.mem_write(base,p.get_memory_mapped_image());u.mem_map(0x30000000,0x40000);u.mem_map(0x40000000,0x400000);sp=0x30020008;heap=0x40000000
lib=ctypes.CDLL('/usr/lib/libSystem.B.dylib')
for n,k in [('ceilf',1),('roundf',1),('cosf',1),('sinf',1),('fmodf',2)]:
 f=getattr(lib,n);f.argtypes=[ctypes.c_float]*k;f.restype=ctypes.c_float
bits=lambda x:struct.unpack('<I',struct.pack('<f',x))[0];num=lambda x:struct.unpack('<f',struct.pack('<I',x&0xffffffff))[0]
proposals=[];rows=[];bricks=[];recent=[];status='running';calls={}
case=int(sys.argv[1]) if len(sys.argv)>1 else 0
fixtures=[(2.5,0.,0.,0.,4.,0,0),(4.2,0.37,3.,-2.,8.,0,0),(1.,0.,0.,0.,0.02,0,0),(2.5,0.,0.,0.,4.,1,0),(2.5,0.,0.,0.,4.,0,2)]
radius,rotation,cx,cy,elevation,removed,style=fixtures[case]
def ret(c):
 r=c.reg_read(UC_X86_REG_RSP);c.reg_write(UC_X86_REG_RIP,struct.unpack('<Q',c.mem_read(r,8))[0]);c.reg_write(UC_X86_REG_RSP,r+8)
def hook(c,pc,size,_):
 global heap,status
 recent.append(pc);del recent[:-30]
 if pc==0x30000000:status='returned';c.emu_stop()
 elif pc==0x141246fb0:c.reg_write(UC_X86_REG_XMM0,bits(0.));calls['terrain_0_bridge']=calls.get('terrain_0_bridge',0)+1;ret(c)
 elif pc==0x141250720:c.reg_write(UC_X86_REG_RAX,1);c.reg_write(UC_X86_REG_XMM0,bits(0.));calls['start_y_0_bridge']=calls.get('start_y_0_bridge',0)+1;ret(c)
 elif pc==0x141fe3240:
  vec=c.reg_read(UC_X86_REG_R8);cap,ptr,n=struct.unpack('<QQQ',c.mem_read(vec,24));proposals.extend([list(struct.unpack('<16I',c.mem_read(ptr+i*64,64))) for i in range(n)]);calls['brick_consumer_executed']=1
 elif pc==0x141fe2150:
  ptr=c.reg_read(UC_X86_REG_R8);n=c.reg_read(UC_X86_REG_R9);proposals.extend([list(struct.unpack('<16I',c.mem_read(ptr+i*64,64))) for i in range(n)]);calls['wood_consumer_executed']=1
 elif pc==0x141fe3272:c.reg_write(UC_X86_REG_RDI,0x3000e000);c.reg_write(UC_X86_REG_RIP,0x141fe32e2);calls['brick_lookup_bridge']=1
 elif pc==0x141fe21d7:c.reg_write(UC_X86_REG_RAX,0x3000e000);c.reg_write(UC_X86_REG_RIP,0x141fe2237);calls['plank_lookup_bridge']=1
 elif pc==0x141fe2a7f:c.reg_write(UC_X86_REG_RDI,0x3000e000);c.reg_write(UC_X86_REG_RIP,0x141fe2af8);calls['brick_lookup_bridge']=1
 elif pc==0x14120c120:bricks.append(list(struct.unpack('<24I',c.mem_read(c.reg_read(UC_X86_REG_RDX),96))));c.reg_write(UC_X86_REG_RAX,0);ret(c)
 elif pc==0x14120c430:rows.append(list(struct.unpack('<16I',c.mem_read(c.reg_read(UC_X86_REG_RDX),64))));c.reg_write(UC_X86_REG_RAX,0);ret(c)
 elif pc in (0x140d65810,0x140613c20,0x140c81c70,0x140c81e10,0x1428cd210,0x1428cdb90):ret(c)
 elif pc in (0x140d657b0,0x140613c10):
  size=c.reg_read(UC_X86_REG_RCX);align=c.reg_read(UC_X86_REG_RDX);heap=(heap+align-1)&-align;c.reg_write(UC_X86_REG_RAX,heap);heap+=size;ret(c)
 elif pc==0x140d65850:
  old=c.reg_read(UC_X86_REG_RCX);oldsize=c.reg_read(UC_X86_REG_RDX);align=c.reg_read(UC_X86_REG_R8);size=c.reg_read(UC_X86_REG_R9);heap=(heap+align-1)&-align;c.mem_write(heap,bytes(c.mem_read(old,min(size,oldsize))));c.reg_write(UC_X86_REG_RAX,heap);heap+=size;ret(c)
 elif pc in (0x142923c00,0x142923c10):
  dst=c.reg_read(UC_X86_REG_RCX);src=c.reg_read(UC_X86_REG_RDX);size=c.reg_read(UC_X86_REG_R8);c.mem_write(dst,bytes(c.mem_read(src,size)));c.reg_write(UC_X86_REG_RAX,dst);ret(c)
 elif pc in (0x142923e60,0x142923fa0,0x142923e80,0x142923fd0,0x142923f10):
  n={0x142923e60:'ceilf',0x142923fa0:'roundf',0x142923e80:'cosf',0x142923fd0:'sinf',0x142923f10:'fmodf'}[pc];args=[num(c.reg_read(UC_X86_REG_XMM0))];
  if n=='fmodf':args.append(num(c.reg_read(UC_X86_REG_XMM1)))
  c.reg_write(UC_X86_REG_XMM0,bits(getattr(lib,n)(*args)));calls[n]=calls.get(n,0)+1;ret(c)
u.hook_add(UC_HOOK_CODE,hook)
u.mem_write(0x30002000+0x140,struct.pack('<Q',0x3000f000));u.mem_write(0x30002000+0x150,struct.pack('<Q',0x3000f000));u.mem_write(0x3000e000+0x28,struct.pack('<Q',16))
u.mem_write(0x30002000+0x280,struct.pack('<f',elevation));u.mem_write(0x30002000+0x298,bytes([removed,style]));u.mem_write(0x30003000,struct.pack('<4f',cx,cy,radius,rotation));u.mem_write(0x30004000,struct.pack('<QQQ',0,4,0));u.mem_write(sp,struct.pack('<Q',0x30000000))
for off,val in [(0x28,4),(0x30,0),(0x38,0x30005000),(0x40,0x30006000),(0x48,0x30007000),(0x50,0x30008000),(0x58,0x30009000),(0x60,0x30004000)]:u.mem_write(sp+off,struct.pack('<Q',val))
for reg,val in [(UC_X86_REG_RSP,sp),(UC_X86_REG_RCX,123),(UC_X86_REG_RDX,0x30002000),(UC_X86_REG_R8,0x30003000),(UC_X86_REG_R9,0x3000a000),(UC_X86_REG_MXCSR,0x1f80)]:u.reg_write(reg,val)
try:u.emu_start(0x14124ba30,0x30000001,count=10000000)
except Exception as e:status='fault:'+str(e)+'@'+hex(u.reg_read(UC_X86_REG_RIP))
cap,ptr,n=struct.unpack('<QQQ',u.mem_read(0x30004000,24));bounds=[list(struct.unpack('<8I',u.mem_read(ptr+i*32,32))) for i in range(n)] if n<1000 else []
d={'status':status,'input':{'radius':radius,'center':[cx,cy],'rotation':rotation,'elevation':elevation,'removed':removed,'style':style,'wall_id':123,'other_walls':[]},'bridges':calls,'proposals':proposals,'bounds':bounds,'planks':rows,'bricks':bricks,'recent':[hex(x) for x in recent],'scope':'Native pillar entry and material consumer execute; fixed terrain/start_y, SSBO lookup and writer bridges; no full terrain/collision oracle.'}
Path(f'/private/tmp/m06-circle-pillar-full-result-{case}.json').write_text(json.dumps(d,indent=2)+'\n');print(status,'proposals',len(proposals),'bounds',len(bounds),calls,'last',list(map(hex,recent[-10:])))
if status!='returned':raise SystemExit(1)
