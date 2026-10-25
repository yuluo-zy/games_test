"""Roof customization original instructions with resolved query/storage boundaries.
Native game writes, compared union fields, notices, sound parameters and History::add
payload execute; dependency event capacity is preallocated; std allocator returns a
buffer and History::add captures its game payload without committing a transaction.
"""
import hashlib,json,random,struct,subprocess
from pathlib import Path
from verify_parameters import cpu,DATA,STACK,ROOT,baseline,ret
from verify_building_state import reset
from verification_provenance import candidate_provenance
from unicorn import Uc,UC_ARCH_X86,UC_MODE_64,UC_HOOK_CODE
from unicorn.x86_const import *
active=None;mutation=None;reached=None
VERSIONS={'B':{'style_entry':0x14229a20a,'style_exit':0x14229a120,'rim_entry':0x14229a599,'rim_exit':0x14229a480,
 'alloc':0x140613c10,'history_add':0x140a84ec0,'style_none':0x14229a16d,'rim_none':0x14229a4d0},
 'C':{'style_entry':0x1428ac95c,'style_exit':0x1428ac860,'rim_entry':0x1428acd0d,'rim_exit':0x1428acbe0,
 'alloc':0x1406a58e0,'history_add':0x140b4b620,'style_none':0x1428ac8b2,'rim_none':0x1428acc33}}
version='B';last_mutation_ptr=None

def hook(c,pc,size,user):
 global mutation,reached,last_mutation_ptr
 conf=VERSIONS[version]
 if pc==conf[active+'_exit']:
  reached=pc;c.emu_stop()
 elif pc==conf['alloc']:
  assert c.reg_read(UC_X86_REG_RCX)==0x50 and c.reg_read(UC_X86_REG_RDX)==8
  last_mutation_ptr=DATA+0x9000;c.mem_write(last_mutation_ptr,bytes([0xcd])*0x50)
  c.reg_write(UC_X86_REG_RAX,last_mutation_ptr);ret(c)
 elif pc==conf['history_add']:
  arc=struct.unpack('<Q',c.mem_read(c.reg_read(UC_X86_REG_RDX)+8,8))[0]
  assert arc==last_mutation_ptr
  mutation=bytes(c.mem_read(arc+0x10,3))+bytes(c.mem_read(arc+0x18,8));ret(c)
 elif pc in [0x140562630,0x140562c30,0x142d6f550,0x142d6f2b0]:
  raise AssertionError(f'Unexpected dependency queue growth {version} {active} {pc:x}')

def native_reset(c):
 for r in [UC_X86_REG_RAX,UC_X86_REG_RBX,UC_X86_REG_RCX,UC_X86_REG_RDX,UC_X86_REG_RSI,UC_X86_REG_RDI,UC_X86_REG_RBP,
  UC_X86_REG_R8,UC_X86_REG_R9,UC_X86_REG_R10,UC_X86_REG_R11,UC_X86_REG_R12,UC_X86_REG_R13,UC_X86_REG_R14,UC_X86_REG_R15]:c.reg_write(r,0)
 for r in range(UC_X86_REG_XMM0,UC_X86_REG_XMM15+1):c.reg_write(r,0)
 c.reg_write(UC_X86_REG_MXCSR,0x1f80)

