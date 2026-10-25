import json, re, glob, os

# Re-parse chapters capturing the 证据 column for each O row
rows = []
for pat in ['chapters/asset_libraries/*.rs.md','chapters/utils/*.rs.md','chapters/startup/*.rs.md',
            'chapters/startup/glade_themes/*.rs.md','chapters/input/*.rs.md','chapters/components/*.rs.md',
            'chapters/geometry/*.rs.md']:
    for p in sorted(glob.glob(pat)):
        text = open(p, encoding='utf-8').read()
        for i, line in enumerate(text.splitlines(), 1):
            line = line.strip()
            if not line.startswith('|'):
                continue
            cells = [c.strip() for c in line.strip('|').split('|')]
            if len(cells) < 6:
                continue
            m = re.search(r'\*\*(O|I|U)\*\*', cells[4])
            if not m:
                continue
            evid = cells[5]
            rows.append({'file': p.replace('\\','/'), 'name': cells[0], 'addr': cells[1],
                         'concl': m.group(1), 'evid': evid, 'line': i})

json.dump(rows, open('round3/REVIEW-UNEXPLORED/_rows_evid.json','w',encoding='utf-8'), ensure_ascii=False)
print('rows with evid:', len(rows))

# classify evidence: decomp file paths vs raw-byte vs notfound patterns
from collections import Counter
pat = Counter()
for r in rows:
    e = r['evid']
    if 'NotFound' in e or '坏文件' in e or '错误文件' in e:
        pat['BAD'] += 1
    elif '原字节' in e or 'rne-read' in e or '机器码' in e:
        pat['RAWBYTE'] += 1
    elif re.search(r'decomp-[0-9a-f]+\.txt', e):
        pat['DECOMP'] += 1
    elif e.strip() == '' or e.strip() in ('—','-'):
        pat['EMPTY'] += 1
    else:
        pat['OTHER'] += 1
print(pat)
