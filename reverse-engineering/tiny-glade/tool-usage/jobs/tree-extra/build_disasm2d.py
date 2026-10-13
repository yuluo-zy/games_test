#!/usr/bin/env python3
# -*- coding: utf-8 -*-
"""tree-extra 2d: signatures only (fast) for spp_* family + fun_d9d0 +
names of functions containing reset inline hosts and convert callers."""
import json

CODE = r'''
import json
prog = currentProgram
fm = prog.getFunctionManager()
out = {}

def dumpfn(label, a, containing=False):
    addr = prog.getAddressFactory().getAddress(a)
    f = fm.getFunctionAt(addr)
    if f is None and containing:
        f = fm.getFunctionContaining(addr)
    if f is None:
        out[label] = None
        return
    out[label] = [str(f.getEntryPoint()), str(f.getBody()), str(f.getName())[:5000]]

for label, a in [("spp_initialize", "0x1409bdd50"), ("spp_run_unsafe", "0x1409bdfc0"),
                 ("spp_validate_param", "0x1409be420"), ("spp_run_if", "0x1409beac0"),
                 ("spp_apply_deferred", "0x1409be3a0")]:
    dumpfn(label, a)
dumpfn("fun_d9d0", "0x141bdb9d0")
dumpfn("reset_host1_contain", "0x141bb29d0", containing=True)
dumpfn("reset_host2_contain", "0x141bcbc10", containing=True)
dumpfn("convert_call_contain", "0x141bcb978", containing=True)

# find run_unsafe wrappers with the exact single-param ResMut<TreeStorage> type
# via Ghidra findFunctions by name query instead of full scan
from ghidra.app.util.importer import MessageLog  # noqa
try:
    from ghidra.program.model.symbol import SymbolTable
except Exception:
    pass

with open("/tmp/tree-extra-2d.json", "w") as fh:
    json.dump(out, fh)
print(json.dumps(out))
'''

JOB = [{
    "tool": "run-script",
    "args": {"programPath": "/tiny-glade.exe", "code": CODE, "timeoutSeconds": 120},
    "timeout_seconds": 300,
}]

with open(
    "/Users/yuluo/Desktop/game/games_test/reverse-engineering/tiny-glade/"
    "tool-usage/jobs/tree-extra/disasm2d.json",
    "w",
) as fh:
    json.dump(JOB, fh, indent=2)
    fh.write("\n")
print("written")
