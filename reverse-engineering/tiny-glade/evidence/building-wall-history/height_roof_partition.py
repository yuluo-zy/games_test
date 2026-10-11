import sys,struct,json,hashlib
from pathlib import Path
ROOT=Path(__file__).resolve().parents[2]
sys.path.insert(0,str(Path(__file__).resolve().parent));source=Path(__file__).with_name('mixed_owner_ordering.py').read_text();ns={'__file__':str(Path(__file__).with_name('mixed_owner_ordering.py'))};exec(source[:source.index('cases=[]')],ns);n=ns['n'];cpu=n.cpu;data=n.data;author=n.author;expand=ns['expand_recorded'];cases=[]
shape=struct.pack('<I6f',1,1,0,2,3,4,5)
for order in['HR','RH','HHR','HRR','RHR','HRHR','CH','HC','CR','RC']:
 for undo in[False,True]:
  for instant in[False,True]:
   made=author.creator_research(1,100,shape,3.2,.7,1,2,7);create_record=bytes(cpu.mem_read(data+0xa0000,24));roof_record=bytes(cpu.mem_read(data+0xa0018,24))
   h=data+0xf0000;cpu.mem_write(h,bytes(32));cpu.reg_write(n.UC_X86_REG_RCX,h);cpu.reg_write(n.UC_X86_REG_RDX,101);cpu.reg_write(n.UC_X86_REG_XMM2,n.nf.bits(3.2));cpu.reg_write(n.UC_X86_REG_XMM3,n.nf.bits(7.5));author.call(0x140a83fb0);assert n.nf.u64(h)==7;height_record=bytes(cpu.mem_read(h,24));owner=data+0xe0000
   records={'H':height_record,'R':roof_record,'C':create_record};cpu.mem_write(owner,b''.join(records[c]for c in order))
   result=expand(owner,len(order),undo,instant);classes=[]
   for bucket in result:
    one=[]
    for row in bucket:
     if row['outer_tag']==5:one.append('R')
     else:
      assert row['outer_tag']==4;groups=row['groups']
      if len(groups)==1 and len(groups[0])==1 and int.from_bytes(bytes.fromhex(groups[0][0]['arc264'])[16:24],'little')==(1<<63)+2:one.append('H')
      else:one.append('C')
    classes.append(''.join(one))
   cases.append({'order':order,'undo':undo,'instant':instant,'actual_height_constructor_return':True,'actual_mixed_expand_return':True,'bucket_classes':classes})
   print(order,undo,instant,classes)
Path(__file__).with_name('B-height_roof_partition-native.json').write_text(json.dumps({'ok':True,'cases':cases,'research_source_sha256':hashlib.sha256(Path(__file__).read_bytes()).hexdigest(),'binary_sha256':n.nf.b['binary']['sha256'],'fixture_hashes':{str(p.relative_to(ROOT)):hashlib.sha256(p.read_bytes()).hexdigest()for p in[ROOT/'evidence/building-wall-history/mixed_owner_ordering.py',ROOT/'evidence/building-wall-history/native_wall_history.py',ROOT/'evidence/building-authoring/native_authoring.py',ROOT/'evidence/building-constructor/native_fixture.py']},'scope':'Whole original heightconstructor andmixed replay_edit -> RET; complete creator capturedinputs/standard ownerVec. C isactualtwo-groupnew-wallcompound, H one-groupheight, R RoofCreate. Native-only partitionresearch.'},indent=2)+'\n')
