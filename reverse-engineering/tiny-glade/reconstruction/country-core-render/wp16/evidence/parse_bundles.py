#!/usr/bin/env python3
"""Compact country-core skeleton bundle parser for WP16. Read-only."""
import gzip
import json
import os
import sys

# Namespaces to hide as noise (reused deps, not game business).
NOISE = ("hashbrown", "alloc::", "core::", "std::", "bevy_", "glam::", "serde::",
         "smallvec", "parking_lot", "log::", "itertools", "bumpalo", "ustr",
         "ron::", "rkyv", "ash::", "winit", "slab", "crossbeam", "rayon",
         "frozen", "miniz", "ahash", "stable_deref_trait", "bitflags",
         "futures", "tokio", "tracing", "nohash", "indexmap", "foldhash",
         "num_cpus", "memmap2", "rawptr", "once_cell", "thiserror", "anyhow",
         "fastrand", "half::")


def is_noise(name):
    return any(name.startswith(p) for p in NOISE)


def load_gz(path):
    with gzip.open(path, "rt", encoding="utf-8") as f:
        return json.load(f)


def load_json(path):
    with open(path, "r", encoding="utf-8") as f:
        return json.load(f)


def summarize_base(base):
    lines = []
    lines.append(f"== FILE {os.path.basename(base)} ==")

    sp = base + ".structures.json.gz"
    if os.path.exists(sp):
        try:
            d = load_gz(sp)
            ents = d.get("named_symbol_entities", [])
            lines.append(f"  entities({len(ents)}):")
            for e in ents:
                name = e.get("name", "?")
                if is_noise(name):
                    continue
                procs = {}
                for p in e.get("procedure_occurrences", []):
                    pn = p.get("name")
                    if pn and not is_noise(pn):
                        procs.setdefault(pn, p.get("rva"))
                if procs:
                    for pn, rva in procs.items():
                        lines.append(f"    e[{name}] -> proc rva={rva:#x} {pn}")
                else:
                    lines.append(f"    e[{name}] (no own non-noise procs)")
        except Exception as ex:
            lines.append(f"  (structures error {ex})")

    ep = base + ".enriched.json.gz"
    if os.path.exists(ep):
        try:
            d = load_gz(ep)
            funcs = d.get("functions", [])
            lines.append(f"  enriched functions({len(funcs)}):")
            for fn in funcs:
                name = fn.get("name", "?")
                if is_noise(name):
                    continue
                ci = fn.get("compiled_instances", [])
                ih = fn.get("inline_hosts", [])
                hosts = set()
                for h in ih:
                    hn = h.get("host_name", "")
                    if not is_noise(hn):
                        hosts.add(f"{h.get('host_rva',0):#x}:{hn}")
                if ci or hosts:
                    cistr = "".join(f" ci#{c.get('rva',0):#x}" for c in ci)
                    hstr = " inlined_into{" + ";".join(sorted(hosts)) + "}" if hosts else ""
                    lines.append(f"    f {name}{cistr}{hstr}")
                # interfaces
                cand = fn.get("candidate_interface_ids", [])
                sis = fn.get("system_interfaces_from_own_symbol", [])
                if cand or sis:
                    lines.append(f"      ifaces cand={cand} sys={sis}")
        except Exception as ex:
            lines.append(f"  (enriched error {ex})")

    dp = base + ".dependencies.json"
    if os.path.exists(dp):
        try:
            d = load_json(dp)
            lines.append("  deps: " + json.dumps(d, ensure_ascii=False)[:600])
        except Exception as ex:
            lines.append(f"  (deps error {ex})")
    return "\n".join(lines)


def main():
    scope_path, out_path = sys.argv[1], sys.argv[2]
    scope = json.loads(open(scope_path, encoding="utf-8").read())
    chunks = []
    for label, bases in scope.items():
        chunks.append(f"\n########## SCOPE {label} ##########")
        for base in bases:
            chunks.append(summarize_base(base))
    open(out_path, "w", encoding="utf-8").write("\n".join(chunks))
    print(f"wrote {out_path}")


if __name__ == "__main__":
    main()
