"""Execute the full original rectangular tile generator to the Writer boundary.
Only allocation/CRT imports and the final Writer are bridged. Numeric helpers,
CurveU constructors, RNG, random_splits, matrix conversions and f16 packing all
execute original RNE machine instructions. This is a reference capture, not a
claim that the full rectangular generator has already been ported to Rust.
"""
from pathlib import Path
import struct,json,hashlib,ctypes,math,random
from collections import Counter
from unicorn import Uc,UC_ARCH_X86,UC_MODE_64,UC_HOOK_CODE
from unicorn.x86_const import *
ROOT=Path('D:/game/reverse-engineering/tiny-glade')
raw=Path('D:/game/ljxsj_92385/Tiny Glade/tiny-glade.rne').read_bytes()
assert hashlib.sha256(raw).hexdigest()=='f507a6cea25c5199447b88ba746a5293e64a279f73797ca74c1ce7f4ff2b7d03'
pe=struct.unpack_from('<I',raw,60)[0];ns=struct.unpack_from('<H',raw,pe+6)[0];os=struct.unpack_from('<H',raw,pe+20)[0]
base=struct.unpack_from('<Q',raw,pe+48)[0];sz=struct.unpack_from('<I',raw,pe+80)[0]
cpu=Uc(UC_ARCH_X86,UC_MODE_64);cpu.mem_map(base,(sz+4095)&~4095)
for i in range(ns):
 z,a,s,o=struct.unpack_from('<IIII',raw,pe+24+os+40*i+8);cpu.mem_write(base+a,raw[o:o+s])
DATA=0x20000000;STACK=0x30000000;HEAP=0x50000000;STOP=0x40000000
cpu.mem_map(DATA,0x10000);cpu.mem_map(STACK,0x20000);cpu.mem_map(HEAP,0x4000000);cpu.mem_map(STOP,0x1000)
crt=ctypes.CDLL('ucrtbase.dll');funcs={}
for va,name,n in [(0x142923f70,'powf',2),(0x142923fa0,'roundf',1),(0x142923e60,'ceilf',1),(0x142923e30,'atanf',1),(0x142923fd0,'sinf',1),(0x142923e80,'cosf',1)]:
 f=getattr(crt,name);f.argtypes=[ctypes.c_float]*n;f.restype=ctypes.c_float;funcs[va]=(name,f,n)
def floatbits(x):return struct.unpack('<I',struct.pack('<f',x))[0]
def bitsfloat(x):return struct.unpack('<f',struct.pack('<I',x&0xffffffff))[0]
entries=json.loads((ROOT/'evidence/roof-ridge/rectangular-slice.json').read_text(encoding='utf8'))['functions'][0]['instructions']
calls=Counter();watch=set()
for x in entries:
 if 'CALL 0x' in x:watch.add(int(x.split('CALL ')[1],16))
heap=HEAP;tiles=[];tile_callsites=[];bridge_counts=Counter();allocs={};panic=[];last=[]
def finish(c,value=None):
 if value is not None:c.reg_write(UC_X86_REG_RAX,value)
 rsp=c.reg_read(UC_X86_REG_RSP);ret=struct.unpack('<Q',c.mem_read(rsp,8))[0]
 c.reg_write(UC_X86_REG_RSP,rsp+8);c.reg_write(UC_X86_REG_RIP,ret)
