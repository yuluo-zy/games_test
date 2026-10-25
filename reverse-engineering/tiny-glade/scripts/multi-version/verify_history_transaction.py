"""Original B History transaction/branch/cursor with explicit dependency bridges."""
import hashlib,json,random,struct,subprocess
from verify_parameters import cpu,DATA,STACK,ROOT,baseline,ret
from verification_provenance import candidate_provenance
from unicorn import UC_HOOK_CODE
from unicorn.x86_const import *
H=DATA;ENTRY=DATA+0x1000;SNAPS=DATA+0x6000;ARG=DATA+0x7000
nonce=9000;heap=DATA+0xa000;outcome=None;selected=None;mode=None;last_pc=0

def reset():
 for r in [UC_X86_REG_RAX,UC_X86_REG_RBX,UC_X86_REG_RCX,UC_X86_REG_RDX,UC_X86_REG_RSI,UC_X86_REG_RDI,UC_X86_REG_RBP,UC_X86_REG_R8,UC_X86_REG_R9,UC_X86_REG_R10,UC_X86_REG_R11,UC_X86_REG_R12,UC_X86_REG_R13,UC_X86_REG_R14,UC_X86_REG_R15]:cpu.reg_write(r,0)
 for r in range(UC_X86_REG_XMM0,UC_X86_REG_XMM15+1):cpu.reg_write(r,0)
 cpu.reg_write(UC_X86_REG_MXCSR,0x1f80);cpu.reg_write(UC_X86_REG_RFLAGS,0x202)

def alloc(n):
 global heap
 p=heap;heap=(heap+n+63)&~63
 assert heap<DATA+0x1e000
 cpu.mem_write(p,bytes(n));return p

def get_bytes(raw,off,fmt):return struct.unpack_from(fmt,raw,off)[0]
def put(p,fmt,*v):cpu.mem_write(p,struct.pack(fmt,*v))
def get(p,fmt):return struct.unpack(fmt,cpu.mem_read(p,struct.calcsize(fmt)))
def mutation(ident):
 p=alloc(80);put(p,'<QQ',100,100);cpu.mem_write(p+16,bytes([4,ident%3,(ident+1)%3]));put(p+24,'<Q',ident);return p

def vec_grow(p,width):
 length=get(p+16,'<Q')[0];old=get(p+8,'<Q')[0];dest=alloc(128*width)
 if length:cpu.mem_write(dest,bytes(cpu.mem_read(old,length*width)))
 put(p,'<QQ',128,dest)

def hook(c,pc,size,user):
 global nonce,outcome,selected,last_pc
 last_pc=pc
 if pc==STACK:c.emu_stop()
 elif pc==0x14132ade0:
  nonce+=1;c.reg_write(UC_X86_REG_RAX,nonce);ret(c)
 elif pc==0x140613c10:
  c.reg_write(UC_X86_REG_RAX,alloc(c.reg_read(UC_X86_REG_RCX)));ret(c)
 elif pc==0x1400d9b20:
  c.reg_write(UC_X86_REG_RAX,DATA+0x1f000);ret(c)
 elif pc==0x140562db0:
  vec_grow(c.reg_read(UC_X86_REG_RCX),24);ret(c)
 elif pc in [0x14088bae0,0x14088acf0]:
  # Derived aggregate owners/drop only; game token Drop never bridged.
  ret(c)
 elif pc==0x1428d86b0 and mode=='token_drop':
  # std TLS panic-count query: nonzero global count but current thread unwinds.
  c.reg_write(UC_X86_REG_RAX,0);ret(c)
 elif pc in [0x1428d9390,0x1428d9430,0x1428d7d73]:
  if mode=='start':outcome='E1'
  elif mode in ['add','finish']:outcome='E2' if get(H+0x30,'<Q')[0]==1<<63 and mode=='add' else 'panic'
  elif mode=='cursor':outcome='E3'
  else:outcome='panic'
  c.emu_stop()
 elif mode=='cursor' and pc==0x140af5f9c:
  outcome='none';c.emu_stop()
 elif mode=='cursor' and pc==0x140af6170:
  selected=c.reg_read(UC_X86_REG_RCX);outcome='selected';c.emu_stop()
 elif mode=='expand' and pc==0x140af62bb:
  selected=(get(c.reg_read(UC_X86_REG_RBP)+0x118,'<Q')[0],get(c.reg_read(UC_X86_REG_RBP)+0x120,'<Q')[0]);c.emu_stop()

