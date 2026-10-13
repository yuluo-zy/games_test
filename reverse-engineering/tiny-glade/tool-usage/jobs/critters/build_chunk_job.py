#!/usr/bin/env python3
# -*- coding: utf-8 -*-
"""Build chunked reva disassembly job for the 3 large functions whose stdout
was truncated at 65536 bytes. Each window covers 0x800 bytes of the PDB body.
"""
import json
import os

HERE = os.path.dirname(os.path.abspath(__file__))
OUT = os.path.join(HERE, "critters-reva-chunks.json")

# (entry, pdb_size, name)
BIG = [
    ("0x14230ff00", 8958, "plan_duck_trajectories"),
    ("0x142312200", 6447, "move_ducks_along_trajectories"),
    ("0x142318fa0", 5774, "move_critters"),
]

WIN = 0x800


def chunk_step(start_va, end_va, name, win_idx):
    code = f'''
import json
prog = currentProgram
out = {{"sha256": str(prog.getExecutableSHA256())}}
start = prog.getAddressFactory().getAddress("{start_va}")
end = prog.getAddressFactory().getAddress("{end_va}")
listing = prog.getListing()
it = listing.getInstructions(start, True)
refmgr = prog.getReferenceManager()
fm = prog.getFunctionManager()
insns = []
call_targets = {{}}
count = 0
while it.hasNext():
    monitor.checkCancelled()
    ins = it.next()
    a = ins.getAddress()
    if a >= end:
        break
    mnem = ins.getMnemonicString()
    ops = []
    for i in range(ins.getNumOperands()):
        r = ins.getDefaultOperandRepresentation(i)
        ops.append(r if r is not None else "")
    raw = ins.getBytes()
    hexb = "".join("%02x" % (b & 0xFF) for b in raw)
    insns.append([str(a), mnem, ops, hexb])
    count += 1
    if mnem == "CALL":
        for r in refmgr.getReferencesFrom(a):
            if r.getReferenceType().isCall():
                to = r.getToAddress()
                tgt = fm.getFunctionAt(to)
                nm = str(tgt.getName()) if tgt else str(to)
                call_targets.setdefault(nm, str(to))
out["name"] = "{name}"
out["win"] = {win_idx}
out["range"] = ["{start_va}", "{end_va}"]
out["insn_count"] = count
out["insns"] = insns
out["call_targets"] = call_targets
print(json.dumps(out))
'''
    return {
        "tool": "run-script",
        "args": {
            "programPath": "/tiny-glade.exe",
            "code": code,
            "timeoutSeconds": 90,
        },
        "timeout_seconds": 240,
    }


JOB = []
for entry, size, name in BIG:
    base = int(entry, 16)
    off = 0
    idx = 0
    while off < size:
        s = base + off
        e = min(base + off + WIN, base + size)
        JOB.append(chunk_step(hex(s), hex(e), name, idx))
        off += WIN
        idx += 1

with open(OUT, "w") as fh:
    json.dump(JOB, fh, indent=2)
    fh.write("\n")
print("written", OUT, "steps:", len(JOB))
