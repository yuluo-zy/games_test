"""Prospective, evidence-bound clock policy. Legacy strict remains the default.

No counter is corrected. Microsoft CSwitch is an independent implementation
over the same ETW event stream, never described as an independent clock.
"""
import datetime
import csv
import gzip
import hashlib
import json
import math
from pathlib import Path
import re
import time
import uuid

LEGACY = 'legacy-strict-v1'
CALIBRATED = 'windows-etw-qpc-calibrated-v1'
POLICIES = (LEGACY, CALIBRATED)
TOOLS = ('perf_clock_policy.py', 'perf_headless.py', 'perf_headless_account.py',
         'perf_etw.py', 'perf_trace_archive.py', 'perf_windows.py')
DEFINITIONS = {
    LEGACY: {'owner': 'GetThreadTimes <=2%', 'process': 'GetProcessTimes <=2%',
             'raw_counters_corrected': False},
    CALIBRATED: {'owner': 'actual frame envelope GetThreadTimes <=2%',
                 'process': 'same-window complete PID Microsoft CSwitch <=2%',
                 'official': 'mandatory; shared ETW event stream; independent implementation',
                 'raw_process_coarse_check': 'reported unchanged; not an acceptance reference',
                 'lifetimes': 'unique origin and final lifecycle evidence; preserve terminal kernel tail after T-End',
                 'per_tid_official': '2% or 3000ns absolute CSV/range quantization bound',
                 'raw_counters_corrected': False, 'prospective_capture_only': True}}


def require(value, message):
    if not value:
        raise ValueError(message)


def signature(value):
    return hashlib.sha256(json.dumps(value, sort_keys=True, separators=(',', ':'),
                                     ensure_ascii=False).encode()).hexdigest()


def file_sha(path):
    with Path(path).open('rb') as source:
        return hashlib.file_digest(source, 'sha256').hexdigest()


def definition_sha(policy_id):
    require(policy_id in POLICIES, 'unknown clock policy')
    return signature(DEFINITIONS[policy_id])


def clock_hardware(hardware):
    result = {name: hardware.get(name) for name in ('os', 'physical_cpu', 'logical_cpus')}
    require(all(result.values()), 'clock hardware OS/physical CPU/topology missing')
    cpus = result['physical_cpu'] if isinstance(result['physical_cpu'], list) else [result['physical_cpu']]
    require(type(result['logical_cpus']) is int and result['logical_cpus'] > 0 and
            all(isinstance(cpu, dict) and cpu.get('Name') and
                type(cpu.get('NumberOfCores')) is int and cpu['NumberOfCores'] > 0 and
                type(cpu.get('NumberOfLogicalProcessors')) is int and cpu['NumberOfLogicalProcessors'] > 0
                for cpu in cpus), 'clock physical CPU topology unknown')
    return result


def current_tools():
    root = Path(__file__).resolve().parent
    return {name: file_sha(root/name) for name in TOOLS}


def _reference(reference):
    path = _verified_path(reference)
    return json.loads(path.read_text(encoding='utf-8'))


def _verified_path(reference):
    path = Path(reference['path']).resolve(strict=True)
    require(file_sha(path) == reference['sha256'], 'clock evidence artifact SHA changed: '+str(path))
    return path


def _trace_header(path):
    from perf_trace_archive import open_trace_text
    with open_trace_text(path) as source:
        for line in source:
            match = re.search(r'Trace Start:\s*(\d+)', line)
            if match:
                lost = re.search(r'Events Lost:\s*(\d+),\s*Buffers lost:\s*(\d+)', line)
                require(lost is not None and int(lost[1]) == int(lost[2]) == 0,
                        'calibration raw trace loss counters absent/nonzero')
                return int(match[1])
    raise ValueError('calibration raw trace start timestamp missing')


def _file_input_hash(quality, name):
    values = [value for path, value in quality.get('files', {}).items() if Path(path).name == name]
    require(len(values) == 1, 'calibration analysis input SHA missing/ambiguous: '+name)
    return values[0]


