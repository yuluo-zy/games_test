"""Run an already frozen CPU fixture with separate GPU command timestamps.

No Cargo/build/ETW operation and no clean CPU or production-window acceptance.
Standard library only. The six frozen CPU runtime-policy tools are untouched.
"""
import argparse
import hashlib
import json
import math
import os
from pathlib import Path
import re
import subprocess
import time

TEST = 'cpu_measurement::profile_shipped_session_cpu'
SCENES = ('static', 'many_draws', 'local_edit', 'continuous_edit')
FIXTURE_FILES = ('crates/tiny-glade/tests/cpu_measurement/mod.rs',
                 'crates/tiny-glade/tests/cpu_measurement/workload_probe.rs',
                 'crates/tiny-glade/tests/cpu_measurement/gpu_diagnostic.rs')
TERMINATED_AFTER_TARGET = 0xE000C0DE
# Review-bound source versions. These hashes cover the existing freezer's
# before/after source equality guard and private-Job requested-target protocol.
# A changed helper requires an explicit new source review and whitelist entry.
REVIEWED_TARGET_FREEZER = {
    'tools/perf_headless.py':'770a08d7c31a442ba087a78525dc6951ff42a09d1ad67e63c0adf329db100ed8',
    'tools/perf_build_target.py':'7fdd373b73872490d52a4e1ce8a41832dbfc2d997c45688b6c1c7dd8dfb02b88',
}


def require(value, message):
    if not value:
        raise ValueError(message)


def digest(path):
    with Path(path).open('rb') as source:
        return hashlib.file_digest(source, 'sha256').hexdigest()


def signature(value):
    return hashlib.sha256(json.dumps(value, sort_keys=True, separators=(',', ':'),
                                     ensure_ascii=False).encode()).hexdigest()


def dump(path, value):
    Path(path).write_text(json.dumps(value, indent=2, ensure_ascii=False)+'\n', encoding='utf-8')


