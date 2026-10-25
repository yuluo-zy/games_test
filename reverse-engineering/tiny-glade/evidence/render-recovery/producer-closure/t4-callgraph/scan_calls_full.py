#!/usr/bin/env python3
"""Read-only PE direct-callgraph scanner for the Tiny Glade original image.

Builds a whole-image direct call graph by decoding instructions *inside known
function bodies only* (never raw byte-scanning .text), so `E8` bytes that live in
data/jump-tables are not mis-reported as calls.

Emits a JSON document compatible with evidence/render-recovery/workgraph/calls.json:
  { "functions": { "<va>": { "direct":[{site,target}], "external_jumps":[...],
                             "indirect_calls":[...], "instruction_count": N } }, ... }

Scope is selected with --scope:
  rhapsody : only the 450 rhapsody ranges from workgraph/functions.json (gate/repro)
  full     : union of functions.json + country_core + public-symbols(.text) + PE exception dir

Usage:
  python -I scan_calls_full.py <exe> <repo_root> <out_json> [--scope full|rhapsody]
"""
import argparse
import bisect
import json
import os
import re
import struct
import sys
import site as _site

_usp = _site.getusersitepackages()
if _usp and _usp not in sys.path:
    sys.path.insert(0, _usp)  # capstone lives in user site; -I drops it

from capstone import Cs, CS_ARCH_X86, CS_MODE_64
from capstone.x86_const import X86_OP_IMM

IMAGEBASE = 0x140000000
SIZE_RE = re.compile(r"\[size = (\d+)\]")


def parse_pe(path):
    d = open(path, "rb").read()
    pe = struct.unpack_from("<I", d, 0x3C)[0]
    assert d[pe:pe + 4] == b"PE\0\0", "not a PE image"
    nsec = struct.unpack_from("<H", d, pe + 6)[0]
    optsz = struct.unpack_from("<H", d, pe + 20)[0]
    opt = pe + 24
    imgbase = struct.unpack_from("<Q", d, opt + 24)[0]
    exc_rva, exc_size = struct.unpack_from("<II", d, opt + 112 + 3 * 8)
    st = opt + optsz
    secs = []
    for i in range(nsec):
        at = st + i * 40
        nm = d[at:at + 8].rstrip(b"\x00").decode("latin1")
        vsz, va, rsz, ra = struct.unpack_from("<IIII", d, at + 8)
        secs.append({"name": nm, "va": va, "vsz": vsz, "raw": ra, "rsz": rsz})
    return d, imgbase, secs, exc_rva, exc_size


def rva_to_off(secs, rva):
    for s in secs:
        span = max(s["vsz"], s["rsz"])
        if s["va"] <= rva < s["va"] + span:
            delta = rva - s["va"]
            if delta >= s["rsz"]:
                return None
            return s["raw"] + delta
    return None


def exception_ranges(d, secs, imgbase, exc_rva, exc_size):
    off = rva_to_off(secs, exc_rva)
    out = []
    for at in range(off, off + exc_size, 12):
        begin, end, _u = struct.unpack_from("<III", d, at)
        if begin and end > begin:
            out.append((imgbase + begin, imgbase + end, "exception_dir", ""))
    return out


def load_ranges(exe, root, imgbase, secs, exc_rva, exc_size, scope):
    """Return (ranges, start_index). ranges = list of (start,end,source,name)."""
    ranges = []
    d = open(exe, "rb").read()  # only needed for exception dir here
    if scope == "full":
        ranges.extend(exception_ranges(d, secs, imgbase, exc_rva, exc_size))

    wg = json.load(open(os.path.join(root, "evidence/render-recovery/workgraph/functions.json"),
                        encoding="utf-8"))
    for f in wg:
        va = int(f["va"], 16)
        ranges.append((va, va + f["code_size"], "rhapsody_functions", f["names"][0]))

    if scope == "full":
        cc = json.load(open(os.path.join(
            root, "evidence/render-recovery/producer-closure/c1-country-core/country_core_render_symbols.json"),
            encoding="utf-8"))
        for s in cc["render_symbols"]:
            va = int(s["va"], 16)
            if s.get("size"):
                ranges.append((va, va + s["size"], "country_core", s.get("short") or s.get("name")))

        text = next(s for s in secs if s["name"] == ".text")
        tlo, thi = imgbase + text["va"], imgbase + text["va"] + text["vsz"]
        ps = json.load(open(os.path.join(root, "evidence/render-architecture/public-symbols.json"),
                            encoding="utf-8"))
        for x in ps:
            if x.get("section") != 1 or not x.get("va"):
                continue
            m = SIZE_RE.search(x.get("symbol", ""))
            if not m:
                continue
            va = int(x["va"], 16) if isinstance(x["va"], str) else int(x["va"])
            if va < IMAGEBASE:
                va += IMAGEBASE
            if not (tlo <= va < thi):
                continue
            sz = int(m.group(1))
            if sz <= 0:
                continue
            ranges.append((va, va + sz, "public_symbol", x.get("demangled") or x.get("symbol")))
    return ranges


