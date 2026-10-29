"""Tests for x1_msl_translate.py using a stub SPIRV-Cross (no real translator needed)."""
import json
import os
import stat
import sys
import tempfile
import unittest

sys.path.insert(0, os.path.join(os.path.dirname(os.path.abspath(__file__)), 'x1'))
import x1_msl_translate as tool  # noqa: E402

STUB = '''#!/usr/bin/env python3
import sys
args = sys.argv[1:]
if args == ['--revision']:
    print('stub-1.0'); sys.exit(0)
spv = args[0]
out = args[args.index('--output') + 1]
data = open(spv, 'rb').read()
if b'FAIL_ALIAS' in data:
    sys.stderr.write('ERROR: Cannot alias resource %123 with 0x4f in binding 2\\n'); sys.exit(1)
body = 'kernel void main0(device uint* a [[buffer(%d)]], texture2d<float> t [[texture(0)]]) {}\\n'
n = 40 if b'TOO_MANY' in data else 3
open(out, 'w').write(body % n)
'''


def make_fixture(root):
    for name, payload in (('a', b'OK'), ('b', b'FAIL_ALIAS'), ('c', b'TOO_MANY')):
        d = os.path.join(root, name)
        os.makedirs(d)
        with open(os.path.join(d, 'lowered-0.spv'), 'wb') as f:
            f.write(payload)
        with open(os.path.join(d, 'meta.json'), 'w') as f:
            json.dump({'bin': name, 'file': f'{name}.cs.hlsl', 'stages': [{'index': 0, 'stage': 'Compute'}]}, f)


class TranslateTests(unittest.TestCase):
    def setUp(self):
        self.tmp = tempfile.TemporaryDirectory()
        self.addCleanup(self.tmp.cleanup)
        self.inp = os.path.join(self.tmp.name, 'in')
        os.makedirs(self.inp)
        make_fixture(self.inp)
        self.stub = os.path.join(self.tmp.name, 'spirv-cross')
        with open(self.stub, 'w') as f:
            f.write(STUB)
        os.chmod(self.stub, os.stat(self.stub).st_mode | stat.S_IEXEC)

    def run_tool(self, *extra):
        out = os.path.join(self.tmp.name, 'out')
        args = tool.parse_args([self.inp, '--spirv-cross', self.stub, '--out', out, '--jobs', '2', *extra])
        return tool.run(args), args

    def test_statuses_and_grouping(self):
        (version, compiled, rows), args = self.run_tool('--profiles', 'plain')
        self.assertEqual(version, 'stub-1.0')
        self.assertFalse(compiled)
        status = {r['bin']: r['status'] for r in rows}
        self.assertEqual(status, {'a': 'ok', 'b': 'translate_fail', 'c': 'over_limit'})
        reason = next(r['reason'] for r in rows if r['bin'] == 'b')
        self.assertIn('alias resource %N', reason)  # ids normalised so reasons group
        self.assertNotIn('123', reason)
        text = tool.render(version, compiled, rows, args)
        self.assertIn('| plain | 1 | 1 | 1 | 0 | 0 |', text)
        self.assertIn('exceeds buffer 41>31', text)  # slot index 40 -> 41 slots
        self.assertIn('with N in binding N', text)  # hex id normalised once

    def test_every_profile_is_run(self):
        (_, _, rows), _ = self.run_tool('--profiles', 'plain,argbuf-tier2')
        self.assertEqual({r['profile'] for r in rows}, {'plain', 'argbuf-tier2'})
        self.assertEqual(len(rows), 6)

    def test_unknown_profile_rejected(self):
        with self.assertRaises(SystemExit):
            tool.parse_args([self.inp, '--spirv-cross', self.stub, '--out', 'x', '--profiles', 'nope'])

    def test_static_checks_counts_highest_slot(self):
        msl = 'a [[buffer(0)]] b [[buffer(5)]] c [[texture(2)]] d [[sampler(15)]]'
        self.assertEqual(tool.static_checks(msl), {'buffer': 6, 'texture': 3, 'sampler': 16})


if __name__ == '__main__':
    unittest.main()
