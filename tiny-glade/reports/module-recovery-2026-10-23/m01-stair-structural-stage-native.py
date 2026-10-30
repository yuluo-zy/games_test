"""原1407d20c0完整入口到ret。库/物理/图存储桥只记录输入，保主函数全部分支和数学。"""
from pathlib import Path
import hashlib,json,struct
import pefile
from unicorn import Uc,UC_ARCH_X86,UC_MODE_64,UC_HOOK_CODE
from unicorn.x86_const import *
p=Path(__file__).resolve();root=p.parents[3]
raw=(root/'B版本内容/tiny-glade.rne').read_bytes()
assert hashlib.sha256(raw).hexdigest()=='f507a6cea25c5199447b88ba746a5293e64a279f73797ca74c1ce7f4ff2b7d03'
pe=pefile.PE(data=raw,fast_load=True);base=pe.OPTIONAL_HEADER.ImageBase
u=Uc(UC_ARCH_X86,UC_MODE_64);u.mem_map(base,(pe.OPTIONAL_HEADER.SizeOfImage+4095)&~4095);u.mem_write(base,pe.get_memory_mapped_image());u.mem_map(0x200000,0x200000)
read=lambda a,n:bytes(u.mem_read(a,n))
q=lambda a:struct.unpack('<Q',read(a,8))[0]
words=lambda a,n:list(struct.unpack('<'+'I'*n,read(a,n*4)))
writeq=lambda a,*v:u.mem_write(a,struct.pack('<'+'Q'*len(v),*v))
reg=lambda r:u.reg_read(r)
def ret(value=None):
 if value is not None:u.reg_write(UC_X86_REG_RAX,value)
 sp=reg(UC_X86_REG_RSP);u.reg_write(UC_X86_REG_RIP,q(sp));u.reg_write(UC_X86_REG_RSP,sp+8)
def param(at,owner):writeq(at,owner,0,at+0x28,1<<32)
rows=[];cases=[]
for n in [2,3,5]:
 for kind in [0,1,2,3,4]:
  for removed in [False,True]:
   for bits in [0,(1<<n)-1,1,1<<(n-1)]:
    cases.append(dict(n=n,kind=kind,removed=removed,bits=bits,dirty=True,curve=True))
for dirty,curve in [(False,True),(True,False),(False,False)]:cases.append(dict(n=3,kind=0,removed=False,bits=0,dirty=dirty,curve=curve))
cases += [dict(n=3,kind=0,removed=False,bits=0,dirty=True,curve=True,width_len=2),dict(n=3,kind=0,removed=False,bits=0,dirty=True,curve=True,count=3),dict(n=3,kind=0,removed=False,bits=0,dirty=True,curve=True,segments=[]),dict(n=5,kind=0,removed=True,bits=2,dirty=True,curve=True,segments=[(0,2),(0,2)])]
for anchors in range(4):
 for y_mode in range(3):cases.append(dict(n=2,kind=3,removed=False,bits=0,dirty=True,curve=True,anchors=anchors,y_mode=y_mode))
