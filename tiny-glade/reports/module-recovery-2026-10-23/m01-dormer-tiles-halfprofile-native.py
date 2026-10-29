"""B原Gothic瓦片完整函数；仅分配器、CRT、已解析CachedDecoratorTransforms查询与输出writer外部边界。"""
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

def hook(uc,ip,size,_):
 global heap
 if ip==0x141290c41:
  bp=uc.reg_read(UC_X86_REG_RBP);intermediates.append({"section":list(struct.unpack("<7I",uc.mem_read(bp-0x1c,28)))})
 if ip==0x141290cd9:
  bp=uc.reg_read(UC_X86_REG_RBP);intermediates[-1]["anchor"]=list(struct.unpack("<3f",uc.mem_read(bp+0x130,12)))
 seen.append(hex(ip));seen[:]=seen[-16:]
 if ip==0x140613c10:
  n=uc.reg_read(UC_X86_REG_RCX);heap=(heap+15)&~15;uc.reg_write(UC_X86_REG_RAX,heap);heap+=n;ret()
 elif ip==0x140d65850:
  old=uc.reg_read(UC_X86_REG_RCX);oldn=uc.reg_read(UC_X86_REG_RDX);n=uc.reg_read(UC_X86_REG_R9);heap=(heap+15)&~15;uc.mem_write(heap,bytes(uc.mem_read(old,min(n,oldn))));uc.reg_write(UC_X86_REG_RAX,heap);heap+=n;ret()
 elif ip==0x140613c20:ret()
 elif ip==0x140a1aa20:
  uc.mem_write(uc.reg_read(UC_X86_REG_RCX),transform);ret()
 elif ip==0x14128d0e0:
  records.append(bytes(uc.mem_read(uc.reg_read(UC_X86_REG_RDX),64)));uc.reg_write(UC_X86_REG_RAX,0);ret()
 elif ip==0x140b2a7d0:
  uc.mem_write(uc.reg_read(UC_X86_REG_RCX),struct.pack('<I',int(circle))+pack([0.,0.,2.,0.]));ret()
 elif ip==0x140b2a620:
  xmm(UC_X86_REG_XMM0,wall_max_y);ret()
 elif ip==0x140c16a00:uc.reg_write(UC_X86_REG_RAX,0);ret()
 elif ip in math_hooks:
  name=math_hooks[ip].lstrip('_');fn=fns.get(name)
  if fn is None:
   fn=getattr(libm,name);fn.argtypes=[ctypes.c_float]*(2 if name in ['atan2f','hypotf','powf','fmodf'] else 1);fn.restype=ctypes.c_float;fns[name]=fn
  a=scalar(UC_X86_REG_XMM0);b=scalar(UC_X86_REG_XMM1);xmm(UC_X86_REG_XMM0,fn(a,b) if len(fn.argtypes)==2 else fn(a));ret()
 if ip in [0x1428d9b00,0x1428d9a70]:raise RuntimeError('panic')
u.hook_add(UC_HOOK_CODE,hook)
def curve(addr,points):
 global heap
 n=len(points);ptr=heap;u.mem_write(ptr,pack([v for q in points for v in q]));heap+=n*12
 us=[0.];total=0.
 for a,b in zip(points,points[1:]):
  d=[f(y-x) for x,y in zip(a,b)];total=f(total+f(math.sqrt(f(f(f(d[1]*d[1])+f(d[0]*d[0]))+f(d[2]*d[2])))));us.append(total)
 us=[f(v/total) for v in us];uptr=heap;u.mem_write(uptr,pack(us));heap+=n*4
 u.mem_write(addr,struct.pack('<6QfI',n,ptr,n,n,uptr,n,total,0));return dict(points=points,points_u=us,length=total)
def roof(rect=True):
 b=bytearray(88);b[:8]=struct.pack('<Q',7);b[8:36]=struct.pack('<I',1 if rect else 0)+pack([1.,0.,0.,0.,8.,6.] if rect else [0.,0.,4.,0.,0.,0.]);b[0x24:0x40]=pack([0.,0.,.5,.6,0.,0.,0.]);b[0x4c:0x50]=pack([0.]);return bytes(b)

rows=[]
for rank in [1,2,3]:
 for circle in [False,True]:
  for subtype in [7,23]:
   for wall_max_y in [struct.unpack("<f",struct.pack("<I",0x3f75c290))[0],f(f(2.)+f(-0.46)),f(f(3.)+f(-0.46))]:
    heap=0x500000;sp=0x1ff008;u.mem_write(sp,struct.pack('<Q',0x100000)+bytes(0x100));center=[3.,2.,-1.];first=[2.,2.,-.5];second=[4.,2.,-1.5];ns=[.6,.8,1.,0.,.8,-.6]
    points=[[0.,1.26],[.2,1.09],[.35,.87],[.44,.62],[.48,.36],[.48,-.95]] if subtype==7 else [[0.,1.27],[.6,-.0]]
    u.mem_write(0x401000,struct.pack('<3Q',len(points),0x401100,len(points)));u.mem_write(0x401100,pack([v for q in points for v in q]));u.mem_write(0x402000,pack(center));u.mem_write(0x402100,pack(first));u.mem_write(0x402200,pack(second));u.mem_write(sp+0x28,struct.pack('<Q',0x402000));u.mem_write(sp+0x30,struct.pack('<Q',0x402200));u.mem_write(sp+0x38,pack([ns[2]]));u.mem_write(sp+0x40,pack([ns[3]]));u.mem_write(sp+0x48,pack([ns[4]]));u.mem_write(sp+0x50,pack([ns[5]]));u.mem_write(sp+0x58,struct.pack('<Q',0x402100));u.mem_write(sp+0x60,pack([ns[0]]));u.mem_write(sp+0x68,pack([ns[1]]));u.mem_write(sp+0x70,struct.pack('<Q',0x403000));u.reg_write(UC_X86_REG_RSP,sp);u.reg_write(UC_X86_REG_RCX,0x406000);u.reg_write(UC_X86_REG_RDX,rank);u.reg_write(UC_X86_REG_R8,subtype);u.reg_write(UC_X86_REG_R9,0x401000)
    try:u.emu_start(0x1407eb160,0x100000,count=1000000)
    except Exception as e:raise RuntimeError(str(e)+str(seen)+'rip='+hex(u.reg_read(UC_X86_REG_RIP)))
    profiles=[]
    for i in range(2):
     cap,ptr,n,ucap,uptr,un,length,_=struct.unpack('<6QII',u.mem_read(0x406000+i*56,56));profiles.append(dict(points=[list(struct.unpack('<3I',u.mem_read(ptr+j*12,12))) for j in range(n)],points_u=list(struct.unpack('<'+'I'*un,u.mem_read(uptr,un*4))),length=length))
    rows.append(dict(rank=rank,circle=circle,subtype=subtype,max_y=wall_max_y,profile=points,bones=dict(center=center,first=first,second=second,normals=ns),curves=profiles))
Path('/tmp/m01-halfprofile-native.json').write_text(json.dumps(dict(original_sha256=sha,scope='full1407eb160+1407e7dd0+1412ac9b0; allocator and resolved PublicWallState.is_circle/max_y only',rows=rows),indent=2));print('cases',len(rows))
