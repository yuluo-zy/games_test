"""原B cottage get_dormer_panel_xforms内联区间，保留原shape/curve/rotation算法，仅挂系统分配与fmodf/已选roof section边界。"""
from pathlib import Path
import hashlib,json,struct,math,ctypes
import pefile
from unicorn import Uc,UC_ARCH_X86,UC_MODE_64,UC_HOOK_CODE
from unicorn.x86_const import *
root=Path(__file__).resolve().parents[3];raw=(root/'B版本内容/tiny-glade.rne').read_bytes();sha=hashlib.sha256(raw).hexdigest();assert sha=='f507a6cea25c5199447b88ba746a5293e64a279f73797ca74c1ce7f4ff2b7d03'
p=pefile.PE(data=raw,fast_load=True);base=p.OPTIONAL_HEADER.ImageBase;image=p.get_memory_mapped_image();u=Uc(UC_ARCH_X86,UC_MODE_64);u.mem_map(base,(len(image)+4095)&~4095);u.mem_write(base,image);u.mem_map(0x100000,0x200000);u.mem_map(0x400000,0x200000)
def pack(values):return struct.pack('<'+'f'*len(values),*values)
def f32(x):return struct.unpack('<f',pack([x]))[0]
def xmm(r,x):u.reg_write(r,int.from_bytes(pack([x])+bytes(12),'little'))
def scalar(r):return struct.unpack('<f',u.reg_read(r).to_bytes(16,'little')[:4])[0]
def ret():
 sp=u.reg_read(UC_X86_REG_RSP);ip=int.from_bytes(u.mem_read(sp,8),'little');u.reg_write(UC_X86_REG_RSP,sp+8);u.reg_write(UC_X86_REG_RIP,ip)
heap=0x410000;section=b'';seen=[];curve_snapshot=None
# 圆采样依赖Windows CRT超越函数；只绑定该外部数学边界，不替换游戏圆/曲线算法。
p.parse_data_directories(directories=[pefile.DIRECTORY_ENTRY['IMAGE_DIRECTORY_ENTRY_IMPORT']])
imports={f.address:f.name.decode() for dll in p.DIRECTORY_ENTRY_IMPORT for f in dll.imports if f.name}
math_hooks={}
host_libm=ctypes.CDLL(None);host_functions={}
def crt_float(name,a,b=None):
 symbol=name.lstrip('_');fn=host_functions.get(symbol)
 if fn is None:
  fn=getattr(host_libm,symbol);fn.argtypes=[ctypes.c_float]*(2 if symbol in ['atan2f','hypotf','fmodf'] else 1);fn.restype=ctypes.c_float;host_functions[symbol]=fn
 return fn(a,b) if b is not None else fn(a)

for va in range(0x142923000,0x142925000):
 b=bytes(u.mem_read(va,6))
 if b[:2]==b'\xff\x25':
  name=imports.get(va+6+struct.unpack('<i',b[2:])[0],'')
  if name in ['ceilf','floorf','_hypotf','atanf','atan2f','sinf','cosf','atan','atan2','sin','cos']:
   math_hooks[va]=name

