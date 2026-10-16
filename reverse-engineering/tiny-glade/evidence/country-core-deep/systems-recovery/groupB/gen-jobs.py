#!/usr/bin/env python
# Generate RE-MCP decompile jobs for group B re-analyst (dbid tg_xB1)
# crates: critters / clutter / onboarding (business functions only, deduped by VA)
import json, io, os

ROOT = "D:/game/reverse-engineering/tiny-glade"
OUT = os.path.join(ROOT, "evidence/country-core-deep", "systems-recovery", "groupB")

with io.open(os.path.join(ROOT, "evidence/country-core-deep", "systems-recovery", "_groupB_sourcemap.json"), encoding="utf-8") as f:
    data = json.load(f)

seen, by_crate = set(), {"critters": [], "clutter": [], "onboarding": []}
for k, fd in data.items():
    crate = fd["crate"]
    if crate not in by_crate:
        continue
    for fn in fd["functions"]:
        if fn["kind"] != "business" or not fn["va"]:
            continue
        va = fn["va"][0]
        if va in seen:
            continue
        seen.add(va)
        by_crate[crate].append((va, fn["name"], fd["file"]))

os.makedirs(OUT, exist_ok=True)
for crate, fns in by_crate.items():
    steps = []
    for va, name, file in fns:
        steps.append({"tool": "decompile_function",
                      "args": {"database": "tg_xB1", "address": va},
                      "timeout_ms": 240000, "allow_error": True,
                      "_note": f"{file} :: {name[:110]}"})
    job = [
        {"tool": "open_database",
         "args": {"file_path": r"D:/game/reverse-engineering/tiny-glade/workspace/input/tiny-glade.exe",
                  "run_auto_analysis": False, "database_id": "tg_xB1", "keep_open": True},
         "timeout_ms": 600000},
        {"tool": "wait_for_analysis", "args": {"database": "tg_xB1"}, "timeout_ms": 600000},
    ] + steps + [
        {"tool": "close_database", "args": {"database": "tg_xB1", "save": False}, "timeout_ms": 180000},
    ]
    with io.open(os.path.join(OUT, f"job-{crate}.json"), "w", encoding="utf-8") as f:
        json.dump(job, f, ensure_ascii=False, indent=1)
    print(crate, len(fns), "functions")
