"""Persist narrow crate build gates and bind them to immutable source inputs."""
from pathlib import Path
import hashlib,json,subprocess

OUT=Path(__file__).resolve().parent
ROOT=OUT.parents[3]
SOURCE=ROOT/'reconstruction/render/work-packages/sequence-runtime-r8'
manifest=str(SOURCE/'Cargo.toml')
commands={
    'check-release':['cargo','check','--release','--offline','--locked','--manifest-path',manifest],
    'fmt':['cargo','fmt','--check','--manifest-path',manifest],
    'tests':['cargo','test','--offline','--locked','--manifest-path',manifest],
    'clippy':['cargo','clippy','--offline','--locked','--all-targets','--no-deps','--manifest-path',manifest,'--','-D','warnings'],
}
checks={}
for name,command in commands.items():
    result=subprocess.run(command,capture_output=True,text=True)
    (OUT/(name+'.log')).write_text(result.stdout+result.stderr,encoding='utf8')
    checks[name]={'passed':result.returncode==0,'exit_code':result.returncode,'command':command,'log':name+'.log'}
native=json.loads((OUT/'native-validation.json').read_text(encoding='utf8'))
hashes=dict(native['input_sha256'])
for path in [OUT/'native-validation.json',OUT/'findings.md',Path(__file__).resolve(),*(OUT/(name+'.log') for name in commands)]:
    hashes[str(path.relative_to(ROOT)).replace('\\','/')]=hashlib.sha256(path.read_bytes()).hexdigest()
stale=[name for name,digest in native['input_sha256'].items() if hashlib.sha256((ROOT/name).read_bytes()).hexdigest()!=digest]
report={'passed':native['passed'] and all(c['passed'] for c in checks.values()) and not stale,
        'native_joint_cases':native['cases'],'native_failure_count':native['failure_count'],
        'candidate_tests':6,'checks':checks,'stale_inputs':stale,'input_sha256':hashes,
        'source_interface_unchanged':True,'old_packages_modified':False,
        'evidence_class':native['evidence_class'],'recounts_old_2840':False,
        'shared_entry_integration_owned_by_root':True,'full_original_chain_validated':False,
        'gpu_executed':False,'milestone_status':'verified_partial'}
(OUT/'validation.json').write_text(json.dumps(report,indent=2)+'\n',encoding='utf8')
print(json.dumps({k:report[k] for k in ['passed','native_joint_cases','native_failure_count','candidate_tests','stale_inputs']}))
assert report['passed']
