"""Prospective cross-source headless A/B planning and offline acceptance only.

Never launches a build, target process, logger, or benchmark. Acceptance requires
replaying the frozen account implementation at least once and complete workload
witnesses. Default evaluation rechecks artifact SHA bindings against audit seals.
"""
import argparse
from datetime import datetime, timezone
import hashlib
import importlib
import json
import math
from pathlib import Path
import re
import statistics
import sys
import uuid

import perf_headless_workload as workload

SCENES = ('static', 'many_draws', 'local_edit', 'continuous_edit')
MODES = ('constant_batch', 'reflection_scope', 'vao_typed_lookup', 'vao_wrapper_reuse')
PHASES = ('app_update', 'pacing', 'draw_lists_vao', 'declare', 'compile',
          'pending_admission', 'record_submit', 'maintenance')
METRICS = ('render_phase_owner_cpu_ms', 'app_cpu_ms',
           'host_completion_owner_cpu_ms', 'process_cpu_ms', 'non_owner_cpu_ms')
TOOLS = ('perf_clock_policy.py', 'perf_headless.py', 'perf_headless_account.py',
         'perf_etw.py', 'perf_trace_archive.py', 'perf_windows.py')
POLICY = 'windows-etw-qpc-calibrated-v1'
INPUT_POLICY = 'fixed_step_scripted_scene; paired_before_after_trajectory_must_match'
RULES = {
    'warmup': 600, 'samples': 3000, 'orders': ['AB', 'BA', 'AB', 'BA', 'AB'],
    'render_p95_limit_ms': 16.0, 'maximum_regression': .05,
    'regression_metrics': ['render_phase_owner_cpu_ms'], 'regression_quantiles': ['p95_ms', 'p99_ms'],
    'extension_regression_near': .03, 'extension_p95_near_ms': 15.2,
    'extension_delta_range': .03, 'extension_p95_range_ms': .8,
    'five_pair_ci': 'one-sided 95% Student t(4)=2.131846786326649 on paired run ratios',
    'five_pair_t': 2.131846786326649,
    'missing_workload_proof': 'reject, never infer total uploads from partial counters',
    'cost_shift_policy': 'other CPU metrics reported as flags requiring analysis, not automatic rejection',
    'rules_status': 'prospective v2 review proposal; numeric near/CI rules are not retroactive user requirements',
}
SCOPE = ('Headless App+renderer shared owner, exclusive ETW phase CPU. '
         'No production WSI, dedicated render-thread or final 16ms acceptance.')


def require(test, message):
    if not test:
        raise ValueError(message)


def signature(value):
    return hashlib.sha256(json.dumps(value, sort_keys=True, separators=(',', ':'),
                                     ensure_ascii=False, allow_nan=False).encode()).hexdigest()


def digest(path):
    with Path(path).open('rb') as stream:
        return hashlib.file_digest(stream, 'sha256').hexdigest()


def read(path):
    return json.loads(Path(path).read_text(encoding='utf-8'))


def write_new(path, value):
    path = Path(path)
    path.parent.mkdir(parents=True, exist_ok=True)
    with path.open('x', encoding='utf-8', newline='\n') as stream:
        json.dump(value, stream, indent=2, ensure_ascii=False, allow_nan=False)
        stream.write('\n')


def reference(path):
    return {'path': str(Path(path).resolve()), 'sha256': digest(path)}


def checked_ref(ref):
    path = Path(ref['path'])
    require(digest(path) == ref['sha256'], 'referenced artifact SHA changed: '+str(path))
    return path


def timestamp(value):
    parsed = datetime.fromisoformat(value.replace('Z', '+00:00'))
    require(parsed.tzinfo is not None, 'UTC/offset required')
    return parsed


def finite(value, label):
    require(type(value) in (int, float) and math.isfinite(value) and value >= 0,
            'invalid finite nonnegative '+label)
    return value


def modes(value):
    require(isinstance(value, dict) and set(value) == set(MODES)
            and all(v is True for v in value.values()), 'missing/non-default management modes')
    return value


def build_common(build):
    result = {key: build[key] for key in ('profile', 'features', 'allocator', 'incremental',
        'jobs', 'command', 'build_environment', 'build_environment_sha256',
        'cargo_lock_sha256', 'rustc', 'build_helper_sha256')}
    result['runtime_assets'] = {name: sha for name, sha in build['files'].items()
                               if not name.endswith(('.exe', '.pdb'))}
    return result


