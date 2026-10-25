"""Compare private Rust resource-graph rules with untouched original PE instructions.

vk-sync access classification executes as shipped and is supplied to the Rust
runner as a dependency boundary. Vec capacity is preallocated for the original
leaf; no game-owned routine is replaced. Panic entry observation omits unwind.
"""
from pathlib import Path
import struct,json,random,hashlib,subprocess,gzip,collections,time,ctypes
from unicorn import Uc,UC_ARCH_X86,UC_MODE_64,UC_HOOK_CODE
from unicorn.x86_const import *

ROOT=Path('D:/game/reverse-engineering/tiny-glade')
OUT=ROOT/'evidence/render-recovery';OUT.mkdir(exist_ok=True)
raw=Path('D:/game/ljxsj_92385/Tiny Glade/tiny-glade.rne').read_bytes()
sha=hashlib.sha256(raw).hexdigest();assert sha=='f507a6cea25c5199447b88ba746a5293e64a279f73797ca74c1ce7f4ff2b7d03'
pe=struct.unpack_from('<I',raw,60)[0];opt=pe+24
base=struct.unpack_from('<Q',raw,opt+24)[0];size=struct.unpack_from('<I',raw,opt+56)[0]
cpu=Uc(UC_ARCH_X86,UC_MODE_64);cpu.mem_map(base,(size+4095)&~4095)
for i in range(struct.unpack_from('<H',raw,pe+6)[0]):
 vs,va,rs,ro=struct.unpack_from('<IIII',raw,opt+struct.unpack_from('<H',raw,pe+20)[0]+40*i+8)
 cpu.mem_write(base+va,raw[ro:ro+rs])
DATA,STACK,STOP=0x20000000,0x30000000,0x40000000
cpu.mem_map(DATA,0x800000);cpu.mem_map(STACK,0x100000);cpu.mem_map(STOP,0x1000)
state={};panic_entries={0x1428d9518,0x1428d9760,0x1428d9390,0x1428d9430}
crt=ctypes.CDLL('ucrtbase.dll')
for name in ['memcpy','memmove','memset']:
 fn=getattr(crt,name);fn.restype=ctypes.c_void_p
 fn.argtypes=[ctypes.c_void_p,ctypes.c_int if name=='memset'else ctypes.c_void_p,ctypes.c_size_t]
def hook(c,pc,n,user):
 if pc in panic_entries:state['panic']=hex(pc);c.emu_stop()
 elif pc==0x140c10e10:state['classifier_calls']+=1
 elif pc==0x140b5f870:raise RuntimeError('unexpected std Vec allocation: capacity boundary')
 elif pc in [0x142923c00,0x142923c10,0x142923c20]:
  # Call the host CRT dependency, never replace a private game routine.
  dst=c.reg_read(UC_X86_REG_RCX);arg=c.reg_read(UC_X86_REG_RDX);n=c.reg_read(UC_X86_REG_R8)
  buf=ctypes.create_string_buffer(n)
  if pc==0x142923c20:crt.memset(buf,arg&255,n)
  else:
   src=ctypes.create_string_buffer(bytes(c.mem_read(arg,n)),n)
   (crt.memcpy if pc==0x142923c00 else crt.memmove)(buf,src,n)
  c.mem_write(dst,buf.raw);sp=c.reg_read(UC_X86_REG_RSP)
  ret=struct.unpack('<Q',c.mem_read(sp,8))[0];c.reg_write(UC_X86_REG_RAX,dst)
  c.reg_write(UC_X86_REG_RSP,sp+8);c.reg_write(UC_X86_REG_RIP,ret);state['crt_calls']+=1
cpu.hook_add(UC_HOOK_CODE,hook)
def execute(va,rcx,rdx=0,count=12000000):
 state.clear();state.update(classifier_calls=0,crt_calls=0)
 sp=STACK+0x80008;cpu.reg_write(UC_X86_REG_RSP,sp);cpu.mem_write(sp,struct.pack('<Q',STOP))
 cpu.reg_write(UC_X86_REG_RCX,rcx);cpu.reg_write(UC_X86_REG_RDX,rdx)
 try:cpu.emu_start(va,STOP,count=count)
 except Exception as e:
  sp=cpu.reg_read(UC_X86_REG_RSP)
  raise RuntimeError(f'native execution {hex(va)} RIP={hex(cpu.reg_read(UC_X86_REG_RIP))} RSP={hex(sp)} return={bytes(cpu.mem_read(sp,32)).hex()} RCX={hex(cpu.reg_read(UC_X86_REG_RCX))} RDX={hex(cpu.reg_read(UC_X86_REG_RDX))}') from e
 if 'panic' not in state:assert cpu.reg_read(UC_X86_REG_RIP)==STOP,hex(cpu.reg_read(UC_X86_REG_RIP))
 return dict(state)
