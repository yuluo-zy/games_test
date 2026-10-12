#!/usr/bin/env python3
"""Full native PendingWallChanged producer + deferred record, no expected operation injection."""
import argparse,hashlib,json,math,random,struct,subprocess,sys
from pathlib import Path
ROOT=Path(__file__).resolve().parents[2];sys.path.insert(0,str(ROOT/'evidence/building-wall-history'))
import native_pending_wall_changed_research as native
sha=lambda p:hashlib.sha256(p.read_bytes()).hexdigest()
def dto(points):
 lengths=[((b[0]-a[0])**2+(b[1]-a[1])**2)**.5 for a,b in zip(points,points[1:])];total=sum(lengths);run=0.;us=[0.]
 for length in lengths:run+=length;us.append(run/total)
 return ' '.join([b''.join(struct.pack('<2f',*p)for p in points).hex(),b''.join(struct.pack('<f',u)for u in us).hex(),f'{struct.unpack("<I",struct.pack("<f",total))[0]:08x}'])
def main():
 ap=argparse.ArgumentParser();ap.add_argument('--cases',type=int,default=256);ap.add_argument('--report',default='evidence/building-wall-history/B-pending-wall-changed-verification.json');args=ap.parse_args();rng=random.Random(8560);cases=[];lines=[]
 for i in range(args.cases):
  start=bool(i&1);old=[[0.,0.],[1.,1.],[2.,0.]]if i&2 else[[0.,0.],[2.,0.]]
  if i&4:old.insert(1,list(old[0]))
  extend=[0.,.2,1.,5.][(i//8)%4];next=[[-extend,0.]]+old if start and extend else old+[[2.+extend,0.]]if not start and extend else old
  if i>=32:
   old=[[rng.randrange(-12,13)*.5,rng.randrange(-12,13)*.5]for _ in range(2+(i%6))]
   if all(p==old[0]for p in old):old[-1][0]+=1.
   next=[[-7.,-3.]]+old if start else old+[[7.,3.]]
   if i%5==0:next=old
   if i%11==0:old=[[40.,40.],[41.,40.]] # endpoint projection/None boundary
   if i%7==0:next.insert(1,list(next[0]))
  cases.append((old,next,start));lines.append(dto(old)+' '+dto(next)+' '+str(int(start)))
 probe=ROOT/'reconstruction/target/release/wall-pending-changed-probe';before=sha(probe);rows=subprocess.run([str(probe)],input='\n'.join(lines)+'\n',text=True,capture_output=True,check=True).stdout.splitlines();assert len(rows)==len(cases);failures=[];summaries=[]
 for i,(case,row)in enumerate(zip(cases,rows)):
  old,next,start=case;r=native.run(old,next,start);assert r.pop('actual_pending_return')and r.pop('actual_deferred_return');candidate=json.loads(row)
  if candidate!=r:failures.append({'case':i,'input':lines[i],'native':r,'rust':candidate})
  summaries.append({'case':i,'native':r})
 paths=['reconstruction/building/wall_pending_changed.rs','reconstruction/building/wall_history_edit.rs','reconstruction/src/bin/wall-pending-changed-probe.rs','reconstruction/building/wall_authoring.rs','reconstruction/building/wall_freeform_cache.rs','reconstruction/building/wall_collection.rs','reconstruction/building/wall_state.rs','reconstruction/roof/surface.rs','reconstruction/Cargo.lock'];oracles=[Path(__file__),ROOT/'evidence/building-wall-history/native_pending_wall_changed_research.py',ROOT/'evidence/building-wall-history/native_wall_history.py',ROOT/'evidence/building-authoring/native_authoring.py',ROOT/'evidence/building-constructor/native_fixture.py'];assert sha(probe)==before
 report={'ok':not failures,'cases':len(cases),'pending_returns':len(cases),'deferred_returns':len(cases),'none_count':sum(not r['native']['present']for r in summaries),'failure_count':len(failures),'failures':failures[:8],'baseline':'B','original_binary_sha256':native.nf.b['binary']['sha256'],'candidate_binary_sha256':before,'candidate_sources':{p:sha(ROOT/p)for p in paths},'oracle_sources':{str(p.relative_to(ROOT)):sha(p)for p in oracles},'inputs':lines,'native':summaries,'scope':'OriginalPendingWallChanged.try_new1408dc9f0 + history_record_wall_changed1408dccf0 complete entry-toRET; game approx_coord/u projection/first-tie/duplicate denom floor/None edge; actual old/next snapshots plus operation0 range, oneConcurrentgroup and pending clear. Input typed Curve arrays/Us/length are resolved caller DTOs, not injected Pending or prebuilt RNG outputs. No snapshotEq skip; no original freehand UI selection/outer schedule claimed.'}
 (ROOT/args.report).write_text(json.dumps(report,indent=2)+'\n');print(json.dumps({k:report[k]for k in['ok','cases','pending_returns','deferred_returns','none_count','failure_count']}))
 if failures:sys.exit(1)
if __name__=='__main__':main()