def load_build(path):
    path = Path(path).resolve()
    build = read(path)
    require(build['schema'] == 'headless-frozen-build-v1', 'unknown frozen build schema')
    require(build['profile'] == 'release' and build['dirty'] is False,
            'release clean frozen source required; debug/dirty rejected')
    require(build['features'] == ['default(mimalloc)', 'cpu-probe']
            and build['cpu_probe_feature'] is True and build['allocator'] == 'mimalloc',
            'only clean cpu-probe/default mimalloc builds accepted')
    require(build['incremental'] is False and build['jobs'] == 1,
            'build configuration differs from fixed baseline')
    require(build['requested_test_target_completed'] is True
            and build['cargo_lifecycle']['active_build_processes_remaining'] is False,
            'requested target incomplete or cargo still active')
    require(signature(build['build_environment']) == build['build_environment_sha256'],
            'build environment hash mismatch')
    require(re.fullmatch(r'[0-9a-f]{40,64}', build['source_commit']) is not None,
            'missing full source commit')
    require(re.fullmatch(r'[0-9a-f]{64}', build['source_content_sha256']) is not None,
            'missing source content identity')
    source = read(path.with_name('source.json'))
    require(source['commit'] == build['source_commit'] and not source['status']
            and source['content_sha256'] == build['source_content_sha256']
            and signature(source['files']) == source['content_sha256'],
            'source snapshot differs from frozen build')
    require(Path(build['executable']).resolve().parent == path.parent,
            'executable outside frozen build directory')
    require(Path(build['executable']).name in build['files']
            and build['files'][Path(build['executable']).name] == build['executable_sha256'],
            'executable identity not frozen')
    require(any(name.endswith('.pdb') for name in build['files']), 'missing frozen PDB identity')
    for name, expected in build['files'].items():
        require(Path(name).name == name, 'frozen file escapes build directory')
        require(digest(path.parent/name) == expected, 'frozen executable/PDB/DLL changed: '+name)
    controls = build.get('management_controls') or {}
    require(controls.get('default_modes') is None and controls.get('probe_runtime_controls') == [],
            'common harness must freeze no runtime mode-control capability, never invented four booleans')
    require(controls['source_content_sha256'] == build['source_content_sha256'],
            'management control source identity mismatch')
    return build


def module_identity(tools_dir):
    return {name: digest(Path(tools_dir)/name) for name in TOOLS}


