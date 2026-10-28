"""Run an explicit external OS-counter diagnostic on an existing frozen EXE.

This invokes the existing ready/go test fixture. It never compiles Rust, changes
priority/timer/driver settings, or changes any 2% / 16ms acceptance criterion.
"""
import argparse
import hashlib
import json
import os
from pathlib import Path
import platform
import subprocess
import sys
import time

sys.dont_write_bytecode = True
os.environ['PYTHONDONTWRITEBYTECODE'] = '1'
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
import perf_headless as perf
if Path(perf.__file__).resolve().parent != REPO_TOOLS:
    raise RuntimeError('perf_headless was imported from a different repository')
from native_thread_cpu import WindowsCPU, RetainedThreads
if Path(sys.modules['native_thread_cpu'].__file__).resolve().parent != Path(__file__).resolve().parent:
    raise RuntimeError('native_thread_cpu was imported from a different calibration package')


def identities():
    ensure_local_modules()
    result = {}
    for path in (Path(__file__), Path(__file__).with_name('native_thread_cpu.py'),
                 Path(__file__).with_name('synthetic_sha256.py'), Path(__file__).with_name('thread_cpu_account.py'),
                 Path(__file__).with_name('predeclare.py'),
                 Path(perf.__file__), Path(perf.__file__).with_name('perf_etw.py'),
                 Path(perf.__file__).with_name('perf_trace_archive.py'),
                 Path(perf.__file__).with_name('perf_headless_account.py'),
                 Path(perf.__file__).with_name('perf_windows.py'),
                 Path(perf.__file__).with_name('perf_build_target.py'),
                 Path(perf.__file__).with_name('perf_clock_policy.py')):
        with path.open('rb') as source:
            result[path.resolve().relative_to(REPO_TOOLS.parent).as_posix()] = hashlib.file_digest(source, 'sha256').hexdigest()
    return result


def cpu_hardware(destination):
    command = ['powershell', '-NoProfile', '-NonInteractive', '-Command',
               "$ErrorActionPreference='Stop'; Get-CimInstance Win32_Processor | Select-Object Name,NumberOfCores,NumberOfLogicalProcessors | ConvertTo-Json -Compress"]
    result = subprocess.run(command, capture_output=True, text=True, timeout=30, check=True)
    value = {'os': platform.platform(), 'physical_cpu': json.loads(result.stdout),
             'logical_cpus': os.cpu_count(), 'python': sys.version,
             'scope': 'CPU-only synthetic calibration; no Vulkan enumeration/renderer/GPU'}
    perf.dump(destination/'hardware.json', value)
    return value


