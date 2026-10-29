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

# 资源接口隔离，原几何/噪声/周长/矩形与所有split主体指令实际执行。
subtype=2;ops=[];missing_info=False;missing_collision=False

def hook(uc,ip,size,_):
 global heap
 if ip==0x140a1c800:
  uc.reg_write(UC_X86_REG_RAX,0 if missing_info else 0x401000);ret()
 elif ip==0x140b26970:uc.reg_write(UC_X86_REG_RAX,subtype);ret()
 elif ip==0x14125cf90:uc.reg_write(UC_X86_REG_RAX,0 if missing_collision else 0x408000);ret()
 elif ip==0x141276e30:
  ops.append(dict(op=list(u.mem_read(u.reg_read(UC_X86_REG_RCX),112)),position=list(u.mem_read(u.reg_read(UC_X86_REG_R8),16)),state=u.reg_read(UC_X86_REG_RDX)&255));ret()
 elif ip==0x140613c10:
  n=uc.reg_read(UC_X86_REG_RCX);heap=(heap+15)&~15;uc.reg_write(UC_X86_REG_RAX,heap);heap+=n;ret()
 elif ip==0x140d65850:
  old=uc.reg_read(UC_X86_REG_RCX);oldn=uc.reg_read(UC_X86_REG_RDX);n=uc.reg_read(UC_X86_REG_R9);heap=(heap+15)&~15;uc.mem_write(heap,bytes(uc.mem_read(old,min(n,oldn))));uc.reg_write(UC_X86_REG_RAX,heap);heap+=n;ret()
 elif ip==0x140613c20:ret()
 elif ip in math_hooks:
  name=math_hooks[ip].lstrip('_');fn=fns.get(name)
  if fn is None:
   fn=getattr(libm,name);fn.argtypes=[ctypes.c_float]*(2 if name in ['atan2f','hypotf','powf','fmodf'] else 1);fn.restype=ctypes.c_float;fns[name]=fn
  a=scalar(UC_X86_REG_XMM0);b=scalar(UC_X86_REG_XMM1);xmm(UC_X86_REG_XMM0,fn(a,b) if len(fn.argtypes)==2 else fn(a));ret()
 elif ip in [0x1428d9330,0x1428d9390,0x1428d9430,0x1428d9310,0x1428d9518,0x140d8b480]:raise OriginalPanic(hex(ip))
u.hook_add(UC_HOOK_CODE,hook)
def bits(xs):return list(struct.unpack('<'+'I'*len(xs),pack(xs)))
def roof(rect=True):
 b=bytearray(88);b[:8]=struct.pack('<Q',7);b[8:36]=struct.pack('<I',int(rect))+pack([1.,0.,0.,0.,8.,6.] if rect else [0.,0.,4.,0.,0.,0.]);b[0x24:0x40]=pack([0.,0.,.5,.6,.3,.2,0.]);return bytes(b)
def info(rank,ty,uv):
 b=bytearray((i*13+7)%256 for i in range(72));b[:8]=pack(uv);b[8:12]=pack([.7]);b[0x22]=2;b[0x41]=6;b[0x44:0x48]=bytes([rank,51,ty,1]);return bytes(b)
