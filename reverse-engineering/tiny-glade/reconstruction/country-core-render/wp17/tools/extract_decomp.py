#!/usr/bin/env python3
"""WP17 — extract decompiled_code from a Ghidra batch result into evidence/*.c"""
import json, sys, os, re

def main():
    result_path = sys.argv[1]
    outdir = sys.argv[2] if len(sys.argv) > 2 else "evidence"
    os.makedirs(outdir, exist_ok=True)
    r = json.load(open(result_path))
    for step in r.get("results", []):
        if step.get("tool") != "get_functions":
            continue
        res = step.get("result", {})
        for c in res.get("content", []):
            if c.get("type") != "text":
                continue
            data = json.loads(c["text"])
            for addr, info in data.get("functions", {}).items():
                code = info.get("decompiled_code")
                if not code:
                    print("NO CODE", addr, info.get("name"))
                    continue
                safe = addr.lower()
                path = os.path.join(outdir, f"{safe}.c")
                with open(path, "w") as f:
                    f.write(f"// {addr} {info.get('name','')}\n")
                    f.write(code)
                print(path, len(code))

if __name__ == "__main__":
    main()
