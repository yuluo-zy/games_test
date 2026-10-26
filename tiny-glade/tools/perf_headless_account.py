"""Strict headless CPU accounting. ETW running time is distinct from wall time.

This scope includes App and renderer on one owner. It never establishes WSI or
production 16 ms acceptance. The first measured row is an attribution baseline.
"""
import argparse
import hashlib
import json
import math
import statistics
from pathlib import Path
from perf_etw import Timeline, distribution, read_trace

PHASES = ('app_update', 'pacing', 'draw_lists_vao', 'declare', 'compile',
          'pending_admission', 'record_submit', 'maintenance')
SCOPE = ('Headless shipped session; App and renderer share one owner. ETW CPU '
         'is exclusive running time, worker/non-owner CPU is additional work. '
         'No WSI, production rendering-thread or final 16 ms acceptance.')


def signature(value):
    return hashlib.sha256(json.dumps(value, sort_keys=True, separators=(',', ':'),
                                     ensure_ascii=False).encode('utf-8')).hexdigest()



def accounting_tools():
    root = Path(__file__).resolve().parent
    result = {}
    for name in ('perf_headless_account.py', 'perf_etw.py'):
        with (root/name).open('rb') as source:
            result[name] = hashlib.file_digest(source, 'sha256').hexdigest()
    return result


def require(condition, message):
    if not condition:
        raise ValueError(message)


def integer(value, name, minimum=0):
    require(isinstance(value, int) and not isinstance(value, bool) and value >= minimum,
            'invalid integer: ' + name)
    return value



STRICT_INPUT_POLICY = 'strict_static_input_unchanged'
ANIMATED_INPUT_POLICY = 'no_edit_fixed_step_with_autonomous_animation; paired_before_after_trajectory_must_match'
SCRIPTED_INPUT_POLICY = 'fixed_step_scripted_scene; paired_before_after_trajectory_must_match'
SCENES = ('static', 'many_draws', 'local_edit', 'continuous_edit')


def input_identity(report):
    policy = report.get('input_policy', STRICT_INPUT_POLICY)
    require(policy in (STRICT_INPUT_POLICY, ANIMATED_INPUT_POLICY, SCRIPTED_INPUT_POLICY), 'unknown semantic input policy')
    require(report.get('scenario', 'static') in SCENES, 'unknown scenario')
    if policy == SCRIPTED_INPUT_POLICY:
        require(report.get('scenario') in SCENES and isinstance(report.get('workload'), dict),
                'scripted scene requires scenario and raw workload summary')
    before, after = report.get('input_before'), report.get('input_after')
    require(before is not None and after is not None, 'semantic input absent')
    unchanged = before == after
    if policy == STRICT_INPUT_POLICY:
        require(unchanged, 'semantic input changed under strict-static policy')
    else:
        require(isinstance(report.get('input_unchanged'), bool), 'animated policy needs input_unchanged')
    if 'input_unchanged' in report:
        require(report['input_unchanged'] is unchanged, 'input_unchanged contradicts raw manifests')
    # Both raw endpoints participate, including transforms and autonomous actors.
    return {'policy': policy, 'before': before, 'after': after}