def build_acceptance_proof(directory, build, source, frozen_exe):
    """One scope decision over the known producer's metadata and raw events."""
    completed=build.get('entire_cargo_build_passed')
    require(type(completed) is bool, 'Cargo completion scope missing')
    standalone_path=directory/'cargo-lifecycle.json'
    known=any(key in build for key in ('requested_test_target_completed','requested_target_evidence_sha256',
        'build_jsonl_sha256','build_helper_sha256','cargo_lifecycle'))
    known |= standalone_path.exists() or (directory/'requested-target-artifact.json').exists()
    if not known:
        require(completed, 'legacy build without requested-target proof must have whole Cargo success')
        return {'policy':'whole-cargo-build-passed','entire_cargo_build_passed':True}

    lifecycle=build.get('cargo_lifecycle')
    require(isinstance(lifecycle,dict) and lifecycle.get('strategy')=='requested test target only'
            and build.get('requested_test_target_completed') is True
            and lifecycle.get('requested_target_completed') is True
            and lifecycle.get('entire_cargo_build_passed') is completed
            and lifecycle.get('active_build_processes_remaining') is False
            and lifecycle.get('remaining_targets_are_not_accepted') is True,
            'known build completion scope/target/private-Job lifecycle missing or contradictory')
    proof_names=['requested-target-artifact.json','build.jsonl','build.stderr.log']
    if standalone_path.exists():
        require(json.loads(standalone_path.read_text(encoding='utf-8'))==lifecycle,
                'inline and standalone Cargo lifecycle differ')
        proof_names.append('cargo-lifecycle.json')
    artifact_path=directory/'requested-target-artifact.json'
    jsonl_path=directory/'build.jsonl'
    require(digest(artifact_path)==build.get('requested_target_evidence_sha256')
            and digest(jsonl_path)==build.get('build_jsonl_sha256'),
            'requested artifact/build JSONL evidence SHA mismatch')
    artifact=json.loads(artifact_path.read_text(encoding='utf-8'))
    require(build.get('features')==['default(mimalloc)','cpu-probe']
            and build.get('cpu_probe_feature') is True
            and build.get('profile') in ('debug','release'),
            'known build declared features/profile differ from requested artifact protocol')
    expected=['cargo','build','-p','tiny-glade','--test','original_gameplay_chain',
              '--locked','--offline','-j1','--message-format=json-render-diagnostics','--features','cpu-probe']
    if build['profile']=='release':expected.append('--release')
    require(build.get('command')==expected,'requested build command mismatch')
    manifest=Path(artifact.get('manifest_path','')).resolve()
    cwd=manifest.parents[2]
    target=artifact.get('target') or {}
    profile=artifact.get('profile') or {}
    original_exe=Path(artifact.get('executable','')).resolve()
    require(artifact.get('reason')=='compiler-artifact' and target.get('kind')==['test']
            and target.get('name')=='original_gameplay_chain' and profile.get('test') is True
            and artifact.get('features')==['cpu-probe','default','mimalloc']
            and original_exe.name==frozen_exe.name
            and original_exe.parent==(cwd/'target'/build['profile']/'deps').resolve()
            and manifest==(cwd/'crates/tiny-glade/Cargo.toml').resolve()
            and (build['profile']!='release' or profile.get('opt_level')=='3'),
            'recorded compiler artifact is not the requested completed test executable')
    filenames=[Path(name).resolve() for name in artifact.get('filenames',[])]
    require(original_exe in filenames and original_exe.with_suffix('.pdb') in filenames,
            'requested compiler artifact EXE/PDB filenames missing')
    events=[]
    for line in jsonl_path.read_text(encoding='utf-8').splitlines():
        try:event=json.loads(line)
        except ValueError:continue
        require(isinstance(event,dict),'unexpected build JSONL event shape')
        events.append(event)
    require(not any((event.get('reason')=='compiler-message' and event.get('message',{}).get('level')=='error')
                    or (event.get('reason')=='build-finished' and event.get('success') is not True)
                    for event in events),'compiler/build error observed in known build evidence')
    requested=[event for event in events if event.get('reason')=='compiler-artifact'
               and event.get('target',{}).get('name')=='original_gameplay_chain']
    require(requested==[artifact],'requested compiler artifact absent/ambiguous/differs from JSONL')
    stderr=re.sub(r'\x1b\[[0-9;]*[A-Za-z]','',(directory/'build.stderr.log').read_text(encoding='utf-8'))
    require(not re.search(r'(?m)^\s*error(?:\[E\d+\])?:',stderr,re.I),'compiler error present in build stderr')

    starts=[event for event in events if event.get('reason')=='perf-owned-cargo-start']
    finishes=[event for event in events if event.get('reason')=='perf-owned-cargo-finish']
    owned=any(lifecycle.get(key) is not None for key in ('wrapper_pid','cargo_pid')) or bool(starts or finishes)
    if owned:
        require(lifecycle.get('job_assigned_before_cargo_start') is True
                and len(starts)==1 and type(lifecycle.get('wrapper_pid')) is int and lifecycle['wrapper_pid']>0
                and type(lifecycle.get('cargo_pid')) is int and lifecycle['cargo_pid']>0
                and lifecycle['cargo_pid']!=lifecycle['wrapper_pid']
                and starts[0].get('pid')==lifecycle['cargo_pid']
                and starts[0].get('wrapper_pid')==lifecycle['wrapper_pid']
                and events.index(starts[0])<events.index(artifact),
                'private owned Cargo start/artifact ordering mismatch')
        spec=json.loads((directory/'build-job-spec.json').read_text(encoding='utf-8'))
        gate=directory/'build-job-assigned.go'
        require(Path(spec['cwd']).resolve()==cwd and spec.get('command')==expected
                and Path(spec['gate']).resolve()==gate.resolve()
                and gate.read_text(encoding='utf-8')=='assigned','private Job command/assignment gate mismatch')
        proof_names.extend(['build-job-spec.json','build-job-assigned.go'])

    if completed:
        # A known producer may claim whole success only after actual successful
        # Cargo finish events, not after rewriting mutually agreeing flags.
        build_finishes=[event for event in events if event.get('reason')=='build-finished']
        require(lifecycle.get('cargo_exit_code')==0 and lifecycle.get('wrapper_exit_code') in (None,0)
                and len(build_finishes)==1 and build_finishes[0].get('success') is True
                and events.index(artifact)<events.index(build_finishes[0]),
                'whole Cargo success lacks successful build-finished event/zero exits')
        if owned:
            require(len(finishes)==1 and finishes[0].get('pid')==lifecycle['cargo_pid']
                    and finishes[0].get('exit_code')==0
                    and events.index(build_finishes[0])<events.index(finishes[0]),
                    'whole owned Cargo success lacks matching zero-exit Cargo finish event')
        policy='whole-cargo-build-passed'
    else:
        # The whitelisted freezer publishes the frozen EXE only after exact
        # before/after source-inventory equality. Later live edits are irrelevant.
        require(build.get('dirty') is False and source.get('status')==''
                and re.fullmatch('[0-9a-f]{40}',source.get('commit') or '')
                and all(source['files'].get(name)==sha for name,sha in REVIEWED_TARGET_FREEZER.items())
                and build.get('build_helper_sha256')==REVIEWED_TARGET_FREEZER['tools/perf_build_target.py'],
                'target-only route requires clean committed source and reviewed freezer source guard')
        require(owned and standalone_path.exists()
                and lifecycle.get('termination_was_intentional_after_requested_artifact') is True
                and lifecycle.get('job_termination_exit_code')==TERMINATED_AFTER_TARGET
                and lifecycle.get('wrapper_exit_code') in (0,TERMINATED_AFTER_TARGET)
                and lifecycle.get('cargo_exit_code') in (None,TERMINATED_AFTER_TARGET),
                'target-only Job did not end safely after the completed requested artifact')
        policy='requested-target-private-job-reviewed-freezer-v1'
    proof={'policy':policy,'requested_test_target_completed':True,'entire_cargo_build_passed':completed,
           'remaining_targets_are_not_accepted':True,
           'files':{name:digest(directory/name) for name in proof_names}}
    if not completed:
        proof.update(reviewed_freezer_sources=REVIEWED_TARGET_FREEZER,
            source_before_after_guard='hash-whitelisted freezer publishes only after inventory equality; later current-worktree edits do not alter the frozen EXE')
    return proof

