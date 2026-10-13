#!/usr/bin/env python3
# -*- coding: utf-8 -*-
"""Find FunctionSystem wrappers containing BOTH PrefabInstanceData and
TerrainHeight (the recalc signature)."""
import json

CODE = r'''
import json
prog = currentProgram
fm = prog.getFunctionManager()
needles = ("PrefabInstanceData", "TerrainHeight")
found = []
it = fm.getFunctions(True)
for fn in it:
    monitor.checkCancelled()
    nm = fn.getName()
    if "FunctionSystem" not in nm:
        continue
    if not (needles[0] in nm and needles[1] in nm):
        continue
    found.append([str(fn.getEntryPoint()), str(fn.getBody()), nm[:1600]])
with open("/tmp/tree-rest-recalc-sig.json", "w") as fh:
    json.dump(found, fh)
print(json.dumps({"count": len(found),
                  "summaries": [[f[0], f[1]] for f in found]}))
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
    "tool-usage/jobs/tree-rest/scan-recalc-sig.json",
    "w",
) as fh:
    json.dump(JOB, fh, indent=2)
    fh.write("\n")
print("written")
