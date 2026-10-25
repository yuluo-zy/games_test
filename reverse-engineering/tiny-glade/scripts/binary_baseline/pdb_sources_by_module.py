#!/usr/bin/env python3
"""Map source files to modules from an llvm-pdbutil dump -files export.

Usage:
  pdb_sources_by_module.py <ver=a|b> <kw1> [kw2 ...]

Reads evidence/multi-baseline/pdb-exports/<ver>_files.txt, finds every module
whose .rs source list contains one of the keywords, and prints JSON:
{version, modules, water_modules...} with each hit's module id, module name,
and the matching source files (path relative to country-slice-private/, sha8).

Files export format (from `llvm-pdbutil dump -files`):
  Mod 0001 | `name`:
  - (SHA-256: <HEX>) <path>
"""
import json
import re
import sys

MOD_RE = re.compile(r"^Mod (\d+) \| `(.*)`")
FILE_RE = re.compile(r"^\s*- \(SHA-256: ([0-9A-Fa-f]{8,})\) (.+)$")

def parse(path):
    cur = None
    rows = []
    for line in open(path, encoding="utf-8", errors="replace"):
        m = MOD_RE.match(line)
        if m:
            cur = {"mod": int(m.group(1)), "name": m.group(2).strip(), "files": []}
            rows.append(cur)
            continue
        if cur is not None:
            f = FILE_RE.match(line)
            if f:
                cur["files"].append({"path": f.group(2), "sha": f.group(1)})
    return rows

def main():
    ver = sys.argv[1] if len(sys.argv) > 1 else "b"
    kws = [k.lower() for k in sys.argv[2:]] or ["water"]
    base = "/Users/yuluo/Desktop/game/games_test/reverse-engineering/tiny-glade"
    rows = parse(f"{base}/evidence/multi-baseline/pdb-exports/{ver}_files.txt")
    prefix = "/home/h3/code/pouncelight/country-slice-private/"
    hits = []
    for r in rows:
        matched = []
        for f in r["files"]:
            p = f["path"].lower()
            if p.endswith(".rs") and any(k in p for k in kws):
                rel = f["path"]
                if rel.startswith(prefix):
                    rel = rel[len(prefix):]
                matched.append({"path": rel, "sha8": f["sha"][:8] if f["sha"] else None})
        if matched:
            hits.append({"mod": r["mod"], "name": r["name"],
                         "files": sorted(matched, key=lambda x: x["path"])})
    print(json.dumps({"version": ver, "keywords": sys.argv[2:] or ["water"],
                      "modules_total": len(rows), "hit_modules": len(hits),
                      "modules": hits}, ensure_ascii=False, indent=2))

if __name__ == "__main__":
    main()
