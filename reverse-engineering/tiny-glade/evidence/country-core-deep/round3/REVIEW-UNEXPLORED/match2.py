import json, re
from collections import Counter

biz = json.load(open('round3/REVIEW-UNEXPLORED/_biz.json', encoding='utf-8'))
rows = json.load(open('round3/REVIEW-UNEXPLORED/_rows.json', encoding='utf-8'))

def short(fullname):
    n = fullname.split('::')[-1]
    return re.sub(r'<.*>', '', n)

addr2row = {}
for r in rows:
    for m in re.finditer(r'0x[0-9a-fA-F]+', r['addr']):
        addr2row.setdefault(m.group(0).lower(), []).append(r)

unmatched, matched = [], []
for k, funcs in biz.items():
    for f in funcs:
        hit = None
        if f.get('va'):
            for va in f['va']:
                if va.lower() in addr2row:
                    hit = addr2row[va.lower()][0]
                    break
        if hit is None:
            sn = short(f['name'])
            for r in rows:
                if short(r['name']) == sn and k.split('/')[0] in r['file']:
                    hit = r
                    break
        (matched if hit else unmatched).append((k, f['name'], (f.get('va') or ['?'])[0]))

print('matched:', len(matched), 'unmatched:', len(unmatched))
c = Counter(u[0] for u in unmatched)
for k in sorted(c):
    print('  %s: %d' % (k, c[k]))

print()
print('=== unmatched details (non-ssbo) ===')
for u in unmatched:
    if 'ssbo_library' not in u[0]:
        print('  %s | %s | %s' % (u[0], u[1][-100:], u[2]))

# ssbo family breakdown
fam = Counter()
for u in unmatched:
    if 'ssbo_library' in u[0]:
        n = u[1]
        m = re.search(r'::impl\$(\d+)::(\w+)<', n)
        if m:
            fam[(m.group(1), m.group(2))] += 1
        else:
            m2 = re.search(r'::(\w+)<', n)
            fam[('?', m2.group(1) if m2 else '?')] += 1
print()
print('=== ssbo unmatched families ===')
for k, v in sorted(fam.items()):
    print('  impl%s::%s: %d' % (k[0], k[1], v))

# the single I row
print()
print('=== I conclusion rows ===')
for r in rows:
    if r['concl'] == 'I':
        print('  ', r)
