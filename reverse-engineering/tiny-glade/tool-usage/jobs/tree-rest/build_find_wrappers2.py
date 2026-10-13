#!/usr/bin/env python3
# -*- coding: utf-8 -*-
"""Debug: count functions, sample tree-tool related names."""
import json

CODE = r'''
import json
prog = currentProgram
fm = prog.getFunctionManager()
it = fm.getFunctions(True)
total = 0
found = []
sample = []
for fn in it:
    monitor.checkCancelled()
    nm = fn.getName()
    total += 1
    if "tree" in nm.lower():
        if "FunctionSystem" in nm or "system_type" in nm.lower():
            found.append([str(fn.getEntryPoint()), str(fn.getBody()), nm[:500]])
        elif len(sample) < 12:
            sample.append([str(fn.getEntryPoint()), nm[:300]])
print(json.dumps({"total": total, "found": found, "sample": sample}))
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
    "tool-usage/jobs/tree-rest/find-wrappers2.json",
    "w",
) as fh:
    json.dump(JOB, fh, indent=2)
    fh.write("\n")
print("written")
