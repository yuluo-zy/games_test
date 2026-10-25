"""Actual World raw/raster/decorator80 -> stairs/holes/arch/complete brick chain."""
import sys,struct,json,hashlib
from verification_provenance import ROOT,candidate_provenance
from verify_wall_production_lifecycle import frames_with_resources,verify_wall,verify_masks,packed
from verify_wall_platform_stair_chain import decode_segment
sys.path.insert(0,str(ROOT/'evidence/building-segmentation'))
from native_platform_blockers_research import evaluate
from native_platform_stair_segments import prepare_steps
from native_platform_partitions import partitions
from native_platform_geometry import produce as cell_produce
from native_platform_holes import produce as hole_produce
from native_platform_rows_research import produce as row_produce

def f32(v):return struct.unpack('<f',struct.pack('<f',v))[0]
def cell_hex(c):
 return packed([c['top'],c['render_bottom']]).hex()+struct.pack('<Q',c['rise_count']).hex()+packed([c['depth']]).hex()+packed([v for p in c['vertices']for v in p]).hex()+struct.pack('<36I',*c['indices']).hex()
def native_cell_hex(c):return c['top']+c['bottom']+struct.pack('<Q',c['rise_count']).hex()+c['depth']+c['vertices']+c['indices']
def verify_stairs(entry,terrain):
 s=entry['source'];points=s['prepared_positions'];us=s['prepared_u'];public=bytes.fromhex(entry['public_hex']);shape=bytes.fromhex(s['shape_hex']);maximum=struct.unpack_from('<f',public,0x10c)[0];extra=struct.unpack_from('<f',public,0x284)[0]
 encoded,lo,hi,fix=terrain;blockers=[]
 for row in entry['decorator_rows_hex']:
  observed=evaluate(s['raw_curve_points'],bytes.fromhex(row),encoded,lo,hi,fix);assert not observed.get('invalid_extra')
  blockers.extend([(list(v[:2]),v[2])for v in struct.iter_unpack('<3f',bytes.fromhex(observed['payload']))])
 fields=tuple(struct.unpack_from('<f',shape,o)[0]for o in (12,20,24))
 segments,steps=prepare_steps(s['raw_curve_points'],points,us,s['prepared_segment_values'],s['structural_arch'],s['shape_tag'],int(s['special']),maximum,blockers,fields)
 cells=[];holes=[];records=[];length=0.
 for segment in segments:
  positions,su=decode_segment(segment);holes.append(hole_produce(s['raw_curve_points'],positions,maximum).hex())
  coords,seg_length=partitions(positions,su,points,us)
  if seg_length>=.5:length=f32(length+seg_length)
  for pair in coords:
   cell=cell_produce(points,pair,maximum,extra,encoded,lo,hi,fix,len(cells)*8);cells.append(native_cell_hex(cell))
   _,rows=row_produce(points,pair,maximum,extra,encoded,lo,hi,fix,shape,int(entry['wall_id']));records.extend(r.hex()for r in rows)
 actual_holes=[packed(h['center']+h['dimensions']).hex()+bytes([h['kind']]).hex()for h in entry['holes']]
 assert actual_holes==holes,(entry['wall_id'],'actual raw staircase holes')
 assert [cell_hex(c)for c in entry['cell_geometry']]==cells,(entry['wall_id'],'geometry/collider/index bytes')
 assert entry['render_records_hex']==records,(entry['wall_id'],'ordered render96')
 assert packed([entry['stairs_length']])==packed([length]),(entry['wall_id'],'stairs length')
 return len(cells),len(holes),len(records),length

