#!/usr/bin/env python3
# -*- coding: utf-8 -*-
"""Build reva run-script job #2 for system-water: blend closures, replay
chain, bilinear sample. Writes JSON to /Users/yuluo/ghidra_scripts/water-owner/."""
import json

BASE = (
    "import json, os\n"
    "from ghidra.app.decompiler import DecompInterface\n"
)
OUTDIR = "/Users/yuluo/ghidra_scripts/water-owner"

FUNCS = {
    "blend_closure0": "0x1425dcda0",
    "blend_closure1": "0x1425dc570",
    "blend_closure2": "0x1425dbd00",
    "retain_copytostaging": "0x1425da150",
    "publish_dirtytiles": "0x1425db920",
    "replay_raster_water": "0x1409f4650",
    "generate_replay_cmds": "0x141444c70",
    "replay_water_closure2": "0x140a01560",
    "bilinear_sample": "0x140a14df0",
}

SCRIPT = BASE + '''
prog = currentProgram
out = {{"sha256": str(prog.getExecutableSHA256()), "program": str(prog.getName())}}
addr = prog.getAddressFactory().getAddress("{va}")
fm = prog.getFunctionManager()
f = fm.getFunctionAt(addr)
if f is None:
    raise RuntimeError("function absent at " + "{va}")
out["name"] = str(f.getName())
out["entry"] = str(f.getEntryPoint())
out["body"] = str(f.getBody())
out["signature"] = str(f.getSignature())
out["size"] = f.getBody().getNumAddresses() if f.getBody().contains(addr) else -1
monitor.checkCancelled()
d = DecompInterface()
try:
    d.openProgram(prog)
    res = d.decompileFunction(f, 120, monitor)
    if not res.decompileCompleted():
        out["decompile_error"] = str(res.getErrorMessage())
        out["decompiled"] = None
    else:
        out["decompiled"] = res.getDecompiledFunction().getC()
finally:
    d.dispose()
monitor.checkCancelled()
body = f.getBody()
listing = prog.getListing()
it = listing.getInstructions(body, True)
insns = []
refmgr = prog.getReferenceManager()
call_targets = {{}}
mem_refs = []
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
    if mnem in ("LEA", "MOV", "MOVSD", "MOVSS", "CMP", "TEST"):
        for i in range(ins.getNumOperands()):
            refs = refmgr.getReferencesFrom(a, i)
            for r in refs:
                rt = r.getReferenceType()
                to = r.getToAddress()
                if str(to)[0:3] in ("140", "141", "142", "143"):
                    target = fm.getFunctionAt(to)
                    tname = str(target.getName()) if target else ""
                    mem_refs.append([str(a), str(to), str(rt), tname])
out["insn_count"] = len(insns)
out["insns"] = insns
out["call_targets"] = call_targets
out["mem_refs"] = mem_refs
path = os.path.join("{outdir}", "{key}" + ".json")
with open(path, "w") as fh:
    json.dump(out, fh)
print(json.dumps({{"name": out["name"], "entry": out["entry"], "size": out["size"],
                   "insn_count": len(insns), "calls": len(call_targets),
                   "mem_refs": len(mem_refs), "file": path,
                   "decompile_error": out.get("decompile_error")}}))
'''

# Also add a caller-discovery step for the world_raster chain entry points.
CALLER_SCRIPT = '''
prog = currentProgram
out = {{"target": "{va}", "callers": []}}
addr = prog.getAddressFactory().getAddress("{va}")
fm = prog.getFunctionManager()
f = fm.getFunctionAt(addr)
if f is None:
    raise RuntimeError("function absent at " + "{va}")
refmgr = prog.getReferenceManager()
refs = refmgr.getReferencesTo(addr)
seen = set()
for r in refs:
    monitor.checkCancelled()
    if not r.getReferenceType().isCall():
        continue
    frm = r.getFromAddress()
    if frm in seen:
        continue
    seen.add(frm)
    caller = fm.getFunctionContaining(frm)
    cname = str(caller.getName()) if caller else ""
    caddr = str(caller.getEntryPoint()) if caller else str(frm)
    out["callers"].append([str(frm), caddr, cname])
print(json.dumps(out))
'''

JOB = []
for key, va in FUNCS.items():
    JOB.append({
        "tool": "run-script",
        "args": {"programPath": "/tiny-glade.exe",
                 "code": SCRIPT.format(va=va, outdir=OUTDIR, key=key),
                 "timeoutSeconds": 300},
        "timeout_seconds": 600,
    })
for va in ("0x142261d10", "0x142261760", "0x142262a60", "0x142262680"):
    JOB.append({
        "tool": "run-script",
        "args": {"programPath": "/tiny-glade.exe",
                 "code": CALLER_SCRIPT.format(va=va),
                 "timeoutSeconds": 60},
        "timeout_seconds": 120,
    })

path = ("/Users/yuluo/Desktop/game/games_test/reverse-engineering/tiny-glade/"
        "tool-usage/jobs/water-cpu/reva-chain-replay.json")
with open(path, "w") as fh:
    json.dump(JOB, fh, indent=2)
    fh.write("\n")
print("written", path, len(JOB), "steps")
