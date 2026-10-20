from pathlib import Path
import struct,json,hashlib,subprocess,random
import pefile
from unicorn import Uc,UC_ARCH_X86,UC_MODE_64,UC_HOOK_CODE
from unicorn.x86_const import *
OUT=Path(__file__).resolve().parent
raw=Path('D:/game/ljxsj_92385/Tiny Glade/tiny-glade.rne').read_bytes()
SHA=hashlib.sha256(raw).hexdigest();assert SHA=='f507a6cea25c5199447b88ba746a5293e64a279f73797ca74c1ce7f4ff2b7d03'
pe=pefile.PE(data=raw);base=pe.OPTIONAL_HEADER.ImageBase;image=pe.get_memory_mapped_image()
cpu=Uc(UC_ARCH_X86,UC_MODE_64);cpu.mem_map(base,(pe.OPTIONAL_HEADER.SizeOfImage+4095)&~4095);cpu.mem_write(base,image)
DATA,STACK,STOP=0x20000000,0x30000000,0x50000000
cpu.mem_map(DATA,0x100000);cpu.mem_map(STACK,0x200000);cpu.mem_map(STOP,0x1000)
bp=STACK+0x100000
def q(a,v):cpu.mem_write(a,struct.pack('<Q',v&0xffffffffffffffff))
def d(a,v):cpu.mem_write(a,struct.pack('<I',v&0xffffffff))
def uq(a):return struct.unpack('<Q',cpu.mem_read(a,8))[0]
def ud(a):return struct.unpack('<I',cpu.mem_read(a,4))[0]
def setup():
 cpu.mem_write(DATA,bytes(0x10000));cpu.mem_write(STACK,bytes(0x200000))
 for r in [UC_X86_REG_RAX,UC_X86_REG_RBX,UC_X86_REG_RCX,UC_X86_REG_RDX,UC_X86_REG_RBP,UC_X86_REG_RSI,UC_X86_REG_RDI,UC_X86_REG_R8,UC_X86_REG_R9,UC_X86_REG_R10,UC_X86_REG_R11,UC_X86_REG_R12,UC_X86_REG_R13,UC_X86_REG_R14,UC_X86_REG_R15]:cpu.reg_write(r,0)
 for r in range(UC_X86_REG_XMM0,UC_X86_REG_XMM15+1):cpu.reg_write(r,0)
 cpu.reg_write(UC_X86_REG_RBP,bp);cpu.reg_write(UC_X86_REG_RSP,bp-0x5000);cpu.reg_write(UC_X86_REG_MXCSR,0x1f80);cpu.reg_write(UC_X86_REG_RFLAGS,0x202)
stop_address=0
cpu.hook_add(UC_HOOK_CODE,lambda c,pc,sz,_: c.emu_stop() if pc==stop_address else None)
def run(a,b):
 global stop_address
 stop_address=b
 try: cpu.emu_start(a,b,count=2000000)
 except Exception as e:raise RuntimeError(str(e)+' @'+hex(cpu.reg_read(UC_X86_REG_RIP)))
 assert cpu.reg_read(UC_X86_REG_RIP)==b,hex(cpu.reg_read(UC_X86_REG_RIP))
ranges=[]
for name,a,b in [('compatibility-full',0x141471d3c,0x1414732b3),('load-full',0x14146d030,0x14146d030+15561),('legacy15',0x141471f9d,0x14147207f),('window-cold',0x14146f6d8,0x14146f79e),('arrow-records',0x14146ec35,0x14146ec9c),('window-gates',0x14146ed83,0x14146ee0c),('metadata',0x141472e8e,0x141472eb7),('identity-fallback',0x141673140,0x141673140+704)]:
 code=image[a-base:b-base];(OUT/(name+'.bin')).write_bytes(code)
 r=subprocess.run(['C:/Program Files/LLVM/bin/llvm-objdump.exe','-d','--no-show-raw-insn',f'--start-address={a:#x}',f'--stop-address={b:#x}','D:/game/ljxsj_92385/Tiny Glade/tiny-glade.rne'],capture_output=True,check=True)
 (OUT/(name+'.asm')).write_bytes(r.stdout);ranges.append({'name':name,'va':hex(a),'bytes':b-a,'sha256':hashlib.sha256(code).hexdigest()})