def validate(report):
    require(report.get('schema') == 'headless-cpu-closure-v1', 'unknown session schema')
    require(report.get('valid') is True, 'fixture marked session invalid')
    require(report.get('mode') in ('on', 'off'), 'invalid probe mode')
    input_identity(report)
    feature = report.get('cpu_probe_feature', True)
    require(isinstance(feature, bool), 'cpu_probe_feature must be boolean')
    require(feature or report['mode'] == 'off', 'feature-off binary cannot enable probe session')
    require(feature or not report.get('diagnostic_bill'), 'feature-off binary cannot enable bill')
    integer(report.get('process_id'), 'process_id', 1)
    integer(report.get('thread_id'), 'thread_id', 1)
    integer(report.get('qpc_frequency'), 'qpc_frequency', 1)
    samples = integer(report.get('samples'), 'samples', 1)
    integer(report.get('warmup'), 'warmup', 1)
    envelope = report.get('envelope', {})
    require(envelope.get('frames') == samples + 1, 'envelope frame count mismatch')
    begin, end = envelope.get('begin'), envelope.get('end')
    require(isinstance(begin, list) and isinstance(end, list) and len(begin) == len(end) == 3,
            'missing envelope clocks')
    for clock in begin + end:
        integer(clock, 'envelope clock')
    require(end[0] > begin[0] and end[1] >= begin[1] and end[2] >= begin[2],
            'envelope clock regression')
    require(end[2] - begin[2] >= end[1] - begin[1], 'process CPU smaller than owner CPU')
    rows = report.get('frames')
    require(isinstance(rows, list), 'frame records absent')
    if report['mode'] == 'off':
        require(not rows and report.get('session') is None, 'off mode contains per-frame probes')
        return
    require(report.get('phase_names') == list(PHASES), 'unknown phase layout')
    require(len(rows) == samples + 1, 'missing/extra frame rows')
    session = report.get('session') or {}
    require(session.get('schema') == 'render-cpu-clean-v2' and session.get('valid') is True
            and session.get('complete') is True, 'raw CPU session invalid/incomplete')
    require(not session.get('truncated') and session.get('dropped') == 0, 'probe overflow')
    require(session.get('requested_samples') == samples and session.get('recorded') == samples
            and session.get('clean_samples') == samples, 'sample count mismatch')
    for key in ('process_id', 'thread_id', 'qpc_frequency'):
        require(session.get(key) == report[key], 'raw session identity mismatch: ' + key)
    points = session.get('completion_points', [])
    require(len(points) == samples + 1, 'completion-point count mismatch')
    raw_samples = session.get('samples', [])
    require(len(raw_samples) == samples and all(item.get('kind') == 'clean' for item in raw_samples),
            'non-clean raw CPU samples')
    previous = None
    for index, (row, point) in enumerate(zip(rows, points)):
        qpc = row.get('qpc')
        require(isinstance(qpc, list) and len(qpc) == 9, 'phase endpoint count mismatch')
        for clock in qpc:
            integer(clock, 'phase QPC')
        require(all(a <= b for a, b in zip(qpc, qpc[1:])), 'phase clock regression')
        require(begin[0] <= qpc[0] <= qpc[-1] <= point.get('qpc', -1) <= end[0],
                'phase/completion outside envelope')
        require(point.get('observed') is True, 'unobserved completion')
        integer(point.get('cumulative_cpu_ns'), 'raw owner clock')
        for key in ('world_tick', 'receipt', 'attempt'):
            integer(row.get(key), key)
            require(row[key] == point.get(key), 'frame/completion attribution mismatch: ' + key)
        if previous:
            old_row, old_point = previous
            require(old_point['qpc'] <= qpc[0] and old_point['qpc'] < point['qpc'],
                    'overlapping frames or duplicate completion QPC')
            require(point.get('app_epoch') == old_point.get('app_epoch'), 'cross-App frame')
            require(point['cumulative_cpu_ns'] >= old_point['cumulative_cpu_ns'], 'raw CPU regression')
            require(raw_samples[index-1].get('cpu_ns') == point['cumulative_cpu_ns']-old_point['cumulative_cpu_ns'],
                    'raw CPU interval does not match cumulative readings')
            for key in ('world_tick', 'receipt', 'attempt'):
                require(row[key] > old_row[key], 'non-increasing frame identity: ' + key)
            sample = raw_samples[index - 1]
            for key, raw_key in (('world_tick', 'frame'), ('receipt', 'receipt'), ('attempt', 'attempt')):
                require(row[key] == sample.get(raw_key), 'raw sample attribution mismatch: ' + key)
        previous = (row, point)


def relative_error(actual, reference):
    return abs(actual - reference) / max(actual, reference, 1)


