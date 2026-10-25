import sys,struct,json,hashlib
from pathlib import Path
ROOT=Path(__file__).resolve().parents[2]
sys.path.insert(0,str(Path(__file__).resolve().parent))
# Only read frozen fixtures; copied originalcall wrapper supplies standard ownerVec.
source=Path(__file__).with_name('mixed_owner_ordering.py').read_text();namespace={'__file__':str(Path(__file__).with_name('mixed_owner_ordering.py'))};exec(source[:source.index('cases=[]')],namespace)
n=namespace['n'];author=n.author;cpu=n.cpu;data=n.data;expand=namespace['expand_recorded']
shape=struct.pack('<I6f',1,1,0,2,3,4,5);cases=[]
for number in[0,1,2,8,9,10,17,18,19,27,28,50]:
 for undo in[False,True]:
  for instant in[False,True]:
   created=author.creator_research(1,100,shape,3.2,.7,1,2,7);record=bytes(cpu.mem_read(data+0xa0018,24));arc=n.nf.u64(data+0xa0020)
   # Distinct library-owned Arc clones of actual observed Roof payload, each a
   # valid input historyowner with a distinct wallID; never expected output.
   owners=data+0xe0000
   for j in range(number):
    a=data+0xd0000+j*0x100;raw=bytearray(cpu.mem_read(arc,80));struct.pack_into('<QQ',raw,0,1,1);struct.pack_into('<Q',raw,16+48,100+j);cpu.mem_write(a,bytes(raw));cpu.mem_write(owners+j*24,struct.pack('<3Q',8,a,0))
   buckets=expand(owners,number,undo,instant);lengths=[len(b)for b in buckets];assert lengths==[1]*number
   ids=[]
   for b in buckets:
    assert all(r['outer_tag']==5 for r in b)
    ids.extend(struct.unpack_from('<Q',bytes.fromhex(r['roof_payload']),48)[0]for r in b)
   assert ids==list(range(100,100+number))[::(-1 if undo else 1)]
   cases.append({'input_count':number,'undo':undo,'instant':instant,'actual_return':True,'bucket_lengths':lengths,'wall_ids':ids})
report={'ok':True,'cases':cases,'research_source_sha256':hashlib.sha256(Path(__file__).read_bytes()).hexdigest(),'binary_sha256':n.nf.b['binary']['sha256'],'fixture_hashes':{str(p.relative_to(ROOT)):hashlib.sha256(p.read_bytes()).hexdigest()for p in[ROOT/'evidence/building-wall-history/mixed_owner_ordering.py',ROOT/'evidence/building-wall-history/native_wall_history.py',ROOT/'evidence/building-authoring/native_authoring.py',ROOT/'evidence/building-constructor/native_fixture.py']},'scope':'Whole original B replay_edit140af6170->RET Roof-only partitiontail; valid originalRoofCreate input payload clones andemptyaux snapshots. Every Roof5 command receives itsownbucket; Roof5+Roof5 compatibility isfalse. Cardinalitiesup to50 tested. No candidate Rust changes/acceptance.','binary_sha256':n.nf.b['binary']['sha256']}
Path(__file__).with_name('B-roof_partition-native.json').write_text(json.dumps(report,indent=2)+'\n');print(json.dumps({'ok':True,'cases':len(cases),'roof_commands_per_bucket':1}))
