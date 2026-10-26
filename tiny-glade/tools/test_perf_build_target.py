"""Requested-target acceptance tests; set TG_BUILD_JOB_SMOKE=1 for native isolation."""
import copy
import json
import os
from pathlib import Path
import subprocess
import sys
import tempfile
import time
import unittest

import perf_build_target as target


class RequestedTarget(unittest.TestCase):
    def setUp(self):
        self.temp = tempfile.TemporaryDirectory(prefix='requested-build-target-')
        self.addCleanup(self.temp.cleanup)
        self.root = Path(self.temp.name)
        self.executable = self.root/'target/release/deps/original_gameplay_chain.exe'
        self.executable.parent.mkdir(parents=True)
        self.executable.write_bytes(b'unit fixture; never executed')
        self.event = {'reason': 'compiler-artifact',
                      'target': {'name': target.TARGET, 'kind': ['test']},
                      'manifest_path': str(self.root/'crates/tiny-glade/Cargo.toml'),
                      'executable': str(self.executable),
                      'features': ['cpu-probe', 'default', 'mimalloc'],
                      'profile': {'test': True, 'opt_level': '3'}}
        self.command = ['cargo', 'build', '-p', 'tiny-glade', '--test', target.TARGET,
                        '--locked', '--offline', '-j1', '--message-format=json-render-diagnostics',
                        '--features', 'cpu-probe', '--release']

    def test_accepts_completed_requested_test_not_other_targets(self):
        self.assertTrue(target.validate_artifact(self.event, self.root, True, True))
        self.assertFalse(target.validate_artifact({'reason': 'compiler-artifact', 'target': {'name': 'tiny-glade-host'}}, self.root, True, True))
        self.assertFalse(target.validate_artifact({'reason': 'compiler-message', 'target': {'name': target.TARGET}}, self.root, True, True))

    def test_command_scope_rejects_other_cargo_work(self):
        target.validate_command(self.command)
        for old, new in [(target.TARGET, 'other_test'), ('build', 'test'), ('-j1', '-j8')]:
            with self.assertRaises(ValueError):
                target.validate_command([new if item == old else item for item in self.command])

    def test_rejects_feature_and_profile_mismatch(self):
        with self.assertRaisesRegex(ValueError, 'feature'):
            target.validate_artifact(self.event, self.root, False, True)
        for field, value in [('test', False), ('opt_level', '0')]:
            changed = copy.deepcopy(self.event)
            changed['profile'][field] = value
            with self.assertRaisesRegex(ValueError, 'profile'):
                target.validate_artifact(changed, self.root, True, True)

    def test_rejects_wrong_kind_or_missing_executable(self):
        for field, value in [('kind', ['bin']), ('kind', ['lib'])]:
            changed = copy.deepcopy(self.event)
            changed['target'][field] = value
            with self.assertRaises(ValueError):
                target.validate_artifact(changed, self.root, True, True)
        self.executable.unlink()
        with self.assertRaises(ValueError):
            target.validate_artifact(self.event, self.root, True, True)

    def test_rejects_different_package_or_target_directory(self):
        changed = copy.deepcopy(self.event)
        changed['manifest_path'] = str(self.root/'other/Cargo.toml')
        with self.assertRaises(ValueError):
            target.validate_artifact(changed, self.root, True, True)
        changed = copy.deepcopy(self.event)
        other = self.root/'other.exe'
        other.write_bytes(b'not requested target')
        changed['executable'] = str(other)
        with self.assertRaises(ValueError):
            target.validate_artifact(changed, self.root, True, True)


@unittest.skipUnless(os.name == 'nt' and os.environ.get('TG_BUILD_JOB_SMOKE') == '1',
                     'explicit Windows process isolation smoke: TG_BUILD_JOB_SMOKE=1')
class OwnedBuildJob(unittest.TestCase):
    def test_termination_contains_descendants_and_preserves_outsider(self):
        # All processes are created here. Never select or terminate by name/PID scan.
        outsider = subprocess.Popen([sys.executable, '-c', 'import time; time.sleep(90)'])
        process, job = None, target.Job()
        try:
            with tempfile.TemporaryDirectory(prefix='owned-cargo-job-test-') as temp:
                gate, ready = Path(temp)/'go', Path(temp)/'child.json'
                code = ('import pathlib,sys,time,subprocess,json; '
                        'gate=pathlib.Path(sys.argv[1]); ready=pathlib.Path(sys.argv[2]); '
                        '\nwhile not gate.exists(): time.sleep(.01)\n'
                        'p=subprocess.Popen([sys.executable,"-c","import time; time.sleep(90)"]); '
                        'ready.with_suffix(".tmp").write_text(json.dumps({"child":p.pid})); '
                        'ready.with_suffix(".tmp").replace(ready); p.wait()')
                process = subprocess.Popen([sys.executable, '-c', code, str(gate), str(ready)])
                job.assign(process)
                gate.write_text('assigned')
                deadline = time.monotonic()+20
                while not ready.exists():
                    self.assertIsNone(process.poll())
                    self.assertLess(time.monotonic(), deadline)
                    time.sleep(.01)
                self.assertGreater(json.loads(ready.read_text())['child'], 0)
                job.terminate()
                self.assertEqual(process.wait(timeout=15), target.TERMINATED_AFTER_TARGET)
                job.wait_empty()
                self.assertIsNone(outsider.poll(), 'outside process must survive Job termination')
        finally:
            job.close()
            if process is not None and process.poll() is None:
                process.wait(timeout=15)
            outsider.terminate()
            outsider.wait(timeout=15)


if __name__ == '__main__':
    unittest.main()
