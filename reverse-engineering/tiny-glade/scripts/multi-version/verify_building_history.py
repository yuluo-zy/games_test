"""B mixed Roof owners: real record/start/add/finish/cursor/expand/reverse/process."""
import argparse,hashlib,json,random,struct,subprocess
import verify_history_transaction as h
from verification_provenance import candidate_provenance
from unicorn.x86_const import *
from unicorn import UC_HOOK_CODE

ROOT=h.ROOT
wall_capture=None
def allocator_bridge(c,pc,size,user):
 global wall_capture
 if pc==0x140613c30:
  # std __rust_realloc: preserve old bytes while supplying the native Vec's
  # requested capacity; no command count/order or policy is supplied here.
  old=c.reg_read(UC_X86_REG_RCX);old_size=c.reg_read(UC_X86_REG_RDX);new_size=c.reg_read(UC_X86_REG_R9)
  new=h.alloc(new_size);c.mem_write(new,bytes(c.mem_read(old,min(old_size,new_size))))
  c.reg_write(UC_X86_REG_RAX,new);h.ret(c)
 elif h.mode=='delete_producer' and pc==0x14062f06e:c.emu_stop()
 elif pc==0x140613c20:h.ret(c)
 elif h.mode=='replay_roof_loop' and pc==0x142923c10:
  destination=c.reg_read(UC_X86_REG_RCX);source=c.reg_read(UC_X86_REG_RDX);length=c.reg_read(UC_X86_REG_R8)
  c.mem_write(destination,bytes(c.mem_read(source,length)));c.reg_write(UC_X86_REG_RAX,destination);h.ret(c)
 elif h.mode=='wall_constructor' and pc==0x142923c00:
  destination=c.reg_read(UC_X86_REG_RCX);source=c.reg_read(UC_X86_REG_RDX);length=c.reg_read(UC_X86_REG_R8)
  c.mem_write(destination,bytes(c.mem_read(source,length)));c.reg_write(UC_X86_REG_RAX,destination);h.ret(c)
 elif h.mode=='wall_constructor' and pc==0x141696f50:
  # Capture the actual constructed one-group / one-operation owner BEFORE
  # library boxed-from-iter; no constructor field comes from expected values.
  group=c.reg_read(UC_X86_REG_RCX);vector=h.get(group,'<Q')[0]
  arc_ptr=h.get(vector+8,'<Q')[0];arc=h.get(arc_ptr,'<Q')[0]
  wall_capture=bytes(c.mem_read(arc,40));c.emu_stop()
 elif h.mode=='wall_height_dispatch' and pc==0x141327210:
  wall_capture=bytes(c.mem_read(c.reg_read(UC_X86_REG_RDX),40));c.emu_stop()
h.cpu.hook_add(UC_HOOK_CODE,allocator_bridge)
def native_state():
 cursor,anchor=h.get(h.H+0x68,'<ii');nonce=h.get(h.H+0x60,'<Q')[0]
 ptr,length=h.get(h.H+8,'<QQ');groups=[]
 for i in range(length):
  edits,n=h.get(ptr+i*40+8,'<QQ');payloads=[]
  for j in range(n):
   arc=h.get(edits+j*24+8,'<Q')[0];payloads.append(bytes(h.cpu.mem_read(arc+16,64)).hex())
  groups.append(','.join(payloads))
 pending=-1 if h.get(h.H+0x30,'<Q')[0]==1<<63 else h.get(h.H+0x40,'<Q')[0]
 return f'{cursor} {anchor} {nonce} {pending} '+(';'.join(groups) or '-')

def add(payload):
 arc=h.alloc(80);h.put(arc,'<QQ',100,100);h.cpu.mem_write(arc+16,payload)
 edit=h.alloc(24);h.put(edit,'<QQQ',8,arc,0)
 h.mode='add';h.outcome=None;h.call(0x140a84ec0,h.H,edit)
 return h.outcome or 'ok'

