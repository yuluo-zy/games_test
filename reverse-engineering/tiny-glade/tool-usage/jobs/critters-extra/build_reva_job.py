#!/usr/bin/env python3
# -*- coding: utf-8 -*-
"""Build reva job for critters-extra (B persistent DB, worker critter-extra-owner).

One run-script step per function for decompiled C, plus chunked/full linear
disassembly. Each step prints a single compact JSON line to stdout which the
operator saves to the result file.
"""
import json
import os

HERE = os.path.dirname(os.path.abspath(__file__))
OUT = os.path.join(HERE, "critters-extra-reva.json")

# (addr, size, name) from critters-inventory.json (functions_b)
FUNCS = [
    ("0x142326a80", 19140, "sim_wren_boids"),
    ("0x141dc42f0", 412, "wren_boids_closure3"),
    ("0x141dc3510", 3548, "Boid_frame_update_wren"),
    ("0x141dc2540", 503, "BoidFlock_remove_wren"),
    ("0x141dc2170", 456, "Boid_random_vel_change"),
    ("0x140f90cd0", 7533, "fish"),
    ("0x140f909f0", 533, "FishSpawnLocations_new"),
    ("0x140f90c10", 191, "despawn_fish"),
    ("0x141dc2740", 3528, "Boid_frame_update_fish"),
    ("0x141dc2340", 503, "BoidFlock_remove_fish"),
    ("0x141d84d40", 4894, "update_butterfly_states"),
    ("0x141d86060", 1861, "update_butterfly_goals"),
    ("0x141d867b0", 1780, "set_butterfly_velocities_towards_goals"),
    ("0x141d86eb0", 642, "update_butterfly_audio"),
    ("0x142316940", 1779, "update_firefly_goals"),
    ("0x142317040", 1336, "set_firefly_velocities_towards_goals"),
    ("0x140f8ee90", 885, "DuckSpawnLocations_new"),
    ("0x140f8f210", 1694, "spawn_ducks"),
    ("0x140f8f8b0", 4280, "despawn_ducks"),
    ("0x1409d49a0", 5361, "startup_critters"),
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
        "args": {"programPath": "/tiny-glade.exe", "code": code, "timeoutSeconds": 120},
        "timeout_seconds": 300,
    }


def step_disasm(addr, size, name, start_va, end_va, win_idx):
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
        "args": {"programPath": "/tiny-glade.exe", "code": code, "timeoutSeconds": 90},
        "timeout_seconds": 240,
    }


WIN = 0x800
CHUNK_THRESHOLD = 2500


def disasm_steps(addr, size, name):
    base = int(addr, 16)
    if size <= CHUNK_THRESHOLD:
        return [step_disasm(addr, size, name, addr, hex(base + size), 0)]
    steps = []
    off = 0
    idx = 0
    while off < size:
        s = base + off
        e = min(base + off + WIN, base + size)
        steps.append(step_disasm(addr, size, name, hex(s), hex(e), idx))
        off += WIN
        idx += 1
    return steps


JOB = []
for addr, size, name in FUNCS:
    JOB.append(step_decompile(addr, name))
    JOB.extend(disasm_steps(addr, size, name))

with open(OUT, "w") as fh:
    json.dump(JOB, fh, indent=2)
    fh.write("\n")
print("written", OUT, "steps:", len(JOB))
