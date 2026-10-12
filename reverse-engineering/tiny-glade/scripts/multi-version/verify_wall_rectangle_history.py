#!/usr/bin/env python3
"""Complete original rectangle-finish owner and niche4 replay differential."""
import argparse,hashlib,json,math,random,struct,subprocess,sys
from pathlib import Path
ROOT=Path(__file__).resolve().parents[2]
sys.path.insert(0,str(ROOT/'evidence/building-wall-history'))
import native_rectangle_finish_research as finish
import native_wall_timed as native
from native_rectangle_history import instant_enclosure_process
bits=lambda f:struct.unpack('<I',struct.pack('<f',f))[0]
f32=lambda b:struct.unpack('<f',struct.pack('<I',b))[0]
sha=lambda p:hashlib.sha256(p.read_bytes()).hexdigest()
def main():
 ap=argparse.ArgumentParser();ap.add_argument('--cases',type=int,default=192);ap.add_argument('--report',default='evidence/building-wall-history/B-rectangle-enclosure-verification.json');args=ap.parse_args();rng=random.Random(1464);cases=[];lines=[]
 for i in range(args.cases):
  a=[0.,.3,2.8,-2.9,.0001,.7][(i//8)%6];b=[.4,-.3,.1,2.9,.0002,-1.4][(i//8)%6]
  old=(math.cos(a),math.sin(a),rng.uniform(-4,4),rng.uniform(-4,4),rng.uniform(.3,8),rng.uniform(.3,8))
  new=(math.cos(b),math.sin(b),rng.uniform(-4,4),rng.uniform(-4,4),rng.uniform(.3,8),rng.uniform(.3,8))
  old=struct.unpack('<6f',struct.pack('<6f',*old));new=struct.unpack('<6f',struct.pack('<6f',*new));undo=bool(i&1);timed=bool(i&2);present=bool(i&4);duration=.4;elapsed=0.;dts=[.1,.1,.2,.1,.1]if timed else[.1,.1];finished=(i//96)%2==1
  lines.append(f'{struct.pack("<I6f",1,*old).hex()} {struct.pack("<I6f",1,*new).hex()} {int(undo)} {int(timed)} {int(present)} {bits(duration):08x} {bits(elapsed):08x} {int(finished)} '+','.join(f'{bits(v):08x}'for v in dts)+' 9')
  cases.append((old,new,undo,timed,present,duration,elapsed,dts,finished))
 probe=ROOT/'reconstruction/target/release/wall-rectangle-history-probe';binary_hash=sha(probe);rows=subprocess.run([str(probe)],input='\n'.join(lines)+'\n',capture_output=True,text=True,check=True).stdout.splitlines();assert len(rows)==len(cases);failures=[];summaries=[];returns=0
 for i,(case,row)in enumerate(zip(cases,rows)):
  old,new,undo,timed,present,duration,elapsed,dts,finished=case
  recorded=finish.run(1+i%2,old,new);assert recorded['actual_return'] and len(recorded['history'])==1
  owner=native.data+0x5000;arc=native.owner_arcs(owner)[0][0];assert native.nf.u64(arc+16)==0x8000000000000004
  source_pointer=native.data+0x18000-0x70;expected=[];raw=[]
  for dt in dts:
   if timed:r=native.timed_process(arc,undo,source_pointer,9,present,elapsed,dt,duration,finished)
   else:r=instant_enclosure_process(arc,undo,source_pointer,9,present,elapsed,dt,duration,finished)
   returns+=1;assert r['actual_return']
   raw.append(r);audio=None
   for e in r['events']:
    assert e['count']<=1
    if not e['count']:continue
    b=bytes.fromhex(e['bytes']);assert e['slot']==3;audio={'wall':struct.unpack_from('<Q',b,8)[0],'anchor':b[16:28].hex(),'kind':b[28],'flag':b[29]}
   elapsed=f32(struct.unpack('<I',bytes.fromhex(r['elapsed_bits']))[0]);finished=bool(r['finished'])
   expected.append({'elapsed':bits(elapsed),'finished':finished,'source_count':r['source_count'],'updated':sum(p==native.data+0x60 for p,w in r['notifications']),'source':r.get('source')if present else None,'audio':audio})
   assert r['counter']==100 and all(w==9 and p==native.data+0x60 for p,w in r['notifications'])
   source_pointer=native.data+0x18000-0x70
  candidate=json.loads(row)
  if candidate!=expected:failures.append({'case':i,'input':lines[i],'native':expected,'rust':candidate})
  summaries.append({'case':i,'finish':recorded,'native_steps':raw})
 paths=['reconstruction/building/wall_history_edit.rs','reconstruction/building/wall_history_replay.rs','reconstruction/building/wall_history_transition.rs','reconstruction/src/bin/wall-rectangle-history-probe.rs','reconstruction/building/wall_authoring.rs','reconstruction/building/wall_freeform_cache.rs','reconstruction/building/wall_collection.rs','reconstruction/building/wall_state.rs','reconstruction/roof/surface.rs','reconstruction/history/building_transactions.rs','reconstruction/Cargo.lock']
 oracles=[Path(__file__),ROOT/'evidence/building-wall-history/native_rectangle_finish_research.py',ROOT/'evidence/building-wall-history/native_rectangle_history.py',ROOT/'evidence/building-wall-history/native_wall_timed.py',ROOT/'evidence/building-wall-history/native_wall_history.py',ROOT/'evidence/building-authoring/native_authoring.py',ROOT/'evidence/building-constructor/native_fixture.py'];assert sha(probe)==binary_hash
 report={'ok':not failures,'cases':len(cases),'finish_returns':len(cases),'process_returns':returns,'failure_count':len(failures),'failures':failures[:8],'baseline':'B','original_binary_sha256':native.nf.b['binary']['sha256'],'candidate_binary_sha256':binary_hash,'candidate_sources':{p:sha(ROOT/p)for p in paths},'oracle_sources':{str(p.relative_to(ROOT)):sha(p)for p in oracles},'inputs':lines,'native':summaries,'scope':'Actual rectangle finish141464a60 full entry-toRET creates niche4 owner; original process140abfb30 whole RET tests Rectangle2d::lerp/Curve regeneration/source writes/notif count/audio/completion across dims/orientation/center/undo/instant/timed/absent/alreadyfinished. glam angle_to dependency reused; valid finite positive dimensions/unit axes. Circle/mixed shape not accepted. Nonempty cancel and producer threshold Rust closure separate.'}
 (ROOT/args.report).write_text(json.dumps(report,indent=2)+'\n');print(json.dumps({k:report[k]for k in['ok','cases','finish_returns','process_returns','failure_count']}))
 if failures:sys.exit(1)
if __name__=='__main__':main()
