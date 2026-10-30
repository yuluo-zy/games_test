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
def run(seed,style,profile,rail,n,flags=0,suppress=0):
 global records,live,cursor
 records=[];live={};cursor=H
 u.mem_write(D,bytes(0x20000));u.mem_write(S,bytes([0xa5])*0x100000)
 points=[(i*.7-1.25,1.2+i*.31,(-1)**i*.35+i*.17)for i in range(n)]
 widths=[1.2+i*.2 for i in range(n)];tangents=[(.7,-.3+i*.17)for i in range(n)];aux=[.15+i*.11 for i in range(n)]
 vec(D+0x300,D+0x2000,widths,1);vec(D+0x400,D+0x3000,tangents,2);vec(D+0x500,D+0x4000,points,3);floats(D+0x5000,aux);q(D+0x600,seed)
 w(D+0x800,1);floats(D+0x804,[.07]);w(D+0x808,1);floats(D+0x80c,[.2])
 args=[D+0x200,D+0x300,D+0x400,D+0x500,D+0x600,100003,42,D+0x700,flags,style,profile,rail,suppress&1,(suppress>>1)&1,D+0x800,D+0x5000,n]
 sp=S+0xf0008;q(sp,STOP)
 for reg,value in zip([UC_X86_REG_RCX,UC_X86_REG_RDX,UC_X86_REG_R8,UC_X86_REG_R9],args):u.reg_write(reg,value)
 for i,value in enumerate(args[4:]):q(sp+0x28+i*8,value)
 u.reg_write(UC_X86_REG_RSP,sp)
 try:u.emu_start(0x1407f98e0,STOP,count=2000000)
 except Exception as e:raise RuntimeError((seed,style,profile,rail,n,hex(r(UC_X86_REG_RIP)),e))
 assert r(UC_X86_REG_RIP)==STOP;assert not live,live
 return dict(seed=seed,style=style,profile=profile,rail=rail,flags=flags,suppress=suppress,points=points,widths=widths,tangents=tangents,aux=aux,endpoint_heights=[.07,.2],records=records,rng_after=uq(D+0x600))

rays=[]
def tips_hook(uc,a,size,_):
 rcx,rdx,r8,r9=[r(x)for x in [UC_X86_REG_RCX,UC_X86_REG_RDX,UC_X86_REG_R8,UC_X86_REG_R9]]
 if a==0x14125d6a0:ret(D+0x11000 if align_tag>=0 else 0)
 elif a==0x1408fe8b0:ret(D+0x11500)
 elif a==0x14125de50:ret(D+0x12000 if degree>=0 else 0)
 elif a==0x140a1c510:ret(address_tag);u.reg_write(UC_X86_REG_RDX,55)
 elif a==0x1408ccc50:
  sp=r(UC_X86_REG_RSP);ray=bytes(u.mem_read(r8,24)).hex();maxbits=r(UC_X86_REG_XMM3)&0xffffffff
  rays.append(dict(ray=ray,max_toi_bits=maxbits,mask=uw(sp+0x28),some=uq(sp+0x30),wall=uq(sp+0x38)))
  assert rays[-1]['mask']==1 and rays[-1]['some']==1 and rays[-1]['wall']==55
  hit=(len(rays)-1)%3==hit_index
  u.mem_write(rcx,bytes([0 if hit else 17])+bytes(27));ret(rcx)
u.hook_add(UC_HOOK_CODE,tips_hook)
def run_tip(seed,style,n,flags,deg=1,tag=1,addr=0,hit=-1):
 global records,live,cursor,rays,degree,align_tag,address_tag,hit_index
 records=[];live={};cursor=H;rays=[];degree=deg;align_tag=tag;address_tag=addr;hit_index=hit
 u.mem_write(D,bytes(0x20000));u.mem_write(S,bytes([0xa5])*0x100000)
 points=[(i*.7-1.25,1.2+i*.31,(-1)**i*.35+i*.17)for i in range(n)]
 widths=[1.2+i*.2 for i in range(n)];tangents=[(.7,-.3+i*.17)for i in range(n)]
 floats(D+0x2000,widths);floats(D+0x3000,[v for x in tangents for v in x]);floats(D+0x4000,[v for x in points for v in x]);q(D+0x600,seed)
 q(D+0x1000,31|(32<<32));q(D+0x12010,max(degree,0));u.mem_write(D+0x11000,bytes([max(tag,0)]));
 args=[D+0x100,D+0x180,D+0x1000,D+0x600,100003,42,style,D+0x200,D+0x2000,n,D+0x3000,n,D+0x4000,n,D+0x700,D+0x800,D+0x900,flags]
 sp=S+0xf0008;q(sp,STOP)
 for reg,value in zip([UC_X86_REG_RCX,UC_X86_REG_RDX,UC_X86_REG_R8,UC_X86_REG_R9],args):u.reg_write(reg,value)
 for i,value in enumerate(args[4:]):q(sp+0x28+i*8,value)
 u.reg_write(UC_X86_REG_RSP,sp)
 try:u.emu_start(0x1407fcc80,STOP,count=2000000)
 except Exception as e:raise RuntimeError((seed,style,n,flags,deg,tag,addr,hit,hex(r(UC_X86_REG_RIP)),e))
 assert r(UC_X86_REG_RIP)==STOP;assert not live,live
 return dict(seed=seed,style=style,n=n,flags=flags,degree=degree,align_tag=tag,address_tag=addr,hit_index=hit,points=points,widths=widths,tangents=tangents,rays=rays,records=records,rng_after=uq(D+0x600))
rows=[]
for seed,style,n,flags,hit in itertools.product([0,19],range(4),[2,4],[0,1,0x100,0x101],[-1,0,1,2]):rows.append(run_tip(seed,style,n,flags,hit=hit))
for deg,tag,addr in itertools.product([-1,0,1,2],[-1,0,1,2],range(4)):rows.append(run_tip(19,2,2,0x101,deg,tag,addr))
Path(__file__).with_name('m01-railing-tips-native.json').write_text(json.dumps(dict(binary_sha256=sha,bridges=['full native geometry/random/cap entry; Sparse push and heap/CRT from main probe','IndexMap/graph/storage read-only dependency results','Rapier result only: retain original ray setup and ordered short circuit; actual physics validated separately'],rows=rows),indent=2)+'\n')
print(json.dumps(dict(cases=len(rows),records=sum(len(x['records'])for x in rows),rays=sum(len(x['rays'])for x in rows))))
