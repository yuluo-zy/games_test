#!/usr/bin/env python3
"""Complete rectangle finish-entry decision/owner differential; cancellation is required storage work."""
import argparse,hashlib,json,math,struct,subprocess,sys
from pathlib import Path
ROOT=Path(__file__).resolve().parents[2];sys.path.insert(0,str(ROOT/'evidence/building-wall-history'))
import native_rectangle_finish_research as native
sha=lambda p:hashlib.sha256(p.read_bytes()).hexdigest()
def main():
 ap=argparse.ArgumentParser();ap.add_argument('--report',default='evidence/building-wall-history/B-rectangle-finish-verification.json');args=ap.parse_args();cases=[];lines=[]
 for mode in[1,2]:
  for event in[False,True]:
   for consumed in[False,True]:
    for delta in[0.,.005,.01,.011,.05,.2]:
     for angle in[0.,.0001,.001,.01]:
      old=(1.,0.,2.,3.,4.,5.);new=(math.cos(angle),math.sin(angle),2.,3.,4.+delta,5.);source=(math.cos(.7),math.sin(.7),9.,-2.,6.,7.)
      state=bytearray(80);struct.pack_into('<I',state,0,mode);struct.pack_into('<Q',state,16 if mode==1 else 8,9);struct.pack_into('<6f',state,32 if mode==1 else 24,*old);struct.pack_into('<2f',state,56 if mode==1 else 48,*new[:2]);struct.pack_into('<2f',state,72 if mode==1 else 64,*new[4:6]);
      line=f'finish {state.hex()} {struct.pack("<I6f",1,*source).hex()} {int(event and not consumed)}';cases.append((mode,event,consumed,old,new,source));lines.append(line)
 probe=ROOT/'reconstruction/target/release/wall-rectangle-history-probe';before=sha(probe);rows=subprocess.run([str(probe)],input='\n'.join(lines)+'\n',text=True,capture_output=True,check=True).stdout.splitlines();assert len(rows)==len(cases);failures=[];summaries=[]
 for i,(case,row)in enumerate(zip(cases,rows)):
  mode,event,consumed,old,new,source=case;r=native.run(mode,old,new,event,consumed,actual_source=source);assert r['actual_return'];events=r['events'];assert events[0]['count']==1 and events[1]['count']<=1 and events[2]['count']<=1
  history=None
  if r['history']:
   assert len(r['history'])==1 and r['history'][0]['tag']==7 and len(r['history'][0]['groups'])==1 and len(r['history'][0]['groups'][0])==1
   owner=r['history'][0]['groups'][0][0];assert owner['niche']=='0x8000000000000004';history={k:owner[k]for k in['wall','previous','next']}
  expected={'ui_tag':r['ui_tag'],'active_snapshot':events[0]['bytes'][:160],'history':history,'restore':event and not consumed and history is None,'empty_notification':bool(events[1]['count']),'wall_notification':struct.unpack('<Q',bytes.fromhex(events[2]['bytes'])[8:16])[0]if events[2]['count']else None}
  candidate=json.loads(row)
  if candidate!=expected:failures.append({'case':i,'input':lines[i],'native':expected,'rust':candidate})
  summaries.append({'case':i,'native':r})
 paths=['reconstruction/building/wall_rectangle_edit.rs','reconstruction/building/wall_history_edit.rs','reconstruction/building/wall_history_replay.rs','reconstruction/building/wall_history_transition.rs','reconstruction/src/bin/wall-rectangle-history-probe.rs','reconstruction/building/wall_authoring.rs','reconstruction/building/wall_freeform_cache.rs','reconstruction/building/wall_collection.rs','reconstruction/building/wall_state.rs','reconstruction/roof/surface.rs','reconstruction/history/building_transactions.rs','reconstruction/Cargo.lock'];oracles=[Path(__file__),ROOT/'evidence/building-wall-history/native_rectangle_finish_research.py',ROOT/'evidence/building-wall-history/native_wall_history.py',ROOT/'evidence/building-authoring/native_authoring.py',ROOT/'evidence/building-constructor/native_fixture.py'];assert sha(probe)==before
 report={'ok':not failures,'cases':len(cases),'finish_returns':len(cases),'failure_count':len(failures),'failures':failures[:8],'baseline':'B','original_binary_sha256':native.nf.b['binary']['sha256'],'candidate_binary_sha256':before,'candidate_sources':{p:sha(ROOT/p)for p in paths},'oracle_sources':{str(p.relative_to(ROOT)):sha(p)for p in oracles},'inputs':lines,'native':summaries,'scope':'Original141464a60 whole RET versus pure decision core: active state80B, dim/orientation thresholds, unread/new/no/consumed reader, actualSource enclosure (decorrelated from UIcurrent), niche4 compound group, empty/WallId notifications, UIclear. Cancel decision requires actual separate DecoratorArchivist restoration; native runs actual helper with empty maps/options, nonempty closure separate. Typed missingSource/nonRectangle/inactiveFinish errors exclude original panic partial effects.'}
 (ROOT/args.report).write_text(json.dumps(report,indent=2)+'\n');print(json.dumps({k:report[k]for k in['ok','cases','finish_returns','failure_count']}))
 if failures:sys.exit(1)
if __name__=='__main__':main()
