#!/usr/bin/env python3
# -*- coding: utf-8 -*-
"""radare2 oracle cross-check for despawn_trees (0x1422686d0).

Dumps:
- prologue instructions at entry
- the UCOMISS branch block (0x14226892c..0x142268957)
- the event-send block (0x142268943..0x1422689a8)
- the second query path block (0x14226888a..0x142268925)
- float constant at 0x142960674
- .data string at 0x143034d10 (as raw bytes)
"""
import json

STEPS = [
    ("prologue", "0x1422686d0", 14),
    ("ucomiss_block", "0x14226892c", 12),
    ("send_block", "0x142268943", 14),
    ("path2_head", "0x14226888a", 14),
    ("branch_108", "0x142268712", 6),
]

JOB = [{
    "tool": "open_file",
    "args": {"file_path": "/Users/yuluo/Desktop/game/ljxsj_92385/Tiny Glade/tiny-glade.rne"},
    "timeout_seconds": 120,
}]
for name, addr, n in STEPS:
    JOB.append({
        "tool": "disassemble",
        "args": {"address": addr, "num_instructions": n},
        "timeout_seconds": 120,
    })
JOB.append({
    "tool": "hexdump",
    "args": {"address": "0x142960674", "size": "8"},
    "timeout_seconds": 120,
})
JOB.append({
    "tool": "hexdump",
    "args": {"address": "0x143034d10", "size": "48"},
    "timeout_seconds": 120,
})
JOB.append({
    "tool": "close_file",
    "args": {},
    "timeout_seconds": 120,
})

with open(
    "/Users/yuluo/Desktop/game/games_test/reverse-engineering/tiny-glade/"
    "tool-usage/jobs/tree-rest/radare-despawn.json",
    "w",
) as fh:
    json.dump(JOB, fh, indent=2)
    fh.write("\n")
print("written")
