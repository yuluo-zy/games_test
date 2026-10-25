import io, re

p = r"C:/Users/liyu/.claude/projects/D--game-reverse-engineering-tiny-glade-reconstruction/c5b12dd9-9d78-4eaa-a8fe-28d7f94cb36c/tool-results/b6zarqjuc.txt"
txt = io.open(p, encoding="utf-8", errors="replace").read()
lines = txt.splitlines()
seen = set()
for l in lines:
    if not l.startswith("0x14003bb80"):
        continue
    sym = l.split("|")[0].split("`")[-1]
    dm = l.split("|")[-1].strip()
    if any(k in dm for k in ["draw_path", "slider3d", "steam", "world_raster", "FilteredWallTime", "collision..world", "ButterflyCamera", "decorator_subtype", "decorator_archivist"]):
        # method name: mangled Rust name contains e.g. '6apply', '5queue', '13new_archetype', '11init_state', '8validate'
        m = re.search(r"(6apply|5queue|14new_archetype|11init_state|8validate|9get_param|12queue_state|15validate_param|5get_d|8constrain|13update_marker|6marker|3new|4init|7register|12apply_state|9queue_all)", sym)
        key = (dm[:60], m.group(1) if m else "?")
        if key not in seen:
            seen.add(key)
            print(dm[:80], "||", m.group(1) if m else "NO-METHOD")
