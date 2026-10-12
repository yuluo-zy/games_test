"""B authoring authority sections; no private business-output bridges."""
import argparse,hashlib,json,math,random,struct,subprocess,sys
from pathlib import Path
from verification_provenance import ROOT
sys.path.insert(0,str(ROOT/'evidence/building-authoring'))
from native_authoring import create,mutate,ui_event,remove_source,inactive_mode,creator_research,appearance,edit_rectangle,invalid_roof_notifications,nf

def bits(f):return struct.unpack('<I',struct.pack('<f',f))[0]
def main():
 parser=argparse.ArgumentParser();parser.add_argument('--no-report',action='store_true');a=parser.parse_args();rng=random.Random(20261023)
 probe=ROOT/'reconstruction/target/release/wall-authoring-probe'
 source_paths=['reconstruction/building/roof_creation.rs','reconstruction/roof/parameters.rs','reconstruction/building/roof_invalid_notifications.rs','reconstruction/history/building_transactions.rs','reconstruction/building/wall_authoring.rs','reconstruction/src/bin/wall-authoring-probe.rs','reconstruction/building/wall_freeform_cache.rs','reconstruction/building/wall_collection.rs','reconstruction/building/wall_state.rs','reconstruction/roof/surface.rs','reconstruction/Cargo.lock']
 before={p:hashlib.sha256((ROOT/p).read_bytes()).hexdigest()for p in source_paths};binary_hash=hashlib.sha256(probe.read_bytes()).hexdigest()
 creations=[];lines=[]
 edge=[0,0x80000000,0x3f000000,0xc02ccccd,0x7f800000,0xff800000,0x7fc00001]
 for i in range(256):
  if i%2:
   angle=rng.uniform(-3,3);shape=struct.pack('<I6f',1,math.cos(angle),math.sin(angle),rng.uniform(-50,50),rng.uniform(-50,50),rng.uniform(.15,12),rng.uniform(.15,12))
  else:shape=struct.pack('<I6f',0,rng.uniform(-50,50),rng.uniform(-50,50),rng.uniform(.03,8),rng.uniform(-25,25),0.,0.)
  placement=bytearray(rng.randbytes(80));placement[0x2c]=i%3;height=edge[i%len(edge)]if i<64 else bits(rng.uniform(.5,14));elevation=edge[(i+3)%len(edge)]if i<64 else bits(rng.uniform(-2.7,20));struct.pack_into('<II',placement,0x3c,elevation,height)
  counter=[0,100000,0xffffffff,0xffffffffffffffff][i%4]if i<64 else rng.getrandbits(64);style=i%2
  creations.append((counter,style,shape,bytes(placement)));lines.append(f'create {counter} {style} {shape.hex()} {placement.hex()}')
 rows=subprocess.run([str(probe)],input='\n'.join(lines)+'\n',capture_output=True,text=True,check=True).stdout.splitlines();assert len(rows)==len(creations)*2
 failed=[];records=0
 for i,case in enumerate(creations):
  want=create(*case);counter,style,shape,placement=case;fields=rows[2*i].split();source=bytes.fromhex(want['source']);records+=want['n'];active=20 if struct.unpack_from('<I',shape)[0]==0 else 28
  expected=[str(want['wall']),str(want['counter']),str(want['wall']),str(want['wall']),f'{struct.unpack_from("<I",source,0x54)[0]:08x}',f'{struct.unpack_from("<I",source,0x58)[0]:08x}',f'{struct.unpack_from("<I",source,0x5c)[0]:08x}',str(source[0x68]),str(source[0x65]),str(source[0x66])]
  diffs=[]
  if fields[:10]!=expected:diffs.append('source_fields_or_id')
  if bytes.fromhex(fields[10])[:active]!=source[0x38:0x38+active]:diffs.append('active_enclosure')
  if fields[11]!=want['points']:diffs.append('raw_curve_points')
  if fields[12]!=f'{want["length"]:08x}':diffs.append('raw_curve_length')
  if fields[13]!=str(source[0x64]):diffs.append('source_flag64')
  if rows[2*i+1]!='u '+want['us']:diffs.append('raw_curve_u')
  if diffs:failed.append({'case':i,'input':lines[i],'different_fields':diffs,'native':want,'rust':rows[2*i:2*i+2]})
 event_lines=[f'event {shape.hex()} {placement.hex()}'for counter,style,shape,placement in creations]
 event_rows=subprocess.run([str(probe)],input='\n'.join(event_lines)+'\n',capture_output=True,text=True,check=True).stdout.splitlines();assert len(event_rows)==len(creations)
 for i,((counter,style,shape,placement),got)in enumerate(zip(creations,event_rows)):
  want=ui_event(shape,placement)
  if want!=got:failed.append({'event_case':i,'input':event_lines[i],'native':want,'rust':got})
 preset_lines=[f'preset {shape.hex()} {i%3} none'for i,(counter,style,shape,placement)in enumerate(creations)]
 presets=subprocess.run([str(probe)],input='\n'.join(preset_lines)+'\n',capture_output=True,text=True,check=True).stdout.splitlines();assert len(presets)==len(creations)
 full_records=0;full_case_summaries=[]
 for i,(counter,style,shape,placement)in enumerate(creations):
  elevation,height=struct.unpack_from('<ff',placement,0x3c);tag=struct.unpack_from('<I',shape)[0]
  original=creator_research(tag,counter,shape,height,elevation,style,i%3,i%32);source=bytes.fromhex(original['source']);roof=bytes.fromhex(original['roof']);full_records+=original['curve']['n']
  fields=rows[2*i].split();diffs=[]
  if fields[11]!=original['curve']['points']or rows[2*i+1]!='u '+original['curve']['us']:diffs.append('full_creator_curve')
  if fields[:2]!=[str(original['id']),str(original['id'])]or source[0x68]!=style:diffs.append('id_or_style')
  if source[0x54:0x5c]!=placement[0x40:0x44]+placement[0x3c:0x40]:diffs.append('source_height_elevation')
  if bytes.fromhex(presets[i])[:26]!=roof[32:58]:diffs.append('roof_parameters')
  if struct.unpack_from('<Q',roof,8)[0]!=original['id']or struct.unpack_from('<I',roof,16)[0]!=i%32 or roof[20]!=0:diffs.append('roof_event_binding')
  if original['notifications']!=[(nf.data+0x30,original['id'])]+[(nf.data+0x60,original['id'])]*3:diffs.append('new_update_collections')
  if original['history_pending_count']!=2:diffs.append('actual_wall_and_roof_history')
  full_case_summaries.append({'case':i,'actual_return':True,'wall_id':original['id'],'point_count':original['curve']['n'],'roof_active_parameters':roof[32:58].hex(),'history_pending_count':original['history_pending_count']})
  if diffs:failed.append({'full_creator_case':i,'input':lines[i],'different_fields':diffs,'native':original,'rust_preset':presets[i]})
 mutations=[];mutation_lines=[]
 for i in range(1024):
  kind=['elevation','enclosure','style'][i%3];wall=rng.getrandbits(64);present=i%8!=7;source=rng.randbytes(112)
  payload=struct.pack('<I',edge[i%len(edge)]if i<128 else rng.getrandbits(32))if kind=='elevation'else rng.randbytes(28)if kind=='enclosure'else bytes([rng.randrange(256)])
  token=str(payload[0])if kind=='style'else f'{struct.unpack("<I",payload)[0]:08x}'if kind=='elevation'else payload.hex()
  mutations.append((present,wall,source,kind,payload));mutation_lines.append(f'mutate {kind} {int(present)} {wall} {source.hex()} {token}')
 mutation_rows=subprocess.run([str(probe)],input='\n'.join(mutation_lines)+'\n',capture_output=True,text=True,check=True).stdout.splitlines();assert len(mutation_rows)==len(mutations)
 for i,(case,got)in enumerate(zip(mutations,mutation_rows)):
  want=mutate(*case)
  if want!=got:failed.append({'mutation_case':i,'input':mutation_lines[i],'native':want,'rust':got})
 removals=[];remove_lines=[]
 for i in range(256):
  wall=rng.getrandbits(64);source=bytearray(rng.randbytes(112));struct.pack_into('<Q',source,0,i%31);present=i%4!=3
  removals.append((present,wall,bytes(source)));remove_lines.append(f'remove {int(present)} {wall} {source.hex()}')
 remove_rows=subprocess.run([str(probe)],input='\n'.join(remove_lines)+'\n',capture_output=True,text=True,check=True).stdout.splitlines();assert len(remove_rows)==len(removals)
 for i,(case,got)in enumerate(zip(removals,remove_rows)):
  want=remove_source(*case)
  if want!=got:failed.append({'removal_case':i,'input':remove_lines[i],'native':want,'rust':got})
 inactive_cases=[];inactive_lines=[]
 for i in range(128):
  tag=i%2;mode=[0,6,7,0xffffffff][i%4];subtype=[0,3,255][i%3];preview=rng.randbytes(40)
  inactive_cases.append((mode,subtype,tag,preview));inactive_lines.append(f'inactive {mode} {subtype} {tag} {preview.hex()}')
 inactive_rows=subprocess.run([str(probe)],input='\n'.join(inactive_lines)+'\n',capture_output=True,text=True,check=True).stdout.splitlines();assert len(inactive_rows)==len(inactive_cases)
 for i,(case,got)in enumerate(zip(inactive_cases,inactive_rows)):
  want=inactive_mode(*case)
  if want!=got:failed.append({'inactive_case':i,'input':inactive_lines[i],'native':want,'rust':got})
 appearances=[];appearance_lines=[]
 for i in range(512):
  settings=(rng.getrandbits(32),rng.getrandbits(32),rng.getrandbits(32),i%4)
  previous=None if i%8==0 else(rng.randrange(2,300),float(i%31+1),i%4,i%256,rng.getrandbits(32),rng.getrandbits(32))
  if previous is None:pub='none'
  else:
   n,l,t,st,w,ti=previous;pub=struct.pack('<QfIB3xII4x',n,l,t,st,w,ti).hex()
  roof=None
  if i%3:
   roof=bytearray(rng.randbytes(88));roof[0x40]=i%2;roof=bytes(roof)
  appearances.append((settings,previous,roof));appearance_lines.append('appearance '+' '.join(map(str,settings))+' '+pub+' '+('none'if roof is None else roof.hex()))
 appearance_rows=subprocess.run([str(probe)],input='\n'.join(appearance_lines)+'\n',capture_output=True,text=True,check=True).stdout.splitlines();assert len(appearance_rows)==len(appearances)
 for i,(case,got)in enumerate(zip(appearances,appearance_rows)):
  want=appearance(*case)
  if want!=got:failed.append({'appearance_case':i,'input':appearance_lines[i],'native':want,'rust':got})
 contexts=[];context_lines=[]
 for i in range(256):
  counter,style,shape,placement=creations[i];placement=bytearray(placement);struct.pack_into('<Q',placement,0x20,9);placement[0x2c]=2 if i%8==0 else i%2
  settings=(rng.getrandbits(32),rng.getrandbits(32),rng.getrandbits(32),i%4,i%3)
  previous=None if i%8 in[0,1] else(rng.randrange(2,300),float(i%31+1),i%4,i%4,rng.getrandbits(32),rng.getrandbits(32))
  roof=None
  if i%4!=0:
   roof=bytearray(rng.randbytes(88));roof[0x3d]=i%3;roof[0x40]=i%2
   struct.pack_into('<I',roof,0x30,[0,0x3dcccccd,0x3dccccce,0xbdcccccd,0x7fc00001,0x7f800000,0xff800000][i%7]);roof=bytes(roof)
  pub='none'if previous is None else struct.pack('<QfIB3xII4x',*previous).hex()
  contexts.append((counter,shape,bytes(placement),settings,previous,roof))
  context_lines.append(f'building {counter} {shape.hex()} {placement.hex()} '+' '.join(map(str,settings))+' '+pub+' '+('none'if roof is None else roof.hex()))
 context_rows=subprocess.run([str(probe)],input='\n'.join(context_lines)+'\n',capture_output=True,text=True,check=True).stdout.splitlines();assert len(context_rows)==len(contexts)
 context_records=0;context_summaries=[]
 for i,((counter,shape,placement,settings,previous,roof),got)in enumerate(zip(contexts,context_rows)):
  w,r,t,style,rim=settings;elevation,height=struct.unpack_from('<ff',placement,0x3c);hint=placement[0x2c]!=2;tag=struct.unpack_from('<I',shape)[0]
  original=creator_research(tag,counter,shape,height,elevation,style,rim,r,previous,roof,hint,w,t);source=bytes.fromhex(original['source']);record=bytes.fromhex(original['roof']);fields=got.split();active=20 if tag==0 else 28;context_records+=original['curve']['n'];diffs=[]
  selected=appearance((w,r,t,style),previous if hint else None,roof if hint else None).split()
  expected=[str(original['id'])]*2+[str(source[0x68])]+[f'{struct.unpack_from("<I",source,p)[0]:08x}'for p in[0x54,0x58,0x5c]]
  if fields[:6]!=expected:diffs.append('context_raw_fields_or_id')
  if fields[14]!=str(source[0x64]):diffs.append('context_source_flag64')
  if bytes.fromhex(fields[6])[:active]!=source[0x38:0x38+active]:diffs.append('context_enclosure')
  if fields[7]!=original['curve']['points']or fields[9]!=original['curve']['us']or fields[8]!=f'{original["curve"]["length"]:08x}':diffs.append('context_curve')
  if fields[10:13]!=selected[:3]:diffs.append('context_selected_colours')
  rust_roof=bytes.fromhex(fields[13]);active_roof=record[8:21]+(record[21:32]if record[20]else bytes(11))+record[32:58]
  active_rust=rust_roof[8:21]+(rust_roof[21:32]if rust_roof[20]else bytes(11))+rust_roof[32:58]
  if active_roof!=active_rust:diffs.append('context_roof_record')
  if original['notifications']!=[(nf.data+0x30,original['id'])]+[(nf.data+0x60,original['id'])]*3 or original['history_pending_count']!=2:diffs.append('context_owner_side_effects')
  context_summaries.append({'case':i,'actual_return':True,'hint':hint,'public_shape':None if previous is None else previous[2],'has_roof':roof is not None,'point_count':original['curve']['n'],'roof_active':active_roof.hex()})
  if diffs:failed.append({'context_case':i,'input':context_lines[i],'different_fields':diffs,'native':original,'rust':got})
 edits=[];edit_lines=[]
 for i in range(512):
  _,_,rectangle,_=creations[(i%128)*2+1];wall=rng.getrandbits(64);source=rng.randbytes(112);present=i%4!=3
  def event(j):
   e=bytearray(rng.randbytes(64));struct.pack_into('<QQ',e,0,j,wall);e[16:40]=rectangle[4:28];e[40:52]=struct.pack('<3f',rng.uniform(-100,100),rng.uniform(-10,10),rng.uniform(-100,100));struct.pack_into('<I',e,52,edge[(i+j)%len(edge)]if i<128 else bits(rng.uniform(-10,100)));e[56]=(i+j)%2;return bytes(e)
  old=[event(j)for j in range(1+i%3)];new=[event(10+j)for j in range(i%4)];total=len(old)+len(new);cursor=i%total;selected=(new or old)[-1];dt=edge[(i+3)%len(edge)]if i<128 else bits(rng.uniform(.001,.2))
  edits.append((present,old,new,cursor,source,dt));edit_lines.append(f'edit {int(present)} {selected.hex()} {source.hex()} {dt:08x}')
 edit_rows=subprocess.run([str(probe)],input='\n'.join(edit_lines)+'\n',capture_output=True,text=True,check=True).stdout.splitlines();assert len(edit_rows)==len(edits)
 edit_summaries=[];edit_records=0
 for i,(case,got)in enumerate(zip(edits,edit_rows)):
  present,old,new,cursor,source,dt=case;original=edit_rectangle(*case);fields=got.split();selected=(new or old)[-1];wall=struct.unpack_from('<Q',selected,8)[0];material=bytes.fromhex(original['material']);motion=bytes.fromhex(original['motion']);diffs=[]
  if fields[:2]!=[str(wall),str(int(present))]:diffs.append('edit_id_or_dirty')
  if present:
   state=bytes.fromhex(original['source']);curve=original['curve'];edit_records+=curve['n']
   if fields[2]!=state[0x38:0x54].hex():diffs.append('edit_shape')
   if fields[3]!=curve['points']or fields[5]!=curve['us']or fields[4]!=f'{curve["length"]:08x}':diffs.append('edit_curve')
   if state[0x54:]!=source[0x54:]:diffs.append('native_edit_metadata_changed')
  if fields[15]!='4' or fields[6]!=material[16:28].hex()or material[28:30]!=struct.pack('<H',4)or motion[16:28]!=material[16:28]:diffs.append('edit_audio_anchor_kind')
  if fields[7]!=f'{struct.unpack_from("<I",motion,28)[0]:08x}'or fields[8]!=str(motion[32]):diffs.append('edit_motion_audio')
  expected=[f'{struct.unpack_from("<I",source,p)[0]:08x}'for p in[0x54,0x58,0x5c]]+[str(source[0x68]),str(int(source[0x65]!=0)),str(source[0x66])]
  if fields[9:15]!=expected or fields[16]!=str(source[0x64]):diffs.append('edit_rust_metadata_changed')
  if original['cursor']!=len(old)+len(new):diffs.append('edit_reader_last')
  edit_summaries.append({'case':i,'actual_return':True,'present':present,'old_count':len(old),'new_count':len(new),'cursor_before':cursor,'cursor_after':original['cursor'],'material_active':material[8:30].hex(),'motion_active':motion[8:33].hex()})
  if diffs:failed.append({'edit_case':i,'input':edit_lines[i],'different_fields':diffs,'native':original,'rust':got})
 empty_edit_cases=[]
 for i in range(64):
  n=i%4;events=[struct.pack('<QQ6f4fB7x',j,9,1,0,0,0,2,3,0,0,0,1,0)for j in range(n)];original=edit_rectangle(True,events,[],n,bytes(112),bits(.1));assert not original['updated']and not original['notifications']and original['cursor']==n;empty_edit_cases.append({'case':i,'actual_return':True,'total':n,'cursor':n})
 invalid_roof_cases=[];invalid_roof_lines=[]
 for i in range(192):
  walls=[9,10,11,12][:i%5];roofs=[]
  for wall in walls:
   roof=bytearray(rng.randbytes(88));struct.pack_into('<Q',roof,0,wall);roofs.append(bytes(roof))
  invalid=[9,9,11,99]if i%3==0 else[10,12]if i%3==1 else[];marked=i%2!=0;reset=i%8==0
  original=invalid_roof_notifications(roofs,invalid,marked,reset)
  want=walls if reset else[]if marked else[w for w in walls if w in invalid]
  got=[struct.unpack_from('<Q',bytes.fromhex(e),8)[0]for e in original['events']]
  assert got==want and original['history_count']==(0 if reset else len(want))
  assert all(bytes.fromhex(e)[16]==0 for e in original['events'])
  invalid_roof_lines.append(f'invalid {int(marked)} {int(reset)} '+(','.join(r.hex()for r in roofs)or'-')+' '+(','.join(map(str,invalid))or'-'))
  invalid_roof_cases.append({'case':i,'inputs':{'walls':walls,'roof_rows':[r.hex()for r in roofs],'invalid_ids':invalid,'marked':marked,'reset_all':reset},'native':original})
 invalid_roof_rows=subprocess.run([str(probe)],input='\n'.join(invalid_roof_lines)+'\n',capture_output=True,text=True,check=True).stdout.splitlines();assert len(invalid_roof_rows)==len(invalid_roof_cases)
 for i,(case,line)in enumerate(zip(invalid_roof_cases,invalid_roof_rows)):
  native=case['native'];parts=line.split('|')if line else[];diffs=[]
  if len(parts)!=len(native['events']):diffs.append('invalid_roof_event_count')
  else:
   for j,(part,event)in enumerate(zip(parts,native['events'])):
    wall,flag,payload=part.split(':');event=bytes.fromhex(event)
    if [wall,flag]!=[str(struct.unpack_from('<Q',event,8)[0]),str(event[16])]:diffs.append('invalid_roof_event_fields')
    if case['inputs']['reset_all']:
     if payload!='-':diffs.append('reset_has_unexpected_history')
    else:
     original=bytes.fromhex(native['history_payloads'][j]);candidate=bytes.fromhex(payload)
     if candidate[0]!=original[0]or candidate[4:56]!=original[4:56]:diffs.append('invalid_roof_history_payload')
  if diffs:failed.append({'invalid_roof_case':i,'input':invalid_roof_lines[i],'different_fields':diffs,'native':native,'rust':line})
 assert before=={p:hashlib.sha256((ROOT/p).read_bytes()).hexdigest()for p in source_paths};assert binary_hash==hashlib.sha256(probe.read_bytes()).hexdigest()
 report={'ok':not failed,'baseline':'B','creation_cases':len(creations),'raw_curve_points':records,'mutation_cases':len(mutations),'failure_count':len(failed),'failures':failed[:16],'inputs':{'creation':lines,'mutation':mutation_lines},'candidate_sources':before,'candidate_binary_sha256':binary_hash,'binary_sha256':nf.b['binary']['sha256'],'oracle_sources':{str(p.relative_to(ROOT)):hashlib.sha256(p.read_bytes()).hexdigest()for p in[Path(__file__),ROOT/'evidence/building-authoring/native_authoring.py',ROOT/'evidence/building-constructor/native_fixture.py']},'executed_business':{'circle_creator_geometry':['0x14097e9c4','0x14097ea3c'],'rectangle_geometry':'0x140c98cf0 entry->actualRET','new_wall_row_and_id':'0x14085a370 entry->actualRET','elevation':'0x140aaaac0 entry->actualRET','enclosure':'0x140aa9610 entry->actualRET','style':'0x140aaa350 entry->actualRET'},'bridges':['Generic HashMap insertion andNone old-value ownership; native game new ID/default payload execute','Generic HashSet dirty insertion andRandomState; native actualID map comparisons/sourcewrites execute','IAT standard allocator/free/realloc/memory/CRT math only; no private shape/curve result bridge'],'limitations':['This proof establishes constructor authority sections, not wholecreate_circle/create_rectangle entry->return','Accepted create event andselected color/style owner are not supplied by this verifier; UI event source pending','Circle enum inactive trailing8bytes andother source padding/metadata not claimed as initialized values','Original History, Roof/Color/audio/tree notifications, editing/cancel/deletion andWorld integration remain pending','A/C behavior remains separate']}
 report['rectangle_edit_cases']=len(edits);report['rectangle_edit_point_records']=edit_records;report['rectangle_edit_inputs']=edit_lines;report['rectangle_edit_summaries']=edit_summaries;report['empty_edit_cases']=empty_edit_cases;report['invalid_roof_native_cases']=invalid_roof_cases
 report['executed_business']['rectangle_edit']='14097f620 full entry->RET including real EventReader::last, actualcurve/enclosure writes, missing-ID no-op, two audio events; GameTime+58 and20/200 scale'
 report['executed_business']['invalid_roof_notifications']='14062eb80 full entry->RET actualQuery optionalMarked, RoofIsInvalid IDs,24B DeleteRoof flag0 andactualHistory RoofDelete append;resetallbranch also proven. Rust differential compares24B event meaningfulfields andactualHistory tag/presence44B/wallID'
 report['context_creator_cases']=len(contexts);report['context_point_records']=context_records;report['context_inputs']=context_lines;report['context_summaries']=context_summaries
 report['ui_event_cases']=len(creations)
 report['full_creator_cases']=len(creations);report['full_creator_point_records']=full_records;report['full_creator_summaries']=full_case_summaries
 report['inactive_mode_cases']=len(inactive_cases)
 report['appearance_cases']=len(appearances)
 report['executed_business']['appearance']='Whole140a94880->RET actualvalid regular/freeform publichost androof EntityQuery/noobject cases; fullactive colour/style/appearance fields'
 report['removal_cases']=len(removals);report['missing_removal_guards']=64
 report['executed_business']['source_remove']='0x14085aa60 completeentry->RET for192present;64 missing-ID actualpanic guards'
 report['executed_business']['whole_creators']='create_circle14097e560/create_rectangle140a93790 fullentry->RET, actual source/geometry/getcolors/Easing/ev_new_wall/History.add; including inherited regular/freeform/no-public host with actual priorRoof EntityQuery orNone'
 report['executed_business']['inactive_ui']='UI Circle14165bed0/UI Rectangle14155d410 whole mode-mismatch entry->RET, exactpreview bytes+tick; no creation event'
 report['limitations'][0]='Wholecreator proof includes no-prior and256 inherited-context cases with activeHistory; randomizedcolor setting remains pending'
 report['limitations'][3]='Candidate Wall/roof geometry andraw state are compared; original Wall+Roof History calls execute and2 pendingrecords are checked, but their complete Wall snapshot encoding/Rust history integration is not claimed. Active drag-cancel/click/border decisions andWorld integration pending; source-remove +90 proven'
 report['executed_business']['ui_create_event']={'circle':['0x14165c5f2','0x14165c712'],'rectangle':['0x14155ddf7','0x14155e391'],'effect':'Actual hint conditional, shape/scalar copy, Event counter andchange tick; allactive fields compared'}
 report['limitations'][1]='Successful UI event source+40/+3c andhint/shape binding is proven; click/border/active drag cancel decisions still pending. Colour/style owner is separatelywholefunction512 cases and256 completecreator inherited-context bindings'
 if not a.no_report:(ROOT/'evidence/building-authoring'/('B-authority-sections-verification.json'if not failed else'B-authority-sections-failure.json')).write_text(json.dumps(report,indent=2)+'\n')
 print(json.dumps({'ok':not failed,'creation_cases':len(creations),'points':records,'mutation_cases':len(mutations),'ui_event_cases':len(creations),'failures':len(failed),'first':[{k:v for k,v in f.items()if k not in['native','rust']}for f in failed[:2]]}));assert not failed,failed[:1]
if __name__=='__main__':main()
