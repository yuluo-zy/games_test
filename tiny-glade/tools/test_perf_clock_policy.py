"""Prospective policy regressions using synthetic artifacts, never real runs."""
import argparse
import datetime
import json
from pathlib import Path
import tempfile
import unittest

import perf_clock_policy as clocks
import perf_headless as runner
from perf_headless_account import account, paired_overhead
from test_perf_headless import report_fixture, trace_fixture, overhead_runs


def save(path, value):
    path.write_text(json.dumps(value, indent=2)+'\n', encoding='utf-8', newline='\n')


class ClockPolicy(unittest.TestCase):
    def setUp(self):
        self.temp = tempfile.TemporaryDirectory()
        self.addCleanup(self.temp.cleanup)
        self.root = Path(self.temp.name)
        self.hardware = {'os': 'Windows-test-19045', 'physical_cpu': {'Name': 'test CPU',
                         'NumberOfCores': 8, 'NumberOfLogicalProcessors': 8}, 'logical_cpus': 8}
        self.workload = {'algorithm': 'Windows CNG SHA256', 'workers': 4, 'chunks_per_worker': 4096,
                         'chunk_bytes': 1048576, 'bytes_per_worker': 4294967296,
                         'total_bytes': 17179869184, 'input_chunk_sha256': 'a'*64}
        declared = datetime.datetime.now(datetime.timezone.utc)-datetime.timedelta(seconds=10)
        self.declaration = {'schema': 'predeclared-cpu-clock-calibration-v1',
            'declared_at_utc': declared.isoformat(), 'pairs': 3,
            'order': [['continuous', 'burst'], ['burst', 'continuous'], ['continuous', 'burst']],
            'workload': self.workload, 'workload_sha256': clocks.signature(self.workload),
            'total_cpu_work_identical_claimed': False,
            'burst_wait_ms': 10,
            'gate_percent': 2,
            'identities': {'old-tools/perf_etw.py': clocks.current_tools()['perf_etw.py'],
                           'old-tools/perf_headless.py': '0'*64, 'old-native-helper.py': '1'*64},
            'primary_worker_roles': ['clock-hash-'+str(i) for i in range(4)]}
        self.declaration_path = self.root/'predeclared.json'
        save(self.declaration_path, self.declaration)
        self.calibrations = []
        trace_start = round(declared.timestamp()*1e7)+116444736010000000
        for index, pattern in enumerate(p for pair in self.declaration['order'] for p in pair):
            directory = self.root/('calibration-'+str(index))
            directory.mkdir()
            session = {'schema': 'synthetic-thread-clock-v1', 'valid': True, 'process_id': 40+index,
                       'thread_id': 1, 'qpc_frequency': 1000,
                       'pattern': pattern, 'workload': self.workload,
                       'burst_wait_ms': 10 if pattern == 'burst' else 0,
                       'workload_sha256': self.declaration['workload_sha256'],
                       'workers': [{'worker': i, 'thread_id': 10+i, 'sha256': 'b'*64,
                                    'chunks': 4096, 'bytes': 4294967296} for i in range(4)]}
            save(directory/'session.json', session)
            raw = f'OS Version: test, Events Lost: 0, Buffers lost: 0, Trace Start: {trace_start+index},\n'
            (directory/'schedule.csv').write_text(raw, encoding='utf-8', newline='\n')
            (directory/'schedule.etl').write_bytes(b'synthetic ETL fixture only')
            def coarse(os_ms, etw_ms, begin=(100, 101), end=(5000, 5001)):
                error = abs(os_ms-etw_ms)/max(os_ms, etw_ms, 1e-6)
                return {'os_cpu_ms': os_ms, 'etw_nominal_cpu_ms': etw_ms,
                        'os_kernel_ms': 0, 'os_user_ms': os_ms,
                        'begin_qpc': list(begin), 'end_qpc': list(end),
                        'os_minus_etw_ms': os_ms-etw_ms, 'nominal_relative_error': error,
                        'within_existing_2pct': error <= .02}
            os_worker = 800 if pattern == 'burst' else 1001
            def sample(os_ms, qpc, creation=1, final=False):
                return {'qpc': list(qpc), 'kernel_100ns': 0, 'user_100ns': os_ms*10000,
                        'kernel_ns': 0, 'user_ns': os_ms*1000000, 'cpu_ns': os_ms*1000000,
                        'creation_filetime': creation, 'signaled_before_read': final}
            native = {'schema': 'retained-thread-counters-v1', 'process_id': 40+index,
                      'qpc_frequency': 1000, 'primary_worker_tids': [10+i for i in range(4)],
                      'errors': [], 'reused_thread_ids': [], 'tracking_overflow': False,
                      'process_exit_code': 0,
                      'initial': {'process_before_threads': sample(0, (90, 91)),
                                  'process_after_threads': sample(0, (110, 111))},
                      'final': {'process_before_threads': sample(os_worker*4, (4900, 4901), final=True),
                                'process_after_threads': sample(os_worker*4, (5100, 5101), final=True)},
                      'threads': [{'thread_id': tid, 'description': 'clock-hash-'+str(tid-10) if tid != 1 else None,
                                   'creation_filetime': tid,
                                   'baseline': sample(0, (100, 101), tid),
                                   'final': sample(os_worker if tid != 1 else 0, (5000, 5001), tid, final=True)}
                                  for tid in [1, 10, 11, 12, 13]]}
            save(directory/'native-counters.json', native)
            capture = {'valid_capture': True, 'exit_code': 0, 'etw_stop_exit_code': 0,
                       'process_id': 40+index, 'fixture_session_sha256': clocks.file_sha(directory/'session.json'),
                       'native_counters_sha256': clocks.file_sha(directory/'native-counters.json'),
                       'hardware': self.hardware, 'helpers': self.declaration['identities']}
            save(directory/'capture.json', capture)
            (directory/'official-owner.csv').write_text('1000,game ('+str(40+index)+'),1\n'+''.join(
                '1000000,game ('+str(40+index)+'),'+str(10+i)+'\n' for i in range(4)), encoding='utf-8')
            quality = {'schema': 'external-thread-cpu-diagnostic-v1', 'process_id': 40+index,
                       'qpc_frequency': 1000, 'parser_file': 'old-tools/perf_etw.py',
                       'parser_sha256': clocks.current_tools()['perf_etw.py'],
                       'coverage_complete': True, 'thread_sum_within_process_sandwich': True,
                       'acceptance_gate_changed': False, 'diagnostic_only': True,
                       'untracked_running_threads': [], 'unresolved_thread_baselines': [],
                       'reused_thread_ids': [], 'etw_reused_thread_ids': [], 'native_errors': [],
                       'microsoft_independent_process': {'input_trace_shared': True, 'relative_error': 0,
                                                         'cpu_ms': 4001, 'within_existing_2pct': True},
                       'process_before_sandwich': coarse(os_worker*4, 4001, (90, 91), (5100, 5101)),
                       'process_after_sandwich': coarse(os_worker*4, 4001, (110, 111), (4900, 4901)),
                       'thread_os_cpu_sum_ms': os_worker*4,
                       'process_os_delta_sandwich_ms': [os_worker*4, os_worker*4],
                       'rows': [{'thread_id': tid, 'description': 'clock-hash-'+str(tid-10) if tid != 1 else None,
                                 'creation_filetime': tid,
                                 'common_process_window_etw_cpu_ms': 1000 if tid != 1 else 1,
                                 'microsoft_common_window_cpu_ms': 1000 if tid != 1 else 1,
                                 'microsoft_common_window_relative_error': 0,
                                 **coarse(os_worker if tid != 1 else 0, 1000 if tid != 1 else 1)}
                                for tid in [1, 10, 11, 12, 13]],
                       'files': {str(directory/'schedule.csv'): clocks.file_sha(directory/'schedule.csv'),
                                 str(directory/'native-counters.json'): clocks.file_sha(directory/'native-counters.json')}}
            save(directory/'thread-cpu.json', quality)
            self.calibrations.append(directory)
        self.evidence_path = self.root/'evidence.json'
        clocks.freeze_evidence(self.evidence_path, self.declaration_path, self.calibrations)

    def update_evidence_reference(self, run_index, which, edit):
        evidence = json.loads(self.evidence_path.read_text())
        reference = evidence['runs'][run_index][which]
        path = Path(reference['path'])
        value = json.loads(path.read_text()); edit(value); save(path, value)
        reference['sha256'] = clocks.file_sha(path)
        save(self.evidence_path, evidence)

    def fresh_capture(self, report=None, label='formal'):
        report = report or report_fixture()
        directory = self.root/label
        directory.mkdir(exist_ok=True)
        key, intent_ref = clocks.prepare_capture(clocks.CALIBRATED, self.evidence_path,
            self.hardware, directory, clocks.current_tools())
        intent = json.loads(Path(intent_ref['path']).read_text())
        session = directory/'session.json'; save(session, report)
        raw = 'OS Version: test, Events Lost: 0, Buffers lost: 0, Trace Start: '+str(intent['created_filetime']+10000)+',\n'
        raw += 'T-DCStart,TimeStamp,Process Name ( PID),ThreadID\nT-End,TimeStamp,Process Name ( PID),ThreadID\n'
        raw += '# 0 # 0x0 # 0\nT-DCStart,0,game (7),11\nT-DCStart,0,game (7),12\n'
        raw += trace_fixture()
        raw += '# 0 # 0x262 # 0\nT-End,0,game (7),11\nT-End,0,game (7),12\n'
        trace = directory/'schedule.csv'; trace.write_text(raw, encoding='utf-8', newline='\n')
        (directory/'schedule.etl').write_bytes(b'synthetic ETL fixture only')
        (directory/'official-owner.csv').write_text(
            'Time (us),Process Name ( PID),ThreadID\n390000,game (7),11\n400000,game (7),12\n'
            '999999,other (77),11\n', encoding='utf-8', newline='\n')
        sha = clocks.file_sha(trace)
        official = clocks.official_capture_totals(report, directory, sha)
        record = {'clock_policy': key, 'clock_policy_intent': intent_ref,
                  'session_path': str(session.resolve()), 'session_sha256': clocks.file_sha(session),
                  'hardware': self.hardware, 'measurement_tools': clocks.current_tools(),
                  'trace_csv_sha256': sha, 'trace_etl_sha256': official['trace_etl_sha256']}
        return report, trace, record, official

    def test_legacy_remains_strict_default_even_with_matching_official_owner(self):
        report = report_fixture(); report['envelope']['end'][2] = 600002000
        trace = self.root/'old.csv'; trace.write_text(trace_fixture(), encoding='utf-8')
        result = account(report, trace, official_owner_ns=390000000)
        self.assertFalse(result['valid'])
        self.assertEqual(result['clock_policy']['id'], clocks.LEGACY)
        self.assertFalse(result['clock_quality']['coarse_checks']['process_getprocesstimes_within_2pct'])

    def test_calibrated_policy_retains_failed_raw_process_check_without_correction(self):
        report = report_fixture(); report['envelope']['end'][2] = 600002000
        report, trace, record, official = self.fresh_capture(report)
        result = account(report, trace, capture_record=record, official_totals=official)
        self.assertTrue(result['valid'], result['issues'])
        self.assertEqual(result['schema'], 'headless-etw-account-v2')
        self.assertEqual(result['envelope']['process_os_cpu_ns'], 600000000)
        self.assertFalse(result['clock_quality']['coarse_checks']['process_getprocesstimes_within_2pct'])
        self.assertFalse(result['clock_quality']['independent_clock'])
        self.assertTrue(result['clock_quality']['shared_event_stream'])
        self.assertFalse(result['clock_quality']['raw_counters_corrected'])

    def test_owner_os_2pct_gate_is_always_mandatory(self):
        report = report_fixture(); report['envelope']['end'][1] = 350001000
        report, trace, record, official = self.fresh_capture(report)
        result = account(report, trace, capture_record=record, official_totals=official)
        self.assertFalse(result['valid'])
        self.assertIn('ETW owner / GetThreadTimes cumulative discrepancy exceeds 2%', result['issues'])

    def test_official_complete_pid_required_not_only_matching_owner(self):
        report, trace, record, official = self.fresh_capture()
        with self.assertRaisesRegex(ValueError, 'complete Microsoft PID'):
            account(report, trace, official_owner_ns=390000000, capture_record=record)
        official['process_cpu_ns'] += 100000000
        with self.assertRaisesRegex(ValueError, 'sum conservation'):
            account(report, trace, capture_record=record, official_totals=official)

    def test_coherent_but_wrong_official_process_is_invalid(self):
        report, trace, record, official = self.fresh_capture()
        path = Path(official['output']['path'])
        path.write_text('390000,game (7),11\n500000,game (7),12\n', encoding='utf-8')
        official = clocks.official_capture_totals(report, path.parent, record['trace_csv_sha256'])
        result = account(report, trace, capture_record=record, official_totals=official)
        self.assertFalse(result['valid'])
        self.assertIn('ETW process / independent Microsoft CSwitch discrepancy exceeds 2%', result['issues'])

    def test_old_run_cannot_be_reinterpreted_as_new_policy(self):
        report, trace, record, official = self.fresh_capture()
        record.pop('clock_policy_intent')
        with self.assertRaisesRegex(ValueError, 'prospective capture intent'):
            account(report, trace, capture_record=record, official_totals=official)

    def test_etw_must_postdate_intent(self):
        report, trace, record, official = self.fresh_capture()
        text = trace.read_text(); text = text.replace(text.splitlines()[0],
            'OS Version: test, Events Lost: 0, Buffers lost: 0, Trace Start: 1,')
        trace.write_text(text, encoding='utf-8', newline='\n')
        record['trace_csv_sha256'] = clocks.file_sha(trace)
        official = clocks.official_capture_totals(report, trace.parent, record['trace_csv_sha256'])
        with self.assertRaisesRegex(ValueError, 'predates prospective'):
            account(report, trace, capture_record=record, official_totals=official)

    def test_missing_or_duplicate_tid_lifecycle_is_rejected(self):
        report, trace, record, official = self.fresh_capture()
        trace.write_text(trace.read_text().replace('T-DCStart,0,game (7),12\n', ''), encoding='utf-8')
        record['trace_csv_sha256'] = clocks.file_sha(trace)
        official = clocks.official_capture_totals(report, trace.parent, record['trace_csv_sha256'])
        with self.assertRaisesRegex(ValueError, 'lifecycle origin'):
            account(report, trace, capture_record=record, official_totals=official)

    def test_terminal_kernel_cpu_after_thread_end_event_is_retained(self):
        report, trace, record, official = self.fresh_capture()
        trace.write_text(trace.read_text().replace('# 0 # 0x262 # 0\nT-End',
            '# 0 # 0x1e0 # 0\nT-End'), encoding='utf-8')
        record['trace_csv_sha256'] = clocks.file_sha(trace)
        official = clocks.official_capture_totals(report, trace.parent, record['trace_csv_sha256'])
        result = account(report, trace, capture_record=record, official_totals=official)
        self.assertTrue(result['valid'], result['issues'])
        self.assertEqual(result['envelope']['process_cpu_ms'], 790)
        self.assertTrue(any(row['terminal_tail_cpu_ns'] > 0 for row in result['clock_quality']['lifecycle']))

    def test_terminal_tail_crossing_dpc_or_isr_is_not_a_new_schedule_in(self):
        for event in ('DPC', 'Interrupt'):
            with self.subTest(event=event):
                report, trace, record, official = self.fresh_capture(label='formal-'+event)
                text = trace.read_text().replace('# 0 # 0x262 # 0\nT-End',
                    '# 0 # 0xd2 # 0\nT-End')  # T-End210, exclusion[220,230].
                if event == 'Interrupt':
                    text = text.replace('DPC,TimeStamp,CPU,ElapsedTime', 'Interrupt,TimeStamp,CPU,ElapsedTime')
                    text = text.replace('DPC,2,0,10000', 'Interrupt,2,0,10000')
                trace.write_text(text, encoding='utf-8')
                record['trace_csv_sha256'] = clocks.file_sha(trace)
                official = clocks.official_capture_totals(report, trace.parent, record['trace_csv_sha256'])
                result = account(report, trace, capture_record=record, official_totals=official)
                self.assertTrue(result['valid'], result['issues'])
                self.assertEqual(result['envelope']['process_cpu_ms'], 790)

    def test_real_schedule_in_after_thread_end_is_rejected(self):
        report, trace, record, official = self.fresh_capture()
        text = trace.read_text().replace('# 0 # 0x262 # 0\nT-End', '# 0 # 0xd2 # 0\nT-End')
        text = text.replace('# 4 # 0x258 # 0',
            '# 0 # 0xfa # 0\nCSwitch,0,Idle (0),0,game (7),11,0\n'
            '# 0 # 0x12c # 0\nCSwitch,0,game (7),11,Idle (0),0,0\n# 4 # 0x258 # 0')
        trace.write_text(text, encoding='utf-8')
        record['trace_csv_sha256'] = clocks.file_sha(trace)
        official = clocks.official_capture_totals(report, trace.parent, record['trace_csv_sha256'])
        with self.assertRaisesRegex(ValueError, 'lifecycle completion'):
            account(report, trace, capture_record=record, official_totals=official)

    def test_declared_raw_etl_or_csv_identity_cannot_be_ignored(self):
        original = self.evidence_path.read_bytes()
        for key in ('trace_etl', 'trace_csv'):
            evidence = json.loads(original)
            evidence['runs'][0][key]['sha256'] = '0'*64
            save(self.evidence_path, evidence)
            with self.assertRaisesRegex(ValueError, 'artifact SHA'):
                clocks.load_evidence(self.evidence_path, self.hardware)
        self.evidence_path.write_bytes(original)
        evidence = json.loads(original); evidence['runs'][0]['trace_csv_raw_sha256'] = '0'*64
        save(self.evidence_path, evidence)
        with self.assertRaisesRegex(ValueError, 'raw CSV identity'):
            clocks.load_evidence(self.evidence_path, self.hardware)

    def test_calibration_official_csv_raw_totals_are_recomputed(self):
        self.update_evidence_reference(0, 'quality', lambda q: q['microsoft_independent_process'].update(cpu_ms=1))
        with self.assertRaisesRegex(ValueError, 'raw process total'):
            clocks.load_evidence(self.evidence_path, self.hardware)

    def test_calibration_worker_official_derived_error_is_not_trusted(self):
        self.update_evidence_reference(0, 'quality', lambda q: q['rows'][1].update(microsoft_common_window_cpu_ms=1))
        with self.assertRaisesRegex(ValueError, 'raw TID total'):
            clocks.load_evidence(self.evidence_path, self.hardware)

    def test_calibration_official_and_native_file_sha_are_bound(self):
        original = self.evidence_path.read_bytes()
        for key in ('official', 'native'):
            evidence = json.loads(original); evidence['runs'][0][key]['sha256'] = '0'*64
            save(self.evidence_path, evidence)
            with self.assertRaisesRegex(ValueError, 'artifact SHA'):
                clocks.load_evidence(self.evidence_path, self.hardware)
        self.evidence_path.write_bytes(original)

    def test_calibration_actual_analysis_parser_identity_is_bound(self):
        self.update_evidence_reference(0, 'quality', lambda q: q.update(parser_sha256='0'*64))
        with self.assertRaisesRegex(ValueError, 'actual analysis parser'):
            clocks.load_evidence(self.evidence_path, self.hardware)

    def test_calibration_worker_roles_are_bound_to_native_and_session_tids(self):
        self.update_evidence_reference(0, 'quality', lambda q: q['rows'][1].update(description='clock-hash-3'))
        with self.assertRaisesRegex(ValueError, 'worker role'):
            clocks.load_evidence(self.evidence_path, self.hardware)

    def test_evidence_tools_hardware_and_artifact_sha_are_pinned(self):
        key, evidence = clocks.load_evidence(self.evidence_path, self.hardware)
        self.assertEqual(key['calibration_sha256'], clocks.file_sha(self.evidence_path))
        wrong_hardware = dict(self.hardware, os='another OS')
        with self.assertRaisesRegex(ValueError, 'hardware/OS'):
            clocks.load_evidence(self.evidence_path, wrong_hardware)
        evidence['tools']['perf_headless.py'] = 'changed'
        save(self.evidence_path, evidence)
        with self.assertRaisesRegex(ValueError, 'tools changed'):
            clocks.load_evidence(self.evidence_path, self.hardware)

    def test_old_calibration_controller_and_new_policy_controller_are_distinct(self):
        self.assertNotEqual(self.declaration['identities']['old-tools/perf_headless.py'],
                            clocks.current_tools()['perf_headless.py'])
        self.assertEqual(clocks.load_evidence(self.evidence_path, self.hardware)[0]['id'], clocks.CALIBRATED)

    def test_changed_calibrated_parser_requires_recalibration(self):
        declaration = json.loads(self.declaration_path.read_text())
        declaration['identities']['old-tools/perf_etw.py'] = '0'*64
        save(self.declaration_path, declaration)
        evidence = json.loads(self.evidence_path.read_text())
        evidence['predeclared']['sha256'] = clocks.file_sha(self.declaration_path)
        evidence['calibration_identities'] = declaration['identities']
        save(self.evidence_path, evidence)
        with self.assertRaisesRegex(ValueError, 'parser differs'):
            clocks.load_evidence(self.evidence_path, self.hardware)

    def test_missing_calibration_round_bad_order_checksum_or_coverage_rejected(self):
        cases = [('session', lambda d: d.update(pattern='wrong'), 'order'),
                 ('session', lambda d: d['workers'][0].update(sha256='c'*64), 'checksums'),
                 ('quality', lambda d: d.update(coverage_complete=False), 'coverage')]
        original = self.evidence_path.read_bytes()
        for which, edit, message in cases:
            evidence = json.loads(original)
            path = Path(evidence['runs'][0][which]['path']); old = path.read_bytes()
            try:
                self.update_evidence_reference(0, which, edit)
                with self.assertRaisesRegex(ValueError, message):
                    clocks.load_evidence(self.evidence_path, self.hardware)
            finally:
                path.write_bytes(old); self.evidence_path.write_bytes(original)

    def test_policies_or_calibration_evidence_cannot_mix_in_overhead(self):
        runs = overhead_runs()
        changed = {'id': clocks.CALIBRATED, 'calibration_sha256': 'new'}
        runs[1]['clock_policy'] = changed; runs[1]['account']['clock_policy'] = changed
        with self.assertRaisesRegex(ValueError, 'clock policy/evidence'):
            paired_overhead(runs)

    def test_cli_keeps_legacy_default_and_no_evidence_for_legacy(self):
        parser = argparse.ArgumentParser(); runner.measurement_options(parser)
        self.assertEqual(parser.parse_args([]).clock_policy, clocks.LEGACY)
        with self.assertRaisesRegex(ValueError, 'must not attach'):
            clocks.prepare_capture(clocks.LEGACY, self.evidence_path, self.hardware, self.root, {})

    def test_existing_frozen_evidence_is_not_overwritten(self):
        old = self.evidence_path.read_bytes()
        with self.assertRaises(FileExistsError):
            clocks.freeze_evidence(self.evidence_path, self.declaration_path, self.calibrations)
        self.assertEqual(self.evidence_path.read_bytes(), old)


if __name__ == '__main__':
    unittest.main()
