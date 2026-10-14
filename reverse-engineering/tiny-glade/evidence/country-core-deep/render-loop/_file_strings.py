import re, glob, os, json
# Build addr -> string map from _string_table.txt
stbl = {}
for line in open('evidence/country-core-deep/render-loop/_string_table.txt', encoding='utf-8', errors='replace'):
    m = re.match(r'^([0-9a-f]+)\s+(.*)$', line.strip())
    if m:
        s = m.group(2)
        if s.startswith("'") and s.endswith("'"):
            stbl[m.group(1)] = s[1:-1]
out = {}
for fn in sorted(glob.glob('evidence/country-core-deep/render-loop/decomp/decomp-*.txt')):
    base = os.path.basename(fn)[7:-4]
    text = open(fn, encoding='utf-8', errors='replace').read()
    if text.startswith('{"error"'):
        out[base] = ['<NotFound>']
        continue
    addrs = set(re.findall(r'UNK_142[0-9a-f]+', text))
    strs = []
    for a in addrs:
        key = a[4:]
        if key in stbl and stbl[key]:
            strs.append(stbl[key])
    # de-dup preserving order, filter binary garbage
    seen = set(); clean = []
    for s in strs:
        if s in seen: continue
        seen.add(s)
        # printable-ish only
        if re.fullmatch(r'[\x20-\x7e]+', s) and len(s) > 1 and len(s) < 300:
            clean.append(s)
    out[base] = clean
json.dump(out, open('evidence/country-core-deep/render-loop/_per_file_strings.json','w'), ensure_ascii=False, indent=1)
print('wrote', len(out), 'files')
