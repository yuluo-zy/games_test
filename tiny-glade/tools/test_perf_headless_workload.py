"""Synthetic strict-contract tests; no actual benchmark/capture is started."""
import copy
import json
from pathlib import Path
import tempfile
import unittest

import perf_headless_workload as work


def save(path, value):
    Path(path).write_text(json.dumps(value), encoding='utf-8')


def reference(path):
    return {'path': str(path), 'sha256': work.digest(path)}


def collect(tasks=2, waves=1):
    return dict(valid=True, overflow=False, tasks=tasks, waves=waves,
        cross_pass_tasks=0, cross_pass_waves=0, failed_waves=0)


def device(n, generation=1):
    return {'valid': True, 'generation': generation, **dict.fromkeys(work.DEVICE_BYTES, n)}


def actual_fixture(samples=2):
    frames, cpu = [], []
    for index in range(samples+1):
        frame = {'world_tick': index+100, 'receipt': index+101, 'attempt': index+101,
            'valid': True, 'native_commands': {'valid': True,
                **dict.fromkeys((*work.DRAW_APIS, *work.DISPATCH_APIS), 1),
                **dict.fromkeys(work.GRAPH_BYTES, 8)},
            **dict.fromkeys(work.GRAPH_COST, 2), 'graph_retained_passes': 6,
            'draw_list_lengths': [2]*21, 'collect': collect(),
            'collect_cumulative': collect(tasks=2*(index+1), waves=index+1),
            'queued_buffer_h2d_submitted_jobs': 1,
            'queued_buffer_h2d_submitted_payload_bytes': 16,
            'queued_upload_snapshot': {'manager_id': 7, 'submitted_jobs': index+1,
                                      'submitted_payload_bytes': 16*(index+1)},
            'device_upload_submitted_payload': device(8),
            'device_upload_snapshot': device(8*(index+1))}
        frames.append(frame)
        if index:
            cpu.append({**{key: frame[key] for key in ('world_tick', 'receipt', 'attempt')}, 'app_epoch': 0})
    raw = {'schema': work.SCHEMA, 'valid': True, 'errors': [], 'scope': 'synthetic-only',
        'coverage': {'included': ['synthetic-paths-for-unit-test-only'],
            'excluded': ['mapped_cpu_writes', 'device_to_host_readback', 'clear_fill_commands'],
            'no_all_upload_total': True, 'collect_cross_pass_is_subset': True,
            'graph_chunks_are_not_collect_tasks': True},
        'capacity': samples+1, 'frame_count': samples+1, 'truncated': False,
        'queued_upload_before_measurement': {'manager_id': 0, 'submitted_jobs': 0, 'submitted_payload_bytes': 0},
        'queued_upload_end': frames[-1]['queued_upload_snapshot'],
        'device_upload_before_measurement': device(0),
        'device_upload_end': frames[-1]['device_upload_snapshot'],
        'collect_session_end': frames[-1]['collect_cumulative'], 'frames': frames}
    session = {'samples': samples, 'scenario': 'static', 'actual_workload': raw,
        'frames': [{key: f[key] for key in ('world_tick', 'receipt', 'attempt')} for f in frames]}
    return session, {'frames': cpu}, {'coverage': copy.deepcopy(raw['coverage'])}


