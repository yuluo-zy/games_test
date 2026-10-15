import json, os
rep = json.load(open('gap-out.json', encoding='utf-8'))
n = 0
for s in rep.get('results', []):
    if s.get('tool') != 'decompile_function':
        continue
    try:
        t = json.loads(s['result']['content'][0]['text'])
    except Exception:
        continue
    if 'decompiled_code' in t:
        va = t['address']
        with open('decomp/decomp-%s.txt' % va, 'w', encoding='utf-8') as f:
            f.write(t['decompiled_code'])
        n += 1
    else:
        print('ERR/NotFound:', s['args'].get('address'), t.get('error', ''))
print('extracted', n, 'decompiles')
