"""执行原create_cart_chain两侧射线几何；仅桥接已解析Rectangle extent和raycast记录。"""
from pathlib import Path
import hashlib,json,struct,itertools
import pefile
from unicorn import Uc,UC_ARCH_X86,UC_MODE_64,UC_HOOK_CODE
from unicorn.x86_const import *
root=Path(__file__).resolve().parents[4];raw=(root/'B版本内容/tiny-glade.rne').read_bytes();sha=hashlib.sha256(raw).hexdigest();assert sha=='f507a6cea25c5199447b88ba746a5293e64a279f73797ca74c1ce7f4ff2b7d03'
pe=pefile.PE(data=raw,fast_load=True);base=pe.OPTIONAL_HEADER.ImageBase;im=pe.get_memory_mapped_image();u=Uc(UC_ARCH_X86,UC_MODE_64);u.mem_map(base,(len(im)+4095)&~4095);u.mem_write(base,im)
data,stack=0x20000000,0x30000000;u.mem_map(data,0x10000);u.mem_map(stack,0x10000);rbp,rsp=stack+0x8000,stack+0x7008
state={}
def pack(v):return int.from_bytes(struct.pack('<4f',*(list(v)+[0.]*(4-len(v)))),'little')
def ret():
 sp=u.reg_read(UC_X86_REG_RSP);ra=struct.unpack('<Q',u.mem_read(sp,8))[0];u.reg_write(UC_X86_REG_RSP,sp+8);u.reg_write(UC_X86_REG_RIP,ra)
def hook(uc,address,size,user):
 if address==0x1403be2f0:uc.emu_stop();return
 if address==0x140c98660:
  uc.mem_write(uc.reg_read(UC_X86_REG_RCX),struct.pack('<3f',*state['extent']));ret();return
 if address==0x1408cc2e0:
  assert uc.reg_read(UC_X86_REG_XMM3)&0xffffffff==0x40000000
  assert struct.unpack('<I',uc.mem_read(uc.reg_read(UC_X86_REG_RSP)+0x28,4))[0]==0x101
  ptr=uc.reg_read(UC_X86_REG_R8);state['rays'].append(struct.unpack('<6I',uc.mem_read(ptr,24)))
  out=uc.reg_read(UC_X86_REG_RCX);uc.mem_write(out+0x34,struct.pack('<I',2));ret()
u.hook_add(UC_HOOK_CODE,hook)
rows=[]
quats=[(0,0,0,1),(0,.70710677,0,.70710677),(.2,.4,.6,.66),(-.3,.1,-.25,.91),(0,0,0,-1)]
for q,p,extent in itertools.product(quats,[(0,0,0),(1,-2,4),(-.1,2.7,-3.8)],[(2,1,3),(.7,1.8,1.1),(0,0,0)]):
 state.update(extent=extent,rays=[]);u.mem_write(stack,b'\0'*0x10000)
 u.reg_write(UC_X86_REG_RBP,rbp);u.reg_write(UC_X86_REG_RSP,rsp);u.reg_write(UC_X86_REG_R14,data);u.reg_write(UC_X86_REG_R13,data+256)
 u.reg_write(UC_X86_REG_XMM9,pack(q));u.reg_write(UC_X86_REG_XMM15,pack(p[:2]));u.reg_write(UC_X86_REG_XMM7,pack([p[2]]));u.reg_write(UC_X86_REG_XMM6,pack([2.]));u.reg_write(UC_X86_REG_XMM14,int.from_bytes(struct.pack('<4I',*[0x80000000]*4),'little'))
 u.emu_start(0x1403be6c6,0x1403be2f0,count=10000);assert len(state['rays'])==2
 rows.append((q,p,extent,state['rays']))
fixture=root/'tiny-glade/tests/fixtures/native/toolbar-rollercoaster-neighbor-rays.ron'
def bits(v):return list(struct.unpack('<'+'I'*len(v),struct.pack('<'+'f'*len(v),*v)))
fixture.write_text('[\n'+''.join('('+','.join(str(bits(x)).replace(' ','') for x in row[:3])+',['+','.join(str(list(r)).replace(' ','') for r in row[3])+']),\n' for row in rows)+']\n')
out=dict(binary_sha256=sha,cases=len(rows),entry='0x1403be6c6',exit='0x1403be2f0',bridges=['Rectangle3d::extent resolved extent','Raycasting::raycast only records native origin/direction; asserts max_toi2 and mask0x101; returns miss'],scope='原车Quaternion后乘Y四分之一转、两侧射线origin/direction及顺序；不覆盖碰撞求交和链遍历/ECS',fixture_sha256=hashlib.sha256(fixture.read_bytes()).hexdigest())
Path(__file__).with_name('toolbar-rollercoaster-neighbor-rays-native.json').write_text(json.dumps(out,ensure_ascii=False,indent=2)+'\n');print(json.dumps(out,ensure_ascii=False))