def record_source(records,visual):
 # Actual game producer reads a prepared two-generation Bevy source queue.
 # Library iteration internals are not reconstructed; game policy executes.
 events=h.alloc(80);old=h.alloc(max(1,len(records))*len(records[0]) if records else 16)
 new=8
 for i,record in enumerate(records):h.cpu.mem_write(old+i*len(record),record)
 h.put(events,'<QQQQQQQQQ',max(1,len(records)),old,len(records),0,0,new,0,len(records),len(records))
 count=h.alloc(8);h.put(count,'<Q',0);reader=h.alloc(16);h.put(reader,'<QQ',count,events)
 history=h.alloc(32);tick=h.alloc(8);h.put(history,'<QQQII',h.H,0,tick,0,0x1234)
 h.mode='record_visual' if visual else 'record_direction';h.outcome=None
 before=h.get(h.H+0x40,'<Q')[0]
 h.call(0x141a37fd0 if visual else 0x141a3ac20,history if visual else reader,reader if visual else history)
 assert h.cpu.reg_read(UC_X86_REG_RIP)==h.STACK
 assert h.get(count,'<Q')[0]==len(records)
 after=h.get(h.H+0x40,'<Q')[0]
 return str(after-before)

def record_delete(roof):
 h.mode='delete_producer';h.outcome=None;h.reset()
 row=h.alloc(88);h.cpu.mem_write(row,roof);tick=h.alloc(4)
 sp=h.STACK+0x8000;rbp=h.STACK+0x9000
 h.cpu.mem_write(sp,bytes(0x100));h.cpu.mem_write(rbp-0x100,bytes(0x200))
 h.put(rbp-0x48,'<Q',tick);h.put(rbp+0x40,'<I',0x1234);h.put(rbp-0x40,'<Q',h.H)
 h.cpu.reg_write(UC_X86_REG_RSP,sp);h.cpu.reg_write(UC_X86_REG_RBP,rbp);h.cpu.reg_write(UC_X86_REG_R14,row)
 h.cpu.emu_start(0x14062efc6,0,count=3000)
 assert h.cpu.reg_read(UC_X86_REG_RIP)==0x14062f06e
 return h.outcome or 'ok'

def process(record):
 h.mode='process';h.reset();cpu=h.cpu;cpu.mem_write(h.REPLAY,record[8:32])
 queues=[h.REPLAY+0x1000+i*0x1000 for i in range(5)]
 writers=[h.REPLAY+0x8000+i*0x40 for i in range(5)]
 for i,(q,w) in enumerate(zip(queues,writers)):
  cpu.mem_write(q,bytes(0x300));h.put(q+0x20,'<QQQQQ',16,q+0x100,0,0,0)
  cpu.mem_write(q+0x100,bytes([0xa5])*64)
  h.put(w,'<QQQII',q,0,h.REPLAY+0x9000+i*4,0,0x1234)
 sp=h.STACK+0x8000;cpu.mem_write(sp,bytes(0x100));h.put(sp,'<Q',h.STACK)
 for off,w in zip([0x28,0x30,0x38],writers[2:]):h.put(sp+off,'<Q',w)
 cpu.reg_write(UC_X86_REG_RSP,sp);cpu.reg_write(UC_X86_REG_RCX,h.REPLAY)
 cpu.reg_write(UC_X86_REG_R8,writers[0]);cpu.reg_write(UC_X86_REG_R9,writers[1]);cpu.reg_write(UC_X86_REG_XMM1,0)
 cpu.emu_start(0x1421b48e0,0,count=3000)
 assert cpu.reg_read(UC_X86_REG_RIP)==h.STACK
 emitted=[]
 for tag,q in enumerate(queues):
  n=h.get(q+0x30,'<Q')[0]
  assert n<=1 and h.get(q+0x40,'<Q')[0]==n
  if n:
   offset,size=(0,53) if tag==2 else (8,{0:53,1:9,3:9,4:11}[tag])
   emitted.append(f'{tag}:'+bytes(cpu.mem_read(q+0x100+offset,size)).hex())
 assert len(emitted)<=1
 arc=struct.unpack_from('<Q',record,16)[0];payload=bytes(cpu.mem_read(arc+16,64))
 state=bytes(cpu.mem_read(h.REPLAY,24))
 return f'{payload.hex()}:{record[8:16].hex()}:{record[28]}:'+(emitted[0]if emitted else '-')+f':{state[16:20].hex()}:{state[20:22].hex()}'

