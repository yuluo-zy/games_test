"""Build/freeze and measure the existing shipped-session headless fixture.

Requires elevated Windows xperf for run/overhead. Warmup happens before tracing;
only this invocation's successfully started NT Kernel Logger is stopped.
Headless measurements do not replace production-window acceptance.
"""
import argparse
import csv
import json
import math
import os
from pathlib import Path
import platform
import re
import shutil
import subprocess
import sys
import time
from perf_windows import ROOT, PERF, INSTALL, XPERF, digest, dump, git, label
from perf_headless_account import account, diagnostic_summary, paired_overhead, signature, validate, input_identity, SCENES
from perf_build_target import run_requested_test
from perf_trace_archive import archive_run_csv, open_trace_text
import perf_clock_policy as clocks

TEST = 'cpu_measurement::profile_shipped_session_cpu'

# Controls are set once when constructing the host, never per draw.
CONTROL_ENV = {'reflection_scope': 'TG_REFLECTION_SCOPE',
               'vao_typed_lookup': 'TG_VAO_TYPED_LOOKUP',
               'vao_wrapper_reuse': 'TG_VAO_WRAPPER_REUSE',
               'constant_batch': 'TG_CONSTANT_BATCH'}


def frozen_control_capabilities(snapshot, cpu_probe):
    """Prove controls belong to the source snapshot used for this executable.

    Runtime report verification remains mandatory; strings alone never establish
    that a requested switch was honored by the process.
    """
    import hashlib
    parts = []
    for name, expected in sorted(snapshot['files'].items()):
        if ((name.startswith('crates/tiny-glade/src/') and name.endswith('.rs'))
                or name == 'crates/tiny-glade/tests/cpu_measurement/mod.rs'):
            content = (ROOT/name).read_bytes()
            if hashlib.sha256(content).hexdigest() != expected:
                raise ValueError('management capability source changed since freeze: '+name)
            parts.append(content.decode('utf-8'))
    source = '\n'.join(parts)
    known = '"management_modes"' in source and all(flag in source for flag in CONTROL_ENV.values())
    return {'probe_runtime_controls': sorted(CONTROL_ENV) if known and cpu_probe else [],
            'default_modes': {name: True for name in CONTROL_ENV} if known else None,
            'evidence': 'frozen source declares modes report and all four host-construction controls' if known else 'legacy or unsupported',
            'source_content_sha256': snapshot['content_sha256']}


def requested_management(args, frozen):
    selected = {name: False for name in CONTROL_ENV if getattr(args, 'disable_'+name, False)}
    capabilities = frozen.get('management_controls') or {}
    supported = set(capabilities.get('probe_runtime_controls', []))
    if selected and ('cpu-probe' not in frozen['features'] or not set(selected) <= supported):
        raise ValueError('ablation requires a frozen cpu-probe build with known management-control capabilities')
    defaults = capabilities.get('default_modes')
    expected = None if defaults is None else dict(defaults)
    if selected:
        if expected is None or set(expected) != set(CONTROL_ENV):
            raise ValueError('ablation requires declared default algorithm modes')
        expected.update(selected)
    return selected, expected


def actual_management(report):
    modes = report.get('management_modes')
    if modes is None:
        return None
    if not isinstance(modes, dict) or set(modes) != set(CONTROL_ENV) or any(type(value) is not bool for value in modes.values()):
        raise ValueError('actual management modes must report exactly four booleans')
    return dict(modes)



def environment(diagnostic=False):
    result = os.environ.copy()
    # Do not inherit a different scene, profiler, pacing policy or log probe.
    for name in list(result):
        if name.startswith('TG_') or name.startswith('RHAPSODY_'):
            result.pop(name)
    result.update(TG_INSTALL_ROOT=str(INSTALL), CARGO_INCREMENTAL='0', RUST_LOG='warn',
                  RHAPSODY_REQUIRE_VULKAN='1', TG_TEST_SIZE='1280x720')
    if diagnostic:
        result.update(TG_CPU_BILL='1', TG_FRAME_TIMING='1')
    return result


