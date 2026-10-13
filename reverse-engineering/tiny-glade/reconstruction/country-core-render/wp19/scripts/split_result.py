#!/usr/bin/env python3
"""Split the WP19 ghidra result into per-function .c files.
Result shape: results[].result.content[].text = JSON {"program","functions":{addr:{...}}}
Output dir: wp19/analysis/decomp/<name>_<addr>.c
"""
import json, os, re, sys

res_path = sys.argv[1] if len(sys.argv) > 1 else "reconstruction/country-core-render/wp19/scripts/result-wp19.json"
outdir = sys.argv[2] if len(sys.argv) > 2 else "reconstruction/country-core-render/wp19/analysis/decomp"
os.makedirs(outdir, exist_ok=True)

d = json.load(open(res_path))
nfiles = 0
for step in d["results"]:
    r = step.get("result") or {}
    if r.get("isError"):
        print("STEP ERROR:", json.dumps(r)[:300])
        continue
    for block in r.get("content", []):
        if not isinstance(block, dict):
            continue
        txt = block.get("text", "")
        if not txt:
            continue
        try:
            parsed = json.loads(txt)
        except Exception:
            continue
        funcs = parsed.get("functions")
        if not isinstance(funcs, dict):
            continue
        for addr, f in funcs.items():
            name = f.get("name") or addr
            code = f.get("decompiled_code") or ""
            sig = f.get("signature") or ""
            callees = f.get("callees")
            nm = re.sub(r'[^\w.]', '_', str(name))[:80] + "_" + str(addr).replace("0x", "")
            path = os.path.join(outdir, nm + ".c")
            with open(path, "w") as out:
                out.write(f"// {name}  {addr}\n")
                if sig:
                    out.write(f"// signature: {sig}\n")
                if callees:
                    out.write(f"// callees: {json.dumps(callees)}\n")
                out.write("\n" + code)
            nfiles += 1
print("wrote", nfiles, "files to", outdir)
