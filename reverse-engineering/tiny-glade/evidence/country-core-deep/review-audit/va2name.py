#!/usr/bin/env python
"""VA -> rust-demangled name lookup from sysres-procedures-v3.json (1864 procs).

Usage: va2name.py 0x14089d4c0 [0x...]
Prints one line per VA: <va> <name>
"""
import io, json, sys

IDX = r"D:/game/reverse-engineering/tiny-glade/evidence/country-core-deep/sysres-procedures-v3.json"


def build():
    d = json.load(io.open(IDX, encoding="utf-8"))
    m = {}
    for k, v in d.items():
        for p in v.get("procedures", []):
            va = p.get("va")
            if va:
                m.setdefault(va, []).append(p["name"])
    return m


def main():
    m = build()
    for a in sys.argv[1:]:
        va = a.lower()
        names = m.get(va)
        if names is None:
            # try prefix match
            cands = [k for k in m if k.startswith(va)]
            if not cands:
                print(f"{a}\t<NOT FOUND>")
                continue
            va = cands[0]
            names = m[va]
        for n in names:
            print(f"{a}\t{n}")


if __name__ == "__main__":
    main()
