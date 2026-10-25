#!/usr/bin/env python3
# Extract per-case write blocks (definitive slot+weight order) from populate decompile.
import re

path = "/Users/yuluo/Desktop/game/games_test/tiny-glade/crates/country-core/recovery/agent-team/reports/re-analysis/hardpoints-ecstate/evidence/decomp/populate_autoclutter_regions_1422d6ed0.c"
lines = open(path, encoding="utf-8").read().splitlines()

switch_line = None
for i, ln in enumerate(lines):
    if "switch(*(char *)((longlong)local_138 + -7))" in ln:
        switch_line = i
        break

# Parse: cases -> list of (var, index) slot assignments; then write block lines.
cases = {}   # case -> {"slots": [(var, idx)], "writes": [line]}
cur = None
state = None
for i in range(switch_line, min(switch_line + 2600, len(lines))):
    ln = lines[i]
    m = re.match(r"\s*case (\S+):", ln)
    if m:
        cur = m.group(1)
        cases.setdefault(cur, {"slots": [], "writes": []})
        state = "slots"
        continue
    if re.match(r"\s*default:", ln):
        cur = "default"
        cases.setdefault(cur, {"slots": [], "writes": []})
        state = "slots"
        continue
    if cur is None:
        continue
    ms = re.search(r"local_100\[(0x[0-9a-fA-F]+|\d+)\]", ln)
    if state == "slots":
        if ms:
            var = re.search(r"^\s*([A-Za-z_][A-Za-z0-9_]*)\s*=", ln)
            cases[cur]["slots"].append((var.group(1), ms.group(1)))
        elif "goto LAB_1422d7d2a" in ln:
            state = "writes"
            continue
    if state == "writes" and "local_188 +" in ln and "lVar27" in ln:
        cases[cur]["writes"].append(ln.strip())

# Decode hex weight constants.
def weight_of(var, writes):
    """Find the weight written for a given slot var in the writes."""
    # weights are the second write in each pair; find var name references
    return None

for k, v in cases.items():
    print(f"===== case {k} ({len(v['slots'])} slots, {len(v['writes'])} write lines) =====")
    for w in v["writes"]:
        print("   ", w)
    print()
