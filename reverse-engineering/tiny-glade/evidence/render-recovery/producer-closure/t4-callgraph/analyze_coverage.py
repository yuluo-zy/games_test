#!/usr/bin/env python3
"""Coverage + false-positive characterisation for the full-image direct callgraph.

Reports:
  - byte coverage of .text by the union of known function ranges
  - section distribution of direct-call targets that are NOT a known function start
  - how many unresolved targets look like data / IAT / real code
"""
import json
import os
import re
import struct
import sys
import site as _site

_usp = _site.getusersitepackages()
if _usp and _usp not in sys.path:
    sys.path.insert(0, _usp)

IMAGEBASE = 0x140000000
SIZE_RE = re.compile(r"\[size = (\d+)\]")


def parse_pe(path):
    d = open(path, "rb").read()
    pe = struct.unpack_from("<I", d, 0x3C)[0]
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
        if s["va"] <= rva < s["va"] + max(s["vsz"], s["rsz"]):
            delta = rva - s["va"]
            return None if delta >= s["rsz"] else s["raw"] + delta
    return None


def section_of(secs, imgbase, va):
    rva = va - imgbase
    for s in secs:
        if s["va"] <= rva < s["va"] + max(s["vsz"], s["rsz"]):
            return s["name"]
    return None


def main():
    exe, root, graph_path = sys.argv[1], sys.argv[2], sys.argv[3]
    d, imgbase, secs, exc_rva, exc_size = parse_pe(exe)

    # rebuild range union from the same four sources
    ranges = []
    off = rva_to_off(secs, exc_rva)
    for at in range(off, off + exc_size, 12):
        b, e, _u = struct.unpack_from("<III", d, at)
        if b and e > b:
            ranges.append((imgbase + b, imgbase + e))
    for f in json.load(open(os.path.join(root, "evidence/render-recovery/workgraph/functions.json"), encoding="utf-8")):
        v = int(f["va"], 16); ranges.append((v, v + f["code_size"]))
    for s in json.load(open(os.path.join(root, "evidence/render-recovery/producer-closure/c1-country-core/country_core_render_symbols.json"), encoding="utf-8"))["render_symbols"]:
        if s.get("size"):
            v = int(s["va"], 16); ranges.append((v, v + s["size"]))
    text = next(s for s in secs if s["name"] == ".text")
    tlo, thi = imgbase + text["va"], imgbase + text["va"] + text["vsz"]
    for x in json.load(open(os.path.join(root, "evidence/render-architecture/public-symbols.json"), encoding="utf-8")):
        if x.get("section") != 1 or not x.get("va"):
            continue
        m = SIZE_RE.search(x.get("symbol", ""))
        if not m:
            continue
        v = int(x["va"], 16) if isinstance(x["va"], str) else int(x["va"])
        if v < IMAGEBASE:
            v += IMAGEBASE
        if not (tlo <= v < thi):
            continue
        ranges.append((v, v + int(m.group(1))))

    ranges = [(a, min(b, thi)) for a, b in ranges if a < thi and b > tlo]
    ranges.sort()
    covered = 0
    cur_s = cur_e = None
    for a, b in ranges:
        if cur_e is None:
            cur_s, cur_e = a, b
        elif a <= cur_e:
            cur_e = max(cur_e, b)
        else:
            covered += cur_e - cur_s
            cur_s, cur_e = a, b
    if cur_e is not None:
        covered += cur_e - cur_s

    starts = set()
    for a, b in ranges:
        starts.add(a)
    # union coverage vs .text
    text_bytes = text["vsz"]
    print(f".text vsz bytes           : {text_bytes:#x} ({text_bytes})")
    print(f"union covered bytes       : {covered:#x} ({covered})  = {100.0*covered/text_bytes:.2f}% of .text")

    graph = json.load(open(graph_path, encoding="utf-8"))
    funcs = graph["functions"]
    from collections import Counter
    unresolved_sec = Counter()
    unresolved_samples = []
    below_text = midrange = 0
    total = 0
    tgt_sections = Counter()
    for sv, e in funcs.items():
        for c in e.get("direct", []):
            total += 1
            t = int(c["target"], 16)
            if t in starts:
                sect = section_of(secs, imgbase, t) or "outside"
                tgt_sections[sect] += 1
                continue
            sect = section_of(secs, imgbase, t) or "outside-image"
            tgt_sections[f"UNRESOLVED:{sect}"] += 1
            unresolved_sec[sect] += 1
            if len(unresolved_samples) < 25:
                unresolved_samples.append((c["site"], c["target"], sect, sv))
    print("direct call targets by section:")
    for k, v in tgt_sections.most_common():
        print(f"   {k:28} {v}")
    print("sample unresolved targets (site -> target [section]) caller:")
    for s, t, sec, cv in unresolved_samples:
        print(f"   {s} -> {t}  [{sec}]  in {cv}")


if __name__ == "__main__":
    main()
