#!/usr/bin/env python3
# -*- coding: utf-8 -*-
"""convert_preplaced_trees boundary: find wrapper + decompile body head."""
import json

SCAN = r'''
import json
prog = currentProgram
fm = prog.getFunctionManager()
found = []
it = fm.getFunctions(True)
for fn in it:
    monitor.checkCancelled()
    nm = fn.getName()
    if "convert_preplaced_trees" in nm:
        found.append([str(fn.getEntryPoint()), str(fn.getBody()), nm[:400]])
with open("/tmp/tree-rest-convert.json", "w") as fh:
    json.dump(found, fh)
print(json.dumps({"count": len(found),
                  "summaries": [[f[0], f[1]] for f in found]}))
'''

DECOMP = r'''
import json
from ghidra.app.decompiler import DecompInterface
prog = currentProgram
addr = prog.getAddressFactory().getAddress("0x14226be70")
fm = prog.getFunctionManager()
f = fm.getFunctionAt(addr)
out = {"name": str(f.getName()), "entry": str(f.getEntryPoint()), "body": str(f.getBody()),
       "signature": str(f.getSignature())}
d = DecompInterface()
try:
    d.openProgram(prog)
    res = d.decompileFunction(f, 120, monitor)
    if not res.decompileCompleted():
        raise RuntimeError(str(res.getErrorMessage()))
    out["decompiled"] = res.getDecompiledFunction().getC()
finally:
    d.dispose()
with open("/tmp/tree-rest-convert-decomp.json", "w") as fh:
    json.dump(out, fh)
print(json.dumps({"name": out["name"], "body": out["body"], "decompiled_len": len(out["decompiled"])}))
'''

JOB = [
    {
        "tool": "run-script",
        "args": {"programPath": "/tiny-glade.exe", "code": SCAN, "timeoutSeconds": 180},
        "timeout_seconds": 300,
    },
    {
        "tool": "run-script",
        "args": {"programPath": "/tiny-glade.exe", "code": DECOMP, "timeoutSeconds": 180},
        "timeout_seconds": 300,
    },
]

with open(
    "/Users/yuluo/Desktop/game/games_test/reverse-engineering/tiny-glade/"
    "tool-usage/jobs/tree-rest/convert.json",
    "w",
) as fh:
    json.dump(JOB, fh, indent=2)
    fh.write("\n")
print("written")
