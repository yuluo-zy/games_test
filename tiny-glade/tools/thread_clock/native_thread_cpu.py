"""Read-only Windows CPU counters with retained handles and QPC brackets."""
import ctypes
from ctypes import wintypes
import os
import time

QUERY_THREAD = 0x0800  # THREAD_QUERY_LIMITED_INFORMATION
QUERY_PROCESS = 0x1000  # PROCESS_QUERY_LIMITED_INFORMATION
SYNCHRONIZE = 0x00100000
SNAP_THREAD = 0x00000004
ERROR_NO_MORE_FILES = 18
ERROR_BAD_LENGTH = 24


class FILETIME(ctypes.Structure):
    _fields_ = [('low', wintypes.DWORD), ('high', wintypes.DWORD)]


class THREADENTRY32(ctypes.Structure):
    _fields_ = [('dwSize', wintypes.DWORD), ('cntUsage', wintypes.DWORD),
                ('th32ThreadID', wintypes.DWORD), ('th32OwnerProcessID', wintypes.DWORD),
                ('tpBasePri', wintypes.LONG), ('tpDeltaPri', wintypes.LONG),
                ('dwFlags', wintypes.DWORD)]


def filetime_value(value):
    return (int(value.high) << 32) | int(value.low)


class WindowsCPU:
    def __init__(self):
        if os.name != 'nt':
            raise RuntimeError('Windows native CPU diagnostic only')
        self.api = ctypes.WinDLL('kernel32', use_last_error=True)
        self.api.QueryPerformanceCounter.argtypes = [ctypes.POINTER(ctypes.c_longlong)]
        self.api.QueryPerformanceFrequency.argtypes = [ctypes.POINTER(ctypes.c_longlong)]
        self.api.CreateToolhelp32Snapshot.argtypes = [wintypes.DWORD, wintypes.DWORD]
        self.api.CreateToolhelp32Snapshot.restype = wintypes.HANDLE
        for name in ('Thread32First', 'Thread32Next'):
            getattr(self.api, name).argtypes = [wintypes.HANDLE, ctypes.POINTER(THREADENTRY32)]
        self.api.OpenThread.argtypes = [wintypes.DWORD, wintypes.BOOL, wintypes.DWORD]
        self.api.OpenThread.restype = wintypes.HANDLE
        self.api.OpenProcess.argtypes = [wintypes.DWORD, wintypes.BOOL, wintypes.DWORD]
        self.api.OpenProcess.restype = wintypes.HANDLE
        self.api.GetProcessIdOfThread.argtypes = [wintypes.HANDLE]
        self.api.GetProcessIdOfThread.restype = wintypes.DWORD
        self.api.GetThreadId.argtypes = [wintypes.HANDLE]
        self.api.GetThreadId.restype = wintypes.DWORD
        self.api.WaitForSingleObject.argtypes = [wintypes.HANDLE, wintypes.DWORD]
        self.api.WaitForSingleObject.restype = wintypes.DWORD
        self.api.CloseHandle.argtypes = [wintypes.HANDLE]
        for name in ('GetThreadTimes', 'GetProcessTimes'):
            getattr(self.api, name).argtypes = [wintypes.HANDLE]+[ctypes.POINTER(FILETIME)]*4
        self.description_api = getattr(self.api, 'GetThreadDescription', None)
        if self.description_api:
            self.description_api.argtypes = [wintypes.HANDLE, ctypes.POINTER(ctypes.c_void_p)]
            self.description_api.restype = wintypes.LONG
            self.api.LocalFree.argtypes = [ctypes.c_void_p]
            self.api.LocalFree.restype = ctypes.c_void_p
        frequency = ctypes.c_longlong()
        if not self.api.QueryPerformanceFrequency(ctypes.byref(frequency)) or frequency.value <= 0:
            raise ctypes.WinError(ctypes.get_last_error())
        self.frequency = frequency.value

    def qpc(self):
        value = ctypes.c_longlong()
        if not self.api.QueryPerformanceCounter(ctypes.byref(value)):
            raise ctypes.WinError(ctypes.get_last_error())
        return value.value

    def close(self, handle):
        if handle and not self.api.CloseHandle(handle):
            raise ctypes.WinError(ctypes.get_last_error())

    def wait(self, handle, milliseconds):
        status = self.api.WaitForSingleObject(handle, milliseconds)
        if status == 0:
            return True
        if status == 258:  # WAIT_TIMEOUT
            return False
        raise ctypes.WinError(ctypes.get_last_error())

    def signaled(self, handle):
        return self.wait(handle, 0)

    def process(self, process_id):
        handle = self.api.OpenProcess(QUERY_PROCESS | SYNCHRONIZE, False, process_id)
        if not handle:
            raise ctypes.WinError(ctypes.get_last_error())
        return handle

    def thread(self, process_id, thread_id):
        handle = self.api.OpenThread(QUERY_THREAD | SYNCHRONIZE, False, thread_id)
        if not handle:
            raise ctypes.WinError(ctypes.get_last_error())
        if self.api.GetProcessIdOfThread(handle) != process_id or self.api.GetThreadId(handle) != thread_id:
            self.close(handle)
            raise RuntimeError('thread identity changed between snapshot and OpenThread')
        return handle

    def times(self, handle, *, thread):
        was_signaled = self.signaled(handle)
        create, exit_time, kernel, user = FILETIME(), FILETIME(), FILETIME(), FILETIME()
        function = self.api.GetThreadTimes if thread else self.api.GetProcessTimes
        before = self.qpc()
        succeeded = function(handle, ctypes.byref(create), ctypes.byref(exit_time), ctypes.byref(kernel), ctypes.byref(user))
        error = ctypes.get_last_error() if not succeeded else None
        after = self.qpc()
        if not succeeded:
            raise ctypes.WinError(error)
        kernel_raw, user_raw = filetime_value(kernel), filetime_value(user)
        return {'qpc': [before, after], 'kernel_100ns': kernel_raw, 'user_100ns': user_raw,
                'kernel_ns': kernel_raw*100, 'user_ns': user_raw*100,
                'cpu_ns': (kernel_raw+user_raw)*100,
                'creation_filetime': filetime_value(create),
                'exit_filetime': filetime_value(exit_time) if was_signaled else None,
                'signaled_before_read': was_signaled, 'signaled_after_read': self.signaled(handle)}

    def description(self, handle):
        if self.description_api is None:
            return None
        value = ctypes.c_void_p()
        result = self.description_api(handle, ctypes.byref(value))
        if result < 0:
            return None
        try:
            return ctypes.wstring_at(value.value) if value.value else ''
        finally:
            if value.value:
                self.api.LocalFree(value)

    def thread_ids(self, process_id):
        invalid = ctypes.c_void_p(-1).value
        snapshot = None
        for attempt in range(8):
            snapshot = self.api.CreateToolhelp32Snapshot(SNAP_THREAD, 0)
            if snapshot != invalid:
                break
            error = ctypes.get_last_error()
            if error != ERROR_BAD_LENGTH:
                raise ctypes.WinError(error)
            time.sleep(.001)
        if snapshot == invalid:
            raise RuntimeError('thread snapshot remained unavailable after 8 bounded attempts')
        result = []
        try:
            entry = THREADENTRY32()
            entry.dwSize = ctypes.sizeof(entry)
            succeeded = self.api.Thread32First(snapshot, ctypes.byref(entry))
            while succeeded:
                if entry.dwSize < THREADENTRY32.th32OwnerProcessID.offset+ctypes.sizeof(wintypes.DWORD):
                    raise RuntimeError('Toolhelp thread record is too short')
                if entry.th32OwnerProcessID == process_id:
                    result.append(int(entry.th32ThreadID))
                entry.dwSize = ctypes.sizeof(entry)
                succeeded = self.api.Thread32Next(snapshot, ctypes.byref(entry))
            if ctypes.get_last_error() != ERROR_NO_MORE_FILES:
                raise ctypes.WinError(ctypes.get_last_error())
            return sorted(set(result))
        finally:
            self.close(snapshot)