def hook(uc,ip,size,_):
 global heap,curve_snapshot
 seen.append(hex(ip));seen[:]=seen[-12:]
 if ip==0x1407c89e6 and section[:4]==bytes(4):
  bp=uc.reg_read(UC_X86_REG_RBP);record=bytes(uc.mem_read(bp+0x1a0,56));cap,ptr,n,ucap,uptr,un=struct.unpack('<6Q',record[:48]);curve_snapshot={'points':[list(struct.unpack('<3I',uc.mem_read(ptr+i*12,12))) for i in range(n)],'points_u':list(struct.unpack('<'+'I'*un,uc.mem_read(uptr,un*4))),'length':struct.unpack('<I',record[48:52])[0]}
 if ip==0x140613c10:
  n=uc.reg_read(UC_X86_REG_RCX);heap=(heap+15)&~15;uc.reg_write(UC_X86_REG_RAX,heap);heap+=n;ret()
 elif ip==0x140d65850:
  old=uc.reg_read(UC_X86_REG_RCX);old_size=uc.reg_read(UC_X86_REG_RDX);n=uc.reg_read(UC_X86_REG_R9);heap=(heap+15)&~15;uc.mem_write(heap,bytes(uc.mem_read(old,min(old_size,n))));uc.reg_write(UC_X86_REG_RAX,heap);heap+=n;ret()
 elif ip==0x140613c20:ret()
 elif ip==0x142923f10:
  xmm(UC_X86_REG_XMM0,crt_float('fmodf',scalar(UC_X86_REG_XMM0),scalar(UC_X86_REG_XMM1)));ret()
 elif ip in math_hooks:
  name=math_hooks[ip];single=name.endswith('f');fmt='<f' if single else '<d';n=4 if single else 8
  value=lambda r:struct.unpack(fmt,uc.reg_read(r).to_bytes(16,'little')[:n])[0]
  a=value(UC_X86_REG_XMM0);b=value(UC_X86_REG_XMM1)
  op=(name[:-1] if single else name).lstrip('_')
  result=(crt_float(name,a,b) if op in ['atan2','hypot'] else crt_float(name,a)) if single else (getattr(math,op)(a,b) if op in ['atan2','hypot'] else getattr(math,op)(a))
  uc.reg_write(UC_X86_REG_XMM0,int.from_bytes(struct.pack(fmt,result)+bytes(16-n),'little'));ret()
 elif ip==0x1408e4750:
  uc.mem_write(uc.reg_read(UC_X86_REG_RCX),section);ret()
 if 0x1407c9b00<=ip<0x1407ca6c0:raise RuntimeError('original failure '+hex(ip))
u.hook_add(UC_HOOK_CODE,hook)
rows=[]
for rank in [1,2,3]:
 for center,pos,axis,size in [([0.,0.],[-4.,1.25,0.],[1.,0.],[8.,6.]),([0.,0.],[-4.,1.25,-2.9],[1.,0.],[8.,6.]),([3.,-2.],[0.25,2.5,-4.],[0.6,0.8],[9.,7.]),([0.,0.],[-3.,1.25,0.],[1.,0.],[-3.,0.])]:
  section=(struct.pack('<I',1)+pack([*axis,*center,*size])) if size[0]>0 else (struct.pack('<I',0)+pack([*center,-size[0],0.,0.,0.]));heap=0x410000;curve_snapshot=None;bp=0x180000;sp=0x17fc00;u.mem_write(bp-0x100,bytes(0x600));u.reg_write(UC_X86_REG_RBP,bp);u.reg_write(UC_X86_REG_RSP,sp);u.reg_write(UC_X86_REG_RBX,0x400000);info=0x401000;u.mem_write(info,bytes(80));u.mem_write(info+0x44,bytes([rank]));u.mem_write(bp+0x220,struct.pack('<Q',info+0x48));u.mem_write(bp+0x20,pack([0.,0.,0.,1.,*pos,1.,1.,1.,0.,0.]));xmm(UC_X86_REG_XMM8,[0.8,1.5,2.2][rank-1]);xmm(UC_X86_REG_XMM13,1.0)
  try:u.emu_start(0x1407c82ec,0x1407c8f04,count=100000)
  except Exception as e:raise RuntimeError(str(e)+' rank='+str(rank)+' trace='+str(seen)+' rip='+hex(u.reg_read(UC_X86_REG_RIP)))
  assert u.reg_read(UC_X86_REG_RIP)==0x1407c8f04
  count=int.from_bytes(u.mem_read(bp+0x130,8),'little');ptr=int.from_bytes(u.mem_read(bp+0x228,8),'little');data=bytes(u.mem_read(ptr,count*48));rows.append(dict(curve=curve_snapshot,rank=rank,shape=list(struct.unpack('<7I',section)),position=list(struct.unpack('<3I',pack(pos))),transforms=[list(struct.unpack('<10I',data[i:i+40])) for i in range(0,len(data),48)]))