def account(report, trace_path, official_owner_ns=None):
    validate(report)
    frequency = report['qpc_frequency']
    timelines, unmatched, trace_start, trace_end = read_trace(trace_path, report['process_id'], frequency)
    start, stop = report['envelope']['begin'][0], report['envelope']['end'][0]
    require(trace_start < start < stop < trace_end, 'ETW does not strictly enclose envelope')
    require(not any(start <= time <= stop for time, _ in unmatched),
            'unmatched target scheduling event inside envelope')
    require(report['thread_id'] in timelines, 'owner missing from ETW trace')
    owner = timelines[report['thread_id']]
    scale_ns = 1_000_000_000 / frequency
    scale_ms = 1000 / frequency
    owner_ticks = owner.between(start, stop)
    process_ticks = sum(t.between(start, stop) for t in timelines.values())
    os_owner = report['envelope']['end'][1] - report['envelope']['begin'][1]
    os_process = report['envelope']['end'][2] - report['envelope']['begin'][2]
    owner_error = relative_error(owner_ticks * scale_ns, os_owner)
    process_error = relative_error(process_ticks * scale_ns, os_process)
    issues = []
    if owner_error > .02:
        issues.append('ETW owner / GetThreadTimes cumulative discrepancy exceeds 2%')
    if process_error > .02:
        issues.append('ETW process / GetProcessTimes cumulative discrepancy exceeds 2%')
    official_error = None
    if official_owner_ns is not None:
        official_error = relative_error(owner_ticks * scale_ns, official_owner_ns)
        if official_error > .02:
            issues.append('independent Microsoft CSwitch discrepancy exceeds 2%')
    count = report['envelope']['frames']
    totals = {'frames': count, 'wall_ms': (stop-start)*scale_ms,
              'owner_cpu_ms': owner_ticks*scale_ms, 'process_cpu_ms': process_ticks*scale_ms,
              'non_owner_cpu_ms': (process_ticks-owner_ticks)*scale_ms,
              'owner_mean_cpu_ms_per_executed_frame': owner_ticks*scale_ms/count,
              'process_mean_cpu_ms_per_executed_frame': process_ticks*scale_ms/count,
              'owner_os_cpu_ns': os_owner, 'process_os_cpu_ns': os_process,
              'owner_os_relative_error': owner_error, 'process_os_relative_error': process_error,
              'official_owner_cpu_ns': official_owner_ns, 'official_owner_relative_error': official_error}
    result = {'schema': 'headless-etw-account-v1', 'valid': not issues, 'issues': issues,
              'scope': SCOPE, 'mode': report['mode'], 'input_sha256': signature(input_identity(report)),
              'diagnostic_bill': report.get('diagnostic_bill', False), 'envelope': totals,
              'scenario': report.get('scenario', 'static'),
              'workload_sha256': signature(report.get('workload')),
              'input_policy': input_identity(report)['policy'],
              'input_unchanged': report['input_before'] == report['input_after'],
              'strict_static_scene': input_identity(report)['policy'] == STRICT_INPUT_POLICY,
              'non_owner_note': 'All other target-process threads, including workers, driver and helpers; not a pool-only counter.',
              'source': 'CSwitch absolute QPC, unioned per-CPU DPC/ISR subtraction',
              'accounting_tools': accounting_tools(),
              'production_16ms_acceptance': False}
    if report['mode'] == 'off':
        return result
    frames = []
    rows, points = report['frames'], report['session']['completion_points']
    for index in range(1, len(rows)):
        row, previous, current = rows[index], points[index-1], points[index]
        qpc = row['qpc']
        phases = {}
        for phase, left, right in zip(PHASES, qpc, qpc[1:]):
            ticks = owner.between(left, right) if right > left else 0
            phases[phase] = {'cpu_ms': ticks*scale_ms, 'wall_ms': (right-left)*scale_ms,
                             'not_running_ms': ((right-left)-ticks)*scale_ms}
        interval_start, interval_end = previous['qpc'], current['qpc']
        host = owner.between(interval_start, interval_end)*scale_ms
        process = sum(t.between(interval_start, interval_end) for t in timelines.values())*scale_ms
        render = sum(phases[name]['cpu_ms'] for name in PHASES[2:])
        phase_total = sum(p['cpu_ms'] for p in phases.values())
        frames.append({'world_tick': row['world_tick'], 'receipt': row['receipt'], 'attempt': row['attempt'],
                       'app_epoch': current['app_epoch'], 'phases': phases,
                       'render_phase_owner_cpu_ms': render,
                       'app_cpu_ms': phases['app_update']['cpu_ms'],
                       'pacing_not_running_ms': phases['pacing']['not_running_ms'],
                       'host_completion_owner_cpu_ms': host, 'process_cpu_ms': process,
                       'non_owner_cpu_ms': process-host,
                       'bookkeeping_owner_cpu_ms': host-phase_total,
                       'wall_ms': (interval_end-interval_start)*scale_ms})
    # Row 0 seeds completion/management baselines and is never an observation.
    result['samples'] = len(frames)
    result['statistics'] = {key: distribution([frame[key] for frame in frames]) for key in
                            ('render_phase_owner_cpu_ms', 'app_cpu_ms', 'pacing_not_running_ms',
                             'host_completion_owner_cpu_ms', 'process_cpu_ms', 'non_owner_cpu_ms')}
    result['phase_statistics'] = {name: {metric: distribution([frame['phases'][name][metric] for frame in frames])
                                  for metric in ('cpu_ms', 'wall_ms', 'not_running_ms')} for name in PHASES}
    result['frames'] = frames
    return result


