#!/usr/bin/env python3
"""Parse skeleton PDB bundles for WP5 (render_loop frame passes).

Usage:
  parse_bundle.py <bundle_base_path> <mode>
    modes: funcs | deps | lines | structs | owners | all

Reads the .rs.{pdb.json, structures.json.gz, enriched.json.gz, dependencies.json}
bundle family for a render_loop pass file. Output is JSON on stdout.
"""
import json
import gzip
import sys


def load(base):
    """Load the bundle family for a base path like
    .../compute_passes/gpu_generate_draw_lists.rs"""
    data = {}
    try:
        with open(base + ".pdb.json") as f:
            data["pdb"] = json.load(f)
    except FileNotFoundError:
        data["pdb"] = None
    for key, fname in (("structures", ".structures.json.gz"),
                       ("enriched", ".enriched.json.gz")):
        try:
            with gzip.open(base + fname, "rt") as f:
                data[key] = json.load(f)
        except FileNotFoundError:
            data[key] = None
    try:
        with open(base + ".dependencies.json") as f:
            data["deps"] = json.load(f)
    except FileNotFoundError:
        data["deps"] = None
    return data


def dump_funcs(data):
    out = []
    if data.get("enriched"):
        for fn in data["enriched"]["functions"]:
            out.append({
                "name": fn.get("name"),
                "address": fn.get("address"),
                "line": fn.get("line_number"),
            })
    if data.get("pdb") and data["pdb"].get("procedure_instances"):
        # pdb top-level has counts only; the jsonl.gz holds records
        pass
    return out


def dump_deps(data):
    if not data.get("deps"):
        return None
    d = data["deps"]
    return {
        "imports": d.get("imports"),
        "imports_count": d.get("imports_count") or (len(d.get("imports", [])) if isinstance(d.get("imports"), list) else None),
        "key": list(d.keys()),
    }


def dump_lines(data):
    if not data.get("pdb"):
        return None
    p = data["pdb"]
    return {
        "line_blocks": p.get("line_blocks"),
        "inline_sites": p.get("inline_sites"),
        "procedure_instances": p.get("procedure_instances"),
        "metadata_records": p.get("metadata_records"),
        "source_sha256": p.get("source_sha256"),
        "modules": p.get("modules"),
    }


def dump_structs(data):
    if not data.get("structures"):
        return None
    out = []
    for ent in data["structures"].get("named_symbol_entities", []):
        out.append({
            "name": ent["name"],
            "kind": ent.get("kind"),
            "size_bytes": ent.get("size_bytes"),
            "fields": ent.get("fields"),
            "def_status": ent.get("definition_status"),
        })
    return out


def dump_owners(data):
    """Who owns each imported symbol: from dependencies.json imports sections."""
    if not data.get("deps"):
        return None
    out = []
    imports = data["deps"].get("imports")
    if isinstance(imports, dict):
        for owner, items in imports.items():
            out.append({"owner": owner, "items": items if isinstance(items, list) else []})
    return out


if __name__ == "__main__":
    base = sys.argv[1]
    mode = sys.argv[2] if len(sys.argv) > 2 else "all"
    data = load(base)
    funcs = {
        "funcs": dump_funcs,
        "deps": dump_deps,
        "lines": dump_lines,
        "structs": dump_structs,
        "owners": dump_owners,
    }
    if mode == "all":
        print(json.dumps({
            "funcs": dump_funcs(data),
            "lines": dump_lines(data),
            "structs": dump_structs(data),
            "deps": dump_deps(data),
            "owners": dump_owners(data),
        }, ensure_ascii=False, indent=1))
    else:
        fn = funcs[mode]
        print(json.dumps(fn(data), ensure_ascii=False, indent=1))
