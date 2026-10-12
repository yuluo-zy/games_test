"""Actual path-resource and Roof lifecycle inputs, full native wall output oracle."""
import hashlib,json,struct,subprocess,sys
from pathlib import Path
from verification_provenance import ROOT,candidate_provenance
from verify_raw_wall_production_runtime import original_cache,packed
from verify_wall_construction_policy import original as original_policy
from verify_wall_construction_sources import bits
from verify_gable_arch import original as original_gable
sys.path.insert(0,str(ROOT/'evidence/building-production'))
from native_production import produce
sys.path.insert(0,str(ROOT/'evidence/building-production-authority'))
from native_path_empty import prepare as original_prepare
sys.path.insert(0,str(ROOT/'evidence/building-segmentation'))
from native_segmentation import produce as original_masks
from native_platform_segmentation import process as original_platform_masks
import verify_wall_segmentation_material_stats as stats_oracle

def verify_masks(wall,entry,processed):
 s=wall['source'];shape=bytes.fromhex(s['shape_hex'])
 fields=tuple(struct.unpack_from('<f',shape,offset)[0]for offset in (12,20,24))
 expected=original_masks(s['prepared_positions'],s['prepared_segment_values'],s['shape_tag'],int(s['special']),fields)
 actual=entry['masks'];assert len(actual)==len(expected)
 for got,want in zip(actual,expected):
  assert got=={'flags_hex':want['flags'],'coords_hex':want['coords'],'range_hex':want['range'],'length_bits':f'{struct.unpack("<I",bytes.fromhex(want["length"]))[0]:08x}'},(wall['wall_id'],got,want)
 native=stats_oracle;native.cpu.mem_write(native.data,bytes(0x200))
 native.invoke(0x140a88280,[(native.UC_X86_REG_RCX,native.data),(native.UC_X86_REG_R8,int(s['special'])),(native.UC_X86_REG_XMM1,bits(s['prepared_length']))])
 native.invoke(0x140a88440,[(native.UC_X86_REG_RCX,native.data),(native.UC_X86_REG_RDX,len(expected))])
 stats=entry['material_stats']
 for key,offset in [('regular_path_length',0x90),('special_path_length',0x9c)]:assert packed([stats[key]])==bytes(native.cpu.mem_read(native.data+offset,4))
 assert stats['segmentation_count']==struct.unpack('<I',native.cpu.mem_read(native.data+0xac,4))[0]
 roof=None if s['roof_hex']is None else struct.unpack_from('<I',bytes.fromhex(s['roof_hex']),0x30)[0]
 masks,steps=original_platform_masks(s['prepared_positions'],s['prepared_segment_values'],s['shape_tag'],int(s['special']),s['base_maximum_y'],roof,fields)
 projection=lambda mask:{'flags_hex':mask['flags'],'coords_hex':mask['coords'],'range_hex':mask['range'],'length_bits':f'{struct.unpack("<I",bytes.fromhex(mask["length"]))[0]:08x}'}
 assert processed['masks']==[projection(mask)for mask in masks]
 assert processed['steps']==[{'start':step['start'],'end':step['end'],'range_hex':step['range']}for step in steps]
 return len(expected)

def frames_with_resources(fixture):
 operations=json.loads(fixture.read_text());resources=[];current=None
 for op in operations:
  if op['kind']=='set_path_rasters':
   current=([Path(grid['file']).read_bytes()if 'file'in grid else bytes([grid['constant']])*(512*512)for grid in op['grids_u8']],op['sampling_fix'])
  if op['kind']=='step':resources.append(current)
 binary=ROOT/'reconstruction/target/release/building-run';run=subprocess.run([str(binary),str(fixture)],capture_output=True,text=True,check=True)
 frames=[json.loads(line)for line in run.stdout.splitlines()];assert len(frames)==len(resources)
 return frames,resources

