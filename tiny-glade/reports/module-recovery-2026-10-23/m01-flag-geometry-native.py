"""原 B 墙旗/地旗真实SSE位姿片段与高度帽；无业务桥接。"""
from pathlib import Path
import struct,hashlib,json,itertools,pefile
from unicorn import Uc,UC_ARCH_X86,UC_MODE_64
from unicorn.x86_const import *
root=Path(__file__).resolve().parents[3]
raw=(root/'B版本内容/tiny-glade.rne').read_bytes()
digest=hashlib.sha256(raw).hexdigest()
assert digest=='f507a6cea25c5199447b88ba746a5293e64a279f73797ca74c1ce7f4ff2b7d03'
pe=pefile.PE(data=raw,fast_load=True);base=pe.OPTIONAL_HEADER.ImageBase;image=pe.get_memory_mapped_image()
u=Uc(UC_ARCH_X86,UC_MODE_64);u.mem_map(base,(len(image)+4095)&~4095);u.mem_write(base,image)
D=0x20000000;u.mem_map(D,0x10000)
def write(at,values):u.mem_write(at,struct.pack('<'+'f'*len(values),*values))
def xmm(reg,values):u.reg_write(reg,int.from_bytes(struct.pack('<4f',*(list(values)+[0.]*(4-len(values)))),'little'))
def bits(values):return list(struct.unpack('<'+'I'*len(values),struct.pack('<'+'f'*len(values),*values)))
def out(at,n):return list(struct.unpack('<'+'I'*n,u.mem_read(at,n*4)))
rows=[]
for q,p in itertools.product([(0.,0.,0.,1.),(0.,.6,0.,.8),(.1,.2,.3,.4),(.5,-.5,.5,.5)],[(0.,0.,0.),(1.,2.,3.),(-2.,-3.,4.),(1e-12,1e8,-1e8)]):
 for corner in [False,True]:
  u.reg_write(UC_X86_REG_RBP,D+0x2000);u.reg_write(UC_X86_REG_R8,12 if corner else 11)
  write(D+0x2290,q);write(D+0x22a0,p+(0.,));write(D+0x22b0,[1.,2.,3.,0.])
  u.emu_start(0x1408000f4,0x1408002dd,count=300)
  assert u.reg_read(UC_X86_REG_RIP)==0x1408002dd
  rows.append([0,int(corner),bits(q+p),out(D+0x1ff0,7)+out(D+0x2290,7)])
 # 地旗算术片段的ID/seed输入只供整数累加；不参与位姿。
 u.reg_write(UC_X86_REG_RBP,D+0x2000);xmm(UC_X86_REG_XMM6,q)
 write(D+0x22a0,p+(0.,));write(D+0x22b0,[1.,2.,3.,0.])
 u.mem_write(D+0x22f8,struct.pack('<Q',D+0x5000));u.mem_write(D+0x2310,struct.pack('<I',9))
 u.emu_start(0x140801910,0x140801a97,count=300)
 assert u.reg_read(UC_X86_REG_RIP)==0x140801a97
 rows.append([1,0,bits(q+p),out(D+0x2290,7)+[u.reg_read(UC_X86_REG_XMM6)&0xffffffff,u.reg_read(UC_X86_REG_XMM7)&0xffffffff]+[0]*5])
caps=[]
for h,y in itertools.product([-4.,-0.,0.,1.5,2.,5.,1e8,float('nan')],[-2.,-0.,0.,1.,2.,5.,float('nan')]):
 u.reg_write(UC_X86_REG_RBP,D+0x2000);xmm(UC_X86_REG_XMM6,[h]);write(D+0x2040,[y,0.,0.,0.])
 u.emu_start(0x140800856,0x14080087e,count=20)
 caps.append([bits([h,y]),u.reg_read(UC_X86_REG_XMM6)&0xffffffff])
p=Path(__file__).with_name('m01-flag-geometry-native-cases.rs')
p.write_text('(&[\n'+''.join('('+','.join(str(x).replace(' ','')for x in row)+'),\n'for row in rows)+'],&[\n'+''.join('('+str(a).replace(' ','')+','+str(b)+'),\n'for a,b in caps)+'])\n')
meta={'binary_sha256':digest,'ranges':['0x1408000f4..1408002dd','0x140801910..140801a97','0x140800856..14080087e'],'bridges':[], 'pose_cases':len(rows),'cap_cases':len(caps),'fixture_sha256':hashlib.sha256(p.read_bytes()).hexdigest()}
Path(__file__).with_suffix('.json').write_text(json.dumps(meta,indent=2)+'\n');print(json.dumps(meta))
