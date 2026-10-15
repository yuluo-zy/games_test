import json

pub = json.load(open(r"D:/game/reverse-engineering/tiny-glade/evidence/render-architecture/public-symbols.json", encoding="utf-8"))
targets = {0x140002370, 0x14003bb80, 0x140949170, 0x14067dd80, 0x14092e6a0, 0x14092e6c0}
KEYS = ["draw_path", "slider3d", "steam", "UiNav", "first", "ReplayBuffer", "sync_point_count",
        "as_inner_mut", "events", "constrain", "flowery", "autumn", "is_dragging", "is_started",
        "olden", "Critter", "Sheep", "world_raster", "FilteredWallTime", "decorator_subtype",
        "decorator_archivist", "ImageFormationConfigs", "DecoAddrChangeTracker", "clone_behavior"]
from collections import defaultdict
by_va = defaultdict(list)
for r in pub:
    va = int(r["va"], 16) if r.get("va") else None
    if va in targets:
        by_va[va].append(r["symbol"])

for va in sorted(targets):
    print("===", hex(va), len(by_va[va]), "===")
    for sym in by_va[va]:
        if any(k in sym for k in KEYS):
            print("   ", sym[:220])
