#!/usr/bin/env python3
# -*- coding: utf-8 -*-
"""tree-extra 3: decompile UI systems with callers; disasm reset run_unsafe."""
import json

JOB = []
for name, addr in [("ui_place_tree", "0x142269170"),
                   ("ui_move_tree", "0x14226b1a0"),
                   ("ui_delete_tree", "0x141bd2510"),
                   ("convert", "0x14226be70")]:
    JOB.append({
        "tool": "get-decompilation",
        "args": {
            "programPath": "/tiny-glade.exe",
            "functionNameOrAddress": addr,
            "offset": 1,
            "limit": 4000,
            "includeCallers": True,
        },
        "timeout_seconds": 300,
    })

RESET_CODE = r'''
import json
prog = currentProgram
fm = prog.getFunctionManager()
addr = prog.getAddressFactory().getAddress("0x141bb19d0")
f = fm.getFunctionContaining(addr)
if f is None:
    raise RuntimeError("no fn at 141bb19d0")
out = {"name": str(f.getName()), "entry": str(f.getEntryPoint()), "body": str(f.getBody())}
listing = prog.getListing()
it = listing.getInstructions(f.getBody(), True)
refmgr = prog.getReferenceManager()
insns = []
call_targets = {}
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
    if mnem in ("MOV", "LEA", "CMP", "TEST", "XOR", "MOVQ", "MOVSD", "MOVSS", "MOVUPS"):
        for r in refmgr.getReferencesFrom(a):
            rt = r.getReferenceType()
            if rt.isData():
                data_refs.append([str(a), mnem, str(to := r.getToAddress()), str(rt)])
out["insns"] = insns
out["call_targets"] = call_targets
out["data_refs"] = data_refs
with open("/tmp/tree-extra-reset_run_unsafe.json", "w") as fh:
    json.dump(out, fh)
print(json.dumps({"name": out["name"], "entry": out["entry"], "body": out["body"],
                  "insn_count": len(insns), "call_targets": call_targets,
                  "data_refs": data_refs}))
'''

JOB.append({
    "tool": "run-script",
    "args": {"programPath": "/tiny-glade.exe", "code": RESET_CODE, "timeoutSeconds": 180},
    "timeout_seconds": 300,
})

with open(
    "/Users/yuluo/Desktop/game/games_test/reverse-engineering/tiny-glade/"
    "tool-usage/jobs/tree-extra/decomp3.json",
    "w",
) as fh:
    json.dump(JOB, fh, indent=2)
    fh.write("\n")
print("written steps:", len(JOB))
