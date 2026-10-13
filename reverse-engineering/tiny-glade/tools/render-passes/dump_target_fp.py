#!/usr/bin/env python3
"""Dump target fingerprints for all bins belonging to given source paths."""
import json, sys
from pathlib import Path

CC = 'evidence/render-passes/validation/compile-commands.json'
GAME = '/Users/yuluo/Desktop/game/ljxsj_92385/Tiny Glade/compiled-shaders'

def main():
    sys.path.insert(0, 'tools/render-passes')
    import spirv_parse
    g = spirv_parse.Grammar('skeleton/_tools/spirv.core.grammar.json')
    cc = json.load(open(CC))
    srcs = sys.argv[1:]
    for c in cc['compile_commands']:
        if c.get('main_source_path') not in srcs:
            continue
        print('=' * 100)
        print('BIN', c['file'])
        blob = (Path(GAME) / c['file']).read_bytes()
        for off, payload in spirv_parse.iter_modules(blob):
            m = spirv_parse.parse_module(payload, g)
            fp = spirv_parse.structural_fingerprint(m)
            print('--- module offset %d  entries=%s localsize=%s' % (off, fp['entries'], fp['local_size']))
            print('    capabilities:', fp['capabilities'])
            for b in fp['bindings']:
                print('    binding:', b)
            hist = fp['opcode_histogram']
            print('    hist:', json.dumps(hist, sort_keys=True))

if __name__ == '__main__':
    main()
