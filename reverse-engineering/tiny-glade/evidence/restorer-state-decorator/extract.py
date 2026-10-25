import json, sys, pathlib
def extract(result_path, outdir):
    outdir = pathlib.Path(outdir); outdir.mkdir(parents=True, exist_ok=True)
    r = json.load(open(result_path))
    for step in r.get('results', []):
        res = step.get('result', {})
        for c in res.get('content', []):
            if c.get('type') == 'text':
                try: data = json.loads(c['text'])
                except Exception: continue
                for addr, fn in (data.get('functions') or {}).items():
                    name = (fn or {}).get('name', addr)
                    code = (fn or {}).get('decompiled_code', '')
                    safe = name.replace('::','__').replace('<','_').replace('>','_').replace('/','_')
                    (outdir / f'{addr}_{safe}.c').write_text(code)
                    print(f'{addr}\t{name}\t{len(code)}B')
if __name__ == '__main__':
    extract(sys.argv[1], sys.argv[2])
