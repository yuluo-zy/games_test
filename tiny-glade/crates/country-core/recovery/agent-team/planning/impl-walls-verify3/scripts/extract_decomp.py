import json, sys, os
def extract(result_path, outdir):
    d = json.load(open(result_path))
    os.makedirs(outdir, exist_ok=True)
    n = 0
    for r in d.get('results', []):
        if r.get('isError'):
            continue
        text = r['result']['content'][0]['text']
        try:
            payload = json.loads(text)
        except Exception:
            payload = text
        fns = payload.get('functions') if isinstance(payload, dict) else None
        if isinstance(fns, dict):
            for addr, fn in fns.items():
                nm = addr.replace('0x','')
                sig = fn.get('signature','') or fn.get('name','')
                nm2 = ''.join(c for c in str(sig) if c.isalnum() or c in '._-')[:80]
                path = os.path.join(outdir, f"{nm}-{nm2}.json")
                json.dump(fn, open(path,'w'), indent=1)
                n += 1
        elif isinstance(fns, list):
            for fn in fns:
                addr = fn.get('address') or fn.get('entry_point') or '?'
                nm = str(addr).replace('0x','')
                sig = fn.get('signature','') or fn.get('name','')
                nm2 = ''.join(c for c in str(sig) if c.isalnum() or c in '._-')[:80]
                path = os.path.join(outdir, f"{nm}-{nm2}.json")
                json.dump(fn, open(path,'w'), indent=1)
                n += 1
        else:
            json.dump(payload, open(os.path.join(outdir,'RAW.json'),'w'), indent=1)
    print(f"extracted {n} functions to {outdir}")
if __name__ == '__main__':
    extract(sys.argv[1], sys.argv[2])
