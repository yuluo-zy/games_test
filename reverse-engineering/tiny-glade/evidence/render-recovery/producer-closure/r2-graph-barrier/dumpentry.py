import json,sys
p=sys.argv[1]; idxs=[int(x) for x in sys.argv[2:]]
d=json.load(open(p,encoding='utf-8'))
for i in idxs:
    print("="*70)
    print("ENTRY",i,json.dumps(d['results'][i],ensure_ascii=False)[:3000])
