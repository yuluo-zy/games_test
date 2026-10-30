"""原 B 箭孔端点到四元数的真实完整调用链；无业务桥接。"""
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
for first,second in itertools.product([(0.,0.,0.),(1.,2.,3.),(-1.,-2.,3.),(.3,.4,.7)],[(1.,2.,3.),(-4.,-2.,8.),(0.,2.,4.),(-1.,2.,-3.),(8.,2.,.25)]):
 u.reg_write(UC_X86_REG_RBP,D+0x2000);u.reg_write(UC_X86_REG_RSP,D+0xf008)
 write(D+0x21c0,first);write(D+0x21d8,second)
 u.emu_start(0x1407bc708,0x1407bc7d9,count=1500)
 assert u.reg_read(UC_X86_REG_RIP)==0x1407bc7d9
 rows.append([bits(first+second),out(D+0x21b0,4)])
p=Path(__file__).with_name('m01-arrow-pose-native-cases.rs')
p.write_text('&[\n'+''.join('('+','.join(str(x).replace(' ','')for x in row)+'),\n'for row in rows)+']\n')
meta={'binary_sha256':digest,'ranges':['0x1407bc708..1407bc7d9'],'callees':['0x140c9cdc0','0x140c911d0'], 'bridges':[], 'cases':len(rows),'fixture_sha256':hashlib.sha256(p.read_bytes()).hexdigest()}
Path(__file__).with_suffix('.json').write_text(json.dumps(meta,indent=2)+'\n');print(json.dumps(meta))
