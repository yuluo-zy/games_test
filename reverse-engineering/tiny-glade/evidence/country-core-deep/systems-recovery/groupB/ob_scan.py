import json, io, re, sys
def get(va):
    txt = io.open(f'decomp-{va}.txt', encoding='utf-8').read()
    return json.loads(txt[txt.index('{'):])['decompiled_code']
def scan(va):
    code = get(va)
    out = []
    for l in code.splitlines():
        s = l.strip()
        if not s: continue
        if re.search(r'FUN_142|func_0x0001|func_0x0001409e8|FUN_141c4|UNK_143038', s) and ('=' in s or '!=' in s or '<' in s or '>' in s or '(' in s):
            out.append(s[:150])
    # dedupe
    seen = set()
    res = []
    for c in out:
        key = re.sub(r'\b(lVar|uVar|fVar|auVar|puVar|pbVar|cVar|iVar|uStack|lStack|fStack|auStack|iStack|bStack|pplStack|pcVar|pfVar|plVar|puStack|pfStack)\d+', 'V', c)
        if key in seen: continue
        seen.add(key); res.append(c)
    return res
for va in sys.argv[1:]:
    print('='*10, va)
    for l in scan(va): print(l)