class RetainedThreads:
    def __init__(self, native, process_id, capacity=4096):
        self.native, self.process_id, self.capacity = native, process_id, capacity
        self.records, self.handles, self.current = [], [], {}
        self.errors, self.polls, self.overflow, self.reused_ids = [], 0, 0, set()

    def discover(self, *, initial=False):
        self.polls += 1
        for thread_id in self.native.thread_ids(self.process_id):
            previous = self.current.get(thread_id)
            if previous is not None:
                if not self.native.signaled(self.handles[previous]):
                    continue
            if len(self.records) >= self.capacity:
                self.overflow += 1
                continue
            handle = None
            try:
                handle = self.native.thread(self.process_id, thread_id)
                sample = self.native.times(handle, thread=True)
                if previous is not None:
                    if sample['creation_filetime'] == self.records[previous]['creation_filetime']:
                        # The snapshot raced with termination, not TID reuse.
                        continue
                    self.reused_ids.add(thread_id)
                record = {'thread_id': thread_id, 'description': self.native.description(handle),
                          'creation_filetime': sample['creation_filetime'], 'first_sample': sample,
                          'baseline': None, 'initial_discovery': initial, 'final': None,
                          'handle_retained_until_final_sample': True}
                self.current[thread_id] = len(self.records)
                self.records.append(record)
                self.handles.append(handle)
                handle = None
            except Exception as error:
                self.errors.append({'thread_id': thread_id, 'phase': 'discover', 'error': repr(error)})
            finally:
                if handle:
                    self.native.close(handle)

    def baseline(self):
        self.discover(initial=True)
        for record, handle in zip(self.records, self.handles):
            record['baseline'] = self.native.times(handle, thread=True)

    def finalize(self):
        for record, handle in zip(self.records, self.handles):
            try:
                record['final'] = self.native.times(handle, thread=True)
            except Exception as error:
                self.errors.append({'thread_id': record['thread_id'], 'phase': 'final', 'error': repr(error)})

    def report(self):
        return {'threads': self.records, 'enumeration_polls': self.polls, 'tracking_capacity': self.capacity,
                'tracking_overflow': self.overflow, 'reused_thread_ids': sorted(self.reused_ids),
                'errors': self.errors}

    def close(self):
        # Retained thread objects remain queryable after termination. Close only
        # the handles opened here; never terminate or suspend target threads.
        failures = []
        for handle in self.handles:
            try:
                self.native.close(handle)
            except Exception as error:
                failures.append(repr(error))
        self.handles.clear()
        if failures:
            raise RuntimeError('retained handle cleanup errors: '+repr(failures))
