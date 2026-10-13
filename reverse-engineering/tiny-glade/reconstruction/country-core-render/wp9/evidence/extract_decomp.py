#!/usr/bin/env python3
"""Extract decompiled_code from a ghidra result json into .c files.
Usage: extract_decomp.py <result.json> <outdir>
"""
import json, sys, os

def main():
    result_path, outdir = sys.argv[1], sys.argv[2]
    d = json.load(open(result_path))
    os.makedirs(outdir, exist_ok=True)
    n = 0
    for r in d.get('results', []):
        if r.get('isError'):
            print("ERR:", r.get('tool'), str(r.get('error'))[:200])
            continue
        content = r['result']['content']
        for block in content:
            if block.get('type') == 'text':
                txt = block.get('text', '')
                try:
                    payload = json.loads(txt)
                except Exception:
                    print("non-json text block:", txt[:200])
                    continue
                funcs = payload.get('functions', {})
                for addr, info in funcs.items():
                    name = (info.get('name') or addr).replace('::', '__')
                    code = info.get('decompiled_code', '')
                    sig = info.get('signature', '')
                    fn = os.path.join(outdir, f"{addr}-{name}.c")
                    with open(fn, 'w') as f:
                        f.write(f"// signature: {sig}\n")
                        f.write(code)
                    n += 1
                    print(f"wrote {fn} ({len(code)} bytes)")
    print("total", n)

if __name__ == '__main__':
    main()
