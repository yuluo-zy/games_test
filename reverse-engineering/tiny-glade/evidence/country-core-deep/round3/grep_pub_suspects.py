import re

txt = open(r"D:/game/reverse-engineering/tiny-glade/evidence/country-core-deep/round3/pub_full.txt", encoding="utf-8").read()
secs = txt.split("==========")
for sec in secs[1:]:
    lines = sec.splitlines()
    head = lines[0].strip()
    mhead = re.match(r"0x([0-9a-f]+) (\d+) entries", head)
    if not mhead:
        continue
    va = "0x" + mhead.group(1)
    print("===", va, "===")
    for l in lines[1:]:
        l = l.strip()
        if "`" not in l:
            continue
        mang = l.split("`")[1]
        dm = l.split("->")[-1].strip() if "->" in l else ""
        tags = ["apply", "new_archetype", "queue", "validate", "init_state", "get_param",
                "first", "last", "len", "sync_point_count", "as_inner_mut", "events",
                "constrain_to_be_above_terrain", "is_dragging", "is_started", "is_flowery",
                "is_autumn", "is_olden", "clone_behavior"]
        hit = [t for t in tags if t in mang]
        if any(k in mang for k in ["draw_path", "slider3d", "steam", "UiNav", "first",
                                   "ReplayBuffer", "sync_point_count", "as_inner_mut",
                                   "events", "constrain", "flowery", "autumn",
                                   "is_dragging", "is_started", "olden", "Critter",
                                   "Sheep", "world_raster", "FilteredWallTime",
                                   "decorator_subtype", "decorator_archivist",
                                   "ImageFormationConfigs", "DecoAddrChangeTracker"]):
            print("   ", hit, "|", dm[:100] if dm else mang[-100:])
