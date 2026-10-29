"""B原屋顶合并完整提议与碰撞谓词；桥接已解析资源查询，执行实际原几何/分支，不仿写内部容器。"""
from pathlib import Path
import struct,json,hashlib,math,ctypes,pefile
from unicorn import *
from unicorn.x86_const import *
root=Path('/Users/yuluo/Desktop/game');raw=(root/'B版本内容/tiny-glade.rne').read_bytes();sha=hashlib.sha256(raw).hexdigest();assert sha=='f507a6cea25c5199447b88ba746a5293e64a279f73797ca74c1ce7f4ff2b7d03'
p=pefile.PE(data=raw,fast_load=True);base=p.OPTIONAL_HEADER.ImageBase;image=p.get_memory_mapped_image();u=Uc(UC_ARCH_X86,UC_MODE_64);u.mem_map(base,(len(image)+4095)&~4095);u.mem_write(base,image);u.mem_map(0x100000,0x200000);u.mem_map(0x400000,0x800000)
def pack(xs):return struct.pack('<'+'f'*len(xs),*xs)
def f(x):return struct.unpack('<f',pack([x]))[0]
def xmm(r,x):u.reg_write(r,int.from_bytes(pack([x])+bytes(12),'little'))
def scalar(r):return struct.unpack('<f',u.reg_read(r).to_bytes(16,'little')[:4])[0]
def ret():
 sp=u.reg_read(UC_X86_REG_RSP);ip=int.from_bytes(u.mem_read(sp,8),'little');u.reg_write(UC_X86_REG_RSP,sp+8);u.reg_write(UC_X86_REG_RIP,ip)
heap=0x500000;records=[];transform=b'';seen=[];stats={};intermediates=[]
p.parse_data_directories(directories=[pefile.DIRECTORY_ENTRY['IMAGE_DIRECTORY_ENTRY_IMPORT']]);imports={i.address:i.name.decode() for d in p.DIRECTORY_ENTRY_IMPORT for i in d.imports if i.name};math_hooks={};libm=ctypes.CDLL(None);fns={}
for va in range(0x142923000,0x142925000):
 b=bytes(u.mem_read(va,6))
 if b[:2]==b'\xff\x25':
  name=imports.get(va+6+struct.unpack('<i',b[2:])[0],'')
  if name in ['ceilf','floorf','roundf','_hypotf','atanf','atan2f','sinf','cosf','powf','fmodf']:
   math_hooks[va]=name


# 容器仅在已解析查表边界桥接；不伪造Rust/Bevy/Hashbrown内部布局。
active_targets={};active_transforms={};missing_collision=False;subtype=8;quats=[];target_box=None
class OriginalPanic(Exception):pass

