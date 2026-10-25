"""原初始化采样搜索循环；桥接已解析Spline位置，采样t/距离/首个最小值均执行原指令。"""
from pathlib import Path
import hashlib,json,struct,itertools,math
import pefile
from unicorn import Uc,UC_ARCH_X86,UC_MODE_64,UC_HOOK_CODE
from unicorn.x86_const import *
root=Path(__file__).resolve().parents[4];raw=(root/'B版本内容/tiny-glade.rne').read_bytes();sha=hashlib.sha256(raw).hexdigest()
assert sha=='f507a6cea25c5199447b88ba746a5293e64a279f73797ca74c1ce7f4ff2b7d03'
pe=pefile.PE(data=raw,fast_load=True);base=pe.OPTIONAL_HEADER.ImageBase;image=pe.get_memory_mapped_image()
u=Uc(UC_ARCH_X86,UC_MODE_64);u.mem_map(base,(len(image)+4095)&~4095);u.mem_write(base,image)
u.mem_map(0x20000000,0x10000);u.mem_map(0x30000000,0x10000)
rbp,rsp=0x30008000,0x30007008
f32=lambda n:struct.unpack('<f',struct.pack('<f',n))[0]
bits=lambda n:struct.unpack('<I',struct.pack('<f',n))[0]
def hook(uc,address,size,user):
 if address==0x142923e60:
  value=struct.unpack('<f',struct.pack('<I',uc.reg_read(UC_X86_REG_XMM0)&0xffffffff))[0]
  uc.reg_write(UC_X86_REG_XMM0,bits(float(math.ceil(value))))
  sp=uc.reg_read(UC_X86_REG_RSP);ra=struct.unpack('<Q',uc.mem_read(sp,8))[0];uc.reg_write(UC_X86_REG_RSP,sp+8);uc.reg_write(UC_X86_REG_RIP,ra)
 if address==0x140c946a0:
  t=struct.unpack('<f',struct.pack('<I',uc.reg_read(UC_X86_REG_XMM2)&0xffffffff))[0]
  # 一个已解析线性Spline3D段：P(t)=(3*t,0,0)。
  uc.mem_write(uc.reg_read(UC_X86_REG_RCX),struct.pack('<3f',f32(t*3),0,0))
  sp=uc.reg_read(UC_X86_REG_RSP);ra=struct.unpack('<Q',uc.mem_read(sp,8))[0];uc.reg_write(UC_X86_REG_RSP,sp+8);uc.reg_write(UC_X86_REG_RIP,ra)
u.hook_add(UC_HOOK_CODE,hook)
rows=[]
for max_t,px,py,pz in itertools.product([0.,0.2,0.21,1.,2.],[-3.,0.,1.5,3.,6.,9.],[0.,4.],[0.,-3.]):
 u.mem_write(0x20000100+0x1c,struct.pack('<f',max_t))
 u.mem_write(0x20000200,struct.pack('<QQQ',1,0x20001000,1))
 u.mem_write(0x20001010,struct.pack('<3f',px,py,pz))
 u.reg_write(UC_X86_REG_RSP,0x3000f008)
 for reg,value in [(UC_X86_REG_RCX,0x20000000),(UC_X86_REG_RDX,0x20000100),(UC_X86_REG_R8,0x20000200),(UC_X86_REG_R9,0)]:u.reg_write(reg,value)
 u.emu_start(0x1403b6b40,0x1403b6d46,count=10000)
 assert u.reg_read(UC_X86_REG_RIP)==0x1403b6d46
 rows.append(dict(count=u.reg_read(UC_X86_REG_R15),max_t=max_t,position=[px,py,pz],t_bits=u.reg_read(UC_X86_REG_XMM7)&0xffffffff))
fixture=root/'tiny-glade/tests/fixtures/native/toolbar-rollercoaster-initial-t.ron'
fixture.write_text('[\n'+''.join(f"({r['count']},{r['max_t']},({','.join(str(x) for x in r['position'])}),{r['t_bits']}),\n" for r in rows)+']\n')
result=dict(binary_sha256=sha,entry='0x1403b6b40',exit='0x1403b6d46',cases=len(rows),fixture_sha256=hashlib.sha256(fixture.read_bytes()).hexdigest(),bridges=['Spline3D::position_at_t: resolved linear P(t)=(3*t,0,0)', 'ceilf library boundary; original sample-count construction/cast/max2 executes'],scope='原count构造、采样距/顺序/除乘次序逐bit；不证明Spline求值或libm ceil实现以及非有限max_t全域。',rows=rows)
Path(__file__).with_name('toolbar-rollercoaster-initial-t-native.json').write_text(json.dumps(result,ensure_ascii=False,indent=2)+'\n');print(len(rows))
