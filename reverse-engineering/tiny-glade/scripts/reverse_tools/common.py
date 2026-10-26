"""Windows-local runtime configuration and owned-process identity checks."""
import ctypes
import hashlib
import json
import os
from pathlib import Path
import re
import shutil
import time

ROOT = Path(__file__).resolve().parents[2]
WORKSPACE = ROOT.parents[1]
PYTHON = Path('D:/tools/reverse-engineering/venvs/reverse-engineering-assistant/Scripts/python.exe')
GHIDRA = Path('D:/tools/rea/ghidra/ghidra_12.1.4_PUBLIC')
JAVA = Path('D:/tools/rea/jdk/jdk-21.0.12.1+1')
TARGET = WORKSPACE / 'ljxsj_92385/Tiny Glade/tiny-glade.exe'
SHA256 = 'cf048e27acdc266b2efc7c094dd435bacecb3f26c2afe54de4d09cd3f7036323'
RUNTIME = WORKSPACE / 'tools/reverse-runtime'
SEED = RUNTIME / 'B-seed'

def clone_seed(directory):
    """Copy only a frozen, verified, closed seed; never copy a live worker."""
    project = Path(directory) / 'project'
    if (project / 'TinyGlade.gpr').exists() or not (SEED / 'manifest.json').exists():
        return False
    manifest = json.loads((SEED / 'manifest.json').read_text(encoding='utf-8'))
    if manifest['program_sha256'] != SHA256:
        raise ValueError('Seed target identity mismatch')
    if project.exists() and any(project.iterdir()):
        raise ValueError('Partial project exists; inspect it before reusing this worker')
    for row in manifest['files']:
        # Ghidra rotates ~index.bak/~journal.bak every time a project is opened. They hold
        # no program content, so a later open of the seed must not invalidate the frozen
        # database (.gbf), properties and index files that are still verified below.
        name = Path(row['path']).name
        if name.startswith('~') and name.endswith('.bak'):
            continue
        path = (SEED / 'project' / row['path']).resolve()
        if not path.is_relative_to((SEED / 'project').resolve()): raise ValueError('Seed path escapes project')
        with path.open('rb') as stream:
            if hashlib.file_digest(stream,'sha256').hexdigest() != row['sha256']:
                raise ValueError('Frozen seed checksum mismatch: '+row['path'])
    shutil.copytree(SEED / 'project', project, dirs_exist_ok=True)
    atomic_json(Path(directory) / 'seed-origin.json', {
        'program_sha256':SHA256, 'seed_manifest_sha256':hashlib.sha256((SEED/'manifest.json').read_bytes()).hexdigest(),
        'historical_mac_overlays':False, 'copy_kind':'independent Windows filesystem copy'})
    return True

def worker_dir(name):
    if not re.fullmatch(r'[a-zA-Z0-9][a-zA-Z0-9_-]{0,63}', name):
        raise ValueError('Worker must be 1–64 letters, digits, underscores or hyphens')
    return RUNTIME / name

def atomic_json(path, value):
    path = Path(path)
    path.parent.mkdir(parents=True, exist_ok=True)
    tmp = path.with_name(path.name + '.' + str(os.getpid()) + '.tmp')
    tmp.write_text(json.dumps(value, ensure_ascii=False, indent=2) + '\n', encoding='utf-8')
    tmp.replace(path)

def process_birth(pid):
    """PID plus creation timestamp protects against Windows PID reuse."""
    if os.name != 'nt':
        raise RuntimeError('This runtime is the Windows route; use the retained Mac operator on macOS')
    kernel = ctypes.WinDLL('kernel32', use_last_error=True)
    kernel.OpenProcess.argtypes = [ctypes.c_uint32, ctypes.c_int, ctypes.c_uint32]
    kernel.OpenProcess.restype = ctypes.c_void_p
    kernel.GetProcessTimes.argtypes = [ctypes.c_void_p] + [ctypes.c_void_p] * 4
    kernel.CloseHandle.argtypes = [ctypes.c_void_p]
    handle = kernel.OpenProcess(0x1000, False, int(pid))
    if not handle:
        return None
    times = [ctypes.c_uint64() for _ in range(4)]
    try:
        if not kernel.GetProcessTimes(handle, *(ctypes.byref(t) for t in times)):
            return None
        # An exited process can briefly retain an identity while its handle exists.
        if times[1].value:
            return None
        return times[0].value
    finally:
        kernel.CloseHandle(handle)

def owned_alive(state):
    return state.get('pid') and process_birth(state['pid']) == state.get('process_birth')

def target_digest():
    with TARGET.open('rb') as stream:
        actual = hashlib.file_digest(stream, 'sha256').hexdigest()
    if actual != SHA256:
        raise RuntimeError('B target identity changed: ' + actual)
    return actual

class WorkerLock:
    """Serializes jobs on one project; distinct workers remain independent."""
    def __init__(self, directory, timeout=120):
        self.directory, self.timeout = Path(directory), timeout
    def __enter__(self):
        import msvcrt
        self.directory.mkdir(parents=True, exist_ok=True)
        self.file = (self.directory / 'client.lock').open('a+b')
        self.file.seek(0, 2)
        if self.file.tell() == 0:
            self.file.write(b'0'); self.file.flush()
        deadline = time.monotonic() + self.timeout
        while True:
            try:
                self.file.seek(0)
                msvcrt.locking(self.file.fileno(), msvcrt.LK_NBLCK, 1)
                return self
            except OSError:
                if time.monotonic() >= deadline:
                    self.file.close()
                    raise TimeoutError('Worker is busy; use a distinct worker for parallel work')
                time.sleep(.1)
    def __exit__(self, *args):
        import msvcrt
        self.file.seek(0)
        msvcrt.locking(self.file.fileno(), msvcrt.LK_UNLCK, 1)
        self.file.close()