def frozen_identity(build_path):
    path = Path(build_path).resolve(strict=True)
    build = json.loads(path.read_text(encoding='utf-8'))
    require(build.get('schema') == 'headless-frozen-build-v1', 'unknown frozen metadata schema')
    require('cpu-probe' in build.get('features', []) and build.get('allocator') == 'mimalloc'
            and build.get('incremental') is False,
            'requires frozen mimalloc/cpu-probe build')
    files = build.get('files')
    require(isinstance(files, dict) and files, 'frozen files missing')
    for name, sha in files.items():
        require(name not in ('.','..') and Path(name).name == name and re.fullmatch('[0-9a-f]{64}', sha or ''), 'unsafe frozen file identity')
        require(digest(path.parent/name) == sha, 'frozen EXE/PDB/DLL SHA mismatch: '+name)
    exe = Path(build['executable']).resolve(strict=True)
    require(exe.parent == path.parent and files.get(exe.name) == build.get('executable_sha256')
            and digest(exe) == build['executable_sha256'], 'executable identity mismatch')
    require(exe.with_suffix('.pdb').name in files, 'frozen PDB identity required')
    require(all(name in files for name in ('fmod.dll', 'fmodstudio.dll')), 'frozen FMOD identities required')
    source_path = path.parent/'source.json'
    source = json.loads(source_path.read_text(encoding='utf-8'))
    require(isinstance(source.get('files'), dict) and source['files'], 'source inventory missing')
    require(all(re.fullmatch('[0-9a-f]{64}', source['files'].get(name) or '') for name in FIXTURE_FILES),
            'frozen source lacks diagnostic/workload fixture identities')
    require(signature(source['files']) == source.get('content_sha256') == build.get('source_content_sha256')
            and source.get('commit') == build.get('source_commit'), 'frozen source metadata mismatch')
    proof=build_acceptance_proof(path.parent,build,source,exe)
    return build, {'build_json_sha256': digest(path), 'source_json_sha256': digest(source_path),
                   'source_content_sha256': source['content_sha256'], 'files': dict(files),
                   'build_acceptance_scope':proof}


