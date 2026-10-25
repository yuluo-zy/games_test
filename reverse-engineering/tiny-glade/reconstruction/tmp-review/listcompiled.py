#!/usr/bin/env python3
# -*- coding: utf-8 -*-
"""List all compiled_procedure functions grouped by source file, with va, and identify ALG table rows
that mention each short name (grouped coverage)."""
import json, os, re

DOMAINS = r"D:/game/reverse-engineering/tiny-glade/evidence/country-core-deep/rhapsody/domains"
SKEL = os.path.join(DOMAINS, "raw-core.json")
ALG = os.path.join(DOMAINS, "raw-core", "ALGORITHMS.md")

with open(SKEL, "r", encoding="utf-8") as f:
    skel = json.load(f)

compiled = [e for e in skel if e.get("form") == "compiled_procedure"]

# ALG table rows
with open(ALG, "r", encoding="utf-8") as f:
    lines = f.readlines()
rows = []
for ln in lines:
    s = ln.strip()
    if s.startswith("|") and len([c for c in s.split("|")]) >= 4:
        cells = [c.strip() for c in s.split("|")]
        first = cells[1].strip().strip("`*")
        if first and set(first) != set("-") and not set(first).issubset(set("-: ")):
            rows.append(first)

def short(fn):
    n = fn
    out = []
    depth = 0
    for ch in n:
        if ch == "<":
            depth += 1
            break
        out.append(ch)
    return "".join(out)

def lastseg(fn):
    base = short(fn)
    return base.rsplit("::", 1)[-1]

def covered(fn):
    """Return matching ALG rows (loose)."""
    base = short(fn)
    ls = lastseg(fn)
    hits = []
    for r in rows:
        r2 = r
        if ls in r2 or base in r2:
            hits.append(r)
    return hits

from collections import OrderedDict
byfile = OrderedDict()
for e in compiled:
    byfile.setdefault(e["file"], []).append(e)

for fname, ents in byfile.items():
    print(f"\n===== {fname}  ({len(ents)}) =====")
    for e in ents:
        va = e.get("va")
        hits = covered(e["name"])
        mark = ""
        if not hits:
            mark = "  <<< NO ALG ROW MATCH"
        print(f"  va={va} {e['name']}{mark}")
        if hits:
            print(f"        -> {hits[:2]}")