def _native_cpu(sample):
    require(isinstance(sample, dict), 'calibration native OS sample missing')
    require(all(type(sample.get(k)) is int and sample[k] >= 0
                for k in ('kernel_100ns', 'user_100ns', 'cpu_ns')), 'invalid native OS counter integer')
    require(sample['cpu_ns'] == (sample['kernel_100ns']+sample['user_100ns'])*100,
            'native OS counter units differ')
    require(sample.get('kernel_ns') == sample['kernel_100ns']*100 and
            sample.get('user_ns') == sample['user_100ns']*100, 'native kernel/user units differ')
    qpc = sample.get('qpc', [])
    require(len(qpc) == 2 and all(type(q) is int and q > 0 for q in qpc) and qpc[0] <= qpc[1],
            'native OS QPC read bracket invalid')
    return sample['cpu_ns']


def _native_row(row, begin, end):
    delta = (_native_cpu(end)-_native_cpu(begin))/1e6
    require(row['begin_qpc'] == begin['qpc'] and row['end_qpc'] == end['qpc'] and
            abs(row['os_cpu_ms']-delta) < 1e-7 and
            abs(row.get('os_kernel_ms', math.inf)-(end['kernel_100ns']-begin['kernel_100ns'])/10000) < 1e-7 and
            abs(row.get('os_user_ms', math.inf)-(end['user_100ns']-begin['user_100ns'])/10000) < 1e-7,
            'calibration raw counters/window differ')
    return delta


def _validate_calibration_inputs(reference, session, capture, quality, parser_sha):
    """Rehash actual raw files and derive reference totals, not eligibility flags."""
    require(quality.get('parser_sha256') == parser_sha and Path(quality.get('parser_file', '')).name == 'perf_etw.py',
            'calibration actual analysis parser differs from declaration')
    _verified_path(reference['trace_etl'])
    csv_path = _verified_path(reference['trace_csv'])
    raw_sha = raw_trace_sha(csv_path) if csv_path.suffix.lower() == '.gz' else reference['trace_csv']['sha256']
    require(raw_sha == reference.get('trace_csv_raw_sha256') == _file_input_hash(quality, 'schedule.csv'),
            'calibration raw CSV identity differs from analyzed input')
    require(_trace_header(csv_path) == reference['trace_start_filetime'], 'calibration raw trace timestamp differs')
    native = _reference(reference['native'])
    require(reference['native']['sha256'] == _file_input_hash(quality, 'native-counters.json') ==
            capture.get('native_counters_sha256'), 'calibration native input identity differs')
    require(native.get('schema') == 'retained-thread-counters-v1' and
            native.get('process_id') == session['process_id'] and
            native.get('qpc_frequency') == session.get('qpc_frequency') == quality.get('qpc_frequency') and
            native.get('errors') == [] and native.get('reused_thread_ids') == [] and
            native.get('tracking_overflow') in (False, 0) and native.get('process_exit_code') == 0,
            'calibration native process/clock/coverage differs')
    official_path = _verified_path(reference['official'])
    official = official_csv_totals(official_path, session['process_id'], session['thread_id'])
    threads = {t['thread_id']: t for t in native['threads']}
    rows = {r['thread_id']: r for r in quality['rows']}
    require(len(threads) == len(native['threads']) and len(rows) == len(quality['rows']) and
            set(rows) == set(threads), 'calibration native/quality TID coverage differs')
    require(not {tid for tid, cpu in official.items() if cpu > 0}-set(rows), 'Microsoft active TID missing from calibration')
    outer = (native['initial']['process_before_threads'], native['final']['process_after_threads'])
    inner = (native['initial']['process_after_threads'], native['final']['process_before_threads'])
    process_os = []
    for field, (begin, end) in (('process_before_sandwich', outer), ('process_after_sandwich', inner)):
        process_os.append(_native_row(quality[field], begin, end))
    require(quality['process_os_delta_sandwich_ms'] == process_os[::-1],
            'calibration raw process sandwich bounds differ')
    require(reference.get('official_window_qpc') == [outer[0]['qpc'][0], outer[1]['qpc'][0]],
            'calibration Microsoft QPC window differs')
    for tid, row in rows.items():
        item = threads[tid]
        begin, end = item.get('baseline'), item.get('final')
        # This declared ready/go calibration precreates and retains all threads.
        _native_row(row, begin, end)
        require(end.get('signaled_before_read') is True and
                begin.get('creation_filetime') == end.get('creation_filetime') == item.get('creation_filetime') == row.get('creation_filetime'),
                'calibration retained native thread identity differs')
        actual_ms = official.get(tid, 0)/1e6
        own_ms = row['common_process_window_etw_cpu_ms']
        error = abs(actual_ms-own_ms)/max(actual_ms, own_ms, 1e-6)
        require(abs(row.get('microsoft_common_window_cpu_ms', math.inf)-actual_ms) < 1e-7 and
                abs(row.get('microsoft_common_window_relative_error', math.inf)-error) < 1e-12,
                'calibration Microsoft raw TID total/derived error differs')
    process_ms = sum(official.values())/1e6
    own_ms = sum(row['common_process_window_etw_cpu_ms'] for row in rows.values())
    error = abs(process_ms-own_ms)/max(process_ms, own_ms, 1e-6)
    summary = quality.get('microsoft_independent_process', {})
    require(abs(summary.get('cpu_ms', math.inf)-process_ms) < 1e-7 and
            abs(summary.get('relative_error', math.inf)-error) < 1e-12 and
            summary.get('within_existing_2pct') is (error <= .02) and error <= .02,
            'calibration Microsoft raw process total/derived error differs')
    workers = {w['worker']: w['thread_id'] for w in session['workers']}
    require(set(workers) == set(range(4)) and len(set(workers.values())) == 4 and
            native.get('primary_worker_tids') == [workers[i] for i in range(4)], 'calibration primary worker TID mapping differs')
    for index, tid in workers.items():
        role = 'clock-hash-'+str(index)
        require(tid in rows and rows[tid].get('description') == threads[tid].get('description') == role,
                'calibration session/native/quality worker role differs')


