#!/usr/bin/env python3
"""Dump decompiled code per function from a Ghidra result JSON (read-only)."""
import json, sys

def dump(path):
    d = json.load(open(path))
    print("=" * 80)
    print("RESULT FILE:", path)
    print("=" * 80)
    for step in d.get("results", []):
        r = step.get("result", {})
        if r.get("isError"):
            print("STEP ERROR:", r.get("error", r))
            continue
        content = r.get("content", [])
        for block in content:
            if block.get("type") == "text":
                txt = block.get("text", "")
                if txt.strip():
                    print(txt)
            elif block.get("type") == "json":
                print(json.dumps(block.get("data", {}), ensure_ascii=False, indent=1))
        print("-" * 80)

if __name__ == "__main__":
    for p in sys.argv[1:]:
        dump(p)
