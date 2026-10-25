# -*- coding: utf-8 -*-
"""terrain-water supplement sampling (seed 20261015, excluding round1+round2 checked addresses)."""
import json, os, random, re

ROOT = r"D:/game/reverse-engineering/tiny-glade"
CH = os.path.join(ROOT, "evidence/country-core-deep/chapters")
OUT = os.path.join(ROOT, "evidence/country-core-deep/round3")

DOMAINS = {
    "terrain-water": ["terrain", "water", "world_raster", "raster_stroke", "garden", "in_progress_curve",
                      "drawing_input_smoother", "brush_sizes", "cursor_terrain_raycast", "mouse_terrain_raycast",
                      "read_mouse_depth_value", "mesh_follow_terrain", "rocky_terrain", "glade_border",
                      "garden_mask", "water_mask"],
}


def domain_of(f):
    f = f.replace("\\", "/")
    for d, pats in DOMAINS.items():
        if any(p in f for p in pats):
            return d
    return None


rows = []
for dirpath, _d, fns in os.walk(CH):
    for fn in fns:
        if not fn.endswith(".md"):
            continue
        rel = os.path.relpath(os.path.join(dirpath, fn), CH)[:-3].replace("\\", "/")
        if domain_of(rel) != "terrain-water":
            continue
        for line in open(os.path.join(dirpath, fn), encoding="utf-8", errors="replace"):
            if not line.startswith("|") or line.startswith("|---"):
                continue
            cells = [c.strip() for c in line.strip("|").split("|")]
            if len(cells) < 6 or not cells[0].startswith("`"):
                continue
            concl = cells[4]
            if concl.startswith("**"):
                if not re.match(r"^\*{2}O\*{2}", concl):
                    continue
                claim, fmt = cells[3], "B"
            elif concl.startswith("O"):
                claim, fmt = concl, "A"
            else:
                continue
            addr = cells[1]
            if not re.match(r"^0x[0-9a-fA-F]{8,16}$", addr):
                continue
            rows.append({"domain": "terrain-water", "rel": rel, "fn": cells[0], "addr": addr,
                         "claim": claim, "evid": cells[5], "fmt": fmt})

# exclusions
r1 = json.load(open(OUT + "/O-sample-42.json", encoding="utf-8"))
r2 = json.load(open(OUT + "/O-sample-round2.json", encoding="utf-8"))
excl = set()
for e in r1:
    excl.add(e["addr"])
for e in r2:
    if e["domain"] == "terrain-water":
        excl.add(e["addr"])
pool = [r for r in rows if r["addr"] not in excl]
print("terrain-water pool:", len(rows), "excl:", len(excl), "remaining:", len(pool))

rng = random.Random(20261015)
supp = rng.sample(pool, min(10, len(pool)))
json.dump(supp, open(OUT + "/O-sample-round2-terrain-supp.json", "w", encoding="utf-8"),
          ensure_ascii=False, indent=1)
for r in supp:
    print(r["rel"], "|", r["addr"], "|", r["claim"][:70])
