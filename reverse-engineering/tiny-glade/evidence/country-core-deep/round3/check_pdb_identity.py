import json, sys
from collections import Counter

OUT = r"D:/game/reverse-engineering/tiny-glade/evidence/country-core-deep/round3"
rows = json.load(open(OUT + "/oa2-pdb-identity.json", encoding="utf-8"))
sample = json.load(open(OUT + "/O-sample-round2.json", encoding="utf-8"))
byaddr = {r["addr"]: r for r in rows}

mism = []
nopdb = []
for r in sample:
    a = r["addr"]
    p = byaddr[a]
    fn = r["fn"].strip("`").split("::")[-1] if r["fn"] else ""
    has_exact = "pdb_exact" in p
    if has_exact:
        nm = p["pdb_exact"]["name"]
        last = nm.split("::")[-1]
        # simple name-match heuristic
        if fn and fn not in nm and last != fn:
            mism.append((a, r["rel"], fn, nm))
    else:
        nopdb.append((a, r["rel"], r["fn"][:80], p.get("owner", {}).get("name", "—")[:120] if "owner" in p else "—"))

print("== exact-PDB name mismatch candidates ==")
for m in mism:
    print(f"  {m[0]} | {m[1][:50]} | claim fn: {m[2][:50]}")
    print(f"      pdb: {m[3][:120]}")
print(f"\n== no exact PDB proc at addr ({len(nopdb)}) ==")
for n in nopdb:
    print(f"  {n[0]} | {n[1][:50]} | {n[2][:60]}")
    print(f"      owner: {n[3][:120]}")
