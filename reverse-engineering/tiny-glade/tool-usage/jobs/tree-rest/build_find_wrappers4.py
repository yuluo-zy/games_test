#!/usr/bin/env python3
# -*- coding: utf-8 -*-
"""Find system_type FunctionSystem wrappers for specific tree-tool systems."""
import json

CODE = r'''
import json
prog = currentProgram
fm = prog.getFunctionManager()
keys = ("tree_move_bend", "recalc_tree_position_on_terrain_height_change",
        "despawn_trees", "replay_tree", "convert_preplaced_trees",
        "hide_trees", "ui_place_tree", "ui_move_tree", "ui_delete_tree",
        "process_tree_cmds")
found = []
it = fm.getFunctions(True)
for fn in it:
    monitor.checkCancelled()
    nm = fn.getName()
    if "FunctionSystem" not in nm:
        continue
    low = nm.lower()
    for key in keys:
        if key in low:
            found.append([key, str(fn.getEntryPoint()), str(fn.getBody()), nm])
            break
with open("/tmp/tree-rest-wrappers-full.json", "w") as fh:
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
    "tool-usage/jobs/tree-rest/find-wrappers4.json",
    "w",
) as fh:
    json.dump(JOB, fh, indent=2)
    fh.write("\n")
print("written")
