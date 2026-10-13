#!/usr/bin/env python3
# -*- coding: utf-8 -*-
"""Probe Ghidra function boundaries for tree-tool rest functions.

Checks what Ghidra function exists at / contains each target VA, its name,
entry point and body, plus signature when present.
"""
import json

TARGETS = [
    "0x1422686d0",  # despawn_trees (inventory PDB VA, size 751)
    "0x141bc34f0",  # replay_tree (inventory, size 2722)
    "0x1422689c0",  # tree_move_bend (inventory, size 1019)
    "0x142268dc0",  # recalc_tree_position_on_terrain_height_change (size 902)
    "0x14226be70",  # convert_preplaced_trees (size 4827)
    "0x141bb2110",  # bevy system_type wrapper for despawn_trees (prior evidence)
]

CODE = r'''
import json
prog = currentProgram
fm = prog.getFunctionManager()
out = {{}}
for a in {targets!r}:
    addr = prog.getAddressFactory().getAddress(a)
    f = fm.getFunctionAt(addr)
    containing = fm.getFunctionContaining(addr)
    rec = {{}}
    if f is not None:
        rec["at_name"] = str(f.getName())
        rec["at_entry"] = str(f.getEntryPoint())
        rec["at_body"] = str(f.getBody())
        rec["at_sig"] = str(f.getSignature())
    if containing is not None:
        rec["cont_name"] = str(containing.getName())
        rec["cont_entry"] = str(containing.getEntryPoint())
        rec["cont_body"] = str(containing.getBody())
    out[a] = rec
print(json.dumps(out))
'''

JOB = [
    {
        "tool": "run-script",
        "args": {
            "programPath": "/tiny-glade.exe",
            "code": CODE.format(targets=TARGETS),
            "timeoutSeconds": 120,
        },
        "timeout_seconds": 240,
    }
]

with open(
    "/Users/yuluo/Desktop/game/games_test/reverse-engineering/tiny-glade/"
    "tool-usage/jobs/tree-rest/probe.json",
    "w",
) as fh:
    json.dump(JOB, fh, indent=2)
    fh.write("\n")
print("written")
