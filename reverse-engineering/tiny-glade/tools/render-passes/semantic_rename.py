#!/usr/bin/env python3
"""Role-based semantic renaming of vNN temporaries inside lifted shader sections.

Per-section (guard `#if defined(TGV_...)`) processing so the same vNN id in
different variants gets its own name. Only identifiers change; arithmetic,
control flow, bindings and guards are untouched. Output must still pass
readable_check.py (debug-stripped logic hash equality).

Curate JSON: { "<hash>_<STAGE>": { "vNN": "semantic_name", ... } }
Remaining vNN get role-based names from their defining expression.
"""
import json
import re
import sys


def find_sections(text):
    lines = text.split('\n')
    sections = []
    stack = []
    for i, l in enumerate(lines, 1):
        s = l.strip()
        if s.startswith('#if') and 'TGV_' in s:
            h = re.search(r'TGV_([0-9a-f]{16}|0)\)', s)
            st = re.search(r'__SHADER_STAGE_(\w+)', s)
            label = '%s_%s' % ((h.group(1) if h else '?'), (st.group(1) if st else '?'))
            stack.append((i, label))
        elif s.startswith('#endif') and stack:
            start, label = stack.pop()
            sections.append((label, start, i))
    return sections


def extract_type(decl):
    m = re.match(r'\s*(?:static\s+)?(?:const\s+)?(?:uniform\s+)?([\w\[\],\d\* ]+?)\s+(\bv\d+\b)\s*[;=\[]', decl)
    if m:
        return m.group(1).split()[-1]
    m = re.match(r'\s*for\s*\(\s*(\w[\w]*)\s+(\bv\d+\b)\s*=', decl)
    if m:
        return 'uint'
    m = re.match(r'\s*(\w[\w]*)\s+(\bv\d+\b)\s*[;=]', decl)
    if m:
        return m.group(1)
    return 'tmp'


ROLE_PATTERNS = [
    (r'water_noise_texture\.Sample', 'noise'),
    (r'half_depth_tex\.Sample', 'depth_sample'),
    (r'1\.00000202655792236328125f', 'depth'),
    (r'ripple_source_buffer', 'ripple'),
    (r'shadow_map\.', 'shadow'),
    (r'QuadReadAcross', 'quad'),
    (r'asfloat\(|asuint\(', 'bits'),
    (r'reflect\(', 'reflect'),
    (r'normalize\(', 'normal'),
    (r'length\(', 'dist'),
    (r'lerp\(0\.0f, 1\.0f, pow|lerp\(0\.0f, 1\.0f, min', 'march_t'),
    (r'is_snow_theme', 'snow'),
    (r'InterlockedAdd', 'atomic'),
]


def role_of(decl, vtype):
    for pat, role in ROLE_PATTERNS:
        if re.search(pat, decl):
            return role
    if vtype.startswith('bool'):
        return 'flag'
    if vtype.startswith('float2x2') or vtype.startswith('float3x3') or vtype.startswith('float4x4'):
        return 'rot'
    return 'val'


def build_map(lines, start, end, curate):
    used = set(curate.values())
    result = dict(curate)
    counters = {}
    for i in range(start - 1, end):
        for m in re.finditer(r'\bv(\d+)\b', lines[i]):
            v = m.group(0)
            if v in result:
                continue
            decl = lines[i].strip()[:200]
            vtype = extract_type(decl)
            role = role_of(decl, vtype)
            while True:
                n = counters.get(role, 0) + 1
                counters[role] = n
                cand = '%s%d' % (role, n)
                if cand not in used:
                    break
            used.add(cand)
            result[v] = cand
    return result


def apply_rename(text, start, end, rename_map):
    lines = text.split('\n')
    pat = re.compile(r'\b(%s)\b' % '|'.join(map(re.escape, sorted(rename_map, key=len, reverse=True))))
    for i in range(start - 1, end):
        lines[i] = pat.sub(lambda m: rename_map[m.group(1)], lines[i])
    return '\n'.join(lines)


def main():
    path = sys.argv[1]
    curate_json = sys.argv[2] if len(sys.argv) > 2 else None
    text = open(path, encoding='utf-8').read()
    curate_all = json.load(open(curate_json)) if curate_json else {}
    for label, start, end in find_sections(text):
        curate = curate_all.get(label, {})
        smap = build_map(text.split('\n'), start, end, curate)
        if not smap:
            continue
        text = apply_rename(text, start, end, smap)
        leftover = len(re.findall(r'\bv\d+\b', '\n'.join(text.split('\n')[start - 1:end])))
        print('section %s (%d..%d): renamed %d, leftover vNN %d' % (label, start, end, len(smap), leftover))
    open(path, 'w', encoding='utf-8').write(text)
    print('wrote', path)


if __name__ == '__main__':
    main()
