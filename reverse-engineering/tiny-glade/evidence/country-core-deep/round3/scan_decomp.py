import json, os

ROOT = r"D:/game/reverse-engineering/tiny-glade/evidence/country-core-deep"
OUT = ROOT + "/round3"
sample = json.load(open(OUT + "/O-sample-round2.json", encoding="utf-8"))

# index all decomp files under evidence/country-core-deep
decomp_index = {}
for dirpath, _d, fns in os.walk(ROOT):
    for fn in fns:
        if not fn.endswith(".txt"):
            continue
        low = fn.lower()
        if "decomp" in low or low.startswith("r2-") or low.startswith("rrc"):
            full = os.path.join(dirpath, fn)
            try:
                size = os.path.getsize(full)
            except OSError:
                size = 0
            decomp_index.setdefault(fn, []).append((full, size))

report = {}
for r in sample:
    va = r["addr"]
    found = []
    for fn, hits in decomp_index.items():
        if fn == f"decomp-{va}.txt" or fn.startswith(f"decomp-{va}-"):
            for full, size in hits:
                found.append((full, size))
    # also check evid column paths
    evid_hits = []
    for ev in r["evid"].split(","):
        ev = ev.strip()
        if ev.endswith(".txt"):
            p = os.path.join(ROOT, ev.replace("\\", "/"))
            if os.path.exists(p):
                evid_hits.append((p, os.path.getsize(p)))
    report[va] = {"decomp_files": sorted(set(found)), "evid_txt": evid_hits}

json.dump(report, open(OUT + "/oa2-decomp-index.json", "w", encoding="utf-8"), ensure_ascii=False, indent=1)

missing = {va: v for va, v in report.items() if not v["decomp_files"]}
print("rows without any decomp-<va>.txt file:", len(missing))
for va in sorted(missing):
    r = next(x for x in sample if x["addr"] == va)
    print(f"  {va} | {r['rel'][:60]} | {r['evid'][:80]}")
