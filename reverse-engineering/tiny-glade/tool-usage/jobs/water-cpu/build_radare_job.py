#!/usr/bin/env python3
# -*- coding: utf-8 -*-
"""Build radare2 job for system-water RNE oracle verification.

Disassembles a header window at each target function entry on the release
RNE (original machine code oracle), then a handful of known internal
offsets/branches supplied per function.
"""
import json

# name -> (va, header_window_insns)
FUNCS = {
    "startup_water": ("0x1408f1a20", 24),
    "water_rasterize_from_cpu": ("0x140a12680", 24),
    "rasterize_stroke_512": ("0x142261d10", 24),
    "copy_to_staging_512": ("0x142261760", 16),
    "copy_composite_to_scratch_512": ("0x142262680", 12),
    "commit_scratch_to_composite_512": ("0x142262a60", 12),
    "replay_raster_water": ("0x1409f4650", 24),
    "generate_replay_cmds_replay_water": ("0x141444c70", 24),
    "replay_water_closure2": ("0x140a01560", 16),
    "bilinear_sample_rasterbufref": ("0x140a14df0", 16),
}

JOB = [
    {
        "tool": "open_file",
        "args": {
            "file_path": "/Users/yuluo/Desktop/game/ljxsj_92385/Tiny Glade/tiny-glade.rne",
        },
        "timeout_seconds": 120,
    },
    {
        "tool": "show_info",
        "args": {},
        "timeout_seconds": 120,
    },
]

for name, (va, n) in FUNCS.items():
    JOB.append({
        "tool": "hexdump",
        "args": {"address": va, "size": "24"},
        "timeout_seconds": 120,
    })
    JOB.append({
        "tool": "disassemble",
        "args": {"address": va, "num_instructions": n},
        "timeout_seconds": 120,
    })

JOB.append({
    "tool": "close_file",
    "args": {},
    "timeout_seconds": 120,
})

path = ("/Users/yuluo/Desktop/game/games_test/reverse-engineering/tiny-glade/"
        "tool-usage/jobs/water-cpu/radare-oracle.json")
with open(path, "w") as fh:
    json.dump(JOB, fh, indent=2)
    fh.write("\n")
print("written", path, len(JOB), "steps")
