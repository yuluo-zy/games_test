import copy
import unittest
from perf_etw import Timeline
from perf_headless_account import validate_boundary_uncertainty, boundary_uncertainty_summary


class BoundaryClockTests(unittest.TestCase):
    def report(self):
        return {'thread_id': 11, 'qpc_frequency': 1000,
                'envelope': {'begin': [100, 1, 2], 'end': [900, 3, 4]},
                'frames': [{'qpc': [150]}],
                'boundary_uncertainty': {'schema': 'cpu-envelope-brackets-v1', 'clock': 'qpc',
                     'begin_owner': [110, 120], 'begin_process': [130, 140],
                     'end_owner': [910, 920], 'end_process': [930, 940]}}

    def test_legacy_report_without_brackets_is_not_reinterpreted(self):
        self.assertIsNone(validate_boundary_uncertainty({}))
        self.assertIsNone(boundary_uncertainty_summary({}, {}, 0, 1000))

    def test_accepts_four_ordered_brackets(self):
        report = self.report()
        self.assertEqual(validate_boundary_uncertainty(report), report['boundary_uncertainty'])

    def test_rejects_malformed_regressed_or_misordered_brackets(self):
        for name, span in [('begin_owner', [False, 120]), ('begin_owner', [120, 110]),
                           ('begin_owner', [99, 120]), ('begin_process', [119, 140]),
                           ('end_owner', [899, 920]), ('end_process', [919, 940]),
                           ('end_process', [930])]:
            report = self.report()
            report['boundary_uncertainty'][name] = span
            with self.subTest(name=name, span=span), self.assertRaises(ValueError):
                validate_boundary_uncertainty(report)

    def test_rejects_bracket_overlapping_first_frame(self):
        report = self.report()
        report['frames'][0]['qpc'][0] = 139
        with self.assertRaisesRegex(ValueError, 'first measured frame'):
            validate_boundary_uncertainty(report)

    def test_quantifies_read_window_without_altering_acceptance_or_legacy_interval(self):
        report = self.report()
        before = copy.deepcopy(report)
        owner = Timeline([(100, 200), (850, 950)])
        worker = Timeline([(100, 1000)])
        summary = boundary_uncertainty_summary(report, {11: owner, 12: worker}, 0, 1100)
        self.assertFalse(summary['acceptance_adjusted'])
        self.assertEqual(summary['owner']['etw_cpu_lower_ms'], 140)
        self.assertEqual(summary['owner']['etw_cpu_upper_ms'], 160)
        self.assertEqual(summary['process']['etw_cpu_lower_ms'], 930)
        self.assertEqual(summary['process']['etw_cpu_upper_ms'], 970)
        self.assertEqual(owner.between(100, 900), 150)
        self.assertEqual(report, before)

    def test_requires_trace_to_enclose_all_api_brackets(self):
        report = self.report()
        with self.assertRaisesRegex(ValueError, 'enclose OS API'):
            boundary_uncertainty_summary(report, {11: Timeline([(100, 1000)])}, 0, 935)

    def test_touching_read_windows_have_zero_lower_bound(self):
        report = self.report()
        report['frames'] = []
        report['boundary_uncertainty']['begin_owner'] = [110, 900]
        report['boundary_uncertainty']['begin_process'] = [900, 900]
        report['boundary_uncertainty']['end_owner'] = [900, 920]
        summary = boundary_uncertainty_summary(report, {11: Timeline([(100, 1000)])}, 0, 1100)
        self.assertEqual(summary['owner']['etw_cpu_lower_ms'], 0)


    def test_read_windows_never_relax_failed_two_percent_gate(self):
        import tempfile
        from pathlib import Path
        from perf_headless_account import account
        from test_perf_headless import report_fixture, trace_fixture
        report = report_fixture()
        report['envelope']['end'][1] = 350001000
        report['boundary_uncertainty'] = {
            'schema': 'cpu-envelope-brackets-v1', 'clock': 'qpc',
            'begin_owner': [101, 102], 'begin_process': [103, 104],
            'end_owner': [501, 502], 'end_process': [503, 504]}
        before = copy.deepcopy(report)
        with tempfile.TemporaryDirectory() as directory:
            path = Path(directory)/'trace.csv'
            path.write_text(trace_fixture(), encoding='utf-8')
            result = account(report, path)
        self.assertFalse(result['valid'])
        self.assertGreater(result['envelope']['owner_os_relative_error'], .02)
        self.assertFalse(result['boundary_uncertainty']['acceptance_adjusted'])
        self.assertEqual(report, before)


if __name__ == '__main__':
    unittest.main()