Path(__file__).with_suffix('.json').write_text(json.dumps(dict(original_sha256=sha,scope=__doc__,crt_provider='host C float libm sinf/cosf/atan2f/hypotf/ceilf/fmodf; Windows CRT body not executed',range=['0x1407c82ec','0x1407c8f04'],rows=rows),indent=2));print(json.dumps({'cases':len(rows)}))

# 独立原孔写面：不替换quat乘法、bounds.extent或48B原constructor。
holes=[]
def xmm_vec(r,values):u.reg_write(r,int.from_bytes(pack(values),'little'))
for family in ['gothic','cottage']:
 for rank in [1,2,3]:
  for quat in [[0.,0.,0.,1.],[0.,0.6,0.,0.8],[0.6,0.,0.,0.8]]:
   bp=0x180000;sp=0x17fc00;u.mem_write(bp-0x100,bytes(0x600));u.reg_write(UC_X86_REG_RBP,bp);u.reg_write(UC_X86_REG_RSP,sp);position=[2.25,1.125,-3.5];wall=7
   if family=='gothic':
    bounds=[1.,0.,0.2,-0.3,0.8,0.7,0.,1.1]
    u.mem_write(bp+0x60,pack(bounds));u.mem_write(bp+0x10,pack(position+[0.]));u.mem_write(bp+0x198,pack([position[2]]));u.mem_write(bp+0x188,struct.pack('<Q',wall));info=0x401000;u.mem_write(info+0x44,bytes([rank]));u.mem_write(bp+0x1f0,struct.pack('<Q',info+0x48));xmm_vec(UC_X86_REG_XMM7,quat);xmm(UC_X86_REG_XMM13,0.);xmm(UC_X86_REG_XMM14,-.5);xmm(UC_X86_REG_XMM15,.05);xmm_vec(UC_X86_REG_XMM8,[-.5,-.5,0.,0.]);xmm_vec(UC_X86_REG_XMM9,[.05,.05,0.,0.]);start,end=0x1407c6af0,0x1407c6cf1
   else:
    bounds=[];u.mem_write(0x402000,pack(quat+position+[1.,1.,1.,0.,0.]));u.reg_write(UC_X86_REG_RSI,0x402000);u.mem_write(bp+0x100,struct.pack('<Q',wall));xmm(UC_X86_REG_XMM6,0.);xmm(UC_X86_REG_XMM10,0.);u.reg_write(UC_X86_REG_XMM11,int.from_bytes(u.mem_read(0x14292e6b0,8),'little'));xmm_vec(UC_X86_REG_XMM12,[0.,0.,1.,0.]);xmm_vec(UC_X86_REG_XMM14,[0.,0.,1.,0.]);xmm_vec(UC_X86_REG_XMM15,[.05,-.05,.05,0.]);start,end=0x1407c962c,0x1407c979f
   u.emu_start(start,end,count=10000);assert u.reg_read(UC_X86_REG_RIP)==end
   holes.append(dict(family=family,rank=rank,rotation=[*struct.unpack('<4I',pack(quat))],position=[*struct.unpack('<3I',pack(position))],bounds=[*struct.unpack('<8I',pack(bounds))] if bounds else [],words=[*struct.unpack('<12I',u.mem_read(bp+0x1a0,48))]))
Path(__file__).with_name('m01-dormer-holes-native.json').write_text(json.dumps(dict(original_sha256=sha,ranges={'gothic':['0x1407c6af0','0x1407c6cf1'],'cottage':['0x1407c962c','0x1407c979f']},rows=holes),indent=2));print(json.dumps({'hole_cases':len(holes)}))
