import json, re, glob, os

rows = json.load(open('round3/REVIEW-UNEXPLORED/_rows_evid.json', encoding='utf-8'))
index = {}
for p in glob.glob('**/decomp-0x*.txt', recursive=True) + glob.glob('../**/decomp-0x*.txt', recursive=True):
    base = os.path.basename(p)
    index.setdefault(base, []).append(p)

# check each referenced file's content for NotFound / badness
bad = []
checked = set()
for r in rows:
    for m in re.finditer(r'decomp-0x[0-9a-f]+\.txt', r['evid']):
        fname = m.group(0)
        if fname in checked:
            continue
        checked.add(fname)
        for p in index.get(fname, []):
            try:
                head = open(p, encoding='utf-8', errors='replace').read(4000)
            except Exception:
                bad.append((fname, p, 'UNREADABLE'))
                continue
            if 'No function at' in head or 'NotFound' in head or '"error"' in head:
                bad.append((fname, p, 'NOTFOUND-STUB'))
            elif len(head.strip()) < 50:
                bad.append((fname, p, 'TINY(%dB)' % len(head)))

print('checked files:', len(checked))
print('bad files:', len(bad))
for b in bad:
    print('  %s | %s | %s' % b)