def verify_wall(wall,resource):
 s=wall['source'];grids,fix=resource;prepared=original_prepare(s['mapped_positions'],s['relative_heights'],s['observed_path_scalar'],fix,grids)
 for key,values in [('points',[v for p in s['prepared_positions']for v in p]),('points_u',s['prepared_u']),('length',[s['prepared_length']]),('segments',s['prepared_segment_values'])]:assert prepared[key]==packed(values).hex(),(wall['wall_id'],key)
 original_cache(s)
 roof=None if s['roof_hex']is None else bytes.fromhex(s['roof_hex'])
 if roof is not None and struct.unpack_from('<f',roof,0x30)[0]>.1 and struct.unpack_from('<f',roof,0x34)[0]==1:
  result=original_gable(roof,s['prepared_positions'],s['base_maximum_y'],s['holes_length'],s['style']);maximum,heights=result.split(' ')
  assert maximum==f'{bits(s["maximum_y_gable"]):08x}'
  assert heights==','.join(f'{bits(v):08x}'for v in s['structural_arch'])
 else:assert packed(s['structural_arch'])==packed(s['base_arch_cache'])
 verify_structures(wall)
 # All source policy choices are compared to the actual native caller blocks.
 case=(int(wall['wall_id']),len(s['mapped_positions']),bits(s['holes_length']),s['clutter_version'],s['style'],int(s['special']),bits(s['base_height']),bits(s['wall_float']),bits(s['base_maximum_y']),bytes.fromhex(s['shape_hex']),roof,[bits(v)for v in s['published_arch']])
 top=s['top_bounds'];tag=int(top[-1]);top_text='2 - - - - -'if tag==2 else str(tag)+' '+' '.join(f'{bits(v):08x}'for v in top[:5])
 actual=f'{s["kind"]} {s["seed"]} {s["material"]} {bits(s["base_height"]):08x} {bits(s["perturb"]):08x} {bits(s["wall_float"]):08x} {top_text} | '+' '.join(f'{bits(v):08x}'for v in s['arch'])
 assert original_policy(case).rstrip()==actual.rstrip()
 inputs={k:s[k]for k in ['points','us','length','arch','second','third','holes','material','closed','base_height','perturb','width_padding','thickness_scale','wall_float','holes_length','kind','top_bounds','fence_fit']};inputs['seed']=int(s['seed']);inputs['top_bounds']=[*top[:5],tag]
 expected=produce(**inputs);assert [record.hex()for record in expected]==wall['records_hex']
 return len(expected),prepared['observed_counts']

def verify_structures(wall):
 s=wall['source']
 if s['special']:return # Full lintel writer proof lives in its actual World verifier.
 from verify_wall_arches import mask_from_native,original as arches_original
 from verify_wall_arches_freehand import original as free_original
 shape=bytes.fromhex(s['shape_hex']);public=bytes.fromhex(s['public_hex'])
 fields=tuple(struct.unpack_from('<f',shape,o)[0]for o in (12,20,24))
 roof=None if s['roof_hex']is None else struct.unpack_from('<I',bytes.fromhex(s['roof_hex']),0x30)[0]
 masks,_=original_platform_masks(s['prepared_positions'],s['prepared_segment_values'],s['shape_tag'],0,s['base_maximum_y'],roof,fields)
 case=dict(shape=s['shape_tag'],flag=public[0x294],roof=roof is not None,native_event_count=0,wall_float=struct.unpack_from('<f',public,0x284)[0],public_288=struct.unpack_from('<f',public,0x288)[0],clutter=s['clutter_version'],wall_id=int(wall['wall_id']),source_length=s['holes_length'],points=s['prepared_positions'],us=s['prepared_u'],values=s['prepared_segment_values'],arch=s['structural_arch'],third=s['base_third'],masks=[mask_from_native(m)for m in masks])
 native=(free_original if case['shape']==2 else arches_original)(case)
 assert packed(native['arch'])==packed(s['published_arch']),(wall['wall_id'],'final arch writer')
 assert packed(native['third'])==packed(s['third']),(wall['wall_id'],'final third writer')
 return native

