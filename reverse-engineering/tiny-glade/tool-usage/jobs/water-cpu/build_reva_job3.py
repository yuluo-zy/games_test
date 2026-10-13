#!/usr/bin/env python3
# -*- coding: utf-8 -*-
"""Build reva job #3: draw_raster orchestrator, system_water build, callers."""
import json

CALLER_SCRIPT = '''
import json
prog = currentProgram
out = {{"target": "{va}", "callers": []}}
addr = prog.getAddressFactory().getAddress("{va}")
fm = prog.getFunctionManager()
f = fm.getFunctionAt(addr)
if f is None:
    raise RuntimeError("function absent at " + "{va}")
refmgr = prog.getReferenceManager()
refs = refmgr.getReferencesTo(addr)
seen = set()
for r in refs:
    monitor.checkCancelled()
    if not r.getReferenceType().isCall():
        continue
    frm = r.getFromAddress()
    if frm in seen:
        continue
    seen.add(frm)
    caller = fm.getFunctionContaining(frm)
    cname = str(caller.getName()) if caller else ""
    caddr = str(caller.getEntryPoint()) if caller else str(frm)
    out["callers"].append([str(frm), caddr, cname])
print(json.dumps(out))
'''

FUNC_SCRIPT = '''
import json, os
from ghidra.app.decompiler import DecompInterface
prog = currentProgram
out = {{"sha256": str(prog.getExecutableSHA256())}}
addr = prog.getAddressFactory().getAddress("{va}")
fm = prog.getFunctionManager()
f = fm.getFunctionAt(addr)
if f is None:
    raise RuntimeError("function absent at " + "{va}")
out["name"] = str(f.getName())
out["entry"] = str(f.getEntryPoint())
out["body"] = str(f.getBody())
out["signature"] = str(f.getSignature())
out["size"] = f.getBody().getNumAddresses() if f.getBody().contains(addr) else -1
d = DecompInterface()
try:
    d.openProgram(prog)
    res = d.decompileFunction(f, 120, monitor)
    out["decompiled"] = res.getDecompiledFunction().getC() if res.decompileCompleted() else None
finally:
    d.dispose()
print(json.dumps(out))
'''

FUNCS = {
    "draw_raster_water": "0x142260ae0",
    "system_water_build": "0x1406501d0",
}
CALLERS = ["0x141444c70", "0x1406501d0"]

JOB = []
for key, va in FUNCS.items():
    JOB.append({
        "tool": "run-script",
        "args": {"programPath": "/tiny-glade.exe",
                 "code": FUNC_SCRIPT.format(va=va), "timeoutSeconds": 300},
        "timeout_seconds": 600,
    })
for va in CALLERS:
    JOB.append({
        "tool": "run-script",
        "args": {"programPath": "/tiny-glade.exe",
                 "code": CALLER_SCRIPT.format(va=va), "timeoutSeconds": 60},
        "timeout_seconds": 120,
    })

path = ("/Users/yuluo/Desktop/game/games_test/reverse-engineering/tiny-glade/"
        "tool-usage/jobs/water-cpu/reva-draw.json")
with open(path, "w") as fh:
    json.dump(JOB, fh, indent=2)
    fh.write("\n")
print("written", path, len(JOB), "steps")
