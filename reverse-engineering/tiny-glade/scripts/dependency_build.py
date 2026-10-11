"""Build verification runners against Cargo's exact original dependency pins.

This links published dependencies instead of compiling handwritten substitutes.
The runner remains a disposable executable with no game runtime/scene work.
"""
from pathlib import Path
import json, subprocess, shutil

ROOT = Path(__file__).resolve().parents[1]
CARGO = shutil.which('cargo')
RUSTC = shutil.which('rustc')
if not CARGO or not RUSTC:
    raise RuntimeError('cargo and rustc must be available in PATH')
_dependencies = None

def runner_command(source, output, edition='2024'):
    global _dependencies
    if _dependencies is None:
        result = subprocess.run([CARGO, 'build', '--release', '--locked', '--manifest-path',
            str(ROOT/'reconstruction/Cargo.toml'), '--lib', '--message-format=json'],
            capture_output=True, text=True, check=True)
        _dependencies = {}
        for line in result.stdout.splitlines():
            record = json.loads(line)
            if record.get('reason') != 'compiler-artifact':
                continue
            name = record['target']['name']
            if name in ['fastrand', 'glam', 'half']:
                _dependencies[name] = next(p for p in record['filenames'] if p.endswith('.rlib'))
        assert set(_dependencies) == {'fastrand','glam','half'}, _dependencies
    args = [RUSTC, '--edition='+edition, '--cfg', 'feature="library"', '-Awarnings', '-O',
        str(source), '-o', str(output)]
    dep_path = Path(_dependencies['half']).parent
    args += ['-L', 'dependency='+str(dep_path)]
    for name, path in _dependencies.items():
        args += ['--extern', name+'='+path]
    return args

def build_runner(source, output, edition='2024'):
    Path(output).parent.mkdir(parents=True, exist_ok=True)
    return subprocess.run(runner_command(source, output, edition), capture_output=True, text=True, check=True)
