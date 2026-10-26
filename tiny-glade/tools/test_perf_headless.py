"""Synthetic scheduling, attribution, diagnostic and capture-safety regressions."""
import argparse
import copy
import json
from pathlib import Path
import tempfile
import unittest
from unittest.mock import patch
import perf_headless as runner
from perf_headless_account import account, diagnostic_summary, paired_overhead, validate, input_identity, signature, ANIMATED_INPUT_POLICY, SCRIPTED_INPUT_POLICY, PHASES


def report_fixture(mode='on', bill=False):
    rows, points, samples = [], [], []
    for index in range(3):
        row = {'world_tick': 10+index, 'receipt': 20+index, 'attempt': 30+index,
               'qpc': list(range(110+100*index, 191+100*index, 10)),
               'management': [100+index*2]*12+[7-index] if bill else None,
               'declaration': {'collect_tasks': 4, 'collect_ns': [10, 20, 30],
                               'stage_ns': [1, 2, 3, 4, 5, 6]} if bill else None}
        rows.append(row)
        points.append({**{k: row[k] for k in ('world_tick', 'receipt', 'attempt')},
                       'qpc': 200+100*index, 'app_epoch': 0, 'observed': True,
                       'cumulative_cpu_ns': [100000000, 190000000, 290000000][index]})
        if index:
            samples.append({'frame': row['world_tick'], 'receipt': row['receipt'], 'attempt': row['attempt'],
                            'kind': 'clean', 'cpu_ns': [0, 90000000, 100000000][index]})
    raw = {'schema': 'render-cpu-clean-v2', 'valid': True, 'complete': True,
           'process_id': 7, 'thread_id': 11, 'qpc_frequency': 1000,
           'truncated': False, 'dropped': 0, 'requested_samples': 2,
           'recorded': 2, 'clean_samples': 2, 'completion_points': points, 'samples': samples}
    return {'schema': 'headless-cpu-closure-v1', 'valid': True, 'process_id': 7, 'thread_id': 11,
            'qpc_frequency': 1000, 'mode': mode, 'warmup': 600, 'samples': 2,
            'allocator': 'mimalloc', 'diagnostic_bill': bill,
            'input_before': {'scene': 'static', 'draws': [5, 12]},
            'input_after': {'scene': 'static', 'draws': [5, 12]},
            'envelope': {'begin': [100, 1000, 2000], 'end': [500, 390001000, 790002000], 'frames': 3},
            'phase_names': list(PHASES),
            'management_names': ['counter'+str(index) for index in range(12)]+['cached_reflection_variants'],
            'frames': rows if mode == 'on' else [], 'session': raw if mode == 'on' else None}


def trace_fixture():
    return '''OS Version: test, Events Lost: 0, Buffers lost: 0,
CSwitch,TimeStamp,New Process Name ( PID),New TID,Old Process Name ( PID),Old TID,CPU
DPC,TimeStamp,CPU,ElapsedTime
# 0 # 0x0 # 0
CSwitch,0,game (7),11,Idle (0),0,0
# 1 # 0x5a # 0
CSwitch,1,game (7),12,Idle (0),0,1
# 2 # 0xe6 # 0
DPC,2,0,10000
# 3 # 0x1fe # 0
CSwitch,3,Idle (0),0,game (7),12,1
# 4 # 0x258 # 0
CSwitch,4,Idle (0),0,game (7),11,0
'''


