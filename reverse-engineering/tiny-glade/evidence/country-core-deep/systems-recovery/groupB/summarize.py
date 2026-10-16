#!/usr/bin/env python
"""Compact summary of decomp files: signature + key call lines + constants."""
import json, io, re, sys

def summarize(va):
    txt = io.open(f'decomp-{va}.txt', encoding='utf-8').read()
    d = json.loads(txt[txt.index('{'):])
    code = d['decompiled_code']
    out = []
    sig = re.search(r'^(?:undefined8 |void |bool |float |int |char |uint |byte |longlong |undefined4 )?FUN_[\w]+\([^)]*\)', code, re.M)
    if sig:
        out.append('SIG: ' + sig.group(0).replace('\r','').replace('\n',' ')[:200])
    calls = []
    for line in code.splitlines():
        s = line.strip()
        if re.search(r'(FUN_|func_0x)', s):
            calls.append(s[:150])
        elif re.search(r'_UNK_142', s) and ('=' in s or '<' in s or '>' in s):
            calls.append('  C: ' + s[:150])
    # dedupe calls
    seen = set()
    for c in calls:
        key = re.sub(r'\b(lVar|uVar|fVar|auVar|puVar|pbVar|cVar|iVar|uStack|lStack|fStack|auStack|iStack|bStack|pplStack|pcVar|pfVar|plVar|puStack|pfStack|aiStack|piStack|pauStack)\d+', 'V', c)
        if key in seen: continue
        seen.add(key)
        out.append(c)
    return out

for va in sys.argv[1:]:
    print('=' * 8, va, '=' * 8)
    try:
        for l in summarize(va):
            print(l)
    except Exception as e:
        print('ERR', e)
