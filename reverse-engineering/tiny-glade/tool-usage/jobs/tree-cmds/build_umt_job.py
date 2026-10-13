#!/usr/bin/env python3
# -*- coding: utf-8 -*-
"""Build reva job for ui_move_tree (0x14226b1a0) to inspect MoveTreeCmd write."""
import json

TARGET = "0x14226b1a0"

STEP1 = '''
import json
from ghidra.app.decompiler import DecompInterface
prog = currentProgram
out = {"sha256": str(prog.getExecutableSHA256()), "program": str(prog.getName())}
addr = prog.getAddressFactory().getAddress("''' + TARGET + '''")
fm = prog.getFunctionManager()
f = fm.getFunctionAt(addr)
if f is None:
    raise RuntimeError("function absent at " + "''' + TARGET + '''")
out["name"] = str(f.getName())
out["entry"] = str(f.getEntryPoint())
out["signature"] = str(f.getSignature())
monitor.checkCancelled()
d = DecompInterface()
try:
    d.openProgram(prog)
    res = d.decompileFunction(f, 60, monitor)
    if not res.decompileCompleted():
        raise RuntimeError(str(res.getErrorMessage()))
    out["decompiled"] = res.getDecompiledFunction().getC()
finally:
    d.dispose()
print(json.dumps(out))
'''

STEP2 = '''
import json
prog = currentProgram
out = {"sha256": str(prog.getExecutableSHA256())}
addr = prog.getAddressFactory().getAddress("''' + TARGET + '''")
fm = prog.getFunctionManager()
f = fm.getFunctionAt(addr)
if f is None:
    raise RuntimeError("function absent at " + "''' + TARGET + '''")
body = f.getBody()
it = prog.getListing().getInstructions(body, True)
insns = []
while it.hasNext():
    monitor.checkCancelled()
    ins = it.next()
    a = ins.getAddress()
    mnem = ins.getMnemonicString()
    ops = []
    for i in range(ins.getNumOperands()):
        r = ins.getDefaultOperandRepresentation(i)
        ops.append(r if r is not None else "")
    raw = ins.getBytes()
    hexb = "".join("%02x" % (b & 0xFF) for b in raw)
    insns.append([str(a), mnem, ops, hexb])
out["insn_count"] = len(insns)
out["insns"] = insns
print(json.dumps(out))
'''

JOB = [
    {"tool": "run-script", "args": {"programPath": "/tiny-glade.exe", "code": STEP1, "timeoutSeconds": 60}, "timeout_seconds": 120},
    {"tool": "run-script", "args": {"programPath": "/tiny-glade.exe", "code": STEP2, "timeoutSeconds": 120}, "timeout_seconds": 240},
]
with open(
    "/Users/yuluo/Desktop/game/games_test/reverse-engineering/tiny-glade/"
    "tool-usage/jobs/tree-cmds/ui-move-tree-reva.json", "w",
) as fh:
    json.dump(JOB, fh, indent=2)
    fh.write("\n")
print("written ui_move_tree job")
