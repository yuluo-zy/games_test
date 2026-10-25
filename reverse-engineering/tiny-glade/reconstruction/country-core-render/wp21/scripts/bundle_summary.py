#!/usr/bin/env python3
"""Summarize skeleton bundles (enriched.json.gz) for a list of modules.

Usage: bundle_summary.py <skeleton_root> <module...>   (module = relative src path
without extension, e.g. resources/walls/arches)
Prints, per module: function names, entity names, file_interface_candidates count.
"""
import gzip, json, sys

def load(p):
    try:
        return json.load(gzip.open(p))
    except Exception as e:
        return {"__error__": str(e)}

def main():
    root = sys.argv[1]
    mods = sys.argv[2:]
    for m in mods:
        p = f"{root}/{m}.rs.enriched.json.gz"
        d = load(p)
        print(f"===== {m} =====")
        if "__error__" in d:
            print("  ERR", d["__error__"]); continue
        src = d.get("source", {})
        print("  sha:", (src.get("source_sha256") or ["?"])[0][:12], "proc:", src.get("procedure_instances"), "inline_sites:", src.get("inline_sites"), "type_idx:", src.get("type_indices"))
        for f in d.get("functions", []):
            nm = f.get("name", "?")
            # compact: strip module prefix
            short = nm.split("::")
            if short and short[0].startswith("country_core"):
                short = short[2:]
            print("  F", "::".join(short), "| inst:", len(f.get("compiled_instances", [])), "inline:", f.get("inline_hosts") is not None and len(f.get("inline_hosts", [])))
        cands = d.get("file_interface_candidates", [])
        if cands:
            print("  candidates:")
            for c in cands[:30]:
                print("    C", c.get("candidate") if isinstance(c, dict) else c)

if __name__ == "__main__":
    main()
