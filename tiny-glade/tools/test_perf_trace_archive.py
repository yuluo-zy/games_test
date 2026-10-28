"""Lossless preservation and streaming replay; no capture, Cargo or sleep."""
import argparse
import gzip
import hashlib
import json
from pathlib import Path
import tempfile
import unittest
from unittest.mock import patch

import perf_headless as runner
import perf_trace_archive as archive
from perf_etw import read_trace
from perf_headless_account import account
from test_perf_headless import report_fixture, trace_fixture


def sha(data):
    return hashlib.sha256(data).hexdigest()


class TraceArchival(unittest.TestCase):
    def setUp(self):
        self.temp = tempfile.TemporaryDirectory()
        self.addCleanup(self.temp.cleanup)
        self.root = Path(self.temp.name)/'headless-runs'
        self.run = self.root/'new-run'
        self.run.mkdir(parents=True)
        self.csv = self.run/'schedule.csv'
        self.gz = self.run/'schedule.csv.gz'
        self.etl = self.run/'schedule.etl'
        self.etl.write_bytes(b'untouched original ETL')
        # Cross multiple chunks and preserve arbitrary bytes, BOM and CRLF.
        self.raw = b'\xef\xbb\xbf' + bytes(range(256)) * 9000 + b'\r\nlast\x00\xff'
        self.csv.write_bytes(self.raw)

    def archive(self):
        return archive.archive_run_csv(self.run, self.root, sha(self.raw))

    def test_exact_roundtrip_raw_and_compressed_sha_and_etl_retained(self):
        result = self.archive()
        self.assertFalse(self.csv.exists())
        self.assertEqual(gzip.decompress(self.gz.read_bytes()), self.raw)
        self.assertEqual(result['raw_sha256'], sha(self.raw))
        self.assertEqual(result['compressed_sha256'], sha(self.gz.read_bytes()))
        self.assertEqual(result['raw_bytes'], len(self.raw))
        self.assertEqual(result['compressed_bytes'], self.gz.stat().st_size)
        self.assertTrue(result['roundtrip_verified'])
        self.assertTrue(result['raw_removed'])
        self.assertEqual(self.etl.read_bytes(), b'untouched original ETL')

    def test_existing_archive_is_never_overwritten_and_raw_stays(self):
        self.gz.write_bytes(b'earlier archive')
        with self.assertRaises(FileExistsError):
            self.archive()
        self.assertEqual(self.csv.read_bytes(), self.raw)
        self.assertEqual(self.gz.read_bytes(), b'earlier archive')

    def test_compression_failure_preserves_raw(self):
        with patch.object(archive.gzip, 'GzipFile', side_effect=OSError('disk full')):
            with self.assertRaisesRegex(OSError, 'disk full'):
                self.archive()
        self.assertEqual(self.csv.read_bytes(), self.raw)
        self.assertFalse(self.gz.exists())

    def test_roundtrip_mismatch_preserves_raw(self):
        with patch.object(archive, '_verify_archive', return_value=('0'*64, len(self.raw))):
            with self.assertRaisesRegex(ValueError, 'roundtrip'):
                self.archive()
        self.assertEqual(self.csv.read_bytes(), self.raw)
        self.assertFalse(self.gz.exists())

    def test_wrong_recorded_raw_sha_preserves_raw(self):
        with self.assertRaisesRegex(ValueError, 'changed since'):
            archive.archive_run_csv(self.run, self.root, '0'*64)
        self.assertEqual(self.csv.read_bytes(), self.raw)
        self.assertFalse(self.gz.exists())

    def test_changed_source_during_verification_preserves_edited_raw(self):
        verify = archive._verify_archive
        def change(path):
            result = verify(path)
            self.csv.write_bytes(b'new bytes after compression')
            return result
        with patch.object(archive, '_verify_archive', side_effect=change):
            with self.assertRaisesRegex(ValueError, 'changed during'):
                self.archive()
        self.assertEqual(self.csv.read_bytes(), b'new bytes after compression')
        self.assertFalse(self.gz.exists())

    def test_unlink_failure_keeps_verified_archive_and_original(self):
        unlink = Path.unlink
        def reject(path, *args, **kwargs):
            if path == self.csv:
                raise PermissionError('original is busy')
            return unlink(path, *args, **kwargs)
        with patch.object(Path, 'unlink', reject):
            with self.assertRaisesRegex(PermissionError, 'busy'):
                self.archive()
        self.assertEqual(self.csv.read_bytes(), self.raw)
        self.assertEqual(gzip.decompress(self.gz.read_bytes()), self.raw)

    def test_interrupt_after_raw_removal_never_removes_verified_archive(self):
        unlink = Path.unlink
        def interrupt(path, *args, **kwargs):
            result = unlink(path, *args, **kwargs)
            if path == self.csv:
                raise KeyboardInterrupt()
            return result
        with patch.object(Path, 'unlink', interrupt), self.assertRaises(KeyboardInterrupt):
            self.archive()
        self.assertFalse(self.csv.exists())
        self.assertEqual(gzip.decompress(self.gz.read_bytes()), self.raw)

    def test_paths_outside_root_or_nested_run_are_rejected_without_changes(self):
        for wrong_root in (self.root/'prefix-sibling', self.root.parent):
            wrong_root.mkdir(exist_ok=True)
            with self.assertRaises(ValueError):
                archive.archive_run_csv(self.run, wrong_root, sha(self.raw))
        self.assertEqual(self.csv.read_bytes(), self.raw)
        self.assertFalse(self.gz.exists())

    def test_missing_original_etl_preserves_raw(self):
        self.etl.unlink()
        with self.assertRaisesRegex(ValueError, 'ETL'):
            self.archive()
        self.assertEqual(self.csv.read_bytes(), self.raw)

    def test_gzip_replay_has_identical_accounting_and_loss_detection(self):
        self.csv.write_bytes(b'\xef\xbb\xbf'+trace_fixture().replace('\n', '\r\n').encode())
        expected = account(report_fixture(), self.csv)
        result = archive.archive_run_csv(self.run, self.root, sha(self.csv.read_bytes()))
        self.assertEqual(account(report_fixture(), Path(result['path'])), expected)
        bad = self.run/'lost.csv.gz'
        with gzip.open(bad, 'wb') as out:
            out.write(b'OS Version: test, Events Lost: 1, Buffers lost: 0,\n')
        with self.assertRaisesRegex(ValueError, 'lost events'):
            read_trace(bad, 7, 1000)

    def test_run_option_and_no_trace_rejection(self):
        parser = argparse.ArgumentParser()
        runner.measurement_options(parser)
        self.assertFalse(parser.parse_args([]).archive_trace_csv)
        self.assertTrue(parser.parse_args(['--archive-trace-csv']).archive_trace_csv)
        with self.assertRaisesRegex(ValueError, 'requires an ETW'):
            runner.capture(argparse.Namespace(archive_trace_csv=True, without_etw=True))

    def test_capture_archives_only_after_official_and_account_completed(self):
        report = report_fixture()
        self.csv.write_text(trace_fixture(), encoding='utf-8', newline='\n')
        session = self.run/'session.json'
        session.write_text(json.dumps(report), encoding='utf-8')
        args = argparse.Namespace(samples=2, warmup=600, diagnostic=False,
                                  without_etw=False, official=True, archive_trace_csv=True)
        record = {'process_id': 7, 'cpu_probe_feature': True, 'scenario': 'static',
                  'valid': False, 'etw_stop_exit_code': 0}
        steps = []
        actual_archive = runner.archive_run_csv
        def official(*args):
            steps.append('official')
            return 390_000_000
        def accounted(*args):
            steps.append('account')
            return account(*args)
        def archived(*args):
            steps.append('archive')
            self.assertEqual(steps, ['official', 'account', 'archive'])
            self.assertTrue((self.run/'cpu.json').is_file())
            return actual_archive(*args)
        with patch.object(runner, 'PERF', self.root.parent), \
                patch.object(runner.subprocess, 'run'), \
                patch.object(runner, 'official_owner_total', side_effect=official), \
                patch.object(runner, 'account', side_effect=accounted), \
                patch.object(runner, 'archive_run_csv', side_effect=archived), patch('builtins.print'):
            result = runner.finish_capture(args, record, self.run, session,
                                           {'dirty': False, 'profile': 'release'}, 'on')
        self.assertTrue(result['valid'])
        self.assertEqual(json.loads((self.run/'run.json').read_text()), result)
        self.assertEqual(result['trace_csv_sha256'], result['trace_csv_archive']['raw_sha256'])
        self.assertEqual(result['trace_csv_path'], str(self.gz))
        self.assertFalse(self.csv.exists())


if __name__ == '__main__':
    unittest.main()