def capture(args):
    synthetic = getattr(args, 'synthetic', False)
    if synthetic:
        from predeclare import verify_capture_binding
        verify_capture_binding(args)
    if synthetic:
        frozen = {'kind': 'CPU-only native SHA256 synthetic child', 'python': sys.executable,
                  'python_sha256': perf.digest(Path(sys.executable)), 'program_sha256': identities()}
    else:
        frozen = perf.load_build(args.build)
        if args.mode == 'on' and not frozen['cpu_probe_feature']:
            raise ValueError('active mode requires a frozen cpu-probe feature build')
        if args.scene != 'static' and args.warmup < 200:
            raise ValueError('existing workload protocol requires warmup >=200 for non-static scenarios')
    destination = perf.PERF/'thread-cpu-diagnostics'/perf.label(args.run)
    destination.mkdir(parents=True, exist_ok=False)
    helper_ids = identities()
    hardware = cpu_hardware(destination) if synthetic else perf.hardware(destination)
    native = WindowsCPU()
    session = destination/'session.json'
    env = perf.environment()
    if synthetic:
        command = [sys.executable, str(Path(__file__).with_name('synthetic_sha256.py')), args.pattern,
                   '--output', str(session), '--workers', str(args.workers), '--chunks', str(args.chunks),
                   '--chunk-bytes', str(args.chunk_bytes), '--burst-wait-ms', str(args.burst_wait_ms)]
    else:
        env.update(TG_CPU_PROBE_GATE='1', TG_CPU_PROBE_OUT=str(session), TG_CPU_PROBE_MODE=args.mode,
                   TG_CPU_PROBE_WARMUP=str(args.warmup), TG_CPU_PROBE_SAMPLES=str(args.samples),
                   TG_PERF_SCENARIO=args.scene)
        command = [frozen['executable'], perf.TEST, '--exact', '--ignored', '--nocapture', '--test-threads=1']
    record = {'schema': 'external-thread-counter-capture-v1', 'valid_capture': False,
              'diagnostic_only': True, 'production_16ms_acceptance': False,
              'acceptance_gate_changed': False, 'frozen_build': frozen, 'hardware': hardware,
              'command': command, 'scene': args.scene, 'warmup': args.warmup, 'samples': args.samples,
              'mode': args.mode, 'poll_interval_ms': args.poll_ms,
              'target_kind': 'synthetic' if synthetic else 'frozen_game',
              'scope': 'External ready/go through process exit; monitoring adds diagnostic cost and includes post-frame shutdown.',
              'helpers': helper_ids, 'trace_started_by_this_capture': False,
              'declaration_binding': getattr(args, 'declaration_binding', None)}
    counters = {'schema': 'retained-thread-counters-v1', 'qpc_frequency': native.frequency,
                'initial': None, 'final': None, 'process_exit_code': None}
    process = tracker = process_handle = None
    started = False
    error = None
    try:
        with (destination/'fixture.log').open('w', encoding='utf-8') as log:
            process = subprocess.Popen(command, cwd=perf.ROOT, env=env, stdout=log,
                                       stderr=subprocess.STDOUT)
            record['process_id'] = counters['process_id'] = process.pid
            process_handle = native.process(process.pid)
            tracker = RetainedThreads(native, process.pid, capacity=args.max_threads)
            deadline = time.monotonic()+args.timeout
            while not session.with_suffix('.ready').exists():
                if process.poll() is not None:
                    raise RuntimeError('existing fixture exited before ready handshake; see fixture.log')
                if time.monotonic() >= deadline:
                    raise TimeoutError('fixture warmup timed out')
                time.sleep(.02)
            tracker.discover(initial=True)
            perf.start_trace(destination)
            started = True
            record['trace_started_by_this_capture'] = True
            before = native.times(process_handle, thread=False)
            tracker.baseline()
            after = native.times(process_handle, thread=False)
            counters['initial'] = {'process_before_threads': before, 'process_after_threads': after}
            counters['go_qpc_before'] = native.qpc()
            session.with_suffix('.go').write_text('go', encoding='utf-8')
            counters['go_qpc_after'] = native.qpc()
            while process.poll() is None:
                tracker.discover()
                if time.monotonic() >= deadline:
                    raise TimeoutError('frozen fixture runtime exceeded timeout')
                # External discovery only; no change to target pacing or timers.
                time.sleep(args.poll_ms/1000)
            record['exit_code'] = counters['process_exit_code'] = process.returncode
            counters['process_exit_observed_qpc'] = native.qpc()
            before = native.times(process_handle, thread=False)
            tracker.finalize()
            after = native.times(process_handle, thread=False)
            counters['final'] = {'process_before_threads': before, 'process_after_threads': after}
            if process.returncode:
                raise RuntimeError('existing fixture failed; native evidence retained')
            if before['cpu_ns'] != after['cpu_ns']:
                raise RuntimeError('process CPU counter changed after observed process exit')
            fixture = json.loads(session.read_text(encoding='utf-8'))
            if fixture['process_id'] != process.pid or fixture['qpc_frequency'] != native.frequency:
                raise RuntimeError('fixture/native process or QPC identity mismatch')
            if synthetic:
                if fixture.get('schema') != 'synthetic-thread-clock-v1' or fixture.get('pattern') != args.pattern:
                    raise RuntimeError('unexpected synthetic fixture identity')
                if fixture['workload']['workers'] != args.workers or fixture['workload']['chunk_bytes'] != args.chunk_bytes or fixture['workload']['chunks_per_worker'] != args.chunks:
                    raise RuntimeError('synthetic work differs from declared configuration')
                counters['primary_worker_tids'] = [row['thread_id'] for row in fixture['workers']]
                record['synthetic_workload'] = fixture['workload']
                record['synthetic_workload_sha256'] = fixture['workload_sha256']
            elif fixture['samples'] != args.samples or fixture['warmup'] != args.warmup or fixture['mode'] != args.mode:
                raise RuntimeError('fixture configuration differs from external request')
            record['fixture_schema_valid'] = fixture.get('valid') is True
            record['fixture_envelope'] = fixture.get('envelope')
            record['fixture_owner_thread_id'] = fixture['thread_id']
            counters['fixture_owner_thread_id'] = fixture['thread_id']
            record['fixture_session_sha256'] = perf.digest(session)
            record['valid_capture'] = fixture.get('valid') is True and not tracker.overflow
    except BaseException as failure:
        error = failure
        record['error'] = repr(failure)
        record['valid_capture'] = False
    finally:
        cleanup_errors = []
        if process is not None and process.poll() is None:
            # Only this capture's explicitly created test EXE is terminated.
            try:
                process.terminate()
                try:
                    process.wait(timeout=15)
                except subprocess.TimeoutExpired:
                    process.kill()
                    process.wait(timeout=15)
            except BaseException as cleanup_error:
                cleanup_errors.append({'phase': 'owned process cleanup', 'error': repr(cleanup_error)})
            counters['process_exit_code'] = process.returncode
            record['cleanup_terminated_owned_fixture'] = True
        if tracker is not None:
            if counters['final'] is None and process_handle is not None:
                try:
                    before = native.times(process_handle, thread=False)
                    tracker.finalize()
                    after = native.times(process_handle, thread=False)
                    counters['final'] = {'process_before_threads': before, 'process_after_threads': after}
                except Exception as final_error:
                    record['final_counter_error'] = repr(final_error)
            counters.update(tracker.report())
            try:
                tracker.close()
            except BaseException as cleanup_error:
                cleanup_errors.append({'phase': 'retained handle cleanup', 'error': repr(cleanup_error)})
        if process_handle is not None:
            try:
                native.close(process_handle)
            except BaseException as cleanup_error:
                cleanup_errors.append({'phase': 'process handle cleanup', 'error': repr(cleanup_error)})
        if started:
            try:
                with (destination/'xperf-stop.log').open('w', encoding='utf-8') as log:
                    record['etw_stop_exit_code'] = subprocess.run([str(perf.XPERF), '-stop', 'NT Kernel Logger'],
                        stdout=log, stderr=subprocess.STDOUT).returncode
            except BaseException as cleanup_error:
                cleanup_errors.append({'phase': 'own ETW stop', 'error': repr(cleanup_error)})
            if record.get('etw_stop_exit_code') != 0:
                record['valid_capture'] = False
        try:
            if identities() != helper_ids:
                record['helpers_changed_during_capture'] = True
                record['valid_capture'] = False
        except BaseException as identity_error:
            cleanup_errors.append({'phase': 'helper identity recheck', 'error': repr(identity_error)})
        if cleanup_errors:
            record['cleanup_errors'] = cleanup_errors
            record['valid_capture'] = False
        perf.dump(destination/'native-counters.json', counters)
        record['native_counters_sha256'] = perf.digest(destination/'native-counters.json')
        perf.dump(destination/'capture.json', record)
    if error:
        raise error
    if not record['valid_capture'] or record.get('etw_stop_exit_code') != 0:
        raise RuntimeError('capture quality failed; preserve native/trace evidence and do not accept')
    print(json.dumps({'capture': str(destination), 'valid_capture': True,
                      'diagnostic_only': True, 'tracked_thread_objects': len(counters['threads']),
                      'tracking_overflow': counters['tracking_overflow']}, indent=2), flush=True)
    if not args.capture_only:
        postprocess(destination)
    return record


