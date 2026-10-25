"""执行 B 抬升目标算术与反馈选择原指令片段；不宣称完整系统差分。"""
from pathlib import Path
import json,hashlib,struct,itertools
import pefile
from unicorn import Uc,UC_ARCH_X86,UC_MODE_64
from unicorn.x86_const import *
root=Path(__file__).resolve().parents[4];raw=(root/'B版本内容/tiny-glade.rne').read_bytes()
sha=hashlib.sha256(raw).hexdigest();assert sha=='f507a6cea25c5199447b88ba746a5293e64a279f73797ca74c1ce7f4ff2b7d03'
pe=pefile.PE(data=raw,fast_load=True);image=pe.get_memory_mapped_image();base=pe.OPTIONAL_HEADER.ImageBase
u=Uc(UC_ARCH_X86,UC_MODE_64);u.mem_map(base,(len(image)+4095)&~4095);u.mem_write(base,image)
data=0x20000000;u.mem_map(data,0x10000)
def bits(x):return struct.unpack('<I',struct.pack('<f',x))[0]
values=[-float('inf'),-1.,-0.,0.,1.,30.,60.,100.,float('inf'),float('nan')]
target=[]
for original,projected,initial,floor in itertools.product(values,repeat=4):
 local=bytearray(28);struct.pack_into('<f',local,4,original);struct.pack_into('<f',local,12,initial);struct.pack_into('<f',local,24,floor)
 u.mem_write(data,bytes(local));u.mem_write(data+0x1000+0xb8,struct.pack('<f',projected))
 for reg,val in [(UC_X86_REG_RSI,data),(UC_X86_REG_RBP,data+0x1000),(UC_X86_REG_XMM12,bits(60.))]:u.reg_write(reg,val)
 u.emu_start(0x140955ce2,0x140955d24,count=100);assert u.reg_read(UC_X86_REG_RIP)==0x140955d24
 target.append(([bits(x)for x in [original,projected,initial,floor]],u.reg_read(UC_X86_REG_XMM7)&0xffffffff))
feedback=[]
for previous,current in itertools.product(values,repeat=2):
 for reg,val in [(UC_X86_REG_XMM11,bits(previous)),(UC_X86_REG_XMM7,bits(current)),(UC_X86_REG_XMM12,bits(60.)),(UC_X86_REG_XMM10,0),(UC_X86_REG_RSI,0)]:u.reg_write(reg,val)
 # 两个出口均在原片段内，code hook只停机，不替换业务指令。
 def stop(_,address,size,user):
  if address in [0x1409561ea,0x140956282]:u.emu_stop()
 from unicorn import UC_HOOK_CODE
 hook=u.hook_add(UC_HOOK_CODE,stop)
 u.emu_start(0x1409561a4,0,count=100);u.hook_del(hook)
 rip=u.reg_read(UC_X86_REG_RIP);assert rip in [0x1409561ea,0x140956282]
 kind=u.reg_read(UC_X86_REG_RSI)&255 if rip==0x1409561ea else 255
 feedback.append(([bits(previous),bits(current)],kind))
normal=[]
for xyz in itertools.product(values,repeat=3):
 dx,dy,dz=xyz
 u.reg_write(UC_X86_REG_XMM0,bits(dx)|(bits(dy)<<32))
 u.reg_write(UC_X86_REG_XMM1,bits(dz));u.reg_write(UC_X86_REG_XMM7,bits(1.))
 u.reg_write(UC_X86_REG_RBP,data+0x1000)
 u.emu_start(0x140954074,0x1409540d3,count=100);assert u.reg_read(UC_X86_REG_RIP)==0x1409540d3
 normal.append(([bits(x)for x in xyz],[u.reg_read(UC_X86_REG_XMM1)&0xffffffff,u.reg_read(UC_X86_REG_XMM2)&0xffffffff]))
normal_fixture=root/'tiny-glade/tests/fixtures/native/toolbar-raise-normal.ron'
normal_fixture.write_text('[\n'+''.join('(('+','.join(map(str,row))+'),('+','.join(map(str,out))+')),\n' for row,out in normal)+']\n')
fixture=root/'tiny-glade/tests/fixtures/native/toolbar-raise-target.ron'
fixture.write_text('[\n'+''.join('(('+','.join(map(str,row))+'),'+str(out)+'),\n'for row,out in target)+']\n')
ff=root/'tiny-glade/tests/fixtures/native/toolbar-raise-feedback.ron'
ff.write_text('[\n'+''.join('(('+','.join(map(str,row))+'),'+str(out)+'),\n'for row,out in feedback)+']\n')
Path(__file__).with_name('toolbar-raise-arithmetic-native.json').write_text(json.dumps({'binary_sha256':sha,'ranges':['140955ce2..140955d24','1409561a4..{1409561ea,140956282}','140954074..1409540d3'],'bridges':[],'target_cases':len(target),'feedback_cases':len(feedback),'normal_cases':len(normal),'fixture_sha256':{p.name:hashlib.sha256(p.read_bytes()).hexdigest()for p in [fixture,ff,normal_fixture]},'scope':'只执行命名纯数值片段，完整事件/历史/归档/投影函数另外验证；非NaN逐bit，NaN仅分类'},ensure_ascii=False,indent=2)+'\n')
print(len(target),len(feedback))
