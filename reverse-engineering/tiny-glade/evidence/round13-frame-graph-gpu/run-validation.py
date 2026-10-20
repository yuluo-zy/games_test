"""One integration owner, original assets, strict Vulkan, recorded source identity."""
import hashlib
import json
import os
from pathlib import Path
import subprocess
import sys
import time

base = Path(__file__).resolve().parent
workspace = Path('D:/game/tiny-glade')
label = sys.argv[1]
command_args = sys.argv[2:]
files = sorted(p for p in (workspace / 'crates').rglob('*')
               if p.is_file() and (p.suffix == '.rs' or p.name == 'Cargo.toml'))
files += [workspace / 'Cargo.toml', workspace / 'Cargo.lock']
manifest = {str(p.relative_to(workspace)): hashlib.sha256(p.read_bytes()).hexdigest()
            for p in files}
(base / (label + '-source-start.json')).write_text(json.dumps(manifest, indent=2), encoding='utf-8')
toolchain = 'C:/Users/liyu/.rustup/toolchains/stable-x86_64-pc-windows-msvc/bin'
env = os.environ.copy()
env.update(CARGO_TARGET_DIR=str(workspace / 'target-round12'),
           RUSTC=toolchain + '/rustc.exe', RUSTDOC=toolchain + '/rustdoc.exe',
           RHAPSODY_REQUIRE_VULKAN='1', RHAPSODY_VULKAN_LIBRARY='vulkan-1.dll',
           TG_INSTALL_ROOT='D:/game/ljxsj_92385/Tiny Glade',
           TG_COMPILED_SHADERS='D:/game/ljxsj_92385/Tiny Glade/compiled-shaders')
env['PATH'] = env['TG_INSTALL_ROOT'] + ';' + toolchain + ';' + env.get('PATH', '')
args = [toolchain + '/cargo.exe'] + command_args
log = base / (label + '.log')
started = time.time()
with log.open('wb') as output:
    process = subprocess.Popen(args, cwd=workspace, env=env, stdout=output, stderr=subprocess.STDOUT)
    print('cargo_pid', process.pid, flush=True)
    result = process.wait()
changed = [str(p.relative_to(workspace)) for p in files
           if p.exists() and manifest[str(p.relative_to(workspace))] != hashlib.sha256(p.read_bytes()).hexdigest()]
(base / (label + '-result.json')).write_text(json.dumps({
    'command': args, 'exit_code': result, 'duration_seconds': time.time() - started,
    'changed_sources': changed, 'strict_vulkan': True,
}, indent=2), encoding='utf-8')
print('cargo_exit', result, 'changed_sources', len(changed), flush=True)
print(log.read_text(encoding='utf-8', errors='replace')[-16000:]
      .encode('ascii', 'backslashreplace').decode(), flush=True)
raise SystemExit(result)
