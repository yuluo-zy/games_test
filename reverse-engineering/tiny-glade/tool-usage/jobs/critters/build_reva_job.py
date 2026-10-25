#!/usr/bin/env python3
# -*- coding: utf-8 -*-
"""Build reva job for critters behaviour core chain (B persistent DB).

Two run-script steps per function:
  stepA: identity + function info + signature + decompiled C
  stepB: full linear disassembly (instruction, operands, raw bytes) + call targets
Each step prints a single compact JSON line to stdout which the operator
saves to the result file.
"""
import json
import os

HERE = os.path.dirname(os.path.abspath(__file__))
OUT = os.path.join(HERE, "critters-reva.json")

FUNCS = [
    ("0x14230ff00", "plan_duck_trajectories"),
    ("0x142312200", "move_ducks_along_trajectories"),
    ("0x142313b30", "update_duck_goals"),
    ("0x142314470", "set_duck_velocities_towards_goals"),
    ("0x142314b70", "update_duck_ripples"),
    ("0x1423179a0", "critters_avoid_collisions"),
    ("0x142318c10", "smooth_critter_velocities"),
    ("0x142318fa0", "move_critters"),
    ("0x14231bd80", "sheep_walk_on_terrain"),
    ("0x142315050", "update_sheep_goals"),
]

BASE_IMPORTS = (
    "import json\n"
    "from ghidra.app.decompiler import DecompInterface\n"
)


def step_decompile(addr, name):
    code = BASE_IMPORTS + f'''
prog = currentProgram
out = {{"sha256": str(prog.getExecutableSHA256()), "program": str(prog.getName())}}
addr = prog.getAddressFactory().getAddress("{addr}")
fm = prog.getFunctionManager()
f = fm.getFunctionAt(addr)
if f is None:
    raise RuntimeError("function absent at " + "{addr}")
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
    return {
        "tool": "run-script",
        "args": {
            "programPath": "/tiny-glade.exe",
            "code": code,
            "timeoutSeconds": 90,
        },
        "timeout_seconds": 180,
    }


def step_disasm(addr, name):
    code = BASE_IMPORTS + f'''
prog = currentProgram
out = {{"sha256": str(prog.getExecutableSHA256())}}
addr = prog.getAddressFactory().getAddress("{addr}")
fm = prog.getFunctionManager()
f = fm.getFunctionAt(addr)
if f is None:
    raise RuntimeError("function absent at " + "{addr}")
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
                call_targets.setdefault(nm, str(to))
out["insn_count"] = len(insns)
out["insns"] = insns
out["call_targets"] = call_targets
print(json.dumps(out))
'''
    return {
        "tool": "run-script",
        "args": {
            "programPath": "/tiny-glade.exe",
            "code": code,
            "timeoutSeconds": 120,
        },
        "timeout_seconds": 300,
    }


JOB = []
for addr, name in FUNCS:
    JOB.append(step_decompile(addr, name))
    JOB.append(step_disasm(addr, name))

with open(OUT, "w") as fh:
    json.dump(JOB, fh, indent=2)
    fh.write("\n")
print("written", OUT, "steps:", len(JOB))
