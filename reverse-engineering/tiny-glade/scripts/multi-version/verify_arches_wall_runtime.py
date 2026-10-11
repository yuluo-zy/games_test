"""Actual raw World source/resources -> native preparation/cache/masks/arches -> CPU bytes + game commands."""
import json,hashlib,struct
from pathlib import Path
from verification_provenance import ROOT,candidate_provenance
from verify_wall_production_lifecycle import frames_with_resources,verify_wall,verify_masks,verify_structures,original_prepare,original_cache,packed

def main():
 fixture=ROOT/'reconstruction/crates/tg-building-runtime/fixtures/arches-wall-production-cycle.json';frames,resources=frames_with_resources(fixture);assert len(frames)==12
 previous={};wall_frames=arch_records=gate_count=brick_records=0;coverage=set()
 for frame,resource in zip(frames,resources):
  assert not frame['faults'],frame['faults'];expect_gate=[];current={}
  for entry in frame['wall_arches']:
   wall_frames+=1;s=entry['source'];assert s is not None,(entry['wall_id'],'missing raw source trace');coverage.add(s['shape_tag'])
   prepared=original_prepare(s['mapped_positions'],s['relative_heights'],s['observed_path_scalar'],resource[1],resource[0])
   for key,values in [('points',[v for p in s['prepared_positions']for v in p]),('points_u',s['prepared_u']),('length',[s['prepared_length']]),('segments',s['prepared_segment_values'])]:assert prepared[key]==packed(values).hex()
   original_cache(s)
   raw=next(v for v in frame['wall_path_masks']if v['wall_id']==entry['wall_id']);processed=next(v for v in frame['processed_wall_masks']if v['wall_id']==entry['wall_id']);verify_masks(entry,raw,processed)
   native=verify_structures(entry);assert [r.hex()for r in native['records']]==entry['records_hex'],(entry['wall_id'],'arch instances')
   arch_records+=len(native['records']);current[entry['wall_id']]=entry['generation']
   if previous.get(entry['wall_id'])!=entry['generation']:
    for payload in native['gate_commands']:
     wall,ordinal=struct.unpack('<QQ',payload[-16:]);expect_gate.append(dict(wall_id=str(wall),ordinal=ordinal,projection_hex=payload.hex()))
  assert frame['gate_commands']==expect_gate,(frame['tick'],frame['gate_commands'],expect_gate);gate_count+=len(expect_gate);previous=current
  for wall in frame['wall_bricks']:
   brick_records+=verify_wall(wall,resource)[0]
 assert coverage=={1,2}and arch_records>0 and gate_count>0
 assert not any(v['wall_id']=='12345'for v in frames[8]['wall_arches'])
 assert not any(v['wall_id']=='12345'for v in frames[10]['wall_arches'])
 deps=[Path(__file__),ROOT/'scripts/multi-version/verify_wall_production_lifecycle.py',ROOT/'scripts/multi-version/verify_wall_arches.py',ROOT/'scripts/multi-version/verify_wall_arches_freehand.py',ROOT/'evidence/building-arches/native_arches.py',ROOT/'evidence/building-arches-freehand/native_arches_freehand.py',ROOT/'evidence/building-structures/native_structures.py',ROOT/'evidence/building-production/native_production.py',ROOT/'evidence/building-constructor/native_fixture.py',ROOT/'evidence/building-production-authority/native_path_empty.py',ROOT/'evidence/building-segmentation/native_segmentation.py',ROOT/'evidence/building-segmentation/native_platform_segmentation.py']
 binary=ROOT/'reconstruction/target/release/building-run';report=dict(ok=True,frames=len(frames),arch_wall_frames=wall_frames,arch_records=arch_records,gate_commands=gate_count,normal_brick_records=brick_records,shapes=sorted(coverage),fixture_sha256=hashlib.sha256(fixture.read_bytes()).hexdigest(),strip_sha256=hashlib.sha256((fixture.parent/'arches-path-strip.u8').read_bytes()).hexdigest(),candidate_binary_sha256=hashlib.sha256(binary.read_bytes()).hexdigest(),oracle_sources={str(p.relative_to(ROOT)):hashlib.sha256(p.read_bytes()).hexdigest()for p in deps},scope='Actual raw publication/raster/path/cache and processed masks feeding original arches/freehand profile/walker/cache and wholefrom_curve normalbricks. Entire ordered96B arch/bricks and fullfinalarch/third match; current actualBevy EventWriter ConstructGateCmd120 payloads match native output; dependency EventId excluded. Roof/source/path/idle/remove/recreate/special-transition chain; no gate EventReader/door geometry completion claim.',**candidate_provenance());p=ROOT/'evidence/building-arches-freehand/B-arches-world-verification.json';p.write_text(json.dumps(report,indent=2)+'\n');print(json.dumps({k:report[k]for k in['ok','frames','arch_wall_frames','arch_records','gate_commands','normal_brick_records','shapes']}))
if __name__=='__main__':main()
