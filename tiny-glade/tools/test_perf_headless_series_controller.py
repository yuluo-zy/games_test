"""Controller tests with synthetic data; never replay real ETW or run a target."""
import copy
from datetime import datetime, timezone
import json
from pathlib import Path
import tempfile
import unittest
from unittest.mock import patch
import perf_headless_series as series
import perf_headless_workload as work
from test_perf_headless_workload import actual_fixture

def save(path, value):
    Path(path).parent.mkdir(parents=True, exist_ok=True)
    Path(path).write_text(json.dumps(value), encoding='utf-8')

class ControllerTests(unittest.TestCase):
    def setUp(self):
        self.temp = tempfile.TemporaryDirectory()
        self.addCleanup(self.temp.cleanup)
        self.root = Path(self.temp.name)
        self.tools = self.root/'tools'; self.tools.mkdir()
        for name in series.TOOLS:
            (self.tools/name).write_text('# synthetic dependency', encoding='utf-8')
        self.a = self.make_build('baseline', 'a'*40)
        self.b = self.make_build('candidate', 'b'*40)
        self.calibration = self.root/'calibration.json'
        save(self.calibration, {'schema': 'windows-clock-policy-evidence-v1',
            'policy_id': series.POLICY, 'definition_sha256': 'f'*64,
            'tools': series.module_identity(self.tools),
            'hardware': {'os': 'synthetic-os', 'physical_cpu': 'synthetic', 'logical_cpus': 1}})

    def make_build(self, label, commit):
        folder = self.root/'.perf'/'headless-builds'/label; folder.mkdir(parents=True)
        for name in ('fixture.exe', 'fixture.pdb', 'fmod.dll'):
            (folder/name).write_bytes((label if name != 'fmod.dll' else 'shared').encode())
        source_archive = folder/'archived-producer.rs'
        source_archive.write_text('fn recorded() {}', encoding='utf-8')
        files = {'crates/fixture.rs': series.digest(source_archive)}
        archives = {'crates/fixture.rs': series.reference(source_archive)}
        for index, rel in enumerate(work.COMMON_HARNESS_FILES):
            archive = folder/('common'+str(index)+'.rs')
            archive.write_text(work.ALGORITHM_SELECTION, encoding='utf-8')
            files[rel] = series.digest(archive); archives[rel] = series.reference(archive)
        source = {'commit': commit, 'status': '', 'files': files, 'content_sha256': series.signature(files)}
        # Source differs by commit even when this synthetic manifest is identical.
        save(folder/'source.json', source)
        build = {'schema': 'headless-frozen-build-v1', 'label': label,
            'executable': str(folder/'fixture.exe'), 'executable_sha256': series.digest(folder/'fixture.exe'),
            'files': {name: series.digest(folder/name) for name in ('fixture.exe', 'fixture.pdb', 'fmod.dll')},
            'source_commit': commit, 'source_content_sha256': source['content_sha256'],
            'dirty': False, 'profile': 'release', 'features': ['default(mimalloc)', 'cpu-probe'],
            'cpu_probe_feature': True, 'allocator': 'mimalloc', 'incremental': False, 'jobs': 1,
            'command': ['cargo', 'build', '--release', '-j1', '--locked', '--offline'],
            'build_environment': {'CARGO_INCREMENTAL': '0'},
            'build_environment_sha256': series.signature({'CARGO_INCREMENTAL': '0'}),
            'cargo_lock_sha256': '2'*64, 'rustc': 'synthetic-rustc', 'build_helper_sha256': '3'*64,
            'requested_test_target_completed': True,
            'cargo_lifecycle': {'active_build_processes_remaining': False},
            'management_controls': {'default_modes': None, 'probe_runtime_controls': [],
                                    'source_content_sha256': source['content_sha256']}}
        save(folder/'build.json', build)
        _, _, model = actual_fixture()
        fields = {key: {'meaning': meaning, 'symbols': ['recorded'], 'source_paths': ['crates/fixture.rs']}
                  for key, meaning in work.FIELD_GROUPS.items()}
        report = {'schema': 'headless-producer-source-review-v1', 'source_commit': commit,
            'source_content_sha256': source['content_sha256'], 'source_sha256': files,
            'fields': fields, 'coverage': model['coverage'], 'findings': [],
            'algorithm_selection': work.ALGORITHM_SELECTION, 'runtime_ablation_supported': False,
            'controller_disable_controls_absent_checked': True,
            'receipt_attribution_checked': True, 'successful_prefix_failure_invalid_checked': True,
            'counter_overflow_checked': True}
        save(folder/'producer-source-review.json', report)
        cap = {'schema': 'headless-producer-capability-v1', 'raw_schema': work.SCHEMA,
            'source_commit': commit, 'source_content_sha256': source['content_sha256'],
            'source_files': archives, 'algorithm_selection': work.ALGORITHM_SELECTION,
            'review_report': series.reference(folder/'producer-source-review.json'),
            'fields': fields, 'coverage': model['coverage']}
        save(folder/'producer-review.json', cap)
        return folder/'build.json'

    def plan(self):
        path = self.root/'series.json'
        value = series.make_plan('synthetic', self.a, self.b, self.calibration,
                                 self.tools, path, baseline_producer_review=self.a.with_name('producer-review.json'),
                                 candidate_producer_review=self.b.with_name('producer-review.json'))
        return value, path

    def legacy_capture(self, plan, entry):
        folder = Path(entry['run_dir']); folder.mkdir(parents=True)
        build = plan['build_records'][entry['side']]
        modes = dict.fromkeys(series.MODES, True)
        session = {'schema': 'headless-cpu-closure-v1', 'diagnostic_bill': False,
            'mode': 'on', 'cpu_probe_feature': True, 'scenario': entry['scene'],
            'warmup': 600, 'samples': 3000, 'valid': True, 'clock_valid': True,
            'management_modes': modes, 'input_policy': series.INPUT_POLICY,
            'input_before': {'clock_camera': 'synthetic'},
            'input_after': {'transform_trajectory': 'synthetic-trajectory'},
            'workload': {'schema': 'real-ecs-raster-workload-v1', 'scenario': entry['scene'],
                         'policy': {'extent': [1280, 720], 'world_scale': 1.0, 'fixed_delta': '1/60'}},
            'frames': [{}]*3001, 'envelope': {'begin': [1, 0, 0], 'end': [4, 1, 2]}}
        identity = {'policy': series.INPUT_POLICY, 'before': session['input_before'], 'after': session['input_after']}
        frames = []
        witness = []
        for index in range(3000):
            frame = {'world_tick': index, 'receipt': index+1, 'attempt': index+1, 'app_epoch': 0}
            frame.update(dict.fromkeys(series.METRICS, 1.0))
            frames.append(frame)
            witness.append({'world_tick': index, 'receipt': index+1, 'attempt': index+1, 'app_epoch': 0,
                            'actual_draw_list_entries': 1001, 'submitted_collect_tasks': 20,
                            'all_gpu_upload_bytes': 4000})
        coverage = ['actual_draw_list_entries', 'submitted_collect_tasks', 'all_gpu_upload_bytes']
        session['workload_counters'] = {'schema': 'headless-workload-counters-v1',
                                        'coverage': coverage, 'frames': witness}
        (folder/'vulkan-summary.txt').write_bytes(b'synthetic-vulkan')
        hardware = {'vulkan_summary_sha256': series.digest(folder/'vulkan-summary.txt'),
                    'vulkan_environment': {}}
        hardware['sha256'] = series.signature(hardware)
        settings = {'clock_policy': plan['clock_policy'], 'scenario': entry['scene'],
            'dimensions': '1280x720', 'warmup': 600, 'samples': 3000,
            'features': build['features'], 'allocator': 'mimalloc', 'driver_environment': {},
            'worker_configuration': 'existing default; TG overrides cleared', 'diagnostic': False,
            'process_priority': 'NORMAL_PRIORITY_CLASS', 'thread_priority': 'existing fixture default',
            'management_ablation': {}, 'management_modes_expected': modes}
        cpu = {'schema': 'headless-etw-account-v2', 'diagnostic_bill': False, 'mode': 'on',
            'scenario': entry['scene'], 'samples': 3000, 'valid': True, 'issues': [],
            'clock_policy': plan['clock_policy'], 'management_modes': modes,
            'production_16ms_acceptance': False, 'input_policy': series.INPUT_POLICY,
            'input_sha256': series.signature(identity), 'workload_sha256': series.signature(session['workload']),
            'frames': frames, 'statistics': {m: {'n': 3000, 'p50_ms': 1.0, 'p95_ms': 1.0, 'p99_ms': 1.0} for m in series.METRICS},
            'clock_quality': {'coarse_checks': {'owner_getthreadtimes_within_2pct': True},
                             'lifecycle_complete': True, 'raw_counters_corrected': False,
                             'shared_event_stream': True, 'independent_clock': False},
            'envelope': {'owner_os_relative_error': .001, 'official_process_relative_error': .001},
            'accounting_tools': {name: plan['measurement_tools'][name] for name in
                ('perf_headless_account.py', 'perf_etw.py', 'perf_trace_archive.py', 'perf_clock_policy.py')}}
        save(folder/'session.json', session)
        (folder/'schedule.etl').write_bytes(b'synthetic-etl-no-measurement')
        (folder/'schedule.csv').write_bytes(b'synthetic-csv-no-measurement')
        (folder/'official-owner.csv').write_bytes(b'synthetic-official')
        intent = {'created_filetime': round(datetime.now(timezone.utc).timestamp()*1e7)+116444736000000001}
        save(folder/'clock-policy-intent.json', intent)
        run = {'schema': 'headless-capture-v1', 'valid': True, 'clean_release_candidate': True,
            'build': build, 'diagnostic': False, 'mode': 'on', 'cpu_probe_feature': True,
            'etw': True, 'trace_started_by_this_run': True, 'exit_code': 0, 'etw_stop_exit_code': 0,
            'session_path': str(folder/'session.json'), 'session_sha256': series.digest(folder/'session.json'),
            'clock_policy_intent': series.reference(folder/'clock-policy-intent.json'),
            'scenario': entry['scene'], 'warmup': 600, 'samples': 3000, 'clock_policy': plan['clock_policy'],
            'measurement_tools': plan['measurement_tools'], 'measurement_tools_sha256': series.signature(plan['measurement_tools']),
            'hardware': hardware, 'hardware_sha256': hardware['sha256'], 'settings': settings,
            'settings_sha256': series.signature(settings), 'management_ablation': {},
            'attribution_scope': 'default_algorithms', 'management_modes': modes,
            'management_modes_sha256': series.signature(modes), 'production_16ms_acceptance': False,
            'input_policy': series.INPUT_POLICY, 'input_sha256': series.signature(identity),
            'workload': session['workload'], 'workload_sha256': series.signature(session['workload']),
            'trace_csv_path': str(folder/'schedule.csv'), 'trace_etl_sha256': series.digest(folder/'schedule.etl'),
            'official_cswitch': {'output': series.reference(folder/'official-owner.csv')},
            'account': cpu, 'executable_sha256': build['executable_sha256'],
            'command': [build['executable'], 'cpu_measurement::profile_shipped_session_cpu']}
        save(folder/'cpu.json', cpu); save(folder/'run.json', run)
        proof = {'schema': 'headless-exact-workload-v1', 'run_sha256': series.digest(folder/'run.json'),
            'session_sha256': run['session_sha256'], 'executable_sha256': run['executable_sha256'],
            'coverage': coverage, 'raw_session_field': 'workload_counters', 'raw': session['workload_counters'],
            'producer': {'kind': 'frozen-clean-fixture-counters',
                'source_content_sha256': build['source_content_sha256'],
                'fields': {name: {'source_path': 'crates/fixture.rs', 'source_sha256': '1'*64,
                    'symbol': 'synthetic_fixture', 'semantics': 'synthetic test only',
                    'unit': 'bytes' if name.endswith('bytes') else 'count'} for name in coverage}}}
        save(folder/'workload-proof.json', proof)
        return run, session, cpu


    def capture(self, plan, entry):
        run, session, cpu = self.legacy_capture(plan, entry)
        actual_session, actual_cpu, _ = actual_fixture(samples=3000)
        session['actual_workload'] = actual_session['actual_workload']
        session['frames'] = actual_session['frames']
        session.update(measurement_class='clean_cpu', clean_cpu_acceptance_eligible=True, gpu_diagnostic=None,
            harness='common-cpu-workload-v1', management_diagnostics_available=False,
            algorithm_selection=work.ALGORITHM_SELECTION, management_modes=None)
        run['management_modes'] = cpu['management_modes'] = None
        run['management_modes_expected'] = None
        run['management_modes_sha256'] = series.signature(None)
        run['settings']['management_modes_expected'] = None
        run['settings_sha256'] = series.signature(run['settings'])
        for old, new in zip(cpu['frames'], actual_cpu['frames']): old.update(new)
        folder = Path(entry['run_dir'])
        save(folder/'session.json', session)
        run['session_sha256'] = series.digest(folder/'session.json')
        run['account'] = cpu
        save(folder/'cpu.json', cpu); save(folder/'run.json', run)
        proof = {'schema': 'headless-actual-workload-proof-v1',
            'run_sha256': series.digest(folder/'run.json'), 'session_sha256': run['session_sha256'],
            'executable_sha256': run['build']['executable_sha256'],
            'producer_review': plan['producer_reviews'][entry['side']],
            'raw_session_field': 'actual_workload', 'raw': session['actual_workload']}
        save(folder/'workload-proof.json', proof)
        return run, session, cpu

    def test_new_plan_binds_adapter_and_reviews_without_launching(self):
        with patch('subprocess.Popen', side_effect=AssertionError('must not launch')):
            plan, _ = self.plan()
        self.assertEqual(plan['schema'], 'headless-cross-source-series-v2')
        self.assertEqual(len(plan['entries']), 40)
        self.assertEqual(plan['workload_adapter_sha256'], series.digest(work.__file__))
        series.validate_plan(plan)

    def test_legacy_plan_cannot_be_reclassified(self):
        plan, _ = self.plan(); plan['schema'] = 'headless-cross-source-series-v1'
        with self.assertRaisesRegex(ValueError, 'unknown'): series.validate_plan(plan)

    def test_real_schema_synthetic_admission_stays_pending(self):
        plan, _ = self.plan(); entry = plan['entries'][0]
        _, _, cpu = self.capture(plan, entry)
        row = series.load_run(plan, entry, lambda *args, **kwargs: cpu)
        self.assertTrue(row['proof']['coverage_pending'])

    def test_handwritten_sidecar_cannot_replace_raw_actual(self):
        plan, _ = self.plan(); entry = plan['entries'][0]
        run, session, cpu = self.capture(plan, entry)
        proof_path = Path(entry['run_dir'])/'workload-proof.json'; proof = series.read(proof_path)
        proof['raw']['valid'] = False; save(proof_path, proof)
        with self.assertRaisesRegex(ValueError, 'exact raw'):
            series.workload_proof(proof_path, Path(entry['run_dir'])/'run.json', Path(entry['run_dir'])/'session.json',
                                  session, cpu, review_reference=plan['producer_reviews']['A'])

    def test_raw_invalid_cannot_be_repaired_by_proof_flags(self):
        plan, _ = self.plan(); entry = plan['entries'][0]
        run, session, cpu = self.capture(plan, entry); folder = Path(entry['run_dir'])
        session['actual_workload']['valid'] = False; save(folder/'session.json', session)
        run['session_sha256'] = series.digest(folder/'session.json'); save(folder/'run.json', run)
        proof = series.read(folder/'workload-proof.json'); proof.update(raw=session['actual_workload'],
            session_sha256=run['session_sha256'], run_sha256=series.digest(folder/'run.json'))
        save(folder/'workload-proof.json', proof)
        with self.assertRaisesRegex(ValueError, 'invalid'):
            series.load_run(plan, entry, lambda *args, **kwargs: cpu)

    def test_review_changed_after_declaration_rejected(self):
        plan, _ = self.plan(); cap = self.a.with_name('producer-review.json'); value = series.read(cap)
        value['fields']['native_commands']['symbols'] = ['forged']; save(cap, value)
        with self.assertRaisesRegex(ValueError, 'SHA changed'): series.validate_plan(plan)

    def test_unchanged_native_work_allows_task_cost_flag_not_false_semantic_rejection(self):
        plan, _ = self.plan(); rows = {}
        for entry in plan['entries'][:2]:
            _, _, cpu = self.capture(plan, entry)
            rows[entry['side']] = series.load_run(plan, entry, lambda *args, **kwargs: cpu)
        rows['B']['proof']['cost_frames'][0]['collect']['tasks'] = 1
        rows['B']['proof']['cost_difference_explanation'] = 'synthetic reduced task management'
        result = series.pair_results(rows)
        self.assertTrue(result['workload_comparison']['cost_difference_review_required'])
        self.assertFalse(result['failures'])
        self.assertFalse(result['workload_comparison']['explanation_is_approval'])

    def test_existing_real_debug600_smoke_remains_ineligible(self):
        fixture = series.read(Path(__file__).with_name('fixtures')/'real-smoke-admission.json')
        self.assertEqual(fixture['run']['samples'], 600)
        self.assertEqual(fixture['run']['build']['profile'], 'debug')
        plan, _ = self.plan(); entry = plan['entries'][0]
        folder = Path(entry['run_dir']); folder.mkdir(parents=True)
        for name in ('run', 'session', 'cpu'): save(folder/(name+'.json'), fixture[name])
        with self.assertRaisesRegex(ValueError, 'non-release'):
            series.load_run(plan, entry, lambda *args, **kwargs: {})

    def test_legacy_none_without_algorithm_marker_rejected(self):
        plan, _ = self.plan(); entry = plan['entries'][0]
        run, session, cpu = self.capture(plan, entry); del session['algorithm_selection']
        save(Path(entry['run_dir'])/'session.json', session)
        with self.assertRaisesRegex(ValueError, 'algorithm selection'): series.load_run(plan, entry, lambda *a, **kw: cpu)

    def test_gpu_diagnostic_cannot_claim_clean_with_diagnostic_false(self):
        plan, _ = self.plan(); entry = plan['entries'][0]
        run, session, cpu = self.capture(plan, entry)
        session['measurement_class'] = 'gpu_diagnostic'; session['clean_cpu_acceptance_eligible'] = False
        save(Path(entry['run_dir'])/'session.json', session)
        with self.assertRaisesRegex(ValueError, 'diagnostic'): series.load_run(plan, entry, lambda *a, **kw: cpu)

    def test_disable_control_rejected_even_with_common_source_contract(self):
        plan, _ = self.plan(); plan['entries'][0]['command'].append('--disable-constant-batch')
        with self.assertRaisesRegex(ValueError, 'disable controls'): series.validate_plan(plan)

    def test_fake_runtime_four_boolean_modes_rejected(self):
        plan, _ = self.plan(); entry = plan['entries'][0]
        run, session, cpu = self.capture(plan, entry); session['management_modes'] = dict.fromkeys(series.MODES, True)
        path = Path(entry['run_dir']); save(path/'session.json', session)
        run['session_sha256'] = series.digest(path/'session.json'); save(path/'run.json', run)
        with self.assertRaisesRegex(ValueError, 'four-boolean'): series.load_run(plan, entry, lambda *a, **kw: cpu)

    def test_numeric_success_never_clears_partial_coverage(self):
        result = series.apply_workload_pending({'status': 'headless_cpu_gate_passed', 'headless_cpu_gate_passed': True},
            [{'coverage_pending': ['partial'], 'payload_review_required': False, 'cost_difference_review_required': False}])
        self.assertEqual(result['status'], 'pending_workload_review')
        self.assertTrue(result['render_numeric_gate_passed'])
        self.assertFalse(result['headless_cpu_gate_passed'])

    def test_hard_failure_never_washed_into_pending_or_passed(self):
        result = series.apply_workload_pending({'status': 'rejected', 'headless_cpu_gate_passed': False},
            [{'coverage_pending': ['partial'], 'payload_review_required': False, 'cost_difference_review_required': False}])
        self.assertEqual(result['status'], 'rejected')
        self.assertFalse(result['headless_cpu_gate_passed'])

if __name__ == '__main__': unittest.main()
