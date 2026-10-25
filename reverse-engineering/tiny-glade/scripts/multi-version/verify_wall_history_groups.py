#!/usr/bin/env python3
"""Full replay_wall original system versus typed internal ConcurrentEdits."""
import argparse, hashlib, json, random, struct, subprocess, sys
from pathlib import Path
ROOT=Path(__file__).resolve().parents[2]
sys.path.insert(0,str(ROOT/'evidence/building-wall-history'))
import native_wall_timed as n
def sha(p):return hashlib.sha256(p.read_bytes()).hexdigest()
def bits(f):return struct.unpack('<I',struct.pack('<f',f))[0]
def main():
 ap=argparse.ArgumentParser();ap.add_argument('--cases',type=int,default=96);ap.add_argument('--report',default='evidence/building-wall-history/B-concurrent-groups-verification.json');a=ap.parse_args();rng=random.Random(951);inputs=[];cases=[];dts=[0.,.1,.1,.2,0.,.1,.1,.1]
 for i in range(a.cases):
  shape=struct.pack('<I6f',0,rng.uniform(-3,3),rng.uniform(-3,3),rng.uniform(.3,5),rng.uniform(-3,3),0,0)if i%2==0 else struct.pack('<I6f',1,1,0,rng.uniform(-3,3),rng.uniform(-3,3),rng.uniform(.3,5),rng.uniform(.3,5))
  tail=bytearray(56);tail[:28]=shape;extra=rng.uniform(.1,8);floor=rng.uniform(-2,2);struct.pack_into('<3fI',tail,28,extra,floor,2.8,rng.getrandbits(32));tail[44:50]=bytes([i%256,(i//8)%2,i%3,(i*13)%256,i%4,(i*7)%256]);disjoint=i>=64;undo=bool(i&2)if not disjoint else False;instant=bool(i&4);wall=201+i;color=i%17
  inputs.append(f'system {int(undo)} {int(instant)} {1 if disjoint else 0:08x} 00000000 0 {bits(extra):08x} {bits(floor):08x} '+','.join(f'{bits(dt):08x}'for dt in dts)+f' {shape.hex()} {tail.hex()} {wall} {color}');cases.append((shape,bytes(tail),wall,color,undo,instant,disjoint))
 probe=ROOT/'reconstruction/target/release/wall-history-timed-probe';binary=sha(probe);rows=subprocess.run([str(probe)],input='\n'.join(inputs)+'\n',capture_output=True,text=True,check=True).stdout.splitlines();assert len(rows)==len(cases);failures=[];native=[]
 for i,((shape,tail,wall,color,undo,instant,disjoint),row)in enumerate(zip(cases,rows)):
  ctor=n.new_wall_owner(shape,tail,wall,color)
  if disjoint:n.cpu.mem_write(n.owner_arcs(ctor['owner_pointer'])[1][0]+240,struct.pack('<Q',wall+1000))
  expanded=n.expand_owner(ctor['owner_pointer'],undo,instant);want=[];raw=[]
  for step,dt in enumerate(dts):
   r=n.replay_wall_system(n.data+0x9000,undo,ctor['source_pointer'],wall,undo,dt,initialize=step==0);raw.append(r);audio=[];colors=[]
   for event in r['events']:
    b=bytes.fromhex(event['bytes']);assert event['count']<=1
    if not event['count']:continue
    if event['slot']==3:audio.append({'wall':struct.unpack_from('<Q',b,8)[0],'anchor':b[16:28].hex(),'kind':b[28],'flag':b[29]})
    elif event['slot']in[1,2]:colors.append({'wall':struct.unpack_from('<Q',b,8)[0],'color':struct.unpack_from('<I',b,16)[0],'slot':event['slot']})
    else:raise AssertionError('unexpected height in new-wall groups')
   assert r['bucket_count']==1 and r['command_count']<=1
   notification_names={n.data+0x30:'new',n.data+0x60:'updated',n.data+0x90:'removed'};notifications=[notification_names[p]for p,w in r['notifications']];assert all(w in[wall,wall+1000] for p,w in r['notifications'])
   want.append({'group_count':r['commands'][0]['group_count']if r['commands']else 0,'source_count':r['source_count'],'source':r.get('source'),'notifications':notifications,'audio':audio,'colors':colors})
  candidate=json.loads(row)
  if candidate!=want:failures.append({'case':i,'input':inputs[i],'native':want,'rust':candidate})
  native.append({'case':i,'actual_constructor_return':True,'actual_expansion_return':expanded['actual_return'],'actual_system_returns':len(raw),'steps':raw})
 paths=[ROOT/'reconstruction/building/wall_history_transition.rs',ROOT/'reconstruction/building/wall_history_replay.rs',ROOT/'reconstruction/building/wall_history_edit.rs',ROOT/'reconstruction/src/bin/wall-history-timed-probe.rs',ROOT/'reconstruction/building/wall_authoring.rs',ROOT/'reconstruction/building/wall_freeform_cache.rs',ROOT/'reconstruction/building/wall_collection.rs',ROOT/'reconstruction/building/wall_state.rs',ROOT/'reconstruction/roof/surface.rs',ROOT/'reconstruction/history/building_transactions.rs',ROOT/'reconstruction/Cargo.lock'];oracles=[Path(__file__),ROOT/'evidence/building-wall-history/native_wall_timed.py',ROOT/'evidence/building-wall-history/native_wall_history.py',ROOT/'evidence/building-authoring/native_authoring.py',ROOT/'evidence/building-constructor/native_fixture.py'];assert sha(probe)==binary
 report={'ok':not failures,'cases':len(cases),'full_system_returns':len(cases)*len(dts),'failure_count':len(failures),'failures':failures[:8],'inputs':inputs,'native':native,'candidate_binary_sha256':binary,'candidate_sources':{str(p.relative_to(ROOT)):sha(p)for p in paths},'oracle_sources':{str(p.relative_to(ROOT)):sha(p)for p in oracles},'original_binary_sha256':n.nf.b['binary']['sha256'],'scope':'Full replay_wall140951b80 actual RET over full ev_new_wall+replay_edit owners. Original RandomState TLS/HashSet insert/is_disjoint use std semantics with actual key sets; all game branching original. Creation two groups intersect WallId and stop after completed front group even instant. Outer bucket pop is deliberately separate. Additional32 ABI owner matrices mutate only the next StateChange WallId; real is_disjoint permits same-call advance with a missing second authority. This tests the barrier branch, not an unrecovered game producer. Arbitrary multioperation/multigroup producers are not claimed.'}
 (ROOT/a.report).write_text(json.dumps(report,indent=2)+'\n');print(json.dumps({k:report[k]for k in['ok','cases','full_system_returns','failure_count']}));sys.exit(bool(failures))
if __name__=='__main__':main()
