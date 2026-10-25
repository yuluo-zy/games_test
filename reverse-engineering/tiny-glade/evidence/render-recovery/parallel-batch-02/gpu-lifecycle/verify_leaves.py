from pathlib import Path
import sys, struct, json, random, hashlib, subprocess, gzip
sys.path.insert(0, 'C:/Users/liyu/AppData/Local/Packages/PythonSoftwareFoundation.Python.3.12_qbz5n2kfra8p0/LocalCache/local-packages/Python312/site-packages')
from unicorn import Uc, UC_ARCH_X86, UC_MODE_64, UC_HOOK_CODE
from unicorn.x86_const import UC_X86_REG_RCX, UC_X86_REG_RDX, UC_X86_REG_RSP, UC_X86_REG_RIP

ROOT=Path('D:/game/reverse-engineering/tiny-glade')
OUT=ROOT/'evidence/render-recovery/parallel-batch-02/gpu-lifecycle'
CANDIDATE=ROOT/'reconstruction/render/work-packages/gpu-lifecycle-r3'
raw=Path('D:/game/ljxsj_92385/Tiny Glade/tiny-glade.rne').read_bytes()
digest=hashlib.sha256(raw).hexdigest()
assert digest=='f507a6cea25c5199447b88ba746a5293e64a279f73797ca74c1ce7f4ff2b7d03'
header=struct.unpack_from('<I',raw,60)[0]
optional=header+24
base=struct.unpack_from('<Q',raw,optional+24)[0]
size=struct.unpack_from('<I',raw,optional+56)[0]
cpu=Uc(UC_ARCH_X86,UC_MODE_64)
cpu.mem_map(base,(size+4095)&~4095)
for index in range(struct.unpack_from('<H',raw,header+6)[0]):
 virtual_size,address,raw_size,offset=struct.unpack_from('<IIII',raw,optional+struct.unpack_from('<H',raw,header+20)[0]+40*index+8)
 cpu.mem_write(base+address,raw[offset:offset+raw_size])
DATA,STACK,STOP=0x20000000,0x30000000,0x40000000
cpu.mem_map(DATA,0x10000);cpu.mem_map(STACK,0x10000);cpu.mem_map(STOP,0x1000)
state={}
def hook(machine,pc,size,user):
 if pc in (0x1428d9390,0x1428d9430):
  state['panic']=hex(pc);machine.emu_stop()
cpu.hook_add(UC_HOOK_CODE,hook)
def execute(entry,first,second=0):
 state.clear();stack=STACK+0x8008
 cpu.reg_write(UC_X86_REG_RSP,stack);cpu.mem_write(stack,struct.pack('<Q',STOP))
 cpu.reg_write(UC_X86_REG_RCX,first);cpu.reg_write(UC_X86_REG_RDX,second)
 cpu.emu_start(entry,STOP,count=500)
 assert 'panic' in state or cpu.reg_read(UC_X86_REG_RIP)==STOP
 return 'panic' if 'panic' in state else 'ok'
rng=random.Random(20261012)
lines=[];original=[];counts={}
def record(line,result):
 lines.append(line);original.append(result);kind=line.split()[0];counts[kind]=counts.get(kind,0)+1
values=[0,1,2,3,0xffffffff,0x100000000,0x7fffffffffffffff,0x8000000000000000,0xffffffffffffffff]
for old in values:
 for incoming in values:
  for generation,handle_generation in [(0,0),(1,1),(0xffffffff,0xffffffff),(1,2)]:
   for tag in [0,1,2,3,4,0x10001,0xffffffff]:
    cpu.mem_write(DATA,struct.pack('<QQIHBx',DATA+0x400,DATA+0x100,handle_generation,0x1234,5))
    cpu.mem_write(DATA+0x100,struct.pack('<QII',old,7,generation))
    cpu.mem_write(DATA+0x200,struct.pack('<I4xQ',tag,incoming))
    result=execute(0x140bd6a70,DATA,DATA+0x200)
    after=struct.unpack('<Q',cpu.mem_read(DATA+0x100,8))[0]
    record(f'track {old:x} {generation:x} {handle_generation:x} {tag:x} {incoming:x}',f'{result} {after:x}')
for _ in range(5000):
 old=rng.getrandbits(64);incoming=rng.getrandbits(64);generation=rng.getrandbits(32)
 handle_generation=generation if rng.randrange(3) else generation^1;tag=rng.choice([0,1,1,1,2,3,0xffffffff])
 cpu.mem_write(DATA,struct.pack('<QQIHBx',DATA+0x400,DATA+0x100,handle_generation,0xabcd,2))
 cpu.mem_write(DATA+0x100,struct.pack('<QII',old,7,generation));cpu.mem_write(DATA+0x200,struct.pack('<I4xQ',tag,incoming))
 result=execute(0x140bd6a70,DATA,DATA+0x200);after=struct.unpack('<Q',cpu.mem_read(DATA+0x100,8))[0]
 record(f'track {old:x} {generation:x} {handle_generation:x} {tag:x} {incoming:x}',f'{result} {after:x}')
