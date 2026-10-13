#!/usr/bin/env python3
# -*- coding: utf-8 -*-
"""Find bevy system wrappers for the tree UI systems + reset_tree_tool +
the real callers of convert_preplaced_trees (instruction addresses beyond the
alias-group wrapper bodies). Read-only scan on tree-extra-owner worker."""
import json

CODE = r'''
import json
prog = currentProgram
fm = prog.getFunctionManager()
out = {}

# 1) containing functions of the two real call instructions of convert
for label, a in [("call_contain_978", "0x141bcb978"),
                 ("call_contain_d9d4", "0x141bdb9d4"),
                 ("call_contain_200", "0x141bcb200"),
                 ("call_contain_b250", "0x141bdb250")]:
    addr = prog.getAddressFactory().getAddress(a)
    f = fm.getFunctionContaining(addr)
    if f is None:
        out[label] = None
        continue
    out[label] = [str(f.getEntryPoint()), str(f.getBody()), str(f.getName())[:3000]]

# 2) scan functions by keyword
KEYS = ["SpawnPreplacedTrees", "ui_place_tree", "ui_move_tree", "ui_delete_tree",
        "PlaceTreeCmd", "reset_tree_tool", "startup_tree_tool"]
for key in KEYS:
    found = []
    it = fm.getFunctions(True)
    for fn in it:
        monitor.checkCancelled()
        nm = fn.getName()
        if key in nm:
            found.append([str(fn.getEntryPoint()), str(fn.getBody()), nm[:2200]])
    out["key_"+key] = found

with open("/tmp/tree-extra-wrappers.json", "w") as fh:
    json.dump(out, fh)
print(json.dumps({k: (v if isinstance(v, list) else v) for k, v in out.items()}, indent=1))
'''

JOB = [{
    "tool": "run-script",
    "args": {"programPath": "/tiny-glade.exe", "code": CODE, "timeoutSeconds": 300},
    "timeout_seconds": 600,
}]

with open(
    "/Users/yuluo/Desktop/game/games_test/reverse-engineering/tiny-glade/"
    "tool-usage/jobs/tree-extra/find-wrappers.json",
    "w",
) as fh:
    json.dump(JOB, fh, indent=2)
    fh.write("\n")
print("written")
