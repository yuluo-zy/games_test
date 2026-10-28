"""Same native SHA256 work as continuous or short bursts; no renderer/GPU.

All buffers/hash objects and worker threads exist before ready. Pattern changes
only ordinary nonsignaled-event waits between identical BCryptHashData chunks.
No timer resolution, thread priority, affinity, driver, or power setting is set.
"""
import argparse
import ctypes
from ctypes import wintypes
import hashlib
import json
import os
from pathlib import Path
import sys
import threading
import time

from native_thread_cpu import WindowsCPU


class SHA256:
    def __init__(self):
        if os.name != 'nt':
            raise RuntimeError('Windows CNG synthetic workload only')
        self.api = ctypes.WinDLL('bcrypt', use_last_error=True)
        self.api.BCryptOpenAlgorithmProvider.argtypes = [ctypes.POINTER(ctypes.c_void_p), wintypes.LPCWSTR, wintypes.LPCWSTR, wintypes.ULONG]
        self.api.BCryptGetProperty.argtypes = [ctypes.c_void_p, wintypes.LPCWSTR, ctypes.c_void_p, wintypes.ULONG, ctypes.POINTER(wintypes.ULONG), wintypes.ULONG]
        self.api.BCryptCreateHash.argtypes = [ctypes.c_void_p, ctypes.POINTER(ctypes.c_void_p), ctypes.c_void_p, wintypes.ULONG, ctypes.c_void_p, wintypes.ULONG, wintypes.ULONG]
        self.api.BCryptHashData.argtypes = [ctypes.c_void_p, ctypes.c_void_p, wintypes.ULONG, wintypes.ULONG]
        self.api.BCryptFinishHash.argtypes = [ctypes.c_void_p, ctypes.c_void_p, wintypes.ULONG, wintypes.ULONG]
        self.api.BCryptDestroyHash.argtypes = [ctypes.c_void_p]
        self.api.BCryptCloseAlgorithmProvider.argtypes = [ctypes.c_void_p, wintypes.ULONG]
        for name in ('BCryptOpenAlgorithmProvider', 'BCryptGetProperty', 'BCryptCreateHash', 'BCryptHashData',
                     'BCryptFinishHash', 'BCryptDestroyHash', 'BCryptCloseAlgorithmProvider'):
            getattr(self.api, name).restype = wintypes.LONG
        self.algorithm, self.hash = ctypes.c_void_p(), ctypes.c_void_p()
        self.storage = None
        try:
            self.checked(self.api.BCryptOpenAlgorithmProvider(ctypes.byref(self.algorithm), 'SHA256', None, 0))
            size, returned = wintypes.ULONG(), wintypes.ULONG()
            self.checked(self.api.BCryptGetProperty(self.algorithm, 'ObjectLength', ctypes.byref(size), ctypes.sizeof(size), ctypes.byref(returned), 0))
            self.storage = ctypes.create_string_buffer(size.value)
            self.checked(self.api.BCryptCreateHash(self.algorithm, ctypes.byref(self.hash), self.storage, size.value, None, 0, 0))
        except BaseException:
            self.close()
            raise

    @staticmethod
    def checked(status):
        if status < 0:
            raise OSError('Windows CNG NTSTATUS '+hex(status & 0xffffffff))

    def update(self, buffer, length):
        self.checked(self.api.BCryptHashData(self.hash, buffer, length, 0))

    def finish(self):
        value = ctypes.create_string_buffer(32)
        self.checked(self.api.BCryptFinishHash(self.hash, value, 32, 0))
        return bytes(value.raw).hex()

    def close(self):
        if self.hash.value:
            self.api.BCryptDestroyHash(self.hash)
            self.hash = ctypes.c_void_p()
        if self.algorithm.value:
            self.api.BCryptCloseAlgorithmProvider(self.algorithm, 0)
            self.algorithm = ctypes.c_void_p()