rows=[]
for rect in [False,True]:
 for subtype in [2,8,7]:
  for rank in [0,1,2,3,4]:
   for part in range(max(1,rank)):
    for ty in [0x38,0x3a]:
     for uv in [(.01,.6),(.99,.6)]:
      heap=0x500000;ops=[];record=info(rank,ty,uv);roof_bytes=roof(rect);position=(-4.,2.,0.);pose=[0.,0.,0.,1.,*position,1.,1.,1.]
      sp=0x1ff008;u.mem_write(sp,struct.pack('<Q',0x100000)+bytes(0x100));u.reg_write(UC_X86_REG_RSP,sp)
      u.mem_write(0x401000,record);u.mem_write(0x402000,roof_bytes);u.mem_write(0x403000,pack(pose));u.mem_write(0x404000,struct.pack('<2Q',0,7));u.mem_write(0x405000,bytes(32));u.mem_write(0x405100,struct.pack('<I',100));u.mem_write(0x405000,struct.pack('<QQB',0x405100,0,0));u.mem_write(0x406000,b'\xcd'*256);u.mem_write(0x408030,pack([3.1,0.,-.2,2.3]));u.mem_write(0x409000,struct.pack('<2Q',0x40a000,0x40b000))
      for offset,value in [(0x28,41),(0x30,0x403000),(0x38,0x409000),(0x40,0x405000),(0x48,0x40c000),(0x50,0x40d000),(0x58,0x40e000),(0x60,0x40f000),(0x68,1),(0x70,0x402000)]:u.mem_write(sp+offset,struct.pack('<Q',value))
      for reg,value in [(UC_X86_REG_RCX,0x406000),(UC_X86_REG_RDX,part),(UC_X86_REG_R8,0x410000),(UC_X86_REG_R9,0x404000)]:u.reg_write(reg,value)
      try:
       u.emu_start(0x14127bab0,0x100000,count=1000000);assert u.reg_read(UC_X86_REG_RIP)==0x100000,hex(u.reg_read(UC_X86_REG_RIP))
       out=bytes(u.mem_read(0x406000,72));ptr,n=struct.unpack_from('<QQ',out,8);ids=list(struct.unpack('<'+'I'*n,u.mem_read(ptr,4*n))) if n else []
       expected=dict(status='success',ids=ids,moving_id=struct.unpack_from('<I',out,24)[0],destination=list(out[32:72]),ops=ops)
      except OriginalPanic as e:expected=dict(status='panic',why=str(e))
      rows.append(dict(rectangle=rect,roof=list(roof_bytes),info=list(record),part=part,subtype=subtype,pose=bits(pose),bounds=bits([3.1,0.,-.2,2.3]),expected=expected))
meta=dict(binary_sha256=sha,entry='0x14127bab0',service_bridges=['storage required Info lookup','subtype/collision library lookup','allocator','apply_decorator_op captured; effects validated in real World'],math_boundary='actual RNE Roof/noise/perimeter/rectangle; imported C float math uses Darwin libm, Windows CRT body excluded',rows=rows)
Path(__file__).with_suffix('.json').write_text(json.dumps(meta,indent=2)+'\n');print(dict(cases=len(rows),success=sum(r['expected']['status']=='success' for r in rows)))

# 完整picker主体：仅以明确的Query成功结果隔离Bevy内部组件表。
factory_positions=[];display_mode=False

def display_hook(uc,ip,size,_):
 global heap
 if not display_mode:return
 if ip==0x14127a817:
  bp=uc.reg_read(UC_X86_REG_RBP);uc.mem_write(bp+0xe8,struct.pack('<Q',0x402000));uc.reg_write(UC_X86_REG_RAX,0x405000);uc.reg_write(UC_X86_REG_RCX,0);uc.reg_write(UC_X86_REG_RIP,0x14127a9b6)
 elif ip==0x140d79f90:
  heap=(heap+15)&~15;uc.mem_write(heap,b'fixture');uc.mem_write(uc.reg_read(UC_X86_REG_RCX),struct.pack('<3Q',7,heap,7));heap+=16;ret()
 elif ip==0x14089d8b0:
  sp=uc.reg_read(UC_X86_REG_RSP);marker=struct.unpack('<Q',uc.mem_read(sp+0x28,8))[0];env=uc.reg_read(UC_X86_REG_R9)
  factory_positions.append(dict(position=list(struct.unpack('<3I',uc.mem_read(env+16,12))),marker=hex(marker)))
  uc.mem_write(uc.reg_read(UC_X86_REG_RCX),bytes(40));ret()
 elif ip==0x140b2cf90:ret()
