"""Complete only the explicit headless measurement test target.

Windows builds use a private Job assigned before Cargo starts. Once Cargo emits
the completed requested test artifact, only this Job is terminated; other bins
are not accepted as built. Source/hardware/frozen-file validation stays in caller.
"""
import ctypes
from ctypes import wintypes
import json
import os
from pathlib import Path
import subprocess
import sys
import time

TERMINATED_AFTER_TARGET = 0xE000C0DE
TARGET = 'original_gameplay_chain'

class Job:
    def __init__(self):
        if os.name != 'nt':
            raise RuntimeError('Windows Job Object required')
        self.kernel = ctypes.WinDLL('kernel32', use_last_error=True)
        self.kernel.CreateJobObjectW.argtypes = [ctypes.c_void_p, wintypes.LPCWSTR]
        self.kernel.CreateJobObjectW.restype = wintypes.HANDLE
        self.kernel.SetInformationJobObject.argtypes = [wintypes.HANDLE, ctypes.c_int, ctypes.c_void_p, wintypes.DWORD]
        self.kernel.AssignProcessToJobObject.argtypes = [wintypes.HANDLE, wintypes.HANDLE]
        self.kernel.TerminateJobObject.argtypes = [wintypes.HANDLE, wintypes.UINT]
        self.kernel.QueryInformationJobObject.argtypes = [wintypes.HANDLE, ctypes.c_int, ctypes.c_void_p, wintypes.DWORD, ctypes.c_void_p]
        self.kernel.CloseHandle.argtypes = [wintypes.HANDLE]
        self.handle = self.kernel.CreateJobObjectW(None, None)
        if not self.handle:
            raise ctypes.WinError(ctypes.get_last_error())
        class Basic(ctypes.Structure):
            _fields_ = [('PerProcessUserTimeLimit', ctypes.c_int64), ('PerJobUserTimeLimit', ctypes.c_int64),
                        ('LimitFlags', wintypes.DWORD), ('MinimumWorkingSetSize', ctypes.c_size_t),
                        ('MaximumWorkingSetSize', ctypes.c_size_t), ('ActiveProcessLimit', wintypes.DWORD),
                        ('Affinity', ctypes.c_size_t), ('PriorityClass', wintypes.DWORD), ('SchedulingClass', wintypes.DWORD)]
        class IO(ctypes.Structure):
            _fields_ = [(name, ctypes.c_uint64) for name in ('ReadOperationCount', 'WriteOperationCount', 'OtherOperationCount', 'ReadTransferCount', 'WriteTransferCount', 'OtherTransferCount')]
        class Extended(ctypes.Structure):
            _fields_ = [('BasicLimitInformation', Basic), ('IoInfo', IO), ('ProcessMemoryLimit', ctypes.c_size_t),
                        ('JobMemoryLimit', ctypes.c_size_t), ('PeakProcessMemoryUsed', ctypes.c_size_t), ('PeakJobMemoryUsed', ctypes.c_size_t)]
        limits = Extended()
        limits.BasicLimitInformation.LimitFlags = 0x2000  # JOB_OBJECT_LIMIT_KILL_ON_JOB_CLOSE
        if not self.kernel.SetInformationJobObject(self.handle, 9, ctypes.byref(limits), ctypes.sizeof(limits)):
            raise ctypes.WinError(ctypes.get_last_error())

    def assign(self, process):
        if not self.kernel.AssignProcessToJobObject(self.handle, wintypes.HANDLE(int(process._handle))):
            raise ctypes.WinError(ctypes.get_last_error())

    def terminate(self):
        if not self.kernel.TerminateJobObject(self.handle, TERMINATED_AFTER_TARGET):
            raise ctypes.WinError(ctypes.get_last_error())

    def close(self):
        if self.handle:
            self.kernel.CloseHandle(self.handle)
            self.handle = None

    def wait_empty(self):
        class Accounting(ctypes.Structure):
            _fields_ = [('TotalUserTime', ctypes.c_int64), ('TotalKernelTime', ctypes.c_int64),
                        ('ThisPeriodTotalUserTime', ctypes.c_int64), ('ThisPeriodTotalKernelTime', ctypes.c_int64),
                        ('TotalPageFaultCount', wintypes.DWORD), ('TotalProcesses', wintypes.DWORD),
                        ('ActiveProcesses', wintypes.DWORD), ('TotalTerminatedProcesses', wintypes.DWORD)]
        deadline = time.monotonic()+30
        while True:
            values = Accounting()
            if not self.kernel.QueryInformationJobObject(self.handle, 1, ctypes.byref(values), ctypes.sizeof(values), None):
                raise ctypes.WinError(ctypes.get_last_error())
            if values.ActiveProcesses == 0:
                return
            if time.monotonic() > deadline:
                raise TimeoutError('owned build Job still has active processes')
            time.sleep(.05)