def diagnostic_summary(report):
    validate(report)
    require(report['mode'] == 'on' and report.get('diagnostic_bill') is True,
            'diagnostic summary requires on mode and TG_CPU_BILL')
    names = report.get('management_names', [])
    require(len(names) == 13 and names[-1] == 'cached_reflection_variants', 'unknown management layout')
    rows = report['frames']
    deltas = []
    declarations = []
    for old, new in zip(rows, rows[1:]):
        a, b = old.get('management'), new.get('management')
        require(isinstance(a, list) and isinstance(b, list) and len(a) == len(b) == len(names),
                'management counters absent')
        for item in a + b:
            integer(item, 'management counter')
        require(all(after >= before for before, after in zip(a[:-1], b[:-1])), 'management counter regressed')
        deltas.append({name: b[index]-a[index] for index, name in enumerate(names[:-1])})
        require(isinstance(new.get('declaration'), dict), 'declaration delta absent')
        declarations.append(new['declaration'])
    def aggregate(values):
        first = values[0]
        if isinstance(first, dict):
            require(all(isinstance(v, dict) and v.keys() == first.keys() for v in values), 'declaration layout changed')
            return {key: aggregate([v[key] for v in values]) for key in first}
        if isinstance(first, list):
            require(all(isinstance(v, list) and len(v) == len(first) for v in values), 'declaration layout changed')
            return [aggregate([v[i] for v in values]) for i in range(len(first))]
        for value in values:
            integer(value, 'declaration delta')
        return sum(values)
    totals = {name: sum(delta[name] for delta in deltas) for name in names[:-1]}
    return {'schema': 'headless-management-diagnostic-v1', 'diagnostic_only': True,
            'samples': len(deltas), 'input_sha256': signature(input_identity(report)),
            'scenario': report.get('scenario', 'static'),
              'workload_sha256': signature(report.get('workload')),
              'input_policy': input_identity(report)['policy'],
            'input_unchanged': report['input_before'] == report['input_after'],
            'strict_static_scene': input_identity(report)['policy'] == STRICT_INPUT_POLICY,
            'management_totals': totals, 'management_mean_per_frame': {k: v/len(deltas) for k, v in totals.items()},
            'cached_reflection_variants': {'baseline': rows[0]['management'][-1],
                                         'last': rows[-1]['management'][-1],
                                         'maximum': max(row['management'][-1] for row in rows)},
            'declaration_delta_totals': aggregate(declarations),
            'notes': ['Management 0..11 are cumulative: difference against row 0, excluding warmup.',
                      'cached_reflection_variants is a gauge, never a counter delta.',
                      'DeclareProbe take resets counters: each row is already a delta; do not difference twice.',
                      'stage_ns[5] is an import count, not nanoseconds.',
                      'Record and prepare times sum wall over worker/recording threads; not exclusive CPU.',
                      'collected_uniform_write_bytes covers Collect uniform writes, not all uploads.',
                      'binding_resource_inputs counts repeated inputs, not unique physical owners.']}