cpu.hook_add(UC_HOOK_CODE,hook)
REPLAY=0x21000000
cpu.mem_map(REPLAY,0x10000)

def init(length,cursor,anchor,count,initial_nonce,snapshots):
 global nonce,heap
 nonce=9000;heap=DATA+0xa000;cpu.mem_write(DATA,bytes(0x20000));put(DATA+0x1f000,'<QQ',123,456)
 put(H,'<QQQ',128,ENTRY,length);put(H+0x18,'<QQQ',128,SNAPS,len(snapshots));put(H+0x30,'<Q',1<<63)
 put(H+0x58,'<QQiiI',count,initial_nonce,cursor,anchor,0)
 for i in range(length):
  edits=alloc(24);put(edits,'<QQQ',8,mutation(100+i),0)
  deco=alloc(64);stairs=alloc(96);put(deco,'<QQ',100,100);put(stairs,'<QQ',100,100)
  put(ENTRY+i*40,'<QQQQQ',128,edits,1,deco,stairs)
 for i,index in enumerate(snapshots):
  owner=alloc(16);put(owner,'<QQ',100,100);put(SNAPS+i*16,'<QiI',owner,index,0)
 # EventWriter<InputReplayCommand> and Res/AppMode parameter wrappers.
 put(ARG,'<QQQ',DATA+0x7800,0,DATA+0x7900);put(ARG+0x1c,'<I',0x1234)
 put(DATA+0x7820,'<QQQQQ',128,DATA+0x8000,0,0,10)

def state():
 cursor,anchor=get(H+0x68,'<ii');count,change=get(H+0x58,'<QQ');groups=[]
 ptr,length=get(H+8,'<QQ')
 for i in range(length):
  edits,n=get(ptr+i*40+8,'<QQ');ids=[]
  for j in range(n):
   arc=get(edits+j*24+8,'<Q')[0];ids.append(str(get(arc+24,'<Q')[0]))
  groups.append(','.join(ids))
 pending=-1 if get(H+0x30,'<Q')[0]==1<<63 else get(H+0x40,'<Q')[0]
 sptr,slen=get(H+0x20,'<QQ');snaps=[str(get(sptr+i*16+8,'<i')[0]) for i in range(slen)]
 return f'{cursor} {anchor} {count} {change} {pending} {";".join(groups) or "-"} {",".join(snaps) or "-"}'

def call(pc,rcx=H,rdx=ARG,dl=None,r8=DATA+0x7c00,r9=0):
 reset();sp=STACK+0x8000;cpu.mem_write(sp,struct.pack('<Q',STACK));cpu.reg_write(UC_X86_REG_RSP,sp)
 cpu.reg_write(UC_X86_REG_RCX,rcx);cpu.reg_write(UC_X86_REG_RDX,rdx if dl is None else dl)
 cpu.reg_write(UC_X86_REG_R8,r8);cpu.reg_write(UC_X86_REG_R9,r9);cpu.emu_start(pc,0,count=5000)

# Roof branch producer and undo command reversal; game output is never injected.
def expand(entry,undo,instant):
 global mode,selected,outcome
 mode='expand';selected=None;outcome=None
 call(0x140af6170,entry,undo,r8=int(instant),r9=DATA+0x7c00)
 assert selected is not None,hex(last_pc)
 ptr,n=selected
 records=[bytes(cpu.mem_read(ptr+i*48,48)) for i in range(n)]
 return records

