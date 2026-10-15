# -*- coding: utf-8 -*-
import json, os

ROOT = r"D:/game/reverse-engineering/tiny-glade/evidence/country-core-deep"
OUT = ROOT + "/round3"
sample = json.load(open(OUT + "/O-sample-round2.json", encoding="utf-8"))
idx = json.load(open(OUT + "/oa2-decomp-index.json", encoding="utf-8"))

bad = {}
for r in sample:
    va = r["addr"]
    files = idx.get(va, {}).get("decomp_files", [])
    for full, size in files:
        try:
            head = open(full, encoding="utf-8", errors="replace").read(300)
        except OSError:
            continue
        if '"error"' in head or "NotFound" in head or "No function" in head:
            bad.setdefault(va, []).append((full, size, head[:120].replace("\n", " ")))

print("rows with NotFound-error decomp files:", len(bad))
for va in sorted(bad):
    r = next(x for x in sample if x["addr"] == va)
    print(va, "|", r["rel"][:55], "|", r["fn"][:60])
    for full, size, head in bad[va]:
        print("    ", os.path.relpath(full, ROOT), size, "|", head)
