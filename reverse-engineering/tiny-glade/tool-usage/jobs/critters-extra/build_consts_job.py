#!/usr/bin/env python3
# -*- coding: utf-8 -*-
"""Read float constants used by critters-extra systems from B DB."""
import json
import os

HERE = os.path.dirname(os.path.abspath(__file__))
OUT = os.path.join(HERE, "critters-extra-consts.json")

FLOATS = [
    "0x142a7c880",  # fish min speed A
    "0x1429cd078",  # fish min speed B
    "0x142934694",  # fish max speed A
    "0x142934604",  # fish max speed B
    "0x142a171f4",  # fish spawn vel base
    "0x1429a9088",
    "0x1429a9094",
    "0x1429a9098",
    "0x1429a90a4",
    "0x1429a90a8",
    "0x142aebb84",  # butterfly goal state const
    "0x142f67d14",
    "0x142f67d24",
    "0x142f67d44",
    "0x142f7d784",
    "0x142f7d788",
    "0x142f7d78c",
    "0x142f7d7a4",
    "0x142f7d7a8",
    "0x142f7d7ac",
    "0x1429345b8",  # wren landing quat
    "0x142badbc4",  # quat const fish/sim_wren
    "0x14304be94",  # wren spawn
    "0x1429cd0d8",
    "0x1429cd0dc",
    "0x142f7d7b4",
    "0x142f7d7b8",
    "0x142f7d7bc",
    "0x142f7d770",
]

code = '''
import json
prog = currentProgram
out = {}
for a in %r:
    try:
        addr = prog.getAddressFactory().getAddress(a)
        out[a + "_float"] = prog.getMemory().getFloat(addr)
        buf = bytearray(8)
        prog.getMemory().getBytes(addr, buf)
        out[a + "_hex"] = "".join("%%02x" %% (b & 0xFF) for b in buf)
    except Exception as e:
        out[a] = "ERR:" + str(e)[:80]
print(json.dumps(out))
''' % FLOATS

JOB = [
    {
        "tool": "run-script",
        "args": {"programPath": "/tiny-glade.exe", "code": code, "timeoutSeconds": 60},
        "timeout_seconds": 180,
    }
]

with open(OUT, "w") as fh:
    json.dump(JOB, fh, indent=2)
    fh.write("\n")
print("written", OUT)
