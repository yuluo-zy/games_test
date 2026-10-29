"""B原RoofTriggerVolume getter与完整世界floor三角映射collector；仅分配器桥，无CRT/业务结果替身。"""
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


class OriginalPanic(Exception):pass

def hook(uc,ip,size,_):
 global heap
 if ip==0x140613c10:
  n=uc.reg_read(UC_X86_REG_RCX);heap=(heap+15)&~15;uc.reg_write(UC_X86_REG_RAX,heap);heap+=n;ret()
 elif ip==0x1428d9430:raise OriginalPanic('empty original volume')
 elif ip==0x1428d8fe3:raise OriginalPanic('allocation')
u.hook_add(UC_HOOK_CODE,hook)
rows=[]
floors=[0,0x80000000,0x40133333,0xc1880000,1,0x7f7fffff,0x7f800000,0xff800000,0x7fc12345]
points=[[0,0x3f800000,0x80000000],[0x40000000,0xc0600000,0x41200000],[0x80000001,0x3e4ccccd,1]]
for count in [0,1,2,3,4,5]:
 for floor in floors:
  heap=0x500000;sp=0x1ff008;u.mem_write(sp,struct.pack('<Q',0x100000));u.reg_write(UC_X86_REG_RSP,sp)
  triangles=[points[(i+j)%3] for i in range(count) for j in range(3)]
  u.mem_write(0x401000,struct.pack('<3Q',count,0x402000,count));u.mem_write(0x402000,struct.pack('<'+'I'*(len(triangles)*3),*[v for q in triangles for v in q]))
  u.reg_write(UC_X86_REG_RCX,0x403000);u.reg_write(UC_X86_REG_RDX,0x401000);u.reg_write(UC_X86_REG_XMM2,floor)
  try:
   u.emu_start(0x1408e4560,0x100000,count=10000);assert u.reg_read(UC_X86_REG_RIP)==0x100000
   u.mem_write(sp,struct.pack('<Q',0x100000));u.reg_write(UC_X86_REG_RSP,sp);u.reg_write(UC_X86_REG_RCX,0x404000);u.reg_write(UC_X86_REG_RDX,0x403000);u.reg_write(UC_X86_REG_R8,0x142aa3590)
   u.emu_start(0x140dce1d0,0x100000,count=10000);assert u.reg_read(UC_X86_REG_RIP)==0x100000
   cap,ptr,length=struct.unpack('<3Q',u.mem_read(0x404000,24));assert length==count;result={'status':'value','bits':list(struct.unpack('<'+'I'*(count*9),u.mem_read(ptr,count*36)))}
  except OriginalPanic as e:result={'status':'panic','why':str(e)}
  rows.append(dict(count=count,floor=floor,input=[v for q in triangles for v in q],expected=result))
Path('/tmp/m01-roof-floor-native.json').write_text(json.dumps(dict(original_sha256=sha,scope='full original getter1408e4560 and full triangles collector140dce1d0; only allocator external boundary; no math bridge',rows=rows),indent=2));print('floor cases',len(rows))