def process_rim_record(record):
 global mode
 mode='process';reset();cpu.mem_write(REPLAY,record[8:32])
 queues=[REPLAY+0x1000+i*0x1000 for i in range(5)]
 writers=[REPLAY+0x8000+i*0x40 for i in range(5)]
 for i,(q,w) in enumerate(zip(queues,writers)):
  cpu.mem_write(q,bytes(0x300));put(q+0x20,'<QQQQQ',16,q+0x100,0,0,0)
  put(w,'<QQQII',q,0,REPLAY+0x9000+i*4,0,0x1234)
 sp=STACK+0x8000;cpu.mem_write(sp,bytes(0x100));put(sp,'<Q',STACK)
 for off,w in zip([0x28,0x30,0x38],writers[2:]):put(sp+off,'<Q',w)
 cpu.reg_write(UC_X86_REG_RSP,sp);cpu.reg_write(UC_X86_REG_RCX,REPLAY)
 cpu.reg_write(UC_X86_REG_R8,writers[0]);cpu.reg_write(UC_X86_REG_R9,writers[1]);cpu.reg_write(UC_X86_REG_XMM1,0)
 cpu.emu_start(0x1421b48e0,0,count=3000)
 assert cpu.reg_read(UC_X86_REG_RIP)==STACK
 assert [get(q+0x30,'<Q')[0] for q in queues]==[0,0,0,0,1]
 payload=bytes(cpu.mem_read(queues[4]+0x108,11));ident=get_bytes(payload,0,'<Q');old,new,record=payload[8:11]
 assert old==new and record==0
 return f'{ident}={new}'

def run_op(op):
 global mode,outcome,selected
 outcome=None;selected=None
 if op.startswith(('s','t')):
  mode='start';put(DATA+0x7a00,'<I',int(op[1:]));put(DATA+0x7b00,'<Q',DATA+0x7a00)
  call(0x140a86270 if op[0]=='s' else 0x140a86510,H,DATA+0x7b00)
  result=outcome or ('ok' if op[0]=='s' else str(cpu.reg_read(UC_X86_REG_RAX)&1))
 elif op.startswith('a'):
  mode='add';p=alloc(24);put(p,'<QQQ',8,mutation(int(op[1:])),0);call(0x140a84ec0,H,p)
  result=outcome or 'ok'
 elif op=='f':
  mode='finish';was_none=get(H+0x30,'<Q')[0]==1<<63
  call(0x140a86730)
  result=('E2' if was_none else outcome) or str(cpu.reg_read(UC_X86_REG_RAX)&1)
 elif op.startswith('m'):
  mode='mutation';call(0x140a86980,H,0,dl=int(op[1:]));result=outcome or 'ok'
 else:
  mode='cursor';undo=op.startswith('u')
  if get(H+0x30,'<Q')[0]!=1<<63:result='none'
  else:
   reset();sp=STACK+0x8000;cpu.mem_write(sp,bytes(0x200));put(sp+0x28,'<Q',H)
   put(sp+0x58,'<Q',ARG);put(sp+0x50,'<Q',DATA+0x7c00)
   cpu.reg_write(UC_X86_REG_RSP,sp);cpu.reg_write(UC_X86_REG_RSI,0 if undo else 1)
   cpu.reg_write(UC_X86_REG_RBX,1 if op.endswith('!') else 0)
   cpu.emu_start(0x140af605c,0,count=300)
   if outcome!='selected':result=outcome
   else:
    entry=selected;index=(entry-get(H+8,'<Q')[0])//40
    # The cursor selection passes a true original entry pointer; generate and
    # reverse that entry's command list with original business instructions.
    records=expand(entry,undo,op.endswith('!'))
    ids=[]
    for rec in records:
     # Decode after independently inspecting the Roof replay variant layout.
     assert get_bytes(rec,0,'<I')==5
     arc=get_bytes(rec,16,'<Q');ids.append(str(get(arc+24,'<Q')[0]))
    events=[process_rim_record(rec) for rec in records]
    result=f'{index}:{int(undo)}:'+';'.join(ids)+'#'+';'.join(events)
 return str(result)+'|'+state()