def build_index(ranges):
    """start -> (end, source, name); end = max end seen at that start."""
    by_start = {}
    for start, end, src, name in ranges:
        cur = by_start.get(start)
        if cur is None or end > cur[0]:
            by_start[start] = (end, src, name)
    return by_start


def decode_range(md, d, secs, start, end):
    """Decode [start,end) inside .text. Return (direct, ext_jumps, indirect, ninstr)."""
    off = rva_to_off(secs, start - IMAGEBASE)
    if off is None:
        return [], [], [], 0
    raw = d[off:off + (end - start)]
    direct, extj, indir = [], [], []
    n = 0
    for ins in md.disasm(raw, start):
        n += 1
        m = ins.mnemonic
        ops = ins.operands
        if m == "call":
            if len(ops) == 1 and ops[0].type == X86_OP_IMM:
                direct.append({"site": hex(ins.address), "target": hex(ops[0].imm)})
            else:
                indir.append({"site": hex(ins.address), "operand": ins.op_str})
        elif m == "jmp":
            if len(ops) == 1 and ops[0].type == X86_OP_IMM:
                tgt = ops[0].imm
                if not (start <= tgt < end):
                    extj.append({"site": hex(ins.address), "target": hex(tgt)})
    return direct, extj, indir, n


def main():
    ap = argparse.ArgumentParser()
    ap.add_argument("exe")
    ap.add_argument("root")
    ap.add_argument("out")
    ap.add_argument("--scope", choices=["full", "rhapsody"], default="full")
    ap.add_argument("--recover-gap-targets", action="store_true",
                    help="second pass: treat unresolved in-.text call targets as leaf "
                         "function entries and disassemble the gap regions they start")
    args = ap.parse_args()

    d, imgbase, secs, exc_rva, exc_size = parse_pe(args.exe)
    ranges = load_ranges(args.exe, args.root, imgbase, secs, exc_rva, exc_size, args.scope)
    by_start = build_index(ranges)
    starts = sorted(by_start)

    md = Cs(CS_ARCH_X86, CS_MODE_64)
    md.detail = True

    # known-function containment: does a target va fall inside some known range?
    def inside_known(va):
        import bisect
        i = bisect.bisect_right(starts, va) - 1
        if i < 0:
            return None
        s = starts[i]
        if s <= va < by_start[s][0]:
            return s
        return None

    functions = {}
    total_direct = 0
    total_extj = 0
    total_indirect = 0
    total_instr = 0
    for s in starts:
        end = by_start[s][0]
        direct, extj, indir, n = decode_range(md, d, secs, s, end)
        functions[hex(s)] = {
            "direct": direct,
            "external_jumps": extj,
            "indirect_calls": indir,
            "instruction_count": n,
            "end_exclusive": hex(end),
            "source": by_start[s][1],
        }
        total_direct += len(direct)
        total_extj += len(extj)
        total_indirect += len(indir)
        total_instr += n

    # ---- optional second pass: recover leaf functions that are only reached as
    # direct call targets but have no bound of their own (no .pdata / public sym).
    # A direct call target inside .text is by definition a function entry.
    gap_meta = {"gap_targets_discovered": 0, "gap_targets_decoded": 0,
                "gap_direct": 0, "gap_indirect": 0, "gap_extjumps": 0, "gap_instr": 0}
    if args.recover_gap_targets and args.scope == "full":
        text = next(s for s in secs if s["name"] == ".text")
        tlo, thi = imgbase + text["va"], imgbase + text["va"] + text["vsz"]
        starts_set = set(starts)

        def in_known(va):
            i = bisect.bisect_right(starts, va) - 1
            return i >= 0 and starts[i] <= va < by_start[starts[i]][0]

        new_starts = set()
        for s in starts:
            for e in functions[hex(s)]["direct"]:
                t = int(e["target"], 16)
                if t not in starts_set and tlo <= t < thi and not in_known(t):
                    new_starts.add(t)
        gap_meta["gap_targets_discovered"] = len(new_starts)
        allstarts = sorted(set(starts) | new_starts)
        pos = {v: i for i, v in enumerate(allstarts)}
        for t in sorted(new_starts):
            i = pos[t]
            end = allstarts[i + 1] if i + 1 < len(allstarts) else thi
            # never extend across an already-known range boundary: if a known range
            # starts later, we already cut at allstarts[i+1]; good.
            direct, extj, indir, n = decode_range(md, d, secs, t, end)
            functions[hex(t)] = {
                "direct": direct, "external_jumps": extj, "indirect_calls": indir,
                "instruction_count": n, "end_exclusive": hex(end), "source": "gap_target_recovered",
            }
            gap_meta["gap_targets_decoded"] += 1
            gap_meta["gap_direct"] += len(direct)
            gap_meta["gap_indirect"] += len(indir)
            gap_meta["gap_extjumps"] += len(extj)
            gap_meta["gap_instr"] += n
        starts = sorted(functions.keys(), key=lambda x: int(x, 16))
        starts = [int(x, 16) for x in starts]
        starts_set = set(starts)
        by_start = {s: (int(functions[hex(s)]["end_exclusive"], 16), functions[hex(s)]["source"], "") for s in starts}

    # authoritative totals recomputed from the final function map
    total_instr = sum(f["instruction_count"] for f in functions.values())
    total_direct = sum(len(f["direct"]) for f in functions.values())
    total_extj = sum(len(f["external_jumps"]) for f in functions.values())
    total_indirect = sum(len(f["indirect_calls"]) for f in functions.values())

    # target resolution stats
    starts_set = set(starts)
    tn_start = tn_inside = tn_outside = 0
    for s in starts:
        for e in functions[hex(s)]["direct"]:
            t = int(e["target"], 16)
            if t in starts_set:
                tn_start += 1
            elif inside_known(t) is not None:
                tn_inside += 1
            else:
                tn_outside += 1

    doc = {
        "functions": functions,
        "meta": {
            "scope": args.scope,
            "exe": args.exe,
            "known_function_starts": len(starts),
            "total_instructions_decoded": total_instr,
            "total_direct_call_sites": total_direct,
            "total_external_jumps": total_extj,
            "total_indirect_call_sites": total_indirect,
            "direct_targets_at_function_start": tn_start,
            "direct_targets_inside_known_range_not_start": tn_inside,
            "direct_targets_outside_all_known_ranges": tn_outside,
            "gap_recovery": gap_meta,
            "sources": {
                "exception_dir_entries": exc_size // 12,
                "rhapsody_functions": sum(1 for r in ranges if r[2] == "rhapsody_functions"),
                "country_core": sum(1 for r in ranges if r[2] == "country_core"),
                "public_symbol_text": sum(1 for r in ranges if r[2] == "public_symbol"),
            },
            "limitations": [
                "only direct call rel32/jmp rel32 are captured; call [reg] and vtable targets unresolved",
                "static edges do not imply runtime execution (branches/inlining/indirect dispatch)",
                "function bounds come from PDB/public symbols/PE exception dir; leaf gaps possible",
                "ICF-merged addresses retain multiple names at one va",
            ],
        },
    }
    with open(args.out, "w", encoding="utf-8") as f:
        json.dump(doc, f, ensure_ascii=False)
    print(json.dumps(doc["meta"], ensure_ascii=False, indent=1))


if __name__ == "__main__":
    main()
