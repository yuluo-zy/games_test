#!/usr/bin/env python3
"""Build the reconstruction/shader-source/shaders/ placeholder skeleton tree.

One placeholder file per unique resolved source path recorded in
evidence/render-passes/skeleton/per-shader-mapping.json. Each file carries a
`// ==SKELETON==` header block (no HLSL body) noting:

  * mapped .bin file(s) + file sha256 (and per-module sha256),
  * stage / entry / variant,
  * the pass it feeds (bin pass name + catalog family + INDEX.json pass ids),
  * a short sample of embedded names (resource/local/function names from OpName).

`../`-style raw embedded paths are collapsed to their resolved directory
(no literal `..` dirs are created); the raw strings are recorded in the header
and in per-shader-mapping.json.

Usage:
    build_skeleton_tree.py <skeleton_dir> <shader_source_root>
"""

import json
import sys
from pathlib import Path

SKEL_DIR = Path(sys.argv[1])
SHADER_SOURCE = Path(sys.argv[2])

mapping = json.loads((SKEL_DIR / 'per-shader-mapping.json').read_text())
raw_bins = [json.loads(l) for l in (SKEL_DIR / 'raw-per-bin.jsonl').open()]
bin_by_name = {r['bin']: r for r in raw_bins}

# include-only names sample / include count per path from mapping
by_path = {p['path']: p for p in mapping['paths']}


def names_sample(rec):
    """Curated sample of embedded names from a per-bin record (Observation)."""
    names = []
    for m in rec.get('modules', []):
        for n in m.get('debug_names', []):
            if n.startswith(('type.', 'out.var.', 'in.var.', 'param.')):
                continue
            if n not in names:
                names.append(n)
        for b in m.get('bindings', []):
            if b.get('name') and b['name'] not in names:
                names.append(b['name'])
    return names[:12]


def entries_summary(rec):
    out = []
    for m in rec.get('modules', []):
        for e in m.get('entries', []):
            out.append('%s:%s' % (e['stage'].replace('GLCompute', 'cs')
                                  .replace('Vertex', 'vs').replace('Fragment', 'ps'), e['entry']))
    return '; '.join(out) if out else '-'


def local_size(rec):
    sizes = [m.get('local_size') for m in rec.get('modules', []) if m.get('local_size')]
    return json.dumps(sizes[0]) if sizes else '-'


def build_header(path, mp):
    L = []
    L.append('// ==SKELETON==')
    L.append('// SOURCE PATH : %s' % path)
    raw = mp.get('raw_path_variants', [path])
    if len(raw) > 1:
        L.append('// RAW STRINGS : %s' % ' | '.join(raw))
    L.append('// KIND        : %s' % mp['kind'])
    mains = mp.get('main_in_bins', [])
    incs = mp.get('include_in_bins', [])
    if mains:
        L.append('// COMPILED AS MAIN IN (%d bins):' % len(mains))
        for mb in mains:
            rec = bin_by_name.get(mb['bin'])
            L.append('//   %s' % mb['bin'])
            L.append('//     file_sha256  : %s' % mb['file_sha256'])
            L.append('//     pass/stage   : %s (%s) / %s' % (mb['pass'], mb['family'], mb['stage']))
            L.append('//     entry        : %s' % (entries_summary(rec) if rec else '-'))
            L.append('//     variant      : %s' % (mb['variant'] or '-'))
            L.append('//     local_size   : %s' % (local_size(rec) if rec else '-'))
            if mb.get('index_pass_ids'):
                L.append('//     index_pass   : %s (Inference)' % ','.join(mb['index_pass_ids']))
            if rec:
                ns = names_sample(rec)
                if ns:
                    L.append('//     names        : %s' % ', '.join(ns))
    else:
        L.append('// COMPILED AS MAIN IN: none (include-only file)')
    if incs:
        L.append('// ALSO EMBEDDED IN (as include, %d bins):' % len(set(incs)))
        L.append('//   %s' % ', '.join(sorted(set(incs))[:12]) + (' ...' if len(set(incs)) > 12 else ''))
    L.append('// NOTE: skeleton placeholder only — HLSL body not yet reconstructed.')
    return '\n'.join(L) + '\n'


count = 0
created = []
for mp in mapping['paths']:
    path = mp['path']
    rel = path[len('shaders/'):]            # strip the leading shaders/ root
    target = SHADER_SOURCE / rel
    target.parent.mkdir(parents=True, exist_ok=True)
    body = build_header(path, mp)
    target.write_text(body, encoding='utf-8')
    count += 1
    created.append(rel)

print('wrote %d placeholder files under %s' % (count, SHADER_SOURCE))