def _validate_raw_coarse(row):
    os_ms, etw_ms = row.get('os_cpu_ms'), row.get('etw_nominal_cpu_ms')
    require(all(type(value) in (int, float) and math.isfinite(value) and value >= 0
                for value in (os_ms, etw_ms)), 'calibration raw coarse values missing/invalid')
    error = abs(os_ms-etw_ms)/max(os_ms, etw_ms, 1e-6)
    require(abs(row.get('os_minus_etw_ms', math.inf)-(os_ms-etw_ms)) < 1e-7 and
            abs(row.get('nominal_relative_error', math.inf)-error) < 1e-12 and
            row.get('within_existing_2pct') is (error <= .02),
            'calibration raw signed discrepancy or coarse flag changed')


def load_evidence(path, hardware):
    """Recheck small immutable artifacts, preserving failed coarse flags.

    The evidence file is assembled only after the declared calibration series.
    References identify existing reports rather than blessing an eligible flag.
    """
    path = Path(path).resolve(strict=True)
    evidence = json.loads(path.read_text(encoding='utf-8'))
    require(evidence.get('schema') == 'windows-clock-policy-evidence-v1', 'unknown clock evidence schema')
    require(evidence.get('policy_id') == CALIBRATED and
            evidence.get('definition_sha256') == definition_sha(CALIBRATED), 'clock evidence policy definition differs')
    require(evidence.get('tools') == current_tools(), 'clock evidence measurement tools changed')
    hardware_identity = clock_hardware(hardware)
    require(evidence.get('hardware') == hardware_identity, 'clock evidence hardware/OS differs')
    frozen = datetime.datetime.fromisoformat(evidence['frozen_at_utc'])
    require(frozen.tzinfo is not None, 'clock evidence freeze timestamp requires timezone')
    declaration = _reference(evidence['predeclared'])
    require(declaration.get('schema') == 'predeclared-cpu-clock-calibration-v1', 'unknown calibration declaration')
    require(evidence.get('calibration_identities') == declaration['identities'],
            'frozen calibration helper identity differs from declaration')
    parser_ids = [digest for name, digest in declaration['identities'].items() if Path(name).name == 'perf_etw.py']
    require(parser_ids == [current_tools()['perf_etw.py']],
            'CPU reconstruction parser differs from declared calibration; recalibration required')
    declared = datetime.datetime.fromisoformat(declaration['declared_at_utc'])
    require(declared.tzinfo is not None and declared < frozen, 'calibration must be declared before evidence freeze')
    declared_filetime = round(declared.timestamp()*1e7)+116444736000000000
    frozen_filetime = round(frozen.timestamp()*1e7)+116444736000000000
    require(declaration.get('pairs') == 3 and declaration.get('order') ==
            [['continuous', 'burst'], ['burst', 'continuous'], ['continuous', 'burst']],
            'calibration must use predeclared three alternating pairs')
    expected = declaration['workload']
    require(declaration.get('gate_percent') == 2 and declaration.get('primary_worker_roles') ==
            ['clock-hash-'+str(i) for i in range(4)], 'calibration role/gate declaration differs')
    require(expected.get('workers') == 4 and expected.get('chunks_per_worker') == 4096 and
            expected.get('chunk_bytes') == 1048576, 'calibration must use declared four-worker long workload')
    require(signature(expected) == declaration['workload_sha256'], 'declared workload SHA differs')
    require(declaration.get('total_cpu_work_identical_claimed') is False, 'calibration must retain real burst waiting cost')
    references = evidence.get('runs', [])
    require(len(references) == 6, 'six distinct calibration runs required')
    require(len({r['session']['path'] for r in references}) == 6, 'calibration run repeated')
    patterns = [item for pair in declaration['order'] for item in pair]
    checksum = None
    previous_trace_start = declared_filetime
    for reference, pattern in zip(references, patterns):
        require(declared_filetime < reference.get('trace_start_filetime', 0) < frozen_filetime,
                'calibration ETW does not postdate declaration and predate freeze')
        require(reference['trace_start_filetime'] > previous_trace_start, 'calibration chronological order differs')
        previous_trace_start = reference['trace_start_filetime']
        session, capture, quality = (_reference(reference[name]) for name in ('session', 'capture', 'quality'))
        require(session.get('schema') == 'synthetic-thread-clock-v1' and session.get('valid') is True,
                'invalid calibration session')
        require(session.get('pattern') == pattern, 'calibration order differs from declaration')
        require(session.get('burst_wait_ms') == (declaration['burst_wait_ms'] if pattern == 'burst' else 0),
                'calibration wait schedule differs from declaration')
        require(session.get('workload') == expected and session.get('workload_sha256') == declaration['workload_sha256'],
                'calibration hash workload differs')
        workers = session.get('workers', [])
        require(len(workers) == 4, 'calibration worker count differs')
        for worker in workers:
            require(worker.get('chunks') == 4096 and worker.get('bytes') == 4294967296,
                    'calibration worker hash work differs')
            value = worker.get('sha256')
            require(isinstance(value, str) and len(value) == 64, 'calibration final checksum missing')
            checksum = value if checksum is None else checksum
            require(value == checksum, 'calibration final checksums differ')
        require(capture.get('valid_capture') is True and capture.get('exit_code') == 0 and
                capture.get('etw_stop_exit_code') == 0, 'calibration capture incomplete')
        require(capture.get('fixture_session_sha256') == reference['session']['sha256'] and
                capture.get('process_id') == session.get('process_id') == quality.get('process_id'),
                'calibration process/session identity differs')
        require(clock_hardware(capture['hardware']) == hardware_identity, 'calibration capture hardware differs')
        for name, digest in declaration['identities'].items():
            require(capture.get('helpers', {}).get(name) == digest, 'calibration helper differs from declaration')
        require(quality.get('coverage_complete') is True and
                quality.get('thread_sum_within_process_sandwich') is True, 'calibration thread coverage/conservation incomplete')
        require(quality.get('schema') == 'external-thread-cpu-diagnostic-v1', 'unknown calibration quality schema')
        _validate_calibration_inputs(reference, session, capture, quality, parser_ids[0])
        for coarse in [quality['process_before_sandwich'], quality['process_after_sandwich'], *quality['rows']]:
            _validate_raw_coarse(coarse)
        total_os = sum(row['os_cpu_ms'] for row in quality['rows'])
        require(abs(total_os-quality['thread_os_cpu_sum_ms']) < 1e-7 and
                quality['process_os_delta_sandwich_ms'][0] <= total_os <= quality['process_os_delta_sandwich_ms'][1],
                'calibration raw thread/process counter sum differs')
        for name in ('untracked_running_threads', 'unresolved_thread_baselines', 'reused_thread_ids',
                     'etw_reused_thread_ids', 'native_errors'):
            require(quality.get(name) == [], 'calibration lifecycle/native errors: '+name)
        official = quality.get('microsoft_independent_process', {})
        require(official.get('input_trace_shared') is True and
                isinstance(official.get('relative_error'), (int, float)) and
                0 <= official['relative_error'] <= .02, 'calibration Microsoft process crosscheck failed')
        roles = {row.get('description'): row for row in quality['rows']}
        for role in declaration['primary_worker_roles']:
            row = roles.get(role, {})
            error = row.get('microsoft_common_window_relative_error')
            require(isinstance(error, (int, float)) and 0 <= error <= .02,
                    'calibration Microsoft worker crosscheck missing/failed')
        # These are diagnostics: do not require or rewrite coarse passed flags.
        require(quality.get('acceptance_gate_changed') is False and quality.get('diagnostic_only') is True,
                'calibration report scope differs')
    return {'id': CALIBRATED, 'definition_sha256': definition_sha(CALIBRATED),
            'calibration_sha256': file_sha(path), 'hardware_sha256': signature(hardware_identity)}, evidence


