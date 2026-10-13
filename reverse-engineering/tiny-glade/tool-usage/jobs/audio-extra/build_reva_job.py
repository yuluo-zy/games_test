#!/usr/bin/env python3
"""Build reva run-script job (worker audio-extra-owner) for remaining audio systems:
linear disasm + call targets + mem refs + strings. Read-only intent; writes JSON to worker dir."""
import json
BASE = (
    "import json, os\n"
    "from ghidra.app.decompiler import DecompInterface\n"
)
OUTDIR = "/Users/yuluo/ghidra_scripts/audio-extra-owner"
FUNCS = {
    "terrain_stairs_audio_system": "0x141462dc0",
    "water_ambience_audio_system": "0x1409ecea0",
    "water_ambience_audio_init": "0x1409ebf20",
    "menu_audio": "0x140a0b4c0",
    "menu_audio_init": "0x140a0b340",
    "sheep_audio": "0x1409d5ea0",
    "clutter_audio_system": "0x1408c02f0",
    "music_audio_system": "0x140a72810",
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
out["signature"] = str(f.getSignature())
monitor.checkCancelled()
d = DecompInterface()
try:
    d.openProgram(prog)
    res = d.decompileFunction(f, 180, monitor)
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
refmgr = prog.getReferenceManager()
insns = []
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
    if mnem in ("LEA", "MOV", "MOVSD", "MOVSS", "CMP", "TEST", "MULSS", "DIVSS"):
        for i in range(ins.getNumOperands()):
            refs = refmgr.getReferencesFrom(a, i)
            for r in refs:
                rt = r.getReferenceType()
                to = r.getToAddress()
                if str(to)[0:3] in ("140", "141", "142", "143"):
                    tname = ""
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
print(json.dumps({{"name": out["name"], "entry": out["entry"],
                   "insn_count": len(insns), "calls": len(call_targets),
                   "mem_refs": len(mem_refs), "file": path,
                   "decompile_error": out.get("decompile_error")}}))
'''
job = []
for key, va in FUNCS.items():
    code = SCRIPT.format(va=va, outdir=OUTDIR, name=key)
    job.append({"tool":"run-script",
                "args":{"programPath":"/tiny-glade.exe","code":code,"timeoutSeconds":300},
                "timeout_seconds":600})
out = "/Users/yuluo/Desktop/game/games_test/reverse-engineering/tiny-glade/tool-usage/jobs/audio-extra/reva-systems.json"
with open(out,"w") as fh: json.dump(job,fh,indent=2); fh.write("\n")
print("written",out,len(job),"steps")
