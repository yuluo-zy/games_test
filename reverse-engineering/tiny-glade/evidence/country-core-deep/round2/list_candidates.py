# -*- coding: utf-8 -*-
import re, os, sys, json
sys.path.insert(0, os.path.dirname(os.path.abspath(__file__)))
from fill_chapters import KB, name_classify, DOMAIN_PREFIXES, DOMAIN_FILES, BASE

cands = []
for root, dirs, fs in os.walk(BASE):
    for f in fs:
        if not f.endswith('.md'):
            continue
        rel = os.path.relpath(os.path.join(root, f), BASE).replace('\\', '/')
        if not (any(rel.startswith(p) for p in DOMAIN_PREFIXES) or rel in DOMAIN_FILES):
            continue
        txt = open(os.path.join(root, f), encoding='utf-8').read()
        for full, name, va, role, concl, ev in re.findall(
                r'(\| `([^`]+)` \| (0x[0-9a-f]+) \| \d+ \| ([^|]*) \| ([^|]*) \| ([^|]*)\|)', txt):
            hit = any(kva.lower().lstrip('0x').rstrip('l') == va.lstrip('0x').lower()
                      for kva in KB)
            if not hit and not name_classify(name):
                cands.append((rel, va, name))
print('targeted-decompile candidates:', len(cands))
for rel, va, name in cands:
    print(va, rel, name[:90])