def source_identity():
    names = subprocess.check_output(['git', '-c', 'safe.directory='+ROOT.as_posix(), '-C', str(ROOT),
                                     'ls-files', '-z', '--cached', '--others', '--exclude-standard'])
    files = {}
    for name in sorted(set(os.fsdecode(item) for item in names.split(b'\0') if item)):
        path = ROOT/name
        files[name.replace('\\', '/')] = digest(path) if path.is_file() else None
    return {'commit': git('rev-parse', 'HEAD'), 'status': git('status', '--porcelain'),
            'files': files, 'content_sha256': signature(files)}


def hardware(destination):
    destination.mkdir(parents=True, exist_ok=True)
    result = {'os': platform.platform(), 'cpu': platform.processor(), 'logical_cpus': os.cpu_count()}
    command = ['powershell', '-NoProfile', '-NonInteractive', '-Command',
               "$ErrorActionPreference='Stop'; Get-CimInstance Win32_Processor | Select-Object Name,NumberOfCores,NumberOfLogicalProcessors | ConvertTo-Json -Compress"]
    cpu = subprocess.run(command, capture_output=True, text=True, timeout=30)
    if cpu.returncode:
        raise RuntimeError('cannot identify physical CPU: '+cpu.stderr)
    try:
        physical = json.loads(cpu.stdout)
        entries = physical if isinstance(physical, list) else [physical]
        if not entries or any(not item.get('Name') or int(item.get('NumberOfCores', 0)) <= 0
                              or int(item.get('NumberOfLogicalProcessors', 0)) <= 0 for item in entries):
            raise ValueError('missing CPU name/topology')
    except (TypeError, ValueError) as error:
        raise RuntimeError('physical CPU identity unknown; hardware measurement invalid') from error
    result['physical_cpu'] = physical
    vulkan = shutil.which('vulkaninfo')
    if not vulkan:
        fallback = Path(os.environ.get('VULKAN_SDK', ''))/'Bin'/'vulkaninfo.exe'
        vulkan = str(fallback) if fallback.is_file() else None
    if not vulkan:
        raise RuntimeError('vulkaninfo is required for actual GPU/driver identity')
    summary = subprocess.run([vulkan, '--summary'], capture_output=True, text=True, timeout=60)
    (destination/'vulkan-summary.txt').write_text(summary.stdout+summary.stderr, encoding='utf-8')
    if summary.returncode:
        raise RuntimeError('Vulkan enumeration failed')
    # The full summary also includes runtime layers and Instance capabilities.
    result['vulkan_summary_sha256'] = digest(destination/'vulkan-summary.txt')
    result['vulkan_environment'] = {name: value for name, value in sorted(os.environ.items())
                                  if name.startswith(('VK_', 'VULKAN_'))}
    result['sha256'] = signature(result)
    dump(destination/'hardware.json', result)
    return result


