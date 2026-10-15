import json, sys, os
outdir="decomp"; os.makedirs(outdir, exist_ok=True)
for i in range(1,4):
    fn=f"result{i}.json"
    if not os.path.exists(fn): continue
    d=json.load(open(fn))
    for r in d.get('results',[]):
        res=r.get('result',{})
        for blk in res.get('content',[]):
            txt=blk.get('text','')
            try: obj=json.loads(txt)
            except Exception: continue
            fns=obj.get('functions') if isinstance(obj,dict) else None
            if not fns: continue
            for addr,info in fns.items():
                name=info.get('name','unknown')
                code=info.get('decompiled_code','') or ''
                sig=info.get('signature','')
                safe="".join(ch if ch.isalnum() or ch in "_" else "_" for ch in name)
                with open(f"{outdir}/{addr}_{safe}.c","w") as f:
                    f.write(f"// name={name}\n// sig={sig}\n// size={info.get('size')}\n{code}")
                print(addr, name, "lines", code.count(chr(10)))
