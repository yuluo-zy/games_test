"""B原1407d94a0完整分段；桥接已解析World射线查询、wall_normal与标准Vec分配。"""
from pathlib import Path
import struct,json,hashlib,pefile,itertools
from unicorn import *
from unicorn.x86_const import *
root=Path('/Users/yuluo/Desktop/game');raw=(root/'B版本内容/tiny-glade.rne').read_bytes();sha=hashlib.sha256(raw).hexdigest();assert sha=='f507a6cea25c5199447b88ba746a5293e64a279f73797ca74c1ce7f4ff2b7d03'
pe=pefile.PE(data=raw,fast_load=True);base=pe.OPTIONAL_HEADER.ImageBase;image=pe.get_memory_mapped_image();u=Uc(UC_ARCH_X86,UC_MODE_64);u.mem_map(base,(len(image)+4095)&~4095);u.mem_write(base,image);u.mem_map(0x100000,0x200000);u.mem_map(0x400000,0x1000000)
def floats(xs):return struct.pack('<'+'f'*len(xs),*xs)
def words(raw,n):return [list(struct.unpack('<'+'I'*n,raw[i:i+4*n])) for i in range(0,len(raw),4*n)]
def ret():
 sp=u.reg_read(UC_X86_REG_RSP);ip=struct.unpack('<Q',u.mem_read(sp,8))[0];u.reg_write(UC_X86_REG_RSP,sp+8);u.reg_write(UC_X86_REG_RIP,ip)
def alloc(n,alignment=16):
 global heap
 heap=(heap+alignment-1)&-alignment;p=heap;heap+=max(16,n);assert heap<0x1300000;u.mem_write(p,bytes(n));return p
queries=[];normal=[0.,-1.];pattern=[];panicked=False;heap=0x600000
def hook(uc,ip,size,data):
 global panicked
 if ip==0x140613c10:u.reg_write(UC_X86_REG_RAX,alloc(u.reg_read(UC_X86_REG_RCX),u.reg_read(UC_X86_REG_RDX)));ret()
 elif ip==0x140613c20:ret()
 elif ip in [0x1406ae550,0x140650050,0x140562db0]:
  stride={0x1406ae550:16,0x140650050:4,0x140562db0:24}[ip];header=u.reg_read(UC_X86_REG_RCX);cap,p,n=struct.unpack('<3Q',u.mem_read(header,24));newcap=max(4,cap*2);newptr=alloc(newcap*stride);u.mem_write(newptr,bytes(u.mem_read(p,n*stride)) if n else b'');u.mem_write(header,struct.pack('<3Q',newcap,newptr,n));ret()
 elif ip==0x140959e90:
  u.reg_write(UC_X86_REG_XMM0,int.from_bytes(floats([normal[0]])+bytes(12),'little'));u.reg_write(UC_X86_REG_XMM1,int.from_bytes(floats([normal[1]])+bytes(12),'little'));ret()
 elif ip==0x1408ccc20:
  out=u.reg_read(UC_X86_REG_RCX);rp=u.reg_read(UC_X86_REG_R8);ray=struct.unpack('<6f',u.mem_read(rp,24));i=len(queries);hit=pattern[i];dist=.25+i*.125;position=[ray[j]+ray[j+3]*dist for j in range(3)];position=list(struct.unpack('<3f',floats(position)));sp=u.reg_read(UC_X86_REG_RSP);mask=struct.unpack('<I',u.mem_read(sp+0x28,4))[0];bound=words(u.reg_read(UC_X86_REG_XMM3).to_bytes(16,'little')[:4],1)[0][0];queries.append(dict(ray=words(floats(ray),3),bound=bound,mask=mask,hit=hit,position=words(floats(position),3)[0]));u.mem_write(out,bytes(40));u.mem_write(out,bytes([0 if hit else 17]));u.mem_write(out+24,floats(position));ret()
 elif ip in [0x1428d9518,0x1428d9310,0x1428d9000]:panicked=True;u.emu_stop()
u.hook_add(UC_HOOK_CODE,hook)
rows=[]
for n in range(1,8):
 for pattern_tuple in itertools.product([False,True],repeat=max(0,n-2)):
  for normal in [[0.,-1.],[-1.,0.],[.3,.9]]:
   pattern=list(pattern_tuple);positions=[[i*.375-1.,i*.1875+.25,i*i*.125-2.] for i in range(n)];tangents=[[1.-i*.125,.375+i*.25] for i in range(n)];widths=[.75+i*.1875 for i in range(n)];heap=0x600000;sp=0x1ff008;u.mem_write(sp,struct.pack('<Q',0x100000));u.reg_write(UC_X86_REG_RSP,sp)
   for addr,vals in [(0x401000,sum(positions,[])),(0x402000,sum(tangents,[])),(0x403000,widths)]:u.mem_write(addr,floats(vals))
   request=bytearray(160);struct.pack_into('<3Q',request,0,n,0x401000,n);struct.pack_into('<3Q',request,0x38,n,0x402000,n);struct.pack_into('<3Q',request,0x50,n,0x403000,n);u.mem_write(0x404000,bytes(request));u.mem_write(0x405000,struct.pack('<Q',0x406000));u.reg_write(UC_X86_REG_RCX,0x407000);u.reg_write(UC_X86_REG_RDX,0x405000);u.reg_write(UC_X86_REG_R8,0x408000);u.reg_write(UC_X86_REG_R9,0x404000);u.mem_write(sp+0x28,struct.pack('<Q',0x409000));queries=[];panicked=False;u.emu_start(0x1407d94a0,0x100000,count=1000000);assert panicked or u.reg_read(UC_X86_REG_RIP)==0x100000,hex(u.reg_read(UC_X86_REG_RIP))
   def vec(h,stride):
    cap,p,length=struct.unpack('<3Q',u.mem_read(h,24));return p,length
   out=[]
   for off in [0,24]:
    p,length=vec(0x407000+off,16);out.append([list(struct.unpack('<2Q',u.mem_read(p+i*16,16))) for i in range(length)])
   p,length=vec(0x407000+48,24);distances=[]
   for i in range(length):
    pp,ln=vec(p+i*24,4);distances.append([r[0] for r in words(bytes(u.mem_read(pp,ln*4)),1)])
   rows.append(dict(n=n,normal=[x[0] for x in words(floats(normal),1)],positions=words(floats(sum(positions,[])),3),tangents=words(floats(sum(tangents,[])),2),widths=[x[0] for x in words(floats(widths),1)],queries=queries,unsupported=out[0],supported=out[1],distances=distances,panic=panicked))
Path('/tmp/m01-stair-support-split-native.json').write_text(json.dumps(dict(original_sha256=sha,scope='full1407d94a0, World raycast+wall_normal+Vec standard allocations only; all interior hit/miss patterns n1..7',rows=rows),indent=2));print('cases',len(rows),'queries',sum(len(r['queries']) for r in rows),'panic',sum(r['panic'] for r in rows))
