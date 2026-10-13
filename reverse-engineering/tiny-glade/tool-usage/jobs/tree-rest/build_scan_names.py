#!/usr/bin/env python3
# -*- coding: utf-8 -*-
"""Scan function names for the target system names (any wrapper form)."""
import json

CODE = r'''
import json
prog = currentProgram
fm = prog.getFunctionManager()
keys = ("tree_move_bend::tree_move_bend", "recalc_tree_position_on_terrain_height_change",
        "replay_tree::replay_tree", "convert_preplaced_trees::convert_preplaced_trees",
        "despawn_trees", "hide_trees::hide_trees", "reset_tree_tool", "startup_tree_tool")
found = []
it = fm.getFunctions(True)
for fn in it:
    monitor.checkCancelled()
    nm = fn.getName()
    low = nm.lower()
    for key in keys:
        if key in low:
            found.append([key, str(fn.getEntryPoint()), str(fn.getBody()), nm[:600]])
            break
with open("/tmp/tree-rest-names.json", "w") as fh:
    json.dump(found, fh)
print(json.dumps({"count": len(found),
                  "summaries": [[f[0], f[1], f[2]] for f in found]}))
'''

JOB = [
    {
        "tool": "run-script",
        "args": {
            "programPath": "/tiny-glade.exe",
            "code": CODE,
            "timeoutSeconds": 180,
        },
        "timeout_seconds": 300,
    }
]

with open(
    "/Users/yuluo/Desktop/game/games_test/reverse-engineering/tiny-glade/"
    "tool-usage/jobs/tree-rest/scan-names.json",
    "w",
) as fh:
    json.dump(JOB, fh, indent=2)
    fh.write("\n")
print("written")