def prepare_capture(policy_id, evidence_path, hardware, directory, measurement_tools):
    require(policy_id in POLICIES, 'unknown clock policy')
    if policy_id == LEGACY:
        require(evidence_path is None, 'legacy clock policy must not attach new calibration evidence')
        return {'id': LEGACY, 'definition_sha256': definition_sha(LEGACY)}, None
    require(evidence_path is not None, 'calibrated clock policy requires frozen evidence')
    key, evidence = load_evidence(evidence_path, hardware)
    # Written exclusively before the child is started, not minted by account.
    created_filetime = time.time_ns()//100 + 116444736000000000
    require(datetime.datetime.fromisoformat(evidence['frozen_at_utc']).timestamp()*1e7 +
            116444736000000000 <= created_filetime, 'clock evidence is not frozen before capture')
    directory = Path(directory).resolve(strict=True)
    intent = {'schema': 'prospective-clock-capture-intent-v1', 'clock_policy': key,
              'run_directory': str(directory), 'session_path': str(directory/'session.json'),
              'calibration_path': str(Path(evidence_path).resolve()),
              'calibration_identities': evidence['calibration_identities'],
              'measurement_tools': measurement_tools, 'hardware': clock_hardware(hardware),
              'nonce': uuid.uuid4().hex, 'created_filetime': created_filetime}
    path = directory/'clock-policy-intent.json'
    with path.open('x', encoding='utf-8', newline='\n') as out:
        out.write(json.dumps(intent, indent=2)+'\n')
    return key, {'path': str(path), 'sha256': file_sha(path)}


