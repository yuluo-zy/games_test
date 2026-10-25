import io

p = r"C:/Users/liyu/.claude/projects/D--game-reverse-engineering-tiny-glade-reconstruction/c5b12dd9-9d78-4eaa-a8fe-28d7f94cb36c/tool-results/b6zarqjuc.txt"
txt = io.open(p, encoding="utf-8", errors="replace").read()
lines = txt.splitlines()
n = 0
for l in lines:
    if not l.startswith("0x14003bb80"):
        continue
    dm = l.split("|")[-1].strip()
    if "draw_path" in dm or "world_raster_defs::_::" in dm or "slider3d::_" in dm:
        print(l[:400])
        print("----")
        n += 1
        if n >= 6:
            break
