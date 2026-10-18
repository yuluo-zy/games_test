from pathlib import Path
import json,hashlib,tempfile,subprocess,re,gzip
import capstone,pefile
R=Path('/Users/yuluo/Desktop/game');P=R/'games_test/reverse-engineering/tiny-glade/skeleton/task-index/work/TG-e73d16b0f4be6e76bf/terrain-curve-20261018/cursor-plane-20261019';O=Path(__file__).resolve().parent
sha=lambda f:hashlib.sha256(f.read_bytes()).hexdigest()
assert sha(P/'frozen-inputs.json')=='909118c7ba26bd690b2dcd92b6c1ad8effd81365ce0a1a8432560a97e0917b17'
m=json.loads((P/'frozen-inputs.json').read_text())
for x in m['files']:
 f=P/x['path'];assert f.stat().st_size==x['bytes'] and sha(f)==x['sha256']
pepath=R/'ljxsj_92385/Tiny Glade/tiny-glade.rne';h=hashlib.sha256()
with pepath.open('rb') as f:
 for b in iter(lambda:f.read(1<<20),b''):h.update(b)
assert h.hexdigest()=='f507a6cea25c5199447b88ba746a5293e64a279f73797ca74c1ce7f4ff2b7d03';pe=pefile.PE(str(pepath),fast_load=True)
raw=pe.get_data(0xa9fdc0,2844);assert hashlib.sha256(raw).hexdigest()=='3a5c4d4fa63d231fbbc18d7598c394a2ad18e9c206023d17a1bab5333964d78d'
md=capstone.Cs(capstone.CS_ARCH_X86,capstone.CS_MODE_64);ii=list(md.disasm(raw,0x140a9fdc0));assert len(ii)==688 and sum(i.size for i in ii)==len(raw)
old={}
for l in (P/'evidence/140a9fdc0-plane-audit-full-B-rne.asm').read_text().splitlines():
 t=re.match(r'^([0-9a-f]{16}):\s+([0-9a-f]+)\s+',l)
 if t:old[int(t[1],16)]=t[2]
assert len(old)==688
for i in ii:assert old[i.address]==i.bytes.hex()
ins={i.address:i for i in ii};semantic=[]
for a,s in [(0x140aa00fe,'call 0x140924a10'),(0x140aa0103,'cmp eax, 1'),(0x140aa0106,'jne 0x140aa035a'),(0x140aa010f,'movss xmm7, dword ptr [r14]'),(0x140aa0117,'mov dword ptr [rdi], 1'),(0x140aa011d,'movss dword ptr [rdi + 4], xmm7'),(0x140aa0133,'call 0x140969720'),(0x140aa02d9,'call 0x140924900'),(0x140aa02de,'test rax, rax'),(0x140aa02e1,'je 0x140aa06e7'),(0x140aa06e7,'mov dword ptr [r13], r12d'),(0x140aa06eb,'mov dword ptr [rsi], 0')]:
 i=ins[a];assert i.mnemonic+' '+i.op_str==s;semantic.append({'site':hex(a),'asm':s})
# Independently disambiguate the SystemParam from two exact native callers,
# not the grow_one ICF aliases or the catalog's conservative candidate label.
enrich=json.load(gzip.open(R/'games_test/reverse-engineering/tiny-glade/skeleton/B/crates/country-core/src/systems/terrain/ui_draw_terrain.rs.enriched.json.gz','rt'))
f=next(f for f in enrich['functions'] if f['name']=='country_core::systems::terrain::ui_draw_terrain::ui_draw_terrain');refs=f['reference_callers'];assert len(refs)==2
aliases=json.load(gzip.open(R/'games_test/reverse-engineering/tiny-glade/skeleton/_pdb/B/enrichment/address-aliases.json.gz','rt'))
records=[]
for mod,rec in [(488,12160),(1611,2130468),(560,160852)]:
 for l in gzip.open(R/f'games_test/reverse-engineering/tiny-glade/skeleton/_pdb/B/modules/{mod:05d}.jsonl.gz','rt'):
  x=json.loads(l)
  if x.get('record_offset')==rec:records.append(x);break
for ref in refs:
 entry=ref['caller_entry_rva'];a=0x140000000+ref['instruction_rva'];b=pe.get_data(a-0x140000000,5);assert b.hex()==ref['bytes'];i=list(md.disasm(b,a))[0];assert i.mnemonic=='call' and i.op_str=='0x140a9fdc0'
 aa=aliases[str(entry)];assert len(aa)==1;assert 'NotifyHintSystemTerrain>' in aa[0]['name'] and 'NotifyHintSystemToolWasJustUsed>' in aa[0]['name'];assert 'FinishInputRecordingCmd' not in aa[0]['name']
assert len(aliases[str(0xa9fdc0)])==1;assert records[-1]['code_size']==2844
(O/'original-system-param-records.json').write_text(json.dumps({'records':records,'original_calls':refs,'unique_alias_entries':[hex(0x140000000+x['caller_entry_rva']) for x in refs]},indent=2)+'\n')
source=json.loads((P/'source-mapping.json').read_text());drift=[]
with tempfile.TemporaryDirectory(dir='/private/tmp',prefix='cursor-plane-review-') as t:
 for s in source:
  d=Path(t)/s['source'];d.parent.mkdir(parents=True,exist_ok=True);d.write_bytes((P/s['base']).read_bytes());q=subprocess.run(['git','apply',str(P/s['patch'])],cwd=t,capture_output=True,text=True);assert q.returncode==0,q.stderr;assert d.read_bytes()==(P/s['candidate']).read_bytes()
  drift.append({'path':s['source'],'current_matches_captured_base':(R/s['source']).read_bytes()==(P/s['base']).read_bytes()})
# One canonical inner owner; the field is transient and absent from stroke serde.
inner=(P/'candidate/stroke.rs').read_text();assert inner.count('pub cursor_height_override: Option<f32>')==1
core=(P/'evidence/unchanged-core-terrain-editing.rs').read_text();assert 'Deref' in core and 'tg_terrain::stroke::TerrainEditingData' in core and 'cursor_height_override' not in core
result={'author_manifest_sha256':sha(P/'frozen-inputs.json'),'author_files_verified':len(m['files']),'B_PE_whole_identity':True,'native_full_PDB_bytes':2844,'native_instructions_all_bytes_verified':688,'exact_native_caller_SystemParams_disambiguated':True,'no_Finish_writer_in_two_unique_typed_callers':True,'semantic_checks':semantic,'all_three_patch_applications_exact':True,'source_drift_at_review':drift,'canonical_inner_field_no_parallel_Resource':True,'three_new_tests_and_existing_three_frame_extended':'source reviewed, not executed','production_changed':False,'cargo_executed':False,'native_or_JIT_executed':False,'blocking_findings':[],'sole_required_root_integration_risk':'Apply the snapshot-owned one-literal cursor_height_override:None hunk together with the canonical field, retaining the concurrent State-reader owner changes; do not overwrite the whole snapshot.rs from either candidate.'}
(O/'validation.json').write_text(json.dumps(result,indent=2)+'\n');print(json.dumps({k:v for k,v in result.items() if k!='semantic_checks'},indent=2))
