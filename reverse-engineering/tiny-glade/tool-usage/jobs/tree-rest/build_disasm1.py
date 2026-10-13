#!/usr/bin/env python3
# -*- coding: utf-8 -*-
"""Build reva job: full disassembly + call targets + decompiled C for the
tree-tool rest functions. One run-script step per target."""
import json

TARGETS = {
    "despawn_trees": "0x1422686d0",
    "tree_move_bend": "0x1422689c0",
    "recalc": "0x142268dc0",
}

DISM_CODE = r'''
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
body = f.getBody()
listing = prog.getListing()
it = listing.getInstructions(body, True)
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
    # data reads
    if mnem in ("MOV","LEA","MOVSD","MOVSS","MOVUPS","MOVDQU","CMP","TEST","MOVZX","MOVSX","CMPSS","COMISS","ADD","SUB","IMUL","MUL","XOR","VEXTRACTF128","MOVQ","CVTSS2SD","CVTSD2SS","UCOMISS"):
        for r in refmgr.getReferencesFrom(a):
            rt = r.getReferenceType()
            if rt.isData():
                to = r.getToAddress()
                data_refs.append((str(a), mnem, str(to), str(r.getReferenceType())))
out["insn_count"] = len(insns)
out["insns"] = insns
out["call_targets"] = call_targets
out["data_refs"] = data_refs
print(json.dumps(out))
'''

DECOMP_CODE = r'''
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

JOB = []
for name, va in TARGETS.items():
    JOB.append({
        "tool": "run-script",
        "args": {
            "programPath": "/tiny-glade.exe",
            "code": DISM_CODE.format(t=va),
            "timeoutSeconds": 120,
        },
        "timeout_seconds": 240,
    })
    JOB.append({
        "tool": "run-script",
        "args": {
            "programPath": "/tiny-glade.exe",
            "code": DECOMP_CODE.format(t=va),
            "timeoutSeconds": 120,
        },
        "timeout_seconds": 240,
    })

with open(
    "/Users/yuluo/Desktop/game/games_test/reverse-engineering/tiny-glade/"
    "tool-usage/jobs/tree-rest/disasm1.json",
    "w",
) as fh:
    json.dump(JOB, fh, indent=2)
    fh.write("\n")
print("written steps:", len(JOB))
