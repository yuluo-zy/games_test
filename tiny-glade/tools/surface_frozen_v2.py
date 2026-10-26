"""Cold validation of the narrowly reviewed surface private-Job v2 producer.

All proof comes from the frozen bundle; never inspect current source or Cargo.
Evidence seals are returned for the existing before/after process comparisons.
"""
import hashlib
import json
from pathlib import Path
from compare_surface_pixels import artifact_path, sha256

ADAPTER_SHA='689ab3c35546f4d0876b919956cfc31d42a21619d26539c1887e547f911b1a3e'
JOB_SHA='7fdd373b73872490d52a4e1ce8a41832dbfc2d997c45688b6c1c7dd8dfb02b88'
COMMAND=['cargo','build','-p','tiny-glade','--test','gpu_surface_collect',
         '--locked','--offline','-j1','--message-format=json-render-diagnostics','--features','cpu-probe']
TERMINATED=0xE000C0DE

def require(condition,message):
    if not condition:raise ValueError('surface v2: '+message)

def signature(value):
    return hashlib.sha256(json.dumps(value,sort_keys=True,separators=(',',':'),ensure_ascii=False).encode()).hexdigest()

def validate(metadata, frozen, exe):
    folder=Path(metadata).parent
    names=['source-files.json','source-guard.json','surface-build-adapter.py',
        'requested-test-artifact.json','build-job-spec.json','build-job-assigned.go',
        'cargo-lifecycle.json','build.jsonl','build.stderr.log','test-list.txt',
        'hardware.json','vulkan-summary.txt']
    seals={}
    for name in names:
        path=artifact_path(folder,name)
        require(path.is_file() and not (folder/name).is_symlink(),'missing/nonregular evidence '+name)
        seals[name]={'sha256':sha256(path),'size':path.stat().st_size}
    def read(name):return json.loads(artifact_path(folder,name).read_text(encoding='utf-8'))
    require(frozen.get('adapter_sha256')==ADAPTER_SHA==seals['surface-build-adapter.py']['sha256'],'unreviewed adapter')
    require(frozen.get('qualified_job_helper_sha256')==JOB_SHA,'private Job helper identity')
    require(frozen.get('source_guard_sha256')==seals['source-guard.json']['sha256'],'source guard SHA')
    source,guard=read('source-files.json'),read('source-guard.json')
    require(guard.get('expected_source_identity')==source,'source manifest/guard mismatch')
    require(source.get('status')=='' and source.get('commit')==frozen['source_commit'],'unclean/different source commit')
    files=source.get('files')
    require(isinstance(files,dict) and files and all(isinstance(k,str) and isinstance(v,str) and len(v)==64 for k,v in files.items()),'source file SHA inventory')
    require(signature(files)==source.get('content_sha256')==frozen['source_content_sha256'],'source content hash')
    require(files.get('Cargo.lock')==frozen.get('cargo_lock_sha256'),'Cargo.lock identity')
    require(guard.get('qualified_tools_sha256',{}).get('perf_build_target.py')==JOB_SHA,'guard private Job identity')
    require(all(files.get('tools/'+name)==expected for name,expected in
        guard.get('qualified_tools_sha256',{}).items()),'source/qualified tool SHA mismatch')
    require(guard.get('rustc_verbose')==frozen.get('rustc'),'rustc identity')
    root=Path(guard.get('root',''))
    require(root.is_absolute(),'source root must be absolute')
    require(frozen.get('profile')=='debug' and frozen.get('features')==['default','cpu-probe']
        and frozen.get('cargo_artifact_features')==['cpu-probe','default','mimalloc'],'features/profile')
    require(frozen.get('allocator')=='System (correctness fixture; not a CPU benchmark)' and
        frozen.get('jobs')==1 and frozen.get('incremental') is False,'allocator/jobs/incremental')
    require(frozen.get('command')==COMMAND and frozen.get('requested_test_target_completed') is True,'requested target command/completion')
    require(set(frozen['files'])=={exe.name,exe.with_suffix('.pdb').name,'fmod.dll','fmodstudio.dll'},'exact EXE/PDB/FMOD inventory')
    life=read('cargo-lifecycle.json')
    require(life==frozen.get('cargo_lifecycle'),'lifecycle sidecar mismatch')
    require(life.get('strategy')=='surface requested test only / qualified private Job'
        and life.get('requested_target_completed') is True and life.get('job_assigned_before_cargo_start') is True
        and life.get('active_build_processes_remaining') is False and life.get('remaining_targets_are_not_accepted') is True
        and life.get('termination_was_intentional_after_requested_artifact') is True,'private Job lifecycle')
    require(life.get('job_termination_exit_code')==TERMINATED and life.get('wrapper_exit_code')==TERMINATED,'Job termination status')
    require(life.get('cargo_exit_code') is None and life.get('entire_cargo_build_passed') is False
        and frozen.get('entire_cargo_build_passed') is False,'whole Cargo build must remain uncompleted')
    require(all(type(life.get(key)) is int and life[key]>0 for key in ('wrapper_pid','cargo_pid')),'owned process IDs')
    spec=read('build-job-spec.json')
    require(spec.get('command')==COMMAND and spec.get('cwd')==str(root)
        and Path(spec.get('gate','')).resolve()==(folder/'build-job-assigned.go').resolve(),'Job assignment spec')
    require((folder/'build-job-assigned.go').read_text(encoding='utf-8')=='assigned','assignment gate')
    artifact=read('requested-test-artifact.json');target=artifact.get('target',{});profile=artifact.get('profile',{})
    require(artifact.get('reason')=='compiler-artifact' and target.get('name')=='gpu_surface_collect'
        and target.get('kind')==['test'] and profile.get('test') is True and profile.get('opt_level')=='0','requested executable artifact')
    require(artifact.get('features')==['cpu-probe','default','mimalloc'],'artifact features')
    require(Path(artifact.get('manifest_path','')).resolve()==(root/'crates/tiny-glade/Cargo.toml').resolve()
        and Path(target.get('src_path','')).resolve()==(root/'crates/tiny-glade/tests/gpu_surface_collect.rs').resolve(),'artifact package/source')
    original=Path(artifact.get('executable',''))
    require(original.is_absolute() and original.parent.resolve()==(root/'target/debug/deps').resolve()
        and original.name==exe.name,'artifact original executable')
    completed=0;started=0
    with (folder/'build.jsonl').open(encoding='utf-8') as stream:
        for line in stream:
            require(not completed or not line.strip(),'unexpected records after requested target termination')
            try:event=json.loads(line)
            except ValueError:continue
            if event.get('reason')=='surface-owned-cargo-start':
                require(event.get('pid')==life['cargo_pid'] and event.get('wrapper_pid')==life['wrapper_pid'],'owned Cargo start IDs')
                started+=1
            if event.get('reason')=='compiler-message':
                require(event.get('message',{}).get('level')!='error','compiler error before target completion')
            if event.get('reason')=='compiler-artifact' and event.get('target',{}).get('name')=='gpu_surface_collect':
                require(started==1 and event==artifact,'JSONL/artifact or start order mismatch');completed+=1
    require(started==1 and completed==1,'unique owned Cargo start/requested target')
    require('real_lake_water_and_ice_record_collect_and_preserve_raw_pixel_evidence: test' in
        (folder/'test-list.txt').read_text(encoding='utf-8'),'exact surface test listing')
    require(read('hardware.json')==frozen.get('hardware'),'hardware sidecar')
    require(seals['vulkan-summary.txt']['sha256']==frozen['hardware'].get('vulkan_summary_sha256'),'Vulkan summary identity')
    return {'schema':'surface-v2-cold-build-evidence-v1','files':seals,
        'requested_test_target_completed':True,'entire_cargo_build_passed':False,
        'adapter_sha256':ADAPTER_SHA,'qualified_job_helper_sha256':JOB_SHA,
        'validator_sha256':sha256(Path(__file__)),
        'scope':'Frozen producer evidence only; no current checkout or whole-build acceptance.'}
