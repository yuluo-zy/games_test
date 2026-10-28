"""Freeze only gpu_surface_collect via the qualified private Windows Job.

No rendering occurs: --execute builds and runs --list only. Default is read-only.
This correctness fixture uses System, not the headless CPU fixture's mimalloc.
"""
import argparse
import importlib.util
import json
import os
from pathlib import Path
import shutil
import subprocess
import sys
import time
import threading

HERE = Path(__file__).resolve().parent
GUARD_HELPER_SHA = '021fb94acfe1524adf79ff1f67fb04da975664a68742e50d9e03359934b96775'
JOB_SHA = '7fdd373b73872490d52a4e1ce8a41832dbfc2d997c45688b6c1c7dd8dfb02b88'
TARGET = 'gpu_surface_collect'
ENTRY = 'real_lake_surface_same_prefix_zero_tolerance'
COMMAND = ['cargo', 'build', '-p', 'tiny-glade', '--test', TARGET,
           '--locked', '--offline', '-j1', '--message-format=json-render-diagnostics',
           '--features', 'cpu-probe,surface-compare-test']

def load_module(name, path):
    spec = importlib.util.spec_from_file_location(name, path)
    module = importlib.util.module_from_spec(spec)
    spec.loader.exec_module(module)
    return module

def guard_api():
    import hashlib
    path = HERE/'freeze_headless_rich_debug.py'
    if hashlib.sha256(path.read_bytes()).hexdigest() != GUARD_HELPER_SHA:
        raise ValueError('reviewed source guard helper changed')
    return load_module('surface_source_guard', path)

def validate_artifact(event, root):
    target = event.get('target', {})
    if event.get('reason') != 'compiler-artifact' or target.get('name') != TARGET:
        return False
    if target.get('kind') != ['test'] or not event.get('executable'):
        raise ValueError('surface artifact is not an executable test')
    if event.get('features') != ['cpu-probe', 'default', 'mimalloc', 'surface-compare-test']:
        raise ValueError('surface feature identity differs')
    profile = event.get('profile', {})
    if profile.get('test') is not True or profile.get('opt_level') != '0':
        raise ValueError('surface debug/test profile differs')
    if Path(event['manifest_path']).resolve() != (root/'crates/tiny-glade/Cargo.toml').resolve():
        raise ValueError('surface manifest belongs to another package')
    if Path(target.get('src_path', '')).resolve() != (root/'crates/tiny-glade/tests/gpu_surface_collect.rs').resolve():
        raise ValueError('surface target source differs')
    exe = Path(event['executable']).resolve()
    if exe.parent != (root/'target/debug/deps').resolve() or not exe.is_file():
        raise ValueError('surface executable outside the declared debug target')
    if not exe.with_suffix('.pdb').is_file():
        raise ValueError('surface executable has no matching PDB')
    return True

def child(spec_path):
    spec = json.loads(Path(spec_path).read_text(encoding='utf-8'))
    if spec['command'] != COMMAND:
        raise ValueError('only the exact surface debug command is permitted')
    gate = Path(spec['gate'])
    deadline = time.monotonic()+30
    while not gate.exists():
        if time.monotonic() >= deadline:
            raise TimeoutError('private Job was not assigned before Cargo')
        time.sleep(.01)
    cargo = subprocess.Popen(COMMAND, cwd=spec['cwd'])
    print(json.dumps({'reason':'surface-owned-cargo-start', 'pid':cargo.pid,
                      'wrapper_pid':os.getpid()}), flush=True)
    code = cargo.wait()
    print(json.dumps({'reason':'surface-owned-cargo-finish', 'exit_code':code}), flush=True)
    raise SystemExit(code)

