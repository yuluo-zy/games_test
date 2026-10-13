#!/usr/bin/env python3
"""Extract a compact inventory of all wall system source units for WP12.

Reads the B skeleton bundles under skeleton/B/crates/country-core/src/systems/wall/
and prints, per unit: path, function names, file_interface_candidates signatures.
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


def main():
    units = sorted(WALL.rglob("*.enriched.json.gz"))
    out = []
    for u in units:
        try:
            data = read_json(u)
        except Exception as e:
            print(f"ERR {u}: {e}", file=sys.stderr)
            continue
        path = u.relative_to(WALL).with_suffix("").with_suffix("")  # strip .json.gz -> e.g. calc_wall_bend.rs
        entry = {
            "unit": str(path),
            "functions": [],
            "interfaces": [],
        }
        for f in data.get("functions", []):
            entry["functions"].append(f.get("name"))
        for c in data.get("file_interface_candidates", []):
            entry["interfaces"].append({
                "association": c.get("association"),
                "binding": c.get("binding_to_specific_game_function"),
                "signature": c.get("function_type_spelling"),
            })
        out.append(entry)
    print(json.dumps(out, indent=1, ensure_ascii=False))


if __name__ == "__main__":
    main()
