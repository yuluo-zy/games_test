#!/usr/bin/env python3
"""Extract per-function decompiled_code from a ghidra result JSON into files."""
import json, sys, os, re

result_path = sys.argv[1]
outdir = sys.argv[2]
os.makedirs(outdir, exist_ok=True)

r = json.load(open(result_path))
assert r.get("ok"), "job failed: " + json.dumps(r.get("error"))
res = r["results"][0]
assert not res["result"].get("isError"), "step isError"
content = res["result"]["content"]
text = "".join(c["text"] for c in content if c.get("type") == "text")
data = json.loads(text)
funcs = data["functions"]
for addr, info in funcs.items():
    name = info.get("name") or addr
    # sanitize name for filename
    fname = re.sub(r"[^0-9A-Za-z_.]+", "_", name) + f"_{addr}.c"
    body = info.get("decompiled_code") or "(no decompiled_code)"
    sig = info.get("signature") or ""
    with open(os.path.join(outdir, fname), "w") as f:
        f.write(f"// addr={addr} name={name}\n// signature={sig}\n\n{body}\n")
print(f"wrote {len(funcs)} files to {outdir}")