def hook(uc,ip,size,_):
 global heap,target_box
 if ip==0x142001ddc:
  ptr=active_targets.get(uc.reg_read(UC_X86_REG_R12))
  if ptr is None: raise OriginalPanic('missing target')
  uc.reg_write(UC_X86_REG_RBX,uc.reg_read(UC_X86_REG_R8));uc.reg_write(UC_X86_REG_RSI,uc.reg_read(UC_X86_REG_RCX))
  ty=uc.mem_read(uc.reg_read(UC_X86_REG_R8)+0x46,1)[0];uc.reg_write(UC_X86_REG_RDI,ty);uc.reg_write(UC_X86_REG_R13,ty)
  uc.reg_write(UC_X86_REG_R14,ptr+0x48);uc.reg_write(UC_X86_REG_R15,0);uc.reg_write(UC_X86_REG_RIP,0x142001ea2)
 elif ip==0x141fe6a7a:
  uc.reg_write(UC_X86_REG_RSI,uc.reg_read(UC_X86_REG_RCX))
  ptr=active_targets.get(uc.reg_read(UC_X86_REG_RBX))
  if ptr is None:uc.reg_write(UC_X86_REG_RIP,0x141fe6bf9)
  else:
   uc.reg_write(UC_X86_REG_R14,ptr+0x48);uc.reg_write(UC_X86_REG_RDI,0);uc.reg_write(UC_X86_REG_RIP,0x141fe6b32)
 elif ip==0x142001199:
  ptr=active_targets.get(uc.reg_read(UC_X86_REG_R15))
  if ptr is None:uc.reg_write(UC_X86_REG_RIP,0x14200179f)
  else:
   uc.reg_write(UC_X86_REG_RSI,ptr+0x48);uc.reg_write(UC_X86_REG_RDI,0);uc.reg_write(UC_X86_REG_RIP,0x142001240)
 elif ip in [0x140b26970,0x140833ba0]:
  uc.reg_write(UC_X86_REG_RAX,subtype);ret()
 elif ip==0x14125cf90:
  key=bytes(uc.mem_read(uc.reg_read(UC_X86_REG_RDX),2));rank=key[1]
  # 原LibKey是(subtype,rank)；资源桥只返回该真实输入键对应的bounds。
  ptr=0x480000+rank*128;uc.mem_write(ptr+0x30,pack([0.7+rank*0.83,0.,-.2,1.8+rank*.1]));uc.reg_write(UC_X86_REG_RAX,0 if missing_collision else ptr);ret()
 elif ip==0x141fe71db:
  target_box=list(struct.unpack('<8I',uc.mem_read(uc.reg_read(UC_X86_REG_RSI),32)))
 elif ip==0x140a1aa20:
  transform=active_transforms.get(uc.reg_read(UC_X86_REG_R8))
  if transform is None:raise OriginalPanic('missing cached transform')
  uc.mem_write(uc.reg_read(UC_X86_REG_RCX),pack(transform));ret()
 elif ip==0x140613c10:
  n=uc.reg_read(UC_X86_REG_RCX);heap=(heap+15)&~15;uc.reg_write(UC_X86_REG_RAX,heap);heap+=n;ret()
 elif ip==0x140d65850:
  old=uc.reg_read(UC_X86_REG_RCX);oldn=uc.reg_read(UC_X86_REG_RDX);n=uc.reg_read(UC_X86_REG_R9);heap=(heap+15)&~15;uc.mem_write(heap,bytes(uc.mem_read(old,min(n,oldn))));uc.reg_write(UC_X86_REG_RAX,heap);heap+=n;ret()
 elif ip==0x140613c20:ret()
 elif ip==0x140c16a00:uc.reg_write(UC_X86_REG_RAX,0);ret()
 elif ip in math_hooks:
  name=math_hooks[ip].lstrip('_');fn=fns.get(name)
  if fn is None:
   fn=getattr(libm,name);fn.argtypes=[ctypes.c_float]*(2 if name in ['atan2f','hypotf','powf','fmodf'] else 1);fn.restype=ctypes.c_float;fns[name]=fn
  a=scalar(UC_X86_REG_XMM0);b=scalar(UC_X86_REG_XMM1);xmm(UC_X86_REG_XMM0,fn(a,b) if len(fn.argtypes)==2 else fn(a));ret()
 elif ip in [0x1428d9330,0x1428d9390,0x1428d9430,0x1428d9310,0x140d8b480]:raise OriginalPanic(hex(ip)+' rcx='+hex(uc.reg_read(UC_X86_REG_RCX))+' ret='+hex(int.from_bytes(uc.mem_read(uc.reg_read(UC_X86_REG_RSP),8),'little')))
u.hook_add(UC_HOOK_CODE,hook)
def bits(xs):return list(struct.unpack('<'+'I'*len(xs),pack(xs)))
def roof(rect=True):
 b=bytearray(88);b[:8]=struct.pack('<Q',7);b[8:36]=struct.pack('<I',int(rect))+pack([1.,0.,0.,0.,8.,6.] if rect else [0.,0.,4.,0.,0.,0.]);b[0x24:0x40]=pack([0.,0.,.5,.6,.3,.2,0.]);return bytes(b)
def info(rank=1,ty=0x38,tag=2,coord=(.25,.6),yaw=.7):
 b=bytearray((i*13+7)%256 for i in range(72));b[:8]=pack(coord);b[8:12]=pack([yaw]);b[0x22]=tag;b[0x44:0x48]=bytes([rank,51,ty,1]);return bytes(b)
def reset():
 global heap,active_targets,active_transforms
 heap=0x500000;active_targets={};active_transforms={};sp=0x1ff008;u.mem_write(sp,struct.pack('<Q',0x100000)+bytes(0x100));u.mem_write(0x406000,bytes([0xcd])*256);u.reg_write(UC_X86_REG_RSP,sp);return sp