def build_target(root, env, dest, job_module, timeout):
    life = {'strategy':'surface requested test only / qualified private Job',
            'requested_target_completed':False, 'cargo_exit_code':None,
            'wrapper_exit_code':None, 'entire_cargo_build_passed':False,
            'termination_was_intentional_after_requested_artifact':False,
            'active_build_processes_remaining':None,
            'remaining_targets_are_not_accepted':True}
    gate, spec_path = dest/'build-job-assigned.go', dest/'build-job-spec.json'
    spec_path.write_text(json.dumps({'gate':str(gate), 'command':COMMAND, 'cwd':str(root)}), encoding='utf-8')
    job, process, artifact = job_module.Job(), None, None
    timer=None;expired=threading.Event();deadline_errors=[]
    try:
        with (dest/'build.jsonl').open('x',encoding='utf-8') as output, (dest/'build.stderr.log').open('x',encoding='utf-8') as errors:
            process = subprocess.Popen([sys.executable,'-B',str(Path(__file__).resolve()),'--_child',str(spec_path)],
                cwd=root,env=env,stdout=subprocess.PIPE,stderr=errors,text=True,encoding='utf-8')
            job.assign(process)
            life.update(wrapper_pid=process.pid, job_assigned_before_cargo_start=True)
            def expire():
                expired.set()
                try:job.terminate()
                except Exception as error:deadline_errors.append(repr(error))
            timer=threading.Timer(timeout,expire);timer.daemon=True;timer.start()
            life.update(timeout_seconds=timeout,deadline_assigned_before_go=True)
            gate.write_text('assigned',encoding='utf-8')
            for line in process.stdout:
                output.write(line); output.flush()
                try: event=json.loads(line)
                except ValueError: continue
                if event.get('reason')=='surface-owned-cargo-start': life['cargo_pid']=event['pid']
                elif event.get('reason')=='surface-owned-cargo-finish': life['cargo_exit_code']=event['exit_code']
                elif event.get('reason')=='compiler-message' and event.get('message',{}).get('level')=='error':
                    raise RuntimeError('Cargo emitted an error before the requested target completed')
                if validate_artifact(event,root):
                    artifact=event
                    life['requested_target_completed']=True
                    job.terminate()
                    life.update(termination_was_intentional_after_requested_artifact=True,
                                job_termination_exit_code=job_module.TERMINATED_AFTER_TARGET)
                    break
            life['wrapper_exit_code']=process.wait(timeout=30)
            job.wait_empty(); life['active_build_processes_remaining']=False
        if expired.is_set():raise TimeoutError('whole private surface build Job deadline reached')
        if deadline_errors:raise RuntimeError('deadline cleanup failed: '+repr(deadline_errors))
        if artifact is None: raise RuntimeError('Cargo ended without the requested surface artifact')
        life['entire_cargo_build_passed']=life['cargo_exit_code']==0
        return artifact,life
    finally:
        if timer is not None:timer.cancel();timer.join()
        life.update(timed_out=expired.is_set(),deadline_errors=deadline_errors)
        job.close()
        if process is not None and process.poll() is None: life['wrapper_exit_code']=process.wait(timeout=30)
        (dest/'cargo-lifecycle.json').write_text(json.dumps(life,indent=2)+'\n',encoding='utf-8')


def read_policy(path,expected_sha,api):
    if api.digest(path)!=expected_sha:raise ValueError('explicit reviewed local policy SHA differs')
    policy=json.loads(Path(path).read_text(encoding='utf-8'))
    expected={'schema':'surface-local-adapter-policy-v1','adapter_sha256':api.digest(Path(__file__)),
        'job_helper_sha256':JOB_SHA,'source_guard_helper_sha256':GUARD_HELPER_SHA,
        'cargo_command':COMMAND,'artifact_features':['cpu-probe','default','mimalloc','surface-compare-test'],
        'metadata_features':['default','cpu-probe','surface-compare-test'],'fixture_entry':ENTRY,
        'fixture_mode':'same-prefix-local-correctness','profile':'debug',
        'allocator':'System (correctness fixture; not a CPU benchmark)'}
    if any(policy.get(key)!=value for key,value in expected.items()):raise ValueError('local adapter policy constants differ')
    if policy.get('asset_scene_relative')!='assets/starting-builds/full/05-rocky-lake/history.json' or policy.get('asset_scene_sha256')!='f85cba25b4a193d517402a57df3590323f3d388b4f8181899c4b7fed7272fec3':raise ValueError('policy original lake identity differs')
    if policy.get('tg_install_root')!=r'D:\game\ljxsj_92385\Tiny Glade':raise ValueError('policy asset root differs')
    if policy.get('fmod_sha256')!={'fmod.dll':'d563ad96042c096985735b0d1fdab80fe5c7f27663171ff1550ac8ebf4c7eba6','fmodstudio.dll':'dc61fae2e70ec3a16fa78b678a0fc9ff1852491077a21c2b9c11bf4d160207ff'}:raise ValueError('policy FMOD identity differs')
    expected_env={'CARGO_INCREMENTAL':'0','CARGO_BUILD_JOBS':'1','TG_INSTALL_ROOT':policy['tg_install_root'],
        'PYTHONDONTWRITEBYTECODE':'1','VK_INSTANCE_LAYERS':'VK_LAYER_KHRONOS_validation','RHAPSODY_REQUIRE_VULKAN':'1'}
    if policy.get('fixed_environment')!=expected_env:raise ValueError('local fixed environment differs')
    return policy

