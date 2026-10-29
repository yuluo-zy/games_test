"""原阳台门两个采样后数值段，真实140c911d0，无算法桥。"""
from pathlib import Path
import struct,hashlib,json,random,pefile
from unicorn import Uc,UC_ARCH_X86,UC_MODE_64
from unicorn.x86_const import *
root=Path(__file__).resolve().parents[3];raw=(root/'B版本内容/tiny-glade.rne').read_bytes();sha=hashlib.sha256(raw).hexdigest();assert sha=='f507a6cea25c5199447b88ba746a5293e64a279f73797ca74c1ce7f4ff2b7d03'
p=pefile.PE(data=raw,fast_load=True);base=p.OPTIONAL_HEADER.ImageBase;image=p.get_memory_mapped_image();u=Uc(UC_ARCH_X86,UC_MODE_64);u.mem_map(base,(len(image)+4095)&~4095);u.mem_write(base,image)
D,S=0x20000000,0x30000000;u.mem_map(D,0x10000);u.mem_map(S,0x100000);bp=D+0x8000
f=lambda a:struct.pack('<'+'f'*len(a),*a)
def xmm(r,v):u.reg_write(r,int.from_bytes(f(v+[0.]*(4-len(v))),'little'))
def uv(r):return struct.pack('<QQ',u.reg_read(r)&((1<<64)-1),u.reg_read(r)>>64)
def bits(v):return struct.unpack('<'+'I'*len(v),f(v))
rng=random.Random(0x1407f3da0);pairs=[]
for _ in range(1000):pairs.append(([rng.uniform(-130,130)for _ in range(3)],[rng.uniform(-130,130)for _ in range(3)]))
for x,z in [(0.,0.),(-0.,0.),(0.,-0.),(-0.,-0.),(1.,0.),(-1.,0.),(0.,1.),(0.,-1.),(1e-20,1e-20),(1e20,-1e20)]:
 for y in [0.,-0.,1.,-2.5]:pairs.append(([0.,y,0.],[x,y,z]))
rows=[]
for first,second in pairs:
 for mode in [0,1]:
  u.mem_write(D,b'\0'*0x10000);u.reg_write(UC_X86_REG_RBP,bp);u.reg_write(UC_X86_REG_RSP,S+0xf0000)
  if mode==0:
   u.mem_write(bp+0x180,f([second[0],0.,second[2]]));xmm(UC_X86_REG_XMM12,[first[0]]);xmm(UC_X86_REG_XMM11,[first[2]]);xmm(UC_X86_REG_XMM8,[0.,first[1]]);xmm(UC_X86_REG_XMM9,[.5,.5,.5,.5]);xmm(UC_X86_REG_XMM10,[1.])
   u.emu_start(0x1407f459b,0x1407f4648,count=10000);assert u.reg_read(UC_X86_REG_RIP)==0x1407f4648
   result=bytes(u.mem_read(bp+0x48,12))+bytes(u.mem_read(bp+0x180,16))
  else:
   xmm(UC_X86_REG_XMM13,first[:2]);xmm(UC_X86_REG_XMM12,[first[2]]);xmm(UC_X86_REG_XMM14,second[:2]);xmm(UC_X86_REG_XMM7,[second[2]])
   u.emu_start(0x1407f10c7,0x1407f1142,count=10000);assert u.reg_read(UC_X86_REG_RIP)==0x1407f1142;rotation=bytes(u.mem_read(bp+0x1e0,16))
   u.emu_start(0x1407f11a8,0x1407f11f9,count=10000);assert u.reg_read(UC_X86_REG_RIP)==0x1407f11f9;xz=uv(UC_X86_REG_XMM10)
   result=xz[:4]+bytes(u.mem_read(bp+0xa0,4))+xz[4:8]+rotation
  rows.append([mode,*bits(first+second),*struct.unpack('<7I',result)])
out=Path(__file__).with_suffix('.txt');out.write_text('\n'.join(' '.join(f'{v:08x}'for v in row)for row in rows)+'\n')
Path(__file__).with_suffix('.json').write_text(json.dumps({'binary_sha256':sha,'cases':len(rows),'bridges':[],'ranges':['1407f459b..1407f4648','1407f10c7..1407f1142','1407f11a8..1407f11f9'],'nested':'140c911d0 complete','fixture_sha256':hashlib.sha256(out.read_bytes()).hexdigest()},indent=2))
print(len(rows))