QUALITY_PRESETS = ('performance', 'balanced', 'quality')


def environment(install, session, scene, warmup, samples, capacity,
                pass_spans=False, isolate=False, statistics=False, quality='quality'):
    require(quality in QUALITY_PRESETS, f'quality preset must be one of {QUALITY_PRESETS}')
    require(pass_spans or not (isolate or statistics),
            'GPU pass isolation/statistics require pass spans')
    env = {name: value for name, value in os.environ.items()
           if not name.startswith(('TG_', 'RHAPSODY_'))}
    env.update(TG_INSTALL_ROOT=str(install), RUST_LOG='warn', CARGO_INCREMENTAL='0',
               VK_INSTANCE_LAYERS='VK_LAYER_KHRONOS_validation', RHAPSODY_REQUIRE_VULKAN='1',
               TG_TEST_SIZE='1280x720', TG_PERF_SCENARIO=scene, TG_CPU_PROBE_MODE='on',
               TG_CPU_PROBE_OUT=str(session), TG_CPU_PROBE_GATE='1',
               TG_CPU_PROBE_WARMUP=str(warmup), TG_CPU_PROBE_SAMPLES=str(samples),
               TG_GPU_DIAGNOSTIC='1', TG_GPU_DIAGNOSTIC_CAPACITY=str(capacity),
               TG_PERF_QUALITY=quality)
    if pass_spans:
        env['TG_GPU_PASS_SPANS'] = '1'
    if isolate:
        env['TG_GPU_ISOLATE'] = '1'
    if statistics:
        env['TG_GPU_STATS'] = '1'
    return env


def strict_log(text):
    require(not re.search(r'\bVUID[-:]|Validation Error|Validation Warning|VK_ERROR_|failed to.*Vulkan',
                          text, re.I), 'Vulkan error or validation diagnostic present')
    summaries = re.findall(r'test result:\s*ok\.\s*(\d+) passed;\s*(\d+) failed;\s*(\d+) ignored;', text)
    require(summaries == [('1', '0', '0')], 'requires exactly one executed successful test, no ignored/zero-test run')
    # --nocapture may print boot diagnostics between the test-name prefix and
    # its final `ok`; the exact command plus one-test summary establishes pass.
    require(re.search(r'^running 1 test\s*$',text,re.M)
            and re.search(r'^test '+re.escape(TEST)+r'\s+\.\.\.', text, re.M),
            'requested exact fixture did not execute')


