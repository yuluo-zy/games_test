"""Execute resolved/raw headless Bevy scenes and B gable scalar store."""
import json,random,struct,subprocess
from verify_parameters import cpu,DATA,STACK,baseline,bits
from verify_building_state import reset
from verification_provenance import ROOT,candidate_provenance,workspace_test_count
from unicorn.x86_const import *

def main():
 rng=random.Random(20261011);lines=[];want=[]
 for _ in range(256):
  roof=bytearray(88);struct.pack_into('<I',roof,8,1)
  for o,v in [(0x0c,1),(0x10,0),(0x14,0),(0x18,0),(0x1c,rng.uniform(2,10)),(0x20,rng.uniform(2,12)),(0x2c,.5),(0x30,rng.uniform(.11,1)),(0x34,1)]:struct.pack_into('<f',roof,o,v)
  base=bits(rng.uniform(.96,15));reset();frame=STACK+0x8000;cpu.mem_write(DATA,bytes(roof));cpu.mem_write(DATA+0x400,bytes(672));cpu.mem_write(frame,bytes(0x200));cpu.mem_write(frame+0x90,struct.pack('<Q',DATA+0x400+0x2a0))
  cpu.reg_write(UC_X86_REG_RCX,DATA);cpu.reg_write(UC_X86_REG_RBP,frame);cpu.reg_write(UC_X86_REG_RSP,STACK+0x6000);cpu.reg_write(UC_X86_REG_XMM15,base);cpu.reg_write(UC_X86_REG_XMM7,0x3eeb851f)
  cpu.emu_start(0x14078c404,0x14078c42d,count=3000);assert cpu.reg_read(UC_X86_REG_RIP)==0x14078c42d
  record=bytes(cpu.mem_read(DATA+0x400,672));assert struct.unpack_from('<I',record,0x110)[0]==1
  want.append(f'{struct.unpack_from("<I",record,0x114)[0]:08x}');lines.append(f'gable_max {base:08x} {roof.hex()}')
 actual=subprocess.run([str(ROOT/'reconstruction/target/release/lifecycle-probe')],input='\n'.join(lines)+'\n',capture_output=True,text=True,check=True).stdout.splitlines();assert len(actual)==len(want);assert actual==want
 fixture=ROOT/'reconstruction/crates/tg-building-runtime/fixtures/building-cycle.json'
 execution=subprocess.run([str(ROOT/'reconstruction/target/release/building-run'),str(fixture)],capture_output=True,text=True,check=True)
 frames=[json.loads(line) for line in execution.stdout.splitlines()];assert len(frames)==5
 assert frames[0]['created']==1 and frames[0]['roofs'][0]['cpu_tile_records']>0
 assert frames[1]['geometry_changes']==1 and frames[2]['marked']==1 and len(frames[2]['roofs'])==1
 assert frames[3]['despawned']==1 and frames[3]['roofs']==[] and frames[3]['presence_changes']==['19']
 assert frames[4]['roofs']==[] and frames[4]['presence_changes']==[]
 assert all(not f['faults'] for f in frames)
 raw_fixture=ROOT/'reconstruction/crates/tg-building-runtime/fixtures/raw-building-cycle.json'
 raw_execution=subprocess.run([str(ROOT/'reconstruction/target/release/building-run'),str(raw_fixture)],capture_output=True,text=True,check=True)
 raw_frames=[json.loads(line) for line in raw_execution.stdout.splitlines()];assert len(raw_frames)==6
 first=raw_frames[0];assert first['created']==1 and first['nani_requests']['create']==1 and first['nani_staged_entities']==1
 wall=first['walls'][0];assert wall['source_point_count']==5 and wall['source_curve_length']==28 and wall['published_point_count']==57
 assert not wall['gable_profile_pending'] and wall['arch_max']>wall['arch_min']
 assert raw_frames[1]['walls']==first['walls'] and raw_frames[1]['nani_requests']=={'create':0,'update':0,'remove':0}
 assert raw_frames[2]['walls'][0]['publication_revision']==2 and raw_frames[2]['nani_requests']['update']==1
 assert raw_frames[3]['marked']==1 and raw_frames[4]['despawned']==1
 assert raw_frames[4]['nani_requests']['remove']==1 and raw_frames[4]['nani_staged_entities']==0
 assert raw_frames[5]['nani_requests']['remove']==0 and raw_frames[5]['presence_changes']==[]
 assert all(not frame['faults'] for frame in raw_frames)
 edit_fixture=ROOT/'reconstruction/crates/tg-building-runtime/fixtures/roof-edit-cycle.json'
 edit_execution=subprocess.run([str(ROOT/'reconstruction/target/release/building-run'),str(edit_fixture)],capture_output=True,text=True,check=True)
 edit_frames=[json.loads(line) for line in edit_execution.stdout.splitlines()];assert len(edit_frames)==8
 assert edit_frames[0]['created']==1 and edit_frames[0]['roofs'][0]['cpu_tile_records']==0
 assert edit_frames[1]['roofs'][0]['direction']==1 and edit_frames[1]['geometry_changes']==1 and edit_frames[1]['sound_kinds']==[7]
 assert edit_frames[2]['wall_holes_requests']==['18446744073709551615']*2 and edit_frames[2]['rim_history_requested']==1 and edit_frames[2]['rim_history_pending']==1
 assert edit_frames[2]['sound_kinds']==[7,11] and edit_frames[2]['nani_requests']['update']==1
 assert edit_frames[3]['wall_holes_requests']==[] and edit_frames[3]['rim_history_requested']==1 and edit_frames[3]['rim_history_pending']==2 and edit_frames[3]['sound_kinds']==[]
 assert edit_frames[4]['roofs'][0]['direction']==0 and edit_frames[4]['roofs'][0]['rim_style']==1 and edit_frames[4]['rim_history_requested']==0 and edit_frames[4]['rim_history_pending']==2
 assert edit_frames[5]['marked']==1 and edit_frames[6]['despawned']==1 and edit_frames[6]['nani_requests']['remove']==1
 assert edit_frames[7]['roofs']==[] and edit_frames[7]['rim_history_pending']==2 and edit_frames[7]['nani_requests']['remove']==0
 assert all(not frame['faults'] for frame in edit_frames)
 report={'ok':True,'scope':'Actual upstream Bevy0.16 App/World/Commands/events plus CPU tile chain, headless CLI scenario; B gable final scalar store vs Rust (256). No original scheduler/fork/whole-game acceptance.',
  'bevy_integration_tests':{'world_scenarios':5,'raw_chain_and_dirty_mask':4,'nani_registration_and_atlas':4,'roof_edit_chain':5,'openings_and_committed_rim_history':2,'asset_parameter_history_chain':13,'raw_wall_production_chain':5,'wall_authoring_chain':3,'wall_structures_chain':1,'workspace_total':workspace_test_count(),'log':'evidence/multi-version/workspace-test.log'},
  'gable_original_sha256':baseline['binary']['sha256'],'gable_original_range':['0x14078c404','0x14078c42d'],'gable_cases':256,
  'cli_frames':frames,'raw_cli_frames':raw_frames,'edit_cli_frames':edit_frames,'edit_fixture_sha256':__import__('hashlib').sha256(edit_fixture.read_bytes()).hexdigest(),'raw_fixture_sha256':__import__('hashlib').sha256(raw_fixture.read_bytes()).hexdigest(),'limitations':['Raw collection/terrain/dirty mask, circle/rectangle cache and B per-arch gable restored; freeform cache and geometry remain pending.',
  'All 10 B creation components explicitly registered; atlas decisions and Roof StoneFloor CPU payload restored. GPU allocation/upload/active handle backend and Floor payload branch remain pending.',
  'Direction/rim replay dispatch and delete undo restoration are connected. Hole invalidation/door-window contribution/finalized geometry and explicitly initialized instant Rim recording/cursor are integrated and verified separately. Assets/placement, other history owners, snapshots and timed replay remain pending. Unconfigured history retains the adapter queue.','Schedule is explicit reconstruction dependency ordering; original fork registration graph not accepted.'],**candidate_provenance()}
 (ROOT/'evidence/building-runtime/B-runtime-verification.json').write_text(json.dumps(report,ensure_ascii=False,indent=2)+'\n');print(json.dumps({'ok':True,'gable_cases':256,'cli_frames':len(frames)}))
if __name__=='__main__':main()
