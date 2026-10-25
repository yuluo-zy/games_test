import json, re, glob, os

rows = json.load(open('round3/REVIEW-UNEXPLORED/_rows_evid.json', encoding='utf-8'))

# Build filename -> full paths map across country-core-deep + adjacent evidence dirs
root = os.getcwd()
index = {}
for p in glob.glob('**/decomp-0x*.txt', recursive=True):
    base = os.path.basename(p)
    index.setdefault(base, []).append(p)
# also expansion-b2 and oa2-chunks and ui-input, backend-misc under country-core-deep? add:
for p in glob.glob('../**/decomp-0x*.txt', recursive=True):
    base = os.path.basename(p)
    index.setdefault(base, []).append(p)

missing, found = [], []
for r in rows:
    for m in re.finditer(r'decomp-0x[0-9a-f]+\.txt', r['evid']):
        fname = m.group(0)
        if fname in index:
            found.append((r['file'], r['addr'], fname, index[fname][0]))
        else:
            missing.append((r['file'], r['line'], r['name'], r['addr'], fname))

print('decomp refs found:', len(found), 'missing filenames:', len(missing))
print()
print('=== missing ===')
for f in missing:
    print('  %s | %s | %s' % (f[1], f[3], f[4]))
