"""Offline per-thread OS/ETW diagnostics. Never changes acceptance gates."""
import argparse
import csv
import hashlib
import json
from pathlib import Path
import re
import sys

sys.dont_write_bytecode = True
REPO_TOOLS = Path(__file__).resolve().parents[1]
if not (REPO_TOOLS/'perf_headless.py').is_file():
    raise RuntimeError('Run this helper from a repository tools/thread_clock directory')
sys.path.insert(0, str(REPO_TOOLS))

def ensure_local_modules():
    helpers = ('native_thread_cpu', 'synthetic_sha256', 'thread_cpu_diagnostic', 'thread_cpu_account', 'predeclare')
    tools = ('perf_headless', 'perf_headless_account', 'perf_etw', 'perf_trace_archive', 'perf_windows',
             'perf_build_target', 'perf_clock_policy')
    for name, directory in [(name, Path(__file__).resolve().parent) for name in helpers]+[(name, REPO_TOOLS) for name in tools]:
        module = sys.modules.get(name)
        if module is not None and (not getattr(module, '__file__', None) or
                Path(module.__file__).resolve() != (directory/(name+'.py')).resolve()):
            raise RuntimeError(name+' was imported from a different repository/package')

ensure_local_modules()
import perf_etw as _perf_etw
if Path(_perf_etw.__file__).resolve().parent != REPO_TOOLS:
    raise RuntimeError('perf_etw was imported from a different repository')
from perf_etw import RAW, PID, Timeline, read_trace


def require(condition, message):
    if not condition:
        raise ValueError(message)


def sample_valid(sample, description):
    require(isinstance(sample, dict), 'OS sample missing: '+description)
    qpc = sample.get('qpc')
    require(isinstance(qpc, list) and len(qpc) == 2
            and all(isinstance(x, int) and not isinstance(x, bool) and x > 0 for x in qpc)
            and qpc[0] <= qpc[1], 'invalid OS read bracket: '+description)
    require(sample['cpu_ns'] == (sample['kernel_100ns']+sample['user_100ns'])*100,
            'raw OS units mismatch: '+description)
    require(all(isinstance(sample[name], int) and not isinstance(sample[name], bool)
                and sample[name] >= 0 for name in ('cpu_ns', 'kernel_100ns', 'user_100ns')),
            'invalid OS cumulative integer: '+description)


def lifecycle(path, process_id):
    headers, starts, ends, names = {}, {}, {}, {}
    raw_time = None
    with Path(path).open(encoding='utf-8-sig', errors='replace') as source:
        for line in source:
            raw = RAW.match(line)
            if raw:
                raw_time = int(raw[2], 16)
                continue
            tag = line.lstrip().split(',', 1)[0]
            if tag not in ('T-Start', 'T-End', 'ThreadName'):
                continue
            row = [cell.strip() for cell in next(csv.reader([line], skipinitialspace=True))]
            if row[1] == 'TimeStamp':
                headers[tag] = {name: i for i, name in enumerate(row)}
                continue
            fields = headers.get(tag)
            if not fields or raw_time is None:
                continue
            match = PID.search(row[fields['Process Name ( PID)']])
            if not match or int(match[1]) != process_id:
                continue
            thread = int(row[fields['ThreadID']])
            if tag == 'ThreadName':
                names[thread] = row[fields['ThreadName']]
            else:
                (starts if tag == 'T-Start' else ends).setdefault(thread, []).append(raw_time)
    return {'starts': starts, 'ends': ends, 'names': names}


def ticks_between(timeline, left, right):
    return timeline.between(left, right) if right > left else 0


def compare_window(timelines, begin, end, frequency):
    sample_valid(begin, 'begin')
    sample_valid(end, 'end')
    require(begin['qpc'][1] <= end['qpc'][0], 'overlapping/reversed endpoint reads')
    require(end['cpu_ns'] >= begin['cpu_ns'], 'OS CPU clock regressed')
    require(end['kernel_100ns'] >= begin['kernel_100ns']
            and end['user_100ns'] >= begin['user_100ns'], 'OS kernel/user counter regressed')
    factor = 1000/frequency
    nominal = sum(ticks_between(t, begin['qpc'][0], end['qpc'][0]) for t in timelines)*factor
    lower = sum(ticks_between(t, begin['qpc'][1], end['qpc'][0]) for t in timelines)*factor
    upper = sum(ticks_between(t, begin['qpc'][0], end['qpc'][1]) for t in timelines)*factor
    os_ms = (end['cpu_ns']-begin['cpu_ns'])/1_000_000
    error = abs(nominal-os_ms)/max(nominal, os_ms, 1e-6)
    return {'os_cpu_ms': os_ms,
            'os_kernel_ms': (end['kernel_100ns']-begin['kernel_100ns'])/10000,
            'os_user_ms': (end['user_100ns']-begin['user_100ns'])/10000,
            'etw_nominal_cpu_ms': nominal, 'etw_read_window_lower_ms': lower,
            'etw_read_window_upper_ms': upper, 'read_window_width_ms': upper-lower,
            'os_minus_etw_ms': os_ms-nominal, 'nominal_relative_error': error,
            'within_existing_2pct': error <= .02,
            'begin_api_wall_ms': (begin['qpc'][1]-begin['qpc'][0])*factor,
            'end_api_wall_ms': (end['qpc'][1]-end['qpc'][0])*factor,
            'begin_qpc': begin['qpc'], 'end_qpc': end['qpc'],
            'acceptance_adjusted': False}


