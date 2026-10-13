#!/usr/bin/env python3
# -*- coding: utf-8 -*-
"""tree-extra 2c (fast, no full scan): spp_* signatures, FUN_141bdb9d0,
function 0x141bcb900 disasm, region 0x141bb29d0 (reset inline host1)."""
import json

CODE = r'''
import json
prog = currentProgram
fm = prog.getFunctionManager()
listing = prog.getListing()
out = {}

def dumpfn(label, a):
    addr = prog.getAddressFactory().getAddress(a)
    f = fm.getFunctionAt(addr)
    if f is None:
        f = fm.getFunctionContaining(addr)
    if f is None:
        out[label] = None
        return
    out[label] = [str(f.getEntryPoint()), str(f.getBody()), str(f.getName())[:5000]]

for label, a in [("spp_initialize", "0x1409bdd50"), ("spp_run_unsafe", "0x1409bdfc0"),
                 ("spp_validate_param", "0x1409be420"), ("spp_run_if", "0x1409beac0"),
                 ("fun_d9d0", "0x141bdb9d0")]:
    dumpfn(label, a)

def walkrange(key, start, end):
    s = prog.getAddressFactory().getAddress(start)
    e = prog.getAddressFactory().getAddress(end)
    it = listing.getInstructions(s, False)
    rows = []
    while it.hasNext():
        monitor.checkCancelled()
        ins = it.next()
        a = ins.getAddress()
        if a.compareTo(e) > 0:
            break
        ops = [ins.getDefaultOperandRepresentation(i) if ins.getDefaultOperandRepresentation(i) is not None else "" for i in range(ins.getNumOperands())]
        raw = ins.getBytes()
        hexb = "".join("%02x" % (b & 0xFF) for b in raw)
        rows.append([str(a), ins.getMnemonicString(), ops, hexb])
    out[key] = rows

walkrange("fn_141bcb900", "0x141bcb900", "0x141bcc1ab")
walkrange("reset_host1_region", "0x141bb29d0", "0x141bb2c00")
walkrange("fun_d9d0_disasm", "0x141bdb9d0", "0x141bdba08")

with open("/tmp/tree-extra-2c.json", "w") as fh:
    json.dump(out, fh)
print(json.dumps({k: (len(v) if isinstance(v, list) else v) for k, v in out.items()}))
'''

JOB = [{
    "tool": "run-script",
    "args": {"programPath": "/tiny-glade.exe", "code": CODE, "timeoutSeconds": 240},
    "timeout_seconds": 400,
}]

with open(
    "/Users/yuluo/Desktop/game/games_test/reverse-engineering/tiny-glade/"
    "tool-usage/jobs/tree-extra/disasm2c.json",
    "w",
) as fh:
    json.dump(JOB, fh, indent=2)
    fh.write("\n")
print("written")
