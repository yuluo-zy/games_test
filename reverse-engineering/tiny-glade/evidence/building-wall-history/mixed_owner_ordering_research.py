"""Read frozen native fixtures; complete original creation->mixed replay expansion."""
import sys,struct,json,hashlib
from pathlib import Path
ROOT=Path('/Users/yuluo/Desktop/game/games_test/reverse-engineering/tiny-glade')
sys.path.insert(0,str(ROOT/'evidence/building-wall-history'))
import native_wall_history as n
cpu=n.cpu;data=n.data;author=n.author

def expand_recorded(owner,count,undo,instant):
 entry=data+0x8000;aux=data+0x8100;replay=data+0x9000;resource=data+0x9100
 cpu.mem_write(aux,struct.pack('<QQ',1,1));cpu.mem_write(aux+0x10,struct.pack('<4Q',data+0x8200,15,15,0));cpu.mem_write(data+0x8200,bytes([255])*16);cpu.mem_write(aux+0x40,struct.pack('<QQ',1,1))
 cpu.mem_write(entry,struct.pack('<5Q',count,owner,count,aux,aux+0x40));cpu.mem_write(replay,struct.pack('<4Q',0,8,0,0));cpu.mem_write(resource,struct.pack('<QQQI',replay,0,resource+0x60,0)+struct.pack('<I',0x12345678))
 cpu.reg_write(n.UC_X86_REG_RCX,entry);cpu.reg_write(n.UC_X86_REG_RDX,int(undo));cpu.reg_write(n.UC_X86_REG_R8,int(instant));cpu.reg_write(n.UC_X86_REG_R9,resource)
 author.active=True
 try:author.call(0x140af6170)
 finally:author.active=False
 capacity,pointer,head,length=struct.unpack('<4Q',cpu.mem_read(replay,32));result=[]
 for i in range(length):
  bucket=pointer+((head+i)%capacity)*64;commands=n.nf.u64(bucket+8);count=n.nf.u64(bucket+16);rows=[]
  for j in range(count):
   p=commands+j*48;raw=bytes(cpu.mem_read(p,48));tag=raw[0]
   row={'outer_tag':tag,'raw_record':raw.hex()}
   if tag==4:
    cap,gptr,front,glen=struct.unpack('<4Q',raw[8:40]);groups=[]
    for k in range(glen):
     g=gptr+((front+k)%cap)*24;command=n.nf.u64(g+8);number=n.nf.u64(g+16);edits=[]
     for q in range(number):
      p2=command+q*24;b=bytes(cpu.mem_read(p2,24));arc=n.nf.u64(p2+8);op=n.nf.u64(arc+16)
      kind='Create'if op==1<<63 else'Delete'if op==(1<<63)+1 else'StateChange'
      edits.append({'kind':kind,'mode':struct.unpack_from('<I',b)[0],'duration_bits':b[4:8].hex()if kind=='StateChange'else'ignored','elapsed_bits':b[16:20].hex(),'undo':b[20],'finished':b[21],'arc264':bytes(cpu.mem_read(arc,264)).hex()})
     groups.append(edits)
    row['groups']=groups
   elif tag==5:
    row['roof_state']={'mode':struct.unpack_from('<I',raw,8)[0],'duration_bits':raw[12:16].hex(),'elapsed_bits':raw[24:28].hex(),'undo':raw[28],'finished':raw[29]}
    arc=n.nf.u64(p+16);row['roof_payload']=bytes(cpu.mem_read(arc+16,64)).hex()
   else:raise AssertionError(tag)
   rows.append(row)
  result.append(rows)
 return result

cases=[]
for shape_tag in[0,1]:
 for undo in[False,True]:
  for instant in[False,True]:
   shape=struct.pack('<I6f',0,2,3,4,.7,0,0)if shape_tag==0 else struct.pack('<I6f',1,1,0,2,3,4,5)
   created=author.creator_research(shape_tag,100,shape,3.2,.7,1,2,7)
   owner=data+0xa0000;count=created['history_pending_count'];tags=[n.nf.u64(owner+i*24)for i in range(count)]
   assert tags==[7,8]
   buckets=expand_recorded(owner,count,undo,instant);assert len(buckets)==2
   flattened=[r for bucket in buckets for r in bucket]
   print('SCENE',shape_tag,undo,instant,'top',[[r['outer_tag']for r in b]for b in buckets])
   top=[r['outer_tag']for r in flattened];assert top==([5,4]if undo else[4,5])
   wall=next(r for r in flattened if r['outer_tag']==4);kinds=[g[0]['kind']for g in wall['groups']];assert kinds==(['StateChange','Create']if undo else['Create','StateChange'])
   assert all(e['undo']==int(undo)and e['finished']==0 for g in wall['groups']for e in g)
   state=next(e for g in wall['groups']for e in g if e['kind']=='StateChange');assert state['mode']==int(not instant)and state['duration_bits']=='cdcccc3e'
   create=next(e for g in wall['groups']for e in g if e['kind']=='Create');assert create['mode']==0
   roof=next(r for r in flattened if r['outer_tag']==5);assert roof['roof_state']=={'mode':int(not instant),'duration_bits':'cdcc4c3e','elapsed_bits':'00000000','undo':int(undo),'finished':0}
   cases.append({'shape':shape_tag,'undo':undo,'instant':instant,'creator_actual_return':True,'expand_actual_return':True,'history_owner_order':tags,'buckets':buckets})
report={'ok':True,'cases':cases,'scope':'Actual complete creator producesWallCompound7thenRoofCreate8; whole mixed replay_edit->RET. Emptyauxiliary snapshots dependencycarriers; no independenttoy ledger andno business result hook. Top reversal andinternalgroup reversal observed forreal two-group creator. Two actualbuckets preserveWall/Roof barrier inboth instant/timed; no timed execution/global schedule claim.','binary_sha256':n.nf.b['binary']['sha256'],'fixture_hashes':{str(p.relative_to(ROOT)):hashlib.sha256(p.read_bytes()).hexdigest()for p in[ROOT/'evidence/building-wall-history/native_wall_history.py',ROOT/'evidence/building-authoring/native_authoring.py',ROOT/'evidence/building-constructor/native_fixture.py']}}
Path('/tmp/tinyglade-g14-owner-ordering.json').write_text(json.dumps(report,indent=2)+'\n');print(json.dumps({'ok':True,'cases':len(cases),'top_forward':[4,5],'top_undo':[5,4],'group_forward':['Create','StateChange'],'group_undo':['StateChange','Create']}))
