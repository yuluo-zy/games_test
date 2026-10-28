"""执行B两创建工具完整内联尺寸投影片段，不包含替换数学桥接。"""
from pathlib import Path
import itertools,struct,json,hashlib
import pefile
from unicorn import Uc,UC_ARCH_X86,UC_MODE_64
from unicorn.x86_const import *
root=Path(__file__).resolve().parents[4];raw=(root/'B版本内容/tiny-glade.rne').read_bytes()
sha=hashlib.sha256(raw).hexdigest();assert sha=='f507a6cea25c5199447b88ba746a5293e64a279f73797ca74c1ce7f4ff2b7d03'
pe=pefile.PE(data=raw,fast_load=True);image=pe.get_memory_mapped_image();base=pe.OPTIONAL_HEADER.ImageBase
u=Uc(UC_ARCH_X86,UC_MODE_64);u.mem_map(base,(len(image)+4095)&~4095);u.mem_write(base,image)
data=0x20000000;u.mem_map(data,0x10000)
def bits(x):return struct.unpack('<I',struct.pack('<f',x))[0]
def ron(value):
 if isinstance(value,(list,tuple)):return '('+','.join(map(ron,value))+')'
 return str(value)
circle=[];rectangle=[]
centers=[(0.,0.),(3.,4.),(-5.,7.)];planes=[0.,2.,-1.,float('nan')]
origins=[(10.,10.,6.),(0.,0.,0.),(float('inf'),10.,3.),(-float('inf'),-1.,2.),(3.,float('nan'),0.)]
directions=[(0.,-1.,0.),(.3,-.5,-.7),(0.,0.,0.),(float('nan'),1.,0.)]
axes=[(1.,0.),(0.,1.),(.6,.8),(float('nan'),0.),(0.,0.),(-1.,-0.)]
for center,plane,origin,direction in itertools.product(centers,planes,origins,directions):
 placement=bytearray(80);struct.pack_into('<f',placement,0x34,plane);u.mem_write(data,bytes(placement))
 ray=data+0x200;u.mem_write(ray,struct.pack('<6f',*(origin+direction)))
 preview=bytearray(48);struct.pack_into('<2f',preview,0x10,*center);u.mem_write(data+0x100,bytes(preview))
 for reg,value in [(UC_X86_REG_R13,data),(UC_X86_REG_RCX,ray),(UC_X86_REG_RBP,data+0x100)]:u.reg_write(reg,value)
 u.emu_start(0x14165c4eb,0x14165c54d,count=100);assert u.reg_read(UC_X86_REG_RIP)==0x14165c54d
 inputs=([bits(x)for x in center],bits(plane),[bits(x)for x in origin],[bits(x)for x in direction])
 circle.append(inputs+(u.reg_read(UC_X86_REG_XMM0)&0xffffffff,))
 for axis in axes:
  preview=bytearray(48);struct.pack_into('<2f',preview,0x10,*axis);struct.pack_into('<2f',preview,0x18,*center);struct.pack_into('<2f',preview,0x20,6.,6.);u.mem_write(data+0x100,bytes(preview))
  rsp=data+0x1000;u.mem_write(rsp+0x80,struct.pack('<Q',ray))
  for reg,value in [(UC_X86_REG_RSI,data),(UC_X86_REG_RSP,rsp),(UC_X86_REG_RBP,data+0x100)]:u.reg_write(reg,value)
  u.emu_start(0x14155d99f,0x14155da45,count=200);assert u.reg_read(UC_X86_REG_RIP)==0x14155da45
  result=u.reg_read(UC_X86_REG_XMM7)
  rectangle.append(inputs+([bits(x)for x in axis],[result&0xffffffff,(result>>32)&0xffffffff]))
fixtures=[]
for name,rows in [('circle',circle),('rectangle',rectangle)]:
 p=root/f'tiny-glade/tests/fixtures/native/toolbar-create-{name}-geometry.ron';fixtures.append(p)
 p.write_text('[\n'+''.join(ron(row)+',\n'for row in rows)+']\n')
 print(name,len(rows))
Path(__file__).with_name('toolbar-create-geometry-native.json').write_text(json.dumps({'binary_sha256':sha,'ranges':['14165c4eb..14165c54d','14155d99f..14155da45'],'bridges':[],'circle_cases':len(circle),'rectangle_cases':len(rectangle),'fixtures':{p.name:hashlib.sha256(p.read_bytes()).hexdigest()for p in fixtures},'scope':'原射线平面与半径/尺寸完整内联片段；状态机、边界、事件和工厂另外验收'},ensure_ascii=False,indent=2)+'\n')
