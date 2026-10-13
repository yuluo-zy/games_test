#!/usr/bin/env python3
# -*- coding: utf-8 -*-
"""Build radare2 job to read float constants used by water raster chain."""
import json

REGIONS = [
    ("const_1429a3c84", "0x1429a3c84", "0x20"),
    ("const_1429258e8", "0x1429258e8", "0x20"),
    ("const_142a7b658", "0x142a7b658", "0x20"),
    ("const_142925870", "0x142925870", "0x20"),
    ("const_142a7c88c", "0x142a7c88c", "0x20"),
]

JOB = [
    {"tool": "open_file",
     "args": {"file_path": "/Users/yuluo/Desktop/game/ljxsj_92385/Tiny Glade/tiny-glade.rne"},
     "timeout_seconds": 120},
    {"tool": "show_info", "args": {}, "timeout_seconds": 120},
]
for name, addr, size in REGIONS:
    JOB.append({"tool": "hexdump", "args": {"address": addr, "size": size}, "timeout_seconds": 120})
JOB.append({"tool": "close_file", "args": {}, "timeout_seconds": 120})

path = ("/Users/yuluo/Desktop/game/games_test/reverse-engineering/tiny-glade/"
        "tool-usage/jobs/water-cpu/radare-consts.json")
with open(path, "w") as fh:
    json.dump(JOB, fh, indent=2)
    fh.write("\n")
print("written", path)
