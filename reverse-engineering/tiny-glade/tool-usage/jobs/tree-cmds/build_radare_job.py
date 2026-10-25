#!/usr/bin/env python3
# -*- coding: utf-8 -*-
"""Build radare2 oracle job for process_tree_cmds cross-check.

Opens the release RNE, verifies identity, hexdumps key strings, and
disassembles the three branch regions + prologue of 0x142267430.
"""
import json

OPEN = {
    "tool": "open_file",
    "args": {"file_path": "/Users/yuluo/Desktop/game/ljxsj_92385/Tiny Glade/tiny-glade.rne"},
    "timeout_seconds": 120,
}
INFO = {"tool": "show_info", "args": {}, "timeout_seconds": 120}
CLOSE = {"tool": "close_file", "args": {}, "timeout_seconds": 120}

STEPS = [OPEN, INFO]
for addr in [
    "0x142267430",  # prologue
    "0x142267640",  # move loop head
    "0x142267a29",  # insert loop head
    "0x142268201",  # delete loop head
]:
    STEPS.append({
        "tool": "disassemble",
        "args": {"address": addr, "num_instructions": 30},
        "timeout_seconds": 120,
    })
# hexdump key string pointers (MoveTreeCmd log msg, string constants)
for addr, size in [("0x143035048", "40"), ("0x143035058", "40")]:
    STEPS.append({
        "tool": "hexdump",
        "args": {"address": addr, "size": size},
        "timeout_seconds": 120,
    })
STEPS.append(CLOSE)

with open(
    "/Users/yuluo/Desktop/game/games_test/reverse-engineering/tiny-glade/"
    "tool-usage/jobs/tree-cmds/tree-cmds-radare.json", "w",
) as fh:
    json.dump(STEPS, fh, indent=2)
    fh.write("\n")
print("written radare job with", len(STEPS), "steps")