class AccountingTests(unittest.TestCase):
    def setUp(self):
        self.temporary = tempfile.TemporaryDirectory()
        self.addCleanup(self.temporary.cleanup)
        self.trace = Path(self.temporary.name)/'trace.csv'
        self.trace.write_text(trace_fixture(), encoding='utf-8')

    def test_phase_cpu_intersection_and_baseline_exclusion(self):
        result = account(report_fixture(), self.trace)
        self.assertTrue(result['valid'], result['issues'])
        self.assertEqual(result['samples'], 2)
        self.assertEqual([f['world_tick'] for f in result['frames']], [11, 12])
        self.assertEqual(result['envelope']['owner_cpu_ms'], 390)
        self.assertEqual(result['envelope']['process_cpu_ms'], 790)
        self.assertEqual(result['envelope']['non_owner_cpu_ms'], 400)
        self.assertEqual(result['frames'][0]['pacing_not_running_ms'], 10)
        self.assertEqual(result['frames'][0]['phases']['pacing']['cpu_ms'], 0)
        self.assertEqual(result['frames'][0]['render_phase_owner_cpu_ms'], 60)
        self.assertEqual(result['frames'][0]['host_completion_owner_cpu_ms'], 90)
        self.assertEqual(result['frames'][1]['bookkeeping_owner_cpu_ms'], 20)
        self.assertFalse(result['production_16ms_acceptance'])

    def test_off_mode_only_has_envelope(self):
        result = account(report_fixture('off'), self.trace)
        self.assertTrue(result['valid'])
        self.assertNotIn('statistics', result)
        self.assertNotIn('frames', result)

    def test_os_crosscheck_is_mandatory_even_if_official_matches(self):
        report = report_fixture()
        report['envelope']['end'][1] = 350001000
        result = account(report, self.trace, official_owner_ns=390000000)
        self.assertFalse(result['valid'])
        self.assertEqual(result['envelope']['official_owner_relative_error'], 0)

    def test_process_crosscheck_detects_missing_other_thread_work(self):
        report = report_fixture()
        report['envelope']['end'][2] += 100000000
        self.assertFalse(account(report, self.trace)['valid'])

    def test_semantic_input_clock_and_attribution_failures(self):
        edits = [lambda r: r['input_after'].update(scene='edited'),
                 lambda r: r['frames'][1].update(receipt=20),
                 lambda r: r['frames'][1]['qpc'].__setitem__(4, 100),
                 lambda r: r['session']['completion_points'][1].update(app_epoch=2),
                 lambda r: r['session'].update(truncated=True),
                 lambda r: r['frames'].pop(),
                 lambda r: r['session']['samples'][0].update(kind='clock_error')]
        for edit in edits:
            report = report_fixture()
            edit(report)
            with self.subTest(edit=edit), self.assertRaises(ValueError):
                account(report, self.trace)

    def test_trace_loss_and_missing_boundary_are_rejected(self):
        self.trace.write_text(trace_fixture().replace('Events Lost: 0', 'Events Lost: 1'), encoding='utf-8')
        with self.assertRaisesRegex(ValueError, 'lost events'):
            account(report_fixture(), self.trace)
        self.trace.write_text(trace_fixture(), encoding='utf-8')
        report = report_fixture()
        report['envelope']['end'][0] = 700
        with self.assertRaisesRegex(ValueError, 'enclose'):
            account(report, self.trace)

    def test_unmatched_scheduling_inside_envelope_is_rejected(self):
        bad = trace_fixture().replace('# 3 # 0x1fe # 0', '# 3 # 0x190 # 0').replace('game (7),12,1', 'game (7),13,1')
        self.trace.write_text(bad, encoding='utf-8')
        with self.assertRaisesRegex(ValueError, 'unmatched'):
            account(report_fixture(), self.trace)

    def test_diagnostic_cumulative_vs_reset_and_gauge(self):
        result = diagnostic_summary(report_fixture(bill=True))
        self.assertEqual(result['management_totals']['counter0'], 4)
        self.assertEqual(result['management_mean_per_frame']['counter0'], 2)
        self.assertNotIn('cached_reflection_variants', result['management_totals'])
        self.assertEqual(result['cached_reflection_variants'], {'baseline': 7, 'last': 5, 'maximum': 7})
        # Each take() already resets: 4+4, not 4-4.
        self.assertEqual(result['declaration_delta_totals']['collect_tasks'], 8)
        self.assertEqual(result['declaration_delta_totals']['collect_ns'], [20, 40, 60])
        # stage_ns[5] stays an integer count, never converted to milliseconds.
        self.assertEqual(result['declaration_delta_totals']['stage_ns'][5], 12)

    def test_diagnostic_requires_explicit_counters(self):
        with self.assertRaises(ValueError):
            diagnostic_summary(report_fixture())
        report = report_fixture(bill=True)
        report['frames'][1]['management'][0] = 0
        with self.assertRaisesRegex(ValueError, 'regressed'):
            diagnostic_summary(report)


