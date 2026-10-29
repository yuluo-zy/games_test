import os
import sys
import unittest

sys.path.insert(0, os.path.dirname(os.path.abspath(__file__)))
import x2_mac_moltenvk as x2  # noqa: E402


class EnvTests(unittest.TestCase):
    def args(self, *extra):
        a = x2.parse_args(['--install-root', '/game', '--output', 'out', *extra])
        a.icd = '/icd/MoltenVK_icd.json'
        return a

    def test_environment_comes_from_the_controller(self):
        env = x2.build_environment(self.args('--scene', 'many_draws', '--quality', 'performance', '--isolate', '--stats'), '/s.json')
        self.assertEqual(env['TG_PERF_SCENARIO'], 'many_draws')
        self.assertEqual(env['TG_PERF_QUALITY'], 'performance')
        self.assertEqual((env['TG_GPU_PASS_SPANS'], env['TG_GPU_ISOLATE'], env['TG_GPU_STATS']), ('1', '1', '1'))
        self.assertEqual(env['TG_CPU_PROBE_OUT'], '/s.json')
        self.assertEqual(env['TG_INSTALL_ROOT'], '/game')

    def test_validation_is_dropped_unless_requested(self):
        env = x2.build_environment(self.args(), '/s.json')
        self.assertNotIn('VK_INSTANCE_LAYERS', env)
        self.assertNotIn('RHAPSODY_REQUIRE_VULKAN', env)
        env = x2.build_environment(self.args('--validation'), '/s.json')
        self.assertEqual(env['VK_INSTANCE_LAYERS'], 'VK_LAYER_KHRONOS_validation')
        self.assertEqual(env['VK_ICD_FILENAMES'], '/icd/MoltenVK_icd.json')

    def test_pass_spans_always_on_without_isolation(self):
        env = x2.build_environment(self.args(), '/s.json')
        self.assertEqual(env['TG_GPU_PASS_SPANS'], '1')
        self.assertNotIn('TG_GPU_ISOLATE', env)

    def test_command_runs_the_fixture_the_controller_runs(self):
        import perf_gpu_diagnostic as controller
        self.assertIn(controller.TEST, x2.cargo_command(self.args()))
        self.assertIn('cpu-probe', x2.cargo_command(self.args()))


if __name__ == '__main__':
    unittest.main()
