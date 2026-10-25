# -*- coding: utf-8 -*-
import os, re, sys
sys.path.insert(0, os.path.dirname(os.path.abspath(__file__)))
from fill_chapters import BASE, DOMAIN_PREFIXES, DOMAIN_FILES

names = set()
for root, dirs, fs in os.walk(BASE):
    for f in fs:
        if not f.endswith('.md'):
            continue
        rel = os.path.relpath(os.path.join(root, f), BASE).replace('\\', '/')
        if not (any(rel.startswith(p) for p in DOMAIN_PREFIXES) or rel in DOMAIN_FILES):
            continue
        txt = open(os.path.join(root, f), encoding='utf-8').read()
        for m in re.finditer(r'`([^`]+)`（行 \d+）— 待推断', txt):
            names.add(m.group(1))
for n in sorted(names):
    print(n)
print('TOTAL', len(names))
