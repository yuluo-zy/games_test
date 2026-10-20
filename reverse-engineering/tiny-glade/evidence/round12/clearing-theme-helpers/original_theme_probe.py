from pathlib import Path
import struct,json,hashlib,ctypes
import pefile
from unicorn import Uc,UC_ARCH_X86,UC_MODE_64,UC_HOOK_CODE
from unicorn.x86_const import *
OUT=Path(__file__).resolve().parent
raw=Path('D:/game/ljxsj_92385/Tiny Glade/tiny-glade.rne').read_bytes();p=pefile.PE(data=raw);base=p.OPTIONAL_HEADER.ImageBase;image=p.get_memory_mapped_image();sha=hashlib.sha256(raw).hexdigest()
c=Uc(UC_ARCH_X86,UC_MODE_64);c.mem_map(base,(p.OPTIONAL_HEADER.SizeOfImage+4095)&~4095);c.mem_write(base,image)
D,S,STOP=0x20000000,0x30000000,0x50000000;c.mem_map(D,0x400000);c.mem_map(S,0x20000);c.mem_map(STOP,0x1000)
crt=ctypes.CDLL('ucrtbase.dll');crt.sinf.argtypes=[ctypes.c_float];crt.sinf.restype=ctypes.c_float;crt.cosf.argtypes=[ctypes.c_float];crt.cosf.restype=ctypes.c_float
q=lambda a,v:c.mem_write(a,struct.pack('<Q',v&0xffffffffffffffff));u=lambda a:struct.unpack('<Q',c.mem_read(a,8))[0]
def ret():
 sp=c.reg_read(UC_X86_REG_RSP);pc=u(sp);c.reg_write(UC_X86_REG_RSP,sp+8);c.reg_write(UC_X86_REG_RIP,pc)
def string(ptr,n):return bytes(c.mem_read(ptr,n)).decode()
heap=0;trace=[];record=[];load_index=0

def alloc(n):
 global heap
 a=heap;heap=(heap+n+15)&~15;return a

