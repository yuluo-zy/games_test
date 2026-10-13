#!/usr/bin/env python3
# -*- coding: utf-8 -*-
"""Build radare2 job to read the .rdata string tables referenced by
startup_water (0x1408f1a20) and water_rasterize_from_cpu string refs."""
import json

REGIONS = [
    # name, address, size
    ("startup_water_strings_142aa6a00", "0x142aa6a00", "0x400"),
    ("startup_water_strings_142aa7a00", "0x142aa7a00", "0x400"),
]

JOB = [
    {
        "tool": "open_file",
        "args": {"file_path": "/Users/yuluo/Desktop/game/ljxsj_92385/Tiny Glade/tiny-glade.rne"},
        "timeout_seconds": 120,
    },
    {"tool": "show_info", "args": {}, "timeout_seconds": 120},
]
for name, addr, size in REGIONS:
    JOB.append({
        "tool": "hexdump",
        "args": {"address": addr, "size": size},
        "timeout_seconds": 120,
    })
JOB.append({"tool": "close_file", "args": {}, "timeout_seconds": 120})

path = ("/Users/yuluo/Desktop/game/games_test/reverse-engineering/tiny-glade/"
        "tool-usage/jobs/water-cpu/radare-strings.json")
with open(path, "w") as fh:
    json.dump(JOB, fh, indent=2)
    fh.write("\n")
print("written", path)
