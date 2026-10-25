"""Build PDB identity table for all round2 sample addresses (one shared pass).

For each addr: exact proc start? containing owner proc? S_PUB32 symbol?
Output: round3/oa2-pdb-identity.json
"""
import bisect, json, re, sys
from pathlib import Path

sys.path.insert(0, r"D:/game/reverse-engineering/tiny-glade/tools/country-core-audit")
import cca  # noqa: E402

ROOT = r"D:/game/reverse-engineering/tiny-glade"
OUT = ROOT + "/evidence/country-core-deep/round3"

procs = cca.load_procs()
starts = sorted(procs)

PUB = ROOT + "/evidence/render-architecture/public-symbols.json"


def load_pub():
    d = json.load(open(PUB, encoding="utf-8"))
    out = {}
    for r in d:
        s = r.get("symbol", "")
        m = re.match(r"(\d+) \| S_PUB32 \[size = (\d+)\]", s)
        if m and r.get("va"):
            out.setdefault(int(r["va"], 16), (int(m.group(2)), r.get("demangled", "")))
    return out


pub = load_pub()
pub_starts = sorted(pub)

sample = json.load(open(OUT + "/O-sample-round2.json", encoding="utf-8"))
rows = []
for r in sample:
    a = int(r["addr"], 16)
    rec = {"addr": r["addr"], "domain": r["domain"], "rel": r["rel"], "fn": r["fn"][:160]}
    exact = procs.get(a)
    if exact:
        rec["pdb_exact"] = {"size": exact[0], "name": exact[1][:180]}
    j = bisect.bisect_right(starts, a) - 1
    if j >= 0:
        s0, (sz, nm) = starts[j], procs[starts[j]]
        if a < s0 + sz:
            rec["owner"] = {"start": hex(s0), "size": sz, "name": nm[:180], "off": a - s0}
    pj = bisect.bisect_right(pub_starts, a) - 1
    if pj >= 0:
        s0 = pub_starts[pj]
        if a < s0 + pub[s0][0]:
            rec["pubsym"] = {"start": hex(s0), "size": pub[s0][0], "name": pub[s0][1][:180]}
    rows.append(rec)

json.dump(rows, open(OUT + "/oa2-pdb-identity.json", "w", encoding="utf-8"),
          ensure_ascii=False, indent=1)
print("wrote", len(rows))
