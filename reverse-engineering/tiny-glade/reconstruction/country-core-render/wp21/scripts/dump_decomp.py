#!/usr/bin/env python3
"""Dump decompiled_code from a Ghidra result.json to a .txt file.
Usage: dump_decomp.py <result.json> <out.txt>
"""
import json, sys

def main():
    res_path, out_path = sys.argv[1], sys.argv[2]
    d = json.load(open(res_path))
    if not d.get('ok'):
        print('report not ok:', d.get('access')); return
    for r in d.get('results', []):
        res = r.get('result')
        if not isinstance(res, dict): continue
        for c in res.get('content', []):
            txt = c.get('text', '')
            if not txt: continue
            try:
                data = json.loads(txt)
            except Exception:
                continue
            funcs = data.get('functions', {})
            out = []
            for addr, f in funcs.items():
                out.append(f'// ===== {addr} {f.get("name","")} | sig: {f.get("signature","")} | size: {f.get("size")} =====')
                out.append(f.get('decompiled_code',''))
                out.append('')
            open(out_path, 'w').write('\n'.join(out))
            print(f'{out_path}: {len(funcs)} functions')

if __name__ == '__main__':
    main()
