#!/usr/bin/env python3
"""Extract decompiled_code from a ghidra operator result into a readable text file."""
import json
import sys


def main():
    result_path, out_path = sys.argv[1], sys.argv[2]
    r = json.load(open(result_path))
    funcs = {}
    for step in r.get("results", []):
        content = step.get("result", {}).get("content", [])
        for c in content:
            if c.get("type") == "text":
                data = json.loads(c["text"])
                for addr, entry in data.get("functions", {}).items():
                    if isinstance(entry, dict) and "decompiled_code" in entry:
                        funcs[addr] = entry
    lines = []
    for addr in sorted(funcs, key=lambda a: int(a, 16)):
        e = funcs[addr]
        lines.append(f"================ ADDR {addr} ================")
        lines.append("SIGNATURE: " + str(e.get("signature", "")))
        lines.append(e.get("decompiled_code", ""))
        lines.append("")
    open(out_path, "w").write("\n".join(lines))
    print(f"extracted {len(funcs)} funcs -> {out_path}")


if __name__ == "__main__":
    main()