def make_plan(series, baseline_path, candidate_path, calibration_path, tools_dir, output, *,
              python=sys.executable, baseline_producer_review=None, candidate_producer_review=None):
    require(re.fullmatch(r'[a-zA-Z0-9][a-zA-Z0-9_-]{0,60}', series), 'unsafe series label')
    require(Path(workload.__file__).resolve() == Path(__file__).with_name('perf_headless_workload.py').resolve(),
            'shadowed workload adapter from another checkout')
    tools_dir = Path(tools_dir).resolve()
    baseline, candidate = load_build(baseline_path), load_build(candidate_path)
    for path, build in ((baseline_path, baseline), (candidate_path, candidate)):
        require(Path(path).resolve() == tools_dir.parent/'.perf'/'headless-builds'/build['label']/'build.json',
                'build must be frozen in the measurement host selected by tools-dir')
    require(build_common(baseline) == build_common(candidate), 'build configuration differs')
    producer_paths = {'A': baseline_producer_review, 'B': candidate_producer_review}
    require(all(producer_paths.values()), 'predeclared source-reviewed producer capabilities required for both builds')
    for side, build in (('A', baseline), ('B', candidate)):
        source_files = read(Path(build['executable']).with_name('source.json'))['files']
        workload.review_capability(producer_paths[side], build, source_files)
    common_source = {}
    for side, build in (('A', baseline), ('B', candidate)):
        source_files = read(Path(build['executable']).with_name('source.json'))['files']
        common_source[side] = {name: source_files[name] for name in workload.COMMON_HARNESS_FILES}
    require(common_source['A'] == common_source['B'], 'baseline/candidate common CPU harness source bytes differ')
    require((baseline['source_commit'], baseline['source_content_sha256']) !=
            (candidate['source_commit'], candidate['source_content_sha256']),
            'cross-source series requires distinct source identities')
    require(baseline['label'] != candidate['label'], 'frozen build labels must differ')
    calibration = read(calibration_path)
    require(calibration['schema'] == 'windows-clock-policy-evidence-v1'
            and calibration['policy_id'] == POLICY, 'calibrated evidence required')
    ids = module_identity(tools_dir)
    require(ids == calibration['tools'], 'calibration measurement tools differ')
    policy = {'id': POLICY, 'definition_sha256': calibration['definition_sha256'],
              'calibration_sha256': digest(calibration_path),
              'hardware_sha256': signature(calibration['hardware'])}
    root = tools_dir.parent
    runs_root = root/'.perf'/'headless-runs'
    entries = []
    for pair, order in enumerate(RULES['orders'], 1):
        for scene in SCENES:
            for side in order:
                label = f'{series}-p{pair}-{scene}-{side.lower()}'
                require(not (runs_root/label).exists(), 'run already exists; fresh series required: '+label)
                build = baseline if side == 'A' else candidate
                command = [str(Path(python).resolve()), '-B', str(tools_dir/'perf_headless.py'),
                           'run', build['label'], label, '--mode', 'on', '--scene', scene,
                           '--warmup', '600', '--samples', '3000', '--clock-policy', POLICY,
                           '--clock-calibration', str(Path(calibration_path).resolve()),
                           '--official', '--archive-trace-csv']
                entries.append({'ordinal': len(entries)+1, 'pair': pair, 'scene': scene, 'side': side,
                    'extension_only': pair > 3, 'run_label': label, 'run_dir': str(runs_root/label),
                    'command': command,
                    'audit_command': [str(Path(python).resolve()), '-B', str(Path(__file__).resolve()),
                                      'audit-run', str(Path(output).resolve()), '--ordinal', str(len(entries)+1)]})
    plan = {'schema': 'headless-cross-source-series-v2', 'series': series,
        'nonce': str(uuid.uuid4()), 'created_utc': datetime.now(timezone.utc).isoformat(),
        'scope': SCOPE, 'production_16ms_acceptance': False, 'rules': RULES,
        'comparator_sha256': digest(__file__),
        'workload_adapter_sha256': digest(Path(__file__).with_name('perf_headless_workload.py')),
        'producer_reviews': {side: reference(path) for side, path in producer_paths.items()},
        'algorithm_selection': workload.ALGORITHM_SELECTION,
        'common_harness_source_sha256': common_source['A'],
        'python': reference(python),
        'tools_dir': str(tools_dir), 'measurement_tools': ids,
        'calibration': reference(calibration_path), 'clock_policy': policy,
        'builds': {'A': reference(baseline_path), 'B': reference(candidate_path)},
        'build_records': {'A': baseline, 'B': candidate}, 'entries': entries,
        'workload_witness_schema': 'headless-actual-workload-proof-v1',
        'execution': 'commands only, sequential; first 24 entries, last16 only on needs_five_pairs',
        'open_evidence': ['Native API invocations do not prove indirect GPU draw/count-buffer argument work.',
                          'Current actual-workload producer explicitly has no all-upload total; coverage review pending.']}
    write_new(output, plan)
    return plan


def validate_plan(plan):
    require(plan.get('schema') == 'headless-cross-source-series-v2'
            and plan.get('rules') == RULES, 'unknown/modified predeclared series policy')
    require(plan.get('comparator_sha256') == digest(__file__), 'comparator differs from declaration')
    require(Path(workload.__file__).resolve() == Path(__file__).with_name('perf_headless_workload.py').resolve()
            and plan['workload_adapter_sha256'] == digest(workload.__file__), 'workload adapter differs from declaration')
    checked_ref(plan['python'])
    checked_ref(plan['calibration'])
    require(module_identity(plan['tools_dir']) == plan['measurement_tools'], 'measurement tools changed')
    for side in 'AB':
        require(load_build(checked_ref(plan['builds'][side])) == plan['build_records'][side],
                'build identity changed after declaration')
        source_files = read(Path(plan['build_records'][side]['executable']).with_name('source.json'))['files']
        workload.review_capability(checked_ref(plan['producer_reviews'][side]), plan['build_records'][side], source_files)
        require(plan['algorithm_selection'] == workload.ALGORITHM_SELECTION
                and {name: source_files[name] for name in workload.COMMON_HARNESS_FILES}
                == plan['common_harness_source_sha256'], 'common source-default contract differs from declaration')
    expected = [(pair, scene, side) for pair, order in enumerate(RULES['orders'], 1)
                for scene in SCENES for side in order]
    require([(e['pair'], e['scene'], e['side']) for e in plan['entries']] == expected,
            'missing/duplicate/reordered series entry')
    paths = [str(Path(e['run_dir']).resolve()) for e in plan['entries']]
    require(len(set(paths)) == 40, 'run directories reused')
    root = Path(plan['tools_dir']).resolve().parent
    for index, entry in enumerate(plan['entries'], 1):
        label = f"{plan['series']}-p{entry['pair']}-{entry['scene']}-{entry['side'].lower()}"
        require(entry['ordinal'] == index and entry['run_label'] == label
                and Path(entry['run_dir']).resolve() == root/'.perf'/'headless-runs'/label
                and entry['extension_only'] is (entry['pair'] > 3), 'series label/path/ordinal mismatch')
        require(not any(str(arg).startswith('--disable-') for arg in entry['command']),
                'compiled-source-default contract rejects all runtime disable controls')