def input_bytes(size):
    pattern = bytes((i*131+17)&255 for i in range(256))
    return pattern*(size//256)+pattern[:size % 256]


def run(args):
    api = WindowsCPU()
    api.api.CreateEventW.argtypes = [ctypes.c_void_p, wintypes.BOOL, wintypes.BOOL, wintypes.LPCWSTR]
    api.api.CreateEventW.restype = wintypes.HANDLE
    wait_event = api.api.CreateEventW(None, True, False, None)
    if not wait_event:
        raise ctypes.WinError(ctypes.get_last_error())
    data = input_bytes(args.chunk_bytes)
    buffer = ctypes.create_string_buffer(data, len(data))
    work = {'algorithm': 'Windows CNG SHA256', 'chunk_bytes': args.chunk_bytes, 'chunks_per_worker': args.chunks,
            'workers': args.workers, 'bytes_per_worker': args.chunk_bytes*args.chunks,
            'total_bytes': args.chunk_bytes*args.chunks*args.workers,
            'input_chunk_sha256': hashlib.sha256(data).hexdigest()}
    signature = hashlib.sha256(json.dumps(work, sort_keys=True, separators=(',', ':')).encode()).hexdigest()
    ready = [threading.Event() for _ in range(args.workers)]
    start = threading.Event()
    stop = threading.Event()
    results = [None]*args.workers
    errors = [None]*args.workers
    threads = []

    def worker(ordinal):
        state = None
        try:
            # Warm code/buffer pages on an independent hash before measurement.
            warm = SHA256()
            try:
                warm.update(buffer, args.chunk_bytes)
                warm.finish()
            finally:
                warm.close()
            state = SHA256()
            thread_id = threading.get_native_id()
            setter = getattr(api.api, 'SetThreadDescription', None)
            if setter is not None:
                setter.argtypes = [wintypes.HANDLE, wintypes.LPCWSTR]
                setter.restype = wintypes.LONG
                setter(wintypes.HANDLE(-2), 'clock-hash-'+str(ordinal))
            ready[ordinal].set()
            if not start.wait(timeout=180):
                raise TimeoutError('synthetic start event not released')
            first = api.qpc()
            first_chunks = []
            for chunk in range(args.chunks):
                if stop.is_set():
                    raise RuntimeError('synthetic workload cancelled after a controller/preparation error')
                begin = api.qpc() if chunk < 32 else None
                # WinDLL calls release the Python GIL during native hashing.
                state.update(buffer, args.chunk_bytes)
                if begin is not None:
                    first_chunks.append(api.qpc()-begin)
                if args.pattern == 'burst' and chunk+1 < args.chunks:
                    if api.wait(wait_event, args.burst_wait_ms):
                        raise RuntimeError('nonsignaled pacing event unexpectedly signaled')
            last = api.qpc()
            digest = state.finish()
            results[ordinal] = {'worker': ordinal, 'thread_id': thread_id, 'sha256': digest,
                                'chunks': args.chunks, 'bytes': args.chunk_bytes*args.chunks,
                                'work_qpc': [first, last], 'first_32_hash_chunk_ticks': first_chunks}
        except BaseException as error:
            errors[ordinal] = repr(error)
        finally:
            ready[ordinal].set()
            if state is not None:
                state.close()

    try:
        for ordinal in range(args.workers):
            thread = threading.Thread(target=worker, args=(ordinal,), name='clock-hash-'+str(ordinal))
            threads.append(thread)
            thread.start()
        for signal in ready:
            if not signal.wait(timeout=120):
                raise TimeoutError('synthetic worker did not become ready')
        if any(errors):
            raise RuntimeError('synthetic preparation failed: '+repr(errors))
        args.output.parent.mkdir(parents=True, exist_ok=True)
        args.output.with_suffix('.ready').write_text('ready', encoding='utf-8')
        deadline = time.monotonic()+120
        while not args.output.with_suffix('.go').exists():
            if time.monotonic() > deadline:
                raise TimeoutError('external counter/ETW handshake timed out')
            api.wait(wait_event, 1)
        start.set()
        for thread in threads:
            thread.join()
        if any(errors) or any(result is None for result in results):
            raise RuntimeError('synthetic workload failed: '+repr(errors))
        if len({result['sha256'] for result in results}) != 1:
            raise RuntimeError('identical synthetic workers produced different hash results')
        report = {'schema': 'synthetic-thread-clock-v1', 'valid': True,
                  'process_id': os.getpid(), 'thread_id': threading.get_native_id(),
                  'qpc_frequency': api.frequency, 'workload': work, 'workload_sha256': signature,
                  'pattern': args.pattern, 'burst_wait_ms': args.burst_wait_ms if args.pattern == 'burst' else 0,
                  'wait_policy': 'ordinary nonsignaled event WaitForSingleObject; no timer resolution or priority changes',
                  'chunk_timing_policy': 'same first32 chunk QPC pairs in both patterns; remaining chunks unprobed',
                  'workers': results}
        temporary = args.output.with_suffix('.tmp')
        temporary.write_text(json.dumps(report, indent=2)+'\n', encoding='utf-8')
        temporary.replace(args.output)
    finally:
        stop.set()
        start.set()
        for thread in threads:
            thread.join(timeout=5)
        api.close(wait_event)


if __name__ == '__main__':
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument('pattern', choices=('continuous', 'burst'))
    parser.add_argument('--output', type=Path, required=True)
    parser.add_argument('--workers', type=int, default=1)
    parser.add_argument('--chunk-bytes', type=int, default=1048576)
    parser.add_argument('--chunks', type=int, default=4096)
    parser.add_argument('--burst-wait-ms', type=int, default=10)
    args = parser.parse_args()
    if not (1 <= args.workers <= 8 and 1 <= args.chunk_bytes <= 67108864
            and 1 <= args.chunks <= 1000000 and 1 <= args.burst_wait_ms <= 1000):
        parser.error('synthetic configuration exceeds declared bounds')
    run(args)
