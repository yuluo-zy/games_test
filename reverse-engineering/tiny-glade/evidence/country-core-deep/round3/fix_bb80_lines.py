# -*- coding: utf-8 -*-
import io, os

CH = r"D:/game/reverse-engineering/tiny-glade/evidence/country-core-deep/chapters"

OLD_SUF = "（该址唯一符号）；原{old}身份未证实（疑 ICF 折叠 no-op） | rne-read 0x14003bb80；PDB 过程表 |"
NEW_SUF = "（round1 误判\u201c唯一符号\u201d）；S_PUB32 该址含{new}符号 → 身份成立（无状态胶水 no-op，不计业务） | rne-read 0x14003bb80；S_PUB32 public-symbols.json |"

FILES = [
    ("systems/paths/draw_path.rs.md", [
        ("apply", "draw_path SystemParam apply", "PathRastersResources SystemParam apply"),
        ("new_archetype", "draw_path SystemParam new_archetype", "PathRastersResources SystemParam new_archetype"),
        ("queue", "draw_path SystemParam queue", "PathRastersResources SystemParam queue"),
    ]),
    ("systems/ui/resources/slider3d.rs.md", [
        ("apply", "slider3d SystemParam apply", "Slider3d SystemParam apply"),
        ("new_archetype", "slider3d SystemParam new_archetype", "Slider3d SystemParam new_archetype"),
        ("queue", "slider3d SystemParam queue", "Slider3d SystemParam queue"),
    ]),
    ("systems/steam/mod.rs.md", [
        ("apply", "steam SystemParam apply", "SteamOverlayOpenResources SystemParam apply"),
        ("new_archetype", "steam SystemParam new_archetype", "SteamOverlayOpenResources SystemParam new_archetype"),
        ("queue", "steam SystemParam queue", "SteamOverlayOpenResources SystemParam queue"),
    ]),
]

for path, repls in FILES:
    p = os.path.join(CH, path)
    txt = io.open(p, encoding="utf-8").read()
    n = 0
    for method, old_phrase, new_phrase in repls:
        old = OLD_SUF.format(old=old_phrase)
        new = NEW_SUF.format(new=new_phrase)
        if old in txt:
            txt = txt.replace(old, new)
            n += 1
        else:
            print("NOT FOUND in", path, ":", method)
    io.open(p, "w", encoding="utf-8", newline="").write(txt)
    print(path, "replaced", n)
