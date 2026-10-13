#!/usr/bin/env python3
# -*- coding: utf-8 -*-
"""Decompile replay_buffer helpers; radare2 constants cross-check."""
import json

JOB = []
# reva get-decompilation for the two ReplayBuffer helper callees
for name, va in [("buf_call_0x140949170", "0x140949170"),
                 ("buf_call_0x1409f1a80", "0x1409f1a80")]:
    JOB.append({
        "tool": "get-decompilation",
        "args": {
            "programPath": "/tiny-glade.exe",
            "functionNameOrAddress": va,
            "offset": 1,
            "limit": 120,
            "includeIncomingReferences": False,
            "includeReferenceContext": False,
        },
        "timeout_seconds": 240,
    })

with open(
    "/Users/yuluo/Desktop/game/games_test/reverse-engineering/tiny-glade/"
    "tool-usage/jobs/tree-rest/buf-helpers.json",
    "w",
) as fh:
    json.dump(JOB, fh, indent=2)
    fh.write("\n")

# radare2 job
R2 = [
    {"tool": "open_file", "args": {"file_path": "/Users/yuluo/Desktop/game/ljxsj_92385/Tiny Glade/tiny-glade.rne"}, "timeout_seconds": 120},
    # tree_move_bend constants
    {"tool": "hexdump", "args": {"address": "0x142925984", "size": "16"}, "timeout_seconds": 120},
    # recalc constants
    {"tool": "hexdump", "args": {"address": "0x14292e980", "size": "16"}, "timeout_seconds": 120},
    {"tool": "hexdump", "args": {"address": "0x142a83814", "size": "8"}, "timeout_seconds": 120},
    # replay_tree constants
    {"tool": "hexdump", "args": {"address": "0x142930d70", "size": "8"}, "timeout_seconds": 120},
    {"tool": "hexdump", "args": {"address": "0x143034d10", "size": "24"}, "timeout_seconds": 120},
    # replay_tree prologue cross-check
    {"tool": "disassemble", "args": {"address": "0x141bc34f0", "num_instructions": 10}, "timeout_seconds": 120},
    # tree_move_bend UCOMISS/lerp cross-check
    {"tool": "disassemble", "args": {"address": "0x142268c86", "num_instructions": 18}, "timeout_seconds": 120},
    # recalc sample/update cross-check
    {"tool": "disassemble", "args": {"address": "0x142268f74", "num_instructions": 22}, "timeout_seconds": 120},
    {"tool": "close_file", "args": {}, "timeout_seconds": 120},
]
with open(
    "/Users/yuluo/Desktop/game/games_test/reverse-engineering/tiny-glade/"
    "tool-usage/jobs/tree-rest/radare-rest.json",
    "w",
) as fh:
    json.dump(R2, fh, indent=2)
    fh.write("\n")
print("written both")
