import json, re, glob, os

rows = json.load(open('round3/REVIEW-UNEXPLORED/_rows_evid.json', encoding='utf-8'))

# 1. rows whose evidence mentions NotFound/bad files
print('=== BAD-evidence rows ===')
for r in rows:
    if 'NotFound' in r['evid'] or '坏文件' in r['evid'] or '错误文件' in r['evid']:
        print('  %s:%d %s | %s | %s' % (r['file'], r['line'], r['name'][:60], r['addr'], r['evid'][:100]))

# 2. extract decomp paths and verify existence
missing = []
found = []
for r in rows:
    for m in re.finditer(r'([A-Za-z0-9_\-/\\]+/)?decomp-0x[0-9a-f]+\.txt', r['evid']):
        path = m.group(0)
        # try relative resolution from evidence/country-core-deep root and other dirs
        candidates = [
            path,
            'round3/' + path,
            'round3/oa2-chunks/' + path,
            'chapters/' + path,
        ]
        ok = False
        for c in candidates:
            if os.path.exists(c):
                ok = True
                break
        if not ok:
            missing.append((r['file'], r['name'], r['addr'], path, r['evid'][:140]))
        else:
            found.append((r['file'], r['addr'], path))
print()
print('decomp refs found:', len(found), 'missing:', len(missing))
for f in missing[:40]:
    print('  MISS %s | %s | %s' % (f[3], f[1][:50], f[4][:110]))
