#!/usr/bin/env python3
# -*- coding: utf-8 -*-
"""Build reva run-script jobs for system-water CPU chain analysis.

Each run-script step writes its JSON output to
/Users/yuluo/ghidra_scripts/water-owner/<name>.json and prints only a
compact summary, avoiding the ReVa server ~65KB stdout truncation.
"""
import json

BASE = (
    "import json, os\n"
    "from ghidra.app.decompiler import DecompInterface\n"
)

OUTDIR = "/Users/yuluo/ghidra_scripts/water-owner"

# name -> (va string)
FUNCS = {
    "startup_water": "0x1408f1a20",
    "water_rasterize_from_cpu": "0x140a12680",
    "rasterize_stroke_512": "0x142261d10",
    "copy_to_staging_512": "0x142261760",
    "copy_composite_to_scratch_512": "0x142262680",
    "commit_scratch_to_composite_512": "0x142262a60",
    "replay_raster_water": "0x1409f4650",
    "generate_replay_cmds_replay_water": "0x141444c70",
    "replay_water_closure2": "0x140a01560",
    "bilinear_sample_rasterbufref": "0x140a14df0",
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
path = os.path.join("{outdir}", out["name"] + ".json")
with open(path, "w") as fh:
    json.dump(out, fh)
print(json.dumps({{"name": out["name"], "entry": out["entry"], "size": out["size"],
                   "insn_count": len(insns), "calls": len(call_targets),
                   "mem_refs": len(mem_refs), "file": path,
                   "decompile_error": out.get("decompile_error")}}))
'''

JOBS = {
    "reva-startup-raster.json": ["startup_water", "water_rasterize_from_cpu", "rasterize_stroke_512",
                                  "copy_to_staging_512", "copy_composite_to_scratch_512",
                                  "commit_scratch_to_composite_512"],
    "reva-replay.json": ["replay_raster_water", "generate_replay_cmds_replay_water",
                         "replay_water_closure2", "bilinear_sample_rasterbufref"],
}


def build(name, keys):
    job = []
    for key in keys:
        code = SCRIPT.format(va=FUNCS[key], outdir=OUTDIR, name=key)
        job.append({
            "tool": "run-script",
            "args": {
                "programPath": "/tiny-glade.exe",
                "code": code,
                "timeoutSeconds": 300,
            },
            "timeout_seconds": 600,
        })
    path = ("/Users/yuluo/Desktop/game/games_test/reverse-engineering/tiny-glade/"
            "tool-usage/jobs/water-cpu/" + name)
    with open(path, "w") as fh:
        json.dump(job, fh, indent=2)
        fh.write("\n")
    print("written", path, len(job), "steps")


for name, keys in JOBS.items():
    build(name, keys)