def build_environment(api,guard,policy):
    for key,value in os.environ.items():
        if not value:continue
        name=key.upper()
        if (name in ('RUSTFLAGS','CARGO_ENCODED_RUSTFLAGS','RUSTC','RUSTC_WRAPPER','RUSTC_WORKSPACE_WRAPPER','RUSTC_BOOTSTRAP') or name.startswith(('CARGO_PROFILE_','CARGO_TARGET_','CARGO_BUILD_RUST'))) and name!='CARGO_TARGET_DIR':raise ValueError('undeclared codegen environment: '+key)
        if (name.startswith('VK_') and name not in ('VK_SDK_PATH','VK_INSTANCE_LAYERS')) or name.startswith(('DISABLE_VK_','ENABLE_VK_')):raise ValueError('undeclared Vulkan environment: '+key)
    env={key:value for key,value in api.isolated_git_env().items() if not key.upper().startswith(('TG_','RHAPSODY_'))}
    env.update(policy['fixed_environment']);env.update(guard['fixed_environment'])
    for key,value in policy['fixed_environment'].items():
        if env.get(key)!=value:raise ValueError('source guard overrides fixed policy environment')
    if env.get('CARGO_TARGET_DIR')!=str(Path(guard['root'])/'target'):raise ValueError('source guard target directory differs')
    return env

