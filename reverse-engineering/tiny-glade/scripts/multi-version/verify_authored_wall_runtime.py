"""Whole original creators -> actual Bevy source/roof/geometry lifecycle."""
import hashlib,json,struct,sys
from pathlib import Path
from verification_provenance import ROOT,candidate_provenance
from verify_wall_production_lifecycle import frames_with_resources,verify_wall,verify_masks,packed
sys.path.insert(0,str(ROOT/'evidence/building-authoring'))
from native_authoring import creator_research

def main():
 fixture=ROOT/'reconstruction/crates/tg-building-runtime/fixtures/authored-wall-production-cycle.json'
 operations=json.loads(fixture.read_text());frames,resources=frames_with_resources(fixture)
 pending=[];counter=None;settings=None;index=0;previous_frame=None;creation_count=wall_frames=records=masks=0
 for operation in operations:
  if operation['kind']=='configure_wall_authoring':counter=int(operation['id_counter']);settings=operation
  elif operation['kind']=='create_building':pending.append(operation)
  elif operation['kind']=='step':
   frame=frames[index];assert not frame['faults']and not frame['pending_wall_bricks'];index+=1
   for request in pending:
    shape=bytes.fromhex(request['shape_hex']);placement=bytes.fromhex(request['placement_hex']);hint=placement[44]!=2;hint_id=struct.unpack_from('<Q',placement,32)[0]
    previous=None;roof=None
    if hint and previous_frame is not None:
     host=next((host for host in previous_frame['walls']if int(host['wall_id'])==hint_id),None)
     if host is not None:
      source=next(source for source in previous_frame['authoring']['walls']if int(source['wall_id'])==hint_id)
      previous=(host['source_point_count'],host['source_curve_length'],struct.unpack_from('<I',bytes.fromhex(source['shape_hex']))[0],source['style'],source['wall_color_id'],source['timber_color_id'])
     roof_row=next((roof for roof in previous_frame['roofs']if int(roof['wall_id'])==hint_id),None)
     if roof_row is not None:roof=bytes.fromhex(roof_row['record_hex'])
    elevation,height=struct.unpack_from('<ff',placement,60);tag=struct.unpack_from('<I',shape)[0]
    native=creator_research(tag,counter,shape,height,elevation,settings['wall_style'],settings['rim_style'],settings['roof_color_id'],previous,roof,hint,settings['wall_color_id'],settings['timber_color_id'])
    counter=native['id'];actual=next(source for source in frame['authoring']['walls']if int(source['wall_id'])==counter);original=bytes.fromhex(native['source']);curve=native['curve']
    assert packed([value for point in actual['points']for value in point]).hex()==curve['points']
    assert packed(actual['us']).hex()==curve['us']and struct.unpack('<I',packed([actual['length']]))[0]==curve['length']
    for key,offset in [('height',0x54),('elevation',0x58),('path_scalar',0x5c)]:assert packed([actual[key]])==original[offset:offset+4]
    assert actual['style']==original[0x68]and actual['special']==bool(original[0x65])and actual['clutter_version']==original[0x66]
    active=20 if tag==0 else 28;assert bytes.fromhex(actual['shape_hex'])[:active]==original[0x38:0x38+active]
    event=bytes.fromhex(next(row['record_hex']for row in frame['authoring']['creation_roof_requests']if int(row['wall_id'])==counter));expected=bytes.fromhex(native['roof'])
    assert event[8:21]==expected[8:21]and event[32:58]==expected[32:58]
    if expected[20]:assert event[20:32]==expected[20:32]
    assert struct.unpack_from('<I',event,16)[0]==actual['roof_color_id']
    assert native['history_pending_count']==2;creation_count+=1
   pending=[];assert frame['authoring']['id_counter']==str(counter)
   for wall in frame['wall_bricks']:
    count,_=verify_wall(wall,resources[index-1]);records+=count;wall_frames+=1
    raw=next(row for row in frame['wall_path_masks']if row['wall_id']==wall['wall_id']);processed=next(row for row in frame['processed_wall_masks']if row['wall_id']==wall['wall_id']);masks+=verify_masks(wall,raw,processed)
   previous_frame=frame
 assert creation_count==3 and len(frames)==8
 assert frames[1]['authoring']['walls'][1]['wall_color_id']==7
 assert frames[3]['marked']==1 and frames[3]['despawned']==1
 assert frames[6]['marked']==2 and frames[6]['despawned']==2
 for frame in frames[-2:]:assert frame['authoring']['walls']==[]and frame['roofs']==[]and frame['wall_bricks']==[]and frame['wall_path_masks']==[]and frame['processed_wall_masks']==[]
 oracles=[Path(__file__),ROOT/'evidence/building-authoring/native_authoring.py',ROOT/'scripts/multi-version/verify_wall_production_lifecycle.py',ROOT/'evidence/building-constructor/native_fixture.py']
 report={'ok':True,'baseline':'B','creation_cases':creation_count,'frames':len(frames),'wall_frames':wall_frames,'ordered_96byte_records':records,'raw_mask_count':masks,'fixture_sha256':hashlib.sha256(fixture.read_bytes()).hexdigest(),'scenarios':frames,'scope':'Actual accepted creation with explicit original startup/settings and placement -> original whole creator RET source+Roof request oracle -> Bevy raw/public/caches/raw and platform masks/openings/bricks; existing-host appearance inheritance and wall deletion -> invalid Roof event -> fade/despawn; no stale owners after idle update.','excluded':['UI acceptance and automatic ShapeHeight preview production','Native fork scheduling/task graph','Complete Wall creation/deletion compound History encoding/replay; pending buffers are not commits','Circle editor, rotation/translate and all authoring tools','Full structural/GPU/A/C acceptance'],'candidate_binary_sha256':hashlib.sha256((ROOT/'reconstruction/target/release/building-run').read_bytes()).hexdigest(),'oracle_sources':{str(path.relative_to(ROOT)):hashlib.sha256(path.read_bytes()).hexdigest()for path in oracles},**candidate_provenance()}
 (ROOT/'evidence/building-integrate/B-authored-wall-runtime-verification.json').write_text(json.dumps(report,indent=2)+'\n')
 print(json.dumps({'ok':True,'creations':creation_count,'frames':len(frames),'wall_frames':wall_frames,'records':records,'masks':masks}))
if __name__=='__main__':main()
