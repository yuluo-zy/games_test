import json,sys
p=sys.argv[1]
d=json.load(open(p,encoding='utf-8'))
rs=d.get('results',[])
print("total results:",len(rs))
for i,r in enumerate(rs):
    tool=r.get('tool') or r.get('name')
    args=r.get('args') or {}
    addr=args.get('address','')
    ok=r.get('ok')
    res=r.get('result') or {}
    sc=res.get('structuredContent') if isinstance(res,dict) else None
    keys=list(sc.keys()) if isinstance(sc,dict) else (list(res.keys()) if isinstance(res,dict) else None)
    print(f"[{i}] tool={tool} addr={addr} ok={ok} reskeys={keys}")