def cursor(undo,instant):
 h.mode='cursor';h.outcome=None;h.selected=None
 if h.get(h.H+0x30,'<Q')[0]!=1<<63:return 'none'
 h.reset();sp=h.STACK+0x8000;h.cpu.mem_write(sp,bytes(0x200));h.put(sp+0x28,'<Q',h.H)
 h.put(sp+0x58,'<Q',h.ARG);h.put(sp+0x50,'<Q',h.DATA+0x7c00)
 h.cpu.reg_write(UC_X86_REG_RSP,sp);h.cpu.reg_write(UC_X86_REG_RSI,0 if undo else 1)
 h.cpu.reg_write(UC_X86_REG_RBX,int(instant));h.cpu.emu_start(0x140af605c,0,count=300)
 if h.outcome!='selected':return h.outcome
 entry=h.selected;index=(entry-h.get(h.H+8,'<Q')[0])//40
 records=h.expand(entry,undo,instant)
 assert all(struct.unpack_from('<I',r)[0]==5 for r in records)
 return f'{index}:{int(undo)}:'+','.join(process(r) for r in records)

def native_run(line):
 if line.startswith('bucket '):return native_bucket(line)
 if line.startswith('wall_height '):return native_wall_height(line)
 h.init(0,-1,-1,0,777,[]);outputs=[]
 for op in line.split('/'):
  if op.startswith(('s','t')) or op=='f':result=h.run_op(op).split('|')[0]
  elif op.startswith('a'):result=add(bytes.fromhex(op[1:]))
  elif op.startswith('x'):result=record_delete(bytes.fromhex(op[1:]))
  elif op.startswith(('v','d')):result=record_source([bytes.fromhex(r)for r in op[1:].split(',')if r],op[0]=='v')
  else:result=cursor(op[0]=='u',op.endswith('!'))
  outputs.append(f'{result}|{native_state()}')
 return ' / '.join(outputs)

def native_wall_height(line):
 global wall_capture
 _,wall,old,new,undo=line.split();h.init(0,-1,-1,0,777,[]);h.reset()
 out=h.alloc(24);sp=h.STACK+0x8000;h.put(sp,'<Q',h.STACK)
 h.cpu.reg_write(UC_X86_REG_RSP,sp);h.cpu.reg_write(UC_X86_REG_RCX,out);h.cpu.reg_write(UC_X86_REG_RDX,int(wall))
 h.cpu.reg_write(UC_X86_REG_XMM2,int(old,16));h.cpu.reg_write(UC_X86_REG_XMM3,int(new,16))
 h.mode='wall_constructor';wall_capture=None;h.cpu.emu_start(0x140a83fb0,0,count=5000)
 assert wall_capture is not None;arc_bytes=wall_capture;arc=h.alloc(264);h.cpu.mem_write(arc,arc_bytes)
 state=h.alloc(24);h.put(state+20,'<BB',int(undo),0)
 h.mode='wall_height_dispatch';h.reset();rbp=h.STACK+0x9000;h.cpu.mem_write(rbp,bytes([0xa5])*0x200)
 h.cpu.reg_write(UC_X86_REG_RBP,rbp);h.cpu.reg_write(UC_X86_REG_RSI,state);h.cpu.reg_write(UC_X86_REG_R12,arc)
 h.cpu.reg_write(UC_X86_REG_R9,h.ARG);wall_capture=None;h.cpu.emu_start(0x140ac042c,0,count=100)
 assert wall_capture is not None
 return arc_bytes.hex()+' '+wall_capture.hex()

