import json, re
r = json.load(open('reports/wave3/toolbar/jobs/fresh-sigs.result.json'))
full = ''
for s in r.get('results', []):
    res = s.get('result', {})
    for c in res.get('content', []):
        if isinstance(c, dict) and c.get('type') == 'text':
            full += c['text']

# locate switch_app_mode_button caller block
idx = full.find('"0x142341fe0"')
seg = full[idx:idx+8000]
# print from run_unsafe to the next double-quote after a long way
i = seg.find('run_unsafe')
print(seg[i-20:i+6000])
