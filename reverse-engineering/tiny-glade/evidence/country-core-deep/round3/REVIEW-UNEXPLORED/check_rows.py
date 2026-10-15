import json, sys
rows = json.load(open('round3/REVIEW-UNEXPLORED/_rows.json', encoding='utf-8'))
targets = set(sys.argv[1:])
for r in rows:
    f = r['file'].replace('\\', '/')
    for t in targets:
        if t in f:
            print(f.split('chapters/')[-1], '|', r['name'], '|', r['addr'], '|', r['concl'])
            break
