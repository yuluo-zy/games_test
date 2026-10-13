#!/usr/bin/env python3
# -*- coding: utf-8 -*-
"""replay_tree full disasm + decompile, plus decompile of the shared helper
functions (time_continuous_lerp, position, sample_at_world_xz)."""
import json

DISM = r'''
import json
prog = currentProgram
out = {{"sha256": str(prog.getExecutableSHA256())}}
addr = prog.getAddressFactory().getAddress("{t}")
fm = prog.getFunctionManager()
f = fm.getFunctionAt(addr)
if f is None:
    raise RuntimeError("function absent at " + "{t}")
out["name"] = str(f.getName())
out["entry"] = str(f.getEntryPoint())
out["body"] = str(f.getBody())
out["signature"] = str(f.getSignature())
it = prog.getListing().getInstructions(f.getBody(), True)
insns = []
refmgr = prog.getReferenceManager()
call_targets = {{}}
data_refs = []
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
    if mnem in ("MOV","LEA","MOVSD","MOVSS","MOVUPS","MOVDQU","CMP","TEST","MOVZX","MOVSX","CMPSS","COMISS","ADD","SUB","IMUL","XOR","VEXTRACTF128","MOVQ","CVTSS2SD","CVTSD2SS","UCOMISS","SQRTSS"):
        for r in refmgr.getReferencesFrom(a):
            rt = r.getReferenceType()
            if rt.isData():
                data_refs.append((str(a), mnem, str(r.getToAddress()), str(rt)))
out["insn_count"] = len(insns)
out["insns"] = insns
out["call_targets"] = call_targets
out["data_refs"] = data_refs
print(json.dumps(out))
'''

DECOMP = r'''
import json
from ghidra.app.decompiler import DecompInterface
prog = currentProgram
out = {{"sha256": str(prog.getExecutableSHA256())}}
addr = prog.getAddressFactory().getAddress("{t}")
fm = prog.getFunctionManager()
f = fm.getFunctionAt(addr)
if f is None:
    raise RuntimeError("function absent at " + "{t}")
out["name"] = str(f.getName())
out["entry"] = str(f.getEntryPoint())
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
        "args": {"programPath": "/tiny-glade.exe", "code": DISM.format(t="0x141bc34f0"), "timeoutSeconds": 180},
        "timeout_seconds": 300,
    },
    {
        "tool": "run-script",
        "args": {"programPath": "/tiny-glade.exe", "code": DECOMP.format(t="0x141bc34f0"), "timeoutSeconds": 180},
        "timeout_seconds": 300,
    },
]
# helper functions via get-decompilation (shared tool, no script)
for name, va in [
    ("time_continuous_lerp", "0x140c904e0"),
    ("position", "0x140ad51a0"),
    ("sample_at_world_xz_TerrainHeightSsbo", "0x141246fb0"),
]:
    JOB.append({
        "tool": "get-decompilation",
        "args": {
            "programPath": "/tiny-glade.exe",
            "functionNameOrAddress": va,
            "offset": 1,
            "limit": 200,
            "includeIncomingReferences": False,
            "includeReferenceContext": False,
        },
        "timeout_seconds": 240,
    })

with open(
    "/Users/yuluo/Desktop/game/games_test/reverse-engineering/tiny-glade/"
    "tool-usage/jobs/tree-rest/replay.json",
    "w",
) as fh:
    json.dump(JOB, fh, indent=2)
    fh.write("\n")
print("written steps:", len(JOB))
