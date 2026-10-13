#!/usr/bin/env python3
"""Dump compact summary of a render_loop pass bundle: functions + inline deps + structs."""
import json
import gzip
import sys
import os


def load(base):
    data = {}
    for key, fname, isgz in (("pdb", ".pdb.json", False),
                             ("structures", ".structures.json.gz", True),
                             ("enriched", ".enriched.json.gz", True),
                             ("deps", ".dependencies.json", False)):
        path = base + fname
        try:
            if isgz:
                with gzip.open(path, "rt") as f:
                    data[key] = json.load(f)
            else:
                with open(path) as f:
                    data[key] = json.load(f)
        except FileNotFoundError:
            data[key] = None
    return data


def short(n):
    return n.split("::")[-1]


def main(base):
    data = load(base)
    out = {"base": base}
    if data.get("enriched"):
        funcs = []
        for fn in data["enriched"]["functions"]:
            funcs.append({
                "name": short(fn["name"]),
                "inline_hosts": [h["host_name"].split("::")[-1] for h in fn.get("inline_hosts", [])],
                "source_lines": sorted({h["source_line"] for h in fn.get("inline_hosts", [])}),
                "compiled_instances": len(fn.get("compiled_instances", [])),
            })
        out["functions"] = funcs
    if data.get("deps"):
        deps = {}
        for fn in data["deps"].get("functions", []):
            name = short(fn["name"])
            direct = []
            for dep in fn.get("dependencies", []):
                if dep.get("relationship") == "pdb_inline_direct":
                    f = dep["function"].split("::")
                    direct.append("::".join(f[-2:]))
            deps[name] = direct
        out["inline_direct_deps"] = deps
    if data.get("structures"):
        structs = []
        for ent in data["structures"].get("named_symbol_entities", []):
            structs.append({
                "name": ent["name"].split("::")[-1],
                "size": ent.get("size_bytes"),
                "kind": ent.get("kind"),
            })
        out["structs"] = structs
    print(json.dumps(out, ensure_ascii=False, indent=0))


if __name__ == "__main__":
    main(sys.argv[1])
