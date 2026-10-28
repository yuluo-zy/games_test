"""原FocusHeight几何段：完整执行626c8..6296a及两个原纯函数，无依赖桥。"""
from pathlib import Path
import hashlib,json,struct,random,math,pefile
from unicorn import Uc,UC_ARCH_X86,UC_MODE_64,UC_HOOK_CODE
from unicorn.x86_const import *
root=Path(__file__).resolve().parents[4]
raw=(root/'B版本内容/tiny-glade.rne').read_bytes();digest=hashlib.sha256(raw).hexdigest()
assert digest=='f507a6cea25c5199447b88ba746a5293e64a279f73797ca74c1ce7f4ff2b7d03'
pe=pefile.PE(data=raw,fast_load=True);base=pe.OPTIONAL_HEADER.ImageBase;image=pe.get_memory_mapped_image()
u=Uc(UC_ARCH_X86,UC_MODE_64);u.mem_map(base,(len(image)+4095)&~4095);u.mem_write(base,image)
data,stack=0x20000000,0x30000000
u.mem_map(data,0x8000);u.mem_map(stack,0x10000)
frame=stack+0x8000;entry=data+0x3000;camera=data+0x4000;points=data+0x1000
u.mem_write(data+0x200,struct.pack('<Q',data+0x208));u.mem_write(data+0x300,struct.pack('<Q',camera))
identity=[1.,0.,0.,0.,0.,1.,0.,0.,0.,0.,1.,0.,0.,0.,0.,1.]
observed={}
def hook(machine,address,size,opaque):
 if address==0x140a628a2:observed['eye_right']=list(struct.unpack('<III',machine.mem_read(frame+0x70,12)))
u.hook_add(UC_HOOK_CODE,hook)
def f32bits(x):return struct.unpack('<I',struct.pack('<f',x))[0]
def words(values):return [f32bits(v) for v in values]
rng=random.Random(0x464f435553)
rows=[]
for case in range(512):
 n=1+case%23
 pts=[[rng.randrange(-80,81)/4.,rng.randrange(-40,81)/4.,rng.randrange(-80,81)/4.] for _ in range(n)]
 maximum=rng.randrange(-4,81)/4.
 width=[0,1,1920,4294967295][case%4]
 view=identity.copy();view[12:15]=[rng.randrange(-16,17)/4. for _ in range(3)]
 projection=identity.copy()
 if case%3:
  projection=[1.25,0.,0.,0.,0.,1.75,0.,0.,0.,0.,-1.1,-1.,0.,0.,-0.2,0.]
 inv=identity.copy();inv[:3]=[[1.,0.,0.],[0.,0.,1.],[0.5,0.25,-1.],[0.,0.,0.]][case%4]
 if case<16:
  pts=[[-5.,2.,-2.],[-5.,-0.,3.],[-8.,1.,4.]]
  view=identity.copy();projection=identity.copy();maximum=[1.,10.,-0.,float('nan')][case%4]
 if 16<=case<32:
  pts=[[1.,float('nan'),2.],[1.,-0.,3.],[1.,0.,4.]]
 u.mem_write(points,b''.join(struct.pack('<fff',*p) for p in pts))
 u.mem_write(entry-0x258,struct.pack('<Q',len(pts)));u.mem_write(entry-0x260,struct.pack('<Q',points))
 u.mem_write(entry-0x198,struct.pack('<If',1,maximum));u.mem_write(data+0x208,struct.pack('<I',width))
 u.mem_write(camera,struct.pack('<16f',*projection));u.mem_write(camera+0x80,struct.pack('<16f',*view));u.mem_write(camera+0xc0,struct.pack('<16f',*inv))
 for register,value in [(UC_X86_REG_RBP,frame),(UC_X86_REG_RSP,frame-0x80),(UC_X86_REG_RSI,data+0x300),(UC_X86_REG_RDI,data+0x200),(UC_X86_REG_R15,entry)]:u.reg_write(register,value)
 observed.clear();u.emu_start(0x140a626c8,0x140a6296a,count=100000);assert u.reg_read(UC_X86_REG_RIP)==0x140a6296a
 offset=list(struct.unpack('<II',int(u.reg_read(UC_X86_REG_XMM9)).to_bytes(16,'little')[:8]))+[u.reg_read(UC_X86_REG_XMM10)&0xffffffff]
 rows.append({'points':[words(p) for p in pts],'maximum_y':f32bits(maximum),'width':width,'projection':words(projection),'view':words(view),'eye_right':observed['eye_right'],'target':list(struct.unpack('<III',u.mem_read(frame+8,12))),'offset':offset})
fixture=root/'tiny-glade/tests/fixtures/native/toolbar-focus-height-target.json';fixture.write_text(json.dumps(rows,separators=(',',':'))+'\n')
report={'binary_sha256':digest,'range':['0x140a626c8','0x140a6296a'],'included_original_callees':['0x140c96df0 CameraMatrices::eye_right','0x140c9cde0 swizzlers::xvy'],'bridges':[],'skipped_ranges':[],'resolved_inputs':'已验证公开墙的非空轮廓、已存在max_y缓存、相机矩阵、窗口宽度；HashMap查找与缓存就绪panic属于范围外','output':'target与offset逐float bits；NaN仅比较分类（跨ISA无有效几何载荷含义）。eye_right输入是原依赖实际输出，不宣称当前相机依赖全域差分','cases':len(rows),'fixture':str(fixture.relative_to(root)),'fixture_sha256':hashlib.sha256(fixture.read_bytes()).hexdigest(),'no_os_input':True}
Path(__file__).with_name('toolbar-focus-height-target-native.json').write_text(json.dumps(report,ensure_ascii=False,indent=2)+'\n');print(json.dumps(report,ensure_ascii=False))
