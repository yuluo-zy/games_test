from pathlib import Path
import hashlib, json, subprocess
root=Path('D:/game/reverse-engineering/tiny-glade')
out=root/'evidence/render-recovery/parallel-batch-05/pipeline-descriptors'
crate=root/'reconstruction/render/work-packages/pipeline-descriptors-r7'
def digest(path):return hashlib.sha256(path.read_bytes()).hexdigest()
native=json.loads((out/'native-validation.json').read_text())
assert native['source_sha256']==digest(crate/'src/lib.rs')
assert native['mismatches']==0 and native['boundaries']['mismatches']==0
commands=[]
for arguments in [
    ['cargo','fmt','--check','--manifest-path',str(crate/'Cargo.toml')],
    ['cargo','test','--locked','--offline','--manifest-path',str(crate/'Cargo.toml')],
    ['cargo','clippy','--locked','--offline','--all-targets','--manifest-path',str(crate/'Cargo.toml'),'--','-D','warnings'],
]:
    process=subprocess.run(arguments,text=True,capture_output=True)
    commands.append(dict(command=arguments,exit_code=process.returncode,stdout=process.stdout,stderr=process.stderr))
    assert process.returncode==0,commands[-1]
(out/'rust-validation.json').write_text(json.dumps(dict(rust_tests=2,commands=commands),indent=2))
calls=json.loads((root/'evidence/render-recovery/workgraph/calls.json').read_text())['functions']
functions=json.loads((root/'evidence/render-recovery/workgraph/functions.json').read_text())
addresses={'0x140be60a0','0x1418c1a10','0x1416a4b20','0x1416a2320','0x140bfef70','0x140bee150','0x140be66c0','0x140be5a40','0x1418c2150'}
incoming=[]
for caller,evidence in calls.items():
    for call in evidence['direct']:
        if call['target'] in addresses:incoming.append(dict(caller=caller,**call))
(out/'call-evidence.json').write_text(json.dumps(dict(functions=[f for f in functions if f['va'] in addresses],incoming_direct_calls=incoming,outgoing={a:calls.get(a) for a in addresses},coverage='Prior original-machine-code workgraph direct call observations; indirect resolution incomplete. Native missing xrefs do not prove missing callers.'),indent=2))
mcp=json.loads((out/'mcp-report.json').read_text())
close=mcp['results'][-1]
assert close['name']=='close_database' and close['args']['save'] is False and close['result'].get('isError') is False
assert mcp['actual_connection'] and not mcp.get('error')
dependencies=json.loads((out/'dependency-verification.json').read_text())
files=list(crate.glob('src/**/*.rs'))+[crate/'Cargo.toml',crate/'Cargo.lock']+[p for p in out.iterdir() if p.is_file() and p.name not in ['validation.json','checksums.json','mcp-stderr.log']]
checksums={str(p.relative_to(root)).replace('\\','/'):digest(p) for p in sorted(files)}
(out/'checksums.json').write_text(json.dumps(checksums,indent=2))
report=dict(schema_version=1,status='passed',private_source_status='verified_partial',binary_sha256=native['binary_sha256'],original_entries=['0x1418c2150','0x140bfef70','0x1416a4b20'],native_layout_cases=native['cases'],native_pool_entry_cases=native['boundaries']['persistent_pool_entry_cases'],native_compute_slice_cases=native['boundaries']['compute_layout_slice_cases'],native_vulkan_error_cases=len(native['vulkan_failure_branches']),native_cases_total=native['cases']+native['boundaries']['cases']+len(native['vulkan_failure_branches']),mismatches=0,candidate_rust_tests=2,registry_packages_matching_original=len(dependencies['packages']),reflection_candidate_adapter_cases=638,reflection_linking_level='candidate_declaration_adapter_only_not_real_sparse_index_or_pass_binding',source=str((crate/'src/lib.rs').relative_to(root)).replace('\\','/'),source_sha256=native['source_sha256'],checksums_file='checksums.json',mcp_actual_connection=True,mcp_selected_function_count=8,mcp_close_save=False,full_shader_pipeline_create=False,descriptor_writes_recovered=False,real_vulkan_gpu_acceptance=False)
(out/'validation.json').write_text(json.dumps(report,indent=2))
print(json.dumps(report,indent=2))