def account_module(plan):
    """Only load exactly the frozen local implementations, no cached other checkout."""
    tools_dir = Path(plan['tools_dir']).resolve()
    for name in (*TOOLS, 'perf_build_target.py'):
        cached = sys.modules.get(name[:-3])
        if cached is not None:
            require(Path(cached.__file__).resolve() == tools_dir/name, 'shadowed accounting module: '+name)
    sys.dont_write_bytecode = True
    sys.path.insert(0, str(tools_dir))
    return importlib.import_module('perf_headless_account')


def workload_proof(path, run_path, session_path, session, cpu, *, review_reference):
    proof = read(path)
    require(proof.get('schema') == 'headless-actual-workload-proof-v1', 'unknown or legacy workload witness')
    run = read(run_path)
    require(proof['run_sha256'] == digest(run_path) and proof['session_sha256'] == digest(session_path)
            and proof['executable_sha256'] == run['build']['executable_sha256'], 'workload witness capture identity differs')
    require(proof['producer_review'] == review_reference, 'witness producer review differs from predeclaration')
    source_files = read(Path(run['build']['executable']).with_name('source.json'))['files']
    cap = workload.review_capability(checked_ref(review_reference), run['build'], source_files)
    require(proof['raw_session_field'] == 'actual_workload' and proof['raw'] == session.get('actual_workload'),
            'witness must preserve exact raw actual_workload object')
    result = workload.validate_actual(session, cpu, cap)
    result['cost_difference_explanation'] = proof.get('cost_difference_explanation')
    return result


