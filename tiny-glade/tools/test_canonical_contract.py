"""Synthetic cold contract tests. No Cargo, EXE, GPU, ETW or actual source writes."""
import base64
import copy
import hashlib
import importlib.util
import json
import os
from pathlib import Path
import subprocess
import sys
import tempfile
import unittest
from unittest.mock import patch

HERE = Path(__file__).resolve().parent
ACTUAL_TOOLS = Path('C:/game/tools')
sys.path.insert(0, str(ACTUAL_TOOLS))
sys.path.insert(0, str(HERE))
import perf_canonical_contract as canonical
import perf_headless_series as series
import perf_headless_workload as workload
import perf_import_frozen_bundle_canonical as importer
import test_perf_headless_series_controller as old_controller
from test_perf_headless_workload import actual_fixture

RAW = {name: base64.b64decode(value) for name, value in
       json.loads((HERE/'fixtures/canonical_cpu_harness/source.json').read_text(encoding='utf-8')).items()}


def save(path, value):
    Path(path).parent.mkdir(parents=True, exist_ok=True)
    Path(path).write_text(json.dumps(value), encoding='utf-8', newline='\n')


def raw_source():
    files = {name: canonical.bytes_sha(raw) for name, raw in RAW.items()}
    return {'commit': 'a'*40, 'content_sha256': canonical.signature(files), 'status': '', 'files': files}


def preparation():
    return {'schema': canonical.PREPARATION, 'phase': 'fresh_boot_before_first_gameplay_warmup',
        'owner_count': 7, 'chunk_count': 2, 'used_records': 34,
        **dict.fromkeys(('owner_record_bytes_blake3', 'owner_full_chunk_bytes_blake3',
                         'physical_layout_blake3', 'free_stack_blake3'), 'a'*64),
        'normal_ranges': [[32, 32], [96, 2]], 'raw_index_escape_scope': 'synthetic audited scope'}


