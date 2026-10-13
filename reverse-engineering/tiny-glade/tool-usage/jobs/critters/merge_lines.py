#!/usr/bin/env python3
# -*- coding: utf-8 -*-
"""Merge disasm JSON (VA + insns) with PDB line table (offset->line)."""
import json
import gzip
import sys


def load_disasm(path):
    d = json.load(open(path))
    p = d.get('payload', d)
    return p


def load_lines(source_relative, module_hint=None):
    """Load lines records from skeleton B pdb jsonl.gz. Returns list of
    (offset, line) sorted."""
    path = f'skeleton/B/crates/systems/critters/src/{source_relative}'
    out = []
    with gzip.open(path, 'rt') as fh:
        for line in fh:
            line = line.strip()
            if not line:
                continue
            rec = json.loads(line)
            if rec.get('event') == 'lines':
                for e in rec.get('lines', []):
                    out.append((e['offset'], e['line']))
    out.sort()
    return out


def lines_for_function(lines, size, lo_ln=None, hi_ln=None):
    """Keep offsets < size and (optionally) line numbers within a source
    range, so cross-function PDB records do not corrupt attribution."""
    return [(off, ln) for off, ln in lines
            if off < size and (lo_ln is None or ln >= lo_ln) and (hi_ln is None or ln <= hi_ln)]


def annotate(disasm_path, lines_path, size, out_path, lo_ln=None, hi_ln=None):
    p = load_disasm(disasm_path)
    entry = int(p['entry'], 16) if 'entry' in p else int(p['insns'][0][0], 16)
    lines = lines_for_function(load_lines(lines_path), size, lo_ln, hi_ln)
    # build offset->line lookup using largest offset <= off
    def line_for(off):
        best = None
        for o, ln in lines:
            if o <= off:
                best = ln
            else:
                break
        return best
    rows = []
    for ins in p['insns']:
        va = int(ins[0], 16)
        off = va - entry
        rows.append((off, ins[0], ins[1], ins[2], ins[3], line_for(off)))
    with open(out_path, 'w') as fh:
        for off, va, mnem, ops, hexb, ln in rows:
            fh.write(f'{off:6d} {va} {mnem:<6} {" ".join(ops):<40} {hexb}   L{ln}\n')
    print('wrote', out_path, 'rows', len(rows))


if __name__ == '__main__':
    pass
