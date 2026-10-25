import io

p = r"C:/Users/liyu/.claude/projects/D--game-reverse-engineering-tiny-glade-reconstruction/c5b12dd9-9d78-4eaa-a8fe-28d7f94cb36c/tool-results/b6zarqjuc.txt"
txt = io.open(p, encoding="utf-8", errors="replace").read()
lines = txt.splitlines()
seen = set()
for l in lines:
    if not l.startswith("0x14003bb80"):
        continue
    sym = l.split("|")[0].split("`")[-1]
    dm = l.split("|")[-1].strip()
    if any(k in dm for k in ["draw_path", "slider3d", "steam", "world_raster", "FilteredWallTime", "collision..world", "ButterflyCamera", "decorator_subtype", "decorator_archivist", "DecoAddrChangeTracker", "ImageFormationConfigs"]):
        # rust mangled: ends with <digits>17h<hash>E or .llvm...
        tail = sym[-70:]
        if tail not in seen:
            seen.add(tail)
            print(dm[:90])
            print("   mangled tail:", tail)