for ci,case in enumerate(cases):
 u.mem_write(0x200000,bytes(0x200000));n=case['n'];stack=0x3f0008;stop=0x3ff000;request=0x210000;requests=0x211000;structural=0x212000;removedset=0x213000;stairs=0x214000;graph=0x215000;ctrl=0x216000
 positions=0x220000;widths=0x221000;directions=0x222000;contacts=0x223000;segments=0x224000
 for r in [UC_X86_REG_RAX,UC_X86_REG_RBX,UC_X86_REG_RCX,UC_X86_REG_RDX,UC_X86_REG_R8,UC_X86_REG_R9,UC_X86_REG_R10,UC_X86_REG_R11,UC_X86_REG_R12,UC_X86_REG_R13,UC_X86_REG_R14,UC_X86_REG_R15,UC_X86_REG_RSI,UC_X86_REG_RDI,UC_X86_REG_RBP]:u.reg_write(r,0)
 for i in range(16):u.reg_write(UC_X86_REG_XMM0+i,0)
 points=[(i*1.25,i*.375+.25,(-1.0 if i%2 else 1.0)*.5)for i in range(n)]
 if case.get('y_mode')==1:points=[(x,(n-1-i)*.375+.25,z)for i,(x,y,z)in enumerate(points)]
 elif case.get('y_mode')==2:points=[(x,.25,z)for x,y,z in points]
 tangent=[(1.0,i*.125)for i in range(n)];width=[1.0+i*.125 for i in range(n)]
 u.mem_write(positions,b''.join(struct.pack('<fff',*v)for v in points));u.mem_write(directions,b''.join(struct.pack('<ff',*v)for v in tangent));u.mem_write(widths,struct.pack('<'+'f'*n,*width));writeq(contacts,case['bits'])
 writeq(request+0x58,widths,case.get('width_len',n));writeq(request+0x70,directions,n)
 writeq(request+0x98,n if case['curve'] else 1<<63,positions,n);writeq(request+0x110,contacts,n<<3)
 u.mem_write(request+0xe8,b'\1');u.mem_write(request+0x108,b'\1');u.mem_write(request+0x128,struct.pack('<II',3,8));u.mem_write(request+0x148,bytes([case['dirty']]))
 for bit,offset in [(1,0xd0),(2,0xf0)]:
  if case.get('anchors',0)&bit:u.mem_write(request+offset,struct.pack('<I',2));writeq(request+offset+8,91)
 subs=case.get('segments',[(case['kind'],case.get('count',n-1))]);packed=b''.join(struct.pack('<BBH',k,0,c)for k,c in subs)
 if len(subs)<2:u.mem_write(request+0x130,packed);writeq(request+0x140,len(subs))
 else:u.mem_write(segments,packed);writeq(request+0x130,segments,len(subs),len(subs))
 writeq(requests,n,request,1);writeq(removedset,ctrl,15,14,int(case['removed']));u.mem_write(ctrl,b'\0'+b'\xff'*15);u.mem_write(ctrl-8,struct.pack('<II',3,8));writeq(graph,0,8,0)
 for i,owner in enumerate([0x217000,0x218000,structural,requests,removedset,stairs,0x219000]):param(0x200000+i*0x40,owner)
 writeq(stack,stop);writeq(stack+0x28,0x200100,0x200140,0x200180)
 for r,v in [(UC_X86_REG_RCX,0x200000),(UC_X86_REG_RDX,0x200040),(UC_X86_REG_R8,0x200080),(UC_X86_REG_R9,0x2000c0),(UC_X86_REG_RSP,stack)]:u.reg_write(r,v)
 heap=[0x300000];hulls=[];sources=[];upserts=[];spans=[];edges=[];counts=[];panic=[];bridges=set()
 def hook(mu,a,size,data):
  rcx=reg(UC_X86_REG_RCX);rdx=reg(UC_X86_REG_RDX);r8=reg(UC_X86_REG_R8);r9=reg(UC_X86_REG_R9);sp=reg(UC_X86_REG_RSP)
  if a==0x140c16a00:bridges.add(hex(a));ret(0)
  elif a==0x140613c10:
   bridges.add(hex(a));value=heap[0];heap[0]=(value+rcx+31)&~15;assert heap[0]<0x3d0000;ret(value)
  elif a==0x140613c20:bridges.add(hex(a));ret()
  elif a==0x142923c00:bridges.add(hex(a));u.mem_write(rcx,read(rdx,r8));ret(rcx)
  elif a==0x1405b1be0:bridges.add(hex(a));ret(0)
  elif a==0x1407dec90:
   hulls.append(dict(positions=words(rdx,3)+words(r8,3),directions=words(r9,2)+words(q(sp+0x28),2),widths=words(q(sp+0x30),1)+words(q(sp+0x38),1)))
  elif a==0x1408cec10:
   bridges.add(hex(a));hulls[-1]['convex_points']=words(rdx,r8*3);u.mem_write(rcx,b'\1'+bytes(127));ret(rcx)
  elif a==0x1408ceb10:
   bridges.add(hex(a));source=words(r8,6);sources.append(source);u.mem_write(rcx,struct.pack('<Q',rdx)+read(r8,24));ret(rcx)
  elif a==0x141299ca0:
   bridges.add(hex(a));upserts.append(dict(key=words(r9,3),transform=words(q(sp+0x30),7)));slot=sources[-1][0]&255;u.mem_write(rcx,struct.pack('<IIB3x',len(upserts),1,slot));ret(rcx)
  elif a==0x140869d20:
   bridges.add(hex(a));spans.append(dict(key=words(rdx,3),height=reg(UC_X86_REG_XMM3)&0xffffffff));ret()
  elif a==0x141266aa0:
   bridges.add(hex(a));counts.append([rdx&0xffffffff,r8&0xffffffff,r9&0xffffffff]);ret()
  elif a==0x140869ff0:
   bridges.add(hex(a));edges.append(dict(a=words(rdx,3),b=words(r8,3),transient=r9&255));ret()
  elif a==0x1408fe8b0:bridges.add(hex(a));ret(graph)
  elif a in [0x1428d9596,0x1428d98a0,0x1428d9850,0x1428d9840,0x1428d9390,0x1428d9518,0x1428d9310]:panic.append(hex(a));u.emu_stop()
 h=u.hook_add(UC_HOOK_CODE,hook)
 try:u.emu_start(0x1407d20c0,stop,count=100000)
 except Exception as e:raise RuntimeError((ci,case,hex(reg(UC_X86_REG_RIP))))from e
 finally:u.hook_del(h)
 rows.append(dict(input=case,hulls=hulls,sources=sources,upserts=upserts,spans=spans,edges=edges,counts=counts,panic=panic,bridges=sorted(bridges)))
out=p.with_name('m01-stair-structural-stage-native-v3.json')
with out.open('x')as f:json.dump(dict(scope=__doc__,rne_sha256=hashlib.sha256(raw).hexdigest(),entry='0x1407d20c0',graph_boundary='Original graph getter returns an explicit empty input graph; cross-junction phase requires independent nonempty input verification.',rows=rows),f,indent=2)
print(json.dumps(dict(cases=len(rows),panics=sum(bool(x['panic'])for x in rows),hulls=sum(len(x['hulls'])for x in rows))))
fixture=p.with_name('m01-stair-structural-wall-anchor-native-cases.rs')
with fixture.open('x')as f:
 f.write('// 原1407d20c0梯段内联，四墙锚组合×升/降/等高。\nconst ORIGINAL_STRUCTURAL_WALL_ANCHORS:&[(u8,u8,[u32;24])]=&[\n')
 for o in rows[127:]:f.write('(%d,%d,[%s]),\n'%(o['input']['anchors'],o['input']['y_mode'],','.join(map(str,o['hulls'][0]['convex_points']))))
 f.write('];\n')