classifications=[]
for v in range(256):
 execute(0x140c10e10,v,count=20);classifications.append(cpu.reg_read(UC_X86_REG_RAX)&255)

rng=random.Random(20261012);lines=[];expected=[];counts=collections.Counter();std_crt_calls=0
def record(line,result,kind):lines.append(line);expected.append(result);counts[kind]+=1
def concurrent(a,ga,b,gb,kind):
 cpu.mem_write(DATA,struct.pack('<IB3xIB3x',ga,a,gb,b))
 st=execute(0x140bece70,DATA,DATA+8,count=100)
 record(f'concurrent {a:x} {ga:x} {b:x} {gb:x} {classifications[a]} {classifications[b]}',f'{cpu.reg_read(UC_X86_REG_RAX)&255} {st["classifier_calls"]}',kind)
for a in range(256):
 for b in range(256):concurrent(a,0,b,0,'all-byte-pairs-zero-group')
groups=[0,1,2,0x7fffffff,0x80000000,0xffffffff]
for a in range(256):
 for ga in groups:
  for gb in groups:concurrent(a,ga,a,gb,'same-access-group-boundaries')
for _ in range(3000):concurrent(rng.randrange(256),rng.getrandbits(32),rng.randrange(256),rng.getrandbits(32),'mixed-access-groups')

def chain(records,kind):
 b=b''.join(struct.pack('<4IB3s',*r) for r in records)
 cpu.mem_write(DATA,struct.pack('<QQQ',len(records),DATA+0x1000,len(records)))
 if b:cpu.mem_write(DATA+0x1000,b)
 st=execute(0x140bd47d0,DATA,count=300000)
 out=bytes(cpu.mem_read(DATA+0x1000,len(b))).hex() if b else '-'
 record('chain '+(b.hex() if b else '-'),('panic' if 'panic'in st else'ok')+' '+out,kind)
def rec(prev):return (rng.getrandbits(32),prev,rng.getrandbits(32),rng.getrandbits(32),rng.randrange(256),rng.randbytes(3))
chain([],'empty-chain')
for n in [1,2,3,4,8,16,64,255]:
 for mode in range(6):
  prevs=[0xffffffff if mode==0 else (i-1 if mode==1 and i else 0xffffffff) if mode==1 else i if mode==2 else (i+1)%n if mode==3 else 0 if mode==4 else rng.randrange(n+2) for i in range(n)]
  chain([rec(p)for p in prevs],'chain-shapes-and-failures')
for _ in range(600):
 n=rng.randrange(1,130);prevs=[rng.choice([0xffffffff,rng.randrange(n)])for _ in range(n)]
 if rng.randrange(4)==0:prevs[rng.randrange(n)]=rng.choice([n,n+1,0xfffffffe])
 chain([rec(p) for p in prevs],'random-chains-and-partial-panic')

def lifetime(records,initial,kind):
 global std_crt_calls
 b=b''.join(struct.pack('<6I',*r) for r in records);ib=b''.join(struct.pack('<I',v)for v in initial)
 cap=len(initial)+2*len(records)+8;obj=bytearray(0x90)
 struct.pack_into('<QQQ',obj,0x30,len(records),DATA+0x1000,len(records))
 struct.pack_into('<QQQ',obj,0x78,cap,DATA+0x300000,len(initial))
 cpu.mem_write(DATA,bytes(obj))
 if b:cpu.mem_write(DATA+0x1000,b)
 if ib:cpu.mem_write(DATA+0x300000,ib)
 st=execute(0x140bd4860,DATA);assert 'panic'not in st
 std_crt_calls+=st['crt_calls'];n=struct.unpack('<Q',cpu.mem_read(DATA+0x88,8))[0]
 out=bytes(cpu.mem_read(DATA+0x300000,4*n)).hex()if n else'-'
 # Original source records must also remain untouched.
 if b:assert bytes(cpu.mem_read(DATA+0x1000,len(b)))==b
 record('lifetime '+(b.hex()if b else'-')+' '+(ib.hex()if ib else'-'),out,kind)
for first in [0,1,2,3,0xffffffff]:
 for last in [0,1,2,3,0xffffffff]:
  for fp,lp in [(0,0),(1,1),(2,0),(0x7fff,0x8000),(0xffff,0x10000),(0xffffffff,0xfffffffe)]:
   lifetime([(first,fp,last,lp,0xffffffff,0x12345678)],[],'endpoint-tags-and-bit-packing')