def load_run(plan, entry, replay=None, *, require_seal=True):
    folder = Path(entry['run_dir'])
    run_path, session_path, cpu_path = (folder/name for name in ('run.json', 'session.json', 'cpu.json'))
    run, session, cpu = read(run_path), read(session_path), read(cpu_path)
    require(run['schema'] == 'headless-capture-v1' and run['valid'] is True
            and run['clean_release_candidate'] is True, 'invalid/non-release capture')
    require(run['build'] == plan['build_records'][entry['side']], 'run changed frozen source/build identity')
    require(run['executable_sha256'] == run['build']['executable_sha256']
            and Path(run['command'][0]).resolve() == Path(run['build']['executable']).resolve(),
            'executed binary differs from declared frozen build')
    require(run['build']['profile'] == 'release' and run['build']['dirty'] is False, 'debug/dirty capture')
    require(run['diagnostic'] is False and not session['diagnostic_bill'] and not cpu['diagnostic_bill'],
            'diagnostic capture cannot establish clean performance')
    require(session.get('measurement_class') == 'clean_cpu'
            and session.get('clean_cpu_acceptance_eligible') is True
            and session.get('gpu_diagnostic') is None
            and session.get('harness') == 'common-cpu-workload-v1'
            and session.get('management_diagnostics_available') is False
            and session.get('algorithm_selection') == plan['algorithm_selection'] == workload.ALGORITHM_SELECTION,
            'unversioned/diagnostic/unknown common CPU algorithm selection rejected')
    require(run['mode'] == session['mode'] == cpu['mode'] == 'on', 'feature overhead/off capture rejected')
    require(run['cpu_probe_feature'] is session['cpu_probe_feature'] is True, 'cpu-probe feature required')
    require(run['etw'] is True and run['trace_started_by_this_run'] is True
            and run['exit_code'] == run['etw_stop_exit_code'] == 0, 'capture/ETW failed')
    require(Path(run['session_path']).resolve() == session_path.resolve()
            and run['session_sha256'] == digest(session_path), 'session identity mismatch')
    intent = read(checked_ref(run['clock_policy_intent']))
    declared_filetime = round(timestamp(plan['created_utc']).timestamp()*1e7)+116444736000000000
    require(intent['created_filetime'] > declared_filetime, 'run predates series declaration')
    require(run['scenario'] == session['scenario'] == cpu['scenario'] == entry['scene'], 'scenario mismatch')
    require(run['warmup'] == session['warmup'] == 600
            and run['samples'] == session['samples'] == cpu['samples'] == 3000,
            'formal series requires 600 warmup/3000 valid observations')
    require(run['clock_policy'] == cpu['clock_policy'] == plan['clock_policy'], 'mixed clock policy/evidence')
    require(run['measurement_tools'] == plan['measurement_tools']
            and run['measurement_tools_sha256'] == signature(plan['measurement_tools']), 'measurement tools differ')
    require(run['hardware_sha256'] == run['hardware']['sha256']
            == signature({k: v for k, v in run['hardware'].items() if k != 'sha256'}),
            'hardware identity mismatch')
    require(digest(folder/'vulkan-summary.txt') == run['hardware']['vulkan_summary_sha256'],
            'actual Vulkan enumeration/driver changed')
    require(run['settings']['dimensions'] == '1280x720' and not run['settings']['diagnostic'], 'scene config mismatch')
    expected_settings = {'clock_policy': plan['clock_policy'], 'scenario': entry['scene'],
        'dimensions': '1280x720', 'warmup': 600, 'samples': 3000,
        'features': run['build']['features'], 'allocator': 'mimalloc',
        'driver_environment': run['hardware']['vulkan_environment'],
        'worker_configuration': 'existing default; TG overrides cleared', 'diagnostic': False,
        'process_priority': 'NORMAL_PRIORITY_CLASS', 'thread_priority': 'existing fixture default',
        'management_ablation': {}, 'management_modes_expected': None}
    require(run['settings'] == expected_settings, 'unknown/changed clean measurement configuration')
    require(run['settings_sha256'] == signature(run['settings']), 'settings SHA mismatch')
    require(run['management_ablation'] == {} and run['attribution_scope'] == 'default_algorithms', 'ablation rejected')
    require(run['management_modes'] is None and cpu['management_modes'] is None
            and session.get('management_modes') is None and run['management_modes_expected'] is None,
            'source-default contract cannot pretend equal runtime four-boolean modes')
    require(run['management_modes_sha256'] == signature(run['management_modes']), 'management modes SHA mismatch')
    require(cpu['valid'] is True and cpu['issues'] == [] and session['valid'] is True
            and session['clock_valid'] is True, 'invalid CPU/session chain')
    require(cpu['production_16ms_acceptance'] is False and run['production_16ms_acceptance'] is False,
            'headless run incorrectly claims production acceptance')
    require(session['input_policy'] == cpu['input_policy'] == run['input_policy'] == INPUT_POLICY,
            'unsupported semantic input policy')
    identity = {'policy': INPUT_POLICY, 'before': session['input_before'], 'after': session['input_after']}
    require('transform_trajectory' in identity['after'], 'full transform trajectory missing')
    require(signature(identity) == run['input_sha256'] == cpu['input_sha256'], 'raw input trajectory SHA differs')
    require(session['workload'] == run['workload']
            and signature(session['workload']) == run['workload_sha256'] == cpu['workload_sha256'],
            'raw scripted workload SHA differs')
    policy = session['workload']['policy']
    require(policy['extent'] == [1280, 720] and policy['world_scale'] == 1.0
            and policy['fixed_delta'] == '1/60', 'scene resolution/scale/step changed')
    require(len(cpu['frames']) == 3000 and len(session['frames']) == 3001, 'missing valid frame rows')
    trace = Path(run['trace_csv_path'])
    require(trace.resolve().parent == folder.resolve(), 'trace outside capture directory')
    require(cpu == run['account'], 'CPU report differs from embedded capture account')
    require(cpu['schema'] == 'headless-etw-account-v2', 'calibrated account v2 required')
    quality, envelope = cpu['clock_quality'], cpu['envelope']
    require(quality['coarse_checks']['owner_getthreadtimes_within_2pct'] is True
            and envelope['owner_os_relative_error'] <= .02
            and envelope['official_process_relative_error'] <= .02
            and quality['lifecycle_complete'] is True
            and quality['raw_counters_corrected'] is False
            and quality['shared_event_stream'] is True
            and quality['independent_clock'] is False, 'CPU quality gates failed')
    require(cpu['accounting_tools'] == {name: plan['measurement_tools'][name] for name in
        ('perf_headless_account.py', 'perf_etw.py', 'perf_trace_archive.py', 'perf_clock_policy.py')},
        'account implementation identity differs')
    if replay is not None:
        require(digest(folder/'schedule.etl') == run['trace_etl_sha256'], 'raw ETL identity changed')
        # Parser checks zero loss/unmatched/QPC/lifecycle/owner2%/official PID2%.
        fresh = replay(session, trace, capture_record=run, official_totals=run['official_cswitch'])
        require(fresh == cpu, 'CPU report differs from frozen raw-trace replay')
    elif require_seal:
        verify_seal(plan, entry)
    stats = {}
    for metric in METRICS:
        values = [finite(frame[metric], metric) for frame in cpu['frames']]
        ordered = sorted(values)
        stats[metric] = {'n': 3000, 'mean_ms': statistics.mean(values),
                         'p50_ms': ordered[math.ceil(3000*.50)-1],
                         'p95_ms': ordered[math.ceil(3000*.95)-1],
                         'p99_ms': ordered[math.ceil(3000*.99)-1]}
        for key in ('n', 'p50_ms', 'p95_ms', 'p99_ms'):
            require(stats[metric][key] == cpu['statistics'][metric][key], 'summary/frame percentile differs')
    proof = workload_proof(folder/'workload-proof.json', run_path, session_path, session, cpu,
                           review_reference=plan['producer_reviews'][entry['side']])
    return {'entry': entry, 'run': run, 'session': session, 'cpu': cpu, 'input': identity, 'proof': proof,
            'statistics': stats, 'artifacts': {name: reference(folder/name) for name in
                ('run.json', 'session.json', 'cpu.json', 'workload-proof.json')}}