def build(args):
    destination = PERF/'headless-builds'/label(args.label)
    if destination.exists():
        raise ValueError('frozen build exists: '+str(destination))
    before = source_identity()
    if before['status'] and not args.allow_dirty:
        raise ValueError('commit source before freezing; --allow-dirty marks diagnostic only')
    destination.mkdir(parents=True)
    dump(destination/'source.json', before)
    command = ['cargo', 'build', '-p', 'tiny-glade', '--test', 'original_gameplay_chain',
               '--locked', '--offline', '-j1', '--message-format=json-render-diagnostics']
    if not args.without_cpu_probe:
        command.extend(['--features', 'cpu-probe'])
    if not args.debug:
        command.append('--release')
    build_env = environment()
    relevant_env = {name: value for name, value in sorted(build_env.items())
                    if name in ('CARGO_INCREMENTAL', 'CARGO_TARGET_DIR', 'CARGO_BUILD_TARGET',
                                'RUSTFLAGS', 'CARGO_ENCODED_RUSTFLAGS', 'RUSTUP_TOOLCHAIN',
                                'CC', 'CXX', 'AR', 'LLVM_CONFIG_PATH', 'LIBCLANG_PATH')
                    or name.startswith(('CARGO_TARGET_', 'CARGO_PROFILE_'))}
    requested_artifact, build_lifecycle = run_requested_test(
        command, ROOT, build_env, destination/'build.jsonl', destination/'build.stderr.log')
    dump(destination/'requested-target-artifact.json', requested_artifact)
    after = source_identity()
    if before != after:
        raise RuntimeError('source contents/identity changed during build; executable not frozen')
    artifacts = []
    for line in (destination/'build.jsonl').read_text(encoding='utf-8').splitlines():
        try:
            event = json.loads(line)
        except ValueError:
            continue
        if (event.get('reason') == 'compiler-artifact' and event.get('executable')
                and event.get('target', {}).get('name') == 'original_gameplay_chain'):
            artifacts.append(Path(event['executable']))
    if len(artifacts) != 1:
        raise RuntimeError('Cargo executable missing/ambiguous')
    executable = artifacts[0]
    frozen = destination/executable.name
    shutil.copy2(executable, frozen)
    assets = {frozen.name: digest(frozen)}
    pdb = executable.with_suffix('.pdb')
    if pdb.is_file():
        shutil.copy2(pdb, destination/pdb.name)
        assets[pdb.name] = digest(destination/pdb.name)
    for name in ('fmod.dll', 'fmodstudio.dll'):
        shutil.copy2(INSTALL/name, destination/name)
        assets[name] = digest(destination/name)
    # --list is read-only, proves the feature-gated ignored entry was linked.
    listing = subprocess.run([str(frozen), '--list', '--ignored'], cwd=ROOT,
                             env=environment(), capture_output=True, text=True, timeout=60)
    (destination/'test-list.txt').write_text(listing.stdout+listing.stderr, encoding='utf-8')
    if listing.returncode or TEST+': test' not in listing.stdout:
        raise RuntimeError('frozen executable lacks the requested ignored fixture')
    machine = hardware(destination)
    record = {'schema': 'headless-frozen-build-v1', 'label': args.label,
              'executable': str(frozen), 'executable_sha256': digest(frozen), 'files': assets,
              'source_commit': before['commit'], 'source_content_sha256': before['content_sha256'],
              'dirty': bool(before['status']), 'profile': 'debug' if args.debug else 'release',
              'features': ['default(mimalloc)'] + ([] if args.without_cpu_probe else ['cpu-probe']),
              'cpu_probe_feature': not args.without_cpu_probe, 'allocator': 'mimalloc',
              'incremental': False, 'jobs': 1, 'command': command,
              'build_environment': relevant_env, 'build_environment_sha256': signature(relevant_env),
              'cargo_lock_sha256': digest(ROOT/'Cargo.lock'), 'hardware': machine,
              'rustc': subprocess.check_output(['rustc', '-Vv'], text=True),
              'requested_test_target_completed': build_lifecycle['requested_target_completed'],
              'entire_cargo_build_passed': build_lifecycle['entire_cargo_build_passed'],
              'cargo_lifecycle': build_lifecycle,
              'requested_target_evidence_sha256': digest(destination/'requested-target-artifact.json'),
              'build_jsonl_sha256': digest(destination/'build.jsonl'),
              'build_helper_sha256': digest(Path(__file__).with_name('perf_build_target.py')),
              'scope': 'Frozen headless shipped-session test; not WSI acceptance',
              'measurement_capabilities': 'active phase session and diagnostic bill' if not args.without_cpu_probe else 'envelope clocks only; runtime mode off required'}
    record['management_controls'] = frozen_control_capabilities(before, not args.without_cpu_probe)
    dump(destination/'build.json', record)
    print(json.dumps(record, ensure_ascii=False), flush=True)