class ActualTests(unittest.TestCase):
    def setUp(self):
        self.session, self.cpu, self.cap = actual_fixture()

    def validate(self):
        return work.validate_actual(self.session, self.cpu, self.cap)

    def test_valid_current_schema_still_pending_complete_coverage(self):
        result = self.validate()
        self.assertEqual(len(result['semantic_frames']), 2)
        self.assertTrue(result['coverage_pending'])
        self.assertIn('not physical GPU draw counts', result['native_api_note'])

    def test_legacy_missing_actual_is_not_zero(self):
        self.session['actual_workload'] = None
        with self.assertRaisesRegex(ValueError, 'absent'): self.validate()

    def test_raw_valid_false_is_rejected(self):
        self.session['actual_workload']['valid'] = False
        with self.assertRaisesRegex(ValueError, 'invalid'): self.validate()

    def test_raw_overflow_is_rejected(self):
        self.session['actual_workload']['overflow'] = True
        with self.assertRaisesRegex(ValueError, 'overflow'): self.validate()

    def test_raw_error_not_discarded(self):
        self.session['actual_workload']['errors'] = ['missing receipt']
        with self.assertRaisesRegex(ValueError, 'errors'): self.validate()

    def test_missing_error_array_not_assumed_empty(self):
        del self.session['actual_workload']['errors']
        with self.assertRaisesRegex(ValueError, 'errors'): self.validate()

    def test_native_invalid_not_discarded(self):
        self.session['actual_workload']['frames'][0]['native_commands']['valid'] = False
        with self.assertRaisesRegex(ValueError, 'invalid'): self.validate()

    def test_native_error_array_not_discarded(self):
        self.session['actual_workload']['frames'][1]['native_commands']['errors'] = ['overflow']
        with self.assertRaisesRegex(ValueError, 'errors'): self.validate()

    def test_row_invalid_rejected(self):
        self.session['actual_workload']['frames'][1]['valid'] = False
        with self.assertRaisesRegex(ValueError, 'invalid'): self.validate()

    def test_truncation_rejected(self):
        self.session['actual_workload']['truncated'] = True
        with self.assertRaisesRegex(ValueError, 'truncation'): self.validate()

    def test_capacity_mismatch_rejected(self):
        self.session['actual_workload']['capacity'] += 1
        with self.assertRaisesRegex(ValueError, 'capacity'): self.validate()

    def test_receipt_mismatch_rejected(self):
        self.session['actual_workload']['frames'][1]['receipt'] += 1
        with self.assertRaisesRegex(ValueError, 'receipt'): self.validate()

    def test_boolean_count_is_not_integer_work(self):
        self.session['actual_workload']['frames'][1]['native_commands']['indexed_draw_api_calls'] = True
        with self.assertRaisesRegex(ValueError, 'u64'): self.validate()

    def test_negative_count_rejected(self):
        self.session['actual_workload']['frames'][1]['graph_retained_passes'] = -1
        with self.assertRaisesRegex(ValueError, 'u64'): self.validate()

    def test_collect_failed_waves_rejected(self):
        self.session['actual_workload']['frames'][1]['collect']['failed_waves'] = 1
        with self.assertRaisesRegex(ValueError, 'failed'): self.validate()

    def test_collect_overflow_rejected(self):
        self.session['actual_workload']['frames'][1]['collect']['overflow'] = True
        with self.assertRaisesRegex(ValueError, 'overflow'): self.validate()

    def test_crosspass_is_subset_not_added(self):
        row = self.session['actual_workload']['frames'][1]
        row['collect']['cross_pass_tasks'] = 3; row['collect']['cross_pass_waves'] = 1
        with self.assertRaisesRegex(ValueError, 'subset'): self.validate()

    def test_more_than_eight_tasks_per_wave_rejected(self):
        self.session['actual_workload']['frames'][1]['collect']['tasks'] = 9
        with self.assertRaisesRegex(ValueError, 'wave bounds'): self.validate()

    def test_collect_cumulative_rollback_rejected(self):
        self.session['actual_workload']['frames'][1]['collect_cumulative']['tasks'] = 1
        with self.assertRaisesRegex(ValueError, 'cumulative'): self.validate()

    def test_unobserved_collect_tail_rejected(self):
        raw = self.session['actual_workload']; raw['collect_session_end'] = copy.deepcopy(raw['collect_session_end'])
        raw['collect_session_end']['tasks'] += 1
        with self.assertRaisesRegex(ValueError, 'unobserved'): self.validate()

    def test_end_cumulative_boolean_cannot_equal_numeric_zero(self):
        raw = self.session['actual_workload']; raw['collect_session_end'] = copy.deepcopy(raw['collect_session_end'])
        raw['collect_session_end']['failed_waves'] = False
        with self.assertRaisesRegex(ValueError, 'u64'): self.validate()

    def test_end_generation_boolean_cannot_equal_one(self):
        raw = self.session['actual_workload']; raw['device_upload_end'] = copy.deepcopy(raw['device_upload_end'])
        raw['device_upload_end']['generation'] = True
        with self.assertRaisesRegex(ValueError, 'u64'): self.validate()

    def test_manager_replacement_rejected(self):
        self.session['actual_workload']['frames'][1]['queued_upload_snapshot']['manager_id'] = 8
        with self.assertRaisesRegex(ValueError, 'manager changed'): self.validate()

    def test_lazy_manager_zero_to_nonzero_is_valid(self):
        self.assertTrue(self.validate()['coverage_pending'])

    def test_invalid_zero_manager_rejected(self):
        self.session['actual_workload']['frames'][1]['queued_upload_snapshot']['manager_id'] = 0
        with self.assertRaisesRegex(ValueError, 'zero manager'): self.validate()

    def test_explicit_queued_invalid_is_not_swallowed_by_optional_valid(self):
        self.session['actual_workload']['frames'][1]['queued_upload_snapshot']['valid'] = False
        with self.assertRaisesRegex(ValueError, 'invalid'): self.validate()

    def test_upload_success_prefix_wrong_delta_rejected(self):
        self.session['actual_workload']['frames'][1]['queued_buffer_h2d_submitted_payload_bytes'] = 0
        with self.assertRaisesRegex(ValueError, 'delta mismatch'): self.validate()

    def test_device_generation_replacement_rejected(self):
        self.session['actual_workload']['frames'][1]['device_upload_snapshot']['generation'] = 8
        with self.assertRaisesRegex(ValueError, 'generation changed'): self.validate()

    def test_device_delta_none_is_not_zero(self):
        self.session['actual_workload']['frames'][1]['device_upload_submitted_payload'] = None
        with self.assertRaisesRegex(ValueError, 'missing producer'): self.validate()

    def test_device_sentinel_rejected_even_with_valid_true(self):
        self.session['actual_workload']['frames'][1]['device_upload_snapshot'][work.DEVICE_BYTES[0]] = (1 << 64)-1
        with self.assertRaisesRegex(ValueError, 'sentinel'): self.validate()

    def test_draw_list_size_missing_rejected(self):
        self.session['actual_workload']['frames'][1]['draw_list_lengths'].pop()
        with self.assertRaisesRegex(ValueError, '21 actual'): self.validate()

    def test_many_draws_requires1000_real_entries(self):
        self.session['scenario'] = 'many_draws'
        with self.assertRaisesRegex(ValueError, 'below1000'): self.validate()

    def test_claiming_all_uploads_without_new_schema_review_rejected(self):
        self.session['actual_workload']['coverage']['no_all_upload_total'] = False
        self.cap['coverage'] = copy.deepcopy(self.session['actual_workload']['coverage'])
        with self.assertRaisesRegex(ValueError, 'completeness'): self.validate()

    def test_native_api_difference_is_semantic_hard_mismatch(self):
        a = self.validate(); b = copy.deepcopy(a)
        b['semantic_frames'][0]['native_draw_api_calls'][work.DRAW_APIS[0]] += 1
        with self.assertRaisesRegex(ValueError, 'native draw API'): work.compare_workload(a, b)

    def test_retained_pass_difference_hard_mismatch(self):
        a = self.validate(); b = copy.deepcopy(a); b['semantic_frames'][0]['graph_retained_passes'] -= 1
        with self.assertRaisesRegex(ValueError, 'pass'): work.compare_workload(a, b)

    def test_task_reduction_is_cost_difference_requiring_review(self):
        a = self.validate(); b = copy.deepcopy(a); b['cost_frames'][0]['collect']['tasks'] = 1
        result = work.compare_workload(a, b, 'removed redundant tasks')
        self.assertTrue(result['cost_difference_review_required'])
        self.assertFalse(result['explanation_is_approval'])

    def test_payload_difference_cannot_be_green_by_free_text(self):
        a = self.validate(); b = copy.deepcopy(a)
        b['payload_frames'][0]['queued_buffer_h2d_submitted_payload_bytes'] = 0
        result = work.compare_workload(a, b, 'trust this optimization')
        self.assertTrue(result['payload_review_required'])
        self.assertFalse(result['h2d_d2d_payload_equal'])
        self.assertFalse(result['explanation_is_approval'])

    def test_process_local_ids_are_not_paired_semantic_fields(self):
        a = self.validate()
        raw = self.session['actual_workload']
        for row in raw['frames']: row['queued_upload_snapshot']['manager_id'] = 99
        raw['queued_upload_end'] = raw['frames'][-1]['queued_upload_snapshot']
        for obj in [raw['device_upload_before_measurement'], raw['device_upload_end']]: obj['generation'] = 13
        for row in raw['frames']:
            row['device_upload_snapshot']['generation'] = 13; row['device_upload_submitted_payload']['generation'] = 13
        b = self.validate()
        self.assertTrue(work.compare_workload(a, b)['cpu_task_and_recording_cost_equal'])


