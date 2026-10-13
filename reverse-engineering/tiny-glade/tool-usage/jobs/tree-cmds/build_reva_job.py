#!/usr/bin/env python3
# -*- coding: utf-8 -*-
"""Build reva job for process_tree_cmds (0x142267430) analysis.

Generates tree-cmds-reva.json with three run-script steps:
  step1: identity + function info + signature + callees + full decompiled C
  step2: full disassembly (all instructions in body) + call targets
Each step prints a single compact JSON line to stdout which the operator
saves to the result file.
"""
import json

TARGET = "0x142267430"

BASE_IMPORTS = (
    "import json\n"
    "from ghidra.app.decompiler import DecompInterface\n"
)

# ---------------------------------------------------------------------------
# step1: identity / function / decompiled C
# ---------------------------------------------------------------------------
STEP1 = BASE_IMPORTS + f'''
prog = currentProgram
out = {{"sha256": str(prog.getExecutableSHA256()), "program": str(prog.getName())}}
addr = prog.getAddressFactory().getAddress("{TARGET}")
fm = prog.getFunctionManager()
f = fm.getFunctionAt(addr)
if f is None:
    raise RuntimeError("function absent at " + "{TARGET}")
out["name"] = str(f.getName())
out["entry"] = str(f.getEntryPoint())
out["body"] = str(f.getBody())
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

# ---------------------------------------------------------------------------
# step2: full disassembly + call targets
# ---------------------------------------------------------------------------
STEP2 = BASE_IMPORTS + f'''
prog = currentProgram
out = {{"sha256": str(prog.getExecutableSHA256())}}
addr = prog.getAddressFactory().getAddress("{TARGET}")
fm = prog.getFunctionManager()
f = fm.getFunctionAt(addr)
if f is None:
    raise RuntimeError("function absent at " + "{TARGET}")
body = f.getBody()
listing = prog.getListing()
it = listing.getInstructions(body, True)
insns = []
refmgr = prog.getReferenceManager()
call_targets = {{}}
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
    if mnem == "CALL":
        for r in refmgr.getReferencesFrom(a):
            if r.getReferenceType().isCall():
                to = r.getToAddress()
                tgt = fm.getFunctionAt(to)
                nm = str(tgt.getName()) if tgt else str(to)
                if nm not in call_targets:
                    call_targets[nm] = str(to)
out["insn_count"] = len(insns)
out["insns"] = insns
out["call_targets"] = call_targets
print(json.dumps(out))
'''

JOB = [
    {
        "tool": "run-script",
        "args": {
            "programPath": "/tiny-glade.exe",
            "code": STEP1,
            "timeoutSeconds": 60,
        },
        "timeout_seconds": 120,
    },
    {
        "tool": "run-script",
        "args": {
            "programPath": "/tiny-glade.exe",
            "code": STEP2,
            "timeoutSeconds": 120,
        },
        "timeout_seconds": 240,
    },
]

with open(
    "/Users/yuluo/Desktop/game/games_test/reverse-engineering/tiny-glade/"
    "tool-usage/jobs/tree-cmds/tree-cmds-reva.json",
    "w",
) as fh:
    json.dump(JOB, fh, indent=2)
    fh.write("\n")
print("written")
