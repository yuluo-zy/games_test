"""按B PDB inline_site精确起止执行，不桥接业务：输入EAX三角数，输出ECX图集索引。"""
from pathlib import Path
import struct,json,hashlib,pefile
from unicorn import *
from unicorn.x86_const import *
root=Path('/Users/yuluo/Desktop/game');raw=(root/'B版本内容/tiny-glade.rne').read_bytes();sha=hashlib.sha256(raw).hexdigest();assert sha=='f507a6cea25c5199447b88ba746a5293e64a279f73797ca74c1ce7f4ff2b7d03'
pe=pefile.PE(data=raw,fast_load=True);base=pe.OPTIONAL_HEADER.ImageBase;im=pe.get_memory_mapped_image();u=Uc(UC_ARCH_X86,UC_MODE_64);u.mem_map(base,(len(im)+4095)&~4095);u.mem_write(base,im)
cases=sorted(set(list(range(130))+[max(0,min(0xffffffff,(1<<n)+d)) for n in range(33) for d in [-2,-1,0,1,2]]));rows=[]
for count in cases:
 u.reg_write(UC_X86_REG_EAX,count);u.emu_start(0x1407dd80c,0x1407dd832,count=100);assert u.reg_read(UC_X86_REG_RIP)==0x1407dd832;rows.append([count,u.reg_read(UC_X86_REG_ECX)])
Path('/tmp/m01-stair-floor-atlas-native.json').write_text(json.dumps(dict(original_sha256=sha,scope='PDB inline floor_tri_count_to_atlas_idx physical1407dc170+5788,38B + 25B cold branch; no bridges',rows=rows),indent=2))
print('cases',len(rows),'indices',sorted(set(x[1] for x in rows)))
