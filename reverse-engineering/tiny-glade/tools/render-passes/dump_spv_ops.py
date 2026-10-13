#!/usr/bin/env python3
"""Minimal SPIR-V disassembler for codegen comparison.

Prints each instruction as `opcode_name operand_words...` (words decoded where
cheaply possible: strings, ids as %N). Use to compare target vs candidate
modules produced by recompile_check. Not a full disassembler.
"""
import struct, sys
from pathlib import Path
sys.path.insert(0, str(Path(__file__).resolve().parents[1] / 'render-passes'))
import spirv_parse

def words_to_str(ws):
    raw = struct.pack('<' + 'I' * len(ws), *ws)
    return raw.split(b'\x00')[0].decode('utf-8', errors='replace')

def dump(payload, grammar, filter_ops=None, max_lines=None):
    n = len(payload) // 4
    w = struct.unpack('<' + 'I' * n, payload)
    i = 5
    out = []
    while i < n:
        wc = w[i] >> 16
        op = w[i] & 0xffff
        if wc == 0 or i + wc > n:
            break
        a = w[i + 1:i + wc]
        name = grammar.op(op)
        if filter_ops and name not in filter_ops:
            i += wc
            continue
        ops = []
        for j, x in enumerate(a):
            if op in (spirv_parse.OP_NAME, spirv_parse.OP_MEMBER_NAME, 15):
                if j >= 1:
                    ops.append('"%s"' % words_to_str(a[2:] if op == spirv_parse.OP_NAME else a[1:]))
                    break
            if op == 7 or op == 37 or op == 330:  # OpSourceContinued / OpString / dxc
                s = words_to_str(a[1:] if op == 37 else a)
                ops.append(repr(s[:100]))
                break
            ops.append(str(x))
        out.append('%5d %s %s' % (wc, name, ' '.join(ops)))
        i += wc
    print('\n'.join(out[:max_lines] if max_lines else out))

if __name__ == '__main__':
    import argparse
    ap = argparse.ArgumentParser()
    ap.add_argument('spv')
    ap.add_argument('--grammar', default='skeleton/_tools/spirv.core.grammar.json')
    ap.add_argument('--filter', default='')
    ap.add_argument('--max', type=int, default=0)
    args = ap.parse_args()
    g = spirv_parse.Grammar(args.grammar)
    payload = Path(args.spv).read_bytes()
    dump(payload, g, filter_ops=set(x.strip() for x in args.filter.split(',') if x.strip()),
         max_lines=args.max or None)
