"""Audit full filesystem coverage, inline evidence, baseline and artifact scopes."""
import gzip,hashlib,json
from pathlib import Path
ROOT=Path(__file__).resolve().parents[1];PROJECT=ROOT.parent
START='// === WHOLE PROJECT EVIDENCE V1 BEGIN ===';END='// === WHOLE PROJECT EVIDENCE V1 END ==='
def sha(p):return hashlib.sha256(p.read_bytes()).hexdigest()
def load(p):return json.load(gzip.open(p,'rt'))
results=[]
for v in 'ABC':
 report=json.loads((ROOT/'_pdb'/v/'structures/project-coverage.json').read_text());actual={p.relative_to(ROOT/v).as_posix() for p in (ROOT/v).rglob('*.rs')};indexed={f['path'] for f in report['files']};assert actual==indexed,(v,sorted(actual-indexed),sorted(indexed-actual))
 structure=json.loads((ROOT/'_pdb'/v/'structures/report.json').read_text());assert {f['path'] for f in structure['files_index']}==actual
 catalog=load(ROOT/'_pdb'/v/'structures/project-evidence-catalog.json.gz');native_fields=schemas=links=0;proofs=set()
 for e in catalog.values():assert sha(PROJECT/e['path'])==e['sha256'];assert e['field_promotion'].startswith('none')
 for f in report['files']:
  source=ROOT/v/f['path'];text=source.read_text();assert text.count(START)==text.count(END)==1
  assert text.count('// === PDB STRUCTURES V1 BEGIN ===')==1
  section=START+text.split(START,1)[1].split(END,1)[0]+END+'\n';assert hashlib.sha256(section.encode()).hexdigest()==f['block_sha256'],(v,f['path'])
  companion=ROOT/v/f['metadata_file'];assert sha(companion)==f['companion_sha256'];p=load(companion);assert p['version']==v and p['path']==f['path']
  assert all(not line or line.startswith('//') for line in section.splitlines())
  for native in p['native_partial_layouts']:
   assert native['baseline']==v and native['rust_repr']=='unknown';native_fields+=len(native['fields'])
   for field in native['fields']:assert json.dumps(field,ensure_ascii=False) in section
   for e in native['evidence']:assert sha(PROJECT/e['path'])==e['sha256'];proofs.add(e['path'])
   if v=='A':assert native['name']=='DecoratorInfo destination accessor (version specific)'
   if v=='C':assert native['name'] in ['Sheep C pet state projection','DecoratorInfo destination accessor (version specific)']
   if native['name']=='DecoratorInfo destination accessor (version specific)':assert native['fields'][0]['offset']=={'A':0x21,'B':0x22,'C':0x32}[v]
   if native['name']=='Sheep C pet state projection':assert v=='C';assert [x['offset'] for x in native['fields']]==[0x18,0x1c,0x20,0x58,0x60];assert all(x['original_name']=='unknown' for x in native['fields'])
  for schema in p['serialized_schema_observations']:
   schemas+=1;assert v==schema['baseline']=='B';assert schema['field_offsets']==schema['Rust_numeric_types']=='unknown'
   for e in schema['evidence']:assert sha(PROJECT/e['path'])==e['sha256'];proofs.add(e['path'])
  for link in p['evidence_links']:
   links+=1;assert link['evidence_id'] in catalog
 assert native_fields==report['new_native_field_observations'] and schemas==report['serialized_schema_observations'] and links==report['evidence_links']
 result={'version':v,'all_existing_source_files_verified':len(actual),'missing_sources':[],'project_groups':report['package_groups'],'native_field_observations_verified':native_fields,'serialized_schema_observations_verified':schemas,'evidence_links_verified':links,'non_roof_evidence_present':any(f['new_native_partial_layouts'] or f['serialized_schemas'] for f in report['files'] if '/roofs/' not in f['path'] and '/roof/' not in f['path']),'C_rhapsody2_files_verified':sum(p.startswith('rhapsody2/') for p in actual),'C_shabriri_files_verified':sum(p.startswith('shabriri/') for p in actual),'artifact_and_native_layout_scopes_separate':True,'no_guessed_fields':True}
 assert result['non_roof_evidence_present'];results.append(result);print(json.dumps(result,ensure_ascii=False),flush=True)
previous=json.loads((ROOT/'_pdb/production-source-snapshot-before-structures.json').read_text());current={p.relative_to(PROJECT).as_posix():sha(p) for p in (PROJECT/'reconstruction').rglob('*') if p.is_file() and (p.suffix in ('.rs','.toml') or p.name=='Cargo.lock')};assert previous==current
report={'ok':True,'versions':results,'all_existing_source_files':sum(r['all_existing_source_files_verified'] for r in results),'production_files_unchanged':len(current),'complete_native_layout_or_full_game_claim':False}
(ROOT/'pdb-whole-project-verification.json').write_text(json.dumps(report,ensure_ascii=False,indent=2)+'\n')
