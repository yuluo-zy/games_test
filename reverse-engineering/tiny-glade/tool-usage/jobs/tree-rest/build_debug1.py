#!/usr/bin/env python3
# -*- coding: utf-8 -*-
"""Debug why wrapper scan found 0: probe known wrapper + count system_type names."""
import json

CODE = r'''
import json
prog = currentProgram
fm = prog.getFunctionManager()
out = {}
a = prog.getAddressFactory().getAddress("0x141bb2110")
f = fm.getFunctionAt(a)
out["at_141bb2110"] = str(f.getName()) if f else None
it = fm.getFunctions(True)
cnt_system_type = 0
cnt_mangled = 0
sample = []
for fn in it:
    monitor.checkCancelled()
    nm = fn.getName()
    if "system_type" in nm:
        cnt_system_type += 1
    if "system_tree::" in nm:
        cnt_mangled += 1
        if len(sample) < 20:
            sample.append([str(fn.getEntryPoint()), nm[:250]])
out["cnt_system_type"] = cnt_system_type
out["cnt_system_tree_mangled"] = cnt_mangled
out["sample"] = sample
with open("/tmp/tree-rest-debug1.json", "w") as fh:
    json.dump(out, fh)
print(json.dumps({"at": out["at_141bb2110"],
                  "cnt_system_type": cnt_system_type,
                  "cnt_system_tree_mangled": cnt_mangled,
                  "sample_count": len(sample)}))
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
    "tool-usage/jobs/tree-rest/debug1.json",
    "w",
) as fh:
    json.dump(JOB, fh, indent=2)
    fh.write("\n")
print("written")
