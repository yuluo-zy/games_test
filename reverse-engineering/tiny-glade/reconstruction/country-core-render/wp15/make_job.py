#!/usr/bin/env python3
"""Create a ghidra get_functions job JSON from an address list (read-only)."""
import json, sys

out = sys.argv[1]
addrs = sys.argv[2:]
step = {
    "tool": "get_functions",
    "args": {
        "program": "tiny-glade.exe",
        "functions": ",".join(addrs),
        "fields": "signature,decompiled_code,callees",
        "include_call_context": False,
    },
    "timeout_seconds": 180,
}
with open(out, "w") as f:
    json.dump([step], f)
print(f"wrote {len(addrs)} functions -> {out}")