def pair_results(runs):
    baseline, candidate = runs['A'], runs['B']
    for field in ('hardware', 'hardware_sha256', 'settings', 'measurement_tools', 'clock_policy',
                  'workload', 'workload_sha256', 'input_sha256', 'management_modes'):
        require(baseline['run'][field] == candidate['run'][field], 'A/B mismatch: '+field)
    require(baseline['input'] == candidate['input'], 'A/B full raw input/trajectory differs')
    workload_comparison = workload.compare_workload(baseline['proof'], candidate['proof'],
        candidate['proof'].get('cost_difference_explanation'))
    ratios = {}
    failures, cost_flags = [], []
    for metric in METRICS:
        ratios[metric] = {}
        for q in ('mean_ms', 'p50_ms', 'p95_ms', 'p99_ms'):
            a, b = baseline['statistics'][metric][q], candidate['statistics'][metric][q]
            ratio = (b/a-1) if a else (0.0 if b == 0 else None)
            ratios[metric][q] = ratio
            if b > a*1.05:
                if metric == METRICS[0] and q in ('p95_ms', 'p99_ms'):
                    failures.append(f'{metric}.{q} regression exceeds5%')
                elif metric != METRICS[0]:
                    cost_flags.append(f'{metric}.{q} increase exceeds5%; attribution review required')
    p95 = candidate['statistics']['render_phase_owner_cpu_ms']['p95_ms']
    if p95 > 16:
        failures.append('candidate render phase owner p95 exceeds16ms')
    return {'ratios': ratios, 'candidate_render_p95_ms': p95, 'failures': failures,
            'cost_shift_flags': cost_flags, 'workload_comparison': workload_comparison,
            'baseline': baseline['statistics'], 'candidate': candidate['statistics']}


def decide(results):
    """No pooled percentile and no averaging away a failed individual run."""
    require(set(results) == set(SCENES), 'four scenes required')
    count = len(results['static'])
    require(count in (3, 5) and all(len(v) == count for v in results.values()), 'three or five complete pairs required')
    failures = [f'{scene}:pair{i+1}:{error}' for scene, rows in results.items()
                for i, row in enumerate(rows) for error in row['failures']]
    if failures:
        return {'status': 'rejected', 'headless_cpu_gate_passed': False, 'failures': failures}
    near = []
    for scene, rows in results.items():
        p95s = [row['candidate_render_p95_ms'] for row in rows]
        if any(v >= 15.2 for v in p95s) or max(p95s)-min(p95s) >= .8:
            near.append(scene+':render-p95-near-or-variable')
        for metric in RULES['regression_metrics']:
            for q in ('p95_ms', 'p99_ms'):
                deltas = [row['ratios'][metric][q] for row in rows]
                require(all(v is not None and math.isfinite(v) for v in deltas), 'undefined regression ratio')
                if any(v >= .03 for v in deltas) or max(deltas)-min(deltas) >= .03:
                    near.append(scene+':'+metric+':'+q)
    if count == 3 and near:
        return {'status': 'needs_five_pairs', 'headless_cpu_gate_passed': False,
                'extension_reasons': near, 'required_extra_orders': ['BA', 'AB']}
    upper = {}
    if count == 5:
        for scene, rows in results.items():
            for metric in RULES['regression_metrics']:
                for q in ('p95_ms', 'p99_ms'):
                    deltas = [row['ratios'][metric][q] for row in rows]
                    bound = statistics.mean(deltas)+RULES['five_pair_t']*statistics.stdev(deltas)/math.sqrt(5)
                    key = scene+':'+metric+':'+q
                    upper[key] = bound
                    if bound > .05:
                        failures.append(key+':one-sided95% paired-run upper bound exceeds5%')
    return {'status': 'inconclusive' if failures else 'headless_cpu_gate_passed',
            'headless_cpu_gate_passed': not failures, 'failures': failures,
            'five_pair_upper_bounds': upper, 'near_threshold_signals': near}


