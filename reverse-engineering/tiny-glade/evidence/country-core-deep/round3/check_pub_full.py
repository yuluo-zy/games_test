import json

pub = json.load(open(r"D:/game/reverse-engineering/tiny-glade/evidence/render-architecture/public-symbols.json", encoding="utf-8"))
targets = {0x14003bb80, 0x140002370, 0x140949170, 0x14067dd80, 0x14092e6a0, 0x14092e6c0}
from collections import defaultdict
by_va = defaultdict(list)
for r in pub:
    va = int(r["va"], 16) if r.get("va") else None
    if va in targets:
        by_va[va].append(r)

for va in sorted(targets):
    print("=" * 10, hex(va), len(by_va[va]), "entries", "=" * 10)
    # full mangled names
    for r in by_va[va]:
        print("   ", r["symbol"])
        dm = r.get("demangled") or ""
        if len(dm) > 110:
            print("      ->", dm[:110])
