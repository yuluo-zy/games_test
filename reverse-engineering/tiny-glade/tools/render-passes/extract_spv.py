#!/usr/bin/env python3
"""Extract SPIR-V module(s) from a game .bin and (optionally) decompile to HLSL.

Usage:
  extract_spv.py <bin_path> [--outdir DIR] [--spirv-cross PATH] [--as-hlsl]
"""
import argparse, json, shutil, subprocess, sys
from pathlib import Path
sys.path.insert(0, str(Path(__file__).resolve().parent))
import spirv_parse

def main():
    ap = argparse.ArgumentParser()
    ap.add_argument('bin')
    ap.add_argument('--outdir', default='/tmp/fv_extract')
    ap.add_argument('--spirv-cross', default='spirv-cross')
    ap.add_argument('--as-hlsl', action='store_true')
    ap.add_argument('--grammar', default=str(Path(__file__).resolve().parents[2] / 'skeleton' / '_tools' / 'spirv.core.grammar.json'))
    args = ap.parse_args()

    blob = Path(args.bin).read_bytes()
    grammar = spirv_parse.Grammar(args.grammar)
    outdir = Path(args.outdir); outdir.mkdir(parents=True, exist_ok=True)
    stem = Path(args.bin).name
    for i, (offset, payload) in enumerate(spirv_parse.iter_modules(blob)):
        mod = spirv_parse.parse_module(payload, grammar)
        if mod is None:
            continue
        spv = outdir / ('%s_m%d_%d.spv' % (stem, i, offset))
        spv.write_bytes(payload)
        print('WROTE', spv, 'size', len(payload))
        entries = ', '.join('%s:%s' % (e['stage'], e['name']) for e in mod['entries'])
        print('  entry:', entries)
        print('  bindings:', json.dumps([list(b) for b in mod['bindings']])[:400])
        print('  localsize:', mod.get('local_size'))
        if args.as_hlsl:
            for e in mod['entries']:
                cmd = ['spirv-cross', '--hlsl', '--shader-model', '64',
                       '--entry', e['name'], str(spv)]
                proc = subprocess.run(cmd, capture_output=True, text=True)
                out = outdir / ('%s_m%d_%d_%s.hlsl' % (stem, i, offset, e['name']))
                out.write_text(proc.stdout + ('\n//---STDERR---\n' + proc.stderr if proc.stderr else ''))
                print('WROTE', out)

if __name__ == '__main__':
    main()
