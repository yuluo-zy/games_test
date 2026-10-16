#!/usr/bin/env python
"""Extract per-function decomp text from a RE-MCP batch report into decomp-<va>.txt"""
import json, io, os, sys, re

REPORT = sys.argv[1]
OUTDIR = sys.argv[2]
os.makedirs(OUTDIR, exist_ok=True)

with io.open(REPORT, encoding="utf-8") as f:
    rep = json.load(f)

n_ok = n_err = 0
for r in rep["results"]:
    if r["tool"] != "decompile_function":
        continue
    args, result = r["args"], r["result"]
    va = args["address"].lower()
    if result.get("isError"):
        n_err += 1
        with io.open(os.path.join(OUTDIR, f"ERR-{va}.txt"), "w", encoding="utf-8") as f:
            f.write(json.dumps(result, ensure_ascii=False))
        continue
    text = ""
    for c in result.get("content", []):
        text += c.get("text", "")
    note = args.get("_note", "")
    with io.open(os.path.join(OUTDIR, f"decomp-{va}.txt"), "w", encoding="utf-8") as f:
        f.write(f"// {note}\n")
        f.write(text)
    n_ok += 1
print(f"ok={n_ok} err={n_err} total={n_ok+n_err}")
