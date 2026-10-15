#!/usr/bin/env python3
"""Resolve FUN_ addresses from decompiled .c files to PDB demangled names.

Inputs:
  - evidence/render-architecture/public-symbols.json (S_PUB32, demangled + va)
  - decompiled .c files listed in ARGS
Outputs JSON: per file, sorted call list [addr, count, name].
"""
import json, re, sys, collections

PUB = r"D:/game/reverse-engineering/tiny-glade/evidence/render-architecture/public-symbols.json"

def build_map():
    d = json.load(open(PUB))
    m = {}
    for e in d:
        va = e.get("va")
        if va and e.get("demangled"):
            m[va] = e["demangled"]
    return m

def short(n):
    # strip hash suffix for readability
    n = re.sub(r"::h[0-9a-f]{16}$", "", n)
    return n

def main(files):
    m = build_map()
    out = {}
    for f in files:
        try:
            text = open(f, encoding="utf-8", errors="replace").read()
        except OSError as e:
            print(f"skip {f}: {e}", file=sys.stderr)
            continue
        counts = collections.Counter(re.findall(r"FUN_([0-9a-f]+)", text))
        rows = []
        for addr, c in counts.most_common():
            # Ghidra writes FUN_140<addr-hex>; captured starts with "140" prefix.
            if addr.startswith("140"):
                addr = addr[3:]
            va = "0x140" + addr
            name = m.get(va, "?")
            rows.append({"addr": va, "count": c, "name": short(name) if name != "?" else "?"})
        out[f] = rows
        print(f"== {f}: {len(rows)} unique calls")
        for r in rows[:60]:
            print(f"  {r['addr']} x{r['count']}  {r['name']}")
    json.dump(out, open(r"D:/game/reverse-engineering/tiny-glade/evidence/country-core-deep/rhapsody/resolved-calls.json", "w"),
              ensure_ascii=False, indent=1)

if __name__ == "__main__":
    main(sys.argv[1:])