def analyze_data(native, timelines, events, trace_start, trace_end):
    frequency = native['qpc_frequency']
    first = native['initial']['process_before_threads']
    last = native['final']['process_after_threads']
    sample_valid(first, 'process initial')
    sample_valid(last, 'process final')
    require(trace_start < first['qpc'][0] and last['qpc'][1] < trace_end,
            'ETW does not enclose the external OS diagnostic window')
    final_before = native['final']['process_before_threads']
    initial_after = native['initial']['process_after_threads']
    sample_valid(initial_after, 'initial sandwich end')
    sample_valid(final_before, 'final sandwich start')
    require(first['qpc'][1] <= initial_after['qpc'][0] <= final_before['qpc'][0]
            and final_before['qpc'][1] <= last['qpc'][0], 'process sandwich bracket order failed')
    require(first['cpu_ns'] <= initial_after['cpu_ns'] <= final_before['cpu_ns'] <= last['cpu_ns'],
            'process sandwich CPU counters regressed')
    rows, missing, unresolved = [], [], []
    ids = [r['thread_id'] for r in native['threads']]
    repeated_ids = sorted({thread for thread in ids if ids.count(thread) > 1}
                          | set(native.get('reused_thread_ids', [])))
    # Reused IDs require lifetime partitioning. Preserve evidence and reject
    # complete-coverage claims instead of merging generations silently.
    for item in native['threads']:
        thread, final = item['thread_id'], item.get('final')
        if final is None:
            unresolved.append({'thread_id': thread, 'reason': 'missing final OS reading'})
            continue
        sample_valid(final, 'thread final')
        require(final['creation_filetime'] == item['creation_filetime'], 'retained thread identity changed')
        require(final.get('signaled_before_read') is True, 'thread final counter is not stable after exit')
        initial = item.get('baseline')
        kind = 'retained_initial_thread'
        if initial is None:
            births = events['starts'].get(thread, [])
            if len(births) != 1 or births[0] < initial_after['qpc'][1]:
                unresolved.append({'thread_id': thread, 'reason': 'new thread lacks unique post-baseline ETW birth'})
                continue
            birth = births[0]
            initial = {'qpc': [birth, birth], 'kernel_100ns': 0, 'user_100ns': 0,
                       'kernel_ns': 0, 'user_ns': 0, 'cpu_ns': 0}
            kind = 'etw_proven_new_thread_zero_lifetime_baseline'
        else:
            sample_valid(initial, 'thread baseline')
            require(first['qpc'][0] <= initial['qpc'][0] <= initial['qpc'][1] <= initial_after['qpc'][1],
                    'initial thread baseline escapes process sandwich')
            require(initial['creation_filetime'] == final['creation_filetime'], 'retained thread object identity changed')
        timeline = timelines.get(thread, Timeline([]))
        row = compare_window((timeline,), initial, final, frequency)
        common_ms = ticks_between(timeline, first['qpc'][0], last['qpc'][0])*1000/frequency
        row['common_process_window_etw_cpu_ms'] = common_ms
        row['common_minus_thread_read_window_etw_ms'] = common_ms-row['etw_nominal_cpu_ms']
        row.update(thread_id=thread, description=item.get('description') or events['names'].get(thread),
                   baseline_kind=kind, creation_filetime=item.get('creation_filetime'))
        rows.append(row)
    tracked = set(ids)
    for thread, timeline in timelines.items():
        cpu = ticks_between(timeline, first['qpc'][0], last['qpc'][1])*1000/frequency
        if thread not in tracked and cpu > 0:
            missing.append({'thread_id': thread, 'etw_cpu_ms': cpu,
                            'description': events['names'].get(thread)})
    reused_lifetimes = sorted(thread for thread, births in events['starts'].items() if len(births) > 1)
    coverage = (not missing and not unresolved and not repeated_ids and not reused_lifetimes
                and not native.get('tracking_overflow') and native.get('process_exit_code') == 0)
    process = compare_window(tuple(timelines.values()), first, last, frequency)
    process_after_start = compare_window(tuple(timelines.values()), initial_after, final_before, frequency)
    thread_sum = sum(row['os_cpu_ms'] for row in rows)
    minimum_delta = (final_before['cpu_ns']-initial_after['cpu_ns'])/1_000_000
    maximum_delta = (last['cpu_ns']-first['cpu_ns'])/1_000_000
    result = {'schema': 'external-thread-cpu-diagnostic-v1',
              'diagnostic_only': True, 'production_16ms_acceptance': False,
              'acceptance_gate_changed': False, 'coverage_complete': coverage,
              'scope': 'ready/go to complete process exit; includes post-envelope manifests/drain/shutdown. Not the Rust frame envelope or a clean performance run.',
              'process_id': native['process_id'], 'qpc_frequency': frequency,
              'process_before_sandwich': process, 'process_after_sandwich': process_after_start,
              'thread_os_cpu_sum_ms': thread_sum,
              'process_os_delta_sandwich_ms': [minimum_delta, maximum_delta],
              'thread_sum_within_process_sandwich': minimum_delta <= thread_sum <= maximum_delta,
              'thread_sum_minus_process_before_ms': thread_sum-process['os_cpu_ms'],
              'untracked_running_threads': missing, 'unresolved_thread_baselines': unresolved,
              'reused_thread_ids': repeated_ids, 'etw_reused_thread_ids': reused_lifetimes,
              'native_errors': native.get('errors', []),
              'rows': sorted(rows, key=lambda row: -row['etw_nominal_cpu_ms'])}
    return result