def overhead_runs(ratios=(.001, .002, .0015)):
    runs = []
    for index, ratio in enumerate(ratios):
        for mode in (('on', 'off') if index % 2 == 0 else ('off', 'on')):
            factor = 1+ratio if mode == 'on' else 1
            runs.append({'valid': True, 'mode': mode, 'diagnostic': False,
                         'executable_sha256': 'exe', 'hardware_sha256': 'hw', 'input_sha256': 'input',
                         'measurement_tools_sha256': 'tools', 'settings_sha256': 'settings', 'scenario': 'static', 'workload_sha256': 'workload',
                         'warmup': 600, 'samples': 3000,
                         'account': {'accounting_tools': {'perf_etw.py': 'etw', 'perf_headless_account.py': 'account'},
                                     'valid': True, 'diagnostic_bill': False, 'envelope': {
                             'owner_mean_cpu_ms_per_executed_frame': 100*factor,
                             'process_mean_cpu_ms_per_executed_frame': 200*factor}}})
    return runs


class OverheadTests(unittest.TestCase):
    def test_small_stable_paired_difference_passes(self):
        result = paired_overhead(overhead_runs())
        self.assertTrue(result['valid'], result['issues'])
        self.assertEqual(result['pairs'], 3)

    def test_noise_and_real_regression_are_not_accepted(self):
        for ratios in ((.03, .03, .03), (-.04, .04, .001), (-.02, -.02, -.02)):
            with self.subTest(ratios=ratios):
                self.assertFalse(paired_overhead(overhead_runs(ratios))['valid'])

    def test_workload_binary_hardware_and_order_must_match(self):
        for field in ('executable_sha256', 'hardware_sha256', 'input_sha256', 'settings_sha256', 'samples', 'scenario', 'workload_sha256'):
            runs = overhead_runs()
            runs[2][field] = 'different'
            with self.subTest(field=field), self.assertRaisesRegex(ValueError, 'mismatch'):
                paired_overhead(runs)
        runs = overhead_runs(); runs[2]['mode'] = 'on'
        with self.assertRaisesRegex(ValueError, 'alternate'):
            paired_overhead(runs)
        with self.assertRaisesRegex(ValueError, 'three'):
            paired_overhead(overhead_runs()[:4])

    def test_invalid_or_heavy_run_cannot_pass(self):
        runs = overhead_runs(); runs[1]['account']['valid'] = False
        self.assertFalse(paired_overhead(runs)['valid'])
        runs = overhead_runs(); runs[1]['diagnostic'] = True
        with self.assertRaisesRegex(ValueError, 'diagnostic'):
            paired_overhead(runs)


class CaptureSafetyTests(unittest.TestCase):
    def test_existing_kernel_logger_is_never_replaced(self):
        with tempfile.TemporaryDirectory() as directory:
            with patch.object(runner.subprocess, 'check_output', return_value='Logger Name : NT Kernel Logger\n'), \
                    patch.object(runner.subprocess, 'run') as run:
                with self.assertRaisesRegex(RuntimeError, 'another NT Kernel Logger'):
                    runner.start_trace(Path(directory))
                run.assert_not_called()

    def test_trace_flags_include_interrupt_and_qpc(self):
        with tempfile.TemporaryDirectory() as directory:
            with patch.object(runner.subprocess, 'check_output', return_value='Logger Name : Other User Session\n'), \
                    patch.object(runner.subprocess, 'run') as run:
                runner.start_trace(Path(directory))
                command = run.call_args.args[0]
                self.assertIn('PROC_THREAD+CSWITCH+DISPATCHER+DPC+INTERRUPT', command)
                self.assertIn('PerfCounter', command)

    def test_environment_clears_inherited_probe_workload_and_profiler_settings(self):
        with patch.dict(runner.os.environ, {'TG_CPU_BILL': '1', 'TG_FRAME_TIMING': '1',
                                            'TG_PERF_SCENARIO': 'other', 'RHAPSODY_SUBMIT_TRACE': '1'}):
            env = runner.environment()
            self.assertNotIn('TG_CPU_BILL', env)
            self.assertNotIn('TG_FRAME_TIMING', env)
            self.assertNotIn('TG_PERF_SCENARIO', env)
            self.assertNotIn('RHAPSODY_SUBMIT_TRACE', env)
            self.assertEqual(env['CARGO_INCREMENTAL'], '0')
            self.assertEqual(env['TG_TEST_SIZE'], '1280x720')
            self.assertEqual(runner.environment(True)['TG_CPU_BILL'], '1')