def verify_capture_context(record, report, raw_trace_sha256):
    """Old raw sessions cannot acquire the new policy through account CLI."""
    key = record.get('clock_policy', {'id': LEGACY})
    if key['id'] == LEGACY:
        return key, None
    require(key['id'] == CALIBRATED, 'unknown capture clock policy')
    reference = record.get('clock_policy_intent')
    require(reference is not None, 'new clock policy requires prospective capture intent')
    intent = _reference(reference)
    require(intent.get('schema') == 'prospective-clock-capture-intent-v1', 'unknown clock capture intent')
    require(intent.get('clock_policy') == key and intent.get('nonce'), 'clock capture intent policy differs')
    require(record.get('measurement_tools') == intent['measurement_tools'] and
            intent['measurement_tools'] == current_tools(), 'clock capture tool identity differs')
    hardware = record.get('hardware', {})
    require(intent.get('hardware') == clock_hardware(hardware), 'prospective clock hardware differs')
    actual, evidence = load_evidence(intent['calibration_path'], hardware)
    require(actual == key, 'clock evidence identity differs from prospective capture')
    require(intent.get('calibration_identities') == evidence['calibration_identities'],
            'prospective calibration helper identity differs')
    session_path = Path(record['session_path']).resolve(strict=True)
    require(str(session_path) == intent['session_path'] and
            str(session_path.parent) == intent['run_directory'], 'clock capture paths differ')
    require(file_sha(session_path) == record.get('session_sha256') and
            json.loads(session_path.read_text(encoding='utf-8')) == report, 'clock capture session identity differs')
    require(raw_trace_sha256 == record.get('trace_csv_sha256'), 'clock capture raw trace identity differs')
    return key, intent


