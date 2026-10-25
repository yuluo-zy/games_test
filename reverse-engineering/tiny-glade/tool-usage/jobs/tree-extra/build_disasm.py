#!/usr/bin/env python3
# -*- coding: utf-8 -*-
"""tree-extra batch 2: full disassembly of convert_preplaced_trees,
ui_place_tree/place_tree/random_tree_prefab, ui_move_tree, ui_delete_tree,
reset_tree_tool inline hosts, and the convert/place system wrappers.

ReVa worker tree-extra-owner (read-only script; dedicated --write profile
required per AI_REVERSE_TOOLS.md). Writes full disasm JSON to /tmp and
prints a compact summary.
"""
import json

TARGETS = {
    "convert": "0x14226be70",
    "ui_place_tree": "0x142269170",
    "place_tree": "0x142269970",
    "random_tree_prefab": "0x14226a170",
    "ui_move_tree": "0x14226b1a0",
    "ui_delete_tree": "0x141bd2510",
    "reset_host1": "0x141bb29d0",
    "reset_host2": "0x141bcbc10",
    "convert_wrapper1": "0x141bcaeb0",
    "convert_wrapper2": "0x141bdb250",
}

DISM_CODE = r'''
import json
prog = currentProgram
out = {"sha256": str(prog.getExecutableSHA256())}
_ADDR = "@T@"
_KEY = "@KEY@"
addr = prog.getAddressFactory().getAddress(_ADDR)
fm = prog.getFunctionManager()
f = fm.getFunctionContaining(addr)
if f is None:
    raise RuntimeError("no function containing " + _ADDR)
out["name"] = str(f.getName())
out["entry"] = str(f.getEntryPoint())
out["body"] = str(f.getBody())
try:
    out["signature"] = str(f.getSignature())
except Exception:
    out["signature"] = None
body = f.getBody()
listing = prog.getListing()
it = listing.getInstructions(body, True)
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
    if mnem in ("MOV", "LEA", "MOVSD", "MOVSS", "MOVUPS", "MOVDQU", "CMP",
                "TEST", "MOVZX", "MOVSX", "CMPSS", "COMISS", "ADD", "SUB",
                "IMUL", "MUL", "XOR", "MOVQ", "CVTSS2SD", "CVTSD2SS",
                "UCOMISS", "MOVD", "MOVQ", "VBROADCASTSS"):
        for r in refmgr.getReferencesFrom(a):
            rt = r.getReferenceType()
            if rt.isData():
                to = r.getToAddress()
                data_refs.append([str(a), mnem, str(to), str(rt)])
out["insn_count"] = len(insns)
out["insns"] = insns
out["call_targets"] = call_targets
out["data_refs"] = data_refs
with open("/tmp/tree-extra-" + _KEY + ".json", "w") as fh:
    json.dump(out, fh)
print(json.dumps({
    "sha256": out["sha256"],
    "name": out["name"],
    "entry": out["entry"],
    "body": out["body"],
    "insn_count": out["insn_count"],
    "call_targets": out["call_targets"],
    "data_refs": out["data_refs"],
}))
'''

JOB = []
for key, va in TARGETS.items():
    JOB.append({
        "tool": "run-script",
        "args": {
            "programPath": "/tiny-glade.exe",
            "code": DISM_CODE.replace("@T@", va).replace("@KEY@", key),
            "timeoutSeconds": 180,
        },
        "timeout_seconds": 300,
    })

with open(
    "/Users/yuluo/Desktop/game/games_test/reverse-engineering/tiny-glade/"
    "tool-usage/jobs/tree-extra/disasm.json",
    "w",
) as fh:
    json.dump(JOB, fh, indent=2)
    fh.write("\n")
print("written steps:", len(JOB))
