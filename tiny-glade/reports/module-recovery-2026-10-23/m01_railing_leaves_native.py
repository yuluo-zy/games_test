"""执行完整原 dispatcher 的StartPlace优先路径，UI库叶只桥接注册，不复制事件选择算法。"""
from pathlib import Path
import pefile,struct,hashlib,json,itertools
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

records=[]
def hook(uc,a,size,_):
 if a in (0x14128cdd0,0x14128d0e0):
  n=96 if a==0x14128cdd0 else 64
  records.append(bytes(u.mem_read(r(UC_X86_REG_RDX),n)).hex());ret(0)
u.hook_add(UC_HOOK_CODE,hook)
def setup(args):
 global records
 records=[];u.mem_write(D,bytes(0x20000));u.mem_write(S,bytes([0xa5])*0x100000)
 sp=S+0xf0008;q(sp,STOP)
 for reg,value in zip([UC_X86_REG_RCX,UC_X86_REG_RDX,UC_X86_REG_R8,UC_X86_REG_R9],args):u.reg_write(reg,value)
 for i,value in enumerate(args[4:]):q(sp+0x28+i*8,value)
 u.reg_write(UC_X86_REG_RSP,sp)
 return sp
def floats(a,seq):u.mem_write(a,struct.pack('<'+'f'*len(seq),*seq))
rows=[]
for seed,slope,n,auxkind in itertools.product([0,1,19,0xdeadbeef],[0.,1.5999999,1.6,2.],range(5),[0,1]):
 points=[(i*.7-1.25,1.2+i*.31,(-1)**i*.35+i*.17)for i in range(n)]
 aux=[0. if auxkind==0 else .15+i*.31 for i in range(n)]
 sp=setup([D+0x100,42,D+0x200,0,D+0x1000,n,D+0x2000,n]);q(D+0x100,seed)
 floats(D+0x1000,[v for p in points for v in p]);floats(D+0x2000,aux)
 u.reg_write(UC_X86_REG_XMM3,struct.unpack('<I',struct.pack('<f',slope))[0])
 u.emu_start(0x1407fdf90,STOP,count=1000000);assert r(UC_X86_REG_RIP)==STOP
 rows.append(dict(kind='wood',seed=seed,slope=slope,points=points,aux=aux,records=records,rng_after=uq(D+0x100)))
for n,kind,flags,direction in itertools.product([1,2,4],range(4),range(4),[(1.,0.),(-1.,0.),(.7,-.3),(0.,0.)]):
 points=[(i*.7-1.25,1.2+i*.31,(-1)**i*.35+i*.17)for i in range(n)]
 widths=[.1,.9,1.4,2.1][:n];tangents=[(.7+i*.2,-.3+i*.17)for i in range(n)]
 sp=setup([0,100003,kind,D+0x200,D+0x1000,n,D+0x2000,n,D+0x3000,n,flags&1,(flags>>1)&1,*[struct.unpack('<I',struct.pack('<f',x))[0]for x in direction]])
 floats(D+0x1000,widths);floats(D+0x2000,[v for p in tangents for v in p]);floats(D+0x3000,[v for p in points for v in p])
 u.emu_start(0x1407febb0,STOP,count=1000000);assert r(UC_X86_REG_RIP)==STOP
 rows.append(dict(kind='corner',style=kind,flags=flags,direction=direction,points=points,widths=widths,tangents=tangents,records=records))
Path(__file__).with_name('m01-railing-leaves-native.json').write_text(json.dumps(dict(binary_sha256=sha,bridges=['Sparse writer push: capture complete96/64B, return original success bool0'],rows=rows),indent=2)+'\n')
print(json.dumps(dict(cases=len(rows),records=sum(len(x['records'])for x in rows))))