legacy=[];rng=random.Random(20261019)
for shape in [0,1]:
 for valid in [False,True]:
  for _ in range(8):
   setup();old=bytearray(rng.randbytes(0x110));wall=rng.getrandbits(64);payload=rng.randbytes(24);struct.pack_into('<QQI',old,0x10,0x800000000000000e,wall,shape);old[0x24:0x3c]=payload;old[0x3c]=valid;cpu.mem_write(DATA,bytes(old));cpu.reg_write(UC_X86_REG_R8,DATA);cpu.reg_write(UC_X86_REG_R14,0x800000000000000e);cpu.reg_write(UC_X86_REG_RDI,2)
   run(0x141471f9d,0x14147207f);after=bytes(cpu.mem_read(DATA,len(old)));before_tag=ud(DATA+0x20);after_tag=ud(DATA+0x3c)
   assert uq(DATA+0x10)==0x8000000000000003;assert uq(DATA+0x18)==wall
   assert (before_tag,after_tag)==((2,shape) if valid else (shape,2))
   assert after[(0x40 if valid else 0x24):(0x58 if valid else 0x3c)]==payload
   legacy.append({'shape_tag':shape,'is_valid':valid,'wall_id':wall,'payload_hex':payload.hex(),'before_tag':before_tag,'after_tag':after_tag,'active_payload_hex':after[(0x40 if valid else 0x24):(0x58 if valid else 0x3c)].hex()})
colors=[0,1,2,3,4,5,6,7,8,9,999,1000,0xffffffff]
rows=[(ty,color) for ty in [0x37,0x38,0x39,0x3a,0x3b] for color in colors]
def records():
 setup();q(bp+0x218,1);q(bp+0x210,DATA);q(DATA+0x18,DATA+0x1000);q(DATA+0x20,len(rows))
 for i,(ty,color) in enumerate(rows):
  cpu.mem_write(DATA+0x1000+i*0x58,bytes([0xa5])*0x58);cpu.mem_write(DATA+0x1000+i*0x58+0x52,bytes([ty]));d(DATA+0x1000+i*0x58+0x30,color)
def result():return [ud(DATA+0x1000+i*0x58+0x30) for i in range(len(rows))]
records();run(0x14146f6d8,0x14146f79e);windows=result()
records();run(0x14146ec35,0x14146ec9c);arrows=result()
records();run(0x14146ec35,0x14146ec9c);run(0x14146f6d8,0x14146f79e);both=result()
setup();q(bp+0x248,len(colors));q(bp+0x240,DATA)
for i,c in enumerate(colors):q(DATA+i*16,i);d(DATA+i*16+8,c)
run(0x14146ed83,0x14146f6d8);gates=[ud(DATA+i*16+8) for i in range(len(colors))]
metadata=[]
for window in [0,1]:
 for arrow in [0,1]:
  for legacy_flag in [0,1]:
   setup();cpu.reg_write(UC_X86_REG_R14,DATA);cpu.reg_write(UC_X86_REG_R8,window);cpu.reg_write(UC_X86_REG_RAX,1);cpu.reg_write(UC_X86_REG_RBX,legacy_flag);d(bp+0x304,arrow);run(0x141472e8e,0x141472eb7)
   flags=uq(DATA+8);assert flags>>40&1==window;assert flags>>48&1==arrow;assert flags>>56&1==legacy_flag
   metadata.append({'window':window,'arrow':arrow,'pre19':legacy_flag,'second_word_hex':hex(flags)})
# Execute untouched original generic hash leaf for exact original pair outputs.
pairs=[]
for a,b in [(0,0),(1,2),(0xffffffffffffffff,999),(0x123456789abcdef0,0x8877665544332211)]:
 setup();q(DATA,a);q(DATA+8,DATA+0x100);q(DATA+0x100,b);cpu.reg_write(UC_X86_REG_RCX,DATA);sp=bp-0x5008;q(sp,STOP);cpu.reg_write(UC_X86_REG_RSP,sp);run(0x14145a8e0,STOP);pairs.append({'words':[a,b],'native_hash':cpu.reg_read(UC_X86_REG_RAX)})
report={'original_sha256':SHA,'ranges':ranges,'legacy15':legacy,'colors':[{'type':t,'input':c,'window':w,'arrow':a,'both':b} for (t,c),w,a,b in zip(rows,windows,arrows,both)],'gate_colors':list(zip(colors,gates)),'wrapper':metadata,'hash_pairs':pairs,'scope':'read-only production; unchanged native region execution; no candidate codec/roundtrip used'}
(OUT/'original-results.json').write_text(json.dumps(report,indent=2)+'\n');print(json.dumps({'legacy15_cases':len(legacy),'color_cases':len(rows),'wrapper_cases':len(metadata),'gate_cases':len(gates),'identity_hash_pairs':pairs}))
