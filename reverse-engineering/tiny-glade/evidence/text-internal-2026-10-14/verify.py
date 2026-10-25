"""Source-bound checks for the recovered tg-text CPU internals."""
from pathlib import Path
import hashlib
import json
import os
import re
import subprocess
import time

OUT = Path(__file__).resolve().parent
WORKSPACE = OUT.parents[3] / 'tiny-glade'
CARGO = 'C:/Users/liyu/.cargo/bin/cargo.exe'
def sha(path):
    return hashlib.sha256(path.read_bytes()).hexdigest()
def binding():
    files = [p for p in (WORKSPACE / 'crates/tg-text').rglob('*') if p.is_file() and 'target' not in p.parts]
    files += [WORKSPACE / 'Cargo.toml', WORKSPACE / 'Cargo.lock']
    return {p.relative_to(WORKSPACE).as_posix(): sha(p) for p in sorted(files)}
env = os.environ.copy()
msvc = Path('C:/Program Files (x86)/Microsoft Visual Studio/2022/BuildTools/VC/Tools/MSVC/14.38.33130')
sdk = Path('C:/Program Files (x86)/Windows Kits/10')
env['PATH'] = ';'.join(str(p) for p in [msvc / 'bin/Hostx64/x64', sdk / 'bin/10.0.22621.0/x64']) + ';' + env['PATH']
env['LIB'] = ';'.join(str(p) for p in [msvc / 'lib/x64', sdk / 'Lib/10.0.22621.0/um/x64', sdk / 'Lib/10.0.22621.0/ucrt/x64'])
env['INCLUDE'] = ';'.join(str(p) for p in [msvc / 'include', sdk / 'Include/10.0.22621.0/ucrt', sdk / 'Include/10.0.22621.0/um', sdk / 'Include/10.0.22621.0/shared'])
commands = [
    ('test', ['test', '--offline', '--locked', '-p', 'tg-text', '--all-targets']),
    ('release-test', ['test', '--offline', '--locked', '-p', 'tg-text', '--all-targets', '--release']),
    ('clippy', ['clippy', '--offline', '--locked', '-p', 'tg-text', '--all-targets', '--', '-D', 'warnings']),
    ('gpu-api-check', ['check', '--offline', '--locked', '-p', 'tg-text', '--features', 'gpu', '--all-targets']),
    ('fmt', ['fmt', '-p', 'tg-text', '--check']),
]
report = {'baseline': 'B Windows x86_64', 'before': binding(), 'checks': [], 'gpu_executed': False, 'game_callers_changed': False}
for name, arguments in commands:
    start = time.monotonic()
    path = OUT / (name + '.log')
    with path.open('wb') as log:
        process = subprocess.run([CARGO, *arguments], cwd=WORKSPACE, env=env, stdout=log, stderr=subprocess.STDOUT)
    output = path.read_text(encoding='utf-8', errors='replace')
    row = {'name': name, 'command': [CARGO, *arguments], 'exit_code': process.returncode,
           'elapsed_seconds': round(time.monotonic() - start, 3), 'log': path.name, 'log_sha256': sha(path)}
    counts = [tuple(map(int, c)) for c in re.findall(r'test result: \w+\. (\d+) passed; (\d+) failed; (\d+) ignored;', output)]
    if counts:
        row.update(dict(zip(['passed', 'failed', 'ignored'], [sum(c[i] for c in counts) for i in range(3)])))
    report['checks'].append(row)
    (OUT / 'verification.json').write_text(json.dumps(report, ensure_ascii=False, indent=2) + '\n', encoding='utf-8')
    print(json.dumps(row), flush=True)
report['after'] = binding()
report['source_unchanged_during_checks'] = report['before'] == report['after']
report['ok'] = report['source_unchanged_during_checks'] and all(c['exit_code'] == 0 for c in report['checks'])
(OUT / 'verification.json').write_text(json.dumps(report, ensure_ascii=False, indent=2) + '\n', encoding='utf-8')
raise SystemExit(0 if report['ok'] else 1)
