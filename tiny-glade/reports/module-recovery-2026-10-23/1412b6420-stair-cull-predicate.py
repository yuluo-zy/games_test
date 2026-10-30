"""原invalid请求端点扫描与缺缓存位置选择，不替代资源或predicate函数。"""
from pathlib import Path
import struct,json,hashlib,random,pefile
from unicorn import Uc,UC_ARCH_X86,UC_MODE_64,UC_HOOK_CODE
from unicorn.x86_const import *
root=Path(__file__).resolve().parents[3];raw=(root/'B版本内容/tiny-glade.rne').read_bytes();sha=hashlib.sha256(raw).hexdigest();assert sha=='f507a6cea25c5199447b88ba746a5293e64a279f73797ca74c1ce7f4ff2b7d03'
p=pefile.PE(data=raw,fast_load=True);base=p.OPTIONAL_HEADER.ImageBase;image=p.get_memory_mapped_image();u=Uc(UC_ARCH_X86,UC_MODE_64);u.mem_map(base,(len(image)+4095)&~4095);u.mem_write(base,image)
D,S=0x20000000,0x30000000;u.mem_map(D,0x20000);u.mem_map(S,0x100000);bp=D+0x18000;info=D+0x100;requests=D+0x200;data=D+0x1000
q=lambda a,v:u.mem_write(a,struct.pack('<Q',v))
def stop(uc,address,size,user):
 if address in (0x1412b65c8,0x1412b6800):uc.emu_stop()
u.hook_add(UC_HOOK_CODE,stop);rng=random.Random(0x1412b6420);rows=[]
values=[0,1,2,7,0x7fffffff,0x80000000,0xfffffffe,0xffffffff]
for i in range(1024):
 a,b=rng.choice(values),rng.choice(values);entries=[tuple(sorted((rng.choice(values),rng.choice(values))))for _ in range(i%7)]
 if entries and i%3==0:entries[i%len(entries)]=(min(a,b),max(a,b))
 if entries and i%3==1:entries[i%len(entries)]=(min(a,b),max(a,b))
 u.mem_write(D,b'\0'*0x20000);q(info,a|(b<<32));u.mem_write(info+0x22,b'\x04');q(requests+8,data);q(requests+16,len(entries));q(bp+0xb8,requests)
 for n,(x,y)in enumerate(entries):q(data+n*0x150+0x128,x|(y<<32))
 for r,v in [(UC_X86_REG_R14,info+0x48),(UC_X86_REG_R15,0),(UC_X86_REG_RBP,bp),(UC_X86_REG_RSP,S+0xf0000)]:u.reg_write(r,v)
 u.emu_start(0x1412b6788,0,count=10000);rip=u.reg_read(UC_X86_REG_RIP);assert rip in (0x1412b65c8,0x1412b6800)
 rows.append([a,b,len(entries),*[v for pair in entries for v in pair],int(rip==0x1412b6800)])
out=Path(__file__).with_suffix('.txt');out.write_text('\n'.join(' '.join(f'{v:08x}'for v in row)for row in rows)+'\n')
positions=[]
for tag in (0,1):
 for xyz in [(0,0,0),(0x80000000,0x447a0000,0x00000001),(0x3f800000,0xc0000000,0x40400000),(0x7f7fffff,0xff7fffff,0x00800000)]:
  u.mem_write(bp+0xd0,struct.pack('<4I',tag,*xyz));u.reg_write(UC_X86_REG_RBP,bp);u.reg_write(UC_X86_REG_XMM6,0x80000000);u.reg_write(UC_X86_REG_XMM7,0xc47a000080000000)
  u.emu_start(0x1412b7000,0x1412b6db9,count=1000);assert u.reg_read(UC_X86_REG_RIP)==0x1412b6db9
  positions.append([tag,*xyz,*struct.unpack('<3I',u.mem_read(bp+0x70,12))])
Path(__file__).with_suffix('.json').write_text(json.dumps({'binary_sha256':sha,'cases':len(rows),'fixture_sha256':hashlib.sha256(out.read_bytes()).hexdigest(),'ranges':['1412b6788..1412b65c8/1412b6800','1412b7000..1412b6db9'],'nested_actual':'140833470 dst_as_stair','bridges':[],'position_cases':positions},indent=2))
print(json.dumps({'predicate_cases':len(rows),'position_cases':len(positions)}))

Path(__file__).with_suffix('.positions.txt').write_text('\n'.join(' '.join(f'{v:08x}'for v in row)for row in positions)+'\n')
