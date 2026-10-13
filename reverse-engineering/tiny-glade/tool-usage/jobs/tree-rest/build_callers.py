#!/usr/bin/env python3
# -*- coding: utf-8 -*-
"""Get names of the bevy wrapper callers of the target systems."""
import json

ADDRS = {
    "despawn_caller1": "0x141bcc540",
    "despawn_caller2": "0x141bd92e0",
    "replay_caller1": "0x140e9c870",
    "replay_caller2": "0x141bcd720",
    "recalc_caller1": "0x141bcc6d0",
    "recalc_caller2": "0x141bda800",
    "tmb_caller1": "0x141bcc3b0",
    "tmb_caller2": "0x141bda0c0",
    "convert_caller1": "0x141bcb200",
    "convert_caller2": "0x141bdb260",
    "despawn_wrapper": "0x141bb2110",
}

CODE = r'''
import json
prog = currentProgram
fm = prog.getFunctionManager()
out = {}
for key, a in __ADDRS__.items():
    addr = prog.getAddressFactory().getAddress(a)
    f = fm.getFunctionContaining(addr)
    if f is None:
        out[key] = None
        continue
    out[key] = [str(f.getEntryPoint()), str(f.getBody()), str(f.getName())[:4000]]
with open("/tmp/tree-rest-callers.json", "w") as fh:
    json.dump(out, fh)
print(json.dumps({k: (v[0] if v else None) for k, v in out.items()}))
'''

JOB = [
    {
        "tool": "run-script",
        "args": {
            "programPath": "/tiny-glade.exe",
            "code": CODE.replace("__ADDRS__", json.dumps(ADDRS)),
            "timeoutSeconds": 120,
        },
        "timeout_seconds": 240,
    }
]

with open(
    "/Users/yuluo/Desktop/game/games_test/reverse-engineering/tiny-glade/"
    "tool-usage/jobs/tree-rest/callers.json",
    "w",
) as fh:
    json.dump(JOB, fh, indent=2)
    fh.write("\n")
print("written")
