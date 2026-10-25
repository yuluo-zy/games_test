#!/usr/bin/env python3
"""Generate a ghidra get_functions batch job for the roof domain.
Usage: make_job.py <name> <va1> [va2 ...] > job.json
"""
import json, sys

name = sys.argv[1]
vas = sys.argv[2:]
job = [
    {
        "tool": "get_functions",
        "args": {
            "program": "tiny-glade.exe",
            "functions": ",".join(vas),
            "fields": "signature,decompiled_code,callees",
            "include_call_context": False,
        },
        "timeout_seconds": 240,
    }
]
with open(f"/Users/yuluo/Desktop/game/games_test/reverse-engineering/tiny-glade/reconstruction/country-core-render/wp9/evidence/{name}-job.json", "w") as f:
    json.dump(job, f, indent=1)
print(f"wrote {name}-job.json with {len(vas)} functions")
