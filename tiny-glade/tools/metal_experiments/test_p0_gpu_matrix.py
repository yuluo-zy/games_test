import os
import sys
import unittest

sys.path.insert(0, os.path.dirname(os.path.abspath(__file__)))
import p0_gpu_matrix as m  # noqa: E402


class PlanTests(unittest.TestCase):
    def test_interleaved_by_repeat(self):
        runs = m.plan(['static', 'many_draws'], ['quality', 'performance'], ['command', 'isolated'], 3)
        self.assertEqual(len(runs), 2 * 2 * 2 * 3)
        per_round = 2 * 2 * 2
        for round_index in range(3):
            chunk = runs[round_index * per_round:(round_index + 1) * per_round]
            self.assertEqual({r['repeat'] for r in chunk}, {round_index + 1})
        self.assertEqual(len({r['name'] for r in runs}), len(runs))

    def test_default_matrix_size(self):
        args = m.parse_args(['--build-json', 'b.json', '--output', 'o'])
        self.assertEqual(len(m.plan(args.scenes, args.qualities, args.modes, args.repeats)), 4 * 3 * 2 * 3)

    def test_isolated_mode_enables_all_pass_diagnostics_and_command_mode_none(self):
        args = m.parse_args(['--build-json', 'b.json', '--output', 'o'])
        iso = m.namespace_for({'name': 'n', 'scene': 'static', 'quality': 'balanced', 'mode': 'isolated'}, args)
        cmd = m.namespace_for({'name': 'n', 'scene': 'static', 'quality': 'balanced', 'mode': 'command'}, args)
        self.assertEqual((iso.pass_spans, iso.isolate, iso.stats), (True, True, True))
        self.assertEqual((cmd.pass_spans, cmd.isolate, cmd.stats), (False, False, False))
        self.assertEqual(iso.quality, 'balanced')

    def test_rejects_unknown_values(self):
        with self.assertRaises(SystemExit):
            m.parse_args(['--build-json', 'b', '--output', 'o', '--qualities', 'ultra'])
        with self.assertRaises(SystemExit):
            m.parse_args(['--build-json', 'b', '--output', 'o', '--repeats', '0'])

    def test_namespace_satisfies_controller_requirements(self):
        # The controller reads these attributes; a missing one would only fail on the Windows machine.
        sys.path.insert(0, os.path.join(os.path.dirname(os.path.abspath(__file__)), '..'))
        import perf_gpu_diagnostic as controller
        args = m.parse_args(['--build-json', 'b.json', '--output', 'o'])
        ns = m.namespace_for(m.plan(['static'], ['quality'], ['isolated'], 1)[0], args)
        for attr in ('build_json', 'output', 'workspace', 'install_root', 'scene', 'warmup', 'samples',
                     'capacity', 'pass_spans', 'isolate', 'stats', 'quality', 'timeout'):
            self.assertTrue(hasattr(ns, attr), attr)
        self.assertIn(ns.scene, controller.SCENES)


if __name__ == '__main__':
    unittest.main()
