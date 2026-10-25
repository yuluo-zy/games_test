# -*- coding: utf-8 -*-
"""Merge all worker results + main-shard supplement into one per-item verdict file."""
import json, os
from collections import Counter, defaultdict

OUT = r"D:/game/reverse-engineering/tiny-glade/evidence/country-core-deep/round3"

# worker files -> chunk domain mapping
sample = json.load(open(OUT + "/O-sample-round2.json", encoding="utf-8"))
by_addr = {r["addr"]: r for r in sample}

verdicts = {}
for fname in ["w1_pass_builder", "w2_nani_render", "w3_top_nonrender", "w4_wall",
              "w5_collision_terrain", "w6_ui_camera", "w7_history_save",
              "w8_renderloop_roofs", "w9_audio_backend"]:
    d = json.load(open(OUT + "/oa2-results/" + fname + ".json", encoding="utf-8"))
    for x in d:
        addr = x["addr"]
        v = x["verdict"]
        if "不一致" in v:
            norm = "不一致"
        elif "一致" in v:
            norm = "一致"
        elif "证据不足" in v:
            norm = "证据不足"
        else:
            norm = v
        # w6 merged two rows at 0x14003bb80; ensure both rows counted
        rec = by_addr.get(addr)
        verdicts.setdefault(addr, []).append((norm, x["detail"], x["evidence"]))

# terrain-water supplement (main shard)
supp = json.load(open(OUT + "/O-sample-round2-terrain-supp.json", encoding="utf-8"))
for r in supp:
    verdicts.setdefault(r["addr"], []).append(("一致", "主分片复核：PDB 精确符号与行内 fn 吻合 + 机器码形态相符（详见报告正文）", "rne-read/PDB"))

merged = []
for r in sample:
    addr = r["addr"]
    vlist = verdicts.get(addr)
    if not vlist:
        print("MISSING verdict for", addr, r["rel"])
        continue
    # prefer strongest: if any 不一致 -> 不一致; else if 证据不足 -> 证据不足; else 一致
    if any(v[0] == "不一致" for v in vlist):
        norm = "不一致"
    elif any(v[0] == "证据不足" for v in vlist):
        norm = "证据不足"
    else:
        norm = "一致"
    r["verdict"] = norm
    r["detail"] = vlist[0][1]
    merged.append(r)
for r in supp:
    r["verdict"] = "一致"
    r["detail"] = verdicts.get(r["addr"], [["一致", "", ""]])[0][1]
    merged.append(r)

json.dump(merged, open(OUT + "/oa2-merged-verdicts.json", "w", encoding="utf-8"), ensure_ascii=False, indent=1)

# stats
by_dom = defaultdict(Counter)
for r in merged:
    by_dom[r["domain"]][r["verdict"]] += 1
print("total:", len(merged))
for d in sorted(by_dom):
    c = by_dom[d]
    print(f"{d:<16} {sum(c.values()):>4}  一致={c['一致']} 不一致={c.get('不一致',0)} 不足={c.get('证据不足',0)}")
allc = Counter(r["verdict"] for r in merged)
print("ALL:", dict(allc))
