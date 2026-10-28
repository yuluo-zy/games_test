"""Portable paths/declarations only; these tests never start a calibration."""
import argparse
import json
import os
from pathlib import Path
import subprocess
import sys
import tempfile
import unittest
from unittest.mock import patch

import predeclare
import thread_cpu_diagnostic as runner


class PortableDeclaration(unittest.TestCase):
    def fake_metadata(self, nonce):
        return {'identities': runner.identities(),
                'source': {'runtime_content_sha256': predeclare.signature(runner.identities()),
                           'git': {'available': False, 'commit': None, 'dirty': None}},
                'hardware': {'os': 'Windows-test', 'physical_cpu': {'Name': 'fixture'}, 'logical_cpus': 8},
                'host_binding': {'method': 'synthetic unit fixture', 'sha256': predeclare.signature(nonce)},
                'python': {'path': sys.executable, 'sha256': '1'*64, 'version': sys.version},
                'xperf': {'path': 'not executed', 'sha256': '2'*64}}

    def test_predeclare_only_writes_new_manifest_without_starting_any_run(self):
        with tempfile.TemporaryDirectory() as directory, \
                patch.object(predeclare, 'metadata', side_effect=self.fake_metadata), \
                patch.object(runner, 'capture', side_effect=AssertionError('capture forbidden')), \
                patch.object(runner, 'synthetic_pair', side_effect=AssertionError('pair forbidden')), \
                patch.object(runner.perf, 'start_trace', side_effect=AssertionError('logger forbidden')), \
                patch.object(runner, 'WindowsCPU', side_effect=AssertionError('native workload forbidden')), \
                patch.object(predeclare.subprocess, 'Popen', side_effect=AssertionError('child forbidden')):
            path = Path(directory)/'new.json'
            result = predeclare.create('fresh-series', path)
            declaration = json.loads(path.read_text())
            self.assertFalse(result['measurement_started'])
            self.assertEqual(declaration['pairs'], 3)
            self.assertEqual(declaration['workload']['workers'], 4)
            self.assertEqual(declaration['workload']['chunks_per_worker'], 4096)
            self.assertFalse(declaration['total_cpu_work_identical_claimed'])
            self.assertIn('--predeclared', result['command_argv'])
            self.assertEqual(declaration['workload_sha256'], 'bfbaae1eb6959dc93c97e5672cc8e2a186d3ca27968e175293d0bf84bf3534be')
            with self.assertRaises(FileExistsError):
                predeclare.create('fresh-series', path)

    def test_runtime_closure_uses_twelve_repository_relative_sha_keys(self):
        ids = runner.identities()
        self.assertEqual(len(ids), 12)
        for name in ('perf_build_target.py', 'perf_clock_policy.py', 'perf_windows.py'):
            self.assertIn('tools/'+name, ids)
        for name, digest in ids.items():
            self.assertFalse(Path(name).is_absolute())
            self.assertNotIn('..', Path(name).parts)
            self.assertEqual(len(digest), 64)

    def test_new_series_settings_host_and_source_must_match_declaration(self):
        with tempfile.TemporaryDirectory() as directory, patch.object(predeclare, 'metadata', side_effect=self.fake_metadata):
            path = Path(directory)/'new.json'
            predeclare.create('fresh-series', path)
            args = argparse.Namespace(series='fresh-series', predeclared=path, **predeclare.SETTINGS)
            binding = predeclare.validate_series(args)
            args.declaration_binding = binding
            predeclare.verify_capture_binding(args)
            args.workers = 1
            with self.assertRaisesRegex(ValueError, 'settings differ'):
                predeclare.validate_series(args)
            args.workers = 4
            with patch.object(predeclare, 'metadata', return_value={**self.fake_metadata(binding['nonce']),
                        'host_binding': {'method': 'another host', 'sha256': '3'*64}}):
                with self.assertRaisesRegex(ValueError, 'host/source/tool identity'):
                    predeclare.validate_series(args)

    def test_historical_declaration_or_missing_binding_cannot_start_new_package(self):
        with tempfile.TemporaryDirectory() as directory:
            path = Path(directory)/'old.json'
            path.write_text(json.dumps({'schema': 'predeclared-cpu-clock-calibration-v1'}))
            with self.assertRaisesRegex(ValueError, 'historical declarations'):
                predeclare.validate_series(argparse.Namespace(predeclared=path))
            with self.assertRaisesRegex(ValueError, 'newly validated'):
                predeclare.verify_capture_binding(argparse.Namespace())

    def test_repo_discovery_is_independent_of_working_directory(self):
        package = Path(__file__).resolve().parent
        with tempfile.TemporaryDirectory() as directory:
            code = 'import sys,json;sys.path.insert(0,'+repr(str(package))+');import thread_cpu_diagnostic as r;print(json.dumps([str(r.REPO_TOOLS),str(r.perf.__file__)]))'
            result = subprocess.check_output([sys.executable, '-B', '-c', code], cwd=directory, text=True)
        tools, module = json.loads(result)
        self.assertEqual(Path(tools).resolve(), package.parent)
        self.assertEqual(Path(module).resolve().parent, package.parent)

    def test_preloaded_foreign_perf_module_is_rejected_instead_of_using_other_checkout(self):
        package = Path(__file__).resolve().parent
        with tempfile.TemporaryDirectory() as directory:
            fake = Path(directory)/'perf_headless.py'
            fake.write_text('FOREIGN=True\n', encoding='utf-8')
            code = 'import sys;sys.path.insert(0,'+repr(directory)+');import perf_headless;sys.path.insert(0,'+repr(str(package))+');import thread_cpu_diagnostic'
            result = subprocess.run([sys.executable, '-B', '-c', code], cwd=directory, capture_output=True, text=True)
        self.assertNotEqual(result.returncode, 0)
        self.assertIn('different repository', result.stderr)

    def test_transitive_foreign_perf_dependency_is_rejected_before_import(self):
        package = Path(__file__).resolve().parent
        for name in ('perf_windows', 'perf_headless_account'):
            with self.subTest(module=name), tempfile.TemporaryDirectory() as directory:
                (Path(directory)/(name+'.py')).write_text('FOREIGN=True\n', encoding='utf-8')
                code = 'import sys;sys.path.insert(0,'+repr(directory)+');import '+name+';sys.path.insert(0,'+repr(str(package))+');import thread_cpu_diagnostic'
                result = subprocess.run([sys.executable, '-B', '-c', code], cwd=directory, capture_output=True, text=True)
            self.assertNotEqual(result.returncode, 0)
            self.assertIn('different repository', result.stderr)


if __name__ == '__main__':
    unittest.main()
