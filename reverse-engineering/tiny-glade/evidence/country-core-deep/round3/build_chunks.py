"""Build per-worker chunk files for round-2 O-audit verification.

Each chunk item carries: addr, rel, fn, claim, evid, pdb identity (exact/owner),
decomp files available, and a `step` hint (what to check).
"""
import json, os

ROOT = r"D:/game/reverse-engineering/tiny-glade/evidence/country-core-deep"
OUT = ROOT + "/round3"
CHUNK = OUT + "/oa2-chunks"
os.makedirs(CHUNK, exist_ok=True)
os.makedirs(OUT + "/oa2-results", exist_ok=True)

sample = json.load(open(OUT + "/O-sample-round2.json", encoding="utf-8"))
pdb = {r["addr"]: r for r in json.load(open(OUT + "/oa2-pdb-identity.json", encoding="utf-8"))}
dec = json.load(open(OUT + "/oa2-decomp-index.json", encoding="utf-8"))

for r in sample:
    pid = pdb.get(r["addr"], {})
    r["pdb_exact"] = pid.get("pdb_exact")
    r["pdb_owner"] = pid.get("owner")
    r["pdb_pubsym"] = pid.get("pubsym")
    d = dec.get(r["addr"], {})
    r["decomp_files"] = d.get("decomp_files", [])
    r["evid_txt"] = [x for x in d.get("evid_txt", []) if x]

# domain groups -> worker
GROUPS = {
    "w1_pass_builder": ("??", ["render/pass_builder/"]),
    "w2_nani_render": ("??", ["render/nani/", "render/raster_single", "render/mesh", "render/view_constants", "render/render_config"]),
    "w3_top_nonrender": ("??", ["components/", "asset_libraries/", "game_", "utils/", "startup/", "meta_commander", "name_hash"]),
    "w4_wall": ("wall", None),
    "w5_collision_terrain": (None, ["collision-eco", "terrain-water"]),
    "w6_ui_camera": (None, ["ui-input", "camera-time"]),
    "w7_history_save": (None, ["history", "session-save"]),
    "w8_renderloop_roofs": (None, ["render-loop", "roofs-rt"]),
    "w9_audio_backend": (None, ["audio-misc", "backend-misc"]),
}


def pick(r, dom, subs):
    if dom is not None:
        return r["domain"] == dom and (subs is None or any(r["rel"].startswith(s) for s in subs))
    return r["domain"] in subs


for wname, (dom, subs) in GROUPS.items():
    items = [r for r in sample if pick(r, dom, subs)]
    json.dump(items, open(CHUNK + f"/{wname}.json", "w", encoding="utf-8"), ensure_ascii=False, indent=1)
    print(f"{wname:<22}{len(items):>3} items")
