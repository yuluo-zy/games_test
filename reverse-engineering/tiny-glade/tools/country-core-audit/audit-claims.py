#!/usr/bin/env python3
"""Spot-check whether claimed addresses really belong to the claimed symbols.

Usage:
  python -I audit-claims.py --claims claims.json

claims.json = [{"addr": "0x140a8a5d0", "claimed": "AppMode::is_tool_selected",
                "expect_not": true, "note": "ui-input REVIEW ..."}]

For each claim print:
  - the PDB procedure starting exactly at addr (if any)
  - the PDB procedure containing addr (owner + offset)
  - the S_PUB32 public symbol at/near addr
Nearest-symbol lookups make "地址错位" claims falsifiable without opening Ghidra.
"""
import argparse
import bisect
import json
import re
from pathlib import Path

HERE = Path(__file__).resolve().parent
import sys
sys.path.insert(0, str(HERE))
from cca import load_procs  # noqa: E402

ROOT = HERE.parents[1]
PUB = ROOT / "evidence" / "render-architecture" / "public-symbols.json"


def load_pub():
    """{va:int -> (size, demangled)}"""
    d = json.loads(PUB.read_text(encoding="utf-8"))
    out = {}
    for r in d:
        s = r.get("symbol", "")
        m = re.match(r"(\d+) \| S_PUB32 \[size = (\d+)\]", s)
        if m and r.get("va"):
            out.setdefault(int(r["va"], 16), (int(m.group(2)), r.get("demangled", "")))
    return out


def main():
    ap = argparse.ArgumentParser()
    ap.add_argument("--claims", required=True)
    ap.add_argument("--json", default=None)
    args = ap.parse_args()

    procs = load_procs()
    starts = sorted(procs)
    pub = load_pub()
    pub_starts = sorted(pub)

    claims = json.loads(Path(args.claims).read_text(encoding="utf-8"))
    rows = []
    for c in claims:
        a = int(c["addr"], 16)
        exact = procs.get(a)
        j = bisect.bisect_right(starts, a) - 1
        owner = None
        if j >= 0:
            s0, (sz, nm) = starts[j], procs[starts[j]]
            if a < s0 + sz:
                owner = {"start": hex(s0), "size": sz, "name": nm, "off": a - s0}
        pj = bisect.bisect_right(pub_starts, a) - 1
        psym = None
        if pj >= 0:
            s0 = pub_starts[pj]
            if a < s0 + pub[s0][0]:
                psym = {"start": hex(s0), "name": pub[s0][1]}

        claimed = c.get("claimed", "")
        verdict = "?"
        if exact:
            nm = exact[1]
            hit = claimed and (claimed.split("::")[-1] in nm)
            verdict = "MATCH" if hit else "MISMATCH"
        else:
            verdict = "NOT-A-PROC-START"
        rows.append({"addr": c["addr"], "claimed": claimed, "note": c.get("note", ""),
                     "verdict": verdict, "pdbsym": exact[1] if exact else None,
                     "owner": owner, "pubsym": psym})
        print(f"{c['addr']}  claim={claimed!r}")
        print(f"    verdict={verdict}")
        print(f"    pdb@exact = {exact[1] if exact else '—'}")
        if owner:
            print(f"    owner     = {owner['name'][:150]} +{owner['off']:#x}")
        if psym:
            print(f"    pubsym    = {psym['name'][:150]}")

    if args.json:
        Path(args.json).write_text(json.dumps(rows, ensure_ascii=False, indent=1), encoding="utf-8")
        print(f"-> {args.json}")


if __name__ == "__main__":
    main()