def validate_session(report, pid, scene, warmup, samples,
                     pass_spans=False, isolate=False, statistics=False):
    require(pass_spans or not (isolate or statistics),
            'GPU pass isolation/statistics require pass spans')
    require(report.get('schema') == 'headless-cpu-closure-v1' and report.get('valid') is True,
            'fixture session invalid')
    require(report.get('process_id') == pid and report.get('scenario') == scene
            and report.get('warmup') == warmup and report.get('samples') == samples,
            'fixture process/config identity mismatch')
    require(report.get('measurement_class') == 'gpu_diagnostic'
            and report.get('clean_cpu_acceptance_eligible') is False
            and report.get('diagnostic_bill') is False and report.get('mode') == 'on'
            and report.get('cpu_probe_feature') is True and report.get('allocator') == 'mimalloc',
            'diagnostic mode not honored by frozen binary')
    require(isinstance(report.get('input_before'), dict) and report['input_before']
            and isinstance(report.get('input_after'), dict) and report['input_after'],
            'raw semantic input manifests missing')
    actual = report.get('actual_workload') or {}
    rows = actual.get('frames')
    require(actual.get('schema') == 'headless-actual-workload-v1' and actual.get('valid') is True
            and actual.get('errors') == [] and actual.get('truncated') is False
            and actual.get('frame_count') == samples+1 and isinstance(rows, list) and len(rows) == samples+1,
            'actual workload absent/incomplete/invalid')
    cpu_rows = report.get('frames')
    require(isinstance(cpu_rows, list) and len(cpu_rows) == samples+1, 'CPU receipt observations missing')
    for row, cpu in zip(rows, cpu_rows):
        require(row.get('valid') is True and row.get('native_commands', {}).get('valid') is True
                and row.get('collect', {}).get('valid') is True and row['collect'].get('overflow') is False
                and row['collect'].get('failed_waves') == 0, 'invalid per-frame workload')
        require(all(row.get(key) == cpu.get(key) for key in ('world_tick','receipt','attempt')),
                'workload/CPU receipt association differs')
        native=row['native_commands']
        for name in ('direct_draw_api_calls','indexed_draw_api_calls','indirect_draw_api_calls',
                     'indirect_count_draw_api_calls','indexed_indirect_draw_api_calls',
                     'indexed_indirect_count_draw_api_calls','direct_dispatch_api_calls','indirect_dispatch_api_calls',
                     'staging_h2d_copy_recorded_bytes','immutable_producer_h2d_copy_recorded_bytes',
                     'cached_d2d_copy_recorded_bytes'):
            require(type(native.get(name)) is int and 0<=native[name]<2**64,'native command classification missing/invalid')
        require(isinstance(row.get('draw_list_lengths'),list) and len(row['draw_list_lengths'])==21
                and all(type(n) is int and n>=0 for n in row['draw_list_lengths']), 'draw-list observation missing')
    gpu = report.get('gpu_diagnostic') or {}
    frames, points, commands = gpu.get('frames'), gpu.get('receipt_points'), gpu.get('samples')
    require(gpu.get('schema') == 'headless-gpu-command-diagnostic-v1' and gpu.get('valid') is True
            and gpu.get('pass_spans') is pass_spans
            and gpu.get('isolate_passes') is isolate
            and gpu.get('pass_statistics') is statistics
            and gpu.get('command_span_sum_and_max_are_not_frame_latency') is True,
            'GPU diagnostic missing/invalid or overlapping pass spans enabled')
    require(isinstance(frames,list) and len(frames)==warmup+samples+1 and frames
            and isinstance(points,list) and points and isinstance(commands,list) and commands,
            'GPU diagnostic observations missing')
    stats=gpu.get('ring_stats') or {}
    pass_commands = gpu.get('pass_samples') or []
    expected_timestamp_count = len(commands) + len(pass_commands)
    require(all(stats.get(k)==0 for k in ('pending','capacity_skips','retained_samples'))
            and stats.get('completed')==gpu.get('drained_samples')==expected_timestamp_count
            and gpu.get('sample_count')==len(commands),
            'GPU timing pending/skipped/evicted/truncated')
    require(type(gpu.get('timestamp_period_ns')) in (int,float) and math.isfinite(gpu['timestamp_period_ns'])
            and gpu['timestamp_period_ns']>0,
            'GPU timestamp period missing')
    bits=gpu.get('queue_timestamp_valid_bits')
    require(isinstance(bits,list) and bits and all(type(n) is int and 1<=n<=64 for n in bits),
            'GPU timestamp validBits invalid')
    owners={}
    for point in points:
        queue,value,frame=point.get('queue'),point.get('value'),point.get('frame')
        require(type(queue) is int and 0<=queue<len(bits) and type(value) is int and value>0
                and type(frame) is int and 0<=frame<len(frames) and (queue,value) not in owners,
                'duplicate/invalid GPU receipt point')
        owners[queue,value]=frame
    counts=[0]*len(frames)
    associations=[]
    epochs={}
    for command in commands:
        frame=owners.get((command.get('queue'),command.get('completion_value')))
        require(frame is not None and type(command.get('duration_ns')) is int and command['duration_ns']>=0
                and type(command.get('recording_epoch')) is int and command['recording_epoch']>0,
                'unmatched/invalid raw GPU sample')
        epoch=command['recording_epoch']
        require(epochs.setdefault(frame,epoch)==epoch, 'multiple recording epochs in one successful frame')
        counts[frame]+=1
        associations.append(frame)
    require(associations==gpu.get('sample_frame_indices'), 'declared GPU sample association mismatch')
    for index, (frame,count) in enumerate(zip(frames,counts)):
        require(type(frame.get('expected_commands')) is int and frame['expected_commands']>0
                and count==frame['expected_commands'] and frame.get('measured') is (index>=warmup),
                'missing/duplicated command timings or warmup association mismatch')
    pass_associations = gpu.get('pass_sample_frame_indices')
    statistic_fields = ('input_vertices','input_primitives','vertex_invocations',
                        'clipping_invocations','clipping_primitives',
                        'fragment_invocations','compute_invocations')
    if pass_spans:
        require(isinstance(pass_commands,list) and pass_commands
                and isinstance(pass_associations,list)
                and len(pass_commands)==len(pass_associations)==gpu.get('pass_sample_count'),
                'GPU pass timing samples missing')
        per_frame=[0]*len(frames)
        unique=set()
        for sample, frame in zip(pass_commands,pass_associations):
            require(type(frame) is int and 0<=frame<len(frames), 'GPU pass sample has no frame owner')
            require(type(sample.get('pass_index')) is int and sample['pass_index']>=0
                    and isinstance(sample.get('pass_name'),str) and sample['pass_name'],
                    'GPU pass identity invalid')
            require(type(sample.get('duration_ns')) is int and sample['duration_ns']>=0
                    and type(sample.get('recording_epoch')) is int and sample['recording_epoch']>0,
                    'GPU pass timing invalid')
            require((frame,sample['pass_index']) not in unique, 'duplicate GPU pass timing')
            unique.add((frame,sample['pass_index']))
            require(owners.get((sample.get('queue'),sample.get('completion_value')))==frame,
                    'GPU pass receipt association differs')
            pass_statistics = sample.get('statistics')
            if statistics:
                require(isinstance(pass_statistics,dict)
                        and all(type(pass_statistics.get(name)) is int and pass_statistics[name]>=0
                                for name in statistic_fields),
                        'GPU pass pipeline statistics missing')
            else:
                require(pass_statistics is None, 'unexpected GPU pass statistics')
            per_frame[frame]+=1
        require(all(count>0 for count in per_frame), 'GPU pass timing missing from a frame')
    else:
        require(not pass_commands, 'pass spans unexpectedly enabled')
    require(all(all(a.get(k)==b.get(k) for k in ('world_tick','receipt','attempt'))
                for a,b in zip(frames[warmup:],rows)), 'GPU/measured workload association mismatch')
    require(all(a['expected_commands']==b.get('graph_commands_recorded')
                for a,b in zip(frames[warmup:],rows)), 'GPU expected command count differs from real receipt stats')


