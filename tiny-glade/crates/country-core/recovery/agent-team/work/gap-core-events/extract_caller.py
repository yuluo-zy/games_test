import json, re
r = json.load(open('reports/wave3/toolbar/jobs/fresh-sigs.result.json'))
full = ''
for s in r.get('results', []):
    res = s.get('result', {})
    for c in res.get('content', []):
        if isinstance(c, dict) and c.get('type') == 'text':
            full += c['text']

# find the switch_app_mode_button entry
m = re.search(r'"0x142341fe0":\{.*?callers.*?\}', full, re.S)
if m:
    seg = m.group(0)
    mm = re.search(r'run_unsafe<(.*?)>', seg)
    if mm:
        sig = mm.group(1)
        print('CALLER SIG LEN', len(sig))
        print(sig[:5000])
    else:
        print('no run_unsafe; segment head:')
        print(seg[:3000])
else:
    print('no entry found')
