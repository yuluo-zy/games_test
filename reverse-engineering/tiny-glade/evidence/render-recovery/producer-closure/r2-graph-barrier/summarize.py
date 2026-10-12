import json,sys
d=json.load(open(sys.argv[1],encoding='utf-8'))
os.mkdir if False else None
import os
os.makedirs('decomp',exist_ok=True)
for i,r in enumerate(d['results']):
    t=r.get('tool'); a=r.get('args') or {}
    key=a.get('address') or a.get('filter_pattern')
    sc=r.get('result') if isinstance(r.get('result'),dict) else {}
    code=sc.get('decompiled_code') if isinstance(sc,dict) else None
    items=sc.get('items') if isinstance(sc,dict) else None
    info=sc.get('function_count') if isinstance(sc,dict) else None
    tag=f"n_ins={sc.get('instruction_count')}" if isinstance(sc,dict) and 'instruction_count' in sc else ''
    print(f"[{i}] {t} {key} ok={r.get('ok')} declen={len(code) if code else 0} items={(len(items) if isinstance(items,list) else items)} {tag}")
    if code and key and key.startswith('0x'):
        open(f"decomp/{key}.c",'w',encoding='utf-8').write(code)
    if isinstance(items,list) and items:
        print("   sample:",json.dumps(items[:3],ensure_ascii=False)[:600])
    if isinstance(sc,dict) and not code and not isinstance(items,list):
        print("   sc:",json.dumps(sc,ensure_ascii=False)[:500])