def validate_command(command):
    expected = ['cargo', 'build', '-p', 'tiny-glade', '--test', TARGET,
                '--locked', '--offline', '-j1', '--message-format=json-render-diagnostics']
    remaining = list(command[len(expected):])
    if list(command[:len(expected)]) != expected:
        raise ValueError('target-only builder accepts only the existing headless measurement build command')
    if remaining[:2] == ['--features', 'cpu-probe']:
        remaining = remaining[2:]
    if remaining not in ([], ['--release']):
        raise ValueError('unexpected build flags; no other Cargo command is authorized by this helper')


def validate_artifact(event, cwd, feature, release):
    target = event.get('target', {})
    if event.get('reason') != 'compiler-artifact' or target.get('name') != TARGET:
        return False
    if target.get('kind') != ['test'] or not event.get('executable'):
        raise ValueError('requested artifact is not an executable test target')
    if event.get('features') != (['cpu-probe', 'default', 'mimalloc'] if feature else ['default', 'mimalloc']):
        raise ValueError('requested target feature identity mismatch')
    if not event.get('profile', {}).get('test') or (release and event['profile'].get('opt_level') != '3'):
        raise ValueError('requested test profile mismatch')
    executable = Path(event['executable']).resolve()
    if executable.parent != (Path(cwd)/'target'/('release' if release else 'debug')/'deps').resolve() or not executable.is_file():
        raise ValueError('requested executable does not exist in expected target directory')
    if Path(event['manifest_path']).resolve() != (Path(cwd)/'crates/tiny-glade/Cargo.toml').resolve():
        raise ValueError('requested artifact belongs to a different package')
    return True


def _child(spec_path):
    spec = json.loads(Path(spec_path).read_text(encoding='utf-8'))
    gate = Path(spec['gate'])
    deadline = time.monotonic()+30
    while not gate.exists():
        if time.monotonic() >= deadline:
            raise TimeoutError('private Job assignment handshake was not released')
        time.sleep(.01)
    validate_command(spec['command'])
    cargo = subprocess.Popen(spec['command'], cwd=spec['cwd'])
    print(json.dumps({'reason': 'perf-owned-cargo-start', 'pid': cargo.pid,
                      'wrapper_pid': os.getpid()}), flush=True)
    code = cargo.wait()
    print(json.dumps({'reason': 'perf-owned-cargo-finish', 'pid': cargo.pid, 'exit_code': code}), flush=True)
    raise SystemExit(code)


