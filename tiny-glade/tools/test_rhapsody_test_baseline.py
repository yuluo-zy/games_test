import os
import sys
import tempfile
import unittest

sys.path.insert(0, os.path.dirname(os.path.abspath(__file__)))
import rhapsody_test_baseline as t  # noqa: E402

LOG = """\
     Running unittests src/lib.rs (target/debug/deps/rhapsody-0123456789abcdef)

running 2 tests
test a::first ... ok
test a::second ... FAILED
     Running tests/one.rs (target/debug/deps/one-fedcba9876543210)

running 3 tests
test a::first ... ok
test slow ... ignored, needs a device
test other ... ok
   Doc-tests rhapsody

running 1 test
test src/lib.rs - doc (line 3) ... ok
"""


class ParseTests(unittest.TestCase):
    def test_binary_names_are_stable_and_same_test_name_in_two_binaries_is_distinct(self):
        rows = t.parse(LOG)
        self.assertIn(('rhapsody', 'a::first', 'ok'), rows)
        self.assertIn(('one', 'a::first', 'ok'), rows)  # same test name, other binary
        self.assertIn(('one', 'slow', 'ignored'), rows)
        self.assertIn(('doctest:rhapsody', 'src/lib.rs - doc (line 3)', 'ok'), rows)
        self.assertEqual(t.summarise(rows), {'ok': 4, 'FAILED': 1, 'ignored': 1})


class CompareTests(unittest.TestCase):
    def base(self):
        return {('b', 'pass'): 'ok', ('b', 'known'): 'FAILED', ('b', 'ign'): 'ignored', ('b', 'gone'): 'ok'}

    def test_regression_removal_and_flip_are_flagged_known_failure_is_not(self):
        after = {('b', 'pass'): 'FAILED', ('b', 'known'): 'FAILED', ('b', 'ign'): 'ok', ('b', 'new'): 'ok'}
        r = t.compare(self.base(), after)
        self.assertEqual(r['regress'], [('b', 'pass')])
        self.assertEqual(r['removed'], [('b', 'gone')])
        self.assertEqual(r['flips'], [('b', 'ign')])
        self.assertEqual(r['still_failing'], [('b', 'known')])
        self.assertEqual(r['added'], [('b', 'new')])

    def test_identical_is_clean_and_exit_status_reflects_it(self):
        with tempfile.TemporaryDirectory() as d:
            a, b = os.path.join(d, 'a.tsv'), os.path.join(d, 'b.tsv')
            t.write_tsv(sorted((k[0], k[1], v) for k, v in self.base().items()), a)
            t.write_tsv(sorted((k[0], k[1], v) for k, v in self.base().items()), b)
            self.assertEqual(t.main(['compare', a, b]), 0)
            changed = dict(self.base()); changed[('b', 'pass')] = 'FAILED'
            t.write_tsv(sorted((k[0], k[1], v) for k, v in changed.items()), b)
            self.assertEqual(t.main(['compare', a, b]), 1)

    def test_a_disappearing_test_is_a_change(self):
        after = dict(self.base()); del after[('b', 'gone')]
        self.assertTrue(t.compare(self.base(), after)['removed'])


if __name__ == '__main__':
    unittest.main()