def synthetic_pair(args):
    ensure_local_modules()
    from predeclare import validate_series
    args.declaration_binding = validate_series(args)
    destination = perf.PERF/'thread-cpu-calibrations'/perf.label(args.series)
    destination.mkdir(parents=True, exist_ok=False)
    summary = {'schema': 'synthetic-clock-calibration-series-v1', 'diagnostic_only': True,
               'acceptance_gate_changed': False, 'valid_same_workload': False,
               'declaration_binding': args.declaration_binding,
               'predeclared_pairs': args.pairs, 'workers': args.workers, 'chunks': args.chunks,
               'chunk_bytes': args.chunk_bytes, 'burst_wait_ms': args.burst_wait_ms,
               'runs': [], 'order': 'continuous/burst, burst/continuous alternating'}
    signatures, results = set(), set()
    try:
        for pair in range(args.pairs):
            order = ('continuous', 'burst') if pair % 2 == 0 else ('burst', 'continuous')
            for pattern in order:
                name = args.series+'-'+str(pair+1)+'-'+pattern
                current = argparse.Namespace(**vars(args))
                current.synthetic, current.pattern, current.run = True, pattern, name
                current.mode, current.scene, current.warmup, current.samples = 'synthetic', 'synthetic', 0, args.chunks
                current.build, current.capture_only = None, False
                capture(current)
                run = perf.PERF/'thread-cpu-diagnostics'/name
                fixture = json.loads((run/'session.json').read_text(encoding='utf-8'))
                cpu = json.loads((run/'thread-cpu.json').read_text(encoding='utf-8'))
                signatures.add(fixture['workload_sha256'])
                results.add(tuple(row['sha256'] for row in sorted(fixture['workers'], key=lambda row: row['worker'])))
                workers = {row['thread_id'] for row in fixture['workers']}
                primary = [row for row in cpu['rows'] if row['thread_id'] in workers]
                summary['runs'].append({'directory': str(run), 'pattern': pattern,
                    'coverage_complete': cpu['coverage_complete'],
                    'process_relative_error': cpu['process_before_sandwich']['nominal_relative_error'],
                    'process_within_existing_2pct': cpu['process_before_sandwich']['within_existing_2pct'],
                    'primary_worker_errors': [{'thread_id': row['thread_id'], 'relative_error': row['nominal_relative_error'],
                        'within_existing_2pct': row['within_existing_2pct']} for row in primary],
                    'primary_workers_complete': len(primary) == args.workers,
                    'workload_sha256': fixture['workload_sha256']})
                perf.dump(destination/'summary.json', summary)
        summary['valid_same_workload'] = len(signatures) == len(results) == 1
        if not summary['valid_same_workload']:
            raise RuntimeError('synthetic workload signature or final checksums differ between patterns')
        summary['all_runs_within_existing_2pct'] = all(
            row['coverage_complete'] and row['primary_workers_complete'] and row['process_within_existing_2pct']
            and all(worker['within_existing_2pct'] for worker in row['primary_worker_errors'])
            for row in summary['runs'])
    except BaseException as error:
        summary['error'] = repr(error)
        raise
    finally:
        perf.dump(destination/'summary.json', summary)
    print(json.dumps(summary, indent=2), flush=True)


