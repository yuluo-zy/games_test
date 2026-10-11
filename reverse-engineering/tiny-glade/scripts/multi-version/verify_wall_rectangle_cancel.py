#!/usr/bin/env python3
"""Nonempty cancellation calls actual original finish + storage helpers through their RETs."""
import argparse,hashlib,json,random,struct,subprocess,sys
from pathlib import Path
ROOT=Path(__file__).resolve().parents[2];sys.path.insert(0,str(ROOT/'evidence/building-wall-history'))
import native_rectangle_cancel as native
import native_rectangle_finish_research as finish
sha=lambda p:hashlib.sha256(p.read_bytes()).hexdigest()
def encode(d):return ' '.join([','.join(map(str,d['removed']))or'-',','.join(f'{k}:{v}'for k,v in d['items'])or'-',','.join(map(str,d['added']))or'-'])
def main():
 ap=argparse.ArgumentParser();ap.add_argument('--cases',type=int,default=192);ap.add_argument('--report',default='evidence/building-wall-history/B-rectangle-cancel-verification.json');args=ap.parse_args();rng=random.Random(1559);cases=[];lines=[]
 for i in range(args.cases):
  owner=i%3;missing=bool(i&4);count=[0,1,3,8][(i//8)%4]
  info=lambda k:bytes([rng.randrange(256)for _ in range(71)]+[k%2]).hex()
  src={'removed':[10,11,1]if i&1 else[],'items':[[k+1,info(k)]for k in range(count)],'added':[12,13]if i&2 else list(range(1,count+1))}
  cur={'removed':[],'items':[],'added':[]}
  if i&8 and not missing:cur={'removed':[20,21,1],'items':[[1,info(8)],[30,info(9)]],'added':[30,31]}
  saved=[1,40]if i&16 else[];cases.append((owner,missing,src,cur,saved));lines.append(f'{owner} {int(missing)} '+encode(src)+' '+encode(cur)+' '+(','.join(map(str,saved))or'-'))
 probe=ROOT/'reconstruction/target/release/wall-rectangle-cancel-probe';before=sha(probe);rows=subprocess.run([str(probe)],input='\n'.join(lines)+'\n',text=True,capture_output=True,check=True).stdout.splitlines();assert len(rows)==len(cases);failures=[];summaries=[]
 for i,(case,row)in enumerate(zip(cases,rows)):
  owner,missing,src,cur,saved=case;target=[None]
  def prepare(a,s):target[0]=native.prepare_resources(a,s,src,cur,owner,missing,saved)
  native.active=True
  try:result=finish.run(1+i%2,(1.,0.,2.,3.,4.,5.),(1.,0.,2.,3.,4.005,5.),prepare_archivist=prepare)
  finally:native.active=False
  r=native.collect(native.data+0xa000,target[0]);assert result['actual_return']and result['ui_tag']==0 and not result['history'];assert r['restore_entries']==r['restore_returns']==1
  expected={k:r[k]for k in['target','index','archive_wall_count','archive_special_present','saved_removed_count']};expected['index']=[[k,tag,wall if tag==0 else 0]for k,tag,wall in expected['index']];candidate=json.loads(row)
  if candidate!=expected:failures.append({'case':i,'input':lines[i],'native':expected,'rust':candidate})
  summaries.append({'case':i,'finish':result,'restore':r})
 paths=['reconstruction/building/wall_decorator_archivist.rs','reconstruction/src/bin/wall-rectangle-cancel-probe.rs','reconstruction/building/wall_state.rs','reconstruction/Cargo.lock'];oracles=[Path(__file__),ROOT/'evidence/building-wall-history/native_rectangle_cancel.py',ROOT/'evidence/building-wall-history/native_rectangle_finish_research.py',ROOT/'evidence/building-wall-history/native_wall_history.py',ROOT/'evidence/building-authoring/native_authoring.py',ROOT/'evidence/building-constructor/native_fixture.py'];assert sha(probe)==before
 report={'ok':not failures,'cases':len(cases),'finish_returns':len(cases),'restore_returns':len(cases),'failure_count':len(failures),'failures':failures[:8],'baseline':'B','original_binary_sha256':native.nf.b['binary']['sha256'],'candidate_binary_sha256':before,'candidate_sources':{p:sha(ROOT/p)for p in paths},'oracle_sources':{str(p.relative_to(ROOT)):sha(p)for p in oracles},'inputs':lines,'native':summaries,'scope':'Original full141464a60 nochange cancel entryRET with nonempty archived Decorators; actual140b2c4b0 entry + RET traced, get_or_insert_default140a1cf30 and tracked_clone_from14155a950 all game instructions execute. Wall/special1/special2, present empty/nonempty and missing destination, saved removed-by-address, overlaps old/newID, three tracking maps and live global ID index; arbitrary observed Info72 fields carried. Typed scopes valid liveInfo(lastbyte0/1) and consistent unique index; original panic partial effects excluded. Collision source maps genuinely empty, handle-drop is std owner; multi archived wall traversal and cross-owner duplicate failure not yet matrix tested. Only std hash/table clone/insert/remove/TLS/alloc/CRT math bridged; every candidate source/hash/bridge identity retained. Special address inactive WallId bytes ignored.'}
 (ROOT/args.report).write_text(json.dumps(report,indent=2)+'\n');print(json.dumps({k:report[k]for k in['ok','cases','finish_returns','restore_returns','failure_count']}))
 if failures:sys.exit(1)
if __name__=='__main__':main()