for old,incoming in [(left,right) for left in values for right in values]+[(rng.getrandbits(64),rng.getrandbits(64)) for _ in range(3000)]:
 for tag in [0,1,2,3,0x10001]:
  cpu.mem_write(DATA,struct.pack('<Q',old));cpu.mem_write(DATA+0x200,struct.pack('<I4xQ',tag,incoming))
  result=execute(0x140becfd0,DATA,DATA+0x200);after=struct.unpack('<Q',cpu.mem_read(DATA,8))[0]
  record(f'seq {old:x} {tag:x} {incoming:x}',f'{result} {after:x}')
for count in [0,1,2,0x7fffffff,0x80000000,0xfffffffe,0xffffffff]+[rng.getrandbits(32) for _ in range(3000)]:
 source=struct.pack('<QQIHBx',DATA+0x400,DATA+0x100,rng.getrandbits(32),rng.getrandbits(16),rng.randrange(256))
 cpu.mem_write(DATA,source);cpu.mem_write(DATA+0x100,struct.pack('<QII',rng.getrandbits(64),count,rng.getrandbits(32)))
 cpu.mem_write(DATA+0x200,b'\xa5'*24)
 result=execute(0x140bd6b00,DATA+0x200,DATA);assert result=='ok'
 assert bytes(cpu.mem_read(DATA+0x200,23))==source[:23]
 assert bytes(cpu.mem_read(DATA+0x200+23,1))==b'\xa5'
 after=struct.unpack('<I',cpu.mem_read(DATA+0x108,4))[0];record(f'clone {count:x}',f'{after:x}')
for count in [0,2,3,0x7fffffff,0x80000000,0xffffffff]+[rng.randrange(2,0xffffffff) for _ in range(2000)]:
 for moved in [0,1]:
  cpu.mem_write(DATA,struct.pack('<QQIHBx',DATA+0x400,0 if moved else DATA+0x100,0,0,0))
  cpu.mem_write(DATA+0x100,struct.pack('<QII',0,count,0))
  result=execute(0x140be24b0,DATA);after=struct.unpack('<I',cpu.mem_read(DATA+0x108,4))[0]
  if not moved:assert struct.unpack('<Q',cpu.mem_read(DATA+8,8))[0]==0
  record(f'drop {count:x} {moved:x}',f'{result} {after:x}')
build=subprocess.run(['cargo','build','--release','--offline','--manifest-path',str(CANDIDATE/'Cargo.toml')],capture_output=True,text=True)
assert build.returncode==0,build.stderr
run=subprocess.run([str(CANDIDATE/'target/release/tg-gpu-leaf-oracle.exe')],input='\n'.join(lines)+'\n',capture_output=True,text=True)
assert run.returncode==0,run.stderr
candidate=run.stdout.splitlines()
assert len(candidate)==len(original)
failures=[{'input':line,'original':expected,'candidate':actual} for line,expected,actual in zip(lines,original,candidate) if expected!=actual]
with gzip.open(OUT/'leaf-oracle.jsonl.gz','wt',encoding='utf8') as stream:
 for line,expected in zip(lines,original):stream.write(json.dumps({'input':line,'original':expected})+'\n')
report={'binary_sha256':digest,'passed':not failures,'cases':len(lines),'counts':counts,'failures':failures[:20],'candidate_hashes':{name:hashlib.sha256((CANDIDATE/name).read_bytes()).hexdigest() for name in ['lib.rs','runner.rs','Cargo.toml']},'boundaries':['Untouched original entry instructions under Unicorn; panic dependency entries observed without unwind.','Drop final-reference enqueue path is static-only; non-last drop and moved/null failure tested.','Clone count overflow/zero raw representations tested; these do not establish valid safe-Rust handle states.','No concurrent CAS race scheduling, Vulkan execution, allocation failure, or dependency implementation recovery.']}
(OUT/'leaf-validation.json').write_text(json.dumps(report,indent=2),encoding='utf8')
for start,length,name in [(0x140bd6a70,138,'handle-track'),(0x140bd6b00,48,'handle-clone'),(0x140be24b0,340,'handle-drop'),(0x140becfd0,122,'sequence-track'),(0x1418c0a70,2207,'take-out-waste'),(0x1418c6af0,1348,'dispose-buffer-resource')]:
 result=subprocess.run(['C:/Program Files/LLVM/bin/llvm-objdump.exe','-d',f'--start-address={start:#x}',f'--stop-address={start+length:#x}','D:/game/ljxsj_92385/Tiny Glade/tiny-glade.rne'],capture_output=True,text=True,check=True)
 (OUT/(name+'.asm.txt')).write_text(result.stdout,encoding='utf8')
print(json.dumps(report,indent=2))
assert not failures
events=subprocess.run([str(CANDIDATE/'target/release/tg-gpu-event-oracle.exe')],capture_output=True,text=True,check=True)
(OUT/'event-oracle.json').write_text(json.dumps(json.loads(events.stdout),indent=2),encoding='utf8')