def configure(c,kind,roof,event,tick,now):
 global active,mutation,reached,last_mutation_ptr
 active=kind;mutation=None;reached=None;last_mutation_ptr=None;native_reset(c)
 c.mem_write(DATA,roof);c.mem_write(DATA+0x1000,event);c.mem_write(DATA+0x2000,struct.pack('<I',tick))
 sp=STACK+0x8000;c.mem_write(sp,bytes(0x400));c.reg_write(UC_X86_REG_RSP,sp)
 for off,ptr in [(0x3000,DATA+0x4000),(0x5000,DATA+0x6000)]:
  c.mem_write(DATA+off,bytes(0x80));c.mem_write(DATA+off+0x20,struct.pack('<QQQQQ',16,ptr,0,0,42))
 c.mem_write(DATA+0x7000,struct.pack('<III',0xaabbccdd,0xbbccddee,0xccddeeff))
 if version=='B':
  for off,val in [(0x3c,now),(0x4c,now),(0x60,now),(0x68,now)]:c.mem_write(sp+off,struct.pack('<I',val))
  for off,val in [(0x40,DATA+0x3000),(0xb0,DATA+0x7000),(0x28,DATA+0x5000),(0x58,DATA+0x7004),(0x60,DATA+0x7008)]:c.mem_write(sp+off,struct.pack('<Q',val))
  # Separate sound current-tick slot and History tick slot by mode.
  if kind=='style':c.mem_write(sp+0x60,struct.pack('<I',now))
  if kind=='rim':c.mem_write(sp+0x68,struct.pack('<I',now))
  for r,val in [(UC_X86_REG_R11,DATA if kind=='style' else 0),(UC_X86_REG_R10,DATA+0x2000 if kind=='style' else DATA),
   (UC_X86_REG_R14,DATA+0x1000),(UC_X86_REG_R13,DATA+0x1000),(UC_X86_REG_R9,0),(UC_X86_REG_R8,0)]:c.reg_write(r,val)
  c.mem_write(sp+0x28,struct.pack('<Q',DATA+0x5000 if kind=='style' else DATA+0x2000))
 else:
  for off in [0x3c,0x5c,0xa8]:c.mem_write(sp+off,struct.pack('<I',now))
  for off,val in [(0x40,DATA+0x3000),(0xc8,DATA+0x7000),(0xa8,DATA+0x2000),(0x30,DATA+0x2000),
   (0x20,DATA+0x5000),(0x78,DATA+0x7004),(0x70,DATA+0x7008),(0x68,DATA+0x8000)]:c.mem_write(sp+off,struct.pack('<Q',val))
  if kind=='rim':c.mem_write(sp+0xa8,struct.pack('<I',now))
  else:
   c.mem_write(sp+0x30,struct.pack('<Q',DATA+0x5000));c.mem_write(sp+0x68,struct.pack('<Q',DATA+0x7004))
   c.mem_write(sp+0x70,struct.pack('<I',now))
  for r,val in [(UC_X86_REG_R11,DATA),(UC_X86_REG_RBX,DATA),(UC_X86_REG_RDI,DATA+0x1000),
   (UC_X86_REG_R9,0),(UC_X86_REG_RDX,0),(UC_X86_REG_R15,struct.unpack_from('<Q',event,8)[0]),
   (UC_X86_REG_R12,struct.unpack_from('<Q',event,8)[0])]:c.reg_write(r,val)
 return sp

def original(c,kind,roof,event,tick,now):
 configure(c,kind,roof,event,tick,now)
 c.emu_start(VERSIONS[version][kind+'_entry'],0,count=1500)
 assert reached==VERSIONS[version][kind+'_exit'],hex(c.reg_read(UC_X86_REG_RIP))
 updated=bytes(c.mem_read(DATA,88)).hex();newtick=struct.unpack('<I',c.mem_read(DATA+0x2000,4))[0]
 holes=struct.unpack('<Q',c.mem_read(DATA+0x3030,8))[0];sounds=struct.unpack('<Q',c.mem_read(DATA+0x5030,8))[0]
 assert holes in [0,1] and sounds in [0,1]
 sound='-';pivot='-'
 if sounds:
  sound=bytes(c.mem_read(DATA+0x6008,1)).hex();pivot=bytes(c.mem_read(DATA+0x6010,12)).hex()
 return f'{updated} {newtick:08x} {holes} {holes} {sound} {pivot} {mutation.hex() if mutation is not None else "-"}'

def target_guard(c,kind,roof,event,tick,now,present):
 configure(c,kind,roof,event,tick,now)
 c.reg_write(UC_X86_REG_RAX,0x100000001 if present else 0)
 c.reg_write(UC_X86_REG_RBP,DATA+0xa000);c.mem_write(DATA+0xa000,bytes(0x200))
 if version=='C' and kind=='style':c.reg_write(UC_X86_REG_R12,0xffffffff)
 c.emu_start(VERSIONS[version][kind+'_none'],0,count=50)
 assert reached==VERSIONS[version][kind+'_exit']
 assert bytes(c.mem_read(DATA,88))==roof
 assert struct.unpack('<I',c.mem_read(DATA+0x2000,4))[0]==tick
 assert struct.unpack('<Q',c.mem_read(DATA+0x3030,8))[0]==0
 assert struct.unpack('<Q',c.mem_read(DATA+0x5030,8))[0]==0
 assert mutation is None
 assert bytes(c.mem_read(DATA+0x7000,12))==struct.pack('<III',0xaabbccdd,0xbbccddee,0xccddeeff)
 return 'skip'

