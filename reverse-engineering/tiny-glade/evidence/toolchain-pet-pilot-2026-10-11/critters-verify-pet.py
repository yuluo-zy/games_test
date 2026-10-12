"""C original per-row five-phase dispatch and complete being_petted RET.
No original ECS/event iterator/UI claim. Candidate is an isolated real C-Bevy
crate; this oracle supplies only the selected source, delta, membership/tick.
"""
from pathlib import Path
import hashlib,json,struct,subprocess,itertools
from unicorn import Uc,UC_ARCH_X86,UC_MODE_64
from unicorn.x86_const import *
ROOT=Path(__file__).resolve().parents[2]
OUT=Path(__file__).resolve().parent
b=json.loads((ROOT/'tracking/baselines.json').read_text())['baselines']['C']
raw=Path(b['binary']['path']).read_bytes();assert hashlib.sha256(raw).hexdigest()==b['binary']['sha256']
base=b['pe']['image_base']
def read(a,n):
 s=next(s for s in b['pe']['sections']if s['rva']<=a-base<s['rva']+s['raw_size'])
 p=s['raw_pointer']+a-base-s['rva'];return raw[p:p+n]
cpu=Uc(UC_ARCH_X86,UC_MODE_64)
pages={}
def map_original(a,n):
 start=a&~4095;end=(a+n+4095)&~4095
 for page in range(start,end,4096):
  if page not in pages:cpu.mem_map(page,4096);pages[page]=True
 cpu.mem_write(a,read(a,n))
map_original(0x14293a350,1261)
table=0x14293a6e1+0xcb22f7;map_original(table,20)
predicate=0x140935910;map_original(predicate,16)
threshold=0x14093591c+0x25ed8c4;map_original(threshold,4)
data=0x60000000;cpu.mem_map(data,0x10000)
rowp=data+0x100;tick=data+0x300;time=data+0x400;sp=data+0x8000;ret=data+0x9000
cpu.mem_write(ret,b'\x90')
def native(row,requested,delta):
 cpu.mem_write(rowp,row);cpu.mem_write(tick,struct.pack('<I',0xabcdef98))
 cpu.mem_write(time+0x28,struct.pack('<I',delta));cpu.mem_write(sp+0x30,struct.pack('<Q',time));cpu.mem_write(sp+0x2c,struct.pack('<I',0x12345678))
 for r,v in [(UC_X86_REG_RCX,rowp),(UC_X86_REG_RDX,int(requested)),(UC_X86_REG_R12,tick),(UC_X86_REG_RSP,sp),(UC_X86_REG_XMM0,0x3e99999a),(UC_X86_REG_XMM1,0x3f800000),(UC_X86_REG_XMM2,0x3e4ccccd),(UC_X86_REG_MXCSR,0x1f80)]:cpu.reg_write(r,v)
 cpu.emu_start(0x14293a6f0,0x14293a489,count=200)
 assert cpu.reg_read(UC_X86_REG_RIP)==0x14293a489,'dispatch did not reach actual query-loop join'
 output=bytes(cpu.mem_read(rowp,104));changed=struct.unpack('<I',cpu.mem_read(tick,4))[0]==0x12345678
 cpu.mem_write(sp,struct.pack('<Q',ret));cpu.reg_write(UC_X86_REG_RSP,sp);cpu.reg_write(UC_X86_REG_RCX,rowp)
 cpu.emu_start(predicate,ret,count=100)
 assert cpu.reg_read(UC_X86_REG_RIP)==ret,'original being_petted did not RET'
 return output.hex(),int(changed),cpu.reg_read(UC_X86_REG_RAX)&255
cases=[]
for phase,requested,progress,delta in itertools.product(range(5),[False,True],[0,0x3f800000,0x3f800001,0x7fc12345],[0,0x3e19999a,0x3e99999a]):
 row=bytearray((i*17+phase*3)&255 for i in range(104));struct.pack_into('<I',row,0x18,phase);struct.pack_into('<I',row,0x1c,progress);struct.pack_into('<Q',row,0x20,31);struct.pack_into('<I',row,0x58,0x3c23d70a);row[0x60]=1
 cases.append((bytes(row),requested,delta))
probe=ROOT/'reconstruction/critters/c-harness/target/release/sheep-pet-probe'
sources=[ROOT/'reconstruction/critters/sheep_pet.rs',ROOT/'reconstruction/critters/sheep_pet_runtime.rs',ROOT/'reconstruction/critters/sheep_pet_c.rs',ROOT/'reconstruction/critters/c-harness/probe.rs',ROOT/'reconstruction/critters/c-harness/Cargo.toml',ROOT/'reconstruction/critters/c-harness/Cargo.lock',Path(__file__)]
hashes={str(p.relative_to(ROOT)):hashlib.sha256(p.read_bytes()).hexdigest()for p in sources}
lines=[f'{row.hex()} {int(requested)} {delta:08x}'for row,requested,delta in cases]
result=subprocess.run([str(probe)],input='\n'.join(lines)+'\n',capture_output=True,text=True,check=True)
outputs=result.stdout.splitlines();assert len(outputs)==len(cases)
failures=[]
for i,((row,requested,delta),actual)in enumerate(zip(cases,outputs)):
 expected=native(row,requested,delta);wanted=f'{expected[0]} {expected[1]} {expected[2]}'
 if actual!=wanted:failures.append({'case':i,'input':lines[i],'native':wanted,'rust':actual})
report={'ok':not failures,'baseline':'C','original_binary_sha256':b['binary']['sha256'],'cases':len(cases),'failure_count':len(failures),'failures':failures,'scope':'Original C per-row complete five-phase dispatch14293a6f0 -> actual query-loop join14293a489; full104B preserved output and explicit changedtick; being_petted140935910 complete entry->RET. Resolved event membership; not fullSystem entry, originalUI or scheduler. Unordered-progress ABI arithmetic included; not a claim those states arise from World.','candidate_sources':hashes,'candidate_binary_sha256':hashlib.sha256(probe.read_bytes()).hexdigest(),'inputs':lines,'table':{'va':hex(table),'targets':[hex(table+struct.unpack('<i',read(table+i*4,4))[0])for i in range(5)]},'bridges':['Selected actual source row andmembership/tick/GameTime contexts supplied; no game math/transition/output replacement','No dependency call occurs in recovered dispatch or being_petted; original SSE instructions andtable execute'],'runtime_scope':'Separate C Bevy public API adapter tests cover events/state/locomotion/finish/source removal; original ECS query/Message writer/UI picking remain gaps.'}
assert hashes=={str(p.relative_to(ROOT)):hashlib.sha256(p.read_bytes()).hexdigest()for p in sources}
(OUT/'critters-C-pet-verification.json').write_text(json.dumps(report,indent=2)+'\n')
print(json.dumps({k:report[k]for k in ['ok','cases','failure_count']}))
if failures:raise SystemExit(1)
