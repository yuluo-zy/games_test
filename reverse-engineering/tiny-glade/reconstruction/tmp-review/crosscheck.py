#!/usr/bin/env python3
# -*- coding: utf-8 -*-
"""Cross-check raw-core.json compiled functions vs ALGORITHMS.md per-function tables vs decomp files."""
import json, os, re, sys, glob

DOMAINS = r"D:/game/reverse-engineering/tiny-glade/evidence/country-core-deep/rhapsody/domains"
SKEL = os.path.join(DOMAINS, "raw-core.json")
ALG = os.path.join(DOMAINS, "raw-core", "ALGORITHMS.md")
DECOMP = os.path.join(DOMAINS, "raw-core", "decomp")

# 1) Parse skeleton
with open(SKEL, "r", encoding="utf-8") as f:
    skel = json.load(f)

from collections import Counter
form_cnt = Counter(e.get("form") for e in skel)
print("=== skeleton forms ===")
for k, v in form_cnt.items():
    print(f"  {k}: {v}")
print("  TOTAL entries:", len(skel))

compiled = [e for e in skel if e.get("form") == "compiled_procedure"]
compiled_va = [e for e in compiled if e.get("va")]
compiled_nova = [e for e in compiled if not e.get("va")]
print(f"\ncompiled_procedure: {len(compiled)} ; with va: {len(compiled_va)} ; without va: {len(compiled_nova)}")

# unique short names
def shortname(name):
    # strip generic params after > maybe; keep crate path module + fn
    n = name
    # remove the <...> generic suffix
    depth = 0
    out = []
    for ch in n:
        if ch == "<":
            depth += 1
            break
        out.append(ch)
    return "".join(out)

# 2) Parse ALGORITHMS.md function tables (section 2.x)
with open(ALG, "r", encoding="utf-8") as f:
    alg_lines = f.readlines()

# Collect function names in the tables: rows start with `| ` and first cell is a name or `impl$...`
table_rows = []
in_table = False
for ln in alg_lines:
    s = ln.strip()
    if s.startswith("|"):
        table_rows.append(s)
    elif s.startswith("## ") or s.startswith("### "):
        pass

# Extract first cell of each table row
alg_first_cells = []
for s in table_rows:
    # skip header/separator rows
    cells = [c.strip() for c in s.split("|")]
    # first cell may be empty (leading |) ; second is content
    if len(cells) >= 3:
        first = cells[1].strip()
        if first and not set(first) <= set("-:"):
            alg_first_cells.append(first)

print(f"\nALGORITHMS table rows (non-header): {len(alg_first_cells)}")

# Build set of first cells for matching (strip generic suffix too)
alg_cells_short = set()
for c in alg_first_cells:
    c2 = c.replace("`", "").replace("**", "")
    alg_cells_short.add(c2)

# 3) decomp files: size classification
decomp_files = sorted(glob.glob(os.path.join(DECOMP, "decomp-*.txt")))
notfound = []
real = []
for p in decomp_files:
    sz = os.path.getsize(p)
    with open(p, "r", encoding="utf-8", errors="replace") as f:
        head = f.read(200)
    if sz <= 100 or ("error" in head.lower() and "not found" in head.lower()):
        notfound.append((os.path.basename(p), sz))
    else:
        real.append((os.path.basename(p), sz))
print(f"\ndecomp files total: {len(decomp_files)} ; NotFound-ish: {len(notfound)} ; real: {len(real)}")

nf_va = set()
for b, sz in notfound:
    m = re.search(r"decomp-(0x[0-9a-f]+)\.txt", b)
    if m:
        nf_va.add(m.group(1).lower())
real_va = set()
for b, sz in real:
    m = re.search(r"decomp-(0x[0-9a-f]+)\.txt", b)
    if m:
        real_va.add(m.group(1).lower())

# 4) For each compiled function with va, check decomp presence + Notfound
print("\n=== compiled functions WITH va: decomp status ===")
print("name | va | decomp_status")
missing_decomp = []
for e in compiled_va:
    va = e["va"].lower()
    name = e["name"]
    if va in nf_va:
        st = "NotFound-file"
    elif va in real_va:
        st = "real"
    else:
        st = "NO-DECOMP-FILE"
        missing_decomp.append((name, e["va"]))
    print(f"  {name} | {e['va']} | {st}")

# 5) Cross-check: compiled names vs ALGORITHMS table first cells
print("\n=== compiled functions possibly NOT covered by ALGORITHMS table ===")
# ALGORITHMS uses short names (function name after ::), sometimes with impl$N names
def last_seg(name):
    # take everything after last "::"
    return name.rsplit("::", 1)[-1]

alg_last = set()
for c in alg_first_cells:
    c2 = c.strip("`* ")
    if "::" in c2:
        alg_last.add(c2.rsplit("::", 1)[-1])
    else:
        alg_last.add(c2)

uncovered = []
for e in compiled:
    n = e["name"]
    ls = n.rsplit("::", 1)[-1]
    # generic suffix stripped
    ls_short = ls.split("<")[0] if "<" in ls else ls
    # check match against alg cells (by last segment, and by full short name)
    matched = False
    if ls_short in alg_last:
        matched = True
    # also try matching the module path of function
    if not matched:
        for c in alg_last:
            if c == ls_short:
                matched = True
                break
    if not matched:
        uncovered.append((n, e.get("va"), e.get("form")))

print(f"\nTotal compiled: {len(compiled)}; possibly uncovered: {len(uncovered)}")
for n, va, fm in uncovered:
    print(f"  [{fm}] va={va} {n}")

# 6) dump compiled entries without va that look business-significant
print("\n=== compiled WITHOUT va (no address) ===")
for e in compiled_nova:
    print(f"  {e['name']}")

# 7) write machine-readable output
out = {
    "compiled_total": len(compiled),
    "compiled_with_va": len(compiled_va),
    "compiled_without_va": len(compiled_nova),
    "decomp_total": len(decomp_files),
    "decomp_notfound": len(notfound),
    "decomp_real": len(real),
    "missing_decomp_for_va": missing_decomp,
    "uncovered_compiled": [n for n, va, fm in uncovered],
}
with open(os.path.join(os.path.dirname(__file__), "crosscheck-out.json"), "w", encoding="utf-8") as f:
    json.dump(out, f, ensure_ascii=False, indent=1)
print("\nWrote crosscheck-out.json")
