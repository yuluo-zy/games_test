"""Actual continuous World CLI: holes, decorator lifecycle, committed Rim history."""
import json,hashlib,subprocess
from verification_provenance import ROOT,candidate_provenance,workspace_test_count

def main():
 fixture=ROOT/'reconstruction/crates/tg-building-runtime/fixtures/openings-history-cycle.json';probe=ROOT/'reconstruction/target/release/building-run'
 execution=subprocess.run([str(probe),str(fixture)],capture_output=True,text=True,check=True);frames=[json.loads(line) for line in execution.stdout.splitlines()];assert len(frames)==15
 assert all(not f['faults'] for f in frames)
 assert [o['raw_holes'] for o in frames[0]['openings']]==[1,3]
 assert frames[0]['hole_geometry'][0]['source_records']==4 and frames[0]['hole_geometry'][0]['final_records']==5
 assert frames[0]['hole_geometry'][0]['wall_length']==28
 assert frames[1]['hole_geometry']==frames[0]['hole_geometry'] and frames[1]['hole_geometry_rebuilt']==[]
 assert frames[2]['history']['committed']==[0] and frames[2]['history']['entries']==1 and frames[2]['rim_history_pending']==0
 assert frames[2]['openings'][0]['generation']==2 and len(frames[2]['on_wall_changed'])==2
 # Full replay_edit rejects Roof->Roof bucket compatibility. Each owner
 # advances in its own actual frame, including instant replay.
 for first,last,style in [(3,4,0),(5,6,3),(7,8,0)]:
  assert frames[first]['history']['replayed_mutations']==1 and frames[first]['history']['replay_buckets']==1
  assert frames[first]['roofs'][0]['rim_style']==1
  assert frames[last]['history']['replayed_mutations']==1 and frames[last]['history']['replay_buckets']==0
  assert frames[last]['roofs'][0]['rim_style']==style
 assert frames[3]['history']['cursor']==-1 and frames[5]['history']['cursor']==0 and frames[7]['history']['cursor']==-1
 assert frames[9]['history']['entries']==1 and frames[9]['history']['committed']==[0] and frames[9]['roofs'][0]['rim_style']==2
 assert frames[10]['history']['replayed_mutations']==0 and frames[10]['roofs'][0]['rim_style']==2
 assert frames[11]['hole_geometry'][0]['bounds']!=frames[10]['hole_geometry'][0]['bounds'] and len(frames[11]['hole_geometry_rebuilt'])==1
 assert [o['id'] for o in frames[12]['openings']]==['1'] and frames[12]['hole_geometry'][0]['source_records']==1
 assert frames[13]['openings']==[] and frames[13]['hole_geometry'][0]['bounds']==[]
 assert frames[14]['walls']==[] and frames[14]['hole_geometry']==[]
 report={'ok':True,'scope':'Real Bevy App/Commands/Events continuous resolved decorator→RawWallHole→cached finalized geometry plus explicitly initialized B Rim history commit/instant undo/redo/branch; not full assets/placement/wall mesh/history owners/timed replay/fork/GPU acceptance.','frames':frames,'fixture_sha256':hashlib.sha256(fixture.read_bytes()).hexdigest(),'candidate_binary_sha256':hashlib.sha256(probe.read_bytes()).hexdigest(),'workspace_tests':workspace_test_count(),'new_world_tests':2,**candidate_provenance()}
 (ROOT/'evidence/building-holes/B-world-chain-verification.json').write_text(json.dumps(report,ensure_ascii=False,indent=2)+'\n');print(json.dumps({'ok':True,'frames':len(frames)}))
if __name__=='__main__':main()
