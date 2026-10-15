#!/usr/bin/env python3
"""Batch-scan register_required_components fns: Self identity (PDB) + required
components (TypeId movabs constants + 16B rodata loads resolved against binary).

Usage:
  python -I comp_register_map.py --va 0x... [--rodata] [--json OUT]
  python -I comp_register_map.py --all [--rodata] [--json OUT]
"""
import argparse
import bisect
import json
import re
import struct
import subprocess
import sys
from pathlib import Path

HERE = Path(__file__).resolve().parent
sys.path.insert(0, str(HERE))
from cca import load_procs, objdump  # noqa: E402
import typeid_resolve as tid  # noqa: E402

MOVABS = re.compile(r"\bmovabs\s+r[a-z0-9]+,\s*(-?0x[0-9a-f]+)\b")
XMM = re.compile(r"\b(movdqu|movups|movdqa|movaps|movupd)\s+xmm\d+,\s+xmmword ptr \[rip[^\]]*\]")
COMMENT_VA = re.compile(r"#\s*(0x[0-9a-f]+)")
INSN = re.compile(r"^\s*([0-9a-f]+):\s*(.*)$")

# PE sections for raw-file reads
def load_sections():
    data = tid.e
    e_lfanew = struct.unpack_from("<I", data, 0x3C)[0]
    nsec = struct.unpack_from("<H", data, e_lfanew + 6)[0]
    opt_size = struct.unpack_from("<H", data, e_lfanew + 20)[0]
    sec_off = e_lfanew + 24 + opt_size
    out = []
    for i in range(nsec):
        o = sec_off + i * 40
        vsize, va, rawsize, rawptr = struct.unpack_from("<IIII", data, o + 8)
        out.append((va, vsize, rawptr, rawsize))
    return out

SECS = load_sections()

def scan(va, size):
    out = subprocess.run(
        [objdump(), "-d", "--no-show-raw-insn", "-M", "intel",
         f"--start-address={va:#x}", f"--stop-address={va + size:#x}", tid.EXE],
        capture_output=True, text=True).stdout
    imms = []   # (addr, value)
    rodata = [] # (addr, rdata VA) of 16B xmm loads
    for line in out.splitlines():
        m = INSN.match(line)
        if not m:
            continue
        addr = int(m.group(1), 16)
        ins = m.group(2)
        for tok in MOVABS.findall(ins.split("#")[0]):
            v = int(tok, 16) if not tok.startswith("-") else (-int(tok[1:], 16)) & 0xFFFFFFFFFFFFFFFF
            if v >> 32 and not (0x140000000 <= v < 0x144000000):
                imms.append((addr, v))
        for m2 in XMM.finditer(ins):
            c = COMMENT_VA.search(ins[m2.end():])
            if c:
                rodata.append((addr, int(c.group(1), 16)))
    return imms, rodata

def rodata_qwords(va):
    """Read qwords at rdata VA from the raw file."""
    rva = va - 0x140000000
    for va0, vsize, rawptr, rawsize in SECS:
        if va0 <= rva < va0 + vsize:
            off = rawptr + (rva - va0)
            raw = tid.e[off:off + 128]
            return [int.from_bytes(raw[i:i + 8], "little") for i in range(0, min(len(raw), 128), 8)]
    return []

def owner_names(u):
    """Symbol names of all functions containing the 8-byte pattern u."""
    names = set()
    for va in tid.find(u):
        f = tid.lookup(va)
        if f:
            names.add(f[2])
    return names

def names_from(owner):
    """Extract concrete type candidates from an owner symbol name."""
    m = re.search(r"_\d+\$LT\$(.*?)\$u20\$as\$u20\$", owner)
    if m:
        nm = m.group(1).replace("$u20$", " ").replace("..", "::").replace("$LT$", "<").replace("$GT$", ">").replace("$u2c$", ",")
        return {nm}
    g = re.findall(r"::(?:init_state|register_component|component_ids|init_component|init_resource|register_resource|get_resource_id|get_component_id|register_type)<([^<>]*(?:<[^<>]*>)?[^<>]*)>", owner)
    out = set()
    for x in g:
        x = re.sub(r"^ref(_mut)?\$<", "", x).rstrip(">")
        x = re.sub(r"^&(mut )?", "", x)
        if "::" in x and "closure" not in x:
            out.add(x)
    return out

def resolve_pair(h1, h2):
    names = set()
    for u in (h1, h2):
        for on in owner_names(u):
            names |= names_from(on)
    return sorted(n for n in names if not n.startswith("_"))

def main():
    ap = argparse.ArgumentParser()
    ap.add_argument("--va", nargs="*", default=[])
    ap.add_argument("--all", action="store_true")
    ap.add_argument("--rodata", action="store_true",
                    help="also resolve 16B rodata xmm loads as TypeId pairs")
    ap.add_argument("--json", default=None)
    args = ap.parse_args()

    procs = load_procs()
    starts = sorted(procs)
    pub = {}
    if not args.va:
        for p in [Path(r"D:/game/reverse-engineering/tiny-glade/evidence/render-architecture/public-symbols.json"),
                  Path(r"D:/game/reverse-engineering/tiny-glade/evidence/render-skeleton/public-symbols.json")]:
            if not p.exists():
                continue
            for e in json.load(open(p, encoding="utf8")):
                dm = e.get("demangled")
                va = e.get("va")
                if va and dm:
                    pub[int(va, 16)] = dm
        targets = [(va, dm) for va, dm in pub.items()
                   if "register_required_components" in dm or "clone_behavior" in dm]
    else:
        targets = [(int(a, 16), "ARG") for a in args.va]
    targets.sort()

    rows = []
    for va, dm in targets:
        j = bisect.bisect_right(starts, va) - 1
        size = procs[starts[j]][0] if starts[j] <= va < starts[j] + procs[starts[j]][0] else 0x400
        imms, rod = scan(va, size)
        # pair consecutive imms within 0x80 bytes
        pairs = []
        i = 0
        while i + 1 < len(imms):
            if imms[i + 1][0] - imms[i][0] < 0x80:
                pairs.append((imms[i], imms[i + 1]))
                i += 2
            else:
                i += 1
        req = []
        seen = set()
        for (a1, h1), (a2, h2) in pairs:
            names = resolve_pair(h1, h2)
            key = (h1, h2)
            if key in seen:
                continue
            seen.add(key)
            req.append({"hi": hex(h1), "lo": hex(h2), "kind": "imm", "names": names})
        if args.rodata:
            for a, rva in rod:
                qw = rodata_qwords(rva)
                for k in range(0, len(qw) - 1):
                    h1, h2 = qw[k], qw[k + 1]
                    if not (h1 >> 32) or not (h2 >> 32):
                        continue
                    if 0x140000000 <= h1 < 0x144000000 or 0x140000000 <= h2 < 0x144000000:
                        continue
                    key = (h1, h2)
                    if key in seen:
                        continue
                    seen.add(key)
                    req.append({"hi": hex(h1), "lo": hex(h2), "kind": "rodata",
                                "names": resolve_pair(h1, h2)})
        rows.append({"va": hex(va), "pdb": dm[:160], "n_imm": len(imms),
                     "n_rodata": len(rod), "typeids": req})
        print(f"{va:#x} {dm[:110]}")
        for r in req:
            print(f"    [{r['kind']}] {r['hi']}/{r['lo']} -> {r['names']}")
    if args.json:
        Path(args.json).write_text(json.dumps(rows, ensure_ascii=False, indent=1), encoding="utf8")

if __name__ == "__main__":
    main()
