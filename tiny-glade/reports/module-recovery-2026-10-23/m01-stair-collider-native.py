"""B原1407de8b0全部循环/几何；只在Parry new_convex已解析边界捕获8点并返回None。"""
from pathlib import Path
import struct,json,hashlib,pefile
from unicorn import *
from unicorn.x86_const import *
root=Path('/Users/yuluo/Desktop/game');raw=(root/'B版本内容/tiny-glade.rne').read_bytes();sha=hashlib.sha256(raw).hexdigest();assert sha=='f507a6cea25c5199447b88ba746a5293e64a279f73797ca74c1ce7f4ff2b7d03'
pe=pefile.PE(data=raw,fast_load=True);base=pe.OPTIONAL_HEADER.ImageBase;image=pe.get_memory_mapped_image();u=Uc(UC_ARCH_X86,UC_MODE_64);u.mem_map(base,(len(image)+4095)&~4095);u.mem_write(base,image);u.mem_map(0x100000,0x200000);u.mem_map(0x400000,0x800000)
def floats(xs):return struct.pack('<'+'f'*len(xs),*xs)
def words(raw,n):return [list(struct.unpack('<'+'I'*n,raw[i:i+4*n])) for i in range(0,len(raw),4*n)]
def ret():
 sp=u.reg_read(UC_X86_REG_RSP);ip=struct.unpack('<Q',u.mem_read(sp,8))[0];u.reg_write(UC_X86_REG_RSP,sp+8);u.reg_write(UC_X86_REG_RIP,ip)
captured=[];panicked=False
def hook(uc,ip,size,data):
 global panicked
 if ip==0x1408cec10:
  out=u.reg_read(UC_X86_REG_RCX);p=u.reg_read(UC_X86_REG_RDX);n=u.reg_read(UC_X86_REG_R8);assert n==8;captured.append(words(bytes(u.mem_read(p,12*n)),3));u.mem_write(out,bytes(128));ret()
 elif ip in [0x1428d9430,0x1428d9518]:panicked=True;u.emu_stop()
u.hook_add(UC_HOOK_CODE,hook)
rows=[]
cases=[(n,n,n,step,False) for n in range(10) for step in [0,1,2,3,5]]
cases += [(9,7,5,s,False) for s in [1,2,3,5,8]]+[(5,9,8,s,False) for s in [1,2,4,8]]+[(n,n,n,s,True) for n in [2,5,9] for s in [1,2,3]]
for lengths in cases:
 np,nt,nw,step,zeros=lengths
 points=[[(-0.0 if i%2 else 0.0) if zeros else i*.4375-1.25, i*.28125-0.0625, i*i*.125-2.25] for i in range(np)]
 tangents=[[(-0.0 if i%2 else 0.0) if zeros else -.75+i*.125,(-0.0 if i%2 else 0.0) if zeros else .25+i*.1875] for i in range(nt)]
 widths=[0.0 if zeros else .75+i*.21875 for i in range(nw)]
 sp=0x1ff008;u.mem_write(sp,struct.pack('<Q',0x100000));u.reg_write(UC_X86_REG_RSP,sp)
 for addr,vals in [(0x401000,sum(points,[])),(0x402000,sum(tangents,[])),(0x403000,widths)]:u.mem_write(addr,floats(vals) or bytes(16))
 for addr,ptr in [(0x404000,0x410000),(0x405000,0x440000),(0x406000,0x470000)]:u.mem_write(addr,struct.pack('<3Q',4096,ptr,0))
 u.reg_write(UC_X86_REG_RCX,0x401000);u.reg_write(UC_X86_REG_RDX,np);u.reg_write(UC_X86_REG_R8,0x402000);u.reg_write(UC_X86_REG_R9,nt)
 for off,val in [(0x28,0x403000),(0x30,nw),(0x38,step),(0x40,0x404000),(0x48,0x405000),(0x50,0x406000)]:u.mem_write(sp+off,struct.pack('<Q',val))
 captured=[];panicked=False
 u.emu_start(0x1407de8b0,0x100000,count=1000000);assert panicked or u.reg_read(UC_X86_REG_RIP)==0x100000,hex(u.reg_read(UC_X86_REG_RIP))
 def output(addr,stride):
  cap,p,n=struct.unpack('<3Q',u.mem_read(addr,24));return words(bytes(u.mem_read(p,n*stride)),stride//4)
 rows.append(dict(lengths=lengths,points=words(floats(sum(points,[])),3),tangents=words(floats(sum(tangents,[])),2),widths=[x[0] for x in words(floats(widths),1)],panic=panicked,rt=output(0x405000,12),top=output(0x406000,16),corners=captured))
Path('/tmp/m01-stair-collider-native.json').write_text(json.dumps(dict(original_sha256=sha,scope='full1407de8b0 + TupleWindows/StepBy + 141fe5f70; only new_convex boundary captures8pts,returnsNone',rows=rows),indent=2))
print('cases',len(rows),'panic',sum(r['panic'] for r in rows),'segments',sum(len(r['corners']) for r in rows))
for r in rows[:20]:print(r['lengths'],len(r['corners']))
