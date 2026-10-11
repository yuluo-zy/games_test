"""Actual resource/cache/masks -> whole original lintel output and third writer."""
import hashlib,json,struct
from pathlib import Path
from verification_provenance import ROOT,candidate_provenance
from verify_wall_production_lifecycle import frames_with_resources,verify_masks,original_prepare,packed,verify_wall
from verify_raw_wall_production_runtime import original_cache
from verify_lintel_structures import raw_output,native

def main():
 fixture=ROOT/'reconstruction/crates/tg-building-runtime/fixtures/lintel-wall-production-cycle.json'
 frames,resources=frames_with_resources(fixture);assert len(frames)==10
 total=wall_frames=mask_count=0
 for frame,resource in zip(frames,resources):
  assert not frame['faults']and not frame['pending_wall_path_masks']
  for output in frame['wall_lintels']:
   s=output['source'];prepared=original_prepare(s['mapped_positions'],s['relative_heights'],s['observed_path_scalar'],resource[1],resource[0])
   for key,values in [('points',[value for point in s['prepared_positions']for value in point]),('points_u',s['prepared_u']),('length',[s['prepared_length']]),('segments',s['prepared_segment_values'])]:assert prepared[key]==packed(values).hex()
   original_cache(s)
   raw=next(row for row in frame['wall_path_masks']if row['wall_id']==output['wall_id']);processed=next(row for row in frame['processed_wall_masks']if row['wall_id']==output['wall_id']);mask_count+=verify_masks(output,raw,processed)
   sources=[];third=s['third'][:]
   for mask in processed['masks']:
    coords=bytes.fromhex(mask['coords_hex']);case=dict(flags=list(bytes.fromhex(mask['flags_hex'])),coords=[struct.unpack_from('<If',coords,j)for j in range(0,len(coords),8)],loop_range=list(struct.unpack('<3i',bytes.fromhex(mask['range_hex']))),segment_values=s['prepared_segment_values'],arch=s['published_arch'],points=s['prepared_positions'],us=s['prepared_u'],regular=s['shape_tag']<2,closed=s['shape_tag']!=2)
    actual,third=raw_output(case,s['public_284'],s['public_288'],int(output['wall_id']),third,True);sources.extend(actual)
   records=native.produce_support_batch(sources,int(output['wall_id']),s['shape_tag']!=2)
   assert [record.hex()for record in records]==output['records_hex']
   assert packed(third)==packed(s['final_third']);total+=len(records);wall_frames+=1
  for wall in frame['wall_bricks']:verify_wall(wall,resource)
 first=frames[1]['wall_lintels'][0];assert first['records_hex']
 assert first==frames[2]['wall_lintels'][0]
 low=frames[3]['wall_lintels'][0];assert any(a<b for a,b in zip(low['source']['final_third'],first['source']['final_third']))
 assert frames[4]['processed_wall_masks'][0]['steps']and frames[4]['wall_lintels'][0]['records_hex']==[]
 assert all(value==0. for value in frames[4]['wall_lintels'][0]['source']['final_third'])
 assert frames[5]['roofs']==[]and frames[5]['wall_lintels'][0]['records_hex']==first['records_hex']
 assert frames[6]['wall_lintels'][0]['records_hex']==[]and frames[7]['wall_lintels']==[]
 assert frames[8]['wall_lintels'][0]['records_hex']==first['records_hex']and frames[8]['wall_lintels'][0]['source']['final_third']==first['source']['final_third']
 assert frames[9]['wall_lintels']==[]and frames[9]['wall_bricks']
 oracles=[Path(__file__),ROOT/'scripts/multi-version/verify_lintel_structures.py',ROOT/'evidence/building-structures/native_structures.py',ROOT/'scripts/multi-version/verify_wall_production_lifecycle.py',ROOT/'evidence/building-constructor/native_fixture.py',ROOT/'evidence/building-segmentation/native_platform_segmentation.py']
 report={'ok':True,'baseline':'B','frames':len(frames),'wall_frames':wall_frames,'ordered_64byte_records':total,'raw_mask_count':mask_count,'scenarios':frames,'scope':'Real raw special publication/path resource/prepared cache/raw and processed masks -> native materializer/caller height replacement/full support writer/third accumulation, complete output compared each frame. Lowered source scalar, FlatRoof mask elimination, roof removal, path clearing, Wall deletion/recreated ID, special->regular owner cleanup.','excluded':['Other special construction outputs','Normal stone arches/third writers','Original EventReader/fork scheduling/SSBO upload/GPU/A/C'],'fixture_sha256':hashlib.sha256(fixture.read_bytes()).hexdigest(),'candidate_binary_sha256':hashlib.sha256((ROOT/'reconstruction/target/release/building-run').read_bytes()).hexdigest(),'oracle_sources':{str(path.relative_to(ROOT)):hashlib.sha256(path.read_bytes()).hexdigest()for path in oracles},**candidate_provenance()}
 (ROOT/'evidence/building-integrate/B-lintel-wall-runtime-verification.json').write_text(json.dumps(report,indent=2)+'\n');print(json.dumps({'ok':True,'frames':len(frames),'wall_frames':wall_frames,'records':total,'masks':mask_count}))
if __name__=='__main__':main()
