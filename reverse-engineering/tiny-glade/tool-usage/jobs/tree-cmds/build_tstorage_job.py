#!/usr/bin/env python3
# -*- coding: utf-8 -*-
"""Build reva job: disassemble TreeStorage::{get,insert,remove} + read log strings."""
import json

FUNCS = [
    ("TreeStorage_get", "0x140a38460"),
    ("TreeStorage_insert", "0x140a38560"),
    ("TreeStorage_remove", "0x140a385a0"),
]

steps = []
for tag, addr in FUNCS:
    steps.append({
        "tool": "run-script",
        "args": {
            "programPath": "/tiny-glade.exe",
            "code": (
                "import json\n"
                "prog = currentProgram\n"
                'out = {"sha256": str(prog.getExecutableSHA256())}\n'
                f'addr = prog.getAddressFactory().getAddress("{addr}")\n'
                "fm = prog.getFunctionManager()\n"
                "f = fm.getFunctionAt(addr)\n"
                f'out["tag"] = "{tag}"\n'
                "if f is None:\n"
                f'    raise RuntimeError("function absent at {addr}")\n'
                'out["name"] = str(f.getName())\n'
                'out["entry"] = str(f.getEntryPoint())\n'
                'out["size"] = str(f.getBody())\n'
                'out["signature"] = str(f.getSignature())\n'
                "it = prog.getListing().getInstructions(f.getBody(), True)\n"
                "insns = []\n"
                "while it.hasNext():\n"
                "    monitor.checkCancelled()\n"
                "    ins = it.next()\n"
                "    ops = []\n"
                "    for i in range(ins.getNumOperands()):\n"
                "        r = ins.getDefaultOperandRepresentation(i)\n"
                "        ops.append(r if r is not None else \"\")\n"
                "    raw = ins.getBytes()\n"
                '    hexb = "".join("%02x" % (b & 0xFF) for b in raw)\n'
                "    insns.append([str(ins.getAddress()), ins.getMnemonicString(), ops, hexb])\n"
                'out["insns"] = insns\n'
                "print(json.dumps(out))\n"
            ),
            "timeoutSeconds": 90,
        },
        "timeout_seconds": 180,
    })

# read log strings from .data
steps.append({
    "tool": "run-script",
    "args": {
        "programPath": "/tiny-glade.exe",
        "code": (
            "import json\n"
            "prog = currentProgram\n"
            "out = {}\n"
            "for a in [\"0x143035018\", \"0x14303504e8\", \"0x143035000\", \"0x143035030\"]:\n"
            "    addr = prog.getAddressFactory().getAddress(a)\n"
            "    mem = prog.getMemory().getBytes(addr, 0x60)\n"
            '    out[a] = "".join("%02x" % (b & 0xFF) for b in mem)\n'
            "print(json.dumps(out))\n"
        ),
        "timeoutSeconds": 60,
    },
    "timeout_seconds": 120,
})

with open(
    "/Users/yuluo/Desktop/game/games_test/reverse-engineering/tiny-glade/"
    "tool-usage/jobs/tree-cmds/tree-storage-methods.json", "w",
) as fh:
    json.dump(steps, fh, indent=2)
    fh.write("\n")
print("written tree-storage-methods job with", len(steps), "steps")
