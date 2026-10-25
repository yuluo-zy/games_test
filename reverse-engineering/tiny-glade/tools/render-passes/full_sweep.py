#!/usr/bin/env python3
"""Independent full-tree validation of human-readable-shaders vs shader-source.

For every manifest-keyed shader: compile orig (shader-source root) and new
(human-readable root, --new-include-root) with the same dxc command per variant
module; require identical debug-stripped SPIR-V hash (file_ok and full_ok).
Also checks: no `_NN` SPIR-V ids remain, and #if/#endif guard structure matches.
"""
import json
import re
import subprocess
import sys
import time
from pathlib import Path

BASE = Path('/Users/yuluo/Desktop/game/games_test/reverse-engineering/tiny-glade')
os = __import__('os')

manifest = json.load(open(BASE / 'evidence/render-passes/validation/readable-manifest.json'))
report = {'files': {}, 'summary': {'checked': 0, 'pass': 0, 'fail': 0, 'guard_mismatch': 0, 'nn_left': 0}}


def check_file(rel):
    orig = BASE / 'reconstruction/shader-source/shaders' / rel
    new = BASE / 'reconstruction/human-readable-shaders/shaders' / rel
    if not new.exists():
        return {'ok': False, 'reason': 'missing-new'}
    r = subprocess.run(
        [sys.executable, str(BASE / 'tools/render-passes/readable_check.py'),
         '--orig', str(orig), '--new', str(new),
         '--manifest', str(BASE / 'evidence/render-passes/validation/readable-manifest.json'),
         '--hlsl', rel,
         '--new-include-root', str(BASE / 'reconstruction/human-readable-shaders/shaders'),
         '--out', '/tmp/sweep_%s.json' % re.sub(r'[^A-Za-z0-9]', '_', rel)],
        capture_output=True, text=True)
    out = r.stdout
    m = re.search(r"verdict': '(\w+)'", out)
    ok = bool(m) and m.group(1) == 'PASS'
    # _NN leftover check
    nn = bool(re.search(r'\b_[0-9]+\b', new.read_text()))
    # guard structure
    def guards(p):
        return [l.strip() for l in p.read_text().split('\n')
                if l.strip().startswith(('#if', '#endif', '#elif', '#else'))]
    guard_ok = guards(orig) == guards(new)
    return {'ok': ok, 'reason': None if ok else out[-120:], 'nn': nn, 'guard': guard_ok}


start = time.time()
for rel in sorted(manifest):
    res = check_file(rel)
    report['files'][rel] = res
    report['summary']['checked'] += 1
    if res['ok']:
        report['summary']['pass'] += 1
    else:
        report['summary']['fail'] += 1
    if res.get('nn'):
        report['summary']['nn_left'] += 1
    if not res.get('guard', True):
        report['summary']['guard_mismatch'] += 1
    print('%s %s' % ('PASS' if res['ok'] else 'FAIL', rel), flush=True)

report['summary']['seconds'] = round(time.time() - start, 1)
json.dump(report, open('/tmp/full_sweep_report.json', 'w'), ensure_ascii=False, indent=1)
print('DONE', report['summary'])
