"""Generate a RE-MCP decompile job for a module subset of sysres-procedures.json."""
import json, sys, os
from collections import OrderedDict

idx = json.load(open(r"D:/game/reverse-engineering/tiny-glade/evidence/country-core-deep/sysres-procedures.json", encoding="utf-8"))
sub = sys.argv[1]          # module substring, e.g. "resources/history"
out_job = sys.argv[2]
extra = json.loads(sys.argv[3]) if len(sys.argv) > 3 else []  # extra (address, note) pairs

jobs = []
seen = set()
for f in sorted(idx):
    if sub not in f: continue
    for p in idx[f]["procedures"]:
        va = p["va"]
        if va in seen: continue
        seen.add(va)
        jobs.append({"tool": "decompile_function",
                     "args": {"database": "DBID", "address": va},
                     "timeout_ms": 180000, "_note": f"{f} :: {p['name'][:90]}"})
for addr, note in extra:
    if addr in seen: continue
    seen.add(addr)
    jobs.append({"tool": "decompile_function",
                 "args": {"database": "DBID", "address": addr},
                 "timeout_ms": 180000, "_note": note})
print(f"{len(jobs)} decompile steps")
json.dump(jobs, open(out_job, "w"), ensure_ascii=False, indent=1)