def main():
 rng=random.Random(20261013);cases=[]
 sequences=['s0,a211,a212,f,u,r,u,a299','s0,f,u,r','t0,t0,a211,f,u!,r!','s13,t13,a211,f',
  's0,a211,s0,a212,f,u,r','s0,u,r,a211,f,u,u,r,r','m0,m1','s0,a211,f,u,s0,a212,f,r',
  'u,s0,f,r','u,s0,a211,f,r','s0,a211,a212,a213,f,u!']
 for n in range(0,6):
  for cur in range(-1,n):
   for anchor in [-1,0]:
    for seq in sequences:
     cases.append((n,cur,anchor,n+2,777,[-1,0,cur,n-1,n+1],seq))
 # Aux owner replay is deliberately outside this Rim-only proof.
 lines=[' '.join(map(str,row[:5]))+' '+(','.join(map(str,row[5])) or '-')+' '+row[6] for row in cases]
 probe=ROOT/'reconstruction/target/release/history-transaction-probe'
 actual=subprocess.run([str(probe)],input='\n'.join(lines)+'\n',text=True,capture_output=True,check=True).stdout.splitlines()
 assert len(actual)==len(cases), (len(actual),len(cases))
 failures=[]
 for i,(case,candidate) in enumerate(zip(cases,actual)):
  init(*case[:6]);original=' / '.join(run_op(op) for op in case[6].split(','))
  if original!=candidate:failures.append({'case':i,'input':lines[i],'original':original,'rust':candidate})
 # Execute game-owned token guard independently of aggregate-owner drops.
 # This is original-only guard evidence; candidate comparison above covers
 # finish_potential_mutation consuming both flags, not Rust panic unwinding.
 global mode,outcome
 token_guards=[];iat=0x14362c420;old_iat=bytes(cpu.mem_read(iat,8))
 for active,global_count,expected in [(0,0,'return'),(1,0,'panic'),(1,1,'return')]:
  init(1,0,-1,1,777,[0]);mode='token_drop';outcome=None
  put(iat,'<Q',DATA+0x7e00);put(DATA+0x7e00,'<Q',global_count);put(ARG,'<B',active)
  before=state();call(0x14088f8b0,ARG)
  observed=outcome or 'return'
  assert observed==expected and state()==before,(active,global_count,observed)
  token_guards.append({'active':active,'global_panic_count':global_count,'result':observed,'ledger_unchanged':True})
 cpu.mem_write(iat,old_iat)
 report={'ok':not failures,'cases':len(cases),'failures':failures,'source_sha256':baseline['binary']['sha256'],
  'scope':'B Rim-only recording/commit/redo truncation/cursor, original replay_edit expansion/reversal and RoofReplayCmd::process emits the Rim event with record_history=0; no original ECS/history loader/auxiliary-owner replay/timed command scheduler',
  'candidate_binary_sha256':hashlib.sha256(probe.read_bytes()).hexdigest(),
  'build_mode':'workspace release binary, not a separate arithmetic candidate',
  'original_only_token_drop_guard_cases':token_guards,
  'executed_business':{
   'add':'0x140a84ec0','start':'0x140a86270','try_start':'0x140a86510',
   'finish':'0x140a86730','finish_potential_mutation':'0x140a86980',
   'nuke_future_states':'0x140a84c50','resolved_cursor_range':['0x140af605c','0x140af6170'],
   'replay_edit_expand_reverse':['0x140af6170','0x140af62bb'],
   'generate_replay_cmds_from_edit':'0x14143fb50','RoofReplayCmd_process':'0x1421b48e0'},
  'dependency_bridges':{
   '0x140613c10':'allocator memory, no game policy',
   '0x1400d9b20':'RandomState TLS hash seeds, not mutation nonce',
   '0x14132ade0':'fastrand TLS result injected monotonically for exact state comparison',
   '0x140562db0':'Vec<EditType> reserve into existing arena',
   '0x14088bae0':'MultiEdit aggregate owner drop; original game token Drop is not bridged',
   '0x14088acf0':'auxiliary aggregate owner drop'},
  'excluded':['startup History constructor','auxiliary edit/snapshot owners','full token producer graph',
   'game timed scheduler for instant=false','A/C runtime acceptance','complete ABI/padding reproduction']}
 report.update(candidate_provenance())
 (ROOT/'evidence/building-history/B-transaction-verification.json').write_text(json.dumps(report,indent=2)+'\n')
 print(json.dumps({'ok':report['ok'],'cases':len(cases),'failures':len(failures)}))
 if failures:print(json.dumps(failures[:2]));raise SystemExit(1)
if __name__=='__main__':main()
