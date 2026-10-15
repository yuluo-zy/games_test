import json, os
from collections import Counter

OUT = r"D:/game/reverse-engineering/tiny-glade/evidence/country-core-deep/round3"
s = json.load(open(os.path.join(OUT, "O-sample-round2.json"), encoding="utf-8"))
print("total:", len(s))
print("by domain:", dict(Counter(r["domain"] for r in s)))
print("fmt:", dict(Counter(r.get("fmt", "?") for r in s)))
print("supp:", sum(1 for r in s if "supp" in r))

# ?? composition
q = [r for r in s if r["domain"] == "??"]
c = Counter((r["rel"].split("/")[0] if "/" in r["rel"] else "(top)"))
print("?? by top:", dict(c))

# check for any duplicate (fn,addr) across whole sample
pairs = [(r["domain"], r["addr"]) for r in s]
dup = [p for p, n in Counter(pairs).items() if n > 1]
print("dups:", dup)

# sanity: evid non-empty?
noev = [r for r in s if not r["evid"]]
print("rows without evid:", len(noev))
for r in noev[:10]:
    print("  noev:", r["domain"], r["rel"], r["addr"], r["claim"][:40])
