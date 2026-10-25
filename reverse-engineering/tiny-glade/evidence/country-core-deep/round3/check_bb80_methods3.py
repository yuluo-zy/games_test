import io, re

p = r"C:/Users/liyu/.claude/projects/D--game-reverse-engineering-tiny-glade-reconstruction/c5b12dd9-9d78-4eaa-a8fe-28d7f94cb36c/tool-results/b6zarqjuc.txt"
txt = io.open(p, encoding="utf-8", errors="replace").read()
lines = txt.splitlines()
seen = set()
for l in lines:
    if not l.startswith("0x14003bb80"):
        continue
    parts = l.split("|")
    if len(parts) < 3 or "`" not in parts[1]:
        continue
    mangled = parts[1].split("`")[1]
    dm = parts[2].strip()
    if any(k in dm for k in ["draw_path", "slider3d", "steam", "world_raster", "FilteredWallTime", "collision..world", "ButterflyCamera", "decorator_subtype", "decorator_archivist", "DecoAddrChangeTracker"]):
        # Rust v0 mangling: method segment before '17h<hash>E' ; impl paths are '_$LT$...$GT$'
        m = re.findall(r"(\d+(?:apply|queue|new_archetype|init_state|validate|get_param|constrain|marker|remove|update|render|draw|operate|diff|on_input|provide|static_drop|finish|cleanup))", mangled)
        key = (dm[:70], tuple(m))
        if key not in seen:
            seen.add(key)
            print(dm[:88])
            print("    methods:", m)