def seal_artifacts(folder):
    run = read(Path(folder)/'run.json')
    paths = {
        'run': Path(folder)/'run.json', 'session': Path(folder)/'session.json',
        'cpu': Path(folder)/'cpu.json', 'workload': Path(folder)/'workload-proof.json',
        'etl': Path(folder)/'schedule.etl', 'csv': Path(run['trace_csv_path']),
        'official': Path(run['official_cswitch']['output']['path']),
        'intent': Path(run['clock_policy_intent']['path']),
        'vulkan': Path(folder)/'vulkan-summary.txt'}
    require(all(path.resolve().parent == Path(folder).resolve() for path in paths.values()),
            'audit artifact escapes capture directory')
    return {name: reference(path) for name, path in paths.items()}


def verify_seal(plan, entry):
    seal = read(Path(entry['run_dir'])/'series-audit.json')
    require(seal['schema'] == 'headless-series-audit-v2' and seal['raw_replay_valid'] is True,
            'strict raw-trace audit required before cached evaluation')
    require(seal['plan_sha256'] == signature(plan) and seal['ordinal'] == entry['ordinal']
            and seal['comparator_sha256'] == digest(__file__), 'audit seal belongs to another series/tool/run')
    require(seal['artifacts'] == seal_artifacts(entry['run_dir']), 'raw/account/workload bytes changed after audit')


def audit_run(plan_path, ordinal):
    plan = read(plan_path)
    validate_plan(plan)
    require(type(ordinal) is int and 1 <= ordinal <= 40, 'invalid run ordinal')
    entry = plan['entries'][ordinal-1]
    module = account_module(plan)
    load_run(plan, entry, module.account)
    seal = {'schema': 'headless-series-audit-v2', 'plan_sha256': signature(plan),
        'ordinal': ordinal, 'raw_replay_valid': True, 'comparator_sha256': digest(__file__),
        'audited_utc': datetime.now(timezone.utc).isoformat(),
        'artifacts': seal_artifacts(entry['run_dir']),
        'integrity_scope': 'SHA binds reviewed bytes; not a cryptographic attestation of producer honesty'}
    write_new(Path(entry['run_dir'])/'series-audit.json', seal)
    return seal


def apply_workload_pending(outcome, comparisons):
    pending = sorted({reason for row in comparisons for reason in row['coverage_pending']})
    if any(row['payload_review_required'] for row in comparisons):
        pending.append('H2D/D2D per-source per-receipt payload changed; manual review required, free text cannot pass')
    if any(row['cost_difference_review_required'] for row in comparisons):
        pending.append('CPU task/dispatch/recording cost changed; source and cost-shift review required')
    outcome = dict(outcome)
    outcome['render_numeric_gate_passed'] = outcome['headless_cpu_gate_passed']
    outcome['workload_review_pending'] = pending
    if outcome['headless_cpu_gate_passed'] and pending:
        outcome['status'] = 'pending_workload_review'
        outcome['headless_cpu_gate_passed'] = False
    return outcome


