import hashlib, json, struct, ctypes, os
from pathlib import Path
import pefile
from unicorn import Uc,UC_ARCH_X86,UC_MODE_64,UC_HOOK_CODE
from unicorn.x86_const import *
raw=Path('B版本内容/tiny-glade.rne').read_bytes();assert hashlib.sha256(raw).hexdigest()=='f507a6cea25c5199447b88ba746a5293e64a279f73797ca74c1ce7f4ff2b7d03'
p=pefile.PE(data=raw,fast_load=True);base=p.OPTIONAL_HEADER.ImageBase
u=Uc(UC_ARCH_X86,UC_MODE_64);u.mem_map(base,(p.OPTIONAL_HEADER.SizeOfImage+4095)&~4095);u.mem_write(base,p.get_memory_mapped_image());u.mem_map(0x30000000,0x20000);u.mem_map(0x40000000,0x100000);sp=0x30010008;heap=0x40000000
lib=ctypes.CDLL('/usr/lib/libSystem.B.dylib')
for n in ('ceilf','cosf','sinf','fmodf','roundf'):
 f=getattr(lib,n);f.argtypes=[ctypes.c_float]*(2 if n=='fmodf' else 1);f.restype=ctypes.c_float
bits=lambda x:struct.unpack('<I',struct.pack('<f',x))[0]
num=lambda x:struct.unpack('<f',struct.pack('<I',x&0xffffffff))[0]
rows=[];intermediates=[];inputs=[];rays=[];recent=[];status='unknown'
def ret(c):
 r=c.reg_read(UC_X86_REG_RSP);c.reg_write(UC_X86_REG_RIP,struct.unpack('<Q',c.mem_read(r,8))[0]);c.reg_write(UC_X86_REG_RSP,r+8)
def hook(c,pc,size,_):
 global heap,status
 recent.append(pc)
 if len(recent)>160:recent.pop(0)
 if pc==0x14125519f:
  a=c.reg_read(UC_X86_REG_XMM14);b=c.reg_read(UC_X86_REG_XMM10);inputs.append([a&0xffffffff,(a>>32)&0xffffffff,c.reg_read(UC_X86_REG_XMM13)&0xffffffff,b&0xffffffff,(b>>32)&0xffffffff,c.reg_read(UC_X86_REG_XMM15)&0xffffffff])
 elif pc==0x14089d560:intermediates.append(list(struct.unpack('<16I',c.mem_read(c.reg_read(UC_X86_REG_RDX),64))))
 elif pc==0x1408ccc50 and os.getenv('M06_HIT'):
  dst=c.reg_read(UC_X86_REG_RCX);rays.append(list(struct.unpack('<6I',c.mem_read(c.reg_read(UC_X86_REG_R8),24))));c.mem_write(dst,struct.pack('<3Q4f',6,0,0,0.,0.,0.,1.));ret(c)
 elif pc==0x141246fb0 and os.getenv('M06_HIT'):c.reg_write(UC_X86_REG_XMM0,bits(0.));ret(c)
 elif pc==0x141254280:c.reg_write(UC_X86_REG_RBX,0x30005000);c.reg_write(UC_X86_REG_RIP,0x1412542ef)
 elif pc==0x140c16a00:c.reg_write(UC_X86_REG_RAX,0);ret(c)
 elif pc==0x1400d9b20:c.reg_write(UC_X86_REG_RAX,0x30001000);ret(c)
 elif pc==0x140a91ef0:c.reg_write(UC_X86_REG_RAX,0x30007000);ret(c)
 elif pc in (0x140c81c70,0x140c81e10,0x140d65810):ret(c)
 elif pc==0x140d657b0:
  size=c.reg_read(UC_X86_REG_RCX);align=c.reg_read(UC_X86_REG_RDX);heap=(heap+align-1)&-align;c.reg_write(UC_X86_REG_RAX,heap);heap+=size;ret(c)
 elif pc==0x140d65850:
  old=c.reg_read(UC_X86_REG_RCX);oldsize=c.reg_read(UC_X86_REG_RDX);align=c.reg_read(UC_X86_REG_R8);size=c.reg_read(UC_X86_REG_R9);heap=(heap+align-1)&-align;c.mem_write(heap,bytes(c.mem_read(old,min(size,oldsize))));c.reg_write(UC_X86_REG_RAX,heap);heap+=size;ret(c)
 elif pc==0x142923c20:
  dst=c.reg_read(UC_X86_REG_RCX);value=c.reg_read(UC_X86_REG_RDX)&255;size=c.reg_read(UC_X86_REG_R8);c.mem_write(dst,bytes([value])*size);c.reg_write(UC_X86_REG_RAX,dst);ret(c)
 elif pc==0x142923c00:
  dst=c.reg_read(UC_X86_REG_RCX);src=c.reg_read(UC_X86_REG_RDX);size=c.reg_read(UC_X86_REG_R8);c.mem_write(dst,bytes(c.mem_read(src,size)));c.reg_write(UC_X86_REG_RAX,dst);ret(c)
 elif pc in (0x142923e60,0x142923e80,0x142923fd0,0x142923fa0):
  func={0x142923e60:lib.ceilf,0x142923e80:lib.cosf,0x142923fd0:lib.sinf,0x142923fa0:lib.roundf}[pc];c.reg_write(UC_X86_REG_XMM0,bits(func(num(c.reg_read(UC_X86_REG_XMM0)))));ret(c)
 elif pc==0x142923f10:c.reg_write(UC_X86_REG_XMM0,bits(lib.fmodf(num(c.reg_read(UC_X86_REG_XMM0)),num(c.reg_read(UC_X86_REG_XMM1)))));ret(c)
 elif pc==0x14120c430:
  rows.append(list(struct.unpack('<16I',c.mem_read(c.reg_read(UC_X86_REG_RDX),64))));c.reg_write(UC_X86_REG_RAX,0);ret(c)
 elif pc==0x30000000:status='returned';c.emu_stop()
