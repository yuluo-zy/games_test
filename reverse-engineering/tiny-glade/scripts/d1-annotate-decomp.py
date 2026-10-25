"""Annotate a decomp file: FUN_/func_0x -> names from va-name-index; list UNK_ constants.
Usage: python scripts/d1-annotate-decomp.py <decomp.txt> [--replace]
"""
import json, re, sys

IDX = r"D:/game/reverse-engineering/tiny-glade/evidence/country-core-deep/systems-recovery/groupD/decorator_visual/va-name-index.json"
nameindex = json.load(open(IDX, encoding="utf-8"))

def resolve(va):
    names = nameindex.get(va)
    if not names:
        return None
    return sorted(names)[0]

def main():
    path = sys.argv[1]
    text = open(path, encoding="utf-8").read()
    funs = {}
    for m in re.finditer(r'\bFUN_([0-9a-f]{6,})\b', text):
        va = '0x' + m.group(1)
        funs[va] = resolve(va)
    for m in re.finditer(r'\bfunc_0x000140([0-9a-f]{5,})\b', text):
        va = '0x' + m.group(1)
        funs[va] = resolve(va)
    unks = sorted(set(re.findall(r'\bUNK_([0-9a-f]{5,})\b', text)))
    out = []
    for va in sorted(funs):
        nm = funs[va]
        short = nm.split('::')[-1] if nm else '?'
        out.append(f"{va} -> {short}")
    out.append('UNKS: ' + ', '.join(unks))
    print('\n'.join(out))
    if '--replace' in sys.argv:
        for va, nm in funs.items():
            if nm:
                hexpart = va[2:]
                text = text.replace('FUN_' + hexpart, nm)
        text = re.sub(r'\bfunc_0x000140([0-9a-f]{5,})\b', lambda m: funs.get('0x'+m.group(1)) or m.group(0), text)
        open(path + '.annotated', 'w', encoding='utf-8').write(text)

if __name__ == "__main__":
    main()
