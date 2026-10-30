"""原RNE的自由墙标量平移环和邻边阈值分支；不宣称执行完整Bevy系统。"""
from pathlib import Path
import hashlib,json,struct
import pefile
from unicorn import Uc,UC_ARCH_X86,UC_MODE_64,UC_HOOK_CODE
from unicorn.x86_const import *
root=Path(__file__).resolve().parents[3]
raw=(root/'B版本内容/tiny-glade.rne').read_bytes()
sha=hashlib.sha256(raw).hexdigest()
assert sha=='f507a6cea25c5199447b88ba746a5293e64a279f73797ca74c1ce7f4ff2b7d03'
pe=pefile.PE(data=raw,fast_load=True);base=pe.OPTIONAL_HEADER.ImageBase
image=pe.get_memory_mapped_image();u=Uc(UC_ARCH_X86,UC_MODE_64)
u.mem_map(base,(len(image)+4095)&~4095);u.mem_write(base,image);data=0x200000;u.mem_map(data,4096)
def pair_bits(x):return list(struct.unpack('<II',struct.pack('<ff',*x)))
def simd(x):return int.from_bytes(struct.pack('<ff',*x)+bytes(8),'little')
def stop(uc,address,size,arg):
 if address in (0x140978f67,0x140acb077,0x140acb0b6): uc.emu_stop()
u.hook_add(UC_HOOK_CODE,stop)
translations=[]
for points,delta in [([[1.,2.],[3.,5.],[8.,7.]],[3.,4.]),([[-0.,0.],[16777216.,-16777216.]],[-0.,1.]),([[1e-30,-1e-30],[123.45,-8.25]],[-1e-30,0.3])]:
 u.mem_write(data,b''.join(struct.pack('<ff',*p) for p in points));u.reg_write(UC_X86_REG_RAX,data);u.reg_write(UC_X86_REG_R15,data+8*len(points));u.reg_write(UC_X86_REG_XMM8,simd(delta));u.emu_start(0x140978f50,0x140978f67,count=200)
 translations.append(dict(points=[pair_bits(p) for p in points],delta=pair_bits(delta),output=list(struct.unpack('<'+'I'*len(points)*2,u.mem_read(data,len(points)*8)))))
edges=[]
for target,neighbor in [([0.,0.],[0.1,0.]),([0.,0.],[0.099,0.]),([1.9,0.],[2.,0.]),([1.95,0.],[2.,0.]),([0.,0.],[0.,0.]),([0.,0.],[float('nan'),1.])]:
 u.mem_write(data,struct.pack('<ff',*neighbor));u.reg_write(UC_X86_REG_RAX,0);u.reg_write(UC_X86_REG_R13,1);u.reg_write(UC_X86_REG_R14,data);u.reg_write(UC_X86_REG_XMM13,simd(target));u.reg_write(UC_X86_REG_XMM10,simd([0.1,0.]));u.emu_start(0x140acb052,0x140acb0b7,count=100)
 edges.append(dict(target=pair_bits(target),neighbor=pair_bits(neighbor),rejected=u.reg_read(UC_X86_REG_RIP)==0x140acb0b6))
result=dict(original_sha256=sha,scope=__doc__,translation_range=['0x140978f50','0x140978f67'],neighbor_range=['0x140acb052','0x140acb077','0x140acb0b6'],translations=translations,edges=edges)
Path(__file__).with_suffix('.json').write_text(json.dumps(result,indent=2))
print(json.dumps(dict(translations=len(translations),edges=len(edges),original_sha256=sha)))
