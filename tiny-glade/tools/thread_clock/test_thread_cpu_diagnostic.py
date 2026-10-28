import argparse
import copy
import ctypes
import hashlib
import json
import os
from pathlib import Path
import subprocess
import sys
import tempfile
import threading
import time
import unittest
from unittest.mock import patch

import native_thread_cpu as native
import thread_cpu_account as account
from thread_cpu_account import Timeline
import thread_cpu_diagnostic as runner


def sample(before, after, cpu_ms, *, creation=1, exited=False):
    raw = round(cpu_ms*10000)
    return {'qpc': [before, after], 'cpu_ns': raw*100, 'kernel_100ns': raw//3,
            'user_100ns': raw-raw//3, 'kernel_ns': (raw//3)*100,
            'user_ns': (raw-raw//3)*100, 'creation_filetime': creation,
            'exit_filetime': 999 if exited else None,
            'signaled_before_read': exited, 'signaled_after_read': exited}


class Accounting(unittest.TestCase):
    def fixture(self):
        old_initial = sample(102, 103, 10)
        data = {'schema': 'retained-thread-counters-v1', 'process_id': 7,
                'process_exit_code': 0, 'qpc_frequency': 1000,
                'initial': {'process_before_threads': sample(100, 101, 20),
                            'process_after_threads': sample(110, 111, 20)},
                'final': {'process_before_threads': sample(900, 901, 200, exited=True),
                          'process_after_threads': sample(920, 921, 200, exited=True)},
                'tracking_overflow': 0, 'reused_thread_ids': [], 'errors': [],
                'threads': [{'thread_id': 11, 'description': 'owner', 'creation_filetime': 1,
                             'baseline': old_initial, 'first_sample': old_initial,
                             'final': sample(905, 906, 110, exited=True)},
                            {'thread_id': 22, 'description': 'worker', 'creation_filetime': 2,
                             'baseline': None, 'first_sample': sample(500, 501, 80, creation=2),
                             'final': sample(910, 911, 80, creation=2, exited=True)}]}
        timelines = {11: Timeline([(200, 300)]), 22: Timeline([(400, 480)])}
        events = {'starts': {22: [400]}, 'ends': {}, 'names': {}}
        return data, timelines, events

    def test_matching_complete_window_and_process_sandwich(self):
        data, spans, events = self.fixture()
        result = account.analyze_data(data, spans, events, 0, 1000)
        self.assertTrue(result['coverage_complete'])
        self.assertTrue(result['thread_sum_within_process_sandwich'])
        self.assertEqual(result['thread_os_cpu_sum_ms'], 180)
        self.assertEqual(result['process_os_delta_sandwich_ms'], [180, 180])
        self.assertTrue(all(row['within_existing_2pct'] for row in result['rows']))
        self.assertFalse(result['acceptance_gate_changed'])

    def test_new_thread_includes_cpu_consumed_before_first_poll(self):
        data, spans, events = self.fixture()
        result = account.analyze_data(data, spans, events, 0, 1000)
        worker = next(row for row in result['rows'] if row['thread_id'] == 22)
        self.assertEqual(worker['os_cpu_ms'], 80)
        self.assertEqual(worker['baseline_kind'], 'etw_proven_new_thread_zero_lifetime_baseline')

    def test_new_thread_requires_proven_post_baseline_birth(self):
        data, spans, events = self.fixture()
        events['starts'][22] = [105]
        result = account.analyze_data(data, spans, events, 0, 1000)
        self.assertFalse(result['coverage_complete'])
        self.assertEqual(result['unresolved_thread_baselines'][0]['thread_id'], 22)

    def test_short_lived_unobserved_thread_is_a_coverage_failure(self):
        data, spans, events = self.fixture()
        spans[33] = Timeline([(500, 550)])
        result = account.analyze_data(data, spans, events, 0, 1000)
        self.assertFalse(result['coverage_complete'])
        self.assertEqual(result['untracked_running_threads'], [{'thread_id': 33, 'etw_cpu_ms': 50, 'description': None}])

    def test_tid_reuse_and_capacity_overflow_cannot_claim_complete_coverage(self):
        for key, value in [('reused_thread_ids', [11]), ('tracking_overflow', 1)]:
            data, spans, events = self.fixture()
            data[key] = value
            self.assertFalse(account.analyze_data(data, spans, events, 0, 1000)['coverage_complete'])

    def test_large_brackets_do_not_relax_the_2pct_comparison(self):
        begin, end = sample(100, 200, 0), sample(300, 400, 50, exited=True)
        result = account.compare_window((Timeline([(100, 400)]),), begin, end, 1000)
        self.assertEqual(result['etw_read_window_lower_ms'], 100)
        self.assertEqual(result['etw_read_window_upper_ms'], 300)
        self.assertFalse(result['within_existing_2pct'])
        self.assertFalse(result['acceptance_adjusted'])

    def test_final_counters_must_be_stable_and_object_identity_must_match(self):
        for field, value in [('signaled_before_read', False), ('creation_filetime', 444)]:
            data, spans, events = self.fixture()
            data['threads'][0]['final'][field] = value
            with self.assertRaises(ValueError):
                account.analyze_data(data, spans, events, 0, 1000)

    def test_rejects_regression_wrong_units_or_outside_trace(self):
        data, spans, events = self.fixture()
        with self.assertRaisesRegex(ValueError, 'enclose'):
            account.analyze_data(data, spans, events, 101, 1000)
        changed = sample(1, 2, 10)
        changed['cpu_ns'] += 100
        with self.assertRaisesRegex(ValueError, 'units'):
            account.sample_valid(changed, 'unit')
        with self.assertRaisesRegex(ValueError, 'regressed'):
            account.compare_window((Timeline([(1, 100)]),), sample(1, 2, 10), sample(20, 21, 9), 1000)

    def test_lifecycle_reads_actual_start_not_rundown_as_new_birth(self):
        text = '''T-Start,TimeStamp,Process Name ( PID),ThreadID
T-DCStart,TimeStamp,Process Name ( PID),ThreadID
ThreadName,TimeStamp,Process Name ( PID),ThreadID,ThreadName
# 0 # 0x64 # 0
T-DCStart,0,game (7),11
# 0 # 0xc8 # 0
T-Start,0,game (7),22
# 0 # 0xc9 # 0
ThreadName,0,game (7),22,worker
'''
        with tempfile.TemporaryDirectory() as directory:
            path = Path(directory)/'lifecycle.csv'
            path.write_text(text)
            result = account.lifecycle(path, 7)
        self.assertEqual(result['starts'], {22: [200]})
        self.assertEqual(result['names'], {22: 'worker'})


@unittest.skipUnless(os.name == 'nt', 'native Windows API smoke; no game/GPU/ETW')
class NativeReadOnly(unittest.TestCase):
    def test_handle_survives_thread_exit_and_counter_units_are_valid(self):
        api = native.WindowsCPU()
        ready, release = threading.Event(), threading.Event()
        def work():
            ready.set()
            release.wait()
            deadline = time.perf_counter()+.015
            while time.perf_counter() < deadline:
                pass
        thread = threading.Thread(target=work)
        thread.start()
        handle = None
        try:
            self.assertTrue(ready.wait(5))
            self.assertIn(thread.native_id, api.thread_ids(os.getpid()))
            handle = api.thread(os.getpid(), thread.native_id)
            before = api.times(handle, thread=True)
            release.set()
            thread.join(5)
            self.assertFalse(thread.is_alive())
            self.assertTrue(api.wait(handle, 5000))
            after = api.times(handle, thread=True)
            self.assertTrue(after['signaled_before_read'])
            self.assertGreaterEqual(after['cpu_ns'], before['cpu_ns'])
            self.assertEqual(after['creation_filetime'], before['creation_filetime'])
            self.assertGreaterEqual(after['exit_filetime'], after['creation_filetime'])
            account.sample_valid(before, 'native before')
            account.sample_valid(after, 'native after')
        finally:
            release.set()
            thread.join(5)
            if handle:
                api.close(handle)

    def test_process_read_retains_kernel_and_user_and_no_priority_mutation(self):
        api = native.WindowsCPU()
        handle = api.process(os.getpid())
        try:
            value = api.times(handle, thread=False)
            account.sample_valid(value, 'self process')
            self.assertGreater(value['creation_filetime'], 0)
        finally:
            api.close(handle)

    def test_cng_sha256_small_native_work_matches_independent_digest(self):
        from synthetic_sha256 import SHA256, input_bytes
        for size in (1, 257):
            data = input_bytes(size)
            buffer = ctypes.create_string_buffer(data, len(data))
            state = SHA256()
            try:
                state.update(buffer, len(data))
                state.update(buffer, len(data))
                self.assertEqual(state.finish(), hashlib.sha256(data+data).hexdigest())
            finally:
                state.close()

    def test_synthetic_child_ready_go_patterns_keep_identical_bytes_and_digest(self):
        from synthetic_sha256 import input_bytes
        values = []
        with tempfile.TemporaryDirectory() as directory:
            for pattern in ('continuous', 'burst'):
                output = Path(directory)/(pattern+'.json')
                command = [sys.executable, str(Path(__file__).with_name('synthetic_sha256.py')),
                           pattern, '--output', str(output), '--workers', '2', '--chunks', '2',
                           '--chunk-bytes', '257', '--burst-wait-ms', '1']
                child = subprocess.Popen(command, stdout=subprocess.PIPE, stderr=subprocess.PIPE, text=True)
                try:
                    deadline = time.monotonic()+10
                    while not output.with_suffix('.ready').exists():
                        self.assertIsNone(child.poll())
                        self.assertLess(time.monotonic(), deadline)
                        time.sleep(.01)
                    self.assertFalse(output.exists(), 'child must wait for explicit go')
                    output.with_suffix('.go').write_text('go')
                    stdout, stderr = child.communicate(timeout=10)
                    self.assertEqual(child.returncode, 0, stdout+stderr)
                    report = json.loads(output.read_text())
                    self.assertTrue(report['valid'])
                    self.assertEqual(report['workload']['total_bytes'], 1028)
                    expected = hashlib.sha256(input_bytes(257)*2).hexdigest()
                    self.assertEqual([worker['sha256'] for worker in report['workers']], [expected, expected])
                    values.append(report['workload_sha256'])
                finally:
                    if child.poll() is None:
                        child.terminate()
                        child.wait(timeout=10)
        self.assertEqual(values[0], values[1])


class HandleOwnership(unittest.TestCase):
    class API:
        def __init__(self):
            self.counter, self.closed = 0, []
            self.exited, self.creation = False, 1
        def thread_ids(self, pid): return [11]
        def signaled(self, handle): return self.exited
        def thread(self, pid, tid): self.counter += 1; return self.counter
        def times(self, handle, thread): return sample(100, 101, 0, creation=self.creation, exited=self.exited)
        def description(self, handle): return 'synthetic'
        def close(self, handle): self.closed.append(handle)

    def test_exiting_same_object_is_not_tid_reuse(self):
        api = self.API()
        held = native.RetainedThreads(api, 7)
        held.discover(initial=True)
        api.exited = True
        held.discover()
        self.assertEqual(len(held.records), 1)
        self.assertFalse(held.reused_ids)
        self.assertEqual(api.closed, [2])
        held.close()
        self.assertEqual(api.closed, [2, 1])

    def test_new_creation_with_same_tid_is_reported_and_both_handles_retained(self):
        api = self.API()
        held = native.RetainedThreads(api, 7)
        held.discover(initial=True)
        api.exited, api.creation = True, 2
        held.discover()
        self.assertEqual(len(held.records), 2)
        self.assertEqual(held.reused_ids, {11})
        self.assertFalse(api.closed)
        held.close()
        self.assertEqual(api.closed, [1, 2])

    def test_handle_capacity_overflow_is_explicit(self):
        api = self.API()
        api.thread_ids = lambda pid: [11, 22]
        held = native.RetainedThreads(api, 7, capacity=1)
        held.discover(initial=True)
        self.assertEqual(held.report()['tracking_overflow'], 1)
        held.close()


class CaptureOwnership(unittest.TestCase):
    def test_failed_trace_start_never_stops_someone_elses_logger(self):
        class API:
            frequency = 1000
            def process(self, pid): return 100
            def thread_ids(self, pid): return []
            def times(self, handle, thread): return sample(100, 101, 0, exited=True)
            def close(self, handle): pass
        class Process:
            pid = 4242
            returncode = None
            terminated = False
            def __init__(self, command, **kwargs):
                (temp/'thread-cpu-diagnostics/test/session.ready').write_text('ready')
            def poll(self): return self.returncode
            def terminate(self): self.terminated = True; self.returncode = -15
            def wait(self, **kwargs): return self.returncode
        with tempfile.TemporaryDirectory() as directory:
            temp = Path(directory)
            args = argparse.Namespace(build='fake', run='test', mode='on', scene='static', warmup=1,
                samples=1, timeout=5, max_threads=8, poll_ms=500, capture_only=True)
            with patch.object(runner.perf, 'PERF', temp), patch.object(runner.perf, 'hardware', return_value={}), \
                 patch.object(runner.perf, 'load_build', return_value={'cpu_probe_feature': True, 'executable': 'not-executed'}), \
                 patch.object(runner, 'WindowsCPU', API), patch.object(runner.subprocess, 'Popen', Process), \
                 patch.object(runner.perf, 'start_trace', side_effect=RuntimeError('foreign logger exists')), \
                 patch.object(runner.subprocess, 'run') as executed:
                with self.assertRaisesRegex(RuntimeError, 'foreign logger'):
                    runner.capture(args)
                executed.assert_not_called()
            result = json.loads((temp/'thread-cpu-diagnostics/test/capture.json').read_text())
            self.assertFalse(result['valid_capture'])
            self.assertFalse(result['trace_started_by_this_capture'])
            self.assertTrue(result['cleanup_terminated_owned_fixture'])


if __name__ == '__main__':
    unittest.main()
