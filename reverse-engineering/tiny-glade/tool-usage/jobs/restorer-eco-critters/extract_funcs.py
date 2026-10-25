#!/usr/bin/env python3
"""Extract per-function decompiled code from a reverse-operator result file."""
import json, sys, os

result_path = sys.argv[1]
outdir = sys.argv[2] if len(sys.argv) > 2 else os.path.dirname(result_path)
os.makedirs(outdir, exist_ok=True)

with open(result_path) as f:
    r = json.load(f)
for step in r.get('results', []):
    if step.get('tool') != 'get_functions':
        continue
    res = step.get('result', {})
    if res.get('isError'):
        print('step isError, skipping')
        continue
    content = res.get('content', [])
    text = content[0]['text'] if isinstance(content, list) and content else content
    data = json.loads(text) if isinstance(text, str) else text
    funcs = data.get('functions', {})
    if isinstance(funcs, list):
        funcs = {f.get('address'): f for f in funcs}
    for addr, f in funcs.items():
        name = f.get('name') or addr
        safe = addr.replace('0x', '')
        code = f.get('decompiled_code', '')
        sig = f.get('signature', '')
        out = os.path.join(outdir, f'{safe}.c')
        with open(out, 'w') as fo:
            fo.write(f'// {addr} {name}\n// SIG: {sig}\n')
            fo.write(code)
        print(f'{safe} {name} -> {out} ({len(code)} chars)')