def hook(cpu,pc,sz,_):
 global load_index
 rcx=cpu.reg_read(UC_X86_REG_RCX);rdx=cpu.reg_read(UC_X86_REG_RDX);r8=cpu.reg_read(UC_X86_REG_R8);r9=cpu.reg_read(UC_X86_REG_R9);sp=cpu.reg_read(UC_X86_REG_RSP)
 if pc==STOP:cpu.emu_stop();return
 if pc==0x140d62810:
  a=string(rdx,r8);b=string(r9,u(sp+0x28));b=(a+'/'+b).encode();p=alloc(len(b));cpu.mem_write(p,b);q(rcx,len(b));q(rcx+8,p);q(rcx+16,len(b))
 elif pc==0x140d76ed0:cpu.reg_write(UC_X86_REG_RAX,u(rcx+8));cpu.reg_write(UC_X86_REG_RDX,u(rcx+16))
 elif pc==0x1409365d0:
  path=string(u(r9+8),u(r9+16));trace.append(('placement_load',path));data=record[load_index%len(record)];load_index+=1;p=alloc(len(data));cpu.mem_write(p,data);q(rcx,len(data)//48);q(rcx+8,p);q(rcx+16,len(data)//48);q(rcx+24,load_index+100)
 elif pc==0x14078cb80:
  oldcap=u(rcx);oldptr=u(rcx+8);n=u(rcx+16);p=alloc(0x10000)
  if oldcap and n:cpu.mem_write(p,bytes(cpu.mem_read(oldptr,n*28)))
  q(rcx,0x1000);q(rcx+8,p)
 elif pc==0x140b0eeb0:trace.append(('mesh_load',string(rdx,r8)));cpu.mem_write(rcx,bytes(0xa0))
 elif pc==0x140b1aca0:trace.append(('mesh_register',string(rdx,r8)));cpu.reg_write(UC_X86_REG_RAX,0x8000+len(trace))
 elif pc==0x140937aa0:trace.append(('shader_lookup',string(rdx,r8)));cpu.reg_write(UC_X86_REG_RAX,1);cpu.reg_write(UC_X86_REG_RDX,0x9001)
 elif pc==0x140d146a0:
  for i,v in enumerate([0,D+0x3000,D+0x3000,0,0x1234]):q(rcx+8*i,v)
 elif pc in [0x142923fd0,0x142923e80]:
  val=struct.unpack('<f',struct.pack('<I',cpu.reg_read(UC_X86_REG_XMM0)&0xffffffff))[0];out=crt.sinf(val) if pc==0x142923fd0 else crt.cosf(val);cpu.reg_write(UC_X86_REG_XMM0,struct.unpack('<I',struct.pack('<f',out))[0])
 elif pc==0x140613c20:pass
 else:return
 ret()
c.hook_add(UC_HOOK_CODE,hook)

def run(which,theme,rows):
 global heap,trace,record,load_index
 heap=D+0x10000;trace=[];record=rows;load_index=0;c.mem_write(D,bytes(0x400000));c.mem_write(S,bytes(0x20000))
 for r in [UC_X86_REG_RAX,UC_X86_REG_RBX,UC_X86_REG_RCX,UC_X86_REG_RDX,UC_X86_REG_RBP,UC_X86_REG_RSI,UC_X86_REG_RDI,UC_X86_REG_R8,UC_X86_REG_R9,UC_X86_REG_R10,UC_X86_REG_R11,UC_X86_REG_R12,UC_X86_REG_R13,UC_X86_REG_R14,UC_X86_REG_R15]:c.reg_write(r,0)
 for r in range(UC_X86_REG_XMM0,UC_X86_REG_XMM15+1):c.reg_write(r,0)
 q(D+0x3000,0x100000);q(D+0x3008,D+0x40000);q(D+0x3010,0)
 c.mem_write(D+0x2000+0x58,bytes([theme]));sp=S+0x18000-8;q(sp,STOP)
 if which=='olden':
  args=[D+0x1000,D+0x1100,D+0x1200,D+0x2000,D+0x1400,D+0x1500,D+0x1600,D+0x1700];entry=0x1415539d0
 else:
  q(D+0x1000,D+0x1500);q(D+0x1100,D+0x1600);q(D+0x1300,D+0x1700);q(D+0x1400,D+0x1800);q(D+0x1400+0x10,D+0x1900);q(D+0x1600,D+0x2000)
  args=[D+0x1000,D+0x1100,D+0x1200,D+0x1300,D+0x1400,D+0x1500,D+0x1600];entry=0x141553120
 for reg,val in zip([UC_X86_REG_RCX,UC_X86_REG_RDX,UC_X86_REG_R8,UC_X86_REG_R9],args):c.reg_write(reg,val)
 for i,val in enumerate(args[4:]):q(sp+0x28+i*8,val)
 c.reg_write(UC_X86_REG_RSP,sp);c.reg_write(UC_X86_REG_MXCSR,0x1f80);c.reg_write(UC_X86_REG_RFLAGS,0x202)
 try:c.emu_start(entry,STOP,count=150000)
 except Exception as e:raise RuntimeError(f'{e} pc={c.reg_read(UC_X86_REG_RIP):x} trace={trace}')
 n=u(D+0x3010);queue=bytes(c.mem_read(D+0x40000,n));result={'theme':theme,'trace':trace.copy(),'native_queue_hex':queue.hex(),'native_queue_bytes':n,'input_placements_hex':[x.hex() for x in rows]}
 if which=='olden':
  length=u(D+0x1010);payload=bytes(c.mem_read(u(D+0x1008),length*28));result.update({'native_rectangles_hex':payload.hex(),'rectangles':[{'rectangle':list(struct.unpack_from('<6f',payload,i*28)),'tag':payload[i*28+24]} for i in range(length)]});assert length==sum(len(x)//48 for x in rows)
 return result

rows=[]
for i,angle in enumerate([0.0,0.7,-0.4,90.0,110.3,-250.0]):rows.append(struct.pack('<5fi3I2fi',1+i,2,3-i,0.75+i*0.25,angle,52,7,9,0x3c003c00,0,0,0))
# Struct has precisely twelve words, including three packed/default channels.
assert all(len(x)==48 for x in rows)
results={}
for which in ['olden','flowery']:results[which]=[run(which,t,rows if which=='olden' else [b''.join(rows)]) for t in range(5)]
(OUT/'original-theme-fixtures.json').write_text(json.dumps({'original_sha256':sha,'scope':'Entire original theme helpers execute. Hosted boundaries: std path/Vec allocation/deallocation and CRT sinf/cosf, existing PlacementLibrary load fixture records, mesh load/register, shader lookup and Bevy reserve-entity. Native game loops, rectangle constructor, tags, transforms, component tuples and deferred queue writes remain untouched. This is not full asset/physical GPU proof.','cases':results},indent=2))
print({k:len(v) for k,v in results.items()})