def load_build(name):
    path = PERF/'headless-builds'/label(name)/'build.json'
    frozen = json.loads(path.read_text(encoding='utf-8'))
    if frozen.get('schema') != 'headless-frozen-build-v1':
        raise ValueError('unknown frozen build schema')
    for name, expected in frozen['files'].items():
        if digest(path.parent/name) != expected:
            raise ValueError('frozen binary/PDB/DLL identity changed: '+name)
    if frozen.get('allocator') != 'mimalloc' or frozen.get('incremental') is not False:
        raise ValueError('unexpected build configuration')
    return frozen


def start_trace(destination, xperf=XPERF):
    loggers = subprocess.check_output([str(xperf), '-loggers'], text=True, stderr=subprocess.STDOUT)
    (destination/'loggers-before.txt').write_text(loggers, encoding='utf-8')
    if re.search(r'^\s*Logger Name\s*:\s*NT Kernel Logger\s*$', loggers, re.M | re.I):
        raise RuntimeError('another NT Kernel Logger is active; no session was started or stopped')
    command = [str(xperf), '-on', 'PROC_THREAD+CSWITCH+DISPATCHER+DPC+INTERRUPT',
               '-ClockType', 'PerfCounter', '-BufferSize', '256', '-MinBuffers', '64',
               '-MaxBuffers', '512', '-f', str(destination/'schedule.etl')]
    with (destination/'xperf-start.log').open('w', encoding='utf-8') as out:
        subprocess.run(command, stdout=out, stderr=subprocess.STDOUT, check=True)


def official_owner_total(report, csv_path, etl_path, destination, xperf=XPERF):
    origin = None
    with open_trace_text(csv_path) as source:
        for line in source:
            match = re.match(r'^#\s+\d+\s+#\s+(0x[0-9a-fA-F]+)', line)
            if match:
                origin = int(match[1], 16)
                break
    if origin is None:
        raise ValueError('raw QPC origin missing')
    frequency = report['qpc_frequency']
    first, last = report['envelope']['begin'][0], report['envelope']['end'][0]
    left = math.floor((first-origin)*1_000_000/frequency)
    right = math.ceil((last-origin)*1_000_000/frequency)
    output = destination/'official-owner.csv'
    with (destination/'xperf-official.log').open('w', encoding='utf-8') as out:
        subprocess.run([str(xperf), '-i', str(etl_path), '-o', str(output), '-a', 'cswitch',
                        '-util', '-thread', '-exc_dpcisr', '-range', str(left), str(right)],
                       stdout=out, stderr=subprocess.STDOUT, check=True)
    values = []
    for line in output.read_text(encoding='utf-8-sig', errors='replace').splitlines():
        row = [cell.strip() for cell in next(csv.reader([line], skipinitialspace=True))]
        if len(row) == 3 and row[0].isdigit() and row[2].isdigit() and int(row[2]) == report['thread_id']:
            values.append(int(row[0])*1000)
    if len(values) != 1:
        raise ValueError('independent Microsoft thread accounting missing/ambiguous')
    return values[0]


