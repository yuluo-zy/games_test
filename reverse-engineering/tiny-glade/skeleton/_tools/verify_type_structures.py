"""Independent coverage/source/raw-record audit of structure metadata."""
import collections,gzip,hashlib,json,struct
from pathlib import Path
import yaml
ROOT=Path(__file__).resolve().parents[1];PROJECT=ROOT.parent
START='// === PDB STRUCTURES V1 BEGIN ===';END='// === PDB STRUCTURES V1 END ==='
UDT={'LF_STRUCTURE','LF_CLASS','LF_UNION','LF_ENUM'}
def sha(p):return hashlib.sha256(p.read_bytes()).hexdigest()
def body(d):return next((v for k,v in d.items() if k!='Kind' and isinstance(v,dict)),{})
def load(p):return json.load(gzip.open(p,'rt'))
def normalized(row):
 d=yaml.load(row['yaml'],Loader=yaml.CSafeLoader) or {};raw=yaml.load(row['yaml'],Loader=yaml.CBaseLoader) or {}
 def walk(a,b):
  if isinstance(a,dict):
   for k,v in a.items():
    if k in ('Name','String','UniqueName'):a[k]=b[k]
    else:walk(v,b[k])
  elif isinstance(a,list):
   for v,w in zip(a,b):walk(v,w)
 walk(d,raw);return d
checks=[]
for v in ['A','B','C']:
 report=json.loads((ROOT/'_pdb'/v/'structures'/'report.json').read_text());tpi={};ipi={}
 for l in gzip.open(ROOT/'_pdb'/v/'types.jsonl.gz','rt'):
  r=json.loads(l);raw=bytes.fromhex(r['raw_hex']);length,kind=struct.unpack_from('<HH',raw);assert length+2==len(raw) and kind==r['kind']
  (tpi if r['namespace']=='TPI' else ipi)[r['index']]=r
 definitions={x['index']:x for x in load(ROOT/'_pdb'/v/'structures'/'type-definitions.json.gz')}
 expected={i for i,r in tpi.items() if r.get('decoded',{}).get('Kind') in UDT};assert set(definitions)==expected
 def flattened(idx):
  if not idx:return []
  result=[]
  for x in normalized(tpi[idx])['FieldList']:
   if x['Kind']=='LF_INDEX':result+=flattened(body(x)['ContinuationIndex'])
   else:result.append(x)
  return result
 fieldcount=enumcount=0
 for i,d in definitions.items():
  record=normalized(tpi[i]);assert record==d['record'];b=body(record);forward='ForwardReference' in b.get('Options',[])
  assert (d['size_bytes'] is None if forward else d['size_bytes']==b.get('Size'))
  assert d['alignment']=='unknown' and d['rust_repr']=='unknown'
  assert [x['record'] for x in d['fields_and_relations']]==flattened(b.get('FieldList',0))
  for x in d['fields_and_relations']:
   fieldcount+=x['kind']=='LF_MEMBER';enumcount+=x['kind']=='LF_ENUMERATE'
  for ref in d['source_records']:
   src=body(normalized(ipi[ref['ipi_index']]));assert src['UDT']==i and src['SourceFile']==ref['source_string_id'] and src['LineNumber']==ref['line']
   assert d['source_mapping']=='PDB_UDT_source_line'
 names=load(ROOT/'_pdb'/v/'structures'/'game-named-symbols.json.gz');assert len(names)==report['named_symbol_entities']
 for n in names:
  assert n['fields']==n['size_bytes']==n['enum_values']=='unknown'
  for ref in n['ipi_occurrences']:assert ref['index'] in ipi
 covered=set();attached=set();known_evidence=set();native=0;graphs=0
 for f in report['files_index']:
  path=ROOT/v/f['path'];text=path.read_text();assert text.count(START)==text.count(END)==1,(v,f['path'])
  section=START+text.split(START,1)[1].split(END,1)[0]+END+'\n'
  assert all(not l or l.startswith('//') for l in section.splitlines())
  if f.get('structures_block_sha256'):assert hashlib.sha256(section.encode()).hexdigest()==f['structures_block_sha256']
  meta=ROOT/v/f['metadata_file'];assert sha(meta)==f['companion_sha256'],meta
  p=load(meta);assert p['version']==v and p['source']==f['path']
  for d in p['definitions']:assert d==definitions[d['index']];attached.add(d['index'])
  for r in list(p.get('type_graph',{}).values())+p['unowned_type_records']:
   raw=tpi[r['index']];assert r['raw_hex']==raw['raw_hex'] and r['kind']==raw['kind'];covered.add(r['index']);graphs+=1
  for o in p['observed_game_structures']:
   native+=1;assert o['baseline']==v
   for e in o['evidence']:
    assert sha(PROJECT/e['path'])==e['sha256'];known_evidence.add(e['path'])
   if o.get('native_constructor'):
    version=o['native_constructor'];assert version['baseline']==v
   if v!='B':assert o['name'] in ['country_core::resources::roofs::roof_shape::Roof','Roof Nani creation relationship (version specific)'],(v,o['name'])
   if o['name']=='country_core::resources::walls::public_wall_state::PublicWallState':
    fields={x['offset']:x for x in o['fields']};assert fields[0x108]['width']==fields[0x110]['width']==1
    assert fields[0x10]['observed_semantics']=='raw Curve2 point count';assert fields[0x30]['observed_semantics']=='raw Curve2 length'
   if o['name']=='B Roof creation Bundle component graph':assert o['required_component_edges']==[]
 assert covered==set(tpi),(v,len(covered),len(tpi));assert attached==expected
 assert fieldcount==report['member_records'] and enumcount==report['enum_values'] and native==report['native_structure_attachments']
 checks.append({'version':v,'UDT_records_verified':len(expected),'definition_records':report['definition_records'],'field_entries_verified':fieldcount,'enumerators_verified':enumcount,'all_TPI_records_verified':len(covered),'source_files_verified':len(report['files_index']),'named_entities_verified':len(names),'native_structure_attachments_verified':native,'evidence_files_sha256_verified':sorted(known_evidence),'raw_TPI_IPI_records_verified':len(tpi)+len(ipi),'forward_size_unknown':True,'no_cross_version_native_layout_transfer':True})
 print(json.dumps(checks[-1],ensure_ascii=False),flush=True)
before=json.loads((ROOT/'_pdb'/'production-source-snapshot-before-structures.json').read_text())
after={p.relative_to(PROJECT).as_posix():sha(p) for p in (PROJECT/'reconstruction').rglob('*') if p.is_file() and (p.suffix in ('.rs','.toml') or p.name=='Cargo.lock')}
assert before==after
result={'ok':True,'versions':checks,'production_files_unchanged':len(before),'no_new_delivery_directory':True,'scope':'Metadata/source mapping and retained evidence audit; no game behavior or complete Rust ABI claim'}
(ROOT/'pdb-structures-verification.json').write_text(json.dumps(result,ensure_ascii=False,indent=2)+'\n')