# Two-sided 95% Student t, conservative for >30 degrees of freedom.
T95 = {2: 4.303, 3: 3.182, 4: 2.776, 5: 2.571, 6: 2.447, 7: 2.365,
       8: 2.306, 9: 2.262, 10: 2.228, 11: 2.201, 12: 2.179, 13: 2.160,
       14: 2.145, 15: 2.131, 16: 2.120, 17: 2.110, 18: 2.101,
       19: 2.093, 20: 2.086, 21: 2.080, 22: 2.074, 23: 2.069,
       24: 2.064, 25: 2.060, 26: 2.056, 27: 2.052, 28: 2.048, 29: 2.045, 30: 2.043}


def paired_overhead(runs, kind='runtime_session'):
    require(kind in ('runtime_session', 'compiled_feature_session_on', 'compiled_feature_session_off'),
            'unknown overhead kind')
    require(len(runs) >= 6 and len(runs) % 2 == 0, 'at least three complete on/off pairs required')
    issues = []
    expected = runs[0]
    compile_pair = kind != 'runtime_session'
    by_role = {}
    for index, run in enumerate(runs):
        require(run.get('account') is not None, 'run has no ETW accounting')
        if compile_pair:
            require(isinstance(run.get('cpu_probe_feature'), bool), 'compiled feature identity missing')
            role = 'on' if run['cpu_probe_feature'] else 'off'
            expected_mode = 'on' if role == 'on' and kind == 'compiled_feature_session_on' else 'off'
            require(run['mode'] == expected_mode, 'wrong runtime mode for compiled-feature pair')
        else:
            role = run['mode']
            require(run.get('cpu_probe_feature', True) is True, 'runtime pair requires cpu-probe feature')
        expected_role = (('on', 'off') if (index//2) % 2 == 0 else ('off', 'on'))[index % 2]
        require(role == expected_role, 'order must alternate on/off, off/on, on/off')
        if not run.get('valid') or not run['account'].get('valid'):
            issues.append('one or more runs invalid')
        require(not run.get('diagnostic') and not run['account'].get('diagnostic_bill'), 'heavy diagnostics mixed into overhead')
        tools = run['account'].get('accounting_tools')
        require(isinstance(tools, dict) and tools, 'accounting tool identity absent')
        require(tools == expected['account'].get('accounting_tools'), 'accounting tools changed during series')
        fields = ['measurement_tools_sha256', 'hardware_sha256', 'input_sha256', 'scenario', 'workload_sha256', 'warmup', 'samples']
        fields += ['comparison_settings_sha256'] if compile_pair else ['executable_sha256', 'settings_sha256']
        for field in fields:
            require(run.get(field) is not None and run[field] == expected.get(field), 'paired identity mismatch: ' + field)
        if compile_pair:
            build = run.get('build', {})
            reference = expected.get('build', {})
            for field in ('source_commit', 'source_content_sha256', 'cargo_lock_sha256', 'profile',
                          'allocator', 'incremental', 'jobs', 'rustc', 'build_environment_sha256'):
                require(field in build and build[field] == reference.get(field), 'compiled pair build mismatch: ' + field)
            require(build['allocator'] == 'mimalloc', 'allocator changed in compiled pair')
            features = build.get('features')
            require(isinstance(features, list), 'build features absent')
            require(('cpu-probe' in features) == run['cpu_probe_feature'], 'feature identity contradiction')
            base_features = sorted(feature for feature in features if feature != 'cpu-probe')
            expected_features = sorted(feature for feature in reference.get('features', []) if feature != 'cpu-probe')
            require(base_features == expected_features, 'features differ beyond cpu-probe')
        prior = by_role.setdefault(role, run)
        require(prior['executable_sha256'] == run['executable_sha256'], 'role binary changed during series')
    metrics = {}
    for name in ('owner', 'process'):
        field = name + '_mean_cpu_ms_per_executed_frame'
        ratios, on_values, off_values = [], [], []
        for index in range(0, len(runs), 2):
            pair = {}
            for run in runs[index:index+2]:
                role = ('on' if run['cpu_probe_feature'] else 'off') if compile_pair else run['mode']
                pair[role] = run['account']['envelope'][field]
            require(pair['off'] > 0 and pair['on'] > 0, 'zero CPU in paired run')
            ratios.append(pair['on']/pair['off']-1)
            on_values.append(pair['on']); off_values.append(pair['off'])
        mean = statistics.mean(ratios)
        margin = T95.get(min(len(ratios)-1, 30), 4.303)*statistics.stdev(ratios)/math.sqrt(len(ratios))
        low, high = mean-margin, mean+margin
        ratio_of_means = statistics.mean(on_values)/statistics.mean(off_values)-1
        passed = low >= -.01 and high <= .01 and abs(ratio_of_means) <= .01
        metrics[name] = {'paired_relative_change': ratios, 'mean_paired_relative_change': mean,
                         'ratio_of_means_relative_change': ratio_of_means,
                         'confidence_95_relative_change': [low, high], 'within_1pct': passed}
        if not passed:
            issues.append(name + ' overhead/noise is not established within +/-1%')
    kinds = {'runtime_session': 'same_cpu_probe_binary_runtime_session_on_off',
             'compiled_feature_session_on': 'compiled_cpu_probe_active_session_vs_default_feature_off',
             'compiled_feature_session_off': 'compiled_cpu_probe_disabled_session_vs_default_feature_off'}
    uncovered = ['compiled cpu-probe feature overhead vs default production feature-off binary',
                 'disabled per-draw management/probe gates shared by both runtime modes'] if not compile_pair else []
    if kind == 'compiled_feature_session_off':
        uncovered.append('active session recording overhead; both runtime sessions are off')
    return {'schema': 'headless-probe-overhead-v1', 'valid': not issues, 'issues': sorted(set(issues)),
            'pairs': len(runs)//2, 'metrics': metrics, 'overhead_kind': kinds[kind],
            'uncovered_overhead': uncovered,
            'input_policy': expected.get('input_policy', STRICT_INPUT_POLICY),
            'scenario': expected.get('scenario', 'static'),
            'workload_sha256': expected.get('workload_sha256'),
            'input_comparison': 'policy and both raw before/after trajectory endpoints must match in all runs',
            'acceptance_rule': 'Paired 95% t interval and ratio of arithmetic CPU means both inside [-1%, +1%], owner and process; all identities and ETW checks valid.',
            'scope': SCOPE, 'production_16ms_acceptance': False}


def main():
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument('session', type=Path)
    parser.add_argument('trace_csv', type=Path, nargs='?')
    parser.add_argument('--official-owner-ns', type=int)
    parser.add_argument('--diagnostic', action='store_true')
    parser.add_argument('--output', type=Path, required=True)
    args = parser.parse_args()
    report = json.loads(args.session.read_text(encoding='utf-8'))
    result = diagnostic_summary(report) if args.diagnostic else account(report, args.trace_csv, args.official_owner_ns)
    args.output.parent.mkdir(parents=True, exist_ok=True)
    args.output.write_text(json.dumps(result, indent=2, ensure_ascii=False)+'\n', encoding='utf-8')
    if not result.get('valid', result.get('diagnostic_only', False)):
        raise SystemExit(2)


if __name__ == '__main__':
    main()
