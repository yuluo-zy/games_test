from pathlib import Path
import gzip, hashlib, json, struct, subprocess
root=Path('D:/game/reverse-engineering/tiny-glade')
out=root/'evidence/render-recovery/parallel-batch-06/descriptor-binding'
crate=root/'reconstruction/render/work-packages/descriptor-binding-r8'
def read(p):return json.loads(p.read_text(encoding='utf8'))
def digest(p):return hashlib.sha256(p.read_bytes()).hexdigest()
native=read(out/'native-validation.json');assert native['source_sha256']==digest(crate/'src/lib.rs') and native['mismatches']==0
commands=[]
for command in [
    ['cargo','fmt','--check','--manifest-path',str(crate/'Cargo.toml')],
    ['cargo','test','--locked','--offline','--manifest-path',str(crate/'Cargo.toml')],
    ['cargo','clippy','--locked','--offline','--all-targets','--manifest-path',str(crate/'Cargo.toml'),'--','-D','warnings'],
]:
    run=subprocess.run(command,text=True,capture_output=True)
    commands.append(dict(command=command,exit_code=run.returncode,stdout=run.stdout,stderr=run.stderr));assert run.returncode==0,commands[-1]
(out/'rust-validation.json').write_text(json.dumps(dict(rust_tests=2,commands=commands),indent=2),encoding='utf8')
mcp=read(out/'mcp-report.json')
assert mcp['ok'] and mcp['actual_mcp_connection'] and mcp['cleanup_ok']
assert any(x['tool']=='close_database' and x['args']['save'] is False and x['ok'] for x in mcp['cleanup'])
addresses=set(native['original_entries'].values())|{'0x140b05f50','0x140a96740','0x140bee150','0x140bff400'}
calls=read(root/'evidence/render-recovery/workgraph/calls.json')['functions']
funcs=read(root/'evidence/render-recovery/workgraph/functions.json')
incoming=[dict(caller=caller,**call) for caller,result in calls.items() for call in result['direct'] if call['target'] in addresses]
(out/'call-evidence.json').write_text(json.dumps(dict(functions=[f for f in funcs if f['va'] in addresses],incoming_direct=incoming,outgoing={a:calls.get(a) for a in sorted(addresses)},coverage='Workgraph/PDB observed direct-call boundaries; helper functions may be absent from scoped workgraph. Indirect calls unresolved and xrefs incomplete.'),indent=2),encoding='utf8')
raw=Path('D:/game/ljxsj_92385/Tiny Glade/tiny-glade.rne').read_bytes();h=struct.unpack_from('<I',raw,60)[0];o=h+24;base=struct.unpack_from('<Q',raw,o+24)[0]
so=o+struct.unpack_from('<H',raw,h+20)[0];sections=[struct.unpack_from('<IIII',raw,so+40*i+8) for i in range(struct.unpack_from('<H',raw,h+6)[0])]
def slice_bytes(va,n):return next(raw[p+va-base-r:p+va-base-r+n] for vs,r,sz,p in sections if r<=va-base<r+vs)
machine=[]
for entry in read(out/'pe-boundaries.json'):
    start=int(entry['start'],16);machine.append(dict(**entry,body_sha256=hashlib.sha256(slice_bytes(start,entry['length'])).hexdigest()))
for va,n in [(0x142d06398,6),(0x142ae7890,10)]:
    data=slice_bytes(va,n*4);machine.append(dict(jump_table=hex(va),targets=[hex(va+x) for x in struct.unpack('<'+'i'*n,data)],sha256=hashlib.sha256(data).hexdigest()))
(out/'machine-code-identity.json').write_text(json.dumps(machine,indent=2),encoding='utf8')
with gzip.open(out/'native-oracle.jsonl.gz','rt',encoding='utf8') as stream:records=[json.loads(line) for line in stream]
assert len(records)==native['cases']
input_hash=hashlib.sha256(('\n'.join(json.dumps(record['input'],sort_keys=True,separators=(',',':')) for record in records)+'\n').encode()).hexdigest()
error_cases=sum('Err' in r['native'] for r in records)
dependencies=read(out/'dependency-verification.json');assert dependencies['matching_original']
source_files=list(crate.glob('src/**/*.rs'))+[crate/'Cargo.toml',crate/'Cargo.lock']
dependencies_files=list((root/'reconstruction/render/work-packages/pipeline-descriptors-r7').glob('src/**/*.rs'))+[root/'reconstruction/render/work-packages/pipeline-descriptors-r7/Cargo.toml',root/'reconstruction/render/work-packages/pipeline-descriptors-r7/Cargo.lock']
evidence=[p for p in out.iterdir() if p.is_file() and p.name not in ['checksums.json','validation.json']]
inputs=[root/'tracking/original-Cargo.lock',root/'scripts/render-recovery/mcp-evidence-job.mjs',root/'evidence/render-recovery/AGENT_EXECUTION_GUIDE.md',root/'evidence/render-recovery/WORKFLOW_RESET.md']
checksums={str(p.relative_to(root)).replace('\\','/'):digest(p) for p in sorted(source_files+dependencies_files+evidence+inputs)}
(out/'checksums.json').write_text(json.dumps(checksums,indent=2),encoding='utf8')
report=dict(schema_version=1,status='passed',source_status='verified_partial',consumer_entry='prepare_compute_pass',backend_entry='BindPlan::execute',source='reconstruction/render/work-packages/descriptor-binding-r8/src/lib.rs',source_sha256=native['source_sha256'],harness_sha256=digest(out/'verify.py'),binary_sha256=native['binary_sha256'],native_cases=native['cases'],native_groups=native['groups'],native_error_cases=error_cases,mismatches=0,candidate_rust_tests=2,registry_packages_matching_original=len(dependencies['packages']),dependency_reuse='frozen R7 source and original ash/serde/serde_json; all source/checksums match original lock',input_semantic_sha256=input_hash,checksums_file='checksums.json',mcp_actual_connection=True,mcp_function_count=7,mcp_close_save=False,shader_reflection_producer='typed caller input, private selection unresolved',sparse_tuple_consumer='native first chained mapper through write/update/bind verified',reflected_catalog_adapter=False,shared_render_integration='root-owner separately verifies actual callback wiring; independent crate tests do not establish integration',candidate_wrapper_scope='prepare_compute_pass composes verified rules and derives set/declared keys from typed interface; not a native whole-pipeline oracle',private_descriptor_set_allocation='injected unresolved private boundary',real_vulkan_gpu=False,real_game_pass_binding=False)
(out/'validation.json').write_text(json.dumps(report,indent=2),encoding='utf8');print(json.dumps(report,indent=2))
