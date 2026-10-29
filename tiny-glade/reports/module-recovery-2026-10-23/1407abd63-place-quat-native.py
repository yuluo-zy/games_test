"""原Place四元数复合内联完整kernel，独立执行并保留原128位header门样本。"""
from pathlib import Path
import hashlib,json,random,struct,pefile
from unicorn import Uc,UC_ARCH_X86,UC_MODE_64
from unicorn.x86_const import *
root=Path(__file__).resolve().parents[3];raw=(root/'B版本内容/tiny-glade.rne').read_bytes()
sha=hashlib.sha256(raw).hexdigest();assert sha=='f507a6cea25c5199447b88ba746a5293e64a279f73797ca74c1ce7f4ff2b7d03'
p=pefile.PE(data=raw,fast_load=True);base=p.OPTIONAL_HEADER.ImageBase;image=p.get_memory_mapped_image()
u=Uc(UC_ARCH_X86,UC_MODE_64);u.mem_map(base,(len(image)+4095)&~4095);u.mem_write(base,image)
D=0x20000000;u.mem_map(D,0x10000);bp=D+0x1000;u.reg_write(UC_X86_REG_RBP,bp)
rng=random.Random(0x1407abd63);inputs=[]
for _ in range(512):inputs.append(([rng.uniform(-5,5)for _ in range(4)],[rng.uniform(-5,5)for _ in range(4)]))
for sign in range(16):
 q=[-0.0 if sign>>i&1 else 0.0 for i in range(4)]
 inputs.extend([(q,[0.,0.,0.,1.]),([0.,0.,0.,1.],q),(q,q)])
rows=[]
for relative,camera in inputs:
    rb=struct.pack('<4f',*relative);cb=struct.pack('<4f',*camera)
    u.mem_write(bp+0x5e0,cb);u.reg_write(UC_X86_REG_XMM6,int.from_bytes(rb,'little'))
    u.emu_start(0x1407abd63,0x1407abdc3,count=1000)
    output=bytes(u.mem_read(bp+0x5e0,16))
    rows.append(' '.join(f'{v:08x}'for v in struct.unpack('<12I',rb+cb+output)))
out=Path(__file__).with_suffix('.txt');out.write_text('\n'.join(rows)+'\n')
gates=[];mask=(1<<64)-1
for header in range(7):
    u.reg_write(UC_X86_REG_RAX,(header-2)&mask);u.reg_write(UC_X86_REG_RCX,mask if header<2 else 0)
    u.emu_start(0x1407aae14,0x1407aae30,count=100)
    gates.append({'valid_header':header,'common_gate':u.reg_read(UC_X86_REG_RAX)&0xff})
Path(__file__).with_suffix('.json').write_text(json.dumps({'binary_sha256':sha,'cases':len(rows),'bridges':[],'ranges':['1407abd63..1407abdc3 original complete inline quaternion kernel','1407aae14..1407aae30 original gate after u128 subtract2'],'valid_header_gates':gates,'fixture_sha256':hashlib.sha256(out.read_bytes()).hexdigest(),'boundary':'finite quaternion kernel and valid header0..6; no complete UI acceptance'},indent=2))
print(len(rows),gates)
