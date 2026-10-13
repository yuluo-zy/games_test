#!/usr/bin/env python3
# -*- coding: utf-8 -*-
"""Build small reva job: caller discovery for world_raster chain + a few
small water functions. All read-only run-script, prints compact JSON."""
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
import json
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
    res = d.decompileFunction(f, 60, monitor)
    out["decompiled"] = res.getDecompiledFunction().getC() if res.decompileCompleted() else None
finally:
    d.dispose()
print(json.dumps(out))
'''

CALLERS = ["0x142261d10", "0x142261760", "0x142262a60", "0x142262680",
           "0x140a12680", "0x1409f4650"]
FUNCS = {
    "brush_set_water": "0x1409f18a0",
    "brush_reset_water": "0x1409f1890",
    "rasterize_write_closure0": "0x142821d90",
}

JOB = []
for va in CALLERS:
    JOB.append({
        "tool": "run-script",
        "args": {"programPath": "/tiny-glade.exe",
                 "code": CALLER_SCRIPT.format(va=va), "timeoutSeconds": 60},
        "timeout_seconds": 120,
    })
for key, va in FUNCS.items():
    JOB.append({
        "tool": "run-script",
        "args": {"programPath": "/tiny-glade.exe",
                 "code": FUNC_SCRIPT.format(va=va), "timeoutSeconds": 300},
        "timeout_seconds": 600,
    })

path = ("/Users/yuluo/Desktop/game/games_test/reverse-engineering/tiny-glade/"
        "tool-usage/jobs/water-cpu/reva-callers.json")
with open(path, "w") as fh:
    json.dump(JOB, fh, indent=2)
    fh.write("\n")
print("written", path, len(JOB), "steps")
