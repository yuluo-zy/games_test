import json, re, glob

biz = json.load(open('round3/REVIEW-UNEXPLORED/_biz.json', encoding='utf-8'))
alg = {}
for mod, path in [('asset_libraries','asset-libraries/ALGORITHMS.md'),
                  ('utils','utils/ALGORITHMS.md'),
                  ('startup','startup/ALGORITHMS.md'),
                  ('input','input/ALGORITHMS.md'),
                  ('components','components-geometry/ALGORITHMS.md'),
                  ('geometry','components-geometry/ALGORITHMS.md')]:
    alg[mod] = open(path, encoding='utf-8').read()

def short(fullname):
    n = fullname.split('::')[-1]
    return re.sub(r'<.*>', '', n)

notcovered = []
covered = []
for k, funcs in biz.items():
    mod = k.split('/')[0]
    text = alg.get(mod, '')
    for f in funcs:
        # skip template-monomorph families (ssbo) - covered by family rows
        if mod == 'asset_libraries' and 'ssbo_library' in k:
            continue
        sn = short(f['name'])
        # search both short name and PDB fullname fragments
        if sn in text or f['name'].split('::')[-2][:20] in text:
            covered.append((k, sn, f.get('va') or ['?']))
        else:
            notcovered.append((k, f['name'], (f.get('va') or ['?'])[0]))

print('ALGORITHMS covered:', len(covered), 'not covered:', len(notcovered))
for k, name, va in notcovered:
    print('  %s | %s | %s' % (k, name, va))