def policy_pair_identity(run):
    legacy = {'id': LEGACY, 'definition_sha256': definition_sha(LEGACY)}
    key = run.get('clock_policy', legacy)
    account_key = run.get('account', {}).get('clock_policy', legacy)
    require(key == account_key, 'run/account clock policy differs')
    return key


def raw_trace_sha(path):
    path = Path(path)
    opener = gzip.open if path.suffix.lower() == '.gz' else open
    with opener(path, 'rb') as source:
        return hashlib.file_digest(source, 'sha256').hexdigest()


def trace_metadata(path, process_id):
    """Lifecycle side scan; deliberately leaves calibrated CPU parser unchanged."""
    from perf_trace_archive import open_trace_text
    metadata = {'thread_lifecycle': {}, 'raw_scheduled_in': {}}
    headers, raw_time = {}, None
    tags = {'T-Start', 'T-End', 'T-DCStart', 'T-DCEnd'}
    with open_trace_text(path) as source:
        for line in source:
            if line.startswith('#'):
                raw = re.match(r'^#\s+\d+\s+#\s+(0x[0-9a-fA-F]+)', line)
                if raw:
                    raw_time = int(raw[1], 16)
                continue
            if 'Trace Start:' in line:
                value = re.search(r'Trace Start:\s*(\d+)', line)
                require(value is not None, 'ETW trace start timestamp malformed')
                metadata['trace_start_filetime'] = int(value[1])
            event = line.lstrip().split(',', 1)[0]
            if event not in tags and event != 'CSwitch':
                continue
            row = [cell.strip() for cell in next(csv.reader([line], skipinitialspace=True))]
            if row[1] == 'TimeStamp':
                headers[event] = {name: i for i, name in enumerate(row)}
                continue
            require(event in headers and raw_time is not None, 'ETW lifecycle header/QPC missing')
            fields = headers[event]
            if event == 'CSwitch':
                match = re.search(r'\(\s*(\d+)\s*\)\s*$', row[fields['New Process Name ( PID)']])
                if match and int(match[1]) == process_id:
                    tid = int(row[fields['New TID']])
                    metadata['raw_scheduled_in'].setdefault(tid, []).append(raw_time)
                continue
            match = re.search(r'\(\s*(\d+)\s*\)\s*$', row[fields['Process Name ( PID)']])
            if match and int(match[1]) == process_id:
                tid = int(row[fields['ThreadID']])
                metadata['thread_lifecycle'].setdefault(tid, {}).setdefault(event, []).append(raw_time)
    return metadata


def official_csv_totals(path, process_id, owner_id):
    totals = {}
    with Path(path).open(encoding='utf-8-sig', newline='') as source:
        for row in csv.reader(source, skipinitialspace=True):
            if len(row) != 3 or not row[0].strip().isdigit() or not row[2].strip().isdigit():
                continue
            match = re.search(r'\(\s*(\d+)\s*\)\s*$', row[1])
            if not match or int(match[1]) != process_id:
                continue
            thread = int(row[2])
            require(thread not in totals, 'ambiguous duplicate Microsoft TID')
            totals[thread] = int(row[0])*1000
    require(owner_id in totals and totals[owner_id] > 0, 'Microsoft owner CPU missing')
    return totals


def official_capture_totals(report, directory, trace_sha256):
    directory = Path(directory)
    path = directory/'official-owner.csv'
    by_tid = official_csv_totals(path, report['process_id'], report['thread_id'])
    return {'schema': 'microsoft-cswitch-pid-totals-v1',
            'process_id': report['process_id'], 'thread_id': report['thread_id'],
            'qpc_frequency': report['qpc_frequency'],
            'window_qpc': [report['envelope']['begin'][0], report['envelope']['end'][0]],
            'cpu_by_tid_ns': {str(tid): value for tid, value in by_tid.items()},
            'owner_cpu_ns': by_tid[report['thread_id']], 'process_cpu_ns': sum(by_tid.values()),
            'output': {'path': str(path.resolve()), 'sha256': file_sha(path)},
            'trace_csv_sha256': trace_sha256,
            'trace_etl_sha256': file_sha(directory/'schedule.etl'),
            'shared_event_stream': True,
            'source': 'Microsoft xperf CSwitch -util -thread -exc_dpcisr, same QPC range'}


