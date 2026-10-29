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
 if ip==0x141fe6a7a:
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
 elif ip==0x140b26970:
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
 elif ip in [0x1428d9330,0x1428d9390,0x1428d9430,0x1428d9310,0x140d8b480]:raise OriginalPanic(hex(ip))
u.hook_add(UC_HOOK_CODE,hook)
def bits(xs):return list(struct.unpack('<'+'I'*len(xs),pack(xs)))
def roof(rect=True):
 b=bytearray(88);b[:8]=struct.pack('<Q',7);b[8:36]=struct.pack('<I',int(rect))+pack([1.,0.,0.,0.,8.,6.] if rect else [0.,0.,4.,0.,0.,0.]);b[0x24:0x40]=pack([0.,0.,.5,.6,.3,.2,0.]);return bytes(b)
def info(rank=1,ty=0x38,tag=2,coord=(.25,.6),yaw=.7):
 b=bytearray((i*13+7)%256 for i in range(72));b[:8]=pack(coord);b[8:12]=pack([yaw]);b[0x22]=tag;b[0x44:0x48]=bytes([rank,51,ty,1]);return bytes(b)
def reset():
 global heap,active_targets,active_transforms
 heap=0x500000;active_targets={};active_transforms={};sp=0x1ff008;u.mem_write(sp,struct.pack('<Q',0x100000)+bytes(0x100));u.mem_write(0x406000,bytes([0xcd])*256);u.reg_write(UC_X86_REG_RSP,sp);return sp

def proposal(incoming,targets,ids,new_coord,position,rect,missing=False):
 global missing_collision
 missing_collision=missing;sp=reset();u.mem_write(0x401000,incoming);u.mem_write(0x402000,roof(rect));u.mem_write(0x403000,pack(position));u.mem_write(0x404000,struct.pack('<2Q',0x409000,0x40a000));u.mem_write(0x405000,struct.pack('<'+'I'*len(ids),*ids) if ids else bytes(4))
 for i,(id,record) in enumerate(targets):
  ptr=0x410000+i*0x100;active_targets[id]=ptr;u.mem_write(ptr,record)
 for offset,value in [(0x28,0x403000),(0x30,0x402000),(0x38,0x405000),(0x40,len(ids)),(0x48,0x407000),(0x50,0x404000)]:u.mem_write(sp+offset,struct.pack('<Q',value))
 u.reg_write(UC_X86_REG_RCX,0x406000);u.reg_write(UC_X86_REG_RDX,0x401000);xmm(UC_X86_REG_XMM2,new_coord[0]);xmm(UC_X86_REG_XMM3,new_coord[1])
 try:
  u.emu_start(0x142000e60,0x100000,count=1000000);assert u.reg_read(UC_X86_REG_RIP)==0x100000,hex(u.reg_read(UC_X86_REG_RIP));out=bytes(u.mem_read(0x406000,112));tag=struct.unpack('<I',out[:4])[0];result={'status':'none' if tag==4 else 'some'}
  if tag!=4:result.update(rotation_coord=list(struct.unpack('<3I',out[4:16])),info=list(out[32:104]),target_id=struct.unpack('<I',out[104:108])[0],distance=struct.unpack('<I',out[108:112])[0])
 except OriginalPanic as e:result={'status':'panic','why':str(e)}
 return dict(incoming=list(incoming),targets=[[id,list(record)] for id,record in targets],ids=ids,new_coord=bits(new_coord),position=bits(position),rectangle=rect,missing_collision=missing,expected=result)
rows=[]
for rect in [False,True]:
 for a,b in [(0x38,0x38),(0x39,0x39),(0x3a,0x3a),(0x3f,0x3f),(0x3f,0x40),(0x41,0x42),(0x41,0x41),(0x37,0x37),(0x38,0x39)]:
  for r1,r2 in [(0,0),(1,1),(1,2),(2,2)]:
   for nc in [(.1,.6),(.7,.9),(.25,.6)]:rows.append(proposal(info(r1,a),[(9,info(r2,b))],[9],nc,(-4.,2.,0.),rect))
for ids in [[],[9],[9,10],[10,9],[11,9],[9,9]]:
 for coords in [(.25,.6),(.7,.2),(float('nan'),.2)]:rows.append(proposal(info(),[(9,info()),(10,info(coord=(.4,.7))),(11,info(ty=0x39))],ids,coords,(-4.,2.,0.),True))
