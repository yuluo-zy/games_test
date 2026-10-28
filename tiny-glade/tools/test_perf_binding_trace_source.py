import copy
import base64
import json
import os
import sys
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


def canonical_fixture(enabled):
    report = fixture(enabled)
    prep = {'schema': 'canonical-brick-layout-v1',
        'phase': 'fresh_boot_before_first_gameplay_warmup',
        'owner_count': 12, 'chunk_count': 8, 'used_records': 129,
        **dict.fromkeys(('owner_record_bytes_blake3', 'owner_full_chunk_bytes_blake3',
                         'physical_layout_blake3', 'free_stack_blake3'), 'a' * 64),
        'normal_ranges': [[0, 32], [64, 1]], 'raw_index_escape_scope': 'synthetic audited scope'}
    report.update(canonical_layout_input_policy=prep['schema'], canonical_brick_layout=prep)
    for endpoint in ('input_before', 'input_after'):
        report[endpoint]['canonical_brick_layout'] = copy.deepcopy(prep)
    return report


class CanonicalTraceSourceTests(unittest.TestCase):
    def source_model(self, root, raw):
        contract = tool.canonical_contract()
        for name, value in raw.items():
            path = root / name
            path.parent.mkdir(parents=True, exist_ok=True)
            path.write_bytes(value)
        files = {name: contract.bytes_sha(value) for name, value in raw.items()}
        source = {'commit': 'a' * 40, 'files': files,
                  'content_sha256': tool.counts.frozen_tools.signature(files)}
        path = root/'headless-builds/frozen/source.json'
        tool.runner.dump(path, source)
        identity = {'source_json_sha256': tool.counts.frozen_tools.digest(path),
                    'source_content_sha256': source['content_sha256']}
        return source, path, identity

    def test_frozen_source_archive_contains_exact_bytes_and_is_not_a_metadata_rewrite(self):
        # The shared contract separately tests the real reviewed literal bodies.
        # Here a narrow fake checks the wrapper passes frozen hashes AND raw
        # bytes through that API and retains its result without mutation.
        contract = tool.canonical_contract()
        with tempfile.TemporaryDirectory() as temporary:
            root = Path(temporary)
            raw = {'test.rs': b'raw\r\nsource bytes\n'}
            _, path, identity = self.source_model(root, raw)
            before = path.read_bytes()
            def check(files, values):
                self.assertEqual(files, {name: contract.bytes_sha(value) for name, value in raw.items()})
                self.assertEqual(values, raw)
                return {'schema': 'synthetic-rich-reviewed-contract', 'harness': None}
            fake = SimpleNamespace(REQUIRED=('test.rs',), rich_source_contract=check, bytes_sha=contract.bytes_sha)
            with patch.object(tool.runner, 'PERF', root), patch.object(tool.runner, 'ROOT', root), \
                 patch.object(tool, 'canonical_contract', return_value=fake):
                evidence = tool.canonical_source_evidence(SimpleNamespace(build_label='frozen'), identity)
            self.assertEqual(path.read_bytes(), before)
            self.assertIsNone(evidence['contract']['harness'])
            self.assertEqual(base64.b64decode(evidence['files']['test.rs']['raw_base64']), raw['test.rs'])
            self.assertEqual(evidence['files']['test.rs']['sha256'], contract.bytes_sha(raw['test.rs']))
            self.assertIn('owner checks still required', evidence['scope'])

    def test_changed_source_inventory_is_refused_before_a_capability_read(self):
        with tempfile.TemporaryDirectory() as temporary:
            root = Path(temporary)
            _, path, identity = self.source_model(root, {'test.rs': b'bytes'})
            path.write_bytes(path.read_bytes() + b' ')
            with patch.object(tool.runner, 'PERF', root), \
                 patch.object(tool, 'canonical_contract') as contract:
                with self.assertRaisesRegex(ValueError, 'source inventory changed'):
                    tool.canonical_source_evidence(SimpleNamespace(build_label='frozen'), identity)
                contract.assert_not_called()

    def test_real_shared_contract_refuses_old_caller_and_changed_helper_in_frozen_inventory(self):
        contract = tool.canonical_contract()
        # Pin failures are checked before literal-body validation, so no full
        # source snapshot is needed for these intentionally invalid archives.
        for changed in ('crates/tiny-glade/tests/cpu_measurement/mod.rs',
                        'crates/tiny-glade/tests/support/performance_sparse_layout.rs'):
            with self.subTest(changed=changed), tempfile.TemporaryDirectory() as temporary:
                root = Path(temporary)
                raw = {name: b'intentionally invalid source\n' for name in contract.REQUIRED}
                source, path, _ = self.source_model(root, raw)
                source['files'].update(contract.SHARED)
                source['files']['crates/tiny-glade/tests/cpu_measurement/mod.rs'] = contract.RICH_CALLER_SHA256
                source['files'][changed] = '0' * 64
                source['content_sha256'] = tool.counts.frozen_tools.signature(source['files'])
                tool.runner.dump(path, source)
                identity = {'source_json_sha256': tool.counts.frozen_tools.digest(path),
                            'source_content_sha256': source['content_sha256']}
                with patch.object(tool.runner, 'PERF', root), patch.object(tool.runner, 'ROOT', root):
                    with self.assertRaisesRegex(ValueError, 'shared input source bytes differ'):
                        tool.canonical_source_evidence(SimpleNamespace(build_label='frozen'), identity)

    def test_opt_in_added_after_cleanup_both_roles_and_inherited_flag_never_enables_default(self):
        original = tool.runner.environment
        with patch.dict(os.environ, {'TG_PERF_CANONICAL_WALL_LAYOUT': '1'}, clear=False):
            for requested in (False, True):
                for enabled in (False, True):
                    with self.subTest(requested=requested, enabled=enabled), \
                         tool.trace_environment(requested), tool.counts.requested_environment(enabled):
                        env = tool.runner.environment(True)
                        self.assertEqual(env['TG_BINDING_SCOPE_REUSE'], str(int(enabled)))
                        self.assertEqual(env['TG_CPU_BILL'], '1')
                        self.assertEqual(env['RHAPSODY_SUBMIT_TRACE'], '1')
                        self.assertEqual(env.get('TG_PERF_CANONICAL_WALL_LAYOUT'), '1' if requested else None)
        self.assertIs(tool.runner.environment, original)

    def test_optional_source_dependency_is_not_loaded_for_default_tool_identity(self):
        with patch.object(tool, 'canonical_contract', side_effect=AssertionError('not requested')):
            self.assertNotIn('perf_canonical_contract.py', tool.tool_identities())

    def test_optional_contract_module_must_match_its_reviewed_bytes(self):
        with patch.object(tool.counts.frozen_tools, 'digest', return_value='0' * 64):
            with self.assertRaisesRegex(ValueError, 'unreviewed canonical source/stamp contract'):
                tool.canonical_contract()

    def test_preloaded_foreign_contract_is_rejected_even_with_identical_bytes(self):
        original = tool.canonical_contract()
        with tempfile.TemporaryDirectory() as temporary:
            foreign = Path(temporary)/'perf_canonical_contract.py'
            foreign.write_bytes(Path(original.__file__).read_bytes())
            with patch.dict(sys.modules, {'perf_canonical_contract': SimpleNamespace(__file__=str(foreign))}):
                with self.assertRaisesRegex(ValueError, 'not the reviewed sibling'):
                    tool.canonical_contract()

    def test_raw_preparation_is_retained_and_empty_owner_lanes_are_valid(self):
        report = canonical_fixture(False)
        before = copy.deepcopy(report)
        evidence = tool.canonical_preparation(report)
        self.assertEqual(report, before)
        self.assertGreater(evidence['canonical_brick_layout']['owner_count'],
                           evidence['canonical_brick_layout']['chunk_count'])
        self.assertEqual(evidence['input_before_canonical_brick_layout'], report['canonical_brick_layout'])
        self.assertEqual(evidence['input_after_canonical_brick_layout'], report['canonical_brick_layout'])
        self.assertIn('not a final/current', evidence['scope'])
        self.assertNotIn('complete', evidence['canonical_brick_layout'])

    def test_missing_runtime_marker_or_real_stamp_fields_cannot_be_inferred_from_source(self):
        for mutation in ('policy', 'old_schema', 'wrong_phase', 'zero_owners', 'bool_count',
                         'impossible_records', 'huge_chunks', 'huge_records',
                         'digest', 'overlap', 'bounds', 'visible_total', 'before', 'after', 'clean'):
            report = canonical_fixture(False)
            prep = report['canonical_brick_layout']
            if mutation == 'policy': report.pop('canonical_layout_input_policy')
            elif mutation == 'old_schema': prep['schema'] = 'prep-v1'
            elif mutation == 'wrong_phase': prep['phase'] = 'after_warmup'
            elif mutation == 'zero_owners': prep['owner_count'] = 0
            elif mutation == 'bool_count': prep['owner_count'] = True
            elif mutation == 'impossible_records': prep['used_records'] = 257
            elif mutation == 'huge_chunks': prep['chunk_count'] = 0x10001
            elif mutation == 'huge_records': prep['used_records'] = 0x200001
            elif mutation == 'digest': prep['owner_full_chunk_bytes_blake3'] = 'g' * 64
            elif mutation == 'overlap': prep['normal_ranges'] = [[0, 32], [31, 1]]
            elif mutation == 'bounds': prep['normal_ranges'] = [[0x200000, 1]]
            elif mutation == 'visible_total': prep['normal_ranges'] = [[0, 130]]
            elif mutation == 'before': report['input_before'].pop('canonical_brick_layout')
            elif mutation == 'after': report['input_after']['canonical_brick_layout']['physical_layout_blake3'] = 'b' * 64
            else: report['clean_cpu_acceptance_eligible'] = True
            # Keep endpoints consistent for type/range failures: reject the
            # invalid value itself, not just its different endpoint copy.
            if mutation not in ('before', 'after'):
                for endpoint in ('input_before', 'input_after'):
                    report[endpoint]['canonical_brick_layout'] = copy.deepcopy(prep)
            with self.subTest(mutation=mutation), self.assertRaises(ValueError):
                tool.canonical_preparation(report)

    def test_cross_process_preparation_difference_is_semantic_input_failure(self):
        off, on = canonical_fixture(False), canonical_fixture(True)
        on['canonical_brick_layout']['physical_layout_blake3'] = 'b' * 64
        for endpoint in ('input_before', 'input_after'):
            on[endpoint]['canonical_brick_layout'] = copy.deepcopy(on['canonical_brick_layout'])
        with self.assertRaisesRegex(ValueError, 'semantic input'):
            tool.compare_sources(off, on, canonical_layout=True)

    def test_canonical_prep_does_not_hide_measured_range_or_native_work_differences(self):
        off, on = canonical_fixture(False), canonical_fixture(True)
        on['actual_workload']['frames'][1]['native_commands']['direct_dispatch_api_calls'] += 2
        on['wall_dispatch_source_bill']['observations'][1]['source']['normal'][1][0] += 32
        result = tool.compare_sources(off, on, canonical_layout=True)
        self.assertEqual(result['actual_native_differences'][0]['on'] - result['actual_native_differences'][0]['off'], 2)
        self.assertFalse(result['normalized_actual_workload_equal'])
        self.assertFalse(result['wall_source_equal'])
        with self.assertRaises(ValueError): tool.counts.compare_reports(off, on)

    def test_both_requested_captures_retain_raw_stamp_and_no_clean_acceptance(self):
        with tempfile.TemporaryDirectory() as temporary:
            root = Path(temporary)
            args = SimpleNamespace(build_label='frozen', series='canonical', scene='static',
                                   warmup=60, samples=2, timeout=30, canonical_layout=True)
            for role in ('off', 'on'):
                path = root / 'headless-runs' / ('canonical-' + role)
                expected = canonical_fixture(role == 'on')
                def capture(arguments):
                    env = tool.runner.environment(arguments.diagnostic)
                    self.assertEqual(env['TG_PERF_CANONICAL_WALL_LAYOUT'], '1')
                    self.assertEqual(env['TG_BINDING_SCOPE_REUSE'], str(int(role == 'on')))
                    path.mkdir(parents=True)
                    raw = {'valid': False, 'diagnostic': True, 'etw': False, 'process_id': 7,
                        'executable_sha256': 'b' * 64,
                        'build': {'executable': str(root/'frozen.exe'), 'source_content_sha256': 'a' * 64}}
                    tool.runner.dump(path/'run.json', raw)
                    tool.runner.dump(path/'session.json', expected)
                    (path/'fixture.log').write_text(native_log(), encoding='utf-8')
                    return raw
                identity = {'source_content_sha256': 'a' * 64, 'files': {'frozen.exe': 'b' * 64}}
                with patch.object(tool.runner, 'PERF', root), \
                     patch.object(tool.runner, 'capture', side_effect=capture), \
                     patch.object(tool.counts.frozen_tools, 'frozen_identity', return_value=({}, identity)), \
                     patch.object(tool.counts, 'tool_identities', return_value={}), \
                     patch.object(tool, 'tool_identities', return_value={}), \
                     patch.object(tool, 'canonical_source_evidence', return_value={'source': 'audited'}) as source:
                    report, record = tool.capture_role(args, role, identity, {}, {'source': 'audited'})
                self.assertEqual(source.call_count, 2)
                self.assertEqual(report, expected)
                self.assertFalse(record['raw_run_valid'])
                self.assertEqual(record['canonical_preparation']['canonical_brick_layout'], expected['canonical_brick_layout'])
                self.assertEqual(json.loads((path/'session.json').read_text()), expected)
                self.assertFalse(json.loads((path/'run.json').read_text())['valid'])

    def test_unreviewed_frozen_capability_is_refused_before_first_child_and_summary_stays_false(self):
        with tempfile.TemporaryDirectory() as temporary:
            root = Path(temporary)
            args = SimpleNamespace(build_label='old', series='no-capability', scene='static',
                                   warmup=60, samples=30, timeout=30, canonical_layout=True)
            with patch.object(tool.runner, 'PERF', root), \
                 patch.object(tool.counts.frozen_tools, 'frozen_identity', return_value=({}, {})), \
                 patch.object(tool, 'tool_identities', return_value={}), \
                 patch.object(tool, 'canonical_source_evidence', side_effect=ValueError('unreviewed source')), \
                 patch.object(tool, 'capture_role') as child:
                with self.assertRaisesRegex(ValueError, 'unreviewed source'): tool.run(args)
                child.assert_not_called()
            summary = json.loads((root/'binding-trace-source/no-capability/summary.json').read_text())
            self.assertFalse(summary['valid'])
            self.assertFalse(summary['clean_cpu_acceptance'])
            self.assertFalse(summary['original_count_comparison_reclassified'])
            self.assertEqual(summary['runs'], [])

    def test_ignored_runtime_request_fails_with_original_raw_evidence_sealed(self):
        with tempfile.TemporaryDirectory() as temporary:
            root = Path(temporary)
            path = root/'headless-runs/no-stamp-off'
            args = SimpleNamespace(series='no-stamp', canonical_layout=True)
            raw_report = fixture(False)
            def capture(*arguments):
                path.mkdir(parents=True)
                tool.runner.dump(path/'run.json', {'valid': False})
                tool.runner.dump(path/'session.json', raw_report)
                (path/'fixture.log').write_text(native_log(), encoding='utf-8')
                return raw_report, {'artifacts': tool.counts.seal_capture(path)}
            with patch.object(tool.runner, 'PERF', root), \
                 patch.object(tool, 'tool_identities', return_value={}), \
                 patch.object(tool, 'canonical_source_evidence', return_value={'reviewed': True}), \
                 patch.object(tool.counts, 'capture_mode', side_effect=capture):
                with self.assertRaisesRegex(ValueError, 'policy was not honored'):
                    tool.capture_role(args, 'off', {}, {}, {'reviewed': True})
            self.assertEqual(json.loads((path/'session.json').read_text()), raw_report)
            self.assertFalse(json.loads((path/'run.json').read_text())['valid'])
            sealed = json.loads((path/'trace-source-artifacts.json').read_text())
            self.assertEqual(set(sealed['artifacts']), {'run.json', 'session.json', 'fixture.log'})
            for name, digest in sealed['artifacts'].items():
                self.assertEqual(tool.counts.frozen_tools.digest(path/name), digest)

    def test_successful_synthetic_series_archives_source_without_any_acceptance_claim(self):
        with tempfile.TemporaryDirectory() as temporary:
            root = Path(temporary)
            args = SimpleNamespace(build_label='frozen', series='canonical-series', scene='static',
                                   warmup=60, samples=30, timeout=30, canonical_layout=True)
            evidence = {'schema': 'synthetic-archive', 'contract': {'harness': None}, 'files': {}}
            def capture(arguments, role, identity, tools, canonical_source):
                self.assertEqual(canonical_source, evidence)
                return canonical_fixture(role == 'on'), {'raw_run_valid': False,
                    'canonical_preparation': tool.canonical_preparation(canonical_fixture(role == 'on'))}
            with patch.object(tool.runner, 'PERF', root), \
                 patch.object(tool.counts.frozen_tools, 'frozen_identity', return_value=({}, {})), \
                 patch.object(tool, 'tool_identities', return_value={}), \
                 patch.object(tool, 'canonical_source_evidence', return_value=evidence), \
                 patch.object(tool, 'capture_role', side_effect=capture):
                result = tool.run(args)
            archive = result['canonical_source_archive']
            self.assertEqual(json.loads(Path(archive['path']).read_text()), evidence)
            self.assertEqual(tool.counts.frozen_tools.digest(Path(archive['path'])), archive['sha256'])
            self.assertEqual(result['settings']['TG_PERF_CANONICAL_WALL_LAYOUT'], '1')
            self.assertTrue(result['valid'])
            for key in ('clean_cpu_acceptance', 'cpu_performance_evidence', 'production_16ms_acceptance',
                        'formal_source_A_B', 'original_count_comparison_reclassified'):
                self.assertFalse(result[key])


if __name__ == '__main__':
    unittest.main()
