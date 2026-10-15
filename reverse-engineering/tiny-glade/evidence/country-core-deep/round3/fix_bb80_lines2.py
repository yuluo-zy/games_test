# -*- coding: utf-8 -*-
import io, os

CH = r"D:/game/reverse-engineering/tiny-glade/evidence/country-core-deep/chapters"

# method -> (fn marker, new concl)
PLANS = {
    "draw_path": {
        "apply": "PathRastersResources SystemParam apply",
        "new_archetype": "PathRastersResources SystemParam new_archetype",
        "queue": "PathRastersResources SystemParam queue",
    },
    "slider3d": {
        "apply": "Slider3d SystemParam apply",
        "new_archetype": "Slider3d SystemParam new_archetype",
        "queue": "Slider3d SystemParam queue",
    },
    "steam": {
        "apply": "SteamOverlayOpenResources SystemParam apply",
        "new_archetype": "SteamOverlayOpenResources SystemParam new_archetype",
        "queue": "SteamOverlayOpenResources SystemParam queue",
    },
}

NEW_EVID = "rne-read 0x14003bb80；S_PUB32 public-symbols.json"

def new_concl(marker, method):
    return ("O（round3 round2 复查修正）：0x14003bb80=ICF 折叠区（S_PUB32 437 符号，含 "
            "Plugin::finish/cleanup、iced Widget 空方法、SystemParam 胶水、marker 等），"
            "该址含 {sym} 符号 → 身份成立；机器码 1B ret = 无状态胶水 no-op "
            "（round1 曾误判\u201c该址唯一符号=Plugin::finish\u201d，身份未证实结论作废）").format(sym=marker)

files = {
    "draw_path": "systems/paths/draw_path.rs.md",
    "slider3d": "systems/ui/resources/slider3d.rs.md",
    "steam": "systems/steam/mod.rs.md",
}

total = 0
for key, path in files.items():
    p = os.path.join(CH, path)
    lines = io.open(p, encoding="utf-8").read().splitlines(keepends=True)
    out = []
    for line in lines:
        stripped = line.rstrip("\r\n")
        cells = [c.strip() for c in stripped.strip("|").split("|")]
        if len(cells) >= 6 and cells[1] == "0x14003bb80":
            fn = cells[0]
            # find which method this row is
            for method in PLANS[key]:
                if fn.endswith("::" + method + "`"):
                    cells[4] = new_concl(PLANS[key][method], method)
                    cells[5] = NEW_EVID
                    line = "| " + " | ".join(cells) + " |" + (line[-1] if line.endswith("\n") else "")
                    total += 1
                    break
        out.append(line)
    io.open(p, "w", encoding="utf-8", newline="").write("".join(out))
    print(path, "done")

print("total replaced:", total)
