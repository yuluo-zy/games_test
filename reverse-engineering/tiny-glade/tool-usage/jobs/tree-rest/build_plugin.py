#!/usr/bin/env python3
# -*- coding: utf-8 -*-
"""Decompile system_tree plugin build + replay_tree full disasm/decompile."""
import json

CODE = r'''
import json
from ghidra.app.decompiler import DecompInterface
prog = currentProgram
out = {"sha256": str(prog.getExecutableSHA256())}
addr = prog.getAddressFactory().getAddress("0x140650520")
fm = prog.getFunctionManager()
f = fm.getFunctionAt(addr)
out["name"] = str(f.getName())
out["entry"] = str(f.getEntryPoint())
out["body"] = str(f.getBody())
out["signature"] = str(f.getSignature())
d = DecompInterface()
try:
    d.openProgram(prog)
    res = d.decompileFunction(f, 120, monitor)
    if not res.decompileCompleted():
        raise RuntimeError(str(res.getErrorMessage()))
    out["decompiled"] = res.getDecompiledFunction().getC()
finally:
    d.dispose()
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
    "tool-usage/jobs/tree-rest/plugin-build.json",
    "w",
) as fh:
    json.dump(JOB, fh, indent=2)
    fh.write("\n")
print("written")