class ContractTests(unittest.TestCase):
    def test_embedded_importer_contract_is_same_reviewed_code(self):
        self.assertEqual(importer.COMMON4, canonical.COMMON4)
        self.assertEqual(importer.SHARED, canonical.SHARED)
        self.assertEqual(importer.GETTERS, canonical.GETTERS)
        self.assertEqual(importer.REQUIRED, canonical.REQUIRED)
        self.assertEqual(importer.OBLIGATIONS, canonical.OBLIGATIONS)
        standalone = Path(importer.__file__).read_text(encoding='utf-8')
        module = Path(canonical.__file__).read_text(encoding='utf-8')
        self.assertIn(module[module.index('import base64'):].rstrip(), standalone)

    def test_fourth_path_is_support_and_common_v1_constants_unchanged(self):
        self.assertEqual(len(canonical.COMMON4), 4)
        self.assertIn('crates/tiny-glade/tests/support/performance_sparse_layout.rs', canonical.COMMON4)
        self.assertEqual(len(workload.COMMON_HARNESS_FILES), 3)
        self.assertEqual(series.IMPORTER_SHA256, 'cab6757fffd00eecd7eb786b293b9999c1d99d34bfea48fe359b0af7f767b6d1')
        self.assertEqual(hashlib.sha256((ACTUAL_TOOLS/'fixtures/frozen_bundle_importer.py').read_bytes()).hexdigest(),
                         series.IMPORTER_SHA256)

    def test_source_contract_preserves_full_manifest_and_own_getter_hashes(self):
        value = canonical.source_contract(raw_source()['files'], RAW)
        self.assertEqual(value['common_source_sha256'], canonical.COMMON4)
        self.assertEqual(set(value['own_getter_source_sha256']), set(canonical.GETTERS))
        self.assertEqual(set(value['reviewed_source_sha256']), set(canonical.REQUIRED))

    def test_missing_fourth_source_or_wrong_frozen_hash_rejects(self):
        raw = dict(RAW); del raw[next(name for name in raw if name.endswith('performance_sparse_layout.rs'))]
        with self.assertRaisesRegex(ValueError, 'closure'): canonical.source_contract(raw_source()['files'], raw)
        files = dict(raw_source()['files']); files[next(iter(canonical.COMMON4))] = '0'*64
        with self.assertRaisesRegex(ValueError, 'shared input'): canonical.source_contract(files, RAW)

    def test_same_metadata_cannot_cover_changed_getter_or_boot(self):
        for name in (next(iter(canonical.GETTERS)), canonical.BOOT_PATH):
            raw = dict(RAW); raw[name] = raw[name].replace(b'fn ', b'fn changed_', 1)
            files = {key: canonical.bytes_sha(value) for key, value in raw.items()}
            with self.subTest(name=name), self.assertRaisesRegex(ValueError, 'getter body|boot body'):
                canonical.source_contract(files, raw)

    def test_unrelated_test_bytes_may_differ_but_own_source_stays_bound(self):
        raw = dict(RAW); raw[canonical.BOOT_PATH] += b'\n// another source has an unrelated test\n'
        files = {key: canonical.bytes_sha(value) for key, value in raw.items()}
        result = canonical.source_contract(files, raw)
        self.assertNotEqual(result['own_boot_source_sha256'], raw_source()['files'][canonical.BOOT_PATH])
        self.assertEqual(result['boot_body_sha256'], canonical.bytes_sha(canonical.BOOT.encode()))

    def test_preparation_allows_empty_owner_lanes_but_is_not_final_layout(self):
        prep = preparation()
        session = {'harness': canonical.HARNESS, 'canonical_layout_input_policy': canonical.PREPARATION,
            'canonical_brick_layout': prep, 'input_before': {'canonical_brick_layout': prep},
            'input_after': {'canonical_brick_layout': copy.deepcopy(prep)}}
        self.assertGreater(prep['owner_count'], prep['chunk_count'])
        self.assertEqual(canonical.session_preparation(session), prep)
        session['input_after']['canonical_brick_layout']['used_records'] += 1
        with self.assertRaisesRegex(ValueError, 'endpoint'): canonical.session_preparation(session)

    def test_legacy_runtime_cannot_be_upgraded_by_new_annotation(self):
        session, cpu, cap = actual_fixture()
        session['canonical_brick_layout'] = preparation()
        with self.assertRaisesRegex(ValueError, 'historical v1'): workload.validate_actual(session, cpu, cap)

    def test_preparation_refuses_impossible_typed_capacities(self):
        for key, value in (('chunk_count', 0x10001), ('used_records', 0x200001)):
            prep = preparation(); prep[key] = value
            with self.subTest(key=key), self.assertRaisesRegex(ValueError, 'typed chunk/record'):
                canonical.preparation_stamp(prep, {'canonical_brick_layout': prep},
                                            {'canonical_brick_layout': prep})

    def test_optional_rich_source_is_separate_diagnostic_contract(self):
        raw = dict(RAW)
        rich = HERE/'fixtures/canonical_cpu_harness/rich_optional_mod.rs'
        raw['crates/tiny-glade/tests/cpu_measurement/mod.rs'] = rich.read_bytes()
        files = {key: canonical.bytes_sha(value) for key, value in raw.items()}
        result = canonical.rich_source_contract(files, raw)
        self.assertEqual(result['schema'], 'rich-optional-canonical-source-diagnostic-v1')
        self.assertIsNone(result['harness'])
        with self.assertRaisesRegex(ValueError, 'shared input'): canonical.source_contract(files, raw)

    def test_new_preparation_never_removes_existing_coverage_pending(self):
        session, cpu, cap = actual_fixture(); prep = preparation()
        cap['canonical_source_contract'] = {'schema': canonical.CONTRACT}
        session.update(harness=canonical.HARNESS, canonical_layout_input_policy=canonical.PREPARATION,
                       canonical_brick_layout=prep, input_before={'canonical_brick_layout': prep},
                       input_after={'canonical_brick_layout': prep})
        result = workload.validate_actual(session, cpu, cap)
        self.assertEqual(len(result['coverage_pending']), 2)
        self.assertTrue(any('all-GPU-upload' in item for item in result['coverage_pending']))
        session['actual_workload']['frames'][1]['native_commands']['valid'] = False
        with self.assertRaisesRegex(ValueError, 'invalid'): workload.validate_actual(session, cpu, cap)


