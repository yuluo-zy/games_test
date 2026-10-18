from pathlib import Path
import json,hashlib,re,subprocess,tempfile,shutil,gzip
import pefile,capstone
ROOT=Path('/Users/yuluo/Desktop/game');SRC=ROOT/'games_test/reverse-engineering/tiny-glade/skeleton/task-index/work/TG-889aa76ca030a2e3cd/terrain-curve-20261018';OUT=ROOT/'games_test/reverse-engineering/tiny-glade/skeleton/task-index/work/TG-f7070bb3446b7a37eb/building-audit-20261018/v2/decorator-storage-independent-review';OUT.mkdir(parents=True,exist_ok=True)
def sha(p):
 h=hashlib.sha256()
 with p.open('rb') as f:
  for b in iter(lambda:f.read(1<<20),b''):h.update(b)
 return h.hexdigest()
manifestfiles=[SRC/'frozen-inputs.json',SRC/'shared-conversion-review/frozen-inputs.json',SRC/'snapshot-bucket-identity-addendum/frozen-inputs.json'];files=[];checked=[]
for f in manifestfiles:
 d=json.loads(f.read_text());rows=d.get('files',d.get('inputs'));count=0
 for r in rows:
  p=Path(r['path']);p=p if p.is_absolute() else f.parent/p
  assert sha(p)==r['sha256'] and p.stat().st_size==r.get('bytes',r.get('size')),(str(p),'hash')
  files.append(p);count+=1
 checked.append({'manifest':str(f),'sha256':sha(f),'inputs':count})
pepath=ROOT/'ljxsj_92385/Tiny Glade/tiny-glade.rne';pdbpath=pepath.with_name('tiny_glade.pdb');assert sha(pepath)=='f507a6cea25c5199447b88ba746a5293e64a279f73797ca74c1ce7f4ff2b7d03';assert sha(pdbpath)=='2d9686b8eaaf5c672c8dbc3aed85fbfeaf2a98a0dc64353191cd1a123a5d8911'
pe=pefile.PE(str(pepath),fast_load=True);md=capstone.Cs(capstone.CS_ARCH_X86,capstone.CS_MODE_64);md.detail=True;packs=[];ins={}
for p in files:
 if p.suffix!='.json':continue
 d=json.loads(p.read_text())
 if not isinstance(d,dict) or not isinstance(d.get('instructions'),list):continue
 assert d['status']=='ok';assert d['artifact']['sha256']==sha(pepath);assert d['artifact']['debug']['sha256']==sha(pdbpath);start=int(d['range']['start_va'],16);end=int(d['range']['stop_va_exclusive'],16);pos=start
 asm={}
 for line in d['raw_assembly'].splitlines():
  m=re.match(r'^([0-9a-fA-F]+):\s*((?:[0-9a-fA-F]{2}\s+)+)(.*)$',line)
  if m:asm[int(m[1],16)]=(bytes.fromhex(m[2]),' '.join(m[3].split()))
 for row in d['instructions']:
  a=int(row['va'],16);raw=bytes.fromhex(row['bytes']);assert a==pos;assert len(raw)==row['size'];assert raw==pe.get_data(a-0x140000000,len(raw));ii=list(md.disasm(raw,a));assert len(ii)==1 and ii[0].size==len(raw);ins[a]=ii[0]
  assert asm[a][0]==raw;assert asm[a][1]==' '.join(row['assembly'].split()),(str(p),hex(a),asm[a],row['assembly'])
  pos+=len(raw)
 assert pos==end+d['range']['instruction_overhang_bytes'];assert len(asm)==len(d['instructions'])
 # JSON raw assembly and separately retained LLVM text must be identical.
 ap=p.with_name(p.stem+'.asm.txt');assert ap.exists() and ap.read_text()==d['raw_assembly']
 packs.append({'path':str(p),'start':hex(start),'requested_bytes':end-start,'decoded_bytes':pos-start,'instructions':len(d['instructions']),'overhang':pos-end})