class CapabilityTests(unittest.TestCase):
    def setUp(self):
        self.temp = tempfile.TemporaryDirectory(); self.addCleanup(self.temp.cleanup)
        self.root = Path(self.temp.name)
        source = self.root/'archived-producer.rs'; source.write_text('fn recorded() {}')
        self.sources = {'crates/producer.rs': work.digest(source)}
        archives = {'crates/producer.rs': reference(source)}
        for index, name in enumerate(work.COMMON_HARNESS_FILES):
            archive = self.root/('common'+str(index)+'.rs')
            archive.write_text(work.ALGORITHM_SELECTION, encoding='utf-8')
            self.sources[name] = work.digest(archive); archives[name] = reference(archive)
        self.build = {'source_commit': 'a'*40, 'source_content_sha256': work.signature(self.sources)}
        fields = {key: {'meaning': value, 'symbols': ['recorded'], 'source_paths': ['crates/producer.rs']}
                  for key, value in work.FIELD_GROUPS.items()}
        _, _, cap = actual_fixture()
        report = {'schema': 'headless-producer-source-review-v1', **self.build,
            'source_sha256': self.sources, 'fields': fields, 'coverage': cap['coverage'], 'findings': [],
            'algorithm_selection': work.ALGORITHM_SELECTION, 'runtime_ablation_supported': False,
            'controller_disable_controls_absent_checked': True,
            'receipt_attribution_checked': True, 'successful_prefix_failure_invalid_checked': True,
            'counter_overflow_checked': True}
        save(self.root/'review.json', report)
        self.cap = {'schema': 'headless-producer-capability-v1', 'raw_schema': work.SCHEMA, **self.build,
            'source_files': archives, 'algorithm_selection': work.ALGORITHM_SELECTION,
            'review_report': reference(self.root/'review.json'), 'fields': fields, 'coverage': cap['coverage']}
        self.path = self.root/'capability.json'; save(self.path, self.cap)

    def validate(self):
        return work.review_capability(self.path, self.build, self.sources)

    def test_bound_review_archive_and_fields_are_required(self):
        self.assertEqual(self.validate(), self.cap)

    def test_arbitrary_symbol_string_no_longer_suffices(self):
        value = copy.deepcopy(self.cap); value['fields']['native_commands']['symbols'] = ['not_a_producer']
        save(self.path, value)
        with self.assertRaisesRegex(ValueError, 'field capability'): self.validate()

    def test_archived_source_tamper_rejected(self):
        (self.root/'archived-producer.rs').write_text('changed')
        with self.assertRaisesRegex(ValueError, 'SHA changed'): self.validate()

    def test_review_report_tamper_rejected(self):
        (self.root/'review.json').write_text('{}')
        with self.assertRaisesRegex(ValueError, 'SHA changed'): self.validate()

    def test_source_commit_swap_rejected(self):
        value = copy.deepcopy(self.cap); value['source_commit'] = 'b'*40; save(self.path, value)
        with self.assertRaisesRegex(ValueError, 'commit'): self.validate()

    def test_review_of_other_source_rejected(self):
        value = copy.deepcopy(self.cap); value['source_content_sha256'] = '0'*64; save(self.path, value)
        with self.assertRaisesRegex(ValueError, 'another frozen source'): self.validate()

    def test_mapping_outside_reviewed_source_set_rejected(self):
        cap = copy.deepcopy(self.cap); report = work.read(self.root/'review.json')
        cap['fields']['native_commands']['source_paths'] = ['other.rs']
        report['fields'] = cap['fields']; save(self.root/'review.json', report)
        cap['review_report'] = reference(self.root/'review.json'); save(self.path, cap)
        with self.assertRaisesRegex(ValueError, 'unbound'): self.validate()

    def test_partial_source_coverage_cannot_be_renamed_to_total(self):
        value = copy.deepcopy(self.cap); value['coverage']['no_all_upload_total'] = False; save(self.path, value)
        with self.assertRaisesRegex(ValueError, 'coverage differs'): self.validate()


if __name__ == '__main__':
    unittest.main()