import random
r=random.Random(0x147)
def wall_proposal(new_ty,old_ty,new_rank,old_rank,old_coord,new_coord,loop,length,corner,sub):
 global subtype
 subtype=sub;sp=reset();incoming=bytearray(info(new_rank,new_ty,tag=0,coord=new_coord));target=bytearray(info(old_rank,old_ty,tag=0,coord=old_coord));target[0x20]=corner;target[0x21]=1
 target[8:16]=struct.pack('<Q',0x0000000200000003)
 u.mem_write(0x401000,bytes(incoming));u.mem_write(0x404000,struct.pack('<2Q',0x409000,0x40a000))
 # 原桶迭代器的一项只读fixture：唯一目标本来就被两个排除ID之一过滤。
 ctrl=0x412000;ptr=ctrl-80+8;u.mem_write(ctrl,bytes([0])+bytes([255])*15);u.mem_write(ctrl-80,struct.pack('<II',9,0)+bytes(target))
 active_targets[9]=ptr;u.mem_write(0x407000,bytes(0x90));u.mem_write(0x407030,struct.pack('<4Q',ctrl,0,0,1))
 for offset,value in [(0x28,0),(0x30,int(loop)),(0x50,9),(0x58,0x407000),(0x60,0x404000)]:u.mem_write(sp+offset,struct.pack('<Q',value))
 u.mem_write(sp+0x38,pack([length]));u.mem_write(sp+0x40,pack([new_coord[0]]));u.mem_write(sp+0x48,pack([new_coord[1]]))
 u.reg_write(UC_X86_REG_RCX,0x406000);u.reg_write(UC_X86_REG_RDX,177);u.reg_write(UC_X86_REG_R8,0x401000);u.reg_write(UC_X86_REG_R9,4)
 try:
  u.emu_start(0x142001d40,0x100000,count=1000000);assert u.reg_read(UC_X86_REG_RIP)==0x100000
  out=bytes(u.mem_read(0x406000,104));status='none' if out[95]==2 else 'some';expected=dict(status=status)
  if status=='some':expected.update(coord=list(struct.unpack('<2I',out[:8])),platform=struct.unpack('<Q',out[8:16])[0],attachment=list(out[16:19]),info=list(out[24:96]),target=struct.unpack('<I',out[96:100])[0],distance=struct.unpack('<I',out[100:104])[0])
 except OriginalPanic as e:expected=dict(status='panic',why=str(e))
 return dict(incoming=list(incoming),target_info=list(target),coord=bits(new_coord),loop=loop,length=bits([length])[0],subtype=sub,expected=expected)
rows=[]
for loop in [False,True]:
 for new_ty,old_ty in [(0x38,0x38),(0x39,0x39),(0x3a,0x3a),(0x3f,0x40),(0x3f,0x3f),(0x40,0x40),(0x40,0x41)]:
  for new_rank,old_rank in [(1,1),(1,2),(2,2),(255,1)]:
   for corner in [0,2]:
    for sub in [0,9]:rows.append(wall_proposal(new_ty,old_ty,new_rank,old_rank,(.4,.8),(1.7,1.2),loop,10.,corner,sub))