def capture(args, run_label=None, mode=None, build_label=None):
    if getattr(args, 'archive_trace_csv', False) and args.without_etw:
        raise ValueError('--archive-trace-csv requires an ETW capture')
    policy_id = getattr(args, 'clock_policy', clocks.LEGACY)
    if policy_id == clocks.CALIBRATED and (args.without_etw or getattr(args, 'clock_calibration', None) is None):
        raise ValueError('calibrated policy requires ETW and frozen --clock-calibration evidence')
    frozen = load_build(build_label or args.label)
    mode = mode or args.mode
    feature = 'cpu-probe' in frozen['features']
    if not feature and (mode != 'off' or args.diagnostic):
        raise ValueError('feature-off build supports only --mode off and no diagnostic bill')
    selected_management, expected_management = requested_management(args, frozen)
    scene = getattr(args, 'scene', 'static')
    if scene not in SCENES or (scene != 'static' and args.warmup < 200):
        raise ValueError('known scene required; non-static scenarios require warmup >=200 (formal runs use600)')
    destination = PERF/'headless-runs'/label(run_label or args.run_label)
    destination.mkdir(parents=True, exist_ok=False)
    machine = hardware(destination)
    measurement_tools = clocks.current_tools()
    policy_key, policy_intent = clocks.prepare_capture(
        policy_id, getattr(args, 'clock_calibration', None), machine, destination, measurement_tools)
    session_path = destination/'session.json'
    settings = {'clock_policy': policy_key, 'scenario': scene, 'dimensions': '1280x720', 'warmup': args.warmup, 'samples': args.samples,
                'features': frozen['features'], 'allocator': 'mimalloc',
                'driver_environment': machine['vulkan_environment'],
                'worker_configuration': 'existing default; TG overrides cleared',
                'diagnostic': args.diagnostic, 'process_priority': 'NORMAL_PRIORITY_CLASS',
                'thread_priority': 'existing fixture default',
                'management_ablation': selected_management, 'management_modes_expected': expected_management}
    env = environment(args.diagnostic)
    for name in selected_management:
        env[CONTROL_ENV[name]] = '0'
    env.update(TG_PERF_SCENARIO=scene, TG_CPU_PROBE_GATE='1', TG_CPU_PROBE_OUT=str(session_path),
               TG_CPU_PROBE_MODE=mode, TG_CPU_PROBE_WARMUP=str(args.warmup),
               TG_CPU_PROBE_SAMPLES=str(args.samples))
    record = {'measurement_tools': measurement_tools, 'measurement_tools_sha256': signature(measurement_tools),
              'schema': 'headless-capture-v1', 'valid': False, 'mode': mode,
              'clock_policy': policy_key, 'clock_policy_intent': policy_intent,
              'scenario': scene, 'warmup': args.warmup, 'samples': args.samples, 'diagnostic': args.diagnostic,
              'build': frozen, 'hardware': machine, 'hardware_sha256': machine['sha256'],
              'executable_sha256': frozen['executable_sha256'], 'settings': settings,
              'cpu_probe_feature': feature, 'management_ablation': selected_management,
              'management_modes_expected': expected_management,
              'attribution_scope': 'same_candidate_algorithm_ablation' if selected_management else 'default_algorithms',
              'original_baseline_gain_evidence': False if selected_management else None,
              'comparison_settings_sha256': signature({k: v for k, v in settings.items() if k != 'features'}),
              'settings_sha256': signature(settings), 'session_path': str(session_path),
              'start_utc': time.strftime('%Y-%m-%dT%H:%M:%SZ', time.gmtime()),
              'etw': not args.without_etw, 'trace_started_by_this_run': False,
              'production_16ms_acceptance': False,
              'archive_trace_csv_requested': bool(getattr(args, 'archive_trace_csv', False))}
    process = None
    started = False
    try:
        with (destination/'fixture.log').open('w', encoding='utf-8') as log:
            command = [frozen['executable'], TEST, '--exact', '--ignored', '--nocapture', '--test-threads=1']
            record['command'] = command
            flags = getattr(subprocess, 'NORMAL_PRIORITY_CLASS', 0)
            process = subprocess.Popen(command, cwd=ROOT, env=env, stdout=log,
                                       stderr=subprocess.STDOUT, creationflags=flags)
            record['process_id'] = process.pid
            deadline = time.monotonic()+args.timeout
            ready = session_path.with_suffix('.ready')
            while not ready.exists():
                if process.poll() is not None:
                    raise RuntimeError('fixture exited before warmup handshake; see fixture.log')
                if time.monotonic() >= deadline:
                    raise TimeoutError('fixture warmup timed out')
                time.sleep(.05)
            record['ready_utc'] = time.strftime('%Y-%m-%dT%H:%M:%SZ', time.gmtime())
            if not args.without_etw:
                start_trace(destination)
                started = True
                record['trace_started_by_this_run'] = True
            session_path.with_suffix('.go').write_text('go', encoding='utf-8')
            record['exit_code'] = process.wait(timeout=max(.1, deadline-time.monotonic()))
            if record['exit_code']:
                raise RuntimeError('fixture failed; see '+str(destination/'fixture.log'))
    except BaseException as error:
        record['error'] = repr(error)
        raise
    finally:
        # Never stop a logger that this invocation did not successfully start.
        if process is not None and process.poll() is None:
            process.terminate()
            try:
                process.wait(timeout=15)
            except subprocess.TimeoutExpired:
                process.kill(); process.wait(timeout=15)
        if started:
            with (destination/'xperf-stop.log').open('w', encoding='utf-8') as out:
                record['etw_stop_exit_code'] = subprocess.run([str(XPERF), '-stop', 'NT Kernel Logger'],
                                                              stdout=out, stderr=subprocess.STDOUT).returncode
        dump(destination/'run.json', record)
    try:
        return finish_capture(args, record, destination, session_path, frozen, mode)
    except BaseException as error:
        record['error'] = repr(error)
        dump(destination/'run.json', record)
        raise