def hook(c,pc,length,user):
 global heap
 if pc in watch:calls[hex(pc)]+=1
 if pc==0x140613c10:
  size=c.reg_read(UC_X86_REG_RCX);align=c.reg_read(UC_X86_REG_RDX);addr=(heap+max(align,16)-1)&~(max(align,16)-1);heap=addr+size
  assert heap<HEAP+0x4000000
  allocs[addr]=size;bridge_counts['__rust_alloc']+=1;finish(c,addr)
 elif pc==0x140613c20:bridge_counts['__rust_dealloc']+=1;finish(c)
 elif pc==0x140613c30:
  old=c.reg_read(UC_X86_REG_RCX);oldsize=c.reg_read(UC_X86_REG_RDX);align=c.reg_read(UC_X86_REG_R8);size=c.reg_read(UC_X86_REG_R9)
  addr=(heap+max(align,16)-1)&~(max(align,16)-1);heap=addr+size;assert heap<HEAP+0x4000000
  c.mem_write(addr,bytes(c.mem_read(old,min(oldsize,size))));allocs[addr]=size;bridge_counts['__rust_realloc']+=1;finish(c,addr)
 elif pc in funcs:
  name,f,n=funcs[pc];args=[bitsfloat(c.reg_read(UC_X86_REG_XMM0+i)) for i in range(n)]
  c.reg_write(UC_X86_REG_XMM0,floatbits(f(*args)));bridge_counts[name]+=1;finish(c)
 elif pc==0x1421b2ca0:
  ptr=c.reg_read(UC_X86_REG_RDX);tiles.append(bytes(c.mem_read(ptr,64)).hex());rsp=c.reg_read(UC_X86_REG_RSP);tile_callsites.append(hex(struct.unpack('<Q',c.mem_read(rsp,8))[0]-5));bridge_counts['Writer boundary']+=1;finish(c)
 elif pc in {0x1428d9518,0x1428d9390,0x1428d9430,0x1428d9310,0x140d8b480,0x1428d9000}:
  panic.append(hex(pc));c.emu_stop()
cpu.hook_add(UC_HOOK_CODE,hook)
def case(shape,roof,flag):
 global heap
 heap=HEAP;tiles.clear();tile_callsites.clear();bridge_counts.clear();allocs.clear();calls.clear();panic.clear()
 cpu.mem_write(DATA,shape);cpu.mem_write(DATA+0x100,bytes(roof))
 cpu.reg_write(UC_X86_REG_RCX,DATA);cpu.reg_write(UC_X86_REG_RDX,DATA+0x100);cpu.reg_write(UC_X86_REG_R8,flag);cpu.reg_write(UC_X86_REG_R9,0)
 cpu.reg_write(UC_X86_REG_MXCSR,0x1f80);cpu.reg_write(UC_X86_REG_RSP,STACK+0x10008)
 cpu.mem_write(STACK+0x10008,struct.pack('<Q',STOP));cpu.mem_write(STACK+0x10030,struct.pack('<Q',DATA+0x400))
 try:cpu.emu_start(0x1421aeef0,STOP,count=2000000)
 except Exception as e:return {'error':str(e),'rip':hex(cpu.reg_read(UC_X86_REG_RIP)),'tile_records':list(tiles),'calls':dict(calls),'bridges':dict(bridge_counts)}
 return {'completed':cpu.reg_read(UC_X86_REG_RIP)==STOP,'panic':list(panic),'tile_records':list(tiles),'tile_callsites':list(tile_callsites),'calls':dict(calls),'bridges':dict(bridge_counts),'heap_bytes':heap-HEAP}
roof=bytearray(88);struct.pack_into('<I',roof,0,41);struct.pack_into('<I',roof,8,1)
for o,v in [(0xc,1.),(0x10,0.),(0x14,0.),(0x18,0.),(0x1c,6.),(0x20,9.),(0x24,0.),(0x28,0.),(0x2c,.5),(0x30,.4),(0x34,.6),(0x38,.7),(0x4c,3.)]:struct.pack_into('<f',roof,o,v)
roof[0x3c]=0
shape=struct.pack('<6f',1.,0.,0.,0.,6.,9.)
if __name__=='__main__':
 result=case(shape,roof,0)
 result.update(source_sha256=hashlib.sha256(raw).hexdigest(),roof_bytes=roof.hex(),rectangle_bytes=shape.hex(),observation_boundary='0x1421b2ca0 TileWriter closure: 64-byte TileInstance argument read before SSBO emission',full_rust_candidate_complete=False)
 (ROOT/'evidence/roof-ridge/rectangular-reference.json').write_text(json.dumps(result,indent=2,ensure_ascii=False),encoding='utf8')
 print(json.dumps({k:v for k,v in result.items() if k not in ['tile_records','tile_callsites']},indent=2));print('tiles',len(tiles));print('phases',dict(Counter(result['tile_callsites'])))

