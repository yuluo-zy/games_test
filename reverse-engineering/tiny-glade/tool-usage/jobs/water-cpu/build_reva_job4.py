#!/usr/bin/env python3
# -*- coding: utf-8 -*-
"""Build reva job #4: WorldRaster::new, clear_buffer, draw_water top-level."""
import json

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
    res = d.decompileFunction(f, 120, monitor)
    out["decompiled"] = res.getDecompiledFunction().getC() if res.decompileCompleted() else None
finally:
    d.dispose()
print(json.dumps(out))
'''

FUNCS = {
    "worldraster_new_512": "0x140a15530",
    "worldraster_subscribe": "0x140a158e0",
    "clear_buffer_512": "0x140a15a10",
    "draw_water_top": "0x141ba9130",
}

JOB = []
for key, va in FUNCS.items():
    JOB.append({
        "tool": "run-script",
        "args": {"programPath": "/tiny-glade.exe",
                 "code": FUNC_SCRIPT.format(va=va), "timeoutSeconds": 300},
        "timeout_seconds": 600,
    })

path = ("/Users/yuluo/Desktop/game/games_test/reverse-engineering/tiny-glade/"
        "tool-usage/jobs/water-cpu/reva-final.json")
with open(path, "w") as fh:
    json.dump(JOB, fh, indent=2)
    fh.write("\n")
print("written", path, len(JOB), "steps")