for n in [0,1,2,4,9,10,11,12,16,32,64,128,1024]:
 lifetime([(1,rng.getrandbits(32),1,rng.getrandbits(32),rng.getrandbits(32),rng.getrandbits(32))for _ in range(n)],[rng.getrandbits(32)for _ in range(3)],'native-stdlib-sort-and-existing-events')
for _ in range(300):
 n=rng.randrange(65);records=[(rng.randrange(3),rng.getrandbits(32),rng.randrange(3),rng.getrandbits(32),rng.getrandbits(32),rng.getrandbits(32))for _ in range(n)]
 lifetime(records,[rng.getrandbits(32)for _ in range(rng.randrange(8))],'mixed-lifetimes')
for index in [0xffff,0x10000,0x10001]:
 records=[(0,0,0,0,0,0)]*(index+1);records[index]=(1,0,1,1,0xffffffff,0xffffffff)
 lifetime(records,[],'unmasked-resource-index-boundaries')

cargo='C:/Users/liyu/.cargo/bin/cargo.exe';manifest=ROOT/'reconstruction/render/Cargo.toml'
build=subprocess.run([cargo,'build','--release','--locked','--manifest-path',str(manifest),'--bin','tg-render-oracle'],capture_output=True,text=True)
if build.returncode:raise RuntimeError(build.stderr)
binary=ROOT/'reconstruction/render/target/release/tg-render-oracle.exe'
run=subprocess.run([str(binary)],input='\n'.join(lines)+'\n',capture_output=True,text=True,check=True)
actual=run.stdout.splitlines()
fail=[{'index':i,'kind':lines[i].split()[0],'input':lines[i][:600],'original':a[:600],'candidate':b[:600]}for i,(a,b)in enumerate(zip(expected,actual))if a!=b]
with gzip.open(OUT/'resource-info-oracle.jsonl.gz','wt',encoding='utf8')as f:
 for line,exp in zip(lines,expected):f.write(json.dumps({'input':line,'original':exp})+'\n')
sources=['reconstruction/render/resource_info.rs','scripts/render-recovery/runner.rs']
report={'ok':len(actual)==len(expected)and not fail,'source_sha256':sha,'total_cases':len(lines),'counts':dict(counts),'failures':fail[:20],
 'candidate_sources':{s:hashlib.sha256((ROOT/s).read_bytes()).hexdigest()for s in sources},
 'candidate_cargo_lock_sha256':hashlib.sha256((ROOT/'reconstruction/render/Cargo.lock').read_bytes()).hexdigest(),
 'original_functions':[{'name':'link_up_next_access_chains','entry':'0x140bd47d0','code_size':136},{'name':'calculate_lifetime_events','entry':'0x140bd4860','code_size':359},{'name':'can_be_concurrent_with','entry':'0x140bece70','code_size':84}],
 'original_dependency_classifier':{'name':'vk_sync_fork::AccessType::is_write','entry':'0x140c10e10','observed_all_256_bytes':classifications,'implemented_as_private_code':False},
 'native_std_crt_memory_calls':std_crt_calls,'locked_release_build':True,
 'limitations':['Record types are observed numeric projections, not a complete original Rust ABI','No Vec allocation failure, panic messages, unwind or sorting performance comparison','Original lifetime event Vec uses preallocated capacity; Rust uses standard Vec/sort directly','All byte classifiers include raw representations not proven to be valid original enum values','Vulkan command submission and GPU state are outside these CPU leaf tests']}
(OUT/'resource-info-verification.json').write_text(json.dumps(report,indent=2),encoding='utf8')
for name,start,end in [('chain',0x140bd47d0,0x140bd4858),('lifetimes',0x140bd4860,0x140bd49c7),('concurrency',0x140bece70,0x140becec4)]:
 dis=subprocess.run(['C:/Program Files/LLVM/bin/llvm-objdump.exe','-d',f'--start-address={hex(start)}',f'--stop-address={hex(end)}','D:/game/ljxsj_92385/Tiny Glade/tiny-glade.rne'],capture_output=True,text=True,check=True)
 (OUT/(name+'.asm.txt')).write_text(dis.stdout,encoding='utf8')
print(json.dumps({k:report[k]for k in ['ok','total_cases','counts','failures','native_std_crt_memory_calls']},indent=2))
if not report['ok']:raise SystemExit(1)
