# -*- coding: utf-8 -*-
import io, os

CH = r"D:/game/reverse-engineering/tiny-glade/evidence/country-core-deep/chapters"

PATCHES = [
    ("resources/walls/inner_walls.rs.md",
     "| `country_core::resources::walls::inner_walls::InnerWalls::remove_all` | 0x14085a730 | 120 | InnerWalls::max_id | O | wall\\decomp\\decomp-0x14085a730.txt |",
     "| `country_core::resources::walls::inner_walls::InnerWalls::remove_all` | 0x14085a730 | 120 | InnerWalls::remove_all（round3 round2 复查修正：原\u201cmax_id\u201d误标；PDB 唯一符号=remove_all，行为=清空 map 头 4 qword+128 位占用位图写 0） | O | wall\\decomp\\decomp-0x14085a730.txt |"),

    ("resources/walls/decorator_archivist.rs.md",
     "| `country_core::resources::walls::decorator_archivist::DecoratorArchivist::decorator_cull_allow_wall` | 0x140b2bc00 | 312 | decorator_cull_allow_any（0x98 步长 map 命中回调） | O | wall\\decomp\\decomp-0x140b2bc00.txt |",
     "| `country_core::resources::walls::decorator_archivist::DecoratorArchivist::decorator_cull_allow_wall` | 0x140b2bc00 | 312 | decorator_cull_allow_wall（round3 round2 复查修正：原\u201ccull_allow_any\u201d误标；PDB exact=cull_allow_wall） | O | wall\\decomp\\decomp-0x140b2bc00.txt |"),
]

for rel, old, new in PATCHES:
    p = os.path.join(CH, rel)
    txt = io.open(p, encoding="utf-8").read()
    if old not in txt:
        print("NOT FOUND:", rel)
        continue
    io.open(p, "w", encoding="utf-8", newline="").write(txt.replace(old, new, 1))
    print("patched:", rel)
