"""原锁定dolly look_at实例执行；调用方单位化方向作为显式输入，无数学桥接。"""
from pathlib import Path
import hashlib,json,struct,itertools,math
import pefile
from unicorn import Uc,UC_ARCH_X86,UC_MODE_64
from unicorn.x86_const import *
root=Path(__file__).resolve().parents[4];raw=(root/'B版本内容/tiny-glade.rne').read_bytes();sha=hashlib.sha256(raw).hexdigest();assert sha=='f507a6cea25c5199447b88ba746a5293e64a279f73797ca74c1ce7f4ff2b7d03'
pe=pefile.PE(data=raw,fast_load=True);base=pe.OPTIONAL_HEADER.ImageBase;im=pe.get_memory_mapped_image();u=Uc(UC_ARCH_X86,UC_MODE_64);u.mem_map(base,(len(im)+4095)&~4095);u.mem_write(base,im)
data,stack=0x20000000,0x30000000;u.mem_map(data,0x10000);u.mem_map(stack,0x10000);rsp=stack+0x8008;end=data+0xf000
rows=[]
def f(v):return struct.unpack('<f',struct.pack('<f',v))[0]
def bits(v):return list(struct.unpack('<'+'I'*len(v),struct.pack('<'+'f'*len(v),*v)))
for xyz in itertools.product([-1.,-.2,0.,.3,1.],repeat=3):
 if xyz==(0.,0.,0.) or xyz[0]==xyz[2]==0:continue
 inv=f(1/f(math.sqrt(f(f(f(xyz[1]*xyz[1])+f(xyz[0]*xyz[0]))+f(xyz[2]*xyz[2])))))
 direction=[f(v*inv) for v in xyz];u.mem_write(data,struct.pack('<3f',*direction));u.mem_write(rsp,struct.pack('<Q',end));u.reg_write(UC_X86_REG_RSP,rsp);u.reg_write(UC_X86_REG_RCX,data+0x100);u.reg_write(UC_X86_REG_RDX,data)
 u.emu_start(0x140334d80,end,count=10000)
 rows.append((bits(direction),list(struct.unpack('<4I',u.mem_read(data+0x100,16)))))
fixture=root/'tiny-glade/tests/fixtures/native/toolbar-rollercoaster-look-at.ron';fixture.write_text('[\n'+''.join('('+','.join(str(x).replace(' ','') for x in row)+'),\n' for row in rows)+']\n')
out=dict(binary_sha256=sha,cases=len(rows),entry='0x140334d80',fixture_sha256=hashlib.sha256(fixture.read_bytes()).hexdigest(),bridges=[],scope='原dolly::util::look_at<RightHanded,Vec3,Quat>有限单位方向实例；排除退化零方向/垂直up共线，实际Spline输入另验')
Path(__file__).with_name('toolbar-rollercoaster-look-at-native.json').write_text(json.dumps(out,ensure_ascii=False,indent=2)+'\n');print(json.dumps(out,ensure_ascii=False))