u.hook_add(UC_HOOK_CODE,hook)
fixture=json.loads(Path('tiny-glade/reports/module-recovery-2026-10-23/' + ('m06-finder-rectangle-output-oracle.json' if os.getenv('M06_RECT') else 'm06-finder-intersection-output-oracle.json')).read_text());fixture=fixture['segments'][0] if os.getenv('M06_RECT') else fixture
points=fixture['points_bits']; us=fixture['points_u_bits']; n=len(us)
u.mem_write(0x3000b000,struct.pack('<'+'I'*len(points),*points));u.mem_write(0x3000c000,struct.pack('<'+'I'*n,*us))
record=struct.pack('<6QI4xQII?7x',n,0x3000b000,n,n,0x3000c000,n,fixture['length_bits'],2,*fixture['interval_bits'],fixture['flag']);u.mem_write(0x30009000,record)
u.mem_write(0x30007000+0x258,struct.pack('<I6f',*( (1,1.,0.,0.,0.,2.,2.) if os.getenv('M06_RECT') else (0,0.,0.,2.,0.,0.,0.) )));u.mem_write(0x30007000+0x280,struct.pack('<f',1.));u.mem_write(0x30001000,struct.pack('<2Q',1,0));u.mem_write(0x30002000+0x150,struct.pack('<Q',0x30004000));u.mem_write(0x30002000+0x140,struct.pack('<Q',0x30004000));u.mem_write(0x30002000+0x280,struct.pack('<2f',4.,14.));u.mem_write(0x30002000+0x299,bytes([2 if os.getenv('M06_WOOD') else 0]));u.mem_write(0x30002000+0x258,struct.pack('<I6f',1,1.,0.,0.,0.,5.,5.));u.mem_write(0x30003000,struct.pack('<6f',1.,0.,0.,0.,5.,5.));u.mem_write(0x30005000+0x28,struct.pack('<Q',16));u.mem_write(sp,struct.pack('<Q',0x30000000))
for off,val in [(0x28,0x30009000),(0x30,1),(0x38,0x3000d000),(0x40,0x3000e000),(0x48,0x30006000),(0x50,0x3000a000),(0x58,0x3000f000)]:u.mem_write(sp+off,struct.pack('<Q',val))
for reg,val in [(UC_X86_REG_RSP,sp),(UC_X86_REG_RCX,123),(UC_X86_REG_RDX,0x30002000),(UC_X86_REG_R8,0x30003000),(UC_X86_REG_R9,0x30008000),(UC_X86_REG_MXCSR,0x1f80)]:u.reg_write(reg,val)
try:u.emu_start(0x141253fc0,0x30000001,count=3000000)
except Exception as e:status='fault:'+str(e)+'@'+hex(u.reg_read(UC_X86_REG_RIP))
print(status,'rows',len(rows),'count',struct.unpack('<Q',u.mem_read(0x3000f000,8))[0],'last',[hex(x) for x in recent])
# 非空输出与正常返回需独立确认，不用失败结果做断言
if status=='returned': Path('tiny-glade/reports/module-recovery-2026-10-23/' + ('m06-large-bracket-hit-native-output.json' if os.getenv('M06_HIT') else 'm06-large-bracket-native-output.json')).write_text(json.dumps(rows,indent=2)+'\n')


if status=='returned' and os.getenv('M06_HIT'):Path('tiny-glade/reports/module-recovery-2026-10-23/m06-large-bracket-hit-affine-native.json').write_text(json.dumps(intermediates,indent=2)+'\n')
print('affines',len(intermediates))

if status=='returned' and os.getenv('M06_HIT'):Path('tiny-glade/reports/module-recovery-2026-10-23/m06-large-bracket-hit-endpoints-native.json').write_text(json.dumps(inputs,indent=2)+'\n')
print('inputs',len(inputs))

if status=='returned' and os.getenv('M06_HIT'):Path('tiny-glade/reports/module-recovery-2026-10-23/m06-large-bracket-hit-rays-native.json').write_text(json.dumps(rays,indent=2)+'\n')
print('rays',len(rays))
