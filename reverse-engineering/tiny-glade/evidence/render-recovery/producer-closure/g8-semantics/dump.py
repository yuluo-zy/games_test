import json, sys, os, re
rep = sys.argv[1]
r = json.load(open(rep))
outdir = os.path.join(os.path.dirname(rep), 'decomp')
os.makedirs(outdir, exist_ok=True)
idx = {}
for res in r.get('results', []):
    res_result = res.get('result') or {}
    sc = res_result.get('structuredContent') or {}
    code = sc.get('decompiled_code') or res_result.get('decompiled_code')
    if not code:
        continue
    addr = (res.get('args') or {}).get('address') or res_result.get('address') or 'unknown'
    fn = os.path.join(outdir, addr + '.c')
    open(fn, 'w').write(code)
    idx[addr] = fn
    print('=====', addr, '=====')
    print(code)
    print()
json.dump(idx, open(os.path.join(outdir, 'index.json'), 'w'), indent=2)
