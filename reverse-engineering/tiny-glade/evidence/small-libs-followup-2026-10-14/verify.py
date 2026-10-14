"""Run focused checks against the development crates; retain source bindings."""
from pathlib import Path
import hashlib
import json
import os
import re
import subprocess
import time

REPO = Path(__file__).resolve().parents[4]
WORKSPACE = REPO / 'tiny-glade'
OUTPUT = Path(__file__).resolve().parent
PACKAGES = ['tg-package-integrity', 'tg-text']

def digest(path):
    return hashlib.sha256(path.read_bytes()).hexdigest()

def source_binding():
    result = {}
    for package in PACKAGES:
        for path in sorted((WORKSPACE / 'crates' / package).rglob('*')):
            if path.is_file() and 'target' not in path.parts:
                result[path.relative_to(WORKSPACE).as_posix()] = digest(path)
    for name in ['Cargo.toml', 'Cargo.lock', 'provenance/original-Cargo.lock']:
        result[name] = digest(WORKSPACE / name)
    return result

env = os.environ.copy()
msvc = Path('C:/Program Files (x86)/Microsoft Visual Studio/2022/BuildTools/VC/Tools/MSVC/14.38.33130')
sdk = Path('C:/Program Files (x86)/Windows Kits/10')
env['PATH'] = str(msvc / 'bin/Hostx64/x64') + ';' + str(sdk / 'bin/10.0.22621.0/x64') + ';' + env['PATH']
env['LIB'] = ';'.join(str(p) for p in [msvc / 'lib/x64', sdk / 'Lib/10.0.22621.0/um/x64', sdk / 'Lib/10.0.22621.0/ucrt/x64'])
env['INCLUDE'] = ';'.join(str(p) for p in [msvc / 'include', sdk / 'Include/10.0.22621.0/ucrt', sdk / 'Include/10.0.22621.0/um', sdk / 'Include/10.0.22621.0/shared'])
cargo = str(Path('C:/Users/liyu/.cargo/bin/cargo.exe'))
commands = [
    ('test', [cargo, 'test', '--offline', '--locked', '-p', PACKAGES[0], '-p', PACKAGES[1], '--all-targets']),
    ('clippy', [cargo, 'clippy', '--offline', '--locked', '-p', PACKAGES[0], '-p', PACKAGES[1], '--all-targets', '--', '-D', 'warnings']),
    ('gpu-check', [cargo, 'check', '--offline', '--locked', '-p', 'tg-text', '--features', 'gpu', '--all-targets']),
    ('fmt', [cargo, 'fmt', '-p', PACKAGES[0], '-p', PACKAGES[1], '--check']),
]
report = {'baseline': 'B', 'platform': 'Windows x86_64', 'workspace': str(WORKSPACE),
          'checks': [], 'before': source_binding(), 'whole_original_behavior_accepted': False}
for name, command in commands:
    start = time.monotonic()
    log = OUTPUT / (name + '.log')
    with log.open('wb') as stream:
        process = subprocess.run(command, cwd=WORKSPACE, env=env, stdout=stream, stderr=subprocess.STDOUT)
    output = log.read_text(encoding='utf-8', errors='replace')
    counts = [tuple(map(int, m)) for m in re.findall(r'test result: \w+\. (\d+) passed; (\d+) failed; (\d+) ignored;', output)]
    row = {'name': name, 'command': command, 'exit_code': process.returncode,
           'elapsed_seconds': round(time.monotonic() - start, 3), 'log': log.name,
           'log_sha256': digest(log)}
    if counts:
        row.update(dict(zip(['passed', 'failed', 'ignored'], [sum(c[i] for c in counts) for i in range(3)])))
    report['checks'].append(row)
    (OUTPUT / 'verification.json').write_text(json.dumps(report, ensure_ascii=False, indent=2) + '\n', encoding='utf-8')
    print(json.dumps(row, ensure_ascii=False), flush=True)
report['after'] = source_binding()
report['source_unchanged_during_checks'] = report['before'] == report['after']
report['ok'] = all(row['exit_code'] == 0 for row in report['checks']) and report['source_unchanged_during_checks']
(OUTPUT / 'verification.json').write_text(json.dumps(report, ensure_ascii=False, indent=2) + '\n', encoding='utf-8')
raise SystemExit(0 if report['ok'] else 1)
