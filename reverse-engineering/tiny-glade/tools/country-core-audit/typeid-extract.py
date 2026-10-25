#!/usr/bin/env python3
"""Resolve component TypeIds for every `register_required_components` /
`register_component` / `init_state` site in a set of decompiled functions.

Method (proven on components::impl$2 == FollowMouse):
  1. disassemble the function range (llvm-objdump, intel syntax),
  2. collect immediate 64-bit constants from `movabs rXX, 0x...` / `mov rXX, 0x...`
     that look like TypeId halves (exclude small ints and addresses),
  3. pair consecutive halves (hi,lo) and look them up in the binary;
     the owning generic function name carries the concrete component type.

Usage:
  python -I typeid-extract.py --decomp-dir <dir> [--json OUT] [--min-bytes 400]
  python -I typeid-extract.py --va 0x140844f20 [--va ...]
"""
import argparse
import json
import os
import re
import struct
import subprocess
import sys
from pathlib import Path

HERE = Path(__file__).resolve().parent
sys.path.insert(0, str(HERE))
import typeid_resolve as tid  # noqa: E402
from cca import objdump, load_procs  # noqa: E402

MOVABS = re.compile(r"\bmovabs\s+r[a-z0-9]+,\s*(-?0x[0-9a-f]+)\b")
WIDE = re.compile(r"\b0x([0-9a-f]{16})\b")

# concrete component type as the generic arg of a registration/query function
GEN = re.compile(
    r"::(?:register_required_components|register_component|init_state|"
    r"get_component_id|component_ids|register_type|init_resource|"
    r"resource_id|register_resource|init_component|get_resource_id)"
    r"<([^<>]*(?:<[^<>]*>)?[^<>]*)>"
)


def to_u64(tok):
    v = int(tok, 16) if tok.startswith(("0x", "-0x")) else int(tok, 16)
    if tok.startswith("-"):
        v = (-v) & 0xFFFFFFFFFFFFFFFF
    return v


def typeid_lookup(u):
    """Return {symbol_owner_name: full_symbol_name} for TypeId half `u`."""
    hits = {}
    for va in tid.find(u):
        f = tid.lookup(va)
        if f:
            hits[f[2]] = f[2]
    return hits


def concrete_types(symbols):
    """Pull concrete component type names out of the owning symbol names."""
    names = set()
    for s in symbols:
        for g in GEN.findall(s):
            g = re.sub(r"^ref(_mut)?\$<", "", g).rstrip(">")
            g = re.sub(r"^&(mut )?", "", g)
            if "::" in g and "closure" not in g:
                names.add(g)
    return names


def scan_function(start, end, procs):
    """Disassemble [start,end) and return (hi,lo) TypeId candidate pairs."""
    out = subprocess.run(
        [objdump(), "-d", "--no-show-raw-insn", "-M", "intel",
         f"--start-address={start:#x}", f"--stop-address={end:#x}", str(tid.EXE)],
        capture_output=True, text=True).stdout
    imm = []
    for line in out.splitlines():
        m = re.match(r"\s*([0-9a-f]+):\s*(.*)", line)
        if not m:
            continue
        ins = m.group(2)
        # strip the objdump trailing comment ("# 0x142961690") so it is not
        # mistaken for an immediate
        ins = ins.split("#")[0]
        for tok in MOVABS.findall(ins):
            v = to_u64(tok)
            # TypeId halves are full 64-bit values, not small ints / addresses
            if v >> 32 and not (0x140000000 <= v < 0x144000000):
                imm.append(v)
    pairs = [(imm[i], imm[i + 1]) for i in range(len(imm) - 1)]
    return pairs, imm


def main():
    ap = argparse.ArgumentParser()
    ap.add_argument("--decomp-dir")
    ap.add_argument("--va", nargs="*", default=[])
    ap.add_argument("--json", default=None)
    ap.add_argument("--min-bytes", type=int, default=0)
    ap.add_argument("--with-i", action="store_true",
                    help="also emit hi-half-only guesses (noisy; off by default)")
    args = ap.parse_args()

    procs = load_procs()
    starts = sorted(procs)

    jobs = []  # (va, note)
    if args.va:
        for a in args.va:
            jobs.append((int(a, 16), ""))
    if args.decomp_dir:
        d = Path(args.decomp_dir)
        for f in sorted(d.glob("decomp-*.txt")):
            m = re.search(r"decomp-(0x[0-9a-f]+)\.txt$", f.name)
            if not m:
                continue
            sz = f.stat().st_size
            if sz < args.min_bytes:
                continue
            jobs.append((int(m.group(1), 16), (f.read_text(encoding="utf-8",
                        errors="replace").splitlines() or [""])[0][:110]))

    out = []
    for va, note in jobs:
        # find size
        size = 0
        for i in sorted(procs, key=lambda x: abs(x - va))[:1]:
            pass
        import bisect
        j = bisect.bisect_right(starts, va) - 1
        if j >= 0 and starts[j] <= va < starts[j] + procs[starts[j]][0]:
            size = procs[starts[j]][0]
        end = va + (size or 0x400)
        pairs, imm = scan_function(va, end, procs)
        resolved = []
        seen = set()
        for hi, lo in pairs:
            a = typeid_lookup(hi)
            b = typeid_lookup(lo)
            if not a:
                continue
            if not b:
                # hi-only: the lo half is usually in .rdata with no PDB symbol,
                # but this mode also picks up coincidental data hits -> opt-in.
                if not args.with_i:
                    continue
                for t in sorted(concrete_types(a)):
                    if "components::" in t and t not in seen:
                        seen.add(t)
                        resolved.append({"type": t, "conf": "I"})
                continue
            for t in sorted(concrete_types(set(a) & set(b))):
                if t not in seen:
                    seen.add(t)
                    resolved.append({"type": t, "conf": "O"})
        out.append({"va": hex(va), "note": note, "n_imm": len(imm),
                    "types": resolved})
        print(f"{va:#x}  imm={len(imm):<4} types={resolved}")

    if args.json:
        Path(args.json).write_text(json.dumps(out, ensure_ascii=False, indent=1),
                                   encoding="utf-8")
        print(f"-> {args.json}")


if __name__ == "__main__":
    main()