def run(args):
    require(args.scene in SCENES and args.warmup>=1 and 0<args.samples<=100000
            and (args.scene=='static' or args.warmup>=200), 'invalid workload configuration')
    capacity=args.capacity or min((args.warmup+args.samples+1)*128,2000000)
    require(0<capacity<=2000000 and args.timeout>0, 'invalid diagnostic bounds')
    build_path=Path(args.build_json).resolve(strict=True)
    build,identity=frozen_identity(build_path)
    destination=Path(args.output).resolve()
    destination.mkdir(parents=True,exist_ok=False)
    session=destination/'session.json'
    log_path=destination/'fixture.log'
    controller=Path(__file__).resolve()
    record={'schema':'headless-gpu-diagnostic-run-v1','valid':False,
            'clean_cpu_acceptance_eligible':False,'production_16ms_acceptance':False,
            'scope':'separate command-span timestamp experiment; no ETW CPU result or whole-frame GPU latency',
            'controller':{'path':str(controller),'sha256':digest(controller)},
            'frozen_build':identity,'build_json':str(build_path),'settings':{
                'scene':args.scene,'warmup':args.warmup,'samples':args.samples,'capture_capacity':capacity,
                'ring_pending_capacity':4096,'ring_retained_capacity':4096,
                'pass_spans':args.pass_spans,'isolate_passes':args.isolate,
                'pass_statistics':args.stats,
                'quality_preset':getattr(args,'quality','quality'),
                'dimensions':'1280x720','process_priority':'NORMAL_PRIORITY_CLASS'},
            'artifacts':{},'started_utc':time.strftime('%Y-%m-%dT%H:%M:%SZ',time.gmtime())}
    process=None
    try:
        env=environment(Path(args.install_root).resolve(strict=True),session,args.scene,args.warmup,args.samples,capacity,
                        args.pass_spans,args.isolate,args.stats,getattr(args,'quality','quality'))
        command=[build['executable'],TEST,'--exact','--ignored','--nocapture','--test-threads=1']
        record['command']=command
        record['runtime_vulkan_environment']={k:v for k,v in sorted(env.items()) if k.startswith(('VK_','VULKAN_'))}
        with log_path.open('w',encoding='utf-8') as log:
            process=subprocess.Popen(command,cwd=args.workspace,env=env,stdout=log,stderr=subprocess.STDOUT,
                                     creationflags=getattr(subprocess,'NORMAL_PRIORITY_CLASS',0))
            record['process_id']=process.pid
            deadline=time.monotonic()+args.timeout
            while not session.with_suffix('.ready').exists():
                if process.poll() is not None:
                    raise RuntimeError('fixture exited before warmup handshake; see fixture.log')
                if time.monotonic()>=deadline:
                    raise TimeoutError('fixture warmup handshake timed out')
                time.sleep(.05)
            _,pre_go_identity=frozen_identity(build_path)
            require(pre_go_identity==identity,'frozen build changed during warmup')
            session.with_suffix('.go').write_text('go',encoding='ascii')
            record['exit_code']=process.wait(timeout=max(.1,deadline-time.monotonic()))
        require(record['exit_code']==0,'GPU diagnostic fixture failed; see fixture.log')
        strict_log(log_path.read_text(encoding='utf-8'))
        validate_session(json.loads(session.read_text(encoding='utf-8')),process.pid,args.scene,args.warmup,args.samples,
                         args.pass_spans,args.isolate,args.stats,getattr(args,'quality','quality'))
        _,after=frozen_identity(build_path)
        require(after==identity and digest(controller)==record['controller']['sha256'],
                'frozen executable/source/controller identity changed')
        record['valid']=True
    except BaseException as error:
        record['error']=repr(error)
        raise
    finally:
        if process is not None and process.poll() is None:
            process.terminate()
            try:
                process.wait(timeout=15)
            except subprocess.TimeoutExpired:
                process.kill();process.wait(timeout=15)
        for path in (session,log_path,session.with_suffix('.ready'),session.with_suffix('.go')):
            if path.is_file():
                sha=digest(path)
                record['artifacts'][path.name]=sha
                path.with_name(path.name+'.sha256').write_text(sha+'  '+path.name+'\n',encoding='ascii')
        dump(destination/'run.json',record)
    return record


