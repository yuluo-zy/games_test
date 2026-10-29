"""原1407d1f80完整执行；仅WallHoles.add边界捕获，hashbrown表由显式16槽夹具提供。"""
from pathlib import Path
import hashlib,json,struct,pefile
from unicorn import *
from unicorn.x86_const import *
r=Path('/Users/yuluo/Desktop/game');raw=(r/'B版本内容/tiny-glade.rne').read_bytes();sha=hashlib.sha256(raw).hexdigest();assert sha=='f507a6cea25c5199447b88ba746a5293e64a279f73797ca74c1ce7f4ff2b7d03'
p=pefile.PE(data=raw,fast_load=True);im=p.get_memory_mapped_image();u=Uc(UC_ARCH_X86,UC_MODE_64);u.mem_map(p.OPTIONAL_HEADER.ImageBase,(len(im)+4095)&~4095);u.mem_write(p.OPTIONAL_HEADER.ImageBase,im);u.mem_map(0x100000,0x200000);u.mem_map(0x400000,0x400000)
def q(a,x):u.mem_write(a,struct.pack('<Q',x))
def ret():
 sp=u.reg_read(UC_X86_REG_RSP);ip=struct.unpack('<Q',u.mem_read(sp,8))[0];u.reg_write(UC_X86_REG_RSP,sp+8);u.reg_write(UC_X86_REG_RIP,ip)
out=[]
def hook(uc,ip,size,data):
 if ip==0x140f86500:
  assert u.reg_read(UC_X86_REG_RCX)==0x405000
  out.append([u.reg_read(UC_X86_REG_RDX),*struct.unpack('<4I',u.mem_read(u.reg_read(UC_X86_REG_R8),16)),u.reg_read(UC_X86_REG_R9)&255]);ret()
u.hook_add(UC_HOOK_CODE,hook);rows=[]
for mask in [0,1,2,0x8000,0x8001,0xa481,0xffff]:
 for variant in range(4):
  ctrl=0x410000;u.mem_write(ctrl,bytes(0 if mask>>i&1 else 255 for i in range(16)))
  q(0x401000,0x402000);q(0x402000,ctrl);q(0x402008,15);q(0x402018,mask.bit_count());q(0x403000,0x405000);q(0x403010,0x406000);u.mem_write(0x40301c,struct.pack('<I',42))
  expected=[];inputs=[]
  for i in range(16):
   if not mask>>i&1:continue
   count=(i+variant)%4;ptr=0x420000+i*0x200;cap=count+5
   u.mem_write(ctrl-i*0x110-0xd8,struct.pack('<3Q',cap,ptr,count));recs=[]
   for j in range(count):
    wall=42 if i%2 else 100+i
    words=[0x80000000 if j==0 else 0x3f800000+j,0x7fc01234 if variant==3 else 0x40000000+i,0x40800000+j,0x41200000]
    u.mem_write(ptr+j*24,struct.pack('<Q4I',wall,*words));expected.append([wall,*words,4]);recs.append([wall,*words])
   inputs.append([i,recs])
  sp=0x1ff008;q(sp,0x100000);u.reg_write(UC_X86_REG_RSP,sp);u.reg_write(UC_X86_REG_RCX,0x401000);u.reg_write(UC_X86_REG_RDX,0x403000);out=[]
  u.emu_start(0x1407d1f80,0x100000,count=1000000);assert u.reg_read(UC_X86_REG_RIP)==0x100000;assert out==expected
  rows.append(dict(mask=mask,variant=variant,inputs=inputs,output=out))
Path('/tmp/m01-stair-hole-cache-native.json').write_text(json.dumps(dict(original_sha256=sha,scope='full1407d1f80; only WallHoles.add capture bridge, explicit hashbrown fixture',rows=rows),indent=2));print('original cases',len(rows),'records',sum(len(x['output']) for x in rows))
