#!/usr/bin/env python3
"""Gate: compare the objdump-based calls.json (450 rhapsody fns) against two
reproductions and the capstone scanner, edge-for-edge.

Inputs (all produced under t4-callgraph/):
  A = repo workgraph/calls.json                    (objdump, committed)
  B = t4-callgraph/repro-objdump/calls.json        (objdump rerun, same script)
  C = t4-callgraph/repro-capstone-rhapsody.json    (this scanner, scope=rhapsody)

Writes a machine-readable verdict to compare-report.json and prints a summary.
"""
import json
import sys


def norm_direct(lst):
    return sorted((int(e["site"], 16), int(e["target"], 16)) for e in lst)


def norm_sites(lst):
    return sorted(int(e["site"], 16) for e in lst)


def edge_sig(functions):
    out = {}
    for k, v in functions.items():
        out[int(k, 16)] = {
            "direct": norm_direct(v.get("direct", [])),
            "ext": norm_direct(v.get("external_jumps", [])),
            "ind": norm_sites(v.get("indirect_calls", [])),
        }
    return out


def main():
    a_path, b_path, c_path, out_path = sys.argv[1:5]
    A = json.load(open(a_path, encoding="utf-8"))
    B = json.load(open(b_path, encoding="utf-8"))
    C = json.load(open(c_path, encoding="utf-8"))
    sa, sb, sc = edge_sig(A["functions"]), edge_sig(B["functions"]), edge_sig(C["functions"])

    keys = sorted(set(sa) | set(sb) | set(sc))

    def diff(x, y, field):
        bad = []
        for k in keys:
            ex = x.get(k, {}).get(field, [])
            ey = y.get(k, {}).get(field, [])
            if ex != ey:
                bad.append({"va": hex(k), "only_left": sorted(set(ex) - set(ey)),
                            "only_right": sorted(set(ey) - set(ex))})
        return bad

    verdict = {
        "A_vs_B": {
            "direct_diff": diff(sa, sb, "direct"),
            "ext_diff": diff(sa, sb, "ext"),
            "ind_diff": diff(sa, sb, "ind"),
        },
        "A_vs_C": {
            "direct_diff": diff(sa, sc, "direct"),
            "ext_diff": diff(sa, sc, "ext"),
            "ind_diff": diff(sa, sc, "ind"),
        },
        "counts": {
            "A": {"fns": len(sa), "direct": sum(len(v["direct"]) for v in sa.values()),
                  "ext": sum(len(v["ext"]) for v in sa.values()),
                  "ind": sum(len(v["ind"]) for v in sa.values())},
            "B": {"fns": len(sb), "direct": sum(len(v["direct"]) for v in sb.values()),
                  "ext": sum(len(v["ext"]) for v in sb.values()),
                  "ind": sum(len(v["ind"]) for v in sb.values())},
            "C": {"fns": len(sc), "direct": sum(len(v["direct"]) for v in sc.values()),
                  "ext": sum(len(v["ext"]) for v in sc.values()),
                  "ind": sum(len(v["ind"]) for v in sc.values())},
        },
    }
    json.dump(verdict, open(out_path, "w", encoding="utf-8"), ensure_ascii=False, indent=1)
    print(json.dumps(verdict["counts"], ensure_ascii=False, indent=1))
    print("A_vs_B edge mismatches:",
          len(verdict["A_vs_B"]["direct_diff"]) + len(verdict["A_vs_B"]["ext_diff"]) + len(verdict["A_vs_B"]["ind_diff"]))
    print("A_vs_C edge mismatches:",
          len(verdict["A_vs_C"]["direct_diff"]) + len(verdict["A_vs_C"]["ext_diff"]) + len(verdict["A_vs_C"]["ind_diff"]))


if __name__ == "__main__":
    main()