def main():
 root=ROOT/'reconstruction/crates/tg-building-runtime/fixtures';summary=[];total=cases=mask_count=0;coverage={}
 for filename,count in [('path-wall-production-cycle.json',6),('roof-wall-production-cycle.json',10),('platform-wall-production-cycle.json',7)]:
  fixture=root/filename;frames,resources=frames_with_resources(fixture);assert len(frames)==count;assert all(not f['faults']and not f['pending_wall_bricks']and not f['pending_wall_path_masks']for f in frames)
  if filename.startswith('path'):
   first=frames[0]['wall_bricks'][0];changed=frames[1]['wall_bricks'][0]
   assert len(changed['source']['prepared_positions'])>len(first['source']['prepared_positions']);assert any(v>0 for v in changed['source']['prepared_segment_values'])
   assert changed==frames[2]['wall_bricks'][0];assert frames[3]['wall_bricks'][0]['records_hex']==first['records_hex'];assert frames[4]['walls']==[]and frames[4]['wall_bricks']==[]
   assert frames[5]['wall_bricks'][0]['records_hex']==changed['records_hex']
   assert frames[1]['wall_path_masks'][0]['mask_count']>0
   assert frames[2]['wall_path_masks']==frames[1]['wall_path_masks']
   assert frames[3]['wall_path_masks'][0]['masks']==frames[0]['wall_path_masks'][0]['masks']
   assert frames[4]['wall_path_masks']==[]
   assert frames[5]['wall_path_masks'][0]['masks']==frames[1]['wall_path_masks'][0]['masks']
  elif filename.startswith('roof'):
   assert {(f['wall_bricks'][0]['source']['kind'],int(f['wall_bricks'][0]['source']['top_bounds'][-1]))for f in frames}>={(0,0),(1,2),(3,2),(0,1),(2,2)}
   assert frames[7]['despawned']==1 and frames[7]['roofs']==[]
   assert frames[7]['wall_bricks'][0]==frames[8]['wall_bricks'][0]==frames[9]['wall_bricks'][0]
  else:
   assert frames[0]['wall_path_masks'][0]['mask_count']==0
   assert frames[1]['processed_wall_masks'][0]['steps']
   assert frames[2]['processed_wall_masks'][0]['masks']and not frames[2]['processed_wall_masks'][0]['steps']
   assert frames[4]['roofs']==[]and not frames[4]['processed_wall_masks'][0]['steps']
   assert frames[4]['processed_wall_masks'][0]['masks']==frames[4]['wall_path_masks'][0]['masks']
   assert frames[5]['processed_wall_masks'][0]['masks']==[]and frames[6]['processed_wall_masks']==[]
  for f,resource in zip(frames,resources):
   for wall in f['wall_bricks']:
    n,observed=verify_wall(wall,resource);total+=n;cases+=1
    entry=next(m for m in f['wall_path_masks']if m['wall_id']==wall['wall_id']);processed=next(m for m in f['processed_wall_masks']if m['wall_id']==wall['wall_id']);mask_count+=verify_masks(wall,entry,processed)
    for name,value in observed.items():coverage[name]=coverage.get(name,0)+value
  summary.append({'fixture':filename,'fixture_sha256':hashlib.sha256(fixture.read_bytes()).hexdigest(),'frames':frames})
 binary=ROOT/'reconstruction/target/release/building-run';oracles=[Path(__file__),ROOT/'scripts/multi-version/verify_raw_wall_production_runtime.py',ROOT/'scripts/multi-version/verify_wall_construction_policy.py',ROOT/'scripts/multi-version/verify_gable_arch.py',ROOT/'evidence/building-production/native_production.py',ROOT/'evidence/building-production-authority/native_path_empty.py',ROOT/'evidence/building-constructor/native_fixture.py',ROOT/'evidence/building-segmentation/native_segmentation.py',ROOT/'scripts/multi-version/verify_wall_segmentation_material_stats.py',ROOT/'evidence/building-segmentation/native_platform_segmentation.py']
 report={'ok':True,'baseline':'B','scenarios':summary,'wall_frames':cases,'ordered_96byte_records':total,'original_mask_frames':cases,'original_masks':mask_count,'observed_path_native_counts':coverage,'scope':'Real Bevy encoded path replacement/restoration and Pitched/Gable direction -> Flat/rim/style -> Pitched -> final Roof deletion. Current native prepared curve/arrays/raw and platform-processed mask flags,coords,range,length/step active fields/material-stat fields/source policy/complete from_curve compared for every wall frame, including same-frame final-cache and CPU consistency.','excluded':['Platform-stairs CPU consumer, mask/arch consumers and subsequent third-height writers not yet accepted','Full corner/skirts/structural outputs','Source create/editor tools covered separately','Original fork scheduling/tasks, GPU, A/C migration'], 'candidate_binary_sha256':hashlib.sha256(binary.read_bytes()).hexdigest(),'fixture_resource_sha256':{'wall-production-path0.u8':hashlib.sha256((root/'wall-production-path0.u8').read_bytes()).hexdigest()},'oracle_sources':{str(p.relative_to(ROOT)):hashlib.sha256(p.read_bytes()).hexdigest()for p in oracles},**candidate_provenance()}
 (ROOT/'evidence/building-integrate/B-wall-production-lifecycle-verification.json').write_text(json.dumps(report,indent=2)+'\n');print(json.dumps({'ok':True,'wall_frames':cases,'records':total,'masks':mask_count,'coverage':coverage}))
if __name__=='__main__':main()
