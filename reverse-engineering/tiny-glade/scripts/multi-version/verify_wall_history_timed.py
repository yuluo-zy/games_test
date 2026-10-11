#!/usr/bin/env python3
"""Differential real B timed WallReplayEditCmd, including source and event writes."""
import argparse, hashlib, json, random, struct, subprocess, sys
from pathlib import Path
ROOT = Path(__file__).resolve().parents[2]
sys.path.insert(0, str(ROOT/'evidence/building-wall-history'))
import native_wall_timed as native
def bits(f): return struct.unpack('<I', struct.pack('<f', f))[0]
def f32(b): return struct.unpack('<f', struct.pack('<I', b))[0]
def sha(p): return hashlib.sha256(p.read_bytes()).hexdigest()
def main():
 ap=argparse.ArgumentParser();ap.add_argument('--cases',type=int,default=128);ap.add_argument('--report',default='evidence/building-wall-history/B-timed-verification.json');args=ap.parse_args()
 rng=random.Random(140951);cases=[];lines=[]
 times=[(.4,0.,[.1,.1,.2,0.,.1]),(.4,-.1,[.1,.1,.15,.15,.1]),(.4,.4,[.1,.1]),(0.,0.,[.1,.1]),(-.4,0.,[.1,.1]),(.4,.1,[0.,-.1,.2,.3]),(.4,float('nan'),[.1,.1]),(float('inf'),.2,[.1,.1])]
 for i in range(args.cases):
  if i%2: shape=struct.pack('<I6f',1,1,0,rng.uniform(-4,4),rng.uniform(-4,4),rng.uniform(.3,8),rng.uniform(.3,8))
  else: shape=struct.pack('<I6f',0,rng.uniform(-4,4),rng.uniform(-4,4),rng.uniform(.3,8),rng.uniform(-3,3),0,0)
  tail=bytearray(56);tail[:28]=shape;extra=rng.uniform(.1,7);floor=rng.uniform(-2,2);struct.pack_into('<3fI',tail,28,extra,floor,2.8,rng.getrandbits(32));tail[44:50]=bytes([rng.randrange(256),i%2,i%3,rng.randrange(256),i%4,rng.randrange(256)])
  duration,elapsed,dts=times[(i//8)%len(times)];undo=bool(i&2);present=bool(i&4);finished=(i//64)%2==1
  oldextra=extra if i%3 else rng.uniform(.1,7);oldfloor=floor if i%3 else rng.uniform(-2,2)
  # Preserve the exact f32 bit input passed to both implementations.
  oldextra=f32(bits(oldextra));oldfloor=f32(bits(oldfloor));wall=101+i;color=i%13
  case=dict(shape=shape,tail=bytes(tail),wall=wall,color=color,undo=undo,present=present,duration=duration,elapsed=elapsed,dts=dts,finished=finished,oldextra=oldextra,oldfloor=oldfloor)
  cases.append(case);lines.append(f'{int(undo)} {int(present)} {bits(duration):08x} {bits(elapsed):08x} {int(finished)} {bits(oldextra):08x} {bits(oldfloor):08x} '+','.join(f'{bits(dt):08x}'for dt in dts)+f' {shape.hex()} {tail.hex()} {wall} {color}')
 probe=ROOT/'reconstruction/target/release/wall-history-timed-probe';before=sha(probe)
 rows=subprocess.run([str(probe)],input='\n'.join(lines)+'\n',text=True,capture_output=True,check=True).stdout.splitlines();assert len(rows)==len(cases)
 failures=[];summaries=[];steps=0
 for i,(case,row)in enumerate(zip(cases,rows)):
  c=case;ctor=native.new_wall_owner(c['shape'],c['tail'],c['wall'],c['color']);arc=native.owner_arcs(ctor['owner_pointer'])[1][0]
  native.cpu.mem_write(arc+16+84,struct.pack('<2f',c['oldextra'],c['oldfloor']))
  source_pointer=arc+128 if c['undo'] else arc+16;elapsed=c['elapsed'];finished=c['finished'];expected=[];raw=[]
  for dt in c['dts']:
   result=native.timed_process(arc,c['undo'],source_pointer,c['wall'],c['present'],elapsed,dt,c['duration'],finished);raw.append(result);steps+=1
   audio=None;height=None
   for event in result['events']:
    assert event['count']<=1
    if not event['count']:continue
    b=bytes.fromhex(event['bytes'])
    if event['slot']==3:audio={'wall':struct.unpack_from('<Q',b,8)[0],'anchor':b[16:28].hex(),'kind':b[28],'flag':b[29]}
    elif event['slot']==0:height={'wall':struct.unpack_from('<Q',b,24)[0],'height':struct.unpack_from('<I',b,20)[0],'flag':b[32]}
    else:raise AssertionError('unexpected color event')
   gotelapsed=struct.unpack('<I',bytes.fromhex(result['elapsed_bits']))[0]
   expected.append({'elapsed':gotelapsed,'finished':bool(result['finished']),'source_count':result['source_count'],'updated':sum(p==native.data+0x60 for p,w in result['notifications']),'source':result.get('source'),'audio':audio,'height':height})
   assert result['counter']==100 and all(w==c['wall'] and p==native.data+0x60 for p,w in result['notifications'])
   elapsed=f32(gotelapsed);finished=bool(result['finished']);source_pointer=native.data+0x18000-0x70
  candidate=json.loads(row)
  if candidate!=expected:failures.append({'case':i,'input':lines[i],'native':expected,'rust':candidate})
  summaries.append({'case':i,'actual_constructor_return':True,'actual_process_returns':len(raw),'native_steps':raw})
 paths=[ROOT/'reconstruction/building/wall_history_transition.rs',ROOT/'reconstruction/building/wall_history_replay.rs',ROOT/'reconstruction/building/wall_history_edit.rs',ROOT/'reconstruction/src/bin/wall-history-timed-probe.rs',ROOT/'reconstruction/building/wall_authoring.rs',ROOT/'reconstruction/building/wall_freeform_cache.rs',ROOT/'reconstruction/building/wall_collection.rs',ROOT/'reconstruction/building/wall_state.rs',ROOT/'reconstruction/roof/surface.rs',ROOT/'reconstruction/history/building_transactions.rs',ROOT/'reconstruction/Cargo.lock']
 oracles=[Path(__file__),ROOT/'evidence/building-wall-history/native_wall_timed.py',ROOT/'evidence/building-wall-history/native_wall_history.py',ROOT/'evidence/building-authoring/native_authoring.py',ROOT/'evidence/building-constructor/native_fixture.py']
 assert sha(probe)==before
 report={'ok':not failures,'cases':len(cases),'process_returns':steps,'failure_count':len(failures),'failures':failures[:8],'baseline':'B','original_binary_sha256':native.nf.b['binary']['sha256'],'candidate_binary_sha256':before,'candidate_sources':{str(p.relative_to(ROOT)):sha(p)for p in paths},'oracle_sources':{str(p.relative_to(ROOT)):sha(p)for p in oracles},'inputs':lines,'native':summaries,'scope':'Full process140abfb30 timed operation0 new-wall growth; actual entry-to-RET including curve source/elevation writes, height requests and audio. Source absent/found, undo/redo, existing finished, duration/elapsed edges. Does not prove outer replay_wall concurrent-group barrier or operation1/2.'}
 (ROOT/args.report).write_text(json.dumps(report,indent=2)+'\n');print(json.dumps({k:report[k]for k in['ok','cases','process_returns','failure_count']}))
 if failures:sys.exit(1)
if __name__=='__main__':main()