def main():
    parser=argparse.ArgumentParser(description=__doc__)
    parser.add_argument('--build-json',required=True)
    parser.add_argument('--output',required=True)
    parser.add_argument('--workspace',default=str(Path(__file__).resolve().parents[1]))
    parser.add_argument('--install-root',default=r'D:\game\ljxsj_92385\Tiny Glade')
    parser.add_argument('--scene',choices=SCENES,default='static')
    parser.add_argument('--warmup',type=int,default=600)
    parser.add_argument('--samples',type=int,default=3000)
    parser.add_argument('--capacity',type=int)
    parser.add_argument('--pass-spans',action='store_true',
                        help='record named per-pass GPU timestamp spans')
    parser.add_argument('--isolate',action='store_true',
                        help='serialise pass spans to measure exclusive pass cost; requires pass spans')
    parser.add_argument('--stats',action='store_true',
                        help='record pipeline statistics for each pass; requires pass spans')
    parser.add_argument('--quality',choices=QUALITY_PRESETS,default='quality',
                        help='quality preset: performance=TGSR(Performance)+no DoF, '
                             'balanced=TGSR(Quality)+DoF, quality=no upscaling+DoF (default, the original evidence preset)')
    parser.add_argument('--timeout',type=float,default=3600)
    print(json.dumps(run(parser.parse_args()),indent=2,ensure_ascii=False))


if __name__=='__main__':
    main()