def calibrated_quality(report, timelines, metadata, record, official):
    key, intent = verify_capture_context(record, report, record.get('_verified_raw_trace_sha256'))
    require(key['id'] == CALIBRATED, 'calibrated quality requires new policy capture')
    require(metadata.get('trace_start_filetime', 0) > intent['created_filetime'],
            'ETW predates prospective capture intent; old runs cannot be reclassified')
    require(official is not None and official.get('schema') == 'microsoft-cswitch-pid-totals-v1',
            'calibrated clock policy requires complete Microsoft PID totals')
    start, stop = report['envelope']['begin'][0], report['envelope']['end'][0]
    require(official.get('process_id') == report['process_id'] and
            official.get('thread_id') == report['thread_id'] and
            official.get('qpc_frequency') == report['qpc_frequency'] and
            official.get('window_qpc') == [start, stop], 'Microsoft process/window identity differs')
    require(official.get('shared_event_stream') is True, 'Microsoft reference must declare shared event stream')
    require(official.get('trace_csv_sha256') == record['trace_csv_sha256'], 'Microsoft raw trace identity differs')
    require(official.get('trace_etl_sha256') == record.get('trace_etl_sha256'), 'Microsoft ETL identity differs')
    require(file_sha(Path(record['session_path']).parent/'schedule.etl') == official['trace_etl_sha256'],
            'retained ETL identity changed')
    path = official['output']['path']
    require(file_sha(path) == official['output']['sha256'], 'Microsoft CSV identity changed')
    by_tid = official_csv_totals(path, report['process_id'], report['thread_id'])
    require({str(k): v for k, v in by_tid.items()} == official.get('cpu_by_tid_ns') and
            sum(by_tid.values()) == official.get('process_cpu_ns') and
            by_tid[report['thread_id']] == official.get('owner_cpu_ns'), 'Microsoft PID sum conservation failed')
    factor = 1e9/report['qpc_frequency']
    active = {tid: timeline.between(start, stop)*factor for tid, timeline in timelines.items()
              if timeline.between(start, stop) > 0}
    lifecycle = metadata.get('thread_lifecycle', {})
    lifecycle_rows = []
    issues = []
    for tid, cpu in active.items():
        events = lifecycle.get(tid, {})
        origins = events.get('T-Start', [])+events.get('T-DCStart', [])
        endings = events.get('T-End', [])+events.get('T-DCEnd', [])
        spans = [(max(a, start), min(b, stop)) for a, b in timelines[tid].intervals if b > start and a < stop]
        require(len(origins) == 1 and origins[0] <= min(a for a, _ in spans),
                'missing/ambiguous lifecycle origin for active TID '+str(tid))
        raw_spans = [(a, b) for a, b in timelines[tid].intervals if b > start and a < stop]
        # T-End can be emitted before a thread's final kernel cleanup switches
        # out. Keep that tail's CPU; reject any later scheduling-in generation.
        scheduled_in = [qpc for qpc in metadata.get('raw_scheduled_in', {}).get(tid, []) if qpc < stop]
        require(scheduled_in and len(endings) == 1 and endings[0] >= max(scheduled_in),
                'missing/ambiguous lifecycle completion for active TID '+str(tid))
        tail = sum(max(0, min(b, stop)-max(a, start, endings[0])) for a, b in raw_spans)*factor
        lifecycle_rows.append({'thread_id': tid, 'origin_qpc': origins[0], 'final_qpc': endings[0],
                               'terminal_tail_cpu_ns': tail, 'cpu_ns': cpu})
    per_tid = []
    for tid in sorted(set(active)|set(by_tid)):
        own, reference = active.get(tid, 0), by_tid.get(tid, 0)
        delta = abs(own-reference)
        # One thread runs on at most one CPU. Each range endpoint expands by
        # <1us, with <1us integer-us output quantization: total bound 3000ns.
        permitted = max(.02*max(own, reference), 3000)
        passed = delta <= permitted
        per_tid.append({'thread_id': tid, 'etw_cpu_ns': own, 'microsoft_cpu_ns': reference,
                        'microsoft_tid_present': tid in by_tid,
                        'absolute_difference_ns': delta, 'quantization_bound_ns': 3000,
                        'within_2pct_or_quantization': passed})
        if not passed:
            issues.append('Microsoft per-TID attribution differs: '+str(tid))
    return key, {'reference': official['source'], 'shared_event_stream': True,
                 'independent_implementation': True, 'independent_clock': False,
                 'calibration_helpers': intent['calibration_identities'],
                 'measurement_tools': record['measurement_tools'],
                 'cpu_parser_identical_to_calibration': True,
                 'process_sum_tid_cpu_ns': sum(active.values()),
                 'lifecycle_complete': True, 'lifecycle': lifecycle_rows, 'per_tid': per_tid}, issues


