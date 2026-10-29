"""B原3435B梯子生成；只桥接ceilf和WoodenPlank输出writer，保留全部几何与RNG指令。"""
from pathlib import Path
import struct,json,hashlib,pefile,math
from unicorn import *
from unicorn.x86_const import *
root=Path('/Users/yuluo/Desktop/game');raw=(root/'B版本内容/tiny-glade.rne').read_bytes();sha=hashlib.sha256(raw).hexdigest();assert sha=='f507a6cea25c5199447b88ba746a5293e64a279f73797ca74c1ce7f4ff2b7d03'
pe=pefile.PE(data=raw,fast_load=True);base=pe.OPTIONAL_HEADER.ImageBase;image=pe.get_memory_mapped_image();u=Uc(UC_ARCH_X86,UC_MODE_64);u.mem_map(base,(len(image)+4095)&~4095);u.mem_write(base,image);u.mem_map(0x100000,0x200000);u.mem_map(0x400000,0x800000)
def floats(xs):return struct.pack('<'+'f'*len(xs),*xs)
def words(raw,n):return [list(struct.unpack('<'+'I'*n,raw[i:i+4*n])) for i in range(0,len(raw),4*n)]
def ret():
 sp=u.reg_read(UC_X86_REG_RSP);ip=struct.unpack('<Q',u.mem_read(sp,8))[0];u.reg_write(UC_X86_REG_RSP,sp+8);u.reg_write(UC_X86_REG_RIP,ip)
records=[];columns=[];panicked=False
def hook(uc,ip,size,data):
 global panicked
 if ip==0x14128d0e0:
  p=u.reg_read(UC_X86_REG_RDX);records.append(list(struct.unpack('<16I',u.mem_read(p,64))));ret()
 elif ip==0x142923e60:
  x=struct.unpack('<f',u.reg_read(UC_X86_REG_XMM0).to_bytes(16,'little')[:4])[0];value=float(math.ceil(x)) if math.isfinite(x) else x;u.reg_write(UC_X86_REG_XMM0,int.from_bytes(floats([value])+bytes(12),'little'));ret()
 elif ip==0x14089d500:
  sp=u.reg_read(UC_X86_REG_RSP);pointers=[u.reg_read(r) for r in [UC_X86_REG_RDX,UC_X86_REG_R8,UC_X86_REG_R9]]+[struct.unpack('<Q',u.mem_read(sp+0x28,8))[0]];columns.append([words(bytes(u.mem_read(p,12)),3)[0] for p in pointers])
 elif ip==0x1428d9310:panicked=True;u.emu_stop()
u.hook_add(UC_HOOK_CODE,hook)
rows=[]
inputs=[([0.,0.,0.],[2.,3.,0.]),([1.,2.,3.],[-2.,-1.,5.]),([1.,-1.,2.],[1.,4.,5.]),([-2.,1.,1.],[3.,1.,-4.]),([-.25,.5,1.25],[.75,1.0,2.25])]
for start,end in inputs:
 for widths in [[1.,1.],[.5,2.],[3.,.25]]:
  for support in range(6):
   for seed in [0,0x123456789abcdef0]:
    sp=0x1ff008;u.mem_write(sp,struct.pack('<Q',0x100000));u.reg_write(UC_X86_REG_RSP,sp);u.mem_write(0x401000,floats(start+end));u.mem_write(0x402000,floats(widths));u.mem_write(0x403000,bytes([support])+bytes(15));u.mem_write(0x404000,struct.pack('<Q',seed));u.reg_write(UC_X86_REG_RCX,0x405000);u.reg_write(UC_X86_REG_RDX,0x401000);u.reg_write(UC_X86_REG_R8,2);u.reg_write(UC_X86_REG_R9,0x402000)
    for off,val in [(0x28,2),(0x30,0x403000),(0x38,42),(0x40,0x404000)]:u.mem_write(sp+off,struct.pack('<Q',val))
    records=[];columns=[];panicked=False;u.emu_start(0x1407db3e0,0x100000,count=100000);assert panicked or u.reg_read(UC_X86_REG_RIP)==0x100000,hex(u.reg_read(UC_X86_REG_RIP));rows.append(dict(start=words(floats(start),3)[0],end=words(floats(end),3)[0],widths=[r[0] for r in words(floats(widths),1)],support=support,seed=seed,final_seed=struct.unpack('<Q',u.mem_read(0x404000,8))[0],panic=panicked,records=records,columns=columns))
Path('/tmp/m01-stair-ladder-native.json').write_text(json.dumps(dict(original_sha256=sha,scope='full3435B1407db3e0; only CRT ceilf and actual WoodenPlank writer push resolved boundaries',rows=rows),indent=2))
print('cases',len(rows),'records',sum(len(r['records']) for r in rows))
for r in rows[:12]:print(r['support'],r['seed'],len(r['records']),[[[struct.unpack('<f',struct.pack('<I',x))[0] for x in col] for col in cols] for cols in r['columns'][:2]])
