import os
import sys
import tempfile
import unittest

sys.path.insert(0, os.path.dirname(os.path.abspath(__file__)))
import clean_superseded_test_binaries as c  # noqa: E402


def touch(path, size, mtime):
    with open(path, 'wb') as f:
        f.write(b'x' * size)
    os.utime(path, (mtime, mtime))


class SupersededTests(unittest.TestCase):
    def test_keeps_newest_per_name_and_never_touches_other_files(self):
        with tempfile.TemporaryDirectory() as d:
            touch(os.path.join(d, 'foo-0123456789abcdef'), 100, 1000)    # older copy of foo
            touch(os.path.join(d, 'foo-fedcba9876543210'), 100, 2000)    # newest foo: kept
            touch(os.path.join(d, 'bar-0123456789abcdef'), 100, 1500)    # only copy of bar: kept
            touch(os.path.join(d, 'foo-0123456789abcdef.d'), 100, 500)   # has an extension: ignored
            touch(os.path.join(d, 'libfoo-0123456789abcdef.rlib'), 100, 500)
            touch(os.path.join(d, 'tiny-0123456789abcdef'), 10, 500)     # below the size floor: ignored
            touch(os.path.join(d, 'notahash'), 100, 500)
            distinct, stale = c.superseded(d, 50)
            self.assertEqual(distinct, 2)
            self.assertEqual([os.path.basename(p) for _, _, p in stale], ['foo-0123456789abcdef'])

    def test_dry_run_deletes_nothing_and_apply_deletes_only_stale(self):
        with tempfile.TemporaryDirectory() as d:
            deps = os.path.join(d, 'debug', 'deps')
            os.makedirs(deps)
            old, new = os.path.join(deps, 'foo-0123456789abcdef'), os.path.join(deps, 'foo-fedcba9876543210')
            touch(old, 2_000_000, 1000)
            touch(new, 2_000_000, 2000)
            self.assertEqual(c.main(['--target-dir', d, '--min-mb', '1']), 0)
            self.assertTrue(os.path.exists(old))
            self.assertEqual(c.main(['--target-dir', d, '--min-mb', '1', '--apply']), 0)
            self.assertFalse(os.path.exists(old))
            self.assertTrue(os.path.exists(new))

    def test_missing_directory_is_an_error(self):
        with tempfile.TemporaryDirectory() as d:
            self.assertEqual(c.main(['--target-dir', d]), 2)


if __name__ == '__main__':
    unittest.main()