rows.append(proposal(info(),[],[99],(.1,.1),(0.,2.,0.),True))
rows.append(proposal(info(),[(9,info())],[9],(.1,.1),(0.,2.,0.),True,True))
rows.append(proposal(info(),[(9,info(tag=4))],[9],(.1,.1),(0.,2.,0.),True))
Path('/tmp/m01-roof-merge-native.json').write_text(json.dumps(dict(original_sha256=sha,scope='full142000e60+141fe6a40; resolved hash/subtype/collision query boundaries; actual native Roof shape/noise/perimeter/rectangle helpers; Darwin C float math',rows=rows),indent=2));print('proposal',len(rows),'some',sum(r['expected']['status']=='some' for r in rows),'panic',sum(r['expected']['status']=='panic' for r in rows))

pred_rows=[]
for tag in [0,1,2,3,4,9]:
 for ty in [0x38,0x39,0x3f,0x40]:
  for exclude in [0,9]:
   for quat in [[0.,0.,0.,1.],[0.,f(math.sin(.35)),0.,f(math.cos(.35))],[f(math.sin(.2)),0.,0.,f(math.cos(.2))]]:
    for dx in [0.,5.]:
     sp=reset();missing_collision=False;target_box=None
     incoming=info(1,0x38);target=info(1,ty,tag);pos=[dx,1.,0.];transform=quat+[0.,0.,0.,2.,3.,4.];active_transforms[9]=transform
     u.mem_write(0x401000,incoming);u.mem_write(0x402000,pack(pos)+bytes(4)+pack(quat));u.mem_write(0x403000,struct.pack('<2Q',0x409000,0x40a000))
     for offset,value in [(0x28,0x407000),(0x30,0x403000),(0x38,0x405000),(0x40,0x40b000),(0x48,0x403000)]:u.mem_write(sp+offset,struct.pack('<Q',value))
     u.reg_write(UC_X86_REG_RCX,0x401000);u.reg_write(UC_X86_REG_RDX,0x402000);xmm(UC_X86_REG_XMM2,1.7);xmm(UC_X86_REG_XMM3,2.2)
     u.emu_start(0x141fe7280,0x141fe73c2,count=100000);assert u.reg_read(UC_X86_REG_RIP)==0x141fe73c2
     query=bytes(u.mem_read(u.reg_read(UC_X86_REG_R14),32));u.mem_write(0x408000,query)
     # 完整原谓词环境仅含借用的真实语义对象；hash/cache查表在原getter处已解析桥接。
     u.mem_write(0x404000,struct.pack('<6Q',0x405000,0x401046,0x40b000,0x403000,0x403000,0x408000));u.mem_write(0x405000,struct.pack('<Q',0x405100));u.mem_write(0x405100,struct.pack('<I',exclude));u.mem_write(0x405200,struct.pack('<I',9));u.mem_write(0x405300,target);u.mem_write(0x406000,struct.pack('<Q',0x404000));u.mem_write(sp,struct.pack('<Q',0x100000))
     u.reg_write(UC_X86_REG_RSP,sp);u.reg_write(UC_X86_REG_RCX,0x406000);u.reg_write(UC_X86_REG_RDX,0x405200);u.reg_write(UC_X86_REG_R8,0x405300)
     try:u.emu_start(0x141fe6f20,0x100000,count=100000);assert u.reg_read(UC_X86_REG_RIP)==0x100000;expected={'status':'value','hit':bool(u.reg_read(UC_X86_REG_RAX)&255)}
     except OriginalPanic as e:expected={'status':'panic','why':str(e)}
     pred_rows.append(dict(incoming=list(incoming),target=list(target),exclude=exclude,quat=bits(quat),position=bits(pos),transform=bits(transform),width=bits([1.7])[0],height=bits([2.2])[0],query=list(struct.unpack('<8I',query)),target_box=target_box,expected=expected))
Path('/tmp/m01-roof-predicate-native.json').write_text(json.dumps(dict(original_sha256=sha,scope='complete141fe6f20 + original141fe7280 query constructor prefix; resolved subtype/collision/cached lookup boundaries, all quaternion/rectangle overlap original native instructions',rows=pred_rows),indent=2));print('predicate',len(pred_rows),'hit',sum(r['expected'].get('hit',False) for r in pred_rows),'panic',sum(r['expected']['status']=='panic' for r in pred_rows))
