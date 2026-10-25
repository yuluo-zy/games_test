#!/usr/bin/env python3
"""Dump function records (S_GPROC32/S_LPROC32) from a list of module ids.

For each module, runs `llvm-pdbutil dump -symbols -modi=N`, parses the
`S_GPROC32` / `S_LPROC32` record name line and its following `addr =
sec:off, ... code size = N` line, converts to VA (section 1 -> base+off).
Prints JSON grouped by module.

Usage:
  dump_pdb_functions.py <pdb> <base_hex> <mod...>

addr field of S_GPROC32/S_LPROC32 from llvm-pdbutil is a section-1 offset
printed in DECIMAL (image rva minus 0x1000). VA = off + section_va_base
(0x140001000 for the shipped EXEs). Pass base=0x140001000.
"""
import json, re, subprocess, sys

PDBUTIL = "/opt/homebrew/Cellar/llvm/22.1.2/bin/llvm-pdbutil"
PDB = sys.argv[1]          # path to pdb
BASE = int(sys.argv[2], 16) if len(sys.argv) > 2 else 0x140001000
mods = [int(m) for m in sys.argv[3:]]
NAME_RE = re.compile(r"^\s*(\d+) \| S_(GPROC32|LPROC32) \[[^\]]+\] `([^`]+)`")
ADDR_RE = re.compile(r"addr = (\d+):(\d+), code size = (\d+)")

out = []
for mod in mods:
    p = subprocess.run([PDBUTIL, "dump", "-symbols", f"-modi={mod}", PDB],
                       capture_output=True, text=True, encoding="utf-8",
                       errors="replace")
    if p.returncode:
        out.append({"mod": mod, "error": p.stderr.strip()[:200]})
        continue
    funcs = []
    pending = None
    for line in p.stdout.splitlines():
        m = NAME_RE.match(line)
        if m:
            pending = {"mod": mod, "kind": m.group(2), "name": m.group(3)}
            continue
        if pending and "addr =" in line:
            a = ADDR_RE.search(line)
            if a:
                sec, off, size = int(a.group(1)), int(a.group(2)), int(a.group(3))
                pending["va"] = f"{BASE + off:#x}" if sec == 1 else None
                pending["section"] = sec
                pending["code_size"] = size
            funcs.append(pending)
            pending = None
    out.append({"mod": mod, "functions": funcs, "count": len(funcs)})

print(json.dumps(out, ensure_ascii=False, indent=2))
