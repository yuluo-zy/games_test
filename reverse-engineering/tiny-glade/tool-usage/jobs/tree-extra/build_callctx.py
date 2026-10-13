#!/usr/bin/env python3
# -*- coding: utf-8 -*-
"""Disassemble call-site context around convert calls (0x141bca978,
0x141bda9d4) to observe the actual argument setup."""
import json

CODE = r'''
import json
prog = currentProgram
fm = prog.getFunctionManager()
listing = prog.getListing()
out = {}

def disasm(key, entry, calladdr, before=70, after=25):
    e = prog.getAddressFactory().getAddress(entry)
    ca = prog.getAddressFactory().getAddress(calladdr)
    f = fm.getFunctionContaining(e)
    if f is None:
        f = fm.getFunctionAt(e)
    body = f.getBody()
    rows = []
    it = listing.getInstructions(body, True)
    seq = []
    while it.hasNext():
        monitor.checkCancelled()
        ins = it.next()
        seq.append(ins)
    idx = None
    for i, ins in enumerate(seq):
        if ins.getAddress().equals(ca):
            idx = i
            break
    start = max(0, idx - before)
    end = min(len(seq), idx + after)
    for i in range(start, end):
        ins = seq[i]
        a = ins.getAddress()
        ops = [ins.getDefaultOperandRepresentation(j) if ins.getDefaultOperandRepresentation(j) is not None else "" for j in range(ins.getNumOperands())]
        raw = ins.getBytes()
        hexb = "".join("%02x" % (b & 0xFF) for b in raw)
        mark = " >>>" if i == idx else ""
        rows.append([str(a), ins.getMnemonicString(), ops, hexb, mark])
    out[key] = rows
    return str(f.getName())

out["meta1"] = disasm("ctx1", "0x141bca200", "0x141bca978")
out["meta2"] = disasm("ctx2", "0x141bda260", "0x141bda9d4")

with open("/tmp/tree-extra-callctx.json", "w") as fh:
    json.dump(out, fh)
print(json.dumps({"meta1": out["meta1"], "meta2": out["meta2"],
                  "len1": len(out["ctx1"]), "len2": len(out["ctx2"])}))
'''

JOB = [{
    "tool": "run-script",
    "args": {"programPath": "/tiny-glade.exe", "code": CODE, "timeoutSeconds": 180},
    "timeout_seconds": 300,
}]

with open(
    "/Users/yuluo/Desktop/game/games_test/reverse-engineering/tiny-glade/"
    "tool-usage/jobs/tree-extra/callctx.json",
    "w",
) as fh:
    json.dump(JOB, fh, indent=2)
    fh.write("\n")
print("written")
