import copy
import json
import os
from pathlib import Path
import tempfile
from types import SimpleNamespace
import unittest
from unittest.mock import patch

from test_perf_binding_scope_diagnostic import fixture as binding_fixture
import perf_binding_trace_source as tool


def fixture(enabled):
    report = binding_fixture(enabled)
    report['warmup'] = 60
    observations = []
    for row in report['frames']:
        source = {'schema': 'wall-dispatch-source-ranges-v1', 'phase': 'after_App_update_before_prepare',
                  'capacity_elements': 0x200000, 'highlight_type': 0, 'highlighted_allocation_selected': False,
                  'normal': [[0, 32], [64, 1]], 'highlighted': [], 'normal_range_count': 2,
                  'highlighted_range_count': 0, 'normal_instance_count': 33, 'highlighted_instance_count': 0,
                  'callback_dispatches_if_invoked': 2, 'native_dispatches_observed': False,
                  'raw_ranges_blake3': [0] * 32}
        observations.append({**{k: row[k] for k in ('world_tick', 'receipt', 'attempt')},
                             'source': source, 'raw_ranges_blake3_hex': '00' * 32})
    report['wall_dispatch_source_bill'] = {'schema': 'wall-dispatch-source-bill-v1', 'valid': True,
        'capacity': 3, 'retained_observations': 3, 'warmup_observations': 0,
        'native_dispatches_observed': False, 'source_phase': 'after_App_update_before_prepare',
        'observations': observations}
    return report


def native_log(invalid=False):
    fields = {'valid': True, **{name: 0 for name in tool.native_trace.FIELDS}}
    fields['direct_dispatch_api_calls'] = 2
    body = ', '.join(f'{key}: {str(value).lower()}' for key, value in fields.items())
    return ('running 1 test\ntest ' + tool.counts.frozen_tools.TEST + ' ... boot diagnostics\n'
        'RHAPSODY_RECORD_TRACE id=1 graph=7 pass=8 command=9 name="Generate wall draw lists" '
        f'callback_ok=Some(true) unwind=false native_workload=Some(CommandWorkload {{ {body} }})\n'
        'RHAPSODY_SUBMIT_TRACE scope=record_owner_transfer counts=' + repr([
            ('command', 9), ('graph', 7), ('direct_dispatch_api_calls', 999 if invalid else 2),
            ('native_valid', 1), ('recorded_passes', 1)]) + '\n'
        'ok\ntest result: ok. 1 passed; 0 failed; 0 ignored;\n')


