"""Declare a new CPU-only clock calibration; this never starts measurement.

Run the printed synthetic-pair command separately, then freeze its six new
captures. Existing declarations, historical helpers and bundles are immutable.
"""
import argparse
import datetime
import hashlib
import json
import os
from pathlib import Path
import platform
import subprocess
import sys
import uuid

import thread_cpu_diagnostic as runner
from synthetic_sha256 import input_bytes
if Path(runner.__file__).resolve().parent != Path(__file__).resolve().parent:
    raise RuntimeError('thread_cpu_diagnostic was imported from a different calibration package')

SETTINGS = {'pairs': 3, 'workers': 4, 'chunks': 4096, 'chunk_bytes': 1048576,
            'burst_wait_ms': 10, 'poll_ms': 500, 'max_threads': 4096, 'timeout': 7200}


def require(value, message):
    if not value:
        raise ValueError(message)


def signature(value):
    return hashlib.sha256(json.dumps(value, sort_keys=True, separators=(',', ':')).encode()).hexdigest()


def source_state(identities):
    # Read-only Git calls cannot inherit a redirect to another checkout.
    env = {key: value for key, value in os.environ.items() if not key.startswith('GIT_')}
    env['GIT_TERMINAL_PROMPT'] = '0'
    prefix = ['git', '-c', 'safe.directory='+runner.perf.ROOT.as_posix(), '-C', str(runner.perf.ROOT)]
    try:
        commit = subprocess.check_output(prefix+['rev-parse', '--verify', 'HEAD'], env=env,
                                         text=True, stderr=subprocess.DEVNULL).strip()
        status = subprocess.check_output(prefix+['status', '--porcelain'], env=env,
                                         text=True, stderr=subprocess.DEVNULL)
        git = {'available': True, 'commit': commit, 'dirty': bool(status), 'status': status}
    except (OSError, subprocess.CalledProcessError):
        git = {'available': False, 'commit': None, 'dirty': None, 'status': None}
    return {'git': git, 'runtime_content_sha256': signature(identities),
            'content_scope': 'all twelve local calibration/runtime modules; not whole-repository contents'}


def metadata(nonce):
    if os.name != 'nt':
        raise RuntimeError('Windows-only CNG clock calibration')
    import winreg
    with winreg.OpenKey(winreg.HKEY_LOCAL_MACHINE, r'SOFTWARE\Microsoft\Cryptography') as key:
        machine = winreg.QueryValueEx(key, 'MachineGuid')[0]
    cpu = subprocess.check_output(['powershell', '-NoProfile', '-NonInteractive', '-Command',
        "$ErrorActionPreference='Stop'; Get-CimInstance Win32_Processor | Select-Object Name,NumberOfCores,NumberOfLogicalProcessors | ConvertTo-Json -Compress"],
        text=True, stderr=subprocess.STDOUT)
    ids = runner.identities()
    return {'identities': ids, 'source': source_state(ids),
            'hardware': {'os': platform.platform(), 'physical_cpu': json.loads(cpu), 'logical_cpus': os.cpu_count()},
            'host_binding': {'method': 'nonce-salted Windows MachineGuid and hostname SHA256; not physical attestation',
                             'sha256': signature([nonce, machine, platform.node()])},
            'python': {'path': str(Path(sys.executable).resolve()), 'sha256': runner.perf.digest(Path(sys.executable)),
                       'version': sys.version},
            'xperf': {'path': str(runner.perf.XPERF.resolve()), 'sha256': runner.perf.digest(runner.perf.XPERF)}}


def create(series, output=None):
    series = runner.perf.label(series)
    output = Path(output or runner.perf.PERF/'thread-cpu-calibrations'/(series+'.predeclared.json')).resolve()
    nonce = uuid.uuid4().hex
    current = metadata(nonce)
    # Only one input chunk is hashed; no CNG context, worker or 4 GiB hash run.
    work = {'algorithm': 'Windows CNG SHA256', 'chunk_bytes': SETTINGS['chunk_bytes'],
            'chunks_per_worker': SETTINGS['chunks'], 'workers': SETTINGS['workers'],
            'bytes_per_worker': 4294967296, 'total_bytes': 17179869184,
            'input_chunk_sha256': hashlib.sha256(input_bytes(SETTINGS['chunk_bytes'])).hexdigest()}
    command = [sys.executable, str(Path(runner.__file__).resolve()), 'synthetic-pair', series,
               '--predeclared', str(output)]
    for name, value in SETTINGS.items():
        command.extend(['--'+name.replace('_', '-'), str(value)])
    declaration = {'schema': 'predeclared-cpu-clock-calibration-v1',
        'producer': 'repository-portable-thread-clock-v1', 'declaration_nonce': nonce,
        'declared_at_utc': datetime.datetime.now(datetime.timezone.utc).isoformat(),
        'status': 'prepared_not_started', 'series': series, 'diagnostic_only': True,
        'acceptance_gate_changed': False, 'command_argv': command,
        'environment': {'PYTHONDONTWRITEBYTECODE': '1'}, 'settings': SETTINGS,
        'pairs': 3, 'order': [['continuous', 'burst'], ['burst', 'continuous'], ['continuous', 'burst']],
        'workload': work, 'workload_sha256': signature(work), 'native_hash_work_identical': True,
        'total_cpu_work_identical_claimed': False,
        'waiting_cost': 'Burst adds real WaitForSingleObject/Python/scheduling CPU; never subtract or call total work equal.',
        'burst_wait_ms': 10, 'gate_percent': 2, 'final_render_p95_ms': 16,
        'primary_worker_roles': ['clock-hash-'+str(i) for i in range(4)],
        'settings_unchanged': ['timer resolution', 'priority', 'affinity', 'driver', 'allocator'],
        **current}
    output.parent.mkdir(parents=True, exist_ok=True)
    with output.open('x', encoding='utf-8', newline='\n') as destination:
        destination.write(json.dumps(declaration, indent=2)+'\n')
    return {'path': str(output), 'sha256': runner.perf.digest(output), 'command_argv': command,
            'measurement_started': False}


def validate_series(args):
    path = Path(args.predeclared).resolve(strict=True)
    declaration = json.loads(path.read_text(encoding='utf-8'))
    require(declaration.get('producer') == 'repository-portable-thread-clock-v1' and
            declaration.get('schema') == 'predeclared-cpu-clock-calibration-v1',
            'create a new portable declaration; historical declarations/bundles cannot start this series')
    require(declaration.get('series') == runner.perf.label(args.series), 'new calibration series differs from declaration')
    require(declaration.get('settings') == SETTINGS and
            all(getattr(args, name) == value for name, value in SETTINGS.items()), 'calibration settings differ from declaration')
    nonce = declaration.get('declaration_nonce')
    require(isinstance(nonce, str) and len(nonce) == 32, 'declaration nonce missing')
    current = metadata(nonce)
    for name, value in current.items():
        require(declaration.get(name) == value, 'calibration host/source/tool identity differs: '+name)
    return {'path': str(path), 'sha256': runner.perf.digest(path), 'nonce': nonce,
            'metadata_sha256': signature(current)}


def verify_capture_binding(args):
    require(getattr(args, 'declaration_binding', None) is not None,
            'synthetic capture requires a newly validated portable series declaration')
    require(validate_series(args) == args.declaration_binding, 'declaration changed since series start')


if __name__ == '__main__':
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument('series')
    parser.add_argument('--output', type=Path)
    args = parser.parse_args()
    print(json.dumps(create(args.series, args.output), indent=2))