def load_c():
 meta=json.loads((ROOT/'tracking/baselines.json').read_text())['baselines']['C'];raw=Path(meta['binary']['path']).read_bytes()
 assert hashlib.sha256(raw).hexdigest()==meta['binary']['sha256']
 pe=struct.unpack_from('<I',raw,60)[0];opt=pe+24;base=struct.unpack_from('<Q',raw,opt+24)[0];size=struct.unpack_from('<I',raw,opt+56)[0]
 c=Uc(UC_ARCH_X86,UC_MODE_64);c.mem_map(base,(size+4095)&~4095)
 for i in range(struct.unpack_from('<H',raw,pe+6)[0]):
  _,rva,n,offset=struct.unpack_from('<IIII',raw,opt+struct.unpack_from('<H',raw,pe+20)[0]+40*i+8)
  if n:c.mem_write(base+rva,raw[offset:offset+n])
 c.mem_map(DATA,0x20000);c.mem_map(STACK,0x20000);return c,meta

def main():
 global version
 rng=random.Random(20261012);cases=[]
 for kind in ['style','rim']:
  for i in range(768):
   roof=bytearray(rng.randbytes(88));struct.pack_into('<I',roof,8,i%2)
   now=rng.getrandbits(32);tick=rng.getrandbits(32)
   e=bytearray(rng.randbytes(32 if kind=='style' else 24))
   if kind=='style':
    roof[0x40]=i%3;e[16:28]=roof[0x40:0x4c]
    choice=(i//3)%5
    if choice==1:e[16]=(i+1)%3
    if choice==2:e[17]^=1
    if choice==3:e[20]^=1
    if choice==4:e[24]^=1
    if i%4==0:e[18]^=1 # Padding changes must copy without extra notification.
    e[28]=(i//4)%3
   else:
    e[16]=i%3;e[17]=roof[0x3d] if i%2 else (roof[0x3d]+1)%256;e[18]=(i//2)%3
   cases.append((kind,bytes(roof),bytes(e),tick,now))
 probe=ROOT/'reconstruction/target/release/roof-customization-probe'
 actual=subprocess.run([str(probe)],input=''.join(f'{k} {r.hex()} {e.hex()} {t:08x} {n:08x}\n' for k,r,e,t,n in cases),text=True,capture_output=True,check=True).stdout.splitlines()
 assert len(actual)==len(cases)
 results={};failures=[];cpu.hook_add(UC_HOOK_CODE,hook)
 for v in ['B','C']:
  version=v
  if v=='B':c=cpu;meta=baseline
  else:c,meta=load_c();c.hook_add(UC_HOOK_CODE,hook)
  before=len(failures)
  for i,((kind,r,e,t,n),candidate) in enumerate(zip(cases,actual)):
   expected=original(c,kind,r,e,t,n)
   if candidate!=expected:failures.append({'version':v,'case':i,'kind':kind,'roof':r.hex(),'event':e.hex(),'original':expected,'rust':candidate})
  guards=[]
  for kind in ['style','rim']:
   sample=next(row for row in cases if row[0]==kind)
   for present in [False,True]:
    guards.append({'kind':kind,'case':'invalid ECS lookup after association' if present else 'missing association',
     'outcome':target_guard(c,*sample,present)})
  results[v]={'target_guards':guards,'cases':len(cases),'failures':len(failures)-before,'binary_sha256':meta['binary']['sha256'],'ranges':VERSIONS[v]}
 report={'ok':not failures,'versions':results,'failures':failures,'scope':'B Rust customization resolved-row consumer verified separately on B/C original ranges; no original ECS/hash storage or History commit',
  'bridges':['preallocated EventWriter storage','std allocator buffer','History::add capture boundary'],
  'probe_sha256':hashlib.sha256(probe.read_bytes()).hexdigest(),'candidate_build':'workspace release roof-customization-probe'}
 report.update(candidate_provenance())
 (ROOT/'evidence/building-edits/flat-customization-verification.json').write_text(json.dumps(report,indent=2)+'\n')
 print(json.dumps({'ok':report['ok'],'versions':results,'failures':len(failures)}))
 if failures:print(json.dumps(failures[:2]));raise SystemExit(1)
if __name__=='__main__':main()