class HandshakeCleanupTests(unittest.TestCase):
    def simulate(self, directory, fail_start=False, timeout=False):
        destination = Path(directory)
        args = argparse.Namespace(label='build', run_label='run', mode='off', warmup=10,
                                  samples=30, diagnostic=False, without_etw=False,
                                  official=False, timeout=60)
        children = []
        class Child:
            pid = 123
            code = None
            def __init__(self, command, **kwargs):
                self.output = Path(kwargs['env']['TG_CPU_PROBE_OUT'])
                self.output.with_suffix('.ready').write_text('ready', encoding='utf-8')
                self.terminated = False
                children.append(self)
            def poll(self):
                return self.code
            def wait(self, timeout=None):
                if self.code is not None:
                    return self.code
                if forced_timeout:
                    raise runner.subprocess.TimeoutExpired('fixture', timeout)
                self.code = 0
                return 0
            def terminate(self):
                self.terminated = True
                self.code = -15
            def kill(self):
                self.code = -9
        forced_timeout = timeout
        def start(path):
            self.assertTrue((path/'session.ready').exists())
            self.assertFalse((path/'session.go').exists(), 'go must follow successful ETW start')
            if fail_start:
                raise RuntimeError('existing kernel logger')
        frozen = {'executable': 'frozen.exe', 'executable_sha256': 'exe', 'features': ['cpu-probe'],
                  'dirty': False, 'profile': 'release'}
        machine = {'sha256': 'hw', 'vulkan_environment': {}}
        with patch.object(runner, 'PERF', destination), patch.object(runner, 'load_build', return_value=frozen), \
                patch.object(runner, 'hardware', return_value=machine), \
                patch.object(runner.subprocess, 'Popen', side_effect=Child), \
                patch.object(runner, 'start_trace', side_effect=start), \
                patch.object(runner.subprocess, 'run', return_value=runner.subprocess.CompletedProcess([], 0)) as command, \
                patch.object(runner, 'finish_capture', return_value={'valid': True}):
            if fail_start:
                with self.assertRaisesRegex(RuntimeError, 'existing'):
                    runner.capture(args)
            elif timeout:
                with self.assertRaises(runner.subprocess.TimeoutExpired):
                    runner.capture(args)
            else:
                self.assertEqual(runner.capture(args), {'valid': True})
        return children, command, json.loads((destination/'headless-runs'/'run'/'run.json').read_text(encoding='utf-8'))

    def test_start_failure_terminates_only_child_and_never_stops_other_logger(self):
        with tempfile.TemporaryDirectory() as directory:
            children, command, record = self.simulate(directory, fail_start=True)
            self.assertTrue(children[0].terminated)
            command.assert_not_called()
            self.assertFalse(record['trace_started_by_this_run'])
            self.assertIn('existing kernel logger', record['error'])

    def test_timeout_after_start_always_stops_owned_logger(self):
        with tempfile.TemporaryDirectory() as directory:
            children, command, record = self.simulate(directory, timeout=True)
            self.assertTrue(children[0].terminated)
            self.assertTrue(record['trace_started_by_this_run'])
            self.assertEqual(command.call_args.args[0][-2:], ['-stop', 'NT Kernel Logger'])
            self.assertEqual(command.call_count, 1)

    def test_success_starts_after_ready_stops_after_exit(self):
        with tempfile.TemporaryDirectory() as directory:
            children, command, record = self.simulate(directory)
            self.assertFalse(children[0].terminated)
            self.assertTrue(record['trace_started_by_this_run'])
            self.assertEqual(command.call_count, 1)
            self.assertEqual(record['exit_code'], 0)



