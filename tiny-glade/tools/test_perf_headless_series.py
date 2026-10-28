"""Offline synthetic admission tests; fake CPU values never establish a benchmark."""
import copy
from datetime import datetime, timezone
import json
from pathlib import Path
import tempfile
import unittest
from unittest.mock import patch

import perf_headless_series as series


def save(path, value):
    Path(path).parent.mkdir(parents=True, exist_ok=True)
    Path(path).write_text(json.dumps(value), encoding='utf-8')


def synthetic_row(p95=10, delta=0):
    ratios = {m: {'p95_ms': delta, 'p99_ms': delta} for m in series.METRICS}
    return {'ratios': ratios, 'candidate_render_p95_ms': p95, 'failures': [],
            'cost_shift_flags': []}


class DecisionTests(unittest.TestCase):
    def rows(self, n=3, **kwargs):
        return {scene: [synthetic_row(**kwargs) for _ in range(n)] for scene in series.SCENES}

    def test_three_safe_pairs_pass_headless_only(self):
        self.assertTrue(series.decide(self.rows())['headless_cpu_gate_passed'])

    def test_exact_near_regression_triggers_five(self):
        self.assertEqual(series.decide(self.rows(delta=.03))['status'], 'needs_five_pairs')

    def test_exact_near_p95_triggers_five(self):
        self.assertEqual(series.decide(self.rows(p95=15.2))['status'], 'needs_five_pairs')

    def test_variable_pair_deltas_trigger_five(self):
        rows = self.rows()
        rows['static'][1]['ratios'][series.METRICS[0]]['p99_ms'] = -.04
        self.assertEqual(series.decide(rows)['status'], 'needs_five_pairs')

    def test_variable_candidate_p95_triggers_five(self):
        rows = self.rows()
        rows['static'][1]['candidate_render_p95_ms'] = 11
        self.assertEqual(series.decide(rows)['status'], 'needs_five_pairs')

    def test_failed_run_never_averaged_away(self):
        rows = self.rows(n=5)
        rows['static'][0]['failures'] = ['candidate p95 exceeded16']
        self.assertEqual(series.decide(rows)['status'], 'rejected')

    def test_five_stable_near_passes(self):
        self.assertTrue(series.decide(self.rows(n=5, delta=.04))['headless_cpu_gate_passed'])

    def test_five_uncertain_ratios_inconclusive(self):
        rows = self.rows(n=5)
        for scene in rows:
            for row, delta in zip(rows[scene], (-.08, .049, .049, .049, .049)):
                row['ratios'][series.METRICS[0]]['p95_ms'] = delta
        self.assertEqual(series.decide(rows)['status'], 'inconclusive')

    def test_missing_scene_rejected(self):
        rows = self.rows(); del rows['continuous_edit']
        with self.assertRaises(ValueError): series.decide(rows)

    def test_four_pairs_rejected(self):
        with self.assertRaises(ValueError): series.decide(self.rows(n=4))

    def test_nonowner_signal_does_not_invent_hard_gate(self):
        rows = self.rows()
        rows['static'][0]['ratios']['non_owner_cpu_ms']['p95_ms'] = 1
        self.assertTrue(series.decide(rows)['headless_cpu_gate_passed'])



if __name__ == '__main__':
    unittest.main()
