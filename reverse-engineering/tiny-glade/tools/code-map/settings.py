"""Host settings and evidence relocation. Recorded evidence identities stay intact."""
import json
import os
from pathlib import Path
from functools import lru_cache
import shutil
import subprocess

SOURCE = Path(__file__).resolve().parent
WORKSPACE = Path(os.environ.get('CODEMAP_WORKSPACE', SOURCE.parents[3])).expanduser().resolve()
CONFIG_FILE = Path(os.environ.get('CODEMAP_CONFIG', WORKSPACE/'tools/code-map.local.json')).expanduser()
CONFIG = json.loads(CONFIG_FILE.read_text()) if CONFIG_FILE.exists() else {}

def option(name, default):
    return os.environ.get('CODEMAP_'+name.upper(), CONFIG.get(name, default))

def host_path(value):
    p = Path(value).expanduser()
    return (WORKSPACE/p).resolve() if not p.is_absolute() else p.resolve()

RUNTIME = host_path(option('runtime', 'tools/code-map-runtime'))
PROJECT = WORKSPACE/'reverse-engineering/tiny-glade'
COLLECTOR = host_path(option('collector', SOURCE/'vendor/native_collect.py'))

def llvm_home():
    value = option('llvm', None)
    if value:
        return host_path(value)
    config = shutil.which('llvm-config')
    if config:
        return Path(subprocess.check_output([config, '--prefix'], text=True).strip())
    for p in ('/opt/homebrew/opt/llvm', '/usr/local/opt/llvm', '/usr/lib/llvm-21', '/usr/lib/llvm-20', '/usr/lib/llvm-19'):
        if (Path(p)/'bin/llvm-objdump').is_file():
            return Path(p)
    return Path('/usr')

LLVM = llvm_home()

def llvm_library():
    explicit = option('llvm_library', None)
    if explicit:
        return host_path(explicit)
    candidates = [LLVM/'lib/libLLVM.dylib', LLVM/'lib/libLLVM.so']
    candidates += sorted((LLVM/'lib').glob('libLLVM-*.so*'))
    for p in candidates:
        if p.is_file():
            return p.resolve()
    raise FileNotFoundError('LLVM shared library missing; set llvm_library in tools/code-map.local.json')

def java_home():
    explicit = option('java_home', os.environ.get('JAVA_HOME'))
    if explicit:
        return host_path(explicit)
    # Reuse the configured host only if that installation still exists.
    lock = RUNTIME/'runtime-lock.json'
    if lock.exists():
        old = Path(json.loads(lock.read_text())['java_home'])
        if (old/'bin/java').is_file():
            return old
    java = shutil.which('java')
    return Path(java).resolve().parent.parent if java else Path('/missing-java-21')

JAVA = java_home()

@lru_cache(maxsize=1)
def origin_workspace():
    relocation = RUNTIME/'relocation.json'
    if relocation.exists():
        return Path(json.loads(relocation.read_text())['origin_workspace'])
    # Historical baseline paths are provenance, never rewritten on disk.
    baseline = PROJECT/'tracking/baselines.json'
    if baseline.exists():
        return Path(json.loads(baseline.read_text())['baselines']['B']['binary']['path']).parent.parent
    return WORKSPACE

def resolve_evidence(value):
    """Map only a recorded workspace prefix; never guess from a basename."""
    path = Path(value)
    if path.is_absolute():
        origin = origin_workspace()
        # Validation/restoration can be nested below the old workspace. A real
        # local path must not have that destination prefix applied a second time.
        if path.is_relative_to(RUNTIME) or (WORKSPACE != origin and path.is_relative_to(WORKSPACE)):
            return path
        if path.is_relative_to(origin):
            relative = path.relative_to(origin)
            runtime_prefix = Path('tools/code-map-runtime')
            if relative.is_relative_to(runtime_prefix):
                return RUNTIME/relative.relative_to(runtime_prefix)
            return WORKSPACE/relative
        return path
    return WORKSPACE/path