def native_bucket(line):
 _,undo,instant,payloads,times=line.split();h.init(0,-1,-1,0,777,[]);h.run_op('s0')
 for payload_hex in payloads.split(','):add(bytes.fromhex(payload_hex))
 h.run_op('f');records=h.expand(h.get(h.H+8,'<Q')[0],undo=='1',instant=='1')
 buffer=h.alloc(64);bucket=h.alloc(64);ptr=h.alloc(len(records)*48)
 h.put(buffer,'<QQQQ',1,bucket,0,1);h.put(buffer+0x28,'<f',1.)
 h.put(bucket,'<QQQ',len(records),ptr,len(records))
 for i,record in enumerate(records):h.cpu.mem_write(ptr+i*48,record)
 resource=h.alloc(32);tick=h.alloc(4);h.put(resource,'<QQQII',buffer,0,tick,0,0x1234)
 time=h.alloc(64);time_resource=h.alloc(8);h.put(time_resource,'<Q',time)
 queues=[h.REPLAY+0x1000+i*0x1000 for i in range(5)]
 writers=[h.REPLAY+0x8000+i*0x40 for i in range(5)];outputs=[]
 for delta in times.split(','):
  h.put(time+0x28,'<I',int(delta,16))
  for i,(q,w)in enumerate(zip(queues,writers)):
   h.cpu.mem_write(q,bytes(0x500));h.put(q+0x20,'<QQQQQ',16,q+0x100,0,0,0)
   h.put(w,'<QQQII',q,0,h.REPLAY+0x9000+i*4,0,0x1234)
  h.mode='replay_roof_loop';h.reset();sp=h.STACK+0x8000;h.cpu.mem_write(sp,bytes(0x100));h.put(sp,'<Q',h.STACK)
  for offset,w in zip([0x28,0x30,0x38],writers[2:]):h.put(sp+offset,'<Q',w)
  h.cpu.reg_write(UC_X86_REG_RSP,sp);h.cpu.reg_write(UC_X86_REG_RCX,resource);h.cpu.reg_write(UC_X86_REG_RDX,time_resource)
  h.cpu.reg_write(UC_X86_REG_R8,writers[0]);h.cpu.reg_write(UC_X86_REG_R9,writers[1]);h.cpu.emu_start(0x141a37ad0,0,count=20000)
  assert h.cpu.reg_read(UC_X86_REG_RIP)==h.STACK,hex(h.last_pc)
  counts=[h.get(q+0x30,'<Q')[0]for q in queues];remaining=[]
  command_ptr,n=h.get(bucket+8,'<QQ')
  for i in range(n):
   record=bytes(h.cpu.mem_read(command_ptr+i*48,48));arc=struct.unpack_from('<Q',record,16)[0]
   remaining.append(bytes(h.cpu.mem_read(arc+16,64)).hex()+':'+record[24:30].hex())
  outputs.append(','.join(map(str,counts))+'|'+(','.join(remaining)or '-'))
 return ' / '.join(outputs)

def payload(rng,tag,wall):
 p=bytearray(64);p[0]=tag
 if tag<=1:
  p[4:48]=rng.randbytes(44);struct.pack_into('<Q',p,48,wall)
 elif tag==2:
  for off in [4,8,12,16,20,24,28,32,48,52,56,60]:struct.pack_into('<f',p,off,rng.uniform(-2,2))
  struct.pack_into('<Q',p,40,wall)
 else:p[1:3]=bytes([rng.randrange(2),rng.randrange(2)]);struct.pack_into('<Q',p,8,wall)
 return p.hex()