def main():
    p=argparse.ArgumentParser()
    p.add_argument('--guard',type=Path,required=True);p.add_argument('--guard-sha256',required=True)
    p.add_argument('--label',required=True);p.add_argument('--execute',action='store_true')
    p.add_argument('--local-policy',type=Path,required=True);p.add_argument('--local-policy-sha256',required=True)
    p.add_argument('--timeout-seconds',type=int,default=600)
    args=p.parse_args();api=guard_api()
    if api.digest(args.guard)!=args.guard_sha256: raise ValueError('source guard changed')
    import re
    if not re.fullmatch(r'[A-Za-z0-9_-]+',args.label): raise ValueError('unsafe frozen label')
    guard=json.loads(args.guard.read_text(encoding='utf-8'));root=Path(guard['root'])
    before=api.verify(guard)
    if not 1<=args.timeout_seconds<=1800:raise ValueError('surface build deadline outside fixed1..1800s range')
    policy=read_policy(args.local_policy,args.local_policy_sha256,api)
    if guard.get('label')!=args.label or guard.get('fixture_entry')!=ENTRY or guard.get('expected_cargo_command')!=COMMAND or guard.get('local_adapter_policy_sha256')!=args.local_policy_sha256:
        raise ValueError('local source guard does not declare this exact policy/entry/build')
    scene=Path(policy['tg_install_root'])/policy['asset_scene_relative']
    if api.digest(scene)!=policy['asset_scene_sha256']:raise ValueError('original lake scene identity changed')
    dest=root/'.perf/surface-builds'/args.label
    if dest.exists(): raise ValueError('surface label already exists; no overwrite')
    helper=root/'tools/perf_build_target.py'
    if api.digest(helper)!=JOB_SHA: raise ValueError('qualified private Job helper changed')
    env=build_environment(api,guard,policy)
    for key in guard['reject_layout_environment']:
        if env.get(key): raise ValueError('undeclared build layout environment '+key)
    env.update(guard['fixed_environment'])
    print(json.dumps({'execute':args.execute,'command':COMMAND,'destination':str(dest),
        'commit':before['commit'],'content_sha256':before['content_sha256'],
        'adapter_sha256':api.digest(Path(__file__)),'job_helper_sha256':JOB_SHA,
        'allocator':'System; GPU correctness fixture, not CPU acceptance'},indent=2),flush=True)
    if not args.execute:return
    if os.name!='nt': raise RuntimeError('Windows private Job required')
    dest.mkdir(parents=True,exist_ok=False)
    (dest/'source-files.json').write_text(json.dumps(before,indent=2)+'\n',encoding='utf-8')
    shutil.copyfile(args.guard,dest/'source-guard.json')
    shutil.copyfile(Path(__file__),dest/'surface-build-adapter.py')
    job_module=load_module('surface_qualified_job',helper)
    shutil.copyfile(args.local_policy,dest/'local-adapter-policy.json')
    try:artifact,life=build_target(root,env,dest,job_module,args.timeout_seconds)
    finally:
        after=api.snapshot(root)
        (dest/'source-after.json').write_text(json.dumps(after,indent=2)+'\n',encoding='utf-8')
        if after!=before:raise ValueError('source changed during local surface build')
    if life.get('timed_out') or life.get('deadline_errors'):raise RuntimeError('owned build deadline/cleanup failed')
    if api.verify(guard)!=before:raise ValueError('source changed during surface build')
    (dest/'requested-test-artifact.json').write_text(json.dumps(artifact,indent=2)+'\n',encoding='utf-8')
    sys.path.insert(0,str(root/'tools'))
    import perf_headless as perf
    from perf_windows import INSTALL
    exe=Path(artifact['executable']);files={}
    for src in (exe,exe.with_suffix('.pdb'),INSTALL/'fmod.dll',INSTALL/'fmodstudio.dll'):
        frozen=dest/src.name;expected=api.digest(src);shutil.copyfile(src,frozen)
        if src.name in policy['fmod_sha256'] and expected!=policy['fmod_sha256'][src.name]:raise ValueError('original FMOD DLL identity differs')
        if api.digest(frozen)!=expected:raise ValueError('frozen byte mismatch')
        files[frozen.name]=expected
    frozen=dest/exe.name
    listed=subprocess.run([str(frozen),'--list'],cwd=root,env=env,capture_output=True,text=True,timeout=60)
    (dest/'test-list.txt').write_text(listed.stdout+listed.stderr,encoding='utf-8')
    if listed.returncode or not re.search(r'(?m)^'+re.escape(ENTRY)+r': test\s*$',listed.stdout):raise RuntimeError('local surface exact test entry missing')
    if api.verify(guard)!=before:raise ValueError('source changed during surface freeze')
    record={'schema':'surface-frozen-build-v2','label':args.label,'executable':str(frozen),
        'executable_sha256':files[frozen.name],'files':files,'source_commit':before['commit'],
        'source_content_sha256':before['content_sha256'],'cargo_lock_sha256':api.digest(root/'Cargo.lock'),
        'profile':'debug','features':['default','cpu-probe','surface-compare-test'],'cargo_artifact_features':artifact['features'],
        'allocator':'System (correctness fixture; not a CPU benchmark)','jobs':1,'incremental':False,
        'command':COMMAND,'requested_test_target_completed':True,'cargo_lifecycle':life,
        'entire_cargo_build_passed':life['entire_cargo_build_passed'],
        'adapter_sha256':api.digest(Path(__file__)),'qualified_job_helper_sha256':JOB_SHA,
        'source_guard_sha256':args.guard_sha256,'hardware':perf.hardware(dest),
        'fixture_entry':ENTRY,'fixture_mode':'same-prefix-local-correctness',
        'local_adapter_policy_sha256':args.local_policy_sha256,
        'build_timeout_seconds':args.timeout_seconds,
        'original_scene':{'path':str(scene),'sha256':policy['asset_scene_sha256']},
        'controlled_environment':{key:env.get(key) for key in policy['fixed_environment']},
        'rustc':guard['rustc_verbose'],'scope':'same-prefix original-lake GPU correctness only; no CPU/WSI acceptance'}
    if api.verify(guard)!=before:raise ValueError('source changed during hardware metadata enumeration')
    if api.digest(scene)!=policy['asset_scene_sha256']:raise ValueError('original lake scene changed during freeze')
    final_source=api.snapshot(root)
    if final_source!=before:raise ValueError('source changed at end of local freeze')
    (dest/'source-after.json').write_text(json.dumps(final_source,indent=2)+'\n',encoding='utf-8')
    (dest/'build.json').write_text(json.dumps(record,indent=2)+'\n',encoding='utf-8')
    print(json.dumps({'metadata':str(dest/'build.json'),'exe_sha256':files[frozen.name]}))

if __name__=='__main__':
    if len(sys.argv)==3 and sys.argv[1]=='--_child':child(sys.argv[2])
    main()
