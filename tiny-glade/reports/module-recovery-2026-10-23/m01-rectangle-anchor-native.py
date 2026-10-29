"""执行B矩形装饰非角锚点原SSE片段；排除ECS/hash/allocator，未桥接业务算术。"""
from pathlib import Path
import struct, hashlib, itertools, json, pefile
from unicorn import Uc,UC_ARCH_X86,UC_MODE_64
from unicorn.x86_const import *
root=Path(__file__).resolve().parents[3]
raw=(root/'B版本内容/tiny-glade.rne').read_bytes(); digest=hashlib.sha256(raw).hexdigest()
assert digest=='f507a6cea25c5199447b88ba746a5293e64a279f73797ca74c1ce7f4ff2b7d03'
pe=pefile.PE(data=raw,fast_load=True);base=pe.OPTIONAL_HEADER.ImageBase;image=pe.get_memory_mapped_image()
u=Uc(UC_ARCH_X86,UC_MODE_64);u.mem_map(base,(len(image)+4095)&~4095);u.mem_write(base,image)
D=0x20000000;u.mem_map(D,0x10000)
def xmm(reg,values):u.reg_write(reg,int.from_bytes(struct.pack('<4f',*(list(values)+[0.]*(4-len(values)))),'little'))
def bits(values):return list(struct.unpack('<'+'I'*len(values),struct.pack('<'+'f'*len(values),*values)))
rows=[]
for axis,new_axis,center,delta,size,new_size,point in itertools.product(
 [(1.,0.),(.6,.8),(0.,-1.),(.3,.4)],[(1.,0.),(.8,.6)],[(0.,0.),(2.,-3.)],[(0.,0.),(1.,-.5)],
 [(10.,4.),(0.,4.)],[(12.,8.)],[(8.,0.),(0.,-9.),(3.,1.),(0.,0.),(float('nan'),2.)]):
 before=list(axis+center+size);after=list(new_axis+(center[0]+delta[0],center[1]+delta[1])+new_size)
 u.reg_write(UC_X86_REG_RSP,D+0x800)
 xmm(UC_X86_REG_XMM9,[axis[0],axis[1],-axis[1],axis[0]])
 xmm(UC_X86_REG_XMM10,[new_axis[0],new_axis[1],-new_axis[1],new_axis[0]])
 xmm(UC_X86_REG_XMM7,center);xmm(UC_X86_REG_XMM6,after[2:4]);xmm(UC_X86_REG_XMM8,size)
 u.emu_start(0x140819ef6,0x140819f5e,count=100)
 assert u.reg_read(UC_X86_REG_RIP)==0x140819f5e
 xmm(UC_X86_REG_XMM15,[point[0]]);xmm(UC_X86_REG_XMM14,[point[1]])
 u.mem_write(D+0x870,struct.pack('<2f',*new_size))
 u.emu_start(0x14081a174,0x14081a230,count=150)
 assert u.reg_read(UC_X86_REG_RIP)==0x14081a230
 result=list(struct.unpack('<2I',u.mem_read(D+0x840,8)))
 rows.append([bits(before),bits(after),bits(point),result])
fixture=Path(__file__).with_name('m01-rectangle-anchor-native-cases.rs')
fixture.write_text('&[\n'+''.join('('+','.join(str(x).replace(' ','')for x in row)+'),\n'for row in rows)+']\n')
Path(__file__).with_suffix('.json').write_text(json.dumps({'binary_sha256':digest,'ranges':['0x140819ef6..140819f5e','0x14081a174..14081a230'],'bridges':[],'resolved_inputs':'original before/after orientation_mat2 and immutable archive anchor, ECS/hash/allocator excluded','cases':len(rows),'fixture_sha256':hashlib.sha256(fixture.read_bytes()).hexdigest()},indent=2)+'\n')
print(json.dumps({'cases':len(rows),'fixture_sha256':hashlib.sha256(fixture.read_bytes()).hexdigest()}))
# 同一原版上执行旋转角计算：完整normalize/dot/acos多项式/π/sign片段。
angles=[]
for a,b in itertools.product([(1.,0.),(-1.,-0.),(0.,0.),(.6,.8),(.3,.4),(1e-12,-2e-12),(float('nan'),1.)],repeat=2):
 xmm(UC_X86_REG_XMM3,[a[0],b[0]])
 xmm(UC_X86_REG_XMM2,[a[1],b[1]])
 u.emu_start(0x140ad9472,0x140ad95a2,count=180)
 assert u.reg_read(UC_X86_REG_RIP)==0x140ad95a2
 angles.append([bits(a),bits(b),u.reg_read(UC_X86_REG_XMM2)&0xffffffff])
angle_fixture=Path(__file__).with_name('m01-rectangle-angle-native-cases.rs')
angle_fixture.write_text('&[\n'+''.join('('+','.join(str(x).replace(' ','')for x in row)+'),\n'for row in angles)+']\n')
meta=json.loads(Path(__file__).with_suffix('.json').read_text())
meta['angle']={'ranges':['0x140ad9472..140ad95a2'],'cases':len(angles),'bridges':[],
 'fixture_sha256':hashlib.sha256(angle_fixture.read_bytes()).hexdigest()}
Path(__file__).with_suffix('.json').write_text(json.dumps(meta,indent=2)+'\n')
print(json.dumps(meta['angle']))
