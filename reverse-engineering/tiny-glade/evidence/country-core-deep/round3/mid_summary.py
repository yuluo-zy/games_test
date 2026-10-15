# -*- coding: utf-8 -*-
import io, json

OUT = r"D:/game/reverse-engineering/tiny-glade/evidence/country-core-deep/round3"
buf = io.StringIO()
for wname in ["w1_pass_builder", "w3_top_nonrender", "w5_collision_terrain", "w7_history_save", "w8_renderloop_roofs"]:
    import os
    p = OUT + "/oa2-results/" + wname + ".json"
    if not os.path.exists(p):
        buf.write(f"== {wname}: NOT DONE\n")
        continue
    d = json.load(open(p, encoding="utf-8"))
    from collections import Counter
    c = Counter(x["verdict"] for x in d)
    buf.write(f"== {wname}: {len(d)} items {dict(c)}\n")
    for x in d:
        if x["verdict"] != "一致":
            buf.write(f"  {x['addr']} {x['verdict']} | {x['detail'][:260]}\n")
io.open(OUT + "/w-mid-summary.txt", "w", encoding="utf-8").write(buf.getvalue())
print("written")