def postprocess(destination):
    destination = Path(destination)
    record = json.loads((destination/'capture.json').read_text(encoding='utf-8'))
    if not record.get('valid_capture') or record.get('etw_stop_exit_code') != 0:
        raise RuntimeError('cannot account invalid/incomplete external capture')
    csv_path = destination/'schedule.csv'
    if csv_path.exists():
        raise ValueError('CSV already exists; use thread_cpu_account.py to reaccount without overwriting evidence')
    with (destination/'xperf-dump.log').open('w', encoding='utf-8') as log:
        subprocess.run([str(perf.XPERF), '-i', str(destination/'schedule.etl'), '-o', str(csv_path),
                        '-a', 'dumper', '-add_rawdata'], stdout=log, stderr=subprocess.STDOUT, check=True)
    native_counters = json.loads((destination/'native-counters.json').read_text(encoding='utf-8'))
    oracle_thread = (native_counters.get('primary_worker_tids') or [native_counters['fixture_owner_thread_id']])[0]
    independent_report = {'qpc_frequency': native_counters['qpc_frequency'], 'thread_id': oracle_thread,
        'envelope': {'begin': [native_counters['initial']['process_before_threads']['qpc'][0]],
                     'end': [native_counters['final']['process_after_threads']['qpc'][0]]}}
    # Existing Microsoft CSwitch integrator, same common process window. This
    # validates another implementation, never replaces any failed OS counter.
    perf.official_owner_total(independent_report, csv_path, destination/'schedule.etl', destination)
    from thread_cpu_account import account
    result = account(destination/'native-counters.json', csv_path, destination/'thread-cpu.json', destination/'official-owner.csv')
    print(json.dumps({'coverage_complete': result['coverage_complete'],
                      'process': result['process_before_sandwich'],
                      'thread_os_cpu_sum_ms': result['thread_os_cpu_sum_ms'],
                      'process_os_delta_sandwich_ms': result['process_os_delta_sandwich_ms'],
                      'thread_sum_within_process_sandwich': result['thread_sum_within_process_sandwich'],
                      'diagnostic_only': True, 'acceptance_gate_changed': False}, indent=2), flush=True)