class TraceSourceTests(unittest.TestCase):
    def test_only_trace_flag_added_after_existing_cleanup_and_mode_bill_requests(self):
        original = tool.runner.environment
        with patch.dict(os.environ, {'RHAPSODY_SUBMIT_TRACE': 'wrong', 'TG_BINDING_SCOPE_REUSE': 'wrong',
                                     'TG_GPU_DIAGNOSTIC': '1', 'TG_FRAME_TIMING': '1'}, clear=False):
            with tool.trace_environment(), tool.counts.requested_environment(False):
                env = tool.runner.environment(True)
                self.assertEqual(env['RHAPSODY_SUBMIT_TRACE'], '1')
                self.assertEqual(env['TG_BINDING_SCOPE_REUSE'], '0')
                self.assertEqual(env['TG_CPU_BILL'], '1')
                self.assertNotIn('TG_GPU_DIAGNOSTIC', env)
                self.assertNotIn('TG_FRAME_TIMING', env)
                self.assertNotIn('TG_WALL_DISPATCH_SOURCE_TRACE', env)
        self.assertIs(tool.runner.environment, original)

    def test_wall_missing_receipt_bounds_totals_or_digest_encoding_cannot_pass(self):
        for mutation in ('missing', 'receipt', 'bounds', 'total', 'digest', 'native'):
            report = fixture(False)
            if mutation == 'missing':
                report.pop('wall_dispatch_source_bill')
            else:
                row = report['wall_dispatch_source_bill']['observations'][0]
                if mutation == 'receipt': row['receipt'] += 1
                elif mutation == 'bounds': row['source']['normal'][0] = [0x200000, 1]
                elif mutation == 'total': row['source']['normal_instance_count'] += 1
                elif mutation == 'digest': row['raw_ranges_blake3_hex'] = '01' * 32
                else: row['source']['native_dispatches_observed'] = True
            with self.subTest(mutation=mutation), self.assertRaises(ValueError):
                tool.wall_observations(report)

    def test_native_and_source_differences_remain_findings_and_old_comparator_still_rejects(self):
        off, on = fixture(False), fixture(True)
        on['actual_workload']['frames'][1]['native_commands']['direct_dispatch_api_calls'] += 2
        on['actual_workload']['frames'][1]['graph_recording_chunks'] -= 1
        source = on['wall_dispatch_source_bill']['observations'][1]['source']
        source['normal'][1][0] += 32
        result = tool.compare_sources(off, on)
        self.assertEqual(result['actual_native_differences'][0]['on'] - result['actual_native_differences'][0]['off'], 2)
        self.assertFalse(result['normalized_actual_workload_equal'])
        self.assertFalse(result['wall_source_equal'])
        self.assertTrue(any('graph_recording_chunks' in row['field'] for row in result['actual_workload_differences']))
        self.assertTrue(any('normal' in row['field'] for row in result['wall_source_differences']))
        with self.assertRaises(ValueError):
            tool.counts.compare_reports(off, on)

    def test_changed_raw_semantic_input_is_rejected(self):
        off, on = fixture(False), fixture(True)
        on['input_before']['draws'][0] += 1
        on['input_after']['draws'][0] += 1
        with self.assertRaisesRegex(ValueError, 'semantic input'):
            tool.compare_sources(off, on)

    def test_by_name_view_retains_scope_identity_and_is_never_semantic_matching(self):
        parsed = tool.native_trace.parse_trace(native_log())
        self.assertTrue(parsed['trace_valid'])
        row = copy.deepcopy(parsed['records'][0])
        row.update(scope_id=2, graph=70, command=90)
        row['workload']['direct_dispatch_api_calls'] = 3
        parsed['records'].append(row)
        view = tool.by_pass_name(parsed)
        group = view['groups'][0]
        self.assertTrue(view['presentation_only_not_semantic_match'])
        self.assertEqual(group['count'], 2)
        self.assertEqual([row['scope_id'] for row in group['identities']], [1, 2])
        self.assertEqual(group['native']['direct_dispatch_api_calls'], {'min': 2, 'max': 3, 'count': 2})

    def capture(self, invalid=False, vuid=False):
        with tempfile.TemporaryDirectory() as temporary:
            root = Path(temporary)
            exe = root / 'frozen.exe'
            identity = {'source_content_sha256': 'a' * 64, 'files': {exe.name: 'b' * 64}}
            args = SimpleNamespace(build_label='frozen', series='trace-test', scene='static',
                                   warmup=60, samples=2, timeout=30)
            def capture(arguments):
                env = tool.runner.environment(arguments.diagnostic)
                self.assertEqual(env['RHAPSODY_SUBMIT_TRACE'], '1')
                self.assertEqual(env['TG_CPU_BILL'], '1')
                self.assertTrue(arguments.without_etw)
                self.assertFalse(arguments.official)
                path = root / 'headless-runs' / arguments.run_label
                path.mkdir(parents=True)
                record = {'etw': False, 'diagnostic': True, 'valid': False, 'process_id': 7,
                    'executable_sha256': 'b' * 64,
                    'build': {'executable': str(exe), 'source_content_sha256': 'a' * 64}}
                tool.runner.dump(path / 'run.json', record)
                tool.runner.dump(path / 'session.json', fixture(False))
                (path / 'fixture.log').write_text(native_log(invalid) + ('\nVUID-test-001\n' if vuid else ''), encoding='utf-8')
                return record
            with patch.object(tool.runner, 'PERF', root), patch.object(tool.runner, 'capture', side_effect=capture), \
                 patch.object(tool.counts.frozen_tools, 'frozen_identity', return_value=({}, identity)), \
                 patch.object(tool.counts, 'tool_identities', return_value={'old': 'c' * 64}), \
                 patch.object(tool, 'tool_identities', return_value={'new': 'd' * 64}):
                if invalid or vuid:
                    with self.assertRaises(ValueError):
                        tool.capture_role(args, 'off', identity, {'new': 'd' * 64})
                else:
                    _, result = tool.capture_role(args, 'off', identity, {'new': 'd' * 64})
                    self.assertFalse(result['raw_run_valid'])
                    self.assertTrue(result['trace_valid'])
            path = root / 'headless-runs/trace-test-off'
            self.assertFalse(json.loads((path / 'run.json').read_text())['valid'])
            sidecar = json.loads((path / 'trace-source-artifacts.json').read_text())
            for name, sha in sidecar['artifacts'].items():
                self.assertEqual(tool.counts.frozen_tools.digest(path / name), sha)
            self.assertIn('fixture.log', sidecar['artifacts'])
            self.assertIs(tool.runner.environment, tool.counts.runner.environment)

    def test_qualified_capture_keeps_raw_false_and_seals_full_scope_only_trace(self):
        self.capture()

    def test_invalid_native_trace_or_vuid_preserves_failed_raw_artifacts(self):
        for invalid, vuid in ((True, False), (False, True)):
            with self.subTest(invalid=invalid, vuid=vuid):
                self.capture(invalid, vuid)

    def test_first_capture_failure_stops_pair_and_summary_cannot_claim_acceptance(self):
        with tempfile.TemporaryDirectory() as temporary:
            root = Path(temporary)
            args = SimpleNamespace(build_label='frozen', series='failure', scene='static', warmup=60, samples=30, timeout=30)
            with patch.object(tool.runner, 'PERF', root), \
                 patch.object(tool.counts.frozen_tools, 'frozen_identity', return_value=({}, {})), \
                 patch.object(tool, 'tool_identities', return_value={}), \
                 patch.object(tool, 'capture_role', side_effect=RuntimeError('failed real fixture')) as child:
                with self.assertRaises(RuntimeError): tool.run(args)
                self.assertEqual(child.call_count, 1)
            summary = json.loads((root / 'binding-trace-source/failure/summary.json').read_text())
            for key in ('valid', 'clean_cpu_acceptance', 'cpu_performance_evidence', 'production_16ms_acceptance',
                        'formal_source_A_B', 'original_count_comparison_reclassified'):
                self.assertFalse(summary[key])
            self.assertEqual([row['role'] for row in summary['runs']], ['off'])

    def test_existing_role_directory_is_never_overwritten(self):
        with tempfile.TemporaryDirectory() as temporary:
            root = Path(temporary)
            (root / 'headless-runs/old-off').mkdir(parents=True)
            args = SimpleNamespace(build_label='frozen', series='old', scene='static', warmup=60, samples=30, timeout=30)
            with patch.object(tool.runner, 'PERF', root), patch.object(tool, 'capture_role') as child:
                with self.assertRaisesRegex(ValueError, 'fresh per-role'): tool.run(args)
                child.assert_not_called()


if __name__ == '__main__':
    unittest.main()
