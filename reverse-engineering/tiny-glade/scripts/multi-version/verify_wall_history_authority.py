"""B full Wall constructor/replay business differential, no game result hooks."""
import argparse,hashlib,json,random,struct,subprocess,sys,math
from pathlib import Path
from verification_provenance import ROOT
sys.path.insert(0,str(ROOT/'evidence/building-wall-history'))
import native_wall_history as original

def main():
 parser=argparse.ArgumentParser();parser.add_argument('--no-report',action='store_true');args=parser.parse_args();rng=random.Random(20261026)
 source_paths=['reconstruction/building/wall_history_edit.rs','reconstruction/src/bin/wall-history-authority-probe.rs','reconstruction/building/wall_authoring.rs','reconstruction/building/wall_freeform_cache.rs','reconstruction/building/wall_collection.rs','reconstruction/building/wall_state.rs','reconstruction/roof/surface.rs','reconstruction/Cargo.lock']
 hashes={p:hashlib.sha256((ROOT/p).read_bytes()).hexdigest()for p in source_paths};probe=ROOT/'reconstruction/target/release/wall-history-authority-probe';probe_hash=hashlib.sha256(probe.read_bytes()).hexdigest()
 cases=[];lines=[]
 for i in range(512):
  if i%2:
   a=rng.uniform(-3,3);shape=struct.pack('<I6f',1,math.cos(a),math.sin(a),rng.uniform(-40,40),rng.uniform(-40,40),rng.uniform(.001,.09)if i<64 else rng.uniform(.1,12),rng.uniform(.001,.09)if i<64 else rng.uniform(.1,12))
  else:shape=struct.pack('<I6f',0,rng.uniform(-40,40),rng.uniform(-40,40),rng.uniform(.001,.02)if i<64 else rng.uniform(.03,12),rng.uniform(-50,50),0,0)
  tail=bytearray(rng.randbytes(56));tail[:28]=shape;struct.pack_into('<I',tail,0,i%4)
  if i%4==2:tail[:28]=struct.pack('<I',2)+bytes(24)
  if i%4==3:tail[:28]=struct.pack('<I',3)+bytes(24)
  tail[44]=i%2;tail[45]=i%2;tail[46]=i%3;tail[47]=(i//2)%2;tail[48]=i%4;tail[49]=[0,5,255][i%3]
  wall=rng.getrandbits(64);color=rng.getrandbits(32);cases.append((shape,bytes(tail),wall,color));lines.append(f'new {shape.hex()} {tail.hex()} {wall} {color}')
 rows=subprocess.run([str(probe)],input='\n'.join(lines)+'\n',capture_output=True,text=True,check=True).stdout.splitlines();assert len(rows)==len(cases)
 failures=[];summaries=[]
 for i,(case,line)in enumerate(zip(cases,rows)):
  result=original.new_wall_owner(*case);got=json.loads(line);want=result['groups']
  if got!=want:failures.append({'case':i,'input':lines[i],'native':want,'rust':got})
  summaries.append({'case':i,'actual_return':result['actual_return'],'group_lengths':[len(g)for g in want],'seed_point_count':want[0][0]['source']['curve']['n'],'full_point_count':want[1][0]['next']['curve']['n']})
 deletion_lines=[line.replace('new ','delete ',1)for line in lines[:256]]
 deletion_rows=subprocess.run([str(probe)],input='\n'.join(deletion_lines)+'\n',capture_output=True,text=True,check=True).stdout.splitlines();assert len(deletion_rows)==len(deletion_lines)
 for i,(case,line)in enumerate(zip(cases[:256],deletion_rows)):
  result=original.deleted_wall_owner(*case);got=json.loads(line)
  if got!=result['groups']:failures.append({'delete_case':i,'input':deletion_lines[i],'native':result['groups'],'rust':got})
 instant_cases=[];instant_lines=[]
 for i,case in enumerate(cases[:128]):
  shape,tail,wall,color=case
  for kind,index in[('new',0),('new',1),('delete',0)]:
   for undo in[False,True]:
    restore=(kind=='new'and index==0 and not undo)or(kind=='delete'and undo)
    present=not restore
    if kind=='new'and index==1:present=i%4!=3
    instant_cases.append((case,kind,index,undo,present));instant_lines.append(f'instant {kind} {index} {int(undo)} {int(present)} {shape.hex()} {tail.hex()} {wall} {color}')
 instant_rows=subprocess.run([str(probe)],input='\n'.join(instant_lines)+'\n',capture_output=True,text=True,check=True).stdout.splitlines();assert len(instant_rows)==len(instant_cases)
 instant_summaries=[]
 for i,((case,kind,index,undo,present),line)in enumerate(zip(instant_cases,instant_rows)):
  shape,tail,wall,color=case;ctor=(original.new_wall_owner if kind=='new'else original.deleted_wall_owner)(*case);arc=original.owner_arcs(ctor['owner_pointer'])[index][0]
  result=original.instant_process(arc,undo,ctor['source_pointer'],wall,present);events=result['events'];notifications=result['notifications']
  selected_color=None;audio=None
  for event in events:
   if event['count']:
    b=bytes.fromhex(event['bytes']);assert event['count']==1
    if event['slot']in[1,2]:selected_color={'wall':struct.unpack_from('<Q',b,8)[0],'color':struct.unpack_from('<I',b,16)[0],'slot':event['slot']};assert b[20]==0
    elif event['slot']==3:audio={'wall':struct.unpack_from('<Q',b,8)[0],'anchor':b[16:28].hex(),'kind':b[28],'flag':b[29]}
    else:raise AssertionError('Unexpected height event ininstantprocess')
  want={'source_count':result['source_count'],'source':result.get('source'),'new':any(p==original.data+0x30 for p,w in notifications),'updated':any(p==original.data+0x60 for p,w in notifications),'removed':any(p==original.data+0x90 for p,w in notifications),'color':selected_color,'audio':audio}
  got=json.loads(line)
  if got!=want:failures.append({'instant_case':i,'input':instant_lines[i],'native':want,'rust':got})
  assert result['counter']==100 and result['finished']==1 and all(w==wall for p,w in notifications)
  instant_summaries.append({'case':i,'actual_return':True,'kind':kind,'group':index,'undo':undo,'present':present,'counter':100,'notifications':notifications,'events':events})
 expansion_summaries=[]
 for i,case in enumerate(cases[:64]):
  for undo in[False,True]:
   for instant in[False,True]:
    ctor=original.new_wall_owner(*case);result=original.expand_owner(ctor['owner_pointer'],undo,instant);assert result['actual_return'];expansion_summaries.append({'case':i,'undo':undo,'instant':instant,'actual_return':True,'native':result})
 assert hashes=={p:hashlib.sha256((ROOT/p).read_bytes()).hexdigest()for p in source_paths};assert probe_hash==hashlib.sha256(probe.read_bytes()).hexdigest()
 report={'deletion_constructor_cases':len(deletion_lines),'instant_authority_cases':len(instant_cases),'instant_inputs':instant_lines,'instant_cases':instant_summaries,'whole_expansion_cases':len(expansion_summaries),'expansion_cases':expansion_summaries,'ok':not failures,'baseline':'B','source_constructor_cases':len(cases),'full_constructor_return_cases':len(cases),'failures':failures[:16],'failure_count':len(failures),'inputs':lines,'cases':summaries,'candidate_sources':hashes,'candidate_binary_sha256':probe_hash,'original_binary_sha256':original.nf.b['binary']['sha256'],'oracle_sources':{str(p.relative_to(ROOT)):hashlib.sha256(p.read_bytes()).hexdigest()for p in[Path(__file__),ROOT/'evidence/building-wall-history/native_wall_history.py',ROOT/'evidence/building-authoring/native_authoring.py',ROOT/'evidence/building-constructor/native_fixture.py']},'executed_business':'Complete ev_new_wall140a83300 -> actualRET, includingoriginalcrashy_slice,copy/clones,ConcurrentEdits two-group build andHistory.add; geometry originalprivatecreator helper','limitations':['Creation/deletion snapshot andinstant source authorities proven; timed state-change replay andmixed ledger integration remainpending','G01 source+69 globalstartup producer unresolved,metadata isrequired observedinput','No A/C equivalence ororiginalBevy fork schedule claim']}
 if not args.no_report:(ROOT/'evidence/building-wall-history'/('B-constructor-verification.json'if not failures else'B-constructor-failure.json')).write_text(json.dumps(report,indent=2)+'\n')
 print(json.dumps({'ok':not failures,'cases':len(cases),'failure_count':len(failures),'first':failures[:1]}));assert not failures
if __name__=='__main__':main()
