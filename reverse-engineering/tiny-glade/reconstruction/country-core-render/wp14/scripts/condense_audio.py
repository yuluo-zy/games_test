#!/usr/bin/env python3
"""Condense the per-module audio dumps to game-relevant lines only.

Keeps: FUNC headers, inline_into host lines, reference_callers that are direct
calls into game systems (skip boilerplate), imports summary, DEPS that name
country_core/startup/systems resources (game glue) — dropping std/bevy inline noise.
Writes under wp14/_analysis/_condensed/.
"""
import glob
import os
import re

ROOT = "/Users/yuluo/Desktop/game/games_test/reverse-engineering/tiny-glade"
SRC = os.path.join(ROOT, "reconstruction", "country-core-render", "wp14", "_analysis")
DST = os.path.join(SRC, "_condensed")
os.makedirs(DST, exist_ok=True)

DROP_DEP_RE = re.compile(r"(rust_standard_library|/rustc/|bevy_|/home/h3/\.cargo)")


def is_interesting(line, in_deps):
    s = line.strip()
    if s.startswith("==== "):
        return True
    if s.startswith("-- named_symbol_entities"):
        return True
    if s.startswith("-- FUNC ") or s.startswith("-- DEPS "):
        return True
    if s.startswith("-- definitions"):
        return True
    if s.startswith("imports: "):
        return True
    if " :: " in s and s.startswith("  "):  # named entity rows
        return True
    if "inline_into " in s:
        # keep hosts in country_core
        if "country_core" in s or "tiny_glade" in s:
            return True
        return False
    if s.startswith("    called_by") and "caller_entry_rva" in s:
        # keep only callers whose entry rva maps to game systems we already know;
        # skip to reduce noise
        return False
    if in_deps:
        if s.startswith("    ["):
            if DROP_DEP_RE.search(s):
                return False
            return True
        return False
    return False


def main():
    for path in sorted(glob.glob(os.path.join(SRC, "*.txt"))):
        name = os.path.basename(path)
        out_lines = []
        in_deps = False
        with open(path, "r", encoding="utf-8") as fh:
            for line in fh:
                s = line.strip()
                if s.startswith("-- DEPS "):
                    in_deps = True
                elif s.startswith("-- FUNC "):
                    in_deps = False
                elif s.startswith("-- named") or s.startswith("-- definitions") or s.startswith("==== ") or s.startswith("imports:"):
                    in_deps = False
                if is_interesting(line, in_deps):
                    out_lines.append(line.rstrip("\n"))
        with open(os.path.join(DST, name), "w", encoding="utf-8") as fh:
            fh.write("\n".join(out_lines))
        print(name, "->", len(out_lines), "lines")


if __name__ == "__main__":
    main()
