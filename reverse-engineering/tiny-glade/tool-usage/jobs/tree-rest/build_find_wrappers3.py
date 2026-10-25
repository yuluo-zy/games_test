#!/usr/bin/env python3
# -*- coding: utf-8 -*-
"""Find tree-tool bevy FunctionSystem wrappers; dump full names to a local
file and print only a short summary to stdout."""
import json

CODE = r'''
import json
prog = currentProgram
fm = prog.getFunctionManager()
it = fm.getFunctions(True)
total = 0
found = []
for fn in it:
    monitor.checkCancelled()
    nm = fn.getName()
    total += 1
    if "FunctionSystem" not in nm:
        continue
    low = nm.lower()
    for key in ("tree_move_bend", "recalc_tree_position_on_terrain_height_change",
                "despawn_trees", "replay_tree", "convert_preplaced_trees",
                "startup_tree_tool", "reset_tree_tool"):
        if key in low:
            found.append([key, str(fn.getEntryPoint()), str(fn.getBody()), nm])
            break
with open("/tmp/tree-rest-findwraps.json", "w") as fh:
    json.dump({"total": total, "found": found}, fh)
print(json.dumps({"total": total, "found_count": len(found),
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
    "tool-usage/jobs/tree-rest/find-wrappers3.json",
    "w",
) as fh:
    json.dump(JOB, fh, indent=2)
    fh.write("\n")
print("written")
