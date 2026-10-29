"""原B矩形创建朝向分支与真实eye_right；不桥接相机归一化或朝向算法。"""
from pathlib import Path
import struct,json,hashlib,itertools,pefile
from unicorn import Uc,UC_ARCH_X86,UC_MODE_64
from unicorn.x86_const import *
root=Path(__file__).resolve().parents[3]
raw=(root/'B版本内容/tiny-glade.rne').read_bytes();sha=hashlib.sha256(raw).hexdigest()
assert sha=='f507a6cea25c5199447b88ba746a5293e64a279f73797ca74c1ce7f4ff2b7d03'
p=pefile.PE(data=raw,fast_load=True);base=p.OPTIONAL_HEADER.ImageBase;image=p.get_memory_mapped_image()
u=Uc(UC_ARCH_X86,UC_MODE_64);u.mem_map(base,(len(image)+4095)&~4095);u.mem_write(base,image)
D,S=0x20000000,0x30000000;u.mem_map(D,0x10000);u.mem_map(S,0x10000)
vectors=[(1.,0.,0.),(-1.,0.,0.),(0.,0.,1.),(0.,0.,-1.),(3.,4.,5.),(.6,-.7,-.8),
         (-0.,0.,1.),(1.,0.,-0.),(-0.,0.,-1.),(0.,1.,0.),(0.,-1.,0.),(0.,0.,0.)]
normals=[(1.,0.),(-1.,0.),(0.,1.),(0.,-1.),(-0.,1.),(1.,-0.),(-0.,-1.),(-1.,-0.),
         (.6,-.8),(3.,4.),(0.,0.),(-0.,-0.)]
def bits(values):return struct.unpack('<'+'I'*len(values),struct.pack('<'+'f'*len(values),*values))
rows=[]
for tag,surface,vector,normal in itertools.product(range(3),range(2),vectors,normals):
 u.mem_write(D,b'\0'*0x1000)
 camera=D+0x400;placement=D+0x100
 # 原CameraMatrices inverse_view位于+c0；其他列保持单位，最后列保留平移。
 matrix=[vector[0],vector[1],vector[2],0., 0.,1.,0.,0., 0.,0.,1.,0., 1.,2.,3.,1.]
 u.mem_write(camera+0xc0,struct.pack('<16f',*matrix));u.mem_write(D,struct.pack('<Q',camera))
 u.mem_write(placement+8,struct.pack('<I',surface));u.mem_write(placement+0x20,struct.pack('<Q',77))
 u.mem_write(placement+0x18,struct.pack('<2f',*normal));u.mem_write(placement+0x2c,bytes([tag]))
 u.reg_write(UC_X86_REG_R14,D);u.reg_write(UC_X86_REG_RSI,placement);u.reg_write(UC_X86_REG_RSP,S+0x8000)
 u.emu_start(0x14155d915,0x14155dae9,count=1000)
 out=u.reg_read(UC_X86_REG_XMM0)&0xffffffffffffffff
 rows.append([tag,surface,*bits(vector),*bits(normal),out&0xffffffff,out>>32])
out=Path(__file__).with_suffix('.txt');out.write_text('\n'.join(' '.join(f'{v:08x}'for v in row)for row in rows)+'\n')
Path(__file__).with_suffix('.json').write_text(json.dumps({'binary_sha256':sha,'range':['0x14155d915','0x14155dae9'],'original_callee':'0x140c96df0','cases':len(rows),'bridges':[],'fixture_sha256':hashlib.sha256(out.read_bytes()).hexdigest()},indent=2))
print(json.dumps({'cases':len(rows),'sha':sha}))
