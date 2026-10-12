#!/usr/bin/env python3
"""Country-core layer call-graph subgraph extractor.

Consumes the whole-image direct call graph produced by scan_calls_full.py and
reports, for a set of key country_core / render_loop functions, the direct
callees (targets) and direct callers (reverse edges), annotated with names.

Also emits a per-symbol CSV/JSON of in/out degree for the country_core render
symbols so the layer's fan-in/fan-out can be inspected without loading the
whole graph.

Usage:
  python -I country_core_subgraph.py <callgraph_full.json> <repo_root> <out_json>
"""
import json
import os
import re
import sys
import site as _site

_usp = _site.getusersitepackages()
if _usp and _usp not in sys.path:
    sys.path.insert(0, _usp)

IMAGEBASE = 0x140000000
SIZE_RE = re.compile(r"\[size = (\d+)\]")

KEY = {
    0x1404E8010: "tiny_glade::render_loop::render",
    0x1404EC110: "tiny_glade::render_loop::render_world",
    0x140161CD0: "rhapsody::rg::state::RenderGraphState::compile",
}


def build_names(root):
    names = {}
    def add(va, n):
        if va and n:
            names.setdefault(va, n)
    wg = json.load(open(os.path.join(root, "evidence/render-recovery/workgraph/functions.json"), encoding="utf-8"))
    for f in wg:
        add(int(f["va"], 16), f["names"][0])
    cc = json.load(open(os.path.join(
        root, "evidence/render-recovery/producer-closure/c1-country-core/country_core_render_symbols.json"),
        encoding="utf-8"))
    for s in cc["render_symbols"]:
        add(int(s["va"], 16), s.get("short") or s.get("name"))
    ps = json.load(open(os.path.join(root, "evidence/render-architecture/public-symbols.json"), encoding="utf-8"))
    for x in ps:
        if x.get("va") and x.get("section") == 1:
            v = int(x["va"], 16) if isinstance(x["va"], str) else int(x["va"])
            if v < IMAGEBASE:
                v += IMAGEBASE
            add(v, x.get("demangled") or x.get("symbol"))
    return names


def main():
    graph = json.load(open(sys.argv[1], encoding="utf-8"))
    root = sys.argv[2]
    out = sys.argv[3]
    funcs = graph["functions"]
    names = build_names(root)

    # rebuild reverse index
    rev = {}
    for sv, e in funcs.items():
        s = int(sv, 16)
        for c in e.get("direct", []):
            rev.setdefault(c["target"].lower(), []).append({"site": c["site"], "caller": sv})

    report = {"keys": {}, "reverse": {}}
    for va, label in KEY.items():
        sv = hex(va)
        e = funcs.get(sv)
        callees = []
        if e:
            for c in e["direct"]:
                t = int(c["target"], 16)
                callees.append({"site": c["site"], "target": c["target"], "name": names.get(t, "UNKNOWN")})
        callers = []
        for r in rev.get(sv, []):
            callers.append({"site": r["site"], "caller": r["caller"], "name": names.get(int(r["caller"], 16), "UNKNOWN")})
        report["keys"][sv] = {
            "label": label,
            "known_end": e.get("end_exclusive") if e else None,
            "instruction_count": e.get("instruction_count") if e else None,
            "direct_callees": callees,
            "direct_caller_count": len(callers),
            "direct_callers": callers,
            "indirect_call_sites": len(e.get("indirect_calls", [])) if e else None,
            "external_jumps": len(e.get("external_jumps", [])) if e else None,
        }

    # country_core subgraph degree summary
    sub = {}
    for s in json.load(open(os.path.join(
            root, "evidence/render-recovery/producer-closure/c1-country-core/country_core_render_symbols.json"),
            encoding="utf-8"))["render_symbols"]:
        v = int(s["va"], 16)
        sv = hex(v)
        e = funcs.get(sv)
        sub[sv] = {
            "name": s.get("short") or s.get("name"),
            "out": len(e.get("direct", [])) if e else (None),
            "indirect": len(e.get("indirect_calls", [])) if e else None,
            "in": len(rev.get(sv, [])),
            "present_in_scan": e is not None,
        }
    report["country_core_degree"] = sub

    json.dump(report, open(out, "w", encoding="utf-8"), ensure_ascii=False, indent=1)
    # console summary
    for va in KEY:
        r = report["keys"][hex(va)]
        print(f"== {hex(va)} {r['label']} (callees={len(r['direct_callees'])}, callers={r['direct_caller_count']}, indirect={r['indirect_call_sites']}, extjumps={r['external_jumps']})")
        for c in r["direct_callees"][:40]:
            print(f"   -> {c['target']} {c['name'][:90]}")
        for c in r["direct_callers"][:15]:
            print(f"   <- {c['caller']} {c['name'][:90]}")
    ncc = len(sub)
    present = sum(1 for v in sub.values() if v["present_in_scan"])
    print(f"country_core render symbols present in scan: {present}/{ncc}")


if __name__ == "__main__":
    main()