class InputPolicyTests(unittest.TestCase):
    def test_animated_policy_keeps_both_raw_endpoints_and_marks_non_static(self):
        report = report_fixture()
        report['input_policy'] = ANIMATED_INPUT_POLICY
        report['input_unchanged'] = False
        report['input_after']['sheep_transform_bits'] = [1, 2, 3]
        validate(report)
        identity = input_identity(report)
        self.assertEqual(identity['before'], report['input_before'])
        self.assertEqual(identity['after'], report['input_after'])
        first = signature(identity)
        report['input_after']['sheep_transform_bits'][2] = 4
        self.assertNotEqual(first, signature(input_identity(report)))
        report['input_policy'] = 'strict_static_input_unchanged'
        with self.assertRaisesRegex(ValueError, 'strict-static'):
            validate(report)

    def test_policy_flag_and_unknown_policy_cannot_bypass_raw_data(self):
        report = report_fixture()
        report['input_policy'] = ANIMATED_INPUT_POLICY
        report['input_unchanged'] = False
        with self.assertRaisesRegex(ValueError, 'contradicts'):
            validate(report)
        report['input_policy'] = 'ignore_transforms'
        with self.assertRaisesRegex(ValueError, 'unknown'):
            validate(report)

    def test_same_before_but_different_after_rejects_pair(self):
        runs = overhead_runs()
        report = report_fixture()
        report['input_policy'] = ANIMATED_INPUT_POLICY
        report['input_unchanged'] = False
        report['input_after']['sheep'] = [1, 2, 3]
        for run in runs:
            run['input_sha256'] = signature(input_identity(report))
        report['input_after']['sheep'][0] = 8
        runs[3]['input_sha256'] = signature(input_identity(report))
        with self.assertRaisesRegex(ValueError, 'input_sha256'):
            paired_overhead(runs)


def compiled_runs(session_on=True):
    runs = overhead_runs()
    for run in runs:
        role = run['mode']
        run['cpu_probe_feature'] = role == 'on'
        run['mode'] = 'on' if role == 'on' and session_on else 'off'
        run['executable_sha256'] = 'exe-'+role
        run['comparison_settings_sha256'] = 'same-nonfeature-settings'
        run['settings_sha256'] = 'settings-'+role
        run['build'] = {'source_commit': 'source', 'source_content_sha256': 'content',
                        'cargo_lock_sha256': 'lock', 'profile': 'release', 'allocator': 'mimalloc',
                        'incremental': False, 'jobs': 1, 'rustc': 'rustc-version', 'build_environment_sha256': 'env',
                        'features': ['default(mimalloc)']+(['cpu-probe'] if role == 'on' else [])}
    return runs


class CompiledFeatureTests(unittest.TestCase):
    def test_feature_total_and_feature_disabled_paths_have_distinct_scopes(self):
        active = paired_overhead(compiled_runs(), 'compiled_feature_session_on')
        dormant = paired_overhead(compiled_runs(False), 'compiled_feature_session_off')
        self.assertTrue(active['valid'])
        self.assertTrue(dormant['valid'])
        self.assertNotEqual(active['overhead_kind'], dormant['overhead_kind'])
        self.assertFalse(active['uncovered_overhead'])
        self.assertIn('active session', dormant['uncovered_overhead'][0])
        self.assertNotEqual(active['overhead_kind'], paired_overhead(overhead_runs())['overhead_kind'])

    def test_compiled_pair_requires_matching_source_config_and_nonfeature_inputs(self):
        for field in ('source_commit', 'source_content_sha256', 'cargo_lock_sha256', 'profile', 'allocator', 'rustc', 'build_environment_sha256'):
            runs = compiled_runs(); runs[1]['build'][field] = 'different'
            with self.subTest(field=field), self.assertRaisesRegex(ValueError, 'build mismatch'):
                paired_overhead(runs, 'compiled_feature_session_on')
        runs = compiled_runs(); runs[1]['build']['features'].append('pprof')
        with self.assertRaisesRegex(ValueError, 'beyond cpu-probe'):
            paired_overhead(runs, 'compiled_feature_session_on')
        runs = compiled_runs(); runs[1]['comparison_settings_sha256'] = 'other'
        with self.assertRaisesRegex(ValueError, 'comparison_settings'):
            paired_overhead(runs, 'compiled_feature_session_on')

    def test_same_binary_runtime_pair_cannot_accept_feature_off_or_two_binaries(self):
        with self.assertRaises(ValueError):
            paired_overhead(compiled_runs())
        runs = compiled_runs(False)
        with self.assertRaisesRegex(ValueError, 'runtime mode'):
            paired_overhead(runs, 'compiled_feature_session_on')

    def test_feature_off_cannot_enable_session_or_bill(self):
        report = report_fixture()
        report['cpu_probe_feature'] = False
        with self.assertRaisesRegex(ValueError, 'cannot enable probe'):
            validate(report)
        report = report_fixture('off', bill=True)
        report['cpu_probe_feature'] = False
        with self.assertRaisesRegex(ValueError, 'cannot enable bill'):
            validate(report)
        report['diagnostic_bill'] = False
        validate(report)

    def test_runner_rejects_on_or_bill_before_launching_feature_off_exe(self):
        args = argparse.Namespace(label='off', mode='on', diagnostic=False)
        with patch.object(runner, 'load_build', return_value={'features': ['default(mimalloc)']}), \
                patch.object(runner.subprocess, 'Popen') as process:
            with self.assertRaisesRegex(ValueError, 'supports only'):
                runner.capture(args)
            process.assert_not_called()