def account(native_path, csv_path, output, official_path=None):
    native = json.loads(Path(native_path).read_text(encoding='utf-8'))
    require(native.get('schema') == 'retained-thread-counters-v1', 'unknown native counter schema')
    timelines, failures, first, last = read_trace(csv_path, native['process_id'], native['qpc_frequency'])
    start = native['initial']['process_before_threads']['qpc'][0]
    stop = native['final']['process_after_threads']['qpc'][1]
    require(not any(start <= time <= stop for time, thread in failures), 'unmatched target CSwitch inside diagnostic')
    result = analyze_data(native, timelines, lifecycle(csv_path, native['process_id']), first, last)
    if official_path is not None:
        official = {}
        for line in Path(official_path).read_text(encoding='utf-8-sig').splitlines():
            row = [value.strip() for value in next(csv.reader([line], skipinitialspace=True))]
            if len(row) != 3 or not row[0].isdigit() or not row[2].isdigit():
                continue
            match = PID.search(row[1])
            if match and int(match[1]) == native['process_id']:
                thread = int(row[2])
                require(thread not in official, 'duplicate TID in Microsoft independent accounting')
                official[thread] = int(row[0])/1000
        require(bool(official), 'Microsoft independent process accounting missing')
        total = sum(official.values())
        measured = result['process_before_sandwich']['etw_nominal_cpu_ms']
        error = abs(total-measured)/max(total, measured, 1e-6)
        result['microsoft_independent_process'] = {'cpu_ms': total, 'relative_error': error,
                                                  'within_existing_2pct': error <= .02,
                                                  'input_trace_shared': True}
        for row in result['rows']:
            row['microsoft_common_window_cpu_ms'] = official.get(row['thread_id'])
            if row['thread_id'] in official:
                value = official[row['thread_id']]
                own = row['common_process_window_etw_cpu_ms']
                row['microsoft_common_window_relative_error'] = abs(value-own)/max(value, own, 1e-6)
    result['files'] = {}
    for path in (Path(native_path), Path(csv_path)):
        with path.open('rb') as source:
            result['files'][str(path)] = hashlib.file_digest(source, 'sha256').hexdigest()
    result['parser_file'] = str(Path(sys.modules['perf_etw'].__file__).resolve())
    with Path(result['parser_file']).open('rb') as source:
        result['parser_sha256'] = hashlib.file_digest(source, 'sha256').hexdigest()
    Path(output).write_text(json.dumps(result, indent=2, ensure_ascii=False)+'\n', encoding='utf-8')
    return result


if __name__ == '__main__':
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument('native', type=Path)
    parser.add_argument('trace', type=Path)
    parser.add_argument('--output', type=Path, required=True)
    parser.add_argument('--official', type=Path)
    args = parser.parse_args()
    result = account(args.native, args.trace, args.output, args.official)
    print(json.dumps({key: result[key] for key in ('coverage_complete', 'thread_os_cpu_sum_ms',
                      'process_os_delta_sandwich_ms', 'thread_sum_within_process_sandwich',
                      'process_before_sandwich')}, indent=2))