def evaluate(plan_path, *, replay=False):
    plan = read(plan_path)
    validate_plan(plan)
    exists = [Path(e['run_dir']).exists() for e in plan['entries']]
    count = 5 if any(exists[24:]) else 3
    require(all(exists[:count*8]), 'incomplete series; no dropping/selecting individual rounds')
    module = account_module(plan) if replay else None
    loaded = []
    results = {scene: [] for scene in SCENES}
    prior_end_qpc, prior_start = 0, timestamp(plan['created_utc'])
    hardware_identity = None
    settings_by_scene = {}
    for pair in range(1, count+1):
        for scene in SCENES:
            sides = {}
            for entry in [e for e in plan['entries'] if e['pair'] == pair and e['scene'] == scene]:
                row = load_run(plan, entry, module.account if module else None)
                start = timestamp(row['run']['start_utc'])
                begin, end = row['session']['envelope']['begin'][0], row['session']['envelope']['end'][0]
                require(begin > prior_end_qpc and start >= prior_start.replace(microsecond=0),
                        'actual A/B temporal order overlaps/reorders previous run')
                prior_end_qpc, prior_start = end, start
                if hardware_identity is None:
                    hardware_identity = row['run']['hardware']
                require(row['run']['hardware'] == hardware_identity, 'series hardware drift across scenes')
                settings = {key: row['run'][key] for key in ('hardware', 'settings', 'input_sha256', 'workload_sha256')}
                if scene in settings_by_scene:
                    require(settings == settings_by_scene[scene], 'series workload/settings drift between rounds')
                else:
                    settings_by_scene[scene] = settings
                sides[entry['side']] = row
                loaded.append(row['artifacts'])
            results[scene].append(pair_results(sides))
    if count == 5:
        first_three = decide({scene: rows[:3] for scene, rows in results.items()})
        require(first_three['status'] == 'needs_five_pairs',
                'undeclared/selective extension: initial three must require extension')
    outcome = decide(results)
    comparisons = [row['workload_comparison'] for rows in results.values() for row in rows]
    outcome = apply_workload_pending(outcome, comparisons)
    return {'schema': 'headless-cross-source-result-v2', 'scope': SCOPE,
            'production_16ms_acceptance': False, 'series': reference(plan_path),
            'comparator_sha256': digest(__file__), 'pairs': count, 'outcome': outcome,
            'validation_mode': 'raw_replay' if replay else 'previous_raw_replay_seals_and_current_artifact_hashes',
            'candidate_accepted': False, 'cost_shift_review_required': True,
            'results': results, 'artifacts': loaded,
            'limitations': ['All non-owner process threads include audio/driver/runtime, not worker-only.',
                '5-pair CI treats whole runs as units, not independent/autocorrelated frame samples.',
                'Pixel correctness, probe overhead, long-run stability and WSI remain separate gates.']}


def main():
    parser = argparse.ArgumentParser(description=__doc__)
    sub = parser.add_subparsers(dest='action', required=True)
    p = sub.add_parser('plan')
    p.add_argument('series'); p.add_argument('--baseline-build', type=Path, required=True)
    p.add_argument('--candidate-build', type=Path, required=True)
    p.add_argument('--calibration', type=Path, required=True)
    p.add_argument('--tools-dir', type=Path, default=Path(__file__).resolve().parent)
    p.add_argument('--output', type=Path, required=True)
    p.add_argument('--baseline-producer-review', type=Path, required=True)
    p.add_argument('--candidate-producer-review', type=Path, required=True)
    p = sub.add_parser('evaluate')
    p.add_argument('series', type=Path); p.add_argument('--output', type=Path, required=True)
    p.add_argument('--replay', action='store_true', help='strict raw replay of every run; default requires audit-run seals')
    p = sub.add_parser('audit-run')
    p.add_argument('series', type=Path); p.add_argument('--ordinal', type=int, required=True)
    args = parser.parse_args()
    try:
        if args.action == 'plan':
            result = make_plan(args.series, args.baseline_build, args.candidate_build,
                               args.calibration, args.tools_dir, args.output,
                               baseline_producer_review=args.baseline_producer_review,
                               candidate_producer_review=args.candidate_producer_review)
        elif args.action == 'evaluate':
            result = evaluate(args.series, replay=args.replay)
            write_new(args.output, result)
        else:
            result = audit_run(args.series, args.ordinal)
        print(json.dumps({'schema': result['schema'], 'measurement_started': False,
                          'output': str(getattr(args, 'output', 'per-run series-audit.json'))}, ensure_ascii=False))
        if args.action == 'evaluate':
            status = result['outcome']['status']
            return {'headless_cpu_gate_passed': 0, 'rejected': 1,
                    'needs_five_pairs': 3, 'inconclusive': 4, 'pending_workload_review': 5}[status]
    except (ValueError, KeyError, OSError, TypeError) as error:
        if args.action == 'evaluate' and not args.output.exists():
            write_new(args.output, {'schema': 'headless-cross-source-result-v2',
                'valid': False, 'headless_cpu_gate_passed': False,
                'production_16ms_acceptance': False, 'error': str(error), 'scope': SCOPE})
        parser.exit(2, str(error)+'\n')


if __name__ == '__main__':
    sys.exit(main())
