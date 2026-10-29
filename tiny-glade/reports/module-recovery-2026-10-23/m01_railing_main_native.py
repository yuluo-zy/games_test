"""执行完整原 dispatcher 的StartPlace优先路径，UI库叶只桥接注册，不复制事件选择算法。"""
from pathlib import Path
import pefile,struct,hashlib,json,itertools,math
from unicorn import Uc,UC_ARCH_X86,UC_MODE_64,UC_HOOK_CODE
from unicorn.x86_const import *
root=Path(__file__).resolve().parents[3];raw=(root/'B版本内容/tiny-glade.rne').read_bytes();sha=hashlib.sha256(raw).hexdigest();assert sha=='f507a6cea25c5199447b88ba746a5293e64a279f73797ca74c1ce7f4ff2b7d03'
pe=pefile.PE(data=raw,fast_load=True);base=pe.OPTIONAL_HEADER.ImageBase;blob=pe.get_memory_mapped_image()
u=Uc(UC_ARCH_X86,UC_MODE_64);u.mem_map(base,(len(blob)+4095)&~4095);u.mem_write(base,blob)
D,S,STOP=0x20000000,0x30000000,0x200ff000;u.mem_map(D,0x100000);u.mem_map(S,0x100000)
q=lambda a,v:u.mem_write(a,struct.pack('<Q',v))
w=lambda a,v:u.mem_write(a,struct.pack('<I',v))
uq=lambda a:struct.unpack('<Q',u.mem_read(a,8))[0]
uw=lambda a:struct.unpack('<I',u.mem_read(a,4))[0]
r=lambda reg:u.reg_read(reg)
def ret(v=0):
 sp=r(UC_X86_REG_RSP);u.reg_write(UC_X86_REG_RIP,uq(sp));u.reg_write(UC_X86_REG_RSP,sp+8);u.reg_write(UC_X86_REG_RAX,v)


H=0x40000000;u.mem_map(H,0x1000000)
records=[];live={};cursor=H
class NativePanic(Exception):pass
def hook(uc,a,size,_):
 global cursor
 rcx,rdx,r8,r9=[r(x)for x in [UC_X86_REG_RCX,UC_X86_REG_RDX,UC_X86_REG_R8,UC_X86_REG_R9]]
 if a in (0x14128cdd0,0x14128d0e0):
  n=96 if a==0x14128cdd0 else 64
  records.append(('b'if n==96 else'w',bytes(u.mem_read(rdx,n)).hex()));ret(0)
 elif a in (0x140613c10,0x140613c40):
  size=rcx;align=rdx;at=(cursor+max(align,16)-1)&~(max(align,16)-1);cursor=at+size;assert cursor<H+0x1000000
  live[at]=size
  u.mem_write(at,bytes(size)if a==0x140613c40 else bytes([0xa5])*size);ret(at)
 elif a==0x140613c20:
  assert rcx in live,(hex(a),hex(rcx),live);del live[rcx];ret(0)
 elif a==0x140613c30:
  assert rcx in live;at=(cursor+max(r8,16)-1)&~(max(r8,16)-1);cursor=at+r9
  u.mem_write(at,bytes(u.mem_read(rcx,min(rdx,r9)))+bytes(max(0,r9-rdx)));del live[rcx];live[at]=r9;ret(at)
 elif a in (0x142923c00,0x142923c10):
  u.mem_write(rcx,bytes(u.mem_read(rdx,r8)));ret(rcx)
 elif a==0x142923c20:
  u.mem_write(rcx,bytes([rdx&255])*r8);ret(rcx)
 elif a in (0x142923e60,0x142923ee0,0x142924060):
  x=struct.unpack("<f",struct.pack("<I",r(UC_X86_REG_XMM0)&0xffffffff))[0]
  y=math.ceil(x) if a==0x142923e60 else math.floor(x) if a==0x142923ee0 else math.trunc(x)
  ret();u.reg_write(UC_X86_REG_XMM0,struct.unpack("<I",struct.pack("<f",float(y)))[0])
 elif a==0x142923fa0:
  x=struct.unpack("<f",struct.pack("<I",r(UC_X86_REG_XMM0)&0xffffffff))[0];y=math.copysign(math.floor(abs(x)+.5),x)
  ret();u.reg_write(UC_X86_REG_XMM0,struct.unpack("<I",struct.pack("<f",y))[0])
 elif a in (0x1428d9518,0x1428d9596,0x1428d9310,0x1428d8fe3):raise NativePanic(hex(a))
