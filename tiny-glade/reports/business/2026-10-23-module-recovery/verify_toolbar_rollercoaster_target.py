"""执行原空闲过山车的目标判定；raycast/HashMap查询按已解析对象边界桥接，保留游戏条件指令。"""
from pathlib import Path
import hashlib,json,struct,itertools
import pefile
from unicorn import Uc,UC_ARCH_X86,UC_MODE_64,UC_HOOK_CODE
from unicorn.x86_const import *
root=Path(__file__).resolve().parents[4]; raw=(root/'B版本内容/tiny-glade.rne').read_bytes();sha=hashlib.sha256(raw).hexdigest()
assert sha=='f507a6cea25c5199447b88ba746a5293e64a279f73797ca74c1ce7f4ff2b7d03'
pe=pefile.PE(data=raw,fast_load=True);base=pe.OPTIONAL_HEADER.ImageBase;image=pe.get_memory_mapped_image()
u=Uc(UC_ARCH_X86,UC_MODE_64);u.mem_map(base,(len(image)+4095)&~4095);u.mem_write(base,image)
data,stack=0x20000000,0x30000000;u.mem_map(data,0x10000);u.mem_map(stack,0x10000)
rbp,rsp=stack+0x8000,stack+0x7008
state={}
def ret(value=None):
 if value is not None:u.reg_write(UC_X86_REG_RAX,value)
 sp=u.reg_read(UC_X86_REG_RSP);ra=struct.unpack('<Q',u.mem_read(sp,8))[0];u.reg_write(UC_X86_REG_RSP,sp+8);u.reg_write(UC_X86_REG_RIP,ra)
def hook(uc,address,size,user):
 if address in [0x1403bae09,0x1403bb0c5]:
  uc.emu_stop();return
 if address==0x1408cc2e0:
  assert uc.reg_read(UC_X86_REG_XMM3)&0xffffffff==0x40800000
  assert struct.unpack('<I',uc.mem_read(uc.reg_read(UC_X86_REG_RSP)+0x28,4))[0]==0x101
  out=uc.reg_read(UC_X86_REG_RCX);hit=bytearray(64);struct.pack_into('<QQQ',hit,0,(7<<32)|state['source'],state['subtag'],0);struct.pack_into('<I',hit,0x34,state['miss']);uc.mem_write(out,bytes(hit));ret();return
 if address==0x140a1c930:
  assert uc.reg_read(UC_X86_REG_RDX)&0xffffffff==7
  ret(data+0x2800 if state['present'] else 0);return
 if address==0x1405b0ed0:
  ret(0);return
u.hook_add(UC_HOOK_CODE,hook)
base_case=dict(mode=19,camera=3,active=False,miss=0,source=4,subtag=2,present=True,ty=23,visual=1)
cases=[]
for ty in range(256):cases.append(dict(base_case,ty=ty))
for field,vals in dict(mode=[0,18,20],camera=[0,1,2,4,5],active=[True],miss=[2],source=[0,3,11],subtag=[0,1,3],present=[False],visual=[0,2]).items():
 for value in vals:cases.append(dict(base_case,**{field:value}))
rows=[]
for c in cases:
 state.update(c);u.mem_write(data,b'\0'*0x10000);u.mem_write(stack,b'\0'*0x10000)
 u.mem_write(0x143634cd9,b'\1')
 u.mem_write(rbp+0x348,struct.pack('<Q',data+0x100));u.mem_write(data+0x100,struct.pack('<I',c['mode']))
 u.mem_write(rbp+0x620,struct.pack('<Q',data+0x200));u.mem_write(data+0x200,struct.pack('<Q',data+0x1000))
 u.mem_write(data+0x14d0,bytes([c['camera']]))
 # camera inverse_view: real eye_position/forward leaf reads use identity.
 u.mem_write(data+0x10c0,struct.pack('<16f',1,0,0,0,0,1,0,0,0,0,1,0,0,0,0,1))
 u.mem_write(rbp+0x780,struct.pack('<Q',data+0x300));u.mem_write(data+0x300,struct.pack('<Q',8 if c['active'] else 0x8000000000000000))
 u.mem_write(rbp+0x360,struct.pack('<Q',data+0x400));u.mem_write(rbp+0x628,struct.pack('<Q',data+0x500))
 info=bytearray(72);struct.pack_into('<Q',info,0,(12<<32)|11);info[0x22]=4;info[0x46]=c['ty'];u.mem_write(data+0x2800,bytes(info))
 # Native hashbrown one bucket; actual probe and PersistentVisualEntities accessor execute.
 u.mem_write(data+0x518,struct.pack('<QQQQ',data+0x3000,7,0,1 if c['visual'] else 0))
 u.mem_write(data+0x3000,b'\0'+b'\xff'*15)
 u.mem_write(data+0x3000-40,struct.pack('<IIQQQQ',7,0,0,1,data+0x4000,1))
 u.mem_write(data+0x4000,struct.pack('<Q',(1<<32)|42 if c['visual']!=2 else 42))
 u.reg_write(UC_X86_REG_RBP,rbp);u.reg_write(UC_X86_REG_RSP,rsp)
 u.emu_start(0x1403bad37,0x1403bb0c5,count=10000)
 rip=u.reg_read(UC_X86_REG_RIP);assert rip in [0x1403bae09,0x1403bb0c5],hex(rip)
 prompt=u.mem_read(0x143634cd9,1)[0];assert prompt==int(rip==0x1403bb0c5)
 rows.append(dict(c,prompt=bool(prompt)))
fixture=root/'tiny-glade/tests/fixtures/native/toolbar-rollercoaster-target.ron'
fixture.write_text('[\n'+''.join('('+','.join(str(r[k]).lower() for k in ['mode','camera','active','miss','source','subtag','present','ty','visual','prompt'])+'),\n' for r in rows)+']\n')
out=dict(binary_sha256=sha,entry='0x1403bad37',exits=['0x1403bae09','0x1403bb0c5'],cases=len(rows),rows=rows,fixture_sha256=hashlib.sha256(fixture.read_bytes()).hexdigest(),bridges=['Raycasting::raycast resolved result; asserts max_toi4/mask0x101','DecoratorStorage::lookup_decorator_info resolved pointer; asserts DecoratorId7','HashMap hasher returns zero; actual one-bucket native hash probe still executes'],scope='完整空闲ride候选判定：256种ty和各拒绝前置分支；止于确认键之前。物理射线/地图查找作为已解析对象边界，不验证骑乘/轨道/音频/OS输入。')
Path(__file__).with_name('toolbar-rollercoaster-target-native.json').write_text(json.dumps(out,ensure_ascii=False,indent=2)+'\n')
print(json.dumps({k:out[k] for k in ['cases','fixture_sha256','scope']},ensure_ascii=False))
