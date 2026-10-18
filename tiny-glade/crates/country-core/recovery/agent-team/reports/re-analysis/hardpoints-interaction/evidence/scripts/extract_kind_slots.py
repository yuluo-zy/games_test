#!/usr/bin/env python3
# Extract per-kind mesh slot tables from populate_autoclutter_regions decompile.
import re

path = "/Users/yuluo/Desktop/game/games_test/tiny-glade/crates/country-core/recovery/agent-team/reports/re-analysis/hardpoints-ecstate/evidence/decomp/populate_autoclutter_regions_1422d6ed0.c"
lines = open(path, encoding="utf-8").read().splitlines()

switch_line = None
for i, ln in enumerate(lines):
    if "switch(*(char *)((longlong)local_138 + -7))" in ln:
        switch_line = i
        break
print("switch at line", switch_line)

cur = None
slots = {}
cur_slots = []
in_case = False
for i in range(switch_line + 1, min(switch_line + 2600, len(lines))):
    ln = lines[i]
    m = re.match(r"\s*case (\S+):", ln)
    m2 = re.match(r"\s*default:", ln)
    if m:
        if in_case:
            slots[cur] = cur_slots
        cur = m.group(1)
        cur_slots = []
        in_case = True
        continue
    if m2:
        if in_case:
            slots[cur] = cur_slots
        cur = "default"
        cur_slots = []
        in_case = True
        continue
    if in_case:
        ms = re.search(r"local_100\[(0x[0-9a-fA-F]+|\d+)\]", ln)
        if ms and ("local_" in ln or "ppppp" in ln or "uStack" in ln):
            idx = ms.group(1)
            var = re.search(r"^\s*([A-Za-z_][A-Za-z0-9_]*)\s*=", ln)
            vn = var.group(1) if var else "?"
            cur_slots.append([idx, vn])
if in_case:
    slots[cur] = cur_slots

for k, v in slots.items():
    print(f"case {k}: {len(v)} slots -> {[s[0] for s in v]}")
