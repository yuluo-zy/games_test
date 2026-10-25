# -*- coding: utf-8 -*-
"""Full evidence sweep: every O row's decomp refs checked for existence + error-file status."""
import io, sys, re, os, json
from collections import Counter
sys.stdout.reconfigure(encoding='utf-8')
BASE = r'D:/game/reverse-engineering/tiny-glade/evidence/country-core-deep'
with io.open(os.path.join(BASE, 'round3/REVIEW-UNEXPLORED/groupA_rows.json'), encoding='utf-8') as f:
    rows = json.load(f)
decomp_re = re.compile(r'decomp[-\\]?(0x[0-9a-fA-F]+)\.txt')
loc_dirs = ['<dom>/decomp', 'decomp', 'round2', 'round3']
stats = Counter()
details = []
for dom in ['history', 'session-save', 'camera-time']:
    for r in rows[dom]:
        if r['concl'] != 'O':
            continue
        refs = decomp_re.findall(r.get('evid', ''))
        if not refs:
            stats['no-ref'] += 1
            continue
        for va in refs:
            found = None
            for d in loc_dirs:
                cand = os.path.join(BASE, d.replace('<dom>', dom), 'decomp-%s.txt' % va)
                if os.path.exists(cand):
                    found = cand
                    break
            if not found:
                stats['MISSING'] += 1
                details.append((dom, r['addr'], r['file'], va, 'MISSING'))
                continue
            with io.open(found, encoding='utf-8', errors='replace') as fh:
                c = fh.read(400)
            if 'NotFound' in c or '"error"' in c or 'No function' in c:
                stats['ERRORFILE'] += 1
                details.append((dom, r['addr'], r['file'], va, 'ERRORFILE ' + found.split('country-core-deep')[-1]))
            else:
                stats['OK'] += 1
print(stats)
print()
for d in details:
    print(d)
