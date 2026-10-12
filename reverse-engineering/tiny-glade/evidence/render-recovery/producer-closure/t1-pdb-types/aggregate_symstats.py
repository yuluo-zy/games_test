"""Aggregate llvm-pdbutil --sym-stats per-module symbol-kind counts.

Answers Q5 (which symbol-record kinds exist in module streams at all): S_LOCAL /
S_REGREL32 / S_BPREL32 require full debug info and are absent under
-C debuginfo=line-tables-only.
Usage: python -I aggregate_symstats.py symbol_stats.txt
"""
import re, sys, json
from collections import Counter
tot = Counter(); mods = 0
pat = re.compile(r"^\s+(S_\w+|LF_\w+):\s+(\d+) entries")
for line in open(sys.argv[1], encoding="utf-8", errors="replace"):
    if line.startswith("Mod "): mods += 1
    m = pat.match(line)
    if m: tot[m.group(1)] += int(m.group(2))
print(json.dumps({"modules": mods, "symbol_kinds_total": sum(tot.values()),
                  "kinds": dict(tot.most_common()), "kinds_alpha": dict(sorted(tot.items()))}, indent=2))
