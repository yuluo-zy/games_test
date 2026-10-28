"""完整执行 B 原投影174B；零桥接，保留负 w 与非仿射 view.w 的行为。"""
from pathlib import Path
import json,hashlib,struct,itertools
import pefile
from unicorn import Uc,UC_ARCH_X86,UC_MODE_64
from unicorn.x86_const import *
root=Path(__file__).resolve().parents[4]
raw=(root/'B版本内容/tiny-glade.rne').read_bytes()
sha=hashlib.sha256(raw).hexdigest()
assert sha=='f507a6cea25c5199447b88ba746a5293e64a279f73797ca74c1ce7f4ff2b7d03'
pe=pefile.PE(data=raw,fast_load=True);image=pe.get_memory_mapped_image();base=pe.OPTIONAL_HEADER.ImageBase
u=Uc(UC_ARCH_X86,UC_MODE_64);u.mem_map(base,(len(image)+4095)&~4095);u.mem_write(base,image)
data,stack,stop=0x20000000,0x30000000,0x20005000
u.mem_map(data,0x6000);u.mem_map(stack,0x10000)
def bits(f):return struct.unpack('<I',struct.pack('<f',f))[0]
rows=[]
# 列主序；view.w故意非仿射以检验原投影忽略第一次变换w的特殊合同。
for point,wh,w in itertools.product([(0.,0.,0.),(1.,-2.,3.),(-5.,7.,-11.)],[(1200.,800.),(0.,0.),(200.,100.)],[1.,-1.,0.,2.]):
 projection=[1.,0.,0.,0., 0.,2.,0.,0., 0.,0.,1.,0., .5,-.25,0.,w]
 view=[2.,0.,0.,4., 0.,3.,0.,5., 0.,0.,.5,6., 1.,2.,3.,7.]
 camera=bytearray(0x100);struct.pack_into('<16f',camera,0,*projection);struct.pack_into('<16f',camera,0x80,*view)
 u.mem_write(data,struct.pack('<3f',*point));u.mem_write(data+0x100,bytes(camera));rsp=stack+0xf008;u.mem_write(rsp,struct.pack('<Q',stop))
 for reg,val in [(UC_X86_REG_RSP,rsp),(UC_X86_REG_RCX,data),(UC_X86_REG_R9,data+0x100),(UC_X86_REG_XMM1,bits(wh[0])),(UC_X86_REG_XMM2,bits(wh[1]))]:u.reg_write(reg,val)
 u.emu_start(0x140ac8780,stop,count=500);assert u.reg_read(UC_X86_REG_RIP)==stop
 result=[u.reg_read(UC_X86_REG_XMM0)&0xffffffff,u.reg_read(UC_X86_REG_XMM1)&0xffffffff]
 rows.append(([bits(x) for x in projection],[bits(x) for x in view],[bits(x) for x in point],[bits(x) for x in wh],result))
fixture=root/'tiny-glade/tests/fixtures/native/toolbar-demo-projection.ron'
fixture.write_text('[\n'+''.join('('+','.join('('+','.join(map(str,v))+')' for v in row)+'),\n' for row in rows)+']\n')
Path(__file__).with_name('toolbar-demo-projection-native.json').write_text(json.dumps({'binary_sha256':sha,'function':'0x140ac8780','range_end_exclusive':'0x140ac882e','bridges':[],'cases':len(rows),'fixture_sha256':hashlib.sha256(fixture.read_bytes()).hexdigest(),'scope':'原投影完整原指令有限输入；不包括原Demo系统ECS、采样依赖、GPU或OS输入'},ensure_ascii=False,indent=2)+'\n')
print('original projection cases',len(rows))