def main():
 fixture=ROOT/'reconstruction/crates/tg-building-runtime/fixtures/stair-wall-production-cycle.json';frames,resources=frames_with_resources(fixture);terrain=[];current=None
 for operation in json.loads(fixture.read_text()):
  if operation['kind']=='set_terrain_grid':
   assert 'encoded_u16_constant'in operation # Explicit bound of this World scene; varying native matrix is separate.
   current=([operation['encoded_u16_constant']]*(512*512),operation['minimum_y'],operation['maximum_y'],operation['sampling_fix'])
  if operation['kind']=='step':terrain.append(current)
 total_cells=total_holes=render_records=brick_records=wall_frames=stair_frames=0
 for frame,resource,heightfield in zip(frames,resources,terrain):
  assert not frame['faults']and not frame['pending_wall_bricks'];lengths={}
  for entry in frame['platform_stairs']:
   cells,holes,records,length=verify_stairs(entry,heightfield);total_cells+=cells;total_holes+=holes;render_records+=records;stair_frames+=1;lengths[entry['wall_id']]=length
  for wall in frame['wall_bricks']:
   count,_=verify_wall(wall,resource);brick_records+=count;wall_frames+=1
   raw=next(v for v in frame['wall_path_masks']if v['wall_id']==wall['wall_id']);processed=next(v for v in frame['processed_wall_masks']if v['wall_id']==wall['wall_id'])
   # Native per-update path stats then actual add_stairs_length business method.
   length=lengths.get(wall['wall_id'],0.);stats=raw['material_stats'];assert packed([stats['stairs_length']])==packed([length])
   unadjusted=dict(raw);unadjusted['material_stats']=dict(stats);unadjusted['material_stats']['regular_path_length']=wall['source']['prepared_length']if not wall['source']['special']else 0.
   verify_masks(wall,unadjusted,processed)
   import verify_wall_segmentation_material_stats as native
   native.cpu.mem_write(native.data,bytes(0x200));native.invoke(0x140a88280,[(native.UC_X86_REG_RCX,native.data),(native.UC_X86_REG_R8,int(wall['source']['special'])),(native.UC_X86_REG_XMM1,struct.unpack('<I',packed([wall['source']['prepared_length']]))[0])]);native.invoke(0x140a88590,[(native.UC_X86_REG_RCX,native.data),(native.UC_X86_REG_XMM1,struct.unpack('<I',packed([length]))[0])])
   for key,offset in [('regular_path_length',0x90),('stairs_length',0xa4)]:assert packed([stats[key]])==bytes(native.cpu.mem_read(native.data+offset,4))
 assert len(frames)==9 and len(frames[1]['platform_stairs'])==1 and frames[2]['platform_stairs'][0]['cells']==0
 assert frames[1]['platform_stairs'][0]['render_records_hex']==frames[3]['platform_stairs'][0]['render_records_hex']
 assert frames[1]['wall_bricks'][0]['records_hex']==frames[3]['wall_bricks'][0]['records_hex']
 assert not frames[6]['platform_stairs']and not frames[8]['wall_bricks']and not frames[8]['wall_path_masks']
 paths=[__file__,str(ROOT/'scripts/multi-version/verify_wall_production_lifecycle.py'),str(ROOT/'scripts/multi-version/verify_raw_wall_production_runtime.py')]+[str(ROOT/'evidence/building-segmentation'/n)for n in ['native_platform_blockers_research.py','native_platform_stair_segments.py','native_platform_partitions.py','native_platform_geometry.py','native_platform_holes.py','native_platform_rows_research.py']]
 from pathlib import Path
 report=dict(ok=True,baseline='B',frames=frames,wall_frames=wall_frames,stair_frames=stair_frames,cells=total_cells,holes=total_holes,ordered_stair_records96=render_records,ordered_wall_records96=brick_records,scope='Actual Bevy raw source+encoded path+wall-scoped actual80B decorator records+terrain -> original prepared/cache/calc/platform/blocker/104B/partitions/cell/indices/shape-rises/complete96B stair writer/rawholes and stats, actual opening-storage gathering -> native arches/third -> source policy/full wall from_curve ordered output. Path replacement, manual blocker withdrawal/restoration, height/Roof direction/fade/delete, source delete. No resolved mask/blocker/segment/row input.',excluded=['Original ECS query acquisitions and fork schedule','Manual decorator UI authoring','GPU uploads','C migration and whole building acceptance'],fixture_sha256=hashlib.sha256(fixture.read_bytes()).hexdigest(),candidate_binary_sha256=hashlib.sha256((ROOT/'reconstruction/target/release/building-run').read_bytes()).hexdigest(),oracle_sources={str(Path(p).relative_to(ROOT)):hashlib.sha256(Path(p).read_bytes()).hexdigest()for p in paths},**candidate_provenance())
 (ROOT/'evidence/building-integrate/B-stair-wall-runtime-verification.json').write_text(json.dumps(report,indent=2)+'\n');print(json.dumps({k:report[k]for k in ['ok','wall_frames','stair_frames','cells','holes','ordered_stair_records96','ordered_wall_records96']}))
if __name__=='__main__':main()