def run_requested_test(command, cwd, env, stdout_path, stderr_path):
    """Return (artifact, lifecycle); a completed test does not imply whole build success."""
    validate_command(command)
    stdout_path, stderr_path = Path(stdout_path), Path(stderr_path)
    feature, release = 'cpu-probe' in command, '--release' in command
    lifecycle = {'strategy': 'requested test target only', 'cargo_exit_code': None,
                 'wrapper_exit_code': None, 'requested_target_completed': False,
                 'termination_was_intentional_after_requested_artifact': False,
                 'entire_cargo_build_passed': False,
                 'active_build_processes_remaining': None,
                 'remaining_targets_are_not_accepted': True}
    artifact = None
    if os.name != 'nt':
        with stdout_path.open('w', encoding='utf-8') as output, stderr_path.open('w', encoding='utf-8') as errors:
            built = subprocess.run(command, cwd=cwd, env=env, stdout=output, stderr=errors)
        lifecycle.update(cargo_exit_code=built.returncode,
                         entire_cargo_build_passed=built.returncode == 0,
                         active_build_processes_remaining=False)
        if built.returncode:
            raise RuntimeError('Cargo build failed; see '+str(stderr_path))
        for line in stdout_path.read_text(encoding='utf-8').splitlines():
            try:
                event = json.loads(line)
            except ValueError:
                continue
            if validate_artifact(event, cwd, feature, release):
                if artifact is not None:
                    raise ValueError('ambiguous requested compiler artifact')
                artifact = event
        if artifact is None:
            raise RuntimeError('Cargo did not emit the requested test artifact')
        lifecycle['requested_target_completed'] = True
        return artifact, lifecycle
    gate = stdout_path.with_name('build-job-assigned.go')
    spec_path = stdout_path.with_name('build-job-spec.json')
    if gate.exists() or spec_path.exists():
        raise ValueError('refusing to reuse an old build Job handshake')
    spec_path.write_text(json.dumps({'gate': str(gate), 'command': list(command), 'cwd': str(cwd)}), encoding='utf-8')
    process, job = None, Job()
    try:
        with stdout_path.open('w', encoding='utf-8') as output, stderr_path.open('w', encoding='utf-8') as errors:
            process = subprocess.Popen([sys.executable, str(Path(__file__).resolve()), '--_child', str(spec_path)],
                                       cwd=cwd, env=env, stdout=subprocess.PIPE, stderr=errors,
                                       text=True, encoding='utf-8')
            job.assign(process)
            lifecycle.update(wrapper_pid=process.pid, job_assigned_before_cargo_start=True)
            gate.write_text('assigned', encoding='utf-8')
            for line in process.stdout:
                output.write(line)
                output.flush()
                try:
                    event = json.loads(line)
                except ValueError:
                    continue
                if event.get('reason') == 'perf-owned-cargo-start':
                    lifecycle['cargo_pid'] = event['pid']
                elif event.get('reason') == 'perf-owned-cargo-finish':
                    lifecycle['cargo_exit_code'] = event['exit_code']
                elif event.get('reason') == 'compiler-message' and event.get('message', {}).get('level') == 'error':
                    raise RuntimeError('Cargo emitted compilation error; no target accepted')
                if validate_artifact(event, cwd, feature, release):
                    artifact = event
                    lifecycle['requested_target_completed'] = True
                    job.terminate()
                    lifecycle.update(termination_was_intentional_after_requested_artifact=True,
                                     job_termination_exit_code=TERMINATED_AFTER_TARGET)
                    break
            lifecycle['wrapper_exit_code'] = process.wait(timeout=30)
            job.wait_empty()
            lifecycle['active_build_processes_remaining'] = False
        if artifact is None:
            raise RuntimeError('Cargo ended before requested target completed; see '+str(stderr_path))
        lifecycle['entire_cargo_build_passed'] = lifecycle['cargo_exit_code'] == 0
        return artifact, lifecycle
    finally:
        job.close()
        if process is not None and process.poll() is None:
            lifecycle['wrapper_exit_code'] = process.wait(timeout=30)
        stdout_path.with_name('cargo-lifecycle.json').write_text(json.dumps(lifecycle, indent=2)+'\n', encoding='utf-8')


if __name__ == '__main__':
    if len(sys.argv) == 3 and sys.argv[1] == '--_child':
        _child(sys.argv[2])
    raise SystemExit('This helper is only invoked by perf_headless.py build')