class CanonicalControllerTests(unittest.TestCase):
    """Use real controller plan/validation, with explicitly synthetic frozen binaries."""
    setUp = old_controller.ControllerTests.setUp
    plan = old_controller.ControllerTests.plan

    def make_build(self, label, commit):
        path = old_controller.ControllerTests.make_build(self, label, commit)
        folder = path.parent
        source = raw_source(); source['commit'] = commit
        archives = {}
        for number, (name, raw) in enumerate(RAW.items()):
            target = folder/('canonical-source-'+str(number)+'.rs'); target.write_bytes(raw)
            archives[name] = series.reference(target)
        save(folder/'source.json', source)
        build = series.read(path)
        build['source_content_sha256'] = source['content_sha256']
        build['management_controls']['source_content_sha256'] = source['content_sha256']
        save(path, build)
        _, _, model = actual_fixture()
        fields = {key: {'meaning': meaning, 'symbols': ['synthetic_source_review'],
                       'source_paths': [next(iter(canonical.COMMON4))]}
                  for key, meaning in workload.FIELD_GROUPS.items()}
        contract = canonical.source_contract(source['files'], RAW)
        review = {'schema': 'headless-producer-source-review-v2', 'source_commit': commit,
            'source_content_sha256': source['content_sha256'], 'source_sha256': source['files'],
            'fields': fields, 'coverage': model['coverage'], 'findings': [],
            'algorithm_selection': workload.ALGORITHM_SELECTION, 'runtime_ablation_supported': False,
            'controller_disable_controls_absent_checked': True, 'receipt_attribution_checked': True,
            'successful_prefix_failure_invalid_checked': True, 'counter_overflow_checked': True,
            'canonical_source_contract': contract, **dict.fromkeys(canonical.OBLIGATIONS, True)}
        save(folder/'producer-source-review.json', review)
        cap = {'schema': 'headless-producer-capability-v2', 'raw_schema': workload.SCHEMA,
            'source_commit': commit, 'source_content_sha256': source['content_sha256'],
            'source_files': archives, 'algorithm_selection': workload.ALGORITHM_SELECTION,
            'review_report': series.reference(folder/'producer-source-review.json'),
            'fields': fields, 'coverage': model['coverage'], 'canonical_source_contract': contract}
        save(folder/'producer-review.json', cap)
        return path

    def test_canonical_plan_and_cached_validation_recheck_full_archived_closure(self):
        plan, _ = self.plan()
        self.assertEqual(plan['harness'], canonical.HARNESS)
        self.assertEqual(plan['common_harness_source_sha256'], canonical.SHARED)
        series.validate_plan(plan)
        cap = series.read(self.a.with_name('producer-review.json'))
        helper = Path(cap['source_files']['crates/tiny-glade/tests/support/performance_sparse_layout.rs']['path'])
        helper.write_bytes(helper.read_bytes()+b'\n// changed after plan/audit\n')
        with self.assertRaisesRegex(ValueError, 'SHA changed'): series.validate_plan(plan)

    def test_missing_getter_archive_and_review_obligation_reject_before_plan(self):
        path = self.a.with_name('producer-review.json'); original = series.read(path)
        cap = copy.deepcopy(original); del cap['source_files'][next(iter(canonical.GETTERS))]; save(path, cap)
        with self.assertRaisesRegex(ValueError, 'exact producer|closure'): self.plan()
        save(path, original)
        report_path = self.a.with_name('producer-source-review.json'); report = series.read(report_path)
        report[canonical.OBLIGATIONS[0]] = False; save(report_path, report)
        original['review_report'] = series.reference(report_path); save(path, original)
        with self.assertRaisesRegex(ValueError, 'obligations'): self.plan()

    def test_mixed_versions_and_optional_rich_substitution_reject(self):
        # A canonical source cannot simply retag its capability/review v1.
        path = self.b.with_name('producer-review.json'); cap = series.read(path)
        review_path = self.b.with_name('producer-source-review.json'); report = series.read(review_path)
        cap['schema'] = 'headless-producer-capability-v1'; report['schema'] = 'headless-producer-source-review-v1'
        cap.pop('canonical_source_contract'); report.pop('canonical_source_contract')
        save(review_path, report); cap['review_report'] = series.reference(review_path); save(path, cap)
        with self.assertRaisesRegex(ValueError, 'canonical producer'): self.plan()

    def test_actual_v1_v2_pair_rejects_instead_of_selecting_intersection(self):
        self.b = old_controller.ControllerTests.make_build(self, 'actual-v1', 'c'*40)
        with self.assertRaisesRegex(ValueError, 'mixed historical/canonical'): self.plan()

    def test_new_import_contract_never_accepts_old_verifier_or_unbound_source(self):
        contract = canonical.source_contract(raw_source()['files'], RAW)
        with self.assertRaisesRegex(ValueError, 'mixed import'):
            series.check_import_harness({'schema': series.IMPORT_CONTRACT}, contract)
        with self.assertRaisesRegex(ValueError, 'archive differs'):
            series.check_import_harness({'schema': series.CANONICAL_IMPORT_CONTRACT,
                'canonical_source_contract': {}}, contract)

    def test_canonical_plan_requires_cold_validator_identity(self):
        plan, _ = self.plan()
        del plan['canonical_contract_module_sha256']
        with self.assertRaisesRegex(ValueError, 'module missing/changed'):
            series.validate_plan(plan)
        plan['canonical_contract_module_sha256'] = '0'*64
        with self.assertRaisesRegex(ValueError, 'module missing/changed'):
            series.validate_plan(plan)


if __name__ == '__main__':
    unittest.main()