def main():
 args=argparse.ArgumentParser();args.add_argument('--no-report',action='store_true');args=args.parse_args()
 rng=random.Random(20261016);cases=[]
 for _ in range(128):
  edits=[payload(rng,t,100+t)for t in rng.sample(range(5),5)]
  a=['a'+p for p in edits]
  cases.append('/'.join(['s0',*a,'f','u!','r!','u!','s0','a'+payload(rng,4,207),'f','r!']))
  cases.append('/'.join(['s0',*a,'u!','r!','f','u','r','s0','f','u!','r!']))
 for count in range(9):
  for _ in range(8):
   records=[]
   for _ in range(count):
    record=bytearray(rng.randbytes(152))
    for off in [0x18,0x1c,0x20,0x24,0x28,0x2c,0x5c,0x60,0x64,0x68,0x6c,0x70]:struct.pack_into('<f',record,off,rng.uniform(-2,2))
    records.append(record.hex())
   cases.append('/'.join(['s0','v'+','.join(records),'f','u!','r!']))
   dirs=[]
   for _ in range(count):
    record=bytearray(rng.randbytes(24));record[16:18]=bytes([rng.randrange(2),rng.randrange(2)]);dirs.append(record.hex())
   cases.append('/'.join(['s0','d'+','.join(dirs),'f','u!','r!']))
 # Explicit start replacement, empty start preserving redo and Replay guards.
 for _ in range(32):
  a='a'+payload(rng,rng.randrange(5),801)
  cases.extend(['/'.join(['s13','t13',a,'f']),'/'.join(['s0',a,'s0','f','u!']),'/'.join(['s0',a,'f','u!','s0','f','r!'])])
 for _ in range(128):
  roof=rng.randbytes(88).hex();cases.append('/'.join(['s0','x'+roof,'f','u!','r!']))
 for undo in [0,1]:
  for instant in [0,1]:
   for _ in range(32):
    payloads=','.join(payload(rng,tag,100+tag)for tag in rng.sample(range(5),5))
    times=','.join(f'{struct.unpack("<I",struct.pack("<f",dt))[0]:08x}'for dt in [0.,.05,.05,.1,0.,.3])
    cases.append(f'bucket {undo} {instant} {payloads} {times}')
 for _ in range(256):
  wall=rng.getrandbits(64);old=rng.getrandbits(32);new=rng.getrandbits(32)
  for undo in [0,1]:cases.append(f'wall_height {wall} {old:08x} {new:08x} {undo}')
 probe=ROOT/'reconstruction/target/release/building-history-probe'
 actual=subprocess.run([str(probe)],input='\n'.join(cases)+'\n',text=True,capture_output=True,check=True).stdout.splitlines()
 assert len(actual)==len(cases)
 failures=[]
 for i,(line,result)in enumerate(zip(cases,actual)):
  original=native_run(line)
  if result!=original:failures.append({'case':i,'input':line,'original':original,'rust':result})
 report={'ok':not failures,'cases':len(cases),'failure_count':len(failures),'failures':failures[:10],
  'scope':'B mixed Roof Create/Delete/VisualEdit/Direction/Rim recording, commit/cursor/branch, actual replay_edit expansion/reversal/process and per-bucket loop; deferred visual last-only, ridge all-event, resolved invalid deletion recording producers; independent WallHeight constructor/instant event payload only',
  'case_groups':{'mixed_roof_ledger_and_recording':496,'resolved_invalid_roof_delete':128,'roof_replay_bucket_scenes':128,'bucket_steps_each':6,'wall_height_constructor_and_dispatch':512},
  'executed_business':{'visual_record':'0x141a37fd0','direction_record':'0x141a3ac20','resolved_invalid_roof_delete_record':['0x14062efc6','0x14062f06e'],'add':'0x140a84ec0','finish':'0x140a86730','expand_reverse':'0x140af6170','process':'0x1421b48e0','roof_replay_bucket':'0x141a37ad0','wall_height_constructor':['0x140a83fb0','0x140a840c3'],'wall_height_instant_dispatch':['0x140ac042c','0x140ac047a']},
  'source_sha256':h.baseline['binary']['sha256'],'candidate_binary_sha256':hashlib.sha256(probe.read_bytes()).hexdigest(),
  'dependency_bridges':'Reuse existing history verifier allocator/Vec-reserve/drop/RNG TLS bridges; realloc/copy/memmove and __rust_dealloc via matching dependency boundaries; original business record/add/start/finish/cursor/expand/process/bucket executed. Prepared event source/writer storage only. WallHeight stops before boxed-from-iter and EventWriter storage to compare actual constructed owner and payload.',
  'excluded':['Original create/delete UI producer guards and preset selection','Complete 152-byte UI event schema','Wall compound create/delete and actual authority consumers','Wall timed easing and auxiliary owners','Global timed scheduler/aux restore/original cross-event consumer schedule','Actual ECS state/geometry consumers (parent separate World acceptance)','A/C behavior acceptance'],
  **candidate_provenance()}
 if not args.no_report:
  destination=ROOT/'evidence/building-history-owners/B-owner-verification.json';destination.parent.mkdir(parents=True,exist_ok=True);destination.write_text(json.dumps(report,indent=2)+'\n')
 print(json.dumps({'ok':not failures,'cases':len(cases),'failures':len(failures),'report_written':not args.no_report}))
 if failures:print(json.dumps(failures[:1]));raise SystemExit(1)

if __name__=='__main__':main()