for _ in range(128):rows.append(wall_proposal(0x38,0x38,1,1,(f(r.uniform(0,10)),f(r.uniform(-2,3))),(f(r.uniform(0,10)),f(r.uniform(-2,3))),bool(r.randrange(2)),10.,0,r.choice([0,9])))
report=Path('/Users/yuluo/Desktop/game/tiny-glade/reports/module-recovery-2026-10-23')
(report/'m01-wall-merge-native.json').write_text(json.dumps(dict(original_sha256=sha,scope='complete142001d40; resolved target-hash/subtype/mesh boundaries, actual single native bucket iteration/exclusions, original WallSpace/geometry/full proposal writes',rows=rows),indent=2));print('wall proposals',len(rows),{s:sum(row['expected']['status']==s for row in rows) for s in ['some','none','panic']})
pred_rows=[]
for loop in [False,True]:
 for ty in [0x38,0x3a]:
  for excluded in [False,True]:
   for dx in [0.,1.,3.,9.9]:
    for dy in [0.,.59,.6,1.,2.]:
     subtype=0;sp=reset();target=info(1,ty,0,coord=(.1,1.));current=177 if excluded else 9
     u.mem_write(0x401000,target);u.mem_write(0x402000,struct.pack('<I',current));u.mem_write(0x403000,pack([.1+dx,1.+dy,1.7,2.2]));u.mem_write(0x404000,struct.pack('<2Q',0x409000,0x40a000));u.mem_write(0x405000,struct.pack('<Q',0x405100));u.mem_write(0x405100,struct.pack('<I',177));u.mem_write(0x405200,pack([10.]));u.mem_write(0x406000,struct.pack('<4Q',0x405000,0x404000,0x403000,0x405200));u.mem_write(0x406100,struct.pack('<Q',0x406000))
     u.reg_write(UC_X86_REG_RCX,0x406100);u.reg_write(UC_X86_REG_RDX,0x402000);u.reg_write(UC_X86_REG_R8,0x401000)
     u.emu_start(0x141fe5530 if loop else 0x141fe5720,0x100000,count=100000);assert u.reg_read(UC_X86_REG_RIP)==0x100000
     pred_rows.append(dict(loop=loop,ty=ty,excluded=excluded,dx=bits([dx])[0],dy=bits([dy])[0],hit=bool(u.reg_read(UC_X86_REG_RAX)&255)))
(report/'m01-wall-intent-predicate-native.json').write_text(json.dumps(dict(original_sha256=sha,scope='complete141fe5530/141fe5720, resolved subtype/collision only; real circular distance142007170',rows=pred_rows),indent=2));print('wall predicates',len(pred_rows))
# 原find_best外层也完整执行：相同距离首命中、候选顺序、无候选、NaN实际比较。
minimum_rows=[]
for ids in [[],[9],[10],[9,10],[10,9],[9,9]]:
 for new_u in [5.,2.3,8.,float('nan')]:
  subtype=0;sp=reset();incoming=info(1,0x38,0,coord=(new_u,.8));u.mem_write(0x401000,incoming)
  ctrl=0x412000;u.mem_write(ctrl,bytes([0,0])+bytes([255])*14)
  for i,(id,coord) in enumerate([(9,(2.,.8)),(10,(8.,.8))]):
   ptr=ctrl-(i+1)*80+8;record=bytearray(info(1,0x38,0,coord=coord));record[0x20]=0;record[0x21]=0
   u.mem_write(ptr-8,struct.pack('<II',id,0)+bytes(record));active_targets[id]=ptr
  u.mem_write(0x407000,bytes(0x90));u.mem_write(0x407030,struct.pack('<4Q',ctrl,1,0,2));u.mem_write(0x404000,struct.pack('<2Q',0x409000,0x40a000));u.mem_write(0x405000,struct.pack('<'+'I'*len(ids),*ids) if ids else bytes(4))
  for offset,value in [(0x28,0),(0x30,1),(0x50,0x405000),(0x58,len(ids)),(0x60,0x407000),(0x68,0x404000)]:u.mem_write(sp+offset,struct.pack('<Q',value))
  u.mem_write(sp+0x38,pack([10.]));u.mem_write(sp+0x40,pack([new_u]));u.mem_write(sp+0x48,pack([.8]));u.reg_write(UC_X86_REG_RCX,0x406000);u.reg_write(UC_X86_REG_RDX,177);u.reg_write(UC_X86_REG_R8,0x401000);u.reg_write(UC_X86_REG_R9,4)
  try:
   u.emu_start(0x142001880,0x100000,count=1000000);assert u.reg_read(UC_X86_REG_RIP)==0x100000
   out=bytes(u.mem_read(0x406000,104));expected=dict(status='none' if out[95]==2 else 'some')
   if expected['status']=='some':expected.update(target=struct.unpack('<I',out[96:100])[0],distance=struct.unpack('<I',out[100:104])[0])
  except OriginalPanic as e:expected=dict(status='panic',why=str(e))
  minimum_rows.append(dict(ids=ids,u=bits([new_u])[0],expected=expected))
(report/'m01-wall-merge-minimum-native.json').write_text(json.dumps(dict(original_sha256=sha,scope='complete142001880+142001d40 + actual native two-record collision bucket; same resolved dependency boundaries',rows=minimum_rows),indent=2));print('minima',len(minimum_rows))
