#!/usr/bin/env python3
# -*- coding: utf-8 -*-
"""tree-extra batch 2b: signatures of SpawnPreplacedTrees-consuming system
family, single-param run_unsafe (reset_tree_tool), FUN_141bdb9d0 body, and
the call region around 0x141bcb978."""
import json

CODE = r'''
import json
prog = currentProgram
fm = prog.getFunctionManager()
out = {}

def dumpfn(label, a):
    addr = prog.getAddressFactory().getAddress(a)
    f = fm.getFunctionAt(addr)
    if f is None:
        f = fm.getFunctionContaining(addr)
    if f is None:
        out[label] = None
        return
    out[label] = [str(f.getEntryPoint()), str(f.getBody()), str(f.getName())[:5000],
                  (str(f.getSignature()) if hasattr(f, "getSignature") else "")]

for label, a in [("spp_initialize", "0x1409bdd50"), ("spp_run_unsafe", "0x1409bdfc0"),
                 ("spp_validate_param", "0x1409be420"), ("spp_run_if", "0x1409beac0"),
                 ("fun_d9d0", "0x141bdb9d0")]:
    dumpfn(label, a)

# scan for single-param ResMut<TreeStorage> FunctionSystem (reset_tree_tool)
NEEDLE = "void_(*)(bevy_ecs::change_detection::ResMut<country_core::resources::tree_storage::TreeStorage>),void_(*)(bevy_ecs::change_detection::ResMut<country_core::resources::tree_storage::TreeStorage>)"
reset_found = []
it = fm.getFunctions(True)
for fn in it:
    monitor.checkCancelled()
    nm = fn.getName()
    if NEEDLE in nm:
        reset_found.append([str(fn.getEntryPoint()), str(fn.getBody()), nm[:3000]])
out["reset_found"] = reset_found

# disassemble 0x141bcb900 call region around 0x141bcb978 (use listing walk)
region = []
start = prog.getAddressFactory().getAddress("0x141bcb900")
end = prog.getAddressFactory().getAddress("0x141bcc1ab")
listing = prog.getListing()
it = listing.getInstructions(start, False)
while it.hasNext():
    monitor.checkCancelled()
    ins = it.next()
    a = ins.getAddress()
    if a.compareTo(end) >= 0:
        break
    ops = [ins.getDefaultOperandRepresentation(i) if ins.getDefaultOperandRepresentation(i) is not None else "" for i in range(ins.getNumOperands())]
    raw = ins.getBytes()
    hexb = "".join("%02x" % (b & 0xFF) for b in raw)
    region.append([str(a), ins.getMnemonicString(), ops, hexb])
out["region_141bcb900"] = region

with open("/tmp/tree-extra-2b.json", "w") as fh:
    json.dump(out, fh)
print(json.dumps({k: (len(v) if isinstance(v, list) else v) for k, v in out.items()}))
'''

JOB = [{
    "tool": "run-script",
    "args": {"programPath": "/tiny-glade.exe", "code": CODE, "timeoutSeconds": 300},
    "timeout_seconds": 600,
}]

with open(
    "/Users/yuluo/Desktop/game/games_test/reverse-engineering/tiny-glade/"
    "tool-usage/jobs/tree-extra/disasm2b.json",
    "w",
) as fh:
    json.dump(JOB, fh, indent=2)
    fh.write("\n")
print("written")
