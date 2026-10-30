"""执行原selected FlatMap闭包的中心、旋转与box实参；不执行Bevy或替换原旋转算法。"""
from pathlib import Path
import hashlib,json,struct
import pefile
from unicorn import Uc,UC_ARCH_X86,UC_MODE_64,UC_HOOK_CODE
from unicorn.x86_const import *
root=Path(__file__).resolve().parents[3];raw=(root/'B版本内容/tiny-glade.rne').read_bytes();sha=hashlib.sha256(raw).hexdigest();assert sha=='f507a6cea25c5199447b88ba746a5293e64a279f73797ca74c1ce7f4ff2b7d03'
p=pefile.PE(data=raw,fast_load=True);base=p.OPTIONAL_HEADER.ImageBase;image=p.get_memory_mapped_image();u=Uc(UC_ARCH_X86,UC_MODE_64);u.mem_map(base,(len(image)+4095)&~4095);u.mem_write(base,image);u.mem_map(0x100000,0x100000);u.mem_map(0x300000,0x10000)
rows=[]
def bits(v):return list(struct.unpack('<'+'I'*len(v),struct.pack('<'+'f'*len(v),*v)))
for point,direction,outward,height in [([-2.,-3.],[1.,0.],[0.,-1.],3.),([-2.,-3.],[0.,1.],[-1.,0.],3.),([4.25,-8.5],[0.6,0.8],[0.8,-0.6],-2.75),([1000.,-1000.],[-0.8,0.6],[-0.6,-0.8],13.125),([0.,0.],[0.,0.],[0.,0.],0.)]:
 sp=0x180000;u.reg_write(UC_X86_REG_RSP,sp);u.mem_write(0x300000,struct.pack('<ff',*point));u.mem_write(0x300100,struct.pack('<ffff',*direction,*outward));u.mem_write(0x300200,struct.pack('<f',height));u.reg_write(UC_X86_REG_RCX,0x300000);u.reg_write(UC_X86_REG_RAX,0x300100);u.reg_write(UC_X86_REG_RDX,0x300200)
 u.emu_start(0x1412f1c0f,0x1412f1cb1,count=4000)
 assert u.reg_read(UC_X86_REG_RIP)==0x1412f1cb1
 transform=struct.unpack('<10I',u.mem_read(sp+0x40,40))
 dimensions=[u.reg_read(r)&0xffffffff for r in (UC_X86_REG_XMM1,UC_X86_REG_XMM2,UC_X86_REG_XMM3)]
 rows.append(dict(point=bits(point),direction=bits(direction),outward=bits(outward),height=bits([height])[0],transform=list(transform),dimensions=dimensions))
Path(__file__).with_suffix('.json').write_text(json.dumps(dict(original_sha256=sha,scope=__doc__,range=['0x1412f1c0f','0x1412f1cb1'],cases=rows),indent=2))
print(json.dumps(dict(cases=len(rows),original_sha256=sha)))
