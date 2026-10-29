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


rows=[]
for height_pair in [(0.,2.),(2.,0.),(1.,1.),(float("nan"),2.),(2.,float("nan"))]:
 for ta in range(3):
  for tb in range(3):
   for degrees in [(1,1),(0,1),(1,2),(2,2)]:
    for anch_heights in [(0.,.2),(.2,0.),(.13,.13),(float("nan"),.2)]:
     for flags in [(0,0),(1,1)]:
      sp=0x1ff008;u.mem_write(sp,struct.pack("<Q",0x100000));u.reg_write(UC_X86_REG_RSP,sp);u.mem_write(0x401000,pack([0.,height_pair[0],0.]));u.mem_write(0x402000,pack([0.,height_pair[1],0.]));nodes=bytearray(64)
      for i,tag in enumerate([ta,tb]):nodes[i*32:i*32+8]=struct.pack("<I",tag)+pack([anch_heights[i]]);nodes[i*32+24]=degrees[i]
      u.mem_write(0x403000,bytes(nodes));u.mem_write(0x404000,bytes([flags[0]])+bytes(19)+bytes([flags[1]])+bytes(19));u.reg_write(UC_X86_REG_RCX,0x401000);u.reg_write(UC_X86_REG_RDX,0x402000);u.reg_write(UC_X86_REG_R8,0x403000);u.reg_write(UC_X86_REG_R9,0x404000);u.emu_start(0x1407de800,0x100000,count=1000);assert u.reg_read(UC_X86_REG_RIP)==0x100000
      rows.append((list(struct.unpack("<2I",pack(height_pair))),[ta,tb],list(degrees),list(struct.unpack("<2I",pack(anch_heights))),list(flags),u.reg_read(UC_X86_REG_RAX)&255))
Path("/tmp/m01-ladder-support-native.json").write_text(json.dumps(dict(original_sha256=sha,scope="full167B1407de800; no bridges",rows=rows),indent=2));print("cases",len(rows),"outputs",sorted(set(r[-1] for r in rows)))