u.hook_add(UC_HOOK_CODE,hook)
def floats(a,seq):u.mem_write(a,struct.pack('<'+'f'*len(seq),*seq))
def vec(a,ptr,values,stride):
 q(a,len(values));q(a+8,ptr);q(a+16,len(values));floats(ptr,[x for v in values for x in v]if stride>1 else values)
def run(seed,style,profile,rail,n,flags=0,suppress=0,geometry=0,width_mode=0,aux_mode=0,heights=(.07,.2)):
 global records,live,cursor
 records=[];live={};cursor=H
 u.mem_write(D,bytes(0x20000));u.mem_write(S,bytes([0xa5])*0x100000)
 points=[(i*.7-1.25,1.2+i*.31,(-1)**i*.35+i*.17)for i in range(n)]
 if geometry==1:points=[(i*.7-1.25,1.2+i*2.,(-1)**i*.35+i*.17)for i in range(n)]
 elif geometry==2:points=[(i*.7-1.25,1.2+i*10.,(-1)**i*.35+i*.17)for i in range(n)]
 elif geometry==3:points=[(0.,0.,0.),(.2,1.,1.),(1.,1.3,.9),(1.1,0.,0.)][:n]
 elif geometry==4:points=[(i*.7-1.25,1.5,(-1)**i*.35+i*.17)for i in range(n)]
 widths=[.32+i*.03 if width_mode==1 else 1.6 if width_mode==2 else 1.2+i*.2 for i in range(n)]
 tangents=[(.7,-.3+i*.17)for i in range(n)]
 aux=[0. if aux_mode==1 else .05 if aux_mode==2 else .15+i*.11 for i in range(n)]
 vec(D+0x300,D+0x2000,widths,1);vec(D+0x400,D+0x3000,tangents,2);vec(D+0x500,D+0x4000,points,3);floats(D+0x5000,aux);q(D+0x600,seed)
 w(D+0x800,int(heights[0]is not None));floats(D+0x804,[heights[0]or 0.]);w(D+0x808,int(heights[1]is not None));floats(D+0x80c,[heights[1]or 0.])
 args=[D+0x200,D+0x300,D+0x400,D+0x500,D+0x600,100003,42,D+0x700,flags,style,profile,rail,suppress&1,(suppress>>1)&1,D+0x800,D+0x5000,n]
 sp=S+0xf0008;q(sp,STOP)
 for reg,value in zip([UC_X86_REG_RCX,UC_X86_REG_RDX,UC_X86_REG_R8,UC_X86_REG_R9],args):u.reg_write(reg,value)
 for i,value in enumerate(args[4:]):q(sp+0x28+i*8,value)
 u.reg_write(UC_X86_REG_RSP,sp)
 try:u.emu_start(0x1407f98e0,STOP,count=2000000)
 except Exception as e:raise RuntimeError((seed,style,profile,rail,n,hex(r(UC_X86_REG_RIP)),e))
 assert r(UC_X86_REG_RIP)==STOP;assert not live,live
 return dict(seed=seed,style=style,profile=profile,rail=rail,flags=flags,suppress=suppress,points=points,widths=widths,tangents=tangents,aux=aux,endpoint_heights=heights,geometry=geometry,width_mode=width_mode,aux_mode=aux_mode,records=records,rng_after=uq(D+0x600))
rows=[]
for seed,style,profile,rail,n in itertools.product([0,19],range(4),range(1,5),range(4),[2,4]):rows.append(run(seed,style,profile,rail,n))
for style,profile,flags,suppress in itertools.product(range(4),[1,2,4],[0,1,0x100,0x101],[0,1,2,3]):
 rows.append(run(0xdeadbeef,style,profile,3,4,flags,suppress))
for geometry,style,profile,wm,am in itertools.product(range(1,5),range(4),[1,2,4],[0,1],[0,1]):
 rows.append(run(19,style,profile,255,4,0x101,0,geometry,wm,am,heights=(None,None)))
Path(__file__).with_name('m01-railing-main-native.json').write_text(json.dumps(dict(binary_sha256=sha,bridges=['Sparse push capture96/64B','Rust allocator/deallocator/reallocator ownership+memory','CRT memcpy bytes'],rows=rows),indent=2)+'\n')
print(json.dumps(dict(cases=len(rows),records=sum(len(x['records'])for x in rows))))
