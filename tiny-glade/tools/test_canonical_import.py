"""Synthetic complete relocation controls, never runnable EXEs or real sources."""
import base64
import copy
import json
from pathlib import Path
import subprocess
import tempfile
import unittest
from unittest.mock import patch

import perf_canonical_contract as canonical
import perf_import_frozen_bundle_canonical as importer
import perf_headless_series as series
import test_perf_import_frozen_bundle as old
from test_canonical_contract import RAW, save


class CanonicalImportTests(unittest.TestCase):
    """Reuse historical synthetic freeze builder with a real canonical closure."""
    write_source = old.ImportTests.write_source
    dump = old.ImportTests.dump
    plan = old.ImportTests.plan
    reject = old.ImportTests.reject

    def setUp(self):
        self.tmp = tempfile.TemporaryDirectory(prefix='canonical-import-synthetic-')
        self.addCleanup(self.tmp.cleanup)
        self.area = Path(self.tmp.name); self.root = self.area/'source'; self.host = self.area/'host'
        self.root.mkdir(); self.host.mkdir()
        for root in (self.root, self.host):
            for name in importer.TOOLS:
                target = root/'tools'/name; target.parent.mkdir(parents=True, exist_ok=True)
                target.write_bytes((Path('C:/game/tools')/name).read_bytes())
        self.write_source('.gitignore', '/.perf/\n/target/\n')
        self.write_source('Cargo.lock', 'synthetic unchanged lock')
        self.write_source('crates/tiny-glade/Cargo.toml', '[package]\nname="tiny-glade"\nversion="0.1.0"\n')
        for name, raw in RAW.items():
            path = self.root/name; path.parent.mkdir(parents=True, exist_ok=True); path.write_bytes(raw)
        for args in (('init', '-q'), ('config', 'core.autocrlf', 'false'),
                     ('config', 'user.name', 'Synthetic'), ('config', 'user.email', 'synthetic@example.invalid'),
                     ('add', '.'), ('commit', '-qm', 'synthetic canonical source only')):
            subprocess.run(['git', *args], cwd=self.root, env=importer.isolated_git_env(),
                           check=True, capture_output=True)
        self.source = importer.source_snapshot(self.root)
        self.label = 'synthetic-canonical'; self.bundle = self.root/'.perf/headless-builds'/self.label
        self.bundle.mkdir(parents=True)
        self.exe = 'original_gameplay_chain-0123456789abcdef.exe'; self.pdb = self.exe[:-4]+'.pdb'
        for name in (self.exe, self.pdb, 'fmod.dll', 'fmodstudio.dll'):
            (self.bundle/name).write_bytes(('synthetic not executable '+name).encode())
        (self.bundle/'vulkan-summary.txt').write_bytes(b'synthetic, no hardware evidence')
        hardware = {'vulkan_summary_sha256': importer.digest(self.bundle/'vulkan-summary.txt')}
        lifecycle = {'requested_target_completed': True, 'active_build_processes_remaining': False,
            'cargo_exit_code': None, 'entire_cargo_build_passed': False,
            'termination_was_intentional_after_requested_artifact': True,
            'job_termination_exit_code': 0xE000C0DE, 'job_assigned_before_cargo_start': True,
            'wrapper_pid': 101, 'cargo_pid': 102}
        artifact = {'reason': 'compiler-artifact', 'target': {'name': importer.TARGET, 'kind': ['test']},
            'features': ['cpu-probe', 'default', 'mimalloc'], 'profile': {'test': True, 'opt_level': '3'},
            'manifest_path': str(self.root/'crates/tiny-glade/Cargo.toml'),
            'executable': str(self.root/'target/release/deps'/self.exe)}
        for name, value in (('source.json', self.source), ('hardware.json', hardware),
                ('cargo-lifecycle.json', lifecycle), ('requested-target-artifact.json', artifact),
                ('build-job-spec.json', {'cwd': str(self.root), 'command': importer.COMMAND,
                                       'gate': str(self.bundle/'build-job-assigned.go')})):
            self.dump(name, value)
        start = {'reason': 'perf-owned-cargo-start', 'pid': 102, 'wrapper_pid': 101}
        (self.bundle/'build.jsonl').write_text(json.dumps(start)+'\n'+json.dumps(artifact)+'\n')
        (self.bundle/'build-job-assigned.go').write_text('assigned')
        (self.bundle/'build.stderr.log').write_text('synthetic log')
        (self.bundle/'test-list.txt').write_text('cpu_measurement::profile_shipped_session_cpu: test\n')
        self.build = {'schema': 'headless-frozen-build-v1', 'label': self.label,
            'source_commit': self.source['commit'], 'source_content_sha256': self.source['content_sha256'],
            'profile': 'release', 'dirty': False, 'features': ['default(mimalloc)', 'cpu-probe'],
            'cpu_probe_feature': True, 'allocator': 'mimalloc', 'jobs': 1, 'incremental': False,
            'command': importer.COMMAND, 'build_environment': {}, 'build_environment_sha256': importer.signature({}),
            'management_controls': importer.caps(self.root, self.source), 'executable': str(self.bundle/self.exe),
            'files': {name: importer.digest(self.bundle/name) for name in (self.exe, self.pdb, 'fmod.dll', 'fmodstudio.dll')},
            'executable_sha256': importer.digest(self.bundle/self.exe), 'requested_test_target_completed': True,
            'cargo_lifecycle': lifecycle, 'entire_cargo_build_passed': False,
            'requested_target_evidence_sha256': importer.digest(self.bundle/'requested-target-artifact.json'),
            'build_jsonl_sha256': importer.digest(self.bundle/'build.jsonl'),
            'build_helper_sha256': importer.TOOLS['perf_build_target.py'], 'hardware': hardware}
        self.dump('build.json', self.build)

    def canonical_plan(self):
        return importer.prepare_plan(self.bundle, self.root, self.host, self.source['commit'],
            self.source['content_sha256'], importer.digest(self.bundle/'build.json'))

    def test_complete_import_keeps_original_bytes_and_cold_verifies_without_origin(self):
        original = importer.inventory(self.bundle); build_bytes = (self.bundle/'build.json').read_bytes()
        plan = self.canonical_plan(); result = importer.execute_plan(plan); dest = Path(plan['destination'])
        self.assertEqual(importer.inventory(self.bundle), original)
        self.assertEqual((dest/'original-build.json').read_bytes(), build_bytes)
        expected = dict(self.build); expected['executable'] = str(dest/self.exe)
        self.assertEqual(importer.read(dest/'build.json'), expected)
        self.assertEqual(result['canonical_source_contract'], plan['canonical_source_contract'])
        self.assertFalse(result['entire_cargo_build_passed_unchanged'])
        with patch.object(importer, 'validate_origin', side_effect=AssertionError('must not read origin')):
            self.assertEqual(importer.verify_import(dest, check_origin=False), result)
        # Same exact verifier proof enters series' cold imported binding.
        binding = series.bundle_import_binding(dest/'build.json', initial=True)
        self.assertEqual(binding['schema'], series.CANONICAL_IMPORT_CONTRACT)
        self.root.rename(self.area/'removed-origin')
        self.assertEqual(series.bundle_import_binding(dest/'build.json', declared=binding), binding)

    def test_changed_or_missing_archived_helper_rejects_cold_without_origin(self):
        plan = self.canonical_plan(); importer.execute_plan(plan); dest = Path(plan['destination'])
        archive_path = dest/'canonical-source-archive.json'; archive = importer.read(archive_path)
        helper = 'crates/tiny-glade/tests/support/performance_sparse_layout.rs'
        archive['files'][helper]['raw_base64'] = base64.b64encode(b'changed helper').decode()
        save(archive_path, archive)
        with self.assertRaisesRegex(ValueError, 'archive bytes|archive changed'):
            importer.verify_import(dest, check_origin=False)
        archive_path.unlink()
        with self.assertRaises((ValueError, FileNotFoundError)):
            importer.verify_import(dest, check_origin=False)

    def test_same_plan_cannot_rewrite_canonical_source_contract(self):
        plan = self.canonical_plan(); plan['canonical_source_contract'] = {}
        with self.assertRaisesRegex(ValueError, 'origin changed'):
            importer.execute_plan(plan)

    def test_extra_derived_metadata_and_unknown_verifier_never_pass(self):
        plan = self.canonical_plan(); importer.execute_plan(plan); dest = Path(plan['destination'])
        derived = importer.read(dest/'build.json'); derived['entire_cargo_build_passed'] = True
        save(dest/'build.json', derived)
        with self.assertRaisesRegex(ValueError, 'more than executable'):
            importer.verify_import(dest, check_origin=False)
        (dest/'bundle-importer.py').write_text('# unqualified code\n')
        with self.assertRaisesRegex(ValueError, 'unqualified'):
            series.import_verifier(dest)


if __name__ == '__main__':
    unittest.main()
