#!/usr/bin/env python3
# -*- coding: utf-8 -*-
"""Find bevy system_type / FunctionSystem wrapper functions for the
tree-tool systems by scanning function names."""
import json

CODE = r'''
import json
prog = currentProgram
out = {}
fm = prog.getFunctionManager()
it = fm.getFunctions(True)
for fn in it:
    monitor.checkCancelled()
    nm = fn.getName()
    if "FunctionSystem" not in nm:
        continue
    low = nm.lower()
    for key in ("tree_move_bend", "recalc_tree_position_on_terrain_height_change",
                "despawn_trees", "replay_tree", "convert_preplaced_trees",
                "process_tree_cmds", "startup_tree_tool", "reset_tree_tool"):
        if key in low:
            out.setdefault(key, []).append(
                [str(fn.getEntryPoint()), str(fn.getBody()), nm[:600]])
for k, v in out.items():
    print(k, len(v))
print("===")
print(json.dumps(out))
'''

JOB = [
    {
        "tool": "run-script",
        "args": {
            "programPath": "/tiny-glade.exe",
            "code": CODE,
            "timeoutSeconds": 120,
        },
        "timeout_seconds": 240,
    }
]

with open(
    "/Users/yuluo/Desktop/game/games_test/reverse-engineering/tiny-glade/"
    "tool-usage/jobs/tree-rest/find-wrappers.json",
    "w",
) as fh:
    json.dump(JOB, fh, indent=2)
    fh.write("\n")
print("written")
