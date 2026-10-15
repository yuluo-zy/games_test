# -*- coding: utf-8 -*-
import bisect, json, sys
sys.path.insert(0, r"D:/game/reverse-engineering/tiny-glade/tools/country-core-audit")
import cca

ROOT = r"D:/game/reverse-engineering/tiny-glade"
OUT = ROOT + "/evidence/country-core-deep/round3"
supp = json.load(open(OUT + "/O-sample-round2-terrain-supp.json", encoding="utf-8"))
procs = cca.load_procs()
starts = sorted(procs)

pub = json.load(open(ROOT + "/evidence/render-architecture/public-symbols.json", encoding="utf-8"))
pubd = {}
for r in pub:
    va = int(r["va"], 16) if r.get("va") else None
    if va:
        pubd.setdefault(va, []).append(r["symbol"])

for r in supp:
    a = int(r["addr"], 16)
    exact = procs.get(a)
    j = bisect.bisect_right(starts, a) - 1
    owner = None
    if j >= 0 and a < starts[j] + procs[starts[j]][0]:
        s0 = starts[j]
        owner = (hex(s0), procs[s0][0], procs[s0][1][:130], a - s0)
    pubs = pubd.get(a, [])
    print(r["addr"], "|", r["rel"][:45])
    print("   exact:", (exact[1][:130] if exact else "—"))
    if owner:
        print("   owner:", owner[1], "|", owner[2], "+%#x" % owner[3])
    for p in pubs[:3]:
        print("   pub  :", p[:120])
