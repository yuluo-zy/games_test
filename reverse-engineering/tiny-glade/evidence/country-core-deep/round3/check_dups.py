import json, sys

sys.path.insert(0, r"D:/game/reverse-engineering/tiny-glade/tools/country-core-audit")
import cca  # noqa: E402

ROOT = r"D:/game/reverse-engineering/tiny-glade"
procs = cca.load_procs()
# duplicates: same start, multiple names
from collections import Counter, defaultdict
starts = Counter()
names_at = defaultdict(list)
for a, (sz, nm) in procs.items():
    starts[a] += 1
    names_at[a].append(nm[:120])

for a in [0x140002370, 0x140949170, 0x14067dd80, 0x14003bb80, 0x14092e6a0, 0x14092e6c0]:
    print(hex(a), "->", starts[a], "entries")
    for nm in names_at[a][:5]:
        print("   ", nm)

# public symbols (S_PUB32)
pub = json.load(open(ROOT + "/evidence/render-architecture/public-symbols.json", encoding="utf-8"))
print("\nS_PUB32 entries at those addresses:")
for r in pub:
    va = int(r["va"], 16) if r.get("va") else None
    if va in [0x140002370, 0x140949170, 0x14067dd80, 0x14003bb80, 0x14092e6a0, 0x14092e6c0]:
        print(hex(va), r["symbol"][:80], "|", (r.get("demangled") or "")[:120])
