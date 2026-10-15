# -*- coding: utf-8 -*-
import io, json
from collections import Counter

OUT = r"D:/game/reverse-engineering/tiny-glade/evidence/country-core-deep/round3"
buf = io.StringIO()
for wname in ["w8_renderloop_roofs"]:
    d = json.load(open(OUT + "/oa2-results/" + wname + ".json", encoding="utf-8"))
    c = Counter(x["verdict"] for x in d)
    buf.write(f"== {wname}: {len(d)} items {dict(c)}\n")
    for x in d:
        v = x["verdict"]
        if v != "一致" or "注" in x.get("detail", "") or "修正" in x.get("detail", ""):
            buf.write(f"  {x['addr']} {v} | {x['detail'][:380]}\n")
io.open(OUT + "/w8-summary.txt", "w", encoding="utf-8").write(buf.getvalue())
print("ok")
