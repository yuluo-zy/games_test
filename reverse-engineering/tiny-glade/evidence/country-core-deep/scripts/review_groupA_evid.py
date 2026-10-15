# -*- coding: utf-8 -*-
"""Evidence-chain sample: pick 10 O rows per instructions, verify cited decomp files exist and are not NotFound error files."""
import io, sys, re, os, json, random
sys.stdout.reconfigure(encoding='utf-8')
BASE = r'D:/game/reverse-engineering/tiny-glade/evidence/country-core-deep'

with io.open(os.path.join(BASE, 'round3/REVIEW-UNEXPLORED/groupA_rows.json'), encoding='utf-8') as f:
    rows = json.load(f)

decomp_re = re.compile(r'decomp[-\\]?(0x[0-9a-fA-F]+)\.txt')

def check_evidence(dom, r):
    evid = r.get('evid', '')
    refs = decomp_re.findall(evid)
    if not refs:
        return ('no-decomp-ref', None, None)
    results = []
    for va in refs:
        # candidates: <domain>/decomp/decomp-<va>.txt ; decomp/decomp-<va>.txt ; round2/...
        cands = []
        for d in (dom,):
            p = os.path.join(BASE, d, 'decomp', 'decomp-%s.txt' % va)
            cands.append(p)
        cands.append(os.path.join(BASE, 'decomp', 'decomp-%s.txt' % va))
        cands.append(os.path.join(BASE, 'round2', 'decomp-%s.txt' % va))
        found = None
        for c in cands:
            if os.path.exists(c):
                found = c
                break
        if found is None:
            results.append((va, 'MISSING', None))
            continue
        try:
            with io.open(found, encoding='utf-8', errors='replace') as fh:
                content = fh.read(600)
        except Exception as e:
            results.append((va, 'READERR', str(e)))
            continue
        if '"error"' in content or 'NotFound' in content or 'No function' in content:
            results.append((va, 'ERRORFILE', found))
        elif len(content.strip()) < 50:
            results.append((va, 'SUSPICIOUS-SMALL', found))
        else:
            results.append((va, 'OK', found))
    return ('refs', results, None)

random.seed(20261015)
for dom in ['history', 'session-save', 'camera-time']:
    orows = [r for r in rows[dom] if r['concl'] == 'O']
    sample = random.sample(orows, min(10, len(orows)))
    print('=' * 25, dom, 'O rows:', len(orows), 'sampled:', len(sample))
    for r in sample:
        st, res, _ = check_evidence(dom, r)
        if st == 'no-decomp-ref':
            print('  [%s] %s:%s %s | evidence no decomp ref: %r' % (r['addr'], r['file'], r['line'], r['fn'][:70], r['concl_raw'][:100]))
        else:
            for va, status, path in res:
                print('  [%s] %s:%s %s | evid decomp-%s.txt -> %s %s' % (r['addr'], r['file'], r['line'], r['fn'][:60], va, status, (path or '')))