def finish_capture(args, record, destination, session_path, frozen, mode):
    report = json.loads(session_path.read_text(encoding='utf-8'))
    validate(report)
    if report['process_id'] != record['process_id'] or report['allocator'] != 'mimalloc':
        raise ValueError('fixture process/allocator identity mismatch')
    if report['samples'] != args.samples or report['warmup'] != args.warmup or report['mode'] != mode:
        raise ValueError('fixture measurement configuration mismatch')
    if report.get('cpu_probe_feature', True) != record['cpu_probe_feature']:
        raise ValueError('fixture compiled feature identity mismatch')
    if report.get('scenario', 'static') != record['scenario']:
        raise ValueError('fixture scenario differs from requested workload')
    actual = actual_management(report)
    expected = record.get('management_modes_expected')
    if expected is not None and actual != expected:
        raise ValueError('requested management modes missing or not honored by frozen executable')
    if record.get('management_ablation') and actual is None:
        raise ValueError('ablation executable did not report actual management modes')
    record['management_modes'] = actual
    record['management_modes_sha256'] = signature(actual)
    record['workload'] = report.get('workload')
    record['workload_sha256'] = signature(record['workload'])
    record['input_sha256'] = signature(input_identity(report))
    record['input_policy'] = input_identity(report)['policy']
    record['input_unchanged'] = report['input_before'] == report['input_after']
    record['strict_static_scene'] = record['input_policy'] == 'strict_static_input_unchanged'
    record['session_sha256'] = digest(session_path)
    if args.diagnostic:
        diagnostic = diagnostic_summary(report)
        dump(destination/'diagnostic.json', diagnostic)
        record['diagnostic_summary'] = str(destination/'diagnostic.json')
    if args.without_etw:
        record['invalid_reasons'] = ['No scheduling trace: diagnostic only; coarse CPU cannot establish frame percentiles']
    else:
        if record.get('etw_stop_exit_code') != 0:
            raise RuntimeError('failed to finalize this run trace')
        csv_path = destination/'schedule.csv'
        with (destination/'xperf-dump.log').open('w', encoding='utf-8') as out:
            subprocess.run([str(XPERF), '-i', str(destination/'schedule.etl'), '-o', str(csv_path),
                            '-a', 'dumper', '-add_rawdata'], stdout=out, stderr=subprocess.STDOUT, check=True)
        calibrated = record.get('clock_policy', {}).get('id') == clocks.CALIBRATED
        record['trace_csv_sha256'] = digest(csv_path)
        independent = official_owner_total(
            report, csv_path, destination/'schedule.etl', destination) if args.official or calibrated else None
        if calibrated:
            official = clocks.official_capture_totals(report, destination, record['trace_csv_sha256'])
            record['official_cswitch'] = official
            record['trace_etl_sha256'] = official['trace_etl_sha256']
            precise = account(report, csv_path, independent, capture_record=record, official_totals=official)
        else:
            precise = account(report, csv_path, independent)
        dump(destination/'cpu.json', precise)
        record['account'] = precise
        record['trace_csv_path'] = str(csv_path)
        # All OS capture, optional Microsoft analysis and CPU accounting are
        # complete. Archive even a numerically invalid result without changing
        # its acceptance status. Preserve the raw hash before removing CSV.
        if getattr(args, 'archive_trace_csv', False):
            dump(destination/'run.json', record)
            record['trace_csv_archive'] = archive_run_csv(
                destination, PERF/'headless-runs', record['trace_csv_sha256'])
            record['trace_csv_path'] = record['trace_csv_archive']['path']
        # Accounting validity is distinct from a clean frozen release candidate.
        record['valid'] = precise['valid'] and not args.diagnostic
        record['clean_release_candidate'] = (record['valid'] and not frozen['dirty'] and frozen['profile'] == 'release'
                                             and not record.get('management_ablation'))
    dump(destination/'run.json', record)
    print(json.dumps({k: record.get(k) for k in ('valid', 'mode', 'diagnostic', 'clean_release_candidate',
                                               'session_path', 'input_sha256')}, ensure_ascii=False), flush=True)
    return record