class ScenarioTests(unittest.TestCase):
    def test_scripted_scene_preserves_schedule_draw_fingerprint_and_transform_trajectory(self):
        report = report_fixture()
        report.update(input_policy=SCRIPTED_INPUT_POLICY, input_unchanged=False,
                      scenario='continuous_edit', workload={'schedule_blake3': 'a', 'draw_lengths_blake3': 'b',
                                                           'producer_dirty_bytes': 4096, 'edits': 2})
        report['input_before']['workload'] = {'edits': 0}
        report['input_after']['workload'] = report['workload']
        report['input_after']['clock_camera'] = {'game_delta_bits': 123, 'camera_bits': [1, 2, 3]}
        validate(report)
        old = signature(input_identity(report))
        report['input_after']['clock_camera']['camera_bits'][0] = 4
        self.assertNotEqual(old, signature(input_identity(report)))
        report['scenario'] = 'unknown'
        with self.assertRaisesRegex(ValueError, 'scenario'):
            validate(report)

    def test_non_static_insufficient_warmup_rejected_before_process_launch(self):
        args = argparse.Namespace(label='build', mode='on', scene='local_edit', warmup=199, diagnostic=False)
        with patch.object(runner, 'load_build', return_value={'features': ['default(mimalloc)', 'cpu-probe']}), \
                patch.object(runner.subprocess, 'Popen') as process:
            with self.assertRaisesRegex(ValueError, 'warmup'):
                runner.capture(args)
            process.assert_not_called()

    def test_raw_workload_mismatch_rejects_even_when_input_hash_is_equal(self):
        runs = overhead_runs()
        runs[1]['workload_sha256'] = 'different-schedule'
        with self.assertRaisesRegex(ValueError, 'workload_sha256'):
            paired_overhead(runs)



class ReviewRegressionTests(unittest.TestCase):
    def test_ratio_ci_does_not_override_actual_ratio_of_cpu_means(self):
        runs = overhead_runs((0, 0, 0, 0, .012))
        for run in runs[-2:]:
            for field in ('owner_mean_cpu_ms_per_executed_frame', 'process_mean_cpu_ms_per_executed_frame'):
                run['account']['envelope'][field] *= 10000
        result = paired_overhead(runs)
        self.assertLess(result['metrics']['owner']['confidence_95_relative_change'][1], .01)
        self.assertGreater(result['metrics']['owner']['ratio_of_means_relative_change'], .01)
        self.assertFalse(result['valid'])

    def test_modified_capture_or_accounting_tool_cannot_mix_into_series(self):
        runs = overhead_runs(); runs[1]['measurement_tools_sha256'] = 'different'
        with self.assertRaisesRegex(ValueError, 'measurement_tools'):
            paired_overhead(runs)
        runs = overhead_runs(); runs[1]['account']['accounting_tools']['perf_etw.py'] = 'different'
        with self.assertRaisesRegex(ValueError, 'accounting tools'):
            paired_overhead(runs)


if __name__ == '__main__':
    unittest.main()