def freeze_evidence(output, predeclared, run_directories):
    """Explicit post-calibration action; never invoked by capture/account."""
    from perf_trace_archive import open_trace_text
    require(len(run_directories) == 6, 'exactly six calibration run directories required')
    def reference(path):
        path = Path(path).resolve(strict=True)
        return {'path': str(path), 'sha256': file_sha(path)}
    captures = [json.loads((Path(path)/'capture.json').read_text(encoding='utf-8')) for path in run_directories]
    declaration = json.loads(Path(predeclared).read_text(encoding='utf-8'))
    evidence = {'schema': 'windows-clock-policy-evidence-v1', 'policy_id': CALIBRATED,
                'definition_sha256': definition_sha(CALIBRATED), 'tools': current_tools(),
                'hardware': clock_hardware(captures[0]['hardware']),
                'calibration_identities': declaration['identities'],
                'predeclared': reference(predeclared),
                'frozen_at_utc': datetime.datetime.now(datetime.timezone.utc).isoformat(), 'runs': []}
    for directory in map(Path, run_directories):
        raw = directory/'schedule.csv'
        if not raw.is_file():
            raw = directory/'schedule.csv.gz'
        trace_start = None
        with open_trace_text(raw) as source:
            for line in source:
                match = re.search(r'Trace Start:\s*(\d+)', line)
                if match:
                    trace_start = int(match[1])
                    break
        require(trace_start is not None, 'calibration raw trace start timestamp missing')
        quality = json.loads((directory/'thread-cpu.json').read_text(encoding='utf-8'))
        csv_hashes = [value for name, value in quality.get('files', {}).items()
                      if Path(name).name == 'schedule.csv']
        raw_sha = raw_trace_sha(raw)
        require(csv_hashes == [raw_sha], 'calibration trace SHA differs from analyzed raw input')
        evidence['runs'].append({'session': reference(directory/'session.json'),
                                 'capture': reference(directory/'capture.json'),
                                 'quality': reference(directory/'thread-cpu.json'),
                                 'native': reference(directory/'native-counters.json'),
                                 'official': reference(directory/'official-owner.csv'),
                                 'trace_csv': reference(raw),
                                 'official_window_qpc': [
                                     json.loads((directory/'native-counters.json').read_text(encoding='utf-8'))['initial']['process_before_threads']['qpc'][0],
                                     json.loads((directory/'native-counters.json').read_text(encoding='utf-8'))['final']['process_after_threads']['qpc'][0]],
                                 'trace_start_filetime': trace_start, 'trace_csv_raw_sha256': raw_sha,
                                 'trace_etl': reference(directory/'schedule.etl')})
    output = Path(output)
    # Exclusive creation never replaces a previously frozen policy/evidence.
    with output.open('x', encoding='utf-8', newline='\n') as out:
        out.write(json.dumps(evidence, indent=2)+'\n')
    try:
        load_evidence(output, captures[0]['hardware'])
    except BaseException:
        output.unlink()
        raise
    return {'path': str(output.resolve()), 'sha256': file_sha(output)}