def run(args):
    result = capture(args)
    if not result['valid'] and not args.diagnostic and not args.without_etw:
        raise SystemExit(2)


def overhead(args):
    if args.pairs < 3:
        raise ValueError('at least three alternating on/off pairs required')
    if args.diagnostic or args.without_etw:
        raise ValueError('paired overhead requires clean scheduling traces')
    if any(getattr(args, 'disable_'+name, False) for name in CONTROL_ENV):
        raise ValueError('probe overhead cannot change algorithm management modes')
    destination = PERF/'headless-overhead'/label(args.series)
    destination.mkdir(parents=True, exist_ok=False)
    runs = []
    try:
        feature_off_label = getattr(args, 'feature_off_label', None)
        feature_on_mode = getattr(args, 'feature_on_mode', 'on')
        if feature_off_label:
            if 'cpu-probe' not in load_build(args.label)['features'] or 'cpu-probe' in load_build(feature_off_label)['features']:
                raise ValueError('compiled pair requires a cpu-probe build and a default feature-off build')
        elif feature_on_mode != 'on':
            raise ValueError('--feature-on-mode off requires --feature-off-label')
        for pair in range(args.pairs):
            order = ('on', 'off') if pair % 2 == 0 else ('off', 'on')
            for role in order:
                name = args.series+'-'+str(pair+1)+'-'+role
                if feature_off_label:
                    mode = feature_on_mode if role == 'on' else 'off'
                    build_label = args.label if role == 'on' else feature_off_label
                else:
                    mode, build_label = role, args.label
                result = capture(args, run_label=name, mode=mode, build_label=build_label)
                runs.append(result)
                dump(destination/'progress.json', {'runs': [r['session_path'] for r in runs]})
        kind = ('compiled_feature_session_'+feature_on_mode) if feature_off_label else 'runtime_session'
        summary = paired_overhead(runs, kind)
    except BaseException as error:
        dump(destination/'summary.json', {'schema': 'headless-probe-overhead-v1', 'valid': False,
                                        'error': repr(error), 'completed_runs': [r['session_path'] for r in runs]})
        raise
    summary['runs'] = [r['session_path'] for r in runs]
    summary['clean_release_candidate'] = summary['valid'] and all(r.get('clean_release_candidate') for r in runs)
    dump(destination/'summary.json', summary)
    print(json.dumps(summary, ensure_ascii=False), flush=True)
    if not summary['valid']:
        raise SystemExit(2)