# Direct typed native caller rows: actual machine call and exact cached PDB spelling.
callers=[]
for m,r,a,t in [(114,1124,0x140000000+3005077,0x1408337c0),(178,1871264,0x140000000+4383591,0x1408337c0)]:
 record=None
 for line in gzip.open(ROOT/f'games_test/reverse-engineering/tiny-glade/skeleton/_pdb/B/modules/{m:05d}.jsonl.gz','rt'):
  d=json.loads(line)
  if d.get('record_offset')==r:record=d;break
 assert record and 'ResMut<country_core::resources::walls::decorator::DeferredDecoratorOpEvents>' in record['name'] and 'EventWriter<country_core::resources::walls::decorator::DecoratorOpEvent>' in record['name']
 i=next(md.disasm(pe.get_data(a-0x140000000,5),a));assert i.mnemonic=='call' and i.operands[0].imm==t
 callers.append({'site':hex(a),'raw':i.bytes.hex(),'target':hex(t),'module':m,'record':r,'name':record['name']})
# Both type-dispatch tables and the old hashing constants are read as data, not ID codecs.
for f,tablekey,targetkey in [(SRC/'restore-projection-jump-tables.json','table','destinations')]:
 for row in json.loads(f.read_text()):
  a=int(row[tablekey],16);raw=bytes.fromhex(row['bytes']);assert raw==pe.get_data(a-0x140000000,len(raw));assert [a+int.from_bytes(raw[i:i+4],'little',signed=True) for i in range(0,len(raw),4)]==[int(x,16) for x in row[targetkey]]
r=json.loads((SRC/'shared-conversion-review/jump-table.json').read_text());a=int(r['va'],16);raw=bytes.fromhex(r['bytes']);assert raw==pe.get_data(a-0x140000000,len(raw));assert[a+int.from_bytes(raw[i:i+4],'little',signed=True) for i in range(0,len(raw),4)]==[int(x,16) for x in r['targets']]
# Three declared pdata bodies are checked directly against PE .pdata table.
for row in json.loads((SRC/'original-pdata-ranges.json').read_text()):
 a=int(row['begin'],16)-0x140000000;b=int(row['end'],16)-0x140000000
 sec=next(s for s in pe.sections if s.Name.rstrip(b'\0')==b'.pdata');data=sec.get_data();assert any(int.from_bytes(data[i:i+4],'little')==a and int.from_bytes(data[i+4:i+8],'little')==b for i in range(0,len(data)-11,12))
# All three patches are applied only to isolated captured source bases.
mapping=json.loads((SRC/'source-mapping.json').read_text())
with tempfile.TemporaryDirectory(prefix='decorator-review-',dir='/private/tmp') as temp:
 for row in mapping:
  dst=Path(temp)/row['source'];dst.parent.mkdir(parents=True,exist_ok=True);shutil.copyfile(SRC/row['base'],dst)
 result=subprocess.run(['git','apply','--check',str(SRC/'root.patch')],cwd=temp,capture_output=True,text=True);assert result.returncode==0,result.stderr
 result=subprocess.run(['git','apply',str(SRC/'root.patch')],cwd=temp,capture_output=True,text=True);assert result.returncode==0,result.stderr
 for row in mapping:assert (Path(temp)/row['source']).read_bytes()==(SRC/row['candidate']).read_bytes()
results={'target':'B','manifests':checked,'frozen_file_count':len(files),'native_packs':packs,'decoded_bytes':sum(r['decoded_bytes'] for r in packs),'decoded_instructions':sum(r['instructions'] for r in packs),'all_raw_instructions_equal_PE':True,'all_LLVM_JSON_texts_equal_retained_assembly':True,'Capstone_boundaries_agree':True,'typed_native_callers':callers,'both_snapshot_and_history_type_dispatch_tables_verified':True,'three_pdata_bodies_verified':True,'root_patch_applies_and_equals_all_three_candidates':True,'native_JIT_execution':False,'cargo_execution':False,'production_modified':False}
(OUT/'verification.json').write_text(json.dumps(results,indent=2)+'\n');print(json.dumps({k:v for k,v in results.items() if k not in ['native_packs','typed_native_callers']},indent=2));print('packs',len(packs))
