import json, sys, os, re

def main(path):
    d = json.load(open(path))
    outdir = os.path.dirname(os.path.abspath(path))
    txt = d['results'][0]['result']['content'][0]['text']
    obj = json.loads(txt)
    funcs = obj.get('functions') or {}
    for va, f in funcs.items():
        name = f.get('name','?')
        safe = re.sub(r'[^A-Za-z0-9_.:-]','_',name)
        fn = os.path.join(outdir, f"{va}__{safe}.c")
        with open(fn,'w') as fh:
            fh.write("// "+name+"  size="+str(f.get('size'))+"\n// "+f.get('signature','')+"\n")
            fh.write(f.get('decompiled_code') or '')
            fh.write("\n\n// CALLEES: "+str(f.get('callees')))
        print(va, f.get('size'), name)
    print('count', len(funcs))
if __name__=='__main__':
    main(sys.argv[1])