u.hook_add(UC_HOOK_CODE,display_hook)
display_rows=[]
for rect in [False,True]:
 for rank in [0,1,2,3,4]:
  for q in [[0,0,0,1],[0,.707106769,0,.707106769],[.2,.3,.4,.5],[0,0,1,0]]:
   for uv in [(.01,.6),(.99,.6)]:
    for point in [(-4.,2.,0.),(-1000.,1.,-1000.),(0.,1.,0.)]:
     for sub in [2,7]:
      display_mode=True;factory_positions=[];heap=0x500000;sp=0x1ff008;record=info(rank,0x38,uv);roof_bytes=roof(rect);pose=q+[-4.,2.,0.,1.,1.,1.]
      u.mem_write(sp,struct.pack('<Q',0x100000)+bytes(0x100));u.mem_write(0x401000,record);u.mem_write(0x402000,roof_bytes);u.mem_write(0x403000,pack(pose));u.mem_write(0x404000,bytes(24)+pack(point)+bytes(64));u.mem_write(0x405000,bytes(72));u.mem_write(0x405008,struct.pack('<2Q',0x423000,1));u.mem_write(0x405038,struct.pack('<2Q',0x424000,1));u.mem_write(0x423000,pack([0,1,0,4,1,0,0,1,4]));u.mem_write(0x424000,pack([0,0,1,0,0,1]))
      for offset,value in [(0x28,sub),(0x30,0x406000),(0x38,0x407000),(0x40,0x408000),(0x48,0x409000),(0x50,0x40a000)]:u.mem_write(sp+offset,struct.pack('<Q',value))
      for reg,value in [(UC_X86_REG_RSP,sp),(UC_X86_REG_RCX,0x404000),(UC_X86_REG_RDX,0x401000),(UC_X86_REG_R8,0x403000),(UC_X86_REG_R9,41)]:u.reg_write(reg,value)
      try:
       u.emu_start(0x14127a790,0x100000,count=1000000);assert u.reg_read(UC_X86_REG_RIP)==0x100000
       expected=dict(status='success',selected=u.reg_read(UC_X86_REG_RDX) if u.reg_read(UC_X86_REG_RAX)&1 else None,factories=factory_positions)
      except OriginalPanic as e:expected=dict(status='panic',why=str(e))
      display_rows.append(dict(info=list(record),roof=list(roof_bytes),pose=bits(pose),hit=bits(point),subtype=sub,expected=expected))
      display_mode=False
Path(__file__).with_name('m01-roof-split-display-native.json').write_text(json.dumps(dict(binary_sha256=sha,entry='14127a790',service_bridges=['Query successful entity/component resolution','formatter and allocation','Factory/stream captured: closure environment world positions; actual mesh factory checked in real World'],math_boundary='actual native geometry and body; Windows CRT body excluded',rows=display_rows),indent=2)+'\n')
print(dict(display_cases=len(display_rows),success=sum(r['expected']['status']=='success' for r in display_rows)))

# 夹具只投影原union的活动字段；原未初始化padding不作为Rust等价证明。
num=lambda xs:','.join(map(str,xs))
lines=['# infohex roofhex part subtype pose_bits bounds_bits status ids moving destination_bits ops(kind/id/rank/coord3/positionflag)']
for row in rows:
 e=row['expected'];line=[bytes(row['info']).hex(),bytes(row['roof']).hex(),str(row['part']),str(row['subtype']),num(row['pose']),num(row['bounds']),e['status']]
 if e['status']=='success':
  values=[]
  for x in e['ops']:
   b=bytes(x['op']);kind=struct.unpack_from('<Q',b)[0];new=kind==3
   # +5c是placement bool；+5e才是rank，不能因原子件二者都为1就混同。
   values.append(num([kind,struct.unpack_from('<I',b,0x54 if new else 0x68)[0],b[0x5e if new else 0x61],*struct.unpack_from('<3I',b,0x18 if new else 0x20),x['position'][12]]))
  line+=[num(e['ids']),str(e['moving_id']),num(struct.unpack_from('<3I',bytes(e['destination']),0)),';'.join(values)]
 lines.append('\t'.join(line))
Path(__file__).with_suffix('.tsv').write_text('\n'.join(lines)+'\n')
lines=['# infohex roofhex pose_bits hit_bits subtype status selected factories(position3|marker)']
for row in display_rows:
 e=row['expected'];line=[bytes(row['info']).hex(),bytes(row['roof']).hex(),num(row['pose']),num(row['hit']),str(row['subtype']),e['status']]
 if e['status']=='success':line+=[str(e['selected']) if e['selected'] is not None else '-1',';'.join(num(x['position'])+'|'+x['marker'] for x in e['factories'])]
 lines.append('\t'.join(line))
Path(__file__).with_name('m01-roof-split-display-native.tsv').write_text('\n'.join(lines)+'\n')
