"""候选 managed 功能链验证与原锁依赖审计；不执行新原码 oracle。"""
from pathlib import Path
import hashlib,json,struct,subprocess,tomllib

OUT=Path(__file__).resolve().parent
ROOT=OUT.parents[3]
PROJECT=ROOT/'reconstruction/rhapsody-rebuild'
owned=[PROJECT/p for p in ['src/raw/managed/handle.rs','src/raw/managed/resources.rs','src/raw/managed/tracking.rs','src/raw/disposal.rs','tests/managed.rs']]
manifest=str(PROJECT/'Cargo.toml')
commands={
    'managed-tests':['cargo','test','--offline','--locked','--manifest-path',manifest,'--test','managed'],
    'shared-store-test':['cargo','test','--offline','--locked','--manifest-path',manifest,'--lib','raw::managed::resources::tests'],
    'fmt-owned':['rustfmt','--check','--edition','2024',*(str(p) for p in owned)],
    'clippy':['cargo','clippy','--offline','--locked','--lib','--tests','--manifest-path',manifest,'--','-D','warnings'],
}
checks={}
for name,command in commands.items():
    result=subprocess.run(command,capture_output=True,text=True)
    (OUT/(name+'.log')).write_text(result.stdout+result.stderr,encoding='utf8')
    checks[name]={'passed':result.returncode==0,'exit_code':result.returncode,'command':command,'log':name+'.log'}

original=tomllib.loads((ROOT/'tracking/original-Cargo.lock').read_text(encoding='utf8'))['package']
candidate=tomllib.loads((PROJECT/'Cargo.lock').read_text(encoding='utf8'))['package']
def git_identity(source):
    if not source.startswith('git+'):return source
    repo=source.split('?')[0].removesuffix('.git')
    return repo+'#'+source.split('#')[-1]
comparisons=[]
for package in candidate:
    if 'source' not in package:continue
    candidates=[p for p in original if p['name']==package['name'] and p['version']==package['version']]
    matches=[p for p in candidates if git_identity(p.get('source',''))==git_identity(package['source']) and p.get('checksum')==package.get('checksum')]
    comparisons.append({'name':package['name'],'version':package['version'],'source':package['source'],'checksum':package.get('checksum'),'original_exact_identity_match':bool(matches)})
selected=[p for p in original if p['name'] in ['gpu-allocator','vk-sync-fork','rspirv','spirq','ash'] or p['name'].startswith('shader-')]

# 已有 update-pending 汇编声称 queue+0x80 writer；直接从同构建 PE 复核这条指令。
target=ROOT/'workspace/input/tiny-glade.exe'
raw=target.read_bytes(); pe=struct.unpack_from('<I',raw,60)[0]; opt=pe+24
base=struct.unpack_from('<Q',raw,opt+24)[0]
def native_bytes(va,length):
    rva=va-base
    for i in range(struct.unpack_from('<H',raw,pe+6)[0]):
        header=opt+struct.unpack_from('<H',raw,pe+20)[0]+40*i
        vsize,address,size,offset=struct.unpack_from('<IIII',raw,header+8)
        if address<=rva<address+max(vsize,size):return raw[offset+rva-address:offset+rva-address+length]
    raise ValueError(hex(va))
writer=native_bytes(0x140bf0c0f,7)
assert writer.hex()=='4889be80000000'

evidence=[ROOT/'evidence/render-recovery/producer-closure/SYNTHESIS.md',ROOT/'evidence/render-recovery/producer-closure/r3-managed-disposal/findings.md',ROOT/'evidence/render-recovery/producer-closure/a4-managed/findings.md',ROOT/'evidence/render-recovery/parallel-batch-06/sequence-runtime/update-pending.asm.txt',ROOT/'tracking/original-Cargo.lock',ROOT/'tracking/dependency-policy.json',PROJECT/'Cargo.toml',PROJECT/'Cargo.lock',Path(__file__).resolve()]
inputs={str(p.relative_to(ROOT)).replace('\\','/'):hashlib.sha256(p.read_bytes()).hexdigest() for p in [*owned,*evidence]}
report={
    'candidate_functional_tests_passed':checks['managed-tests']['passed'] and checks['shared-store-test']['passed'],
    'all_checks_passed':all(c['passed'] for c in checks.values()),'checks':checks,
    'source_input_sha256':inputs,'target_sha256':hashlib.sha256(raw).hexdigest(),
    'new_original_cases':0,'old_908_reused_as_new_engineering_validation':False,'gpu_executed_by_this_worker':False,
    'connected_consumer':['Managed::track consumes real ControlBlock','Managed::Clone/Drop use actual references','shared take_out_dropped_waste calls dispose_control_kind then returns slot','root lifecycle emits completed sequence to raw destruction buckets'],
    'removed_private_fixture':['per-resource domain BTreeMap','seven independent dropped record sources','generation increment on slot after GPU complete','unused EventOracle shadow resource/domain model'],
    'preserved_boundaries':['raw Vulkan allocation/destruction unchanged','legacy receipts only a single-domain candidate adapter','safe Arc/Mutex/Vec storage not original slab/Treiber ABI','9 raw handle Vec + 0x48 records fixed-order destroy mechanism not reconstructed byte for byte','resource-tag3/case9 and cross-queue mapping remain explicit gaps'],
    'static_writer_correction':{'address':'0x140bf0c0f','bytes':writer.hex(),'instruction':'mov qword ptr [rsi+0x80],rdi','entry':'0x140bf0b90','entry_param1_becomes_rsi_at':'0x140bf0b99','after_indirect_query_output_load_at':'0x140bf0bbd','proves':'selected original update-pending function writes queue+0x80','does_not_prove':'frame scheduling/caller reachability or full GPU completion chain'},
    'original_dependency_packages':selected,'candidate_package_comparison':comparisons,
    'all_present_external_packages_match_original_identity':all(p['original_exact_identity_match'] for p in comparisons),
    'absent_original_boundaries':{'gpu-allocator':'0.26.0 local/path identity; repository/commit/checksum not in lock. Dedicated Vulkan allocator candidate is not this allocator implementation.','vk-sync-fork':'0.4.0 local/path identity; original fork source unavailable. CPU access rules must not be represented as restored public classifier.','shader-compiler/shader-pipeline':'0.1.0 local/path; original ownership/source must be classified; candidate uses original rspirv plus saved reflection data, not full original shader toolchain.','shader-prepper':'0.3.0-pre.3 registry package original checksum known; not present in candidate lock.'},
    'root_integration_verification':'root owns executor/Vulkan hooks and complete shared-entry/GPU acceptance; this report validates actual managed module consumers only',
    'milestone_status':'verified_partial',
}
(OUT/'managed-repair.json').write_text(json.dumps(report,ensure_ascii=False,indent=2)+'\n',encoding='utf8')
print(json.dumps({'candidate_functional_tests_passed':report['candidate_functional_tests_passed'],'all_checks_passed':report['all_checks_passed'],'external_package_identity_match':report['all_present_external_packages_match_original_identity'],'source_hash_count':len(inputs),'new_original_cases':0}))
