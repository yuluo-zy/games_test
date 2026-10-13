#!/usr/bin/env python3
"""Per-unit summarizer for WP12 wall systems.

For each B skeleton unit under skeleton/B/crates/country-core/src/systems/wall/,
extract a compact evidence digest:
  - procedure functions (name, source line, code size, VA)
  - native-direct FunctionSystem interface candidates (full signature)
  - private-source callees (game-owned, from dependencies.json)
  - import candidates (crate names)
Writes one markdown-ish text block per unit.
Run with python3 -I.
"""
import json
import gzip
import sys
from pathlib import Path

WALL = Path("/Users/yuluo/Desktop/game/games_test/reverse-engineering/tiny-glade/skeleton/B/crates/country-core/src/systems/wall")


def read_json(path):
    if str(path).endswith(".gz"):
        with gzip.open(path, "rt") as f:
            return json.load(f)
    return json.loads(path.read_text())


def proc_name_short(name):
    return name.replace("country_core::systems::wall::", "")


def main():
    units = sorted(WALL.rglob("*.enriched.json.gz"))
    out = []
    for u in units:
        unit = str(u.relative_to(WALL).with_suffix("").with_suffix(""))
        lines = []
        lines.append(f"#### {unit}")
        enriched = read_json(u)
        # functions list from enriched
        proc_info = {}
        for f in enriched.get("functions", []):
            name = f.get("name", "")
            proc_info[name] = f
        # interfaces: native direct candidates
        for c in enriched.get("file_interface_candidates", []):
            if c.get("association") == "native_direct_generic_caller":
                sig = c.get("function_type_spelling", "").replace("void (*)(", "").rstrip(")")
                lines.append(f"  SYS sig: {sig[:400]}")
        # pdb bundle: procedures with source lines / VA
        pdb_path = u.with_name(u.name.replace(".enriched.json.gz", ".pdb.jsonl.gz"))
        if pdb_path.exists():
            for raw in gzip.open(pdb_path, "rt"):
                obj = json.loads(raw)
                ev = obj.get("event")
                if ev == "symbol" and obj.get("class") == "procedure":
                    name = obj.get("name", "")
                    code_size = obj.get("code_size")
                    lines.append(f"  PROC: {proc_name_short(name)} size={code_size} flags={obj.get('flags')} ")
                elif ev == "lines" and obj.get("file_id"):
                    lines.append(f"  LINEBLOCK: va={obj.get('va')} size={obj.get('code_size')} nlines={len(obj.get('lines', []))}")
        # structures / interface detail for the native-direct candidates
        for c in enriched.get("file_interface_candidates", []):
            if c.get("association") == "native_direct_generic_caller":
                # print function type candidate evidence if present
                pass
        # private-source callees from dependencies.json
        dep_path = u.with_name(u.name.replace(".enriched.json.gz", ".dependencies.json"))
        if dep_path.exists():
            dep = json.loads(dep_path.open().read())
            for imp in dep.get("imports", []):
                crate = imp.get("crate")
                if crate in ("country_core", "utils", "system_wall_constructor", "system_decorator"):
                    lines.append(f"  USE: {crate}")
            priv_callees = set()
            for fn in dep.get("functions", []):
                for d in fn.get("dependencies", []):
                    if d.get("ownership") == "private_source_ownership_candidate":
                        tgt = d.get("function", "").replace("country_core::", "")
                        if not tgt.startswith(("resources::walls::impl", "systems::wall::")):
                            pass
                        priv_callees.add(d.get("function", ""))
            for callee in sorted(priv_callees):
                lines.append(f"  CALL: {proc_name_short(callee)[:180]}")
        out.append("\n".join(lines))
    print("\n".join(out))


if __name__ == "__main__":
    main()