def account_command(args):
    record = json.loads(args.capture_run.read_text(encoding='utf-8')) if args.capture_run else None
    result = account(json.loads(args.session.read_text(encoding='utf-8')), args.trace_csv,
                     capture_record=record, official_totals=record.get('official_cswitch') if record else None)
    dump(args.output, result)
    if not result['valid']:
        raise SystemExit(2)


def measurement_options(parser):
    parser.add_argument('--clock-policy', choices=clocks.POLICIES, default=clocks.LEGACY,
                        help='new policy requires prospective calibrated capture and mandatory Microsoft CSwitch')
    parser.add_argument('--clock-calibration', type=Path, help='frozen evidence JSON for calibrated policy')
    parser.add_argument('--scene', choices=SCENES, default='static')
    parser.add_argument('--warmup', type=int, default=600)
    parser.add_argument('--samples', type=int, default=3000)
    parser.add_argument('--timeout', type=int, default=7200)
    parser.add_argument('--diagnostic', action='store_true')
    parser.add_argument('--without-etw', action='store_true')
    parser.add_argument('--official', action='store_true', help='also compare Microsoft CSwitch integrator')
    parser.add_argument('--archive-trace-csv', action='store_true',
                        help='after accounting, verify lossless gzip then remove only this run CSV; retain ETL')


def main():
    parser = argparse.ArgumentParser(description=__doc__)
    commands = parser.add_subparsers(dest='command', required=True)
    p = commands.add_parser('build')
    p.add_argument('label'); p.add_argument('--debug', action='store_true'); p.add_argument('--allow-dirty', action='store_true'); p.add_argument('--without-cpu-probe', action='store_true')
    p.set_defaults(function=build)
    p = commands.add_parser('run')
    p.add_argument('label'); p.add_argument('run_label'); p.add_argument('--mode', choices=('on', 'off'), default='on')
    for name in CONTROL_ENV:
        p.add_argument('--disable-'+name.replace('_', '-'), dest='disable_'+name, action='store_true')
    measurement_options(p); p.set_defaults(function=run)
    p = commands.add_parser('overhead')
    p.add_argument('label'); p.add_argument('series'); p.add_argument('--pairs', type=int, default=3)
    p.add_argument('--feature-off-label', help='separate default-feature-off frozen build')
    p.add_argument('--feature-on-mode', choices=('on', 'off'), default='on')
    measurement_options(p); p.set_defaults(function=overhead)
    p = commands.add_parser('freeze-clock-policy')
    p.add_argument('output', type=Path); p.add_argument('--predeclared', type=Path, required=True)
    p.add_argument('--calibration-run', type=Path, action='append', required=True,
                   help='six completed directories in predeclared AB/BA/AB order')
    p.set_defaults(function=lambda a: clocks.freeze_evidence(a.output, a.predeclared, a.calibration_run))
    p = commands.add_parser('account')
    p.add_argument('session', type=Path); p.add_argument('trace_csv', type=Path); p.add_argument('--output', type=Path, required=True)
    p.add_argument('--capture-run', type=Path, help='original prospective run.json; old runs remain legacy')
    p.set_defaults(function=account_command)
    p = commands.add_parser('summarize')
    p.add_argument('session', type=Path); p.add_argument('--output', type=Path, required=True)
    p.set_defaults(function=lambda a: dump(a.output, diagnostic_summary(json.loads(a.session.read_text(encoding='utf-8')))))
    args = parser.parse_args()
    if hasattr(args, 'warmup') and not (args.warmup >= 1 and 1 <= args.samples <= 100000 and args.timeout > 0):
        parser.error('warmup>=1, samples=1..100000 and timeout>0 required')
    if getattr(args, 'diagnostic', False) and getattr(args, 'mode', 'on') == 'off':
        parser.error('diagnostic bill requires on mode')
    if getattr(args, 'scene', 'static') != 'static' and args.warmup < 200:
        parser.error('non-static scenarios need warmup>=200; formal comparisons use600')
    args.function(args)


if __name__ == '__main__':
    main()