def main():
    parser = argparse.ArgumentParser(description=__doc__)
    sub = parser.add_subparsers(dest='command', required=True)
    p = sub.add_parser('capture')
    p.add_argument('build'); p.add_argument('run')
    p.add_argument('--mode', choices=('on', 'off'), default='on')
    p.add_argument('--scene', choices=perf.SCENES, default='static')
    p.add_argument('--warmup', type=int, default=60)
    p.add_argument('--samples', type=int, default=300)
    p.add_argument('--poll-ms', type=int, default=500)
    p.add_argument('--max-threads', type=int, default=4096)
    p.add_argument('--timeout', type=int, default=7200)
    p.add_argument('--capture-only', action='store_true')
    p.set_defaults(function=capture)
    p = sub.add_parser('account')
    p.add_argument('directory', type=Path)
    p.set_defaults(function=lambda args: postprocess(args.directory))
    p = sub.add_parser('synthetic-pair')
    p.add_argument('series')
    p.add_argument('--predeclared', type=Path, required=True, help='fresh portable declaration from predeclare.py')
    p.add_argument('--pairs', type=int, default=3)
    p.add_argument('--workers', type=int, default=1)
    p.add_argument('--chunk-bytes', type=int, default=1048576)
    p.add_argument('--chunks', type=int, default=4096)
    p.add_argument('--burst-wait-ms', type=int, default=10)
    p.add_argument('--poll-ms', type=int, default=500)
    p.add_argument('--max-threads', type=int, default=4096)
    p.add_argument('--timeout', type=int, default=7200)
    p.set_defaults(function=synthetic_pair)
    args = parser.parse_args()
    if args.command == 'capture' and not (args.warmup >= 1 and 1 <= args.samples <= 100000
            and 50 <= args.poll_ms <= 10000 and 1 <= args.max_threads <= 65536 and args.timeout > 0):
        parser.error('invalid bounded diagnostic configuration')
    if args.command == 'synthetic-pair' and not (1 <= args.pairs <= 5 and 1 <= args.workers <= 8
            and 1 <= args.chunk_bytes <= 67108864 and 1 <= args.chunks <= 1000000
            and 1 <= args.burst_wait_ms <= 1000 and 50 <= args.poll_ms <= 10000
            and 1 <= args.max_threads <= 65536 and args.timeout > 0):
        parser.error('invalid bounded synthetic configuration')
    args.function(args)


if __name__ == '__main__':
    main()
