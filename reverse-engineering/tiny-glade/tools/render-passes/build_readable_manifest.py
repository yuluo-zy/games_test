#!/usr/bin/env python3
"""Build a per-shader manifest mapping HLSL source file -> oracle variant bins.

Output JSON: {"<hlsl-rel-to-shaders/>": [{"bin":..., "entry":..., "profile":..., "defines":[...]}, ...]}

The TGV define is derived from the bin filename hash (16 hex chars) or the
sentinel `0` when the bin name carries no variant hash. Modules (entry/profile)
come from the embedded dxc command in compile-commands.json.
"""
import json
import re
import sys
from pathlib import Path

ROOT = Path(__file__).resolve().parents[2]  # tiny-glade
CC_PATH = ROOT / 'evidence/render-passes/validation/compile-commands.json'
OUT = ROOT / 'evidence/render-passes/validation/readable-manifest.json'

HASH_RE = re.compile(r'_([0-9a-f]{16})\.bin$')


def bin_defines(bin_name: str, hlsl_path: Path) -> list:
    """Defines required to compile this bin from its HLSL source."""
    m = HASH_RE.search(bin_name)
    tag = m.group(1) if m else '0'
    # Only pass a TGV define when the source actually guards on it.
    src = hlsl_path.read_bytes()
    cand = ('TGV_%s' % tag).encode()
    if cand in src:
        return ['TGV_%s' % tag]
    return []


def main():
    cc = json.load(open(CC_PATH))
    cmds = cc['compile_commands']
    manifest = {}
    for e in cmds:
        hlsl_rel = e['main_source_path'].replace('shaders/', '')
        hlsl_path = ROOT / 'reconstruction/shader-source/shaders' / hlsl_rel
        if not hlsl_path.exists():
            sys.stderr.write('skip missing source %s\n' % hlsl_rel)
            continue
        defines = bin_defines(e['file'], hlsl_path)
        entries = manifest.setdefault(hlsl_rel, [])
        for m in e['modules']:
            entries.append({
                'bin': e['file'],
                'entry': m['entry'],
                'profile': m['profile'],
                'defines': list(defines),
            })
    json.dump(manifest, open(OUT, 'w'), ensure_ascii=False, indent=1, sort_keys=True)
    print('wrote %d source entries -> %s' % (len(manifest), OUT))


if __name__ == '__main__':
    main()
